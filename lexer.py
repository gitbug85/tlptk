
def lex(content: str, characters: str) -> list[str]:

    # Seperate content by the characters listed while discarding spaces
    nimi = ""
    kulupu_nimi = []
    for char in content:
        if char not in characters:
            nimi += char
        else:
            kulupu_nimi.append(nimi)
            nimi = ""
            if char != " ":
                kulupu_nimi.append(char)

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

    final_kulupu_nimi = []
    in_poki_nimi = False
    start = False

    for nimi in asterisk_kulupu_nimi:
        if nimi == "[":
            start = True
        elif nimi == "]":
            in_poki_nimi = False
            final_kulupu_nimi = final_kulupu_nimi[:-2]
            final_kulupu_nimi.append("&")
            final_kulupu_nimi.append("teje+pi+nimi+poki")
        else:
            if in_poki_nimi:
                final_kulupu_nimi.append(nimi)
                final_kulupu_nimi.append("&")
                final_kulupu_nimi.append("meso+pi+nimi+poki")
            elif start:
                final_kulupu_nimi.append(nimi)
                final_kulupu_nimi.append("&")
                final_kulupu_nimi.append("soto+pi+nimi+poki")
                start = False
                in_poki_nimi = True
            else:
                final_kulupu_nimi.append(nimi)

    return final_kulupu_nimi
