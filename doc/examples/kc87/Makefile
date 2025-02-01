all: slides.tap

%.tap: %.com
	./kc87com2tap.py --name SLIDES -o "$@" "$<" 

%.com: %.asm
	z80asm "$<" -o "$@"

clean:
	rm -f *.com *.tap
