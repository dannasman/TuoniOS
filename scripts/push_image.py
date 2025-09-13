#!/usr/bin/python3
import argparse
import math
import serial
import struct
import time

ser = serial.Serial("/dev/ttyUSB0", baudrate=115200, bytesize=8, stopbits=1, write_timeout=0.1)

def parse_arguments():
    parser = argparse.ArgumentParser(prog='push_image', description="push kernel image to board")
    parser.add_argument("--image", type=str, help="path kernel image")

    args = parser.parse_args()
    return args

def wait_for_payload_signal():
    start_time = time.time()
    duration = 20
    count = 0 
    while time.time() - start_time < duration:
        byte = ser.read(1)
        if byte == b'\x03':
            count += 1
        if count == 3:
            return True

    print("Did not receive payload signal in 20 seconds.")
    return False


def push_image():
    args = parse_arguments()
    image = args.image

    received = wait_for_payload_signal()
    if not received:
        ser.close()
        return

    f = open(image, 'rb')

    buf = bytearray(f.read())

    f.close()
    
    size = len(buf)
    ser.write(struct.pack("<i", size))
    print(size)

    if ser.read() != b'O':
        print("Chainloader failed to read size")
        return

    if ser.read() != b'K':
        print("Chainloader failed to read size")
        return

    print("size written.")

    chunk_size = 512
    c = 0
    for i in range(0, size, chunk_size):
        chunk = buf[i:i+chunk_size]
        written = ser.write(chunk)
        c += 1
        print(f"{c}/{math.ceil(size/chunk_size)} chunks written")
    
    print("image written.")

    while True:
        line = ser.readline()
        if line:
            try:
                print(line.decode('utf-8').strip())
            except UnicodeDecodeError:
                pass


push_image()
