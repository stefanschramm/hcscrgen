OFFSET: equ 0x1200

  db 0x01 ; Type: MZ-700 binary
  dm "SLIDES"
  db 0x0d ; string delimiter
  dm "          " ; pad to 17 bytes including delimiter
  dw END - START ; file size
  dw OFFSET ; load address
  dw OFFSET ; entry address
  ds 104, 0x00 ; pad header to 128 bytes

START:

  incbin "slides.bin"

END:
