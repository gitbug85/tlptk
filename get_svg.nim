import std/[os, strutils, re, strformat, xmlparser, xmltree, strtabs]

proc popFirstLine(content: var string): string =
  let idx = content.find('\n')

  if idx == -1:
    result = content
    content = ""
  else:
    result = content[0 ..< idx]
    content = content[(idx + 1) .. ^1]

proc get_svg*(svg_name: string): string =
  var individual_names = svg_name.split(re"&")

  var two_plus = false

  # Ignore glyphs after the first two
  if len(individual_names) >= 2:
    individual_names.setLen(2)
    two_plus = true

  let glyph = individual_names[0]
  let path = "./sitelen_wan" / (glyph & ".svg")
  var content = readFile(path)
  discard popFirstLine(content)
  let root = parseXml(content)

  if root.attrs.isNil:
    root.attrs = newStringTable()

  root.attrs["width"] = "210"
  root.attrs["height"] = "297"
  root.attrs["viewBox"] = "0 0 210 297"
  root.attrs["preserveAspectRatio"] = "none"

  return $root

if paramCount() > 2:
  quit("Error: Wrong parameter count")

let svg_name = paramStr(1)

echo get_svg(svg_name)
