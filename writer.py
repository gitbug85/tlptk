import re
import sys
import copy
import argparse
from pathlib import Path
import xml.etree.ElementTree as ET
from lexer import lex
import subprocess

parser = argparse.ArgumentParser(description="Tool to write TLPTK SVGs")
parser.add_argument("-n", type=str, required=False, help="nasin lon pi ilo sona (path)")
parser.add_argument("-o", type=str, required=False, help="pana open pi sitelen (input content directly)")
args = parser.parse_args()

def write_to_pdf(content: str):
    subprocess.run(["./bin/pdf"])

content = None

if args.n:
    with open(args.n, "r", encoding="utf-8") as file:
        content = file.read()
else:
    content = args.o

write_to_pdf(content)
