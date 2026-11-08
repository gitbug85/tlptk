
def lex(content: str, characters: str) -> list[str]:
    kulupu_nimi = seperate(content, characters)
    kulupu_nimi = postfix_asterisk(kulupu_nimi)
    kulupu_nimi = assign_poki_nimi_glyphs(kulupu_nimi)
    kulupu_nimi = sitelen_lasina(kulupu_nimi)
    kulupu_nimi = replace_quotes(kulupu_nimi)
    return kulupu_nimi

# Seperate content by the characters listed while discarding spaces
# except when inside single quotes.
def seperate(content: str, characters: str):
    nimi = ""
    kulupu_nimi = []
    quote = None

    for char in content:
        if char == "'":
            if quote is None:
                quote = char
            elif quote == char:
                quote = None

        if char not in characters:
            if char != " " or quote is not None:
                nimi += char
        else:
            kulupu_nimi.append(nimi)
            nimi = ""
            if char != " " or quote is not None:
                kulupu_nimi.append(char)

    if nimi:
        kulupu_nimi.append(nimi)

    return kulupu_nimi

def postfix_asterisk(kulupu_nimi):
    kulupu_nimi_sin = []
    last_nimi = ""
    for nimi in kulupu_nimi:
        if nimi == "*":
            last_nimi += nimi
            kulupu_nimi_sin.pop()
            kulupu_nimi_sin.append(last_nimi)
        else:
            last_nimi = nimi
            kulupu_nimi_sin.append(nimi)
    return kulupu_nimi_sin

def assign_poki_nimi_glyphs(kulupu_nimi):
    kulupu_nimi_sin = []
    in_poki_nimi = False
    start = False

    for nimi in kulupu_nimi:
        if nimi == "[":
            start = True
        elif nimi == "]":
            in_poki_nimi = False
            kulupu_nimi_sin = kulupu_nimi_sin[:-2]
            kulupu_nimi_sin.append("&")
            kulupu_nimi_sin.append("teje+pi+nimi+poki")
        else:
            if in_poki_nimi:
                kulupu_nimi_sin.append(nimi)
                kulupu_nimi_sin.append("&")
                kulupu_nimi_sin.append("meso+pi+nimi+poki")
            elif start:
                kulupu_nimi_sin.append(nimi)
                kulupu_nimi_sin.append("&")
                kulupu_nimi_sin.append("soto+pi+nimi+poki")
                start = False
                in_poki_nimi = True
            else:
                kulupu_nimi_sin.append(nimi)
    return kulupu_nimi_sin

def replace_quotes(kulupu_nimi):
    kulupu_nimi_sin = []
    in_quote = False

    for nimi in kulupu_nimi:
        if nimi == '"':
            if in_quote:
                kulupu_nimi_sin.append("to")
                in_quote = False
            else:
                kulupu_nimi_sin.append("te")
                in_quote = True
        else:
            kulupu_nimi_sin.append(nimi)
    return kulupu_nimi_sin

def sitelen_lasina(kulupu_nimi):
    kulupu_nimi_sin = []
    in_sitelen_lasina = False

    for nimi in kulupu_nimi:
        if nimi == "'":
            if in_sitelen_lasina == False:
                in_sitelen_lasina = True
            else:
                in_sitelen_lasina = False
        elif in_sitelen_lasina:
            for char in nimi:
                if char == " ":
                    kulupu_nimi_sin.append("_")
                else:
                    kulupu_nimi_sin.append("letter-" + char)
        else:
            kulupu_nimi_sin.append(nimi)
    return kulupu_nimi_sin
