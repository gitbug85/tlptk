import std/[os, strutils]
import std/re
# nim c ./glyph_data/gen.nim
# ./glyph_data/gen

proc popFirstLine(content: var string): string =
  let idx = content.find('\n')

  if idx == -1:
    result = content
    content = ""
  else:
    result = content[0 ..< idx]
    content = content[(idx + 1) .. ^1]

proc get_svg(svg_name: string): string =
  var individual_names = svg_name.split(re"&")

  # Ignore glyphs after the first two
  if len(individual_names) > 2:
    setLen(individual_names, 2)

  let glyph = individual_names[0]
  let path = "./glyph_data" / (glyph & ".tlsvg")
  var content = readFile(path)
  let first_line = popFirstLine(content)
  echo first_line

  # let second_glyph = individual_names[1]
  # let second_path = "./glyph_data" / second_glyph + ".tlsvg"
  return ""

discard get_svg("toki")
