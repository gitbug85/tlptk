import std/[os, strutils]
# nim c ./glyph_data/gen.nim
# ./glyph_data/gen

let exeDir = getAppDir()

for file in walkFiles(exeDir / "*.tlsvg"):
  if file.endsWith(".tlsvg"):
    let content = readFile(file)

    echo "===== ", file, " ====="
    echo content

# writeFile("./glyph_data/output.svg", content)
