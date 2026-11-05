
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

    final_kulupu_nimi = []
    in_poki_nimi = False

    for nimi in kulupu_nimi_with_poki_nimi:
        if nimi == "'":
            if in_poki_nimi == False:
                in_poki_nimi = True
                print("In")
            else:
                in_poki_nimi = False
                print("Out")

    return kulupu_nimi_with_poki_nimi
