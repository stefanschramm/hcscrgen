#!/usr/bin/env python

import struct
import os
import argparse

def main():

	parser = argparse.ArgumentParser(description='Convert Z 9001 / KC 85/1 / KC 87 COM file to TAP file')
	parser.add_argument('--name', help='Name to be used for tape record (8 chars max, usually uppercase)', default="EXAMPLE")
	parser.add_argument('-o', metavar='OUTPUTFILE', help='Path to the TAP file to save')
	parser.add_argument('--load', help='Memory address where COM file should be loaded (default: 0x0300)', default='0x0300')
	parser.add_argument('--start', help='Memory address of program entry (default: 0x0300)', default='0x0300')
	parser.add_argument('inputfilename', help='Path to the COM file to convert')
	args = parser.parse_args()

	write_tap(
		args.inputfilename,
		filename_out=args.o if args.o != None else "%s.tap" % (args.inputfilename),
		name=str.encode(args.name),
		load_address=int(args.load, 0),
		start_address=int(args.start, 0),
	)

def write_tap(filename_in, filename_out, name, load_address, start_address, block=0):

	infile = open(filename_in, 'rb');
	size = os.stat(filename_in).st_size
	end_address = load_address + size - 1
	filename_out = filename_out

	f = open(filename_out, 'wb')

	# file header
	f.write(b'\xc3KC-TAPE by AF. ')

	# header block
	f.write(struct.pack('b', block))
	block += 1
	f.write(struct.pack('8s', name))
	f.write(b'COM') # filetype
	f.write(b'\x00\x00')
	f.write(b'\x00\x00\x00')
	f.write(b'\x03')
	f.write(struct.pack('h', load_address))
	f.write(struct.pack('h', end_address))
	f.write(struct.pack('h', start_address))
	f.write(b'\x00')
	f.write(b'\x00' * (128 - 24)) # padding

	# data blocks
	while True:
		data = infile.read(128);
		if len(data) == 0:
			break
		if 128 * (block - 1) + len(data) == size:
			block = 0xff
		f.write(struct.pack('B', block))
		f.write(struct.pack('128s', data))
		block += 1

	f.close()

	infile.close()

if __name__ == '__main__':
	main()
