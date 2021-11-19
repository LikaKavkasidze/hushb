# hUShB

Small utility to hide a drive partition entry.

## What's the use case?

I have a flash drive that has two partitions, one is FAT to be compatible with all OSes, and the other is an encrypted LUKS partition. When plugging the drive in a computer that doesn't recognize the LUKS partition, the OS can sometimes ask to format this partition. Running this program hides the partition from the drive by erasing the PTE and saving it somewhere. Therefore, the silly OS doesn't try to erase all the data.

## How to use it?

First, please know what you are doing; this utility can erase your PTE very easily, without any chance of recovering. If you are sure you do want to run this program on your drive, then identify the signature of the drive (for instance by running `fdisk -l`), and run the following command to save the PTE to a file:

```bash
hushb -s [SIGNATURE] [DRIVE]:[PARTITION] > [FILE]
# For instance, to save PTE of /dev/sdd2
hushb -s deadbeef /dev/sdd:2 > hidden.pte
```

The PTE has been saved to a file. You can keep it for as long as you want while you do not modify the partition. To restore the PTE, use the following command:

```bash
hushb -q -s [SIGNATURE] [DRIVE]:[PARTITION] < [FILE]
# Restore the example above
hushb -q -s deadbeef /dev/sdd:2 < hidden.pte
```

The option `-q` stands for "quiet"; you may remove it, but will get weird characters in your terminal. This option also allows for pure erasing of the PTE (for instance if you already saved it previously):

```bash
hushb -q -s deadbeef /dev/sdd:2
```
