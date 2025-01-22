# hcscrgen - Home computer screen contents generator

Takes an image file (for example in PNG format) and converts it to a memory image that can be loaded to the machine for displaying.

It works by splitting the input image into small tiles and compares them with the computer's character set to find the best matching character.

## Usage

    hcscrgen PROFILE INPUTFILE

**Example:**

    hcscrgen sharpmz example.png

Will create `example.png.chars.bin` for the character RAM and `example.png.color.bin` for the color RAM of the Sharp MZ-700. Additionally, `example.png.preview.png` is created for checking the result with an image viewer.

## Available profiles (devices)

Profile name | Device(s)       | Resolution (px) | Character RAM offset | Color RAM offset
-------------| ----------------|-----------------|----------------------|-----------------
c64          | Commodore C 64  | 320x200         | 0x0400               | 0xd800
kc87         | Robotron KC 87  | 320x192         | 0xec00               | 0xe800
sharpmz      | Sharp MZ-700    | 320x200         | 0xd000               | 0xd800

Note: For some machines it's possible to move the memory regions to different locations by configuring the display controller.

### Sharp MZ-700

You can use [RetroLoad](https://retroload.com) to directly load the memory images by specifying the destination address:

#### Load character RAM

    retroload --shortpilot --sharpmznorepeat -f sharpmzgeneric --load d000 example.png.chars.bin

#### Load color RAM

    retroload --shortpilot --sharpmznorepeat -f sharpmzgeneric --load d800 example.png.color.bin

#### Combined loading

Combine the parts first and than load the complete image:

    dd if=example.png.color.bin of=example.png.chars.bin conv=notrunc bs=1 seek=2048
    retroload --shortpilot --sharpmznorepeat -f sharpmzgeneric --load d000 example.png.chars.bin
