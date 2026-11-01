import argparse
import subprocess
from pathlib import Path
import shutil


def convert_svg(image_type: str, path: str, width: int):
    svg_path = Path(path).resolve()

    if not svg_path.is_file():
        raise FileNotFoundError(f"SVG file not found: {svg_path}")

    if image_type not in ("png", "jpg", "webp"):
        raise ValueError("Supported formats: png, jpg, webp")

    if width <= 0:
        raise ValueError("Width must be greater than zero.")

    # Find Inkscape executable.
    inkscape = shutil.which("inkscape")

    if not inkscape:
        raise RuntimeError(
            "Inkscape is not installed or not available in PATH."
        )

    output_path = svg_path.with_suffix(f".{image_type}")

    command = [
        inkscape,
        str(svg_path),
        f"--export-filename={output_path}",
        f"--export-width={width}",
    ]

    # Execute conversion.
    subprocess.run(command, check=True)

    print(f"Converted: {svg_path} -> {output_path}")


def main():
    parser = argparse.ArgumentParser(
        description="Convert SVG to PNG, JPG, or WebP."
    )

    parser.add_argument(
        "-l",
        required=True,
        choices=["png", "jpg", "webp"],
        help="nasin sitelen pi sitelen lukin (image format)"
    )

    parser.add_argument(
        "-n",
        required=True,
        help="nasin lon pi ilo sona (path)"
    )

    parser.add_argument(
        "-p",
        required=True,
        type=int,
        help="suli tawa poka (SVG width)"
    )

    args = parser.parse_args()

    convert_svg(args.l, args.n, args.p)


if __name__ == "__main__":
    main()
