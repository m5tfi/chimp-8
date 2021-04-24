#!/usr/bin/python

import argparse
from pathlib import Path

ignore_files = [
    ".gitignore",
    ".vscode",
    ".idea",
    "Session.vim",
    "rom_list.txt"
]

def generate_list(rom_dir):
    directory = Path(rom_dir).glob("*")
    print("Writing to `rom_list.txt`:")
    with open("rom_list.txt", "w") as f:
        for rom in directory:
            rom_name = str(rom)
            if rom_name in ignore_files:
                continue
            f.write(rom_name + "\n")
            print("- " + rom_name)
    print("done!")


def parse_args():
    parser = argparse.ArgumentParser(
        description="read file names in a folder and write it to a text file.")

    parser.add_argument(
        "-d", "--directory", 
        type=str, 
        required=True,
        help="a directory to scan for files"
    )

    args = parser.parse_args()
    return args.directory

if __name__ == "__main__":
    rom_dir = parse_args()
    generate_list(rom_dir)