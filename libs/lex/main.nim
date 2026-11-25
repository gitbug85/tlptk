import std/options
import std/json
import std/os

# Separate content by the characters listed while discarding spaces
# except when inside single quotes.
proc seperate(content: string, characters: string): seq[string] =
  var nimi = ""
  var kulupuNimi: seq[string] = @[]
  var quote: Option[char] = none(char)

  for c in content:
    if c == '\'':
      if quote.isNone:
        quote = some(c)
      elif quote.get == c:
        quote = none(char)

    if c notin characters:
      if c != ' ' or quote.isSome:
        nimi.add(c)
    else:
      kulupuNimi.add(nimi)
      nimi = ""
      if c != ' ' or quote.isSome:
        kulupuNimi.add($c)

  if nimi.len > 0:
    kulupuNimi.add(nimi)

  return kulupuNimi

proc postfixAsterisk(kulupuNimi: seq[string]): seq[string] =
  var kulupuNimiSin: seq[string] = @[]
  var lastNimi = ""

  for nimi in kulupuNimi:
    if nimi == "*":
      lastNimi.add(nimi)
      kulupuNimiSin.setLen(kulupuNimiSin.len - 1)
      kulupuNimiSin.add(lastNimi)
    else:
      lastNimi = nimi
      kulupuNimiSin.add(nimi)

  return kulupuNimiSin

proc assignPokiNimiGlyphs(kulupuNimi: seq[string]): seq[string] =
  var kulupuNimiSin: seq[string] = @[]
  var inPokiNimi = false
  var start = false

  for nimi in kulupuNimi:
    if nimi == "[":
      start = true
    elif nimi == "]":
      inPokiNimi = false
      kulupuNimiSin.setLen(kulupuNimiSin.len - 2)
      kulupuNimiSin.add("&")
      kulupuNimiSin.add("teje+pi+nimi+poki")
    else:
      if inPokiNimi:
        kulupuNimiSin.add(nimi)
        kulupuNimiSin.add("&")
        kulupuNimiSin.add("meso+pi+nimi+poki")
      elif start:
        kulupuNimiSin.add(nimi)
        kulupuNimiSin.add("&")
        kulupuNimiSin.add("soto+pi+nimi+poki")
        start = false
        inPokiNimi = true
      else:
        kulupuNimiSin.add(nimi)

  return kulupuNimiSin

proc replaceQuotes(kulupuNimi: seq[string]): seq[string] =
  var kulupuNimiSin: seq[string] = @[]
  var inQuote = false

  for nimi in kulupuNimi:
    if nimi == "\"":
      if inQuote:
        kulupuNimiSin.add("to")
        inQuote = false
      else:
        kulupuNimiSin.add("te")
        inQuote = true
    else:
      kulupuNimiSin.add(nimi)

  return kulupuNimiSin

proc sitelenLasina(kulupuNimi: seq[string]): seq[string] =
  var kulupuNimiSin: seq[string] = @[]
  var inSitelenLasina = false

  for nimi in kulupuNimi:
    if nimi == "'":
      inSitelenLasina = not inSitelenLasina
    elif inSitelenLasina:
      for c in nimi:
        if c == ' ':
          kulupuNimiSin.add("_")
        else:
          kulupuNimiSin.add("letter-" & $c)
    else:
      kulupuNimiSin.add(nimi)

  return kulupuNimiSin

proc joinWithAmpersand(kulupuNimi: seq[string]): seq[string] =
  var kulupuNimiSin: seq[string] = @[]
  var i = 0

  while i < kulupuNimi.len:
    if i + 2 < kulupuNimi.len and kulupuNimi[i + 1] == "&":
      kulupuNimiSin.add(
        kulupuNimi[i] & "&" & kulupuNimi[i + 2]
      )
      i += 3
    else:
      kulupuNimiSin.add(kulupuNimi[i])
      i += 1

  return kulupuNimiSin

proc removeEmpty(kulupuNimi: seq[string]): seq[string] =
  var kulupuNimiSin: seq[string] = @[]

  for nimi in kulupuNimi:
    if nimi != "":
      kulupuNimiSin.add(nimi)

  return kulupuNimiSin

proc lex(content: string) =
  var kulupuNimi = seperate(content, "*& \n\t_[]'\".")
  kulupuNimi = postfixAsterisk(kulupuNimi)
  kulupuNimi = assignPokiNimiGlyphs(kulupuNimi)
  kulupuNimi = sitelenLasina(kulupuNimi)
  kulupuNimi = replaceQuotes(kulupuNimi)
  kulupuNimi = joinWithAmpersand(kulupuNimi)
  kulupuNimi = removeEmpty(kulupuNimi)
  let jsonNode = %* kulupuNimi
  let jsonString = $jsonNode
  echo jsonString

proc main() =
  if paramCount() != 1:
    quit("Usage: program <path>", 1)

  let path = paramStr(1)
  let content = readFile(path)

  lex(content)

main()
