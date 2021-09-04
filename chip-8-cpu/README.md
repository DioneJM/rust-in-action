
# Chip 8  CPU Emulator
Basic emulator of the Chip 8 CPU as described in the book.

The emulator features 16 registers, 4kb of memory and call stack depth of 16 calls.

## Supported Instructions

0x0000 - End Program
0x00EE - Return from call
0x8__4 - Addition of two registers where the 3rd and 4th byte represent the two registers to add
0x2nnn - Jump to memory address nnn 