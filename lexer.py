
def lex(content: str, characters: str):
    nimi = ""
    kulupu_nimi = []
    for char in content:
        if char not in characters:
            nimi += char
            print(nimi)
        else:
            kulupu_nimi.append(nimi)
            nimi = ""
            kulupu_nimi.append(char)

    print(kulupu_nimi)
