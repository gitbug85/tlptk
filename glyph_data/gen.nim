import std/[os, strutils]
import std/re
# nim c ./glyph_data/gen.nim
# ./glyph_data/gen

proc get_svg(svg_name: string):
  let individual_names = text.split(re"&")

  # Ignore glyphs after the first two
  if len(individual_names) > 2:
    individual_names.setLen(2)

  # Correct extension?
  for file in walkFiles(exeDir / "*.tlsvg"):
    if file.endsWith(".tlsvg"):
      if file.startsWith()

let exeDir = getAppDir()

for file in walkFiles(exeDir / "*.tlsvg"):
  if file.endsWith(".tlsvg"):
    let content = readFile(file)

    writeFile("./glyph_data/dump/output.svg", content)
