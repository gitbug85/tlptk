
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
    new_kulupu_nimi = []
    last_nimi = ""
    for nimi in kulupu_nimi:
        if nimi == "*":
            last_nimi += nimi
            new_kulupu_nimi.pop()
            new_kulupu_nimi.append(last_nimi)
        else:
            last_nimi = nimi
            new_kulupu_nimi.append(nimi)

    return new_kulupu_nimi
