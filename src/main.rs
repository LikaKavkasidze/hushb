#[macro_use]
extern crate clap;
extern crate atty;

use std::fs::OpenOptions;
use std::io::{ Read, Seek, SeekFrom, self, Write };

const ID_OFFSET: usize = 0x01B8;
const ID_SIZE: usize = 4;
const PTE_OFFSET: usize = 0x01BE;
const PTE_SIZE: usize = 16;

fn char_to_dec(c: u8) -> Option<u8> {
	match c {
		0x30..=0x39 => Some(c - 0x30),
		0x41..=0x46 => Some(c - 0x37),
		0x61..=0x66 => Some(c - 0x57),
		_ => None,
	}
}

fn main() {
	// Prepare stdin & stdout and match arguments
	let mut stdin = io::stdin();
	let mut stdout = io::stdout();

	let arg_matches = clap_app!(hushb =>
		(version: "0.1.0")
		(author: "Titouan (Stalone) S. <talone@boxph.one>")
		(about: "Hide a drive partition entry")
		(@arg INPUT: +required "Path to the drive")
		(@arg id: -s --id +takes_value "Signature of the drive")
		(@arg quiet: -q --quiet "Do not print current PTE to stdout")
	).get_matches();

	// Extract drive and partition ID
	let full_dev = arg_matches.value_of("INPUT").unwrap();
	let splitted: Vec<&str> = full_dev.split(':').collect();

	if splitted.len() != 2  {
		panic!("Please enter both a device and a part number");
	}

	let dev = splitted[0];
	let part_id: u8 = splitted[1].parse().unwrap();

	// Open the drive
	let mut usb_handle = OpenOptions::new()
		.read(true)
		.write(true)
		.open(dev)
		.unwrap();

	// Read Master Boot Record
	let mbr: Vec<u8> = (&usb_handle)
		.bytes()
		.take(512)
		.map(|x| x.unwrap())
		.collect();

	// Check device ID if requested
	if let Some(str_id) = arg_matches.value_of("id") {
		let id: Vec<u8> = str_id
			.as_bytes()
			.chunks(2)
			.map(|c| {
				// Convert from string to hex
				char_to_dec(c[0]).unwrap() * 16 + char_to_dec(c[1]).unwrap()
			})
			.collect();

		for i in 0..ID_SIZE {
			if id[ID_SIZE - i - 1] != mbr[ID_OFFSET + i] {
				panic!("USB identifier doesn't match, aborting.");
			}
		}
	}

	// Print PTE to stdout
	let offset = PTE_OFFSET + ((part_id as usize) - 1) * PTE_SIZE;
	let mut current_pte: [u8; 16] = [0u8; 16];

	for i in 0..PTE_SIZE {
		current_pte[i] = mbr[offset + i];
	}

	if !arg_matches.is_present("quiet") {
		stdout.write(&current_pte[..]).unwrap();
	}

	// Overwrite PTE either with stdin content or zeroes
	let mut final_pte: [u8; 16] = [0u8; 16];

	// Read stdin only if data was piped
	if !atty::is(atty::Stream::Stdin) {
		match stdin.read(&mut final_pte[..]) {
			Result::Ok(rb) => {
				if rb != 16 {
					panic!("Too few bytes in stdin, aborting.");
				}
			},
			Result::Err(_) => panic!("Couldn't read from stdin, aborting."),
		}
	}

	// Go to the PTE and write data
	usb_handle.seek(SeekFrom::Start(offset as u64)).unwrap();

	usb_handle.write(&final_pte[0..PTE_SIZE]).unwrap();
	usb_handle.flush().unwrap();
}
