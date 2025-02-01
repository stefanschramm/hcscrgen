; slide show example for KC 87

IMAGE_COUNT: equ 3
SCREEN_MEMORY: equ 0xec00
COLOR_MEMORY: equ 0xe800
IMAGE_SIZE: equ 0x3c0

  org 0x0300

; header
  jp MAIN
  db 'SLIDES  ',0
  db 0

MAIN:
  call RESET_COLOR
FIRST_IMAGE:
  ld a, 0 ; current image
  ld hl, IMAGES
NEXT_IMAGE:
  ld de, SCREEN_MEMORY
  ld bc, IMAGE_SIZE
  ldir
  push af
  call WAIT
  pop af
  inc a
  cp IMAGE_COUNT
  jr nz, NEXT_IMAGE
  jr FIRST_IMAGE

RESET_COLOR:
  ld bc, IMAGE_SIZE
  ld hl, COLOR_MEMORY
RESET_COLOR_LOOP:
  ld (hl), 01110000b
  inc hl
  dec bc
  ld a, b
  or c
  jr nz, RESET_COLOR_LOOP
  ret

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
  incbin "0.bin"
  incbin "1.bin"
  incbin "2.bin"
