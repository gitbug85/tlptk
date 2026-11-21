| Name | Syntax |
| --- | --- |
| pini suli | pisu |
| ni sulu | nisu |
| sama suli | sasu |

Glyphs to Add:
 - X te
 - X to
 - X pisu
 - X nisu
 - X sasu
 - X mute |||
 - X sewi الله

Generally Add:
 - X Nimi pi lipu suli pi toki pona
 - X Less space between glyphs
 - X Cartouche (poki nimi)
 - X [] denote cartouche
 - X Solve error why it's not writing last glyph
 - X Solve problem of not updating the svg in Rust (Python isn't the problem bc it can successfully write the SVG)
 - X Add sitelen lasina with ''
 - X Add te and to as ""
 - X Change p, u, and maybe w letters
 - X Add a license
 - X Split syntax into a seperate file
 - X Change the ui to look better
 - X Add Nim file to transform text file into svg
 - X Move get_svg.nim up a directory
 - X Work on pdf generation using Rusts printpdf library
 - X Fix glyph sizing issue
 - X Get writer.py to call binary from pdf library
 - X Organize via libs folder
 - Install CMake
 - Make c++ lib to take pdf path, make a internal pdf interpretation, turn that into a jpeg before handing that back to the program that ran it
 - Get sitelen lasina from writer over to pdf to write

Possibly add: (Put it into generally add when you want to. These are ideas)
 - Keyword rotate for when a glyph should be rotated when stacked or nested
 - Functionality to stack glyphs
 - Add autocomplete
 - There is like a rendering problem or something where it shows spaces between glyphs. (Seems like a resizing issue?? Output is the same)
 - Edventually replace the direct insertion of glyphs with overlays of another font which is just smaller (it could also be used for subscript and superscript)
 - While writing sitelen pona show sitelen lasina with yellow highlight to show it's unfinished
 - When typing command to rebuild can add an option to only rebuild certain binaries

Do NOT Add:
 - ANY of the u words (pu, ku, etc...)

Dependencies yet to install in workflow:
cmake   sudo apt install cmake
g++   sudo apt install build-essential

sudo apt install libpoppler-cpp-dev
sudo apt install poppler-utils <- May not be useful
