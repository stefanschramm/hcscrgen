; slide show example for Sharp MZ-700

IMAGE_COUNT: equ 3
SCREEN_MEMORY: equ 0xd000
COLOR_MEMORY: equ 0xd800
IMAGE_SIZE: equ 0x3e8

  org 0x1200

FIRST_IMAGE:
  ld a, 0 ; current image
  ld hl, IMAGES
NEXT_IMAGE:
  ; copy character data
  ld de, SCREEN_MEMORY
  ld bc, IMAGE_SIZE
  ldir
  ; copy color data
  ld de, COLOR_MEMORY
  ld bc, IMAGE_SIZE
  ldir
  push af
  call WAIT
  pop af
  inc a
  cp IMAGE_COUNT
  jr nz, NEXT_IMAGE
  jr FIRST_IMAGE

WAIT:
  ld de, 0x0005
WAIT_OUTER_LOOP:
  ld bc, 0xf000
WAIT_INNER_LOOP:
  dec bc
  ld a, b
  or c
  jr nz, WAIT_INNER_LOOP
  dec de
  ld a, d
  or e
  jr nz, WAIT_OUTER_LOOP
  ret

IMAGES:
  incbin "0.chars.bin"
  incbin "0.color.bin"
  incbin "1.chars.bin"
  incbin "1.color.bin"
  incbin "2.chars.bin"
  incbin "2.color.bin"
