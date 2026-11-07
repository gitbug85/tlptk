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

  var two_plus = false

  # Ignore glyphs after the first two
  if len(individual_names) >= 2:
    individual_names.setLen(2)
    two_plus = true

  let glyph = individual_names[0]
  let path = "./glyph_data" / (glyph & ".tlsvg")
  var content = readFile(path)
  var first_line = popFirstLine(content)

  if startsWith(first_line, "insa"):
    first_line = first_line[4..^1]
    var ratio = first_line[0..1]
    first_line = first_line[0..1]
    var x_center = first_line[0..1]
    first_line = first_line[0..1]
    var y_center = first_line[0..1]
    first_line = first_line[0..1]

    if two_plus:
       let glyph_2 = individual_names[1]
       let path_2 = "./glyph_data" / (glyph_2 & ".tlsvg")
       var content_2 = readFile(path_2) & "</svg>"
       discard popFirstLine(content_2)
       content = content & content_2 & "</svg>"
    else:
      quit("Error: Using insa keyword without two plus glyphs")
  elif startsWith(first_line, "sewi"):
    content = content & "</svg>"
    echo "sewi"
  else:
    quit("Error: No directional indicator in tlsvg")

  echo content
  return ""

discard get_svg("toki&toki")
