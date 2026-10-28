
import re
import sys
import copy
import argparse
from pathlib import Path
import xml.etree.ElementTree as ET

parser = argparse.ArgumentParser(description="Tool to write TLPTK SVGs")
parser.add_argument("-w", type=int, required=True, help="suli pi sitelen wan (glyph cell size)")
parser.add_argument("-n", type=str, required=True, help="nasin pi ilo sona (input text file)")
parser.add_argument("-p", type=int, required=True, help="suli tawa poka (SVG width)")
parser.add_argument("-s", type=int, required=True, help="suli (SVG height)")
args = parser.parse_args()

SVG_NS = "http://www.w3.org/2000/svg"
ET.register_namespace("", SVG_NS)

GLYPH_DIR = Path("./sitelen_wan")
GLYPH_SPACING = -0.5*args.w

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
    w = args.w

    # Split into individual whitespace characters so that
    # repeated spaces and consecutive newlines are handled.
    fragments = re.split(r"(\n)|\s+", content)
    fragments = [x for x in fragments if x is not None]
    print(fragments)

    overlay_next = False

    prev_x = None
    prev_y = None

    for frag in fragments:
        if frag == "":
            continue

        if frag == "\n":
            x = 0
            y += w
            prev_x = None
            prev_y = None
            continue

        if frag in ("\r", "\t", "\v", "\f"):
            if frag == "\t":
                x += 4 * w
            continue

        if frag == "&":
            overlay_next = True
            continue

        # If the previous fragment was &, overlay this glyph
        # on the previous glyph instead of advancing.
        if prev_x is not None and prev_y is not None and overlay_next:
            glyph_path = GLYPH_DIR / f"{frag}.svg"
            glyph = load_glyph(glyph_path, prev_x, prev_y, w)
            svg.append(glyph)
            overlay_next = False
            continue

        glyph_path = GLYPH_DIR / f"{frag}.svg"
        glyph = load_glyph(glyph_path, x, y, w)
        svg.append(glyph)

        prev_x, prev_y = x, y
        x += w + GLYPH_SPACING
        overlay_next = False


def interpret():
    if min(args.w, args.p, args.s) <= 0:
        parser.error("-w, -p, and -s must be positive integers")

    input_path = Path(args.n)

    try:
        content = input_path.read_text(encoding="utf-8")
    except OSError as exc:
        parser.error(f"Cannot read input file: {exc}")

    svg = ET.Element(
        tag("svg"),
        {
            "version": "1.1",
            "width": str(args.p),
            "height": str(args.s),
            "viewBox": f"0 0 {args.p} {args.s}",
            "style": "background-color: white;",
        },
    )

    try:
        draw(content, svg)
    except (OSError, ET.ParseError, ValueError) as exc:
        parser.error(str(exc))

    output_path = input_path.parent / (input_path.stem + ".svg")

    ET.ElementTree(svg).write(
        output_path,
        encoding="utf-8",
        xml_declaration=True,
    )

    print(f"SVG written to {output_path}")


if __name__ == "__main__":
    interpret()
