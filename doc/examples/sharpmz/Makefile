Z80_ASM=z80asm

all: slides.mzf

%.mzf: %.mzf.asm %.bin
	$(Z80_ASM) "$<" -o "$@"

%.bin: %.bin.asm
	$(Z80_ASM) "$<" -o "$@"

clean:
	rm -f slides.bin slides.mzf
