# Home computer screen generator

Takes an image file (like PNG) and converts it to a memory image that can be loaded to the Sharp MZ-700 for displaying.

It works by splitting the input image into small tiles and compares them with the computer's character set to find the best matching character.

Support for other home computers may be added in later versions.

## Usage

    hcscrgen example.png

Will create `example.png.chars.bin` for the character RAM and `example.png.color.bin` for the color RAM.

## Sharp MZ-700

Memory Area   | Address
--------------|---------
Character RAM | 0xd000
Color RAM     | 0xd800

You can use [RetroLoad](https://retroload.com) to load the memory images by specifying the load address:

### Load character RAM

    retroload --shortpilot --sharpmznorepeat -f sharpmzgeneric --load d000 example.png.chars.bin

### Load color RAM

    retroload --shortpilot --sharpmznorepeat -f sharpmzgeneric --load d800 example.png.color.bin

### Combined loading

Combine the parts first and than load the complete image:

    dd if=example.png.color.bin of=example.png.chars.bin conv=notrunc bs=1 seek=2048
    retroload --shortpilot --sharpmznorepeat -f sharpmzgeneric --load d000 example.png.chars.bin
