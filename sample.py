from pathlib import Path
import xml.etree.ElementTree as ET

SPACE = -40
folder_path = Path("./sitelen_wan")
output_path = folder_path / "sample.svg"

SVG_NS = "http://www.w3.org/2000/svg"
ET.register_namespace("", SVG_NS)

files = sorted(
    f for f in folder_path.iterdir()
    if f.is_file()
    and f.suffix.lower() == ".svg"
    and f != output_path
)

sampled = ET.Element(
    f"{{{SVG_NS}}}svg",
    {"version": "1.1"}
)

total_width = 0
max_height = 0


def dimensions(root):
    view_box = root.get("viewBox")

    if view_box:
        _, _, width, height = map(
            float, view_box.replace(",", " ").split()
        )
    else:
        width = float(root.get("width", "100").replace("px", ""))
        height = float(root.get("height", "100").replace("px", ""))

    return width, height


for file in files:
    root = ET.parse(file).getroot()
    width, height = dimensions(root)

    # Copy only artwork, excluding editor metadata.
    group = ET.SubElement(
        sampled,
        f"{{{SVG_NS}}}g",
        {"transform": f"translate({total_width}, 0)"}
    )

    for child in list(root):
        tag = child.tag.split("}")[-1]

        if tag in {"namedview", "metadata", "title", "desc"}:
            continue

        group.append(child)

    total_width += width + SPACE
    max_height = max(max_height, height)

if not files:
    raise ValueError("No SVG files found.")

total_width -= SPACE

sampled.set("width", str(total_width))
sampled.set("height", str(max_height))
sampled.set("viewBox", f"0 0 {total_width} {max_height}")

ET.ElementTree(sampled).write(
    output_path,
    encoding="utf-8",
    xml_declaration=True
)

print(f"Saved SVG sample: {output_path}")
