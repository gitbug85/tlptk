import std/[os, strutils]
import std/re
import std/strformat
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

proc get_svg*(svg_name: string): string =
  var individual_names = svg_name.split(re"&")

  var two_plus = false

  # Ignore glyphs after the first two
  if len(individual_names) >= 2:
    individual_names.setLen(2)
    two_plus = true

  let glyph = individual_names[0]
  let path = "./nasin_lukin_pi_sitelen_pi_telo_kule" / (glyph & ".tlsvg")
  var content = readFile(path)
  var first_line = popFirstLine(content)

  if startsWith(first_line, "insa"):
    first_line = first_line[4..^1]
    var ratio = parseFloat(first_line[0..2])
    first_line = first_line[3..^1]
    var x_center = first_line[0..1]
    first_line = first_line[2..^1]
    var y_center = first_line[0..1]
    first_line = first_line[2..^1]

    if two_plus:
       let glyph_2 = individual_names[1]
       let path_2 = "./nasin_lukin_pi_sitelen_pi_telo_kule" / (glyph_2 & ".tlsvg")
       var content_2 = readFile(path_2) & "</svg>"
       discard popFirstLine(content_2)
       content = content & fmt"""<svg x="{x_center}" y="{y_center}" height="{int(ratio*100)}" width="{int(ratio*100)}" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">""" & content_2 & "</svg>" & "</svg>"
    else:
      content = content & "</svg>"
  elif startsWith(first_line, "sewi"):
    content = content & "</svg>"
    echo "sewi"
  else:
    quit("Error: No directional indicator in tlsvg")

  return content

echo get_svg("pona")
