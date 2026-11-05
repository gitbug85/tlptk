
def lex(content: str, characters: str) -> list[str]:

    # Seperate content by the characters listed while discarding spaces
    # except when inside single quotes.

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

    # Attach * as a postfix
    asterisk_kulupu_nimi = []
    last_nimi = ""
    for nimi in kulupu_nimi:
        if nimi == "*":
            last_nimi += nimi
            asterisk_kulupu_nimi.pop()
            asterisk_kulupu_nimi.append(last_nimi)
        else:
            last_nimi = nimi
            asterisk_kulupu_nimi.append(nimi)

    # poki nimi

    kulupu_nimi_with_poki_nimi = []
    in_poki_nimi = False
    start = False

    for nimi in asterisk_kulupu_nimi:
        if nimi == "[":
            start = True
        elif nimi == "]":
            in_poki_nimi = False
            kulupu_nimi_with_poki_nimi = kulupu_nimi_with_poki_nimi[:-2]
            kulupu_nimi_with_poki_nimi.append("&")
            kulupu_nimi_with_poki_nimi.append("teje+pi+nimi+poki")
        else:
            if in_poki_nimi:
                kulupu_nimi_with_poki_nimi.append(nimi)
                kulupu_nimi_with_poki_nimi.append("&")
                kulupu_nimi_with_poki_nimi.append("meso+pi+nimi+poki")
            elif start:
                kulupu_nimi_with_poki_nimi.append(nimi)
                kulupu_nimi_with_poki_nimi.append("&")
                kulupu_nimi_with_poki_nimi.append("soto+pi+nimi+poki")
                start = False
                in_poki_nimi = True
            else:
                kulupu_nimi_with_poki_nimi.append(nimi)

    lasina_kulupu_nimi = []
    in_sitelen_lasina = False

    for nimi in kulupu_nimi_with_poki_nimi:
        if nimi == "'":
            if in_sitelen_lasina == False:
                in_sitelen_lasina = True
            else:
                in_sitelen_lasina = False
        elif in_sitelen_lasina:
            for char in nimi:
                if char == " ":
                    lasina_kulupu_nimi.append("_")
                else:
                    lasina_kulupu_nimi.append("letter-" + char)
        else:
            lasina_kulupu_nimi.append(nimi)

    final_kulupu_nimi = []
    in_quote = False

    for nimi in lasina_kulupu_nimi:
        if nimi == '"':
            if in_quote:
                final_kulupu_nimi.append("to")
                in_quote = False
            else:
                final_kulupu_nimi.append("te")
                in_quote = True
        else:
            final_kulupu_nimi.append(nimi)


    return final_kulupu_nimi
