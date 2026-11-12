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
parser.add_argument("-p", type=int, required=True, help="suli tawa poka (SVG width)")
parser.add_argument("-s", type=int, required=True, help="suli (SVG height)")
parser.add_argument("-o", type=str, required=False, help="pana open pi sitelen (input content directly)")
args = parser.parse_args()

SVG_NS = "http://www.w3.org/2000/svg"
ET.register_namespace("", SVG_NS)

GLYPH_DIR = Path("./sitelen_wan")

def tag(name):
    return f"{{{SVG_NS}}}{name}"


def read_viewbox(root):
    """Get a usable viewBox from a source SVG."""
    viewbox = root.get("viewBox")
    if viewbox:
        return viewbox

    def dimension(value):
        if not value:
            return None
        match = re.match(r"^\s*([\d.]+)", value)
        return float(match.group(1)) if match else None

    width = dimension(root.get("width"))
    height = dimension(root.get("height"))

    if not width or not height:
        raise ValueError("SVG needs a viewBox or valid width and height")

    return f"0 0 {width} {height}"


def load_glyph(svg_path, x, y, cell_size):
    """Embed a glyph SVG inside the output SVG."""
    if not svg_path.is_file():
        raise FileNotFoundError(f"Glyph not found: {svg_path}")

    source = ET.parse(svg_path).getroot()

    # Create a nested SVG viewport, scaling the original glyph
    # to fit within its cell while preserving its aspect ratio.
    glyph = ET.Element(
        tag("svg"),
        {
            "x": str(x),
            "y": str(y),
            "width": str(cell_size),
            "height": str(cell_size),
            "viewBox": read_viewbox(source),
            "preserveAspectRatio": "xMidYMid meet",
        },
    )

    for child in source:
        glyph.append(copy.deepcopy(child))

    return glyph


def draw(content: str, svg: ET.Element):
    """Lay out glyphs, spaces, and lines in the output SVG."""
    x = 0
    y = 0

    fragments = lex(content, "*& \n\t_[]'\"")
    print(fragments)

    for frag in fragments:
        if frag == "":
            continue
        elif frag == "\n":
            x = 0
            y += 100
            continue
        try:
            command = ["./get_svg", frag]

            result = subprocess.run(
                command,
                capture_output=True,
                text=True,
                check=True
            )

            glyph = ET.fromstring(result.stdout)
            glyph.set("transform", f"translate({x}, {y})")
            svg.append(glyph)

            x += 110
            print(x)

        except subprocess.CalledProcessError as e:
            print(f"Binary failed with exit code {e.returncode}")
            print("Error output:", e.stderr)

        except FileNotFoundError:
            print("Error: The specified binary could not be found.")


def interpret():
    if not (args.n or args.o):
        parser.error("Must provide either path or input text using -n or -o respectively")
    if min(args.p, args.s) <= 0:
        parser.error("-w, -p, and -s must be positive integers")

    try:
        if args.o:
            content = args.o
        else:
            if Path(args.n).suffix != ".tlptk":
                sys.exit("Error: File should end in .tlptk")
            content = Path(args.n).read_text(encoding="utf-8")
    except OSError as exc:
        parser.error(f"Cannot read input file: {exc}")

    svg = ET.Element(
        tag("svg"),
        {
            "version": "1.1",
            "width": str(args.p),
            "height": str(args.s),
            "viewBox": f"0 0 {args.p} {args.s}",
        },
    )

    ET.SubElement(
        svg,
        tag("rect"),
        {
            "x": "0",
            "y": "0",
            "width": "100%",
            "height": "100%",
            "fill": "#FFFFFF",
        },
    )
    try:
        draw(content, svg)
    except (OSError, ET.ParseError, ValueError) as exc:
        parser.error(str(exc))

    if args.n:
        output_path = Path(args.n).parent / (Path(args.n).stem + ".svg")
    else:
        output_path = Path("./test/output.svg")

    ET.ElementTree(svg).write(
        output_path,
        encoding="utf-8",
        xml_declaration=True,
    )

    print(f"SVG written to {output_path}")

    return ET.tostring(svg, encoding='utf-8').decode('utf-8')

def main():
    return interpret()

if __name__ == '__main__':
    interpret()
