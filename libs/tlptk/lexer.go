package main

func Lex(content string, characters string) []string {
    kulupuNimi := separate(content, characters)
    kulupuNimi = postfixAsterisk(kulupuNimi)
    kulupuNimi = assignPokiNimiGlyphs(kulupuNimi)
    kulupuNimi = sitelenLasina(kulupuNimi)
    kulupuNimi = replaceQuotes(kulupuNimi)
    kulupuNimi = joinWithAmpersand(kulupuNimi)
    return kulupuNimi
}

// Separate content by the specified characters while discarding spaces,
// except when inside single quotes.
func separate(content string, characters string) []string {
    var nimi string
    var kulupuNimi []string
    var quote rune

    for _, char := range content {
        if char == '\'' {
            if quote == 0 {
                quote = char
            } else if quote == char {
                quote = 0
            }
        }

        if !containsRune(characters, char) {
            if char != ' ' || quote != 0 {
                nimi += string(char)
            }
        } else {
            kulupuNimi = append(kulupuNimi, nimi)
            nimi = ""

            if char != ' ' || quote != 0 {
                kulupuNimi = append(kulupuNimi, string(char))
            }
        }
    }

    if nimi != "" {
        kulupuNimi = append(kulupuNimi, nimi)
    }

    return kulupuNimi
}

// Check whether a string contains a rune.
func containsRune(s string, r rune) bool {
    for _, char := range s {
        if char == r {
            return true
        }
    }
    return false
}

func postfixAsterisk(kulupuNimi []string) []string {
    var result []string
    lastNimi := ""

    for _, nimi := range kulupuNimi {
        if nimi == "*" {
            lastNimi += nimi
            result = result[:len(result)-1]
            result = append(result, lastNimi)
        } else {
            lastNimi = nimi
            result = append(result, nimi)
        }
    }

    return result
}

func assignPokiNimiGlyphs(kulupuNimi []string) []string {
    var result []string
    inPokiNimi := false
    start := false

    for _, nimi := range kulupuNimi {
        if nimi == "[" {
            start = true
        } else if nimi == "]" {
            inPokiNimi = false
            if len(result) >= 2 {
                result = result[:len(result)-2]
            }
            result = append(result, "&", "teje+pi+nimi+poki")
        } else {
            if inPokiNimi {
                result = append(result,
                    nimi,
                    "&",
                    "meso+pi+nimi+poki",
                )
            } else if start {
                result = append(result,
                    nimi,
                    "&",
                    "soto+pi+nimi+poki",
                )
                start = false
                inPokiNimi = true
            } else {
                result = append(result, nimi)
            }
        }
    }

    return result
}

func replaceQuotes(kulupuNimi []string) []string {
    var result []string
    inQuote := false

    for _, nimi := range kulupuNimi {
        if nimi == `"` {
            if inQuote {
                result = append(result, "to")
                inQuote = false
            } else {
                result = append(result, "te")
                inQuote = true
            }
        } else {
            result = append(result, nimi)
        }
    }

    return result
}

func sitelenLasina(kulupuNimi []string) []string {
    var result []string
    inSitelenLasina := false

    for _, nimi := range kulupuNimi {
        if nimi == "'" {
            inSitelenLasina = !inSitelenLasina
        } else if inSitelenLasina {
            for _, char := range nimi {
                if char == ' ' {
                    result = append(result, "_")
                } else {
                    result = append(result, "letter-"+string(char))
                }
            }
        } else {
            result = append(result, nimi)
        }
    }

    return result
}

func joinWithAmpersand(kulupuNimi []string) []string {
    var result []string

    for i := 0; i < len(kulupuNimi); {
        if i+2 < len(kulupuNimi) && kulupuNimi[i+1] == "&" {
            result = append(
                result,
                kulupuNimi[i]+"&"+kulupuNimi[i+2],
            )
            i += 3
        } else {
            result = append(result, kulupuNimi[i])
            i++
        }
    }

    return result
}
