// Adapted from https://gitlab.com/pymarc/pymarc/-/blob/main/pymarc/marc8_mapping.py
// MARC-8 Code Tables: https://www.loc.gov/marc/specifications/specchartables.html

pub const EXTENDED_ARABIC: [(u32, (u16, bool)); 90] = [
    (0xA1, (0x6FD, false)), // DOUBLE ALEF WITH HAMZA ABOVE / ARABIC SIGN SINDHI AMPERSAND
    (0xA2, (0x672, false)), // ARABIC LETTER ALEF WITH WAVY HAMZA ABOVE
    (0xA3, (0x673, false)), // ARABIC LETTER ALEF WITH WAVY HAMZA BELOW
    (0xA4, (0x679, false)), // ARABIC LETTER TTEH
    (0xA5, (0x67A, false)), // ARABIC LETTER TTEHEH
    (0xA6, (0x67B, false)), // ARABIC LETTER BBEH
    (0xA7, (0x67C, false)), // ARABIC LETTER TEH WITH RING
    (0xA8, (0x67D, false)), // ARABIC LETTER TEH WITH THREE DOTS ABOVE DOWNWARDS
    (0xA9, (0x67E, false)), // ARABIC LETTER PEH
    (0xAA, (0x67F, false)), // ARABIC LETTER TEHEH
    (0xAB, (0x680, false)), // ARABIC LETTER BEHEH
    (0xAC, (0x681, false)), // ARABIC LETTER HAH WITH HAMZA ABOVE
    (0xAD, (0x682, false)), // ARABIC LETTER HAH WITH TWO ABOVE DOTS VERTICAL ABOVE
    (0xAE, (0x683, false)), // ARABIC LETTER NYEH
    (0xAF, (0x684, false)), // ARABIC LETTER DYEH
    (0xB0, (0x685, false)), // ARABIC LETTER HAH WITH THREE DOTS ABOVE
    (0xB1, (0x686, false)), // ARABIC LETTER TCHEH
    (0xB2, (0x6BF, false)), // ARABIC LETTER TCHEH WITH DOT ABOVE
    (0xB3, (0x687, false)), // ARABIC LETTER TCHEHEH
    (0xB4, (0x688, false)), // ARABIC LETTER DDAL
    (0xB5, (0x689, false)), // ARABIC LETTER DAL WITH RING
    (0xB6, (0x68A, false)), // ARABIC LETTER DAL WITH DOT BELOW
    (0xB7, (0x68B, false)), // ARABIC LETTER DAL WITH DOT BELOW AND SMALL TAH
    (0xB8, (0x68C, false)), // ARABIC LETTER DAHAL
    (0xB9, (0x68D, false)), // ARABIC LETTER DDAHAL
    (0xBA, (0x68E, false)), // ARABIC LETTER DUL
    (0xBB, (0x68F, false)), // ARABIC LETTER DAL WITH THREE DOTS ABOVE DOWNWARDS
    (0xBC, (0x690, false)), // ARABIC LETTER DAL WITH FOUR DOTS ABOVE
    (0xBD, (0x691, false)), // ARABIC LETTER RREH
    (0xBE, (0x692, false)), // ARABIC LETTER REH WITH SMALL V
    (0xBF, (0x693, false)), // ARABIC LETTER REH WITH RING
    (0xC0, (0x694, false)), // ARABIC LETTER REH WITH DOT BELOW
    (0xC1, (0x695, false)), // ARABIC LETTER REH WITH SMALL V BELOW
    (0xC2, (0x696, false)), // ARABIC LETTER REH WITH DOT BELOW AND DOT ABOVE
    (0xC3, (0x697, false)), // ARABIC LETTER REH WITH TWO DOTS ABOVE
    (0xC4, (0x698, false)), // ARABIC LETTER JEH
    (0xC5, (0x699, false)), // ARABIC LETTER REH WITH FOUR DOTS ABOVE
    (0xC6, (0x69A, false)), // ARABIC LETTER SEEN WITH DOT BELOW AND DOT ABOVE
    (0xC7, (0x69B, false)), // ARABIC LETTER SEEN WITH THREE DOTS BELOW
    (0xC8, (0x69C, false)), // ARABIC LETTER SEEN WITH THREE DOTS BELOW AND THREE DOTS ABOVE
    (0xC9, (0x6FA, false)), // ARABIC LETTER SHEEN WITH DOT BELOW
    (0xCA, (0x69D, false)), // ARABIC LETTER SAD WITH TWO DOTS BELOW
    (0xCB, (0x69E, false)), // ARABIC LETTER SAD WITH THREE DOTS ABOVE
    (0xCC, (0x6FB, false)), // ARABIC LETTER DAD WITH DOT BELOW
    (0xCD, (0x69F, false)), // ARABIC LETTER TAH WITH THREE DOTS ABOVE
    (0xCE, (0x6A0, false)), // ARABIC LETTER AIN WITH THREE DOTS ABOVE
    (0xCF, (0x6FC, false)), // ARABIC LETTER GHAIN WITH DOT BELOW
    (0xD0, (0x6A1, false)), // ARABIC LETTER DOTLESS FEH
    (0xD1, (0x6A2, false)), // ARABIC LETTER FEH WITH DOT MOVED BELOW
    (0xD2, (0x6A3, false)), // ARABIC LETTER FEH WITH DOT BELOW
    (0xD3, (0x6A4, false)), // ARABIC LETTER VEH
    (0xD4, (0x6A5, false)), // ARABIC LETTER FEH WITH THREE DOTS BELOW
    (0xD5, (0x6A6, false)), // ARABIC LETTER PEHEH
    (0xD6, (0x6A7, false)), // ARABIC LETTER QAF WITH DOT ABOVE
    (0xD7, (0x6A8, false)), // ARABIC LETTER QAF WITH THREE DOTS ABOVE
    (0xD8, (0x6A9, false)), // ARABIC LETTER KEHEH
    (0xD9, (0x6AA, false)), // ARABIC LETTER SWASH KAF
    (0xDA, (0x6AB, false)), // ARABIC LETTER KAF WITH RING
    (0xDB, (0x6AC, false)), // ARABIC LETTER KAF WITH DOT ABOVE
    (0xDC, (0x6AD, false)), // ARABIC LETTER NG
    (0xDD, (0x6AE, false)), // ARABIC LETTER KAF WITH THREE DOTS BELOW
    (0xDE, (0x6AF, false)), // ARABIC LETTER GAF
    (0xDF, (0x6B0, false)), // ARABIC LETTER GAF WITH RING
    (0xE0, (0x6B1, false)), // ARABIC LETTER NGOEH
    (0xE1, (0x6B2, false)), // ARABIC LETTER GAF WITH TWO DOTS BELOW
    (0xE2, (0x6B3, false)), // ARABIC LETTER GUEH
    (0xE3, (0x6B4, false)), // ARABIC LETTER GAF WITH THREE DOTS ABOVE
    (0xE4, (0x6B5, false)), // ARABIC LETTER LAM WITH SMALL V
    (0xE5, (0x6B6, false)), // ARABIC LETTER LAM WITH DOT ABOVE
    (0xE6, (0x6B7, false)), // ARABIC LETTER LAM WITH THREE DOTS ABOVE
    (0xE7, (0x6B8, false)), // ARABIC LETTER LAM WITH THREE DOTS BELOW
    (0xE8, (0x6BA, false)), // ARABIC LETTER NOON GHUNNA
    (0xE9, (0x6BB, false)), // ARABIC LETTER RNOON
    (0xEA, (0x6BC, false)), // ARABIC LETTER NOON WITH RING
    (0xEB, (0x6BD, false)), // ARABIC LETTER NOON WITH THREE DOTS ABOVE
    (0xEC, (0x6B9, false)), // ARABIC LETTER NOON WITH DOT BELOW
    (0xED, (0x6BE, false)), // ARABIC LETTER HEH DOACHASHMEE
    (0xEE, (0x6C0, false)), // HEH WITH HAMZA ABOVE / ARABIC LETTER HEH WITH YEH ABOVE
    (0xEF, (0x6C4, false)), // ARABIC LETTER WAW WITH RING
    (0xF0, (0x6C5, false)), // KYRGHYZ OE / ARABIC LETTER KIRGHIZ OE
    (0xF1, (0x6C6, false)), // ARABIC LETTER OE
    (0xF2, (0x6CA, false)), // ARABIC LETTER WAW WITH TWO DOTS ABOVE
    (0xF3, (0x6CB, false)), // ARABIC LETTER VE
    (0xF4, (0x6CD, false)), // ARABIC LETTER YEH WITH TAIL
    (0xF5, (0x6CE, false)), // ARABIC LETTER YEH WITH SMALL V
    (0xF6, (0x6D0, false)), // ARABIC LETTER E
    (0xF7, (0x6D2, false)), // ARABIC LETTER YEH BARREE
    (0xF8, (0x6D3, false)), // ARABIC LETTER YEH BARREE WITH HAMZA ABOVE
    (0xFD, (0x306, true)),  // SHORT E / COMBINING BREVE
    (0xFE, (0x30C, true)),  // SHORT U / COMBINING CARON
];

// Extended Latin (ANSEL)
pub const EXTENDED_LATIN: [(u32, (u16, bool)); 69] = [
    (0x88, (0x98, false)),   // NON-SORT BEGIN / START OF STRING
    (0x89, (0x9C, false)),   // NON-SORT END / STRING TERMINATOR
    (0x8D, (0x200D, false)), // JOINER / ZERO WIDTH JOINER
    (0x8E, (0x200C, false)), // NON-JOINER / ZERO WIDTH NON-JOINER
    (0xA1, (0x141, false)),  // UPPERCASE POLISH L / LATIN CAPITAL LETTER L WITH STROKE
    (0xA2, (0xD8, false)),   // UPPERCASE SCANDINAVIAN O / LATIN CAPITAL LETTER O WITH STROKE
    (0xA3, (0x110, false)),  // UPPERCASE D WITH CROSSBAR / LATIN CAPITAL LETTER D WITH STROKE
    (0xA4, (0xDE, false)),   // UPPERCASE ICELANDIC THORN / LATIN CAPITAL LETTER THORN (Icelandic)
    (0xA5, (0xC6, false)),   // UPPERCASE DIGRAPH AE / LATIN CAPITAL LIGATURE AE
    (0xA6, (0x152, false)),  // UPPERCASE DIGRAPH OE / LATIN CAPITAL LIGATURE OE
    (0xA7, (0x2B9, false)),  // SOFT SIGN, PRIME / MODIFIER LETTER PRIME
    (0xA8, (0xB7, false)),   // MIDDLE DOT
    (0xA9, (0x266D, false)), // MUSIC FLAT SIGN
    (0xAA, (0xAE, false)),   // PATENT MARK / REGISTERED SIGN
    (0xAB, (0xB1, false)),   // PLUS OR MINUS / PLUS-MINUS SIGN
    (0xAC, (0x1A0, false)),  // UPPERCASE O-HOOK / LATIN CAPITAL LETTER O WITH HORN
    (0xAD, (0x1AF, false)),  // UPPERCASE U-HOOK / LATIN CAPITAL LETTER U WITH HORN
    (0xAE, (0x2BC, false)),  // ALIF / MODIFIER LETTER RIGHT HALF RING
    (0xB0, (0x2BB, false)),  // AYN / MODIFIER LETTER TURNED COMMA
    (0xB1, (0x142, false)),  // LOWERCASE POLISH L / LATIN SMALL LETTER L WITH STROKE
    (0xB2, (0xF8, false)),   // LOWERCASE SCANDINAVIAN O / LATIN SMALL LETTER O WITH STROKE
    (0xB3, (0x111, false)),  // LOWERCASE D WITH CROSSBAR / LATIN SMALL LETTER D WITH STROKE
    (0xB4, (0xFE, false)),   // LOWERCASE ICELANDIC THORN / LATIN SMALL LETTER THORN (Icelandic)
    (0xB5, (0xE6, false)),   // LOWERCASE DIGRAPH AE / LATIN SMALL LIGATURE AE
    (0xB6, (0x153, false)),  // LOWERCASE DIGRAPH OE / LATIN SMALL LIGATURE OE
    (0xB7, (0x2BA, false)),  // HARD SIGN, DOUBLE PRIME / MODIFIER LETTER DOUBLE PRIME
    (0xB8, (0x131, false)),  // LOWERCASE TURKISH I / LATIN SMALL LETTER DOTLESS I
    (0xB9, (0xA3, false)),   // BRITISH POUND / POUND SIGN
    (0xBA, (0xF0, false)),   // LOWERCASE ETH / LATIN SMALL LETTER ETH (Icelandic)
    (0xBC, (0x1A1, false)),  // LOWERCASE O-HOOK / LATIN SMALL LETTER O WITH HORN
    (0xBD, (0x1B0, false)),  // LOWERCASE U-HOOK / LATIN SMALL LETTER U WITH HORN
    (0xC0, (0xB0, false)),   // DEGREE SIGN
    (0xC1, (0x2113, false)), // SCRIPT SMALL L
    (0xC2, (0x2117, false)), // SOUND RECORDING COPYRIGHT
    (0xC3, (0xA9, false)),   // COPYRIGHT SIGN
    (0xC4, (0x266F, false)), // MUSIC SHARP SIGN
    (0xC5, (0xBF, false)),   // INVERTED QUESTION MARK
    (0xC6, (0xA1, false)),   // INVERTED EXCLAMATION MARK
    (0xC7, (0xDF, false)),   // ESZETT SYMBOL
    (0xC8, (0x20AC, false)), // EURO SIGN
    (0xE0, (0x309, true)),   // PSEUDO QUESTION MARK / COMBINING HOOK ABOVE
    (0xE1, (0x300, true)),   // GRAVE / COMBINING GRAVE ACCENT (Varia)
    (0xE2, (0x301, true)),   // ACUTE / COMBINING ACUTE ACCENT (Oxia)
    (0xE3, (0x302, true)),   // CIRCUMFLEX / COMBINING CIRCUMFLEX ACCENT
    (0xE4, (0x303, true)),   // TILDE / COMBINING TILDE
    (0xE5, (0x304, true)),   // MACRON / COMBINING MACRON
    (0xE6, (0x306, true)),   // BREVE / COMBINING BREVE (Vrachy)
    (0xE7, (0x307, true)),   // SUPERIOR DOT / COMBINING DOT ABOVE
    (0xE8, (0x308, true)),   // UMLAUT, DIAERESIS / COMBINING DIAERESIS (Dialytika)
    (0xE9, (0x30C, true)),   // HACEK / COMBINING CARON
    (0xEA, (0x30A, true)),   // CIRCLE ABOVE, ANGSTROM / COMBINING RING ABOVE
    (0xEB, (0xFE20, true)),  // LIGATURE, FIRST HALF / COMBINING LIGATURE LEFT HALF
    (0xEC, (0xFE21, true)),  // LIGATURE, SECOND HALF / COMBINING LIGATURE RIGHT HALF
    (0xED, (0x315, true)),   // HIGH COMMA, OFF CENTER / COMBINING COMMA ABOVE RIGHT
    (0xEE, (0x30B, true)),   // DOUBLE ACUTE / COMBINING DOUBLE ACUTE ACCENT
    (0xEF, (0x310, true)),   // CANDRABINDU / COMBINING CANDRABINDU
    (0xF0, (0x327, true)),   // CEDILLA / COMBINING CEDILLA
    (0xF1, (0x328, true)),   // RIGHT HOOK, OGONEK / COMBINING OGONEK
    (0xF2, (0x323, true)),   // DOT BELOW / COMBINING DOT BELOW
    (0xF3, (0x324, true)),   // DOUBLE DOT BELOW / COMBINING DIAERESIS BELOW
    (0xF4, (0x325, true)),   // CIRCLE BELOW / COMBINING RING BELOW
    (0xF5, (0x333, true)),   // DOUBLE UNDERSCORE / COMBINING DOUBLE LOW LINE
    (0xF6, (0x332, true)),   // UNDERSCORE / COMBINING LOW LINE
    (0xF7, (0x326, true)),   // LEFT HOOK (COMMA BELOW) / COMBINING COMMA BELOW
    (0xF8, (0x31C, true)),   // RIGHT CEDILLA / COMBINING LEFT HALF RING BELOW
    (0xF9, (0x32E, true)),   // UPADHMANIYA / COMBINING BREVE BELOW
    (0xFA, (0xFE22, true)),  // DOUBLE TILDE, FIRST HALF / COMBINING DOUBLE TILDE LEFT HALF
    (0xFB, (0xFE23, true)),  // DOUBLE TILDE, SECOND HALF / COMBINING DOUBLE TILDE RIGHT HALF
    (0xFE, (0x313, true)),   // HIGH COMMA, CENTERED / COMBINING COMMA ABOVE (Psili)
];

pub const BASIC_ARABIC: [(u32, (u16, bool)); 83] = [
    (0x21, (0x21, false)),   // EXCLAMATION MARK
    (0x22, (0x22, false)),   // QUOTATION MARK
    (0x23, (0x23, false)),   // NUMBER SIGN
    (0x24, (0x24, false)),   // DOLLAR SIGN
    (0x25, (0x66A, false)),  // PERCENT SIGN / ARABIC PERCENT SIGN
    (0x26, (0x26, false)),   // AMPERSAND
    (0x27, (0x27, false)),   // APOSTROPHE
    (0x28, (0x28, false)),   // OPENING PARENTHESIS / LEFT PARENTHESIS
    (0x29, (0x29, false)),   // CLOSING PARENTHESIS / RIGHT PARENTHESIS
    (0x2A, (0x66D, false)),  // ASTERISK / ARABIC FIVE POINTED STAR
    (0x2B, (0x2B, false)),   // PLUS SIGN
    (0x2C, (0x60C, false)),  // ARABIC COMMA
    (0x2D, (0x2D, false)),   // HYPHEN-MINUS
    (0x2E, (0x2E, false)),   // PERIOD, DECIMAL POINT / FULL STOP
    (0x2F, (0x2F, false)),   // SLASH / SOLIDUS
    (0x30, (0x660, false)),  // ARABIC-INDIC DIGIT ZERO
    (0x31, (0x661, false)),  // ARABIC-INDIC DIGIT ONE
    (0x32, (0x662, false)),  // ARABIC-INDIC DIGIT TWO
    (0x33, (0x663, false)),  // ARABIC-INDIC DIGIT THREE
    (0x34, (0x664, false)),  // ARABIC-INDIC DIGIT FOUR
    (0x35, (0x665, false)),  // ARABIC-INDIC DIGIT FIVE
    (0x36, (0x666, false)),  // ARABIC-INDIC DIGIT SIX
    (0x37, (0x667, false)),  // ARABIC-INDIC DIGIT SEVEN
    (0x38, (0x668, false)),  // ARABIC-INDIC DIGIT EIGHT
    (0x39, (0x669, false)),  // ARABIC-INDIC DIGIT NINE
    (0x3A, (0x3A, false)),   // COLON
    (0x3B, (0x61B, false)),  // ARABIC SEMICOLON
    (0x3C, (0x3C, false)),   // LESS-THAN SIGN
    (0x3D, (0x3D, false)),   // EQUALS SIGN
    (0x3E, (0x3E, false)),   // GREATER-THAN SIGN
    (0x3F, (0x61F, false)),  // ARABIC QUESTION MARK
    (0x41, (0x621, false)),  // HAMZAH / ARABIC LETTER HAMZA
    (0x42, (0x622, false)),  // ARABIC LETTER ALEF WITH MADDA ABOVE
    (0x43, (0x623, false)),  // ARABIC LETTER ALEF WITH HAMZA ABOVE
    (0x44, (0x624, false)),  // ARABIC LETTER WAW WITH HAMZA ABOVE
    (0x45, (0x625, false)),  // ARABIC LETTER ALEF WITH HAMZA BELOW
    (0x46, (0x626, false)),  // ARABIC LETTER YEH WITH HAMZA ABOVE
    (0x47, (0x627, false)),  // ARABIC LETTER ALEF
    (0x48, (0x628, false)),  // ARABIC LETTER BEH
    (0x49, (0x629, false)),  // ARABIC LETTER TEH MARBUTA
    (0x4A, (0x62A, false)),  // ARABIC LETTER TEH
    (0x4B, (0x62B, false)),  // ARABIC LETTER THEH
    (0x4C, (0x62C, false)),  // ARABIC LETTER JEEM
    (0x4D, (0x62D, false)),  // ARABIC LETTER HAH
    (0x4E, (0x62E, false)),  // ARABIC LETTER KHAH
    (0x4F, (0x62F, false)),  // ARABIC LETTER DAL
    (0x50, (0x630, false)),  // ARABIC LETTER THAL
    (0x51, (0x631, false)),  // ARABIC LETTER REH
    (0x52, (0x632, false)),  // ARABIC LETTER ZAIN
    (0x53, (0x633, false)),  // ARABIC LETTER SEEN
    (0x54, (0x634, false)),  // ARABIC LETTER SHEEN
    (0x55, (0x635, false)),  // ARABIC LETTER SAD
    (0x56, (0x636, false)),  // ARABIC LETTER DAD
    (0x57, (0x637, false)),  // ARABIC LETTER TAH
    (0x58, (0x638, false)),  // ARABIC LETTER ZAH
    (0x59, (0x639, false)),  // ARABIC LETTER AIN
    (0x5A, (0x63A, false)),  // ARABIC LETTER GHAIN
    (0x5B, (0x5B, false)),   // OPENING SQUARE BRACKET / LEFT SQUARE BRACKET
    (0x5D, (0x5D, false)),   // CLOSING SQUARE BRACKET / RIGHT SQUARE BRACKET
    (0x60, (0x640, false)),  // ARABIC TATWEEL
    (0x61, (0x641, false)),  // ARABIC LETTER FEH
    (0x62, (0x642, false)),  // ARABIC LETTER QAF
    (0x63, (0x643, false)),  // ARABIC LETTER KAF
    (0x64, (0x644, false)),  // ARABIC LETTER LAM
    (0x65, (0x645, false)),  // ARABIC LETTER MEEM
    (0x66, (0x646, false)),  // ARABIC LETTER NOON
    (0x67, (0x647, false)),  // ARABIC LETTER HEH
    (0x68, (0x648, false)),  // ARABIC LETTER WAW
    (0x69, (0x649, false)),  // ARABIC LETTER ALEF MAKSURA
    (0x6A, (0x64A, false)),  // ARABIC LETTER YEH
    (0x6B, (0x64B, true)),   // ARABIC FATHATAN
    (0x6C, (0x64C, true)),   // ARABIC DAMMATAN
    (0x6D, (0x64D, true)),   // ARABIC KASRATAN
    (0x6E, (0x64E, true)),   // ARABIC FATHA
    (0x6F, (0x64F, true)),   // ARABIC DAMMA
    (0x70, (0x650, true)),   // ARABIC KASRA
    (0x71, (0x651, true)),   // ARABIC SHADDA
    (0x72, (0x652, true)),   // ARABIC SUKUN
    (0x73, (0x671, false)),  // ARABIC LETTER ALEF WASLA
    (0x74, (0x670, false)),  // ARABIC LETTER SUPERSCRIPT ALEF
    (0x78, (0x66C, false)),  // ARABIC THOUSANDS SEPARATOR
    (0x79, (0x201D, false)), // RIGHT DOUBLE QUOTATION MARK
    (0x7A, (0x201C, false)), // LEFT DOUBLE QUOTATION MARK
];

// Basic Hebrew
pub const BASIC_HEBREW: [(u32, (u16, bool)); 78] = [
    (0x21, (0x21, false)),  // EXCLAMATION MARK
    (0x22, (0x5F4, false)), // QUOTATION MARK, GERSHAYIM / HEBREW PUNCTUATION GERSHAYIM
    (0x23, (0x23, false)),  // NUMBER SIGN
    (0x24, (0x24, false)),  // DOLLAR SIGN
    (0x25, (0x25, false)),  // PERCENT SIGN
    (0x26, (0x26, false)),  // AMPERSAND
    (0x27, (0x5F3, false)), // APOSTROPHE, GERESH / HEBREW PUNCTUATION GERESH
    (0x28, (0x28, false)),  // OPENING PARENTHESIS / LEFT PARENTHESIS
    (0x29, (0x29, false)),  // CLOSING PARENTHESIS / RIGHT PARENTHESIS
    (0x2A, (0x2A, false)),  // ASTERISK
    (0x2B, (0x2B, false)),  // PLUS SIGN
    (0x2C, (0x2C, false)),  // COMMA
    (0x2D, (0x5BE, false)), // HYPHEN-MINUS, MAKEF / HEBREW PUNCTUATION MAQAF
    (0x2E, (0x2E, false)),  // PERIOD, DECIMAL POINT / FULL STOP
    (0x2F, (0x2F, false)),  // SLASH / SOLIDUS
    (0x30, (0x30, false)),  // DIGIT ZERO
    (0x31, (0x31, false)),  // DIGIT ONE
    (0x32, (0x32, false)),  // DIGIT TWO
    (0x33, (0x33, false)),  // DIGIT THREE
    (0x34, (0x34, false)),  // DIGIT FOUR
    (0x35, (0x35, false)),  // DIGIT FIVE
    (0x36, (0x36, false)),  // DIGIT SIX
    (0x37, (0x37, false)),  // DIGIT SEVEN
    (0x38, (0x38, false)),  // DIGIT EIGHT
    (0x39, (0x39, false)),  // DIGIT NINE
    (0x3A, (0x3A, false)),  // COLON
    (0x3B, (0x3B, false)),  // SEMICOLON
    (0x3C, (0x3C, false)),  // LESS-THAN SIGN
    (0x3D, (0x3D, false)),  // EQUALS SIGN
    (0x3E, (0x3E, false)),  // GREATER-THAN SIGN
    (0x3F, (0x3F, false)),  // QUESTION MARK
    (0x40, (0x5B7, true)),  // HEBREW POINT PATAH
    (0x41, (0x5B8, true)),  // KAMATS / HEBREW POINT QAMATS
    (0x42, (0x5B6, true)),  // HEBREW POINT SEGOL
    (0x43, (0x5B5, true)),  // TSEREH / HEBREW POINT TSERE
    (0x44, (0x5B4, true)),  // HIRIK / HEBREW POINT HIRIQ
    (0x45, (0x5B9, true)),  // HOLAM, LEFT SIN DOT / HEBREW POINT HOLAM
    (0x46, (0x5BB, true)),  // KUBUTS / HEBREW POINT QUBUTS
    (0x47, (0x5B0, true)),  // HEBREW POINT SHEVA
    (0x48, (0x5B2, true)),  // HEBREW POINT HATAF PATAH
    (0x49, (0x5B3, true)),  // HATAF KAMATS / HEBREW POINT HATAF QAMATS
    (0x4A, (0x5B1, true)),  // HEBREW POINT HATAF SEGOL
    (0x4B, (0x5BC, true)),  // HEBREW POINT DAGESH OR MAPIQ
    (0x4C, (0x5BF, true)),  // RAFEH / HEBREW POINT RAFE
    (0x4D, (0x5C1, true)),  // RIGHT SHIN DOT / HEBREW POINT  SHIN DOT
    (0x4E, (0xFB1E, true)), // VARIKA / HEBREW POINT JUDEO-SPANISH VARIKA
    (0x5B, (0x5B, false)),  // OPENING SQUARE BRACKET / LEFT SQUARE BRACKET
    (0x5D, (0x5D, false)),  // CLOSING SQUARE BRACKET / RIGHT SQUARE BRACKET
    (0x60, (0x5D0, false)), // HEBREW LETTER ALEF
    (0x61, (0x5D1, false)), // HEBREW LETTER BET
    (0x62, (0x5D2, false)), // HEBREW LETTER GIMEL
    (0x63, (0x5D3, false)), // HEBREW LETTER DALET
    (0x64, (0x5D4, false)), // HEBREW LETTER HE
    (0x65, (0x5D5, false)), // HEBREW LETTER VAV
    (0x66, (0x5D6, false)), // HEBREW LETTER ZAYIN
    (0x67, (0x5D7, false)), // HEBREW LETTER HET
    (0x68, (0x5D8, false)), // HEBREW LETTER TET
    (0x69, (0x5D9, false)), // HEBREW LETTER YOD
    (0x6A, (0x5DA, false)), // HEBREW LETTER FINAL KAF
    (0x6B, (0x5DB, false)), // HEBREW LETTER KAF
    (0x6C, (0x5DC, false)), // HEBREW LETTER LAMED
    (0x6D, (0x5DD, false)), // HEBREW LETTER FINAL MEM
    (0x6E, (0x5DE, false)), // HEBREW LETTER MEM
    (0x6F, (0x5DF, false)), // HEBREW LETTER FINAL NUN
    (0x70, (0x5E0, false)), // HEBREW LETTER NUN
    (0x71, (0x5E1, false)), // HEBREW LETTER SAMEKH
    (0x72, (0x5E2, false)), // HEBREW LETTER AYIN
    (0x73, (0x5E3, false)), // HEBREW LETTER FINAL PE
    (0x74, (0x5E4, false)), // HEBREW LETTER PE
    (0x75, (0x5E5, false)), // HEBREW LETTER FINAL TSADI
    (0x76, (0x5E6, false)), // HEBREW LETTER TSADI
    (0x77, (0x5E7, false)), // HEBREW LETTER QOF / KOF
    (0x78, (0x5E8, false)), // HEBREW LETTER RESH
    (0x79, (0x5E9, false)), // HEBREW LETTER SHIN
    (0x7A, (0x5EA, false)), // HEBREW LETTER TAV
    (0x7B, (0x5F0, false)), // HEBREW LIGATURE YIDDISH DOUBLE VAV / TSVEY VOVN
    (0x7C, (0x5F1, false)), // HEBREW LIGATURE YIDDISH VAV YOD / VOV YUD
    (0x7D, (0x5F2, false)), // HEBREW LIGATURE YIDDISH DOUBLE YOD / TSVEY YUDN
];

// Chinese, Japanese, Korean (EACC)
pub const EACC: [(u32, (u16, bool)); 15739] = [
    (0x215556, (0x8461, false)), // East Asian ideograph
    (0x6F5557, (0xC5D1, false)), // Korean hangul
    (0x456324, (0x9F61, false)), // East Asian ideograph
    (0x6F5140, (0xBCCF, false)), // Korean hangul
    (0x6F5558, (0xC5D4, false)), // Korean hangul
    (0x213536, (0x53EC, false)), // East Asian ideograph
    (0x6F5D3C, (0xD64B, false)), // Korean hangul
    (0x215559, (0x8438, false)), // East Asian ideograph
    (0x2D555A, (0x8386, false)), // East Asian ideograph
    (0x6F5C7C, (0xD5D0, false)), // Korean hangul
    (0x295B60, (0x9E55, false)), // East Asian ideograph
    (0x2D555B, (0x8385, false)), // East Asian ideograph
    (0x6F555C, (0xC5E3, false)), // Korean hangul
    (0x6F5141, (0xBCD0, false)), // Korean hangul
    (0x27555D, (0x5E2D, false)), // East Asian ideograph
    (0x23555E, (0x9B1F, false)), // East Asian ideograph
    (0x333F24, (0x7718, false)), // East Asian ideograph
    (0x6F555F, (0xC5ED, false)), // Korean hangul
    (0x6F4F5C, (0xB9AC, false)), // Korean hangul
    (0x6F5560, (0xC5EE, false)), // Korean hangul
    (0x6F4E21, (0xB540, false)), // Korean hangul
    (0x4B4146, (0x6362, false)), // East Asian ideograph
    (0x235031, (0x9874, false)), // East Asian ideograph
    (0x225561, (0x7273, false)), // East Asian ideograph
    (0x274257, (0x6BD9, false)), // East Asian ideograph
    (0x295C28, (0x9E58, false)), // East Asian ideograph
    (0x6F5142, (0xBCD1, false)), // Korean hangul
    (0x6F5562, (0xC5F4, false)), // Korean hangul
    (0x213727, (0x5616, false)), // East Asian ideograph
    (0x215563, (0x84C0, false)), // East Asian ideograph
    (0x215564, (0x8499, false)), // East Asian ideograph
    (0x6F562E, (0xC679, false)), // Korean hangul
    (0x2D4674, (0x51B2, false)), // East Asian ideograph
    (0x6F5565, (0xC5FC, false)), // Korean hangul
    (0x4B4147, (0x633F, false)), // East Asian ideograph
    (0x215566, (0x8490, false)), // East Asian ideograph
    (0x6F5143, (0xBCD2, false)), // Korean hangul
    (0x275567, (0x82CD, false)), // East Asian ideograph
    (0x215568, (0x853D, false)), // East Asian ideograph
    (0x6F5569, (0xC600, false)), // Korean hangul
    (0x27314C, (0x6765, false)), // East Asian ideograph
    (0x276071, (0x517B, false)), // East Asian ideograph
    (0x6F556A, (0xC601, false)), // Korean hangul
    (0x33325D, (0x4FA1, false)), // East Asian ideograph
    (0x6F5839, (0xC9DD, false)), // Korean hangul
    (0x2D6B5F, (0x5273, false)), // East Asian ideograph
    (0x21556B, (0x851A, false)), // East Asian ideograph
    (0x6F5144, (0xBCD5, false)), // Korean hangul
    (0x27556C, (0x83B2, false)), // East Asian ideograph
    (0x22556D, (0x727C, false)), // East Asian ideograph
    (0x21556E, (0x852D, false)), // East Asian ideograph
    (0x6F556F, (0xC610, false)), // Korean hangul
    (0x295721, (0x9C86, false)), // East Asian ideograph
    (0x466074, (0x76B2, false)), // East Asian ideograph
    (0x333529, (0x53DC, false)), // East Asian ideograph
    (0x6F5145, (0xBCF4, false)), // Korean hangul
    (0x225571, (0x727F, false)), // East Asian ideograph
    (0x225D42, (0x7521, false)), // East Asian ideograph
    (0x275949, (0x8A89, false)), // East Asian ideograph
    (0x6F5037, (0xBA84, false)), // Korean hangul
    (0x215573, (0x8514, false)), // East Asian ideograph
    (0x215574, (0x84EC, false)), // East Asian ideograph
    (0x4C2330, (0x5C53, false)), // East Asian ideograph
    (0x69656E, (0x7DD5, false)), // East Asian ideograph
    (0x6F5146, (0xBCF5, false)), // Korean hangul
    (0x215576, (0x8569, false)), // East Asian ideograph
    (0x282441, (0x5C98, false)), // East Asian ideograph
    (0x234021, (0x9132, false)), // East Asian ideograph
    (0x4D4176, (0x91DB, false)), // East Asian ideograph
    (0x335577, (0x8602, false)), // East Asian ideograph
    (0x394022, (0x6443, false)), // East Asian ideograph
    (0x6F5578, (0xC634, false)), // Korean hangul
    (0x6F4F5D, (0xB9AD, false)), // Korean hangul
    (0x2D3749, (0x5650, false)), // East Asian ideograph
    (0x287139, (0x7EE8, false)), // East Asian ideograph
    (0x234024, (0x9126, false)), // East Asian ideograph
    (0x6F557A, (0xC637, false)), // Korean hangul
    (0x213C35, (0x5DDE, false)), // East Asian ideograph
    (0x6F5147, (0xBCF6, false)), // Korean hangul
    (0x6F557B, (0xC639, false)), // Korean hangul
    (0x215945, (0x8B66, false)), // East Asian ideograph
    (0x21372C, (0x5606, false)), // East Asian ideograph (variant of 4B372C which maps to 5606)
    (0x27557C, (0x829C, false)), // East Asian ideograph
    (0x224027, (0x69BF, false)), // East Asian ideograph
    (0x23557D, (0x9B34, false)), // East Asian ideograph
    (0x6F557E, (0xC640, false)), // Korean hangul
    (0x2D4029, (0x5214, false)), // East Asian ideograph
    (0x6F5148, (0xBCF8, false)), // Korean hangul
    (0x23402B, (0x9134, false)), // East Asian ideograph
    (0x21372D, (0x5609, false)), // East Asian ideograph
    (0x23402C, (0x9136, false)), // East Asian ideograph
    (0x6F5876, (0xCC14, false)), // Korean hangul
    (0x22402D, (0x69A3, false)), // East Asian ideograph
    (0x22507C, (0x70DC, false)), // East Asian ideograph
    (0x22402E, (0x69A4, false)), // East Asian ideograph
    (0x6F5149, (0xBCFC, false)), // Korean hangul
    (0x6F575F, (0xC8B0, false)), // Korean hangul
    (0x295A75, (0x9E41, false)), // East Asian ideograph
    (0x4B525A, (0x7FFA, false)), // East Asian ideograph
    (0x234031, (0x913A, false)), // East Asian ideograph
    (0x2D383F, (0x575A, false)), // East Asian ideograph
    (0x294371, (0x94FD, false)), // East Asian ideograph
    (0x234032, (0x913B, false)), // East Asian ideograph
    (0x6F5C27, (0xD38C, false)), // Korean hangul
    (0x224034, (0x69D4, false)), // East Asian ideograph
    (0x6F514A, (0xBD04, false)), // Korean hangul
    (0x335F73, (0x9759, false)), // East Asian ideograph
    (0x6F4C33, (0xB17C, false)), // Korean hangul
    (0x4B525B, (0x66DC, false)), // East Asian ideograph (variant of 39525B which maps to 66DC)
    (0x2E7C2E, (0x831C, false)), // East Asian ideograph
    (0x224038, (0x69C3, false)), // East Asian ideograph
    (0x6F5B4D, (0xD280, false)), // Korean hangul
    (0x2D4039, (0x67C6, false)), // East Asian ideograph
    (0x6F514B, (0xBD05, false)), // Korean hangul
    (0x276036, (0x54CD, false)), // East Asian ideograph
    (0x395477, (0x85A6, false)), // East Asian ideograph
    (0x213730, (0x5617, false)), // East Asian ideograph
    (0x4B525C, (0x8002, false)), // East Asian ideograph
    (0x23403B, (0x9143, false)), // East Asian ideograph
    (0x295B6B, (0x9E57, false)), // East Asian ideograph
    (0x2D5963, (0x8C98, false)), // East Asian ideograph
    (0x224C3C, (0x6F3B, false)), // East Asian ideograph
    (0x22403E, (0x6A11, false)), // East Asian ideograph
    (0x6F5A73, (0xD0C8, false)), // Korean hangul
    (0x6F514C, (0xBD07, false)), // Korean hangul
    (0x213B74, (0x5CFD, false)), // East Asian ideograph
    (0x23403F, (0x9145, false)), // East Asian ideograph
    (0x21594A, (0x8B80, false)), // East Asian ideograph
    (0x213731, (0x560D, false)), // East Asian ideograph
    (0x225D49, (0x752F, false)), // East Asian ideograph
    (0x234040, (0x9148, false)), // East Asian ideograph
    (0x224041, (0x6A00, false)), // East Asian ideograph
    (0x295B6C, (0x9E4B, false)), // East Asian ideograph
    (0x234042, (0x9150, false)), // East Asian ideograph
    (0x234043, (0x914E, false)), // East Asian ideograph
    (0x6F514D, (0xBD09, false)), // Korean hangul
    (0x213B75, (0x5CED, false)), // East Asian ideograph
    (0x394A60, (0x9AE6, false)), // East Asian ideograph
    (0x213732, (0x562E, false)), // East Asian ideograph
    (0x334045, (0x629B, false)), // East Asian ideograph
    (0x292A34, (0x86AC, false)), // East Asian ideograph
    (0x224046, (0x69E6, false)), // East Asian ideograph
    (0x2F2D79, (0x88B5, false)), // East Asian ideograph
    (0x234048, (0x9159, false)), // East Asian ideograph
    (0x6F514E, (0xBD10, false)), // Korean hangul
    (0x276039, (0x9877, false)), // East Asian ideograph
    (0x234049, (0x915C, false)), // East Asian ideograph
    (0x33494A, (0x70D6, false)), // East Asian ideograph
    (0x294372, (0x9513, false)), // East Asian ideograph
    (0x22404B, (0x6A0B, false)), // East Asian ideograph
    (0x22404C, (0x69E5, false)), // East Asian ideograph
    (0x2E2F7A, (0x6738, false)), // East Asian ideograph
    (0x22404D, (0x69E9, false)), // East Asian ideograph
    (0x6F514F, (0xBD14, false)), // Korean hangul
    (0x27603A, (0x9879, false)), // East Asian ideograph
    (0x2D4F29, (0x9F9D, false)), // East Asian ideograph
    (0x213734, (0x564E, false)), // East Asian ideograph
    (0x2D404F, (0x6294, false)), // East Asian ideograph
    (0x6F5039, (0xBA87, false)), // Korean hangul
    (0x224050, (0x69FC, false)), // East Asian ideograph
    (0x6F5B4E, (0xD284, false)), // Korean hangul
    (0x234052, (0x915A, false)), // East Asian ideograph
    (0x6F5150, (0xBD24, false)), // Korean hangul
    (0x213B78, (0x5CF0, false)), // East Asian ideograph
    (0x234053, (0x9161, false)), // East Asian ideograph
    (0x6F596B, (0xCE6B, false)), // Korean hangul
    (0x225D4D, (0x753A, false)), // East Asian ideograph
    (0x224054, (0x6A17, false)), // East Asian ideograph
    (0x4B4C3C, (0x7573, false)), // East Asian ideograph
    (0x224056, (0x69E7, false)), // East Asian ideograph
    (0x224057, (0x69EB, false)), // East Asian ideograph
    (0x6F5151, (0xBD48, false)), // Korean hangul
    (0x213B79, (0x5CF6, false)), // East Asian ideograph
    (0x294621, (0x9553, false)), // East Asian ideograph
    (0x4B6266, (0x9ED2, false)), // East Asian ideograph
    (0x6F5631, (0xC688, false)), // Korean hangul
    (0x22405B, (0x69F1, false)), // East Asian ideograph
    (0x6F5152, (0xBD49, false)), // Korean hangul
    (0x27603D, (0x9884, false)), // East Asian ideograph
    (0x22405E, (0x6A2B, false)), // East Asian ideograph
    (0x29444D, (0x952B, false)), // East Asian ideograph
    (0x22405F, (0x69FF, false)), // East Asian ideograph
    (0x224060, (0x6A20, false)), // East Asian ideograph
    (0x234061, (0x916F, false)), // East Asian ideograph
    (0x6F5153, (0xBD4C, false)), // Korean hangul
    (0x27603E, (0x987C, false)), // East Asian ideograph
    (0x234062, (0x916E, false)), // East Asian ideograph
    (0x275D60, (0x94E8, false)), // East Asian ideograph
    (0x6F5A71, (0xD0C1, false)), // Korean hangul
    (0x4B4E21, (0x7B36, false)), // East Asian ideograph
    (0x224064, (0x69ED, false)), // East Asian ideograph
    (0x28355B, (0x6484, false)), // East Asian ideograph
    (0x6F547D, (0xC545, false)), // Korean hangul
    (0x234066, (0x917A, false)), // East Asian ideograph
    (0x6F582E, (0xC9CA, false)), // Korean hangul
    (0x6F5154, (0xBD50, false)), // Korean hangul
    (0x27603F, (0x987D, false)), // East Asian ideograph
    (0x224067, (0x6A1B, false)), // East Asian ideograph
    (0x213739, (0x5657, false)), // East Asian ideograph
    (0x2D7143, (0x55E2, false)), // East Asian ideograph
    (0x234068, (0x9172, false)), // East Asian ideograph
    (0x2D5A63, (0x8DE5, false)), // East Asian ideograph
    (0x2D384A, (0x5872, false)), // East Asian ideograph
    (0x234069, (0x9179, false)), // East Asian ideograph
    (0x27632C, (0x9F9A, false)), // East Asian ideograph
    (0x23406A, (0x9176, false)), // East Asian ideograph
    (0x4C233F, (0x5C76, false)), // East Asian ideograph
    (0x23406B, (0x9174, false)), // East Asian ideograph
    (0x6F5155, (0xBD58, false)), // Korean hangul
    (0x213B7D, (0x5D1B, false)), // East Asian ideograph
    (0x276040, (0x987F, false)), // East Asian ideograph
    (0x23406C, (0x9173, false)), // East Asian ideograph
    (0x23406D, (0x9185, false)), // East Asian ideograph
    (0x22406E, (0x6A18, false)), // East Asian ideograph
    (0x23406F, (0x9182, false)), // East Asian ideograph
    (0x4B5F30, (0x9686, false)), // East Asian ideograph (variant of 215F30 which maps to 9686)
    (0x234070, (0x918A, false)), // East Asian ideograph
    (0x6F5A75, (0xD0D0, false)), // Korean hangul
    (0x213C38, (0x5DE8, false)), // East Asian ideograph
    (0x6F5156, (0xBD59, false)), // Korean hangul
    (0x276041, (0x9881, false)), // East Asian ideograph
    (0x234071, (0x9186, false)), // East Asian ideograph
    (0x21373B, (0x5653, false)), // East Asian ideograph
    (0x234072, (0x918C, false)), // East Asian ideograph
    (0x234073, (0x9181, false)), // East Asian ideograph
    (0x224075, (0x6A0C, false)), // East Asian ideograph
    (0x6F5157, (0xBD64, false)), // Korean hangul
    (0x224076, (0x6A0F, false)), // East Asian ideograph
    (0x21373C, (0x563F, false)), // East Asian ideograph
    (0x4B3F74, (0x623B, false)), // East Asian ideograph
    (0x282F43, (0x6206, false)), // East Asian ideograph
    (0x454738, (0x6CFA, false)), // East Asian ideograph
    (0x275E6A, (0x9610, false)), // East Asian ideograph
    (0x274F36, (0x5E0C, false)), // East Asian ideograph
    (0x6F5E21, (0xD79D, false)), // Korean hangul
    (0x232B24, (0x876A, false)), // East Asian ideograph
    (0x212B25, (0x300C, false)), // Ideographic left corner bracket
    (0x2F5158, (0x7CC7, false)), // East Asian ideograph
    (0x23407B, (0x9191, false)), // East Asian ideograph
    (0x225D55, (0x754A, false)), // East Asian ideograph
    (0x294628, (0x9552, false)), // East Asian ideograph
    (0x4B3050, (0x4E8A, false)), // East Asian ideograph
    (0x22407C, (0x69EE, false)), // East Asian ideograph
    (0x232B27, (0x874E, false)), // East Asian ideograph
    (0x695B37, (0x6737, false)), // East Asian ideograph
    (0x23407D, (0x9190, false)), // East Asian ideograph
    (0x23407E, (0x918E, false)), // East Asian ideograph
    (0x4C6775, (0x7962, false)), // Unrelated variant of EACC 293032 which maps to 7962
    (0x6F5159, (0xBD81, false)), // Korean hangul
    (0x21373E, (0x5637, false)), // East Asian ideograph
    (0x294629, (0x84E5, false)), // East Asian ideograph
    (0x4B3051, (0x5F10, false)), // East Asian ideograph
    (0x6F503B, (0xBAA9, false)), // Korean hangul
    (0x222B2D, (0x602B, false)), // East Asian ideograph
    (0x6F5B50, (0xD290, false)), // Korean hangul
    (0x455847, (0x8A25, false)), // East Asian ideograph (variant of 215847 which maps to 8A25)
    (0x396B2F, (0x521F, false)), // East Asian ideograph
    (0x6F515A, (0xBD84, false)), // Korean hangul
    (0x6F5861, (0xCAD9, false)), // Korean hangul
    (0x222B30, (0x6019, false)), // East Asian ideograph
    (0x225D57, (0x754E, false)), // East Asian ideograph
    (0x4B3052, (0x6275, false)), // East Asian ideograph
    (0x212B31, (0xFF3B, false)), // Ideographic left square bracket
    (0x4B3749, (0x5668, false)), // East Asian ideograph (variant of 213749 which maps to 5668)
    (0x3A3B7D, (0x67B1, false)), // East Asian ideograph
    (0x216B33, (0x5231, false)), // East Asian ideograph
    (0x23504A, (0x98BF, false)), // East Asian ideograph
    (0x4B5F35, (0x6B92, false)), // East Asian ideograph
    (0x474270, (0x94BC, false)), // East Asian ideograph
    (0x6F515B, (0xBD87, false)), // Korean hangul
    (0x212B35, (0x3001, false)), // Ideographic comma
    (0x216B36, (0x5235, false)), // East Asian ideograph
    (0x4B6268, (0x9ED9, false)), // East Asian ideograph
    (0x3F404F, (0x638A, false)), // East Asian ideograph
    (0x695B7B, (0x6926, false)), // East Asian ideograph
    (0x222B38, (0x601B, false)), // East Asian ideograph
    (0x216B39, (0x5233, false)), // East Asian ideograph
    (0x6F485F, (0xAC00, false)), // Korean hangul
    (0x276047, (0x988A, false)), // East Asian ideograph
    (0x212B3A, (0xFF1A, false)), // Ideographic colon
    (0x225D59, (0x754B, false)), // East Asian ideograph
    (0x333F3F, (0x51F4, false)), // East Asian ideograph
    (0x212B3B, (0xFF1F, false)), // Ideographic question mark
    (0x2D3852, (0x51A2, false)), // East Asian ideograph
    (0x295739, (0x9C9E, false)), // East Asian ideograph
    (0x222B3D, (0x6033, false)), // East Asian ideograph
    (0x276B3E, (0x522D, false)), // East Asian ideograph
    (0x6F515D, (0xBD89, false)), // Korean hangul
    (0x276048, (0x9888, false)), // East Asian ideograph
    (0x275A28, (0x8D42, false)), // East Asian ideograph
    (0x225D5A, (0x7548, false)), // East Asian ideograph
    (0x29462D, (0x9549, false)), // East Asian ideograph
    (0x6F4B45, (0xB07D, false)), // Korean hangul
    (0x274F3C, (0x79F0, false)), // East Asian ideograph
    (0x706B42, (0x80BC, false)), // East Asian ideograph
    (0x6F515E, (0xBD90, false)), // Korean hangul
    (0x276049, (0x9891, false)), // East Asian ideograph
    (0x217B75, (0x5AA0, false)), // East Asian ideograph
    (0x706B44, (0x80BD, false)), // East Asian ideograph
    (0x33615A, (0x8EB0, false)), // East Asian ideograph
    (0x222B45, (0x600D, false)), // East Asian ideograph
    (0x2D3854, (0x5896, false)), // East Asian ideograph
    (0x274F3D, (0x79CD, false)), // East Asian ideograph
    (0x28533C, (0x709D, false)), // East Asian ideograph
    (0x69573B, (0x5F41, false)), // East Asian ideograph
    (0x216B47, (0x5260, false)), // East Asian ideograph
    (0x6F5B51, (0xD291, false)), // Korean hangul
    (0x6F515F, (0xBD91, false)), // Korean hangul
    (0x27604A, (0x9893, false)), // East Asian ideograph
    (0x213744, (0x5678, false)), // East Asian ideograph
    (0x4B3057, (0x4E99, false)), // East Asian ideograph
    (0x6F2477, (0x3154, false)), // Korean hangul
    (0x2F4053, (0x914F, false)), // East Asian ideograph
    (0x226B4B, (0x7B37, false)), // East Asian ideograph
    (0x29573C, (0x9C91, false)), // East Asian ideograph
    (0x706B4C, (0x80E9, false)), // East Asian ideograph
    (0x4B5F3A, (0x967A, false)), // East Asian ideograph
    (0x216B4D, (0x525E, false)), // East Asian ideograph
    (0x6F5160, (0xBD93, false)), // Korean hangul
    (0x6F5940, (0xCCAD, false)), // Korean hangul
    (0x29573D, (0x9C92, false)), // East Asian ideograph
    (0x6F5161, (0xBD95, false)), // Korean hangul
    (0x216B53, (0x5255, false)), // East Asian ideograph
    (0x705F50, (0x549D, false)), // East Asian ideograph
    (0x2E6B54, (0x7B04, false)), // East Asian ideograph
    (0x292B55, (0x86F3, false)), // East Asian ideograph
    (0x6F555A, (0xC5E0, false)), // Korean hangul
    (0x70622A, (0x7339, false)), // East Asian ideograph
    (0x6F5162, (0xBD99, false)), // Korean hangul
    (0x212B59, (0xFF0F, false)), // Ideographic solidus
    (0x2D562E, (0x8024, false)), // East Asian ideograph
    (0x213563, (0x5439, false)), // East Asian ideograph
    (0x216B5B, (0x526E, false)), // East Asian ideograph
    (0x6F4B67, (0xB0C8, false)), // Korean hangul
    (0x4B3D24, (0x53A6, false)), // East Asian ideograph
    (0x6F5163, (0xBD9C, false)), // Korean hangul
    (0x225D60, (0x755B, false)), // East Asian ideograph
    (0x276B5F, (0x672D, false)), // East Asian ideograph
    (0x2D433E, (0x667B, false)), // East Asian ideograph
    (0x235053, (0x98C6, false)), // East Asian ideograph
    (0x345452, (0x7118, false)), // East Asian ideograph
    (0x6F5B40, (0xD1D8, false)), // Korean hangul
    (0x213569, (0x5462, false)), // East Asian ideograph
    (0x4C6B62, (0x7B4C, false)), // East Asian ideograph (variant of 226B62 which maps to 7B4C)
    (0x394634, (0x6B96, false)), // East Asian ideograph (variant of 214634 which maps to 6B96)
    (0x274171, (0x629A, false)), // East Asian ideograph
    (0x6F5165, (0xBDF0, false)), // Korean hangul
    (0x28356D, (0x6512, false)), // East Asian ideograph
    (0x274F44, (0x79EF, false)), // East Asian ideograph
    (0x295742, (0x9C95, false)), // East Asian ideograph
    (0x4B3D27, (0x5EC3, false)), // East Asian ideograph
    (0x6F5166, (0xBE0C, false)), // Korean hangul
    (0x6F4D23, (0xB310, false)), // Korean hangul
    (0x213B61, (0x5C64, false)), // East Asian ideograph (variant of 4B3B61 which maps to 5C64)
    (0x4B5277, (0x8068, false)), // East Asian ideograph
    (0x336162, (0x9A23, false)), // East Asian ideograph
    (0x705F51, (0x54D0, false)), // East Asian ideograph
    (0x6F4A46, (0xAE50, false)), // Korean hangul
    (0x292B6E, (0x86F0, false)), // East Asian ideograph
    (0x2D4A60, (0x6C02, false)), // East Asian ideograph
    (0x222B6F, (0x604C, false)), // East Asian ideograph
    (0x6F5167, (0xBE0D, false)), // Korean hangul
    (0x276052, (0x989B, false)), // East Asian ideograph
    (0x6F4D24, (0xB311, false)), // Korean hangul
    (0x232B72, (0x87AC, false)), // East Asian ideograph
    (0x6F496C, (0xAD49, false)), // Korean hangul
    (0x216B74, (0x5282, false)), // East Asian ideograph
    (0x2D5F2E, (0x661C, false)), // East Asian ideograph
    (0x216B75, (0x5281, false)), // East Asian ideograph
    (0x6F5168, (0xBE10, false)), // Korean hangul
    (0x215966, (0x8C8C, false)), // East Asian ideograph
    (0x6F5621, (0xC641, false)), // Korean hangul
    (0x33515C, (0x7DAB, false)), // East Asian ideograph
    (0x275622, (0x8427, false)), // East Asian ideograph
    (0x215623, (0x859B, false)), // East Asian ideograph
    (0x226B79, (0x7B72, false)), // East Asian ideograph
    (0x215624, (0x8591, false)), // East Asian ideograph
    (0x2D496B, (0x70DF, false)), // East Asian ideograph
    (0x226B7A, (0x7B78, false)), // East Asian ideograph
    (0x6F5169, (0xBE14, false)), // Korean hangul
    (0x275626, (0x8537, false)), // East Asian ideograph
    (0x23487C, (0x9481, false)), // East Asian ideograph
    (0x226B7C, (0x7B67, false)), // East Asian ideograph
    (0x6F5627, (0xC654, false)), // Korean hangul
    (0x2D5635, (0x846F, false)), // East Asian ideograph
    (0x215628, (0x8587, false)), // East Asian ideograph
    (0x29352D, (0x8C30, false)), // East Asian ideograph
    (0x275629, (0x84DD, false)), // East Asian ideograph
    (0x21562A, (0x85A9, false)), // East Asian ideograph
    (0x276055, (0x613F, false)), // East Asian ideograph
    (0x6F4D27, (0xB315, false)), // Korean hangul
    (0x225D67, (0x7563, false)), // East Asian ideograph
    (0x273D2F, (0x5385, false)), // East Asian ideograph
    (0x335D23, (0x8A76, false)), // East Asian ideograph
    (0x6F562D, (0xC678, false)), // Korean hangul
    (0x2E2968, (0x5F51, false)), // East Asian ideograph
    (0x21562E, (0x85C9, false)), // East Asian ideograph
    (0x4B3D2C, (0x53B0, false)), // East Asian ideograph
    (0x21562F, (0x85B0, false)), // East Asian ideograph
    (0x4B527C, (0x8080, false)), // East Asian ideograph
    (0x233F4E, (0x9100, false)), // East Asian ideograph
    (0x4B4E39, (0x5CFA, false)), // East Asian ideograph
    (0x275631, (0x827A, false)), // East Asian ideograph
    (0x215632, (0x85EA, false)), // East Asian ideograph
    (0x695633, (0x5CBE, false)), // East Asian ideograph
    (0x6F516C, (0xBE1F, false)), // Korean hangul
    (0x21596A, (0x8CA0, false)), // East Asian ideograph
    (0x213751, (0x5687, false)), // East Asian ideograph
    (0x275635, (0x836F, false)), // East Asian ideograph
    (0x6F5529, (0xC558, false)), // Korean hangul
    (0x235636, (0x9B43, false)), // East Asian ideograph
    (0x275637, (0x853C, false)), // East Asian ideograph
    (0x6F5C2E, (0xD39C, false)), // Korean hangul
    (0x4C5638, (0x729F, false)), // East Asian ideograph
    (0x275639, (0x853A, false)), // East Asian ideograph
    (0x21563A, (0x8606, false)), // East Asian ideograph
    (0x233F50, (0x9107, false)), // East Asian ideograph
    (0x225927, (0x73EA, false)), // East Asian ideograph
    (0x21563B, (0x860B, false)), // East Asian ideograph
    (0x6F5A22, (0xCEAC, false)), // Korean hangul
    (0x21563C, (0x8607, false)), // East Asian ideograph
    (0x224C5E, (0x6F36, false)), // East Asian ideograph
    (0x21563D, (0x860A, false)), // East Asian ideograph
    (0x4B3D2F, (0x5EF0, false)), // East Asian ideograph
    (0x696576, (0x7E90, false)), // East Asian ideograph
    (0x21563E, (0x862D, false)), // East Asian ideograph
    (0x276059, (0x9885, false)), // East Asian ideograph
    (0x21596C, (0x8CA1, false)), // East Asian ideograph
    (0x2D563F, (0x6A97, false)), // East Asian ideograph
    (0x276023, (0x5DE9, false)), // East Asian ideograph
    (0x275640, (0x85D3, false)), // East Asian ideograph
    (0x346126, (0x6900, false)), // East Asian ideograph
    (0x2D3421, (0x5294, false)), // East Asian ideograph
    (0x235641, (0x9B4B, false)), // East Asian ideograph
    (0x215642, (0x863F, false)), // East Asian ideograph
    (0x4B5F49, (0x51CB, false)), // East Asian ideograph
    (0x2D4971, (0x70A4, false)), // East Asian ideograph
    (0x395643, (0x4E55, false)), // East Asian ideograph
    (0x6F4D2C, (0xB35C, false)), // Korean hangul
    (0x213754, (0x5695, false)), // East Asian ideograph
    (0x275644, (0x4E47, false)), // East Asian ideograph
    (0x70602D, (0x55B9, false)), // East Asian ideograph
    (0x692426, (0x3046, false)), // Hiragana letter U
    (0x2D5A7E, (0x8E7B, false)), // East Asian ideograph
    (0x6F5645, (0xC6CC, false)), // Korean hangul
    (0x6F5646, (0xC6CD, false)), // Korean hangul
    (0x2D572B, (0x8797, false)), // East Asian ideograph
    (0x275647, (0x5904, false)), // East Asian ideograph
    (0x215648, (0x865C, false)), // East Asian ideograph
    (0x27605B, (0x98CE, false)), // East Asian ideograph
    (0x28645A, (0x7817, false)), // East Asian ideograph
    (0x6F4D2D, (0xB35F, false)), // Korean hangul
    (0x225D6D, (0x7579, false)), // East Asian ideograph
    (0x21564A, (0x865F, false)), // East Asian ideograph
    (0x2D563C, (0x8613, false)), // East Asian ideograph
    (0x45564B, (0x865E, false)), // East Asian ideograph (variant of 21564B which maps to 865E)
    (0x21564C, (0x8667, false)), // East Asian ideograph
    (0x6F5171, (0xBE4C, false)), // Korean hangul
    (0x27605C, (0x98D2, false)), // East Asian ideograph
    (0x69564E, (0x5D76, false)), // East Asian ideograph
    (0x29302D, (0x88E3, false)), // East Asian ideograph
    (0x22564F, (0x72B4, false)), // East Asian ideograph
    (0x6F496E, (0xAD6C, false)), // Korean hangul
    (0x277169, (0x5522, false)), // East Asian ideograph
    (0x6F5650, (0xC6EC, false)), // Korean hangul
    (0x6F5C2F, (0xD3A0, false)), // Korean hangul
    (0x235061, (0x98E4, false)), // East Asian ideograph
    (0x4B5F4C, (0x9D8F, false)), // East Asian ideograph
    (0x225652, (0x72B5, false)), // East Asian ideograph
    (0x27605D, (0x53F0, false)), // East Asian ideograph (duplicate simplified)
    (0x6F4D2F, (0xB365, false)), // Korean hangul
    (0x6F5653, (0xC704, false)), // Korean hangul
    (0x294642, (0x94E0, false)), // East Asian ideograph
    (0x692429, (0x3049, false)), // Hiragana letter small O
    (0x333F55, (0x5B3E, false)), // East Asian ideograph
    (0x6F5654, (0xC705, false)), // Korean hangul
    (0x6F5655, (0xC708, false)), // Korean hangul
    (0x225656, (0x72BC, false)), // East Asian ideograph
    (0x695657, (0x5D90, false)), // East Asian ideograph
    (0x27605E, (0x522E, false)), // East Asian ideograph
    (0x6F4D30, (0xB367, false)), // Korean hangul
    (0x225658, (0x72C3, false)), // East Asian ideograph
    (0x217747, (0x5853, false)), // East Asian ideograph
    (0x6F5659, (0xC719, false)), // Korean hangul
    (0x21565A, (0x86CB, false)), // East Asian ideograph
    (0x293537, (0x8C20, false)), // East Asian ideograph
    (0x235063, (0x98E5, false)), // East Asian ideograph
    (0x6F5174, (0xBE55, false)), // Korean hangul
    (0x27605F, (0x98D3, false)), // East Asian ideograph
    (0x6F4D31, (0xB368, false)), // Korean hangul
    (0x23565D, (0x9B74, false)), // East Asian ideograph
    (0x6F4B23, (0xAFB9, false)), // Korean hangul
    (0x4B306C, (0x96E0, false)), // East Asian ideograph
    (0x6F565E, (0xC730, false)), // Korean hangul
    (0x6F5638, (0xC6A7, false)), // Korean hangul
    (0x6F565F, (0xC735, false)), // Korean hangul
    (0x4C6074, (0x76B9, false)), // East Asian ideograph
    (0x224C65, (0x6F2D, false)), // East Asian ideograph
    (0x215660, (0x86DF, false)), // East Asian ideograph
    (0x4B5052, (0x7C56, false)), // East Asian ideograph
    (0x6F5175, (0xBE57, false)), // Korean hangul
    (0x6F4D32, (0xB369, false)), // Korean hangul
    (0x213B64, (0x5C6F, false)), // East Asian ideograph
    (0x6F5662, (0xC73D, false)), // Korean hangul
    (0x396223, (0x9BFD, false)), // East Asian ideograph (variant of 216223)
    (0x333F58, (0x61F4, false)), // East Asian ideograph
    (0x235663, (0x9B68, false)), // East Asian ideograph
    (0x2D3428, (0x5226, false)), // East Asian ideograph
    (0x2E5A40, (0x73B3, false)), // East Asian ideograph
    (0x215664, (0x86DB, false)), // East Asian ideograph
    (0x6F583B, (0xC9E2, false)), // Korean hangul
    (0x293539, (0x8C33, false)), // East Asian ideograph
    (0x215665, (0x86E4, false)), // East Asian ideograph
    (0x4B5F50, (0x96E3, false)), // East Asian ideograph
    (0x4B3B37, (0x51A9, false)), // East Asian ideograph
    (0x6F5176, (0xBE59, false)), // Korean hangul
    (0x2F2F5D, (0x7E48, false)), // East Asian ideograph
    (0x276061, (0x98D5, false)), // East Asian ideograph
    (0x286460, (0x7856, false)), // East Asian ideograph
    (0x21375B, (0x56B6, false)), // East Asian ideograph
    (0x215667, (0x86F9, false)), // East Asian ideograph
    (0x225930, (0x73DB, false)), // East Asian ideograph
    (0x6F5668, (0xC751, false)), // Korean hangul
    (0x22403D, (0x6A12, false)), // East Asian ideograph
    (0x6F5669, (0xC758, false)), // Korean hangul
    (0x224C67, (0x6F34, false)), // East Asian ideograph
    (0x4B566A, (0x8708, false)), // East Asian ideograph (variant of 21566A which maps to 8708)
    (0x21566B, (0x8700, false)), // East Asian ideograph
    (0x276062, (0x98D8, false)), // East Asian ideograph
    (0x285E7A, (0x75D6, false)), // East Asian ideograph
    (0x6F4D34, (0xB36B, false)), // Korean hangul
    (0x22566C, (0x72CC, false)), // East Asian ideograph
    (0x294647, (0x954F, false)), // East Asian ideograph
    (0x6F566D, (0xC77C, false)), // Korean hangul
    (0x334730, (0x6E5F, false)), // East Asian ideograph
    (0x6F5041, (0xBABB, false)), // Korean hangul
    (0x22566E, (0x72DB, false)), // East Asian ideograph
    (0x22566F, (0x72CD, false)), // East Asian ideograph
    (0x21356D, (0x5496, false)), // East Asian ideograph
    (0x276063, (0x98DE, false)), // East Asian ideograph
    (0x6F4D35, (0xB36E, false)), // Korean hangul
    (0x21375D, (0x56C1, false)), // East Asian ideograph
    (0x225D75, (0x7571, false)), // East Asian ideograph
    (0x333F5B, (0x6133, false)), // East Asian ideograph
    (0x235672, (0x9B80, false)), // East Asian ideograph
    (0x235673, (0x9B8C, false)), // East Asian ideograph
    (0x2F5E42, (0x9EC9, false)), // East Asian ideograph
    (0x6F5674, (0xC789, false)), // Korean hangul
    (0x6F543C, (0xC318, false)), // Korean hangul
    (0x2D5675, (0x9F05, false)), // East Asian ideograph
    (0x6F4D36, (0xB370, false)), // Korean hangul
    (0x21375E, (0x56C2, false)), // East Asian ideograph
    (0x275676, (0x8680, false)), // East Asian ideograph
    (0x692430, (0x3050, false)), // Hiragana letter GU
    (0x4D386F, (0x544B, false)), // East Asian ideograph
    (0x2D4122, (0x6485, false)), // East Asian ideograph
    (0x224123, (0x69F0, false)), // East Asian ideograph
    (0x295756, (0x9CA9, false)), // East Asian ideograph
    (0x215679, (0x8774, false)), // East Asian ideograph
    (0x224124, (0x69F2, false)), // East Asian ideograph
    (0x21567A, (0x8766, false)), // East Asian ideograph
    (0x234125, (0x9193, false)), // East Asian ideograph
    (0x6F4D37, (0xB371, false)), // Korean hangul
    (0x23567B, (0x9B7D, false)), // East Asian ideograph
    (0x21774E, (0x5856, false)), // East Asian ideograph
    (0x4B3072, (0x4EED, false)), // East Asian ideograph
    (0x6F4A4A, (0xAE60, false)), // Korean hangul
    (0x33567C, (0x8671, false)), // East Asian ideograph
    (0x6F567D, (0xC798, false)), // Korean hangul
    (0x224128, (0x6A14, false)), // East Asian ideograph
    (0x21567E, (0x8757, false)), // East Asian ideograph
    (0x224129, (0x6A63, false)), // East Asian ideograph
    (0x295030, (0x989E, false)), // East Asian ideograph
    (0x6F517B, (0xBE60, false)), // Korean hangul
    (0x4B412A, (0x6323, false)), // East Asian ideograph
    (0x6F4D38, (0xB374, false)), // Korean hangul
    (0x225742, (0x731E, false)), // East Asian ideograph
    (0x23412B, (0x919D, false)), // East Asian ideograph
    (0x212B33, (0x3002, false)), // Ideographic full stop
    (0x23412C, (0x919A, false)), // East Asian ideograph
    (0x2D342E, (0x8274, false)), // East Asian ideograph
    (0x6F5538, (0xC580, false)), // Korean hangul
    (0x276067, (0x9968, false)), // East Asian ideograph
    (0x6F4D39, (0xB378, false)), // Korean hangul
    (0x234130, (0x91A2, false)), // East Asian ideograph
    (0x3F476F, (0x51C8, false)), // East Asian ideograph
    (0x334131, (0x6425, false)), // East Asian ideograph
    (0x6F5042, (0xBABD, false)), // Korean hangul
    (0x2D4132, (0x642F, false)), // East Asian ideograph
    (0x287130, (0x7EDB, false)), // East Asian ideograph
    (0x234C29, (0x9708, false)), // East Asian ideograph
    (0x6F517D, (0xBE64, false)), // Korean hangul
    (0x234134, (0x919B, false)), // East Asian ideograph (variant of 4D4134 which maps to 919B)
    (0x6F4D3A, (0xB380, false)), // Korean hangul
    (0x213762, (0x56C8, false)), // East Asian ideograph
    (0x336179, (0x9C7B, false)), // East Asian ideograph
    (0x4C3474, (0x631D, false)), // East Asian ideograph
    (0x235D36, (0x9E15, false)), // East Asian ideograph
    (0x274136, (0x62A1, false)), // East Asian ideograph
    (0x274F5C, (0x6D3C, false)), // East Asian ideograph
    (0x6F4F68, (0xB9CC, false)), // Korean hangul
    (0x224137, (0x6A67, false)), // East Asian ideograph
    (0x344138, (0x8022, false)), // East Asian ideograph
    (0x6F517E, (0xBE68, false)), // Korean hangul
    (0x224139, (0x6A43, false)), // East Asian ideograph
    (0x6F4D3B, (0xB383, false)), // Korean hangul
    (0x22413A, (0x6A33, false)), // East Asian ideograph
    (0x225938, (0x73E3, false)), // East Asian ideograph
    (0x22413B, (0x6A32, false)), // East Asian ideograph
    (0x274F5D, (0x7A9D, false)), // East Asian ideograph
    (0x27413C, (0x62E3, false)), // East Asian ideograph
    (0x706247, (0x9987, false)), // East Asian ideograph
    (0x23413D, (0x91AA, false)), // East Asian ideograph
    (0x27606A, (0x996E, false)), // East Asian ideograph
    (0x6F4D3C, (0xB385, false)), // Korean hangul
    (0x213B66, (0x5C79, false)), // East Asian ideograph
    (0x213764, (0x56D1, false)), // East Asian ideograph
    (0x22413F, (0x6A28, false)), // East Asian ideograph
    (0x213321, (0x5167, false)), // East Asian ideograph
    (0x4C3F68, (0x69C7, false)), // East Asian ideograph
    (0x224140, (0x6A48, false)), // East Asian ideograph
    (0x224141, (0x6A50, false)), // East Asian ideograph
    (0x224142, (0x6A52, false)), // East Asian ideograph
    (0x334C2C, (0x754D, false)), // East Asian ideograph
    (0x336058, (0x9855, false)), // East Asian ideograph
    (0x224143, (0x6A72, false)), // East Asian ideograph
    (0x6F4D3D, (0xB38C, false)), // Korean hangul
    (0x274570, (0x6743, false)), // East Asian ideograph
    (0x285836, (0x7315, false)), // East Asian ideograph
    (0x224145, (0x6A3E, false)), // East Asian ideograph
    (0x224146, (0x6A77, false)), // East Asian ideograph
    (0x287134, (0x7ED7, false)), // East Asian ideograph
    (0x224147, (0x6A5B, false)), // East Asian ideograph
    (0x214148, (0x63EA, false)), // East Asian ideograph
    (0x6F4D3E, (0xB3C4, false)), // Korean hangul
    (0x225D7E, (0x757F, false)), // East Asian ideograph
    (0x217755, (0x589A, false)), // East Asian ideograph
    (0x2D3877, (0x5900, false)), // East Asian ideograph
    (0x22414A, (0x6A5E, false)), // East Asian ideograph
    (0x21414B, (0x643E, false)), // East Asian ideograph
    (0x21414C, (0x6413, false)), // East Asian ideograph
    (0x23414D, (0x91B5, false)), // East Asian ideograph
    (0x3F4A60, (0x7266, false)), // East Asian ideograph
    (0x6F4D3F, (0xB3C5, false)), // Korean hangul
    (0x6F4C78, (0xB2F7, false)), // Korean hangul
    (0x335D3B, (0x57DC, false)), // East Asian ideograph
    (0x22414F, (0x6A51, false)), // East Asian ideograph
    (0x2D4150, (0x6428, false)), // East Asian ideograph
    (0x29575F, (0x9CA0, false)), // East Asian ideograph
    (0x224151, (0x6A56, false)), // East Asian ideograph
    (0x2D4152, (0x6447, false)), // East Asian ideograph
    (0x6F4D40, (0xB3C8, false)), // Korean hangul
    (0x224153, (0x6A36, false)), // East Asian ideograph
    (0x2D4154, (0x635C, false)), // East Asian ideograph
    (0x2D3436, (0x52F3, false)), // East Asian ideograph
    (0x274155, (0x62A2, false)), // East Asian ideograph
    (0x217C30, (0x5A93, false)), // East Asian ideograph
    (0x224156, (0x6A7A, false)), // East Asian ideograph
    (0x234157, (0x91BD, false)), // East Asian ideograph
    (0x6F4D41, (0xB3CB, false)), // Korean hangul
    (0x213769, (0x56E4, false)), // East Asian ideograph
    (0x224158, (0x6A3F, false)), // East Asian ideograph
    (0x4B4F7B, (0x7B7A, false)), // East Asian ideograph
    (0x21623B, (0x9D12, false)), // East Asian ideograph (variant of 4B623B which maps to 9D12)
    (0x4B523E, (0x7F9A, false)), // East Asian ideograph (variant of 21523E which maps to 7F9A)
    (0x23415A, (0x91C2, false)), // East Asian ideograph
    (0x293651, (0x8D36, false)), // East Asian ideograph
    (0x23415B, (0x91C4, false)), // East Asian ideograph
    (0x33347D, (0x53C1, false)), // East Asian ideograph
    (0x23415C, (0x91C3, false)), // East Asian ideograph
    (0x275A30, (0x8D24, false)), // East Asian ideograph
    (0x29415D, (0x917D, false)), // East Asian ideograph
    (0x29586A, (0x9CCB, false)), // East Asian ideograph
    (0x274F64, (0x7A83, false)), // East Asian ideograph
    (0x27415F, (0x6402, false)), // East Asian ideograph
    (0x70624E, (0x9995, false)), // East Asian ideograph
    (0x224C76, (0x6EFA, false)), // East Asian ideograph
    (0x224833, (0x6CB4, false)), // East Asian ideograph
    (0x4B5164, (0x770C, false)), // East Asian ideograph
    (0x4C4146, (0x8538, false)), // East Asian ideograph
    (0x234161, (0x91D4, false)), // East Asian ideograph
    (0x284257, (0x68BC, false)), // East Asian ideograph
    (0x294656, (0x955B, false)), // East Asian ideograph
    (0x234162, (0x91D3, false)), // East Asian ideograph
    (0x234163, (0x91D5, false)), // East Asian ideograph
    (0x217639, (0x580E, false)), // East Asian ideograph
    (0x234164, (0x91D9, false)), // East Asian ideograph
    (0x274B22, (0x72EF, false)), // East Asian ideograph
    (0x6F5B59, (0xD2B8, false)), // Korean hangul
    (0x274165, (0x635E, false)), // East Asian ideograph
    (0x286272, (0x770D, false)), // East Asian ideograph
    (0x274166, (0x62E8, false)), // East Asian ideograph
    (0x6F4D44, (0xB3D4, false)), // Korean hangul
    (0x234168, (0x91E2, false)), // East Asian ideograph
    (0x6F534A, (0xC1A9, false)), // Korean hangul
    (0x234169, (0x91ED, false)), // East Asian ideograph
    (0x23416A, (0x91F7, false)), // East Asian ideograph
    (0x333D2F, (0x5E81, false)), // East Asian ideograph
    (0x23416B, (0x91FA, false)), // East Asian ideograph
    (0x6F4D45, (0xB3D5, false)), // Korean hangul
    (0x21775C, (0x5889, false)), // East Asian ideograph
    (0x22416C, (0x69F9, false)), // East Asian ideograph
    (0x215543, (0x83CC, false)), // East Asian ideograph
    (0x4B4E56, (0x78FA, false)), // East Asian ideograph
    (0x22416D, (0x6A64, false)), // East Asian ideograph
    (0x295427, (0x9A85, false)), // East Asian ideograph
    (0x27416E, (0x6251, false)), // East Asian ideograph
    (0x217C31, (0x5AAC, false)), // East Asian ideograph
    (0x23416F, (0x91F2, false)), // East Asian ideograph
    (0x234171, (0x91E8, false)), // East Asian ideograph
    (0x294458, (0x952C, false)), // East Asian ideograph
    (0x4B434D, (0x663F, false)), // East Asian ideograph
    (0x234172, (0x91F6, false)), // East Asian ideograph
    (0x2D343C, (0x52A2, false)), // East Asian ideograph
    (0x334C3E, (0x8E08, false)), // East Asian ideograph
    (0x234173, (0x91EE, false)), // East Asian ideograph
    (0x274174, (0x62E5, false)), // East Asian ideograph
    (0x4B3D4B, (0x5F3E, false)), // East Asian ideograph
    (0x334C36, (0x753B, false)), // East Asian ideograph (variant of 274C36 which maps to 753B)
    (0x4B4C67, (0x761F, false)), // East Asian ideograph (variant of 214C67 which maps to 761F)
    (0x224175, (0x6AA8, false)), // East Asian ideograph
    (0x29465A, (0x955F, false)), // East Asian ideograph
    (0x274176, (0x51FB, false)), // East Asian ideograph
    (0x692441, (0x3061, false)), // Hiragana letter TI
    (0x21332C, (0x5178, false)), // East Asian ideograph
    (0x235D43, (0x9E7B, false)), // East Asian ideograph
    (0x224177, (0x6AA5, false)), // East Asian ideograph
    (0x2D343D, (0x52E7, false)), // East Asian ideograph
    (0x224179, (0x6A96, false)), // East Asian ideograph
    (0x4B3D4C, (0x5F25, false)), // East Asian ideograph (variant of 273D4C)
    (0x222C24, (0x608A, false)), // East Asian ideograph
    (0x27417A, (0x6321, false)), // East Asian ideograph
    (0x6F4D48, (0xB3DB, false)), // Korean hangul
    (0x707360, (0x7B7B, false)), // East Asian ideograph
    (0x286032, (0x75AC, false)), // East Asian ideograph
    (0x21332D, (0x517C, false)), // East Asian ideograph
    (0x27417C, (0x636E, false)), // East Asian ideograph
    (0x216C27, (0x5296, false)), // East Asian ideograph
    (0x27417D, (0x63B3, false)), // East Asian ideograph
    (0x284F26, (0x6CF7, false)), // East Asian ideograph
    (0x224A44, (0x6E12, false)), // East Asian ideograph
    (0x22417E, (0x6A7D, false)), // East Asian ideograph
    (0x6F5D6A, (0xD750, false)), // Korean hangul
    (0x226C29, (0x7B73, false)), // East Asian ideograph
    (0x2E2B74, (0x609B, false)), // East Asian ideograph
    (0x276077, (0x997F, false)), // East Asian ideograph
    (0x6F4D49, (0xB3FC, false)), // Korean hangul
    (0x217760, (0x589B, false)), // East Asian ideograph
    (0x223378, (0x647D, false)), // East Asian ideograph
    (0x232C2C, (0x87EE, false)), // East Asian ideograph
    (0x274B28, (0x72DE, false)), // East Asian ideograph
    (0x222C2F, (0x609E, false)), // East Asian ideograph
    (0x6F4B28, (0xAFC9, false)), // Korean hangul
    (0x217761, (0x587C, false)), // East Asian ideograph
    (0x222C30, (0x6083, false)), // East Asian ideograph
    (0x4B4E5B, (0x783F, false)), // East Asian ideograph
    (0x22483B, (0x6D28, false)), // East Asian ideograph
    (0x216C33, (0x52AE, false)), // East Asian ideograph
    (0x4C4345, (0x67A6, false)), // East Asian ideograph
    (0x222C34, (0x60A7, false)), // East Asian ideograph
    (0x6F4D4B, (0xB410, false)), // Korean hangul
    (0x213773, (0x5718, false)), // East Asian ideograph
    (0x233B2E, (0x8EC9, false)), // East Asian ideograph
    (0x216C38, (0x52BC, false)), // East Asian ideograph
    (0x2D454E, (0x697D, false)), // East Asian ideograph
    (0x217763, (0x5888, false)), // East Asian ideograph
    (0x692446, (0x3066, false)), // Hiragana letter TE
    (0x232C3A, (0x87D6, false)), // East Asian ideograph
    (0x235D48, (0x9E83, false)), // East Asian ideograph
    (0x39302D, (0x534B, false)), // East Asian ideograph
    (0x6F4D4D, (0xB41C, false)), // Korean hangul
    (0x213775, (0x571F, false)), // East Asian ideograph
    (0x286037, (0x763F, false)), // East Asian ideograph
    (0x692447, (0x3067, false)), // Hiragana letter DE
    (0x223731, (0x65C6, false)), // East Asian ideograph
    (0x294478, (0x9522, false)), // East Asian ideograph
    (0x274B2C, (0x730E, false)), // East Asian ideograph
    (0x287144, (0x7EE0, false)), // East Asian ideograph
    (0x234C3D, (0x971D, false)), // East Asian ideograph
    (0x706C43, (0x70C0, false)), // East Asian ideograph
    (0x213333, (0x5191, false)), // East Asian ideograph
    (0x333066, (0x5FC8, false)), // East Asian ideograph
    (0x69613A, (0x7549, false)), // East Asian ideograph
    (0x6F4F6C, (0xB9D1, false)), // Korean hangul
    (0x216C46, (0x52D4, false)), // East Asian ideograph
    (0x234C3E, (0x9719, false)), // East Asian ideograph
    (0x453666, (0x5AD0, false)), // East Asian ideograph
    (0x232C48, (0x87D3, false)), // East Asian ideograph
    (0x6F4D4F, (0xB428, false)), // Korean hangul
    (0x6F4B29, (0xAFCB, false)), // Korean hangul
    (0x294662, (0x956A, false)), // East Asian ideograph
    (0x692449, (0x3069, false)), // Hiragana letter DO
    (0x275F50, (0x96BE, false)), // East Asian ideograph
    (0x235D4B, (0x9E88, false)), // East Asian ideograph
    (0x274B2E, (0x732E, false)), // East Asian ideograph
    (0x235A6B, (0x9D3D, false)), // East Asian ideograph
    (0x292C4C, (0x866E, false)), // East Asian ideograph
    (0x4B346B, (0x5DF5, false)), // East Asian ideograph
    (0x69595E, (0x63B5, false)), // East Asian ideograph
    (0x472C4D, (0x8801, false)), // East Asian ideograph (variant of 232C4D which maps to 8801)
    (0x28603A, (0x75C8, false)), // East Asian ideograph
    (0x454774, (0x6E15, false)), // East Asian ideograph
    (0x6F5564, (0xC5F7, false)), // Korean hangul
    (0x694664, (0x51EA, false)), // East Asian ideograph
    (0x294221, (0x9495, false)), // East Asian ideograph
    (0x6F4975, (0xAD7C, false)), // Korean hangul
    (0x292C55, (0x86CF, false)), // East Asian ideograph
    (0x6F5762, (0xC8C8, false)), // Korean hangul
    (0x4B5F6F, (0x970A, false)), // East Asian ideograph
    (0x2D4F37, (0x7980, false)), // East Asian ideograph
    (0x216C58, (0x52F0, false)), // East Asian ideograph
    (0x294222, (0x9490, false)), // East Asian ideograph
    (0x6F5A48, (0xCF78, false)), // Korean hangul
    (0x216C5A, (0x52F1, false)), // East Asian ideograph
    (0x2D632B, (0x5C28, false)), // East Asian ideograph
    (0x4B4135, (0x6368, false)), // East Asian ideograph
    (0x275C3E, (0x8FC7, false)), // East Asian ideograph
    (0x223B7A, (0x67F6, false)), // East Asian ideograph
    (0x69244D, (0x306D, false)), // Hiragana letter NE
    (0x222C5D, (0x60C4, false)), // East Asian ideograph
    (0x3A284C, (0x53A9, false)), // East Asian ideograph (variant of 4C284C)
    (0x294223, (0x94AD, false)), // East Asian ideograph
    (0x235D4F, (0x9E87, false)), // East Asian ideograph
    (0x3F5E60, (0x9586, false)), // East Asian ideograph
    (0x395773, (0x7DAF, false)), // East Asian ideograph
    (0x4B5F71, (0x9756, false)), // East Asian ideograph
    (0x224844, (0x6D39, false)), // East Asian ideograph
    (0x292C61, (0x86F4, false)), // East Asian ideograph
    (0x6F4D54, (0xB451, false)), // Korean hangul
    (0x6F4B2A, (0xAFCD, false)), // Korean hangul
    (0x69544B, (0x5870, false)), // East Asian ideograph
    (0x213339, (0x51A5, false)), // East Asian ideograph
    (0x294224, (0x94AA, false)), // East Asian ideograph
    (0x6F563F, (0xC6B9, false)), // Korean hangul
    (0x292C64, (0x877E, false)), // East Asian ideograph
    (0x4B5F72, (0x975B, false)), // East Asian ideograph
    (0x4B5365, (0x8133, false)), // East Asian ideograph
    (0x222C66, (0x60E2, false)), // East Asian ideograph
    (0x6F4A50, (0xAE6C, false)), // Korean hangul
    (0x294225, (0x94AB, false)), // East Asian ideograph
    (0x2D5664, (0x9F04, false)), // East Asian ideograph
    (0x6F542A, (0xC2ED, false)), // Korean hangul
    (0x4B5F73, (0x975C, false)), // East Asian ideograph (variant of 215F73 which maps to 975C)
    (0x335228, (0x94B5, false)), // East Asian ideograph
    (0x336C6B, (0x6031, false)), // East Asian ideograph
    (0x6F4D56, (0xB458, false)), // Korean hangul
    (0x692450, (0x3070, false)), // Hiragana letter BA
    (0x4B4E67, (0x79D8, false)), // East Asian ideograph
    (0x6F5C37, (0xD3BC, false)), // Korean hangul
    (0x28714D, (0x7EE1, false)), // East Asian ideograph
    (0x216C6F, (0x530B, false)), // East Asian ideograph
    (0x6F4D57, (0xB460, false)), // Korean hangul
    (0x222C73, (0x6103, false)), // East Asian ideograph
    (0x6F5A21, (0xCEA5, false)), // Korean hangul
    (0x4B3D5C, (0x5F83, false)), // East Asian ideograph
    (0x6F4D58, (0xB461, false)), // Korean hangul
    (0x6F5425, (0xC2E0, false)), // Korean hangul
    (0x692452, (0x3072, false)), // Hiragana letter HI
    (0x21333D, (0x51B6, false)), // East Asian ideograph
    (0x215721, (0x8759, false)), // East Asian ideograph
    (0x6F4F6E, (0xB9D9, false)), // Korean hangul
    (0x6F5723, (0xC7A4, false)), // Korean hangul
    (0x225724, (0x72F4, false)), // East Asian ideograph
    (0x6F5D74, (0xD769, false)), // Korean hangul
    (0x6F4D59, (0xB463, false)), // Korean hangul
    (0x215725, (0x879E, false)), // East Asian ideograph
    (0x695438, (0x57B3, false)), // East Asian ideograph
    (0x692453, (0x3073, false)), // Hiragana letter BI
    (0x6F5726, (0xC7A7, false)), // Korean hangul
    (0x6F5727, (0xC7AC, false)), // Korean hangul
    (0x2F5E66, (0x9B12, false)), // East Asian ideograph
    (0x4B4B71, (0x7F3E, false)), // East Asian ideograph (variant of 2D4B71 which maps to 7F3E)
    (0x6F5728, (0xC7AD, false)), // Korean hangul
    (0x225729, (0x7302, false)), // East Asian ideograph
    (0x697323, (0x9D64, false)), // East Asian ideograph
    (0x6F4D5A, (0xB465, false)), // Korean hangul
    (0x6F572A, (0xC7B4, false)), // Korean hangul
    (0x2E4873, (0x6FA3, false)), // East Asian ideograph
    (0x21572B, (0x87B3, false)), // East Asian ideograph
    (0x333330, (0x518A, false)), // East Asian ideograph
    (0x21572C, (0x87BB, false)), // East Asian ideograph
    (0x29577A, (0x9CAD, false)), // East Asian ideograph
    (0x21572D, (0x87C8, false)), // East Asian ideograph
    (0x39563C, (0x56CC, false)), // East Asian ideograph
    (0x222632, (0x5DB8, false)), // East Asian ideograph
    (0x21572E, (0x87D2, false)), // East Asian ideograph
    (0x4B5D58, (0x9234, false)), // East Asian ideograph
    (0x6F4D5B, (0xB46C, false)), // Korean hangul
    (0x21572F, (0x87BA, false)), // East Asian ideograph
    (0x2D5730, (0x87C7, false)), // East Asian ideograph
    (0x235731, (0x9B92, false)), // East Asian ideograph
    (0x6F5C38, (0xD3C4, false)), // Korean hangul
    (0x275732, (0x86F2, false)), // East Asian ideograph
    (0x6F4E75, (0xB7AC, false)), // Korean hangul
    (0x275733, (0x866B, false)), // East Asian ideograph
    (0x6F4D5C, (0xB480, false)), // Korean hangul
    (0x275734, (0x8749, false)), // East Asian ideograph
    (0x215735, (0x87FB, false)), // East Asian ideograph
    (0x215736, (0x8805, false)), // East Asian ideograph
    (0x2D5228, (0x9262, false)), // East Asian ideograph
    (0x29577C, (0x9CB0, false)), // East Asian ideograph
    (0x695737, (0x5F16, false)), // East Asian ideograph
    (0x222634, (0x5DBF, false)), // East Asian ideograph
    (0x213D21, (0x5EBE, false)), // East Asian ideograph
    (0x6F4D5D, (0xB488, false)), // Korean hangul
    (0x235739, (0x9B9D, false)), // East Asian ideograph
    (0x6F573A, (0xC811, false)), // Korean hangul
    (0x2D3453, (0x758B, false)), // East Asian ideograph
    (0x21573B, (0x8822, false)), // East Asian ideograph
    (0x213132, (0x4F5B, false)), // East Asian ideograph
    (0x21573C, (0x8823, false)), // East Asian ideograph
    (0x21573D, (0x8821, false)), // East Asian ideograph
    (0x23417A, (0x91F8, false)), // East Asian ideograph
    (0x6F4D5E, (0xB4A4, false)), // Korean hangul
    (0x21573E, (0x881F, false)), // East Asian ideograph
    (0x275E69, (0x5173, false)), // East Asian ideograph
    (0x6F5B7A, (0xD330, false)), // Korean hangul
    (0x692458, (0x3078, false)), // Hiragana letter HE
    (0x21573F, (0x8831, false)), // East Asian ideograph
    (0x235D5A, (0x9E95, false)), // East Asian ideograph
    (0x4B5740, (0x8827, false)), // East Asian ideograph
    (0x2D572D, (0x8748, false)), // East Asian ideograph
    (0x215741, (0x8836, false)), // East Asian ideograph
    (0x69533B, (0x555D, false)), // East Asian ideograph
    (0x275742, (0x86EE, false)), // East Asian ideograph
    (0x27614F, (0x9A74, false)), // East Asian ideograph
    (0x6F4D5F, (0xB4B7, false)), // Korean hangul
    (0x215743, (0x8840, false)), // East Asian ideograph
    (0x4C5C3A, (0x73F1, false)), // East Asian ideograph
    (0x6F5744, (0xC82D, false)), // Korean hangul
    (0x694823, (0x7872, false)), // East Asian ideograph
    (0x6F5745, (0xC82F, false)), // Korean hangul
    (0x2D522B, (0x9475, false)), // East Asian ideograph
    (0x215746, (0x8853, false)), // East Asian ideograph (variant of 4B5746 which maps to 8853)
    (0x275747, (0x4E8D, false)), // East Asian ideograph
    (0x2D4562, (0x681D, false)), // East Asian ideograph
    (0x275A36, (0x8D56, false)), // East Asian ideograph
    (0x6F4D60, (0xB4C0, false)), // Korean hangul
    (0x69245A, (0x307A, false)), // Hiragana letter PE
    (0x213345, (0x51DD, false)), // East Asian ideograph
    (0x215749, (0x885B, false)), // East Asian ideograph
    (0x235D5C, (0x9E91, false)), // East Asian ideograph
    (0x2D3164, (0x7AE2, false)), // East Asian ideograph
    (0x4B4A2E, (0x55B6, false)), // East Asian ideograph
    (0x21574A, (0x885D, false)), // East Asian ideograph
    (0x474236, (0x949A, false)), // East Asian ideograph
    (0x2D4425, (0x686E, false)), // East Asian ideograph
    (0x29533D, (0x9A90, false)), // East Asian ideograph
    (0x21574C, (0x8862, false)), // East Asian ideograph
    (0x3B3922, (0x8DB5, false)), // East Asian ideograph
    (0x21574D, (0x8863, false)), // East Asian ideograph
    (0x23574E, (0x9BA0, false)), // East Asian ideograph
    (0x2D3457, (0x62FE, false)), // East Asian ideograph
    (0x21574F, (0x8868, false)), // East Asian ideograph
    (0x6F5750, (0xC885, false)), // Korean hangul
    (0x213D22, (0x5ECA, false)), // East Asian ideograph
    (0x2D4564, (0x68B9, false)), // East Asian ideograph
    (0x215752, (0x8881, false)), // East Asian ideograph
    (0x27602E, (0x97E9, false)), // East Asian ideograph
    (0x6F5753, (0xC88B, false)), // Korean hangul
    (0x69613E, (0x7569, false)), // East Asian ideograph
    (0x6F5754, (0xC88C, false)), // Korean hangul
    (0x215755, (0x8888, false)), // East Asian ideograph
    (0x4B3D67, (0x5F84, false)), // East Asian ideograph (variant of 273D67)
    (0x215756, (0x88AB, false)), // East Asian ideograph
    (0x6F4D63, (0xB4DD, false)), // Korean hangul
    (0x29367E, (0x8D59, false)), // East Asian ideograph
    (0x217337, (0x568A, false)), // East Asian ideograph
    (0x215759, (0x888D, false)), // East Asian ideograph
    (0x6F5967, (0xCE60, false)), // Korean hangul
    (0x21575A, (0x888B, false)), // East Asian ideograph
    (0x21575B, (0x889E, false)), // East Asian ideograph
    (0x4D3C6C, (0x8FB6, false)), // East Asian ideograph
    (0x21575C, (0x88C1, false)), // East Asian ideograph
    (0x273A36, (0x5988, false)), // East Asian ideograph
    (0x6F4921, (0xAC70, false)), // Korean hangul
    (0x23575D, (0x9BC6, false)), // East Asian ideograph
    (0x23575E, (0x9BBF, false)), // East Asian ideograph
    (0x22575F, (0x733B, false)), // East Asian ideograph
    (0x2D5760, (0x5E2C, false)), // East Asian ideograph
    (0x6F4D65, (0xB4E3, false)), // Korean hangul
    (0x4C7265, (0x7DFC, false)), // East Asian ideograph
    (0x69245F, (0x307F, false)), // Hiragana letter MI
    (0x6F4922, (0xAC71, false)), // Korean hangul
    (0x225762, (0x733A, false)), // East Asian ideograph
    (0x6F5125, (0xBC45, false)), // Korean hangul
    (0x2E4670, (0x6CD0, false)), // East Asian ideograph
    (0x284539, (0x6B9A, false)), // East Asian ideograph
    (0x2D345B, (0x6607, false)), // East Asian ideograph
    (0x275763, (0x91CC, false)), // East Asian ideograph
    (0x393054, (0x4F0D, false)), // East Asian ideograph
    (0x226260, (0x777A, false)), // East Asian ideograph
    (0x6F5764, (0xC8D4, false)), // Korean hangul
    (0x232739, (0x85DA, false)), // East Asian ideograph
    (0x215765, (0x88DD, false)), // East Asian ideograph
    (0x395564, (0x6726, false)), // East Asian ideograph
    (0x235766, (0x9BB9, false)), // East Asian ideograph
    (0x6F4E2E, (0xB560, false)), // Korean hangul
    (0x6F5767, (0xC8E0, false)), // Korean hangul
    (0x6F504B, (0xBB3B, false)), // Korean hangul
    (0x6F2469, (0x3138, false)), // Korean hangul
    (0x215768, (0x88F3, false)), // East Asian ideograph
    (0x2D5232, (0x8FA0, false)), // East Asian ideograph
    (0x6F5769, (0xC8F0, false)), // Korean hangul
    (0x6F5B60, (0xD2CB, false)), // Korean hangul
    (0x69576A, (0x603A, false)), // East Asian ideograph
    (0x22576B, (0x7352, false)), // East Asian ideograph
    (0x21334C, (0x51F9, false)), // East Asian ideograph
    (0x27576C, (0x5236, false)), // East Asian ideograph
    (0x225521, (0x721D, false)), // East Asian ideograph
    (0x2D345D, (0x5349, false)), // East Asian ideograph
    (0x6F576D, (0xC8FD, false)), // Korean hangul
    (0x2D5233, (0x7F78, false)), // East Asian ideograph
    (0x23576E, (0x9BC0, false)), // East Asian ideograph
    (0x29312B, (0x89D1, false)), // East Asian ideograph
    (0x2D5361, (0x811A, false)), // East Asian ideograph
    (0x4B576F, (0x8910, false)), // East Asian ideograph (variant of 21576F which maps to 8910)
    (0x6F4D68, (0xB4ED, false)), // Korean hangul
    (0x6F4B2E, (0xAFE9, false)), // Korean hangul
    (0x6F4925, (0xAC77, false)), // Korean hangul
    (0x275771, (0x8934, false)), // East Asian ideograph
    (0x215772, (0x8912, false)), // East Asian ideograph
    (0x294164, (0x948B, false)), // East Asian ideograph
    (0x275773, (0x88E4, false)), // East Asian ideograph
    (0x215774, (0x892A, false)), // East Asian ideograph
    (0x6F4D69, (0xB4EF, false)), // Korean hangul
    (0x29467C, (0x9546, false)), // East Asian ideograph
    (0x3F4926, (0x6E08, false)), // East Asian ideograph
    (0x21334E, (0x51FD, false)), // East Asian ideograph
    (0x6F5776, (0xC92C, false)), // Korean hangul
    (0x234221, (0x91F9, false)), // East Asian ideograph
    (0x215777, (0x893B, false)), // East Asian ideograph
    (0x224222, (0x6A7F, false)), // East Asian ideograph
    (0x6F5A33, (0xCF11, false)), // Korean hangul
    (0x234223, (0x9204, false)), // East Asian ideograph
    (0x216231, (0x9CF6, false)), // East Asian ideograph
    (0x215779, (0x8938, false)), // East Asian ideograph
    (0x224224, (0x6A91, false)), // East Asian ideograph
    (0x21577A, (0x8944, false)), // East Asian ideograph
    (0x214225, (0x64E0, false)), // East Asian ideograph
    (0x6F4927, (0xAC79, false)), // Korean hangul
    (0x22577B, (0x7358, false)), // East Asian ideograph
    (0x224226, (0x6A9F, false)), // East Asian ideograph
    (0x4B4A38, (0x71D7, false)), // East Asian ideograph
    (0x21577C, (0x8960, false)), // East Asian ideograph
    (0x234227, (0x920A, false)), // East Asian ideograph
    (0x27577D, (0x8884, false)), // East Asian ideograph
    (0x234228, (0x9225, false)), // East Asian ideograph
    (0x295347, (0x9A93, false)), // East Asian ideograph
    (0x216232, (0x9CF4, false)), // East Asian ideograph
    (0x21577E, (0x8964, false)), // East Asian ideograph
    (0x274229, (0x6401, false)), // East Asian ideograph
    (0x22422A, (0x6A92, false)), // East Asian ideograph
    (0x692465, (0x3085, false)), // Hiragana letter small YU
    (0x22422B, (0x6AA3, false)), // East Asian ideograph
    (0x6F504C, (0xBB3C, false)), // Korean hangul
    (0x6F545B, (0xC42C, false)), // Korean hangul
    (0x23422C, (0x9228, false)), // East Asian ideograph
    (0x6F5A35, (0xCF15, false)), // Korean hangul
    (0x6F5B61, (0xD2D4, false)), // Korean hangul
    (0x29556C, (0x960B, false)), // East Asian ideograph
    (0x4B5B46, (0x8F0C, false)), // East Asian ideograph
    (0x21422E, (0x64FE, false)), // East Asian ideograph
    (0x456260, (0x5E7A, false)), // East Asian ideograph
    (0x275C57, (0x8FD8, false)), // East Asian ideograph
    (0x23422F, (0x9203, false)), // East Asian ideograph
    (0x276030, (0x827D, false)), // East Asian ideograph
    (0x692466, (0x3086, false)), // Hiragana letter YU
    (0x274230, (0x6446, false)), // East Asian ideograph
    (0x2D567B, (0x8717, false)), // East Asian ideograph
    (0x234231, (0x9200, false)), // East Asian ideograph
    (0x393573, (0x5611, false)), // East Asian ideograph
    (0x234232, (0x9218, false)), // East Asian ideograph
    (0x295C3E, (0x9E37, false)), // East Asian ideograph
    (0x274233, (0x62E6, false)), // East Asian ideograph
    (0x6F4D6D, (0xB518, false)), // Korean hangul
    (0x6F4B2F, (0xAFF0, false)), // Korean hangul
    (0x274234, (0x6400, false)), // East Asian ideograph
    (0x274235, (0x6444, false)), // East Asian ideograph
    (0x234236, (0x9208, false)), // East Asian ideograph
    (0x6F5A37, (0xCF20, false)), // Korean hangul
    (0x224237, (0x6A9B, false)), // East Asian ideograph
    (0x234238, (0x921C, false)), // East Asian ideograph
    (0x2D6079, (0x8218, false)), // East Asian ideograph
    (0x6F492B, (0xAC83, false)), // Korean hangul
    (0x213353, (0x5206, false)), // East Asian ideograph
    (0x27423A, (0x6405, false)), // East Asian ideograph
    (0x287349, (0x7F30, false)), // East Asian ideograph
    (0x2D3464, (0x613D, false)), // East Asian ideograph
    (0x23423B, (0x9224, false)), // East Asian ideograph
    (0x2D3021, (0x5F0C, false)), // East Asian ideograph
    (0x6F5A38, (0xCF24, false)), // Korean hangul
    (0x335772, (0x8943, false)), // East Asian ideograph
    (0x293132, (0x89CC, false)), // East Asian ideograph
    (0x226635, (0x7911, false)), // East Asian ideograph
    (0x33423D, (0x53CE, false)), // East Asian ideograph
    (0x284D2B, (0x6D54, false)), // East Asian ideograph
    (0x2D486B, (0x6F82, false)), // East Asian ideograph
    (0x692469, (0x3089, false)), // Hiragana letter RA
    (0x6F576B, (0xC8F5, false)), // Korean hangul
    (0x2D3C7C, (0x83F4, false)), // East Asian ideograph
    (0x4B516D, (0x7DCF, false)), // East Asian ideograph
    (0x216237, (0x9D23, false)), // East Asian ideograph
    (0x224242, (0x6AA0, false)), // East Asian ideograph
    (0x6F4D70, (0xB524, false)), // Korean hangul
    (0x234243, (0x9212, false)), // East Asian ideograph
    (0x69246A, (0x308A, false)), // Hiragana letter RI
    (0x334244, (0x6559, false)), // East Asian ideograph
    (0x4B4A3E, (0x7235, false)), // East Asian ideograph
    (0x2F5E7D, (0x6641, false)), // East Asian ideograph
    (0x393577, (0x9FA2, false)), // East Asian ideograph
    (0x6F5B62, (0xD1F8, false)), // Korean hangul
    (0x214247, (0x6557, false)), // East Asian ideograph
    (0x6F4D71, (0xB525, false)), // Korean hangul
    (0x6F4C7A, (0xB2FA, false)), // Korean hangul
    (0x234248, (0x91FF, false)), // East Asian ideograph
    (0x285323, (0x8367, false)), // East Asian ideograph
    (0x69246B, (0x308B, false)), // Hiragana letter RU
    (0x217345, (0x5699, false)), // East Asian ideograph
    (0x224249, (0x6A9E, false)), // East Asian ideograph
    (0x22424A, (0x6A87, false)), // East Asian ideograph
    (0x6F5A3B, (0xCF2F, false)), // Korean hangul
    (0x22424B, (0x6A8E, false)), // East Asian ideograph
    (0x23424E, (0x9206, false)), // East Asian ideograph
    (0x27424F, (0x542F, false)), // East Asian ideograph
    (0x6F5A3C, (0xCF30, false)), // Korean hangul
    (0x21623A, (0x9D1B, false)), // East Asian ideograph
    (0x224251, (0x6AAB, false)), // East Asian ideograph
    (0x6F4D73, (0xB528, false)), // Korean hangul
    (0x234252, (0x9249, false)), // East Asian ideograph
    (0x6F4930, (0xAC89, false)), // Korean hangul
    (0x394243, (0x4FF2, false)), // East Asian ideograph
    (0x705F61, (0x54DA, false)), // East Asian ideograph
    (0x234254, (0x924D, false)), // East Asian ideograph
    (0x6F556B, (0xC606, false)), // Korean hangul
    (0x224255, (0x6AC8, false)), // East Asian ideograph
    (0x224864, (0x6D19, false)), // East Asian ideograph
    (0x4C4177, (0x8223, false)), // East Asian ideograph
    (0x274256, (0x655B, false)), // East Asian ideograph
    (0x6F4D74, (0xB529, false)), // Korean hangul
    (0x224257, (0x6AAE, false)), // East Asian ideograph
    (0x6F4931, (0xAC8A, false)), // Korean hangul
    (0x6F497C, (0xAD90, false)), // Korean hangul
    (0x234258, (0x923A, false)), // East Asian ideograph
    (0x2D346A, (0x5918, false)), // East Asian ideograph
    (0x6F4879, (0xAC30, false)), // Korean hangul
    (0x2D3C7D, (0x53A2, false)), // East Asian ideograph
    (0x2D4F3E, (0x7A3E, false)), // East Asian ideograph
    (0x4D4134, (0x919B, false)), // East Asian ideograph
    (0x23425C, (0x922E, false)), // East Asian ideograph
    (0x6F4932, (0xAC8B, false)), // Korean hangul
    (0x22425D, (0x6ABF, false)), // East Asian ideograph
    (0x2D5241, (0x7FA3, false)), // East Asian ideograph
    (0x6F5A3F, (0xCF58, false)), // Korean hangul
    (0x23425F, (0x9233, false)), // East Asian ideograph
    (0x294260, (0x94B7, false)), // East Asian ideograph
    (0x234261, (0x9266, false)), // East Asian ideograph
    (0x6F4933, (0xAC8C, false)), // Korean hangul
    (0x214263, (0x65AC, false)), // East Asian ideograph
    (0x29446D, (0x951B, false)), // East Asian ideograph
    (0x6F5A40, (0xCF5C, false)), // Korean hangul
    (0x224264, (0x6ACA, false)), // East Asian ideograph
    (0x224867, (0x6D0E, false)), // East Asian ideograph
    (0x4B3938, (0x5942, false)), // East Asian ideograph
    (0x214266, (0x65B7, false)), // East Asian ideograph
    (0x6F4934, (0xAC90, false)), // Korean hangul
    (0x22375B, (0x65FB, false)), // East Asian ideograph
    (0x4B4A45, (0x5C13, false)), // East Asian ideograph
    (0x234268, (0x9235, false)), // East Asian ideograph
    (0x4B4B77, (0x4EC0, false)), // East Asian ideograph
    (0x6F5A41, (0xCF64, false)), // Korean hangul
    (0x4B5B52, (0x8F42, false)), // East Asian ideograph
    (0x6F4D78, (0xB530, false)), // Korean hangul
    (0x23426B, (0x9250, false)), // East Asian ideograph
    (0x692472, (0x3092, false)), // Hiragana letter WO
    (0x21335D, (0x5228, false)), // East Asian ideograph
    (0x22375C, (0x65FC, false)), // East Asian ideograph
    (0x23426C, (0x926B, false)), // East Asian ideograph
    (0x23426D, (0x9239, false)), // East Asian ideograph
    (0x6F556C, (0xC607, false)), // Korean hangul
    (0x6F5A42, (0xCF65, false)), // Korean hangul
    (0x6F4B6C, (0xB0E5, false)), // Korean hangul
    (0x23426F, (0x926D, false)), // East Asian ideograph
    (0x234270, (0x926C, false)), // East Asian ideograph
    (0x6F4936, (0xAC9C, false)), // Korean hangul
    (0x234271, (0x924F, false)), // East Asian ideograph
    (0x2D4272, (0x65E3, false)), // East Asian ideograph
    (0x6F5C3E, (0xD3ED, false)), // Korean hangul
    (0x2D3C7E, (0x53A0, false)), // East Asian ideograph
    (0x6F5A43, (0xCF67, false)), // Korean hangul
    (0x294274, (0x94BF, false)), // East Asian ideograph
    (0x6F4D7A, (0xB532, false)), // Korean hangul
    (0x6F4937, (0xAC9F, false)), // Korean hangul
    (0x234277, (0x9260, false)), // East Asian ideograph
    (0x2D302D, (0x4E17, false)), // East Asian ideograph
    (0x6F4D62, (0xB4DC, false)), // Korean hangul
    (0x6F5A44, (0xCF69, false)), // Korean hangul
    (0x234C6A, (0x9741, false)), // East Asian ideograph
    (0x4B5B55, (0x8EE2, false)), // East Asian ideograph
    (0x224279, (0x6AE6, false)), // East Asian ideograph
    (0x216D24, (0x531C, false)), // East Asian ideograph
    (0x6F4D7B, (0xB534, false)), // Korean hangul
    (0x69562E, (0x5CBB, false)), // East Asian ideograph
    (0x6F4938, (0xACA0, false)), // Korean hangul
    (0x6F5B3A, (0xD1A4, false)), // Korean hangul
    (0x275823, (0x88AD, false)), // East Asian ideograph
    (0x29424B, (0x94A3, false)), // East Asian ideograph
    (0x235D77, (0x9EAD, false)), // East Asian ideograph
    (0x6F4F75, (0xB9E5, false)), // Korean hangul
    (0x213D6A, (0x5F97, false)), // East Asian ideograph
    (0x2E3870, (0x714A, false)), // East Asian ideograph
    (0x6F5A45, (0xCF70, false)), // Korean hangul
    (0x22486C, (0x6D00, false)), // East Asian ideograph
    (0x234C6B, (0x9747, false)), // East Asian ideograph
    (0x23427E, (0x9236, false)), // East Asian ideograph
    (0x6F4D7C, (0xB537, false)), // Korean hangul
    (0x6F4B32, (0xB00C, false)), // Korean hangul
    (0x222D2A, (0x610A, false)), // East Asian ideograph
    (0x22442A, (0x6B35, false)), // East Asian ideograph
    (0x216D2E, (0x532D, false)), // East Asian ideograph
    (0x6F4D7D, (0xB538, false)), // Korean hangul
    (0x6F493A, (0xACA8, false)), // Korean hangul
    (0x213362, (0x5230, false)), // East Asian ideograph
    (0x276822, (0x507B, false)), // East Asian ideograph
    (0x2D3473, (0x5374, false)), // East Asian ideograph
    (0x6F5A47, (0xCF74, false)), // Korean hangul
    (0x4C3F7A, (0x6922, false)), // East Asian ideograph
    (0x29535A, (0x9A9F, false)), // East Asian ideograph
    (0x222D32, (0x6112, false)), // East Asian ideograph
    (0x4B5B58, (0x8EE3, false)), // East Asian ideograph
    (0x4B393F, (0x5333, false)), // East Asian ideograph
    (0x216D33, (0x5330, false)), // East Asian ideograph
    (0x2D486E, (0x6F97, false)), // East Asian ideograph (not in Unicode)
    (0x282D34, (0x607D, false)), // East Asian ideograph
    (0x6F497E, (0xADA4, false)), // Korean hangul
    (0x224E32, (0x6FCA, false)), // East Asian ideograph
    (0x6F5C3F, (0xD3F0, false)), // Korean hangul
    (0x2F5A48, (0x9D44, false)), // East Asian ideograph
    (0x22486F, (0x6D33, false)), // East Asian ideograph
    (0x4C5F58, (0x7640, false)), // East Asian ideograph
    (0x6F4E2F, (0xB561, false)), // Korean hangul
    (0x6F493C, (0xACAA, false)), // Korean hangul
    (0x39424F, (0x5554, false)), // East Asian ideograph
    (0x2D3032, (0x7ADD, false)), // East Asian ideograph
    (0x33392F, (0x9029, false)), // East Asian ideograph
    (0x284F5D, (0x6CA3, false)), // East Asian ideograph
    (0x22442D, (0x6B3B, false)), // East Asian ideograph
    (0x216D3E, (0x533D, false)), // East Asian ideograph
    (0x51513B, (0x7E9F, false)), // East Asian ideograph
    (0x222D3F, (0x6121, false)), // East Asian ideograph
    (0x6F4F76, (0xB9E8, false)), // Korean hangul
    (0x292768, (0x8572, false)), // East Asian ideograph
    (0x274B5F, (0x7410, false)), // East Asian ideograph
    (0x6F5A4A, (0xCF85, false)), // Korean hangul
    (0x232D41, (0x8841, false)), // East Asian ideograph
    (0x2D4277, (0x65EE, false)), // East Asian ideograph
    (0x293A6B, (0x8E7F, false)), // East Asian ideograph
    (0x232A57, (0x873E, false)), // East Asian ideograph
    (0x6F4B33, (0xB00D, false)), // Korean hangul
    (0x222D43, (0x6106, false)), // East Asian ideograph
    (0x294251, (0x94C8, false)), // East Asian ideograph
    (0x3B2D44, (0x8842, false)), // East Asian ideograph
    (0x295053, (0x98D9, false)), // East Asian ideograph
    (0x287178, (0x7EF6, false)), // East Asian ideograph
    (0x226D47, (0x7C00, false)), // East Asian ideograph
    (0x2D4141, (0x63B2, false)), // East Asian ideograph
    (0x6F493F, (0xACB0, false)), // Korean hangul
    (0x6F547E, (0xC548, false)), // Korean hangul
    (0x294252, (0x94C9, false)), // East Asian ideograph
    (0x296028, (0x9F86, false)), // East Asian ideograph
    (0x2F3833, (0x8D91, false)), // East Asian ideograph
    (0x216D4B, (0x535D, false)), // East Asian ideograph
    (0x6F4940, (0xACB8, false)), // Korean hangul
    (0x284F61, (0x6EE0, false)), // East Asian ideograph
    (0x6F5C40, (0xD3F4, false)), // Korean hangul
    (0x295360, (0x9A98, false)), // East Asian ideograph
    (0x4B5B5E, (0x5F01, false)), // East Asian ideograph
    (0x232D51, (0x884A, false)), // East Asian ideograph
    (0x6F4941, (0xACB9, false)), // Korean hangul
    (0x294254, (0x94CB, false)), // East Asian ideograph
    (0x286D54, (0x7B5A, false)), // East Asian ideograph
    (0x222D56, (0x53AF, false)), // East Asian ideograph
    (0x232D57, (0x8850, false)), // East Asian ideograph
    (0x21336A, (0x524C, false)), // East Asian ideograph
    (0x294255, (0x94CA, false)), // East Asian ideograph
    (0x29602B, (0x9F85, false)), // East Asian ideograph
    (0x6F4F77, (0xB9EC, false)), // Korean hangul
    (0x21313A, (0x4F38, false)), // East Asian ideograph
    (0x3F462B, (0x5E30, false)), // East Asian ideograph
    (0x274B64, (0x7391, false)), // East Asian ideograph
    (0x6F5A4F, (0xCFB0, false)), // Korean hangul
    (0x696D5A, (0x8F4C, false)), // East Asian ideograph
    (0x29327E, (0x8C07, false)), // East Asian ideograph
    (0x6F4B34, (0xB010, false)), // Korean hangul
    (0x6F4943, (0xACBC, false)), // Korean hangul
    (0x21336B, (0x524B, false)), // East Asian ideograph
    (0x4D4862, (0x9229, false)), // East Asian ideograph
    (0x222D5E, (0x6137, false)), // East Asian ideograph
    (0x28717D, (0x7EFA, false)), // East Asian ideograph
    (0x6F5A50, (0xCFC4, false)), // Korean hangul
    (0x6F4B75, (0xB113, false)), // Korean hangul
    (0x6F4A5A, (0xAEBD, false)), // Korean hangul
    (0x6F4944, (0xACBD, false)), // Korean hangul
    (0x21735B, (0x56C3, false)), // East Asian ideograph
    (0x225541, (0x723F, false)), // East Asian ideograph
    (0x226D63, (0x7C20, false)), // East Asian ideograph
    (0x2D4147, (0x6271, false)), // East Asian ideograph
    (0x216D66, (0x5393, false)), // East Asian ideograph
    (0x6F4945, (0xACC1, false)), // Korean hangul
    (0x21336D, (0x5247, false)), // East Asian ideograph
    (0x294258, (0x94B0, false)), // East Asian ideograph
    (0x6F4B22, (0xAFB8, false)), // Korean hangul
    (0x335941, (0x54D7, false)), // East Asian ideograph
    (0x274B67, (0x73AF, false)), // East Asian ideograph
    (0x234C78, (0x975D, false)), // East Asian ideograph
    (0x234835, (0x942B, false)), // East Asian ideograph
    (0x6F5052, (0xBB4F, false)), // Korean hangul
    (0x276D6D, (0x538D, false)), // East Asian ideograph
    (0x274B68, (0x7477, false)), // East Asian ideograph
    (0x6F5A53, (0xCFE4, false)), // Korean hangul
    (0x225235, (0x712F, false)), // East Asian ideograph
    (0x294F23, (0x9880, false)), // East Asian ideograph
    (0x4B546D, (0x82D3, false)), // East Asian ideograph (variant of 21546D which maps to 82D3)
    (0x6F4C7B, (0xB2FB, false)), // Korean hangul
    (0x6F4947, (0xACC4, false)), // Korean hangul
    (0x21336F, (0x5256, false)), // East Asian ideograph
    (0x225544, (0x7242, false)), // East Asian ideograph
    (0x6F5724, (0xC7A5, false)), // Korean hangul
    (0x274932, (0x6CFB, false)), // East Asian ideograph
    (0x4B682E, (0x4EC2, false)), // East Asian ideograph
    (0x274B69, (0x73BA, false)), // East Asian ideograph
    (0x6F5A54, (0xCFE8, false)), // Korean hangul
    (0x234837, (0x9441, false)), // East Asian ideograph
    (0x213D3F, (0x5F17, false)), // East Asian ideograph
    (0x4D2D75, (0x8872, false)), // East Asian ideograph (variant of 232D75 which maps to 8872)
    (0x213370, (0x525B, false)), // East Asian ideograph
    (0x69252C, (0x30AC, false)), // Katakana letter GA
    (0x225821, (0x734B, false)), // East Asian ideograph
    (0x282D77, (0x60AD, false)), // East Asian ideograph
    (0x215822, (0x896F, false)), // East Asian ideograph
    (0x6F5A55, (0xCFF0, false)), // Korean hangul
    (0x6F557D, (0xC63B, false)), // Korean hangul
    (0x234C7B, (0x975F, false)), // East Asian ideograph
    (0x222D79, (0x6164, false)), // East Asian ideograph
    (0x4B5824, (0x897E, false)), // East Asian ideograph
    (0x696D7A, (0x9027, false)), // East Asian ideograph
    (0x6F4949, (0xACD7, false)), // Korean hangul
    (0x215825, (0x8981, false)), // East Asian ideograph
    (0x29425C, (0x94CC, false)), // East Asian ideograph
    (0x695C29, (0x6925, false)), // East Asian ideograph
    (0x4B5826, (0x8983, false)), // East Asian ideograph (variant of 215826 which maps to 8983)
    (0x232D7C, (0x8879, false)), // East Asian ideograph
    (0x295827, (0x9CB2, false)), // East Asian ideograph
    (0x6F5A56, (0xCFF3, false)), // Korean hangul
    (0x295369, (0x9A7A, false)), // East Asian ideograph
    (0x215828, (0x898B, false)), // East Asian ideograph
    (0x225829, (0x736C, false)), // East Asian ideograph
    (0x6F494A, (0xACE0, false)), // Korean hangul
    (0x21582A, (0x8993, false)), // East Asian ideograph
    (0x21582B, (0x8996, false)), // East Asian ideograph
    (0x2D5259, (0x98DC, false)), // East Asian ideograph
    (0x21582C, (0x89AA, false)), // East Asian ideograph
    (0x284F6B, (0x6F13, false)), // East Asian ideograph
    (0x29536A, (0x9A9D, false)), // East Asian ideograph
    (0x21582D, (0x89A6, false)), // East Asian ideograph
    (0x27582E, (0x89CA, false)), // East Asian ideograph
    (0x6F494B, (0xACE1, false)), // Korean hangul
    (0x22582F, (0x736F, false)), // East Asian ideograph
    (0x275830, (0x89C9, false)), // East Asian ideograph
    (0x215831, (0x89BD, false)), // East Asian ideograph
    (0x6F5A58, (0xCFFC, false)), // Korean hangul
    (0x275832, (0x89C2, false)), // East Asian ideograph
    (0x222871, (0x5EE8, false)), // East Asian ideograph
    (0x22443C, (0x6B43, false)), // East Asian ideograph
    (0x33483B, (0x6CDD, false)), // East Asian ideograph
    (0x2D5833, (0x752A, false)), // East Asian ideograph
    (0x276037, (0x9875, false)), // East Asian ideograph
    (0x6F494C, (0xACE4, false)), // Korean hangul
    (0x215834, (0x89E3, false)), // East Asian ideograph
    (0x29425F, (0x94B6, false)), // East Asian ideograph
    (0x275835, (0x89DE, false)), // East Asian ideograph
    (0x274933, (0x6E0E, false)), // East Asian ideograph
    (0x235B52, (0x9D6F, false)), // East Asian ideograph
    (0x215836, (0x89F8, false)), // East Asian ideograph
    (0x455837, (0x8BA0, false)), // East Asian ideograph
    (0x4C5F69, (0x75EB, false)), // East Asian ideograph
    (0x275838, (0x8BA1, false)), // East Asian ideograph
    (0x6F4B36, (0xB01C, false)), // Korean hangul
    (0x6F494D, (0xACE7, false)), // Korean hangul
    (0x275839, (0x8BA2, false)), // East Asian ideograph
    (0x4D2962, (0x86C9, false)), // East Asian ideograph (variant of 232962 which maps to 86C9)
    (0x223331, (0x640C, false)), // East Asian ideograph
    (0x22583B, (0x7381, false)), // East Asian ideograph
    (0x6F5A5A, (0xD018, false)), // Korean hangul
    (0x2E7431, (0x7F48, false)), // East Asian ideograph
    (0x4B625C, (0x9EB8, false)), // East Asian ideograph (variant of 27625C which maps to 9EB8)
    (0x27583C, (0x8BB0, false)), // East Asian ideograph
    (0x22443E, (0x6B48, false)), // East Asian ideograph
    (0x23483D, (0x9467, false)), // East Asian ideograph
    (0x27583D, (0x8BA8, false)), // East Asian ideograph
    (0x273A63, (0x5B6A, false)), // East Asian ideograph
    (0x6F494E, (0xACE8, false)), // Korean hangul
    (0x27583E, (0x8BA7, false)), // East Asian ideograph
    (0x294261, (0x94B2, false)), // East Asian ideograph
    (0x22583F, (0x7388, false)), // East Asian ideograph
    (0x2D525D, (0x6537, false)), // East Asian ideograph
    (0x224C3F, (0x6F26, false)), // East Asian ideograph
    (0x6F5A5B, (0xD02D, false)), // Korean hangul
    (0x215841, (0x8A16, false)), // East Asian ideograph
    (0x215842, (0x8A17, false)), // East Asian ideograph
    (0x6F494F, (0xACEA, false)), // Korean hangul
    (0x275843, (0x8BAD, false)), // East Asian ideograph
    (0x275844, (0x8BBF, false)), // East Asian ideograph
    (0x233732, (0x8D0C, false)), // East Asian ideograph
    (0x2D3045, (0x4E57, false)), // East Asian ideograph
    (0x275845, (0x8BC0, false)), // East Asian ideograph
    (0x6F5A5C, (0xD034, false)), // Korean hangul
    (0x225846, (0x7395, false)), // East Asian ideograph
    (0x294F2C, (0x988F, false)), // East Asian ideograph
    (0x215847, (0x8A25, false)), // East Asian ideograph
    (0x6F4950, (0xACEC, false)), // Korean hangul
    (0x225848, (0x7397, false)), // East Asian ideograph
    (0x6F5739, (0xC810, false)), // Korean hangul
    (0x285C3A, (0x748E, false)), // East Asian ideograph
    (0x6F5054, (0xBB54, false)), // Korean hangul
    (0x215849, (0x8A2D, false)), // East Asian ideograph
    (0x21584A, (0x8A1B, false)), // East Asian ideograph
    (0x6F5A5D, (0xD035, false)), // Korean hangul
    (0x295370, (0x9A9C, false)), // East Asian ideograph
    (0x286D47, (0x7BA6, false)), // East Asian ideograph
    (0x21584B, (0x8A1F, false)), // East Asian ideograph
    (0x21584C, (0x8A3B, false)), // East Asian ideograph
    (0x2D4153, (0x64E3, false)), // East Asian ideograph
    (0x276038, (0x9876, false)), // East Asian ideograph
    (0x6F4951, (0xACEF, false)), // Korean hangul
    (0x22584D, (0x7394, false)), // East Asian ideograph
    (0x294264, (0x94BA, false)), // East Asian ideograph
    (0x21584E, (0x8A55, false)), // East Asian ideograph
    (0x21584F, (0x8A5E, false)), // East Asian ideograph
    (0x4B576C, (0x523E, false)), // East Asian ideograph
    (0x27486D, (0x6DA6, false)), // East Asian ideograph
    (0x275851, (0x8BC2, false)), // East Asian ideograph
    (0x6F4B37, (0xB01D, false)), // Korean hangul
    (0x6F4952, (0xACF0, false)), // Korean hangul
    (0x225852, (0x73A6, false)), // East Asian ideograph
    (0x227768, (0x80B8, false)), // East Asian ideograph
    (0x223336, (0x6415, false)), // East Asian ideograph
    (0x275854, (0x8BC8, false)), // East Asian ideograph
    (0x6F5A5F, (0xD050, false)), // Korean hangul
    (0x275855, (0x8BCB, false)), // East Asian ideograph
    (0x275856, (0x8BC9, false)), // East Asian ideograph
    (0x6F4A5D, (0xAEC4, false)), // Korean hangul
    (0x215857, (0x8A3A, false)), // East Asian ideograph
    (0x21736A, (0x56D4, false)), // East Asian ideograph
    (0x33333C, (0x6C37, false)), // East Asian ideograph
    (0x215858, (0x8A6B, false)), // East Asian ideograph
    (0x4B4621, (0x6B53, false)), // East Asian ideograph
    (0x235859, (0x9C12, false)), // East Asian ideograph
    (0x6F5A60, (0xD06C, false)), // Korean hangul
    (0x27585A, (0x8BE6, false)), // East Asian ideograph
    (0x27585B, (0x8BD5, false)), // East Asian ideograph
    (0x45456D, (0x6A10, false)), // East Asian ideograph
    (0x2D5F2C, (0x5826, false)), // East Asian ideograph
    (0x6F4954, (0xACF3, false)), // Korean hangul
    (0x21585C, (0x8A69, false)), // East Asian ideograph
    (0x6F4B4A, (0xB08F, false)), // Korean hangul
    (0x225551, (0x724F, false)), // East Asian ideograph
    (0x21585D, (0x8A70, false)), // East Asian ideograph
    (0x29443E, (0x94E4, false)), // East Asian ideograph
    (0x21585E, (0x8A63, false)), // East Asian ideograph
    (0x6F5A61, (0xD070, false)), // Korean hangul
    (0x21625F, (0x9EBB, false)), // East Asian ideograph
    (0x21585F, (0x8A7C, false)), // East Asian ideograph
    (0x215860, (0x8AA0, false)), // East Asian ideograph
    (0x6F4E30, (0xB5A0, false)), // Korean hangul
    (0x2D5F2D, (0x964F, false)), // East Asian ideograph
    (0x275861, (0x5938, false)), // East Asian ideograph
    (0x275840, (0x8BAF, false)), // East Asian ideograph
    (0x6F5055, (0xBB58, false)), // Korean hangul
    (0x215862, (0x8A85, false)), // East Asian ideograph
    (0x6F5441, (0xC329, false)), // Korean hangul
    (0x225863, (0x73A0, false)), // East Asian ideograph
    (0x6F5A62, (0xD074, false)), // Korean hangul
    (0x695375, (0x56CE, false)), // East Asian ideograph
    (0x6F5864, (0xCB18, false)), // Korean hangul
    (0x215865, (0x8A62, false)), // East Asian ideograph
    (0x2D575B, (0x886E, false)), // East Asian ideograph
    (0x3F4956, (0x7832, false)), // East Asian ideograph
    (0x215866, (0x8A71, false)), // East Asian ideograph
    (0x285C40, (0x74D2, false)), // East Asian ideograph
    (0x215867, (0x8A6E, false)), // East Asian ideograph
    (0x295132, (0x997D, false)), // East Asian ideograph
    (0x233739, (0x8D11, false)), // East Asian ideograph
    (0x2D5265, (0x79D0, false)), // East Asian ideograph
    (0x225868, (0x73CF, false)), // East Asian ideograph
    (0x6F5A63, (0xD07C, false)), // Korean hangul
    (0x282D5E, (0x607A, false)), // East Asian ideograph
    (0x6F4A25, (0xADD0, false)), // Korean hangul
    (0x275869, (0x8BF4, false)), // East Asian ideograph
    (0x21586A, (0x8AA6, false)), // East Asian ideograph
    (0x6F4B38, (0xB028, false)), // Korean hangul
    (0x21586B, (0x8AA1, false)), // East Asian ideograph
    (0x215155, (0x7DD6, false)), // East Asian ideograph
    (0x22333B, (0x6422, false)), // East Asian ideograph
    (0x27586D, (0x5FD7, false)), // East Asian ideograph
    (0x22456F, (0x6BE7, false)), // East Asian ideograph
    (0x23586E, (0x9C23, false)), // East Asian ideograph
    (0x27586F, (0x8BEC, false)), // East Asian ideograph
    (0x6F4F36, (0xB839, false)), // Korean hangul
    (0x6F4A5E, (0xAECC, false)), // Korean hangul
    (0x215870, (0x8A8D, false)), // East Asian ideograph
    (0x21736F, (0x56E1, false)), // East Asian ideograph
    (0x294469, (0x94FC, false)), // East Asian ideograph
    (0x215871, (0x8AA4, false)), // East Asian ideograph (variant of 4B5871 which maps to 8AA4)
    (0x23373B, (0x8D12, false)), // East Asian ideograph
    (0x2D5267, (0x79CF, false)), // East Asian ideograph
    (0x215872, (0x8AA8, false)), // East Asian ideograph
    (0x2D4E24, (0x6998, false)), // East Asian ideograph
    (0x215873, (0x8AA5, false)), // East Asian ideograph
    (0x217E60, (0x5BC9, false)), // East Asian ideograph
    (0x215874, (0x8A98, false)), // East Asian ideograph
    (0x4B5D65, (0x8217, false)), // East Asian ideograph
    (0x6F4959, (0xACFD, false)), // Korean hangul
    (0x215875, (0x8A91, false)), // East Asian ideograph
    (0x6F5130, (0xBC9A, false)), // Korean hangul
    (0x275876, (0x8C0A, false)), // East Asian ideograph
    (0x6F4B68, (0xB0C9, false)), // Korean hangul
    (0x215877, (0x8AC4, false)), // East Asian ideograph
    (0x6F5A66, (0xD0A4, false)), // Korean hangul
    (0x4B5176, (0x7E04, false)), // East Asian ideograph
    (0x295379, (0x9A96, false)), // East Asian ideograph
    (0x235878, (0x9C21, false)), // East Asian ideograph
    (0x234323, (0x924E, false)), // East Asian ideograph
    (0x275879, (0x8C08, false)), // East Asian ideograph
    (0x6F495A, (0xAD00, false)), // Korean hangul
    (0x27587A, (0x8BF7, false)), // East Asian ideograph
    (0x224325, (0x6ACC, false)), // East Asian ideograph
    (0x21587B, (0x8AF8, false)), // East Asian ideograph
    (0x23373D, (0x8D14, false)), // East Asian ideograph
    (0x21587C, (0x8AB2, false)), // East Asian ideograph
    (0x234327, (0x9256, false)), // East Asian ideograph
    (0x29537A, (0x9AA2, false)), // East Asian ideograph
    (0x21587D, (0x8ABF, false)), // East Asian ideograph
    (0x224328, (0x6AD1, false)), // East Asian ideograph
    (0x21587E, (0x8AC9, false)), // East Asian ideograph
    (0x4C7D4D, (0x8343, false)), // East Asian ideograph
    (0x2D4329, (0x668E, false)), // East Asian ideograph
    (0x6F495B, (0xAD04, false)), // Korean hangul
    (0x6F5D31, (0xD611, false)), // Korean hangul
    (0x6F4F7C, (0xB9F9, false)), // Korean hangul
    (0x23432B, (0x925A, false)), // East Asian ideograph
    (0x2D3051, (0x5F0D, false)), // East Asian ideograph
    (0x6F5A68, (0xD0A8, false)), // Korean hangul
    (0x216266, (0x9ED1, false)), // East Asian ideograph
    (0x27432D, (0x65F6, false)), // East Asian ideograph
    (0x4B5736, (0x877F, false)), // East Asian ideograph
    (0x23432E, (0x9241, false)), // East Asian ideograph
    (0x6F495C, (0xAD0C, false)), // Korean hangul
    (0x285252, (0x709C, false)), // East Asian ideograph
    (0x275847, (0x8BB7, false)), // East Asian ideograph
    (0x23432F, (0x9283, false)), // East Asian ideograph
    (0x335958, (0x8C4A, false)), // East Asian ideograph
    (0x394330, (0x6644, false)), // East Asian ideograph
    (0x2D526B, (0x7085, false)), // East Asian ideograph
    (0x234331, (0x92A5, false)), // East Asian ideograph
    (0x213065, (0x4ECA, false)), // East Asian ideograph
    (0x6F5626, (0xC653, false)), // Korean hangul
    (0x274332, (0x663C, false)), // East Asian ideograph
    (0x234333, (0x9282, false)), // East Asian ideograph
    (0x2D5F35, (0x78D2, false)), // East Asian ideograph
    (0x6F495D, (0xAD0D, false)), // Korean hangul
    (0x275848, (0x8BB8, false)), // East Asian ideograph
    (0x224334, (0x6ACD, false)), // East Asian ideograph
    (0x234335, (0x92A8, false)), // East Asian ideograph
    (0x2D3053, (0x4E3C, false)), // East Asian ideograph
    (0x29572B, (0x9C90, false)), // East Asian ideograph
    (0x6F5A6A, (0xD0B4, false)), // Korean hangul
    (0x224337, (0x6AEC, false)), // East Asian ideograph
    (0x215E25, (0x938A, false)), // East Asian ideograph
    (0x234338, (0x92A4, false)), // East Asian ideograph
    (0x6F495E, (0xAD0F, false)), // Korean hangul
    (0x224339, (0x6AF3, false)), // East Asian ideograph
    (0x22433A, (0x6AE7, false)), // East Asian ideograph
    (0x2D433B, (0x6662, false)), // East Asian ideograph
    (0x226225, (0x7735, false)), // East Asian ideograph
    (0x4B5B29, (0x8E8D, false)), // East Asian ideograph
    (0x6F495F, (0xAD11, false)), // Korean hangul
    (0x23433E, (0x9276, false)), // East Asian ideograph
    (0x227775, (0x6711, false)), // East Asian ideograph
    (0x22433F, (0x6AEB, false)), // East Asian ideograph
    (0x224340, (0x6AEA, false)), // East Asian ideograph
    (0x21626A, (0x9EDE, false)), // East Asian ideograph
    (0x274341, (0x6655, false)), // East Asian ideograph
    (0x6F5428, (0xC2EB, false)), // Korean hangul
    (0x234342, (0x9288, false)), // East Asian ideograph
    (0x27603B, (0x987A, false)), // East Asian ideograph
    (0x6F4960, (0xAD18, false)), // Korean hangul
    (0x274343, (0x7545, false)), // East Asian ideograph
    (0x6F4F7D, (0xB9FA, false)), // Korean hangul
    (0x223344, (0x6430, false)), // East Asian ideograph
    (0x224344, (0x6AF1, false)), // East Asian ideograph
    (0x4C6C46, (0x7B9F, false)), // East Asian ideograph
    (0x234345, (0x928E, false)), // East Asian ideograph
    (0x6F562A, (0xC65D, false)), // Korean hangul
    (0x234346, (0x92A0, false)), // East Asian ideograph
    (0x4B7954, (0x5968, false)), // East Asian ideograph
    (0x6F4B3A, (0xB045, false)), // Korean hangul
    (0x234347, (0x9277, false)), // East Asian ideograph
    (0x6F4961, (0xAD19, false)), // Korean hangul
    (0x274348, (0x6653, false)), // East Asian ideograph
    (0x274349, (0x5386, false)), // East Asian ideograph (duplicate simplified)
    (0x6F564F, (0xC6E9, false)), // Korean hangul
    (0x224571, (0x6BE8, false)), // East Asian ideograph
    (0x213066, (0x4EC1, false)), // East Asian ideograph
    (0x6F562B, (0xC660, false)), // Korean hangul
    (0x27434B, (0x66A7, false)), // East Asian ideograph
    (0x6F4962, (0xAD1C, false)), // Korean hangul
    (0x27434D, (0x65F7, false)), // East Asian ideograph
    (0x6F4A60, (0xAECF, false)), // Korean hangul
    (0x4B335B, (0x522B, false)), // East Asian ideograph
    (0x23595E, (0x9C6E, false)), // East Asian ideograph
    (0x22434E, (0x6AFD, false)), // East Asian ideograph
    (0x2D3058, (0x4E9C, false)), // East Asian ideograph
    (0x29434F, (0x94DE, false)), // East Asian ideograph
    (0x6F562C, (0xC671, false)), // Korean hangul
    (0x224350, (0x6AFA, false)), // East Asian ideograph
    (0x347D24, (0x83C7, false)), // East Asian ideograph
    (0x6F552E, (0xC561, false)), // Korean hangul
    (0x2D5F3B, (0x96A0, false)), // East Asian ideograph
    (0x6F4963, (0xAD20, false)), // Korean hangul
    (0x6F5132, (0xBCA1, false)), // Korean hangul
    (0x224352, (0x6B01, false)), // East Asian ideograph
    (0x4B4A74, (0x731C, false)), // East Asian ideograph (variant of 214A74 which maps to 731C)
    (0x234354, (0x927E, false)), // East Asian ideograph
    (0x6F5C47, (0xD45C, false)), // Korean hangul
    (0x274355, (0x4E66, false)), // East Asian ideograph
    (0x6F4964, (0xAD28, false)), // Korean hangul
    (0x334357, (0x6702, false)), // East Asian ideograph
    (0x223348, (0x6435, false)), // East Asian ideograph
    (0x224358, (0x6B03, false)), // East Asian ideograph
    (0x224359, (0x6AF8, false)), // East Asian ideograph
    (0x21605F, (0x98B6, false)), // East Asian ideograph
    (0x4D6047, (0x816D, false)), // East Asian ideograph
    (0x27435A, (0x4F1A, false)), // East Asian ideograph
    (0x23435B, (0x9291, false)), // East Asian ideograph
    (0x27603C, (0x987B, false)), // East Asian ideograph
    (0x6F4965, (0xAD29, false)), // Korean hangul
    (0x6F4F7E, (0xBA00, false)), // Korean hangul
    (0x23435D, (0x929B, false)), // East Asian ideograph
    (0x233748, (0x8D6C, false)), // East Asian ideograph
    (0x2D305B, (0x4EBE, false)), // East Asian ideograph
    (0x217573, (0x57D5, false)), // East Asian ideograph
    (0x6F562F, (0xC67C, false)), // Korean hangul
    (0x284B43, (0x8365, false)), // East Asian ideograph
    (0x22435F, (0x6B0D, false)), // East Asian ideograph
    (0x6F4B3B, (0xB048, false)), // Korean hangul
    (0x224360, (0x6B09, false)), // East Asian ideograph
    (0x355053, (0x98C8, false)), // East Asian ideograph
    (0x6F4966, (0xAD2D, false)), // Korean hangul
    (0x224361, (0x6B0E, false)), // East Asian ideograph
    (0x225563, (0x726E, false)), // East Asian ideograph
    (0x234362, (0x927F, false)), // East Asian ideograph
    (0x69243C, (0x305C, false)), // Hiragana letter ZE
    (0x6F5630, (0xC680, false)), // Korean hangul
    (0x234364, (0x92A3, false)), // East Asian ideograph
    (0x6F4967, (0xAD34, false)), // Korean hangul
    (0x275852, (0x8BCF, false)), // East Asian ideograph
    (0x214366, (0x6727, false)), // East Asian ideograph
    (0x224367, (0x6B11, false)), // East Asian ideograph
    (0x2D4E33, (0x78AA, false)), // East Asian ideograph
    (0x3F5631, (0x517F, false)), // East Asian ideograph
    (0x334369, (0x5932, false)), // East Asian ideograph
    (0x234857, (0x9464, false)), // East Asian ideograph
    (0x21436A, (0x672B, false)), // East Asian ideograph
    (0x293032, (0x7962, false)), // East Asian ideograph
    (0x6F4968, (0xAD38, false)), // Korean hangul
    (0x275853, (0x8BC5, false)), // East Asian ideograph
    (0x33496A, (0x934A, false)), // East Asian ideograph
    (0x22436D, (0x6B19, false)), // East Asian ideograph
    (0x4B5179, (0x7F0B, false)), // East Asian ideograph
    (0x6F5632, (0xC68B, false)), // Korean hangul
    (0x23436F, (0x92D0, false)), // East Asian ideograph
    (0x6F4969, (0xAD3C, false)), // Korean hangul
    (0x2D4370, (0x6736, false)), // East Asian ideograph
    (0x215167, (0x7E46, false)), // East Asian ideograph
    (0x234371, (0x92F1, false)), // East Asian ideograph
    (0x234372, (0x92DF, false)), // East Asian ideograph
    (0x275528, (0x835A, false)), // East Asian ideograph
    (0x215E31, (0x93C8, false)), // East Asian ideograph
    (0x6F496A, (0xAD44, false)), // Korean hangul
    (0x214375, (0x6750, false)), // East Asian ideograph
    (0x215168, (0x7E37, false)), // East Asian ideograph
    (0x6F572B, (0xC7BC, false)), // Korean hangul
    (0x234376, (0x92B6, false)), // East Asian ideograph
    (0x4B4638, (0x6BB1, false)), // East Asian ideograph
    (0x234377, (0x92C0, false)), // East Asian ideograph
    (0x6F5634, (0xC694, false)), // Korean hangul
    (0x6F4D2B, (0xB35B, false)), // Korean hangul
    (0x6F4B3C, (0xB04A, false)), // Korean hangul
    (0x234379, (0x92BE, false)), // East Asian ideograph
    (0x6F572E, (0xC7C0, false)), // Korean hangul
    (0x2D3061, (0x4EB0, false)), // East Asian ideograph
    (0x6F5A78, (0xD0D4, false)), // Korean hangul
    (0x6F5635, (0xC695, false)), // Korean hangul
    (0x29437D, (0x94D8, false)), // East Asian ideograph
    (0x696E28, (0x9056, false)), // East Asian ideograph
    (0x4B5746, (0x8853, false)), // East Asian ideograph
    (0x23437E, (0x92D5, false)), // East Asian ideograph
    (0x2D3D2B, (0x5EBF, false)), // East Asian ideograph
    (0x6F4A62, (0xAED1, false)), // Korean hangul
    (0x276E2A, (0x53A3, false)), // East Asian ideograph
    (0x2D527B, (0x8074, false)), // East Asian ideograph
    (0x6F5A79, (0xD0D5, false)), // Korean hangul
    (0x282D74, (0x6004, false)), // East Asian ideograph
    (0x6F5636, (0xC698, false)), // Korean hangul
    (0x23485C, (0x9465, false)), // East Asian ideograph
    (0x226233, (0x774A, false)), // East Asian ideograph
    (0x6F496D, (0xAD50, false)), // Korean hangul
    (0x6F5C49, (0xD478, false)), // Korean hangul
    (0x6F5840, (0xC9ED, false)), // Korean hangul
    (0x222E2F, (0x615C, false)), // East Asian ideograph
    (0x223351, (0x640A, false)), // East Asian ideograph
    (0x21317D, (0x501A, false)), // East Asian ideograph
    (0x6F5A7A, (0xD0DC, false)), // Korean hangul
    (0x2E7451, (0x7F58, false)), // East Asian ideograph
    (0x6F5637, (0xC6A5, false)), // Korean hangul
    (0x215E35, (0x93DD, false)), // East Asian ideograph
    (0x23485D, (0x9455, false)), // East Asian ideograph
    (0x6F4E31, (0xB5A1, false)), // Korean hangul
    (0x225E2C, (0x7590, false)), // East Asian ideograph
    (0x2D3D2D, (0x5396, false)), // East Asian ideograph
    (0x275859, (0x8BE5, false)), // East Asian ideograph
    (0x21516C, (0x7E2B, false)), // East Asian ideograph
    (0x225128, (0x70E0, false)), // East Asian ideograph
    (0x6F5A7B, (0xD0DD, false)), // Korean hangul
    (0x275529, (0x830E, false)), // East Asian ideograph
    (0x22445F, (0x6B6E, false)), // East Asian ideograph
    (0x33485E, (0x67D2, false)), // East Asian ideograph
    (0x226235, (0x7743, false)), // East Asian ideograph
    (0x2D4171, (0x62CA, false)), // East Asian ideograph
    (0x6F496F, (0xAD6D, false)), // Korean hangul
    (0x6F572C, (0xC7BD, false)), // Korean hangul
    (0x27493A, (0x6FD1, false)), // East Asian ideograph
    (0x23596B, (0x9C7A, false)), // East Asian ideograph
    (0x235B59, (0x9DA9, false)), // East Asian ideograph
    (0x27474E, (0x6CFE, false)), // East Asian ideograph
    (0x6F5639, (0xC6A9, false)), // Korean hangul
    (0x215E37, (0x93D8, false)), // East Asian ideograph
    (0x222E3D, (0x61A2, false)), // East Asian ideograph
    (0x2D3D2F, (0x539B, false)), // East Asian ideograph
    (0x6F5946, (0xCCD0, false)), // Korean hangul
    (0x345D6B, (0x756D, false)), // East Asian ideograph
    (0x285D6B, (0x7572, false)), // East Asian ideograph
    (0x222E40, (0x61A8, false)), // East Asian ideograph
    (0x6F563A, (0xC6B0, false)), // Korean hangul
    (0x4B3C2B, (0x67C3, false)), // East Asian ideograph (Version J extension)
    (0x6F4F37, (0xB840, false)), // Korean hangul
    (0x6F4971, (0xAD73, false)), // Korean hangul
    (0x6F4A63, (0xAED8, false)), // Korean hangul
    (0x22512B, (0x70D4, false)), // East Asian ideograph
    (0x23552A, (0x9AEF, false)), // East Asian ideograph
    (0x6F5A7E, (0xD0EC, false)), // Korean hangul
    (0x222E45, (0x6196, false)), // East Asian ideograph
    (0x6F563B, (0xC6B1, false)), // Korean hangul
    (0x6F4972, (0xAD74, false)), // Korean hangul
    (0x6F563C, (0xC6B4, false)), // Korean hangul
    (0x234862, (0x946A, false)), // East Asian ideograph
    (0x282E4C, (0x6126, false)), // East Asian ideograph
    (0x2D5F4B, (0x96D1, false)), // East Asian ideograph
    (0x6F4973, (0xAD75, false)), // Korean hangul
    (0x215171, (0x7E54, false)), // East Asian ideograph
    (0x6F563D, (0xC6B7, false)), // Korean hangul
    (0x215E3B, (0x93FD, false)), // East Asian ideograph
    (0x2D4176, (0x6483, false)), // East Asian ideograph
    (0x2D5F4C, (0x9DC4, false)), // East Asian ideograph
    (0x6F4974, (0xAD76, false)), // Korean hangul
    (0x6F5D32, (0xD613, false)), // Korean hangul
    (0x282E52, (0x6003, false)), // East Asian ideograph
    (0x27493B, (0x6CA5, false)), // East Asian ideograph
    (0x233941, (0x8DFD, false)), // East Asian ideograph
    (0x6F563E, (0xC6B8, false)), // Korean hangul
    (0x4B5773, (0x7ED4, false)), // East Asian ideograph
    (0x213C23, (0x5D22, false)), // East Asian ideograph
    (0x286E56, (0x7BA8, false)), // East Asian ideograph
    (0x2D3D34, (0x5EFE, false)), // East Asian ideograph
    (0x215173, (0x7E5E, false)), // East Asian ideograph
    (0x223359, (0x6407, false)), // East Asian ideograph
    (0x216E58, (0x535F, false)), // East Asian ideograph
    (0x707523, (0x9170, false)), // East Asian ideograph
    (0x6F4B77, (0xB119, false)), // Korean hangul
    (0x222E5A, (0x61CB, false)), // East Asian ideograph
    (0x213C24, (0x5D29, false)), // East Asian ideograph
    (0x33355C, (0x5449, false)), // East Asian ideograph
    (0x273648, (0x95EE, false)), // East Asian ideograph
    (0x282E5C, (0x603F, false)), // East Asian ideograph
    (0x27514C, (0x7EFF, false)), // East Asian ideograph
    (0x6F5579, (0xC635, false)), // Korean hangul
    (0x6F5640, (0xC6BA, false)), // Korean hangul
    (0x6F5951, (0xCD5C, false)), // Korean hangul
    (0x4B397B, (0x5A2F, false)), // East Asian ideograph
    (0x29402C, (0x90D0, false)), // East Asian ideograph
    (0x22623D, (0x7760, false)), // East Asian ideograph
    (0x6F4977, (0xAD7F, false)), // Korean hangul
    (0x6F5136, (0xBCB0, false)), // Korean hangul
    (0x216E61, (0x5414, false)), // East Asian ideograph
    (0x22335B, (0x643B, false)), // East Asian ideograph
    (0x6F4A2E, (0xADFC, false)), // Korean hangul
    (0x6F5641, (0xC6C0, false)), // Korean hangul
    (0x213C26, (0x5D19, false)), // East Asian ideograph
    (0x275863, (0x8BE1, false)), // East Asian ideograph
    (0x695A7E, (0x66BC, false)), // East Asian ideograph
    (0x287E61, (0x82CC, false)), // East Asian ideograph
    (0x232E68, (0x88D2, false)), // East Asian ideograph
    (0x6F5642, (0xC6C1, false)), // Korean hangul
    (0x234868, (0x946B, false)), // East Asian ideograph
    (0x6F5429, (0xC2EC, false)), // Korean hangul
    (0x334425, (0x76C3, false)), // East Asian ideograph
    (0x6F4979, (0xAD82, false)), // Korean hangul
    (0x296062, (0x9F9B, false)), // East Asian ideograph
    (0x22335D, (0x643F, false)), // East Asian ideograph
    (0x23375C, (0x8D7A, false)), // East Asian ideograph
    (0x6F5643, (0xC6C3, false)), // Korean hangul
    (0x213C28, (0x5D50, false)), // East Asian ideograph
    (0x216E6F, (0x541A, false)), // East Asian ideograph
    (0x6F497A, (0xAD88, false)), // Korean hangul
    (0x275422, (0x810F, false)), // East Asian ideograph (duplicate simplified)
    (0x222E71, (0x61E0, false)), // East Asian ideograph
    (0x2D3E40, (0x6052, false)), // East Asian ideograph
    (0x6F5644, (0xC6C5, false)), // Korean hangul
    (0x28702E, (0x56E2, false)), // East Asian ideograph (duplicate simplified)
    (0x6F497B, (0xAD8C, false)), // Korean hangul
    (0x6F4A65, (0xAEF4, false)), // Korean hangul
    (0x455164, (0x53BF, false)), // East Asian ideograph
    (0x215921, (0x8AC2, false)), // East Asian ideograph
    (0x222E77, (0x61E5, false)), // East Asian ideograph
    (0x275922, (0x8C01, false)), // East Asian ideograph
    (0x275923, (0x8BDE, false)), // East Asian ideograph
    (0x232652, (0x85A2, false)), // East Asian ideograph
    (0x6F552F, (0xC564, false)), // Korean hangul
    (0x216E79, (0x5454, false)), // East Asian ideograph
    (0x275924, (0x8BBA, false)), // East Asian ideograph
    (0x235925, (0x9C32, false)), // East Asian ideograph
    (0x215926, (0x8AFA, false)), // East Asian ideograph
    (0x275927, (0x8C0F, false)), // East Asian ideograph
    (0x216E7D, (0x543D, false)), // East Asian ideograph
    (0x235928, (0x9C48, false)), // East Asian ideograph
    (0x282E7E, (0x603C, false)), // East Asian ideograph
    (0x215929, (0x8AE7, false)), // East Asian ideograph
    (0x275868, (0x8BDF, false)), // East Asian ideograph
    (0x6F505D, (0xBBC8, false)), // Korean hangul
    (0x23592A, (0x9C33, false)), // East Asian ideograph
    (0x21592B, (0x8B00, false)), // East Asian ideograph
    (0x6F5B72, (0xD31D, false)), // Korean hangul
    (0x27592C, (0x8C12, false)), // East Asian ideograph
    (0x6F5647, (0xC6D0, false)), // Korean hangul
    (0x29416B, (0x948E, false)), // East Asian ideograph
    (0x213E47, (0x6064, false)), // East Asian ideograph
    (0x23486D, (0x9471, false)), // East Asian ideograph
    (0x27592E, (0x8BFA, false)), // East Asian ideograph
    (0x22592F, (0x73D4, false)), // East Asian ideograph
    (0x27493D, (0x6F47, false)), // East Asian ideograph
    (0x233761, (0x8D84, false)), // East Asian ideograph
    (0x215930, (0x8AED, false)), // East Asian ideograph
    (0x335A7B, (0x8E28, false)), // East Asian ideograph
    (0x275931, (0x8C24, false)), // East Asian ideograph
    (0x6F5648, (0xC6D4, false)), // Korean hangul
    (0x697174, (0x9ADE, false)), // East Asian ideograph
    (0x235932, (0x9C35, false)), // East Asian ideograph
    (0x275933, (0x8C1C, false)), // East Asian ideograph
    (0x27586A, (0x8BF5, false)), // East Asian ideograph
    (0x215934, (0x8B1B, false)), // East Asian ideograph
    (0x215935, (0x8B0A, false)), // East Asian ideograph
    (0x225936, (0x73E7, false)), // East Asian ideograph
    (0x6F5649, (0xC6DC, false)), // Korean hangul
    (0x275937, (0x8A8A, false)), // East Asian ideograph
    (0x215938, (0x8B1D, false)), // East Asian ideograph
    (0x27586B, (0x8BEB, false)), // East Asian ideograph
    (0x6F4A66, (0xAF0D, false)), // Korean hangul
    (0x282F66, (0x6217, false)), // East Asian ideograph
    (0x215939, (0x8B39, false)), // East Asian ideograph
    (0x22513A, (0x70D0, false)), // East Asian ideograph
    (0x21593A, (0x8B2C, false)), // East Asian ideograph
    (0x21593B, (0x8B28, false)), // East Asian ideograph
    (0x6F564A, (0xC6DD, false)), // Korean hangul
    (0x224471, (0x6B82, false)), // East Asian ideograph
    (0x21593C, (0x8B58, false)), // East Asian ideograph
    (0x27593D, (0x8C31, false)), // East Asian ideograph
    (0x6F5138, (0xBCB3, false)), // Korean hangul
    (0x27586C, (0x8BED, false)), // East Asian ideograph
    (0x27593E, (0x8C32, false)), // East Asian ideograph
    (0x22513B, (0x70C7, false)), // East Asian ideograph
    (0x22593F, (0x73E9, false)), // East Asian ideograph
    (0x274E5B, (0x77FF, false)), // East Asian ideograph
    (0x215940, (0x8B5A, false)), // East Asian ideograph
    (0x6F564B, (0xC6DF, false)), // Korean hangul
    (0x395577, (0x854B, false)), // East Asian ideograph
    (0x213C30, (0x5DCD, false)), // East Asian ideograph
    (0x215942, (0x8B4F, false)), // East Asian ideograph
    (0x6F573B, (0xC813, false)), // Korean hangul
    (0x275943, (0x8BAE, false)), // East Asian ideograph
    (0x22472C, (0x6C54, false)), // East Asian ideograph
    (0x22513C, (0x70DA, false)), // East Asian ideograph
    (0x6F5944, (0xCCBC, false)), // Korean hangul
    (0x2D435F, (0x6716, false)), // East Asian ideograph
    (0x6F5B73, (0xD31F, false)), // Korean hangul
    (0x235945, (0x9C51, false)), // East Asian ideograph
    (0x6F564C, (0xC6E0, false)), // Korean hangul
    (0x69255A, (0x30DA, false)), // Katakana letter PE
    (0x213C31, (0x5DD2, false)), // East Asian ideograph
    (0x215947, (0x8B74, false)), // East Asian ideograph
    (0x215948, (0x8B77, false)), // East Asian ideograph
    (0x2E4C7B, (0x6E86, false)), // East Asian ideograph
    (0x22513D, (0x70C6, false)), // East Asian ideograph
    (0x235949, (0x9C63, false)), // East Asian ideograph
    (0x333323, (0x4E21, false)), // East Asian ideograph
    (0x22594A, (0x73F8, false)), // East Asian ideograph
    (0x6F564D, (0xC6E1, false)), // Korean hangul
    (0x6F5275, (0xC0DB, false)), // Korean hangul
    (0x27594B, (0x53D8, false)), // East Asian ideograph
    (0x21594C, (0x8B93, false)), // East Asian ideograph
    (0x27594D, (0x8C36, false)), // East Asian ideograph
    (0x28582B, (0x7303, false)), // East Asian ideograph
    (0x27594E, (0x8C17, false)), // East Asian ideograph
    (0x21594F, (0x8B9A, false)), // East Asian ideograph
    (0x6F564E, (0xC6E8, false)), // Korean hangul
    (0x4B3C2F, (0x5DBA, false)), // East Asian ideograph
    (0x287030, (0x7C9D, false)), // East Asian ideograph
    (0x213C33, (0x5DD6, false)), // East Asian ideograph
    (0x6F5A2C, (0xCEF9, false)), // Korean hangul
    (0x2D3253, (0x50E3, false)), // East Asian ideograph
    (0x6F4A67, (0xAF2C, false)), // Korean hangul
    (0x6F5952, (0xCD78, false)), // Korean hangul
    (0x333768, (0x8FF4, false)), // East Asian ideograph
    (0x21373F, (0x5659, false)), // East Asian ideograph
    (0x393439, (0x61C3, false)), // East Asian ideograph
    (0x215954, (0x8C48, false)), // East Asian ideograph
    (0x395652, (0x87A1, false)), // East Asian ideograph
    (0x215955, (0x8C49, false)), // East Asian ideograph
    (0x215956, (0x8C4C, false)), // East Asian ideograph
    (0x396167, (0x9B2A, false)), // East Asian ideograph
    (0x706B5B, (0x810E, false)), // East Asian ideograph
    (0x275957, (0x7AD6, false)), // East Asian ideograph
    (0x215958, (0x8C50, false)), // East Asian ideograph
    (0x474931, (0x95F6, false)), // East Asian ideograph
    (0x2D5959, (0x8277, false)), // East Asian ideograph
    (0x213665, (0x558A, false)), // East Asian ideograph
    (0x22595A, (0x73FD, false)), // East Asian ideograph
    (0x6F4E32, (0xB5A4, false)), // Korean hangul
    (0x217C24, (0x5AA7, false)), // East Asian ideograph
    (0x2F5973, (0x9CEC, false)), // East Asian ideograph
    (0x6F595B, (0xCDB0, false)), // Korean hangul
    (0x21595C, (0x8C62, false)), // East Asian ideograph
    (0x4B4655, (0x6C17, false)), // East Asian ideograph
    (0x6F595D, (0xCDCC, false)), // Korean hangul
    (0x455D3E, (0x9485, false)), // East Asian ideograph
    (0x6F5B74, (0xD320, false)), // Korean hangul
    (0x21595E, (0x8C6B, false)), // East Asian ideograph
    (0x6F5651, (0xC6F0, false)), // Korean hangul
    (0x69717D, (0x9AF1, false)), // East Asian ideograph
    (0x2D595F, (0x732A, false)), // East Asian ideograph
    (0x2D5960, (0x72B2, false)), // East Asian ideograph
    (0x6F5731, (0xC7C9, false)), // Korean hangul
    (0x513A47, (0x8885, false)), // East Asian ideograph
    (0x6F5962, (0xCE30, false)), // Korean hangul
    (0x39505B, (0x9B3B, false)), // East Asian ideograph
    (0x215963, (0x8C8A, false)), // East Asian ideograph
    (0x6F5652, (0xC6F8, false)), // Korean hangul
    (0x224479, (0x6B8D, false)), // East Asian ideograph
    (0x4B5964, (0x72E2, false)), // East Asian ideograph
    (0x234435, (0x92DD, false)), // East Asian ideograph
    (0x2D5965, (0x72F8, false)), // East Asian ideograph
    (0x2D3D48, (0x5F4A, false)), // East Asian ideograph
    (0x275966, (0x7683, false)), // East Asian ideograph
    (0x213F79, (0x6249, false)), // East Asian ideograph
    (0x215967, (0x8C93, false)), // East Asian ideograph
    (0x215968, (0x8C9D, false)), // East Asian ideograph
    (0x295B77, (0x9E63, false)), // East Asian ideograph
    (0x215969, (0x8C9E, false)), // East Asian ideograph
    (0x217C27, (0x5A9C, false)), // East Asian ideograph
    (0x6F5C2D, (0xD399, false)), // Korean hangul
    (0x27596A, (0x8D1F, false)), // East Asian ideograph
    (0x6F4A68, (0xAF2D, false)), // Korean hangul
    (0x706B5F, (0x8112, false)), // East Asian ideograph
    (0x21596B, (0x8CA2, false)), // East Asian ideograph
    (0x52735D, (0x7E8A, false)), // East Asian ideograph (variant of 22735D which maps to 7E8A)
    (0x695C30, (0x6923, false)), // East Asian ideograph
    (0x225144, (0x7104, false)), // East Asian ideograph
    (0x22596C, (0x7430, false)), // East Asian ideograph
    (0x33332A, (0x4E93, false)), // East Asian ideograph
    (0x21596D, (0x8CAC, false)), // East Asian ideograph
    (0x21596E, (0x8CAB, false)), // East Asian ideograph
    (0x217C28, (0x5A7C, false)), // East Asian ideograph
    (0x697260, (0x9C30, false)), // East Asian ideograph
    (0x27596F, (0x8D27, false)), // East Asian ideograph
    (0x215970, (0x8CAA, false)), // East Asian ideograph
    (0x695C31, (0x6921, false)), // East Asian ideograph
    (0x215971, (0x8CA7, false)), // East Asian ideograph
    (0x6F5C4F, (0xD48B, false)), // Korean hangul
    (0x275E46, (0x9576, false)), // East Asian ideograph
    (0x215972, (0x8CA9, false)), // East Asian ideograph
    (0x215973, (0x8CAF, false)), // East Asian ideograph
    (0x217C29, (0x5A96, false)), // East Asian ideograph
    (0x6F5974, (0xCE85, false)), // Korean hangul
    (0x275975, (0x8D39, false)), // East Asian ideograph
    (0x6F5060, (0xBBF9, false)), // Korean hangul
    (0x4B465A, (0x6C32, false)), // East Asian ideograph
    (0x275976, (0x8D32, false)), // East Asian ideograph
    (0x234421, (0x92C6, false)), // East Asian ideograph
    (0x215977, (0x8CC0, false)), // East Asian ideograph
    (0x6F5656, (0xC70C, false)), // Korean hangul
    (0x277748, (0x57B2, false)), // East Asian ideograph
    (0x215978, (0x8CB4, false)), // East Asian ideograph
    (0x275979, (0x8D34, false)), // East Asian ideograph
    (0x295B2A, (0x9E46, false)), // East Asian ideograph
    (0x6F503E, (0xBAB0, false)), // Korean hangul
    (0x21597A, (0x8CB7, false)), // East Asian ideograph
    (0x234425, (0x92F4, false)), // East Asian ideograph
    (0x27597B, (0x8D2C, false)), // East Asian ideograph
    (0x274426, (0x4E1C, false)), // East Asian ideograph
    (0x27597C, (0x8D3B, false)), // East Asian ideograph
    (0x6F5657, (0xC714, false)), // Korean hangul
    (0x234427, (0x92CF, false)), // East Asian ideograph
    (0x292D51, (0x8511, false)), // East Asian ideograph
    (0x21597D, (0x8CB8, false)), // East Asian ideograph
    (0x23443A, (0x92CA, false)), // East Asian ideograph
    (0x27597E, (0x8D38, false)), // East Asian ideograph
    (0x23442A, (0x92B2, false)), // East Asian ideograph
    (0x225148, (0x70F3, false)), // East Asian ideograph
    (0x29442B, (0x9503, false)), // East Asian ideograph
    (0x6F5324, (0xC120, false)), // Korean hangul
    (0x6F5658, (0xC717, false)), // Korean hangul
    (0x23442C, (0x92E7, false)), // East Asian ideograph
    (0x294F6B, (0x98A1, false)), // East Asian ideograph
    (0x696D41, (0x8EC8, false)), // East Asian ideograph
    (0x23442D, (0x92C7, false)), // East Asian ideograph
    (0x277D40, (0x5AF1, false)), // East Asian ideograph
    (0x2D3D4E, (0x7BF2, false)), // East Asian ideograph
    (0x23442E, (0x92F0, false)), // East Asian ideograph
    (0x6F4A69, (0xAF30, false)), // Korean hangul
    (0x23442F, (0x92DB, false)), // East Asian ideograph
    (0x6F4E5E, (0xB768, false)), // Korean hangul
    (0x234430, (0x92DC, false)), // East Asian ideograph
    (0x2D4E5B, (0x945B, false)), // East Asian ideograph
    (0x234431, (0x92D8, false)), // East Asian ideograph
    (0x224432, (0x6B39, false)), // East Asian ideograph
    (0x234433, (0x92E9, false)), // East Asian ideograph
    (0x224435, (0x6B3F, false)), // East Asian ideograph
    (0x274E5E, (0x783E, false)), // East Asian ideograph
    (0x6F565A, (0xC720, false)), // Korean hangul
    (0x27375A, (0x4E25, false)), // East Asian ideograph
    (0x224437, (0x6B46, false)), // East Asian ideograph
    (0x2D3D50, (0x5F5C, false)), // East Asian ideograph
    (0x224438, (0x6B41, false)), // East Asian ideograph
    (0x234439, (0x92D1, false)), // East Asian ideograph
    (0x6F5061, (0xBBFC, false)), // Korean hangul
    (0x283561, (0x64BA, false)), // East Asian ideograph
    (0x22443A, (0x6B40, false)), // East Asian ideograph
    (0x6F565B, (0xC721, false)), // Korean hangul
    (0x22443B, (0x6B42, false)), // East Asian ideograph
    (0x6F5973, (0xCE84, false)), // Korean hangul
    (0x6F4C7E, (0xB301, false)), // Korean hangul
    (0x23443C, (0x92C2, false)), // East Asian ideograph
    (0x6F4F29, (0xB810, false)), // Korean hangul
    (0x454C3C, (0x7589, false)), // East Asian ideograph
    (0x6F5733, (0xC7D8, false)), // Korean hangul
    (0x23443E, (0x92CC, false)), // East Asian ideograph
    (0x22443F, (0x6B4A, false)), // East Asian ideograph
    (0x235B60, (0x9D98, false)), // East Asian ideograph
    (0x2E525D, (0x715B, false)), // East Asian ideograph
    (0x6F565C, (0xC724, false)), // Korean hangul
    (0x234440, (0x92EF, false)), // East Asian ideograph
    (0x213C41, (0x5DF7, false)), // East Asian ideograph
    (0x234441, (0x92E8, false)), // East Asian ideograph
    (0x6F5D35, (0xD61C, false)), // Korean hangul
    (0x287739, (0x8069, false)), // East Asian ideograph
    (0x27587E, (0x8BFF, false)), // East Asian ideograph
    (0x234443, (0x92EB, false)), // East Asian ideograph
    (0x695C39, (0x697E, false)), // East Asian ideograph
    (0x295D36, (0x9E2C, false)), // East Asian ideograph
    (0x2D4444, (0x69C5, false)), // East Asian ideograph
    (0x6F565D, (0xC728, false)), // Korean hangul
    (0x234445, (0x92F5, false)), // East Asian ideograph
    (0x224446, (0x6B4E, false)), // East Asian ideograph (variant of 4C4446 which maps to 6B4E)
    (0x6F4A6A, (0xAF34, false)), // Korean hangul
    (0x234448, (0x92F2, false)), // East Asian ideograph
    (0x28422B, (0x6A2F, false)), // East Asian ideograph
    (0x334449, (0x6144, false)), // East Asian ideograph
    (0x22444A, (0x6B57, false)), // East Asian ideograph
    (0x2D444B, (0x6852, false)), // East Asian ideograph
    (0x6F5530, (0xC568, false)), // Korean hangul
    (0x6F513C, (0xBCC0, false)), // Korean hangul
    (0x22444C, (0x6B54, false)), // East Asian ideograph
    (0x23444D, (0x9307, false)), // East Asian ideograph
    (0x22444E, (0x6B55, false)), // East Asian ideograph
    (0x6F5C51, (0xD4CC, false)), // Korean hangul
    (0x515E5D, (0x9616, false)), // East Asian ideograph
    (0x2D4450, (0x8308, false)), // East Asian ideograph
    (0x224451, (0x6B5C, false)), // East Asian ideograph
    (0x287A56, (0x8114, false)), // East Asian ideograph
    (0x6F5062, (0xBBFF, false)), // Korean hangul
    (0x212B38, (0xFF0C, false)), // Ideographic variant comma
    (0x225150, (0x70F4, false)), // East Asian ideograph
    (0x23554F, (0x9B10, false)), // East Asian ideograph
    (0x224453, (0x6B5E, false)), // East Asian ideograph
    (0x6F5B77, (0xD328, false)), // Korean hangul
    (0x224454, (0x6B60, false)), // East Asian ideograph
    (0x22625D, (0x777E, false)), // East Asian ideograph
    (0x217C34, (0x5AAE, false)), // East Asian ideograph
    (0x4B4456, (0x6813, false)), // East Asian ideograph
    (0x6F5734, (0xC800, false)), // Korean hangul
    (0x294457, (0x9529, false)), // East Asian ideograph
    (0x212B39, (0xFF1B, false)), // Ideographic semicolon
    (0x234458, (0x931F, false)), // East Asian ideograph
    (0x6F5661, (0xC73C, false)), // Korean hangul
    (0x23445A, (0x9331, false)), // East Asian ideograph
    (0x4D5F70, (0x9F44, false)), // East Asian ideograph
    (0x22445B, (0x6B6B, false)), // East Asian ideograph
    (0x22736B, (0x7E95, false)), // East Asian ideograph
    (0x22445D, (0x6B6C, false)), // East Asian ideograph
    (0x4C284C, (0x53A9, false)), // East Asian ideograph
    (0x4B3C33, (0x5DCC, false)), // East Asian ideograph
    (0x215E60, (0x95BB, false)), // East Asian ideograph
    (0x23445F, (0x930F, false)), // East Asian ideograph
    (0x6F4A6B, (0xAF3C, false)), // Korean hangul
    (0x224461, (0x6B71, false)), // East Asian ideograph
    (0x6F5D2C, (0xD600, false)), // Korean hangul
    (0x234462, (0x9302, false)), // East Asian ideograph
    (0x6F5170, (0xBE4B, false)), // Korean hangul
    (0x274463, (0x6761, false)), // East Asian ideograph
    (0x234464, (0x9324, false)), // East Asian ideograph
    (0x2D5B2F, (0x8EB1, false)), // East Asian ideograph
    (0x214466, (0x6885, false)), // East Asian ideograph
    (0x274468, (0x67AD, false)), // East Asian ideograph
    (0x294F77, (0x989F, false)), // East Asian ideograph
    (0x6F5221, (0xBE70, false)), // Korean hangul
    (0x6F4B7A, (0xB11D, false)), // Korean hangul
    (0x274469, (0x6800, false)), // East Asian ideograph
    (0x2D5F73, (0x975A, false)), // Unrelated variant of EACC 234C76 which maps to 975A
    (0x23446A, (0x9323, false)), // East Asian ideograph
    (0x22446B, (0x6B7E, false)), // East Asian ideograph
    (0x212B3D, (0xFF01, false)), // Ideographic exclamation point
    (0x225155, (0x7111, false)), // East Asian ideograph
    (0x23446C, (0x9321, false)), // East Asian ideograph
    (0x4C683E, (0x79EB, false)), // East Asian ideograph
    (0x27446D, (0x5F03, false)), // East Asian ideograph
    (0x6F5222, (0xBE71, false)), // Korean hangul
    (0x27446E, (0x6816, false)), // East Asian ideograph
    (0x2D4E79, (0x5FA1, false)), // East Asian ideograph
    (0x6F5735, (0xC801, false)), // Korean hangul
    (0x2D4B43, (0x746F, false)), // East Asian ideograph
    (0x274471, (0x680B, false)), // East Asian ideograph
    (0x4B5A68, (0x8DF5, false)), // East Asian ideograph (variant of 275A68 which maps to 8DF5)
    (0x234472, (0x9301, false)), // East Asian ideograph
    (0x6F5223, (0xBE73, false)), // Korean hangul
    (0x6F5326, (0xC124, false)), // Korean hangul
    (0x224473, (0x6B84, false)), // East Asian ideograph
    (0x2D3332, (0x5190, false)), // East Asian ideograph
    (0x234474, (0x9315, false)), // East Asian ideograph
    (0x47366F, (0x8D4D, false)), // East Asian ideograph
    (0x294475, (0x9494, false)), // East Asian ideograph
    (0x235556, (0x9B1D, false)), // East Asian ideograph
    (0x234476, (0x9329, false)), // East Asian ideograph
    (0x23386F, (0x8DBA, false)), // East Asian ideograph
    (0x232F21, (0x88FC, false)), // East Asian ideograph
    (0x2D4A26, (0x713C, false)), // East Asian ideograph
    (0x287035, (0x7C74, false)), // East Asian ideograph
    (0x6F5224, (0xBE74, false)), // Korean hangul
    (0x234478, (0x932E, false)), // East Asian ideograph
    (0x234479, (0x932A, false)), // East Asian ideograph
    (0x6F4A6C, (0xAF3D, false)), // Korean hangul
    (0x27447A, (0x67A3, false)), // East Asian ideograph
    (0x6F5D2D, (0xD601, false)), // Korean hangul
    (0x22447B, (0x6B95, false)), // East Asian ideograph
    (0x21447C, (0x6912, false)), // East Asian ideograph
    (0x216F27, (0x5423, false)), // East Asian ideograph
    (0x213C4D, (0x5E11, false)), // East Asian ideograph
    (0x2D447D, (0x684C, false)), // East Asian ideograph
    (0x2D3D5E, (0x9AF4, false)), // East Asian ideograph
    (0x23447E, (0x9335, false)), // East Asian ideograph
    (0x706058, (0x562D, false)), // East Asian ideograph
    (0x273671, (0x54DF, false)), // East Asian ideograph
    (0x6F5226, (0xBE7B, false)), // Korean hangul
    (0x232F2D, (0x8909, false)), // East Asian ideograph
    (0x23444C, (0x9303, false)), // East Asian ideograph
    (0x4D5B35, (0x9DAB, false)), // East Asian ideograph
    (0x232F2F, (0x8918, false)), // East Asian ideograph
    (0x6F5B79, (0xD32C, false)), // Korean hangul
    (0x213D3C, (0x5F15, false)), // East Asian ideograph
    (0x6F566A, (0xC774, false)), // Korean hangul
    (0x6F5227, (0xBE7C, false)), // Korean hangul
    (0x213C4F, (0x5E25, false)), // East Asian ideograph
    (0x2D3632, (0x8A7B, false)), // East Asian ideograph
    (0x346622, (0x589D, false)), // East Asian ideograph
    (0x295C47, (0x9E68, false)), // East Asian ideograph
    (0x293A2E, (0x8DC4, false)), // East Asian ideograph
    (0x2E6F35, (0x6CD4, false)), // East Asian ideograph
    (0x6F566B, (0xC775, false)), // Korean hangul
    (0x6F5228, (0xBE7D, false)), // Korean hangul
    (0x22516D, (0x7134, false)), // East Asian ideograph
    (0x23444E, (0x931E, false)), // East Asian ideograph
    (0x2F5D49, (0x9EA4, false)), // East Asian ideograph
    (0x2E313A, (0x6332, false)), // East Asian ideograph
    (0x216F3A, (0x546D, false)), // East Asian ideograph
    (0x6F566C, (0xC778, false)), // Korean hangul
    (0x282458, (0x5D03, false)), // East Asian ideograph
    (0x216F3B, (0x5491, false)), // East Asian ideograph
    (0x6F5229, (0xBE80, false)), // Korean hangul
    (0x4D5F7B, (0x97F2, false)), // East Asian ideograph
    (0x222F3D, (0x6201, false)), // East Asian ideograph
    (0x295C49, (0x9E47, false)), // East Asian ideograph
    (0x4B577E, (0x7E7F, false)), // East Asian ideograph
    (0x217C41, (0x5AC4, false)), // East Asian ideograph
    (0x215A28, (0x8CC2, false)), // East Asian ideograph
    (0x697265, (0x9C5A, false)), // East Asian ideograph
    (0x216F42, (0x5494, false)), // East Asian ideograph
    (0x232F43, (0x8915, false)), // East Asian ideograph
    (0x333344, (0x51DB, false)), // East Asian ideograph
    (0x6F566E, (0xC77D, false)), // Korean hangul
    (0x274340, (0x6656, false)), // East Asian ideograph
    (0x6F522B, (0xBE8C, false)), // Korean hangul
    (0x213C53, (0x5E36, false)), // East Asian ideograph
    (0x4C5175, (0x8315, false)), // East Asian ideograph
    (0x225E37, (0x75A2, false)), // East Asian ideograph
    (0x222F47, (0x6214, false)), // East Asian ideograph
    (0x2D3921, (0x591F, false)), // East Asian ideograph
    (0x233345, (0x8AE2, false)), // East Asian ideograph
    (0x216F49, (0x548D, false)), // East Asian ideograph
    (0x355D5C, (0x8C8E, false)), // East Asian ideograph
    (0x6F566F, (0xC783, false)), // Korean hangul
    (0x216F4A, (0x5463, false)), // East Asian ideograph
    (0x6F522C, (0xBE8F, false)), // Korean hangul
    (0x6F5737, (0xC808, false)), // Korean hangul
    (0x225160, (0x70F6, false)), // East Asian ideograph
    (0x6F5670, (0xC784, false)), // Korean hangul
    (0x234453, (0x931D, false)), // East Asian ideograph
    (0x216F7B, (0x551A, false)), // East Asian ideograph
    (0x6F5D68, (0xD744, false)), // Korean hangul
    (0x6F5671, (0xC785, false)), // Korean hangul
    (0x6F522E, (0xBE91, false)), // Korean hangul
    (0x294427, (0x94D7, false)), // East Asian ideograph
    (0x234454, (0x92FA, false)), // East Asian ideograph
    (0x2D3D67, (0x9015, false)), // East Asian ideograph
    (0x6F4A6E, (0xAF41, false)), // Korean hangul
    (0x222F56, (0x6223, false)), // East Asian ideograph
    (0x6F4F43, (0xB8E8, false)), // Korean hangul
    (0x6F5D2F, (0xD608, false)), // Korean hangul
    (0x335561, (0x8462, false)), // East Asian ideograph
    (0x216F58, (0x54A1, false)), // East Asian ideograph
    (0x6F5672, (0xC787, false)), // Korean hangul
    (0x274344, (0x6682, false)), // East Asian ideograph
    (0x6F522F, (0xBE98, false)), // Korean hangul
    (0x213C57, (0x5E3D, false)), // East Asian ideograph
    (0x4B356A, (0x55EC, false)), // East Asian ideograph
    (0x23223C, (0x83F3, false)), // East Asian ideograph
    (0x696F5B, (0x958A, false)), // East Asian ideograph
    (0x273238, (0x4FA6, false)), // East Asian ideograph
    (0x695C4F, (0x69DD, false)), // East Asian ideograph
    (0x222F5D, (0x6224, false)), // East Asian ideograph
    (0x6F5673, (0xC788, false)), // Korean hangul
    (0x216F5E, (0x54BE, false)), // East Asian ideograph
    (0x6F5230, (0xBEA8, false)), // Korean hangul
    (0x213C58, (0x5E40, false)), // East Asian ideograph
    (0x692433, (0x3053, false)), // Hiragana letter KO
    (0x292F60, (0x88E2, false)), // East Asian ideograph
    (0x6F5066, (0xBC0B, false)), // Korean hangul
    (0x4C2F61, (0x622C, false)), // East Asian ideograph
    (0x4B4235, (0x6442, false)), // East Asian ideograph
    (0x6F5B7B, (0xD338, false)), // Korean hangul
    (0x6F5C32, (0xD3AB, false)), // Korean hangul
    (0x6F5231, (0xBED0, false)), // Korean hangul
    (0x213C59, (0x5E4C, false)), // East Asian ideograph
    (0x216F64, (0x54B5, false)), // East Asian ideograph
    (0x225E2E, (0x7594, false)), // East Asian ideograph
    (0x6F5738, (0xC80A, false)), // Korean hangul
    (0x2D3F24, (0x661A, false)), // East Asian ideograph
    (0x226F66, (0x7CCE, false)), // East Asian ideograph
    (0x4B4236, (0x643A, false)), // East Asian ideograph
    (0x6F5A30, (0xCF04, false)), // Korean hangul
    (0x2D4A34, (0x718F, false)), // East Asian ideograph
    (0x226F68, (0x7CC8, false)), // East Asian ideograph
    (0x6F5A28, (0xCEF4, false)), // Korean hangul
    (0x6F5232, (0xBED1, false)), // Korean hangul
    (0x222F69, (0x97EF, false)), // East Asian ideograph
    (0x333428, (0x523C, false)), // East Asian ideograph
    (0x6F5676, (0xC78E, false)), // Korean hangul
    (0x216F6D, (0x54AE, false)), // East Asian ideograph
    (0x6F5233, (0xBED4, false)), // Korean hangul
    (0x2D3D6C, (0x5F93, false)), // East Asian ideograph
    (0x6F4A6F, (0xAF42, false)), // Korean hangul
    (0x232F6F, (0x894F, false)), // East Asian ideograph
    (0x2D5B42, (0x8F19, false)), // East Asian ideograph
    (0x393E7D, (0x7609, false)), // East Asian ideograph
    (0x695C53, (0x6A2E, false)), // East Asian ideograph
    (0x225167, (0x70EF, false)), // East Asian ideograph
    (0x235566, (0x9B23, false)), // East Asian ideograph
    (0x216F71, (0x54BF, false)), // East Asian ideograph
    (0x6F5677, (0xC655, false)), // Korean hangul
    (0x292F72, (0x88E5, false)), // East Asian ideograph
    (0x6F5234, (0xBED7, false)), // Korean hangul
    (0x213C5C, (0x5E57, false)), // East Asian ideograph
    (0x4D2F73, (0x7E5D, false)), // East Asian ideograph
    (0x6F4F4C, (0xB93C, false)), // Korean hangul
    (0x2D5B43, (0x8EFD, false)), // East Asian ideograph
    (0x226F75, (0x7CD7, false)), // East Asian ideograph
    (0x225168, (0x7100, false)), // East Asian ideograph
    (0x33334E, (0x51FE, false)), // East Asian ideograph
    (0x225A21, (0x7428, false)), // East Asian ideograph
    (0x215A22, (0x8CC7, false)), // East Asian ideograph
    (0x23445B, (0x9306, false)), // East Asian ideograph
    (0x215A23, (0x8CCA, false)), // East Asian ideograph
    (0x6F536C, (0xC274, false)), // Korean hangul
    (0x4B312D, (0x4F2B, false)), // East Asian ideograph
    (0x235A24, (0x9D04, false)), // East Asian ideograph
    (0x21322A, (0x5003, false)), // East Asian ideograph
    (0x222F7A, (0x6250, false)), // East Asian ideograph
    (0x215A25, (0x8CC4, false)), // East Asian ideograph
    (0x6F5D60, (0xD71C, false)), // Korean hangul
    (0x335568, (0x8406, false)), // East Asian ideograph
    (0x226F7B, (0x7CE8, false)), // East Asian ideograph (variant of 4C6F7B which maps to 7CE8)
    (0x6F5B7C, (0xD339, false)), // Korean hangul
    (0x275A26, (0x8D40, false)), // East Asian ideograph
    (0x6F5679, (0xC790, false)), // Korean hangul
    (0x2E2F7C, (0x634D, false)), // East Asian ideograph
    (0x215A27, (0x8CC3, false)), // East Asian ideograph
    (0x213C5E, (0x5E63, false)), // East Asian ideograph
    (0x276121, (0x998A, false)), // East Asian ideograph
    (0x2D6F7D, (0x8123, false)), // East Asian ideograph
    (0x235A28, (0x9D07, false)), // East Asian ideograph
    (0x2D4472, (0x68CA, false)), // East Asian ideograph
    (0x215A29, (0x8CD3, false)), // East Asian ideograph
    (0x215A2A, (0x8CD1, false)), // East Asian ideograph
    (0x217D2E, (0x5B0D, false)), // East Asian ideograph
    (0x215A2B, (0x8CD2, false)), // East Asian ideograph
    (0x395063, (0x9939, false)), // East Asian ideograph
    (0x6F567A, (0xC791, false)), // Korean hangul
    (0x275A2C, (0x8D54, false)), // East Asian ideograph
    (0x23445D, (0x92F9, false)), // East Asian ideograph
    (0x215A2D, (0x8CE6, false)), // East Asian ideograph
    (0x295C57, (0x9E6B, false)), // East Asian ideograph
    (0x215A2F, (0x8CE3, false)), // East Asian ideograph
    (0x213076, (0x4ED9, false)), // East Asian ideograph
    (0x215A30, (0x8CE2, false)), // East Asian ideograph
    (0x4B3C38, (0x949C, false)), // East Asian ideograph
    (0x275A31, (0x8D31, false)), // East Asian ideograph
    (0x276123, (0x9992, false)), // East Asian ideograph
    (0x225A32, (0x743B, false)), // East Asian ideograph
    (0x4B3130, (0x4FAB, false)), // East Asian ideograph
    (0x6F4A70, (0xAF43, false)), // Korean hangul
    (0x215A33, (0x8CDC, false)), // East Asian ideograph
    (0x275A34, (0x8D28, false)), // East Asian ideograph
    (0x215A35, (0x8CED, false)), // East Asian ideograph
    (0x2D4A3B, (0x4E89, false)), // East Asian ideograph
    (0x225A36, (0x7444, false)), // East Asian ideograph
    (0x275A37, (0x8D5B, false)), // East Asian ideograph
    (0x215A38, (0x8CFA, false)), // East Asian ideograph
    (0x215A39, (0x8D05, false)), // East Asian ideograph
    (0x293A40, (0x8DF8, false)), // East Asian ideograph
    (0x23556C, (0x9B29, false)), // East Asian ideograph
    (0x215A3A, (0x8CFC, false)), // East Asian ideograph
    (0x215A3B, (0x8D08, false)), // East Asian ideograph (variant of 4B5A3B which maps to 8D08)
    (0x6F5352, (0xC1F0, false)), // Korean hangul
    (0x215A3C, (0x8D0B, false)), // East Asian ideograph
    (0x215A3D, (0x8D0A, false)), // East Asian ideograph
    (0x215A3E, (0x8D0F, false)), // East Asian ideograph
    (0x6F5B7D, (0xD33B, false)), // Korean hangul
    (0x215A3F, (0x8D0D, false)), // East Asian ideograph
    (0x6F567E, (0xC7A0, false)), // Korean hangul
    (0x213D40, (0x5F1B, false)), // East Asian ideograph
    (0x215A40, (0x8D13, false)), // East Asian ideograph
    (0x276126, (0x990D, false)), // East Asian ideograph
    (0x215A41, (0x8D16, false)), // East Asian ideograph
    (0x215A42, (0x8D1B, false)), // East Asian ideograph
    (0x295C5B, (0x9E6C, false)), // East Asian ideograph
    (0x225A43, (0x7458, false)), // East Asian ideograph
    (0x235A44, (0x9D1D, false)), // East Asian ideograph
    (0x225A45, (0x7442, false)), // East Asian ideograph
    (0x6F5A46, (0xCF71, false)), // Korean hangul
    (0x2D3D75, (0x60EA, false)), // East Asian ideograph
    (0x2E4174, (0x6AA9, false)), // East Asian ideograph
    (0x225A47, (0x744B, false)), // East Asian ideograph
    (0x215A48, (0x8D70, false)), // East Asian ideograph
    (0x6F5660, (0xC737, false)), // Korean hangul
    (0x337345, (0x9F67, false)), // East Asian ideograph
    (0x6F5A49, (0xCF80, false)), // Korean hangul
    (0x225A4A, (0x744A, false)), // East Asian ideograph
    (0x213C65, (0x5E74, false)), // East Asian ideograph
    (0x6F5A4B, (0xCF8C, false)), // Korean hangul
    (0x6F5938, (0xCC71, false)), // Korean hangul
    (0x2D3D76, (0x5FB4, false)), // East Asian ideograph
    (0x2D5476, (0x8318, false)), // East Asian ideograph
    (0x6F5A4C, (0xCF8D, false)), // Korean hangul
    (0x6F5573, (0xC628, false)), // Korean hangul
    (0x6F5A4D, (0xCFA1, false)), // Korean hangul
    (0x2D5A4E, (0x8D82, false)), // East Asian ideograph
    (0x215A4F, (0x8D99, false)), // East Asian ideograph
    (0x4B3864, (0x58C7, false)), // East Asian ideograph
    (0x275A50, (0x8D76, false)), // East Asian ideograph
    (0x2D392F, (0x7287, false)), // East Asian ideograph
    (0x6F5A51, (0xCFE0, false)), // Korean hangul
    (0x704C2A, (0x915E, false)), // East Asian ideograph
    (0x224E37, (0x6FAF, false)), // East Asian ideograph
    (0x6F5A52, (0xCFE1, false)), // Korean hangul
    (0x6F5C58, (0xD515, false)), // Korean hangul
    (0x215A53, (0x8DA8, false)), // East Asian ideograph
    (0x6F492E, (0xAC86, false)), // Korean hangul
    (0x6F537D, (0xC2B4, false)), // Korean hangul
    (0x6F523F, (0xBF40, false)), // Korean hangul
    (0x225A55, (0x7457, false)), // East Asian ideograph
    (0x225A56, (0x7451, false)), // East Asian ideograph
    (0x6F5069, (0xBC0F, false)), // Korean hangul
    (0x6F5A57, (0xCFF5, false)), // Korean hangul
    (0x293A46, (0x8E70, false)), // East Asian ideograph
    (0x23512F, (0x9916, false)), // East Asian ideograph
    (0x4B3642, (0x8BF6, false)), // East Asian ideograph
    (0x2E4E41, (0x7032, false)), // East Asian ideograph
    (0x215A59, (0x8DDB, false)), // East Asian ideograph
    (0x6F5240, (0xBF41, false)), // Korean hangul
    (0x706131, (0x5C9C, false)), // East Asian ideograph
    (0x4B357B, (0x54CC, false)), // East Asian ideograph
    (0x225A5A, (0x745D, false)), // East Asian ideograph
    (0x225A5B, (0x7454, false)), // East Asian ideograph
    (0x225174, (0x7131, false)), // East Asian ideograph
    (0x235573, (0x9B2D, false)), // East Asian ideograph
    (0x235130, (0x9914, false)), // East Asian ideograph
    (0x33386E, (0x576F, false)), // East Asian ideograph
    (0x6F5A5E, (0xD038, false)), // Korean hangul
    (0x6F5241, (0xBF44, false)), // Korean hangul
    (0x2D5A5F, (0x8E5F, false)), // East Asian ideograph
    (0x6F5949, (0xCD09, false)), // Korean hangul
    (0x225A60, (0x746D, false)), // East Asian ideograph
    (0x277239, (0x54D4, false)), // East Asian ideograph
    (0x225A61, (0x7462, false)), // East Asian ideograph
    (0x235574, (0x9B2E, false)), // East Asian ideograph (not in Unicode)
    (0x29506C, (0x996B, false)), // East Asian ideograph
    (0x6F532F, (0xC131, false)), // Korean hangul
    (0x215A62, (0x8DF3, false)), // East Asian ideograph
    (0x215A63, (0x8DFA, false)), // East Asian ideograph
    (0x6F5242, (0xBF48, false)), // Korean hangul
    (0x27612D, (0x51AF, false)), // East Asian ideograph
    (0x6F5A64, (0xD07D, false)), // Korean hangul
    (0x4C796B, (0x815F, false)), // East Asian ideograph
    (0x235A65, (0x9D30, false)), // East Asian ideograph
    (0x275021, (0x7B0B, false)), // East Asian ideograph
    (0x235132, (0x9911, false)), // East Asian ideograph
    (0x2F3B63, (0x5E32, false)), // East Asian ideograph (not in Unicode)
    (0x2D4A45, (0x5C12, false)), // East Asian ideograph
    (0x275A68, (0x8DF5, false)), // East Asian ideograph
    (0x6F5243, (0xBF50, false)), // Korean hangul
    (0x213C6B, (0x5E7E, false)), // East Asian ideograph
    (0x215A69, (0x8E22, false)), // East Asian ideograph
    (0x225A6A, (0x7471, false)), // East Asian ideograph
    (0x225A6B, (0x7468, false)), // East Asian ideograph
    (0x4B5936, (0x8B20, false)), // East Asian ideograph
    (0x4D5A6C, (0x9D46, false)), // East Asian ideograph
    (0x216035, (0x97FB, false)), // East Asian ideograph
    (0x2D4A46, (0x58BB, false)), // East Asian ideograph
    (0x6F5A6D, (0xD0B9, false)), // Korean hangul
    (0x4B4857, (0x6F22, false)), // East Asian ideograph
    (0x235A70, (0x9D5C, false)), // East Asian ideograph
    (0x224D35, (0x6F90, false)), // East Asian ideograph
    (0x282951, (0x5F2A, false)), // East Asian ideograph
    (0x275A71, (0x8E0A, false)), // East Asian ideograph
    (0x6F5A72, (0xD0C4, false)), // Korean hangul
    (0x6F5245, (0xBF55, false)), // Korean hangul
    (0x276130, (0x9A6E, false)), // East Asian ideograph
    (0x6F4C27, (0xB141, false)), // Korean hangul
    (0x695A73, (0x6683, false)), // East Asian ideograph
    (0x454B7A, (0x7523, false)), // East Asian ideograph
    (0x6F5A74, (0xD0C9, false)), // Korean hangul
    (0x2D377C, (0x962C, false)), // East Asian ideograph
    (0x215A75, (0x8E4B, false)), // East Asian ideograph
    (0x295822, (0x9CAE, false)), // East Asian ideograph
    (0x6F5A76, (0xD0D1, false)), // Korean hangul
    (0x6F5A77, (0xD0D3, false)), // Korean hangul
    (0x6F5246, (0xBFB0, false)), // Korean hangul
    (0x234522, (0x9314, false)), // East Asian ideograph
    (0x225A78, (0x7460, false)), // East Asian ideograph
    (0x225A79, (0x7472, false)), // East Asian ideograph
    (0x225A7A, (0x7484, false)), // East Asian ideograph
    (0x224525, (0x6B99, false)), // East Asian ideograph
    (0x224D37, (0x6F8D, false)), // East Asian ideograph
    (0x225A7B, (0x7487, false)), // East Asian ideograph
    (0x274526, (0x6781, false)), // East Asian ideograph
    (0x6F5A7C, (0xD0E0, false)), // Korean hangul
    (0x6F5247, (0xBFC0, false)), // Korean hangul
    (0x276132, (0x9A73, false)), // East Asian ideograph
    (0x6F5A7D, (0xD0E4, false)), // Korean hangul
    (0x234528, (0x92FE, false)), // East Asian ideograph
    (0x215A7E, (0x8E7A, false)), // East Asian ideograph
    (0x224529, (0x6B9B, false)), // East Asian ideograph
    (0x27452A, (0x6768, false)), // East Asian ideograph
    (0x3F4C3C, (0x7582, false)), // East Asian ideograph
    (0x27452B, (0x6862, false)), // East Asian ideograph
    (0x2E7062, (0x7D4F, false)), // East Asian ideograph
    (0x6F5248, (0xBFC5, false)), // Korean hangul
    (0x276133, (0x9A7B, false)), // East Asian ideograph
    (0x4B3B61, (0x5C64, false)), // East Asian ideograph
    (0x69242B, (0x304B, false)), // Hiragana letter KA
    (0x27452D, (0x4E1A, false)), // East Asian ideograph
    (0x2D3931, (0x67F0, false)), // East Asian ideograph
    (0x2D7E6A, (0x51A4, false)), // East Asian ideograph
    (0x27452F, (0x67AB, false)), // East Asian ideograph
    (0x6F5C5A, (0xD53D, false)), // Korean hangul
    (0x224D39, (0x6F92, false)), // East Asian ideograph
    (0x692434, (0x3054, false)), // Hiragana letter GO
    (0x6F5249, (0xBFCC, false)), // Korean hangul
    (0x234531, (0x9341, false)), // East Asian ideograph
    (0x217C60, (0x5ADC, false)), // East Asian ideograph
    (0x273764, (0x5631, false)), // East Asian ideograph
    (0x234532, (0x9319, false)), // East Asian ideograph
    (0x6F506B, (0xBBB4, false)), // Korean hangul
    (0x4B4534, (0x6994, false)), // East Asian ideograph (variant of 214534)
    (0x224D3A, (0x6F89, false)), // East Asian ideograph
    (0x234535, (0x934C, false)), // East Asian ideograph
    (0x6F524A, (0xBFCD, false)), // Korean hangul
    (0x224536, (0x6BA2, false)), // East Asian ideograph
    (0x6F4C28, (0xB144, false)), // Korean hangul
    (0x21382F, (0x577C, false)), // East Asian ideograph
    (0x274537, (0x8363, false)), // East Asian ideograph
    (0x224538, (0x6BAA, false)), // East Asian ideograph
    (0x274539, (0x6784, false)), // East Asian ideograph
    (0x235B6A, (0x9DA1, false)), // East Asian ideograph
    (0x2D453A, (0x6760, false)), // East Asian ideograph
    (0x22453B, (0x6BAD, false)), // East Asian ideograph
    (0x234471, (0x9340, false)), // East Asian ideograph
    (0x692531, (0x30B1, false)), // Katakana letter KE
    (0x22453D, (0x6BB0, false)), // East Asian ideograph
    (0x6F4F66, (0xB9C8, false)), // Korean hangul
    (0x6F5663, (0xC740, false)), // Korean hangul
    (0x274871, (0x6D47, false)), // East Asian ideograph
    (0x224D3C, (0x6F8C, false)), // East Asian ideograph
    (0x22453F, (0x6BB3, false)), // East Asian ideograph
    (0x274540, (0x67AA, false)), // East Asian ideograph
    (0x234541, (0x9379, false)), // East Asian ideograph
    (0x4B3144, (0x4F36, false)), // East Asian ideograph (variant of 213144 which maps to 4F36)
    (0x295C6C, (0x9E6A, false)), // East Asian ideograph
    (0x2D4543, (0x6901, false)), // East Asian ideograph
    (0x224D3D, (0x6F62, false)), // East Asian ideograph (variant of 4C4D3D which maps to 6F62)
    (0x33513C, (0x7D4C, false)), // East Asian ideograph
    (0x214544, (0x6A23, false)), // East Asian ideograph
    (0x2D5036, (0x84D1, false)), // East Asian ideograph
    (0x6F524D, (0xBFDC, false)), // Korean hangul
    (0x234A5E, (0x967F, false)), // East Asian ideograph
    (0x223553, (0x6509, false)), // East Asian ideograph
    (0x4B7577, (0x57D3, false)), // East Asian ideograph
    (0x225E4A, (0x75C2, false)), // East Asian ideograph
    (0x214546, (0x6A01, false)), // East Asian ideograph
    (0x2D3932, (0x7AD2, false)), // East Asian ideograph
    (0x274547, (0x6807, false)), // East Asian ideograph
    (0x234548, (0x935C, false)), // East Asian ideograph
    (0x6F5C5B, (0xD540, false)), // Korean hangul
    (0x216037, (0x9801, false)), // East Asian ideograph
    (0x274549, (0x67A2, false)), // East Asian ideograph
    (0x6F524E, (0xBFDD, false)), // Korean hangul
    (0x27454A, (0x697C, false)), // East Asian ideograph
    (0x213833, (0x57AE, false)), // East Asian ideograph
    (0x4B602D, (0x9771, false)), // East Asian ideograph
    (0x2D5B5D, (0x8FA2, false)), // East Asian ideograph
    (0x2D3944, (0x511E, false)), // East Asian ideograph
    (0x27454C, (0x6868, false)), // East Asian ideograph
    (0x23454D, (0x9347, false)), // East Asian ideograph
    (0x293373, (0x8C21, false)), // East Asian ideograph
    (0x27454E, (0x4E50, false)), // East Asian ideograph
    (0x6F524F, (0xBFE1, false)), // Korean hangul
    (0x27454F, (0x679E, false)), // East Asian ideograph
    (0x2D3F2A, (0x5ABF, false)), // East Asian ideograph
    (0x2D4550, (0x58AB, false)), // East Asian ideograph
    (0x2D5B5E, (0x8FA7, false)), // East Asian ideograph
    (0x234551, (0x937A, false)), // East Asian ideograph
    (0x29582C, (0x9CB1, false)), // East Asian ideograph
    (0x274553, (0x692D, false)), // East Asian ideograph
    (0x27767A, (0x57DA, false)), // East Asian ideograph
    (0x224554, (0x6BC8, false)), // East Asian ideograph
    (0x274555, (0x6811, false)), // East Asian ideograph
    (0x234556, (0x937C, false)), // East Asian ideograph
    (0x293A57, (0x8DFB, false)), // East Asian ideograph
    (0x274557, (0x6866, false)), // East Asian ideograph
    (0x29582D, (0x9CB7, false)), // East Asian ideograph
    (0x235140, (0x991C, false)), // East Asian ideograph
    (0x274558, (0x6734, false)), // East Asian ideograph
    (0x474E5C, (0x97DE, false)), // East Asian ideograph (variant of 234E5C which maps to 97DE)
    (0x274366, (0x80E7, false)), // East Asian ideograph
    (0x6F5251, (0xC059, false)), // Korean hangul
    (0x27613C, (0x9A86, false)), // East Asian ideograph
    (0x2D3261, (0x5039, false)), // East Asian ideograph
    (0x27455B, (0x6865, false)), // East Asian ideograph
    (0x275030, (0x8303, false)), // East Asian ideograph
    (0x23455C, (0x9377, false)), // East Asian ideograph
    (0x6F5172, (0xBE4E, false)), // Korean hangul
    (0x27455D, (0x673A, false)), // East Asian ideograph
    (0x213F50, (0x61CA, false)), // East Asian ideograph
    (0x6F5252, (0xC05C, false)), // Korean hangul
    (0x23455E, (0x9358, false)), // East Asian ideograph
    (0x2D5F63, (0x873A, false)), // East Asian ideograph
    (0x27455F, (0x6863, false)), // East Asian ideograph
    (0x224560, (0x6BDA, false)), // East Asian ideograph
    (0x33327A, (0x5150, false)), // East Asian ideograph
    (0x274561, (0x68C0, false)), // East Asian ideograph
    (0x29582F, (0x9CB5, false)), // East Asian ideograph
    (0x274562, (0x6867, false)), // East Asian ideograph
    (0x3F347D, (0x84E1, false)), // East Asian ideograph
    (0x6F5253, (0xC060, false)), // Korean hangul
    (0x274563, (0x67E0, false)), // East Asian ideograph
    (0x235131, (0x9917, false)), // East Asian ideograph
    (0x234564, (0x9376, false)), // East Asian ideograph
    (0x274565, (0x67DC, false)), // East Asian ideograph
    (0x213230, (0x5065, false)), // East Asian ideograph
    (0x224A4C, (0x6E1F, false)), // East Asian ideograph
    (0x274566, (0x69DB, false)), // East Asian ideograph
    (0x294567, (0x9537, false)), // East Asian ideograph
    (0x6F5254, (0xC068, false)), // Korean hangul
    (0x27613F, (0x9A88, false)), // East Asian ideograph
    (0x6F4C2A, (0xB151, false)), // Korean hangul
    (0x2D4569, (0x6AC9, false)), // East Asian ideograph
    (0x4B314C, (0x5F95, false)), // East Asian ideograph
    (0x6F573F, (0xC81C, false)), // Korean hangul
    (0x23456A, (0x9348, false)), // East Asian ideograph
    (0x27325D, (0x4EF7, false)), // East Asian ideograph
    (0x27456B, (0x691F, false)), // East Asian ideograph
    (0x213F3B, (0x616B, false)), // East Asian ideograph
    (0x295831, (0x9CB6, false)), // East Asian ideograph
    (0x27456C, (0x6809, false)), // East Asian ideograph
    (0x2E4E56, (0x9800, false)), // East Asian ideograph
    (0x6F5255, (0xC069, false)), // Korean hangul
    (0x27456D, (0x6A79, false)), // East Asian ideograph
    (0x276140, (0x9A91, false)), // East Asian ideograph
    (0x23447B, (0x933F, false)), // East Asian ideograph
    (0x27456E, (0x680F, false)), // East Asian ideograph
    (0x27456F, (0x6A31, false)), // East Asian ideograph
    (0x224570, (0x6BEA, false)), // East Asian ideograph
    (0x3F456D, (0x8263, false)), // East Asian ideograph
    (0x235145, (0x9927, false)), // East Asian ideograph
    (0x274571, (0x6984, false)), // East Asian ideograph
    (0x2D4A58, (0x7F9D, false)), // East Asian ideograph
    (0x6F5256, (0xC090, false)), // Korean hangul
    (0x217C6D, (0x5AE5, false)), // East Asian ideograph
    (0x286540, (0x7800, false)), // East Asian ideograph
    (0x23447C, (0x933A, false)), // East Asian ideograph
    (0x234573, (0x9360, false)), // East Asian ideograph
    (0x2D4574, (0x5FFB, false)), // East Asian ideograph
    (0x6F5D37, (0xD639, false)), // Korean hangul
    (0x233021, (0x894D, false)), // East Asian ideograph
    (0x6F5257, (0xC091, false)), // Korean hangul
    (0x234577, (0x936E, false)), // East Asian ideograph
    (0x287022, (0x7CC1, false)), // East Asian ideograph
    (0x274578, (0x94A6, false)), // East Asian ideograph
    (0x6F5844, (0xC9F8, false)), // Korean hangul
    (0x213023, (0x4E03, false)), // East Asian ideograph
    (0x225B2D, (0x7486, false)), // East Asian ideograph
    (0x234579, (0x938F, false)), // East Asian ideograph
    (0x233024, (0x895A, false)), // East Asian ideograph
    (0x293A5E, (0x8DF9, false)), // East Asian ideograph
    (0x23457A, (0x93AC, false)), // East Asian ideograph
    (0x6F5C5D, (0xD54C, false)), // Korean hangul
    (0x233025, (0x895E, false)), // East Asian ideograph
    (0x335147, (0x7EEE, false)), // East Asian ideograph
    (0x23457B, (0x9395, false)), // East Asian ideograph
    (0x213026, (0x4E0A, false)), // East Asian ideograph
    (0x4B4E7B, (0x7985, false)), // East Asian ideograph (variant of 274E7B which maps to 7985)
    (0x6F5258, (0xC094, false)), // Korean hangul
    (0x27457C, (0x6B27, false)), // East Asian ideograph
    (0x295175, (0x9993, false)), // East Asian ideograph
    (0x21383D, (0x57DF, false)), // East Asian ideograph
    (0x2E3028, (0x640B, false)), // East Asian ideograph
    (0x27457E, (0x6B24, false)), // East Asian ideograph
    (0x213029, (0x4E10, false)), // East Asian ideograph
    (0x293A5F, (0x8DDE, false)), // East Asian ideograph
    (0x2D4A5B, (0x7282, false)), // East Asian ideograph
    (0x22302B, (0x625A, false)), // East Asian ideograph
    (0x6F5259, (0xC098, false)), // Korean hangul
    (0x276144, (0x817E, false)), // East Asian ideograph
    (0x21302C, (0x4E19, false)), // East Asian ideograph
    (0x21383E, (0x580A, false)), // East Asian ideograph
    (0x22302D, (0x6266, false)), // East Asian ideograph
    (0x22702E, (0x7CF0, false)), // East Asian ideograph
    (0x6F5664, (0xC744, false)), // Korean hangul
    (0x293A60, (0x8E2C, false)), // East Asian ideograph
    (0x21302F, (0x4E18, false)), // East Asian ideograph
    (0x217030, (0x5504, false)), // East Asian ideograph
    (0x6F525A, (0xC0A0, false)), // Korean hangul
    (0x234469, (0x9338, false)), // East Asian ideograph
    (0x692126, (0x30FB, false)), // Ideographic centered point
    (0x223031, (0x6286, false)), // East Asian ideograph
    (0x21383F, (0x5805, false)), // East Asian ideograph
    (0x273032, (0x5E76, false)), // East Asian ideograph
    (0x6F594A, (0xCD0C, false)), // Korean hangul
    (0x2D5B69, (0x5EF5, false)), // East Asian ideograph
    (0x6F4C55, (0xB284, false)), // Korean hangul
    (0x275039, (0x7B03, false)), // East Asian ideograph
    (0x6F5666, (0xC74C, false)), // Korean hangul
    (0x692139, (0x3005, false)), // Ideographic iteration mark
    (0x69213C, (0x30FC, false)), // Vowel elongation mark for kana
    (0x213035, (0x4E32, false)), // East Asian ideograph
    (0x695130, (0x5116, false)), // East Asian ideograph
    (0x6F525B, (0xC0A3, false)), // Korean hangul
    (0x276146, (0x9AA0, false)), // East Asian ideograph
    (0x217C72, (0x5AEA, false)), // East Asian ideograph
    (0x6F4A77, (0xAF64, false)), // Korean hangul
    (0x23403E, (0x9146, false)), // East Asian ideograph
    (0x2D3263, (0x4FAD, false)), // East Asian ideograph
    (0x4B4F29, (0x7A50, false)), // East Asian ideograph
    (0x692152, (0x3008, false)), // Ideographic less than sign
    (0x27503A, (0x7B51, false)), // East Asian ideograph
    (0x692154, (0x300A, false)), // Ideographic left double angle bracket
    (0x692155, (0x300B, false)), // Ideographic right double angle bracket
    (0x217039, (0x54F7, false)), // East Asian ideograph
    (0x21303A, (0x4E43, false)), // East Asian ideograph
    (0x2E4E5D, (0x6DE0, false)), // East Asian ideograph
    (0x6F525C, (0xC0A5, false)), // Korean hangul
    (0x21303B, (0x4E45, false)), // East Asian ideograph
    (0x213841, (0x5806, false)), // East Asian ideograph
    (0x217830, (0x58A3, false)), // East Asian ideograph
    (0x6F576A, (0xC8F1, false)), // Korean hangul
    (0x233376, (0x8B1F, false)), // East Asian ideograph
    (0x33514C, (0x7DD1, false)), // East Asian ideograph
    (0x213E21, (0x5FCD, false)), // East Asian ideograph
    (0x276148, (0x84E6, false)), // East Asian ideograph
    (0x2D4B71, (0x7F3E, false)), // East Asian ideograph
    (0x223041, (0x62A3, false)), // East Asian ideograph
    (0x213042, (0x4E52, false)), // East Asian ideograph
    (0x277255, (0x54D3, false)), // East Asian ideograph
    (0x213043, (0x4E53, false)), // East Asian ideograph
    (0x227044, (0x7D03, false)), // East Asian ideograph
    (0x234544, (0x9386, false)), // East Asian ideograph
    (0x287045, (0x7EA8, false)), // East Asian ideograph
    (0x227C31, (0x82AF, false)), // East Asian ideograph
    (0x234041, (0x9147, false)), // East Asian ideograph
    (0x2D3954, (0x7385, false)), // East Asian ideograph
    (0x213047, (0x4E5D, false)), // East Asian ideograph
    (0x213049, (0x4E5E, false)), // East Asian ideograph
    (0x6F525F, (0xC0AC, false)), // Korean hangul
    (0x28255A, (0x5D5D, false)), // East Asian ideograph
    (0x273F31, (0x60ED, false)), // East Asian ideograph
    (0x225E5C, (0x75CF, false)), // East Asian ideograph
    (0x29472F, (0x94E9, false)), // East Asian ideograph
    (0x21304B, (0x4E73, false)), // East Asian ideograph
    (0x21304C, (0x4E7E, false)), // East Asian ideograph
    (0x27503E, (0x7BD3, false)), // East Asian ideograph
    (0x6F5667, (0xC74D, false)), // Korean hangul
    (0x27304D, (0x4E71, false)), // East Asian ideograph
    (0x23514F, (0x992E, false)), // East Asian ideograph
    (0x6F5870, (0xCBD4, false)), // Korean hangul
    (0x275B36, (0x8F69, false)), // East Asian ideograph
    (0x6F5260, (0xC0AD, false)), // Korean hangul
    (0x27614B, (0x60CA, false)), // East Asian ideograph
    (0x6F4A78, (0xAF65, false)), // Korean hangul
    (0x227050, (0x7D18, false)), // East Asian ideograph
    (0x227051, (0x7D1E, false)), // East Asian ideograph
    (0x277258, (0x6076, false)), // East Asian ideograph (duplicate simplified)
    (0x393052, (0x65BC, false)), // East Asian ideograph
    (0x235150, (0x992C, false)), // East Asian ideograph
    (0x213053, (0x4E95, false)), // East Asian ideograph
    (0x6F5261, (0xC0AE, false)), // Korean hangul
    (0x294040, (0x90E6, false)), // East Asian ideograph
    (0x213055, (0x4E92, false)), // East Asian ideograph
    (0x2E5F6F, (0x75B8, false)), // East Asian ideograph
    (0x27326A, (0x4FEA, false)), // East Asian ideograph
    (0x223057, (0x62D1, false)), // East Asian ideograph
    (0x235151, (0x992A, false)), // East Asian ideograph
    (0x213058, (0x4E9E, false)), // East Asian ideograph
    (0x2D4621, (0x61FD, false)), // East Asian ideograph
    (0x213059, (0x4E9B, false)), // East Asian ideograph
    (0x284333, (0x680E, false)), // East Asian ideograph
    (0x294732, (0x94F4, false)), // East Asian ideograph
    (0x33625E, (0x9EAA, false)), // East Asian ideograph
    (0x22705A, (0x7D3D, false)), // East Asian ideograph
    (0x6F5070, (0xBC18, false)), // Korean hangul
    (0x232223, (0x83FD, false)), // East Asian ideograph
    (0x222224, (0x5BF0, false)), // East Asian ideograph
    (0x232225, (0x841E, false)), // East Asian ideograph
    (0x693C36, (0x96EB, false)), // East Asian ideograph
    (0x232229, (0x83C9, false)), // East Asian ideograph
    (0x23222A, (0x83DF, false)), // East Asian ideograph
    (0x23222C, (0x841F, false)), // East Asian ideograph
    (0x23222E, (0x840F, false)), // East Asian ideograph
    (0x69705D, (0x9786, false)), // East Asian ideograph
    (0x232230, (0x8411, false)), // East Asian ideograph
    (0x222233, (0x5C00, false)), // East Asian ideograph
    (0x222235, (0x5C57, false)), // East Asian ideograph
    (0x232236, (0x839A, false)), // East Asian ideograph
    (0x707438, (0x823E, false)), // East Asian ideograph
    (0x33625F, (0x8534, false)), // East Asian ideograph
    (0x21305F, (0x4EA8, false)), // East Asian ideograph
    (0x22223C, (0x5C15, false)), // East Asian ideograph
    (0x333060, (0x4EAF, false)), // East Asian ideograph
    (0x6F5742, (0xC824, false)), // Korean hangul
    (0x232243, (0x83D1, false)), // East Asian ideograph
    (0x275042, (0x7BAB, false)), // East Asian ideograph
    (0x222246, (0x5C22, false)), // East Asian ideograph
    (0x223061, (0x62E4, false)), // East Asian ideograph
    (0x222248, (0x5C25, false)), // East Asian ideograph
    (0x23224A, (0x848E, false)), // East Asian ideograph
    (0x22224B, (0x5C2A, false)), // East Asian ideograph
    (0x23224C, (0x8439, false)), // East Asian ideograph
    (0x23224D, (0x8476, false)), // East Asian ideograph
    (0x23224E, (0x8479, false)), // East Asian ideograph
    (0x213C6E, (0x5E8A, false)), // East Asian ideograph
    (0x222252, (0x5C2F, false)), // East Asian ideograph
    (0x217C7B, (0x5ADA, false)), // East Asian ideograph
    (0x284335, (0x6A7C, false)), // East Asian ideograph
    (0x4C3B22, (0x6860, false)), // East Asian ideograph
    (0x294734, (0x9566, false)), // East Asian ideograph
    (0x213064, (0x4EBA, false)), // East Asian ideograph
    (0x22225B, (0x5C32, false)), // East Asian ideograph
    (0x23225C, (0x8451, false)), // East Asian ideograph
    (0x23225F, (0x847D, false)), // East Asian ideograph
    (0x232262, (0x845A, false)), // East Asian ideograph
    (0x222263, (0x5C3B, false)), // East Asian ideograph
    (0x222265, (0x5C44, false)), // East Asian ideograph
    (0x232266, (0x8459, false)), // East Asian ideograph
    (0x222267, (0x5C49, false)), // East Asian ideograph
    (0x232269, (0x8473, false)), // East Asian ideograph
    (0x23226E, (0x843E, false)), // East Asian ideograph
    (0x6F5265, (0xC0B4, false)), // Korean hangul
    (0x276150, (0x9AA5, false)), // East Asian ideograph
    (0x232271, (0x846D, false)), // East Asian ideograph
    (0x394735, (0x6C3E, false)), // East Asian ideograph
    (0x223069, (0x62B6, false)), // East Asian ideograph
    (0x232278, (0x847A, false)), // East Asian ideograph
    (0x222279, (0x5C59, false)), // East Asian ideograph
    (0x22227B, (0x5C5D, false)), // East Asian ideograph
    (0x22227C, (0x5C5F, false)), // East Asian ideograph
    (0x22706A, (0x7D3F, false)), // East Asian ideograph
    (0x21306B, (0x4ECD, false)), // East Asian ideograph
    (0x2D306C, (0x8B8E, false)), // East Asian ideograph
    (0x6F5266, (0xC0B5, false)), // Korean hangul
    (0x276151, (0x9A8A, false)), // East Asian ideograph
    (0x284337, (0x6987, false)), // East Asian ideograph
    (0x225E63, (0x75E7, false)), // East Asian ideograph
    (0x33512E, (0x7E8D, false)), // East Asian ideograph
    (0x4B306E, (0x4EE4, false)), // East Asian ideograph (variant of 21306E which maps to 4EE4)
    (0x21306F, (0x4ED8, false)), // East Asian ideograph
    (0x235156, (0x992B, false)), // East Asian ideograph
    (0x454146, (0x63DB, false)), // East Asian ideograph
    (0x213071, (0x4ED6, false)), // East Asian ideograph
    (0x6F5267, (0xC0B6, false)), // Korean hangul
    (0x213072, (0x4EDE, false)), // East Asian ideograph
    (0x6F4E24, (0xB544, false)), // Korean hangul
    (0x6F5A24, (0xCEE4, false)), // Korean hangul
    (0x225254, (0x714F, false)), // East Asian ideograph
    (0x215B21, (0x8E76, false)), // East Asian ideograph
    (0x6F5268, (0xC0BC, false)), // Korean hangul
    (0x276153, (0x80AE, false)), // East Asian ideograph
    (0x213077, (0x4EE5, false)), // East Asian ideograph
    (0x215B22, (0x8E7C, false)), // East Asian ideograph
    (0x21384D, (0x5857, false)), // East Asian ideograph
    (0x333078, (0x5F77, false)), // East Asian ideograph
    (0x225D69, (0x756F, false)), // East Asian ideograph
    (0x335E21, (0x9221, false)), // East Asian ideograph
    (0x213079, (0x4F09, false)), // East Asian ideograph
    (0x6F5B24, (0xD0F1, false)), // Korean hangul
    (0x6F5B25, (0xD130, false)), // Korean hangul
    (0x235158, (0x9931, false)), // East Asian ideograph
    (0x4B3E2A, (0x6035, false)), // East Asian ideograph
    (0x275B26, (0x8DB8, false)), // East Asian ideograph
    (0x6F5269, (0xC0BD, false)), // Korean hangul
    (0x6F4B2C, (0xAFD4, false)), // Korean hangul
    (0x225B27, (0x7482, false)), // East Asian ideograph
    (0x21384E, (0x5858, false)), // East Asian ideograph
    (0x21307D, (0x4F0A, false)), // East Asian ideograph
    (0x215B28, (0x8E8A, false)), // East Asian ideograph
    (0x215B29, (0x8E8D, false)), // East Asian ideograph (variant of 4B5B29 which maps to 8E8D)
    (0x275048, (0x5E18, false)), // East Asian ideograph
    (0x275B2A, (0x8E2F, false)), // East Asian ideograph
    (0x4B6044, (0x9818, false)), // East Asian ideograph
    (0x4D4A6C, (0x9667, false)), // East Asian ideograph
    (0x275B2B, (0x8E51, false)), // East Asian ideograph
    (0x6F526A, (0xC0BF, false)), // Korean hangul
    (0x293B7A, (0x8F8F, false)), // East Asian ideograph
    (0x215A68, (0x8E10, false)), // East Asian ideograph
    (0x23404D, (0x9156, false)), // East Asian ideograph
    (0x215B2D, (0x8EAB, false)), // East Asian ideograph
    (0x235B2E, (0x9D8A, false)), // East Asian ideograph
    (0x3F5F34, (0x9699, false)), // East Asian ideograph (not in Unicode)
    (0x274224, (0x6361, false)), // East Asian ideograph
    (0x212320, (0x3000, false)), // Ideographic space in some implementations
    (0x215B30, (0x8EBA, false)), // East Asian ideograph
    (0x222323, (0x5C63, false)), // East Asian ideograph
    (0x232324, (0x8432, false)), // East Asian ideograph
    (0x225772, (0x735E, false)), // East Asian ideograph
    (0x212328, (0xFF08, false)), // Ideographic left parenthesis
    (0x222329, (0x5C67, false)), // East Asian ideograph
    (0x22232B, (0x5C68, false)), // East Asian ideograph
    (0x23232D, (0x842A, false)), // East Asian ideograph
    (0x23232E, (0x8429, false)), // East Asian ideograph
    (0x222330, (0x5C6D, false)), // East Asian ideograph
    (0x222331, (0x5C6E, false)), // East Asian ideograph
    (0x232332, (0x8471, false)), // East Asian ideograph
    (0x215B33, (0x8ECB, false)), // East Asian ideograph
    (0x232335, (0x845F, false)), // East Asian ideograph
    (0x232336, (0x8460, false)), // East Asian ideograph
    (0x222337, (0x5C74, false)), // East Asian ideograph
    (0x222339, (0x5C73, false)), // East Asian ideograph
    (0x23233A, (0x8446, false)), // East Asian ideograph
    (0x22233B, (0x5C77, false)), // East Asian ideograph
    (0x22233C, (0x5C7A, false)), // East Asian ideograph
    (0x29233D, (0x836E, false)), // East Asian ideograph
    (0x235B35, (0x9D87, false)), // East Asian ideograph
    (0x222340, (0x5C7C, false)), // East Asian ideograph
    (0x6F526C, (0xC0C1, false)), // Korean hangul
    (0x232345, (0x844E, false)), // East Asian ideograph
    (0x222346, (0x5C8F, false)), // East Asian ideograph
    (0x29473C, (0x9568, false)), // East Asian ideograph
    (0x222349, (0x5C88, false)), // East Asian ideograph
    (0x275B37, (0x8F6B, false)), // East Asian ideograph
    (0x22234D, (0x5C99, false)), // East Asian ideograph
    (0x232350, (0x84A1, false)), // East Asian ideograph
    (0x225B38, (0x7480, false)), // East Asian ideograph
    (0x232353, (0x849F, false)), // East Asian ideograph
    (0x222355, (0x5CA6, false)), // East Asian ideograph
    (0x232356, (0x84BA, false)), // East Asian ideograph
    (0x222357, (0x5CA0, false)), // East Asian ideograph
    (0x23515C, (0x993B, false)), // East Asian ideograph
    (0x69255E, (0x30DE, false)), // Katakana letter MA
    (0x22235C, (0x5CA2, false)), // East Asian ideograph
    (0x275B3A, (0x8F72, false)), // East Asian ideograph
    (0x23235E, (0x84C1, false)), // East Asian ideograph
    (0x23235F, (0x84BB, false)), // East Asian ideograph
    (0x222360, (0x5CB5, false)), // East Asian ideograph
    (0x222361, (0x5CA7, false)), // East Asian ideograph
    (0x215B3B, (0x8EF8, false)), // East Asian ideograph
    (0x222366, (0x5CA8, false)), // East Asian ideograph
    (0x222367, (0x5CAC, false)), // East Asian ideograph
    (0x234050, (0x9158, false)), // East Asian ideograph
    (0x225B3C, (0x7481, false)), // East Asian ideograph
    (0x22236B, (0x5CA3, false)), // East Asian ideograph
    (0x22236C, (0x5CB6, false)), // East Asian ideograph
    (0x22236D, (0x5CC1, false)), // East Asian ideograph
    (0x23456E, (0x9351, false)), // East Asian ideograph
    (0x22236F, (0x5CAD, false)), // East Asian ideograph
    (0x232370, (0x84B1, false)), // East Asian ideograph
    (0x232371, (0x849D, false)), // East Asian ideograph
    (0x232372, (0x84D0, false)), // East Asian ideograph
    (0x224666, (0x6C2D, false)), // East Asian ideograph
    (0x232375, (0x8494, false)), // East Asian ideograph
    (0x222378, (0x5CD3, false)), // East Asian ideograph
    (0x232379, (0x84C7, false)), // East Asian ideograph
    (0x23237A, (0x84BD, false)), // East Asian ideograph
    (0x215B3F, (0x8F09, false)), // East Asian ideograph
    (0x23237C, (0x84C2, false)), // East Asian ideograph
    (0x6F526E, (0xC0C8, false)), // Korean hangul
    (0x282569, (0x5D02, false)), // East Asian ideograph
    (0x225B40, (0x7497, false)), // East Asian ideograph
    (0x29473E, (0x94F9, false)), // East Asian ideograph
    (0x23572E, (0x9B93, false)), // East Asian ideograph
    (0x275B41, (0x8F85, false)), // East Asian ideograph
    (0x215B42, (0x8F12, false)), // East Asian ideograph
    (0x27504D, (0x7B79, false)), // East Asian ideograph
    (0x224D5F, (0x6F72, false)), // East Asian ideograph
    (0x225B43, (0x7498, false)), // East Asian ideograph
    (0x6F5B44, (0xD22D, false)), // Korean hangul
    (0x6F526F, (0xC0C9, false)), // Korean hangul
    (0x2E6C46, (0x7BE6, false)), // East Asian ideograph
    (0x225B45, (0x749A, false)), // East Asian ideograph
    (0x284340, (0x67A5, false)), // East Asian ideograph
    (0x692526, (0x30A6, false)), // Katakana letter U
    (0x275B46, (0x8F86, false)), // East Asian ideograph
    (0x2D573B, (0x60F7, false)), // East Asian ideograph
    (0x215B47, (0x8F1F, false)), // East Asian ideograph
    (0x275B48, (0x8F89, false)), // East Asian ideograph
    (0x275B49, (0x8F88, false)), // East Asian ideograph
    (0x6F5270, (0xC0CC, false)), // Korean hangul
    (0x27615B, (0x810F, false)), // East Asian ideograph
    (0x275B4A, (0x8F6E, false)), // East Asian ideograph
    (0x234A65, (0x9688, false)), // East Asian ideograph
    (0x6F5845, (0xC9F9, false)), // Korean hangul
    (0x215B4B, (0x8F1C, false)), // East Asian ideograph
    (0x33422A, (0x62E1, false)), // East Asian ideograph
    (0x275B4C, (0x8F90, false)), // East Asian ideograph
    (0x225B4D, (0x74A4, false)), // East Asian ideograph
    (0x235160, (0x993A, false)), // East Asian ideograph
    (0x275B4E, (0x8F93, false)), // East Asian ideograph
    (0x6F516B, (0xBE1D, false)), // Korean hangul
    (0x276131, (0x9A6F, false)), // East Asian ideograph
    (0x215B4F, (0x8F44, false)), // East Asian ideograph
    (0x225A2B, (0x7424, false)), // East Asian ideograph
    (0x2D3B40, (0x5C06, false)), // East Asian ideograph (variant of 273B40 which maps to 5C06)
    (0x275B51, (0x8F95, false)), // East Asian ideograph
    (0x28544F, (0x70EC, false)), // East Asian ideograph
    (0x224D62, (0x6F57, false)), // East Asian ideograph
    (0x29337A, (0x8BCC, false)), // East Asian ideograph
    (0x235161, (0x9941, false)), // East Asian ideograph
    (0x275B53, (0x8206, false)), // East Asian ideograph
    (0x215B54, (0x8F4D, false)), // East Asian ideograph
    (0x692533, (0x30B3, false)), // Katakana letter KO
    (0x4B316A, (0x723C, false)), // East Asian ideograph
    (0x215B55, (0x8F49, false)), // East Asian ideograph
    (0x2D3F31, (0x6159, false)), // East Asian ideograph
    (0x6F5665, (0xC74A, false)), // Korean hangul
    (0x225B56, (0x748D, false)), // East Asian ideograph
    (0x224667, (0x6C30, false)), // East Asian ideograph
    (0x215B57, (0x8F4E, false)), // East Asian ideograph
    (0x275B58, (0x8F70, false)), // East Asian ideograph
    (0x213C71, (0x5E96, false)), // East Asian ideograph
    (0x275B59, (0x8F94, false)), // East Asian ideograph
    (0x234056, (0x9164, false)), // East Asian ideograph
    (0x225A2D, (0x742D, false)), // East Asian ideograph
    (0x6F4C56, (0xB289, false)), // Korean hangul
    (0x232421, (0x8495, false)), // East Asian ideograph
    (0x692422, (0x3042, false)), // Hiragana letter A
    (0x692423, (0x3043, false)), // Hiragana letter small I
    (0x692424, (0x3044, false)), // Hiragana letter I
    (0x692425, (0x3045, false)), // Hiragana letter small U
    (0x222426, (0x5CE0, false)), // East Asian ideograph
    (0x232427, (0x84AF, false)), // East Asian ideograph
    (0x222428, (0x5CD2, false)), // East Asian ideograph
    (0x232429, (0x84AD, false)), // East Asian ideograph
    (0x69242A, (0x304A, false)), // Hiragana letter O
    (0x22242B, (0x5CCB, false)), // East Asian ideograph
    (0x69242C, (0x304C, false)), // Hiragana letter GA
    (0x69242D, (0x304D, false)), // Hiragana letter KI
    (0x69242E, (0x304E, false)), // Hiragana letter GI
    (0x215B5D, (0x8FA3, false)), // East Asian ideograph
    (0x222430, (0x5CC7, false)), // East Asian ideograph
    (0x222431, (0x5CDC, false)), // East Asian ideograph
    (0x232432, (0x84A8, false)), // East Asian ideograph
    (0x232433, (0x84D6, false)), // East Asian ideograph
    (0x222434, (0x5D00, false)), // East Asian ideograph
    (0x215B5E, (0x8FA8, false)), // East Asian ideograph
    (0x213859, (0x5875, false)), // East Asian ideograph
    (0x692437, (0x3057, false)), // Hiragana letter SI
    (0x692438, (0x3058, false)), // Hiragana letter ZI
    (0x692439, (0x3059, false)), // Hiragana letter SU
    (0x23243A, (0x8493, false)), // East Asian ideograph
    (0x22243B, (0x5CFF, false)), // East Asian ideograph
    (0x22243C, (0x5CEB, false)), // East Asian ideograph
    (0x69243D, (0x305D, false)), // Hiragana letter SO
    (0x69243E, (0x305E, false)), // Hiragana letter ZO
    (0x23243F, (0x84CF, false)), // East Asian ideograph
    (0x692440, (0x3060, false)), // Hiragana letter DA
    (0x232441, (0x84CA, false)), // East Asian ideograph
    (0x692442, (0x3062, false)), // Hiragana letter DI
    (0x692443, (0x3063, false)), // Hiragana letter small TU
    (0x692444, (0x3064, false)), // Hiragana letter TU
    (0x692445, (0x3065, false)), // Hiragana letter DU
    (0x232446, (0x8506, false)), // East Asian ideograph
    (0x232447, (0x850B, false)), // East Asian ideograph
    (0x692448, (0x3068, false)), // Hiragana letter TO
    (0x222449, (0x5D1E, false)), // East Asian ideograph
    (0x22244A, (0x5D12, false)), // East Asian ideograph
    (0x69244B, (0x306B, false)), // Hiragana letter NI
    (0x69244C, (0x306C, false)), // Hiragana letter NU
    (0x23244D, (0x8500, false)), // East Asian ideograph
    (0x69244E, (0x306E, false)), // Hiragana letter NO
    (0x69244F, (0x306F, false)), // Hiragana letter HA
    (0x222450, (0x5D1A, false)), // East Asian ideograph
    (0x692451, (0x3071, false)), // Hiragana letter PA
    (0x222452, (0x5D0C, false)), // East Asian ideograph
    (0x222453, (0x5D20, false)), // East Asian ideograph
    (0x222454, (0x5D21, false)), // East Asian ideograph
    (0x692455, (0x3075, false)), // Hiragana letter HU
    (0x692456, (0x3076, false)), // Hiragana letter BU
    (0x222457, (0x5D27, false)), // East Asian ideograph
    (0x222458, (0x5D0D, false)), // East Asian ideograph
    (0x232459, (0x851F, false)), // East Asian ideograph
    (0x22245A, (0x5D26, false)), // East Asian ideograph
    (0x69245B, (0x307B, false)), // Hiragana letter HO
    (0x23245C, (0x853B, false)), // East Asian ideograph
    (0x22245D, (0x5D2E, false)), // East Asian ideograph
    (0x69245E, (0x307E, false)), // Hiragana letter MA
    (0x23245F, (0x84EA, false)), // East Asian ideograph
    (0x692460, (0x3080, false)), // Hiragana letter MU
    (0x692461, (0x3081, false)), // Hiragana letter ME
    (0x692462, (0x3082, false)), // Hiragana letter MO
    (0x692463, (0x3083, false)), // Hiragana letter small YA
    (0x692464, (0x3084, false)), // Hiragana letter YA
    (0x235B66, (0x9DA4, false)), // East Asian ideograph
    (0x232466, (0x84F4, false)), // East Asian ideograph
    (0x692467, (0x3087, false)), // Hiragana letter small YO
    (0x692468, (0x3088, false)), // Hiragana letter YO
    (0x222469, (0x5D24, false)), // East Asian ideograph
    (0x23246A, (0x850C, false)), // East Asian ideograph
    (0x215B67, (0x8FC5, false)), // East Asian ideograph
    (0x69246C, (0x308C, false)), // Hiragana letter RE
    (0x69246D, (0x308D, false)), // Hiragana letter RO
    (0x69246E, (0x308E, false)), // Hiragana letter small WA
    (0x69246F, (0x308F, false)), // Hiragana letter WA
    (0x692470, (0x3090, false)), // Hiragana letter WI
    (0x222471, (0x5D36, false)), // East Asian ideograph
    (0x222472, (0x5D3E, false)), // East Asian ideograph
    (0x692473, (0x3093, false)), // Hiragana letter N
    (0x222474, (0x5D4B, false)), // East Asian ideograph
    (0x232475, (0x8515, false)), // East Asian ideograph
    (0x222476, (0x5D57, false)), // East Asian ideograph
    (0x222477, (0x5D34, false)), // East Asian ideograph
    (0x6F2478, (0x3155, false)), // Korean hangul
    (0x335E2F, (0x5257, false)), // East Asian ideograph
    (0x23247A, (0x84FC, false)), // East Asian ideograph
    (0x6F247B, (0x3158, false)), // Korean hangul
    (0x23247C, (0x84EB, false)), // East Asian ideograph
    (0x23247D, (0x84FD, false)), // East Asian ideograph
    (0x6F247E, (0x315B, false)), // Korean hangul
    (0x235B6B, (0x9D9A, false)), // East Asian ideograph
    (0x235166, (0x993C, false)), // East Asian ideograph
    (0x225B6C, (0x74A5, false)), // East Asian ideograph
    (0x6F5277, (0xC0DD, false)), // Korean hangul
    (0x6F4C31, (0xB179, false)), // Korean hangul
    (0x215B6D, (0x8FF0, false)), // East Asian ideograph (variant of 275B6D which maps to 8FF0)
    (0x21385C, (0x5885, false)), // East Asian ideograph
    (0x69252E, (0x30AE, false)), // Katakana letter GI
    (0x225B6E, (0x74A8, false)), // East Asian ideograph
    (0x235E30, (0x9EC1, false)), // East Asian ideograph
    (0x295921, (0x9CD9, false)), // East Asian ideograph
    (0x6F5B6F, (0xD310, false)), // Korean hangul
    (0x295854, (0x9CC4, false)), // East Asian ideograph
    (0x215B70, (0x8FEA, false)), // East Asian ideograph
    (0x705B71, (0x57B4, false)), // East Asian ideograph
    (0x6F5278, (0xC0E4, false)), // Korean hangul
    (0x276163, (0x677E, false)), // East Asian ideograph (duplicate simplified)
    (0x6F4E35, (0xB5B0, false)), // Korean hangul
    (0x69252F, (0x30AF, false)), // Katakana letter KU
    (0x234B66, (0x96D8, false)), // East Asian ideograph
    (0x235B74, (0x9DB1, false)), // East Asian ideograph
    (0x6F4B7C, (0xB123, false)), // Korean hangul
    (0x4B6053, (0x985E, false)), // East Asian ideograph
    (0x224926, (0x6D6F, false)), // East Asian ideograph
    (0x235B76, (0x9DB6, false)), // East Asian ideograph
    (0x234621, (0x93B5, false)), // East Asian ideograph
    (0x276164, (0x80E1, false)), // East Asian ideograph (duplicate simplified)
    (0x235B77, (0x9DBC, false)), // East Asian ideograph
    (0x336275, (0x76BC, false)), // East Asian ideograph
    (0x234623, (0x9388, false)), // East Asian ideograph
    (0x295B79, (0x9E5A, false)), // East Asian ideograph
    (0x51496B, (0x852B, false)), // East Asian ideograph
    (0x215B7A, (0x9003, false)), // East Asian ideograph
    (0x234625, (0x93B9, false)), // East Asian ideograph
    (0x215B7B, (0x8FFD, false)), // East Asian ideograph
    (0x6F5173, (0xBE54, false)), // Korean hangul
    (0x6F527A, (0xC0E8, false)), // Korean hangul
    (0x276165, (0x987B, false)), // East Asian ideograph (duplicate simplified)
    (0x235B7C, (0x9DBA, false)), // East Asian ideograph
    (0x21385F, (0x5880, false)), // East Asian ideograph
    (0x234627, (0x93A1, false)), // East Asian ideograph
    (0x215635, (0x85E5, false)), // East Asian ideograph
    (0x275B7D, (0x8FD9, false)), // East Asian ideograph
    (0x234628, (0x93B0, false)), // East Asian ideograph
    (0x235B7E, (0x9DCF, false)), // East Asian ideograph
    (0x234629, (0x93A3, false)), // East Asian ideograph
    (0x21462A, (0x6B77, false)), // East Asian ideograph
    (0x224928, (0x6D61, false)), // East Asian ideograph
    (0x23462B, (0x939B, false)), // East Asian ideograph
    (0x276166, (0x9B13, false)), // East Asian ideograph
    (0x4B3F50, (0x61CA, false)), // East Asian ideograph (variant of 213F50)
    (0x22462C, (0x6BF3, false)), // East Asian ideograph
    (0x692532, (0x30B2, false)), // Katakana letter GE
    (0x23462D, (0x9398, false)), // East Asian ideograph
    (0x213238, (0x5075, false)), // East Asian ideograph
    (0x282626, (0x5CC4, false)), // East Asian ideograph
    (0x4B462E, (0x6B81, false)), // East Asian ideograph
    (0x2F5F45, (0x86A1, false)), // East Asian ideograph
    (0x2D4B22, (0x736A, false)), // East Asian ideograph
    (0x29576E, (0x9CA7, false)), // East Asian ideograph
    (0x692521, (0x30A1, false)), // Katakana letter small A
    (0x692522, (0x30A2, false)), // Katakana letter A
    (0x276167, (0x6597, false)), // East Asian ideograph
    (0x232524, (0x851E, false)), // East Asian ideograph
    (0x222525, (0x5D3F, false)), // East Asian ideograph
    (0x222526, (0x5D52, false)), // East Asian ideograph
    (0x222527, (0x5D3D, false)), // East Asian ideograph
    (0x222528, (0x5D4E, false)), // East Asian ideograph
    (0x692529, (0x30A9, false)), // Katakana letter small O
    (0x23252A, (0x8518, false)), // East Asian ideograph
    (0x69252B, (0x30AB, false)), // Katakana letter KA
    (0x22252C, (0x5D59, false)), // East Asian ideograph
    (0x23252D, (0x8526, false)), // East Asian ideograph
    (0x23252E, (0x8507, false)), // East Asian ideograph (variant of 2F252E which maps to 8507)
    (0x22252F, (0x5D32, false)), // East Asian ideograph
    (0x692530, (0x30B0, false)), // Katakana letter GU
    (0x222531, (0x5D42, false)), // East Asian ideograph
    (0x4C2532, (0x5D5B, false)), // East Asian ideograph
    (0x224633, (0x6BF8, false)), // East Asian ideograph
    (0x232534, (0x84F0, false)), // East Asian ideograph
    (0x232535, (0x84EF, false)), // East Asian ideograph
    (0x232536, (0x8556, false)), // East Asian ideograph
    (0x692537, (0x30B7, false)), // Katakana letter SI
    (0x692538, (0x30B8, false)), // Katakana letter ZI
    (0x222539, (0x5D6F, false)), // East Asian ideograph
    (0x22253A, (0x5D6B, false)), // East Asian ideograph
    (0x696136, (0x753C, false)), // East Asian ideograph
    (0x69253C, (0x30BC, false)), // Katakana letter ZE
    (0x69253D, (0x30BD, false)), // Katakana letter SO
    (0x69253E, (0x30BE, false)), // Katakana letter ZO
    (0x274635, (0x6B87, false)), // East Asian ideograph
    (0x692540, (0x30C0, false)), // Katakana letter DA
    (0x276168, (0x95F9, false)), // East Asian ideograph
    (0x692542, (0x30C2, false)), // Katakana letter DI
    (0x692543, (0x30C3, false)), // Katakana letter small TU
    (0x222544, (0x5D4A, false)), // East Asian ideograph
    (0x225E7A, (0x7602, false)), // East Asian ideograph
    (0x232546, (0x8541, false)), // East Asian ideograph
    (0x692534, (0x30B4, false)), // Katakana letter GO
    (0x692548, (0x30C8, false)), // Katakana letter TO
    (0x222549, (0x5D6C, false)), // East Asian ideograph
    (0x22254A, (0x5D62, false)), // East Asian ideograph
    (0x23254B, (0x8558, false)), // East Asian ideograph
    (0x69254C, (0x30CC, false)), // Katakana letter NU
    (0x22254D, (0x5D82, false)), // East Asian ideograph
    (0x23254E, (0x8561, false)), // East Asian ideograph
    (0x23254F, (0x8540, false)), // East Asian ideograph
    (0x222550, (0x5D79, false)), // East Asian ideograph
    (0x224638, (0x6BF9, false)), // East Asian ideograph
    (0x692552, (0x30D2, false)), // Katakana letter HI
    (0x692553, (0x30D3, false)), // Katakana letter BI
    (0x692554, (0x30D4, false)), // Katakana letter PI
    (0x287231, (0x7F03, false)), // East Asian ideograph
    (0x692556, (0x30D6, false)), // Katakana letter BU
    (0x33516D, (0x6374, false)), // East Asian ideograph
    (0x4D222A, (0x83B5, false)), // East Asian ideograph
    (0x692559, (0x30D9, false)), // Katakana letter BE
    (0x22255A, (0x5D81, false)), // East Asian ideograph
    (0x69255B, (0x30DB, false)), // Katakana letter HO
    (0x23255C, (0x8564, false)), // East Asian ideograph
    (0x23255D, (0x855E, false)), // East Asian ideograph
    (0x23255E, (0x8573, false)), // East Asian ideograph
    (0x23255F, (0x8551, false)), // East Asian ideograph
    (0x222560, (0x5D7E, false)), // East Asian ideograph
    (0x692561, (0x30E1, false)), // Katakana letter ME
    (0x692562, (0x30E2, false)), // Katakana letter MO
    (0x27463B, (0x6740, false)), // East Asian ideograph
    (0x232564, (0x8562, false)), // East Asian ideograph
    (0x292535, (0x82C1, false)), // East Asian ideograph
    (0x222566, (0x5D92, false)), // East Asian ideograph
    (0x292567, (0x836C, false)), // East Asian ideograph
    (0x222568, (0x5D99, false)), // East Asian ideograph
    (0x27463C, (0x58F3, false)), // East Asian ideograph
    (0x22256A, (0x5DA2, false)), // East Asian ideograph
    (0x23256B, (0x8563, false)), // East Asian ideograph
    (0x23256C, (0x848D, false)), // East Asian ideograph
    (0x23256D, (0x8542, false)), // East Asian ideograph
    (0x69256E, (0x30EE, false)), // Katakana letter small WA
    (0x23463D, (0x93A4, false)), // East Asian ideograph
    (0x692570, (0x30F0, false)), // Katakana letter WI
    (0x232571, (0x854E, false)), // East Asian ideograph
    (0x692572, (0x30F2, false)), // Katakana letter WO
    (0x222573, (0x5DA1, false)), // East Asian ideograph
    (0x232574, (0x8555, false)), // East Asian ideograph
    (0x222575, (0x5D93, false)), // East Asian ideograph
    (0x232576, (0x855D, false)), // East Asian ideograph
    (0x222577, (0x5DA0, false)), // East Asian ideograph
    (0x4B3E40, (0x6046, false)), // East Asian ideograph
    (0x22257B, (0x5D94, false)), // East Asian ideograph
    (0x27616A, (0x90C1, false)), // East Asian ideograph
    (0x22257E, (0x5DAC, false)), // East Asian ideograph
    (0x6F4E3C, (0xB5C0, false)), // Korean hangul
    (0x284350, (0x68C2, false)), // East Asian ideograph
    (0x234640, (0x93BC, false)), // East Asian ideograph
    (0x692536, (0x30B6, false)), // Katakana letter ZA
    (0x513421, (0x91D6, false)), // East Asian ideograph
    (0x224642, (0x6BFF, false)), // East Asian ideograph
    (0x3F5F49, (0x7431, false)), // East Asian ideograph
    (0x29585C, (0x9CC7, false)), // East Asian ideograph
    (0x6F5C65, (0xD55C, false)), // Korean hangul
    (0x224644, (0x6C06, false)), // East Asian ideograph
    (0x276134, (0x9A7C, false)), // East Asian ideograph
    (0x28656A, (0x789B, false)), // East Asian ideograph
    (0x213865, (0x58C5, false)), // East Asian ideograph
    (0x294750, (0x950E, false)), // East Asian ideograph
    (0x4B3178, (0x5029, false)), // East Asian ideograph (variant of 213178 which maps to 5029)
    (0x234647, (0x93A6, false)), // East Asian ideograph
    (0x29337D, (0x8C27, false)), // East Asian ideograph
    (0x224648, (0x6C04, false)), // East Asian ideograph
    (0x22492E, (0x6D8A, false)), // East Asian ideograph
    (0x453755, (0x56AE, false)), // East Asian ideograph
    (0x2D516A, (0x7DB3, false)), // East Asian ideograph
    (0x6F4E3E, (0xB5CC, false)), // Korean hangul
    (0x23464A, (0x93AA, false)), // East Asian ideograph
    (0x294751, (0x950F, false)), // East Asian ideograph
    (0x33627D, (0x6589, false)), // East Asian ideograph
    (0x213423, (0x5291, false)), // East Asian ideograph
    (0x235060, (0x98E3, false)), // East Asian ideograph
    (0x333C21, (0x7895, false)), // East Asian ideograph
    (0x6F5748, (0xC84C, false)), // Korean hangul
    (0x295153, (0x9967, false)), // East Asian ideograph
    (0x22464C, (0x6C08, false)), // East Asian ideograph
    (0x23464D, (0x939E, false)), // East Asian ideograph
    (0x213C74, (0x5EA0, false)), // East Asian ideograph
    (0x6F4E3F, (0xB5CF, false)), // Korean hangul
    (0x23464F, (0x9397, false)), // East Asian ideograph
    (0x692539, (0x30B9, false)), // Katakana letter SU
    (0x27727A, (0x54D5, false)), // East Asian ideograph
    (0x234651, (0x93BB, false)), // East Asian ideograph
    (0x295825, (0x9CBA, false)), // East Asian ideograph
    (0x224D73, (0x6FB6, false)), // East Asian ideograph
    (0x224652, (0x6C0D, false)), // East Asian ideograph
    (0x234653, (0x93F1, false)), // East Asian ideograph
    (0x225851, (0x739E, false)), // East Asian ideograph
    (0x6F4E40, (0xB5D1, false)), // Korean hangul
    (0x213868, (0x58D5, false)), // East Asian ideograph
    (0x69253A, (0x30BA, false)), // Katakana letter ZU
    (0x274655, (0x6C14, false)), // East Asian ideograph
    (0x234656, (0x93DE, false)), // East Asian ideograph
    (0x234657, (0x93EE, false)), // East Asian ideograph
    (0x234D30, (0x976E, false)), // East Asian ideograph
    (0x274658, (0x6C22, false)), // East Asian ideograph
    (0x27384A, (0x573A, false)), // East Asian ideograph
    (0x223244, (0x63E5, false)), // East Asian ideograph
    (0x224659, (0x6C15, false)), // East Asian ideograph
    (0x51563F, (0x8616, false)), // East Asian ideograph
    (0x23465A, (0x93C7, false)), // East Asian ideograph
    (0x23465B, (0x93F2, false)), // East Asian ideograph
    (0x232625, (0x8580, false)), // East Asian ideograph
    (0x222626, (0x5DA7, false)), // East Asian ideograph
    (0x232628, (0x858F, false)), // East Asian ideograph
    (0x22465C, (0x6C1A, false)), // East Asian ideograph
    (0x22262A, (0x5DB0, false)), // East Asian ideograph
    (0x23262D, (0x8579, false)), // East Asian ideograph
    (0x22262E, (0x5DB4, false)), // East Asian ideograph
    (0x23465D, (0x93D4, false)), // East Asian ideograph
    (0x222630, (0x5DB6, false)), // East Asian ideograph
    (0x232632, (0x857F, false)), // East Asian ideograph
    (0x232633, (0x8577, false)), // East Asian ideograph
    (0x232634, (0x8578, false)), // East Asian ideograph
    (0x22465E, (0x6C1D, false)), // East Asian ideograph
    (0x222636, (0x5DB7, false)), // East Asian ideograph
    (0x6F4C37, (0xB18B, false)), // Korean hangul
    (0x2D6132, (0x99EE, false)), // East Asian ideograph
    (0x23263D, (0x85A4, false)), // East Asian ideograph
    (0x22263E, (0x5DC3, false)), // East Asian ideograph
    (0x224660, (0x6C20, false)), // East Asian ideograph
    (0x232642, (0x857A, false)), // East Asian ideograph
    (0x222644, (0x5DC7, false)), // East Asian ideograph
    (0x232645, (0x8557, false)), // East Asian ideograph
    (0x222646, (0x5DC9, false)), // East Asian ideograph
    (0x222647, (0x5DCB, false)), // East Asian ideograph
    (0x232649, (0x85A8, false)), // East Asian ideograph
    (0x213D4F, (0x5F59, false)), // East Asian ideograph
    (0x234D32, (0x9778, false)), // East Asian ideograph
    (0x224662, (0x6C21, false)), // East Asian ideograph
    (0x22264E, (0x5DD8, false)), // East Asian ideograph
    (0x232650, (0x8599, false)), // East Asian ideograph
    (0x232651, (0x858A, false)), // East Asian ideograph
    (0x222652, (0x5DDC, false)), // East Asian ideograph
    (0x234663, (0x93CA, false)), // East Asian ideograph
    (0x232654, (0x8590, false)), // East Asian ideograph
    (0x232656, (0x8585, false)), // East Asian ideograph
    (0x232657, (0x8588, false)), // East Asian ideograph
    (0x225A40, (0x7447, false)), // East Asian ideograph
    (0x224664, (0x6C2A, false)), // East Asian ideograph
    (0x23265A, (0x85B8, false)), // East Asian ideograph
    (0x6F5749, (0xC870, false)), // Korean hangul
    (0x23265D, (0x85C1, false)), // East Asian ideograph
    (0x334665, (0x6C61, false)), // East Asian ideograph
    (0x232661, (0x85BA, false)), // East Asian ideograph
    (0x222662, (0x5E00, false)), // East Asian ideograph
    (0x222664, (0x51E7, false)), // East Asian ideograph
    (0x234666, (0x93E8, false)), // East Asian ideograph
    (0x224934, (0x6D79, false)), // East Asian ideograph
    (0x232668, (0x85CE, false)), // East Asian ideograph
    (0x23266A, (0x85C2, false)), // East Asian ideograph
    (0x23266B, (0x85B7, false)), // East Asian ideograph
    (0x23266C, (0x85B9, false)), // East Asian ideograph
    (0x23266E, (0x85B3, false)), // East Asian ideograph
    (0x23266F, (0x85BD, false)), // East Asian ideograph
    (0x232670, (0x85C4, false)), // East Asian ideograph
    (0x224668, (0x6C2C, false)), // East Asian ideograph
    (0x222672, (0x5E14, false)), // East Asian ideograph
    (0x222673, (0x5E17, false)), // East Asian ideograph
    (0x232675, (0x85BE, false)), // East Asian ideograph
    (0x222676, (0x5E19, false)), // East Asian ideograph
    (0x224669, (0x6C31, false)), // East Asian ideograph (not in Unicode)
    (0x222678, (0x5E1F, false)), // East Asian ideograph
    (0x22267A, (0x5E23, false)), // East Asian ideograph
    (0x22267B, (0x5E21, false)), // East Asian ideograph
    (0x23267E, (0x85B6, false)), // East Asian ideograph
    (0x295421, (0x9AA3, false)), // East Asian ideograph
    (0x2D4647, (0x6BD8, false)), // East Asian ideograph
    (0x284359, (0x6989, false)), // East Asian ideograph
    (0x2D466D, (0x51B3, false)), // East Asian ideograph
    (0x294758, (0x9561, false)), // East Asian ideograph
    (0x69253F, (0x30BF, false)), // Katakana letter TA
    (0x227C5B, (0x82D0, false)), // East Asian ideograph
    (0x28723C, (0x7F08, false)), // East Asian ideograph
    (0x224670, (0x6C3B, false)), // East Asian ideograph
    (0x295422, (0x9A81, false)), // East Asian ideograph
    (0x234D35, (0x9773, false)), // East Asian ideograph
    (0x276174, (0x9C7C, false)), // East Asian ideograph
    (0x234672, (0x93DA, false)), // East Asian ideograph
    (0x234673, (0x93D0, false)), // East Asian ideograph
    (0x335E42, (0x9452, false)), // East Asian ideograph
    (0x2D353C, (0x6B62, false)), // East Asian ideograph
    (0x234674, (0x93EF, false)), // East Asian ideograph
    (0x6F4E37, (0xB5B3, false)), // Korean hangul
    (0x4B4676, (0x6C89, false)), // East Asian ideograph
    (0x213121, (0x4F11, false)), // East Asian ideograph
    (0x276136, (0x9A77, false)), // East Asian ideograph
    (0x21386F, (0x58E2, false)), // East Asian ideograph
    (0x223C6E, (0x68B2, false)), // East Asian ideograph
    (0x6F2472, (0x314F, false)), // Korean hangul
    (0x224678, (0x6C46, false)), // East Asian ideograph
    (0x6F5078, (0xBC29, false)), // Korean hangul
    (0x28723E, (0x7F0C, false)), // East Asian ideograph
    (0x29364E, (0x8D33, false)), // East Asian ideograph
    (0x22467A, (0x6C52, false)), // East Asian ideograph
    (0x213125, (0x4F01, false)), // East Asian ideograph
    (0x234D37, (0x9783, false)), // East Asian ideograph
    (0x215F69, (0x9739, false)), // East Asian ideograph
    (0x276176, (0x9C81, false)), // East Asian ideograph
    (0x6F4E48, (0xB69D, false)), // Korean hangul
    (0x23467C, (0x93CC, false)), // East Asian ideograph
    (0x6F574A, (0xC871, false)), // Korean hangul
    (0x224D7C, (0x6FC6, false)), // East Asian ideograph
    (0x23517B, (0x9954, false)), // East Asian ideograph
    (0x21312A, (0x4F4F, false)), // East Asian ideograph
    (0x234D38, (0x977A, false)), // East Asian ideograph
    (0x213C76, (0x5EAB, false)), // East Asian ideograph
    (0x21312B, (0x4F4D, false)), // East Asian ideograph
    (0x6F4E49, (0xB6A4, false)), // Korean hangul
    (0x213871, (0x58E9, false)), // East Asian ideograph
    (0x21312C, (0x4F34, false)), // East Asian ideograph
    (0x6F594C, (0xCD18, false)), // Korean hangul
    (0x21342E, (0x52C3, false)), // East Asian ideograph
    (0x21312D, (0x4F47, false)), // East Asian ideograph
    (0x2D5758, (0x890E, false)), // East Asian ideograph
    (0x21312F, (0x4F3A, false)), // East Asian ideograph
    (0x275B3F, (0x8F7D, false)), // East Asian ideograph
    (0x6F4F3D, (0xB86D, false)), // Korean hangul
    (0x28704A, (0x7EBE, false)), // East Asian ideograph
    (0x222722, (0x5E22, false)), // East Asian ideograph
    (0x286577, (0x789C, false)), // East Asian ideograph
    (0x222724, (0x5E28, false)), // East Asian ideograph
    (0x213872, (0x58EB, false)), // East Asian ideograph
    (0x232728, (0x85F7, false)), // East Asian ideograph
    (0x6F5424, (0xC2DD, false)), // Korean hangul
    (0x23272C, (0x85E6, false)), // East Asian ideograph
    (0x223132, (0x6360, false)), // East Asian ideograph
    (0x23272E, (0x85D4, false)), // East Asian ideograph
    (0x232731, (0x85ED, false)), // East Asian ideograph
    (0x6F5D42, (0xD65C, false)), // Korean hangul
    (0x222735, (0x5E44, false)), // East Asian ideograph
    (0x222736, (0x5E43, false)), // East Asian ideograph
    (0x222739, (0x5E42, false)), // East Asian ideograph
    (0x22273F, (0x5E4E, false)), // East Asian ideograph
    (0x6F4E4B, (0xB6AC, false)), // Korean hangul
    (0x232743, (0x85DF, false)), // East Asian ideograph
    (0x232745, (0x85D8, false)), // East Asian ideograph
    (0x692545, (0x30C5, false)), // Katakana letter DU
    (0x222747, (0x5E58, false)), // East Asian ideograph
    (0x222748, (0x5E48, false)), // East Asian ideograph
    (0x513B52, (0x6C3D, false)), // East Asian ideograph
    (0x213137, (0x4F3D, false)), // East Asian ideograph
    (0x23274C, (0x85DC, false)), // East Asian ideograph
    (0x23274E, (0x85F5, false)), // East Asian ideograph
    (0x273138, (0x5E03, false)), // East Asian ideograph
    (0x232752, (0x8622, false)), // East Asian ideograph
    (0x232754, (0x8610, false)), // East Asian ideograph
    (0x285029, (0x6EDF, false)), // East Asian ideograph
    (0x232757, (0x85FC, false)), // East Asian ideograph
    (0x222758, (0x5E61, false)), // East Asian ideograph
    (0x23275B, (0x85FF, false)), // East Asian ideograph
    (0x23313A, (0x89D6, false)), // East Asian ideograph
    (0x23275E, (0x85FE, false)), // East Asian ideograph
    (0x22275F, (0x5E6C, false)), // East Asian ideograph
    (0x222760, (0x5E6A, false)), // East Asian ideograph
    (0x222763, (0x5E6E, false)), // East Asian ideograph
    (0x222764, (0x5E6D, false)), // East Asian ideograph
    (0x222765, (0x5E70, false)), // East Asian ideograph
    (0x232768, (0x8604, false)), // East Asian ideograph
    (0x27313C, (0x5360, false)), // East Asian ideograph
    (0x227C6E, (0x8314, false)), // East Asian ideograph
    (0x22276D, (0x5E75, false)), // East Asian ideograph
    (0x232771, (0x8605, false)), // East Asian ideograph
    (0x216757, (0x50A3, false)), // East Asian ideograph
    (0x232775, (0x862B, false)), // East Asian ideograph
    (0x213D51, (0x5F62, false)), // East Asian ideograph
    (0x222777, (0x5E80, false)), // East Asian ideograph
    (0x21313F, (0x4F5C, false)), // East Asian ideograph
    (0x22277E, (0x5E8B, false)), // East Asian ideograph
    (0x275D38, (0x91CA, false)), // East Asian ideograph
    (0x294760, (0x9563, false)), // East Asian ideograph
    (0x213432, (0x52D2, false)), // East Asian ideograph
    (0x33572E, (0x880E, false)), // East Asian ideograph
    (0x2D3543, (0x4EDD, false)), // East Asian ideograph
    (0x27506F, (0x7EA0, false)), // East Asian ideograph
    (0x215B27, (0x8E85, false)), // East Asian ideograph
    (0x213142, (0x4F4E, false)), // East Asian ideograph
    (0x217D40, (0x5B19, false)), // East Asian ideograph
    (0x213143, (0x4F5D, false)), // East Asian ideograph
    (0x213C77, (0x5EA7, false)), // East Asian ideograph
    (0x213144, (0x4F36, false)), // East Asian ideograph
    (0x6F4B5C, (0xB0AF, false)), // Korean hangul
    (0x6F4E4E, (0xB6F4, false)), // Korean hangul
    (0x213876, (0x58FA, false)), // East Asian ideograph
    (0x223145, (0x6335, false)), // East Asian ideograph
    (0x706067, (0x567B, false)), // East Asian ideograph
    (0x223832, (0x665F, false)), // East Asian ideograph
    (0x295828, (0x9CB4, false)), // East Asian ideograph
    (0x233147, (0x89E1, false)), // East Asian ideograph
    (0x29586E, (0x9CA5, false)), // East Asian ideograph
    (0x213148, (0x4F8D, false)), // East Asian ideograph
    (0x275B40, (0x8F7E, false)), // East Asian ideograph
    (0x6F4E4F, (0xB6F8, false)), // Korean hangul
    (0x275736, (0x8747, false)), // East Asian ideograph
    (0x21314A, (0x4F7F, false)), // East Asian ideograph
    (0x692549, (0x30C9, false)), // Katakana letter DO
    (0x2F5476, (0x9AE1, false)), // East Asian ideograph
    (0x21314B, (0x4F9B, false)), // East Asian ideograph
    (0x275071, (0x7EA3, false)), // East Asian ideograph
    (0x21314C, (0x4F86, false)), // East Asian ideograph
    (0x2D3730, (0x751E, false)), // East Asian ideograph
    (0x21314D, (0x4F6C, false)), // East Asian ideograph
    (0x6F4E50, (0xB700, false)), // Korean hangul
    (0x21314F, (0x4F96, false)), // East Asian ideograph
    (0x213435, (0x52DE, false)), // East Asian ideograph
    (0x233C33, (0x8F47, false)), // East Asian ideograph
    (0x213151, (0x4F83, false)), // East Asian ideograph
    (0x287247, (0x7F11, false)), // East Asian ideograph
    (0x697152, (0x99F2, false)), // East Asian ideograph
    (0x453768, (0x5EFB, false)), // East Asian ideograph
    (0x213153, (0x4F88, false)), // East Asian ideograph
    (0x276138, (0x9A79, false)), // East Asian ideograph
    (0x4B3F51, (0x61D1, false)), // East Asian ideograph
    (0x6F4E51, (0xB701, false)), // Korean hangul
    (0x213154, (0x4F69, false)), // East Asian ideograph
    (0x69254B, (0x30CB, false)), // Katakana letter NI
    (0x213436, (0x52DB, false)), // East Asian ideograph
    (0x6F5826, (0xC998, false)), // Korean hangul
    (0x275073, (0x7EAB, false)), // East Asian ideograph
    (0x354156, (0x91BE, false)), // East Asian ideograph
    (0x295871, (0x9CCE, false)), // East Asian ideograph
    (0x287248, (0x7F0F, false)), // East Asian ideograph
    (0x4B606F, (0x991D, false)), // East Asian ideograph
    (0x233158, (0x89F1, false)), // East Asian ideograph
    (0x294531, (0x9528, false)), // East Asian ideograph
    (0x6F4E52, (0xB728, false)), // Korean hangul
    (0x284366, (0x6924, false)), // East Asian ideograph
    (0x217159, (0x55D0, false)), // East Asian ideograph
    (0x225A4F, (0x7452, false)), // East Asian ideograph
    (0x21315A, (0x4FAF, false)), // East Asian ideograph
    (0x232822, (0x8627, false)), // East Asian ideograph
    (0x21715B, (0x55CD, false)), // East Asian ideograph
    (0x274C31, (0x7544, false)), // East Asian ideograph
    (0x232826, (0x8629, false)), // East Asian ideograph
    (0x23315C, (0x89F3, false)), // East Asian ideograph
    (0x224943, (0x6D94, false)), // East Asian ideograph
    (0x213A61, (0x5B7A, false)), // East Asian ideograph
    (0x21315D, (0x4FE0, false)), // East Asian ideograph
    (0x6F4B5D, (0xB0B1, false)), // Korean hangul
    (0x232832, (0x8637, false)), // East Asian ideograph
    (0x395230, (0x5BD8, false)), // East Asian ideograph
    (0x222835, (0x5EA5, false)), // East Asian ideograph
    (0x222836, (0x5EAF, false)), // East Asian ideograph
    (0x213438, (0x52E2, false)), // East Asian ideograph
    (0x232838, (0x8636, false)), // East Asian ideograph
    (0x21315F, (0x4FB6, false)), // East Asian ideograph
    (0x23283E, (0x863C, false)), // East Asian ideograph
    (0x23283F, (0x8640, false)), // East Asian ideograph
    (0x232840, (0x863A, false)), // East Asian ideograph
    (0x233160, (0x89F6, false)), // East Asian ideograph
    (0x222842, (0x5EB9, false)), // East Asian ideograph
    (0x39365A, (0x8AE0, false)), // East Asian ideograph
    (0x227161, (0x7DA3, false)), // East Asian ideograph
    (0x22284B, (0x5EB3, false)), // East Asian ideograph
    (0x22284C, (0x5EC4, false)), // East Asian ideograph
    (0x217162, (0x55DD, false)), // East Asian ideograph
    (0x6F4E54, (0xB72C, false)), // Korean hangul
    (0x275D3F, (0x9488, false)), // East Asian ideograph
    (0x294767, (0x94E7, false)), // East Asian ideograph
    (0x69254E, (0x30CE, false)), // Katakana letter NO
    (0x222855, (0x5ECB, false)), // East Asian ideograph
    (0x222857, (0x5ECD, false)), // East Asian ideograph
    (0x213164, (0x4FDF, false)), // East Asian ideograph
    (0x22285A, (0x5ED2, false)), // East Asian ideograph
    (0x22285B, (0x5ED1, false)), // East Asian ideograph
    (0x22285C, (0x5ED5, false)), // East Asian ideograph
    (0x23285E, (0x8659, false)), // East Asian ideograph
    (0x22285F, (0x5ED4, false)), // East Asian ideograph
    (0x222860, (0x5ED9, false)), // East Asian ideograph
    (0x222861, (0x5ECE, false)), // East Asian ideograph
    (0x21232D, (0xFF0D, false)), // Ideographic hyphen minus
    (0x232866, (0x8661, false)), // East Asian ideograph
    (0x4C2867, (0x5EDB, false)), // East Asian ideograph
    (0x222868, (0x5EE1, false)), // East Asian ideograph
    (0x232869, (0x8662, false)), // East Asian ideograph
    (0x23286A, (0x8663, false)), // East Asian ideograph
    (0x287167, (0x7EEF, false)), // East Asian ideograph
    (0x22286D, (0x5EE7, false)), // East Asian ideograph
    (0x232871, (0x8669, false)), // East Asian ideograph
    (0x69254F, (0x30CF, false)), // Katakana letter HA
    (0x2D5B7A, (0x8FEF, false)), // East Asian ideograph
    (0x217169, (0x55E9, false)), // East Asian ideograph
    (0x232878, (0x866C, false)), // East Asian ideograph
    (0x23287B, (0x8672, false)), // East Asian ideograph
    (0x22287C, (0x5EED, false)), // East Asian ideograph
    (0x21316A, (0x4FCE, false)), // East Asian ideograph
    (0x23287E, (0x867B, false)), // East Asian ideograph
    (0x274C34, (0x5F02, false)), // East Asian ideograph
    (0x23462A, (0x93B7, false)), // East Asian ideograph
    (0x21316B, (0x4FD7, false)), // East Asian ideograph
    (0x23316C, (0x8A06, false)), // East Asian ideograph
    (0x276139, (0x9A78, false)), // East Asian ideograph
    (0x6F4E56, (0xB730, false)), // Korean hangul
    (0x294769, (0x9564, false)), // East Asian ideograph
    (0x6F507B, (0xBC31, false)), // Korean hangul
    (0x21316E, (0x500D, false)), // East Asian ideograph
    (0x4B5861, (0x4F89, false)), // East Asian ideograph
    (0x21716F, (0x55CF, false)), // East Asian ideograph
    (0x213170, (0x5026, false)), // East Asian ideograph
    (0x4B3E5B, (0x60C5, false)), // East Asian ideograph
    (0x213171, (0x500C, false)), // East Asian ideograph
    (0x6F4E57, (0xB738, false)), // Korean hangul
    (0x223172, (0x639E, false)), // East Asian ideograph
    (0x692551, (0x30D1, false)), // Katakana letter PA
    (0x21343C, (0x52F5, false)), // East Asian ideograph
    (0x273173, (0x4EEC, false)), // East Asian ideograph
    (0x4C4F24, (0x6F46, false)), // East Asian ideograph
    (0x235B7A, (0x9DC1, false)), // East Asian ideograph
    (0x287174, (0x7EF2, false)), // East Asian ideograph
    (0x274C36, (0x753B, false)), // East Asian ideograph
    (0x39365E, (0x559E, false)), // East Asian ideograph
    (0x6F5B21, (0xD0ED, false)), // Korean hangul
    (0x4B5C32, (0x9038, false)), // East Asian ideograph
    (0x6F4B5E, (0xB0B3, false)), // Korean hangul
    (0x6F4E58, (0xB739, false)), // Korean hangul
    (0x2D3177, (0x5E78, false)), // East Asian ideograph
    (0x21343D, (0x52F8, false)), // East Asian ideograph
    (0x217178, (0x55C1, false)), // East Asian ideograph
    (0x6F5C23, (0xD37C, false)), // Korean hangul
    (0x273179, (0x4FE9, false)), // East Asian ideograph
    (0x6F5C24, (0xD37D, false)), // Korean hangul
    (0x29365F, (0x8D47, false)), // East Asian ideograph
    (0x6F5B22, (0xD0EF, false)), // Korean hangul
    (0x6F5C25, (0xD380, false)), // Korean hangul
    (0x21317B, (0x5012, false)), // East Asian ideograph
    (0x6F5C26, (0xD384, false)), // Korean hangul
    (0x6F4E59, (0xB73B, false)), // Korean hangul
    (0x4B317C, (0x5024, false)), // East Asian ideograph
    (0x235C27, (0x9DC3, false)), // East Asian ideograph
    (0x22317D, (0x63AB, false)), // East Asian ideograph
    (0x215C28, (0x901A, false)), // East Asian ideograph
    (0x4C4F26, (0x6EDD, false)), // East Asian ideograph
    (0x69717E, (0x9AF7, false)), // East Asian ideograph
    (0x215C29, (0x9020, false)), // East Asian ideograph
    (0x216764, (0x5095, false)), // East Asian ideograph
    (0x6F5C2A, (0xD390, false)), // Korean hangul
    (0x6F5C2B, (0xD391, false)), // Korean hangul
    (0x6F4E5A, (0xB744, false)), // Korean hangul
    (0x6F5C2C, (0xD398, false)), // Korean hangul
    (0x695C2D, (0x6928, false)), // East Asian ideograph
    (0x4B372F, (0x5C1C, false)), // East Asian ideograph
    (0x274C39, (0x5F53, false)), // East Asian ideograph
    (0x287251, (0x7F1F, false)), // East Asian ideograph
    (0x6F5C6B, (0xD56C, false)), // Korean hangul
    (0x215C2F, (0x902E, false)), // East Asian ideograph
    (0x27613A, (0x9A7D, false)), // East Asian ideograph
    (0x225C30, (0x74C8, false)), // East Asian ideograph
    (0x6F4E5B, (0xB748, false)), // Korean hangul
    (0x222923, (0x5EF4, false)), // East Asian ideograph
    (0x232925, (0x867A, false)), // East Asian ideograph
    (0x232926, (0x8673, false)), // East Asian ideograph
    (0x225C31, (0x74C5, false)), // East Asian ideograph
    (0x29432B, (0x94C6, false)), // East Asian ideograph
    (0x6F5828, (0xC99D, false)), // Korean hangul
    (0x215C32, (0xFA25, false)), // East Asian ideograph
    (0x23292E, (0x8696, false)), // East Asian ideograph
    (0x27507D, (0x7EAF, false)), // East Asian ideograph
    (0x217671, (0x5827, false)), // East Asian ideograph
    (0x29333B, (0x8C00, false)), // East Asian ideograph
    (0x215C33, (0x9032, false)), // East Asian ideograph
    (0x222935, (0x5F07, false)), // East Asian ideograph
    (0x232936, (0x8691, false)), // East Asian ideograph
    (0x232937, (0x869C, false)), // East Asian ideograph
    (0x235C34, (0x9DAC, false)), // East Asian ideograph
    (0x22293A, (0x5F0B, false)), // East Asian ideograph
    (0x23293C, (0x868D, false)), // East Asian ideograph
    (0x23293D, (0x868B, false)), // East Asian ideograph
    (0x6F5C35, (0xD3B5, false)), // Korean hangul
    (0x232940, (0x86A6, false)), // East Asian ideograph
    (0x232942, (0x869D, false)), // East Asian ideograph
    (0x39476F, (0x51C0, false)), // East Asian ideograph
    (0x235C36, (0x9DB2, false)), // East Asian ideograph
    (0x232946, (0x86A0, false)), // East Asian ideograph
    (0x2D3F3A, (0x6185, false)), // East Asian ideograph
    (0x232948, (0x86A7, false)), // East Asian ideograph
    (0x22294A, (0x5F28, false)), // East Asian ideograph
    (0x22294B, (0x5F22, false)), // East Asian ideograph
    (0x22294C, (0x5F23, false)), // East Asian ideograph
    (0x22294D, (0x5F24, false)), // East Asian ideograph
    (0x235B7B, (0x9DB8, false)), // East Asian ideograph
    (0x225C38, (0x74D6, false)), // East Asian ideograph
    (0x222952, (0x5F30, false)), // East Asian ideograph
    (0x6F5A27, (0xCEEC, false)), // Korean hangul
    (0x215C39, (0x9054, false)), // East Asian ideograph
    (0x232958, (0x86BA, false)), // East Asian ideograph
    (0x232959, (0x86B0, false)), // East Asian ideograph
    (0x22295C, (0x5F40, false)), // East Asian ideograph
    (0x215C3A, (0x9055, false)), // East Asian ideograph
    (0x6F4E5D, (0xB764, false)), // Korean hangul
    (0x22295F, (0x5F44, false)), // East Asian ideograph
    (0x232960, (0x86B3, false)), // East Asian ideograph
    (0x232962, (0x86C9, false)), // East Asian ideograph
    (0x215C3B, (0x903C, false)), // East Asian ideograph
    (0x232967, (0x86D8, false)), // East Asian ideograph
    (0x222968, (0x5F50, false)), // East Asian ideograph
    (0x215C3C, (0x9047, false)), // East Asian ideograph
    (0x22296A, (0x5F56, false)), // East Asian ideograph
    (0x22296C, (0x5F58, false)), // East Asian ideograph
    (0x2D3E60, (0x6075, false)), // East Asian ideograph
    (0x23296E, (0x86E3, false)), // East Asian ideograph
    (0x225C3D, (0x74D8, false)), // East Asian ideograph
    (0x222970, (0x5F60, false)), // East Asian ideograph
    (0x232971, (0x86EC, false)), // East Asian ideograph
    (0x222972, (0x5F63, false)), // East Asian ideograph
    (0x222973, (0x809C, false)), // East Asian ideograph
    (0x222974, (0x5F67, false)), // East Asian ideograph
    (0x215C3E, (0x904E, false)), // East Asian ideograph
    (0x232977, (0x86D0, false)), // East Asian ideograph
    (0x222978, (0x5F72, false)), // East Asian ideograph
    (0x222979, (0x5F73, false)), // East Asian ideograph
    (0x23297A, (0x86D1, false)), // East Asian ideograph
    (0x2D5C3F, (0x5FA7, false)), // East Asian ideograph
    (0x22297C, (0x5F74, false)), // East Asian ideograph
    (0x23297E, (0x86DE, false)), // East Asian ideograph
    (0x225C40, (0x74DA, false)), // East Asian ideograph
    (0x213443, (0x5308, false)), // East Asian ideograph
    (0x215C41, (0x9041, false)), // East Asian ideograph
    (0x4C4F2B, (0x701E, false)), // East Asian ideograph
    (0x6F5C42, (0xD3FD, false)), // Korean hangul
    (0x333251, (0x5FBA, false)), // East Asian ideograph
    (0x6F5B28, (0xD138, false)), // Korean hangul
    (0x22494F, (0x6D96, false)), // East Asian ideograph
    (0x695C43, (0x6981, false)), // East Asian ideograph
    (0x4B5C39, (0x9039, false)), // East Asian ideograph
    (0x215C44, (0x9060, false)), // East Asian ideograph
    (0x6F4E5F, (0xB770, false)), // Korean hangul
    (0x273B31, (0x5B9E, false)), // East Asian ideograph
    (0x215C45, (0x905C, false)), // East Asian ideograph
    (0x29432F, (0x94F3, false)), // East Asian ideograph
    (0x696325, (0x7907, false)), // East Asian ideograph
    (0x235C46, (0x9DDE, false)), // East Asian ideograph
    (0x215C47, (0x9065, false)), // East Asian ideograph
    (0x6F5B29, (0xD140, false)), // Korean hangul
    (0x215C48, (0x905E, false)), // East Asian ideograph
    (0x6F4E38, (0xB5B4, false)), // Korean hangul
    (0x27613B, (0x9A87, false)), // East Asian ideograph
    (0x275C49, (0x9002, false)), // East Asian ideograph
    (0x6F4E60, (0xB771, false)), // Korean hangul
    (0x273B32, (0x5B81, false)), // East Asian ideograph
    (0x29255A, (0x8487, false)), // East Asian ideograph
    (0x6F5C4A, (0xD479, false)), // Korean hangul
    (0x225A5D, (0x7440, false)), // East Asian ideograph
    (0x235E5C, (0x9EE7, false)), // East Asian ideograph
    (0x6F5C4B, (0xD47C, false)), // Korean hangul
    (0x2D3556, (0x7343, false)), // East Asian ideograph
    (0x2D532C, (0x6BD3, false)), // East Asian ideograph
    (0x6F5C4C, (0xD480, false)), // Korean hangul
    (0x336321, (0x6B6F, false)), // East Asian ideograph
    (0x6F5B2A, (0xD141, false)), // Korean hangul
    (0x215C4D, (0x9075, false)), // East Asian ideograph
    (0x6F4C3A, (0xB193, false)), // Korean hangul
    (0x6F5C4E, (0xD489, false)), // Korean hangul
    (0x6F4E61, (0xB775, false)), // Korean hangul
    (0x294774, (0x9571, false)), // East Asian ideograph
    (0x215C4F, (0x9078, false)), // East Asian ideograph
    (0x217435, (0x571A, false)), // East Asian ideograph
    (0x215C50, (0x9072, false)), // East Asian ideograph
    (0x4B3231, (0x4EEE, false)), // East Asian ideograph
    (0x275C51, (0x8FC1, false)), // East Asian ideograph
    (0x6F5B2B, (0xD143, false)), // Korean hangul
    (0x275C52, (0x8FBD, false)), // East Asian ideograph
    (0x215C53, (0x907A, false)), // East Asian ideograph
    (0x4D503A, (0x98D1, false)), // East Asian ideograph
    (0x69255C, (0x30DC, false)), // Katakana letter BO
    (0x225C54, (0x74E9, false)), // East Asian ideograph
    (0x217436, (0x571B, false)), // East Asian ideograph
    (0x6F5C55, (0xD508, false)), // Korean hangul
    (0x213A36, (0x5ABD, false)), // East Asian ideograph
    (0x215C56, (0x9081, false)), // East Asian ideograph
    (0x6F5B2C, (0xD144, false)), // Korean hangul
    (0x455F35, (0x9668, false)), // East Asian ideograph (Version J extension)
    (0x215C57, (0x9084, false)), // East Asian ideograph
    (0x6F4F3E, (0xB86F, false)), // Korean hangul
    (0x6F5C33, (0xD3AD, false)), // Korean hangul
    (0x225C58, (0x74F1, false)), // East Asian ideograph
    (0x69255D, (0x30DD, false)), // Katakana letter PO
    (0x6F5C59, (0xD53C, false)), // Korean hangul
    (0x215C5A, (0x9087, false)), // East Asian ideograph
    (0x212A21, (0xE8D0, false)), // EACC component character
    (0x212A22, (0xE8D1, false)), // EACC component character
    (0x215C5B, (0x908A, false)), // East Asian ideograph
    (0x212A24, (0xE8D3, false)), // EACC component character
    (0x232A25, (0x870B, false)), // East Asian ideograph
    (0x212A26, (0xE8D5, false)), // EACC component character
    (0x222A27, (0x5F89, false)), // East Asian ideograph
    (0x212A28, (0xE8D6, false)), // EACC component character
    (0x215C5C, (0x9090, false)), // East Asian ideograph
    (0x212A2A, (0xE8D8, false)), // EACC component character
    (0x222A2B, (0x5F94, false)), // East Asian ideograph
    (0x212A2C, (0xE8DA, false)), // EACC component character
    (0x212A2D, (0xE8DB, false)), // EACC component character
    (0x69727E, (0x9D48, false)), // East Asian ideograph
    (0x215C5D, (0x908F, false)), // East Asian ideograph
    (0x2E284C, (0x5ECF, false)), // East Asian ideograph
    (0x6F4E64, (0xB77C, false)), // Korean hangul
    (0x275D4F, (0x94B4, false)), // East Asian ideograph
    (0x232A33, (0x86F8, false)), // East Asian ideograph
    (0x232A34, (0x8706, false)), // East Asian ideograph
    (0x4B5C5E, (0x961D, false)), // East Asian ideograph
    (0x232A36, (0x870E, false)), // East Asian ideograph
    (0x212A37, (0xE8E4, false)), // EACC component character
    (0x232A38, (0x8709, false)), // East Asian ideograph
    (0x222A39, (0x5F9C, false)), // East Asian ideograph
    (0x232A3A, (0x870A, false)), // East Asian ideograph
    (0x235C5F, (0x9DEB, false)), // East Asian ideograph
    (0x212A3C, (0xE8E9, false)), // EACC component character
    (0x222A3D, (0x5F9A, false)), // East Asian ideograph
    (0x232A3E, (0x870D, false)), // East Asian ideograph
    (0x212A3F, (0xE8EC, false)), // EACC component character
    (0x212A40, (0xE8ED, false)), // EACC component character
    (0x6F5C60, (0xD551, false)), // Korean hangul
    (0x232A42, (0x874A, false)), // East Asian ideograph
    (0x232A43, (0x8723, false)), // East Asian ideograph
    (0x232A44, (0x8737, false)), // East Asian ideograph
    (0x232A45, (0x8728, false)), // East Asian ideograph
    (0x222A46, (0x5FAF, false)), // East Asian ideograph
    (0x225C61, (0x74F4, false)), // East Asian ideograph
    (0x232A49, (0x8740, false)), // East Asian ideograph
    (0x232A4B, (0x872E, false)), // East Asian ideograph
    (0x232A4C, (0x873D, false)), // East Asian ideograph
    (0x232A4E, (0x871E, false)), // East Asian ideograph
    (0x6F4E65, (0xB77D, false)), // Korean hangul
    (0x222A50, (0x5FBC, false)), // East Asian ideograph
    (0x69255F, (0x30DF, false)), // Katakana letter MI
    (0x232A53, (0x8743, false)), // East Asian ideograph
    (0x232A55, (0x8744, false)), // East Asian ideograph
    (0x6F507E, (0xBC38, false)), // Korean hangul
    (0x222A57, (0x5FC9, false)), // East Asian ideograph
    (0x232A59, (0x8729, false)), // East Asian ideograph
    (0x232A5A, (0x8739, false)), // East Asian ideograph
    (0x213241, (0x5091, false)), // East Asian ideograph
    (0x232A5F, (0x871A, false)), // East Asian ideograph
    (0x222A61, (0x5FD2, false)), // East Asian ideograph
    (0x222A63, (0x5FD0, false)), // East Asian ideograph
    (0x232A64, (0x8731, false)), // East Asian ideograph
    (0x232A65, (0x8711, false)), // East Asian ideograph
    (0x232A66, (0x8712, false)), // East Asian ideograph
    (0x222A67, (0x5FCE, false)), // East Asian ideograph
    (0x222A68, (0x5FED, false)), // East Asian ideograph
    (0x6F4C3B, (0xB194, false)), // Korean hangul
    (0x232A6B, (0x874F, false)), // East Asian ideograph
    (0x232A6C, (0x8771, false)), // East Asian ideograph
    (0x232A6D, (0x8763, false)), // East Asian ideograph
    (0x275D51, (0x94B8, false)), // East Asian ideograph
    (0x232A71, (0x8764, false)), // East Asian ideograph
    (0x222A72, (0x5FEE, false)), // East Asian ideograph
    (0x232A73, (0x8765, false)), // East Asian ideograph
    (0x232A74, (0x877D, false)), // East Asian ideograph
    (0x6F5C69, (0xD569, false)), // Korean hangul
    (0x222A78, (0x5FE1, false)), // East Asian ideograph
    (0x232A79, (0x8758, false)), // East Asian ideograph
    (0x222A7B, (0x5FE4, false)), // East Asian ideograph
    (0x215C6A, (0x90E1, false)), // East Asian ideograph
    (0x28725D, (0x7F1C, false)), // East Asian ideograph
    (0x6F5B30, (0xD150, false)), // Korean hangul
    (0x275C6B, (0x5369, false)), // East Asian ideograph
    (0x3F516D, (0x6403, false)), // East Asian ideograph
    (0x235C6C, (0x9DE6, false)), // East Asian ideograph
    (0x6F4E67, (0xB784, false)), // Korean hangul
    (0x275D52, (0x94C0, false)), // East Asian ideograph
    (0x215C6D, (0x90F5, false)), // East Asian ideograph
    (0x33303A, (0x8FFA, false)), // East Asian ideograph
    (0x6F5C6E, (0xD574, false)), // Korean hangul
    (0x6F5C6F, (0xD575, false)), // Korean hangul
    (0x28725E, (0x7F19, false)), // East Asian ideograph
    (0x6F5873, (0xCC0C, false)), // Korean hangul
    (0x705F30, (0x7519, false)), // East Asian ideograph
    (0x215C70, (0x9109, false)), // East Asian ideograph
    (0x215C71, (0x9112, false)), // East Asian ideograph
    (0x6F4E68, (0xB78C, false)), // Korean hangul
    (0x275E68, (0x9617, false)), // East Asian ideograph
    (0x4B5C72, (0x9119, false)), // East Asian ideograph (variant of 215C72 which maps to 9119)
    (0x275153, (0x7EAC, false)), // East Asian ideograph
    (0x215C73, (0x912D, false)), // East Asian ideograph
    (0x235A21, (0x9D02, false)), // East Asian ideograph
    (0x215C74, (0x9130, false)), // East Asian ideograph
    (0x28725F, (0x7F1B, false)), // East Asian ideograph
    (0x6F5B32, (0xD15C, false)), // Korean hangul
    (0x224959, (0x6DAB, false)), // East Asian ideograph
    (0x215C75, (0x9127, false)), // East Asian ideograph
    (0x6F5C76, (0xD589, false)), // Korean hangul
    (0x2D4228, (0x5117, false)), // East Asian ideograph
    (0x215C77, (0x9139, false)), // East Asian ideograph (variant of 4B5C77 which maps to 9139)
    (0x455E60, (0x95EB, false)), // East Asian ideograph (Version J extension)
    (0x6F5C78, (0xD5A5, false)), // Korean hangul
    (0x235A22, (0x9D03, false)), // East Asian ideograph
    (0x2F3D5D, (0x900E, false)), // East Asian ideograph
    (0x6F5C79, (0xD5C8, false)), // Korean hangul
    (0x224724, (0x6C5C, false)), // East Asian ideograph
    (0x6F5C7A, (0xD5C9, false)), // Korean hangul
    (0x27613D, (0x9A8B, false)), // East Asian ideograph
    (0x2E4A6B, (0x6EA6, false)), // East Asian ideograph
    (0x224726, (0x6C5B, false)), // East Asian ideograph
    (0x275D55, (0x94C5, false)), // East Asian ideograph
    (0x292564, (0x8489, false)), // East Asian ideograph
    (0x224727, (0x6C4D, false)), // East Asian ideograph
    (0x225C7D, (0x7507, false)), // East Asian ideograph
    (0x22474D, (0x6C93, false)), // East Asian ideograph
    (0x235A23, (0x9CF7, false)), // East Asian ideograph
    (0x235C7E, (0x9DFD, false)), // East Asian ideograph
    (0x2D4729, (0x6D29, false)), // East Asian ideograph
    (0x213D57, (0x5F6D, false)), // East Asian ideograph
    (0x234D5A, (0x979F, false)), // East Asian ideograph
    (0x6F4C3C, (0xB1A8, false)), // Korean hangul
    (0x294532, (0x9531, false)), // East Asian ideograph
    (0x22472B, (0x6C4B, false)), // East Asian ideograph
    (0x3F4A28, (0x9DF0, false)), // East Asian ideograph
    (0x225A68, (0x7474, false)), // East Asian ideograph
    (0x6F7649, (0xE8BB, false)), // Korean hangul
    (0x6F5751, (0xC886, false)), // Korean hangul
    (0x22472D, (0x6C63, false)), // East Asian ideograph
    (0x6F5B35, (0xD160, false)), // Korean hangul
    (0x4B3D2A, (0x5EE3, false)), // East Asian ideograph
    (0x23472F, (0x93A9, false)), // East Asian ideograph
    (0x28773F, (0x804D, false)), // East Asian ideograph
    (0x232B21, (0x8761, false)), // East Asian ideograph
    (0x692535, (0x30B5, false)), // Katakana letter SA
    (0x222B24, (0x5FEA, false)), // East Asian ideograph
    (0x692566, (0x30E6, false)), // Katakana letter YU
    (0x212B26, (0x300D, false)), // Ideographic right corner bracket
    (0x225A69, (0x746E, false)), // East Asian ideograph
    (0x232B28, (0x875F, false)), // East Asian ideograph
    (0x222B2A, (0x6026, false)), // East Asian ideograph
    (0x222B2C, (0x6029, false)), // East Asian ideograph
    (0x232B2D, (0x876F, false)), // East Asian ideograph
    (0x232B2E, (0x875D, false)), // East Asian ideograph
    (0x232B30, (0x876E, false)), // East Asian ideograph
    (0x222B31, (0x6008, false)), // East Asian ideograph
    (0x212B32, (0xFF3D, false)), // Ideographic right square bracket
    (0x224733, (0x6C76, false)), // East Asian ideograph
    (0x212B34, (0xFF0E, false)), // Ideographic variant full stop
    (0x232B35, (0x8753, false)), // East Asian ideograph
    (0x222B36, (0x600A, false)), // East Asian ideograph
    (0x222B37, (0x600C, false)), // East Asian ideograph
    (0x234D5C, (0x979A, false)), // East Asian ideograph
    (0x214734, (0x6CBB, false)), // East Asian ideograph
    (0x232B3A, (0x87A3, false)), // East Asian ideograph
    (0x4B5E69, (0x95A2, false)), // East Asian ideograph
    (0x222B3C, (0x6017, false)), // East Asian ideograph
    (0x232B3D, (0x8793, false)), // East Asian ideograph
    (0x6F245F, (0x3148, false)), // Korean hangul
    (0x2D4735, (0x6C4E, false)), // East Asian ideograph
    (0x275D58, (0x94C3, false)), // East Asian ideograph
    (0x273B3F, (0x4E13, false)), // East Asian ideograph
    (0x692567, (0x30E7, false)), // Katakana letter small YO
    (0x213452, (0x5331, false)), // East Asian ideograph
    (0x232B45, (0x8799, false)), // East Asian ideograph
    (0x222B46, (0x6010, false)), // East Asian ideograph
    (0x232B48, (0x8788, false)), // East Asian ideograph
    (0x222B4B, (0x6039, false)), // East Asian ideograph
    (0x232B4C, (0x8798, false)), // East Asian ideograph
    (0x222B50, (0x6013, false)), // East Asian ideograph
    (0x224738, (0x6C6C, false)), // East Asian ideograph
    (0x222B53, (0x6054, false)), // East Asian ideograph
    (0x232B54, (0x878B, false)), // East Asian ideograph
    (0x232B55, (0x8784, false)), // East Asian ideograph
    (0x222B57, (0x605D, false)), // East Asian ideograph
    (0x232B58, (0x87A9, false)), // East Asian ideograph
    (0x336B33, (0x524F, false)), // East Asian ideograph
    (0x222B5A, (0x6047, false)), // East Asian ideograph
    (0x2E2B5B, (0x605A, false)), // East Asian ideograph
    (0x232B5D, (0x8789, false)), // East Asian ideograph
    (0x222B5E, (0x6049, false)), // East Asian ideograph
    (0x222B5F, (0x6053, false)), // East Asian ideograph
    (0x232B60, (0x87AD, false)), // East Asian ideograph
    (0x4B4E37, (0x7814, false)), // East Asian ideograph
    (0x23473B, (0x940F, false)), // East Asian ideograph
    (0x232B66, (0x87BE, false)), // East Asian ideograph
    (0x222B68, (0x6067, false)), // East Asian ideograph
    (0x23473C, (0x9420, false)), // East Asian ideograph (not in Unicode)
    (0x232B6E, (0x87C4, false)), // East Asian ideograph
    (0x232B6F, (0x87AF, false)), // East Asian ideograph
    (0x222B71, (0x6041, false)), // East Asian ideograph
    (0x222B72, (0x6077, false)), // East Asian ideograph
    (0x222B74, (0x6042, false)), // East Asian ideograph
    (0x22473E, (0x6C94, false)), // East Asian ideograph
    (0x222B76, (0x605F, false)), // East Asian ideograph
    (0x232B78, (0x87AE, false)), // East Asian ideograph
    (0x2E6C27, (0x7B2E, false)), // East Asian ideograph
    (0x222B7A, (0x6061, false)), // East Asian ideograph
    (0x6F4E6F, (0xB799, false)), // Korean hangul
    (0x232B7E, (0x87BF, false)), // East Asian ideograph
    (0x692569, (0x30E9, false)), // Katakana letter RA
    (0x224740, (0x6C8F, false)), // East Asian ideograph
    (0x333C52, (0x8CEC, false)), // East Asian ideograph
    (0x4B4741, (0x51BD, false)), // East Asian ideograph
    (0x23233C, (0x8452, false)), // East Asian ideograph
    (0x224742, (0x6C65, false)), // East Asian ideograph
    (0x213D58, (0x5F70, false)), // East Asian ideograph
    (0x2D5E61, (0x6FF6, false)), // East Asian ideograph
    (0x6F4C3D, (0xB1CC, false)), // Korean hangul
    (0x69256A, (0x30EA, false)), // Katakana letter RI
    (0x294340, (0x94D6, false)), // East Asian ideograph
    (0x6F5752, (0xC887, false)), // Korean hangul
    (0x4B4B3E, (0xF9AD, false)), // East Asian ideograph
    (0x2D4746, (0x6C79, false)), // East Asian ideograph
    (0x2D6147, (0x99C8, false)), // East Asian ideograph
    (0x224747, (0x6C6F, false)), // East Asian ideograph
    (0x705F39, (0x5416, false)), // East Asian ideograph
    (0x224749, (0x6C9D, false)), // East Asian ideograph
    (0x275D5C, (0x94DC, false)), // East Asian ideograph
    (0x295433, (0x9AA7, false)), // East Asian ideograph
    (0x2F4A2E, (0x90B4, false)), // East Asian ideograph
    (0x22474A, (0x6C69, false)), // East Asian ideograph
    (0x22474B, (0x6C9A, false)), // East Asian ideograph
    (0x21383B, (0x57F7, false)), // East Asian ideograph
    (0x22474C, (0x6C6D, false)), // East Asian ideograph
    (0x23474D, (0x9419, false)), // East Asian ideograph
    (0x275B47, (0x8F8D, false)), // East Asian ideograph
    (0x23474E, (0x940D, false)), // East Asian ideograph
    (0x275D5D, (0x94ED, false)), // East Asian ideograph
    (0x6F4A2F, (0xADFF, false)), // Korean hangul
    (0x234750, (0x9426, false)), // East Asian ideograph
    (0x6F5D4A, (0xD68C, false)), // Korean hangul
    (0x224751, (0x6C87, false)), // East Asian ideograph
    (0x39345B, (0x965E, false)), // East Asian ideograph
    (0x224752, (0x6C6E, false)), // East Asian ideograph
    (0x6F4E73, (0xB7A9, false)), // Korean hangul
    (0x69256D, (0x30ED, false)), // Katakana letter RO
    (0x4D4754, (0x9544, false)), // East Asian ideograph
    (0x294343, (0x94D2, false)), // East Asian ideograph
    (0x276D2E, (0x5326, false)), // East Asian ideograph
    (0x235A2C, (0x9CF8, false)), // East Asian ideograph
    (0x224756, (0x6C95, false)), // East Asian ideograph
    (0x234758, (0x9414, false)), // East Asian ideograph
    (0x275D5F, (0x94EC, false)), // East Asian ideograph
    (0x6F4A31, (0xAE01, false)), // Korean hangul
    (0x274759, (0x6CEA, false)), // East Asian ideograph
    (0x6F582D, (0xC9C8, false)), // Korean hangul
    (0x4C715A, (0x7EE6, false)), // East Asian ideograph
    (0x22475A, (0x6C82, false)), // East Asian ideograph
    (0x2D5340, (0x812C, false)), // East Asian ideograph
    (0x2D3B6E, (0x5D17, false)), // East Asian ideograph
    (0x232C24, (0x87BD, false)), // East Asian ideograph
    (0x23475C, (0x9422, false)), // East Asian ideograph
    (0x222C2B, (0x6092, false)), // East Asian ideograph
    (0x222C2C, (0x609D, false)), // East Asian ideograph
    (0x222C2D, (0x6081, false)), // East Asian ideograph
    (0x23475D, (0x9406, false)), // East Asian ideograph
    (0x232C30, (0x87F3, false)), // East Asian ideograph
    (0x232C31, (0x87F0, false)), // East Asian ideograph
    (0x222C32, (0x6097, false)), // East Asian ideograph
    (0x215673, (0x8725, false)), // East Asian ideograph
    (0x232C34, (0x87EA, false)), // East Asian ideograph
    (0x29475E, (0x9562, false)), // East Asian ideograph
    (0x232C36, (0x87DB, false)), // East Asian ideograph
    (0x232C37, (0x87E2, false)), // East Asian ideograph
    (0x232C39, (0x87EB, false)), // East Asian ideograph
    (0x222C3A, (0x6095, false)), // East Asian ideograph
    (0x2D475F, (0x51C4, false)), // East Asian ideograph
    (0x4D2C3C, (0x87E5, false)), // East Asian ideograph
    (0x222C3E, (0x60C7, false)), // East Asian ideograph
    (0x232C3F, (0x87F5, false)), // East Asian ideograph
    (0x217D48, (0x5B21, false)), // East Asian ideograph
    (0x234760, (0x9410, false)), // East Asian ideograph
    (0x222C42, (0x60B0, false)), // East Asian ideograph
    (0x6F4D33, (0xB36A, false)), // Korean hangul
    (0x222C46, (0x60BE, false)), // East Asian ideograph
    (0x232C47, (0x87E0, false)), // East Asian ideograph
    (0x222C48, (0x60D4, false)), // East Asian ideograph
    (0x232C49, (0x87DC, false)), // East Asian ideograph
    (0x232C4C, (0x87E3, false)), // East Asian ideograph
    (0x232C4D, (0x8801, false)), // East Asian ideograph
    (0x222C4E, (0x60CE, false)), // East Asian ideograph
    (0x232C4F, (0x8803, false)), // East Asian ideograph
    (0x232C50, (0x880A, false)), // East Asian ideograph
    (0x222C51, (0x60CF, false)), // East Asian ideograph
    (0x222C53, (0x60D9, false)), // East Asian ideograph
    (0x222C54, (0x60B3, false)), // East Asian ideograph
    (0x232C55, (0x87F6, false)), // East Asian ideograph
    (0x222C56, (0x60DD, false)), // East Asian ideograph
    (0x232C57, (0x87F7, false)), // East Asian ideograph
    (0x235A2F, (0x9D2A, false)), // East Asian ideograph
    (0x232C5C, (0x880B, false)), // East Asian ideograph
    (0x232C5D, (0x8806, false)), // East Asian ideograph
    (0x232C5F, (0x87FE, false)), // East Asian ideograph
    (0x222C60, (0x60B1, false)), // East Asian ideograph
    (0x232C61, (0x8810, false)), // East Asian ideograph
    (0x222C62, (0x60E3, false)), // East Asian ideograph
    (0x232C63, (0x8819, false)), // East Asian ideograph
    (0x232C64, (0x8811, false)), // East Asian ideograph
    (0x224766, (0x6CEF, false)), // East Asian ideograph
    (0x232C66, (0x8818, false)), // East Asian ideograph
    (0x222C67, (0x60E5, false)), // East Asian ideograph
    (0x222C69, (0x60DB, false)), // East Asian ideograph
    (0x232C6A, (0x8813, false)), // East Asian ideograph
    (0x232C6B, (0x8816, false)), // East Asian ideograph
    (0x6F5236, (0xBEE0, false)), // Korean hangul
    (0x275D62, (0x950C, false)), // East Asian ideograph
    (0x222C6E, (0x60E9, false)), // East Asian ideograph
    (0x692571, (0x30F1, false)), // Katakana letter WE
    (0x222C70, (0x6114, false)), // East Asian ideograph
    (0x274768, (0x6D45, false)), // East Asian ideograph
    (0x232C72, (0x8834, false)), // East Asian ideograph
    (0x232C73, (0x881C, false)), // East Asian ideograph
    (0x222C75, (0x6119, false)), // East Asian ideograph
    (0x234769, (0x93F7, false)), // East Asian ideograph
    (0x232C7A, (0x881B, false)), // East Asian ideograph
    (0x222C7C, (0x60FD, false)), // East Asian ideograph
    (0x222C7D, (0x610D, false)), // East Asian ideograph
    (0x6F5B41, (0xD1F4, false)), // Korean hangul
    (0x29323B, (0x8BCE, false)), // East Asian ideograph
    (0x287042, (0x7EA1, false)), // East Asian ideograph
    (0x4B476C, (0x51C5, false)), // East Asian ideograph
    (0x275D63, (0x9511, false)), // East Asian ideograph
    (0x6F4A35, (0xAE0D, false)), // Korean hangul
    (0x223E61, (0x6971, false)), // East Asian ideograph
    (0x22476E, (0x6CAD, false)), // East Asian ideograph (variant of 4C476E which maps to 6CAD)
    (0x2D5344, (0x8107, false)), // East Asian ideograph
    (0x23476F, (0x940E, false)), // East Asian ideograph
    (0x6F5B2E, (0xD14C, false)), // Korean hangul
    (0x29323C, (0x8BD2, false)), // East Asian ideograph
    (0x6F4E39, (0xB5B5, false)), // Korean hangul
    (0x224770, (0x6CAF, false)), // East Asian ideograph
    (0x295925, (0x9CCC, false)), // East Asian ideograph
    (0x234771, (0x9411, false)), // East Asian ideograph
    (0x692573, (0x30F3, false)), // Katakana letter N
    (0x275921, (0x8C04, false)), // East Asian ideograph
    (0x294349, (0x94D5, false)), // East Asian ideograph
    (0x2D386E, (0x58CA, false)), // East Asian ideograph
    (0x217E23, (0x5B62, false)), // East Asian ideograph
    (0x274774, (0x6E0A, false)), // East Asian ideograph
    (0x6F5B43, (0xD22C, false)), // Korean hangul
    (0x275551, (0x80E1, false)), // East Asian ideograph (duplicate simplified)
    (0x22496A, (0x6DAC, false)), // East Asian ideograph
    (0x6F5B3E, (0xD1B3, false)), // Korean hangul
    (0x225265, (0x7168, false)), // East Asian ideograph
    (0x2D467C, (0x6CB2, false)), // East Asian ideograph
    (0x29464A, (0x953C, false)), // East Asian ideograph
    (0x3F3E47, (0x5379, false)), // East Asian ideograph
    (0x21345F, (0x5352, false)), // East Asian ideograph
    (0x274777, (0x6CA6, false)), // East Asian ideograph
    (0x2D6159, (0x9AC4, false)), // East Asian ideograph
    (0x213223, (0x5021, false)), // East Asian ideograph
    (0x234779, (0x9429, false)), // East Asian ideograph
    (0x213224, (0x500B, false)), // East Asian ideograph
    (0x695457, (0x58B8, false)), // East Asian ideograph
    (0x22477A, (0x6CBA, false)), // East Asian ideograph
    (0x223225, (0x6387, false)), // East Asian ideograph
    (0x22477B, (0x7553, false)), // East Asian ideograph
    (0x223226, (0x637A, false)), // East Asian ideograph
    (0x6F4B65, (0xB0C5, false)), // Korean hangul
    (0x6F4A38, (0xAE34, false)), // Korean hangul
    (0x213460, (0x5354, false)), // East Asian ideograph
    (0x233227, (0x8A51, false)), // East Asian ideograph
    (0x27477D, (0x6D8C, false)), // East Asian ideograph
    (0x213228, (0x4FF3, false)), // East Asian ideograph
    (0x6F5330, (0xC136, false)), // Korean hangul
    (0x3F3D6F, (0x8986, false)), // East Asian ideograph
    (0x287272, (0x7F21, false)), // East Asian ideograph
    (0x6F4C44, (0xB205, false)), // Korean hangul
    (0x213229, (0x502D, false)), // East Asian ideograph
    (0x28742E, (0x7F42, false)), // East Asian ideograph
    (0x6F4F3F, (0xB871, false)), // Korean hangul
    (0x22322A, (0x6386, false)), // East Asian ideograph
    (0x4C5C61, (0x74F4, false)), // East Asian ideograph (variant of 225C61 which maps to 74F4)
    (0x6F4E7C, (0xB7F0, false)), // Korean hangul
    (0x6F5237, (0xBEE3, false)), // Korean hangul
    (0x21722B, (0x55F9, false)), // East Asian ideograph
    (0x692576, (0x30F6, false)), // Katakana letter small KE
    (0x223860, (0x6673, false)), // East Asian ideograph
    (0x21322D, (0x502B, false)), // East Asian ideograph
    (0x6F5D4C, (0xD69F, false)), // Korean hangul
    (0x6F5B46, (0xD234, false)), // Korean hangul
    (0x21322E, (0x505C, false)), // East Asian ideograph
    (0x22496D, (0x6DD5, false)), // East Asian ideograph
    (0x232B53, (0x8785, false)), // East Asian ideograph
    (0x21322F, (0x504F, false)), // East Asian ideograph
    (0x225F2F, (0x760F, false)), // East Asian ideograph
    (0x292657, (0x835F, false)), // East Asian ideograph
    (0x233230, (0x8A56, false)), // East Asian ideograph
    (0x232D23, (0x8828, false)), // East Asian ideograph
    (0x217231, (0x560C, false)), // East Asian ideograph
    (0x232D2A, (0x8832, false)), // East Asian ideograph
    (0x222D2C, (0x6110, false)), // East Asian ideograph
    (0x232D2E, (0x882E, false)), // East Asian ideograph
    (0x213E35, (0x600E, false)), // East Asian ideograph
    (0x232D32, (0x882D, false)), // East Asian ideograph
    (0x213233, (0x5049, false)), // East Asian ideograph
    (0x222D34, (0x60F2, false)), // East Asian ideograph
    (0x222D37, (0x6125, false)), // East Asian ideograph
    (0x277234, (0x551B, false)), // East Asian ideograph
    (0x222D3B, (0x60F8, false)), // East Asian ideograph
    (0x232D3C, (0x883C, false)), // East Asian ideograph
    (0x6F4E7E, (0xB7FC, false)), // Korean hangul
    (0x273235, (0x4FA7, false)), // East Asian ideograph
    (0x222D41, (0x60FC, false)), // East Asian ideograph
    (0x232D42, (0x4610, false)), // East Asian ideograph (not in Unicode)
    (0x275926, (0x8C1A, false)), // East Asian ideograph
    (0x232D44, (0x8844, false)), // East Asian ideograph
    (0x227236, (0x7DF9, false)), // East Asian ideograph
    (0x222D48, (0x6149, false)), // East Asian ideograph
    (0x222D4A, (0x614A, false)), // East Asian ideograph
    (0x232D4B, (0x8847, false)), // East Asian ideograph
    (0x222D4E, (0x612B, false)), // East Asian ideograph
    (0x287275, (0x7D77, false)), // East Asian ideograph
    (0x222D50, (0x6129, false)), // East Asian ideograph
    (0x222D51, (0x6150, false)), // East Asian ideograph
    (0x232D53, (0x884E, false)), // East Asian ideograph
    (0x232D56, (0x8852, false)), // East Asian ideograph
    (0x233239, (0x8A48, false)), // East Asian ideograph
    (0x222D58, (0x6130, false)), // East Asian ideograph
    (0x232D59, (0x8856, false)), // East Asian ideograph
    (0x232D5A, (0x8855, false)), // East Asian ideograph
    (0x222D5B, (0x6141, false)), // East Asian ideograph
    (0x21323A, (0x5055, false)), // East Asian ideograph
    (0x232D5E, (0x885C, false)), // East Asian ideograph
    (0x232D5F, (0x885A, false)), // East Asian ideograph
    (0x222D61, (0x6146, false)), // East Asian ideograph
    (0x22323B, (0x6390, false)), // East Asian ideograph
    (0x222D66, (0x615E, false)), // East Asian ideograph
    (0x222D67, (0x6175, false)), // East Asian ideograph
    (0x222D68, (0x6174, false)), // East Asian ideograph
    (0x232D69, (0x8869, false)), // East Asian ideograph
    (0x21316C, (0x5009, false)), // East Asian ideograph
    (0x222D6B, (0x6183, false)), // East Asian ideograph
    (0x232D6D, (0x886D, false)), // East Asian ideograph
    (0x232D6E, (0x887A, false)), // East Asian ideograph
    (0x21323D, (0x508D, false)), // East Asian ideograph
    (0x222D70, (0x6171, false)), // East Asian ideograph
    (0x232D71, (0x8875, false)), // East Asian ideograph
    (0x222757, (0x5E5E, false)), // East Asian ideograph
    (0x222D74, (0x616A, false)), // East Asian ideograph
    (0x232D75, (0x8872, false)), // East Asian ideograph
    (0x6F5045, (0xBB0F, false)), // Korean hangul
    (0x222D77, (0x6173, false)), // East Asian ideograph
    (0x232D79, (0x887D, false)), // East Asian ideograph
    (0x455564, (0x6FDB, false)), // East Asian ideograph (Version J extension)
    (0x222D7B, (0x6153, false)), // East Asian ideograph
    (0x224234, (0x6A99, false)), // East Asian ideograph
    (0x232D7D, (0x887F, false)), // East Asian ideograph
    (0x232D7E, (0x887E, false)), // East Asian ideograph
    (0x275928, (0x8BB3, false)), // East Asian ideograph
    (0x294350, (0x94DF, false)), // East Asian ideograph
    (0x233240, (0x8A3D, false)), // East Asian ideograph
    (0x696126, (0x74F2, false)), // East Asian ideograph
    (0x6F567B, (0xC794, false)), // Korean hangul
    (0x273241, (0x6770, false)), // East Asian ideograph
    (0x6F5874, (0xCC0D, false)), // Korean hangul
    (0x6F5B4A, (0xD241, false)), // Korean hangul
    (0x213242, (0x5080, false)), // East Asian ideograph
    (0x4B5C5B, (0x8FBA, false)), // East Asian ideograph
    (0x223243, (0x63DE, false)), // East Asian ideograph
    (0x6F5238, (0xBEE4, false)), // Korean hangul
    (0x213244, (0x5098, false)), // East Asian ideograph
    (0x6F4A3E, (0xAE44, false)), // Korean hangul
    (0x275929, (0x8C10, false)), // East Asian ideograph
    (0x2D4539, (0x6406, false)), // East Asian ideograph
    (0x213246, (0x50B3, false)), // East Asian ideograph
    (0x274C60, (0x75A1, false)), // East Asian ideograph
    (0x6F5B4B, (0xD264, false)), // Korean hangul
    (0x273247, (0x503A, false)), // East Asian ideograph
    (0x227248, (0x7DF6, false)), // East Asian ideograph
    (0x23492E, (0x9585, false)), // East Asian ideograph
    (0x213249, (0x50C5, false)), // East Asian ideograph
    (0x6F4A3F, (0xAE45, false)), // Korean hangul
    (0x27592A, (0x8C0D, false)), // East Asian ideograph
    (0x223866, (0x666D, false)), // East Asian ideograph
    (0x21724B, (0x564B, false)), // East Asian ideograph
    (0x274C61, (0x759F, false)), // East Asian ideograph
    (0x6F5B4C, (0xD277, false)), // Korean hangul
    (0x21324C, (0x50B7, false)), // East Asian ideograph
    (0x393246, (0x4F1D, false)), // East Asian ideograph
    (0x21324D, (0x50AF, false)), // East Asian ideograph
    (0x275D6E, (0x9530, false)), // East Asian ideograph
    (0x6F4A40, (0xAE4A, false)), // Korean hangul
    (0x27592B, (0x8C0B, false)), // East Asian ideograph
    (0x6F5830, (0xC9D1, false)), // Korean hangul
    (0x21324F, (0x50EE, false)), // East Asian ideograph
    (0x213250, (0x50F1, false)), // East Asian ideograph
    (0x274C62, (0x75EA, false)), // East Asian ideograph
    (0x333963, (0x59C9, false)), // East Asian ideograph
    (0x213251, (0x50E5, false)), // East Asian ideograph
    (0x217252, (0x5640, false)), // East Asian ideograph
    (0x292433, (0x8298, false)), // East Asian ideograph (duplicate simplified)
    (0x69515E, (0x51E9, false)), // East Asian ideograph
    (0x287253, (0x7F12, false)), // East Asian ideograph
    (0x6F575A, (0xC89F, false)), // Korean hangul
    (0x223B63, (0x67D8, false)), // East Asian ideograph
    (0x213255, (0x50D5, false)), // East Asian ideograph
    (0x274C63, (0x75AF, false)), // East Asian ideograph
    (0x4C523A, (0x717A, false)), // East Asian ideograph
    (0x213256, (0x507D, false)), // East Asian ideograph
    (0x234D74, (0x97AB, false)), // East Asian ideograph
    (0x22674B, (0x798A, false)), // East Asian ideograph
    (0x213257, (0x50CF, false)), // East Asian ideograph
    (0x234931, (0x958C, false)), // East Asian ideograph
    (0x213258, (0x50D1, false)), // East Asian ideograph
    (0x224235, (0x6A9D, false)), // East Asian ideograph
    (0x27592D, (0x8C13, false)), // East Asian ideograph
    (0x294355, (0x94EB, false)), // East Asian ideograph
    (0x273259, (0x4EEA, false)), // East Asian ideograph
    (0x6F567C, (0xC796, false)), // Korean hangul
    (0x21325A, (0x5104, false)), // East Asian ideograph
    (0x6F5B4F, (0xD288, false)), // Korean hangul
    (0x222E23, (0x618B, false)), // East Asian ideograph
    (0x2D5129, (0x7D25, false)), // East Asian ideograph
    (0x232E28, (0x88A2, false)), // East Asian ideograph
    (0x21325C, (0x50F5, false)), // East Asian ideograph
    (0x232E2A, (0x88A4, false)), // East Asian ideograph
    (0x222E2C, (0x616F, false)), // East Asian ideograph
    (0x222E2D, (0x6165, false)), // East Asian ideograph
    (0x6F5239, (0xBEE5, false)), // Korean hangul
    (0x232E2F, (0x88AA, false)), // East Asian ideograph
    (0x276135, (0x9A7E, false)), // East Asian ideograph
    (0x222E32, (0x619D, false)), // East Asian ideograph
    (0x222E33, (0x61A6, false)), // East Asian ideograph
    (0x232E34, (0x889A, false)), // East Asian ideograph
    (0x27325E, (0x4FAC, false)), // East Asian ideograph
    (0x235A3F, (0x9D1E, false)), // East Asian ideograph
    (0x232E3A, (0x8890, false)), // East Asian ideograph
    (0x232E3B, (0x888C, false)), // East Asian ideograph
    (0x232E3D, (0x88A0, false)), // East Asian ideograph
    (0x232E40, (0x8899, false)), // East Asian ideograph
    (0x213260, (0x5108, false)), // East Asian ideograph
    (0x222E42, (0x619C, false)), // East Asian ideograph
    (0x222E43, (0x61AF, false)), // East Asian ideograph
    (0x232E45, (0x8897, false)), // East Asian ideograph
    (0x222E46, (0x6197, false)), // East Asian ideograph
    (0x222E47, (0x61AD, false)), // East Asian ideograph
    (0x232E48, (0x88C9, false)), // East Asian ideograph
    (0x232E49, (0x88BF, false)), // East Asian ideograph
    (0x232E4A, (0x88BA, false)), // East Asian ideograph
    (0x222E4C, (0x6192, false)), // East Asian ideograph
    (0x213262, (0x5110, false)), // East Asian ideograph
    (0x232E4F, (0x88C0, false)), // East Asian ideograph
    (0x6F4A44, (0xAE4D, false)), // Korean hangul
    (0x232E51, (0x88B2, false)), // East Asian ideograph
    (0x222E52, (0x61AE, false)), // East Asian ideograph
    (0x213263, (0x5118, false)), // East Asian ideograph
    (0x232E54, (0x88BC, false)), // East Asian ideograph
    (0x222E55, (0x618D, false)), // East Asian ideograph
    (0x232E57, (0x88B7, false)), // East Asian ideograph
    (0x232E59, (0x88BD, false)), // East Asian ideograph
    (0x232E5A, (0x88C4, false)), // East Asian ideograph
    (0x2D313A, (0x62BB, false)), // East Asian ideograph
    (0x222E5C, (0x61CC, false)), // East Asian ideograph
    (0x222E5D, (0x61C6, false)), // East Asian ideograph
    (0x232E5E, (0x88CB, false)), // East Asian ideograph
    (0x213265, (0x5114, false)), // East Asian ideograph
    (0x232E60, (0x88CC, false)), // East Asian ideograph
    (0x232E62, (0x88DB, false)), // East Asian ideograph
    (0x232E64, (0x88CE, false)), // East Asian ideograph
    (0x224535, (0x6BA3, false)), // East Asian ideograph
    (0x234934, (0x9597, false)), // East Asian ideograph
    (0x222E68, (0x61BA, false)), // East Asian ideograph
    (0x222E6A, (0x61B8, false)), // East Asian ideograph
    (0x273267, (0x507F, false)), // East Asian ideograph
    (0x6F4A45, (0xAE4E, false)), // Korean hangul
    (0x275930, (0x8C15, false)), // East Asian ideograph
    (0x294358, (0x94EF, false)), // East Asian ideograph
    (0x232E71, (0x88F1, false)), // East Asian ideograph
    (0x232E72, (0x88FE, false)), // East Asian ideograph
    (0x6F5D66, (0xD734, false)), // Korean hangul
    (0x232E75, (0x88F2, false)), // East Asian ideograph
    (0x233269, (0x8A8F, false)), // East Asian ideograph
    (0x232E78, (0x8900, false)), // East Asian ideograph
    (0x213F2E, (0x6176, false)), // East Asian ideograph
    (0x232E7A, (0x88F0, false)), // East Asian ideograph
    (0x6F5B52, (0xD293, false)), // Korean hangul
    (0x222E7D, (0x61DC, false)), // East Asian ideograph
    (0x222E7E, (0x61DF, false)), // East Asian ideograph
    (0x69723B, (0x9B96, false)), // East Asian ideograph
    (0x21326B, (0x513C, false)), // East Asian ideograph
    (0x276069, (0x996A, false)), // East Asian ideograph
    (0x294359, (0x94E5, false)), // East Asian ideograph
    (0x4B5434, (0x6319, false)), // East Asian ideograph
    (0x6F5B53, (0xD295, false)), // Korean hangul
    (0x21326F, (0x5145, false)), // East Asian ideograph
    (0x223270, (0x63BE, false)), // East Asian ideograph
    (0x234936, (0x958E, false)), // East Asian ideograph
    (0x2D4249, (0x53D9, false)), // East Asian ideograph
    (0x213271, (0x5146, false)), // East Asian ideograph
    (0x224236, (0x6A7E, false)), // East Asian ideograph
    (0x6F4A47, (0xAE54, false)), // Korean hangul
    (0x275932, (0x8C26, false)), // East Asian ideograph
    (0x217272, (0x5660, false)), // East Asian ideograph
    (0x295834, (0x9CBB, false)), // East Asian ideograph
    (0x223273, (0x63DD, false)), // East Asian ideograph
    (0x6F5B54, (0xD29C, false)), // Korean hangul
    (0x22497B, (0x6DBF, false)), // East Asian ideograph
    (0x213275, (0x514C, false)), // East Asian ideograph
    (0x6F523A, (0xBEEC, false)), // Korean hangul
    (0x2D3A26, (0x5A3F, false)), // East Asian ideograph
    (0x6F5D21, (0xD5D9, false)), // Korean hangul
    (0x283F5C, (0x6769, false)), // East Asian ideograph
    (0x29435B, (0x94E3, false)), // East Asian ideograph
    (0x213277, (0x514D, false)), // East Asian ideograph
    (0x6F5D22, (0xD5DB, false)), // Korean hangul
    (0x2D5D23, (0x9167, false)), // East Asian ideograph
    (0x274C6A, (0x75AE, false)), // East Asian ideograph
    (0x6F4C2E, (0xB158, false)), // Korean hangul
    (0x213279, (0x5154, false)), // East Asian ideograph
    (0x705F54, (0x54B4, false)), // East Asian ideograph
    (0x6F5D24, (0xD5E4, false)), // Korean hangul
    (0x29324F, (0x8BD6, false)), // East Asian ideograph
    (0x393460, (0x604A, false)), // East Asian ideograph
    (0x273859, (0x5C18, false)), // East Asian ideograph
    (0x225D25, (0x750E, false)), // East Asian ideograph
    (0x21327B, (0x5157, false)), // East Asian ideograph
    (0x6F5D26, (0xD5E8, false)), // Korean hangul
    (0x275934, (0x8BB2, false)), // East Asian ideograph
    (0x223870, (0x6684, false)), // East Asian ideograph
    (0x235D27, (0x9E0E, false)), // East Asian ideograph
    (0x6F577C, (0xC961, false)), // Korean hangul
    (0x21327D, (0x5162, false)), // East Asian ideograph
    (0x225D28, (0x750D, false)), // East Asian ideograph
    (0x213E38, (0x6059, false)), // East Asian ideograph
    (0x23327E, (0x8AB6, false)), // East Asian ideograph
    (0x295D29, (0x9E71, false)), // East Asian ideograph
    (0x293250, (0x8BD3, false)), // East Asian ideograph
    (0x275D2A, (0x9154, false)), // East Asian ideograph
    (0x235D2B, (0x9E11, false)), // East Asian ideograph
    (0x2F4A4A, (0x5F8F, false)), // East Asian ideograph
    (0x275935, (0x8C0E, false)), // East Asian ideograph
    (0x225D2C, (0x7511, false)), // East Asian ideograph
    (0x225D2D, (0x750F, false)), // East Asian ideograph
    (0x2D3140, (0x4F32, false)), // East Asian ideograph
    (0x6F5B57, (0xD2AC, false)), // Korean hangul
    (0x6F5D2E, (0xD604, false)), // Korean hangul
    (0x215D2F, (0x919E, false)), // East Asian ideograph
    (0x275D79, (0x952E, false)), // East Asian ideograph
    (0x275D30, (0x4E11, false)), // East Asian ideograph
    (0x6F4A4B, (0xAE61, false)), // Korean hangul
    (0x232F23, (0x88EF, false)), // East Asian ideograph
    (0x232F24, (0x8903, false)), // East Asian ideograph
    (0x39456D, (0x826B, false)), // East Asian ideograph
    (0x6F5758, (0xC89C, false)), // Korean hangul
    (0x215D31, (0x91AB, false)), // East Asian ideograph
    (0x225648, (0x72A8, false)), // East Asian ideograph
    (0x222F29, (0x61F3, false)), // East Asian ideograph
    (0x275D32, (0x9171, false)), // East Asian ideograph
    (0x274C6D, (0x75E8, false)), // East Asian ideograph
    (0x212F30, (0x3007, false)), // East Asian ideograph (number zero)
    (0x225D33, (0x7513, false)), // East Asian ideograph
    (0x232F35, (0x8906, false)), // East Asian ideograph
    (0x232F36, (0x890C, false)), // East Asian ideograph
    (0x232F37, (0x8919, false)), // East Asian ideograph
    (0x215D34, (0x91C0, false)), // East Asian ideograph
    (0x232F3D, (0x890A, false)), // East Asian ideograph
    (0x6F4B69, (0xB0D0, false)), // Korean hangul
    (0x215D35, (0x91C1, false)), // East Asian ideograph
    (0x6F4A4C, (0xAE62, false)), // Korean hangul
    (0x222F41, (0x6204, false)), // East Asian ideograph
    (0x235742, (0x9B9E, false)), // East Asian ideograph
    (0x222F43, (0x6207, false)), // East Asian ideograph
    (0x222F44, (0x6209, false)), // East Asian ideograph
    (0x232F45, (0x892F, false)), // East Asian ideograph
    (0x232F47, (0x8930, false)), // East Asian ideograph
    (0x345E47, (0x75FE, false)), // East Asian ideograph
    (0x235D37, (0x9E18, false)), // East Asian ideograph
    (0x274C6E, (0x7597, false)), // East Asian ideograph
    (0x232F4E, (0x8921, false)), // East Asian ideograph
    (0x232F4F, (0x8927, false)), // East Asian ideograph
    (0x232F51, (0x891F, false)), // East Asian ideograph
    (0x232F53, (0x8931, false)), // East Asian ideograph
    (0x232F54, (0x891E, false)), // East Asian ideograph
    (0x295029, (0x98A5, false)), // East Asian ideograph
    (0x232F56, (0x8926, false)), // East Asian ideograph
    (0x232F57, (0x8922, false)), // East Asian ideograph
    (0x232F5A, (0x8935, false)), // East Asian ideograph
    (0x222F5B, (0x6225, false)), // East Asian ideograph
    (0x232F5D, (0x8941, false)), // East Asian ideograph
    (0x275938, (0x8C22, false)), // East Asian ideograph
    (0x232F60, (0x8933, false)), // East Asian ideograph
    (0x222F61, (0x6229, false)), // East Asian ideograph
    (0x235D3B, (0x9E1D, false)), // East Asian ideograph
    (0x232F66, (0x8954, false)), // East Asian ideograph
    (0x222F67, (0x622D, false)), // East Asian ideograph
    (0x6F5D50, (0xD6C5, false)), // Korean hangul
    (0x215D3C, (0x91CF, false)), // East Asian ideograph
    (0x6F5B5A, (0xD2B9, false)), // Korean hangul
    (0x222F6E, (0x6239, false)), // East Asian ideograph
    (0x222F6F, (0x623A, false)), // East Asian ideograph
    (0x222F70, (0x623D, false)), // East Asian ideograph
    (0x232F72, (0x8947, false)), // East Asian ideograph
    (0x27385A, (0x57AB, false)), // East Asian ideograph
    (0x222F75, (0x6243, false)), // East Asian ideograph
    (0x222F77, (0x6246, false)), // East Asian ideograph
    (0x222F78, (0x6245, false)), // East Asian ideograph
    (0x222F79, (0x624A, false)), // East Asian ideograph
    (0x232F7A, (0x894C, false)), // East Asian ideograph
    (0x232F7B, (0x8946, false)), // East Asian ideograph
    (0x222F7C, (0x625E, false)), // East Asian ideograph
    (0x295859, (0x9CC6, false)), // East Asian ideograph
    (0x275D40, (0x9489, false)), // East Asian ideograph
    (0x275D41, (0x948A, false)), // East Asian ideograph
    (0x6F5B5B, (0xD2BC, false)), // Korean hangul
    (0x215D42, (0x91DC, false)), // East Asian ideograph
    (0x6F4E3A, (0xB5BB, false)), // Korean hangul
    (0x275D43, (0x9497, false)), // East Asian ideograph
    (0x45604E, (0x984F, false)), // East Asian ideograph
    (0x215D44, (0x91E6, false)), // East Asian ideograph
    (0x6F4A4F, (0xAE69, false)), // Korean hangul
    (0x27593A, (0x8C2C, false)), // East Asian ideograph
    (0x273721, (0x545C, false)), // East Asian ideograph
    (0x275D45, (0x9493, false)), // East Asian ideograph
    (0x2D535E, (0x8193, false)), // East Asian ideograph
    (0x275D46, (0x948F, false)), // East Asian ideograph
    (0x33632B, (0x7ADC, false)), // East Asian ideograph
    (0x6F5B5C, (0xD2BF, false)), // Korean hangul
    (0x705F5B, (0x54A3, false)), // East Asian ideograph
    (0x215D47, (0x9223, false)), // East Asian ideograph
    (0x293256, (0x8BE9, false)), // East Asian ideograph
    (0x2E493B, (0x6E7C, false)), // East Asian ideograph
    (0x275D48, (0x949D, false)), // East Asian ideograph
    (0x4B4921, (0x6CA2, false)), // East Asian ideograph
    (0x235D49, (0x9E84, false)), // East Asian ideograph
    (0x27606B, (0x996D, false)), // East Asian ideograph
    (0x27593B, (0x8C1F, false)), // East Asian ideograph
    (0x6F5759, (0xC89D, false)), // Korean hangul
    (0x275D4A, (0x94A0, false)), // East Asian ideograph
    (0x215D4B, (0x9214, false)), // East Asian ideograph
    (0x6F5B5D, (0xD2C0, false)), // Korean hangul
    (0x275D4C, (0x94A7, false)), // East Asian ideograph
    (0x284C2E, (0x6D52, false)), // East Asian ideograph
    (0x697246, (0x9BD1, false)), // East Asian ideograph
    (0x275D4D, (0x94A4, false)), // East Asian ideograph
    (0x2D3356, (0x5211, false)), // East Asian ideograph (not in Unicode)
    (0x6F4B6A, (0xB0D1, false)), // Korean hangul
    (0x6F5D4E, (0xD6A8, false)), // Korean hangul
    (0x6F4A51, (0xAE70, false)), // Korean hangul
    (0x27593C, (0x8BC6, false)), // East Asian ideograph
    (0x294364, (0x94F7, false)), // East Asian ideograph
    (0x233C77, (0x8FCB, false)), // East Asian ideograph
    (0x213A38, (0x5AC2, false)), // East Asian ideograph
    (0x275D50, (0x94B9, false)), // East Asian ideograph
    (0x2D3147, (0x5002, false)), // East Asian ideograph
    (0x6F5B5E, (0xD2C8, false)), // Korean hangul
    (0x215D51, (0x923D, false)), // East Asian ideograph
    (0x396074, (0x55B0, false)), // East Asian ideograph
    (0x215D52, (0x923E, false)), // East Asian ideograph
    (0x6F523C, (0xBF09, false)), // Korean hangul
    (0x275D53, (0x94BE, false)), // East Asian ideograph
    (0x21347A, (0x53AD, false)), // East Asian ideograph
    (0x213037, (0x4E38, false)), // East Asian ideograph
    (0x4B4B63, (0x749C, false)), // East Asian ideograph
    (0x215D55, (0x925B, false)), // East Asian ideograph
    (0x6F5B5F, (0xD2C9, false)), // Korean hangul
    (0x275D56, (0x94A9, false)), // East Asian ideograph
    (0x22675C, (0x799A, false)), // East Asian ideograph
    (0x275D57, (0x94C2, false)), // East Asian ideograph
    (0x234942, (0x95AC, false)), // East Asian ideograph
    (0x225D58, (0x7547, false)), // East Asian ideograph
    (0x6F4A53, (0xAE79, false)), // Korean hangul
    (0x224830, (0x6CD1, false)), // East Asian ideograph
    (0x275D59, (0x94F0, false)), // East Asian ideograph
    (0x235A4F, (0x9D41, false)), // East Asian ideograph
    (0x275D5A, (0x94F6, false)), // East Asian ideograph
    (0x274C75, (0x765E, false)), // East Asian ideograph
    (0x213021, (0x4E00, false)), // East Asian ideograph
    (0x213022, (0x4E01, false)), // East Asian ideograph
    (0x215D5B, (0x92AC, false)), // East Asian ideograph
    (0x213024, (0x4E09, false)), // East Asian ideograph
    (0x213025, (0x4E0B, false)), // East Asian ideograph
    (0x223026, (0x6268, false)), // East Asian ideograph
    (0x213027, (0x4E08, false)), // East Asian ideograph
    (0x223028, (0x6260, false)), // East Asian ideograph
    (0x233029, (0x895B, false)), // East Asian ideograph
    (0x21302A, (0x4E0D, false)), // East Asian ideograph
    (0x21302B, (0x4E14, false)), // East Asian ideograph
    (0x22302C, (0x6262, false)), // East Asian ideograph
    (0x21302D, (0x4E16, false)), // East Asian ideograph
    (0x21302E, (0x4E15, false)), // East Asian ideograph
    (0x215D5D, (0x9298, false)), // East Asian ideograph
    (0x213030, (0x4E22, false)), // East Asian ideograph
    (0x233031, (0x8966, false)), // East Asian ideograph
    (0x223032, (0x628E, false)), // East Asian ideograph
    (0x213034, (0x4E2D, false)), // East Asian ideograph
    (0x275D5E, (0x94E2, false)), // East Asian ideograph
    (0x213036, (0x51E1, false)), // East Asian ideograph
    (0x233037, (0x896D, false)), // East Asian ideograph
    (0x213038, (0x4E39, false)), // East Asian ideograph
    (0x213039, (0x4E3B, false)), // East Asian ideograph
    (0x23303A, (0x896B, false)), // East Asian ideograph
    (0x23303B, (0x896E, false)), // East Asian ideograph
    (0x23303C, (0x896C, false)), // East Asian ideograph
    (0x21303D, (0x4E4B, false)), // East Asian ideograph
    (0x21303E, (0x5C39, false)), // East Asian ideograph
    (0x21303F, (0x4E4F, false)), // East Asian ideograph
    (0x213040, (0x4E4E, false)), // East Asian ideograph
    (0x233041, (0x8976, false)), // East Asian ideograph
    (0x233042, (0x8974, false)), // East Asian ideograph
    (0x223043, (0x6282, false)), // East Asian ideograph
    (0x213044, (0x4E56, false)), // East Asian ideograph
    (0x213045, (0x4E58, false)), // East Asian ideograph
    (0x213046, (0x4E59, false)), // East Asian ideograph
    (0x215D61, (0x929C, false)), // East Asian ideograph
    (0x213048, (0x4E5F, false)), // East Asian ideograph
    (0x233049, (0x897B, false)), // East Asian ideograph
    (0x23304A, (0x897C, false)), // East Asian ideograph
    (0x22304B, (0x629D, false)), // East Asian ideograph
    (0x27304C, (0x5E72, false)), // East Asian ideograph
    (0x225D62, (0x7564, false)), // East Asian ideograph
    (0x6F4A55, (0xAE7C, false)), // Korean hangul
    (0x213050, (0x4E8B, false)), // East Asian ideograph
    (0x213051, (0x4E8C, false)), // East Asian ideograph
    (0x213052, (0x4E8E, false)), // East Asian ideograph
    (0x233053, (0x8984, false)), // East Asian ideograph
    (0x213054, (0x4E94, false)), // East Asian ideograph
    (0x233055, (0x8985, false)), // East Asian ideograph
    (0x223056, (0x62A6, false)), // East Asian ideograph
    (0x213057, (0x4E99, false)), // East Asian ideograph (variant of 4B3057 which maps to 4E99)
    (0x273058, (0x4E9A, false)), // East Asian ideograph
    (0x215D64, (0x92B3, false)), // East Asian ideograph
    (0x21305A, (0x4E9F, false)), // East Asian ideograph
    (0x274C77, (0x7663, false)), // East Asian ideograph
    (0x21305C, (0x4EA6, false)), // East Asian ideograph
    (0x21305D, (0x4EA5, false)), // East Asian ideograph
    (0x21305E, (0x4EA4, false)), // East Asian ideograph
    (0x215D65, (0x92EA, false)), // East Asian ideograph
    (0x213060, (0x4EAB, false)), // East Asian ideograph
    (0x213061, (0x4EAC, false)), // East Asian ideograph
    (0x233062, (0x8991, false)), // East Asian ideograph
    (0x213063, (0x4EAE, false)), // East Asian ideograph
    (0x233064, (0x8997, false)), // East Asian ideograph
    (0x215D66, (0x92B7, false)), // East Asian ideograph
    (0x233066, (0x8998, false)), // East Asian ideograph
    (0x4B5830, (0x899A, false)), // East Asian ideograph
    (0x213068, (0x4EC3, false)), // East Asian ideograph
    (0x213069, (0x4EC4, false)), // East Asian ideograph
    (0x22306A, (0x62C3, false)), // East Asian ideograph
    (0x23306B, (0x899C, false)), // East Asian ideograph
    (0x21306C, (0x4EC7, false)), // East Asian ideograph
    (0x21306D, (0x4ECB, false)), // East Asian ideograph
    (0x21306E, (0x4EE4, false)), // East Asian ideograph
    (0x23306F, (0x89A1, false)), // East Asian ideograph
    (0x213070, (0x4ED5, false)), // East Asian ideograph
    (0x275D68, (0x9504, false)), // East Asian ideograph
    (0x223072, (0x630D, false)), // East Asian ideograph
    (0x213073, (0x4EE3, false)), // East Asian ideograph
    (0x213074, (0x4ED4, false)), // East Asian ideograph
    (0x213075, (0x4ED7, false)), // East Asian ideograph
    (0x233076, (0x89A5, false)), // East Asian ideograph
    (0x275D69, (0x9509, false)), // East Asian ideograph
    (0x213078, (0x4EFF, false)), // East Asian ideograph
    (0x233079, (0x89A9, false)), // East Asian ideograph
    (0x6F5B63, (0xD2F0, false)), // Korean hangul
    (0x21307C, (0x4EFB, false)), // East Asian ideograph
    (0x275D6A, (0x950B, false)), // East Asian ideograph
    (0x21307E, (0x4F15, false)), // East Asian ideograph
    (0x224547, (0x6BBD, false)), // East Asian ideograph
    (0x215D6B, (0x9320, false)), // East Asian ideograph
    (0x292A2F, (0x86F1, false)), // East Asian ideograph
    (0x6F5D6C, (0xD754, false)), // Korean hangul
    (0x29436A, (0x9512, false)), // East Asian ideograph
    (0x215D6D, (0x92F8, false)), // East Asian ideograph
    (0x235A53, (0x9D36, false)), // East Asian ideograph
    (0x225D6E, (0x757A, false)), // East Asian ideograph
    (0x6F535B, (0xC218, false)), // Korean hangul
    (0x274C79, (0x766B, false)), // East Asian ideograph
    (0x6F5B64, (0xD2F1, false)), // Korean hangul
    (0x275D6F, (0x9519, false)), // East Asian ideograph
    (0x29325E, (0x8BDC, false)), // East Asian ideograph
    (0x6F5721, (0xC7A1, false)), // Korean hangul
    (0x2F575F, (0x9ABE, false)), // East Asian ideograph
    (0x275D70, (0x94B1, false)), // East Asian ideograph
    (0x4B5832, (0x89B3, false)), // East Asian ideograph
    (0x225D71, (0x7577, false)), // East Asian ideograph
    (0x6F4A58, (0xAE85, false)), // Korean hangul
    (0x275D72, (0x9521, false)), // East Asian ideograph
    (0x22343C, (0x649D, false)), // East Asian ideograph
    (0x275D73, (0x94EE, false)), // East Asian ideograph
    (0x6F5B65, (0xD2F4, false)), // Korean hangul
    (0x275D74, (0x5F55, false)), // East Asian ideograph
    (0x6F5722, (0xC7A3, false)), // Korean hangul
    (0x69724E, (0x9BF2, false)), // East Asian ideograph
    (0x215D75, (0x9310, false)), // East Asian ideograph
    (0x234948, (0x95BC, false)), // East Asian ideograph
    (0x4B325F, (0x50BB, false)), // East Asian ideograph
    (0x215D76, (0x9326, false)), // East Asian ideograph
    (0x6F5835, (0xC9DA, false)), // Korean hangul
    (0x692153, (0x3009, false)), // Ideographic greater than sign
    (0x215D77, (0x934D, false)), // East Asian ideograph
    (0x2D632D, (0x4E80, false)), // East Asian ideograph
    (0x215D78, (0x9382, false)), // East Asian ideograph
    (0x274C7B, (0x53D1, false)), // East Asian ideograph
    (0x6F5B66, (0xD2F8, false)), // Korean hangul
    (0x225D79, (0x757D, false)), // East Asian ideograph
    (0x6F5432, (0xC2FC, false)), // Korean hangul
    (0x224824, (0x6CD8, false)), // East Asian ideograph
    (0x4B5C77, (0x9139, false)), // East Asian ideograph
    (0x235D7A, (0x9EB0, false)), // East Asian ideograph
    (0x6F5D7B, (0xD790, false)), // Korean hangul
    (0x27606D, (0x9974, false)), // East Asian ideograph
    (0x224826, (0x6CC6, false)), // East Asian ideograph
    (0x6F575B, (0xC8A0, false)), // Korean hangul
    (0x275D7C, (0x9505, false)), // East Asian ideograph
    (0x2E5452, (0x71FE, false)), // East Asian ideograph
    (0x234827, (0x93F4, false)), // East Asian ideograph
    (0x275D7D, (0x951A, false)), // East Asian ideograph
    (0x234828, (0x9436, false)), // East Asian ideograph
    (0x6F5B67, (0xD300, false)), // Korean hangul
    (0x275D7E, (0x953E, false)), // East Asian ideograph
    (0x224829, (0x6CE9, false)), // East Asian ideograph
    (0x226764, (0x799D, false)), // East Asian ideograph
    (0x23494A, (0x95CD, false)), // East Asian ideograph
    (0x395E3D, (0x9295, false)), // East Asian ideograph
    (0x6F4A5B, (0xAEBE, false)), // Korean hangul
    (0x23482B, (0x943B, false)), // East Asian ideograph
    (0x275946, (0x8BD1, false)), // East Asian ideograph
    (0x22527C, (0x717B, false)), // East Asian ideograph
    (0x23482D, (0x9424, false)), // East Asian ideograph
    (0x6F5B68, (0xD301, false)), // Korean hangul
    (0x6F5725, (0xC7A6, false)), // Korean hangul
    (0x284E42, (0x6D4D, false)), // East Asian ideograph
    (0x21482F, (0x6E34, false)), // East Asian ideograph
    (0x6F523E, (0xBF1D, false)), // Korean hangul
    (0x6F4A5C, (0xAEC0, false)), // Korean hangul
    (0x234830, (0x9437, false)), // East Asian ideograph
    (0x213122, (0x4F10, false)), // East Asian ideograph
    (0x213123, (0x4F0F, false)), // East Asian ideograph
    (0x213124, (0x4EF2, false)), // East Asian ideograph
    (0x223125, (0x62F5, false)), // East Asian ideograph
    (0x213126, (0x4EF3, false)), // East Asian ideograph
    (0x213127, (0x4EF6, false)), // East Asian ideograph
    (0x213128, (0x4EF0, false)), // East Asian ideograph
    (0x23312A, (0x89B8, false)), // East Asian ideograph
    (0x23312B, (0x89B7, false)), // East Asian ideograph
    (0x23312C, (0x89B6, false)), // East Asian ideograph
    (0x234832, (0x9440, false)), // East Asian ideograph
    (0x21312E, (0x4F57, false)), // East Asian ideograph
    (0x23312F, (0x89BC, false)), // East Asian ideograph
    (0x213130, (0x4F5E, false)), // East Asian ideograph
    (0x223131, (0x630C, false)), // East Asian ideograph
    (0x233132, (0x89BF, false)), // East Asian ideograph
    (0x213133, (0x4F55, false)), // East Asian ideograph
    (0x213134, (0x4F30, false)), // East Asian ideograph
    (0x213135, (0x4F50, false)), // East Asian ideograph
    (0x213136, (0x4F51, false)), // East Asian ideograph
    (0x223137, (0x62F6, false)), // East Asian ideograph
    (0x213138, (0x4F48, false)), // East Asian ideograph
    (0x213139, (0x4F46, false)), // East Asian ideograph
    (0x22313A, (0x6331, false)), // East Asian ideograph
    (0x23313B, (0x89D5, false)), // East Asian ideograph
    (0x21313C, (0x4F54, false)), // East Asian ideograph
    (0x21313D, (0x4F3C, false)), // East Asian ideograph
    (0x21313E, (0x4F63, false)), // East Asian ideograph
    (0x23313F, (0x89DA, false)), // East Asian ideograph
    (0x213140, (0x4F60, false)), // East Asian ideograph
    (0x213141, (0x4F2F, false)), // East Asian ideograph
    (0x223142, (0x6345, false)), // East Asian ideograph
    (0x233143, (0x89E5, false)), // East Asian ideograph
    (0x223144, (0x6343, false)), // East Asian ideograph
    (0x234836, (0x942D, false)), // East Asian ideograph
    (0x213146, (0x4F6F, false)), // East Asian ideograph
    (0x223147, (0x6353, false)), // East Asian ideograph
    (0x223148, (0x6364, false)), // East Asian ideograph
    (0x223149, (0x6336, false)), // East Asian ideograph
    (0x22314A, (0x6344, false)), // East Asian ideograph
    (0x224837, (0x6D1D, false)), // East Asian ideograph
    (0x23314C, (0x89E9, false)), // East Asian ideograph
    (0x23314D, (0x89EB, false)), // East Asian ideograph
    (0x21314E, (0x4F8B, false)), // East Asian ideograph
    (0x27314F, (0x4ED1, false)), // East Asian ideograph
    (0x234838, (0x9431, false)), // East Asian ideograph
    (0x213152, (0x4F7B, false)), // East Asian ideograph
    (0x233153, (0x89ED, false)), // East Asian ideograph
    (0x223154, (0x6339, false)), // East Asian ideograph
    (0x213155, (0x4F8F, false)), // East Asian ideograph
    (0x213156, (0x4F7E, false)), // East Asian ideograph
    (0x213157, (0x4FE1, false)), // East Asian ideograph
    (0x223158, (0x6357, false)), // East Asian ideograph
    (0x213159, (0x4FB5, false)), // East Asian ideograph
    (0x22315A, (0x633C, false)), // East Asian ideograph
    (0x22315B, (0x6358, false)), // East Asian ideograph
    (0x21315C, (0x4FDE, false)), // East Asian ideograph
    (0x27315D, (0x4FA0, false)), // East Asian ideograph
    (0x21315E, (0x4FCF, false)), // East Asian ideograph
    (0x22315F, (0x6354, false)), // East Asian ideograph
    (0x213160, (0x4FDA, false)), // East Asian ideograph
    (0x213161, (0x4FDD, false)), // East Asian ideograph
    (0x213162, (0x4FC3, false)), // East Asian ideograph
    (0x213163, (0x4FD8, false)), // East Asian ideograph
    (0x233164, (0x89F7, false)), // East Asian ideograph
    (0x213165, (0x4FCA, false)), // East Asian ideograph
    (0x213166, (0x4FAE, false)), // East Asian ideograph
    (0x213167, (0x4FD0, false)), // East Asian ideograph
    (0x223168, (0x637D, false)), // East Asian ideograph
    (0x273169, (0x7CFB, false)), // East Asian ideograph (duplicate simplified)
    (0x22316A, (0x63B6, false)), // East Asian ideograph
    (0x22316B, (0x6382, false)), // East Asian ideograph
    (0x27316C, (0x4ED3, false)), // East Asian ideograph
    (0x23316D, (0x8A07, false)), // East Asian ideograph
    (0x22316E, (0x639F, false)), // East Asian ideograph
    (0x21483D, (0x6E9D, false)), // East Asian ideograph
    (0x233170, (0x8A0F, false)), // East Asian ideograph
    (0x233171, (0x8A11, false)), // East Asian ideograph
    (0x233172, (0x8A12, false)), // East Asian ideograph
    (0x233173, (0x8A0D, false)), // East Asian ideograph
    (0x213174, (0x4FF8, false)), // East Asian ideograph
    (0x213175, (0x5028, false)), // East Asian ideograph
    (0x213176, (0x5014, false)), // East Asian ideograph
    (0x213177, (0x5016, false)), // East Asian ideograph
    (0x213178, (0x5029, false)), // East Asian ideograph
    (0x223179, (0x6381, false)), // East Asian ideograph
    (0x23317A, (0x8A27, false)), // East Asian ideograph
    (0x22317B, (0x6397, false)), // East Asian ideograph
    (0x21317C, (0x503C, false)), // East Asian ideograph
    (0x23317D, (0x8A29, false)), // East Asian ideograph
    (0x21317E, (0x4FFA, false)), // East Asian ideograph
    (0x234840, (0x9445, false)), // East Asian ideograph
    (0x274841, (0x6C85, false)), // East Asian ideograph
    (0x6F5B6C, (0xD30C, false)), // Korean hangul
    (0x234842, (0x9450, false)), // East Asian ideograph
    (0x273266, (0x4F18, false)), // East Asian ideograph
    (0x4B513B, (0x7CF8, false)), // East Asian ideograph
    (0x6F4B6D, (0xB0EC, false)), // Korean hangul
    (0x274844, (0x6E7F, false)), // East Asian ideograph
    (0x2D4845, (0x6E29, false)), // East Asian ideograph
    (0x4B4846, (0x78C6, false)), // East Asian ideograph
    (0x226F69, (0x7CC5, false)), // East Asian ideograph
    (0x274848, (0x6CA7, false)), // East Asian ideograph
    (0x4B3622, (0x8C18, false)), // East Asian ideograph
    (0x6F4A61, (0xAED0, false)), // Korean hangul
    (0x273733, (0x5578, false)), // East Asian ideograph
    (0x23484A, (0x944A, false)), // East Asian ideograph
    (0x27484B, (0x51C6, false)), // East Asian ideograph
    (0x6F5B6E, (0xD30E, false)), // Korean hangul
    (0x2F3639, (0x8C7C, false)), // East Asian ideograph
    (0x4B484C, (0x6F91, false)), // East Asian ideograph
    (0x22484D, (0x6D26, false)), // East Asian ideograph
    (0x22484E, (0x6D27, false)), // East Asian ideograph
    (0x294375, (0x9514, false)), // East Asian ideograph
    (0x22484F, (0x6D0F, false)), // East Asian ideograph
    (0x224850, (0x6D0A, false)), // East Asian ideograph
    (0x2D4466, (0x6973, false)), // East Asian ideograph
    (0x224851, (0x6D3F, false)), // East Asian ideograph
    (0x226329, (0x77BE, false)), // East Asian ideograph
    (0x234853, (0x9466, false)), // East Asian ideograph
    (0x47594E, (0x9C3A, false)), // East Asian ideograph
    (0x274854, (0x6E0D, false)), // East Asian ideograph
    (0x514E5B, (0x9271, false)), // East Asian ideograph
    (0x274855, (0x6DA8, false)), // East Asian ideograph
    (0x6F5B70, (0xD314, false)), // Korean hangul
    (0x274842, (0x706D, false)), // East Asian ideograph
    (0x6F572D, (0xC7BF, false)), // Korean hangul
    (0x284C41, (0x6CA4, false)), // East Asian ideograph
    (0x234560, (0x93BE, false)), // East Asian ideograph (not in Unicode)
    (0x29243A, (0x83BC, false)), // East Asian ideograph
    (0x274857, (0x6C49, false)), // East Asian ideograph
    (0x273B79, (0x5C9B, false)), // East Asian ideograph
    (0x234858, (0x9462, false)), // East Asian ideograph
    (0x2F252D, (0x6A22, false)), // East Asian ideograph
    (0x6F575D, (0xC8A8, false)), // Korean hangul
    (0x3F4621, (0x9A69, false)), // East Asian ideograph
    (0x274859, (0x6D9F, false)), // East Asian ideograph
    (0x286622, (0x7857, false)), // East Asian ideograph
    (0x22485A, (0x6D07, false)), // East Asian ideograph
    (0x4B4D7B, (0x77D7, false)), // East Asian ideograph (variant of 214D7B which maps to 77D7)
    (0x6F5B71, (0xD31C, false)), // Korean hangul
    (0x213221, (0x5018, false)), // East Asian ideograph
    (0x213222, (0x4FF1, false)), // East Asian ideograph
    (0x22485B, (0x6D04, false)), // East Asian ideograph
    (0x273224, (0x4E2A, false)), // East Asian ideograph
    (0x213225, (0x5019, false)), // East Asian ideograph
    (0x273226, (0x4F25, false)), // East Asian ideograph
    (0x223227, (0x638E, false)), // East Asian ideograph
    (0x233228, (0x8A4A, false)), // East Asian ideograph
    (0x22485C, (0x6CDA, false)), // East Asian ideograph
    (0x23322A, (0x8A4E, false)), // East Asian ideograph
    (0x21322B, (0x4FFE, false)), // East Asian ideograph
    (0x21322C, (0x502A, false)), // East Asian ideograph
    (0x27322D, (0x4F26, false)), // East Asian ideograph
    (0x27322E, (0x4EC3, false)), // East Asian ideograph (duplicate simplified)
    (0x22322F, (0x6375, false)), // East Asian ideograph
    (0x223230, (0x63AF, false)), // East Asian ideograph
    (0x213231, (0x5047, false)), // East Asian ideograph
    (0x213232, (0x505A, false)), // East Asian ideograph
    (0x273233, (0x4F1F, false)), // East Asian ideograph
    (0x213234, (0x5043, false)), // East Asian ideograph
    (0x23485E, (0x945E, false)), // East Asian ideograph
    (0x213236, (0x5076, false)), // East Asian ideograph
    (0x213237, (0x504E, false)), // East Asian ideograph
    (0x223238, (0x63B0, false)), // East Asian ideograph
    (0x223239, (0x63AE, false)), // East Asian ideograph
    (0x22323A, (0x637C, false)), // East Asian ideograph
    (0x27485F, (0x6EDE, false)), // East Asian ideograph
    (0x21323C, (0x5077, false)), // East Asian ideograph
    (0x22323D, (0x63AD, false)), // East Asian ideograph
    (0x27323E, (0x5BB6, false)), // East Asian ideograph
    (0x21323F, (0x5085, false)), // East Asian ideograph
    (0x273240, (0x5907, false)), // East Asian ideograph
    (0x224860, (0x6D2E, false)), // East Asian ideograph
    (0x233242, (0x8A45, false)), // East Asian ideograph
    (0x273243, (0x4F27, false)), // East Asian ideograph
    (0x273244, (0x4F1E, false)), // East Asian ideograph
    (0x213245, (0x50AD, false)), // East Asian ideograph
    (0x273246, (0x4F20, false)), // East Asian ideograph
    (0x224861, (0x6D35, false)), // East Asian ideograph
    (0x213248, (0x50B2, false)), // East Asian ideograph
    (0x273249, (0x4EC5, false)), // East Asian ideograph
    (0x27324A, (0x503E, false)), // East Asian ideograph
    (0x21324B, (0x50AC, false)), // East Asian ideograph
    (0x27324C, (0x4F24, false)), // East Asian ideograph
    (0x224862, (0x6D3A, false)), // East Asian ideograph
    (0x21324E, (0x50E7, false)), // East Asian ideograph
    (0x22324F, (0x63BD, false)), // East Asian ideograph
    (0x223250, (0x63C3, false)), // East Asian ideograph
    (0x273251, (0x4FA5, false)), // East Asian ideograph
    (0x223252, (0x63F5, false)), // East Asian ideograph
    (0x213253, (0x50ED, false)), // East Asian ideograph
    (0x213254, (0x50DA, false)), // East Asian ideograph
    (0x273255, (0x4EC6, false)), // East Asian ideograph
    (0x273256, (0x4F2A, false)), // East Asian ideograph
    (0x273257, (0x8C61, false)), // East Asian ideograph
    (0x273258, (0x4FA8, false)), // East Asian ideograph
    (0x233259, (0x8A82, false)), // East Asian ideograph
    (0x27325A, (0x4EBF, false)), // East Asian ideograph
    (0x22325B, (0x63E0, false)), // East Asian ideograph
    (0x22325C, (0x63D5, false)), // East Asian ideograph
    (0x23325D, (0x8A84, false)), // East Asian ideograph
    (0x23325E, (0x8A75, false)), // East Asian ideograph
    (0x274865, (0x6E14, false)), // East Asian ideograph
    (0x273260, (0x4FA9, false)), // East Asian ideograph
    (0x273261, (0x4FED, false)), // East Asian ideograph
    (0x273262, (0x50A7, false)), // East Asian ideograph
    (0x273263, (0x5C3D, false)), // East Asian ideograph (duplicate simplified)
    (0x213264, (0x5112, false)), // East Asian ideograph
    (0x273265, (0x4FE6, false)), // East Asian ideograph
    (0x223266, (0x63C5, false)), // East Asian ideograph (not in Unicode)
    (0x213267, (0x511F, false)), // East Asian ideograph
    (0x213268, (0x5121, false)), // East Asian ideograph
    (0x273269, (0x50A8, false)), // East Asian ideograph
    (0x21326A, (0x5137, false)), // East Asian ideograph
    (0x27326B, (0x4FE8, false)), // East Asian ideograph
    (0x21326C, (0x5140, false)), // East Asian ideograph
    (0x21326D, (0x5143, false)), // East Asian ideograph
    (0x21326E, (0x5141, false)), // East Asian ideograph
    (0x23326F, (0x8A96, false)), // East Asian ideograph
    (0x213270, (0x5144, false)), // East Asian ideograph
    (0x233271, (0x8A9A, false)), // East Asian ideograph
    (0x213272, (0x5149, false)), // East Asian ideograph
    (0x273273, (0x51F6, false)), // East Asian ideograph
    (0x213274, (0x5148, false)), // East Asian ideograph
    (0x274C3C, (0x8FED, false)), // East Asian ideograph
    (0x223276, (0x63D1, false)), // East Asian ideograph (not in Unicode)
    (0x234869, (0x946D, false)), // East Asian ideograph
    (0x213278, (0x5155, false)), // East Asian ideograph
    (0x223279, (0x63C4, false)), // East Asian ideograph
    (0x27327A, (0x513F, false)), // East Asian ideograph
    (0x27327B, (0x5156, false)), // East Asian ideograph
    (0x21327C, (0x515C, false)), // East Asian ideograph
    (0x22486A, (0x6D2B, false)), // East Asian ideograph
    (0x22327E, (0x6412, false)), // East Asian ideograph
    (0x2D4F7C, (0x7B5E, false)), // East Asian ideograph
    (0x22486B, (0x6D11, false)), // East Asian ideograph
    (0x27486C, (0x6CFC, false)), // East Asian ideograph
    (0x6F5838, (0xC9DC, false)), // Korean hangul
    (0x21304D, (0x4E82, false)), // East Asian ideograph
    (0x22486D, (0x6D24, false)), // East Asian ideograph
    (0x6F4E5C, (0xB760, false)), // Korean hangul
    (0x235C65, (0x9DEF, false)), // East Asian ideograph
    (0x293B3F, (0x8F7A, false)), // East Asian ideograph
    (0x27486E, (0x6DA7, false)), // East Asian ideograph
    (0x6F5B75, (0xD321, false)), // Korean hangul
    (0x27486F, (0x6D01, false)), // East Asian ideograph
    (0x6F4870, (0xAC1B, false)), // Korean hangul
    (0x6F4C49, (0xB214, false)), // Korean hangul
    (0x234871, (0x9477, false)), // East Asian ideograph
    (0x275954, (0x5C82, false)), // East Asian ideograph
    (0x2F252E, (0x8507, false)), // East Asian ideograph
    (0x6F4872, (0xAC1D, false)), // Korean hangul
    (0x2D315F, (0x4FA3, false)), // East Asian ideograph
    (0x235622, (0x9B35, false)), // East Asian ideograph
    (0x6F5B76, (0xD325, false)), // Korean hangul
    (0x2D4874, (0x6F5C, false)), // East Asian ideograph
    (0x6F4875, (0xAC24, false)), // Korean hangul
    (0x29457A, (0x9550, false)), // East Asian ideograph
    (0x6F4876, (0xAC2C, false)), // Korean hangul
    (0x227321, (0x7E35, false)), // East Asian ideograph
    (0x224877, (0x6DA5, false)), // East Asian ideograph
    (0x213322, (0x5168, false)), // East Asian ideograph
    (0x4B5361, (0x89D2, false)), // East Asian ideograph (duplicate simplified)
    (0x274878, (0x6E83, false)), // East Asian ideograph
    (0x213323, (0x5169, false)), // East Asian ideograph
    (0x455E21, (0x953A, false)), // East Asian ideograph
    (0x293271, (0x8BEE, false)), // East Asian ideograph
    (0x213324, (0x516B, false)), // East Asian ideograph
    (0x22455B, (0x6BD6, false)), // East Asian ideograph
    (0x6F487A, (0xAC31, false)), // Korean hangul
    (0x213325, (0x516D, false)), // East Asian ideograph
    (0x23487B, (0x9482, false)), // East Asian ideograph
    (0x27373D, (0x5480, false)), // East Asian ideograph
    (0x27487C, (0x6D53, false)), // East Asian ideograph
    (0x213327, (0x516C, false)), // East Asian ideograph
    (0x6F5D56, (0xD6E0, false)), // Korean hangul
    (0x22487D, (0x6D92, false)), // East Asian ideograph
    (0x217328, (0x568C, false)), // East Asian ideograph
    (0x6F487E, (0xAC54, false)), // Korean hangul
    (0x213329, (0x5175, false)), // East Asian ideograph
    (0x215F33, (0x9694, false)), // East Asian ideograph
    (0x276225, (0x9CCF, false)), // East Asian ideograph
    (0x2D332A, (0x4E0C, false)), // East Asian ideograph
    (0x2D3E2B, (0x6060, false)), // East Asian ideograph
    (0x21332B, (0x5177, false)), // East Asian ideograph
    (0x4B5F62, (0x7668, false)), // East Asian ideograph
    (0x3F4629, (0x4E97, false)), // East Asian ideograph
    (0x513051, (0x8CAE, false)), // East Asian ideograph
    (0x22332C, (0x6424, false)), // East Asian ideograph
    (0x274C3B, (0x7574, false)), // East Asian ideograph
    (0x23463C, (0x9389, false)), // East Asian ideograph
    (0x22732D, (0x7E52, false)), // East Asian ideograph
    (0x22534A, (0x719B, false)), // East Asian ideograph
    (0x6F5736, (0xC804, false)), // Korean hangul
    (0x215F34, (0x9699, false)), // East Asian ideograph
    (0x21332F, (0x5189, false)), // East Asian ideograph
    (0x6F4A6D, (0xAF3F, false)), // Korean hangul
    (0x275958, (0x4E30, false)), // East Asian ideograph
    (0x233321, (0x8ABE, false)), // East Asian ideograph
    (0x223322, (0x6410, false)), // East Asian ideograph
    (0x273323, (0x4E24, false)), // East Asian ideograph
    (0x223324, (0x6434, false)), // East Asian ideograph
    (0x233325, (0x8ACF, false)), // East Asian ideograph
    (0x213326, (0x516E, false)), // East Asian ideograph
    (0x233327, (0x8AC6, false)), // East Asian ideograph
    (0x213328, (0x5171, false)), // East Asian ideograph
    (0x223329, (0x641B, false)), // East Asian ideograph
    (0x21332A, (0x5176, false)), // East Asian ideograph
    (0x22332B, (0x6420, false)), // East Asian ideograph
    (0x23332C, (0x8AD1, false)), // East Asian ideograph
    (0x23332D, (0x8AD3, false)), // East Asian ideograph
    (0x21332E, (0x5180, false)), // East Asian ideograph
    (0x22332F, (0x6426, false)), // East Asian ideograph
    (0x213330, (0x518C, false)), // East Asian ideograph
    (0x233331, (0x8AAF, false)), // East Asian ideograph
    (0x213332, (0x5192, false)), // East Asian ideograph
    (0x233333, (0x8AD4, false)), // East Asian ideograph
    (0x213334, (0x5195, false)), // East Asian ideograph
    (0x213335, (0x6700, false)), // East Asian ideograph
    (0x213336, (0x5197, false)), // East Asian ideograph
    (0x213337, (0x51A0, false)), // East Asian ideograph
    (0x233338, (0x8AB9, false)), // East Asian ideograph
    (0x223339, (0x3013, false)), // East Asian ideograph (not found in unified han)
    (0x23333B, (0x8ADB, false)), // East Asian ideograph
    (0x21333C, (0x51B0, false)), // East Asian ideograph
    (0x22333D, (0x6421, false)), // East Asian ideograph
    (0x21333E, (0x51B7, false)), // East Asian ideograph
    (0x23333F, (0x8AD0, false)), // East Asian ideograph
    (0x233340, (0x8AD7, false)), // East Asian ideograph
    (0x213341, (0x51CC, false)), // East Asian ideograph
    (0x233344, (0x8AF3, false)), // East Asian ideograph
    (0x233336, (0x8ACD, false)), // East Asian ideograph
    (0x213347, (0x51F0, false)), // East Asian ideograph
    (0x273348, (0x51EF, false)), // East Asian ideograph
    (0x233349, (0x8B4C, false)), // East Asian ideograph
    (0x223337, (0x6418, false)), // East Asian ideograph
    (0x22334C, (0x6409, false)), // East Asian ideograph
    (0x21334D, (0x51F8, false)), // East Asian ideograph
    (0x23334E, (0x8AF6, false)), // East Asian ideograph
    (0x21334F, (0x5200, false)), // East Asian ideograph
    (0x213350, (0x5201, false)), // East Asian ideograph
    (0x223338, (0x640E, false)), // East Asian ideograph
    (0x213352, (0x5207, false)), // East Asian ideograph
    (0x223353, (0x6440, false)), // East Asian ideograph
    (0x213354, (0x5208, false)), // East Asian ideograph
    (0x213355, (0x520A, false)), // East Asian ideograph
    (0x233356, (0x8B03, false)), // East Asian ideograph
    (0x233357, (0x8AE4, false)), // East Asian ideograph
    (0x233359, (0x8B14, false)), // East Asian ideograph
    (0x21335A, (0x5224, false)), // East Asian ideograph
    (0x21335B, (0x5225, false)), // East Asian ideograph
    (0x235749, (0x9B86, false)), // East Asian ideograph
    (0x23335D, (0x8AFC, false)), // East Asian ideograph
    (0x21335E, (0x5229, false)), // East Asian ideograph
    (0x21335F, (0x5238, false)), // East Asian ideograph
    (0x213360, (0x523B, false)), // East Asian ideograph
    (0x213361, (0x5237, false)), // East Asian ideograph
    (0x233362, (0x8ADE, false)), // East Asian ideograph
    (0x233363, (0x8AE1, false)), // East Asian ideograph
    (0x233364, (0x8B07, false)), // East Asian ideograph
    (0x2D537E, (0x81C8, false)), // East Asian ideograph
    (0x213366, (0x5241, false)), // East Asian ideograph
    (0x213367, (0x5239, false)), // East Asian ideograph
    (0x223368, (0x645B, false)), // East Asian ideograph
    (0x213369, (0x524D, false)), // East Asian ideograph
    (0x22336A, (0x644F, false)), // East Asian ideograph
    (0x27336B, (0x514B, false)), // East Asian ideograph
    (0x21336C, (0x524A, false)), // East Asian ideograph
    (0x27336D, (0x5219, false)), // East Asian ideograph
    (0x21336E, (0x525C, false)), // East Asian ideograph
    (0x22336F, (0x6476, false)), // East Asian ideograph
    (0x273370, (0x521A, false)), // East Asian ideograph
    (0x215F37, (0x969B, false)), // East Asian ideograph
    (0x213372, (0x525D, false)), // East Asian ideograph
    (0x233373, (0x8B16, false)), // East Asian ideograph
    (0x213374, (0x526F, false)), // East Asian ideograph
    (0x213375, (0x5272, false)), // East Asian ideograph
    (0x223376, (0x6474, false)), // East Asian ideograph
    (0x213377, (0x5269, false)), // East Asian ideograph
    (0x273378, (0x521B, false)), // East Asian ideograph
    (0x233379, (0x8B06, false)), // East Asian ideograph
    (0x23337A, (0x8B05, false)), // East Asian ideograph
    (0x21337B, (0x527F, false)), // East Asian ideograph
    (0x27337C, (0x5212, false)), // East Asian ideograph
    (0x21337D, (0x5288, false)), // East Asian ideograph
    (0x27337E, (0x5267, false)), // East Asian ideograph
    (0x213340, (0x51CD, false)), // East Asian ideograph
    (0x217341, (0x569C, false)), // East Asian ideograph
    (0x27484F, (0x6CAA, false)), // East Asian ideograph
    (0x276226, (0x9CA2, false)), // East Asian ideograph
    (0x234960, (0x95D5, false)), // East Asian ideograph
    (0x6F773B, (0xC6FD, false)), // Korean hangul
    (0x213344, (0x51DC, false)), // East Asian ideograph
    (0x23337C, (0x8B0F, false)), // East Asian ideograph
    (0x223345, (0x6441, false)), // East Asian ideograph
    (0x6F5B7E, (0xD33C, false)), // Korean hangul
    (0x282E79, (0x6079, false)), // East Asian ideograph
    (0x213E40, (0x6046, false)), // East Asian ideograph (variant of 4B3E40 which maps to 6046)
    (0x286E68, (0x7B3E, false)), // East Asian ideograph
    (0x22677B, (0x79B4, false)), // East Asian ideograph
    (0x224562, (0x6BDC, false)), // East Asian ideograph
    (0x213348, (0x51F1, false)), // East Asian ideograph
    (0x695626, (0x4E62, false)), // East Asian ideograph
    (0x6F4A72, (0xAF49, false)), // Korean hangul
    (0x213349, (0x51F3, false)), // East Asian ideograph
    (0x513057, (0x4E98, false)), // East Asian ideograph
    (0x21334B, (0x51FA, false)), // East Asian ideograph
    (0x6F573C, (0xC814, false)), // Korean hangul
    (0x23334C, (0x8ADD, false)), // East Asian ideograph
    (0x224563, (0x6BDD, false)), // East Asian ideograph
    (0x234962, (0x95D2, false)), // East Asian ideograph
    (0x6F2525, (0x3160, false)), // Korean hangul
    (0x4C3A33, (0x80AD, false)), // East Asian ideograph (variant of 2E3A33 which maps to 80AD)
    (0x276072, (0x9975, false)), // East Asian ideograph
    (0x27595E, (0x4E88, false)), // East Asian ideograph
    (0x21734E, (0x56AC, false)), // East Asian ideograph
    (0x273745, (0x55B7, false)), // East Asian ideograph
    (0x23334F, (0x8AF4, false)), // East Asian ideograph
    (0x233350, (0x8AF5, false)), // East Asian ideograph
    (0x6F573D, (0xC815, false)), // Korean hangul
    (0x213351, (0x5203, false)), // East Asian ideograph
    (0x395050, (0x7BED, false)), // East Asian ideograph
    (0x22633A, (0x77D1, false)), // East Asian ideograph
    (0x287352, (0x7F34, false)), // East Asian ideograph
    (0x6F4B71, (0xB10B, false)), // Korean hangul
    (0x6F4A74, (0xAF58, false)), // Korean hangul
    (0x233353, (0x8ADF, false)), // East Asian ideograph
    (0x4B3354, (0x82C5, false)), // East Asian ideograph
    (0x4B3355, (0x520B, false)), // East Asian ideograph
    (0x6F573E, (0xC816, false)), // Korean hangul
    (0x213356, (0x5211, false)), // East Asian ideograph
    (0x224565, (0x6BDF, false)), // East Asian ideograph
    (0x213357, (0x5217, false)), // East Asian ideograph
    (0x2D5C48, (0x9013, false)), // East Asian ideograph
    (0x273747, (0x54DD, false)), // East Asian ideograph
    (0x213359, (0x520E, false)), // East Asian ideograph
    (0x6F5D58, (0xD6E8, false)), // Korean hangul
    (0x27735A, (0x55BE, false)), // East Asian ideograph
    (0x2D4F41, (0x4E69, false)), // East Asian ideograph
    (0x4B5D2B, (0x9162, false)), // East Asian ideograph
    (0x273421, (0x5251, false)), // East Asian ideograph
    (0x213422, (0x5289, false)), // East Asian ideograph
    (0x273423, (0x5242, false)), // East Asian ideograph
    (0x223424, (0x6464, false)), // East Asian ideograph
    (0x213425, (0x529F, false)), // East Asian ideograph
    (0x213426, (0x52A0, false)), // East Asian ideograph
    (0x223427, (0x6482, false)), // East Asian ideograph
    (0x223428, (0x645E, false)), // East Asian ideograph
    (0x21335C, (0x5220, false)), // East Asian ideograph
    (0x21342A, (0x52AC, false)), // East Asian ideograph
    (0x21342B, (0x52AA, false)), // East Asian ideograph
    (0x22342C, (0x647B, false)), // East Asian ideograph
    (0x23342D, (0x8B26, false)), // East Asian ideograph
    (0x22342E, (0x645C, false)), // East Asian ideograph
    (0x27342F, (0x52B2, false)), // East Asian ideograph
    (0x233430, (0x8B33, false)), // East Asian ideograph
    (0x213431, (0x52D8, false)), // East Asian ideograph
    (0x21305B, (0x4EA1, false)), // East Asian ideograph
    (0x273433, (0x52A1, false)), // East Asian ideograph
    (0x273434, (0x52A8, false)), // East Asian ideograph
    (0x273435, (0x52B3, false)), // East Asian ideograph
    (0x273436, (0x52CB, false)), // East Asian ideograph
    (0x213437, (0x52DD, false)), // East Asian ideograph
    (0x273438, (0x52BF, false)), // East Asian ideograph
    (0x213439, (0x52E4, false)), // East Asian ideograph
    (0x23343A, (0x8B29, false)), // East Asian ideograph
    (0x2D335F, (0x52B5, false)), // East Asian ideograph
    (0x27343C, (0x52B1, false)), // East Asian ideograph
    (0x27343D, (0x529D, false)), // East Asian ideograph
    (0x21343E, (0x52FB, false)), // East Asian ideograph
    (0x22343F, (0x6499, false)), // East Asian ideograph
    (0x213440, (0x52FF, false)), // East Asian ideograph
    (0x217360, (0x56C5, false)), // East Asian ideograph
    (0x233442, (0x8B48, false)), // East Asian ideograph
    (0x215F3E, (0x96BB, false)), // East Asian ideograph
    (0x213444, (0x530D, false)), // East Asian ideograph
    (0x234966, (0x95DA, false)), // East Asian ideograph
    (0x213446, (0x530F, false)), // East Asian ideograph
    (0x213447, (0x5315, false)), // East Asian ideograph
    (0x213448, (0x5316, false)), // East Asian ideograph
    (0x213449, (0x5317, false)), // East Asian ideograph
    (0x23344A, (0x8B46, false)), // East Asian ideograph
    (0x21344B, (0x53F5, false)), // East Asian ideograph
    (0x21344C, (0x531D, false)), // East Asian ideograph
    (0x22344D, (0x6496, false)), // East Asian ideograph
    (0x21344E, (0x5320, false)), // East Asian ideograph
    (0x21344F, (0x5323, false)), // East Asian ideograph
    (0x213450, (0x532A, false)), // East Asian ideograph
    (0x273451, (0x6C47, false)), // East Asian ideograph
    (0x273452, (0x532E, false)), // East Asian ideograph
    (0x213363, (0x523A, false)), // East Asian ideograph
    (0x213454, (0x533E, false)), // East Asian ideograph
    (0x273455, (0x533A, false)), // East Asian ideograph
    (0x213456, (0x533F, false)), // East Asian ideograph
    (0x213457, (0x5341, false)), // East Asian ideograph
    (0x213458, (0x5343, false)), // East Asian ideograph
    (0x213459, (0x5345, false)), // East Asian ideograph
    (0x21345A, (0x5348, false)), // East Asian ideograph
    (0x22345B, (0x64B6, false)), // East Asian ideograph
    (0x21345C, (0x534A, false)), // East Asian ideograph
    (0x21345D, (0x5349, false)), // East Asian ideograph (variant of 2D345D which maps to 5349)
    (0x6F5741, (0xC820, false)), // Korean hangul
    (0x27345F, (0x5346, false)), // East Asian ideograph
    (0x273460, (0x534F, false)), // East Asian ideograph
    (0x213461, (0x5353, false)), // East Asian ideograph
    (0x223462, (0x649F, false)), // East Asian ideograph
    (0x213463, (0x5357, false)), // East Asian ideograph
    (0x213464, (0x535A, false)), // East Asian ideograph
    (0x223465, (0x64A7, false)), // East Asian ideograph (not in Unicode)
    (0x213466, (0x535E, false)), // East Asian ideograph
    (0x213467, (0x5361, false)), // East Asian ideograph
    (0x233468, (0x8B6B, false)), // East Asian ideograph
    (0x213469, (0x5366, false)), // East Asian ideograph
    (0x22346A, (0x64D7, false)), // East Asian ideograph
    (0x21346B, (0x536E, false)), // East Asian ideograph
    (0x21346C, (0x5370, false)), // East Asian ideograph
    (0x21346D, (0x5371, false)), // East Asian ideograph
    (0x21346E, (0x537D, false)), // East Asian ideograph
    (0x21346F, (0x5375, false)), // East Asian ideograph
    (0x233470, (0x8B78, false)), // East Asian ideograph
    (0x213368, (0x5243, false)), // East Asian ideograph
    (0x213473, (0x537B, false)), // East Asian ideograph
    (0x223474, (0x64BE, false)), // East Asian ideograph
    (0x223475, (0x64D0, false)), // East Asian ideograph
    (0x213476, (0x539A, false)), // East Asian ideograph
    (0x213477, (0x539D, false)), // East Asian ideograph
    (0x213478, (0x539F, false)), // East Asian ideograph
    (0x233479, (0x8B81, false)), // East Asian ideograph
    (0x27347A, (0x538C, false)), // East Asian ideograph
    (0x27347B, (0x5389, false)), // East Asian ideograph
    (0x21347C, (0x53BB, false)), // East Asian ideograph
    (0x27347D, (0x53C2, false)), // East Asian ideograph
    (0x21347E, (0x53C8, false)), // East Asian ideograph
    (0x334968, (0x7133, false)), // East Asian ideograph
    (0x23336B, (0x8B0C, false)), // East Asian ideograph
    (0x6F4B72, (0xB10C, false)), // Korean hangul
    (0x6F4A79, (0xAF79, false)), // Korean hangul
    (0x22336C, (0x646B, false)), // East Asian ideograph
    (0x225676, (0x72EB, false)), // East Asian ideograph
    (0x29583E, (0x9CCA, false)), // East Asian ideograph
    (0x21736D, (0x56DD, false)), // East Asian ideograph
    (0x2D4F45, (0x9834, false)), // East Asian ideograph
    (0x274858, (0x6EE1, false)), // East Asian ideograph
    (0x6F5743, (0xC82C, false)), // Korean hangul
    (0x23336F, (0x8B1C, false)), // East Asian ideograph
    (0x234969, (0x95DE, false)), // East Asian ideograph
    (0x694677, (0x5302, false)), // East Asian ideograph
    (0x217370, (0x56DF, false)), // East Asian ideograph
    (0x6F4A7A, (0xAF80, false)), // Korean hangul
    (0x213371, (0x5254, false)), // East Asian ideograph
    (0x333377, (0x5270, false)), // East Asian ideograph
    (0x2D3372, (0x5265, false)), // East Asian ideograph
    (0x6F5D59, (0xD6F0, false)), // Korean hangul
    (0x6F502A, (0xBA53, false)), // Korean hangul
    (0x696F27, (0x933B, false)), // East Asian ideograph
    (0x295C65, (0x9E69, false)), // East Asian ideograph
    (0x213373, (0x526A, false)), // East Asian ideograph
    (0x395E2F, (0x5277, false)), // East Asian ideograph
    (0x287374, (0x7F35, false)), // East Asian ideograph
    (0x225F3C, (0x760A, false)), // East Asian ideograph
    (0x23496A, (0x95E0, false)), // East Asian ideograph
    (0x217375, (0x56EB, false)), // East Asian ideograph
    (0x334527, (0x6918, false)), // East Asian ideograph
    (0x273376, (0x5240, false)), // East Asian ideograph
    (0x6F516A, (0xBE1C, false)), // Korean hangul
    (0x275E21, (0x949F, false)), // East Asian ideograph
    (0x2D3377, (0x8CF8, false)), // East Asian ideograph
    (0x215E22, (0x9318, false)), // East Asian ideograph
    (0x27615F, (0x53D1, false)), // East Asian ideograph (duplicate simplified)
    (0x233378, (0x8B0B, false)), // East Asian ideograph
    (0x215E23, (0x936C, false)), // East Asian ideograph
    (0x27485A, (0x6E10, false)), // East Asian ideograph
    (0x275E24, (0x953B, false)), // East Asian ideograph
    (0x21337A, (0x527D, false)), // East Asian ideograph
    (0x225E25, (0x7583, false)), // East Asian ideograph
    (0x6F583C, (0xC9E4, false)), // Korean hangul
    (0x22337B, (0x6473, false)), // East Asian ideograph
    (0x27374E, (0x549B, false)), // East Asian ideograph
    (0x2D5E26, (0x7194, false)), // East Asian ideograph
    (0x293F4C, (0x90D3, false)), // East Asian ideograph
    (0x21337C, (0x5283, false)), // East Asian ideograph
    (0x215E27, (0x93AE, false)), // East Asian ideograph
    (0x335635, (0x85AC, false)), // East Asian ideograph
    (0x3F3F24, (0x614E, false)), // East Asian ideograph
    (0x23337D, (0x8B10, false)), // East Asian ideograph
    (0x215E28, (0x9396, false)), // East Asian ideograph
    (0x6F5746, (0xC838, false)), // Korean hangul
    (0x21337E, (0x5287, false)), // East Asian ideograph
    (0x275E29, (0x94A8, false)), // East Asian ideograph
    (0x6F4C4D, (0xB233, false)), // Korean hangul
    (0x215E2A, (0x93B3, false)), // East Asian ideograph
    (0x215E2B, (0x93E1, false)), // East Asian ideograph
    (0x213062, (0x4EAD, false)), // East Asian ideograph
    (0x692564, (0x30E4, false)), // Katakana letter YA
    (0x223461, (0x6498, false)), // East Asian ideograph
    (0x29516D, (0x9991, false)), // East Asian ideograph
    (0x215E2C, (0x93D1, false)), // East Asian ideograph
    (0x225E2D, (0x7592, false)), // East Asian ideograph
    (0x4C4D63, (0x6F99, false)), // East Asian ideograph
    (0x6F5747, (0xC83C, false)), // Korean hangul
    (0x215E2E, (0x93C3, false)), // East Asian ideograph
    (0x213D2C, (0x5EE0, false)), // East Asian ideograph
    (0x275E2F, (0x94F2, false)), // East Asian ideograph
    (0x2D6056, (0x980B, false)), // East Asian ideograph
    (0x6F4A7E, (0xAF95, false)), // Korean hangul
    (0x275969, (0x8D1E, false)), // East Asian ideograph
    (0x215E30, (0x93D7, false)), // East Asian ideograph
    (0x213522, (0x53CB, false)), // East Asian ideograph
    (0x233523, (0x8B8B, false)), // East Asian ideograph
    (0x213524, (0x53CD, false)), // East Asian ideograph
    (0x213525, (0x53D6, false)), // East Asian ideograph
    (0x233526, (0x8B87, false)), // East Asian ideograph
    (0x225E31, (0x7595, false)), // East Asian ideograph
    (0x213528, (0x53DB, false)), // East Asian ideograph
    (0x213529, (0x53DF, false)), // East Asian ideograph
    (0x22352A, (0x64EF, false)), // East Asian ideograph
    (0x27352B, (0x4E1B, false)), // East Asian ideograph
    (0x21352C, (0x53E3, false)), // East Asian ideograph
    (0x22352D, (0x64E1, false)), // East Asian ideograph
    (0x22352E, (0x64E5, false)), // East Asian ideograph
    (0x224873, (0x6D63, false)), // East Asian ideograph
    (0x213530, (0x53EF, false)), // East Asian ideograph
    (0x213531, (0x53E9, false)), // East Asian ideograph
    (0x213532, (0x53F3, false)), // East Asian ideograph
    (0x223533, (0x64E2, false)), // East Asian ideograph
    (0x213534, (0x53E8, false)), // East Asian ideograph
    (0x213535, (0x53E6, false)), // East Asian ideograph
    (0x223536, (0x64ED, false)), // East Asian ideograph
    (0x233537, (0x8B9C, false)), // East Asian ideograph
    (0x223538, (0x64E4, false)), // East Asian ideograph
    (0x275E34, (0x9542, false)), // East Asian ideograph
    (0x21353A, (0x53F1, false)), // East Asian ideograph
    (0x21353B, (0x53ED, false)), // East Asian ideograph
    (0x21353C, (0x53EA, false)), // East Asian ideograph
    (0x23353D, (0x8C3A, false)), // East Asian ideograph
    (0x235E35, (0x9EC6, false)), // East Asian ideograph
    (0x213540, (0x5409, false)), // East Asian ideograph
    (0x213541, (0x5410, false)), // East Asian ideograph
    (0x213542, (0x540F, false)), // East Asian ideograph
    (0x235A7B, (0x9D59, false)), // East Asian ideograph
    (0x233544, (0x8C40, false)), // East Asian ideograph
    (0x233545, (0x8C42, false)), // East Asian ideograph
    (0x213546, (0x5404, false)), // East Asian ideograph
    (0x213547, (0x5403, false)), // East Asian ideograph
    (0x213548, (0x5412, false)), // East Asian ideograph
    (0x2D7164, (0x55D4, false)), // East Asian ideograph (variant of 217164 which maps to 55D4)
    (0x21354A, (0x5406, false)), // East Asian ideograph (not in Unicode)
    (0x23354B, (0x8C47, false)), // East Asian ideograph
    (0x21354D, (0x542D, false)), // East Asian ideograph
    (0x21354E, (0x541D, false)), // East Asian ideograph
    (0x21354F, (0x541E, false)), // East Asian ideograph
    (0x213550, (0x541B, false)), // East Asian ideograph
    (0x213551, (0x544E, false)), // East Asian ideograph
    (0x233552, (0x8C55, false)), // East Asian ideograph
    (0x23496F, (0x95E5, false)), // East Asian ideograph
    (0x233554, (0x8C57, false)), // East Asian ideograph
    (0x213555, (0x5431, false)), // East Asian ideograph
    (0x233556, (0x8C5D, false)), // East Asian ideograph
    (0x213557, (0x543C, false)), // East Asian ideograph
    (0x213558, (0x5443, false)), // East Asian ideograph
    (0x213559, (0x5426, false)), // East Asian ideograph
    (0x21355A, (0x5420, false)), // East Asian ideograph
    (0x22355B, (0x6516, false)), // East Asian ideograph
    (0x23355C, (0x86C3, false)), // East Asian ideograph
    (0x21355D, (0x5435, false)), // East Asian ideograph
    (0x234E26, (0x97B5, false)), // East Asian ideograph
    (0x21355F, (0x544A, false)), // East Asian ideograph
    (0x213560, (0x5448, false)), // East Asian ideograph
    (0x223561, (0x651B, false)), // East Asian ideograph
    (0x213562, (0x5438, false)), // East Asian ideograph
    (0x233563, (0x8C68, false)), // East Asian ideograph
    (0x213564, (0x5442, false)), // East Asian ideograph
    (0x233565, (0x8C6D, false)), // East Asian ideograph
    (0x213566, (0x541F, false)), // East Asian ideograph
    (0x213567, (0x5429, false)), // East Asian ideograph
    (0x213568, (0x5473, false)), // East Asian ideograph
    (0x223569, (0x6527, false)), // East Asian ideograph
    (0x21356A, (0x5475, false)), // East Asian ideograph
    (0x21356B, (0x5495, false)), // East Asian ideograph
    (0x21356C, (0x5478, false)), // East Asian ideograph
    (0x22356D, (0x6522, false)), // East Asian ideograph
    (0x21356E, (0x5477, false)), // East Asian ideograph
    (0x22356F, (0x6529, false)), // East Asian ideograph
    (0x213571, (0x5492, false)), // East Asian ideograph
    (0x223572, (0x6525, false)), // East Asian ideograph
    (0x213573, (0x547C, false)), // East Asian ideograph
    (0x233574, (0x8C76, false)), // East Asian ideograph
    (0x225E3E, (0x75BA, false)), // East Asian ideograph
    (0x213576, (0x548B, false)), // East Asian ideograph
    (0x213577, (0x548C, false)), // East Asian ideograph
    (0x213578, (0x5490, false)), // East Asian ideograph
    (0x213579, (0x547D, false)), // East Asian ideograph
    (0x21357A, (0x5476, false)), // East Asian ideograph
    (0x23357B, (0x8C78, false)), // East Asian ideograph
    (0x22357C, (0x6541, false)), // East Asian ideograph
    (0x23357D, (0x8C7B, false)), // East Asian ideograph
    (0x21357E, (0x54A9, false)), // East Asian ideograph
    (0x275E40, (0x956F, false)), // East Asian ideograph
    (0x23563A, (0x9B48, false)), // East Asian ideograph
    (0x333421, (0x91FC, false)), // East Asian ideograph
    (0x6F574B, (0xC874, false)), // Korean hangul
    (0x234566, (0x9355, false)), // East Asian ideograph
    (0x235E42, (0x9ECC, false)), // East Asian ideograph
    (0x213D30, (0x5EF7, false)), // East Asian ideograph
    (0x6F4C4E, (0xB234, false)), // Korean hangul
    (0x225E43, (0x75B0, false)), // East Asian ideograph
    (0x225E44, (0x75C3, false)), // East Asian ideograph
    (0x27552A, (0x82CB, false)), // East Asian ideograph
    (0x6F5763, (0xC8CC, false)), // Korean hangul
    (0x275E45, (0x94C4, false)), // East Asian ideograph
    (0x225E46, (0x75BF, false)), // East Asian ideograph
    (0x2D5927, (0x8ACC, false)), // East Asian ideograph
    (0x234C38, (0x971B, false)), // East Asian ideograph
    (0x6F574C, (0xC878, false)), // Korean hangul
    (0x225E47, (0x75B4, false)), // East Asian ideograph
    (0x6F5D29, (0xD5F5, false)), // Korean hangul
    (0x6F4B74, (0xB110, false)), // Korean hangul
    (0x23452F, (0x9342, false)), // East Asian ideograph
    (0x27596E, (0x8D2F, false)), // East Asian ideograph
    (0x273755, (0x5411, false)), // East Asian ideograph
    (0x275E49, (0x9523, false)), // East Asian ideograph
    (0x215E4A, (0x947D, false)), // East Asian ideograph
    (0x23563C, (0x9B4E, false)), // East Asian ideograph
    (0x333423, (0x5264, false)), // East Asian ideograph
    (0x275E4B, (0x51FF, false)), // East Asian ideograph
    (0x6F574D, (0xC87A, false)), // Korean hangul
    (0x235E4C, (0x9ED3, false)), // East Asian ideograph
    (0x275E4D, (0x95E8, false)), // East Asian ideograph
    (0x34492F, (0x6D34, false)), // East Asian ideograph
    (0x225E4E, (0x75C1, false)), // East Asian ideograph
    (0x277745, (0x57D9, false)), // East Asian ideograph
    (0x275E4F, (0x95EA, false)), // East Asian ideograph
    (0x4C6F43, (0x7CCD, false)), // East Asian ideograph
    (0x225E50, (0x75B1, false)), // East Asian ideograph
    (0x274863, (0x6D46, false)), // East Asian ideograph
    (0x6F574E, (0xC880, false)), // Korean hangul
    (0x284C62, (0x988D, false)), // East Asian ideograph
    (0x225E51, (0x75C4, false)), // East Asian ideograph
    (0x213D33, (0x5EFA, false)), // East Asian ideograph
    (0x275E52, (0x95F0, false)), // East Asian ideograph
    (0x275970, (0x8D2A, false)), // East Asian ideograph
    (0x215E53, (0x958B, false)), // East Asian ideograph
    (0x275E54, (0x95F2, false)), // East Asian ideograph
    (0x23563E, (0x9B4D, false)), // East Asian ideograph
    (0x215E55, (0x9593, false)), // East Asian ideograph
    (0x274864, (0x6E17, false)), // East Asian ideograph
    (0x6F574F, (0xC881, false)), // Korean hangul
    (0x6F4D29, (0xB355, false)), // Korean hangul
    (0x235E57, (0x9EE3, false)), // East Asian ideograph
    (0x275971, (0x8D2B, false)), // East Asian ideograph
    (0x225E58, (0x75CD, false)), // East Asian ideograph
    (0x215E59, (0x95A8, false)), // East Asian ideograph
    (0x275E5A, (0x95FD, false)), // East Asian ideograph
    (0x6F7727, (0xADD5, false)), // Korean hangul
    (0x213621, (0x54AA, false)), // East Asian ideograph
    (0x213622, (0x54A8, false)), // East Asian ideograph
    (0x213623, (0x54AC, false)), // East Asian ideograph
    (0x213624, (0x54C0, false)), // East Asian ideograph
    (0x213625, (0x54B3, false)), // East Asian ideograph
    (0x213626, (0x54A6, false)), // East Asian ideograph
    (0x213627, (0x54AB, false)), // East Asian ideograph
    (0x213628, (0x54C7, false)), // East Asian ideograph
    (0x213629, (0x54C9, false)), // East Asian ideograph
    (0x21362A, (0x54C4, false)), // East Asian ideograph
    (0x21362B, (0x54C2, false)), // East Asian ideograph
    (0x22362C, (0x6538, false)), // East Asian ideograph
    (0x21362D, (0x54C1, false)), // East Asian ideograph
    (0x23362E, (0x8C88, false)), // East Asian ideograph
    (0x21362F, (0x54CE, false)), // East Asian ideograph
    (0x213630, (0x54B1, false)), // East Asian ideograph
    (0x213631, (0x54BB, false)), // East Asian ideograph
    (0x213632, (0x54AF, false)), // East Asian ideograph
    (0x213633, (0x54C8, false)), // East Asian ideograph
    (0x223634, (0x6542, false)), // East Asian ideograph
    (0x225E5E, (0x75CC, false)), // East Asian ideograph
    (0x213636, (0x5510, false)), // East Asian ideograph
    (0x213637, (0x54EA, false)), // East Asian ideograph
    (0x213638, (0x5514, false)), // East Asian ideograph
    (0x233639, (0x8C94, false)), // East Asian ideograph
    (0x21363A, (0x54E5, false)), // East Asian ideograph
    (0x225E5F, (0x75D0, false)), // East Asian ideograph
    (0x21363C, (0x54F2, false)), // East Asian ideograph
    (0x21363D, (0x54E8, false)), // East Asian ideograph
    (0x21363E, (0x54E1, false)), // East Asian ideograph
    (0x22363F, (0x6555, false)), // East Asian ideograph
    (0x213640, (0x54ED, false)), // East Asian ideograph
    (0x233641, (0x8C9B, false)), // East Asian ideograph
    (0x213642, (0x5509, false)), // East Asian ideograph
    (0x213643, (0x54E6, false)), // East Asian ideograph
    (0x233644, (0x8CA4, false)), // East Asian ideograph
    (0x223645, (0x6567, false)), // East Asian ideograph
    (0x213646, (0x5546, false)), // East Asian ideograph
    (0x223647, (0x6561, false)), // East Asian ideograph
    (0x213648, (0x554F, false)), // East Asian ideograph
    (0x273649, (0x54D1, false)), // East Asian ideograph
    (0x21364A, (0x5566, false)), // East Asian ideograph
    (0x21364B, (0x556A, false)), // East Asian ideograph
    (0x21364C, (0x554A, false)), // East Asian ideograph
    (0x21364D, (0x5544, false)), // East Asian ideograph
    (0x21364E, (0x555C, false)), // East Asian ideograph
    (0x22364F, (0x656D, false)), // East Asian ideograph
    (0x213650, (0x5543, false)), // East Asian ideograph
    (0x213651, (0x552C, false)), // East Asian ideograph
    (0x213652, (0x5561, false)), // East Asian ideograph
    (0x233653, (0x8CB9, false)), // East Asian ideograph
    (0x223654, (0x657A, false)), // East Asian ideograph
    (0x213655, (0x5555, false)), // East Asian ideograph
    (0x213656, (0x552F, false)), // East Asian ideograph
    (0x233657, (0x8CCD, false)), // East Asian ideograph
    (0x213658, (0x5564, false)), // East Asian ideograph
    (0x213659, (0x5538, false)), // East Asian ideograph
    (0x21365A, (0x55A7, false)), // East Asian ideograph
    (0x21365B, (0x5580, false)), // East Asian ideograph
    (0x21365C, (0x557B, false)), // East Asian ideograph
    (0x21365D, (0x557C, false)), // East Asian ideograph
    (0x21365E, (0x5527, false)), // East Asian ideograph
    (0x21365F, (0x5594, false)), // East Asian ideograph
    (0x213660, (0x5587, false)), // East Asian ideograph
    (0x213661, (0x559C, false)), // East Asian ideograph
    (0x213662, (0x558B, false)), // East Asian ideograph
    (0x273663, (0x4E27, false)), // East Asian ideograph
    (0x213664, (0x55B3, false)), // East Asian ideograph
    (0x225E66, (0x75E1, false)), // East Asian ideograph
    (0x213666, (0x5583, false)), // East Asian ideograph
    (0x213667, (0x55B1, false)), // East Asian ideograph
    (0x273668, (0x5355, false)), // East Asian ideograph
    (0x213669, (0x5582, false)), // East Asian ideograph
    (0x21366A, (0x559F, false)), // East Asian ideograph
    (0x225E67, (0x75E6, false)), // East Asian ideograph
    (0x21366C, (0x5598, false)), // East Asian ideograph
    (0x21366D, (0x559A, false)), // East Asian ideograph
    (0x22366E, (0x658C, false)), // East Asian ideograph
    (0x27366F, (0x4E54, false)), // East Asian ideograph
    (0x223670, (0x6592, false)), // East Asian ideograph
    (0x213671, (0x55B2, false)), // East Asian ideograph
    (0x233672, (0x8CDD, false)), // East Asian ideograph
    (0x213673, (0x55E8, false)), // East Asian ideograph
    (0x233674, (0x8CD9, false)), // East Asian ideograph
    (0x223675, (0x659B, false)), // East Asian ideograph
    (0x213676, (0x55DC, false)), // East Asian ideograph
    (0x223677, (0x659D, false)), // East Asian ideograph
    (0x213678, (0x55C7, false)), // East Asian ideograph
    (0x213679, (0x55D3, false)), // East Asian ideograph
    (0x21367A, (0x55CE, false)), // East Asian ideograph
    (0x21367B, (0x55E3, false)), // East Asian ideograph
    (0x23367C, (0x8CF5, false)), // East Asian ideograph
    (0x21367D, (0x55E4, false)), // East Asian ideograph
    (0x23367E, (0x8CFB, false)), // East Asian ideograph
    (0x232760, (0x8600, false)), // East Asian ideograph
    (0x275E6B, (0x8F9F, false)), // East Asian ideograph (duplicate simplified)
    (0x285424, (0x70E8, false)), // East Asian ideograph
    (0x27375C, (0x556D, false)), // East Asian ideograph
    (0x4B5E6C, (0x961D, false)), // East Asian ideograph (duplicate simplified)
    (0x293F5A, (0x90E7, false)), // East Asian ideograph
    (0x235E6F, (0x9EF6, false)), // East Asian ideograph
    (0x4B5422, (0x81D3, false)), // East Asian ideograph
    (0x6F583F, (0xC9EC, false)), // Korean hangul
    (0x27375D, (0x55EB, false)), // East Asian ideograph
    (0x225E71, (0x75E4, false)), // East Asian ideograph
    (0x225E72, (0x75E0, false)), // East Asian ideograph
    (0x4B4759, (0x6D99, false)), // East Asian ideograph
    (0x6F4B66, (0xB0C7, false)), // Korean hangul
    (0x225E73, (0x75D7, false)), // East Asian ideograph
    (0x6F5755, (0xC88D, false)), // Korean hangul
    (0x235E74, (0x9EF9, false)), // East Asian ideograph
    (0x275977, (0x8D3A, false)), // East Asian ideograph
    (0x27375E, (0x56A3, false)), // East Asian ideograph
    (0x235E76, (0x9EFB, false)), // East Asian ideograph
    (0x274921, (0x6CFD, false)), // East Asian ideograph
    (0x293F5C, (0x90AC, false)), // East Asian ideograph
    (0x2D616A, (0x6B1D, false)), // East Asian ideograph
    (0x215E77, (0x964C, false)), // East Asian ideograph
    (0x274922, (0x6D4A, false)), // East Asian ideograph
    (0x6F5756, (0xC890, false)), // Korean hangul
    (0x6F4924, (0xAC74, false)), // Korean hangul
    (0x6F4B76, (0xB118, false)), // Korean hangul
    (0x215E7A, (0x9662, false)), // East Asian ideograph
    (0x224925, (0x6D6D, false)), // East Asian ideograph
    (0x275978, (0x8D35, false)), // East Asian ideograph
    (0x235E7B, (0x9EFE, false)), // East Asian ideograph (not in Unicode)
    (0x274926, (0x6D4E, false)), // East Asian ideograph
    (0x215E7C, (0x965B, false)), // East Asian ideograph
    (0x274927, (0x6CDE, false)), // East Asian ideograph
    (0x235E7D, (0x9F02, false)), // East Asian ideograph
    (0x274928, (0x6EE8, false)), // East Asian ideograph
    (0x6F5757, (0xC894, false)), // Korean hangul
    (0x215E7E, (0x965D, false)), // East Asian ideograph
    (0x224929, (0x6D91, false)), // East Asian ideograph
    (0x287065, (0x7EC2, false)), // East Asian ideograph
    (0x2F4231, (0x8019, false)), // Unrelated variant of EACC 215266 which maps to 8019
    (0x6F492A, (0xAC81, false)), // Korean hangul
    (0x33492E, (0x6F81, false)), // East Asian ideograph
    (0x27492B, (0x6EE5, false)), // East Asian ideograph
    (0x33337B, (0x52E6, false)), // East Asian ideograph
    (0x233871, (0x8DC2, false)), // East Asian ideograph
    (0x22492C, (0x6D81, false)), // East Asian ideograph
    (0x235647, (0x9B51, false)), // East Asian ideograph
    (0x33463C, (0x6BBB, false)), // East Asian ideograph
    (0x27492D, (0x6D9B, false)), // East Asian ideograph
    (0x6F553A, (0xC587, false)), // Korean hangul
    (0x27492E, (0x6DA9, false)), // East Asian ideograph
    (0x22413C, (0x6A5A, false)), // East Asian ideograph
    (0x2E492F, (0x6CD9, false)), // East Asian ideograph
    (0x27597A, (0x4E70, false)), // East Asian ideograph
    (0x213331, (0x518D, false)), // East Asian ideograph
    (0x273761, (0x7F57, false)), // East Asian ideograph (duplicate simplified)
    (0x213721, (0x55DA, false)), // East Asian ideograph
    (0x223722, (0x65A8, false)), // East Asian ideograph
    (0x223723, (0x65A6, false)), // East Asian ideograph
    (0x213724, (0x5600, false)), // East Asian ideograph
    (0x233725, (0x8D04, false)), // East Asian ideograph
    (0x213726, (0x55FE, false)), // East Asian ideograph
    (0x273727, (0x5567, false)), // East Asian ideograph
    (0x213728, (0x55F7, false)), // East Asian ideograph
    (0x213729, (0x5608, false)), // East Asian ideograph
    (0x22372A, (0x65B6, false)), // East Asian ideograph
    (0x21372B, (0x55FD, false)), // East Asian ideograph
    (0x22372C, (0x65B8, false)), // East Asian ideograph
    (0x23372D, (0x8D09, false)), // East Asian ideograph
    (0x21372E, (0x5614, false)), // East Asian ideograph
    (0x22372F, (0x65BF, false)), // East Asian ideograph
    (0x273730, (0x5C1D, false)), // East Asian ideograph
    (0x273731, (0x55BD, false)), // East Asian ideograph
    (0x273732, (0x5520, false)), // East Asian ideograph
    (0x213733, (0x562F, false)), // East Asian ideograph
    (0x223734, (0x65C2, false)), // East Asian ideograph
    (0x213735, (0x5636, false)), // East Asian ideograph
    (0x213736, (0x5632, false)), // East Asian ideograph
    (0x213737, (0x563B, false)), // East Asian ideograph
    (0x213738, (0x5639, false)), // East Asian ideograph
    (0x274934, (0x6E85, false)), // East Asian ideograph
    (0x23373A, (0x8D10, false)), // East Asian ideograph
    (0x22373B, (0x65D0, false)), // East Asian ideograph
    (0x22373C, (0x65D2, false)), // East Asian ideograph
    (0x21373D, (0x5634, false)), // East Asian ideograph
    (0x23373E, (0x8D18, false)), // East Asian ideograph
    (0x224935, (0x6DEF, false)), // East Asian ideograph
    (0x213740, (0x5630, false)), // East Asian ideograph
    (0x213741, (0x566B, false)), // East Asian ideograph
    (0x213742, (0x5664, false)), // East Asian ideograph
    (0x213743, (0x5669, false)), // East Asian ideograph
    (0x223744, (0x65DB, false)), // East Asian ideograph
    (0x213745, (0x5674, false)), // East Asian ideograph
    (0x273746, (0x5F53, false)), // East Asian ideograph (duplicate simplified)
    (0x213747, (0x5665, false)), // East Asian ideograph
    (0x213748, (0x566A, false)), // East Asian ideograph
    (0x213749, (0x5668, false)), // East Asian ideograph
    (0x22374A, (0x65E1, false)), // East Asian ideograph
    (0x27374B, (0x55F3, false)), // East Asian ideograph
    (0x225A23, (0x7429, false)), // East Asian ideograph
    (0x21374D, (0x566C, false)), // East Asian ideograph
    (0x21374E, (0x5680, false)), // East Asian ideograph
    (0x21374F, (0x568E, false)), // East Asian ideograph
    (0x213750, (0x5685, false)), // East Asian ideograph
    (0x214938, (0x701B, false)), // East Asian ideograph
    (0x233752, (0x8D78, false)), // East Asian ideograph
    (0x213753, (0x568F, false)), // East Asian ideograph
    (0x223754, (0x65F4, false)), // East Asian ideograph
    (0x213755, (0x56AE, false)), // East Asian ideograph (variant of 453755 which maps to 56AE)
    (0x273756, (0x5499, false)), // East Asian ideograph
    (0x224939, (0x6D7F, false)), // East Asian ideograph
    (0x213758, (0x56A5, false)), // East Asian ideograph
    (0x213759, (0x56B7, false)), // East Asian ideograph
    (0x22375A, (0x6609, false)), // East Asian ideograph
    (0x27375B, (0x5624, false)), // East Asian ideograph
    (0x21375C, (0x56C0, false)), // East Asian ideograph
    (0x21493A, (0x7028, false)), // East Asian ideograph
    (0x22375E, (0x660A, false)), // East Asian ideograph
    (0x21375F, (0x56BC, false)), // East Asian ideograph
    (0x213760, (0x56CA, false)), // East Asian ideograph
    (0x213761, (0x56C9, false)), // East Asian ideograph
    (0x273762, (0x5453, false)), // East Asian ideograph
    (0x22493B, (0x6D85, false)), // East Asian ideograph
    (0x223764, (0x6603, false)), // East Asian ideograph
    (0x213765, (0x56DB, false)), // East Asian ideograph
    (0x213766, (0x56DA, false)), // East Asian ideograph
    (0x213767, (0x56E0, false)), // East Asian ideograph
    (0x213768, (0x56DE, false)), // East Asian ideograph
    (0x21493C, (0x7015, false)), // East Asian ideograph
    (0x22376A, (0x6611, false)), // East Asian ideograph
    (0x22376B, (0x6615, false)), // East Asian ideograph
    (0x21376C, (0x56FA, false)), // East Asian ideograph
    (0x22376D, (0x6604, false)), // East Asian ideograph
    (0x22376E, (0x6631, false)), // East Asian ideograph
    (0x21376F, (0x570B, false)), // East Asian ideograph
    (0x213770, (0x570D, false)), // East Asian ideograph
    (0x233771, (0x8D94, false)), // East Asian ideograph
    (0x223772, (0x6621, false)), // East Asian ideograph
    (0x273773, (0x56E2, false)), // East Asian ideograph
    (0x273774, (0x56FE, false)), // East Asian ideograph
    (0x223775, (0x662C, false)), // East Asian ideograph
    (0x223777, (0x6635, false)), // East Asian ideograph
    (0x213778, (0x572F, false)), // East Asian ideograph
    (0x213779, (0x5730, false)), // East Asian ideograph
    (0x21377A, (0x5728, false)), // East Asian ideograph
    (0x21377B, (0x5733, false)), // East Asian ideograph
    (0x22377C, (0x661E, false)), // East Asian ideograph
    (0x22377D, (0x663A, false)), // East Asian ideograph
    (0x224940, (0x6D67, false)), // East Asian ideograph
    (0x274941, (0x6D12, false)), // East Asian ideograph
    (0x2E3144, (0x651F, false)), // East Asian ideograph
    (0x274942, (0x6EE9, false)), // East Asian ideograph
    (0x212A34, (0xE8E1, false)), // EACC component character
    (0x214943, (0x7063, false)), // East Asian ideograph
    (0x274944, (0x6EE6, false)), // East Asian ideograph
    (0x4B5A3B, (0x8D08, false)), // East Asian ideograph
    (0x4B4761, (0x6E05, false)), // East Asian ideograph
    (0x213F21, (0x6148, false)), // East Asian ideograph
    (0x224946, (0x6D60, false)), // East Asian ideograph
    (0x2D4B35, (0x73C9, false)), // East Asian ideograph
    (0x2D4947, (0x7AC8, false)), // East Asian ideograph
    (0x394C2D, (0x7546, false)), // East Asian ideograph
    (0x224948, (0x6D98, false)), // East Asian ideograph
    (0x6F516F, (0xBE48, false)), // Korean hangul
    (0x234949, (0x95BE, false)), // East Asian ideograph
    (0x217068, (0x5530, false)), // East Asian ideograph
    (0x295D3A, (0x9E73, false)), // East Asian ideograph
    (0x27494A, (0x707E, false)), // East Asian ideograph
    (0x225352, (0x71A0, false)), // East Asian ideograph
    (0x234644, (0x93BD, false)), // East Asian ideograph
    (0x22494B, (0x6D7C, false)), // East Asian ideograph
    (0x6F575E, (0xC8AC, false)), // Korean hangul
    (0x22494C, (0x6D70, false)), // East Asian ideograph
    (0x21494D, (0x7092, false)), // East Asian ideograph
    (0x23494E, (0x95BA, false)), // East Asian ideograph
    (0x295D3B, (0x9E42, false)), // East Asian ideograph
    (0x23494F, (0x95B6, false)), // East Asian ideograph
    (0x2E3E3F, (0x7BA0, false)), // East Asian ideograph
    (0x234950, (0x95BF, false)), // East Asian ideograph
    (0x225278, (0x7178, false)), // East Asian ideograph
    (0x274951, (0x4E3A, false)), // East Asian ideograph
    (0x213D44, (0x5F29, false)), // East Asian ideograph
    (0x334674, (0x76C5, false)), // East Asian ideograph
    (0x234952, (0x95BD, false)), // East Asian ideograph
    (0x6F4953, (0xACF1, false)), // Korean hangul
    (0x21392A, (0x592E, false)), // East Asian ideograph
    (0x295D3C, (0x5364, false)), // East Asian ideograph
    (0x2D4954, (0x70F1, false)), // East Asian ideograph
    (0x6F4955, (0xACF5, false)), // Korean hangul
    (0x22526B, (0x7153, false)), // East Asian ideograph
    (0x6F5760, (0xC8B8, false)), // Korean hangul
    (0x2D4956, (0x70B0, false)), // East Asian ideograph
    (0x213D45, (0x5F2D, false)), // East Asian ideograph
    (0x4B5871, (0x8AA4, false)), // East Asian ideograph
    (0x6F4B78, (0xB11B, false)), // Korean hangul
    (0x6F4957, (0xACFA, false)), // Korean hangul
    (0x6F4958, (0xACFC, false)), // Korean hangul
    (0x284339, (0x680A, false)), // East Asian ideograph
    (0x22746A, (0x7F7F, false)), // East Asian ideograph
    (0x225251, (0x7146, false)), // East Asian ideograph
    (0x234959, (0x95C9, false)), // East Asian ideograph
    (0x22495A, (0x6DB4, false)), // East Asian ideograph
    (0x6F5761, (0xC8C4, false)), // Korean hangul
    (0x213821, (0x5740, false)), // East Asian ideograph
    (0x233822, (0x8D96, false)), // East Asian ideograph
    (0x213823, (0x574D, false)), // East Asian ideograph
    (0x213824, (0x573E, false)), // East Asian ideograph
    (0x213825, (0x574E, false)), // East Asian ideograph
    (0x223827, (0x6633, false)), // East Asian ideograph
    (0x223828, (0x662B, false)), // East Asian ideograph
    (0x22495C, (0x6DAA, false)), // East Asian ideograph
    (0x21382A, (0x5777, false)), // East Asian ideograph
    (0x22382B, (0x6634, false)), // East Asian ideograph
    (0x22382C, (0x6624, false)), // East Asian ideograph
    (0x21382D, (0x5766, false)), // East Asian ideograph
    (0x21382E, (0x5782, false)), // East Asian ideograph
    (0x21495D, (0x70CF, false)), // East Asian ideograph
    (0x213830, (0x57A0, false)), // East Asian ideograph
    (0x223831, (0x6645, false)), // East Asian ideograph
    (0x213832, (0x57A3, false)), // East Asian ideograph
    (0x233833, (0x8DA6, false)), // East Asian ideograph
    (0x213834, (0x57A2, false)), // East Asian ideograph
    (0x213835, (0x57D4, false)), // East Asian ideograph
    (0x213836, (0x57C2, false)), // East Asian ideograph
    (0x213837, (0x57CE, false)), // East Asian ideograph
    (0x213838, (0x57CB, false)), // East Asian ideograph
    (0x213839, (0x57C3, false)), // East Asian ideograph
    (0x21383A, (0x57F9, false)), // East Asian ideograph
    (0x27383B, (0x6267, false)), // East Asian ideograph
    (0x21383C, (0x57FA, false)), // East Asian ideograph
    (0x22383D, (0x6665, false)), // East Asian ideograph
    (0x22383E, (0x665C, false)), // East Asian ideograph
    (0x22383F, (0x6661, false)), // East Asian ideograph
    (0x213840, (0x5802, false)), // East Asian ideograph
    (0x224960, (0x6DEC, false)), // East Asian ideograph
    (0x213842, (0x57E4, false)), // East Asian ideograph
    (0x213843, (0x57E0, false)), // East Asian ideograph
    (0x273844, (0x62A5, false)), // East Asian ideograph
    (0x273845, (0x5C27, false)), // East Asian ideograph
    (0x213846, (0x5835, false)), // East Asian ideograph
    (0x213847, (0x582A, false)), // East Asian ideograph
    (0x223848, (0x665B, false)), // East Asian ideograph
    (0x223849, (0x6659, false)), // East Asian ideograph
    (0x22384A, (0x6667, false)), // East Asian ideograph
    (0x21384B, (0x5821, false)), // East Asian ideograph
    (0x21384C, (0x585E, false)), // East Asian ideograph
    (0x22384D, (0x6657, false)), // East Asian ideograph
    (0x275541, (0x83B1, false)), // East Asian ideograph
    (0x21384F, (0x5851, false)), // East Asian ideograph
    (0x213850, (0x586B, false)), // East Asian ideograph
    (0x223851, (0x666C, false)), // East Asian ideograph
    (0x233852, (0x8DAB, false)), // East Asian ideograph
    (0x234963, (0x95D3, false)), // East Asian ideograph
    (0x213854, (0x5854, false)), // East Asian ideograph
    (0x273855, (0x575E, false)), // East Asian ideograph
    (0x213856, (0x584A, false)), // East Asian ideograph
    (0x213857, (0x5883, false)), // East Asian ideograph
    (0x213858, (0x587E, false)), // East Asian ideograph
    (0x234964, (0x95D1, false)), // East Asian ideograph
    (0x23385A, (0x8DB0, false)), // East Asian ideograph
    (0x27385B, (0x5811, false)), // East Asian ideograph
    (0x216061, (0x98BC, false)), // East Asian ideograph
    (0x21385D, (0x5893, false)), // East Asian ideograph
    (0x21385E, (0x589E, false)), // East Asian ideograph
    (0x234965, (0x95C3, false)), // East Asian ideograph
    (0x273860, (0x575F, false)), // East Asian ideograph
    (0x273861, (0x5760, false)), // East Asian ideograph
    (0x273862, (0x5815, false)), // East Asian ideograph
    (0x213863, (0x589F, false)), // East Asian ideograph
    (0x273864, (0x575B, false)), // East Asian ideograph
    (0x274966, (0x65E0, false)), // East Asian ideograph
    (0x233866, (0x8DB2, false)), // East Asian ideograph
    (0x273867, (0x57A6, false)), // East Asian ideograph
    (0x223868, (0x6677, false)), // East Asian ideograph
    (0x273869, (0x538B, false)), // East Asian ideograph
    (0x21386A, (0x58D1, false)), // East Asian ideograph
    (0x27386B, (0x5739, false)), // East Asian ideograph
    (0x21386C, (0x58D8, false)), // East Asian ideograph
    (0x27386D, (0x5784, false)), // East Asian ideograph
    (0x23386E, (0x8DBC, false)), // East Asian ideograph
    (0x27386F, (0x575C, false)), // East Asian ideograph
    (0x233870, (0x8DB9, false)), // East Asian ideograph
    (0x223871, (0x668C, false)), // East Asian ideograph
    (0x233872, (0x8DC1, false)), // East Asian ideograph
    (0x213873, (0x58EC, false)), // East Asian ideograph
    (0x213874, (0x58EF, false)), // East Asian ideograph
    (0x223875, (0x668B, false)), // East Asian ideograph
    (0x273876, (0x58F6, false)), // East Asian ideograph
    (0x273877, (0x5BFF, false)), // East Asian ideograph
    (0x213878, (0x590F, false)), // East Asian ideograph
    (0x223879, (0x6694, false)), // East Asian ideograph
    (0x22387A, (0x668A, false)), // East Asian ideograph
    (0x21387B, (0x5916, false)), // East Asian ideograph
    (0x22387C, (0x6698, false)), // East Asian ideograph
    (0x22387D, (0x668D, false)), // East Asian ideograph
    (0x21387E, (0x591C, false)), // East Asian ideograph
    (0x234547, (0x936A, false)), // East Asian ideograph
    (0x22496B, (0x6DB7, false)), // East Asian ideograph
    (0x2D3F54, (0x61D0, false)), // East Asian ideograph
    (0x22496C, (0x6DE2, false)), // East Asian ideograph
    (0x6F5768, (0xC8E4, false)), // Korean hangul
    (0x4B393E, (0x5965, false)), // East Asian ideograph
    (0x27496D, (0x70E6, false)), // East Asian ideograph
    (0x22496E, (0x6DE9, false)), // East Asian ideograph
    (0x6F5765, (0xC8D5, false)), // Korean hangul
    (0x27496F, (0x7080, false)), // East Asian ideograph
    (0x21763E, (0x5810, false)), // East Asian ideograph
    (0x6F4B79, (0xB11C, false)), // Korean hangul
    (0x4D5053, (0x98DA, false)), // East Asian ideograph
    (0x6F4970, (0xAD70, false)), // Korean hangul
    (0x224247, (0x6A90, false)), // East Asian ideograph
    (0x224971, (0x6DF6, false)), // East Asian ideograph
    (0x28433A, (0x69E0, false)), // East Asian ideograph
    (0x295D42, (0x9E7E, false)), // East Asian ideograph
    (0x234972, (0x95E4, false)), // East Asian ideograph
    (0x6F7622, (0x3186, false)), // Korean hangul
    (0x27487B, (0x6DC0, false)), // East Asian ideograph
    (0x6F5766, (0xC8D7, false)), // Korean hangul
    (0x215F64, (0x971C, false)), // East Asian ideograph
    (0x213D4B, (0x5F48, false)), // East Asian ideograph
    (0x274975, (0x6247, false)), // East Asian ideograph
    (0x6F4976, (0xAD7D, false)), // Korean hangul
    (0x213421, (0x528D, false)), // East Asian ideograph
    (0x225257, (0x7160, false)), // East Asian ideograph
    (0x4B4977, (0x7188, false)), // East Asian ideograph
    (0x233422, (0x8B2B, false)), // East Asian ideograph
    (0x6F4978, (0xAD81, false)), // Korean hangul
    (0x4B4339, (0x6674, false)), // East Asian ideograph
    (0x223423, (0x644E, false)), // East Asian ideograph
    (0x223D6A, (0x6910, false)), // East Asian ideograph
    (0x224979, (0x6E0F, false)), // East Asian ideograph
    (0x213424, (0x529B, false)), // East Asian ideograph
    (0x22414B, (0x6A5C, false)), // East Asian ideograph
    (0x23497A, (0x961E, false)), // East Asian ideograph
    (0x283671, (0x6593, false)), // East Asian ideograph
    (0x23497B, (0x9624, false)), // East Asian ideograph
    (0x23497C, (0x9622, false)), // East Asian ideograph
    (0x213427, (0x52A3, false)), // East Asian ideograph
    (0x4B5963, (0x734F, false)), // East Asian ideograph
    (0x27497D, (0x70ED, false)), // East Asian ideograph
    (0x213428, (0x52AB, false)), // East Asian ideograph
    (0x27497E, (0x70EB, false)), // East Asian ideograph
    (0x213429, (0x52A9, false)), // East Asian ideograph
    (0x275D47, (0x9499, false)), // East Asian ideograph
    (0x23342A, (0x8B37, false)), // East Asian ideograph
    (0x273771, (0x56ED, false)), // East Asian ideograph
    (0x6F5843, (0xC9F1, false)), // Korean hangul
    (0x21342C, (0x52BE, false)), // East Asian ideograph
    (0x2D4F6B, (0x7AF8, false)), // East Asian ideograph
    (0x4C2962, (0x5F4D, false)), // East Asian ideograph (variant of 222962 which maps to 5F4D)
    (0x21342D, (0x52C7, false)), // East Asian ideograph
    (0x334C37, (0x8E6F, false)), // East Asian ideograph
    (0x215F67, (0x9727, false)), // East Asian ideograph
    (0x21742E, (0x5707, false)), // East Asian ideograph
    (0x23454C, (0x934F, false)), // East Asian ideograph
    (0x2D6078, (0x9920, false)), // East Asian ideograph
    (0x21342F, (0x52C1, false)), // East Asian ideograph
    (0x273772, (0x5706, false)), // East Asian ideograph
    (0x233921, (0x8DCF, false)), // East Asian ideograph
    (0x233922, (0x8DD6, false)), // East Asian ideograph
    (0x273923, (0x4F19, false)), // East Asian ideograph
    (0x223924, (0x7A25, false)), // East Asian ideograph
    (0x213925, (0x5927, false)), // East Asian ideograph
    (0x213926, (0x592A, false)), // East Asian ideograph
    (0x233927, (0x8DD0, false)), // East Asian ideograph
    (0x213928, (0x5929, false)), // East Asian ideograph
    (0x213929, (0x592D, false)), // East Asian ideograph
    (0x22392A, (0x66A0, false)), // East Asian ideograph
    (0x23392B, (0x8DC5, false)), // East Asian ideograph
    (0x21392C, (0x5937, false)), // East Asian ideograph
    (0x217432, (0x5714, false)), // East Asian ideograph
    (0x27392E, (0x5939, false)), // East Asian ideograph
    (0x23392F, (0x8DE4, false)), // East Asian ideograph
    (0x223930, (0x5C21, false)), // East Asian ideograph
    (0x213931, (0x5948, false)), // East Asian ideograph
    (0x223932, (0x669D, false)), // East Asian ideograph
    (0x213433, (0x52D9, false)), // East Asian ideograph
    (0x213934, (0x5955, false)), // East Asian ideograph
    (0x233935, (0x8DEB, false)), // East Asian ideograph
    (0x233936, (0x8DF4, false)), // East Asian ideograph
    (0x213937, (0x594F, false)), // East Asian ideograph
    (0x233938, (0x8DE9, false)), // East Asian ideograph
    (0x213434, (0x52D5, false)), // East Asian ideograph
    (0x22393A, (0x66B2, false)), // East Asian ideograph
    (0x23393B, (0x8DE3, false)), // East Asian ideograph
    (0x21393C, (0x5960, false)), // East Asian ideograph
    (0x23393D, (0x8DE7, false)), // East Asian ideograph
    (0x21393E, (0x5967, false)), // East Asian ideograph
    (0x23393F, (0x8E09, false)), // East Asian ideograph
    (0x223940, (0x66B5, false)), // East Asian ideograph
    (0x273941, (0x594B, false)), // East Asian ideograph
    (0x213942, (0x5973, false)), // East Asian ideograph
    (0x223943, (0x66AC, false)), // East Asian ideograph
    (0x233944, (0x8DFF, false)), // East Asian ideograph
    (0x213945, (0x5984, false)), // East Asian ideograph
    (0x233946, (0x8E05, false)), // East Asian ideograph
    (0x223947, (0x66B1, false)), // East Asian ideograph
    (0x213948, (0x597D, false)), // East Asian ideograph
    (0x233949, (0x8E01, false)), // East Asian ideograph
    (0x21394A, (0x5982, false)), // East Asian ideograph
    (0x21394B, (0x5981, false)), // East Asian ideograph
    (0x21394C, (0x59A8, false)), // East Asian ideograph
    (0x21394D, (0x5992, false)), // East Asian ideograph
    (0x23394E, (0x8E04, false)), // East Asian ideograph
    (0x22394F, (0x66BE, false)), // East Asian ideograph
    (0x233950, (0x8E06, false)), // East Asian ideograph
    (0x233438, (0x8B3E, false)), // East Asian ideograph
    (0x233952, (0x8E2A, false)), // East Asian ideograph
    (0x273953, (0x5986, false)), // East Asian ideograph
    (0x223954, (0x66C0, false)), // East Asian ideograph
    (0x223955, (0x66C7, false)), // East Asian ideograph
    (0x213956, (0x598A, false)), // East Asian ideograph
    (0x233957, (0x8E2E, false)), // East Asian ideograph
    (0x233958, (0x8E21, false)), // East Asian ideograph
    (0x213959, (0x59BB, false)), // East Asian ideograph
    (0x22395A, (0x66BB, false)), // East Asian ideograph
    (0x21395B, (0x59D1, false)), // East Asian ideograph
    (0x22395C, (0x66C4, false)), // East Asian ideograph
    (0x21343A, (0x52DF, false)), // East Asian ideograph
    (0x21395E, (0x59D0, false)), // East Asian ideograph
    (0x21395F, (0x59D7, false)), // East Asian ideograph
    (0x223960, (0x66CF, false)), // East Asian ideograph
    (0x213961, (0x59D2, false)), // East Asian ideograph
    (0x213962, (0x59D3, false)), // East Asian ideograph
    (0x213963, (0x59CA, false)), // East Asian ideograph
    (0x233964, (0x8E16, false)), // East Asian ideograph
    (0x213965, (0x59CB, false)), // East Asian ideograph
    (0x233966, (0x8E26, false)), // East Asian ideograph
    (0x213967, (0x59E3, false)), // East Asian ideograph
    (0x233968, (0x8E14, false)), // East Asian ideograph
    (0x213969, (0x59FF, false)), // East Asian ideograph
    (0x21396A, (0x59D8, false)), // East Asian ideograph
    (0x21396B, (0x5A03, false)), // East Asian ideograph
    (0x21396C, (0x59E8, false)), // East Asian ideograph
    (0x21396D, (0x59E5, false)), // East Asian ideograph
    (0x21396E, (0x59EA, false)), // East Asian ideograph
    (0x23396F, (0x8E41, false)), // East Asian ideograph
    (0x213970, (0x59FB, false)), // East Asian ideograph
    (0x223971, (0x66DA, false)), // East Asian ideograph
    (0x223972, (0x66DB, false)), // East Asian ideograph
    (0x223973, (0x66E2, false)), // East Asian ideograph
    (0x213974, (0x5A18, false)), // East Asian ideograph
    (0x213975, (0x5A23, false)), // East Asian ideograph
    (0x223976, (0x66E1, false)), // East Asian ideograph
    (0x233977, (0x8E40, false)), // East Asian ideograph
    (0x223978, (0x66E8, false)), // East Asian ideograph
    (0x233979, (0x8E36, false)), // East Asian ideograph
    (0x21397A, (0x5A1F, false)), // East Asian ideograph
    (0x21397B, (0x5A1B, false)), // East Asian ideograph
    (0x22397C, (0x66E9, false)), // East Asian ideograph
    (0x21397D, (0x5A29, false)), // East Asian ideograph
    (0x23397E, (0x8E3D, false)), // East Asian ideograph
    (0x27785A, (0x5785, false)), // East Asian ideograph
    (0x217441, (0x5724, false)), // East Asian ideograph
    (0x6F532A, (0xC12A, false)), // Korean hangul
    (0x213442, (0x5306, false)), // East Asian ideograph
    (0x334550, (0x7F47, false)), // East Asian ideograph
    (0x232337, (0x846E, false)), // East Asian ideograph
    (0x217443, (0x5729, false)), // East Asian ideograph
    (0x4B5521, (0x8332, false)), // East Asian ideograph
    (0x233444, (0x8B54, false)), // East Asian ideograph
    (0x235C71, (0x9E07, false)), // East Asian ideograph
    (0x22525E, (0x7176, false)), // East Asian ideograph
    (0x213445, (0x5310, false)), // East Asian ideograph
    (0x2D4B45, (0x6BEC, false)), // East Asian ideograph
    (0x6F5435, (0xC309, false)), // Korean hangul
    (0x22527B, (0x7187, false)), // East Asian ideograph
    (0x6F576E, (0xC900, false)), // Korean hangul
    (0x6F532B, (0xC12C, false)), // Korean hangul
    (0x6F2527, (0x3162, false)), // Korean hangul
    (0x227447, (0x7F63, false)), // East Asian ideograph
    (0x4B3666, (0x5A1A, false)), // East Asian ideograph
    (0x233448, (0x8B53, false)), // East Asian ideograph
    (0x233449, (0x8B4A, false)), // East Asian ideograph
    (0x275124, (0x7EB8, false)), // East Asian ideograph
    (0x223046, (0x6285, false)), // East Asian ideograph
    (0x21344A, (0x5319, false)), // East Asian ideograph
    (0x6F576F, (0xC904, false)), // Korean hangul
    (0x6F532C, (0xC12D, false)), // Korean hangul
    (0x23356F, (0x8C74, false)), // East Asian ideograph
    (0x6F4B7B, (0xB11E, false)), // Korean hangul
    (0x215B2A, (0x8E91, false)), // East Asian ideograph
    (0x6F4F6F, (0xB9DB, false)), // Korean hangul
    (0x21344D, (0x5321, false)), // East Asian ideograph
    (0x22344E, (0x64A2, false)), // East Asian ideograph
    (0x23344F, (0x8B3F, false)), // East Asian ideograph
    (0x2E7450, (0x7F82, false)), // East Asian ideograph
    (0x213451, (0x532F, false)), // East Asian ideograph
    (0x335230, (0x7F6E, false)), // East Asian ideograph (variant of 215230 which maps to 7F6E)
    (0x4B3668, (0x5358, false)), // East Asian ideograph
    (0x234553, (0x9356, false)), // East Asian ideograph
    (0x21725D, (0x5620, false)), // East Asian ideograph
    (0x27554F, (0x53F6, false)), // East Asian ideograph
    (0x213453, (0x5339, false)), // East Asian ideograph
    (0x223454, (0x6490, false)), // East Asian ideograph
    (0x213455, (0x5340, false)), // East Asian ideograph
    (0x215F6F, (0x9748, false)), // East Asian ideograph
    (0x215B2C, (0x8EAA, false)), // East Asian ideograph
    (0x234554, (0x9371, false)), // East Asian ideograph
    (0x6F5028, (0xBA4D, false)), // Korean hangul
    (0x283457, (0x63B8, false)), // East Asian ideograph
    (0x274B2D, (0x736D, false)), // East Asian ideograph
    (0x2D3458, (0x4EDF, false)), // East Asian ideograph
    (0x6F4C65, (0xB2D0, false)), // Korean hangul
    (0x284934, (0x6D43, false)), // East Asian ideograph
    (0x233459, (0x8B59, false)), // East Asian ideograph
    (0x6F5772, (0xC90D, false)), // Korean hangul
    (0x233A21, (0x8E30, false)), // East Asian ideograph
    (0x213A22, (0x5A49, false)), // East Asian ideograph
    (0x21345B, (0x5347, false)), // East Asian ideograph
    (0x233A24, (0x8E47, false)), // East Asian ideograph
    (0x213A25, (0x5A4A, false)), // East Asian ideograph
    (0x233A26, (0x8E46, false)), // East Asian ideograph
    (0x273A27, (0x5987, false)), // East Asian ideograph
    (0x273A28, (0x5A04, false)), // East Asian ideograph
    (0x213A29, (0x5A3C, false)), // East Asian ideograph
    (0x213A2A, (0x5A62, false)), // East Asian ideograph
    (0x213A2B, (0x5A5A, false)), // East Asian ideograph
    (0x213A2C, (0x5A77, false)), // East Asian ideograph
    (0x213A2D, (0x5A9A, false)), // East Asian ideograph
    (0x233A2E, (0x8E4C, false)), // East Asian ideograph
    (0x213A2F, (0x5A7F, false)), // East Asian ideograph
    (0x223A30, (0x670F, false)), // East Asian ideograph
    (0x224767, (0x6CAC, false)), // East Asian ideograph
    (0x233A32, (0x8E4F, false)), // East Asian ideograph
    (0x223A33, (0x6712, false)), // East Asian ideograph
    (0x223A34, (0x6713, false)), // East Asian ideograph
    (0x233A35, (0x8E62, false)), // East Asian ideograph
    (0x233A36, (0x8E60, false)), // East Asian ideograph
    (0x213A37, (0x5AB2, false)), // East Asian ideograph
    (0x223A38, (0x6719, false)), // East Asian ideograph
    (0x223A39, (0x6718, false)), // East Asian ideograph
    (0x233A3A, (0x8E54, false)), // East Asian ideograph
    (0x273A3B, (0x59AA, false)), // East Asian ideograph
    (0x213A3C, (0x5AD6, false)), // East Asian ideograph
    (0x213A3D, (0x5AE3, false)), // East Asian ideograph
    (0x233A3E, (0x8E5A, false)), // East Asian ideograph
    (0x233A3F, (0x8E5E, false)), // East Asian ideograph
    (0x233A40, (0x8E55, false)), // East Asian ideograph
    (0x273A41, (0x5A34, false)), // East Asian ideograph
    (0x213A42, (0x5B09, false)), // East Asian ideograph
    (0x273A43, (0x5A75, false)), // East Asian ideograph
    (0x273A44, (0x5A07, false)), // East Asian ideograph
    (0x273A45, (0x59A9, false)), // East Asian ideograph
    (0x233A46, (0x8E95, false)), // East Asian ideograph
    (0x223A47, (0x6723, false)), // East Asian ideograph
    (0x233A48, (0x8E6D, false)), // East Asian ideograph
    (0x213A49, (0x5B24, false)), // East Asian ideograph
    (0x273A4A, (0x5A74, false)), // East Asian ideograph
    (0x273A4B, (0x5A76, false)), // East Asian ideograph
    (0x223A4C, (0x673E, false)), // East Asian ideograph
    (0x213462, (0x5351, false)), // East Asian ideograph
    (0x223A4E, (0x673F, false)), // East Asian ideograph
    (0x213A4F, (0x5B53, false)), // East Asian ideograph
    (0x213A50, (0x5B54, false)), // East Asian ideograph
    (0x213A51, (0x5B55, false)), // East Asian ideograph
    (0x213A52, (0x5B57, false)), // East Asian ideograph
    (0x213A53, (0x5B58, false)), // East Asian ideograph
    (0x213A54, (0x5B5D, false)), // East Asian ideograph
    (0x213A55, (0x5B5C, false)), // East Asian ideograph
    (0x233A57, (0x8E8B, false)), // East Asian ideograph
    (0x223A58, (0x6757, false)), // East Asian ideograph
    (0x213A59, (0x5B64, false)), // East Asian ideograph
    (0x213A5A, (0x5B69, false)), // East Asian ideograph
    (0x273A5B, (0x5B59, false)), // East Asian ideograph
    (0x223A5C, (0x6747, false)), // East Asian ideograph
    (0x213A5D, (0x5B73, false)), // East Asian ideograph
    (0x233A5E, (0x8E9A, false)), // East Asian ideograph
    (0x273A5F, (0x5B5A, false)), // East Asian ideograph
    (0x273A60, (0x5B66, false)), // East Asian ideograph
    (0x223A61, (0x6755, false)), // East Asian ideograph
    (0x213A62, (0x5B7D, false)), // East Asian ideograph
    (0x233A63, (0x8E98, false)), // East Asian ideograph
    (0x233A64, (0x8E9E, false)), // East Asian ideograph
    (0x223466, (0x64B3, false)), // East Asian ideograph
    (0x223A66, (0x674C, false)), // East Asian ideograph
    (0x223A67, (0x6759, false)), // East Asian ideograph
    (0x223A68, (0x6748, false)), // East Asian ideograph
    (0x213A69, (0x5B8C, false)), // East Asian ideograph
    (0x275553, (0x8364, false)), // East Asian ideograph
    (0x233A6B, (0x8EA5, false)), // East Asian ideograph
    (0x213A6C, (0x5B97, false)), // East Asian ideograph
    (0x213A6D, (0x5B9A, false)), // East Asian ideograph
    (0x213A6E, (0x5B9C, false)), // East Asian ideograph
    (0x233A6F, (0x8EA7, false)), // East Asian ideograph
    (0x213A70, (0x5B99, false)), // East Asian ideograph
    (0x223A71, (0x674A, false)), // East Asian ideograph
    (0x233A72, (0x8E99, false)), // East Asian ideograph
    (0x213A73, (0x5BA3, false)), // East Asian ideograph
    (0x213A74, (0x5BA6, false)), // East Asian ideograph
    (0x213A75, (0x5BA4, false)), // East Asian ideograph
    (0x213A76, (0x5BA2, false)), // East Asian ideograph
    (0x213A77, (0x5BB0, false)), // East Asian ideograph
    (0x213A78, (0x5BB8, false)), // East Asian ideograph
    (0x233A7A, (0x8EBC, false)), // East Asian ideograph
    (0x213A7B, (0x5BB4, false)), // East Asian ideograph
    (0x223A7C, (0x6785, false)), // East Asian ideograph
    (0x213A7D, (0x5BB9, false)), // East Asian ideograph
    (0x213A7E, (0x5BB3, false)), // East Asian ideograph
    (0x23233F, (0x844A, false)), // East Asian ideograph
    (0x4B763D, (0x57F4, false)), // East Asian ideograph (variant of 21763D which maps to 57F4)
    (0x22746B, (0x7F7E, false)), // East Asian ideograph
    (0x283B7D, (0x53F0, false)), // East Asian ideograph (duplicate simplified)
    (0x22346C, (0x64D3, false)), // East Asian ideograph
    (0x6F5927, (0xCC39, false)), // Korean hangul
    (0x393B39, (0x5BF3, false)), // East Asian ideograph
    (0x213F26, (0x614C, false)), // East Asian ideograph
    (0x235222, (0x9957, false)), // East Asian ideograph (variant of 475222 which maps to 9957)
    (0x2D346E, (0x5373, false)), // East Asian ideograph
    (0x276232, (0x9E23, false)), // East Asian ideograph
    (0x6F5333, (0xC13C, false)), // Korean hangul
    (0x213D5B, (0x5F79, false)), // East Asian ideograph
    (0x213471, (0x5378, false)), // East Asian ideograph
    (0x287472, (0x7F74, false)), // East Asian ideograph
    (0x23344D, (0x8B56, false)), // East Asian ideograph
    (0x335223, (0x7E8E, false)), // East Asian ideograph
    (0x233473, (0x8B45, false)), // East Asian ideograph
    (0x273F3F, (0x51ED, false)), // East Asian ideograph
    (0x213474, (0x537F, false)), // East Asian ideograph
    (0x213475, (0x5384, false)), // East Asian ideograph
    (0x21325D, (0x50F9, false)), // East Asian ideograph
    (0x225F21, (0x75F9, false)), // East Asian ideograph
    (0x217477, (0x576D, false)), // East Asian ideograph
    (0x225F22, (0x75FC, false)), // East Asian ideograph
    (0x23456F, (0x9364, false)), // East Asian ideograph
    (0x275F23, (0x9648, false)), // East Asian ideograph
    (0x213479, (0x53A5, false)), // East Asian ideograph
    (0x275F24, (0x9646, false)), // East Asian ideograph
    (0x22747A, (0x7F91, false)), // East Asian ideograph
    (0x21347B, (0x53B2, false)), // East Asian ideograph
    (0x21392F, (0x5954, false)), // East Asian ideograph
    (0x225269, (0x7150, false)), // East Asian ideograph
    (0x4B4835, (0x6DA3, false)), // East Asian ideograph
    (0x21347D, (0x53C3, false)), // East Asian ideograph
    (0x2D5F28, (0x9665, false)), // East Asian ideograph
    (0x6F5336, (0xC149, false)), // Korean hangul
    (0x6F5329, (0xC127, false)), // Korean hangul
    (0x225F29, (0x7616, false)), // East Asian ideograph
    (0x275F2A, (0x9634, false)), // East Asian ideograph
    (0x275F2B, (0x961F, false)), // East Asian ideograph
    (0x216C41, (0x52D1, false)), // East Asian ideograph
    (0x225F2C, (0x7608, false)), // East Asian ideograph
    (0x6F577A, (0xC958, false)), // Korean hangul
    (0x225F2D, (0x7615, false)), // East Asian ideograph
    (0x295731, (0x9C8B, false)), // East Asian ideograph
    (0x276222, (0x9CC5, false)), // East Asian ideograph
    (0x225F2E, (0x760C, false)), // East Asian ideograph
    (0x23455D, (0x9349, false)), // East Asian ideograph
    (0x6F5567, (0xC5FE, false)), // Korean hangul
    (0x215F2F, (0x9685, false)), // East Asian ideograph
    (0x273340, (0x51BB, false)), // East Asian ideograph
    (0x223B21, (0x677B, false)), // East Asian ideograph
    (0x223B22, (0x6792, false)), // East Asian ideograph
    (0x223B23, (0x6776, false)), // East Asian ideograph
    (0x213B24, (0x5BC4, false)), // East Asian ideograph
    (0x223B25, (0x6791, false)), // East Asian ideograph
    (0x223B26, (0x6799, false)), // East Asian ideograph
    (0x215F31, (0x968D, false)), // East Asian ideograph
    (0x223B28, (0x67A4, false)), // East Asian ideograph
    (0x213B29, (0x5BD0, false)), // East Asian ideograph
    (0x213B2A, (0x5BD3, false)), // East Asian ideograph
    (0x213B2B, (0x5BE1, false)), // East Asian ideograph
    (0x213B2C, (0x5BE5, false)), // East Asian ideograph
    (0x215F32, (0x9698, false)), // East Asian ideograph
    (0x223B2E, (0x678F, false)), // East Asian ideograph
    (0x233B2F, (0x8ECF, false)), // East Asian ideograph
    (0x223B30, (0x6772, false)), // East Asian ideograph
    (0x223B31, (0x6798, false)), // East Asian ideograph (variant of 4C3B31 which maps to 6798)
    (0x223B32, (0x676A, false)), // East Asian ideograph
    (0x233B33, (0x8ED5, false)), // East Asian ideograph
    (0x213B34, (0x5BEE, false)), // East Asian ideograph
    (0x273B35, (0x5BBD, false)), // East Asian ideograph
    (0x273B36, (0x5BA1, false)), // East Asian ideograph
    (0x273B37, (0x5199, false)), // East Asian ideograph
    (0x273B38, (0x5BA0, false)), // East Asian ideograph
    (0x223B39, (0x67AC, false)), // East Asian ideograph
    (0x213B3A, (0x5BF8, false)), // East Asian ideograph
    (0x223B3B, (0x67A0, false)), // East Asian ideograph
    (0x213B3C, (0x5C01, false)), // East Asian ideograph
    (0x213B3D, (0x5C04, false)), // East Asian ideograph
    (0x213B3E, (0x5C09, false)), // East Asian ideograph
    (0x233B3F, (0x8EFA, false)), // East Asian ideograph
    (0x273B40, (0x5C06, false)), // East Asian ideograph
    (0x213B41, (0x5C0A, false)), // East Asian ideograph
    (0x233B42, (0x8EF9, false)), // East Asian ideograph
    (0x273B43, (0x5BF9, false)), // East Asian ideograph
    (0x223B44, (0x67F9, false)), // East Asian ideograph
    (0x213B45, (0x5C0F, false)), // East Asian ideograph
    (0x213B46, (0x5C11, false)), // East Asian ideograph
    (0x213B47, (0x5C16, false)), // East Asian ideograph
    (0x223B48, (0x678D, false)), // East Asian ideograph
    (0x223B49, (0x678C, false)), // East Asian ideograph
    (0x213B4A, (0x5C2C, false)), // East Asian ideograph
    (0x233B4B, (0x8EE8, false)), // East Asian ideograph
    (0x223B4C, (0x67FC, false)), // East Asian ideograph
    (0x213B4D, (0x5C38, false)), // East Asian ideograph
    (0x223B4E, (0x6810, false)), // East Asian ideograph
    (0x233B4F, (0x8EEB, false)), // East Asian ideograph
    (0x213B50, (0x5C40, false)), // East Asian ideograph
    (0x223B51, (0x67C8, false)), // East Asian ideograph
    (0x23455F, (0x935A, false)), // East Asian ideograph
    (0x213B53, (0x5C3E, false)), // East Asian ideograph
    (0x223B54, (0x67CC, false)), // East Asian ideograph
    (0x213B55, (0x5C45, false)), // East Asian ideograph
    (0x233B56, (0x8F00, false)), // East Asian ideograph
    (0x213B57, (0x5C4E, false)), // East Asian ideograph
    (0x223B58, (0x67C5, false)), // East Asian ideograph
    (0x233B59, (0x8F05, false)), // East Asian ideograph
    (0x233B5A, (0x8F08, false)), // East Asian ideograph
    (0x233B5B, (0x8F07, false)), // East Asian ideograph
    (0x223B5C, (0x67BB, false)), // East Asian ideograph
    (0x213B5D, (0x5C5B, false)), // East Asian ideograph (not in Unicode)
    (0x213B5E, (0x5C60, false)), // East Asian ideograph
    (0x223B5F, (0x67B0, false)), // East Asian ideograph
    (0x223B60, (0x6803, false)), // East Asian ideograph
    (0x223B61, (0x67F8, false)), // East Asian ideograph
    (0x213B62, (0x5C65, false)), // East Asian ideograph
    (0x273B63, (0x5C5E, false)), // East Asian ideograph
    (0x233B64, (0x8F2C, false)), // East Asian ideograph
    (0x213B65, (0x5C71, false)), // East Asian ideograph
    (0x225A2A, (0x741B, false)), // East Asian ideograph
    (0x213B67, (0x5C90, false)), // East Asian ideograph
    (0x213B68, (0x5C8C, false)), // East Asian ideograph
    (0x213B69, (0x5C91, false)), // East Asian ideograph
    (0x213B6A, (0x5C94, false)), // East Asian ideograph
    (0x233B6B, (0x8F1E, false)), // East Asian ideograph
    (0x213B6C, (0x5CB8, false)), // East Asian ideograph
    (0x233B6D, (0x8F25, false)), // East Asian ideograph
    (0x233B6E, (0x8F20, false)), // East Asian ideograph
    (0x223B6F, (0x67E4, false)), // East Asian ideograph
    (0x223B70, (0x67D9, false)), // East Asian ideograph
    (0x223B71, (0x67DB, false)), // East Asian ideograph
    (0x223B72, (0x67B5, false)), // East Asian ideograph
    (0x213B73, (0x5D01, false)), // East Asian ideograph
    (0x273B74, (0x5CE1, false)), // East Asian ideograph
    (0x223B75, (0x67F7, false)), // East Asian ideograph
    (0x213B76, (0x5CFB, false)), // East Asian ideograph
    (0x223B77, (0x67B3, false)), // East Asian ideograph
    (0x233B78, (0x8F36, false)), // East Asian ideograph
    (0x233B79, (0x8F2E, false)), // East Asian ideograph
    (0x233B7A, (0x8F33, false)), // East Asian ideograph
    (0x215F3F, (0x96C0, false)), // East Asian ideograph
    (0x223B7C, (0x67EE, false)), // East Asian ideograph
    (0x223B7D, (0x6AAF, false)), // East Asian ideograph
    (0x223B7E, (0x67B2, false)), // East Asian ideograph
    (0x225F40, (0x761B, false)), // East Asian ideograph
    (0x6F577E, (0xC970, false)), // Korean hangul
    (0x6F533B, (0xC158, false)), // Korean hangul
    (0x213D63, (0x5F8A, false)), // East Asian ideograph
    (0x6F4B7E, (0xB125, false)), // Korean hangul
    (0x2D5D2F, (0x9196, false)), // East Asian ideograph
    (0x2D5F43, (0x9CEB, false)), // East Asian ideograph
    (0x6F2459, (0x3137, false)), // Korean hangul
    (0x293B42, (0x8F75, false)), // East Asian ideograph
    (0x235F45, (0x9F22, false)), // East Asian ideograph
    (0x2D5F46, (0x96BD, false)), // East Asian ideograph
    (0x6F533C, (0xC167, false)), // Korean hangul
    (0x213D64, (0x5F87, false)), // East Asian ideograph
    (0x225F47, (0x7619, false)), // East Asian ideograph
    (0x234562, (0x935F, false)), // East Asian ideograph
    (0x235F48, (0x9F2B, false)), // East Asian ideograph
    (0x2E604A, (0x7690, false)), // East Asian ideograph
    (0x235F49, (0x9F26, false)), // East Asian ideograph
    (0x225270, (0x7144, false)), // East Asian ideograph
    (0x6F5D65, (0xD72D, false)), // Korean hangul
    (0x224E2D, (0x6FC9, false)), // East Asian ideograph
    (0x2D4B3F, (0x73CE, false)), // East Asian ideograph
    (0x275F4B, (0x6742, false)), // East Asian ideograph
    (0x276234, (0x9E29, false)), // East Asian ideograph
    (0x6F533D, (0xC168, false)), // Korean hangul
    (0x225F4C, (0x761D, false)), // East Asian ideograph
    (0x275F4D, (0x96CF, false)), // East Asian ideograph
    (0x6F5773, (0xC90F, false)), // Korean hangul
    (0x6F245B, (0x3141, false)), // Korean hangul
    (0x275F4E, (0x53CC, false)), // East Asian ideograph
    (0x216C48, (0x52D6, false)), // East Asian ideograph
    (0x275F4F, (0x79BB, false)), // East Asian ideograph
    (0x215F50, (0x96E3, false)), // East Asian ideograph (variant of 4B5F50 which maps to 96E3)
    (0x6F533E, (0xC170, false)), // Korean hangul
    (0x213D66, (0x5F92, false)), // East Asian ideograph
    (0x215F51, (0x96E8, false)), // East Asian ideograph
    (0x225F3B, (0x7610, false)), // East Asian ideograph
    (0x284345, (0x680C, false)), // East Asian ideograph
    (0x6F5848, (0xCA08, false)), // Korean hangul
    (0x6F245C, (0x3142, false)), // Korean hangul
    (0x235F53, (0x9F2F, false)), // East Asian ideograph
    (0x225F54, (0x762D, false)), // East Asian ideograph
    (0x234571, (0x936B, false)), // East Asian ideograph
    (0x6F505B, (0xBBC0, false)), // Korean hangul
    (0x275F55, (0x7535, false)), // East Asian ideograph
    (0x6F533F, (0xC18C, false)), // Korean hangul
    (0x213D67, (0x5F91, false)), // East Asian ideograph
    (0x6F4D64, (0xB4E0, false)), // Korean hangul
    (0x213924, (0x5922, false)), // East Asian ideograph
    (0x6F245D, (0x3145, false)), // Korean hangul
    (0x275128, (0x7ECB, false)), // East Asian ideograph
    (0x4B5F58, (0xF9B2, false)), // East Asian ideograph
    (0x227049, (0x7D0F, false)), // East Asian ideograph
    (0x224E30, (0x6FA0, false)), // East Asian ideograph
    (0x293338, (0x8BFD, false)), // East Asian ideograph
    (0x2E715A, (0x7E27, false)), // East Asian ideograph
    (0x215F5A, (0x9707, false)), // East Asian ideograph
    (0x6F5340, (0xC18D, false)), // Korean hangul
    (0x223C21, (0x67B9, false)), // East Asian ideograph
    (0x213C22, (0x5D11, false)), // East Asian ideograph
    (0x215B3E, (0x8EFE, false)), // East Asian ideograph
    (0x223C24, (0x67E3, false)), // East Asian ideograph
    (0x213C25, (0x5D14, false)), // East Asian ideograph
    (0x233C26, (0x8F39, false)), // East Asian ideograph
    (0x233C27, (0x8F34, false)), // East Asian ideograph
    (0x273C28, (0x5C9A, false)), // East Asian ideograph
    (0x223C29, (0x67E2, false)), // East Asian ideograph
    (0x273C2A, (0x5D2D, false)), // East Asian ideograph
    (0x273C2B, (0x5C96, false)), // East Asian ideograph
    (0x213C2C, (0x5D9D, false)), // East Asian ideograph
    (0x273C2D, (0x5C7F, false)), // East Asian ideograph
    (0x273C2E, (0x5CB3, false)), // East Asian ideograph
    (0x223C2F, (0x67E7, false)), // East Asian ideograph
    (0x223C30, (0x6849, false)), // East Asian ideograph
    (0x223C31, (0x683E, false)), // East Asian ideograph
    (0x273C32, (0x5DC5, false)), // East Asian ideograph
    (0x214A32, (0x71EC, false)), // East Asian ideograph
    (0x213C34, (0x5DDD, false)), // East Asian ideograph
    (0x215F5E, (0x9711, false)), // East Asian ideograph
    (0x223C36, (0x6814, false)), // East Asian ideograph
    (0x223C37, (0x684B, false)), // East Asian ideograph
    (0x223C38, (0x681E, false)), // East Asian ideograph
    (0x213C39, (0x5DE7, false)), // East Asian ideograph
    (0x213C3A, (0x5DE6, false)), // East Asian ideograph
    (0x223C3B, (0x6833, false)), // East Asian ideograph
    (0x213C3C, (0x5DEE, false)), // East Asian ideograph
    (0x233C3D, (0x8F52, false)), // East Asian ideograph
    (0x213C3E, (0x5DF2, false)), // East Asian ideograph
    (0x213C3F, (0x5DF3, false)), // East Asian ideograph
    (0x223C40, (0x6831, false)), // East Asian ideograph
    (0x215F60, (0x9716, false)), // East Asian ideograph
    (0x223C42, (0x6835, false)), // East Asian ideograph
    (0x223C43, (0x683B, false)), // East Asian ideograph
    (0x223C44, (0x684E, false)), // East Asian ideograph
    (0x213C46, (0x5E06, false)), // East Asian ideograph
    (0x234124, (0x918D, false)), // East Asian ideograph
    (0x233C48, (0x8F56, false)), // East Asian ideograph
    (0x213C49, (0x5E1A, false)), // East Asian ideograph
    (0x223C4A, (0x684D, false)), // East Asian ideograph
    (0x233C4B, (0x8F55, false)), // East Asian ideograph
    (0x233C4C, (0x8F58, false)), // East Asian ideograph
    (0x215F62, (0x970D, false)), // East Asian ideograph
    (0x233C4E, (0x8F5E, false)), // East Asian ideograph
    (0x273C4F, (0x5E05, false)), // East Asian ideograph
    (0x273C51, (0x5E08, false)), // East Asian ideograph
    (0x273C52, (0x5E10, false)), // East Asian ideograph
    (0x273C53, (0x5E26, false)), // East Asian ideograph
    (0x213C54, (0x5E38, false)), // East Asian ideograph
    (0x223C55, (0x685D, false)), // East Asian ideograph
    (0x223C56, (0x685E, false)), // East Asian ideograph
    (0x233C57, (0x8F62, false)), // East Asian ideograph
    (0x273C58, (0x5E27, false)), // East Asian ideograph
    (0x233C59, (0x8F63, false)), // East Asian ideograph
    (0x233C5A, (0x8F64, false)), // East Asian ideograph
    (0x213C5B, (0x5E54, false)), // East Asian ideograph
    (0x273C5C, (0x5E3C, false)), // East Asian ideograph
    (0x213C5D, (0x5E55, false)), // East Asian ideograph
    (0x273C5E, (0x5E01, false)), // East Asian ideograph
    (0x213C5F, (0x5E62, false)), // East Asian ideograph
    (0x273C60, (0x5E1C, false)), // East Asian ideograph
    (0x273C61, (0x5E2E, false)), // East Asian ideograph
    (0x213C63, (0x5E73, false)), // East Asian ideograph
    (0x6F5177, (0xBE5A, false)), // Korean hangul
    (0x223C65, (0x685A, false)), // East Asian ideograph
    (0x233C66, (0x8FA5, false)), // East Asian ideograph
    (0x273C67, (0x5E72, false)), // East Asian ideograph (Version J extension)
    (0x223C68, (0x686B, false)), // East Asian ideograph
    (0x223C69, (0x686C, false)), // East Asian ideograph
    (0x213C6A, (0x5E7D, false)), // East Asian ideograph
    (0x223C6B, (0x6879, false)), // East Asian ideograph
    (0x233C6C, (0x8FB5, false)), // East Asian ideograph
    (0x213C6D, (0x5E87, false)), // East Asian ideograph
    (0x233C6E, (0x8FBB, false)), // East Asian ideograph
    (0x213C6F, (0x5E9A, false)), // East Asian ideograph
    (0x233C70, (0x8FBC, false)), // East Asian ideograph
    (0x215F68, (0x9738, false)), // East Asian ideograph
    (0x223C72, (0x687E, false)), // East Asian ideograph
    (0x213C73, (0x5E95, false)), // East Asian ideograph
    (0x233C74, (0x8FBF, false)), // East Asian ideograph
    (0x233C75, (0x8FD2, false)), // East Asian ideograph
    (0x273C76, (0x5E93, false)), // East Asian ideograph
    (0x225F69, (0x7647, false)), // East Asian ideograph
    (0x213C78, (0x5EAD, false)), // East Asian ideograph
    (0x213C79, (0x5EB7, false)), // East Asian ideograph
    (0x233C7A, (0x8FCA, false)), // East Asian ideograph
    (0x233C7B, (0x8FD3, false)), // East Asian ideograph
    (0x213C7C, (0x5EB5, false)), // East Asian ideograph
    (0x215F6A, (0x9732, false)), // East Asian ideograph
    (0x273C7E, (0x5395, false)), // East Asian ideograph
    (0x275F6B, (0x9701, false)), // East Asian ideograph
    (0x6F5849, (0xCA09, false)), // Korean hangul
    (0x6F2461, (0x314B, false)), // Korean hangul
    (0x275725, (0x8682, false)), // East Asian ideograph
    (0x275122, (0x7EB3, false)), // East Asian ideograph
    (0x6F5273, (0xC0D8, false)), // Korean hangul
    (0x235F6D, (0x9F45, false)), // East Asian ideograph
    (0x4C476E, (0x6CAD, false)), // East Asian ideograph
    (0x225A2C, (0x7432, false)), // East Asian ideograph
    (0x225F6E, (0x764D, false)), // East Asian ideograph
    (0x6F2528, (0x3163, false)), // Korean hangul
    (0x2F312B, (0x89BB, false)), // East Asian ideograph
    (0x235F6F, (0x9F46, false)), // East Asian ideograph
    (0x4B5F70, (0x9752, false)), // East Asian ideograph
    (0x6F2462, (0x314C, false)), // Korean hangul
    (0x235F71, (0x9F48, false)), // East Asian ideograph
    (0x275123, (0x7EA7, false)), // East Asian ideograph
    (0x214A36, (0x720D, false)), // East Asian ideograph
    (0x224E35, (0x6FB4, false)), // East Asian ideograph
    (0x335234, (0x99E1, false)), // East Asian ideograph
    (0x2E3D62, (0x684A, false)), // East Asian ideograph
    (0x235F73, (0x9F49, false)), // East Asian ideograph
    (0x69253B, (0x30BB, false)), // Katakana letter SE
    (0x276230, (0x9E20, false)), // East Asian ideograph
    (0x23456B, (0x9374, false)), // East Asian ideograph
    (0x215F75, (0x9760, false)), // East Asian ideograph
    (0x692431, (0x3051, false)), // Hiragana letter KE
    (0x274A21, (0x70BD, false)), // East Asian ideograph
    (0x23345F, (0x8B4D, false)), // East Asian ideograph
    (0x224D63, (0x6F5F, false)), // East Asian ideograph
    (0x274A22, (0x7096, false)), // East Asian ideograph
    (0x4D446B, (0x954E, false)), // East Asian ideograph
    (0x6F4A23, (0xADC4, false)), // Korean hangul
    (0x6F5346, (0xC19F, false)), // Korean hangul
    (0x276231, (0x9E22, false)), // East Asian ideograph
    (0x215F79, (0x9768, false)), // East Asian ideograph
    (0x274A24, (0x706F, false)), // East Asian ideograph
    (0x345E3B, (0x80AC, false)), // East Asian ideograph
    (0x215F7A, (0x9769, false)), // East Asian ideograph
    (0x274A25, (0x7116, false)), // East Asian ideograph
    (0x275568, (0x8298, false)), // East Asian ideograph
    (0x215F7B, (0x9776, false)), // East Asian ideograph
    (0x274A26, (0x70E7, false)), // East Asian ideograph
    (0x6F5D67, (0xD73C, false)), // Korean hangul
    (0x215F7C, (0x9774, false)), // East Asian ideograph
    (0x6F4A27, (0xADD3, false)), // Korean hangul
    (0x2E3172, (0x5261, false)), // East Asian ideograph
    (0x276236, (0x9E35, false)), // East Asian ideograph
    (0x2D4A28, (0x8B8C, false)), // East Asian ideograph
    (0x6F5347, (0xC1A1, false)), // Korean hangul
    (0x213D6F, (0x5FA9, false)), // East Asian ideograph
    (0x215F7E, (0x9785, false)), // East Asian ideograph
    (0x33456D, (0x826A, false)), // East Asian ideograph
    (0x6F5178, (0xBE5B, false)), // Korean hangul
    (0x224A2A, (0x6DDF, false)), // East Asian ideograph
    (0x6F2465, (0x3132, false)), // Korean hangul
    (0x275126, (0x7ECA, false)), // East Asian ideograph
    (0x23567A, (0x9B95, false)), // East Asian ideograph
    (0x6F4A2C, (0xADF8, false)), // Korean hangul
    (0x224A2D, (0x6DD3, false)), // East Asian ideograph
    (0x6F5348, (0xC1A5, false)), // Korean hangul
    (0x276233, (0x51E4, false)), // East Asian ideograph
    (0x274A2E, (0x8425, false)), // East Asian ideograph
    (0x2E3328, (0x6528, false)), // East Asian ideograph
    (0x6F5D6B, (0xD751, false)), // Korean hangul
    (0x234A2F, (0x9642, false)), // East Asian ideograph
    (0x4B462A, (0x6B74, false)), // East Asian ideograph
    (0x233D21, (0x8FDA, false)), // East Asian ideograph
    (0x233D22, (0x8FD5, false)), // East Asian ideograph
    (0x213D23, (0x5EC9, false)), // East Asian ideograph
    (0x213D24, (0x5EC8, false)), // East Asian ideograph
    (0x223D25, (0x686D, false)), // East Asian ideograph
    (0x213D26, (0x5ED6, false)), // East Asian ideograph
    (0x273D27, (0x5E9F, false)), // East Asian ideograph
    (0x213D28, (0x5EDA, false)), // East Asian ideograph
    (0x213D29, (0x5EDD, false)), // East Asian ideograph
    (0x273D2A, (0x5E7F, false)), // East Asian ideograph
    (0x273D2B, (0x5E99, false)), // East Asian ideograph
    (0x273D2C, (0x5382, false)), // East Asian ideograph
    (0x273D2D, (0x5E9E, false)), // East Asian ideograph
    (0x273D2E, (0x5E90, false)), // East Asian ideograph
    (0x233D2F, (0x8FE4, false)), // East Asian ideograph
    (0x233D30, (0x8FEE, false)), // East Asian ideograph
    (0x223D32, (0x688B, false)), // East Asian ideograph
    (0x274A33, (0x70E9, false)), // East Asian ideograph
    (0x213D34, (0x5EFF, false)), // East Asian ideograph
    (0x233D35, (0x8FF9, false)), // East Asian ideograph
    (0x213D36, (0x5F04, false)), // East Asian ideograph
    (0x213D37, (0x5F08, false)), // East Asian ideograph
    (0x213D38, (0x5F0A, false)), // East Asian ideograph
    (0x223D39, (0x68A3, false)), // East Asian ideograph
    (0x213D3A, (0x5F12, false)), // East Asian ideograph
    (0x213D3B, (0x5F13, false)), // East Asian ideograph
    (0x233D3C, (0x8FFB, false)), // East Asian ideograph
    (0x213D3D, (0x5F14, false)), // East Asian ideograph
    (0x213D3E, (0x5F18, false)), // East Asian ideograph
    (0x214A35, (0x7206, false)), // East Asian ideograph
    (0x223D40, (0x688F, false)), // East Asian ideograph
    (0x213D41, (0x5F1F, false)), // East Asian ideograph
    (0x213D42, (0x5F26, false)), // East Asian ideograph
    (0x223D43, (0x687B, false)), // East Asian ideograph
    (0x233D44, (0x9011, false)), // East Asian ideograph
    (0x224A36, (0x6DDC, false)), // East Asian ideograph
    (0x213D46, (0x5F31, false)), // East Asian ideograph
    (0x273D47, (0x5F20, false)), // East Asian ideograph
    (0x213D48, (0x5F37, false)), // East Asian ideograph
    (0x233D49, (0x9021, false)), // East Asian ideograph
    (0x233D4A, (0x902D, false)), // East Asian ideograph
    (0x274A37, (0x7089, false)), // East Asian ideograph
    (0x273D4C, (0x5F25, false)), // East Asian ideograph
    (0x273D4D, (0x5F2F, false)), // East Asian ideograph
    (0x233D4E, (0x902C, false)), // East Asian ideograph
    (0x273D4F, (0x6C47, false)), // East Asian ideograph (duplicate simplified)
    (0x223D50, (0x692C, false)), // East Asian ideograph
    (0x274A38, (0x70C2, false)), // East Asian ideograph
    (0x213D52, (0x5F64, false)), // East Asian ideograph
    (0x223D53, (0x690C, false)), // East Asian ideograph
    (0x213D54, (0x5F6C, false)), // East Asian ideograph
    (0x213D55, (0x5F69, false)), // East Asian ideograph
    (0x233D56, (0x9037, false)), // East Asian ideograph
    (0x214A39, (0x7228, false)), // East Asian ideograph
    (0x223D58, (0x68D3, false)), // East Asian ideograph
    (0x213D59, (0x5F71, false)), // East Asian ideograph
    (0x223D5B, (0x690A, false)), // East Asian ideograph
    (0x223D5C, (0x6909, false)), // East Asian ideograph
    (0x233D5D, (0x9052, false)), // East Asian ideograph
    (0x213D5E, (0x5F7F, false)), // East Asian ideograph
    (0x213D5F, (0x5F7C, false)), // East Asian ideograph
    (0x213D60, (0x5F85, false)), // East Asian ideograph
    (0x213D61, (0x5F88, false)), // East Asian ideograph
    (0x223D62, (0x68EC, false)), // East Asian ideograph
    (0x223D63, (0x692A, false)), // East Asian ideograph
    (0x223D64, (0x68EA, false)), // East Asian ideograph
    (0x223D65, (0x681F, false)), // East Asian ideograph
    (0x223D66, (0x7439, false)), // East Asian ideograph
    (0x233D67, (0x9049, false)), // East Asian ideograph
    (0x213D68, (0x5F90, false)), // East Asian ideograph
    (0x234A3C, (0x9651, false)), // East Asian ideograph
    (0x233D6A, (0x9044, false)), // East Asian ideograph
    (0x213D6B, (0x5F99, false)), // East Asian ideograph
    (0x273D6C, (0x4ECE, false)), // East Asian ideograph
    (0x223D6E, (0x68D6, false)), // East Asian ideograph
    (0x223D6F, (0x68EB, false)), // East Asian ideograph
    (0x225F48, (0x761E, false)), // East Asian ideograph
    (0x213D71, (0x5FAA, false)), // East Asian ideograph
    (0x213D72, (0x5FAC, false)), // East Asian ideograph
    (0x223D73, (0x68F1, false)), // East Asian ideograph
    (0x273D74, (0x5F7B, false)), // East Asian ideograph
    (0x233D75, (0x905D, false)), // East Asian ideograph
    (0x273D76, (0x5F81, false)), // East Asian ideograph
    (0x213D77, (0x5FBD, false)), // East Asian ideograph
    (0x233D78, (0x905B, false)), // East Asian ideograph
    (0x223D79, (0x68FC, false)), // East Asian ideograph
    (0x213D7A, (0x5FD9, false)), // East Asian ideograph
    (0x233D7B, (0x906B, false)), // East Asian ideograph
    (0x223D7C, (0x6913, false)), // East Asian ideograph
    (0x213D7D, (0x5FD6, false)), // East Asian ideograph
    (0x224E3C, (0x6FA8, false)), // East Asian ideograph
    (0x23523B, (0x99A3, false)), // East Asian ideograph
    (0x6F534C, (0xC1C4, false)), // Korean hangul
    (0x276237, (0x9E2A, false)), // East Asian ideograph
    (0x224173, (0x6A8D, false)), // East Asian ideograph
    (0x274A42, (0x7237, false)), // East Asian ideograph
    (0x223D30, (0x6898, false)), // East Asian ideograph
    (0x6F5925, (0xCC30, false)), // Korean hangul
    (0x2D5C5B, (0x9089, false)), // East Asian ideograph
    (0x6F4A43, (0xAE4C, false)), // Korean hangul
    (0x234A44, (0x965C, false)), // East Asian ideograph
    (0x232435, (0x84DA, false)), // East Asian ideograph
    (0x696E5C, (0x91DF, false)), // East Asian ideograph
    (0x295929, (0x9CA3, false)), // East Asian ideograph
    (0x213E51, (0x608D, false)), // East Asian ideograph
    (0x274A45, (0x5C14, false)), // East Asian ideograph
    (0x233023, (0x8962, false)), // East Asian ideograph
    (0x214A46, (0x7246, false)), // East Asian ideograph
    (0x6F534D, (0xC1C8, false)), // Korean hangul
    (0x276238, (0x9E2D, false)), // East Asian ideograph
    (0x6F5740, (0xC81D, false)), // Korean hangul
    (0x213932, (0x5947, false)), // East Asian ideograph
    (0x295574, (0x9604, false)), // East Asian ideograph
    (0x6F4A48, (0xAE5C, false)), // Korean hangul
    (0x277345, (0x556E, false)), // East Asian ideograph
    (0x6F4A49, (0xAE5D, false)), // Korean hangul
    (0x29592A, (0x9CD3, false)), // East Asian ideograph
    (0x4B4352, (0x66F5, false)), // East Asian ideograph
    (0x234A4A, (0x965F, false)), // East Asian ideograph
    (0x234A4B, (0x9656, false)), // East Asian ideograph
    (0x6F534E, (0xC1D7, false)), // Korean hangul
    (0x213D76, (0x5FB5, false)), // East Asian ideograph
    (0x274A4C, (0x724D, false)), // East Asian ideograph
    (0x6F4A4D, (0xAE65, false)), // Korean hangul
    (0x6F5771, (0xC90C, false)), // Korean hangul
    (0x295928, (0x9CD5, false)), // East Asian ideograph
    (0x6F4A4E, (0xAE68, false)), // Korean hangul
    (0x334050, (0x62D5, false)), // East Asian ideograph
    (0x23523E, (0x99A6, false)), // East Asian ideograph
    (0x4C2539, (0x5D73, false)), // East Asian ideograph
    (0x4D3363, (0x8C25, false)), // East Asian ideograph
    (0x224A50, (0x6E27, false)), // East Asian ideograph
    (0x6F534F, (0xC1E0, false)), // Korean hangul
    (0x27623A, (0x9E33, false)), // East Asian ideograph
    (0x4B485F, (0x6EDE, false)), // East Asian ideograph (variant of 27485F which maps to 6EDE)
    (0x234A51, (0x966C, false)), // East Asian ideograph
    (0x23235C, (0x84B4, false)), // East Asian ideograph
    (0x285A47, (0x73AE, false)), // East Asian ideograph
    (0x2D3165, (0x349E, false)), // East Asian ideograph (not found in unified han)
    (0x6F4A52, (0xAE78, false)), // Korean hangul
    (0x275571, (0x848B, false)), // East Asian ideograph
    (0x274A53, (0x5B83, false)), // East Asian ideograph
    (0x227059, (0x7D35, false)), // East Asian ideograph
    (0x33523F, (0x8B71, false)), // East Asian ideograph
    (0x473422, (0x8C2A, false)), // East Asian ideograph
    (0x224A55, (0x6E49, false)), // East Asian ideograph
    (0x213D78, (0x5FC3, false)), // East Asian ideograph
    (0x213935, (0x5951, false)), // East Asian ideograph
    (0x223D34, (0x686F, false)), // East Asian ideograph
    (0x4B3248, (0x50B2, false)), // East Asian ideograph (not in Unicode)
    (0x6F4A57, (0xAE84, false)), // Korean hangul
    (0x224A58, (0x6E3C, false)), // East Asian ideograph
    (0x6F5D69, (0xD749, false)), // Korean hangul
    (0x6F4A59, (0xAEBC, false)), // Korean hangul
    (0x274A5A, (0x7275, false)), // East Asian ideograph
    (0x6F5351, (0xC1E8, false)), // Korean hangul
    (0x27623C, (0x9E3F, false)), // East Asian ideograph
    (0x223E21, (0x6907, false)), // East Asian ideograph
    (0x213E22, (0x5FEB, false)), // East Asian ideograph
    (0x213E23, (0x5FE0, false)), // East Asian ideograph
    (0x213E24, (0x5FF1, false)), // East Asian ideograph
    (0x233E25, (0x906F, false)), // East Asian ideograph
    (0x233E26, (0x9079, false)), // East Asian ideograph
    (0x213E27, (0x5FF5, false)), // East Asian ideograph
    (0x233E28, (0x9076, false)), // East Asian ideograph
    (0x213E29, (0x6014, false)), // East Asian ideograph
    (0x223E2A, (0x68DE, false)), // East Asian ideograph
    (0x223E2B, (0x691B, false)), // East Asian ideograph
    (0x233E2C, (0x9085, false)), // East Asian ideograph
    (0x223E2D, (0x68FB, false)), // East Asian ideograph
    (0x213E2E, (0x601D, false)), // East Asian ideograph
    (0x234A5D, (0x967B, false)), // East Asian ideograph
    (0x213E30, (0x6021, false)), // East Asian ideograph
    (0x213E31, (0x6020, false)), // East Asian ideograph
    (0x213E32, (0x6028, false)), // East Asian ideograph
    (0x223E33, (0x68E1, false)), // East Asian ideograph
    (0x213E34, (0x6027, false)), // East Asian ideograph
    (0x214A5E, (0x7296, false)), // East Asian ideograph
    (0x213E36, (0x6015, false)), // East Asian ideograph
    (0x223E37, (0x68D1, false)), // East Asian ideograph
    (0x223E38, (0x68D0, false)), // East Asian ideograph
    (0x223E39, (0x6908, false)), // East Asian ideograph
    (0x233E3A, (0x908B, false)), // East Asian ideograph
    (0x213E3B, (0x6043, false)), // East Asian ideograph
    (0x213E3C, (0x6065, false)), // East Asian ideograph
    (0x213E3D, (0x6050, false)), // East Asian ideograph
    (0x223E3E, (0x68E8, false)), // East Asian ideograph
    (0x223E3F, (0x68F0, false)), // East Asian ideograph
    (0x223E40, (0x68C3, false)), // East Asian ideograph
    (0x214A60, (0x729B, false)), // East Asian ideograph
    (0x235164, (0x9940, false)), // East Asian ideograph
    (0x233E43, (0x909B, false)), // East Asian ideograph
    (0x233E44, (0x909C, false)), // East Asian ideograph
    (0x213E45, (0x606F, false)), // East Asian ideograph
    (0x223E46, (0x68D4, false)), // East Asian ideograph
    (0x274A61, (0x728A, false)), // East Asian ideograph
    (0x233E48, (0x90A1, false)), // East Asian ideograph
    (0x223E49, (0x68C6, false)), // East Asian ideograph
    (0x213E4A, (0x608C, false)), // East Asian ideograph
    (0x223E4B, (0x68C7, false)), // East Asian ideograph
    (0x213E4C, (0x607F, false)), // East Asian ideograph
    (0x214A62, (0x72A7, false)), // East Asian ideograph
    (0x213E4E, (0x609A, false)), // East Asian ideograph
    (0x213E4F, (0x6096, false)), // East Asian ideograph
    (0x213E50, (0x6084, false)), // East Asian ideograph
    (0x233E51, (0x90A8, false)), // East Asian ideograph
    (0x224828, (0x6CCE, false)), // East Asian ideograph
    (0x234A63, (0x9684, false)), // East Asian ideograph
    (0x233E54, (0x90A0, false)), // East Asian ideograph
    (0x223E55, (0x6938, false)), // East Asian ideograph
    (0x213E56, (0x60A8, false)), // East Asian ideograph
    (0x273E57, (0x5FF0, false)), // East Asian ideograph
    (0x233E58, (0x90AF, false)), // East Asian ideograph
    (0x233E59, (0x90B3, false)), // East Asian ideograph
    (0x6F4C5D, (0xB2A1, false)), // Korean hangul
    (0x213E5B, (0x60C5, false)), // East Asian ideograph (variant of 4B3E5B which maps to 60C5)
    (0x273E5C, (0x95F7, false)), // East Asian ideograph
    (0x223E5D, (0x6958, false)), // East Asian ideograph
    (0x273E5E, (0x6005, false)), // East Asian ideograph
    (0x213E5F, (0x60BB, false)), // East Asian ideograph
    (0x213E60, (0x60E0, false)), // East Asian ideograph
    (0x233E61, (0x90B2, false)), // East Asian ideograph
    (0x213E62, (0x60DC, false)), // East Asian ideograph
    (0x213E63, (0x60D8, false)), // East Asian ideograph
    (0x223E64, (0x6945, false)), // East Asian ideograph
    (0x223E65, (0x695D, false)), // East Asian ideograph
    (0x223E66, (0x6932, false)), // East Asian ideograph
    (0x213E67, (0x60C6, false)), // East Asian ideograph
    (0x233E68, (0x90C9, false)), // East Asian ideograph
    (0x223E69, (0x696E, false)), // East Asian ideograph
    (0x223E6A, (0x6963, false)), // East Asian ideograph
    (0x223E6B, (0x6948, false)), // East Asian ideograph
    (0x273E6C, (0x60EC, false)), // East Asian ideograph
    (0x213E6D, (0x60F3, false)), // East Asian ideograph
    (0x223E6E, (0x6939, false)), // East Asian ideograph
    (0x233E6F, (0x90D5, false)), // East Asian ideograph
    (0x273E70, (0x607B, false)), // East Asian ideograph
    (0x214A68, (0x72C0, false)), // East Asian ideograph
    (0x233E72, (0x90BE, false)), // East Asian ideograph
    (0x223E73, (0x6937, false)), // East Asian ideograph
    (0x213E74, (0x60F9, false)), // East Asian ideograph
    (0x213E75, (0x6123, false)), // East Asian ideograph
    (0x213E76, (0x60F4, false)), // East Asian ideograph
    (0x273E77, (0x7231, false)), // East Asian ideograph
    (0x233E78, (0x90C8, false)), // East Asian ideograph
    (0x233E79, (0x90C3, false)), // East Asian ideograph
    (0x223E7A, (0x696C, false)), // East Asian ideograph
    (0x223E7B, (0x694E, false)), // East Asian ideograph
    (0x213E7C, (0x6109, false)), // East Asian ideograph
    (0x224A6A, (0x6E51, false)), // East Asian ideograph
    (0x273E7E, (0x607C, false)), // East Asian ideograph
    (0x234137, (0x91A8, false)), // East Asian ideograph
    (0x224A6B, (0x6E44, false)), // East Asian ideograph
    (0x275576, (0x8361, false)), // East Asian ideograph
    (0x27734C, (0x5456, false)), // East Asian ideograph
    (0x21385B, (0x5879, false)), // East Asian ideograph
    (0x293B5B, (0x8F81, false)), // East Asian ideograph
    (0x234A6D, (0x9682, false)), // East Asian ideograph
    (0x234A6E, (0x9683, false)), // East Asian ideograph
    (0x6F5355, (0xC1FC, false)), // Korean hangul
    (0x335238, (0x8989, false)), // East Asian ideograph
    (0x694C68, (0x5301, false)), // East Asian ideograph
    (0x21393A, (0x5958, false)), // East Asian ideograph
    (0x2D5C5A, (0x8FE9, false)), // East Asian ideograph
    (0x2D3A41, (0x5AFA, false)), // East Asian ideograph
    (0x214A70, (0x72F9, false)), // East Asian ideograph
    (0x6F4A71, (0xAF48, false)), // Korean hangul
    (0x213F2D, (0x6177, false)), // East Asian ideograph
    (0x295932, (0x9CD8, false)), // East Asian ideograph
    (0x274A72, (0x72C8, false)), // East Asian ideograph
    (0x23302C, (0x895C, false)), // East Asian ideograph
    (0x276239, (0x9E2F, false)), // East Asian ideograph
    (0x6F4A73, (0xAF4C, false)), // Korean hangul
    (0x6F5356, (0xC1FD, false)), // Korean hangul
    (0x276241, (0x9E45, false)), // East Asian ideograph
    (0x21393B, (0x595A, false)), // East Asian ideograph
    (0x4B324E, (0x50E7, false)), // East Asian ideograph (variant of 21324E which maps to 50E7)
    (0x2E4E72, (0x6F74, false)), // East Asian ideograph
    (0x6F5774, (0xC911, false)), // Korean hangul
    (0x6F4A75, (0xAF5C, false)), // Korean hangul
    (0x6F4A76, (0xAF5D, false)), // Korean hangul
    (0x213521, (0x53C9, false)), // East Asian ideograph
    (0x224A77, (0x6E4E, false)), // East Asian ideograph
    (0x23302D, (0x895D, false)), // East Asian ideograph
    (0x4B4A78, (0x72F0, false)), // East Asian ideograph
    (0x6F5357, (0xC200, false)), // Korean hangul
    (0x213523, (0x53CA, false)), // East Asian ideograph
    (0x276242, (0x9E51, false)), // East Asian ideograph
    (0x275D49, (0x94AE, false)), // East Asian ideograph
    (0x274A79, (0x72B9, false)), // East Asian ideograph
    (0x223D3B, (0x6874, false)), // East Asian ideograph
    (0x234A7A, (0x9697, false)), // East Asian ideograph
    (0x224D68, (0x6F5D, false)), // East Asian ideograph
    (0x6F4A7B, (0xAF84, false)), // Korean hangul
    (0x213526, (0x53D4, false)), // East Asian ideograph
    (0x227061, (0x7D3A, false)), // East Asian ideograph
    (0x6F4A7C, (0xAF88, false)), // Korean hangul
    (0x225A30, (0x7415, false)), // East Asian ideograph
    (0x213527, (0x53D7, false)), // East Asian ideograph
    (0x227C49, (0x82BC, false)), // East Asian ideograph
    (0x6F4A7D, (0xAF90, false)), // Korean hangul
    (0x6F5358, (0xC204, false)), // Korean hangul
    (0x6F7623, (0x317F, false)), // Korean hangul
    (0x274A7E, (0x72F1, false)), // East Asian ideograph
    (0x223D3C, (0x6875, false)), // East Asian ideograph
    (0x227D2B, (0x8344, false)), // East Asian ideograph
    (0x21352A, (0x66FC, false)), // East Asian ideograph
    (0x213936, (0x594E, false)), // East Asian ideograph
    (0x21352B, (0x53E2, false)), // East Asian ideograph
    (0x2D4B5B, (0x78AF, false)), // East Asian ideograph
    (0x6F5359, (0xC20D, false)), // Korean hangul
    (0x23352D, (0x8B95, false)), // East Asian ideograph
    (0x276244, (0x9E4C, false)), // East Asian ideograph
    (0x23352E, (0x8B94, false)), // East Asian ideograph
    (0x3A7970, (0x81D5, false)), // East Asian ideograph
    (0x21352F, (0x53EE, false)), // East Asian ideograph
    (0x6F5331, (0xC138, false)), // Korean hangul
    (0x275138, (0x7EDC, false)), // East Asian ideograph
    (0x223F21, (0x6952, false)), // East Asian ideograph
    (0x213F22, (0x6168, false)), // East Asian ideograph
    (0x233F23, (0x90DF, false)), // East Asian ideograph
    (0x213F24, (0x613C, false)), // East Asian ideograph
    (0x223F25, (0x695B, false)), // East Asian ideograph
    (0x233F26, (0x90E2, false)), // East Asian ideograph
    (0x223531, (0x64EB, false)), // East Asian ideograph
    (0x233F28, (0x90DB, false)), // East Asian ideograph
    (0x273F29, (0x5FFE, false)), // East Asian ideograph
    (0x233F2A, (0x90DC, false)), // East Asian ideograph
    (0x273F2B, (0x6006, false)), // East Asian ideograph
    (0x233F2C, (0x90D7, false)), // East Asian ideograph
    (0x233F2D, (0x90E4, false)), // East Asian ideograph
    (0x233F2E, (0x90EF, false)), // East Asian ideograph
    (0x233F2F, (0x90EA, false)), // East Asian ideograph
    (0x213F30, (0x6170, false)), // East Asian ideograph
    (0x213F31, (0x615A, false)), // East Asian ideograph
    (0x233F32, (0x90F0, false)), // East Asian ideograph
    (0x233F33, (0x90F4, false)), // East Asian ideograph
    (0x233F34, (0x90F2, false)), // East Asian ideograph
    (0x223F35, (0x6978, false)), // East Asian ideograph
    (0x273F36, (0x8651, false)), // East Asian ideograph
    (0x223F37, (0x697B, false)), // East Asian ideograph
    (0x273F38, (0x60E8, false)), // East Asian ideograph
    (0x273F39, (0x60EF, false)), // East Asian ideograph
    (0x273F3A, (0x6078, false)), // East Asian ideograph
    (0x273F3B, (0x6002, false)), // East Asian ideograph
    (0x273F3C, (0x6B32, false)), // East Asian ideograph
    (0x223F3D, (0x6944, false)), // East Asian ideograph
    (0x233F3E, (0x90EB, false)), // East Asian ideograph
    (0x233F3F, (0x90F3, false)), // East Asian ideograph
    (0x213F40, (0x618E, false)), // East Asian ideograph
    (0x273F41, (0x60AF, false)), // East Asian ideograph
    (0x273F42, (0x6124, false)), // East Asian ideograph
    (0x213F43, (0x61AC, false)), // East Asian ideograph
    (0x273F44, (0x60EE, false)), // East Asian ideograph
    (0x273F45, (0x6187, false)), // East Asian ideograph
    (0x233F46, (0x90FC, false)), // East Asian ideograph
    (0x273F47, (0x60EB, false)), // East Asian ideograph
    (0x273F48, (0x5FC6, false)), // East Asian ideograph
    (0x233F49, (0x9104, false)), // East Asian ideograph
    (0x273F4A, (0x5E94, false)), // East Asian ideograph
    (0x213537, (0x53EB, false)), // East Asian ideograph
    (0x233F4C, (0x9106, false)), // East Asian ideograph
    (0x213F4D, (0x61C2, false)), // East Asian ideograph
    (0x273F4E, (0x6073, false)), // East Asian ideograph
    (0x213F4F, (0x61C8, false)), // East Asian ideograph
    (0x213940, (0x596A, false)), // East Asian ideograph
    (0x232368, (0x84CD, false)), // East Asian ideograph
    (0x213F52, (0x61E6, false)), // East Asian ideograph
    (0x213F53, (0x61F2, false)), // East Asian ideograph (variant of 4B3F53 which maps to 61F2)
    (0x273F54, (0x6000, false)), // East Asian ideograph
    (0x273F55, (0x61D2, false)), // East Asian ideograph
    (0x273F56, (0x60AC, false)), // East Asian ideograph
    (0x233F57, (0x910F, false)), // East Asian ideograph
    (0x273F58, (0x5FCF, false)), // East Asian ideograph
    (0x273F59, (0x6151, false)), // East Asian ideograph
    (0x233F5A, (0x9116, false)), // East Asian ideograph
    (0x273F5B, (0x60E7, false)), // East Asian ideograph
    (0x233F5C, (0x9114, false)), // East Asian ideograph
    (0x23353A, (0x8B9F, false)), // East Asian ideograph
    (0x213F5E, (0x620A, false)), // East Asian ideograph
    (0x213F5F, (0x620E, false)), // East Asian ideograph
    (0x223F60, (0x69BC, false)), // East Asian ideograph
    (0x223F61, (0x69A7, false)), // East Asian ideograph
    (0x233F62, (0x9123, false)), // East Asian ideograph (Version J extension)
    (0x233F63, (0x9118, false)), // East Asian ideograph
    (0x233F64, (0x911C, false)), // East Asian ideograph
    (0x213F65, (0x6216, false)), // East Asian ideograph
    (0x233F66, (0x9120, false)), // East Asian ideograph
    (0x233F67, (0x9122, false)), // East Asian ideograph
    (0x223F68, (0x69D9, false)), // East Asian ideograph
    (0x213F69, (0x621F, false)), // East Asian ideograph
    (0x223F6A, (0x698E, false)), // East Asian ideograph
    (0x213F6B, (0x6222, false)), // East Asian ideograph
    (0x213F6C, (0x622A, false)), // East Asian ideograph
    (0x223F6D, (0x69D6, false)), // East Asian ideograph
    (0x273F6E, (0x6218, false)), // East Asian ideograph
    (0x273F6F, (0x620F, false)), // East Asian ideograph
    (0x213F70, (0x6234, false)), // East Asian ideograph
    (0x233F71, (0x9124, false)), // East Asian ideograph
    (0x233F72, (0x911A, false)), // East Asian ideograph
    (0x213F73, (0x623F, false)), // East Asian ideograph
    (0x233F74, (0x9125, false)), // East Asian ideograph
    (0x223F75, (0x69A5, false)), // East Asian ideograph
    (0x213F76, (0x6241, false)), // East Asian ideograph
    (0x233F77, (0x912F, false)), // East Asian ideograph
    (0x223F78, (0x69D1, false)), // East Asian ideograph
    (0x27513B, (0x4E1D, false)), // East Asian ideograph
    (0x223F7A, (0x69F6, false)), // East Asian ideograph
    (0x21753F, (0x579E, false)), // East Asian ideograph
    (0x213F7D, (0x6253, false)), // East Asian ideograph
    (0x223F7E, (0x69D5, false)), // East Asian ideograph
    (0x217540, (0x57B5, false)), // East Asian ideograph
    (0x6F535D, (0xC21C, false)), // Korean hangul
    (0x276248, (0x9E5E, false)), // East Asian ideograph
    (0x223542, (0x64F7, false)), // East Asian ideograph
    (0x213543, (0x540C, false)), // East Asian ideograph
    (0x213544, (0x540A, false)), // East Asian ideograph
    (0x29593A, (0x9C85, false)), // East Asian ideograph
    (0x23524D, (0x99BF, false)), // East Asian ideograph
    (0x213545, (0x540D, false)), // East Asian ideograph
    (0x6F535E, (0xC21F, false)), // Korean hangul
    (0x223546, (0x6504, false)), // East Asian ideograph
    (0x234E5E, (0x97E0, false)), // East Asian ideograph
    (0x2D3547, (0x55AB, false)), // East Asian ideograph
    (0x6F5C66, (0xD560, false)), // Korean hangul
    (0x234141, (0x91AF, false)), // East Asian ideograph
    (0x692436, (0x3056, false)), // Hiragana letter ZA
    (0x696868, (0x84D9, false)), // East Asian ideograph
    (0x233478, (0x8B85, false)), // East Asian ideograph
    (0x4C354A, (0x64B8, false)), // East Asian ideograph
    (0x22587D, (0x73CC, false)), // East Asian ideograph
    (0x22354B, (0x64FD, false)), // East Asian ideograph
    (0x333021, (0x58F9, false)), // East Asian ideograph
    (0x225F5C, (0x7633, false)), // East Asian ideograph
    (0x217933, (0x5940, false)), // East Asian ideograph
    (0x234142, (0x91B1, false)), // East Asian ideograph
    (0x23346B, (0x8B6D, false)), // East Asian ideograph
    (0x23354D, (0x8C4B, false)), // East Asian ideograph
    (0x2D7A44, (0x598D, false)), // East Asian ideograph
    (0x27513E, (0x7EE2, false)), // East Asian ideograph
    (0x23472C, (0x93D3, false)), // East Asian ideograph
    (0x22354F, (0x6508, false)), // East Asian ideograph
    (0x395E42, (0x9274, false)), // East Asian ideograph
    (0x233550, (0x8C4F, false)), // East Asian ideograph
    (0x3F5F35, (0x6B9E, false)), // East Asian ideograph
    (0x39303A, (0x5EFC, false)), // East Asian ideograph
    (0x213552, (0x543E, false)), // East Asian ideograph
    (0x27513F, (0x7EE5, false)), // East Asian ideograph
    (0x213553, (0x5427, false)), // East Asian ideograph
    (0x213554, (0x5440, false)), // East Asian ideograph
    (0x274476, (0x6808, false)), // East Asian ideograph
    (0x233555, (0x8C5C, false)), // East Asian ideograph
    (0x6F5A26, (0xCEE8, false)), // Korean hangul
    (0x213556, (0x5446, false)), // East Asian ideograph
    (0x217557, (0x57A1, false)), // East Asian ideograph
    (0x213266, (0x512A, false)), // East Asian ideograph
    (0x293C30, (0x8F98, false)), // East Asian ideograph
    (0x224B38, (0x6E69, false)), // East Asian ideograph
    (0x6F5332, (0xC139, false)), // Korean hangul
    (0x293725, (0x8D3D, false)), // East Asian ideograph
    (0x287229, (0x7F17, false)), // East Asian ideograph
    (0x223559, (0x651A, false)), // East Asian ideograph
    (0x6F5362, (0xC22B, false)), // Korean hangul
    (0x27624D, (0x9E6D, false)), // East Asian ideograph
    (0x214021, (0x6252, false)), // East Asian ideograph
    (0x214022, (0x625B, false)), // East Asian ideograph
    (0x214023, (0x6263, false)), // East Asian ideograph
    (0x214024, (0x6258, false)), // East Asian ideograph
    (0x214025, (0x6296, false)), // East Asian ideograph
    (0x214026, (0x6297, false)), // East Asian ideograph
    (0x214027, (0x6292, false)), // East Asian ideograph
    (0x214028, (0x6276, false)), // East Asian ideograph
    (0x214029, (0x6289, false)), // East Asian ideograph
    (0x21402A, (0x627F, false)), // East Asian ideograph
    (0x21402B, (0x6279, false)), // East Asian ideograph
    (0x21402C, (0x6280, false)), // East Asian ideograph
    (0x21402D, (0x628A, false)), // East Asian ideograph
    (0x21402E, (0x626D, false)), // East Asian ideograph
    (0x21402F, (0x627C, false)), // East Asian ideograph
    (0x214030, (0x627E, false)), // East Asian ideograph
    (0x214031, (0x626F, false)), // East Asian ideograph
    (0x214032, (0x6284, false)), // East Asian ideograph
    (0x214033, (0x6295, false)), // East Asian ideograph
    (0x214034, (0x6291, false)), // East Asian ideograph
    (0x214035, (0x6298, false)), // East Asian ideograph
    (0x214036, (0x626E, false)), // East Asian ideograph
    (0x214037, (0x6273, false)), // East Asian ideograph
    (0x214038, (0x6293, false)), // East Asian ideograph
    (0x214039, (0x62C9, false)), // East Asian ideograph
    (0x21403A, (0x62C4, false)), // East Asian ideograph
    (0x21403B, (0x62CC, false)), // East Asian ideograph
    (0x21403C, (0x62A8, false)), // East Asian ideograph
    (0x21403D, (0x62DC, false)), // East Asian ideograph
    (0x21403E, (0x62BF, false)), // East Asian ideograph
    (0x21403F, (0x62C2, false)), // East Asian ideograph
    (0x214040, (0x62B9, false)), // East Asian ideograph
    (0x214041, (0x62D2, false)), // East Asian ideograph
    (0x214042, (0x62D3, false)), // East Asian ideograph
    (0x214043, (0x62DB, false)), // East Asian ideograph
    (0x214044, (0x62AB, false)), // East Asian ideograph
    (0x214045, (0x62CB, false)), // East Asian ideograph
    (0x214046, (0x62D4, false)), // East Asian ideograph
    (0x214047, (0x62BD, false)), // East Asian ideograph
    (0x214048, (0x62BC, false)), // East Asian ideograph
    (0x214049, (0x62D0, false)), // East Asian ideograph (variant of 4B4049 which maps to 62D0)
    (0x21404A, (0x62C8, false)), // East Asian ideograph
    (0x21404B, (0x62D9, false)), // East Asian ideograph
    (0x21404C, (0x62DA, false)), // East Asian ideograph
    (0x21404D, (0x62AC, false)), // East Asian ideograph
    (0x21404E, (0x62C7, false)), // East Asian ideograph
    (0x21404F, (0x62B1, false)), // East Asian ideograph
    (0x214050, (0x62D6, false)), // East Asian ideograph
    (0x214051, (0x62D8, false)), // East Asian ideograph
    (0x214052, (0x62CD, false)), // East Asian ideograph
    (0x214053, (0x62B5, false)), // East Asian ideograph
    (0x214054, (0x62CE, false)), // East Asian ideograph
    (0x214055, (0x62D7, false)), // East Asian ideograph
    (0x214056, (0x62C6, false)), // East Asian ideograph
    (0x214057, (0x6309, false)), // East Asian ideograph
    (0x214058, (0x6316, false)), // East Asian ideograph
    (0x214059, (0x62FC, false)), // East Asian ideograph
    (0x21405A, (0x62F3, false)), // East Asian ideograph
    (0x21405B, (0x6308, false)), // East Asian ideograph
    (0x21405C, (0x62ED, false)), // East Asian ideograph
    (0x21405D, (0x6301, false)), // East Asian ideograph
    (0x21405E, (0x62EE, false)), // East Asian ideograph
    (0x21405F, (0x62EF, false)), // East Asian ideograph
    (0x214060, (0x62F7, false)), // East Asian ideograph
    (0x214061, (0x6307, false)), // East Asian ideograph
    (0x214062, (0x62F1, false)), // East Asian ideograph
    (0x214063, (0x62FD, false)), // East Asian ideograph
    (0x214064, (0x6311, false)), // East Asian ideograph
    (0x214065, (0x62EC, false)), // East Asian ideograph
    (0x214066, (0x62F4, false)), // East Asian ideograph (variant of 4B4066 which maps to 62F4)
    (0x214067, (0x62FF, false)), // East Asian ideograph
    (0x224068, (0x6A2D, false)), // East Asian ideograph
    (0x214069, (0x6342, false)), // East Asian ideograph
    (0x21406A, (0x632A, false)), // East Asian ideograph
    (0x21406B, (0x6355, false)), // East Asian ideograph
    (0x21406C, (0x633E, false)), // East Asian ideograph
    (0x21406D, (0x632F, false)), // East Asian ideograph
    (0x21406E, (0x634E, false)), // East Asian ideograph
    (0x21406F, (0x634F, false)), // East Asian ideograph
    (0x214070, (0x6350, false)), // East Asian ideograph
    (0x214071, (0x6349, false)), // East Asian ideograph
    (0x224072, (0x6A1D, false)), // East Asian ideograph
    (0x214073, (0x632B, false)), // East Asian ideograph
    (0x214074, (0x6328, false)), // East Asian ideograph
    (0x214075, (0x633A, false)), // East Asian ideograph
    (0x214076, (0x63A5, false)), // East Asian ideograph
    (0x214077, (0x6369, false)), // East Asian ideograph
    (0x214078, (0x63A0, false)), // East Asian ideograph
    (0x214079, (0x6396, false)), // East Asian ideograph
    (0x21407A, (0x63A7, false)), // East Asian ideograph
    (0x21407B, (0x6372, false)), // East Asian ideograph
    (0x21407C, (0x6377, false)), // East Asian ideograph
    (0x21407D, (0x6383, false)), // East Asian ideograph
    (0x21407E, (0x636B, false)), // East Asian ideograph
    (0x2D5C74, (0x96A3, false)), // East Asian ideograph
    (0x2D5831, (0x89A7, false)), // East Asian ideograph
    (0x21756C, (0x57BE, false)), // East Asian ideograph
    (0x693729, (0x7C82, false)), // East Asian ideograph
    (0x23356D, (0x8C73, false)), // East Asian ideograph
    (0x6F5966, (0xCE5C, false)), // Korean hangul
    (0x29252D, (0x8311, false)), // East Asian ideograph
    (0x6F5366, (0xC232, false)), // Korean hangul
    (0x276251, (0x76D0, false)), // East Asian ideograph
    (0x6F5463, (0xC46C, false)), // Korean hangul
    (0x6F4F23, (0xB800, false)), // Korean hangul
    (0x21356F, (0x547B, false)), // East Asian ideograph
    (0x6F4C71, (0xB2EB, false)), // Korean hangul
    (0x233571, (0x8C75, false)), // East Asian ideograph
    (0x28722A, (0x7F02, false)), // East Asian ideograph
    (0x213572, (0x5484, false)), // East Asian ideograph
    (0x6F4A54, (0xAE7B, false)), // Korean hangul
    (0x39593F, (0x8A3C, false)), // East Asian ideograph
    (0x233573, (0x8C77, false)), // East Asian ideograph
    (0x276252, (0x7877, false)), // East Asian ideograph
    (0x213D7B, (0x5FD8, false)), // East Asian ideograph
    (0x6F4F24, (0xB801, false)), // Korean hangul
    (0x213574, (0x5468, false)), // East Asian ideograph
    (0x223D4B, (0x68B4, false)), // East Asian ideograph
    (0x692568, (0x30E8, false)), // Katakana letter YO
    (0x213575, (0x5486, false)), // East Asian ideograph
    (0x213939, (0x5957, false)), // East Asian ideograph
    (0x6F5A2F, (0xCF01, false)), // Korean hangul
    (0x393B6E, (0x5C97, false)), // East Asian ideograph
    (0x2D6021, (0x978C, false)), // East Asian ideograph
    (0x335652, (0x87C1, false)), // East Asian ideograph
    (0x223577, (0x652E, false)), // East Asian ideograph
    (0x216022, (0x978B, false)), // East Asian ideograph
    (0x216023, (0x978F, false)), // East Asian ideograph
    (0x215B66, (0x8FC6, false)), // East Asian ideograph
    (0x694838, (0x567A, false)), // East Asian ideograph
    (0x216024, (0x9798, false)), // East Asian ideograph
    (0x4B5036, (0x7C14, false)), // East Asian ideograph
    (0x277360, (0x5181, false)), // East Asian ideograph
    (0x21357B, (0x5471, false)), // East Asian ideograph
    (0x21357C, (0x549A, false)), // East Asian ideograph
    (0x454E43, (0x788C, false)), // East Asian ideograph (variant of 214E43 which maps to 788C)
    (0x21357D, (0x548E, false)), // East Asian ideograph
    (0x216028, (0x97AD, false)), // East Asian ideograph
    (0x6F4F26, (0xB808, false)), // Korean hangul
    (0x215724, (0x87A2, false)), // East Asian ideograph
    (0x275E7B, (0x9635, false)), // East Asian ideograph
    (0x6F5D6E, (0xD758, false)), // Korean hangul
    (0x21602B, (0x97C6, false)), // East Asian ideograph
    (0x335259, (0x7E59, false)), // East Asian ideograph
    (0x27602C, (0x97E6, false)), // East Asian ideograph
    (0x27623D, (0x9E3D, false)), // East Asian ideograph
    (0x4B4347, (0x66A8, false)), // East Asian ideograph
    (0x6F536A, (0xC26C, false)), // Korean hangul
    (0x27602D, (0x97E7, false)), // East Asian ideograph
    (0x6F4F27, (0xB809, false)), // Korean hangul
    (0x6F592B, (0xCC3E, false)), // Korean hangul
    (0x213938, (0x5950, false)), // East Asian ideograph
    (0x2D3A60, (0x6588, false)), // East Asian ideograph
    (0x27602F, (0x97EC, false)), // East Asian ideograph
    (0x214121, (0x6367, false)), // East Asian ideograph
    (0x214122, (0x6398, false)), // East Asian ideograph
    (0x214123, (0x639B, false)), // East Asian ideograph
    (0x214124, (0x63AA, false)), // East Asian ideograph
    (0x214125, (0x6371, false)), // East Asian ideograph
    (0x214126, (0x63A9, false)), // East Asian ideograph
    (0x214127, (0x638C, false)), // East Asian ideograph
    (0x214128, (0x6389, false)), // East Asian ideograph
    (0x214129, (0x63A2, false)), // East Asian ideograph
    (0x21412A, (0x6399, false)), // East Asian ideograph
    (0x21412B, (0x63A1, false)), // East Asian ideograph
    (0x21412C, (0x6388, false)), // East Asian ideograph
    (0x21412D, (0x63AC, false)), // East Asian ideograph
    (0x21412E, (0x633D, false)), // East Asian ideograph
    (0x21412F, (0x6392, false)), // East Asian ideograph
    (0x214130, (0x63A3, false)), // East Asian ideograph
    (0x214131, (0x6376, false)), // East Asian ideograph
    (0x214132, (0x638F, false)), // East Asian ideograph
    (0x214133, (0x63A8, false)), // East Asian ideograph
    (0x214134, (0x637B, false)), // East Asian ideograph
    (0x214135, (0x6368, false)), // East Asian ideograph (variant of 4B4135 which maps to 6368)
    (0x214136, (0x6384, false)), // East Asian ideograph
    (0x214137, (0x6380, false)), // East Asian ideograph
    (0x214138, (0x63C6, false)), // East Asian ideograph
    (0x214139, (0x63C9, false)), // East Asian ideograph
    (0x21413A, (0x63CD, false)), // East Asian ideograph
    (0x21413B, (0x63E1, false)), // East Asian ideograph
    (0x21413C, (0x63C0, false)), // East Asian ideograph
    (0x21413D, (0x63E9, false)), // East Asian ideograph
    (0x21413E, (0x63D0, false)), // East Asian ideograph
    (0x21413F, (0x63DA, false)), // East Asian ideograph
    (0x214140, (0x63D6, false)), // East Asian ideograph
    (0x214141, (0x63ED, false)), // East Asian ideograph
    (0x214142, (0x63EE, false)), // East Asian ideograph
    (0x214143, (0x63CF, false)), // East Asian ideograph
    (0x214144, (0x63E3, false)), // East Asian ideograph
    (0x214145, (0x63F4, false)), // East Asian ideograph
    (0x214146, (0x63DB, false)), // East Asian ideograph (variant of 454146 which maps to 63DB)
    (0x214147, (0x63D2, false)), // East Asian ideograph
    (0x234148, (0x91AE, false)), // East Asian ideograph
    (0x214149, (0x641E, false)), // East Asian ideograph
    (0x21414A, (0x642A, false)), // East Asian ideograph
    (0x23414B, (0x91B4, false)), // East Asian ideograph
    (0x23414C, (0x91B2, false)), // East Asian ideograph
    (0x21414D, (0x640F, false)), // East Asian ideograph
    (0x21414E, (0x6414, false)), // East Asian ideograph
    (0x21414F, (0x640D, false)), // East Asian ideograph
    (0x214150, (0x642D, false)), // East Asian ideograph
    (0x214151, (0x643D, false)), // East Asian ideograph
    (0x214152, (0x6416, false)), // East Asian ideograph
    (0x214153, (0x6417, false)), // East Asian ideograph
    (0x214154, (0x641C, false)), // East Asian ideograph
    (0x214155, (0x6436, false)), // East Asian ideograph
    (0x214156, (0x642C, false)), // East Asian ideograph
    (0x214157, (0x6458, false)), // East Asian ideograph
    (0x214158, (0x6469, false)), // East Asian ideograph
    (0x214159, (0x6454, false)), // East Asian ideograph
    (0x21415A, (0x6452, false)), // East Asian ideograph
    (0x21415B, (0x646F, false)), // East Asian ideograph
    (0x21415C, (0x6478, false)), // East Asian ideograph
    (0x21415D, (0x6479, false)), // East Asian ideograph
    (0x21415E, (0x647A, false)), // East Asian ideograph
    (0x21415F, (0x645F, false)), // East Asian ideograph
    (0x214160, (0x6451, false)), // East Asian ideograph
    (0x214161, (0x6467, false)), // East Asian ideograph
    (0x214162, (0x649E, false)), // East Asian ideograph
    (0x214163, (0x64A4, false)), // East Asian ideograph
    (0x214164, (0x6487, false)), // East Asian ideograph
    (0x214165, (0x6488, false)), // East Asian ideograph
    (0x214166, (0x64A5, false)), // East Asian ideograph
    (0x214167, (0x64B0, false)), // East Asian ideograph
    (0x214168, (0x6493, false)), // East Asian ideograph
    (0x214169, (0x6495, false)), // East Asian ideograph
    (0x21416A, (0x6492, false)), // East Asian ideograph
    (0x21416B, (0x64A9, false)), // East Asian ideograph
    (0x21416C, (0x6491, false)), // East Asian ideograph
    (0x21416D, (0x64AE, false)), // East Asian ideograph
    (0x21416E, (0x64B2, false)), // East Asian ideograph
    (0x21416F, (0x64AD, false)), // East Asian ideograph
    (0x214170, (0x649A, false)), // East Asian ideograph
    (0x214171, (0x64AB, false)), // East Asian ideograph
    (0x214172, (0x64AC, false)), // East Asian ideograph
    (0x214173, (0x64C5, false)), // East Asian ideograph
    (0x214174, (0x64C1, false)), // East Asian ideograph
    (0x214175, (0x64D8, false)), // East Asian ideograph
    (0x214176, (0x64CA, false)), // East Asian ideograph
    (0x214177, (0x64BB, false)), // East Asian ideograph
    (0x214178, (0x64C2, false)), // East Asian ideograph
    (0x214179, (0x64BC, false)), // East Asian ideograph
    (0x21417A, (0x64CB, false)), // East Asian ideograph
    (0x21417B, (0x64CD, false)), // East Asian ideograph
    (0x21417C, (0x64DA, false)), // East Asian ideograph
    (0x21417D, (0x64C4, false)), // East Asian ideograph
    (0x21417E, (0x64C7, false)), // East Asian ideograph
    (0x705C50, (0x82C4, false)), // East Asian ideograph
    (0x216040, (0x9813, false)), // East Asian ideograph
    (0x393E61, (0x60AA, false)), // East Asian ideograph
    (0x6F536E, (0xC27D, false)), // Korean hangul
    (0x216041, (0x9812, false)), // East Asian ideograph
    (0x2D3571, (0x546A, false)), // East Asian ideograph
    (0x6F4F2B, (0xB819, false)), // Korean hangul
    (0x29483E, (0x9554, false)), // East Asian ideograph
    (0x276042, (0x9882, false)), // East Asian ideograph
    (0x276043, (0x9887, false)), // East Asian ideograph
    (0x6F5D6F, (0xD759, false)), // Korean hangul
    (0x276044, (0x9886, false)), // East Asian ideograph
    (0x69594B, (0x6327, false)), // East Asian ideograph
    (0x276045, (0x9889, false)), // East Asian ideograph
    (0x27623E, (0x9E49, false)), // East Asian ideograph
    (0x6F536F, (0xC27F, false)), // Korean hangul
    (0x276046, (0x5934, false)), // East Asian ideograph
    (0x6F4F2C, (0xB81B, false)), // Korean hangul
    (0x213954, (0x5999, false)), // East Asian ideograph
    (0x29483F, (0x9572, false)), // East Asian ideograph
    (0x236047, (0x9F76, false)), // East Asian ideograph
    (0x6F5775, (0xC918, false)), // Korean hangul
    (0x216048, (0x9838, false)), // East Asian ideograph
    (0x6F503A, (0xBAA8, false)), // Korean hangul
    (0x216049, (0x983B, false)), // East Asian ideograph
    (0x213E58, (0x60E6, false)), // East Asian ideograph
    (0x222C47, (0x60D3, false)), // East Asian ideograph
    (0x21604A, (0x9839, false)), // East Asian ideograph
    (0x6F5370, (0xC281, false)), // Korean hangul
    (0x27625B, (0x9EA6, false)), // East Asian ideograph
    (0x27604B, (0x9894, false)), // East Asian ideograph
    (0x6F4F2D, (0xB81D, false)), // Korean hangul
    (0x27604C, (0x9890, false)), // East Asian ideograph
    (0x225B2A, (0x748A, false)), // East Asian ideograph
    (0x27604D, (0x9897, false)), // East Asian ideograph
    (0x213269, (0x5132, false)), // East Asian ideograph
    (0x4C725D, (0x7A39, false)), // East Asian ideograph
    (0x27604E, (0x989C, false)), // East Asian ideograph
    (0x27604F, (0x989D, false)), // East Asian ideograph
    (0x2D4730, (0x51B5, false)), // East Asian ideograph
    (0x27625C, (0x9EB8, false)), // East Asian ideograph
    (0x276050, (0x9898, false)), // East Asian ideograph
    (0x276051, (0x989A, false)), // East Asian ideograph
    (0x216052, (0x9853, false)), // East Asian ideograph
    (0x393B78, (0x5CC4, false)), // East Asian ideograph (duplicate simplified)
    (0x276053, (0x7C7B, false)), // East Asian ideograph
    (0x284F39, (0x6CF8, false)), // East Asian ideograph
    (0x276054, (0x98A0, false)), // East Asian ideograph
    (0x6F5372, (0xC289, false)), // Korean hangul
    (0x216055, (0x9858, false)), // East Asian ideograph
    (0x6F4F2F, (0xB825, false)), // Korean hangul
    (0x4B4866, (0x6E89, false)), // East Asian ideograph
    (0x276056, (0x987E, false)), // East Asian ideograph
    (0x69243A, (0x305A, false)), // Hiragana letter ZU
    (0x276057, (0x98A4, false)), // East Asian ideograph
    (0x276058, (0x663E, false)), // East Asian ideograph
    (0x213861, (0x589C, false)), // East Asian ideograph
    (0x4B614D, (0x9A13, false)), // East Asian ideograph
    (0x216059, (0x9871, false)), // East Asian ideograph
    (0x4C4333, (0x6AAA, false)), // East Asian ideograph
    (0x27625E, (0x9762, false)), // East Asian ideograph
    (0x27605A, (0x98A6, false)), // East Asian ideograph
    (0x6F4F30, (0xB828, false)), // Korean hangul
    (0x214221, (0x64CE, false)), // East Asian ideograph
    (0x214222, (0x64D4, false)), // East Asian ideograph
    (0x214223, (0x64D2, false)), // East Asian ideograph
    (0x214224, (0x64BF, false)), // East Asian ideograph
    (0x234225, (0x9201, false)), // East Asian ideograph
    (0x214226, (0x64F0, false)), // East Asian ideograph
    (0x214227, (0x64E6, false)), // East Asian ideograph
    (0x214228, (0x64EC, false)), // East Asian ideograph
    (0x214229, (0x64F1, false)), // East Asian ideograph
    (0x21422A, (0x64F4, false)), // East Asian ideograph
    (0x21422B, (0x64F2, false)), // East Asian ideograph
    (0x21422C, (0x6506, false)), // East Asian ideograph
    (0x21422D, (0x6500, false)), // East Asian ideograph
    (0x27422E, (0x6270, false)), // East Asian ideograph
    (0x21422F, (0x64FB, false)), // East Asian ideograph
    (0x214230, (0x64FA, false)), // East Asian ideograph
    (0x214231, (0x650F, false)), // East Asian ideograph
    (0x214232, (0x6518, false)), // East Asian ideograph
    (0x214233, (0x6514, false)), // East Asian ideograph
    (0x214234, (0x6519, false)), // East Asian ideograph
    (0x214235, (0x651D, false)), // East Asian ideograph
    (0x214236, (0x651C, false)), // East Asian ideograph
    (0x214237, (0x6523, false)), // East Asian ideograph
    (0x214238, (0x6524, false)), // East Asian ideograph
    (0x214239, (0x652B, false)), // East Asian ideograph
    (0x21423A, (0x652A, false)), // East Asian ideograph
    (0x21423B, (0x652C, false)), // East Asian ideograph
    (0x21423C, (0x652F, false)), // East Asian ideograph
    (0x21423D, (0x6536, false)), // East Asian ideograph
    (0x21423E, (0x6539, false)), // East Asian ideograph
    (0x21423F, (0x653B, false)), // East Asian ideograph
    (0x214240, (0x653E, false)), // East Asian ideograph
    (0x214241, (0x653F, false)), // East Asian ideograph
    (0x214242, (0x6545, false)), // East Asian ideograph
    (0x214243, (0x6548, false)), // East Asian ideograph
    (0x214244, (0x654E, false)), // East Asian ideograph
    (0x214245, (0x6556, false)), // East Asian ideograph
    (0x214246, (0x6551, false)), // East Asian ideograph
    (0x274247, (0x8D25, false)), // East Asian ideograph
    (0x214248, (0x655D, false)), // East Asian ideograph
    (0x214249, (0x6558, false)), // East Asian ideograph
    (0x21424A, (0x654F, false)), // East Asian ideograph
    (0x21424B, (0x6566, false)), // East Asian ideograph
    (0x21424C, (0x6562, false)), // East Asian ideograph
    (0x21424D, (0x6563, false)), // East Asian ideograph
    (0x21424E, (0x655E, false)), // East Asian ideograph
    (0x21424F, (0x5553, false)), // East Asian ideograph
    (0x214250, (0x656C, false)), // East Asian ideograph
    (0x214251, (0x6572, false)), // East Asian ideograph
    (0x214252, (0x6575, false)), // East Asian ideograph
    (0x214253, (0x6577, false)), // East Asian ideograph
    (0x214254, (0x6578, false)), // East Asian ideograph
    (0x214255, (0x6574, false)), // East Asian ideograph
    (0x214256, (0x6582, false)), // East Asian ideograph
    (0x214257, (0x6583, false)), // East Asian ideograph
    (0x214258, (0x6587, false)), // East Asian ideograph
    (0x214259, (0x6591, false)), // East Asian ideograph
    (0x21425A, (0x6590, false)), // East Asian ideograph
    (0x6F4F32, (0xB834, false)), // Korean hangul
    (0x21425C, (0x6599, false)), // East Asian ideograph
    (0x21425D, (0x659C, false)), // East Asian ideograph
    (0x21425E, (0x659F, false)), // East Asian ideograph
    (0x21425F, (0x65A1, false)), // East Asian ideograph
    (0x214260, (0x65A4, false)), // East Asian ideograph
    (0x214261, (0x65A5, false)), // East Asian ideograph
    (0x214262, (0x65A7, false)), // East Asian ideograph
    (0x274263, (0x65A9, false)), // East Asian ideograph
    (0x214264, (0x65AF, false)), // East Asian ideograph
    (0x214265, (0x65B0, false)), // East Asian ideograph
    (0x274266, (0x65AD, false)), // East Asian ideograph
    (0x214267, (0x65B9, false)), // East Asian ideograph
    (0x224268, (0x6AB4, false)), // East Asian ideograph
    (0x214269, (0x65BD, false)), // East Asian ideograph
    (0x21426A, (0x65C1, false)), // East Asian ideograph
    (0x21426B, (0x65C5, false)), // East Asian ideograph
    (0x21426C, (0x65CE, false)), // East Asian ideograph
    (0x21426D, (0x65CB, false)), // East Asian ideograph
    (0x21426E, (0x65CC, false)), // East Asian ideograph
    (0x21426F, (0x65CF, false)), // East Asian ideograph
    (0x214270, (0x65D7, false)), // East Asian ideograph
    (0x214271, (0x65D6, false)), // East Asian ideograph
    (0x214272, (0x65E2, false)), // East Asian ideograph
    (0x214273, (0x65E5, false)), // East Asian ideograph
    (0x234274, (0x923F, false)), // East Asian ideograph
    (0x214275, (0x65E9, false)), // East Asian ideograph
    (0x214276, (0x65EC, false)), // East Asian ideograph
    (0x214277, (0x65ED, false)), // East Asian ideograph
    (0x214278, (0x65E8, false)), // East Asian ideograph
    (0x214279, (0x65F1, false)), // East Asian ideograph
    (0x21427A, (0x65FA, false)), // East Asian ideograph
    (0x21427B, (0x6606, false)), // East Asian ideograph
    (0x21427C, (0x6614, false)), // East Asian ideograph
    (0x21427D, (0x660C, false)), // East Asian ideograph
    (0x21427E, (0x6600, false)), // East Asian ideograph
    (0x6F5779, (0xC954, false)), // Korean hangul
    (0x21606B, (0x98EF, false)), // East Asian ideograph
    (0x27606C, (0x9972, false)), // East Asian ideograph
    (0x453F6D, (0x52E0, false)), // East Asian ideograph
    (0x29373A, (0x8D46, false)), // East Asian ideograph
    (0x22606D, (0x76AD, false)), // East Asian ideograph
    (0x6F5377, (0xC2A4, false)), // Korean hangul
    (0x27606E, (0x9971, false)), // East Asian ideograph
    (0x6F4F34, (0xB837, false)), // Korean hangul
    (0x21395C, (0x59B9, false)), // East Asian ideograph
    (0x27606F, (0x9970, false)), // East Asian ideograph
    (0x69243B, (0x305B, false)), // Hiragana letter SE
    (0x276070, (0x997A, false)), // East Asian ideograph
    (0x2D362A, (0x95A7, false)), // East Asian ideograph
    (0x275156, (0x7F04, false)), // East Asian ideograph
    (0x277272, (0x54D2, false)), // East Asian ideograph
    (0x236071, (0x9FA5, false)), // East Asian ideograph
    (0x213862, (0x58AE, false)), // East Asian ideograph
    (0x216072, (0x990C, false)), // East Asian ideograph
    (0x6F5378, (0xC2A5, false)), // Korean hangul
    (0x276073, (0x9977, false)), // East Asian ideograph
    (0x215B76, (0x8FF7, false)), // East Asian ideograph
    (0x21395D, (0x59C6, false)), // East Asian ideograph
    (0x216074, (0x9910, false)), // East Asian ideograph
    (0x276075, (0x9981, false)), // East Asian ideograph
    (0x6F5D71, (0xD761, false)), // Korean hangul
    (0x276076, (0x4F59, false)), // East Asian ideograph
    (0x295955, (0x9C8E, false)), // East Asian ideograph
    (0x6F4B21, (0xAF9C, false)), // Korean hangul
    (0x216077, (0x9913, false)), // East Asian ideograph
    (0x214B22, (0x733E, false)), // East Asian ideograph
    (0x2D4738, (0x6FFC, false)), // East Asian ideograph
    (0x276078, (0x997C, false)), // East Asian ideograph
    (0x214B23, (0x7345, false)), // East Asian ideograph
    (0x276079, (0x9986, false)), // East Asian ideograph
    (0x4B553F, (0x83BD, false)), // East Asian ideograph (variant of 21553F which maps to 83BD)
    (0x224B24, (0x6E5C, false)), // East Asian ideograph
    (0x27607A, (0x996F, false)), // East Asian ideograph
    (0x27607B, (0x9984, false)), // East Asian ideograph
    (0x224E6A, (0x700D, false)), // East Asian ideograph
    (0x27607C, (0x9985, false)), // East Asian ideograph
    (0x214B27, (0x7368, false)), // East Asian ideograph
    (0x6F537A, (0xC2AC, false)), // Korean hangul
    (0x217334, (0x5686, false)), // East Asian ideograph
    (0x214B28, (0x7370, false)), // East Asian ideograph
    (0x29484A, (0x956C, false)), // East Asian ideograph
    (0x27607E, (0x998F, false)), // East Asian ideograph
    (0x6F5D6D, (0xD757, false)), // Korean hangul
    (0x214B29, (0x7372, false)), // East Asian ideograph
    (0x4C5447, (0x71E0, false)), // East Asian ideograph (variant of 225447 which maps to 71E0)
    (0x214B2A, (0x7377, false)), // East Asian ideograph
    (0x2E7374, (0x7E89, false)), // East Asian ideograph
    (0x214B2B, (0x7378, false)), // East Asian ideograph
    (0x2E6060, (0x76A1, false)), // East Asian ideograph
    (0x214B2C, (0x7375, false)), // East Asian ideograph
    (0x6F537B, (0xC2AD, false)), // Korean hangul
    (0x214B2D, (0x737A, false)), // East Asian ideograph
    (0x213960, (0x59AF, false)), // East Asian ideograph
    (0x286222, (0x7726, false)), // East Asian ideograph
    (0x214B2E, (0x737B, false)), // East Asian ideograph
    (0x335F34, (0x90C4, false)), // East Asian ideograph
    (0x21393D, (0x5962, false)), // East Asian ideograph
    (0x274B2F, (0x7321, false)), // East Asian ideograph
    (0x295958, (0x9C9A, false)), // East Asian ideograph
    (0x214321, (0x660E, false)), // East Asian ideograph
    (0x214322, (0x6613, false)), // East Asian ideograph
    (0x214323, (0x6602, false)), // East Asian ideograph
    (0x214324, (0x660F, false)), // East Asian ideograph
    (0x214325, (0x6625, false)), // East Asian ideograph
    (0x214326, (0x6627, false)), // East Asian ideograph
    (0x214327, (0x662F, false)), // East Asian ideograph
    (0x214328, (0x662D, false)), // East Asian ideograph
    (0x214329, (0x6620, false)), // East Asian ideograph
    (0x21432A, (0x661F, false)), // East Asian ideograph
    (0x21432B, (0x6628, false)), // East Asian ideograph
    (0x21432C, (0x664F, false)), // East Asian ideograph
    (0x21432D, (0x6642, false)), // East Asian ideograph
    (0x21432E, (0x6652, false)), // East Asian ideograph
    (0x21432F, (0x6649, false)), // East Asian ideograph
    (0x214330, (0x6643, false)), // East Asian ideograph
    (0x214331, (0x664C, false)), // East Asian ideograph
    (0x214332, (0x665D, false)), // East Asian ideograph
    (0x214333, (0x6664, false)), // East Asian ideograph
    (0x214334, (0x6668, false)), // East Asian ideograph
    (0x214335, (0x6666, false)), // East Asian ideograph
    (0x214336, (0x665A, false)), // East Asian ideograph
    (0x214337, (0x666F, false)), // East Asian ideograph
    (0x214338, (0x666E, false)), // East Asian ideograph
    (0x214339, (0xFA12, false)), // East Asian ideograph
    (0x21433A, (0x6691, false)), // East Asian ideograph
    (0x21433B, (0x6670, false)), // East Asian ideograph
    (0x21433C, (0x6676, false)), // East Asian ideograph
    (0x21433D, (0x667A, false)), // East Asian ideograph
    (0x21433E, (0x6697, false)), // East Asian ideograph
    (0x21433F, (0x6687, false)), // East Asian ideograph
    (0x214340, (0x6689, false)), // East Asian ideograph
    (0x214341, (0x6688, false)), // East Asian ideograph
    (0x214342, (0x6696, false)), // East Asian ideograph
    (0x214343, (0x66A2, false)), // East Asian ideograph
    (0x214344, (0x66AB, false)), // East Asian ideograph
    (0x214345, (0x66B4, false)), // East Asian ideograph
    (0x214346, (0x66AE, false)), // East Asian ideograph
    (0x214347, (0x66C1, false)), // East Asian ideograph
    (0x214348, (0x66C9, false)), // East Asian ideograph
    (0x214349, (0x66C6, false)), // East Asian ideograph
    (0x21434A, (0x66B9, false)), // East Asian ideograph
    (0x21434B, (0x66D6, false)), // East Asian ideograph
    (0x21434C, (0x66D9, false)), // East Asian ideograph
    (0x21434D, (0x66E0, false)), // East Asian ideograph
    (0x21434E, (0x66DD, false)), // East Asian ideograph
    (0x21434F, (0x66E6, false)), // East Asian ideograph
    (0x214350, (0x66F0, false)), // East Asian ideograph
    (0x214351, (0x66F2, false)), // East Asian ideograph
    (0x214352, (0x66F3, false)), // East Asian ideograph
    (0x214353, (0x66F4, false)), // East Asian ideograph
    (0x214354, (0x66F7, false)), // East Asian ideograph
    (0x214355, (0x66F8, false)), // East Asian ideograph
    (0x214356, (0x66F9, false)), // East Asian ideograph
    (0x214357, (0x52D7, false)), // East Asian ideograph
    (0x214358, (0x66FE, false)), // East Asian ideograph
    (0x214359, (0x66FF, false)), // East Asian ideograph
    (0x21435A, (0x6703, false)), // East Asian ideograph
    (0x21435B, (0x6708, false)), // East Asian ideograph
    (0x21435C, (0x6709, false)), // East Asian ideograph
    (0x21435D, (0x670D, false)), // East Asian ideograph
    (0x21435E, (0x670B, false)), // East Asian ideograph
    (0x21435F, (0x6717, false)), // East Asian ideograph
    (0x214360, (0x6715, false)), // East Asian ideograph
    (0x214361, (0x6714, false)), // East Asian ideograph
    (0x214362, (0x671B, false)), // East Asian ideograph
    (0x214363, (0x671D, false)), // East Asian ideograph
    (0x214364, (0x671F, false)), // East Asian ideograph
    (0x6F537E, (0xC2B5, false)), // Korean hangul
    (0x234366, (0x92C8, false)), // East Asian ideograph
    (0x214367, (0x6728, false)), // East Asian ideograph
    (0x214369, (0x672C, false)), // East Asian ideograph
    (0x23436A, (0x92C3, false)), // East Asian ideograph
    (0x21436B, (0x672A, false)), // East Asian ideograph
    (0x29436C, (0x950D, false)), // East Asian ideograph
    (0x21436D, (0x673D, false)), // East Asian ideograph
    (0x22436E, (0x6B17, false)), // East Asian ideograph
    (0x21436F, (0x6731, false)), // East Asian ideograph
    (0x214370, (0x6735, false)), // East Asian ideograph
    (0x214371, (0x675E, false)), // East Asian ideograph
    (0x214372, (0x6751, false)), // East Asian ideograph
    (0x214373, (0x674E, false)), // East Asian ideograph
    (0x214374, (0x675C, false)), // East Asian ideograph
    (0x234375, (0x92E6, false)), // East Asian ideograph
    (0x214376, (0x6756, false)), // East Asian ideograph
    (0x214377, (0x675F, false)), // East Asian ideograph
    (0x214378, (0x674F, false)), // East Asian ideograph
    (0x214379, (0x6749, false)), // East Asian ideograph
    (0x23437A, (0x92D9, false)), // East Asian ideograph
    (0x21437B, (0x676D, false)), // East Asian ideograph
    (0x21437C, (0x678B, false)), // East Asian ideograph
    (0x21437D, (0x6795, false)), // East Asian ideograph
    (0x21437E, (0x6789, false)), // East Asian ideograph
    (0x21777B, (0x58A9, false)), // East Asian ideograph
    (0x4B3F40, (0x618E, false)), // East Asian ideograph (variant of 213F40)
    (0x214B40, (0x73ED, false)), // East Asian ideograph
    (0x27626A, (0x70B9, false)), // East Asian ideograph
    (0x214B41, (0x73EE, false)), // East Asian ideograph
    (0x214B42, (0x73E0, false)), // East Asian ideograph
    (0x214B43, (0x7405, false)), // East Asian ideograph
    (0x6F4B44, (0xB07C, false)), // Korean hangul
    (0x23457E, (0x938B, false)), // East Asian ideograph
    (0x214B45, (0x7403, false)), // East Asian ideograph
    (0x336062, (0x98C3, false)), // East Asian ideograph
    (0x214B46, (0x740A, false)), // East Asian ideograph
    (0x517954, (0x734E, false)), // East Asian ideograph
    (0x274B47, (0x73B0, false)), // East Asian ideograph
    (0x6F577B, (0xC960, false)), // Korean hangul
    (0x214B48, (0x7406, false)), // East Asian ideograph
    (0x214B49, (0x740D, false)), // East Asian ideograph
    (0x282577, (0x5CE4, false)), // East Asian ideograph
    (0x214B4A, (0x743A, false)), // East Asian ideograph
    (0x6F5338, (0xC14D, false)), // Korean hangul
    (0x6F4B4B, (0xB090, false)), // Korean hangul
    (0x213966, (0x59D4, false)), // East Asian ideograph
    (0x6F4B4C, (0xB091, false)), // Korean hangul
    (0x2D584D, (0x548F, false)), // East Asian ideograph
    (0x214B4D, (0x7434, false)), // East Asian ideograph
    (0x2E3A33, (0x80AD, false)), // East Asian ideograph
    (0x213864, (0x58C7, false)), // East Asian ideograph (variant of 4B3864 which maps to 58C7)
    (0x214B4F, (0x7433, false)), // East Asian ideograph
    (0x6F4B50, (0xB099, false)), // Korean hangul
    (0x6F5021, (0xBA38, false)), // Korean hangul
    (0x214B51, (0x7425, false)), // East Asian ideograph
    (0x6F4B52, (0xB09C, false)), // Korean hangul
    (0x213F36, (0x616E, false)), // East Asian ideograph
    (0x234B53, (0x96CA, false)), // East Asian ideograph
    (0x6F4B54, (0xB0A0, false)), // Korean hangul
    (0x6F4B55, (0xB0A1, false)), // Korean hangul
    (0x6F4F40, (0xB8B0, false)), // Korean hangul
    (0x6F5930, (0xCC4C, false)), // Korean hangul
    (0x6F4B56, (0xB0A8, false)), // Korean hangul
    (0x274B57, (0x73F2, false)), // East Asian ideograph
    (0x6F4B58, (0xB0AB, false)), // Korean hangul
    (0x214B59, (0x745E, false)), // East Asian ideograph
    (0x214B5A, (0x745C, false)), // East Asian ideograph
    (0x6F4F41, (0xB8CC, false)), // Korean hangul
    (0x214421, (0x6787, false)), // East Asian ideograph
    (0x214422, (0x6777, false)), // East Asian ideograph
    (0x214423, (0x679D, false)), // East Asian ideograph
    (0x214424, (0x6797, false)), // East Asian ideograph
    (0x214425, (0x676F, false)), // East Asian ideograph
    (0x214426, (0x6771, false)), // East Asian ideograph
    (0x214427, (0x6773, false)), // East Asian ideograph
    (0x214428, (0x679C, false)), // East Asian ideograph
    (0x214429, (0x6775, false)), // East Asian ideograph
    (0x21442A, (0x679A, false)), // East Asian ideograph
    (0x21442B, (0x6790, false)), // East Asian ideograph
    (0x22442C, (0x6B37, false)), // East Asian ideograph
    (0x21442D, (0x677E, false)), // East Asian ideograph
    (0x21442E, (0x67D3, false)), // East Asian ideograph
    (0x21442F, (0x67F1, false)), // East Asian ideograph
    (0x214430, (0x67FF, false)), // East Asian ideograph
    (0x214431, (0x67D4, false)), // East Asian ideograph
    (0x214432, (0x67C4, false)), // East Asian ideograph
    (0x214433, (0x67AF, false)), // East Asian ideograph
    (0x214434, (0x67D0, false)), // East Asian ideograph
    (0x214435, (0x67D1, false)), // East Asian ideograph
    (0x214436, (0x67EF, false)), // East Asian ideograph
    (0x214437, (0x67E9, false)), // East Asian ideograph
    (0x214438, (0x67B6, false)), // East Asian ideograph
    (0x214439, (0x67EC, false)), // East Asian ideograph
    (0x21443A, (0x67E5, false)), // East Asian ideograph
    (0x21443B, (0x67FA, false)), // East Asian ideograph
    (0x21443C, (0x67DA, false)), // East Asian ideograph
    (0x21443D, (0x6805, false)), // East Asian ideograph
    (0x21443E, (0x67DE, false)), // East Asian ideograph
    (0x21443F, (0x67B8, false)), // East Asian ideograph
    (0x214440, (0x67CF, false)), // East Asian ideograph
    (0x214441, (0x67F3, false)), // East Asian ideograph
    (0x214442, (0x6848, false)), // East Asian ideograph
    (0x214443, (0x6821, false)), // East Asian ideograph
    (0x214444, (0x6838, false)), // East Asian ideograph
    (0x214445, (0x6853, false)), // East Asian ideograph
    (0x214446, (0x6846, false)), // East Asian ideograph
    (0x214447, (0x6842, false)), // East Asian ideograph
    (0x214448, (0x6854, false)), // East Asian ideograph
    (0x214449, (0x6817, false)), // East Asian ideograph
    (0x21444A, (0x683D, false)), // East Asian ideograph
    (0x21444B, (0x6851, false)), // East Asian ideograph
    (0x21444C, (0x6829, false)), // East Asian ideograph
    (0x21444D, (0x6850, false)), // East Asian ideograph
    (0x21444E, (0x6839, false)), // East Asian ideograph
    (0x23444F, (0x9344, false)), // East Asian ideograph
    (0x214450, (0x67F4, false)), // East Asian ideograph
    (0x214451, (0x6843, false)), // East Asian ideograph
    (0x214452, (0x6840, false)), // East Asian ideograph
    (0x214453, (0x682A, false)), // East Asian ideograph
    (0x214454, (0x6845, false)), // East Asian ideograph
    (0x214455, (0x683C, false)), // East Asian ideograph
    (0x214456, (0x6813, false)), // East Asian ideograph (variant of 4B4456 which maps to 6813)
    (0x214457, (0x6881, false)), // East Asian ideograph
    (0x214458, (0x6893, false)), // East Asian ideograph
    (0x214459, (0x68AF, false)), // East Asian ideograph
    (0x21445A, (0x6876, false)), // East Asian ideograph
    (0x21445B, (0x68B0, false)), // East Asian ideograph
    (0x21445C, (0x68A7, false)), // East Asian ideograph
    (0x21445D, (0x6897, false)), // East Asian ideograph
    (0x21445E, (0x68B5, false)), // East Asian ideograph
    (0x21445F, (0x68B3, false)), // East Asian ideograph
    (0x214460, (0x68A2, false)), // East Asian ideograph
    (0x214461, (0x687F, false)), // East Asian ideograph
    (0x214462, (0x68B1, false)), // East Asian ideograph
    (0x214463, (0x689D, false)), // East Asian ideograph
    (0x214464, (0x68AD, false)), // East Asian ideograph
    (0x214465, (0x6886, false)), // East Asian ideograph
    (0x234466, (0x9312, false)), // East Asian ideograph
    (0x214467, (0x68A8, false)), // East Asian ideograph
    (0x214468, (0x689F, false)), // East Asian ideograph
    (0x214469, (0x6894, false)), // East Asian ideograph
    (0x21446A, (0x6883, false)), // East Asian ideograph
    (0x21446B, (0x68D5, false)), // East Asian ideograph
    (0x21446C, (0x68FA, false)), // East Asian ideograph
    (0x21446D, (0x68C4, false)), // East Asian ideograph
    (0x21446E, (0x68F2, false)), // East Asian ideograph
    (0x21446F, (0x68D2, false)), // East Asian ideograph
    (0x214470, (0x68E3, false)), // East Asian ideograph
    (0x214471, (0x68DF, false)), // East Asian ideograph
    (0x214472, (0x68CB, false)), // East Asian ideograph
    (0x214473, (0x68EE, false)), // East Asian ideograph
    (0x214474, (0x690D, false)), // East Asian ideograph
    (0x214475, (0x6905, false)), // East Asian ideograph
    (0x214476, (0x68E7, false)), // East Asian ideograph
    (0x214477, (0x68E0, false)), // East Asian ideograph
    (0x214478, (0x68F5, false)), // East Asian ideograph
    (0x214479, (0x68CD, false)), // East Asian ideograph
    (0x21447A, (0x68D7, false)), // East Asian ideograph
    (0x21447B, (0x68D8, false)), // East Asian ideograph
    (0x27447C, (0x832D, false)), // East Asian ideograph
    (0x21447D, (0x68F9, false)), // East Asian ideograph
    (0x21447E, (0x68DA, false)), // East Asian ideograph
    (0x214B6B, (0x74CF, false)), // East Asian ideograph
    (0x275166, (0x7EE9, false)), // East Asian ideograph
    (0x214B6C, (0x74DC, false)), // East Asian ideograph
    (0x6F2457, (0x3131, false)), // Korean hangul
    (0x6F576C, (0xC8FC, false)), // Korean hangul
    (0x214B6D, (0x74E0, false)), // East Asian ideograph
    (0x6F4B6E, (0xB108, false)), // Korean hangul
    (0x70755D, (0x8E3A, false)), // East Asian ideograph
    (0x6F4B6F, (0xB109, false)), // Korean hangul
    (0x39525B, (0x66DC, false)), // East Asian ideograph
    (0x214B71, (0x74F6, false)), // East Asian ideograph
    (0x4B3F4A, (0x5FDC, false)), // East Asian ideograph
    (0x234E35, (0x97BE, false)), // East Asian ideograph
    (0x29474D, (0x956B, false)), // East Asian ideograph
    (0x6F4B73, (0xB10F, false)), // Korean hangul
    (0x6F4F46, (0xB8F0, false)), // Korean hangul
    (0x214B74, (0x750C, false)), // East Asian ideograph
    (0x224B75, (0x6EA4, false)), // East Asian ideograph
    (0x214B76, (0x7518, false)), // East Asian ideograph
    (0x4B3F4B, (0x601C, false)), // East Asian ideograph (variant of 273F4B)
    (0x234B77, (0x96F4, false)), // East Asian ideograph
    (0x2D3622, (0x8AEE, false)), // East Asian ideograph
    (0x6F4D67, (0xB4EC, false)), // Korean hangul
    (0x214B78, (0x751C, false)), // East Asian ideograph
    (0x275E32, (0x9556, false)), // East Asian ideograph
    (0x214B79, (0x751F, false)), // East Asian ideograph
    (0x6F577D, (0xC96C, false)), // Korean hangul
    (0x335F43, (0x9D08, false)), // East Asian ideograph
    (0x333D2A, (0x5E83, false)), // East Asian ideograph
    (0x2D5856, (0x612C, false)), // East Asian ideograph
    (0x274B7A, (0x4EA7, false)), // East Asian ideograph
    (0x4B5437, (0x820E, false)), // East Asian ideograph
    (0x694B7B, (0x9EBF, false)), // East Asian ideograph
    (0x213032, (0x4E26, false)), // East Asian ideograph
    (0x214B7C, (0x7525, false)), // East Asian ideograph
    (0x6F533A, (0xC154, false)), // Korean hangul
    (0x276276, (0x51AC, false)), // East Asian ideograph
    (0x214B7D, (0x7528, false)), // East Asian ideograph
    (0x6F4F48, (0xB8F9, false)), // Korean hangul
    (0x275E33, (0x9557, false)), // East Asian ideograph
    (0x21352D, (0x53F8, false)), // East Asian ideograph
    (0x217629, (0x57E3, false)), // East Asian ideograph
    (0x23362A, (0x8C86, false)), // East Asian ideograph
    (0x213866, (0x58C1, false)), // East Asian ideograph
    (0x23527B, (0x99E3, false)), // East Asian ideograph
    (0x21762C, (0x57F6, false)), // East Asian ideograph
    (0x6F4F49, (0xB8FB, false)), // Korean hangul
    (0x23362D, (0x8C85, false)), // East Asian ideograph
    (0x29485C, (0x9565, false)), // East Asian ideograph
    (0x21352E, (0x53E4, false)), // East Asian ideograph
    (0x4D5858, (0x9BE3, false)), // East Asian ideograph
    (0x6F5D75, (0xD76C, false)), // Korean hangul
    (0x214521, (0x690E, false)), // East Asian ideograph
    (0x214522, (0x68C9, false)), // East Asian ideograph
    (0x214523, (0x6954, false)), // East Asian ideograph
    (0x214524, (0x6930, false)), // East Asian ideograph
    (0x214525, (0x6977, false)), // East Asian ideograph
    (0x214526, (0x6975, false)), // East Asian ideograph
    (0x214527, (0x695A, false)), // East Asian ideograph
    (0x214528, (0x6960, false)), // East Asian ideograph
    (0x214529, (0x696B, false)), // East Asian ideograph
    (0x21452A, (0x694A, false)), // East Asian ideograph
    (0x21452B, (0x6968, false)), // East Asian ideograph
    (0x21452C, (0x695E, false)), // East Asian ideograph
    (0x21452D, (0x696D, false)), // East Asian ideograph
    (0x21452E, (0x6979, false)), // East Asian ideograph
    (0x21452F, (0x6953, false)), // East Asian ideograph
    (0x214530, (0x6986, false)), // East Asian ideograph
    (0x214531, (0x69A8, false)), // East Asian ideograph
    (0x214532, (0x6995, false)), // East Asian ideograph
    (0x214533, (0x699C, false)), // East Asian ideograph
    (0x214534, (0x6994, false)), // East Asian ideograph
    (0x214535, (0x69C1, false)), // East Asian ideograph
    (0x214536, (0x69B7, false)), // East Asian ideograph
    (0x214537, (0x69AE, false)), // East Asian ideograph
    (0x214538, (0x699B, false)), // East Asian ideograph
    (0x214539, (0x69CB, false)), // East Asian ideograph
    (0x21453A, (0x69D3, false)), // East Asian ideograph
    (0x21453B, (0x69BB, false)), // East Asian ideograph
    (0x21453C, (0x69AB, false)), // East Asian ideograph
    (0x21453D, (0x69CC, false)), // East Asian ideograph
    (0x21453E, (0x69AD, false)), // East Asian ideograph
    (0x21453F, (0x69D0, false)), // East Asian ideograph
    (0x214540, (0x69CD, false)), // East Asian ideograph
    (0x214541, (0x69B4, false)), // East Asian ideograph
    (0x214542, (0x6A1F, false)), // East Asian ideograph
    (0x214543, (0x69E8, false)), // East Asian ideograph
    (0x274544, (0x6837, false)), // East Asian ideograph
    (0x214545, (0x69EA, false)), // East Asian ideograph
    (0x274546, (0x6869, false)), // East Asian ideograph
    (0x214547, (0x6A19, false)), // East Asian ideograph
    (0x214548, (0x69FD, false)), // East Asian ideograph
    (0x214549, (0x6A1E, false)), // East Asian ideograph
    (0x21454A, (0x6A13, false)), // East Asian ideograph
    (0x21454B, (0x6A21, false)), // East Asian ideograph
    (0x21454C, (0x69F3, false)), // East Asian ideograph
    (0x21454D, (0x6A0A, false)), // East Asian ideograph
    (0x21454E, (0x6A02, false)), // East Asian ideograph
    (0x21454F, (0x6A05, false)), // East Asian ideograph
    (0x214550, (0x6A3D, false)), // East Asian ideograph
    (0x214551, (0x6A58, false)), // East Asian ideograph
    (0x214552, (0x6A59, false)), // East Asian ideograph
    (0x214553, (0x6A62, false)), // East Asian ideograph
    (0x214554, (0x6A44, false)), // East Asian ideograph
    (0x214555, (0x6A39, false)), // East Asian ideograph
    (0x214556, (0x6A6B, false)), // East Asian ideograph
    (0x214557, (0x6A3A, false)), // East Asian ideograph
    (0x214558, (0x6A38, false)), // East Asian ideograph
    (0x214559, (0x6A47, false)), // East Asian ideograph
    (0x21455A, (0x6A61, false)), // East Asian ideograph
    (0x21455B, (0x6A4B, false)), // East Asian ideograph
    (0x21455C, (0x6A35, false)), // East Asian ideograph
    (0x21455D, (0x6A5F, false)), // East Asian ideograph
    (0x21455E, (0x6A80, false)), // East Asian ideograph
    (0x21455F, (0x6A94, false)), // East Asian ideograph
    (0x214560, (0x6A84, false)), // East Asian ideograph
    (0x214561, (0x6AA2, false)), // East Asian ideograph
    (0x214562, (0x6A9C, false)), // East Asian ideograph
    (0x214563, (0x6AB8, false)), // East Asian ideograph
    (0x214564, (0x6AB3, false)), // East Asian ideograph
    (0x214565, (0x6AC3, false)), // East Asian ideograph
    (0x214566, (0x6ABB, false)), // East Asian ideograph
    (0x234567, (0x9354, false)), // East Asian ideograph
    (0x214568, (0x6AAC, false)), // East Asian ideograph
    (0x214569, (0x6AE5, false)), // East Asian ideograph
    (0x21456A, (0x6ADA, false)), // East Asian ideograph
    (0x21456B, (0x6ADD, false)), // East Asian ideograph
    (0x21456C, (0x6ADB, false)), // East Asian ideograph
    (0x21456D, (0x6AD3, false)), // East Asian ideograph
    (0x21456E, (0x6B04, false)), // East Asian ideograph
    (0x21456F, (0x6AFB, false)), // East Asian ideograph
    (0x214570, (0x6B0A, false)), // East Asian ideograph
    (0x214571, (0x6B16, false)), // East Asian ideograph
    (0x234572, (0x936D, false)), // East Asian ideograph
    (0x214573, (0x6B21, false)), // East Asian ideograph
    (0x214574, (0x6B23, false)), // East Asian ideograph
    (0x27363E, (0x5458, false)), // East Asian ideograph
    (0x214576, (0x6B3E, false)), // East Asian ideograph
    (0x214577, (0x6B3A, false)), // East Asian ideograph
    (0x214578, (0x6B3D, false)), // East Asian ideograph
    (0x214579, (0x6B47, false)), // East Asian ideograph
    (0x21457A, (0x6B49, false)), // East Asian ideograph
    (0x21457B, (0x6B4C, false)), // East Asian ideograph
    (0x21457C, (0x6B50, false)), // East Asian ideograph
    (0x21457D, (0x6B59, false)), // East Asian ideograph
    (0x21457E, (0x6B5F, false)), // East Asian ideograph
    (0x6F7640, (0xE8B2, false)), // Korean hangul
    (0x2D3B27, (0x51A8, false)), // East Asian ideograph
    (0x453421, (0x5271, false)), // East Asian ideograph
    (0x213641, (0x5506, false)), // East Asian ideograph
    (0x4C4D3D, (0x6F62, false)), // East Asian ideograph
    (0x2D3642, (0x6B38, false)), // East Asian ideograph
    (0x335F49, (0x9D70, false)), // East Asian ideograph
    (0x4D5B7E, (0x9DC6, false)), // East Asian ideograph
    (0x27516F, (0x7F2B, false)), // East Asian ideograph
    (0x213867, (0x58BE, false)), // East Asian ideograph
    (0x213644, (0x5556, false)), // East Asian ideograph
    (0x213645, (0x5533, false)), // East Asian ideograph
    (0x6F4F4E, (0xB959, false)), // Korean hangul
    (0x275E39, (0x94D9, false)), // East Asian ideograph
    (0x6F5449, (0xC372, false)), // Korean hangul
    (0x234174, (0x91F4, false)), // East Asian ideograph
    (0x213647, (0x5537, false)), // East Asian ideograph (Version J extension)
    (0x2D3644, (0x5557, false)), // East Asian ideograph
    (0x275170, (0x7F2E, false)), // East Asian ideograph
    (0x6F553F, (0xC591, false)), // Korean hangul
    (0x213649, (0x555E, false)), // East Asian ideograph
    (0x276245, (0x9E4F, false)), // East Asian ideograph
    (0x275E3A, (0x9570, false)), // East Asian ideograph
    (0x6F764C, (0xE8BE, false)), // Korean hangul
    (0x21764D, (0x57FD, false)), // East Asian ideograph
    (0x21764E, (0x57F8, false)), // East Asian ideograph
    (0x21364F, (0x5531, false)), // East Asian ideograph
    (0x2D5749, (0x885E, false)), // East Asian ideograph
    (0x275E3B, (0x9508, false)), // East Asian ideograph
    (0x21574E, (0x521D, false)), // East Asian ideograph
    (0x6F5859, (0xCAC0, false)), // Korean hangul
    (0x233651, (0x8CBA, false)), // East Asian ideograph
    (0x233652, (0x8CB5, false)), // East Asian ideograph
    (0x213653, (0x553E, false)), // East Asian ideograph
    (0x213654, (0x5563, false)), // East Asian ideograph
    (0x6F4F51, (0xB968, false)), // Korean hangul
    (0x275E3C, (0x956D, false)), // East Asian ideograph
    (0x234177, (0x91F1, false)), // East Asian ideograph
    (0x6F7656, (0xE8C8, false)), // Korean hangul
    (0x213657, (0x552E, false)), // East Asian ideograph
    (0x6F4A3A, (0xAE38, false)), // Korean hangul
    (0x34682A, (0x7C7C, false)), // East Asian ideograph
    (0x275E3D, (0x94C1, false)), // East Asian ideograph
    (0x214621, (0x6B61, false)), // East Asian ideograph
    (0x234622, (0x938C, false)), // East Asian ideograph
    (0x214623, (0x6B63, false)), // East Asian ideograph
    (0x214624, (0x6B64, false)), // East Asian ideograph
    (0x214625, (0x6B65, false)), // East Asian ideograph
    (0x214627, (0x6B66, false)), // East Asian ideograph
    (0x214628, (0x6B6A, false)), // East Asian ideograph
    (0x214629, (0x6B72, false)), // East Asian ideograph
    (0x22462A, (0x6BF6, false)), // East Asian ideograph
    (0x21462B, (0x6B78, false)), // East Asian ideograph
    (0x21462C, (0x6B79, false)), // East Asian ideograph
    (0x21462D, (0x6B7B, false)), // East Asian ideograph
    (0x21462E, (0x6B7F, false)), // East Asian ideograph
    (0x21462F, (0x6B83, false)), // East Asian ideograph
    (0x214630, (0x6B86, false)), // East Asian ideograph
    (0x214631, (0x6B8A, false)), // East Asian ideograph
    (0x214632, (0x6B89, false)), // East Asian ideograph
    (0x214633, (0x6B98, false)), // East Asian ideograph
    (0x214634, (0x6B96, false)), // East Asian ideograph
    (0x214635, (0x6BA4, false)), // East Asian ideograph
    (0x214636, (0x6BAE, false)), // East Asian ideograph
    (0x214637, (0x6BAF, false)), // East Asian ideograph
    (0x214638, (0x6BB2, false)), // East Asian ideograph
    (0x214639, (0x6BB5, false)), // East Asian ideograph
    (0x21463A, (0x6BB7, false)), // East Asian ideograph
    (0x21463B, (0x6BBA, false)), // East Asian ideograph
    (0x21463C, (0x6BBC, false)), // East Asian ideograph
    (0x21463D, (0x6BC0, false)), // East Asian ideograph
    (0x21463E, (0x6BBF, false)), // East Asian ideograph
    (0x21463F, (0x6BC5, false)), // East Asian ideograph
    (0x214640, (0x6BC6, false)), // East Asian ideograph
    (0x214641, (0x6BCB, false)), // East Asian ideograph
    (0x214642, (0x6BCD, false)), // East Asian ideograph
    (0x214643, (0x6BCF, false)), // East Asian ideograph
    (0x214644, (0x6BD2, false)), // East Asian ideograph
    (0x214646, (0x6BD4, false)), // East Asian ideograph
    (0x214647, (0x6BD7, false)), // East Asian ideograph
    (0x214648, (0x6BDB, false)), // East Asian ideograph
    (0x214649, (0x6BEB, false)), // East Asian ideograph
    (0x21464A, (0x6BEF, false)), // East Asian ideograph
    (0x21464B, (0x6BFD, false)), // East Asian ideograph
    (0x21464C, (0x6C0F, false)), // East Asian ideograph
    (0x21464D, (0x6C11, false)), // East Asian ideograph
    (0x21464E, (0x6C10, false)), // East Asian ideograph
    (0x21464F, (0x6C13, false)), // East Asian ideograph
    (0x214650, (0x6C16, false)), // East Asian ideograph
    (0x214651, (0x6C1B, false)), // East Asian ideograph
    (0x214652, (0x6C1F, false)), // East Asian ideograph
    (0x214653, (0x6C27, false)), // East Asian ideograph
    (0x214654, (0x6C26, false)), // East Asian ideograph
    (0x214655, (0x6C23, false)), // East Asian ideograph
    (0x214656, (0x6C28, false)), // East Asian ideograph
    (0x214657, (0x6C24, false)), // East Asian ideograph
    (0x214658, (0x6C2B, false)), // East Asian ideograph
    (0x214659, (0x6C2E, false)), // East Asian ideograph
    (0x21465A, (0x6C33, false)), // East Asian ideograph
    (0x21465B, (0x6C2F, false)), // East Asian ideograph (variant of 45465B which maps to 6C2F)
    (0x21465C, (0x6C34, false)), // East Asian ideograph
    (0x21465D, (0x6C38, false)), // East Asian ideograph
    (0x21465E, (0x6C41, false)), // East Asian ideograph
    (0x23465F, (0x93E5, false)), // East Asian ideograph
    (0x214660, (0x6C40, false)), // East Asian ideograph
    (0x214661, (0x6C42, false)), // East Asian ideograph
    (0x214662, (0x6C5E, false)), // East Asian ideograph
    (0x214663, (0x6C57, false)), // East Asian ideograph
    (0x214664, (0x6C5F, false)), // East Asian ideograph
    (0x214665, (0x6C59, false)), // East Asian ideograph
    (0x214666, (0x6C60, false)), // East Asian ideograph
    (0x214667, (0x6C55, false)), // East Asian ideograph
    (0x214668, (0x6C50, false)), // East Asian ideograph
    (0x214669, (0x6C5D, false)), // East Asian ideograph
    (0x21466A, (0x6C9B, false)), // East Asian ideograph
    (0x21466B, (0x6C81, false)), // East Asian ideograph
    (0x21466D, (0x6C7A, false)), // East Asian ideograph
    (0x21466E, (0x6C6A, false)), // East Asian ideograph
    (0x21466F, (0x6C8C, false)), // East Asian ideograph
    (0x214670, (0x6C90, false)), // East Asian ideograph
    (0x214671, (0x6C72, false)), // East Asian ideograph
    (0x214672, (0x6C70, false)), // East Asian ideograph
    (0x214673, (0x6C68, false)), // East Asian ideograph
    (0x214674, (0x6C96, false)), // East Asian ideograph
    (0x234675, (0x93DB, false)), // East Asian ideograph
    (0x214676, (0x6C89, false)), // East Asian ideograph (variant of 4B4676 which maps to 6C89)
    (0x214677, (0x6C99, false)), // East Asian ideograph
    (0x214678, (0x6C7E, false)), // East Asian ideograph
    (0x214679, (0x6C7D, false)), // East Asian ideograph
    (0x21467A, (0x6C92, false)), // East Asian ideograph
    (0x21467B, (0x6C83, false)), // East Asian ideograph
    (0x21467C, (0x6CB1, false)), // East Asian ideograph
    (0x23366A, (0x8CE1, false)), // East Asian ideograph
    (0x21467E, (0x6CF3, false)), // East Asian ideograph
    (0x21366B, (0x559D, false)), // East Asian ideograph
    (0x2D5421, (0x9AD7, false)), // East Asian ideograph
    (0x6F4A56, (0xAE7D, false)), // Korean hangul
    (0x4C4359, (0x6B05, false)), // East Asian ideograph
    (0x27366D, (0x5524, false)), // East Asian ideograph
    (0x21366E, (0x557E, false)), // East Asian ideograph
    (0x294869, (0x9567, false)), // East Asian ideograph
    (0x284027, (0x6864, false)), // East Asian ideograph
    (0x21366F, (0x55AC, false)), // East Asian ideograph
    (0x213670, (0x5589, false)), // East Asian ideograph
    (0x223671, (0x6595, false)), // East Asian ideograph
    (0x213672, (0x55BB, false)), // East Asian ideograph
    (0x27406C, (0x631F, false)), // East Asian ideograph
    (0x4C3B60, (0x6764, false)), // East Asian ideograph
    (0x294228, (0x94AC, false)), // East Asian ideograph
    (0x213674, (0x55DF, false)), // East Asian ideograph
    (0x213675, (0x55D1, false)), // East Asian ideograph
    (0x213869, (0x58D3, false)), // East Asian ideograph
    (0x28734E, (0x7F32, false)), // East Asian ideograph
    (0x233676, (0x8CEE, false)), // East Asian ideograph
    (0x216121, (0x993F, false)), // East Asian ideograph
    (0x213677, (0x55E6, false)), // East Asian ideograph
    (0x4B6122, (0x994B, false)), // East Asian ideograph
    (0x6F4F58, (0xB985, false)), // Korean hangul
    (0x273678, (0x556C, false)), // East Asian ideograph
    (0x275E43, (0x94F8, false)), // East Asian ideograph
    (0x216123, (0x9945, false)), // East Asian ideograph
    (0x6F5263, (0xC0B0, false)), // Korean hangul
    (0x21353D, (0x53F2, false)), // East Asian ideograph
    (0x276124, (0x9976, false)), // East Asian ideograph
    (0x27367A, (0x5417, false)), // East Asian ideograph
    (0x2D5424, (0x5367, false)), // East Asian ideograph
    (0x23367B, (0x8CF1, false)), // East Asian ideograph
    (0x216126, (0x995C, false)), // East Asian ideograph
    (0x6F5851, (0xCA54, false)), // Korean hangul
    (0x21367C, (0x55EF, false)), // East Asian ideograph
    (0x2D475B, (0x51C9, false)), // East Asian ideograph
    (0x276127, (0x998B, false)), // East Asian ideograph
    (0x6F4F59, (0xB987, false)), // Korean hangul
    (0x21767D, (0x5844, false)), // East Asian ideograph
    (0x275E44, (0x9573, false)), // East Asian ideograph
    (0x21367E, (0x55C5, false)), // East Asian ideograph
    (0x396C6B, (0x60A4, false)), // East Asian ideograph
    (0x6F5B37, (0xD168, false)), // Korean hangul
    (0x213E61, (0x60E1, false)), // East Asian ideograph
    (0x224A4A, (0x6DE6, false)), // East Asian ideograph
    (0x4B5D34, (0x91B8, false)), // East Asian ideograph
    (0x27612C, (0x9A6C, false)), // East Asian ideograph
    (0x217971, (0x59A0, false)), // East Asian ideograph
    (0x21353F, (0x540B, false)), // East Asian ideograph
    (0x27612E, (0x9A6D, false)), // East Asian ideograph
    (0x2D6260, (0x5E85, false)), // East Asian ideograph
    (0x27612F, (0x9A70, false)), // East Asian ideograph
    (0x287351, (0x7F33, false)), // East Asian ideograph
    (0x214721, (0x6CE3, false)), // East Asian ideograph
    (0x214722, (0x6CF0, false)), // East Asian ideograph
    (0x214723, (0x6CB8, false)), // East Asian ideograph
    (0x214724, (0x6CD3, false)), // East Asian ideograph
    (0x214725, (0x6CAB, false)), // East Asian ideograph
    (0x214726, (0x6CE5, false)), // East Asian ideograph
    (0x214727, (0x6CBD, false)), // East Asian ideograph
    (0x214728, (0x6CB3, false)), // East Asian ideograph
    (0x214729, (0x6CC4, false)), // East Asian ideograph
    (0x21472A, (0x6CD5, false)), // East Asian ideograph
    (0x21472B, (0x6CE2, false)), // East Asian ideograph
    (0x21472C, (0x6CBC, false)), // East Asian ideograph
    (0x21472D, (0x6CAE, false)), // East Asian ideograph
    (0x21472E, (0x6CB9, false)), // East Asian ideograph
    (0x21472F, (0x6CF1, false)), // East Asian ideograph
    (0x214730, (0x6CC1, false)), // East Asian ideograph
    (0x214731, (0x6CBE, false)), // East Asian ideograph
    (0x214732, (0x6CC5, false)), // East Asian ideograph
    (0x214733, (0x6CD7, false)), // East Asian ideograph
    (0x234734, (0x9413, false)), // East Asian ideograph
    (0x214735, (0x6CDB, false)), // East Asian ideograph
    (0x214736, (0x6CE1, false)), // East Asian ideograph
    (0x214737, (0x6CBF, false)), // East Asian ideograph
    (0x214738, (0x6CCA, false)), // East Asian ideograph
    (0x214739, (0x6CCC, false)), // East Asian ideograph
    (0x21473A, (0x6CC9, false)), // East Asian ideograph
    (0x21473B, (0x6D41, false)), // East Asian ideograph
    (0x21473C, (0x6D0B, false)), // East Asian ideograph
    (0x21473D, (0x6D32, false)), // East Asian ideograph
    (0x21473E, (0x6D25, false)), // East Asian ideograph
    (0x21473F, (0x6D31, false)), // East Asian ideograph
    (0x214740, (0x6D2A, false)), // East Asian ideograph
    (0x214741, (0x6D0C, false)), // East Asian ideograph
    (0x214742, (0x6D1E, false)), // East Asian ideograph
    (0x214743, (0x6D17, false)), // East Asian ideograph
    (0x214744, (0x6D3B, false)), // East Asian ideograph
    (0x214745, (0x6D1B, false)), // East Asian ideograph
    (0x214746, (0x6D36, false)), // East Asian ideograph
    (0x214747, (0x6D3D, false)), // East Asian ideograph
    (0x214748, (0x6D3E, false)), // East Asian ideograph
    (0x214749, (0x6D6A, false)), // East Asian ideograph
    (0x21474A, (0x6D95, false)), // East Asian ideograph
    (0x21474B, (0x6D78, false)), // East Asian ideograph
    (0x21474C, (0x6D66, false)), // East Asian ideograph
    (0x21474D, (0x6D59, false)), // East Asian ideograph
    (0x21474E, (0x6D87, false)), // East Asian ideograph
    (0x21474F, (0x6D88, false)), // East Asian ideograph
    (0x214750, (0x6D6C, false)), // East Asian ideograph
    (0x214751, (0x6D93, false)), // East Asian ideograph
    (0x214752, (0x6D89, false)), // East Asian ideograph
    (0x214753, (0x6D6E, false)), // East Asian ideograph
    (0x214754, (0x6D74, false)), // East Asian ideograph
    (0x214755, (0x6D5A, false)), // East Asian ideograph
    (0x214756, (0x6D69, false)), // East Asian ideograph
    (0x214757, (0x6D77, false)), // East Asian ideograph
    (0x214758, (0x6DD9, false)), // East Asian ideograph
    (0x214759, (0x6DDA, false)), // East Asian ideograph
    (0x21475A, (0x6DF3, false)), // East Asian ideograph
    (0x21475B, (0x6DBC, false)), // East Asian ideograph
    (0x21475C, (0x6DE4, false)), // East Asian ideograph
    (0x21475D, (0x6DB2, false)), // East Asian ideograph
    (0x21475E, (0x6DE1, false)), // East Asian ideograph
    (0x21475F, (0x6DD2, false)), // East Asian ideograph
    (0x214760, (0x6DAE, false)), // East Asian ideograph
    (0x214761, (0x6DF8, false)), // East Asian ideograph
    (0x214762, (0x6DC7, false)), // East Asian ideograph
    (0x214763, (0x6DCB, false)), // East Asian ideograph
    (0x214764, (0x6DC5, false)), // East Asian ideograph
    (0x214765, (0x6DDE, false)), // East Asian ideograph
    (0x214766, (0x6DAF, false)), // East Asian ideograph
    (0x214767, (0x6DB5, false)), // East Asian ideograph
    (0x214768, (0x6DFA, false)), // East Asian ideograph
    (0x214769, (0x6DF9, false)), // East Asian ideograph
    (0x21476A, (0x6DCC, false)), // East Asian ideograph
    (0x21476B, (0x6DF7, false)), // East Asian ideograph
    (0x21476C, (0x6DB8, false)), // East Asian ideograph
    (0x21476D, (0x6DD1, false)), // East Asian ideograph
    (0x21476E, (0x6DF1, false)), // East Asian ideograph
    (0x21476F, (0x6DE8, false)), // East Asian ideograph
    (0x214770, (0x6DEB, false)), // East Asian ideograph
    (0x214771, (0x6DD8, false)), // East Asian ideograph
    (0x214772, (0x6DFB, false)), // East Asian ideograph
    (0x214773, (0x6DEE, false)), // East Asian ideograph
    (0x214774, (0x6DF5, false)), // East Asian ideograph
    (0x214775, (0x6D8E, false)), // East Asian ideograph
    (0x214776, (0x6DC6, false)), // East Asian ideograph
    (0x214777, (0x6DEA, false)), // East Asian ideograph
    (0x214778, (0x6DC4, false)), // East Asian ideograph
    (0x214779, (0x6E54, false)), // East Asian ideograph
    (0x21477A, (0x6E21, false)), // East Asian ideograph
    (0x21477B, (0x6E38, false)), // East Asian ideograph
    (0x21477C, (0x6E32, false)), // East Asian ideograph
    (0x21477D, (0x6E67, false)), // East Asian ideograph
    (0x21477E, (0x6E20, false)), // East Asian ideograph
    (0x4B5D38, (0x91C8, false)), // East Asian ideograph
    (0x226140, (0x76EC, false)), // East Asian ideograph
    (0x6F4F5E, (0xB9B0, false)), // Korean hangul
    (0x6F5936, (0xCC64, false)), // Korean hangul
    (0x276141, (0x9A97, false)), // East Asian ideograph
    (0x6F5777, (0xC950, false)), // Korean hangul
    (0x29442E, (0x9502, false)), // East Asian ideograph
    (0x276142, (0x9A9B, false)), // East Asian ideograph
    (0x276143, (0x9A9E, false)), // East Asian ideograph
    (0x274D3D, (0x76D1, false)), // East Asian ideograph
    (0x6F5C28, (0xD38D, false)), // Korean hangul
    (0x216144, (0x9A30, false)), // East Asian ideograph
    (0x4B624F, (0x9D49, false)), // East Asian ideograph
    (0x276145, (0x9A9A, false)), // East Asian ideograph
    (0x6F4F5F, (0xB9B4, false)), // Korean hangul
    (0x275E4A, (0x94BB, false)), // East Asian ideograph
    (0x273C31, (0x5CE6, false)), // East Asian ideograph
    (0x21575D, (0x88C2, false)), // East Asian ideograph
    (0x6F585C, (0xCACD, false)), // Korean hangul
    (0x217533, (0x5788, false)), // East Asian ideograph
    (0x276147, (0x9A71, false)), // East Asian ideograph
    (0x213860, (0x58B3, false)), // East Asian ideograph
    (0x216148, (0x9A40, false)), // East Asian ideograph
    (0x6F772D, (0xAE5F, false)), // Korean hangul
    (0x287236, (0x7F07, false)), // East Asian ideograph
    (0x6F5C29, (0xD38F, false)), // Korean hangul
    (0x276149, (0x9AA1, false)), // East Asian ideograph
    (0x27614A, (0x9A84, false)), // East Asian ideograph
    (0x6F4C6D, (0xB2E4, false)), // Korean hangul
    (0x6F4F60, (0xB9BC, false)), // Korean hangul
    (0x22614B, (0x7704, false)), // East Asian ideograph
    (0x225B5D, (0x74A1, false)), // East Asian ideograph
    (0x27614C, (0x9A7F, false)), // East Asian ideograph
    (0x27614D, (0x9A8C, false)), // East Asian ideograph
    (0x27614E, (0x9AA4, false)), // East Asian ideograph
    (0x21614F, (0x9A62, false)), // East Asian ideograph
    (0x6F4F61, (0xB9BD, false)), // Korean hangul
    (0x275E4C, (0x957F, false)), // East Asian ideograph
    (0x216150, (0x9A65, false)), // East Asian ideograph
    (0x21575F, (0x88DF, false)), // East Asian ideograph
    (0x216151, (0x9A6A, false)), // East Asian ideograph
    (0x226153, (0x76F7, false)), // East Asian ideograph
    (0x293325, (0x8BF9, false)), // East Asian ideograph
    (0x216154, (0x9AB0, false)), // East Asian ideograph
    (0x6F4F62, (0xB9BF, false)), // Korean hangul
    (0x235F5E, (0x9F39, false)), // East Asian ideograph
    (0x213F3D, (0x61A7, false)), // East Asian ideograph
    (0x216157, (0x9ABC, false)), // East Asian ideograph
    (0x287359, (0x7F31, false)), // East Asian ideograph
    (0x276158, (0x9AC5, false)), // East Asian ideograph
    (0x216159, (0x9AD3, false)), // East Asian ideograph
    (0x6F4F63, (0xB9C1, false)), // Korean hangul
    (0x275E4E, (0x95E9, false)), // East Asian ideograph
    (0x27615A, (0x4F53, false)), // East Asian ideograph
    (0x277C24, (0x5A32, false)), // East Asian ideograph
    (0x456036, (0x97FF, false)), // East Asian ideograph
    (0x214821, (0x6E5B, false)), // East Asian ideograph
    (0x214822, (0x6E1A, false)), // East Asian ideograph
    (0x214823, (0x6E56, false)), // East Asian ideograph
    (0x214824, (0x6E2F, false)), // East Asian ideograph
    (0x214825, (0x6E6E, false)), // East Asian ideograph
    (0x214826, (0x6E58, false)), // East Asian ideograph
    (0x214827, (0x6E23, false)), // East Asian ideograph
    (0x214828, (0x6E24, false)), // East Asian ideograph
    (0x214829, (0x6E1B, false)), // East Asian ideograph
    (0x21482A, (0x6E25, false)), // East Asian ideograph
    (0x21482B, (0x6E4A, false)), // East Asian ideograph
    (0x21482C, (0x6E3A, false)), // East Asian ideograph
    (0x21482D, (0x6E6F, false)), // East Asian ideograph
    (0x21482E, (0x6E2D, false)), // East Asian ideograph
    (0x22482F, (0x6CE0, false)), // East Asian ideograph
    (0x214830, (0x6E2C, false)), // East Asian ideograph
    (0x214831, (0x6E26, false)), // East Asian ideograph
    (0x214832, (0x6E4D, false)), // East Asian ideograph
    (0x214833, (0x6E3E, false)), // East Asian ideograph
    (0x214834, (0x6E43, false)), // East Asian ideograph
    (0x214835, (0x6E19, false)), // East Asian ideograph
    (0x214836, (0x6E1D, false)), // East Asian ideograph
    (0x214837, (0x6ED3, false)), // East Asian ideograph
    (0x214838, (0x6EB6, false)), // East Asian ideograph
    (0x214839, (0x6EC2, false)), // East Asian ideograph
    (0x21483B, (0x6EAF, false)), // East Asian ideograph
    (0x21483C, (0x6EA2, false)), // East Asian ideograph
    (0x27483D, (0x6C9F, false)), // East Asian ideograph
    (0x23483E, (0x944C, false)), // East Asian ideograph
    (0x21483F, (0x6EA5, false)), // East Asian ideograph
    (0x214840, (0x6E98, false)), // East Asian ideograph
    (0x214841, (0x6E90, false)), // East Asian ideograph
    (0x214842, (0x6EC5, false)), // East Asian ideograph
    (0x214843, (0x6EC7, false)), // East Asian ideograph
    (0x214844, (0x6EBC, false)), // East Asian ideograph
    (0x214845, (0x6EAB, false)), // East Asian ideograph
    (0x214846, (0x6ED1, false)), // East Asian ideograph
    (0x214847, (0x6ECB, false)), // East Asian ideograph
    (0x214848, (0x6EC4, false)), // East Asian ideograph
    (0x214849, (0x6ED4, false)), // East Asian ideograph
    (0x21484A, (0x6EAA, false)), // East Asian ideograph
    (0x21484B, (0x6E96, false)), // East Asian ideograph
    (0x21484C, (0x6E9C, false)), // East Asian ideograph
    (0x21484D, (0x6F33, false)), // East Asian ideograph
    (0x21484E, (0x6EF4, false)), // East Asian ideograph
    (0x21484F, (0x6EEC, false)), // East Asian ideograph
    (0x214850, (0x6EFE, false)), // East Asian ideograph
    (0x214851, (0x6F29, false)), // East Asian ideograph
    (0x214852, (0x6F14, false)), // East Asian ideograph
    (0x214853, (0x6F3E, false)), // East Asian ideograph
    (0x214854, (0x6F2C, false)), // East Asian ideograph
    (0x214855, (0x6F32, false)), // East Asian ideograph
    (0x214856, (0x6F0F, false)), // East Asian ideograph
    (0x214857, (0x6F22, false)), // East Asian ideograph (variant of 4B4857 which maps to 6F22)
    (0x214858, (0x6EFF, false)), // East Asian ideograph
    (0x214859, (0x6F23, false)), // East Asian ideograph
    (0x21485A, (0x6F38, false)), // East Asian ideograph
    (0x21485B, (0x6F15, false)), // East Asian ideograph
    (0x21485C, (0x6F31, false)), // East Asian ideograph
    (0x21485D, (0x6F02, false)), // East Asian ideograph
    (0x21485E, (0x6F06, false)), // East Asian ideograph
    (0x21485F, (0x6EEF, false)), // East Asian ideograph
    (0x214860, (0x6F2B, false)), // East Asian ideograph
    (0x214861, (0x6F2F, false)), // East Asian ideograph
    (0x214862, (0x6F20, false)), // East Asian ideograph
    (0x214863, (0x6F3F, false)), // East Asian ideograph
    (0x214864, (0x6EF2, false)), // East Asian ideograph
    (0x214865, (0x6F01, false)), // East Asian ideograph
    (0x214866, (0x6F11, false)), // East Asian ideograph
    (0x214867, (0x6ECC, false)), // East Asian ideograph
    (0x214868, (0x6F2A, false)), // East Asian ideograph
    (0x214869, (0x6F7C, false)), // East Asian ideograph
    (0x21486A, (0x6F88, false)), // East Asian ideograph
    (0x21486B, (0x6F84, false)), // East Asian ideograph
    (0x21486C, (0x6F51, false)), // East Asian ideograph
    (0x21486D, (0x6F64, false)), // East Asian ideograph
    (0x21486E, (0x6F97, false)), // East Asian ideograph
    (0x21486F, (0x6F54, false)), // East Asian ideograph
    (0x214870, (0x6F7A, false)), // East Asian ideograph
    (0x214871, (0x6F86, false)), // East Asian ideograph
    (0x214872, (0x6F8E, false)), // East Asian ideograph
    (0x214873, (0x6F6D, false)), // East Asian ideograph
    (0x214874, (0x6F5B, false)), // East Asian ideograph
    (0x214875, (0x6F6E, false)), // East Asian ideograph
    (0x214876, (0x6F78, false)), // East Asian ideograph
    (0x214877, (0x6F66, false)), // East Asian ideograph
    (0x214878, (0x6F70, false)), // East Asian ideograph
    (0x214879, (0x6F58, false)), // East Asian ideograph
    (0x21487A, (0x6FC2, false)), // East Asian ideograph
    (0x21487B, (0x6FB1, false)), // East Asian ideograph
    (0x21487C, (0x6FC3, false)), // East Asian ideograph
    (0x21487D, (0x6FA7, false)), // East Asian ideograph
    (0x21487E, (0x6FA1, false)), // East Asian ideograph
    (0x4D5875, (0x9CD0, false)), // East Asian ideograph
    (0x2D5D65, (0x8216, false)), // East Asian ideograph
    (0x28735D, (0x7EA9, false)), // East Asian ideograph
    (0x6F5C30, (0xD3A8, false)), // Korean hangul
    (0x22616C, (0x7722, false)), // East Asian ideograph
    (0x22616D, (0x771A, false)), // East Asian ideograph
    (0x6F4F67, (0xB9C9, false)), // Korean hangul
    (0x6F4B24, (0xAFBC, false)), // Korean hangul
    (0x21616F, (0x9B45, false)), // East Asian ideograph
    (0x28336F, (0x629F, false)), // East Asian ideograph
    (0x6F5C31, (0xD3A9, false)), // Korean hangul
    (0x6F245E, (0x3147, false)), // Korean hangul
    (0x4B5D42, (0x91E1, false)), // East Asian ideograph
    (0x27407D, (0x626B, false)), // East Asian ideograph
    (0x6F5029, (0xBA4E, false)), // Korean hangul
    (0x2D4327, (0x6630, false)), // East Asian ideograph
    (0x275E53, (0x5F00, false)), // East Asian ideograph
    (0x276173, (0x9B47, false)), // East Asian ideograph
    (0x3F3573, (0x8B3C, false)), // East Asian ideograph
    (0x226174, (0x7740, false)), // East Asian ideograph
    (0x6F4E41, (0xB610, false)), // Korean hangul
    (0x4B4C36, (0x7575, false)), // East Asian ideograph
    (0x216175, (0x9B77, false)), // East Asian ideograph
    (0x224B34, (0x6E53, false)), // East Asian ideograph
    (0x216176, (0x9B6F, false)), // East Asian ideograph
    (0x214C21, (0x752C, false)), // East Asian ideograph
    (0x226177, (0x7731, false)), // East Asian ideograph
    (0x214C22, (0x752B, false)), // East Asian ideograph
    (0x6F4F69, (0xB9CE, false)), // Korean hangul
    (0x276178, (0x9C9B, false)), // East Asian ideograph
    (0x6F4B26, (0xAFC7, false)), // Korean hangul
    (0x276179, (0x9C9C, false)), // East Asian ideograph
    (0x295269, (0x9A80, false)), // East Asian ideograph
    (0x214C24, (0x7530, false)), // East Asian ideograph
    (0x27617A, (0x9C94, false)), // East Asian ideograph
    (0x2F3143, (0x89F5, false)), // Unrelated variant of EACC 23315E which maps to 89F5
    (0x213A6B, (0x5B8F, false)), // East Asian ideograph
    (0x27617B, (0x9CA8, false)), // East Asian ideograph
    (0x214C26, (0x7531, false)), // East Asian ideograph
    (0x27617C, (0x9CA4, false)), // East Asian ideograph
    (0x214C27, (0x7533, false)), // East Asian ideograph
    (0x6F4F6A, (0xB9CF, false)), // Korean hangul
    (0x275E55, (0x95F4, false)), // East Asian ideograph
    (0x27617D, (0x9CB8, false)), // East Asian ideograph
    (0x6F4B27, (0xAFC8, false)), // Korean hangul
    (0x29593B, (0x9C9F, false)), // East Asian ideograph
    (0x27617E, (0x9CB3, false)), // East Asian ideograph
    (0x214C29, (0x7538, false)), // East Asian ideograph
    (0x215B60, (0x8FAD, false)), // East Asian ideograph
    (0x214C2A, (0x753D, false)), // East Asian ideograph
    (0x294237, (0x949B, false)), // East Asian ideograph
    (0x6F5C34, (0xD3B4, false)), // Korean hangul
    (0x39553C, (0x5D0B, false)), // East Asian ideograph
    (0x6F5360, (0xC228, false)), // Korean hangul
    (0x6F4C2B, (0xB153, false)), // Korean hangul
    (0x2D4C2C, (0x583A, false)), // East Asian ideograph
    (0x6F4F6B, (0xB9D0, false)), // Korean hangul
    (0x274C2D, (0x4EA9, false)), // East Asian ideograph
    (0x214C2E, (0x755C, false)), // East Asian ideograph
    (0x2D3661, (0x6199, false)), // East Asian ideograph
    (0x6F4C2F, (0xB15C, false)), // Korean hangul
    (0x214921, (0x6FA4, false)), // East Asian ideograph
    (0x214922, (0x6FC1, false)), // East Asian ideograph
    (0x214924, (0x6FC0, false)), // East Asian ideograph
    (0x214925, (0x6FB3, false)), // East Asian ideograph
    (0x214926, (0x6FDF, false)), // East Asian ideograph
    (0x214927, (0x6FD8, false)), // East Asian ideograph
    (0x214928, (0x6FF1, false)), // East Asian ideograph
    (0x214929, (0x6FE0, false)), // East Asian ideograph
    (0x21492A, (0x6FEF, false)), // East Asian ideograph
    (0x21492B, (0x6FEB, false)), // East Asian ideograph (variant of 4B492B which maps to 6FEB)
    (0x21492C, (0x6FE1, false)), // East Asian ideograph
    (0x21492D, (0x6FE4, false)), // East Asian ideograph
    (0x21492E, (0x6F80, false)), // East Asian ideograph
    (0x22492F, (0x6D34, false)), // East Asian ideograph (variant of 34492F which maps to 6D34)
    (0x234930, (0x9588, false)), // East Asian ideograph
    (0x214931, (0x700B, false)), // East Asian ideograph
    (0x214932, (0x7009, false)), // East Asian ideograph
    (0x214933, (0x7006, false)), // East Asian ideograph
    (0x214934, (0x6FFA, false)), // East Asian ideograph
    (0x214935, (0x7011, false)), // East Asian ideograph
    (0x214936, (0x6FFE, false)), // East Asian ideograph
    (0x214937, (0x700F, false)), // East Asian ideograph
    (0x234938, (0x959F, false)), // East Asian ideograph
    (0x214939, (0x701A, false)), // East Asian ideograph
    (0x23493A, (0x95A0, false)), // East Asian ideograph
    (0x21493B, (0x701D, false)), // East Asian ideograph
    (0x22493C, (0x6D65, false)), // East Asian ideograph
    (0x21493D, (0x701F, false)), // East Asian ideograph
    (0x22493E, (0x6D5E, false)), // East Asian ideograph
    (0x21493F, (0x703E, false)), // East Asian ideograph
    (0x214940, (0x704C, false)), // East Asian ideograph
    (0x214941, (0x7051, false)), // East Asian ideograph
    (0x214942, (0x7058, false)), // East Asian ideograph
    (0x274943, (0x6E7E, false)), // East Asian ideograph
    (0x214944, (0x7064, false)), // East Asian ideograph
    (0x214945, (0x706B, false)), // East Asian ideograph
    (0x214946, (0x7070, false)), // East Asian ideograph
    (0x214947, (0x7076, false)), // East Asian ideograph
    (0x214948, (0x707C, false)), // East Asian ideograph
    (0x214949, (0x7078, false)), // East Asian ideograph
    (0x21494A, (0x707D, false)), // East Asian ideograph
    (0x21494B, (0x7095, false)), // East Asian ideograph
    (0x21494C, (0x708E, false)), // East Asian ideograph
    (0x23494D, (0x95B9, false)), // East Asian ideograph
    (0x21494E, (0x7099, false)), // East Asian ideograph
    (0x21494F, (0x708A, false)), // East Asian ideograph
    (0x214950, (0x70AB, false)), // East Asian ideograph
    (0x214951, (0x70BA, false)), // East Asian ideograph
    (0x214952, (0x70AC, false)), // East Asian ideograph
    (0x214953, (0x70B3, false)), // East Asian ideograph
    (0x214954, (0x70AF, false)), // East Asian ideograph
    (0x214955, (0x70AD, false)), // East Asian ideograph
    (0x214956, (0x70AE, false)), // East Asian ideograph
    (0x214957, (0x70B8, false)), // East Asian ideograph
    (0x214958, (0x70CA, false)), // East Asian ideograph
    (0x214959, (0x70E4, false)), // East Asian ideograph
    (0x21495A, (0x70D8, false)), // East Asian ideograph
    (0x21495B, (0x70C8, false)), // East Asian ideograph
    (0x21495C, (0x70D9, false)), // East Asian ideograph
    (0x23495D, (0x95CE, false)), // East Asian ideograph
    (0x21495E, (0x70F9, false)), // East Asian ideograph
    (0x21495F, (0x7109, false)), // East Asian ideograph
    (0x214960, (0x710A, false)), // East Asian ideograph
    (0x214961, (0x70FD, false)), // East Asian ideograph
    (0x214962, (0x7119, false)), // East Asian ideograph
    (0x214963, (0x716E, false)), // East Asian ideograph
    (0x214964, (0x711A, false)), // East Asian ideograph
    (0x214965, (0x7136, false)), // East Asian ideograph
    (0x214966, (0x7121, false)), // East Asian ideograph
    (0x214967, (0x7130, false)), // East Asian ideograph
    (0x214968, (0x7126, false)), // East Asian ideograph
    (0x214969, (0x714E, false)), // East Asian ideograph
    (0x21496A, (0x7149, false)), // East Asian ideograph
    (0x21496B, (0x7159, false)), // East Asian ideograph
    (0x21496C, (0x7164, false)), // East Asian ideograph
    (0x21496D, (0x7169, false)), // East Asian ideograph
    (0x21496E, (0x715C, false)), // East Asian ideograph
    (0x21496F, (0x716C, false)), // East Asian ideograph
    (0x214970, (0x7166, false)), // East Asian ideograph
    (0x214971, (0x7167, false)), // East Asian ideograph
    (0x214972, (0x715E, false)), // East Asian ideograph
    (0x214973, (0x7165, false)), // East Asian ideograph
    (0x214974, (0x714C, false)), // East Asian ideograph
    (0x214975, (0x717D, false)), // East Asian ideograph
    (0x234976, (0x95E7, false)), // East Asian ideograph
    (0x214977, (0x7199, false)), // East Asian ideograph
    (0x214978, (0x718A, false)), // East Asian ideograph
    (0x214979, (0x7184, false)), // East Asian ideograph
    (0x21497A, (0x719F, false)), // East Asian ideograph
    (0x21497B, (0x71A8, false)), // East Asian ideograph
    (0x21497C, (0x71AC, false)), // East Asian ideograph
    (0x21497D, (0x71B1, false)), // East Asian ideograph
    (0x21497E, (0x71D9, false)), // East Asian ideograph
    (0x6F4C40, (0xB1DC, false)), // Korean hangul
    (0x2D432E, (0x66EC, false)), // East Asian ideograph
    (0x214C41, (0x7599, false)), // East Asian ideograph
    (0x395E71, (0x5742, false)), // East Asian ideograph
    (0x214C42, (0x759A, false)), // East Asian ideograph
    (0x6F586E, (0xCB64, false)), // Korean hangul
    (0x214C43, (0x75A4, false)), // East Asian ideograph
    (0x6F5C39, (0xD3C5, false)), // Korean hangul
    (0x224C44, (0x6F00, false)), // East Asian ideograph
    (0x4B3B31, (0x5B9F, false)), // East Asian ideograph
    (0x6F4C45, (0xB208, false)), // Korean hangul
    (0x6F4F70, (0xB9DD, false)), // Korean hangul
    (0x275E5B, (0x9601, false)), // East Asian ideograph
    (0x6F4B2D, (0xAFD8, false)), // Korean hangul
    (0x294440, (0x9506, false)), // East Asian ideograph
    (0x333475, (0x9628, false)), // East Asian ideograph
    (0x234C47, (0x9723, false)), // East Asian ideograph
    (0x27727E, (0x54D9, false)), // East Asian ideograph
    (0x21386E, (0x58DE, false)), // East Asian ideograph
    (0x6F4C48, (0xB213, false)), // Korean hangul
    (0x6F5C3A, (0xD3C8, false)), // Korean hangul
    (0x214C49, (0x75B2, false)), // East Asian ideograph
    (0x234E60, (0x97E1, false)), // East Asian ideograph
    (0x214C4A, (0x75BD, false)), // East Asian ideograph
    (0x6F4F71, (0xB9DE, false)), // Korean hangul
    (0x275E5C, (0x9600, false)), // East Asian ideograph
    (0x6F4877, (0xAC2D, false)), // Korean hangul
    (0x214C4B, (0x75BE, false)), // East Asian ideograph
    (0x294441, (0x9507, false)), // East Asian ideograph
    (0x333D54, (0x4EFD, false)), // East Asian ideograph
    (0x695C71, (0x6A78, false)), // East Asian ideograph
    (0x276F69, (0x5459, false)), // East Asian ideograph
    (0x6F5C3B, (0xD3C9, false)), // Korean hangul
    (0x70603A, (0x55EA, false)), // East Asian ideograph
    (0x69554E, (0x5B36, false)), // East Asian ideograph
    (0x214C4E, (0x75D5, false)), // East Asian ideograph
    (0x6F5852, (0xCA5C, false)), // Korean hangul
    (0x6F2460, (0x314A, false)), // Korean hangul
    (0x6F4C4F, (0xB258, false)), // Korean hangul
    (0x6F4F72, (0xB9E1, false)), // Korean hangul
    (0x275E5D, (0x5408, false)), // East Asian ideograph
    (0x214C50, (0x75B5, false)), // East Asian ideograph
    (0x214C51, (0x75CA, false)), // East Asian ideograph (variant of 4B4C51 which maps to 75CA)
    (0x2E3F2D, (0x69B2, false)), // East Asian ideograph
    (0x2F2A73, (0x87CA, false)), // East Asian ideograph
    (0x214C52, (0x75DB, false)), // East Asian ideograph
    (0x6F5C3C, (0xD3D0, false)), // Korean hangul
    (0x285150, (0x70C3, false)), // East Asian ideograph
    (0x213E66, (0x60B2, false)), // East Asian ideograph
    (0x293336, (0x8BE4, false)), // East Asian ideograph
    (0x6F4C54, (0xB274, false)), // Korean hangul
    (0x6F4F73, (0xB9E3, false)), // Korean hangul
    (0x275E5E, (0x9605, false)), // East Asian ideograph
    (0x2E4731, (0x6C73, false)), // East Asian ideograph
    (0x6F4B30, (0xB000, false)), // Korean hangul
    (0x214C56, (0x75D9, false)), // East Asian ideograph
    (0x214C57, (0x75E2, false)), // East Asian ideograph
    (0x6F5C3D, (0xD3EC, false)), // Korean hangul
    (0x234C58, (0x9730, false)), // East Asian ideograph
    (0x6F4C59, (0xB294, false)), // Korean hangul
    (0x275E5F, (0x95FE, false)), // East Asian ideograph
    (0x214C5A, (0x75F0, false)), // East Asian ideograph
    (0x394444, (0x8988, false)), // East Asian ideograph
    (0x214A21, (0x71BE, false)), // East Asian ideograph
    (0x214A22, (0x71C9, false)), // East Asian ideograph
    (0x214A23, (0x71D0, false)), // East Asian ideograph
    (0x214A24, (0x71C8, false)), // East Asian ideograph
    (0x214A25, (0x71DC, false)), // East Asian ideograph
    (0x214A26, (0x71D2, false)), // East Asian ideograph
    (0x214A27, (0x71B9, false)), // East Asian ideograph
    (0x214A28, (0x71D5, false)), // East Asian ideograph
    (0x214A29, (0x71CE, false)), // East Asian ideograph
    (0x214A2A, (0x71C3, false)), // East Asian ideograph
    (0x214A2B, (0x71C4, false)), // East Asian ideograph
    (0x214A2C, (0x71EE, false)), // East Asian ideograph
    (0x214A2D, (0x71E7, false)), // East Asian ideograph
    (0x214A2E, (0x71DF, false)), // East Asian ideograph
    (0x214A2F, (0x71E5, false)), // East Asian ideograph
    (0x214A30, (0x71ED, false)), // East Asian ideograph
    (0x214A31, (0x71E6, false)), // East Asian ideograph
    (0x234A32, (0x963C, false)), // East Asian ideograph
    (0x214A33, (0x71F4, false)), // East Asian ideograph
    (0x214A34, (0x71FB, false)), // East Asian ideograph
    (0x224A35, (0x6DDD, false)), // East Asian ideograph
    (0x274A36, (0x70C1, false)), // East Asian ideograph
    (0x214A37, (0x7210, false)), // East Asian ideograph
    (0x214A38, (0x721B, false)), // East Asian ideograph
    (0x224A39, (0x6DDB, false)), // East Asian ideograph
    (0x214A3A, (0x722A, false)), // East Asian ideograph
    (0x214A3B, (0x722D, false)), // East Asian ideograph
    (0x214A3C, (0x722C, false)), // East Asian ideograph
    (0x214A3D, (0x7230, false)), // East Asian ideograph
    (0x214A3E, (0x7235, false)), // East Asian ideograph (variant of 4B4A3E which maps to 7235)
    (0x214A3F, (0x7236, false)), // East Asian ideograph
    (0x214A40, (0x7238, false)), // East Asian ideograph
    (0x214A41, (0x7239, false)), // East Asian ideograph
    (0x214A42, (0x723A, false)), // East Asian ideograph
    (0x214A43, (0x723B, false)), // East Asian ideograph
    (0x214A44, (0x723D, false)), // East Asian ideograph
    (0x214A45, (0x723E, false)), // East Asian ideograph
    (0x224A46, (0x6DF0, false)), // East Asian ideograph
    (0x214A47, (0x7247, false)), // East Asian ideograph
    (0x214A48, (0x7248, false)), // East Asian ideograph
    (0x214A49, (0x724C, false)), // East Asian ideograph
    (0x214A4A, (0x7252, false)), // East Asian ideograph
    (0x214A4B, (0x7256, false)), // East Asian ideograph
    (0x214A4C, (0x7258, false)), // East Asian ideograph
    (0x214A4D, (0x7259, false)), // East Asian ideograph
    (0x214A4E, (0x725B, false)), // East Asian ideograph
    (0x214A4F, (0x725F, false)), // East Asian ideograph
    (0x214A50, (0x725D, false)), // East Asian ideograph
    (0x214A51, (0x7262, false)), // East Asian ideograph
    (0x214A52, (0x7261, false)), // East Asian ideograph
    (0x214A53, (0x7260, false)), // East Asian ideograph
    (0x214A54, (0x7267, false)), // East Asian ideograph
    (0x214A55, (0x7269, false)), // East Asian ideograph
    (0x214A56, (0x726F, false)), // East Asian ideograph
    (0x214A57, (0x7272, false)), // East Asian ideograph
    (0x214A58, (0x7274, false)), // East Asian ideograph
    (0x214A59, (0x7279, false)), // East Asian ideograph
    (0x214A5A, (0x727D, false)), // East Asian ideograph
    (0x214A5B, (0x7281, false)), // East Asian ideograph
    (0x214A5C, (0x7280, false)), // East Asian ideograph
    (0x214A5D, (0x7284, false)), // East Asian ideograph
    (0x274A5E, (0x8366, false)), // East Asian ideograph
    (0x214A5F, (0x7292, false)), // East Asian ideograph
    (0x224A60, (0x6E8A, false)), // East Asian ideograph
    (0x214A61, (0x72A2, false)), // East Asian ideograph
    (0x274A62, (0x727A, false)), // East Asian ideograph
    (0x214A63, (0x72AC, false)), // East Asian ideograph
    (0x214A64, (0x72AF, false)), // East Asian ideograph
    (0x214A65, (0x72C4, false)), // East Asian ideograph
    (0x214A66, (0x72C2, false)), // East Asian ideograph
    (0x214A67, (0x72D9, false)), // East Asian ideograph
    (0x274A68, (0x72B6, false)), // East Asian ideograph
    (0x214A69, (0x72CE, false)), // East Asian ideograph
    (0x214A6A, (0x72D7, false)), // East Asian ideograph
    (0x214A6B, (0x72D0, false)), // East Asian ideograph
    (0x214A6C, (0x72E1, false)), // East Asian ideograph
    (0x214A6D, (0x72E9, false)), // East Asian ideograph
    (0x214A6E, (0x72E0, false)), // East Asian ideograph
    (0x214A6F, (0x72FC, false)), // East Asian ideograph
    (0x274A70, (0x72ED, false)), // East Asian ideograph
    (0x224A71, (0x6E73, false)), // East Asian ideograph
    (0x214A72, (0x72FD, false)), // East Asian ideograph
    (0x214A73, (0x72F7, false)), // East Asian ideograph
    (0x214A74, (0x731C, false)), // East Asian ideograph
    (0x214A75, (0x731B, false)), // East Asian ideograph
    (0x214A76, (0x7313, false)), // East Asian ideograph
    (0x214A77, (0x7316, false)), // East Asian ideograph
    (0x214A78, (0x7319, false)), // East Asian ideograph
    (0x214A79, (0x7336, false)), // East Asian ideograph
    (0x214A7A, (0x7337, false)), // East Asian ideograph
    (0x214A7B, (0x7329, false)), // East Asian ideograph
    (0x214A7C, (0x7325, false)), // East Asian ideograph
    (0x214A7D, (0x7334, false)), // East Asian ideograph
    (0x214A7E, (0x7344, false)), // East Asian ideograph
    (0x2D4C3C, (0x53E0, false)), // East Asian ideograph
    (0x214C6B, (0x7634, false)), // East Asian ideograph
    (0x6F5C41, (0xD3FC, false)), // Korean hangul
    (0x213D4A, (0x5F46, false)), // East Asian ideograph
    (0x214C6C, (0x7638, false)), // East Asian ideograph
    (0x214C6D, (0x7646, false)), // East Asian ideograph
    (0x6F4F78, (0xB9F4, false)), // Korean hangul
    (0x275E63, (0x9611, false)), // East Asian ideograph
    (0x234C6E, (0x9749, false)), // East Asian ideograph
    (0x233D5B, (0x9046, false)), // East Asian ideograph
    (0x6F4C6F, (0xB2E6, false)), // Korean hangul
    (0x6F4C70, (0xB2E8, false)), // Korean hangul
    (0x214C71, (0x7658, false)), // East Asian ideograph
    (0x4D477B, (0x943E, false)), // East Asian ideograph
    (0x35344D, (0x8B5B, false)), // East Asian ideograph
    (0x6F4F79, (0xB9F5, false)), // Korean hangul
    (0x275E64, (0x95F1, false)), // East Asian ideograph
    (0x274C73, (0x75D2, false)), // East Asian ideograph
    (0x21355E, (0x542E, false)), // East Asian ideograph
    (0x275A21, (0x8D45, false)), // East Asian ideograph
    (0x6F4C74, (0xB2EE, false)), // Korean hangul
    (0x234147, (0x91AD, false)), // East Asian ideograph
    (0x217D7C, (0x5B56, false)), // East Asian ideograph
    (0x214C75, (0x7669, false)), // East Asian ideograph
    (0x6F5C43, (0xD3FF, false)), // Korean hangul
    (0x234C76, (0x975A, false)), // East Asian ideograph
    (0x233721, (0x8CF7, false)), // East Asian ideograph
    (0x287161, (0x7EFB, false)), // East Asian ideograph
    (0x3A787D, (0x80FC, false)), // East Asian ideograph
    (0x214C77, (0x766C, false)), // East Asian ideograph
    (0x273722, (0x545B, false)), // East Asian ideograph
    (0x275E65, (0x677F, false)), // East Asian ideograph
    (0x214C78, (0x7671, false)), // East Asian ideograph
    (0x213723, (0x55E1, false)), // East Asian ideograph
    (0x21754E, (0x579D, false)), // East Asian ideograph
    (0x214C79, (0x7672, false)), // East Asian ideograph (variant of 4B4C79 which maps to 7672)
    (0x694C7A, (0x9453, false)), // East Asian ideograph
    (0x213725, (0x561B, false)), // East Asian ideograph
    (0x6F5C44, (0xD401, false)), // Korean hangul
    (0x214C7B, (0x767C, false)), // East Asian ideograph
    (0x233726, (0x8CFE, false)), // East Asian ideograph
    (0x6F4C7C, (0xB2FF, false)), // Korean hangul
    (0x223727, (0x65AE, false)), // East Asian ideograph
    (0x2E4739, (0x6C67, false)), // East Asian ideograph (variant of 224739 which maps to 6C67)
    (0x214C7D, (0x767D, false)), // East Asian ideograph
    (0x6F7728, (0xAE07, false)), // Korean hangul
    (0x215336, (0x80B4, false)), // East Asian ideograph
    (0x2D4C7E, (0x4F70, false)), // East Asian ideograph
    (0x217729, (0x582D, false)), // East Asian ideograph
    (0x2D5447, (0x824A, false)), // East Asian ideograph
    (0x21372A, (0x561F, false)), // East Asian ideograph
    (0x6F5C45, (0xD440, false)), // Korean hangul
    (0x2D3A47, (0x5ACB, false)), // East Asian ideograph
    (0x4B4358, (0x66FD, false)), // East Asian ideograph
    (0x23372B, (0x8D07, false)), // East Asian ideograph
    (0x217850, (0x58D6, false)), // East Asian ideograph
    (0x334A28, (0x91BC, false)), // East Asian ideograph
    (0x27372C, (0x53F9, false)), // East Asian ideograph
    (0x6F593C, (0xCCA0, false)), // Korean hangul
    (0x275E67, (0x95EF, false)), // East Asian ideograph
    (0x6F4B39, (0xB044, false)), // Korean hangul
    (0x275A24, (0x8D3E, false)), // East Asian ideograph
    (0x27372E, (0x5455, false)), // East Asian ideograph
    (0x21372F, (0x560E, false)), // East Asian ideograph
    (0x6F5C46, (0xD444, false)), // Korean hangul
    (0x224A6D, (0x6E63, false)), // East Asian ideograph
    (0x293340, (0x8C02, false)), // East Asian ideograph
    (0x214B21, (0x733F, false)), // East Asian ideograph
    (0x224B22, (0x6E28, false)), // East Asian ideograph
    (0x274B23, (0x72EE, false)), // East Asian ideograph
    (0x214B24, (0x7350, false)), // East Asian ideograph
    (0x6F4B25, (0xAFC0, false)), // Korean hangul
    (0x214B26, (0x7357, false)), // East Asian ideograph
    (0x274B27, (0x72EC, false)), // East Asian ideograph
    (0x224B28, (0x6E5E, false)), // East Asian ideograph
    (0x274B29, (0x83B7, false)), // East Asian ideograph
    (0x274B2A, (0x72B7, false)), // East Asian ideograph
    (0x274B2B, (0x517D, false)), // East Asian ideograph
    (0x224B2C, (0x6E84, false)), // East Asian ideograph
    (0x223732, (0x65C3, false)), // East Asian ideograph
    (0x224B2E, (0x6E2E, false)), // East Asian ideograph
    (0x234B2F, (0x96A4, false)), // East Asian ideograph
    (0x214B30, (0x7384, false)), // East Asian ideograph
    (0x214B31, (0x7387, false)), // East Asian ideograph
    (0x214B32, (0x7389, false)), // East Asian ideograph
    (0x223733, (0x65C4, false)), // East Asian ideograph
    (0x214B34, (0x7396, false)), // East Asian ideograph
    (0x214B35, (0x739F, false)), // East Asian ideograph
    (0x214B36, (0x73A8, false)), // East Asian ideograph
    (0x214B37, (0x73A9, false)), // East Asian ideograph
    (0x214B38, (0x73AB, false)), // East Asian ideograph
    (0x214B39, (0x73BB, false)), // East Asian ideograph
    (0x214B3A, (0x73CA, false)), // East Asian ideograph
    (0x214B3B, (0x73B7, false)), // East Asian ideograph
    (0x214B3C, (0x73C0, false)), // East Asian ideograph
    (0x6F4B3D, (0xB04C, false)), // Korean hangul
    (0x214B3E, (0x73B2, false)), // East Asian ideograph
    (0x214B3F, (0x73CD, false)), // East Asian ideograph
    (0x224B40, (0x6E2A, false)), // East Asian ideograph
    (0x224B41, (0x6E4C, false)), // East Asian ideograph
    (0x224B42, (0x6E22, false)), // East Asian ideograph
    (0x224B43, (0x6ECE, false)), // East Asian ideograph
    (0x214B44, (0x7409, false)), // East Asian ideograph
    (0x224B45, (0x6E9B, false)), // East Asian ideograph
    (0x224B46, (0x6E9F, false)), // East Asian ideograph
    (0x214B47, (0x73FE, false)), // East Asian ideograph
    (0x224B48, (0x6EC8, false)), // East Asian ideograph
    (0x224B49, (0x6ED8, false)), // East Asian ideograph
    (0x224B4A, (0x6E8F, false)), // East Asian ideograph
    (0x214B4B, (0x7435, false)), // East Asian ideograph
    (0x214B4C, (0x7436, false)), // East Asian ideograph
    (0x224B4D, (0x6E93, false)), // East Asian ideograph
    (0x214B4E, (0x742A, false)), // East Asian ideograph
    (0x224B4F, (0x6EA0, false)), // East Asian ideograph
    (0x214B50, (0x7422, false)), // East Asian ideograph
    (0x224B51, (0x6EB1, false)), // East Asian ideograph
    (0x234B52, (0x96CE, false)), // East Asian ideograph
    (0x214B53, (0x7455, false)), // East Asian ideograph
    (0x214B54, (0x745F, false)), // East Asian ideograph
    (0x214B55, (0x745A, false)), // East Asian ideograph
    (0x214B56, (0x7441, false)), // East Asian ideograph
    (0x214B57, (0x743F, false)), // East Asian ideograph
    (0x214B58, (0x745B, false)), // East Asian ideograph
    (0x224B59, (0x6E92, false)), // East Asian ideograph
    (0x224B5A, (0x6EA7, false)), // East Asian ideograph
    (0x214B5B, (0x7459, false)), // East Asian ideograph
    (0x214B5C, (0x7483, false)), // East Asian ideograph
    (0x214B5D, (0x7469, false)), // East Asian ideograph
    (0x274B5E, (0x739B, false)), // East Asian ideograph
    (0x214B5F, (0x7463, false)), // East Asian ideograph
    (0x214B60, (0x7464, false)), // East Asian ideograph
    (0x214B61, (0x7470, false)), // East Asian ideograph
    (0x214B62, (0x748B, false)), // East Asian ideograph
    (0x214B63, (0x749C, false)), // East Asian ideograph (variant of 4B4B63 which maps to 749C)
    (0x214B64, (0x74A3, false)), // East Asian ideograph
    (0x214B65, (0x74A7, false)), // East Asian ideograph
    (0x214B66, (0x74A9, false)), // East Asian ideograph
    (0x214B67, (0x74B0, false)), // East Asian ideograph
    (0x214B68, (0x74A6, false)), // East Asian ideograph
    (0x214B69, (0x74BD, false)), // East Asian ideograph
    (0x224B6A, (0x6EC9, false)), // East Asian ideograph
    (0x274B6B, (0x73D1, false)), // East Asian ideograph
    (0x224B6C, (0x6EB3, false)), // East Asian ideograph
    (0x224B6D, (0x6EB7, false)), // East Asian ideograph
    (0x214B6E, (0x74E2, false)), // East Asian ideograph
    (0x214B6F, (0x74E3, false)), // East Asian ideograph
    (0x214B70, (0x74E6, false)), // East Asian ideograph
    (0x234B71, (0x96E9, false)), // East Asian ideograph
    (0x214B72, (0x74F7, false)), // East Asian ideograph
    (0x214B73, (0x7504, false)), // East Asian ideograph
    (0x234B74, (0x96F1, false)), // East Asian ideograph
    (0x214B75, (0x7515, false)), // East Asian ideograph
    (0x234B76, (0x96F0, false)), // East Asian ideograph
    (0x214B77, (0x751A, false)), // East Asian ideograph
    (0x234B78, (0x96FA, false)), // East Asian ideograph
    (0x224B79, (0x6ECF, false)), // East Asian ideograph
    (0x214B7A, (0x7522, false)), // East Asian ideograph
    (0x214B7B, (0x7526, false)), // East Asian ideograph
    (0x224B7C, (0x6ECA, false)), // East Asian ideograph
    (0x224B7D, (0x6ED5, false)), // East Asian ideograph
    (0x214B7E, (0x7529, false)), // East Asian ideograph
    (0x273740, (0x53FD, false)), // East Asian ideograph
    (0x6F526B, (0xC0C0, false)), // Korean hangul
    (0x294C76, (0x9753, false)), // East Asian ideograph
    (0x213565, (0x542B, false)), // East Asian ideograph
    (0x277742, (0x57D8, false)), // East Asian ideograph
    (0x293344, (0x8C19, false)), // East Asian ideograph
    (0x273744, (0x5428, false)), // East Asian ideograph
    (0x27624F, (0x9E3E, false)), // East Asian ideograph
    (0x223745, (0x65DC, false)), // East Asian ideograph
    (0x6F593D, (0xCCA8, false)), // Korean hangul
    (0x6F4B3E, (0xB053, false)), // Korean hangul
    (0x213746, (0x5679, false)), // East Asian ideograph
    (0x223747, (0x65DD, false)), // East Asian ideograph
    (0x223748, (0x65DF, false)), // East Asian ideograph
    (0x293345, (0x8BE8, false)), // East Asian ideograph
    (0x217749, (0x583D, false)), // East Asian ideograph
    (0x4B3B43, (0x5BFE, false)), // East Asian ideograph
    (0x276175, (0x9C7F, false)), // East Asian ideograph
    (0x21374A, (0x5671, false)), // East Asian ideograph
    (0x6F5D70, (0xD760, false)), // Korean hangul
    (0x6F4B3F, (0xB054, false)), // Korean hangul
    (0x21374B, (0x566F, false)), // East Asian ideograph
    (0x6F5863, (0xCB14, false)), // Korean hangul
    (0x21374C, (0x5662, false)), // East Asian ideograph (variant of 4B374C which maps to 5662)
    (0x282F47, (0x620B, false)), // East Asian ideograph
    (0x22374E, (0x65E4, false)), // East Asian ideograph
    (0x6F543A, (0xC314, false)), // Korean hangul
    (0x6F4B40, (0xB055, false)), // Korean hangul
    (0x692525, (0x30A5, false)), // Katakana letter small U
    (0x4B4C51, (0x75CA, false)), // East Asian ideograph
    (0x273751, (0x5413, false)), // East Asian ideograph
    (0x213752, (0x5690, false)), // East Asian ideograph
    (0x6F5C4D, (0xD488, false)), // Korean hangul
    (0x70604C, (0x55F5, false)), // East Asian ideograph
    (0x224A74, (0x6E4F, false)), // East Asian ideograph
    (0x334E73, (0x79A5, false)), // East Asian ideograph
    (0x234A30, (0x963D, false)), // East Asian ideograph
    (0x273754, (0x565C, false)), // East Asian ideograph
    (0x2D4343, (0x6636, false)), // East Asian ideograph
    (0x2D4768, (0x6D45, false)), // East Asian ideograph (variant of 274768 which maps to 6D45)
    (0x223755, (0x65F0, false)), // East Asian ideograph
    (0x213756, (0x56A8, false)), // East Asian ideograph
    (0x4B375A, (0x53B3, false)), // East Asian ideograph
    (0x213757, (0x56B0, false)), // East Asian ideograph
    (0x2D3758, (0x54BD, false)), // East Asian ideograph
    (0x294162, (0x9486, false)), // East Asian ideograph
    (0x212A3B, (0xE8E8, false)), // EACC component character
    (0x284056, (0x6920, false)), // East Asian ideograph
    (0x21375A, (0x56B4, false)), // East Asian ideograph
    (0x6F592D, (0xCC44, false)), // Korean hangul
    (0x224C21, (0x6EC3, false)), // East Asian ideograph
    (0x234C22, (0x96FF, false)), // East Asian ideograph
    (0x214C23, (0x752D, false)), // East Asian ideograph
    (0x224C24, (0x6EB4, false)), // East Asian ideograph
    (0x214C25, (0x7532, false)), // East Asian ideograph
    (0x224C26, (0x6EB2, false)), // East Asian ideograph
    (0x234C27, (0x9702, false)), // East Asian ideograph
    (0x214C28, (0x7537, false)), // East Asian ideograph
    (0x224C29, (0x6EB5, false)), // East Asian ideograph
    (0x234C2A, (0x9705, false)), // East Asian ideograph
    (0x214C2B, (0x754F, false)), // East Asian ideograph
    (0x214C2C, (0x754C, false)), // East Asian ideograph
    (0x214C2D, (0x755D, false)), // East Asian ideograph
    (0x224C2E, (0x6EF8, false)), // East Asian ideograph
    (0x214C2F, (0x7554, false)), // East Asian ideograph
    (0x224C30, (0x6F37, false)), // East Asian ideograph
    (0x214C31, (0x7559, false)), // East Asian ideograph
    (0x214C32, (0x7566, false)), // East Asian ideograph
    (0x214C33, (0x7562, false)), // East Asian ideograph
    (0x224C34, (0x6EFD, false)), // East Asian ideograph
    (0x224C35, (0x6F09, false)), // East Asian ideograph
    (0x214C36, (0x756B, false)), // East Asian ideograph
    (0x214C37, (0x756A, false)), // East Asian ideograph
    (0x214C38, (0x7578, false)), // East Asian ideograph
    (0x214C39, (0x7576, false)), // East Asian ideograph
    (0x214C3A, (0x7586, false)), // East Asian ideograph
    (0x214C3B, (0x7587, false)), // East Asian ideograph
    (0x214C3C, (0x758A, false)), // East Asian ideograph
    (0x224C3D, (0x6F63, false)), // East Asian ideograph
    (0x224C3E, (0x6F12, false)), // East Asian ideograph
    (0x214C3F, (0x7591, false)), // East Asian ideograph
    (0x214C40, (0x759D, false)), // East Asian ideograph
    (0x224C41, (0x6F1A, false)), // East Asian ideograph
    (0x224C42, (0x6EF6, false)), // East Asian ideograph
    (0x224C43, (0x6F19, false)), // East Asian ideograph
    (0x214C44, (0x75AB, false)), // East Asian ideograph
    (0x214C45, (0x75A5, false)), // East Asian ideograph
    (0x214C46, (0x75C7, false)), // East Asian ideograph
    (0x214C47, (0x75C5, false)), // East Asian ideograph
    (0x214C48, (0x75B3, false)), // East Asian ideograph
    (0x234C49, (0x9722, false)), // East Asian ideograph
    (0x234C4A, (0x9724, false)), // East Asian ideograph
    (0x224C4B, (0x6F24, false)), // East Asian ideograph
    (0x214C4C, (0x75BC, false)), // East Asian ideograph
    (0x214C4D, (0x75B9, false)), // East Asian ideograph
    (0x234C4E, (0x9728, false)), // East Asian ideograph
    (0x214C4F, (0x75D4, false)), // East Asian ideograph
    (0x234C50, (0x9726, false)), // East Asian ideograph
    (0x224C51, (0x6F18, false)), // East Asian ideograph
    (0x234C52, (0x9731, false)), // East Asian ideograph
    (0x214C53, (0x75E3, false)), // East Asian ideograph
    (0x214C54, (0x75D8, false)), // East Asian ideograph
    (0x214C55, (0x75DE, false)), // East Asian ideograph
    (0x274C56, (0x75C9, false)), // East Asian ideograph
    (0x224C57, (0x6F1F, false)), // East Asian ideograph
    (0x214C58, (0x7601, false)), // East Asian ideograph
    (0x214C59, (0x7600, false)), // East Asian ideograph
    (0x224C5A, (0x6F0A, false)), // East Asian ideograph
    (0x214C5B, (0x75F2, false)), // East Asian ideograph
    (0x234C5C, (0x9736, false)), // East Asian ideograph
    (0x214C5D, (0x75F4, false)), // East Asian ideograph
    (0x214C5E, (0x75FF, false)), // East Asian ideograph
    (0x214C5F, (0x75FA, false)), // East Asian ideograph
    (0x224C60, (0x6EF9, false)), // East Asian ideograph
    (0x224C61, (0x6EEE, false)), // East Asian ideograph
    (0x224C62, (0x6F41, false)), // East Asian ideograph
    (0x214C63, (0x760B, false)), // East Asian ideograph
    (0x224C64, (0x6F95, false)), // East Asian ideograph
    (0x214C65, (0x7620, false)), // East Asian ideograph
    (0x214C66, (0x7629, false)), // East Asian ideograph
    (0x214C67, (0x761F, false)), // East Asian ideograph
    (0x214C68, (0x7624, false)), // East Asian ideograph
    (0x214C69, (0x7626, false)), // East Asian ideograph
    (0x214C6A, (0x7621, false)), // East Asian ideograph
    (0x224C6B, (0x6F49, false)), // East Asian ideograph
    (0x234C6C, (0x9746, false)), // East Asian ideograph
    (0x224C6D, (0x6F30, false)), // East Asian ideograph
    (0x214C6E, (0x7642, false)), // East Asian ideograph
    (0x214C6F, (0x764C, false)), // East Asian ideograph
    (0x214C70, (0x7656, false)), // East Asian ideograph
    (0x274C71, (0x75A0, false)), // East Asian ideograph
    (0x6F4C72, (0xB2EC, false)), // Korean hangul
    (0x214C73, (0x7662, false)), // East Asian ideograph
    (0x214C74, (0x7665, false)), // East Asian ideograph
    (0x234C75, (0x9758, false)), // East Asian ideograph
    (0x214C76, (0x766E, false)), // East Asian ideograph
    (0x224C77, (0x6EEB, false)), // East Asian ideograph
    (0x224C78, (0x6F08, false)), // East Asian ideograph
    (0x224C79, (0x6F0E, false)), // East Asian ideograph
    (0x214C7A, (0x7678, false)), // East Asian ideograph
    (0x224C7B, (0x6F35, false)), // East Asian ideograph
    (0x214C7C, (0x767B, false)), // East Asian ideograph
    (0x234C7D, (0x9764, false)), // East Asian ideograph
    (0x214C7E, (0x767E, false)), // East Asian ideograph
    (0x21376B, (0x56F1, false)), // East Asian ideograph
    (0x6F5C52, (0xD4E8, false)), // Korean hangul
    (0x21376D, (0x5703, false)), // East Asian ideograph
    (0x2D4348, (0x6681, false)), // East Asian ideograph
    (0x2E4747, (0x6D64, false)), // East Asian ideograph
    (0x3F424F, (0x542F, false)), // East Asian ideograph (variant of 27424F which maps to 542F)
    (0x6F4B46, (0xB080, false)), // Korean hangul
    (0x21376E, (0x5708, false)), // East Asian ideograph
    (0x212A2F, (0xE8DD, false)), // EACC component character
    (0x27376F, (0x56EF, false)), // East Asian ideograph
    (0x273770, (0x56F4, false)), // East Asian ideograph
    (0x6F5C53, (0xD504, false)), // Korean hangul
    (0x27632B, (0x9F99, false)), // East Asian ideograph
    (0x213771, (0x5712, false)), // East Asian ideograph
    (0x294163, (0x948C, false)), // East Asian ideograph
    (0x224637, (0x6BFA, false)), // East Asian ideograph
    (0x213772, (0x5713, false)), // East Asian ideograph
    (0x2D4349, (0x66A6, false)), // East Asian ideograph
    (0x6F526D, (0xC0C5, false)), // Korean hangul
    (0x213430, (0x52C9, false)), // East Asian ideograph
    (0x6F4B47, (0xB084, false)), // Korean hangul
    (0x275A32, (0x8D4F, false)), // East Asian ideograph
    (0x216F21, (0x544F, false)), // East Asian ideograph
    (0x213774, (0x5716, false)), // East Asian ideograph
    (0x233775, (0x8D8D, false)), // East Asian ideograph
    (0x6F4E2A, (0xB554, false)), // Korean hangul
    (0x29334E, (0x8C0C, false)), // East Asian ideograph
    (0x276221, (0x9CC3, false)), // East Asian ideograph
    (0x213777, (0x572D, false)), // East Asian ideograph
    (0x216222, (0x9C0D, false)), // East Asian ideograph
    (0x6F4B48, (0xB08C, false)), // Korean hangul
    (0x29445B, (0x9516, false)), // East Asian ideograph
    (0x276223, (0x9CAB, false)), // East Asian ideograph
    (0x276224, (0x9CCD, false)), // East Asian ideograph
    (0x694C5D, (0x6762, false)), // East Asian ideograph
    (0x222225, (0x5BEF, false)), // East Asian ideograph
    (0x706054, (0x5623, false)), // East Asian ideograph
    (0x395568, (0x83DD, false)), // East Asian ideograph
    (0x234E7B, (0x980F, false)), // East Asian ideograph
    (0x216226, (0x9C31, false)), // East Asian ideograph
    (0x276177, (0x9C8D, false)), // East Asian ideograph
    (0x21377C, (0x5751, false)), // East Asian ideograph
    (0x276227, (0x9CD4, false)), // East Asian ideograph
    (0x6F4B49, (0xB08D, false)), // Korean hangul
    (0x21377D, (0x574A, false)), // East Asian ideograph
    (0x276228, (0x9CD7, false)), // East Asian ideograph
    (0x276229, (0x9CDD, false)), // East Asian ideograph
    (0x6F5C56, (0xD50C, false)), // Korean hangul
    (0x27622A, (0x9CDE, false)), // East Asian ideograph
    (0x284D27, (0x6D9D, false)), // East Asian ideograph
    (0x27622B, (0x9CDC, false)), // East Asian ideograph
    (0x27622C, (0x9CD6, false)), // East Asian ideograph
    (0x28405E, (0x67FD, false)), // East Asian ideograph
    (0x21622D, (0x9C77, false)), // East Asian ideograph
    (0x4B4C5B, (0x75F3, false)), // East Asian ideograph
    (0x27622E, (0x9C88, false)), // East Asian ideograph
    (0x2D5238, (0x898A, false)), // East Asian ideograph
    (0x2E363F, (0x52C5, false)), // East Asian ideograph
    (0x6F5C57, (0xD514, false)), // Korean hangul
    (0x27622F, (0x9E1F, false)), // East Asian ideograph
    (0x6F5959, (0xCDA7, false)), // Korean hangul
    (0x214D21, (0x7682, false)), // East Asian ideograph
    (0x214D22, (0x7684, false)), // East Asian ideograph
    (0x214D23, (0x7687, false)), // East Asian ideograph
    (0x214D24, (0x7686, false)), // East Asian ideograph
    (0x234D25, (0x9767, false)), // East Asian ideograph
    (0x214D26, (0x768E, false)), // East Asian ideograph
    (0x214D27, (0x7696, false)), // East Asian ideograph
    (0x214D28, (0x7693, false)), // East Asian ideograph
    (0x214D29, (0x769A, false)), // East Asian ideograph
    (0x214D2A, (0x76AE, false)), // East Asian ideograph
    (0x214D2B, (0x76B0, false)), // East Asian ideograph
    (0x214D2C, (0x76B4, false)), // East Asian ideograph
    (0x274D2D, (0x76B1, false)), // East Asian ideograph
    (0x214D2E, (0x76BF, false)), // East Asian ideograph
    (0x214D2F, (0x76C2, false)), // East Asian ideograph
    (0x224D30, (0x6F60, false)), // East Asian ideograph
    (0x234D31, (0x9777, false)), // East Asian ideograph
    (0x214D32, (0x76C6, false)), // East Asian ideograph
    (0x214D33, (0x76CA, false)), // East Asian ideograph
    (0x214D34, (0x76CD, false)), // East Asian ideograph
    (0x214D35, (0x76CE, false)), // East Asian ideograph
    (0x214D36, (0x76D4, false)), // East Asian ideograph
    (0x214D37, (0x76D2, false)), // East Asian ideograph
    (0x214D38, (0x76DC, false)), // East Asian ideograph
    (0x214D39, (0x76DB, false)), // East Asian ideograph
    (0x234D3A, (0x9780, false)), // East Asian ideograph
    (0x214D3B, (0x76DF, false)), // East Asian ideograph
    (0x234D3C, (0x9781, false)), // East Asian ideograph
    (0x214D3D, (0x76E3, false)), // East Asian ideograph
    (0x274D3E, (0x76D8, false)), // East Asian ideograph
    (0x274D3F, (0x5362, false)), // East Asian ideograph
    (0x214D40, (0x76E5, false)), // East Asian ideograph
    (0x214D41, (0x76EA, false)), // East Asian ideograph
    (0x214D42, (0x76EE, false)), // East Asian ideograph
    (0x214D43, (0x76EF, false)), // East Asian ideograph
    (0x214D44, (0x76F2, false)), // East Asian ideograph
    (0x214D45, (0x76F4, false)), // East Asian ideograph
    (0x214D46, (0x7709, false)), // East Asian ideograph
    (0x214D47, (0x76F9, false)), // East Asian ideograph
    (0x214D48, (0x76F8, false)), // East Asian ideograph
    (0x214D49, (0x7701, false)), // East Asian ideograph
    (0x214D4A, (0x770B, false)), // East Asian ideograph
    (0x214D4B, (0x76FC, false)), // East Asian ideograph
    (0x214D4C, (0x76FE, false)), // East Asian ideograph
    (0x214D4D, (0x7729, false)), // East Asian ideograph
    (0x214D4E, (0x7720, false)), // East Asian ideograph
    (0x214D4F, (0x771E, false)), // East Asian ideograph
    (0x214D50, (0x7728, false)), // East Asian ideograph
    (0x214D51, (0x7737, false)), // East Asian ideograph
    (0x214D52, (0x773C, false)), // East Asian ideograph
    (0x214D53, (0x7736, false)), // East Asian ideograph
    (0x214D54, (0x7738, false)), // East Asian ideograph
    (0x214D55, (0x773A, false)), // East Asian ideograph
    (0x274D56, (0x4F17, false)), // East Asian ideograph
    (0x274D57, (0x56F0, false)), // East Asian ideograph
    (0x214D58, (0x776B, false)), // East Asian ideograph
    (0x214D59, (0x775B, false)), // East Asian ideograph
    (0x214D5A, (0x776A, false)), // East Asian ideograph
    (0x214D5B, (0x7766, false)), // East Asian ideograph
    (0x214D5C, (0x7779, false)), // East Asian ideograph
    (0x274D5D, (0x7750, false)), // East Asian ideograph
    (0x214D5E, (0x7763, false)), // East Asian ideograph
    (0x214D5F, (0x775C, false)), // East Asian ideograph
    (0x214D60, (0x776C, false)), // East Asian ideograph
    (0x214D61, (0x7768, false)), // East Asian ideograph
    (0x214D62, (0x7765, false)), // East Asian ideograph
    (0x214D63, (0x777D, false)), // East Asian ideograph
    (0x214D64, (0x7771, false)), // East Asian ideograph
    (0x214D65, (0x777F, false)), // East Asian ideograph
    (0x214D66, (0x7784, false)), // East Asian ideograph
    (0x214D67, (0x7761, false)), // East Asian ideograph
    (0x214D68, (0x7787, false)), // East Asian ideograph
    (0x214D69, (0x778E, false)), // East Asian ideograph
    (0x214D6A, (0x778C, false)), // East Asian ideograph
    (0x214D6B, (0x7791, false)), // East Asian ideograph
    (0x214D6C, (0x779F, false)), // East Asian ideograph
    (0x214D6D, (0x779E, false)), // East Asian ideograph
    (0x214D6E, (0x77A0, false)), // East Asian ideograph
    (0x214D6F, (0x77A5, false)), // East Asian ideograph
    (0x214D70, (0x77B3, false)), // East Asian ideograph
    (0x214D71, (0x77AA, false)), // East Asian ideograph
    (0x214D72, (0x77B0, false)), // East Asian ideograph
    (0x214D73, (0x77AD, false)), // East Asian ideograph
    (0x214D74, (0x77AC, false)), // East Asian ideograph
    (0x214D75, (0x77A7, false)), // East Asian ideograph
    (0x214D76, (0x77BD, false)), // East Asian ideograph
    (0x214D77, (0x77BF, false)), // East Asian ideograph
    (0x214D78, (0x77BB, false)), // East Asian ideograph
    (0x224D79, (0x6FA6, false)), // East Asian ideograph
    (0x214D7A, (0x77D3, false)), // East Asian ideograph
    (0x214D7B, (0x77D7, false)), // East Asian ideograph
    (0x214D7C, (0x77DA, false)), // East Asian ideograph
    (0x214D7D, (0x77DB, false)), // East Asian ideograph
    (0x214D7E, (0x77DC, false)), // East Asian ideograph
    (0x276240, (0x9E44, false)), // East Asian ideograph
    (0x216241, (0x9D5D, false)), // East Asian ideograph
    (0x233D74, (0x9062, false)), // East Asian ideograph
    (0x216242, (0x9D89, false)), // East Asian ideograph
    (0x293B6D, (0x8F8A, false)), // East Asian ideograph
    (0x276243, (0x9E4A, false)), // East Asian ideograph
    (0x6F4D6A, (0xB4F1, false)), // Korean hangul
    (0x216244, (0x9D6A, false)), // East Asian ideograph
    (0x216245, (0x9D6C, false)), // East Asian ideograph
    (0x6F4B4F, (0xB098, false)), // Korean hangul
    (0x276246, (0x9E64, false)), // East Asian ideograph
    (0x333D75, (0x5FB3, false)), // East Asian ideograph
    (0x276247, (0x83BA, false)), // East Asian ideograph
    (0x6F5C5C, (0xD544, false)), // Korean hangul
    (0x232248, (0x8453, false)), // East Asian ideograph
    (0x276249, (0x9E67, false)), // East Asian ideograph
    (0x27624A, (0x9E25, false)), // East Asian ideograph
    (0x27624B, (0x9E36, false)), // East Asian ideograph
    (0x27624C, (0x9E70, false)), // East Asian ideograph
    (0x2E3645, (0x69E3, false)), // East Asian ideograph
    (0x21624D, (0x9DFA, false)), // East Asian ideograph
    (0x293357, (0x8C14, false)), // East Asian ideograph
    (0x27624E, (0x9E66, false)), // East Asian ideograph
    (0x21624F, (0x9E1E, false)), // East Asian ideograph
    (0x4B4F4C, (0x7A4F, false)), // East Asian ideograph
    (0x6F4B51, (0xB09A, false)), // Korean hangul
    (0x276250, (0x54B8, false)), // East Asian ideograph
    (0x294021, (0x90F8, false)), // East Asian ideograph
    (0x235B4D, (0x9D7B, false)), // East Asian ideograph
    (0x233934, (0x8DEC, false)), // East Asian ideograph
    (0x6F5C5E, (0xD54D, false)), // Korean hangul
    (0x216252, (0x9E7C, false)), // East Asian ideograph
    (0x344177, (0x8264, false)), // East Asian ideograph
    (0x39526B, (0x7094, false)), // East Asian ideograph
    (0x216256, (0x9E97, false)), // East Asian ideograph
    (0x2D5461, (0x8306, false)), // East Asian ideograph
    (0x6F5C5F, (0xD54F, false)), // Korean hangul
    (0x274931, (0x6C88, false)), // East Asian ideograph
    (0x293359, (0x8C11, false)), // East Asian ideograph
    (0x4B5D70, (0x92AD, false)), // East Asian ideograph
    (0x234A42, (0x9660, false)), // East Asian ideograph
    (0x216259, (0x9E9D, false)), // East Asian ideograph
    (0x6F4B53, (0xB09F, false)), // Korean hangul
    (0x294466, (0x9515, false)), // East Asian ideograph
    (0x214E21, (0x77E2, false)), // East Asian ideograph
    (0x214E22, (0x77E3, false)), // East Asian ideograph
    (0x214E23, (0x77E5, false)), // East Asian ideograph
    (0x214E24, (0x77E9, false)), // East Asian ideograph
    (0x214E25, (0x77ED, false)), // East Asian ideograph
    (0x214E26, (0x77EE, false)), // East Asian ideograph
    (0x214E27, (0x77EF, false)), // East Asian ideograph
    (0x214E28, (0x77F3, false)), // East Asian ideograph
    (0x214E29, (0x77FD, false)), // East Asian ideograph
    (0x214E2A, (0x7802, false)), // East Asian ideograph
    (0x214E2B, (0x780D, false)), // East Asian ideograph
    (0x214E2C, (0x780C, false)), // East Asian ideograph
    (0x234E2D, (0x97B8, false)), // East Asian ideograph
    (0x214E2E, (0x7830, false)), // East Asian ideograph
    (0x214E2F, (0x781D, false)), // East Asian ideograph
    (0x214E30, (0x7834, false)), // East Asian ideograph
    (0x214E31, (0x7838, false)), // East Asian ideograph
    (0x214E32, (0x7837, false)), // East Asian ideograph
    (0x214E33, (0x7827, false)), // East Asian ideograph
    (0x214E34, (0x782D, false)), // East Asian ideograph
    (0x214E35, (0x7825, false)), // East Asian ideograph
    (0x214E36, (0x786B, false)), // East Asian ideograph
    (0x214E37, (0x784F, false)), // East Asian ideograph
    (0x234E38, (0x97C0, false)), // East Asian ideograph
    (0x214E39, (0x786C, false)), // East Asian ideograph
    (0x214E3A, (0x785D, false)), // East Asian ideograph
    (0x214E3B, (0x786F, false)), // East Asian ideograph
    (0x214E3C, (0x78B0, false)), // East Asian ideograph
    (0x214E3D, (0x7897, false)), // East Asian ideograph
    (0x214E3E, (0x788E, false)), // East Asian ideograph
    (0x214E3F, (0x7898, false)), // East Asian ideograph
    (0x214E40, (0x7889, false)), // East Asian ideograph
    (0x214E41, (0x7891, false)), // East Asian ideograph
    (0x214E42, (0x787C, false)), // East Asian ideograph
    (0x214E43, (0x788C, false)), // East Asian ideograph
    (0x214E44, (0x78A7, false)), // East Asian ideograph
    (0x214E45, (0x78A9, false)), // East Asian ideograph
    (0x214E46, (0x789F, false)), // East Asian ideograph
    (0x214E47, (0x78B3, false)), // East Asian ideograph
    (0x214E48, (0x78CB, false)), // East Asian ideograph
    (0x214E49, (0x78BA, false)), // East Asian ideograph
    (0x214E4A, (0x78C1, false)), // East Asian ideograph
    (0x214E4B, (0x78C5, false)), // East Asian ideograph
    (0x214E4C, (0x78BC, false)), // East Asian ideograph
    (0x214E4D, (0x78D5, false)), // East Asian ideograph
    (0x214E4E, (0x78BE, false)), // East Asian ideograph
    (0x214E4F, (0x78CA, false)), // East Asian ideograph
    (0x214E50, (0x78D0, false)), // East Asian ideograph
    (0x214E51, (0x78E8, false)), // East Asian ideograph
    (0x214E52, (0x78EC, false)), // East Asian ideograph
    (0x214E53, (0x78DA, false)), // East Asian ideograph
    (0x214E54, (0x78F7, false)), // East Asian ideograph
    (0x214E55, (0x78F4, false)), // East Asian ideograph
    (0x214E56, (0x78FA, false)), // East Asian ideograph (variant of 4B4E56 which maps to 78FA)
    (0x214E57, (0x7901, false)), // East Asian ideograph
    (0x214E58, (0x78EF, false)), // East Asian ideograph
    (0x234E59, (0x97DD, false)), // East Asian ideograph
    (0x214E5A, (0x7919, false)), // East Asian ideograph
    (0x214E5B, (0x7926, false)), // East Asian ideograph
    (0x214E5C, (0x792C, false)), // East Asian ideograph
    (0x224E5D, (0x6FDE, false)), // East Asian ideograph
    (0x214E5E, (0x792B, false)), // East Asian ideograph
    (0x214E5F, (0x793A, false)), // East Asian ideograph
    (0x214E60, (0x7940, false)), // East Asian ideograph
    (0x214E61, (0x793E, false)), // East Asian ideograph
    (0x214E62, (0x7941, false)), // East Asian ideograph
    (0x214E63, (0x7945, false)), // East Asian ideograph
    (0x214E64, (0x7949, false)), // East Asian ideograph
    (0x214E65, (0x7948, false)), // East Asian ideograph
    (0x214E66, (0x7947, false)), // East Asian ideograph
    (0x224E67, (0x700C, false)), // East Asian ideograph
    (0x214E68, (0x7960, false)), // East Asian ideograph
    (0x214E69, (0x7950, false)), // East Asian ideograph
    (0x214E6A, (0x7956, false)), // East Asian ideograph
    (0x214E6B, (0x795E, false)), // East Asian ideograph
    (0x214E6C, (0x795D, false)), // East Asian ideograph
    (0x214E6D, (0x795F, false)), // East Asian ideograph
    (0x214E6E, (0x795A, false)), // East Asian ideograph
    (0x214E6F, (0x7957, false)), // East Asian ideograph
    (0x214E70, (0x7965, false)), // East Asian ideograph
    (0x214E71, (0x7968, false)), // East Asian ideograph
    (0x214E72, (0x796D, false)), // East Asian ideograph
    (0x234E73, (0x97FA, false)), // East Asian ideograph
    (0x214E74, (0x7981, false)), // East Asian ideograph
    (0x214E75, (0x797F, false)), // East Asian ideograph
    (0x214E76, (0x798F, false)), // East Asian ideograph
    (0x214E77, (0x798D, false)), // East Asian ideograph
    (0x214E78, (0x798E, false)), // East Asian ideograph
    (0x214E79, (0x79A6, false)), // East Asian ideograph
    (0x214E7A, (0x79A7, false)), // East Asian ideograph
    (0x214E7B, (0x79AA, false)), // East Asian ideograph
    (0x214E7C, (0x79AE, false)), // East Asian ideograph
    (0x214E7D, (0x79B1, false)), // East Asian ideograph
    (0x214E7E, (0x79B9, false)), // East Asian ideograph
    (0x6F5C63, (0xD558, false)), // Korean hangul
    (0x6F4E2D, (0xB55F, false)), // Korean hangul
    (0x29335D, (0x8C16, false)), // East Asian ideograph
    (0x216461, (0x4EC8, false)), // East Asian ideograph
    (0x234A46, (0x9658, false)), // East Asian ideograph
    (0x69626D, (0x7874, false)), // East Asian ideograph
    (0x6F4B57, (0xB0A9, false)), // Korean hangul
    (0x27626F, (0x515A, false)), // East Asian ideograph
    (0x6F5C64, (0xD559, false)), // Korean hangul
    (0x274936, (0x6EE4, false)), // East Asian ideograph
    (0x6F5821, (0xC974, false)), // Korean hangul
    (0x6F4D53, (0xB450, false)), // Korean hangul
    (0x216271, (0x9EF4, false)), // East Asian ideograph
    (0x216272, (0x9EF7, false)), // East Asian ideograph
    (0x4B6159, (0x81B8, false)), // East Asian ideograph
    (0x216273, (0x9F07, false)), // East Asian ideograph
    (0x216275, (0x9F13, false)), // East Asian ideograph
    (0x274937, (0x6D4F, false)), // East Asian ideograph
    (0x6F5822, (0xC988, false)), // Korean hangul
    (0x216276, (0x9F15, false)), // East Asian ideograph
    (0x274633, (0x6B8B, false)), // East Asian ideograph
    (0x6F4D22, (0xB308, false)), // Korean hangul
    (0x6F4B59, (0xB0AC, false)), // Korean hangul
    (0x4B6278, (0x9F21, false)), // East Asian ideograph
    (0x224D23, (0x6F7E, false)), // East Asian ideograph
    (0x21712D, (0x5593, false)), // East Asian ideograph
    (0x224D24, (0x6F9D, false)), // East Asian ideograph
    (0x21627A, (0x9F34, false)), // East Asian ideograph
    (0x6F4D25, (0xB313, false)), // Korean hangul
    (0x6F5823, (0xC989, false)), // Korean hangul
    (0x23227B, (0x8484, false)), // East Asian ideograph
    (0x22464A, (0x6C05, false)), // East Asian ideograph
    (0x6F4D26, (0xB314, false)), // Korean hangul
    (0x6F534B, (0xC1B0, false)), // Korean hangul
    (0x23227C, (0x8478, false)), // East Asian ideograph
    (0x224D27, (0x6F87, false)), // East Asian ideograph
    (0x6F4B5A, (0xB0AD, false)), // Korean hangul
    (0x21627D, (0x9F4A, false)), // East Asian ideograph
    (0x6F4D28, (0xB354, false)), // Korean hangul
    (0x27627E, (0x658E, false)), // East Asian ideograph
    (0x274D29, (0x7691, false)), // East Asian ideograph
    (0x274D7C, (0x77A9, false)), // East Asian ideograph
    (0x6F5C67, (0xD565, false)), // Korean hangul
    (0x6F4D2A, (0xB358, false)), // Korean hangul
    (0x6F5824, (0xC98C, false)), // Korean hangul
    (0x213C7A, (0x5EB8, false)), // East Asian ideograph
    (0x224D2B, (0x6F6F, false)), // East Asian ideograph
    (0x6F5271, (0xC0CF, false)), // Korean hangul
    (0x234D2C, (0x976B, false)), // East Asian ideograph
    (0x6F4B5B, (0xB0AE, false)), // Korean hangul
    (0x214D2D, (0x76BA, false)), // East Asian ideograph
    (0x6F4C39, (0xB192, false)), // Korean hangul
    (0x29402B, (0x90BA, false)), // East Asian ideograph
    (0x23393E, (0x8DF2, false)), // East Asian ideograph
    (0x282D79, (0x60AB, false)), // East Asian ideograph
    (0x6F4D2E, (0xB364, false)), // Korean hangul
    (0x2D3251, (0x510C, false)), // East Asian ideograph
    (0x6F492C, (0xAC84, false)), // Korean hangul
    (0x6F5C68, (0xD568, false)), // Korean hangul
    (0x224D2F, (0x6F5A, false)), // East Asian ideograph
    (0x293362, (0x8C1D, false)), // East Asian ideograph
    (0x214F21, (0x79BD, false)), // East Asian ideograph
    (0x214F22, (0x842C, false)), // East Asian ideograph
    (0x214F23, (0x79BE, false)), // East Asian ideograph
    (0x214F24, (0x79C0, false)), // East Asian ideograph
    (0x214F25, (0x79C1, false)), // East Asian ideograph
    (0x214F26, (0x79BF, false)), // East Asian ideograph
    (0x214D31, (0x76C8, false)), // East Asian ideograph
    (0x214F28, (0x79D1, false)), // East Asian ideograph
    (0x214F29, (0x79CB, false)), // East Asian ideograph
    (0x214F2A, (0x79D2, false)), // East Asian ideograph
    (0x214F2B, (0x79E4, false)), // East Asian ideograph
    (0x214F2C, (0x79E6, false)), // East Asian ideograph
    (0x214F2D, (0x79E3, false)), // East Asian ideograph
    (0x214F2E, (0x79DF, false)), // East Asian ideograph
    (0x214F2F, (0x79E7, false)), // East Asian ideograph
    (0x214F30, (0x79E9, false)), // East Asian ideograph
    (0x224F31, (0x702D, false)), // East Asian ideograph
    (0x214F32, (0x7A05, false)), // East Asian ideograph
    (0x214F33, (0x7A0D, false)), // East Asian ideograph
    (0x214F34, (0x7A08, false)), // East Asian ideograph
    (0x214F35, (0x7A0B, false)), // East Asian ideograph
    (0x214F36, (0x7A00, false)), // East Asian ideograph
    (0x214F37, (0x7A1F, false)), // East Asian ideograph
    (0x234F38, (0x981F, false)), // East Asian ideograph
    (0x214F39, (0x7A20, false)), // East Asian ideograph
    (0x214F3A, (0x7A1A, false)), // East Asian ideograph
    (0x214F3B, (0x7A14, false)), // East Asian ideograph
    (0x214F3C, (0x7A31, false)), // East Asian ideograph
    (0x214F3D, (0x7A2E, false)), // East Asian ideograph
    (0x214F3E, (0x7A3F, false)), // East Asian ideograph
    (0x214F3F, (0x7A3C, false)), // East Asian ideograph
    (0x274F40, (0x8C37, false)), // East Asian ideograph
    (0x214F41, (0x7A3D, false)), // East Asian ideograph
    (0x214F42, (0x7A37, false)), // East Asian ideograph
    (0x214F43, (0x7A3B, false)), // East Asian ideograph
    (0x214F44, (0x7A4D, false)), // East Asian ideograph
    (0x214F45, (0x7A4E, false)), // East Asian ideograph
    (0x214F46, (0x7A4C, false)), // East Asian ideograph
    (0x214F47, (0x7A46, false)), // East Asian ideograph
    (0x214F48, (0x7A57, false)), // East Asian ideograph
    (0x274F49, (0x7A51, false)), // East Asian ideograph
    (0x214F4A, (0x7A62, false)), // East Asian ideograph
    (0x274F4B, (0x83B7, false)), // East Asian ideograph (duplicate simplified)
    (0x214F4C, (0x7A69, false)), // East Asian ideograph
    (0x214F4D, (0x7A74, false)), // East Asian ideograph
    (0x214F4E, (0x7A76, false)), // East Asian ideograph
    (0x214F4F, (0x7A79, false)), // East Asian ideograph
    (0x214F50, (0x7A7A, false)), // East Asian ideograph
    (0x214F51, (0x7A7F, false)), // East Asian ideograph
    (0x214F52, (0x7A81, false)), // East Asian ideograph
    (0x214F53, (0x7A84, false)), // East Asian ideograph
    (0x214F54, (0x7A88, false)), // East Asian ideograph
    (0x214F55, (0x7A92, false)), // East Asian ideograph
    (0x214F56, (0x7A95, false)), // East Asian ideograph
    (0x214F57, (0x7A98, false)), // East Asian ideograph
    (0x214F58, (0x7A96, false)), // East Asian ideograph
    (0x214F59, (0x7A97, false)), // East Asian ideograph
    (0x214F5A, (0x7A9F, false)), // East Asian ideograph
    (0x214F5B, (0x7AA0, false)), // East Asian ideograph
    (0x214F5C, (0x7AAA, false)), // East Asian ideograph
    (0x214D3A, (0x76DE, false)), // East Asian ideograph
    (0x214F5E, (0x7AAF, false)), // East Asian ideograph
    (0x214F5F, (0x7AAE, false)), // East Asian ideograph
    (0x274F60, (0x7AA5, false)), // East Asian ideograph
    (0x274F61, (0x7A8D, false)), // East Asian ideograph
    (0x274F62, (0x7A9C, false)), // East Asian ideograph
    (0x274F63, (0x7AA6, false)), // East Asian ideograph
    (0x214F64, (0x7ACA, false)), // East Asian ideograph
    (0x214F65, (0x7ACB, false)), // East Asian ideograph
    (0x214F66, (0x7AD9, false)), // East Asian ideograph
    (0x214F67, (0x7AE5, false)), // East Asian ideograph
    (0x214F68, (0x7AE3, false)), // East Asian ideograph
    (0x214D3C, (0x76E1, false)), // East Asian ideograph
    (0x214F6A, (0x7AEF, false)), // East Asian ideograph
    (0x274F6B, (0x7ADE, false)), // East Asian ideograph
    (0x214F6C, (0x7AF9, false)), // East Asian ideograph
    (0x214F6D, (0x7AFA, false)), // East Asian ideograph
    (0x214F6E, (0x7AFF, false)), // East Asian ideograph
    (0x214F6F, (0x7AFD, false)), // East Asian ideograph
    (0x214F70, (0x7B06, false)), // East Asian ideograph
    (0x214F71, (0x7B11, false)), // East Asian ideograph
    (0x214F72, (0x7B20, false)), // East Asian ideograph
    (0x214F73, (0x7B2C, false)), // East Asian ideograph
    (0x214F74, (0x7B28, false)), // East Asian ideograph
    (0x214D3E, (0x76E4, false)), // East Asian ideograph
    (0x214F76, (0x7B1E, false)), // East Asian ideograph
    (0x214F77, (0x7B19, false)), // East Asian ideograph
    (0x214F78, (0x7B26, false)), // East Asian ideograph
    (0x214F79, (0x7B46, false)), // East Asian ideograph
    (0x214F7A, (0x7B49, false)), // East Asian ideograph
    (0x214D3F, (0x76E7, false)), // East Asian ideograph
    (0x214F7C, (0x7B56, false)), // East Asian ideograph
    (0x214F7D, (0x7B52, false)), // East Asian ideograph
    (0x214F7E, (0x7B4B, false)), // East Asian ideograph
    (0x234D40, (0x9784, false)), // East Asian ideograph
    (0x6F4B5F, (0xB0B4, false)), // Korean hangul
    (0x294472, (0x951E, false)), // East Asian ideograph
    (0x4B4D41, (0x862F, false)), // East Asian ideograph
    (0x6F4D42, (0xB3CC, false)), // Korean hangul
    (0x2E3654, (0x657F, false)), // East Asian ideograph
    (0x2D502B, (0x693E, false)), // East Asian ideograph
    (0x234D43, (0x977F, false)), // East Asian ideograph
    (0x6F5829, (0xC9C0, false)), // Korean hangul
    (0x224D44, (0x6F0B, false)), // East Asian ideograph
    (0x2D4362, (0x6722, false)), // East Asian ideograph
    (0x4B4D45, (0x76F4, false)), // East Asian ideograph (variant of 214D45 which maps to 76F4)
    (0x6F4B60, (0xB0B5, false)), // Korean hangul
    (0x217577, (0x57D2, false)), // East Asian ideograph
    (0x6F4D46, (0xB3D7, false)), // Korean hangul
    (0x213145, (0x4F9D, false)), // East Asian ideograph
    (0x217134, (0x5588, false)), // East Asian ideograph
    (0x6F4D47, (0xB3D9, false)), // Korean hangul
    (0x6F5C6D, (0xD571, false)), // Korean hangul
    (0x27493F, (0x6F9C, false)), // East Asian ideograph
    (0x6F582A, (0xC9C1, false)), // Korean hangul
    (0x276256, (0x4E3D, false)), // East Asian ideograph
    (0x224651, (0x6C0C, false)), // East Asian ideograph
    (0x234D49, (0x9789, false)), // East Asian ideograph
    (0x6F4D4A, (0xB400, false)), // Korean hangul
    (0x6F4B61, (0xB0B8, false)), // Korean hangul
    (0x294474, (0x951F, false)), // East Asian ideograph
    (0x224D4B, (0x6F6C, false)), // East Asian ideograph
    (0x294031, (0x909D, false)), // East Asian ideograph
    (0x333944, (0x5B2D, false)), // East Asian ideograph
    (0x6F4D4C, (0xB418, false)), // Korean hangul
    (0x283F30, (0x6966, false)), // East Asian ideograph
    (0x224D4D, (0x6F8B, false)), // East Asian ideograph
    (0x6F582B, (0xC9C4, false)), // Korean hangul
    (0x6F4D4E, (0xB420, false)), // Korean hangul
    (0x2D4364, (0x671E, false)), // East Asian ideograph
    (0x2D4D4F, (0x771F, false)), // East Asian ideograph
    (0x276327, (0x9F89, false)), // East Asian ideograph
    (0x6F586A, (0xCB50, false)), // Korean hangul
    (0x6F4D50, (0xB429, false)), // Korean hangul
    (0x213147, (0x4F75, false)), // East Asian ideograph
    (0x6F4D51, (0xB42B, false)), // Korean hangul
    (0x212329, (0xFF09, false)), // Ideographic right parenthesis
    (0x6F4D52, (0xB42C, false)), // Korean hangul
    (0x6F582C, (0xC9C7, false)), // Korean hangul
    (0x4B3B67, (0x6B67, false)), // East Asian ideograph
    (0x234D54, (0x9794, false)), // East Asian ideograph
    (0x6F4B63, (0xB0BC, false)), // Korean hangul
    (0x335773, (0x88B4, false)), // East Asian ideograph
    (0x6F4D55, (0xB454, false)), // Korean hangul
    (0x27514A, (0x7EF0, false)), // East Asian ideograph
    (0x214D56, (0x773E, false)), // East Asian ideograph
    (0x6F5C70, (0xD578, false)), // Korean hangul
    (0x276B5B, (0x5250, false)), // East Asian ideograph
    (0x214D57, (0x774F, false)), // East Asian ideograph
    (0x3F5959, (0x8276, false)), // East Asian ideograph
    (0x224D58, (0x6E88, false)), // East Asian ideograph
    (0x35347B, (0x8B2D, false)), // East Asian ideograph
    (0x234D59, (0x979B, false)), // East Asian ideograph
    (0x6F4B64, (0xB0C4, false)), // Korean hangul
    (0x224D5A, (0x6F55, false)), // East Asian ideograph
    (0x213149, (0x4F73, false)), // East Asian ideograph
    (0x21623E, (0x9D61, false)), // East Asian ideograph
    (0x235021, (0x9865, false)), // East Asian ideograph
    (0x235022, (0x9866, false)), // East Asian ideograph
    (0x215023, (0x7B54, false)), // East Asian ideograph
    (0x215024, (0x7B60, false)), // East Asian ideograph
    (0x215025, (0x7B77, false)), // East Asian ideograph
    (0x215026, (0x7B75, false)), // East Asian ideograph
    (0x215027, (0x7BA1, false)), // East Asian ideograph
    (0x215028, (0x7B94, false)), // East Asian ideograph
    (0x235029, (0x986C, false)), // East Asian ideograph
    (0x21502A, (0x7B9D, false)), // East Asian ideograph
    (0x21502B, (0x7B8B, false)), // East Asian ideograph
    (0x21502C, (0x7B97, false)), // East Asian ideograph
    (0x21502D, (0x7B8F, false)), // East Asian ideograph
    (0x21502E, (0x7BC7, false)), // East Asian ideograph
    (0x214D5D, (0x775E, false)), // East Asian ideograph
    (0x235030, (0x9873, false)), // East Asian ideograph
    (0x215031, (0x7BB1, false)), // East Asian ideograph
    (0x215032, (0x7BB4, false)), // East Asian ideograph
    (0x215033, (0x7BC0, false)), // East Asian ideograph
    (0x215034, (0x7BC6, false)), // East Asian ideograph
    (0x215035, (0x7BC1, false)), // East Asian ideograph
    (0x215036, (0x7C11, false)), // East Asian ideograph
    (0x215037, (0x7BD9, false)), // East Asian ideograph
    (0x215038, (0x7BDB, false)), // East Asian ideograph
    (0x235039, (0x98AD, false)), // East Asian ideograph
    (0x21503A, (0x7BC9, false)), // East Asian ideograph
    (0x21503B, (0x7BE1, false)), // East Asian ideograph
    (0x21503C, (0x7BE9, false)), // East Asian ideograph
    (0x21503D, (0x7C07, false)), // East Asian ideograph
    (0x21503E, (0x7C0D, false)), // East Asian ideograph
    (0x21503F, (0x7BFE, false)), // East Asian ideograph
    (0x235040, (0x98B4, false)), // East Asian ideograph
    (0x215041, (0x7C21, false)), // East Asian ideograph
    (0x215042, (0x7C2B, false)), // East Asian ideograph
    (0x215043, (0x7C2A, false)), // East Asian ideograph
    (0x215044, (0x7C27, false)), // East Asian ideograph
    (0x215045, (0x7C1E, false)), // East Asian ideograph
    (0x215046, (0x7C23, false)), // East Asian ideograph
    (0x215047, (0x7C3F, false)), // East Asian ideograph
    (0x215048, (0x7C3E, false)), // East Asian ideograph
    (0x215049, (0x7C38, false)), // East Asian ideograph
    (0x21504A, (0x7C37, false)), // East Asian ideograph
    (0x27504B, (0x7B7E, false)), // East Asian ideograph
    (0x21504C, (0x7C43, false)), // East Asian ideograph
    (0x23504D, (0x98BB, false)), // East Asian ideograph
    (0x23504E, (0x98C0, false)), // East Asian ideograph
    (0x21504F, (0x7C50, false)), // East Asian ideograph
    (0x215050, (0x7C60, false)), // East Asian ideograph
    (0x215051, (0x7C5F, false)), // East Asian ideograph
    (0x215052, (0x7C64, false)), // East Asian ideograph
    (0x215053, (0x7C6C, false)), // East Asian ideograph
    (0x215054, (0x7C6E, false)), // East Asian ideograph
    (0x215055, (0x7C72, false)), // East Asian ideograph
    (0x215056, (0x7C73, false)), // East Asian ideograph
    (0x215057, (0x7C89, false)), // East Asian ideograph
    (0x215058, (0x7C92, false)), // East Asian ideograph
    (0x215059, (0x7C97, false)), // East Asian ideograph
    (0x21505A, (0x7C9F, false)), // East Asian ideograph
    (0x21505B, (0x7CA5, false)), // East Asian ideograph
    (0x21505C, (0x7CA4, false)), // East Asian ideograph
    (0x21505D, (0x7CB1, false)), // East Asian ideograph
    (0x21505E, (0x7CB3, false)), // East Asian ideograph
    (0x23505F, (0x98E1, false)), // East Asian ideograph
    (0x275060, (0x7C8B, false)), // East Asian ideograph
    (0x215061, (0xFA1D, false)), // East Asian ideograph
    (0x275062, (0x80E1, false)), // East Asian ideograph (duplicate simplified)
    (0x215063, (0x7CD6, false)), // East Asian ideograph
    (0x215064, (0x7CD5, false)), // East Asian ideograph
    (0x215065, (0x7CE0, false)), // East Asian ideograph
    (0x215066, (0x7CDC, false)), // East Asian ideograph
    (0x215067, (0x7CDF, false)), // East Asian ideograph
    (0x215068, (0x7CDE, false)), // East Asian ideograph
    (0x215069, (0x7CE2, false)), // East Asian ideograph
    (0x21506A, (0x7CD9, false)), // East Asian ideograph
    (0x21506B, (0x7CE7, false)), // East Asian ideograph
    (0x21506C, (0x7CEF, false)), // East Asian ideograph
    (0x2E506D, (0x70B1, false)), // East Asian ideograph
    (0x21506E, (0x7CFB, false)), // East Asian ideograph
    (0x21506F, (0x7CFE, false)), // East Asian ideograph
    (0x215070, (0x7D00, false)), // East Asian ideograph
    (0x215071, (0x7D02, false)), // East Asian ideograph
    (0x215072, (0x7D05, false)), // East Asian ideograph
    (0x225073, (0x70A9, false)), // East Asian ideograph
    (0x215074, (0x7D04, false)), // East Asian ideograph
    (0x215075, (0x7D07, false)), // East Asian ideograph
    (0x215076, (0x7D21, false)), // East Asian ideograph
    (0x215077, (0x7D0B, false)), // East Asian ideograph
    (0x225078, (0x70EA, false)), // East Asian ideograph
    (0x215079, (0x7D20, false)), // East Asian ideograph
    (0x21507A, (0x7D1C, false)), // East Asian ideograph
    (0x21507B, (0x7D22, false)), // East Asian ideograph
    (0x27507C, (0x7EB0, false)), // East Asian ideograph
    (0x234D6A, (0x97AC, false)), // East Asian ideograph
    (0x21507E, (0x7D10, false)), // East Asian ideograph
    (0x6F5C74, (0xD587, false)), // Korean hangul
    (0x6F4D6B, (0xB514, false)), // Korean hangul
    (0x6F5831, (0xC9D3, false)), // Korean hangul
    (0x6F4D6C, (0xB515, false)), // Korean hangul
    (0x6F7641, (0xE8B3, false)), // Korean hangul
    (0x2D4D6D, (0x7792, false)), // East Asian ideograph
    (0x2D3F27, (0x6120, false)), // East Asian ideograph
    (0x69252D, (0x30AD, false)), // Katakana letter KI
    (0x6F4D6E, (0xB51B, false)), // Korean hangul
    (0x4B4C79, (0x7672, false)), // East Asian ideograph
    (0x6F4D6F, (0xB51C, false)), // Korean hangul
    (0x4C4C35, (0x6E0C, false)), // East Asian ideograph
    (0x6F5C75, (0xD588, false)), // Korean hangul
    (0x234D70, (0x97AE, false)), // East Asian ideograph
    (0x6F5832, (0xC9D5, false)), // Korean hangul
    (0x234D71, (0x97A8, false)), // East Asian ideograph
    (0x334A58, (0x89DD, false)), // East Asian ideograph
    (0x6F4D72, (0xB527, false)), // Korean hangul
    (0x4B537D, (0x9ACC, false)), // East Asian ideograph
    (0x6F592A, (0xCC3D, false)), // Korean hangul
    (0x6F5337, (0xC14B, false)), // Korean hangul
    (0x274D73, (0x4E86, false)), // East Asian ideograph
    (0x224D74, (0x6F9F, false)), // East Asian ideograph
    (0x2D325F, (0x50BB, false)), // East Asian ideograph (variant of 4B325F which maps to 50BB)
    (0x6F4D75, (0xB52A, false)), // Korean hangul
    (0x6F5833, (0xC9D6, false)), // Korean hangul
    (0x29416A, (0x948D, false)), // East Asian ideograph
    (0x22465A, (0x6C18, false)), // East Asian ideograph
    (0x2D3821, (0x962F, false)), // East Asian ideograph
    (0x6F5274, (0xC0D9, false)), // Korean hangul
    (0x213822, (0x5747, false)), // East Asian ideograph
    (0x39447D, (0x6AC2, false)), // East Asian ideograph
    (0x234D78, (0x97A5, false)), // East Asian ideograph
    (0x2D4D21, (0x7681, false)), // East Asian ideograph
    (0x6F4D79, (0xB531, false)), // Korean hangul
    (0x287360, (0x7F2C, false)), // East Asian ideograph
    (0x2F3A5E, (0x8E6E, false)), // East Asian ideograph
    (0x234D7A, (0x97B2, false)), // East Asian ideograph
    (0x2D5836, (0x89E6, false)), // East Asian ideograph
    (0x6F5834, (0xC9D9, false)), // Korean hangul
    (0x22465B, (0x6C19, false)), // East Asian ideograph
    (0x216032, (0x7AE0, false)), // East Asian ideograph
    (0x4B372C, (0x5606, false)), // East Asian ideograph
    (0x234D7C, (0x97B4, false)), // East Asian ideograph
    (0x213827, (0x5783, false)), // East Asian ideograph
    (0x224D7D, (0x6FBC, false)), // East Asian ideograph
    (0x213828, (0x576A, false)), // East Asian ideograph
    (0x235B67, (0x9DAA, false)), // East Asian ideograph
    (0x4B3773, (0x56E3, false)), // East Asian ideograph
    (0x213829, (0x5769, false)), // East Asian ideograph
    (0x34782A, (0x90C5, false)), // East Asian ideograph
    (0x284D49, (0x6DA0, false)), // East Asian ideograph
    (0x213F35, (0x6162, false)), // East Asian ideograph
    (0x21382B, (0x5761, false)), // East Asian ideograph
    (0x4B5946, (0x8A33, false)), // East Asian ideograph
    (0x21382C, (0x5764, false)), // East Asian ideograph
    (0x27383E, (0x57A9, false)), // East Asian ideograph
    (0x6F586C, (0xCB59, false)), // Korean hangul
    (0x6F543D, (0xC31C, false)), // Korean hangul
    (0x4B382E, (0x57C0, false)), // East Asian ideograph
    (0x23382F, (0x8DA1, false)), // East Asian ideograph
    (0x693C32, (0x9D2B, false)), // East Asian ideograph
    (0x215121, (0x7D17, false)), // East Asian ideograph
    (0x215122, (0x7D0D, false)), // East Asian ideograph (variant of 455122 which maps to 7D0D)
    (0x215123, (0x7D1A, false)), // East Asian ideograph
    (0x215124, (0x7D19, false)), // East Asian ideograph
    (0x215125, (0x7D1B, false)), // East Asian ideograph
    (0x215126, (0x7D46, false)), // East Asian ideograph
    (0x213831, (0x578B, false)), // East Asian ideograph
    (0x215128, (0x7D3C, false)), // East Asian ideograph
    (0x215129, (0x7D2E, false)), // East Asian ideograph
    (0x21512A, (0x7D39, false)), // East Asian ideograph
    (0x27512B, (0x7EC4, false)), // East Asian ideograph
    (0x21512C, (0x7D30, false)), // East Asian ideograph
    (0x21512D, (0x7D33, false)), // East Asian ideograph
    (0x21512E, (0x7D2F, false)), // East Asian ideograph
    (0x27512F, (0x7ECC, false)), // East Asian ideograph
    (0x275130, (0x7EC8, false)), // East Asian ideograph
    (0x275131, (0x7EDF, false)), // East Asian ideograph
    (0x275132, (0x7EDE, false)), // East Asian ideograph
    (0x215133, (0x7D68, false)), // East Asian ideograph
    (0x275134, (0x7ED3, false)), // East Asian ideograph
    (0x215135, (0x7D2B, false)), // East Asian ideograph
    (0x215136, (0x7D62, false)), // East Asian ideograph
    (0x215137, (0x7D76, false)), // East Asian ideograph
    (0x215138, (0x7D61, false)), // East Asian ideograph
    (0x275139, (0x7ED9, false)), // East Asian ideograph
    (0x21513A, (0x7D6E, false)), // East Asian ideograph
    (0x21513B, (0x7D72, false)), // East Asian ideograph
    (0x27513C, (0x7ECF, false)), // East Asian ideograph
    (0x21513D, (0x7D91, false)), // East Asian ideograph
    (0x21513E, (0x7D79, false)), // East Asian ideograph
    (0x21513F, (0x7D8F, false)), // East Asian ideograph
    (0x275140, (0x7ED1, false)), // East Asian ideograph
    (0x275141, (0x7EFC, false)), // East Asian ideograph
    (0x225142, (0x70F7, false)), // East Asian ideograph
    (0x215143, (0x7DB0, false)), // East Asian ideograph
    (0x275144, (0x7D27, false)), // East Asian ideograph
    (0x275145, (0x7EEB, false)), // East Asian ideograph
    (0x275146, (0x7F00, false)), // East Asian ideograph
    (0x215147, (0x7DBA, false)), // East Asian ideograph
    (0x275148, (0x7F51, false)), // East Asian ideograph
    (0x275149, (0x7EB2, false)), // East Asian ideograph
    (0x22514A, (0x7110, false)), // East Asian ideograph
    (0x21514B, (0x7DB5, false)), // East Asian ideograph
    (0x21514C, (0x7DA0, false)), // East Asian ideograph
    (0x27514D, (0x7EF8, false)), // East Asian ideograph
    (0x27514E, (0x7EF4, false)), // East Asian ideograph
    (0x27514F, (0x7EF5, false)), // East Asian ideograph
    (0x275150, (0x7EB6, false)), // East Asian ideograph
    (0x275151, (0x7F01, false)), // East Asian ideograph
    (0x215152, (0x7DE0, false)), // East Asian ideograph
    (0x235153, (0x9933, false)), // East Asian ideograph
    (0x235154, (0x9942, false)), // East Asian ideograph (variant of 4D5154 which maps to 9942)
    (0x275155, (0x7EEA, false)), // East Asian ideograph
    (0x215156, (0x7DD8, false)), // East Asian ideograph
    (0x275157, (0x7F05, false)), // East Asian ideograph
    (0x275158, (0x7F09, false)), // East Asian ideograph
    (0x275159, (0x7F13, false)), // East Asian ideograph
    (0x27515A, (0x7F18, false)), // East Asian ideograph
    (0x21515B, (0x7DE8, false)), // East Asian ideograph
    (0x21515C, (0x7DDA, false)), // East Asian ideograph
    (0x27515D, (0x7F0D, false)), // East Asian ideograph
    (0x27515E, (0x7F0E, false)), // East Asian ideograph
    (0x27515F, (0x7F23, false)), // East Asian ideograph
    (0x275160, (0x7F22, false)), // East Asian ideograph
    (0x275161, (0x8426, false)), // East Asian ideograph
    (0x275162, (0x7F1A, false)), // East Asian ideograph
    (0x215163, (0x7DFB, false)), // East Asian ideograph
    (0x275164, (0x53BF, false)), // East Asian ideograph (variant of 455164 which maps to 53BF)
    (0x215165, (0x7E2E, false)), // East Asian ideograph
    (0x215166, (0x7E3E, false)), // East Asian ideograph
    (0x275167, (0x7F2A, false)), // East Asian ideograph
    (0x275168, (0x7F15, false)), // East Asian ideograph
    (0x215169, (0x7E32, false)), // East Asian ideograph
    (0x23516A, (0x9948, false)), // East Asian ideograph
    (0x21516B, (0x7E41, false)), // East Asian ideograph
    (0x23516C, (0x9947, false)), // East Asian ideograph
    (0x23516D, (0x9949, false)), // East Asian ideograph
    (0x21516E, (0x7E31, false)), // East Asian ideograph
    (0x22516F, (0x713F, false)), // East Asian ideograph
    (0x235170, (0x9943, false)), // East Asian ideograph
    (0x275171, (0x7EC7, false)), // East Asian ideograph
    (0x215172, (0x7E61, false)), // East Asian ideograph
    (0x235173, (0x994E, false)), // East Asian ideograph
    (0x235174, (0x9950, false)), // East Asian ideograph
    (0x215175, (0x7E6B, false)), // East Asian ideograph
    (0x215176, (0x7E69, false)), // East Asian ideograph
    (0x215177, (0x7E6D, false)), // East Asian ideograph
    (0x215178, (0x7E79, false)), // East Asian ideograph
    (0x215179, (0x7E6A, false)), // East Asian ideograph
    (0x21517A, (0x8FAE, false)), // East Asian ideograph
    (0x27517B, (0x7F24, false)), // East Asian ideograph
    (0x21517C, (0x7E82, false)), // East Asian ideograph
    (0x21517D, (0x7E7C, false)), // East Asian ideograph
    (0x21517E, (0x7E8F, false)), // East Asian ideograph
    (0x6F5947, (0xCCE4, false)), // Korean hangul
    (0x227841, (0x80EF, false)), // East Asian ideograph
    (0x217144, (0x5581, false)), // East Asian ideograph
    (0x4B3774, (0x56F3, false)), // East Asian ideograph
    (0x235729, (0x9B8E, false)), // East Asian ideograph
    (0x287321, (0x7F26, false)), // East Asian ideograph
    (0x6F5C7D, (0xD5D2, false)), // Korean hangul
    (0x6F583A, (0xC9E0, false)), // Korean hangul
    (0x216038, (0x9802, false)), // East Asian ideograph
    (0x213844, (0x5831, false)), // East Asian ideograph
    (0x4B594B, (0x5909, false)), // East Asian ideograph
    (0x6F5D72, (0xD763, false)), // Korean hangul
    (0x4C3B31, (0x6798, false)), // East Asian ideograph
    (0x213845, (0x582F, false)), // East Asian ideograph
    (0x213A30, (0x5ABC, false)), // East Asian ideograph
    (0x6F5C7E, (0xD5D8, false)), // Korean hangul
    (0x213848, (0x5830, false)), // East Asian ideograph
    (0x274638, (0x6B7C, false)), // East Asian ideograph
    (0x213849, (0x5824, false)), // East Asian ideograph
    (0x21384A, (0x5834, false)), // East Asian ideograph
    (0x21784B, (0x58C6, false)), // East Asian ideograph
    (0x394042, (0x646D, false)), // East Asian ideograph
    (0x284B28, (0x6D48, false)), // East Asian ideograph
    (0x22384C, (0x665E, false)), // East Asian ideograph
    (0x69525D, (0x53FA, false)), // East Asian ideograph
    (0x705D46, (0x841C, false)), // East Asian ideograph
    (0x27384D, (0x6D82, false)), // East Asian ideograph
    (0x21603A, (0x9805, false)), // East Asian ideograph
    (0x234A62, (0x967E, false)), // East Asian ideograph
    (0x213158, (0x4FD1, false)), // East Asian ideograph
    (0x223850, (0x667E, false)), // East Asian ideograph
    (0x21387C, (0x5919, false)), // East Asian ideograph
    (0x213851, (0x584C, false)), // East Asian ideograph
    (0x213852, (0x585A, false)), // East Asian ideograph
    (0x213853, (0x586D, false)), // East Asian ideograph
    (0x6F5276, (0xC0DC, false)), // Korean hangul
    (0x217854, (0x58D2, false)), // East Asian ideograph
    (0x213855, (0x5862, false)), // East Asian ideograph
    (0x335B70, (0x5EF8, false)), // East Asian ideograph
    (0x213F4E, (0x61C7, false)), // East Asian ideograph
    (0x273856, (0x5757, false)), // East Asian ideograph
    (0x6F4E33, (0xB5A8, false)), // Korean hangul
    (0x6F583E, (0xC9E7, false)), // Korean hangul
    (0x22687E, (0x7A2D, false)), // East Asian ideograph
    (0x4B3B79, (0x5D8C, false)), // East Asian ideograph
    (0x217428, (0x5704, false)), // East Asian ideograph
    (0x334621, (0x8B99, false)), // East Asian ideograph
    (0x233859, (0x8DAF, false)), // East Asian ideograph
    (0x21385A, (0x588A, false)), // East Asian ideograph
    (0x215221, (0x7E8C, false)), // East Asian ideograph
    (0x275222, (0x7F28, false)), // East Asian ideograph
    (0x215223, (0x7E96, false)), // East Asian ideograph
    (0x215224, (0x7E9C, false)), // East Asian ideograph
    (0x6F5225, (0xBE75, false)), // Korean hangul
    (0x215226, (0x7F38, false)), // East Asian ideograph
    (0x215227, (0x7F3A, false)), // East Asian ideograph
    (0x225228, (0x7135, false)), // East Asian ideograph
    (0x235229, (0x995D, false)), // East Asian ideograph
    (0x6F522A, (0xBE84, false)), // Korean hangul
    (0x21522B, (0x7F50, false)), // East Asian ideograph
    (0x21522C, (0x7F55, false)), // East Asian ideograph
    (0x21522D, (0x7F54, false)), // East Asian ideograph
    (0x21522E, (0x7F5F, false)), // East Asian ideograph
    (0x21522F, (0x7F72, false)), // East Asian ideograph
    (0x215230, (0x7F6E, false)), // East Asian ideograph
    (0x224223, (0x6A89, false)), // East Asian ideograph
    (0x215232, (0x7F6A, false)), // East Asian ideograph
    (0x215233, (0x7F70, false)), // East Asian ideograph
    (0x215234, (0x7F75, false)), // East Asian ideograph
    (0x215235, (0x7F77, false)), // East Asian ideograph
    (0x215236, (0x7F79, false)), // East Asian ideograph
    (0x215237, (0x7F85, false)), // East Asian ideograph
    (0x275238, (0x7F81, false)), // East Asian ideograph
    (0x215239, (0x7F8A, false)), // East Asian ideograph
    (0x21523A, (0x7F8C, false)), // East Asian ideograph
    (0x21523B, (0x7F8E, false)), // East Asian ideograph
    (0x21523C, (0x7F94, false)), // East Asian ideograph
    (0x21523D, (0x7F9E, false)), // East Asian ideograph
    (0x21523E, (0x7F9A, false)), // East Asian ideograph
    (0x21523F, (0x5584, false)), // East Asian ideograph
    (0x215240, (0x7FA8, false)), // East Asian ideograph
    (0x215241, (0x7FA4, false)), // East Asian ideograph
    (0x235242, (0x99AA, false)), // East Asian ideograph
    (0x215243, (0x7FAF, false)), // East Asian ideograph
    (0x215244, (0x7FB2, false)), // East Asian ideograph
    (0x215245, (0x7FB6, false)), // East Asian ideograph
    (0x215246, (0x7FB8, false)), // East Asian ideograph
    (0x215247, (0x7FB9, false)), // East Asian ideograph
    (0x215248, (0x7FBD, false)), // East Asian ideograph
    (0x235249, (0x99B5, false)), // East Asian ideograph
    (0x21524A, (0x7FC5, false)), // East Asian ideograph
    (0x21524B, (0x7FC1, false)), // East Asian ideograph
    (0x21524C, (0x7FCC, false)), // East Asian ideograph
    (0x21524D, (0x7FD2, false)), // East Asian ideograph
    (0x21524E, (0x7FCE, false)), // East Asian ideograph (variant of 4B524E which maps to 7FCE)
    (0x21524F, (0x7FD4, false)), // East Asian ideograph
    (0x215250, (0x7FD5, false)), // East Asian ideograph
    (0x215251, (0x7FE0, false)), // East Asian ideograph
    (0x215252, (0x7FE1, false)), // East Asian ideograph
    (0x215253, (0x7FDF, false)), // East Asian ideograph
    (0x235254, (0x99BD, false)), // East Asian ideograph
    (0x215255, (0x7FF0, false)), // East Asian ideograph
    (0x215256, (0x7FF3, false)), // East Asian ideograph
    (0x215257, (0x7FFC, false)), // East Asian ideograph
    (0x215258, (0x7FF9, false)), // East Asian ideograph
    (0x215259, (0x7FFB, false)), // East Asian ideograph
    (0x21525A, (0x7FF1, false)), // East Asian ideograph
    (0x21525B, (0x8000, false)), // East Asian ideograph
    (0x22525C, (0x7143, false)), // East Asian ideograph
    (0x21525D, (0x8003, false)), // East Asian ideograph
    (0x21525E, (0x8006, false)), // East Asian ideograph
    (0x21525F, (0x8005, false)), // East Asian ideograph
    (0x215260, (0x800C, false)), // East Asian ideograph
    (0x215261, (0x8010, false)), // East Asian ideograph
    (0x215262, (0x800D, false)), // East Asian ideograph
    (0x215263, (0x8012, false)), // East Asian ideograph
    (0x215264, (0x8015, false)), // East Asian ideograph
    (0x215265, (0x8018, false)), // East Asian ideograph
    (0x215266, (0x8019, false)), // East Asian ideograph
    (0x215267, (0x8017, false)), // East Asian ideograph
    (0x215268, (0x801C, false)), // East Asian ideograph
    (0x235269, (0x99D8, false)), // East Asian ideograph
    (0x21526A, (0x8036, false)), // East Asian ideograph
    (0x21526B, (0x803F, false)), // East Asian ideograph
    (0x21526C, (0x803D, false)), // East Asian ideograph
    (0x21526D, (0x804A, false)), // East Asian ideograph
    (0x21526E, (0x8046, false)), // East Asian ideograph
    (0x21526F, (0x8056, false)), // East Asian ideograph
    (0x215270, (0x8058, false)), // East Asian ideograph
    (0x215271, (0x805E, false)), // East Asian ideograph
    (0x215272, (0x805A, false)), // East Asian ideograph
    (0x215273, (0x8071, false)), // East Asian ideograph
    (0x215274, (0x8072, false)), // East Asian ideograph
    (0x215275, (0x8073, false)), // East Asian ideograph
    (0x215276, (0x8070, false)), // East Asian ideograph
    (0x215277, (0x806F, false)), // East Asian ideograph
    (0x215278, (0x8077, false)), // East Asian ideograph
    (0x275279, (0x8042, false)), // East Asian ideograph
    (0x23527A, (0x99F0, false)), // East Asian ideograph
    (0x27527B, (0x542C, false)), // East Asian ideograph
    (0x21527C, (0x807F, false)), // East Asian ideograph
    (0x6F527C, (0xC0F4, false)), // Korean hangul
    (0x21527E, (0x8084, false)), // East Asian ideograph
    (0x21386B, (0x58D9, false)), // East Asian ideograph
    (0x6F5842, (0xC9F0, false)), // Korean hangul
    (0x27386C, (0x5792, false)), // East Asian ideograph
    (0x6F5A23, (0xCEAD, false)), // Korean hangul
    (0x21386D, (0x58DF, false)), // East Asian ideograph
    (0x27386E, (0x574F, false)), // East Asian ideograph
    (0x4B3850, (0x5861, false)), // East Asian ideograph (variant of 213850)
    (0x23395C, (0x8E1E, false)), // East Asian ideograph
    (0x235732, (0x9B98, false)), // East Asian ideograph
    (0x6F4E34, (0xB5AB, false)), // Korean hangul
    (0x213870, (0x58E4, false)), // East Asian ideograph
    (0x276065, (0x9965, false)), // East Asian ideograph
    (0x4B3B7E, (0x5D15, false)), // East Asian ideograph
    (0x273871, (0x575D, false)), // East Asian ideograph
    (0x6F5D76, (0xD770, false)), // Korean hangul
    (0x223872, (0x6693, false)), // East Asian ideograph
    (0x233873, (0x8DBF, false)), // East Asian ideograph
    (0x273874, (0x58EE, false)), // East Asian ideograph
    (0x343875, (0x5FDE, false)), // East Asian ideograph
    (0x284D58, (0x6CA9, false)), // East Asian ideograph
    (0x705C43, (0x82CA, false)), // East Asian ideograph
    (0x223876, (0x6690, false)), // East Asian ideograph
    (0x276321, (0x9F7F, false)), // East Asian ideograph
    (0x213877, (0x58FD, false)), // East Asian ideograph
    (0x276322, (0x9F83, false)), // East Asian ideograph
    (0x4D5D49, (0x9E81, false)), // East Asian ideograph
    (0x226323, (0x77B6, false)), // East Asian ideograph
    (0x213879, (0x5914, false)), // East Asian ideograph
    (0x276324, (0x9F84, false)), // East Asian ideograph
    (0x2E765F, (0x8037, false)), // East Asian ideograph
    (0x21387A, (0x5915, false)), // East Asian ideograph
    (0x284D59, (0x6ED7, false)), // East Asian ideograph
    (0x276325, (0x9F88, false)), // East Asian ideograph
    (0x6F542C, (0xC2F1, false)), // Korean hangul
    (0x213E2A, (0x6035, false)), // East Asian ideograph (variant of 4B3E2A which maps to 6035)
    (0x6F5675, (0xC78A, false)), // Korean hangul
    (0x276326, (0x9F87, false)), // East Asian ideograph
    (0x22787C, (0x8153, false)), // East Asian ideograph
    (0x216327, (0x9F6C, false)), // East Asian ideograph
    (0x21387D, (0x591A, false)), // East Asian ideograph
    (0x276328, (0x9F8A, false)), // East Asian ideograph
    (0x2D517D, (0x7D99, false)), // East Asian ideograph
    (0x4B484A, (0x6E13, false)), // East Asian ideograph
    (0x2D3272, (0x706E, false)), // East Asian ideograph
    (0x232329, (0x845C, false)), // East Asian ideograph
    (0x6F5846, (0xC9FC, false)), // Korean hangul
    (0x27632A, (0x9F8B, false)), // East Asian ideograph
    (0x234A6C, (0x9689, false)), // East Asian ideograph
    (0x22632B, (0x77B9, false)), // East Asian ideograph
    (0x453051, (0x8D30, false)), // East Asian ideograph
    (0x6F4B7D, (0xB124, false)), // Korean hangul
    (0x21632C, (0x9F94, false)), // East Asian ideograph
    (0x27632D, (0x9F9F, false)), // East Asian ideograph
    (0x4B484B, (0x51D6, false)), // East Asian ideograph
    (0x235736, (0x9B9F, false)), // East Asian ideograph
    (0x6F5847, (0xCA00, false)), // Korean hangul
    (0x6F5427, (0xC2E4, false)), // Korean hangul
    (0x275321, (0x8083, false)), // East Asian ideograph
    (0x215322, (0x8087, false)), // East Asian ideograph
    (0x215323, (0x8089, false)), // East Asian ideograph
    (0x235324, (0x9A02, false)), // East Asian ideograph
    (0x215325, (0x808C, false)), // East Asian ideograph
    (0x215326, (0x8093, false)), // East Asian ideograph
    (0x215327, (0x809D, false)), // East Asian ideograph
    (0x215328, (0x8098, false)), // East Asian ideograph
    (0x215329, (0x809B, false)), // East Asian ideograph
    (0x21532A, (0x809A, false)), // East Asian ideograph
    (0x22532B, (0x7180, false)), // East Asian ideograph
    (0x22532C, (0x7189, false)), // East Asian ideograph
    (0x21532D, (0x80AA, false)), // East Asian ideograph
    (0x21532E, (0x80BA, false)), // East Asian ideograph
    (0x21532F, (0x80A5, false)), // East Asian ideograph
    (0x235330, (0x99FB, false)), // East Asian ideograph
    (0x235331, (0x99FD, false)), // East Asian ideograph
    (0x215332, (0x80B1, false)), // East Asian ideograph
    (0x225333, (0x7196, false)), // East Asian ideograph
    (0x215334, (0x80A1, false)), // East Asian ideograph
    (0x215335, (0x80A9, false)), // East Asian ideograph
    (0x27495D, (0x4E4C, false)), // East Asian ideograph
    (0x215337, (0x80D6, false)), // East Asian ideograph
    (0x215338, (0x80CC, false)), // East Asian ideograph
    (0x215339, (0x80E5, false)), // East Asian ideograph
    (0x21533A, (0x80DA, false)), // East Asian ideograph
    (0x21533B, (0x80E1, false)), // East Asian ideograph
    (0x21533C, (0x80C3, false)), // East Asian ideograph
    (0x21533D, (0x80DB, false)), // East Asian ideograph
    (0x21533E, (0x80C4, false)), // East Asian ideograph
    (0x21533F, (0x80CE, false)), // East Asian ideograph
    (0x215340, (0x80DE, false)), // East Asian ideograph
    (0x215341, (0x80E4, false)), // East Asian ideograph
    (0x215342, (0x80F0, false)), // East Asian ideograph
    (0x215343, (0x8102, false)), // East Asian ideograph
    (0x215344, (0x8105, false)), // East Asian ideograph
    (0x215345, (0x80F1, false)), // East Asian ideograph
    (0x215346, (0x80F4, false)), // East Asian ideograph
    (0x215347, (0x80ED, false)), // East Asian ideograph
    (0x235348, (0x9A10, false)), // East Asian ideograph
    (0x215349, (0x8106, false)), // East Asian ideograph
    (0x21534A, (0x80F3, false)), // East Asian ideograph
    (0x21534B, (0x80F8, false)), // East Asian ideograph
    (0x23534C, (0x9A24, false)), // East Asian ideograph
    (0x21534D, (0x8108, false)), // East Asian ideograph
    (0x21534E, (0x812B, false)), // East Asian ideograph
    (0x21534F, (0x812F, false)), // East Asian ideograph
    (0x215350, (0x8116, false)), // East Asian ideograph
    (0x225351, (0x71A4, false)), // East Asian ideograph
    (0x215352, (0x8129, false)), // East Asian ideograph
    (0x215353, (0x8155, false)), // East Asian ideograph
    (0x215354, (0x8154, false)), // East Asian ideograph
    (0x215355, (0x814B, false)), // East Asian ideograph
    (0x215356, (0x8151, false)), // East Asian ideograph
    (0x215357, (0x8150, false)), // East Asian ideograph
    (0x215358, (0x814E, false)), // East Asian ideograph
    (0x275359, (0x80C0, false)), // East Asian ideograph
    (0x21535A, (0x8146, false)), // East Asian ideograph
    (0x21535B, (0x813E, false)), // East Asian ideograph
    (0x21535C, (0x8171, false)), // East Asian ideograph
    (0x21535D, (0x8170, false)), // East Asian ideograph
    (0x21535E, (0x8178, false)), // East Asian ideograph
    (0x21535F, (0x8165, false)), // East Asian ideograph
    (0x215360, (0x816E, false)), // East Asian ideograph
    (0x215361, (0x8173, false)), // East Asian ideograph
    (0x275362, (0x80BF, false)), // East Asian ideograph
    (0x215363, (0x8179, false)), // East Asian ideograph
    (0x215364, (0x817A, false)), // East Asian ideograph
    (0x215365, (0x8166, false)), // East Asian ideograph
    (0x215366, (0x8180, false)), // East Asian ideograph
    (0x225367, (0x71D1, false)), // East Asian ideograph
    (0x215368, (0x817F, false)), // East Asian ideograph
    (0x215369, (0x818A, false)), // East Asian ideograph
    (0x21536A, (0x8188, false)), // East Asian ideograph
    (0x21536B, (0x819D, false)), // East Asian ideograph
    (0x21536C, (0x81A0, false)), // East Asian ideograph
    (0x22536D, (0x71CA, false)), // East Asian ideograph
    (0x21536E, (0x819A, false)), // East Asian ideograph
    (0x21536F, (0x819C, false)), // East Asian ideograph
    (0x215370, (0x81B3, false)), // East Asian ideograph
    (0x275371, (0x817B, false)), // East Asian ideograph
    (0x215372, (0x81A8, false)), // East Asian ideograph
    (0x215373, (0x81C6, false)), // East Asian ideograph
    (0x215374, (0x81BA, false)), // East Asian ideograph
    (0x215375, (0x81C3, false)), // East Asian ideograph
    (0x215376, (0x81C0, false)), // East Asian ideograph
    (0x215377, (0x81C2, false)), // East Asian ideograph
    (0x275378, (0x8113, false)), // East Asian ideograph
    (0x275379, (0x80C6, false)), // East Asian ideograph
    (0x27537A, (0x8138, false)), // East Asian ideograph
    (0x27537B, (0x810D, false)), // East Asian ideograph
    (0x21537C, (0x81CD, false)), // East Asian ideograph
    (0x27537D, (0x8191, false)), // East Asian ideograph
    (0x27537E, (0x814A, false)), // East Asian ideograph
    (0x234156, (0x91BF, false)), // East Asian ideograph
    (0x276B79, (0x523F, false)), // East Asian ideograph
    (0x6F584B, (0xCA0C, false)), // Korean hangul
    (0x2D3B3F, (0x5C02, false)), // East Asian ideograph
    (0x21313B, (0x4F43, false)), // East Asian ideograph
    (0x2D615A, (0x8EC6, false)), // East Asian ideograph
    (0x6F2526, (0x3161, false)), // Korean hangul
    (0x276B7A, (0x523D, false)), // East Asian ideograph
    (0x6F584C, (0xCA0D, false)), // Korean hangul
    (0x69543A, (0x57AA, false)), // East Asian ideograph
    (0x232349, (0x8497, false)), // East Asian ideograph
    (0x6F5279, (0xC0E5, false)), // Korean hangul
    (0x274C33, (0x6BD5, false)), // East Asian ideograph
    (0x213168, (0x4FC4, false)), // East Asian ideograph
    (0x22234B, (0x5C8D, false)), // East Asian ideograph
    (0x213F51, (0x61E3, false)), // East Asian ideograph
    (0x2D3279, (0x514E, false)), // East Asian ideograph
    (0x6F4E36, (0xB5B1, false)), // Korean hangul
    (0x6F2471, (0x3149, false)), // Korean hangul
    (0x6F584D, (0xCA18, false)), // Korean hangul
    (0x224674, (0x6C3F, false)), // East Asian ideograph
    (0x6F4929, (0xAC80, false)), // Korean hangul
    (0x6F5164, (0xBDD4, false)), // Korean hangul
    (0x217E21, (0x5B5B, false)), // East Asian ideograph
    (0x213169, (0x4FC2, false)), // East Asian ideograph
    (0x217158, (0x55CC, false)), // East Asian ideograph
    (0x233967, (0x8E27, false)), // East Asian ideograph
    (0x4B594A, (0x8AAD, false)), // East Asian ideograph
    (0x6F5158, (0xBD80, false)), // Korean hangul
    (0x6F584E, (0xCA4C, false)), // Korean hangul
    (0x6F7721, (0xAD35, false)), // Korean hangul
    (0x213E33, (0x6025, false)), // East Asian ideograph
    (0x295A28, (0x9E28, false)), // East Asian ideograph
    (0x2D4B72, (0x7506, false)), // East Asian ideograph
    (0x6F584F, (0xCA4D, false)), // Korean hangul
    (0x232358, (0x84B9, false)), // East Asian ideograph
    (0x347431, (0x58DC, false)), // East Asian ideograph
    (0x21715A, (0x55DB, false)), // East Asian ideograph
    (0x233969, (0x8E18, false)), // East Asian ideograph
    (0x215421, (0x81DA, false)), // East Asian ideograph
    (0x235422, (0x9A4D, false)), // East Asian ideograph
    (0x215423, (0x81E3, false)), // East Asian ideograph
    (0x235424, (0x9A52, false)), // East Asian ideograph
    (0x275425, (0x4E34, false)), // East Asian ideograph
    (0x215426, (0x81EA, false)), // East Asian ideograph
    (0x215427, (0x81EC, false)), // East Asian ideograph
    (0x215428, (0x81ED, false)), // East Asian ideograph
    (0x215429, (0x81F3, false)), // East Asian ideograph
    (0x22542A, (0x71DE, false)), // East Asian ideograph
    (0x21542B, (0x81FA, false)), // East Asian ideograph
    (0x21542C, (0x81FB, false)), // East Asian ideograph
    (0x21542D, (0x81FC, false)), // East Asian ideograph
    (0x21542E, (0x81FE, false)), // East Asian ideograph
    (0x21542F, (0x8200, false)), // East Asian ideograph
    (0x215430, (0x8202, false)), // East Asian ideograph
    (0x215431, (0x8205, false)), // East Asian ideograph
    (0x215432, (0x8207, false)), // East Asian ideograph
    (0x275433, (0x5174, false)), // East Asian ideograph
    (0x275434, (0x4E3E, false)), // East Asian ideograph
    (0x215435, (0x820A, false)), // East Asian ideograph
    (0x215436, (0x820C, false)), // East Asian ideograph
    (0x215437, (0x820D, false)), // East Asian ideograph
    (0x215438, (0x8210, false)), // East Asian ideograph
    (0x215439, (0x8212, false)), // East Asian ideograph
    (0x23543A, (0x9A6B, false)), // East Asian ideograph
    (0x21543B, (0x821B, false)), // East Asian ideograph
    (0x21543C, (0x821C, false)), // East Asian ideograph
    (0x21543D, (0x821E, false)), // East Asian ideograph
    (0x21543E, (0x821F, false)), // East Asian ideograph
    (0x21543F, (0x8222, false)), // East Asian ideograph
    (0x215440, (0x822A, false)), // East Asian ideograph
    (0x235441, (0x9AAB, false)), // East Asian ideograph
    (0x215442, (0x822C, false)), // East Asian ideograph
    (0x215443, (0x8228, false)), // East Asian ideograph
    (0x215444, (0x8237, false)), // East Asian ideograph
    (0x215445, (0x8235, false)), // East Asian ideograph
    (0x215446, (0x8239, false)), // East Asian ideograph
    (0x215447, (0x8236, false)), // East Asian ideograph
    (0x215448, (0x8247, false)), // East Asian ideograph
    (0x215449, (0x8258, false)), // East Asian ideograph
    (0x21544A, (0x8259, false)), // East Asian ideograph
    (0x21544B, (0x8266, false)), // East Asian ideograph
    (0x21544C, (0x826E, false)), // East Asian ideograph
    (0x21544D, (0x826F, false)), // East Asian ideograph
    (0x21544E, (0x8271, false)), // East Asian ideograph
    (0x21544F, (0x8272, false)), // East Asian ideograph
    (0x215450, (0x827E, false)), // East Asian ideograph
    (0x215451, (0x8292, false)), // East Asian ideograph
    (0x215452, (0x828B, false)), // East Asian ideograph
    (0x215453, (0x828D, false)), // East Asian ideograph
    (0x215454, (0x82B3, false)), // East Asian ideograph
    (0x215455, (0x829D, false)), // East Asian ideograph
    (0x215456, (0x8299, false)), // East Asian ideograph
    (0x215457, (0x82BD, false)), // East Asian ideograph
    (0x215458, (0x82AD, false)), // East Asian ideograph
    (0x215459, (0x82AC, false)), // East Asian ideograph
    (0x21545A, (0x82A5, false)), // East Asian ideograph
    (0x21545B, (0x829F, false)), // East Asian ideograph
    (0x27545C, (0x520D, false)), // East Asian ideograph
    (0x21545D, (0x82B1, false)), // East Asian ideograph
    (0x21545E, (0x82B9, false)), // East Asian ideograph
    (0x69545F, (0x58E5, false)), // East Asian ideograph
    (0x215460, (0x82E7, false)), // East Asian ideograph
    (0x215461, (0x8305, false)), // East Asian ideograph
    (0x215462, (0x8309, false)), // East Asian ideograph
    (0x215463, (0x82E3, false)), // East Asian ideograph
    (0x215464, (0x82DB, false)), // East Asian ideograph
    (0x215465, (0x82E6, false)), // East Asian ideograph
    (0x215466, (0x8304, false)), // East Asian ideograph
    (0x215467, (0x82E5, false)), // East Asian ideograph
    (0x215468, (0x8302, false)), // East Asian ideograph
    (0x215469, (0x82DC, false)), // East Asian ideograph
    (0x21546A, (0x82D7, false)), // East Asian ideograph
    (0x21546B, (0x82F1, false)), // East Asian ideograph
    (0x21546C, (0x8301, false)), // East Asian ideograph
    (0x23546D, (0x9AD6, false)), // East Asian ideograph
    (0x21546E, (0x82D4, false)), // East Asian ideograph
    (0x21546F, (0x82D1, false)), // East Asian ideograph
    (0x215470, (0x82DE, false)), // East Asian ideograph
    (0x215471, (0x82DF, false)), // East Asian ideograph
    (0x215472, (0x832B, false)), // East Asian ideograph
    (0x215473, (0x8352, false)), // East Asian ideograph
    (0x235474, (0x9ADF, false)), // East Asian ideograph
    (0x215475, (0x8338, false)), // East Asian ideograph
    (0x215476, (0x8354, false)), // East Asian ideograph
    (0x235477, (0x9AE2, false)), // East Asian ideograph
    (0x215478, (0x8349, false)), // East Asian ideograph
    (0x215479, (0x8335, false)), // East Asian ideograph
    (0x21547A, (0x8334, false)), // East Asian ideograph
    (0x21547B, (0x8336, false)), // East Asian ideograph
    (0x21547C, (0x8331, false)), // East Asian ideograph
    (0x21547D, (0x8340, false)), // East Asian ideograph
    (0x21547E, (0x8317, false)), // East Asian ideograph
    (0x6F5853, (0xCA5D, false)), // Korean hangul
    (0x295166, (0x9969, false)), // East Asian ideograph
    (0x234A79, (0x9696, false)), // East Asian ideograph
    (0x226450, (0x7826, false)), // East Asian ideograph
    (0x6F5D73, (0xD765, false)), // Korean hangul
    (0x6F4B35, (0xB014, false)), // Korean hangul
    (0x2D6162, (0x9A0C, false)), // East Asian ideograph
    (0x6F5872, (0xCBE7, false)), // Korean hangul
    (0x21316F, (0x4FEF, false)), // East Asian ideograph
    (0x4B4858, (0x6E80, false)), // East Asian ideograph
    (0x6F5854, (0xCA61, false)), // Korean hangul
    (0x222370, (0x5CD5, false)), // East Asian ideograph
    (0x22467B, (0x6C62, false)), // East Asian ideograph
    (0x213E39, (0x6063, false)), // East Asian ideograph
    (0x6F7648, (0xE8BA, false)), // Korean hangul
    (0x4B374C, (0x5662, false)), // East Asian ideograph
    (0x29594F, (0x9CE2, false)), // East Asian ideograph
    (0x696373, (0x7B02, false)), // East Asian ideograph
    (0x295A70, (0x9E48, false)), // East Asian ideograph
    (0x6F5364, (0xC22F, false)), // Korean hangul
    (0x27496A, (0x70BC, false)), // East Asian ideograph
    (0x6F5855, (0xCA84, false)), // Korean hangul
    (0x292375, (0x83B3, false)), // East Asian ideograph
    (0x22467C, (0x6C4A, false)), // East Asian ideograph
    (0x224E21, (0x6FAA, false)), // East Asian ideograph
    (0x6F4E22, (0xB541, false)), // Korean hangul
    (0x6F4E23, (0xB543, false)), // Korean hangul
    (0x225346, (0x719E, false)), // East Asian ideograph
    (0x222379, (0x5C8D, false)), // East Asian ideograph (not in Unicode)
    (0x234E24, (0x97B3, false)), // East Asian ideograph
    (0x6F5856, (0xCA98, false)), // Korean hangul
    (0x224E25, (0x6FBF, false)), // East Asian ideograph
    (0x6F527B, (0xC0EC, false)), // Korean hangul
    (0x224E26, (0x6FC7, false)), // East Asian ideograph
    (0x275A78, (0x8E52, false)), // East Asian ideograph
    (0x274E27, (0x77EB, false)), // East Asian ideograph
    (0x213172, (0x5025, false)), // East Asian ideograph
    (0x6F4E28, (0xB54D, false)), // Korean hangul
    (0x27574A, (0x51B2, false)), // East Asian ideograph (duplicate simplified)
    (0x227E23, (0x83A6, false)), // East Asian ideograph
    (0x234E29, (0x97B9, false)), // East Asian ideograph
    (0x6F5857, (0xCABC, false)), // Korean hangul
    (0x29516A, (0x9990, false)), // East Asian ideograph
    (0x51356A, (0x8BC3, false)), // East Asian ideograph
    (0x6F4E2B, (0xB55C, false)), // Korean hangul
    (0x6F594D, (0xCD19, false)), // Korean hangul
    (0x6F5D5C, (0xD700, false)), // Korean hangul
    (0x6F4E2C, (0xB55D, false)), // Korean hangul
    (0x213173, (0x5011, false)), // East Asian ideograph
    (0x4B613F, (0x9A08, false)), // East Asian ideograph
    (0x214E2D, (0x65AB, false)), // East Asian ideograph
    (0x224E2E, (0x6F5E, false)), // East Asian ideograph
    (0x6F5858, (0xCABD, false)), // Korean hangul
    (0x6F546A, (0xC4F0, false)), // Korean hangul
    (0x224E2F, (0x6FC8, false)), // East Asian ideograph
    (0x2D5763, (0x88E1, false)), // East Asian ideograph
    (0x235521, (0x9AE7, false)), // East Asian ideograph
    (0x215522, (0x834F, false)), // East Asian ideograph
    (0x215523, (0x8339, false)), // East Asian ideograph
    (0x215524, (0x838E, false)), // East Asian ideograph
    (0x215525, (0x8398, false)), // East Asian ideograph
    (0x215526, (0x839E, false)), // East Asian ideograph
    (0x215527, (0x8378, false)), // East Asian ideograph
    (0x215528, (0x83A2, false)), // East Asian ideograph
    (0x225529, (0x7225, false)), // East Asian ideograph
    (0x22552A, (0x7226, false)), // East Asian ideograph
    (0x21552B, (0x83AB, false)), // East Asian ideograph
    (0x21552C, (0x8392, false)), // East Asian ideograph (variant of 4B552C which maps to 8392)
    (0x21552D, (0x838A, false)), // East Asian ideograph
    (0x21552E, (0x8393, false)), // East Asian ideograph
    (0x21552F, (0x83A0, false)), // East Asian ideograph
    (0x215530, (0x8389, false)), // East Asian ideograph
    (0x215531, (0x8377, false)), // East Asian ideograph
    (0x215532, (0x837C, false)), // East Asian ideograph
    (0x215533, (0x837B, false)), // East Asian ideograph
    (0x215534, (0x840D, false)), // East Asian ideograph
    (0x215535, (0x83E0, false)), // East Asian ideograph
    (0x215536, (0x83E9, false)), // East Asian ideograph
    (0x6F5537, (0xC57D, false)), // Korean hangul
    (0x215538, (0x8403, false)), // East Asian ideograph
    (0x215539, (0x83C5, false)), // East Asian ideograph
    (0x21553A, (0x83C1, false)), // East Asian ideograph
    (0x21553B, (0x840B, false)), // East Asian ideograph
    (0x21553C, (0x83EF, false)), // East Asian ideograph
    (0x6F553D, (0xC58F, false)), // Korean hangul
    (0x21553E, (0x83F1, false)), // East Asian ideograph
    (0x21553F, (0x83BD, false)), // East Asian ideograph
    (0x6F5540, (0xC595, false)), // Korean hangul
    (0x235541, (0x9B05, false)), // East Asian ideograph
    (0x215542, (0x840C, false)), // East Asian ideograph
    (0x225543, (0x7241, false)), // East Asian ideograph
    (0x215544, (0x83DC, false)), // East Asian ideograph
    (0x215545, (0x83CA, false)), // East Asian ideograph
    (0x215546, (0x83F2, false)), // East Asian ideograph
    (0x215547, (0x840E, false)), // East Asian ideograph
    (0x215548, (0x8404, false)), // East Asian ideograph
    (0x215549, (0x843D, false)), // East Asian ideograph
    (0x21554A, (0x8482, false)), // East Asian ideograph
    (0x21554B, (0x8431, false)), // East Asian ideograph
    (0x21554C, (0x8475, false)), // East Asian ideograph
    (0x21554D, (0x8466, false)), // East Asian ideograph
    (0x21554E, (0x8457, false)), // East Asian ideograph
    (0x22554F, (0x7250, false)), // East Asian ideograph
    (0x215550, (0x846C, false)), // East Asian ideograph
    (0x214E38, (0x7843, false)), // East Asian ideograph
    (0x215552, (0x845B, false)), // East Asian ideograph
    (0x215553, (0x8477, false)), // East Asian ideograph
    (0x215554, (0x843C, false)), // East Asian ideograph
    (0x215555, (0x8435, false)), // East Asian ideograph
    (0x225556, (0x725A, false)), // East Asian ideograph
    (0x215557, (0x8463, false)), // East Asian ideograph
    (0x215558, (0x8469, false)), // East Asian ideograph
    (0x225559, (0x7263, false)), // East Asian ideograph
    (0x21555A, (0x84B2, false)), // East Asian ideograph
    (0x21555B, (0x849E, false)), // East Asian ideograph
    (0x21555C, (0x84BF, false)), // East Asian ideograph
    (0x21555D, (0x84C6, false)), // East Asian ideograph
    (0x21555E, (0x84C4, false)), // East Asian ideograph
    (0x21555F, (0x84C9, false)), // East Asian ideograph
    (0x215560, (0x849C, false)), // East Asian ideograph
    (0x215561, (0x84CB, false)), // East Asian ideograph
    (0x215562, (0x84B8, false)), // East Asian ideograph
    (0x275563, (0x836A, false)), // East Asian ideograph
    (0x275564, (0x82CE, false)), // East Asian ideograph
    (0x215565, (0x84D3, false)), // East Asian ideograph
    (0x225566, (0x7276, false)), // East Asian ideograph
    (0x215567, (0x84BC, false)), // East Asian ideograph
    (0x225568, (0x7277, false)), // East Asian ideograph
    (0x215569, (0x84FF, false)), // East Asian ideograph
    (0x21556A, (0x8517, false)), // East Asian ideograph
    (0x22556B, (0x727E, false)), // East Asian ideograph
    (0x21556C, (0x84EE, false)), // East Asian ideograph
    (0x21556D, (0x852C, false)), // East Asian ideograph
    (0x27556E, (0x836B, false)), // East Asian ideograph
    (0x21556F, (0x8513, false)), // East Asian ideograph
    (0x6F5570, (0xC61B, false)), // Korean hangul
    (0x215571, (0x8523, false)), // East Asian ideograph
    (0x215572, (0x8521, false)), // East Asian ideograph
    (0x275573, (0x535C, false)), // East Asian ideograph
    (0x225574, (0x7289, false)), // East Asian ideograph
    (0x215575, (0x8525, false)), // East Asian ideograph
    (0x235576, (0x9B2F, false)), // East Asian ideograph
    (0x215577, (0x854A, false)), // East Asian ideograph
    (0x215578, (0x8559, false)), // East Asian ideograph
    (0x215579, (0x8548, false)), // East Asian ideograph
    (0x21557A, (0x8568, false)), // East Asian ideograph
    (0x21557B, (0x8543, false)), // East Asian ideograph
    (0x21557C, (0x856A, false)), // East Asian ideograph
    (0x21557D, (0x8549, false)), // East Asian ideograph
    (0x21557E, (0x8584, false)), // East Asian ideograph
    (0x224E40, (0x6FA5, false)), // East Asian ideograph
    (0x224E41, (0x6FB0, false)), // East Asian ideograph
    (0x4B6048, (0x981A, false)), // East Asian ideograph
    (0x224E42, (0x6FAE, false)), // East Asian ideograph
    (0x2F585C, (0x9C51, false)), // Unrelated variant of EACC 235945 which maps to 9C51
    (0x224E43, (0x6FD9, false)), // East Asian ideograph
    (0x276260, (0x4E48, false)), // East Asian ideograph
    (0x21393F, (0x5969, false)), // East Asian ideograph
    (0x224E44, (0x6FDA, false)), // East Asian ideograph
    (0x274E45, (0x7855, false)), // East Asian ideograph
    (0x6F5B3C, (0xD1B0, false)), // Korean hangul
    (0x273422, (0x5218, false)), // East Asian ideograph
    (0x6F4E46, (0xB664, false)), // Korean hangul
    (0x286B7C, (0x7B15, false)), // East Asian ideograph
    (0x22316C, (0x636C, false)), // East Asian ideograph
    (0x6F4E47, (0xB69C, false)), // Korean hangul
    (0x6F585D, (0xCAD1, false)), // Korean hangul
    (0x295170, (0x998D, false)), // East Asian ideograph
    (0x274E49, (0x786E, false)), // East Asian ideograph
    (0x6F4C4C, (0xB220, false)), // Korean hangul
    (0x6F4E4A, (0xB6AB, false)), // Korean hangul
    (0x213179, (0x5006, false)), // East Asian ideograph
    (0x234E4B, (0x97CE, false)), // East Asian ideograph
    (0x2D753A, (0x9654, false)), // East Asian ideograph
    (0x2D5321, (0x7C9B, false)), // East Asian ideograph
    (0x274E4C, (0x7801, false)), // East Asian ideograph
    (0x3F3078, (0x5023, false)), // East Asian ideograph
    (0x6F585E, (0xCAD2, false)), // Korean hangul
    (0x6F4E4D, (0xB6F0, false)), // Korean hangul
    (0x234E4E, (0x97D0, false)), // East Asian ideograph
    (0x4B552C, (0x8392, false)), // East Asian ideograph
    (0x29546D, (0x9ACB, false)), // East Asian ideograph
    (0x333564, (0x5415, false)), // East Asian ideograph
    (0x275154, (0x7EC3, false)), // East Asian ideograph
    (0x224E50, (0x6FD4, false)), // East Asian ideograph
    (0x213930, (0x5949, false)), // East Asian ideograph
    (0x234E51, (0x97D4, false)), // East Asian ideograph
    (0x6F585F, (0xCAD3, false)), // Korean hangul
    (0x295172, (0x9994, false)), // East Asian ideograph
    (0x21605D, (0x98B1, false)), // East Asian ideograph
    (0x213E44, (0x606C, false)), // East Asian ideograph
    (0x274E53, (0x7816, false)), // East Asian ideograph
    (0x234642, (0x93A7, false)), // East Asian ideograph
    (0x234E54, (0x97D9, false)), // East Asian ideograph
    (0x69245C, (0x307C, false)), // Hiragana letter BO
    (0x6F4E55, (0xB72F, false)), // Korean hangul
    (0x224E56, (0x6FE9, false)), // East Asian ideograph
    (0x6F5860, (0xCAD8, false)), // Korean hangul
    (0x224E57, (0x6FF8, false)), // East Asian ideograph
    (0x4B3758, (0x56A5, false)), // East Asian ideograph (variant of 213758 which maps to 56A5)
    (0x274E58, (0x77F6, false)), // East Asian ideograph
    (0x214E59, (0x790E, false)), // East Asian ideograph
    (0x274E5A, (0x788D, false)), // East Asian ideograph
    (0x2D5C40, (0x5FA8, false)), // East Asian ideograph
    (0x215621, (0x85AA, false)), // East Asian ideograph
    (0x215622, (0x856D, false)), // East Asian ideograph
    (0x235623, (0x9B37, false)), // East Asian ideograph
    (0x275624, (0x59DC, false)), // East Asian ideograph
    (0x215625, (0x857E, false)), // East Asian ideograph
    (0x215626, (0x8594, false)), // East Asian ideograph
    (0x215627, (0x859C, false)), // East Asian ideograph
    (0x225628, (0x728F, false)), // East Asian ideograph
    (0x215629, (0x85CD, false)), // East Asian ideograph (variant of 4B5629 which maps to 85CD)
    (0x27562A, (0x8428, false)), // East Asian ideograph
    (0x21562B, (0x85CF, false)), // East Asian ideograph
    (0x21562C, (0x85AF, false)), // East Asian ideograph
    (0x21562D, (0x85D0, false)), // East Asian ideograph
    (0x27562E, (0x501F, false)), // East Asian ideograph
    (0x214E5D, (0x792A, false)), // East Asian ideograph
    (0x215630, (0x85E9, false)), // East Asian ideograph
    (0x215631, (0x85DD, false)), // East Asian ideograph
    (0x275632, (0x85AE, false)), // East Asian ideograph
    (0x215633, (0x85E4, false)), // East Asian ideograph
    (0x215634, (0x85D5, false)), // East Asian ideograph
    (0x224E5E, (0x6FEE, false)), // East Asian ideograph
    (0x215636, (0x85FB, false)), // East Asian ideograph
    (0x215637, (0x85F9, false)), // East Asian ideograph
    (0x215638, (0x8611, false)), // East Asian ideograph
    (0x215639, (0x85FA, false)), // East Asian ideograph
    (0x27563A, (0x82A6, false)), // East Asian ideograph
    (0x27563B, (0x82F9, false)), // East Asian ideograph
    (0x27563C, (0x82CF, false)), // East Asian ideograph
    (0x27563D, (0x8574, false)), // East Asian ideograph
    (0x27563E, (0x5170, false)), // East Asian ideograph
    (0x21563F, (0x8617, false)), // East Asian ideograph
    (0x215640, (0x861A, false)), // East Asian ideograph
    (0x215641, (0x8638, false)), // East Asian ideograph
    (0x275642, (0x841D, false)), // East Asian ideograph
    (0x215643, (0x864E, false)), // East Asian ideograph
    (0x215644, (0x8650, false)), // East Asian ideograph
    (0x215645, (0x8654, false)), // East Asian ideograph
    (0x215646, (0x5F6A, false)), // East Asian ideograph
    (0x215647, (0x8655, false)), // East Asian ideograph
    (0x275648, (0x864F, false)), // East Asian ideograph
    (0x215649, (0x865B, false)), // East Asian ideograph
    (0x27564A, (0x53F7, false)), // East Asian ideograph
    (0x21564B, (0x865E, false)), // East Asian ideograph
    (0x22564C, (0x72AB, false)), // East Asian ideograph
    (0x224E62, (0x6FF0, false)), // East Asian ideograph
    (0x22564E, (0x72B0, false)), // East Asian ideograph
    (0x21564F, (0x8679, false)), // East Asian ideograph
    (0x215650, (0x86A9, false)), // East Asian ideograph
    (0x215651, (0x86AA, false)), // East Asian ideograph
    (0x215652, (0x868A, false)), // East Asian ideograph
    (0x215653, (0x8693, false)), // East Asian ideograph
    (0x215654, (0x86A4, false)), // East Asian ideograph
    (0x215655, (0x868C, false)), // East Asian ideograph
    (0x215656, (0x86A3, false)), // East Asian ideograph
    (0x215657, (0x86C0, false)), // East Asian ideograph
    (0x215658, (0x86C7, false)), // East Asian ideograph
    (0x215659, (0x86B5, false)), // East Asian ideograph
    (0x27565A, (0x65E6, false)), // East Asian ideograph
    (0x21565B, (0x86B6, false)), // East Asian ideograph
    (0x21565C, (0x86C4, false)), // East Asian ideograph
    (0x21565D, (0x86C6, false)), // East Asian ideograph
    (0x21565E, (0x86B1, false)), // East Asian ideograph
    (0x21565F, (0x86AF, false)), // East Asian ideograph
    (0x225660, (0x72D6, false)), // East Asian ideograph
    (0x215661, (0x86D9, false)), // East Asian ideograph
    (0x215662, (0x86ED, false)), // East Asian ideograph
    (0x215663, (0x86D4, false)), // East Asian ideograph
    (0x225664, (0x72D2, false)), // East Asian ideograph
    (0x224E66, (0x7005, false)), // East Asian ideograph
    (0x215666, (0x86FB, false)), // East Asian ideograph
    (0x225667, (0x72C9, false)), // East Asian ideograph
    (0x215668, (0x8707, false)), // East Asian ideograph
    (0x215669, (0x8703, false)), // East Asian ideograph
    (0x21566A, (0x8708, false)), // East Asian ideograph
    (0x214E67, (0x7955, false)), // East Asian ideograph
    (0x21566C, (0x86FE, false)), // East Asian ideograph
    (0x21566D, (0x8713, false)), // East Asian ideograph
    (0x21566E, (0x8702, false)), // East Asian ideograph
    (0x21566F, (0x871C, false)), // East Asian ideograph
    (0x215670, (0x873F, false)), // East Asian ideograph
    (0x215671, (0x873B, false)), // East Asian ideograph
    (0x215672, (0x8722, false)), // East Asian ideograph
    (0x225673, (0x72E8, false)), // East Asian ideograph
    (0x215674, (0x8734, false)), // East Asian ideograph
    (0x215675, (0x8718, false)), // East Asian ideograph
    (0x215676, (0x8755, false)), // East Asian ideograph
    (0x215677, (0x8760, false)), // East Asian ideograph
    (0x215678, (0x8776, false)), // East Asian ideograph
    (0x225679, (0x72E5, false)), // East Asian ideograph
    (0x27567A, (0x867E, false)), // East Asian ideograph
    (0x21567B, (0x8778, false)), // East Asian ideograph
    (0x21567C, (0x8768, false)), // East Asian ideograph
    (0x21567D, (0x874C, false)), // East Asian ideograph
    (0x22567E, (0x72FA, false)), // East Asian ideograph
    (0x6F4E6B, (0xB790, false)), // Korean hangul
    (0x6F5421, (0xC2B7, false)), // Korean hangul
    (0x234E6C, (0x97F5, false)), // East Asian ideograph
    (0x6F5339, (0xC151, false)), // Korean hangul
    (0x6F4E6D, (0xB797, false)), // Korean hangul
    (0x69245D, (0x307D, false)), // Hiragana letter PO
    (0x6F4E6E, (0xB798, false)), // Korean hangul
    (0x274E6F, (0x53EA, false)), // East Asian ideograph (duplicate simplified)
    (0x6F5865, (0xCB20, false)), // Korean hangul
    (0x6F4E70, (0xB79C, false)), // Korean hangul
    (0x6F5422, (0xC2B9, false)), // Korean hangul
    (0x6F5A2A, (0xCEF7, false)), // Korean hangul
    (0x6F4E71, (0xB7A0, false)), // Korean hangul
    (0x234648, (0x939A, false)), // East Asian ideograph
    (0x213441, (0x5305, false)), // East Asian ideograph
    (0x224E72, (0x7026, false)), // East Asian ideograph
    (0x214E73, (0x797A, false)), // East Asian ideograph
    (0x286C58, (0x7BA7, false)), // East Asian ideograph
    (0x6F5633, (0xC68D, false)), // Korean hangul
    (0x6F4E3B, (0xB5BC, false)), // Korean hangul
    (0x6F5866, (0xCB21, false)), // Korean hangul
    (0x395179, (0x7D75, false)), // East Asian ideograph
    (0x6F4E76, (0xB7AD, false)), // Korean hangul
    (0x213921, (0x5920, false)), // East Asian ideograph
    (0x274E77, (0x7978, false)), // East Asian ideograph
    (0x27785E, (0x5786, false)), // East Asian ideograph
    (0x213922, (0x5924, false)), // East Asian ideograph
    (0x274E78, (0x796F, false)), // East Asian ideograph
    (0x213923, (0x5925, false)), // East Asian ideograph
    (0x234E79, (0x9807, false)), // East Asian ideograph
    (0x273924, (0x68A6, false)), // East Asian ideograph
    (0x6F5867, (0xCB41, false)), // Korean hangul
    (0x213F37, (0x6155, false)), // East Asian ideograph
    (0x6F4E7A, (0xB7EC, false)), // Korean hangul
    (0x6F4D61, (0xB4D0, false)), // Korean hangul
    (0x226464, (0x7876, false)), // East Asian ideograph
    (0x274E7B, (0x7985, false)), // East Asian ideograph
    (0x69482B, (0x7560, false)), // East Asian ideograph
    (0x274E7C, (0x793C, false)), // East Asian ideograph
    (0x4B4F3C, (0x79F0, false)), // East Asian ideograph (variant of 274F3C which maps to 79F0)
    (0x213927, (0x592B, false)), // East Asian ideograph
    (0x274E7D, (0x7977, false)), // East Asian ideograph
    (0x2D5323, (0x5B8D, false)), // East Asian ideograph
    (0x234E7E, (0x980D, false)), // East Asian ideograph
    (0x2D3929, (0x6B80, false)), // East Asian ideograph
    (0x213376, (0x5274, false)), // East Asian ideograph
    (0x6F5868, (0xCB48, false)), // Korean hangul
    (0x6F5433, (0xC300, false)), // Korean hangul
    (0x216066, (0x98E7, false)), // East Asian ideograph
    (0x21392B, (0x5931, false)), // East Asian ideograph
    (0x3F304C, (0x5E79, false)), // East Asian ideograph
    (0x2E3A26, (0x661D, false)), // East Asian ideograph
    (0x225359, (0x71B4, false)), // East Asian ideograph
    (0x21392E, (0x593E, false)), // East Asian ideograph
    (0x6F5869, (0xCB49, false)), // Korean hangul
    (0x396B33, (0x5259, false)), // East Asian ideograph (not in Unicode)
    (0x216067, (0x98E9, false)), // East Asian ideograph
    (0x235721, (0x9B83, false)), // East Asian ideograph
    (0x215722, (0x8783, false)), // East Asian ideograph
    (0x215723, (0x8782, false)), // East Asian ideograph
    (0x275724, (0x8424, false)), // East Asian ideograph
    (0x225725, (0x72FE, false)), // East Asian ideograph
    (0x215726, (0x878D, false)), // East Asian ideograph
    (0x215727, (0x879F, false)), // East Asian ideograph
    (0x215728, (0x87D1, false)), // East Asian ideograph
    (0x215729, (0x87C0, false)), // East Asian ideograph
    (0x21572A, (0x87AB, false)), // East Asian ideograph
    (0x23572B, (0x9B90, false)), // East Asian ideograph
    (0x27572C, (0x877C, false)), // East Asian ideograph
    (0x22572D, (0x7301, false)), // East Asian ideograph
    (0x22572E, (0x72F3, false)), // East Asian ideograph
    (0x23572F, (0x9B97, false)), // East Asian ideograph
    (0x215730, (0x87C6, false)), // East Asian ideograph
    (0x215731, (0x87CB, false)), // East Asian ideograph
    (0x215732, (0x87EF, false)), // East Asian ideograph
    (0x215733, (0x87F2, false)), // East Asian ideograph
    (0x215734, (0x87EC, false)), // East Asian ideograph
    (0x225735, (0x730B, false)), // East Asian ideograph
    (0x225736, (0x7317, false)), // East Asian ideograph
    (0x215737, (0x880D, false)), // East Asian ideograph
    (0x215738, (0x87F9, false)), // East Asian ideograph
    (0x215739, (0x8814, false)), // East Asian ideograph
    (0x21573A, (0x8815, false)), // East Asian ideograph
    (0x22573B, (0x7307, false)), // East Asian ideograph
    (0x23573C, (0x9BAD, false)), // East Asian ideograph
    (0x23573D, (0x9B9A, false)), // East Asian ideograph
    (0x22573E, (0x7318, false)), // East Asian ideograph
    (0x27573F, (0x86CA, false)), // East Asian ideograph
    (0x215740, (0x8839, false)), // East Asian ideograph
    (0x275741, (0x8695, false)), // East Asian ideograph
    (0x215742, (0x883B, false)), // East Asian ideograph
    (0x235743, (0x9B99, false)), // East Asian ideograph
    (0x215744, (0x884C, false)), // East Asian ideograph
    (0x215745, (0x884D, false)), // East Asian ideograph
    (0x225746, (0x7331, false)), // East Asian ideograph
    (0x215747, (0x8857, false)), // East Asian ideograph
    (0x215748, (0x8859, false)), // East Asian ideograph
    (0x225749, (0x7338, false)), // East Asian ideograph
    (0x22574A, (0x7322, false)), // East Asian ideograph
    (0x21574B, (0x8861, false)), // East Asian ideograph
    (0x22574C, (0x7332, false)), // East Asian ideograph
    (0x22574D, (0x732C, false)), // East Asian ideograph
    (0x22574E, (0x7327, false)), // East Asian ideograph
    (0x22574F, (0x732B, false)), // East Asian ideograph
    (0x215750, (0x886B, false)), // East Asian ideograph
    (0x215751, (0x8882, false)), // East Asian ideograph
    (0x225752, (0x732F, false)), // East Asian ideograph
    (0x215753, (0x8870, false)), // East Asian ideograph
    (0x215754, (0x8877, false)), // East Asian ideograph
    (0x225755, (0x7328, false)), // East Asian ideograph
    (0x235756, (0x9BC7, false)), // East Asian ideograph
    (0x215757, (0x8892, false)), // East Asian ideograph
    (0x215758, (0x8896, false)), // East Asian ideograph
    (0x235759, (0x9BD2, false)), // East Asian ideograph
    (0x22575A, (0x7347, false)), // East Asian ideograph
    (0x22575B, (0x7348, false)), // East Asian ideograph
    (0x22575C, (0x7349, false)), // East Asian ideograph
    (0x23393A, (0x8DE6, false)), // East Asian ideograph
    (0x21575E, (0x88B1, false)), // East Asian ideograph
    (0x23575F, (0x9BC1, false)), // East Asian ideograph
    (0x215760, (0x88D9, false)), // East Asian ideograph
    (0x215761, (0x88D8, false)), // East Asian ideograph
    (0x215762, (0x88DC, false)), // East Asian ideograph
    (0x215763, (0x88CF, false)), // East Asian ideograph
    (0x215764, (0x88D4, false)), // East Asian ideograph
    (0x225765, (0x7340, false)), // East Asian ideograph
    (0x215766, (0x88D5, false)), // East Asian ideograph
    (0x215767, (0x8902, false)), // East Asian ideograph
    (0x225768, (0x734D, false)), // East Asian ideograph
    (0x215769, (0x88F8, false)), // East Asian ideograph
    (0x21576A, (0x88F9, false)), // East Asian ideograph
    (0x21576B, (0x88F4, false)), // East Asian ideograph
    (0x23576C, (0x9BD3, false)), // East Asian ideograph
    (0x21576D, (0x88E8, false)), // East Asian ideograph
    (0x21576E, (0x891A, false)), // East Asian ideograph
    (0x21576F, (0x8910, false)), // East Asian ideograph
    (0x6F5770, (0xC906, false)), // Korean hangul
    (0x215771, (0x8913, false)), // East Asian ideograph
    (0x235772, (0x9BC8, false)), // East Asian ideograph
    (0x215773, (0x8932, false)), // East Asian ideograph
    (0x225774, (0x735D, false)), // East Asian ideograph
    (0x215775, (0x8925, false)), // East Asian ideograph
    (0x215776, (0x892B, false)), // East Asian ideograph
    (0x235777, (0x9BD7, false)), // East Asian ideograph
    (0x215778, (0x8936, false)), // East Asian ideograph
    (0x225779, (0x7360, false)), // East Asian ideograph
    (0x23577A, (0x9BD6, false)), // East Asian ideograph
    (0x21577B, (0x895F, false)), // East Asian ideograph
    (0x23577C, (0x9BEB, false)), // East Asian ideograph
    (0x21577D, (0x8956, false)), // East Asian ideograph
    (0x22577E, (0x7362, false)), // East Asian ideograph
    (0x273940, (0x593A, false)), // East Asian ideograph
    (0x223941, (0x66AA, false)), // East Asian ideograph
    (0x232B33, (0x874D, false)), // East Asian ideograph
    (0x2D506F, (0x7CFA, false)), // East Asian ideograph
    (0x6F586D, (0xCB5D, false)), // Korean hangul
    (0x6F7643, (0xE8B5, false)), // Korean hangul
    (0x213943, (0x5974, false)), // East Asian ideograph
    (0x213944, (0x5976, false)), // East Asian ideograph
    (0x4B3322, (0x5168, false)), // East Asian ideograph (variant of 213322 which maps to 5168)
    (0x27564C, (0x4E8F, false)), // East Asian ideograph
    (0x4B5E3D, (0x9421, false)), // East Asian ideograph
    (0x213946, (0x5983, false)), // East Asian ideograph
    (0x6F5365, (0xC231, false)), // Korean hangul
    (0x6F516D, (0xBE44, false)), // Korean hangul
    (0x213947, (0x5978, false)), // East Asian ideograph
    (0x2D4C2D, (0x756E, false)), // East Asian ideograph
    (0x6F542B, (0xC2EF, false)), // Korean hangul
    (0x213E53, (0x6089, false)), // East Asian ideograph
    (0x213949, (0x5979, false)), // East Asian ideograph
    (0x21794B, (0x595C, false)), // East Asian ideograph
    (0x454E75, (0x7984, false)), // East Asian ideograph
    (0x213C7D, (0x5EC2, false)), // East Asian ideograph
    (0x6F586F, (0xCBB8, false)), // Korean hangul
    (0x2D394D, (0x59AC, false)), // East Asian ideograph
    (0x217E43, (0x5B93, false)), // East Asian ideograph
    (0x22394E, (0x66C8, false)), // East Asian ideograph
    (0x4B3324, (0x634C, false)), // East Asian ideograph (variant of 2D3324 which maps to 634C)
    (0x21394F, (0x59A4, false)), // East Asian ideograph
    (0x213F58, (0x61FA, false)), // East Asian ideograph
    (0x213950, (0x59A3, false)), // East Asian ideograph
    (0x6F4E3D, (0xB5C4, false)), // Korean hangul
    (0x213951, (0x5993, false)), // East Asian ideograph
    (0x2F5870, (0x9C1B, false)), // East Asian ideograph
    (0x6F7646, (0xE8B8, false)), // Korean hangul
    (0x213952, (0x599E, false)), // East Asian ideograph
    (0x213E55, (0x60A0, false)), // East Asian ideograph
    (0x4B3768, (0x56D8, false)), // East Asian ideograph
    (0x213953, (0x599D, false)), // East Asian ideograph
    (0x233954, (0x8E23, false)), // East Asian ideograph
    (0x213955, (0x59A5, false)), // East Asian ideograph
    (0x335760, (0x88E0, false)), // East Asian ideograph
    (0x2D3956, (0x59D9, false)), // East Asian ideograph
    (0x6F4F22, (0xB7FF, false)), // Korean hangul
    (0x6F5871, (0xCBE4, false)), // Korean hangul
    (0x6F7647, (0xE8B9, false)), // Korean hangul
    (0x213957, (0x5996, false)), // East Asian ideograph
    (0x213958, (0x59BE, false)), // East Asian ideograph
    (0x333642, (0x8A92, false)), // East Asian ideograph
    (0x2D3F67, (0x621E, false)), // East Asian ideograph
    (0x6F5878, (0xCC1D, false)), // Korean hangul
    (0x4C7959, (0x817D, false)), // East Asian ideograph
    (0x273437, (0x80DC, false)), // East Asian ideograph
    (0x214F63, (0x7AC7, false)), // East Asian ideograph
    (0x4B524E, (0x7FCE, false)), // East Asian ideograph
    (0x21395A, (0x59AE, false)), // East Asian ideograph
    (0x3A4034, (0x6855, false)), // East Asian ideograph
    (0x275821, (0x889C, false)), // East Asian ideograph
    (0x275822, (0x886C, false)), // East Asian ideograph
    (0x215823, (0x8972, false)), // East Asian ideograph
    (0x215824, (0x897F, false)), // East Asian ideograph
    (0x225825, (0x7367, false)), // East Asian ideograph
    (0x215826, (0x8983, false)), // East Asian ideograph
    (0x235827, (0x9BE4, false)), // East Asian ideograph
    (0x275828, (0x89C1, false)), // East Asian ideograph
    (0x275829, (0x89C4, false)), // East Asian ideograph
    (0x27582A, (0x89C5, false)), // East Asian ideograph
    (0x27582B, (0x89C6, false)), // East Asian ideograph
    (0x27582C, (0x4EB2, false)), // East Asian ideograph
    (0x27582D, (0x89CE, false)), // East Asian ideograph
    (0x21582E, (0x89AC, false)), // East Asian ideograph
    (0x27582F, (0x89D0, false)), // East Asian ideograph
    (0x215830, (0x89BA, false)), // East Asian ideograph
    (0x275831, (0x89C8, false)), // East Asian ideograph
    (0x215832, (0x89C0, false)), // East Asian ideograph
    (0x215833, (0x89D2, false)), // East Asian ideograph
    (0x235834, (0x9BD4, false)), // East Asian ideograph
    (0x215835, (0x89F4, false)), // East Asian ideograph
    (0x225836, (0x737C, false)), // East Asian ideograph
    (0x215837, (0x8A00, false)), // East Asian ideograph
    (0x215838, (0x8A08, false)), // East Asian ideograph
    (0x215839, (0x8A02, false)), // East Asian ideograph
    (0x21583A, (0x8A03, false)), // East Asian ideograph
    (0x21583B, (0x8A10, false)), // East Asian ideograph
    (0x21583C, (0x8A18, false)), // East Asian ideograph
    (0x21583D, (0x8A0E, false)), // East Asian ideograph
    (0x23583E, (0x9BFF, false)), // East Asian ideograph
    (0x21583F, (0x8A15, false)), // East Asian ideograph
    (0x215840, (0x8A0A, false)), // East Asian ideograph
    (0x275841, (0x8BAB, false)), // East Asian ideograph
    (0x225842, (0x738E, false)), // East Asian ideograph
    (0x235843, (0x9C06, false)), // East Asian ideograph
    (0x235844, (0x9C15, false)), // East Asian ideograph
    (0x215845, (0x8A23, false)), // East Asian ideograph
    (0x275846, (0x8BB6, false)), // East Asian ideograph
    (0x225847, (0x7392, false)), // East Asian ideograph
    (0x215848, (0x8A31, false)), // East Asian ideograph
    (0x275849, (0x8BBE, false)), // East Asian ideograph
    (0x27584A, (0x8BB9, false)), // East Asian ideograph
    (0x27584B, (0x8BBC, false)), // East Asian ideograph
    (0x27584C, (0x6CE8, false)), // East Asian ideograph
    (0x21584D, (0x8A60, false)), // East Asian ideograph
    (0x27584E, (0x8BC4, false)), // East Asian ideograph
    (0x27584F, (0x8BCD, false)), // East Asian ideograph
    (0x6F5850, (0xCA50, false)), // Korean hangul
    (0x215851, (0x8A41, false)), // East Asian ideograph
    (0x235852, (0x9C02, false)), // East Asian ideograph
    (0x215853, (0x8A5B, false)), // East Asian ideograph
    (0x235854, (0x9C10, false)), // East Asian ideograph
    (0x215855, (0x8A46, false)), // East Asian ideograph
    (0x215856, (0x8A34, false)), // East Asian ideograph
    (0x275857, (0x8BCA, false)), // East Asian ideograph
    (0x275858, (0x8BE7, false)), // East Asian ideograph
    (0x215859, (0x8A72, false)), // East Asian ideograph
    (0x21585A, (0x8A73, false)), // East Asian ideograph
    (0x21585B, (0x8A66, false)), // East Asian ideograph
    (0x27585C, (0x8BD7, false)), // East Asian ideograph
    (0x27585D, (0x8BD8, false)), // East Asian ideograph
    (0x27585E, (0x8BE3, false)), // East Asian ideograph
    (0x27585F, (0x8BD9, false)), // East Asian ideograph
    (0x275860, (0x8BDA, false)), // East Asian ideograph
    (0x215861, (0x8A87, false)), // East Asian ideograph
    (0x275862, (0x8BDB, false)), // East Asian ideograph
    (0x215863, (0x8A6D, false)), // East Asian ideograph
    (0x215864, (0x8A79, false)), // East Asian ideograph
    (0x275865, (0x8BE2, false)), // East Asian ideograph
    (0x275866, (0x8BDD, false)), // East Asian ideograph
    (0x275867, (0x8BE0, false)), // East Asian ideograph
    (0x215868, (0x8A6C, false)), // East Asian ideograph
    (0x235869, (0x9C2F, false)), // East Asian ideograph
    (0x22586A, (0x73C2, false)), // East Asian ideograph
    (0x22586B, (0x73D0, false)), // East Asian ideograph
    (0x21586C, (0x8A9E, false)), // East Asian ideograph
    (0x21586D, (0x8A8C, false)), // East Asian ideograph
    (0x21586E, (0x8A93, false)), // East Asian ideograph
    (0x22586F, (0x73BF, false)), // East Asian ideograph
    (0x275870, (0x8BA4, false)), // East Asian ideograph
    (0x275871, (0x8BEF, false)), // East Asian ideograph
    (0x275872, (0x8BF2, false)), // East Asian ideograph
    (0x275873, (0x8BF0, false)), // East Asian ideograph
    (0x275874, (0x8BF1, false)), // East Asian ideograph
    (0x275875, (0x8BF3, false)), // East Asian ideograph
    (0x215876, (0x8ABC, false)), // East Asian ideograph
    (0x275877, (0x8C06, false)), // East Asian ideograph
    (0x275878, (0x8C05, false)), // East Asian ideograph
    (0x215879, (0x8AC7, false)), // East Asian ideograph
    (0x21587A, (0x8ACB, false)), // East Asian ideograph (variant of 4B587A which maps to 8ACB)
    (0x27587B, (0x8BF8, false)), // East Asian ideograph
    (0x27587C, (0x8BFE, false)), // East Asian ideograph
    (0x27587D, (0x8C03, false)), // East Asian ideograph
    (0x23587E, (0x9C46, false)), // East Asian ideograph
    (0x6F5875, (0xCC10, false)), // Korean hangul
    (0x6F764B, (0xE8BD, false)), // Korean hangul
    (0x21796B, (0x5998, false)), // East Asian ideograph
    (0x6F4B62, (0xB0BB, false)), // Korean hangul
    (0x293C5A, (0x8F73, false)), // East Asian ideograph
    (0x2D396E, (0x4F84, false)), // East Asian ideograph
    (0x28732D, (0x7F2F, false)), // East Asian ideograph
    (0x4B6145, (0x9A12, false)), // East Asian ideograph
    (0x21396F, (0x5A01, false)), // East Asian ideograph
    (0x2D4C35, (0x7567, false)), // East Asian ideograph
    (0x2D3970, (0x5A63, false)), // East Asian ideograph
    (0x213971, (0x59E6, false)), // East Asian ideograph
    (0x6F5879, (0xCC21, false)), // Korean hangul
    (0x213972, (0x59DA, false)), // East Asian ideograph
    (0x213973, (0x5A11, false)), // East Asian ideograph
    (0x2D3974, (0x5B43, false)), // East Asian ideograph
    (0x6F5877, (0xCC1C, false)), // Korean hangul
    (0x6F764D, (0xE8BF, false)), // Korean hangul
    (0x6F5434, (0xC308, false)), // Korean hangul
    (0x213E5C, (0x60B6, false)), // East Asian ideograph
    (0x4B376F, (0x56FD, false)), // East Asian ideograph
    (0x213976, (0x5A1C, false)), // East Asian ideograph
    (0x692421, (0x3041, false)), // Hiragana letter small A
    (0x213977, (0x5A13, false)), // East Asian ideograph
    (0x2D5773, (0x7D5D, false)), // East Asian ideograph
    (0x213978, (0x59EC, false)), // East Asian ideograph
    (0x33354E, (0x608B, false)), // East Asian ideograph
    (0x213979, (0x5A20, false)), // East Asian ideograph
    (0x216424, (0x4E0F, false)), // East Asian ideograph
    (0x6F764E, (0xE8C0, false)), // Korean hangul
    (0x6F535C, (0xC219, false)), // Korean hangul
    (0x213E5D, (0x60D1, false)), // East Asian ideograph
    (0x2D397B, (0x5A31, false)), // East Asian ideograph
    (0x2D3F6E, (0x6226, false)), // East Asian ideograph
    (0x21397C, (0x5A0C, false)), // East Asian ideograph
    (0x692427, (0x3047, false)), // Hiragana letter small E
    (0x6F5841, (0xC9EF, false)), // Korean hangul
    (0x22797D, (0x81A6, false)), // East Asian ideograph
    (0x692428, (0x3048, false)), // Hiragana letter E
    (0x21397E, (0x5A25, false)), // East Asian ideograph
    (0x222429, (0x5CDD, false)), // East Asian ideograph
    (0x6F764F, (0xE8C1, false)), // Korean hangul
    (0x6F5436, (0xC30B, false)), // Korean hangul
    (0x213E5E, (0x60B5, false)), // East Asian ideograph
    (0x33304C, (0x4E79, false)), // East Asian ideograph
    (0x215C34, (0x904B, false)), // East Asian ideograph
    (0x2D3F6F, (0x622F, false)), // East Asian ideograph
    (0x27742E, (0x56F5, false)), // East Asian ideograph
    (0x216D41, (0x534C, false)), // East Asian ideograph
    (0x6F587A, (0xCC22, false)), // Korean hangul
    (0x6F7650, (0xE8C2, false)), // Korean hangul
    (0x6F5437, (0xC30C, false)), // Korean hangul
    (0x69242F, (0x304F, false)), // Hiragana letter KU
    (0x4B3772, (0x5186, false)), // East Asian ideograph
    (0x4B5631, (0x82B8, false)), // East Asian ideograph
    (0x225921, (0x73D3, false)), // East Asian ideograph
    (0x215922, (0x8AB0, false)), // East Asian ideograph
    (0x215923, (0x8A95, false)), // East Asian ideograph
    (0x215924, (0x8AD6, false)), // East Asian ideograph
    (0x275925, (0x8C1B, false)), // East Asian ideograph
    (0x235926, (0x9C44, false)), // East Asian ideograph
    (0x215927, (0x8AEB, false)), // East Asian ideograph
    (0x225928, (0x73E5, false)), // East Asian ideograph
    (0x235929, (0x9C39, false)), // East Asian ideograph
    (0x22592A, (0x73D9, false)), // East Asian ideograph
    (0x22592B, (0x73EF, false)), // East Asian ideograph
    (0x21592C, (0x8B01, false)), // East Asian ideograph (variant of 2D592C which maps to 8B01)
    (0x21592D, (0x8B02, false)), // East Asian ideograph
    (0x21592E, (0x8AFE, false)), // East Asian ideograph
    (0x27592F, (0x8BBD, false)), // East Asian ideograph
    (0x235930, (0x9C47, false)), // East Asian ideograph
    (0x215931, (0x8B17, false)), // East Asian ideograph
    (0x225932, (0x73D6, false)), // East Asian ideograph
    (0x215933, (0x8B0E, false)), // East Asian ideograph
    (0x235934, (0x9C37, false)), // East Asian ideograph
    (0x225935, (0x73BC, false)), // East Asian ideograph
    (0x215936, (0x8B21, false)), // East Asian ideograph
    (0x215937, (0x8B04, false)), // East Asian ideograph
    (0x235938, (0x9C52, false)), // East Asian ideograph
    (0x275939, (0x8C28, false)), // East Asian ideograph
    (0x22593A, (0x73DE, false)), // East Asian ideograph
    (0x23593B, (0x9C58, false)), // East Asian ideograph
    (0x22593C, (0x73E6, false)), // East Asian ideograph
    (0x21593D, (0x8B5C, false)), // East Asian ideograph
    (0x21593E, (0x8B4E, false)), // East Asian ideograph
    (0x21593F, (0x8B49, false)), // East Asian ideograph
    (0x275940, (0x8C2D, false)), // East Asian ideograph
    (0x215941, (0x8B41, false)), // East Asian ideograph
    (0x275942, (0x8BA5, false)), // East Asian ideograph
    (0x215943, (0x8B70, false)), // East Asian ideograph
    (0x215944, (0x8B6C, false)), // East Asian ideograph
    (0x225945, (0x73F6, false)), // East Asian ideograph
    (0x215946, (0x8B6F, false)), // East Asian ideograph
    (0x225947, (0x73FA, false)), // East Asian ideograph
    (0x275948, (0x62A4, false)), // East Asian ideograph
    (0x215949, (0x8B7D, false)), // East Asian ideograph
    (0x27594A, (0x8BFB, false)), // East Asian ideograph
    (0x21594B, (0x8B8A, false)), // East Asian ideograph
    (0x27594C, (0x8BA9, false)), // East Asian ideograph
    (0x21594D, (0x8B96, false)), // East Asian ideograph
    (0x21594E, (0x8B92, false)), // East Asian ideograph
    (0x23594F, (0x9C67, false)), // East Asian ideograph
    (0x6F5950, (0xCD2C, false)), // Korean hangul
    (0x215951, (0x8C41, false)), // East Asian ideograph
    (0x215952, (0x8C3F, false)), // East Asian ideograph
    (0x215953, (0x8C46, false)), // East Asian ideograph
    (0x225954, (0x73F5, false)), // East Asian ideograph
    (0x235955, (0x9C5F, false)), // East Asian ideograph
    (0x235956, (0x9C60, false)), // East Asian ideograph
    (0x215957, (0x8C4E, false)), // East Asian ideograph
    (0x235958, (0x9C6D, false)), // East Asian ideograph
    (0x215959, (0x8C54, false)), // East Asian ideograph
    (0x21595A, (0x8C5A, false)), // East Asian ideograph
    (0x23595B, (0x9C68, false)), // East Asian ideograph
    (0x22595C, (0x7407, false)), // East Asian ideograph
    (0x21595D, (0x8C6A, false)), // East Asian ideograph
    (0x22595E, (0x7412, false)), // East Asian ideograph
    (0x21595F, (0x8C6C, false)), // East Asian ideograph
    (0x215960, (0x8C7A, false)), // East Asian ideograph
    (0x215961, (0x8C79, false)), // East Asian ideograph
    (0x215962, (0x8C82, false)), // East Asian ideograph
    (0x225963, (0x743C, false)), // East Asian ideograph
    (0x215964, (0x8C89, false)), // East Asian ideograph
    (0x215965, (0x8C8D, false)), // East Asian ideograph
    (0x225966, (0x742E, false)), // East Asian ideograph
    (0x225967, (0x742F, false)), // East Asian ideograph
    (0x275968, (0x8D1D, false)), // East Asian ideograph
    (0x225969, (0x7414, false)), // East Asian ideograph
    (0x22596A, (0x742C, false)), // East Asian ideograph
    (0x27596B, (0x8D21, false)), // East Asian ideograph
    (0x27596C, (0x8D22, false)), // East Asian ideograph
    (0x27596D, (0x8D23, false)), // East Asian ideograph
    (0x22596E, (0x742B, false)), // East Asian ideograph
    (0x21596F, (0x8CA8, false)), // East Asian ideograph
    (0x225970, (0x73F7, false)), // East Asian ideograph
    (0x225971, (0x741A, false)), // East Asian ideograph
    (0x275972, (0x8D29, false)), // East Asian ideograph
    (0x235973, (0x9CE7, false)), // East Asian ideograph
    (0x235974, (0x9CF0, false)), // East Asian ideograph
    (0x215975, (0x8CBB, false)), // East Asian ideograph
    (0x215976, (0x8CC1, false)), // East Asian ideograph
    (0x235977, (0x9CF2, false)), // East Asian ideograph
    (0x225978, (0x7416, false)), // East Asian ideograph
    (0x215979, (0x8CBC, false)), // East Asian ideograph
    (0x22597A, (0x7426, false)), // East Asian ideograph
    (0x21597B, (0x8CB6, false)), // East Asian ideograph
    (0x21597C, (0x8CBD, false)), // East Asian ideograph
    (0x27597D, (0x8D37, false)), // East Asian ideograph
    (0x21597E, (0x8CBF, false)), // East Asian ideograph
    (0x222441, (0x5CF4, false)), // East Asian ideograph
    (0x224F2B, (0x701E, false)), // East Asian ideograph (variant of 4C4F2B which maps to 701E)
    (0x6F587E, (0xCC28, false)), // Korean hangul
    (0x275E58, (0x9602, false)), // East Asian ideograph
    (0x6F543B, (0xC315, false)), // Korean hangul
    (0x217E52, (0x5BA7, false)), // East Asian ideograph
    (0x333573, (0x8656, false)), // East Asian ideograph
    (0x222446, (0x5CF1, false)), // East Asian ideograph
    (0x69243F, (0x305F, false)), // Hiragana letter TA
    (0x27613E, (0x9A8F, false)), // East Asian ideograph
    (0x6F5B69, (0xD305, false)), // Korean hangul
    (0x2D5C2F, (0x8FE8, false)), // East Asian ideograph
    (0x2D4C3E, (0x758E, false)), // East Asian ideograph
    (0x6F4E74, (0xB7AB, false)), // Korean hangul
    (0x6F7655, (0xE8C7, false)), // Korean hangul
    (0x225F7B, (0x7657, false)), // East Asian ideograph
    (0x213E64, (0x60D5, false)), // East Asian ideograph
    (0x234662, (0x93F9, false)), // East Asian ideograph
    (0x696449, (0x7C13, false)), // East Asian ideograph
    (0x276137, (0x9A76, false)), // East Asian ideograph
    (0x69244A, (0x306A, false)), // Hiragana letter NA
    (0x4B6147, (0x99C6, false)), // East Asian ideograph
    (0x333556, (0x9A03, false)), // East Asian ideograph
    (0x69644C, (0x7C17, false)), // East Asian ideograph
    (0x6F4D66, (0xB4E4, false)), // Korean hangul
    (0x213E65, (0x60BC, false)), // East Asian ideograph
    (0x69644E, (0x7BF6, false)), // East Asian ideograph
    (0x6F587B, (0xCC27, false)), // Korean hangul
    (0x2D3B33, (0x8A67, false)), // East Asian ideograph
    (0x214B2F, (0x7380, false)), // East Asian ideograph
    (0x6F5438, (0xC30D, false)), // Korean hangul
    (0x6F543E, (0xC324, false)), // Korean hangul
    (0x6F7651, (0xE8C3, false)), // Korean hangul
    (0x216452, (0x4EB3, false)), // East Asian ideograph
    (0x294E43, (0x97AF, false)), // East Asian ideograph
    (0x234664, (0x93C4, false)), // East Asian ideograph
    (0x342453, (0x5CBD, false)), // East Asian ideograph
    (0x692454, (0x3074, false)), // Hiragana letter PI
    (0x225372, (0x71BA, false)), // East Asian ideograph
    (0x4B6455, (0x4EB6, false)), // East Asian ideograph
    (0x6F4F7A, (0xB9F7, false)), // Korean hangul
    (0x6F543F, (0xC327, false)), // Korean hangul
    (0x4B503B, (0x7C12, false)), // East Asian ideograph
    (0x692457, (0x3077, false)), // Hiragana letter PU
    (0x223E23, (0x691A, false)), // East Asian ideograph
    (0x234222, (0x91E4, false)), // East Asian ideograph
    (0x692459, (0x3079, false)), // Hiragana letter BE
    (0x21645A, (0x4EBC, false)), // East Asian ideograph
    (0x4B4444, (0x8988, false)), // East Asian ideograph (Version J extension)
    (0x215A21, (0x8CC5, false)), // East Asian ideograph
    (0x275A22, (0x8D44, false)), // East Asian ideograph
    (0x275A23, (0x8D3C, false)), // East Asian ideograph
    (0x215A24, (0x8CC8, false)), // East Asian ideograph
    (0x275A25, (0x8D3F, false)), // East Asian ideograph
    (0x215A26, (0x8CB2, false)), // East Asian ideograph
    (0x275A27, (0x8D41, false)), // East Asian ideograph
    (0x225A28, (0x7420, false)), // East Asian ideograph
    (0x275A29, (0x5BBE, false)), // East Asian ideograph
    (0x275A2A, (0x8D48, false)), // East Asian ideograph
    (0x275A2B, (0x8D4A, false)), // East Asian ideograph
    (0x215A2C, (0x8CE0, false)), // East Asian ideograph
    (0x275A2D, (0x8D4B, false)), // East Asian ideograph
    (0x6F5A2E, (0xCF00, false)), // Korean hangul
    (0x275A2F, (0x5356, false)), // East Asian ideograph
    (0x235A30, (0x9D25, false)), // East Asian ideograph
    (0x215A31, (0x8CE4, false)), // East Asian ideograph
    (0x215A32, (0x8CDE, false)), // East Asian ideograph
    (0x275A33, (0x8D50, false)), // East Asian ideograph
    (0x215A34, (0x8CEA, false)), // East Asian ideograph
    (0x275A35, (0x8D4C, false)), // East Asian ideograph
    (0x215A36, (0x8CF4, false)), // East Asian ideograph
    (0x215A37, (0x8CFD, false)), // East Asian ideograph
    (0x275A38, (0x8D5A, false)), // East Asian ideograph
    (0x275A39, (0x8D58, false)), // East Asian ideograph
    (0x275A3A, (0x8D2D, false)), // East Asian ideograph
    (0x275A3B, (0x8D60, false)), // East Asian ideograph
    (0x275A3C, (0x8D5D, false)), // East Asian ideograph
    (0x275A3D, (0x8D5E, false)), // East Asian ideograph
    (0x275A3E, (0x8D62, false)), // East Asian ideograph
    (0x275A3F, (0x8D61, false)), // East Asian ideograph
    (0x275A40, (0x8D43, false)), // East Asian ideograph
    (0x275A41, (0x8D4E, false)), // East Asian ideograph
    (0x275A42, (0x8D63, false)), // East Asian ideograph
    (0x215A43, (0x8D64, false)), // East Asian ideograph
    (0x215A44, (0x8D67, false)), // East Asian ideograph
    (0x215A45, (0x8D66, false)), // East Asian ideograph
    (0x215A46, (0x8D6B, false)), // East Asian ideograph
    (0x215A47, (0x8D6D, false)), // East Asian ideograph
    (0x235A48, (0x9D1F, false)), // East Asian ideograph
    (0x215A49, (0x8D74, false)), // East Asian ideograph
    (0x215A4A, (0x8D73, false)), // East Asian ideograph
    (0x215A4B, (0x8D77, false)), // East Asian ideograph
    (0x215A4C, (0x8D85, false)), // East Asian ideograph
    (0x215A4D, (0x8D8A, false)), // East Asian ideograph
    (0x215A4E, (0x8D81, false)), // East Asian ideograph
    (0x275A4F, (0x8D75, false)), // East Asian ideograph
    (0x215A50, (0x8D95, false)), // East Asian ideograph
    (0x215A51, (0x8DA3, false)), // East Asian ideograph
    (0x215A52, (0x8D9F, false)), // East Asian ideograph
    (0x275A53, (0x8D8B, false)), // East Asian ideograph
    (0x215A54, (0x8DB3, false)), // East Asian ideograph
    (0x215A55, (0x8DB4, false)), // East Asian ideograph
    (0x215A56, (0x8DBE, false)), // East Asian ideograph
    (0x215A57, (0x8DCE, false)), // East Asian ideograph
    (0x215A58, (0x8DDD, false)), // East Asian ideograph
    (0x214B33, (0x738B, false)), // East Asian ideograph
    (0x215A5A, (0x8DCB, false)), // East Asian ideograph
    (0x215A5B, (0x8DDA, false)), // East Asian ideograph
    (0x215A5C, (0x8DC6, false)), // East Asian ideograph
    (0x215A5D, (0x8DD1, false)), // East Asian ideograph
    (0x215A5E, (0x8DCC, false)), // East Asian ideograph
    (0x215A5F, (0x8DE1, false)), // East Asian ideograph
    (0x215A60, (0x8DDF, false)), // East Asian ideograph
    (0x215A61, (0x8DE8, false)), // East Asian ideograph
    (0x225A62, (0x7473, false)), // East Asian ideograph
    (0x235A63, (0x9D3E, false)), // East Asian ideograph
    (0x215A64, (0x8DEA, false)), // East Asian ideograph
    (0x215A65, (0x8DEF, false)), // East Asian ideograph
    (0x215A66, (0x8DFC, false)), // East Asian ideograph
    (0x215A67, (0x8E2B, false)), // East Asian ideograph
    (0x235A68, (0x9D42, false)), // East Asian ideograph
    (0x235A69, (0x9D40, false)), // East Asian ideograph
    (0x215A6A, (0x8E1D, false)), // East Asian ideograph
    (0x215A6B, (0x8E0F, false)), // East Asian ideograph
    (0x215A6C, (0x8E29, false)), // East Asian ideograph
    (0x215A6D, (0x8E1F, false)), // East Asian ideograph
    (0x215A6E, (0x8E44, false)), // East Asian ideograph
    (0x215A6F, (0x8E31, false)), // East Asian ideograph
    (0x215A70, (0x8E42, false)), // East Asian ideograph
    (0x215A71, (0x8E34, false)), // East Asian ideograph
    (0x215A72, (0x8E39, false)), // East Asian ideograph
    (0x215A73, (0x8E35, false)), // East Asian ideograph
    (0x215A74, (0x8E49, false)), // East Asian ideograph
    (0x235A75, (0x9D53, false)), // East Asian ideograph
    (0x215A76, (0x8E48, false)), // East Asian ideograph
    (0x215A77, (0x8E4A, false)), // East Asian ideograph
    (0x215A78, (0x8E63, false)), // East Asian ideograph
    (0x215A79, (0x8E59, false)), // East Asian ideograph
    (0x215A7A, (0x8E66, false)), // East Asian ideograph
    (0x215A7B, (0x8E64, false)), // East Asian ideograph
    (0x215A7C, (0x8E72, false)), // East Asian ideograph
    (0x215A7D, (0x8E6C, false)), // East Asian ideograph
    (0x275A7E, (0x8DF7, false)), // East Asian ideograph
    (0x6F5439, (0xC313, false)), // Korean hangul
    (0x6F5443, (0xC343, false)), // Korean hangul
    (0x22646B, (0x789A, false)), // East Asian ideograph
    (0x213A28, (0x5A41, false)), // East Asian ideograph
    (0x6F2458, (0x3134, false)), // Korean hangul
    (0x234226, (0x922B, false)), // East Asian ideograph
    (0x27515C, (0x7EBF, false)), // East Asian ideograph
    (0x475222, (0x9957, false)), // East Asian ideograph
    (0x6F246E, (0x3143, false)), // Korean hangul
    (0x335333, (0x80BB, false)), // East Asian ideograph
    (0x6F2470, (0x3146, false)), // Korean hangul
    (0x692471, (0x3091, false)), // Hiragana letter WE
    (0x232472, (0x852F, false)), // East Asian ideograph
    (0x6F2473, (0x3150, false)), // Korean hangul
    (0x224F35, (0x7021, false)), // East Asian ideograph
    (0x6F2474, (0x3151, false)), // Korean hangul
    (0x6F5445, (0xC368, false)), // Korean hangul
    (0x6F5A31, (0xCF08, false)), // Korean hangul
    (0x6F2476, (0x3153, false)), // Korean hangul
    (0x295729, (0x9C87, false)), // East Asian ideograph
    (0x6F4F21, (0xB7FD, false)), // Korean hangul
    (0x232477, (0x84F7, false)), // East Asian ideograph
    (0x274F22, (0x4E07, false)), // East Asian ideograph
    (0x234F23, (0x980E, false)), // East Asian ideograph
    (0x6F4E42, (0xB611, false)), // Korean hangul
    (0x224F24, (0x7020, false)), // East Asian ideograph
    (0x6F5446, (0xC369, false)), // Korean hangul
    (0x6F247A, (0x3157, false)), // Korean hangul
    (0x274F25, (0x53B6, false)), // East Asian ideograph
    (0x4B333E, (0xF92E, false)), // East Asian ideograph
    (0x224F26, (0x7027, false)), // East Asian ideograph
    (0x214F27, (0x79C9, false)), // East Asian ideograph
    (0x22537A, (0x71BF, false)), // East Asian ideograph
    (0x22537C, (0x71B8, false)), // East Asian ideograph
    (0x29247D, (0x835C, false)), // East Asian ideograph
    (0x6F4F28, (0xB80C, false)), // Korean hangul
    (0x212A38, (0xE8E5, false)), // EACC component character
    (0x276775, (0x4F65, false)), // East Asian ideograph
    (0x6F5447, (0xC36C, false)), // Korean hangul
    (0x6F4F2A, (0xB818, false)), // Korean hangul
    (0x23422A, (0x9292, false)), // East Asian ideograph
    (0x2D5D56, (0x920E, false)), // East Asian ideograph
    (0x234F2C, (0x9826, false)), // East Asian ideograph
    (0x6F5D5A, (0xD6FC, false)), // Korean hangul
    (0x234F2D, (0x981E, false)), // East Asian ideograph
    (0x6F7652, (0xE8C4, false)), // Korean hangul
    (0x6F4F2E, (0xB824, false)), // Korean hangul
    (0x2D3C26, (0x5D18, false)), // East Asian ideograph
    (0x6F5448, (0xC370, false)), // Korean hangul
    (0x213E70, (0x60FB, false)), // East Asian ideograph
    (0x224F2F, (0x702E, false)), // East Asian ideograph
    (0x225B21, (0x7489, false)), // East Asian ideograph
    (0x225B22, (0x747C, false)), // East Asian ideograph
    (0x215B23, (0x8E82, false)), // East Asian ideograph
    (0x215B24, (0x8E81, false)), // East Asian ideograph
    (0x215B25, (0x8E87, false)), // East Asian ideograph
    (0x215B26, (0x8E89, false)), // East Asian ideograph
    (0x214F31, (0x79FB, false)), // East Asian ideograph
    (0x225B28, (0x747E, false)), // East Asian ideograph
    (0x275B29, (0x8DC3, false)), // East Asian ideograph
    (0x235B2A, (0x9D52, false)), // East Asian ideograph
    (0x215B2B, (0x8EA1, false)), // East Asian ideograph
    (0x235B2C, (0x9D77, false)), // East Asian ideograph
    (0x224F39, (0x7018, false)), // East Asian ideograph
    (0x215B2E, (0x8EAC, false)), // East Asian ideograph
    (0x215B2F, (0x8EB2, false)), // East Asian ideograph
    (0x225B30, (0x747A, false)), // East Asian ideograph
    (0x215B31, (0x8EC0, false)), // East Asian ideograph
    (0x215B32, (0x8ECA, false)), // East Asian ideograph
    (0x275B33, (0x8F67, false)), // East Asian ideograph
    (0x215B34, (0x8ECD, false)), // East Asian ideograph
    (0x215B35, (0x8ECC, false)), // East Asian ideograph
    (0x215B36, (0x8ED2, false)), // East Asian ideograph
    (0x215B37, (0x8ED4, false)), // East Asian ideograph
    (0x215B38, (0x8EDF, false)), // East Asian ideograph
    (0x215B39, (0x8EDB, false)), // East Asian ideograph
    (0x215B3A, (0x8EFB, false)), // East Asian ideograph
    (0x275B3B, (0x8F74, false)), // East Asian ideograph
    (0x215B3C, (0x8EFC, false)), // East Asian ideograph
    (0x215B3D, (0x8F03, false)), // East Asian ideograph
    (0x225B3E, (0x747D, false)), // East Asian ideograph
    (0x235B3F, (0x9D78, false)), // East Asian ideograph
    (0x215B40, (0x8F0A, false)), // East Asian ideograph
    (0x215B41, (0x8F14, false)), // East Asian ideograph
    (0x235B42, (0x9D7E, false)), // East Asian ideograph
    (0x215B43, (0x8F15, false)), // East Asian ideograph
    (0x215B44, (0x8F13, false)), // East Asian ideograph
    (0x215B45, (0x8F26, false)), // East Asian ideograph
    (0x215B46, (0x8F1B, false)), // East Asian ideograph
    (0x235B47, (0x9D69, false)), // East Asian ideograph
    (0x215B48, (0x8F1D, false)), // East Asian ideograph
    (0x215B49, (0x8F29, false)), // East Asian ideograph
    (0x215B4A, (0x8F2A, false)), // East Asian ideograph
    (0x275B4B, (0x8F8E, false)), // East Asian ideograph
    (0x215B4C, (0x8F3B, false)), // East Asian ideograph
    (0x215B4D, (0x8F2F, false)), // East Asian ideograph
    (0x215B4E, (0x8F38, false)), // East Asian ideograph
    (0x235B4F, (0x9D83, false)), // East Asian ideograph
    (0x215B50, (0x8F3E, false)), // East Asian ideograph
    (0x215B51, (0x8F45, false)), // East Asian ideograph
    (0x215B52, (0x8F42, false)), // East Asian ideograph (variant of 4B5B52 which maps to 8F42)
    (0x215B53, (0x8F3F, false)), // East Asian ideograph
    (0x225B54, (0x749F, false)), // East Asian ideograph
    (0x225B55, (0x749D, false)), // East Asian ideograph
    (0x215B56, (0x8F54, false)), // East Asian ideograph
    (0x225B57, (0x749E, false)), // East Asian ideograph
    (0x215B58, (0x8F5F, false)), // East Asian ideograph
    (0x215B59, (0x8F61, false)), // East Asian ideograph
    (0x215B5A, (0x8F9B, false)), // East Asian ideograph
    (0x215B5B, (0x8F9C, false)), // East Asian ideograph
    (0x215B5C, (0x8F9F, false)), // East Asian ideograph
    (0x224F3A, (0x7023, false)), // East Asian ideograph
    (0x235B5E, (0x9D92, false)), // East Asian ideograph
    (0x215B5F, (0x8FA6, false)), // East Asian ideograph
    (0x225B60, (0x74B2, false)), // East Asian ideograph
    (0x215B61, (0x8FAF, false)), // East Asian ideograph
    (0x215B62, (0x8FB0, false)), // East Asian ideograph
    (0x215B63, (0x8FB1, false)), // East Asian ideograph
    (0x215B64, (0x8FB2, false)), // East Asian ideograph
    (0x295E6A, (0x9EEA, false)), // East Asian ideograph
    (0x225B66, (0x74B4, false)), // East Asian ideograph
    (0x225B67, (0x74AB, false)), // East Asian ideograph
    (0x215B68, (0x8FC4, false)), // East Asian ideograph
    (0x215B69, (0x5DE1, false)), // East Asian ideograph
    (0x215B6A, (0x8FCE, false)), // East Asian ideograph
    (0x215B6B, (0x8FD1, false)), // East Asian ideograph
    (0x215B6C, (0x8FD4, false)), // East Asian ideograph
    (0x275B6D, (0x8FF0, false)), // East Asian ideograph
    (0x215B6E, (0x8FE6, false)), // East Asian ideograph
    (0x215B6F, (0x8FE2, false)), // East Asian ideograph
    (0x235B70, (0x9D96, false)), // East Asian ideograph
    (0x215B71, (0x8FE5, false)), // East Asian ideograph
    (0x6F544B, (0xC379, false)), // Korean hangul
    (0x215B73, (0x8FEB, false)), // East Asian ideograph
    (0x215B74, (0x9001, false)), // East Asian ideograph
    (0x215B75, (0x9006, false)), // East Asian ideograph
    (0x225B76, (0x74B8, false)), // East Asian ideograph
    (0x215B77, (0x9000, false)), // East Asian ideograph
    (0x6F5B78, (0xD329, false)), // Korean hangul
    (0x235B79, (0x9DC0, false)), // East Asian ideograph
    (0x225B7A, (0x74C0, false)), // East Asian ideograph
    (0x23422E, (0x9207, false)), // East Asian ideograph
    (0x215B7C, (0x9005, false)), // East Asian ideograph
    (0x215B7D, (0x9019, false)), // East Asian ideograph
    (0x215B7E, (0x9023, false)), // East Asian ideograph
    (0x214F40, (0x7A40, false)), // East Asian ideograph
    (0x393C52, (0x8D26, false)), // East Asian ideograph
    (0x224F41, (0x703C, false)), // East Asian ideograph
    (0x213941, (0x596E, false)), // East Asian ideograph
    (0x6F4F42, (0xB8E1, false)), // Korean hangul
    (0x6F544C, (0xC37C, false)), // Korean hangul
    (0x4B4F43, (0x7A32, false)), // East Asian ideograph
    (0x213A31, (0x5A9B, false)), // East Asian ideograph
    (0x224F44, (0x7035, false)), // East Asian ideograph
    (0x213A41, (0x5AFB, false)), // East Asian ideograph
    (0x234F45, (0x9832, false)), // East Asian ideograph
    (0x294331, (0x94F1, false)), // East Asian ideograph
    (0x274F46, (0x7A23, false)), // East Asian ideograph
    (0x6F4F47, (0xB8F8, false)), // Korean hangul
    (0x2D4F48, (0x7A42, false)), // East Asian ideograph
    (0x213A32, (0x5ACC, false)), // East Asian ideograph
    (0x294231, (0x94AF, false)), // East Asian ideograph
    (0x214F49, (0x7A61, false)), // East Asian ideograph
    (0x4C3744, (0x65D9, false)), // East Asian ideograph
    (0x274F4A, (0x79FD, false)), // East Asian ideograph
    (0x214F4B, (0x7A6B, false)), // East Asian ideograph
    (0x227631, (0x8008, false)), // East Asian ideograph
    (0x274F4C, (0x7A33, false)), // East Asian ideograph
    (0x6F544E, (0xC384, false)), // Korean hangul
    (0x6F4F4D, (0xB958, false)), // Korean hangul
    (0x6F5361, (0xC229, false)), // Korean hangul
    (0x213A33, (0x5AC1, false)), // East Asian ideograph
    (0x234F4E, (0x9844, false)), // East Asian ideograph
    (0x4B393A, (0x5F09, false)), // East Asian ideograph
    (0x6F4F4F, (0xB95C, false)), // Korean hangul
    (0x334755, (0x6FEC, false)), // East Asian ideograph
    (0x6F4F50, (0xB960, false)), // Korean hangul
    (0x6F4C69, (0xB2DD, false)), // Korean hangul
    (0x23533E, (0x9A0B, false)), // East Asian ideograph
    (0x224F51, (0x7034, false)), // East Asian ideograph
    (0x274564, (0x69DF, false)), // East Asian ideograph
    (0x6F544F, (0xC388, false)), // Korean hangul
    (0x213E77, (0x611B, false)), // East Asian ideograph
    (0x6F4F52, (0xB96D, false)), // Korean hangul
    (0x213A34, (0x5AC9, false)), // East Asian ideograph
    (0x217E79, (0x5BE0, false)), // East Asian ideograph
    (0x224F53, (0x7039, false)), // East Asian ideograph
    (0x224F54, (0x703A, false)), // East Asian ideograph
    (0x395E6F, (0x7A7D, false)), // East Asian ideograph
    (0x6F4F55, (0xB978, false)), // Korean hangul
    (0x23533F, (0x9A09, false)), // East Asian ideograph
    (0x6F4E44, (0xB618, false)), // Korean hangul
    (0x6F4F56, (0xB97C, false)), // Korean hangul
    (0x6F5450, (0xC399, false)), // Korean hangul
    (0x6F4F57, (0xB984, false)), // Korean hangul
    (0x213A35, (0x5ABE, false)), // East Asian ideograph
    (0x696B27, (0x8977, false)), // East Asian ideograph
    (0x234233, (0x91FE, false)), // East Asian ideograph
    (0x3A2F7C, (0x64C0, false)), // East Asian ideograph
    (0x395A2F, (0x58F2, false)), // East Asian ideograph
    (0x334F59, (0x7A93, false)), // East Asian ideograph
    (0x293C57, (0x8F79, false)), // East Asian ideograph
    (0x6F4F5A, (0xB989, false)), // Korean hangul
    (0x215C21, (0x901F, false)), // East Asian ideograph
    (0x215C22, (0x9017, false)), // East Asian ideograph
    (0x215C23, (0x901D, false)), // East Asian ideograph
    (0x215C24, (0x9010, false)), // East Asian ideograph
    (0x225C25, (0x74BF, false)), // East Asian ideograph
    (0x215C26, (0x900D, false)), // East Asian ideograph
    (0x215C27, (0x901E, false)), // East Asian ideograph
    (0x235C28, (0x9DBB, false)), // East Asian ideograph
    (0x274123, (0x6302, false)), // East Asian ideograph
    (0x215C2A, (0x900F, false)), // East Asian ideograph
    (0x215C2B, (0x9022, false)), // East Asian ideograph
    (0x215C2C, (0x9016, false)), // East Asian ideograph
    (0x215C2D, (0x901B, false)), // East Asian ideograph
    (0x215C2E, (0x9014, false)), // East Asian ideograph
    (0x214F5D, (0x7AA9, false)), // East Asian ideograph
    (0x215C30, (0x9035, false)), // East Asian ideograph
    (0x215C31, (0x9031, false)), // East Asian ideograph
    (0x235C32, (0x9DB9, false)), // East Asian ideograph
    (0x275C33, (0x8FDB, false)), // East Asian ideograph
    (0x275C34, (0x8FD0, false)), // East Asian ideograph
    (0x2D4F5E, (0x7AB0, false)), // East Asian ideograph
    (0x215C36, (0x9053, false)), // East Asian ideograph
    (0x215C37, (0x9042, false)), // East Asian ideograph
    (0x215C38, (0x9050, false)), // East Asian ideograph
    (0x275C39, (0x8FBE, false)), // East Asian ideograph
    (0x275C3A, (0x8FDD, false)), // East Asian ideograph
    (0x274F5F, (0x7A77, false)), // East Asian ideograph
    (0x275C3C, (0x8FC2, false)), // East Asian ideograph
    (0x215C3D, (0x904F, false)), // East Asian ideograph
    (0x235C3E, (0x9DD9, false)), // East Asian ideograph
    (0x215C3F, (0x904D, false)), // East Asian ideograph
    (0x215C40, (0x9051, false)), // East Asian ideograph
    (0x214F60, (0x7ABA, false)), // East Asian ideograph
    (0x215C42, (0x903E, false)), // East Asian ideograph
    (0x215C43, (0x9058, false)), // East Asian ideograph
    (0x275C44, (0x8FDC, false)), // East Asian ideograph
    (0x275C45, (0x900A, false)), // East Asian ideograph
    (0x215C46, (0x9063, false)), // East Asian ideograph
    (0x214F61, (0x7AC5, false)), // East Asian ideograph
    (0x275C48, (0x9012, false)), // East Asian ideograph
    (0x215C49, (0x9069, false)), // East Asian ideograph
    (0x215C4A, (0x906E, false)), // East Asian ideograph
    (0x215C4B, (0x9068, false)), // East Asian ideograph
    (0x215C4C, (0x906D, false)), // East Asian ideograph
    (0x214F62, (0x7AC4, false)), // East Asian ideograph
    (0x215C4E, (0x9074, false)), // East Asian ideograph
    (0x275C4F, (0x9009, false)), // East Asian ideograph
    (0x275C50, (0x8FDF, false)), // East Asian ideograph
    (0x215C51, (0x9077, false)), // East Asian ideograph
    (0x215C52, (0x907C, false)), // East Asian ideograph
    (0x275C53, (0x9057, false)), // East Asian ideograph
    (0x215C54, (0x907F, false)), // East Asian ideograph
    (0x215C55, (0x907D, false)), // East Asian ideograph
    (0x275C56, (0x8FC8, false)), // East Asian ideograph
    (0x235C57, (0x9DF2, false)), // East Asian ideograph
    (0x215C58, (0x9082, false)), // East Asian ideograph
    (0x215C59, (0x9080, false)), // East Asian ideograph
    (0x275C5A, (0x8FE9, false)), // East Asian ideograph (variant of 2D5C5A which maps to 8FE9)
    (0x275C5B, (0x8FB9, false)), // East Asian ideograph
    (0x275C5C, (0x9026, false)), // East Asian ideograph
    (0x275C5D, (0x903B, false)), // East Asian ideograph
    (0x215C5E, (0x9091, false)), // East Asian ideograph
    (0x215C5F, (0x9095, false)), // East Asian ideograph
    (0x215C60, (0x90A3, false)), // East Asian ideograph
    (0x215C61, (0x90A2, false)), // East Asian ideograph
    (0x215C62, (0x90AA, false)), // East Asian ideograph
    (0x215C63, (0x90A6, false)), // East Asian ideograph
    (0x215C64, (0x90B5, false)), // East Asian ideograph
    (0x215C65, (0x90B1, false)), // East Asian ideograph
    (0x215C66, (0x90B8, false)), // East Asian ideograph
    (0x215C67, (0x90CE, false)), // East Asian ideograph
    (0x215C68, (0x90CA, false)), // East Asian ideograph
    (0x4B5564, (0x77C7, false)), // East Asian ideograph
    (0x235C6A, (0x9DED, false)), // East Asian ideograph
    (0x215C6B, (0x90E8, false)), // East Asian ideograph
    (0x215C6C, (0x90ED, false)), // East Asian ideograph
    (0x275C6D, (0x90AE, false)), // East Asian ideograph
    (0x215C6E, (0x90FD, false)), // East Asian ideograph
    (0x215C6F, (0x9102, false)), // East Asian ideograph
    (0x275C70, (0x4E61, false)), // East Asian ideograph
    (0x275C71, (0x90B9, false)), // East Asian ideograph
    (0x215C72, (0x9119, false)), // East Asian ideograph
    (0x275C73, (0x90D1, false)), // East Asian ideograph
    (0x275C74, (0x90BB, false)), // East Asian ideograph
    (0x275C75, (0x9093, false)), // East Asian ideograph
    (0x215C76, (0x9131, false)), // East Asian ideograph
    (0x214F69, (0x7AED, false)), // East Asian ideograph
    (0x215C78, (0x9149, false)), // East Asian ideograph
    (0x215C79, (0x914B, false)), // East Asian ideograph
    (0x215C7A, (0x914A, false)), // East Asian ideograph
    (0x215C7B, (0x9152, false)), // East Asian ideograph
    (0x215C7C, (0x914D, false)), // East Asian ideograph
    (0x215C7D, (0x914C, false)), // East Asian ideograph
    (0x215C7E, (0x9157, false)), // East Asian ideograph
    (0x6F5454, (0xC3DF, false)), // Korean hangul
    (0x6F5A34, (0xCF13, false)), // Korean hangul
    (0x214F6B, (0x7AF6, false)), // East Asian ideograph
    (0x213A39, (0x5AB3, false)), // East Asian ideograph
    (0x234237, (0x9226, false)), // East Asian ideograph
    (0x6F4F6D, (0xB9D8, false)), // Korean hangul
    (0x695A31, (0x64F6, false)), // East Asian ideograph
    (0x6F4E45, (0xB625, false)), // Korean hangul
    (0x234F6F, (0x9857, false)), // East Asian ideograph
    (0x27456A, (0x6988, false)), // East Asian ideograph
    (0x213E7D, (0x6108, false)), // East Asian ideograph
    (0x395821, (0x97E4, false)), // East Asian ideograph
    (0x274F70, (0x5DF4, false)), // East Asian ideograph (duplicate simplified)
    (0x213A3A, (0x5AE1, false)), // East Asian ideograph
    (0x224F71, (0x7052, false)), // East Asian ideograph
    (0x234F72, (0x9856, false)), // East Asian ideograph
    (0x295E7A, (0x9EFE, false)), // East Asian ideograph
    (0x224F73, (0x705C, false)), // East Asian ideograph
    (0x213F39, (0x6163, false)), // East Asian ideograph
    (0x6F4F74, (0xB9E4, false)), // Korean hangul
    (0x6F5456, (0xC3E8, false)), // Korean hangul
    (0x213E7E, (0x60F1, false)), // East Asian ideograph
    (0x214F75, (0x7B1B, false)), // East Asian ideograph
    (0x213A3B, (0x5AD7, false)), // East Asian ideograph
    (0x284E66, (0x6EE2, false)), // East Asian ideograph
    (0x213A21, (0x5A46, false)), // East Asian ideograph
    (0x234F77, (0x9862, false)), // East Asian ideograph
    (0x275235, (0x7F62, false)), // East Asian ideograph
    (0x224F78, (0x7059, false)), // East Asian ideograph
    (0x213A23, (0x5A6A, false)), // East Asian ideograph
    (0x274F79, (0x7B14, false)), // East Asian ideograph
    (0x213A24, (0x5A36, false)), // East Asian ideograph
    (0x6F5457, (0xC3ED, false)), // Korean hangul
    (0x22427E, (0x6AED, false)), // East Asian ideograph
    (0x214F7B, (0x7B50, false)), // East Asian ideograph
    (0x213A26, (0x5A40, false)), // East Asian ideograph
    (0x695E63, (0x6E82, false)), // East Asian ideograph
    (0x275679, (0x80E1, false)), // East Asian ideograph (duplicate simplified)
    (0x224F7C, (0x7061, false)), // East Asian ideograph
    (0x213A27, (0x5A66, false)), // East Asian ideograph
    (0x224F7D, (0x705D, false)), // East Asian ideograph
    (0x223A28, (0x6705, false)), // East Asian ideograph
    (0x335347, (0x81D9, false)), // East Asian ideograph
    (0x293B4F, (0x8F78, false)), // East Asian ideograph
    (0x234F7E, (0x9868, false)), // East Asian ideograph
    (0x6F4F7B, (0xB9F8, false)), // Korean hangul
    (0x6F5458, (0xC3F4, false)), // Korean hangul
    (0x6F5363, (0xC22D, false)), // Korean hangul
    (0x2D5D68, (0x8021, false)), // East Asian ideograph
    (0x394928, (0x6D5C, false)), // East Asian ideograph
    (0x29366A, (0x8D53, false)), // East Asian ideograph
    (0x227A2C, (0x81B5, false)), // East Asian ideograph
    (0x223173, (0x637F, false)), // East Asian ideograph
    (0x2D5179, (0x7E62, false)), // East Asian ideograph
    (0x213A2E, (0x5A92, false)), // East Asian ideograph
    (0x6F5459, (0xC3F5, false)), // Korean hangul
    (0x2D3A2F, (0x58FB, false)), // East Asian ideograph
    (0x4B3351, (0x5204, false)), // East Asian ideograph
    (0x215D21, (0x9163, false)), // East Asian ideograph
    (0x215D22, (0x9165, false)), // East Asian ideograph
    (0x215D23, (0x916C, false)), // East Asian ideograph
    (0x215D24, (0x9169, false)), // East Asian ideograph
    (0x215D25, (0x916A, false)), // East Asian ideograph
    (0x215D26, (0x9175, false)), // East Asian ideograph
    (0x215D27, (0x9178, false)), // East Asian ideograph
    (0x215D28, (0x9177, false)), // East Asian ideograph
    (0x215D29, (0x9187, false)), // East Asian ideograph
    (0x215D2A, (0x9189, false)), // East Asian ideograph
    (0x215D2B, (0x918B, false)), // East Asian ideograph
    (0x215D2C, (0x9183, false)), // East Asian ideograph
    (0x215D2D, (0x9192, false)), // East Asian ideograph
    (0x215D2E, (0x91A3, false)), // East Asian ideograph
    (0x275D2F, (0x915D, false)), // East Asian ideograph
    (0x215D30, (0x919C, false)), // East Asian ideograph
    (0x275D31, (0x533B, false)), // East Asian ideograph
    (0x225D32, (0x7512, false)), // East Asian ideograph
    (0x215D33, (0x91BA, false)), // East Asian ideograph
    (0x275D34, (0x917F, false)), // East Asian ideograph
    (0x275D35, (0x8845, false)), // East Asian ideograph
    (0x215D36, (0x91C7, false)), // East Asian ideograph
    (0x215D37, (0x91C9, false)), // East Asian ideograph
    (0x215D38, (0x91CB, false)), // East Asian ideograph
    (0x235D39, (0x9E1C, false)), // East Asian ideograph
    (0x235D3A, (0x9E1B, false)), // East Asian ideograph
    (0x215D3B, (0x91CE, false)), // East Asian ideograph
    (0x235D3C, (0x9E75, false)), // East Asian ideograph
    (0x275D3D, (0x5398, false)), // East Asian ideograph
    (0x215D3E, (0x91D1, false)), // East Asian ideograph
    (0x215D3F, (0x91DD, false)), // East Asian ideograph
    (0x215D40, (0x91D8, false)), // East Asian ideograph
    (0x215D41, (0x91D7, false)), // East Asian ideograph
    (0x235D42, (0x9E7A, false)), // East Asian ideograph
    (0x215D43, (0x91F5, false)), // East Asian ideograph
    (0x225D44, (0x7524, false)), // East Asian ideograph
    (0x215D45, (0x91E3, false)), // East Asian ideograph
    (0x215D46, (0x91E7, false)), // East Asian ideograph
    (0x235D47, (0x9E80, false)), // East Asian ideograph
    (0x215D48, (0x920D, false)), // East Asian ideograph
    (0x215D49, (0x9215, false)), // East Asian ideograph
    (0x215D4A, (0x9209, false)), // East Asian ideograph
    (0x275D4B, (0x949E, false)), // East Asian ideograph
    (0x215D4C, (0x921E, false)), // East Asian ideograph
    (0x215D4D, (0x9210, false)), // East Asian ideograph
    (0x2D4C5D, (0x7661, false)), // East Asian ideograph
    (0x225D4F, (0x753F, false)), // East Asian ideograph
    (0x215D50, (0x9238, false)), // East Asian ideograph
    (0x225D51, (0x7540, false)), // East Asian ideograph
    (0x225D52, (0x753E, false)), // East Asian ideograph
    (0x215D53, (0x9240, false)), // East Asian ideograph
    (0x215D54, (0x924B, false)), // East Asian ideograph
    (0x235D55, (0x9E90, false)), // East Asian ideograph
    (0x215D56, (0x9264, false)), // East Asian ideograph
    (0x215D57, (0x9251, false)), // East Asian ideograph
    (0x235D58, (0x9E8C, false)), // East Asian ideograph
    (0x215D59, (0x9278, false)), // East Asian ideograph
    (0x215D5A, (0x9280, false)), // East Asian ideograph
    (0x275D5B, (0x94D0, false)), // East Asian ideograph
    (0x215D5C, (0x9285, false)), // East Asian ideograph
    (0x235D5D, (0x9E9B, false)), // East Asian ideograph
    (0x215D5E, (0x9296, false)), // East Asian ideograph
    (0x225D5F, (0x755F, false)), // East Asian ideograph
    (0x215D60, (0x9293, false)), // East Asian ideograph
    (0x275D61, (0x8854, false)), // East Asian ideograph
    (0x215D62, (0x92C5, false)), // East Asian ideograph
    (0x215D63, (0x92BB, false)), // East Asian ideograph
    (0x275D64, (0x9510, false)), // East Asian ideograph
    (0x275D65, (0x94FA, false)), // East Asian ideograph
    (0x275D66, (0x9500, false)), // East Asian ideograph
    (0x215D67, (0x92C1, false)), // East Asian ideograph
    (0x215D68, (0x92E4, false)), // East Asian ideograph
    (0x215D69, (0x92BC, false)), // East Asian ideograph
    (0x215D6A, (0x92D2, false)), // East Asian ideograph
    (0x225D6B, (0x756C, false)), // East Asian ideograph
    (0x215D6C, (0x9336, false)), // East Asian ideograph
    (0x275D6D, (0x952F, false)), // East Asian ideograph
    (0x215D6E, (0x9333, false)), // East Asian ideograph
    (0x215D6F, (0x932F, false)), // East Asian ideograph
    (0x215D70, (0x9322, false)), // East Asian ideograph
    (0x215D71, (0x92FC, false)), // East Asian ideograph
    (0x215D72, (0x932B, false)), // East Asian ideograph
    (0x215D73, (0x931A, false)), // East Asian ideograph
    (0x215D74, (0x9304, false)), // East Asian ideograph
    (0x213A3E, (0x5AE9, false)), // East Asian ideograph
    (0x275D76, (0x9526, false)), // East Asian ideograph
    (0x275D77, (0x9540, false)), // East Asian ideograph
    (0x275D78, (0x9541, false)), // East Asian ideograph
    (0x235D79, (0x9EAF, false)), // East Asian ideograph
    (0x215D7A, (0x9365, false)), // East Asian ideograph
    (0x213A3F, (0x5AD8, false)), // East Asian ideograph
    (0x215D7C, (0x934B, false)), // East Asian ideograph
    (0x215D7D, (0x9328, false)), // East Asian ideograph
    (0x215D7E, (0x9370, false)), // East Asian ideograph
    (0x4B5E3F, (0x922C, false)), // East Asian ideograph
    (0x213A40, (0x5AE6, false)), // East Asian ideograph
    (0x6F5367, (0xC234, false)), // Korean hangul
    (0x233A41, (0x8E61, false)), // East Asian ideograph
    (0x6F5825, (0xC990, false)), // Korean hangul
    (0x6F545D, (0xC434, false)), // Korean hangul
    (0x284971, (0x6D9E, false)), // East Asian ideograph
    (0x224A32, (0x6DFC, false)), // East Asian ideograph
    (0x227A43, (0x81D0, false)), // East Asian ideograph
    (0x213A44, (0x5B0C, false)), // East Asian ideograph
    (0x29366B, (0x8D55, false)), // East Asian ideograph
    (0x233A45, (0x8E74, false)), // East Asian ideograph
    (0x213A46, (0x5B34, false)), // East Asian ideograph
    (0x213A47, (0x5B1D, false)), // East Asian ideograph
    (0x6F545E, (0xC43C, false)), // Korean hangul
    (0x6F5A36, (0xCF1C, false)), // Korean hangul
    (0x273A48, (0x5AD4, false)), // East Asian ideograph
    (0x213A43, (0x5B0B, false)), // East Asian ideograph
    (0x395829, (0x69FB, false)), // East Asian ideograph
    (0x6F4C3E, (0xB1D0, false)), // Korean hangul
    (0x4B3A49, (0x5B37, false)), // East Asian ideograph
    (0x2D3B54, (0x5C4A, false)), // East Asian ideograph
    (0x6F5923, (0xCC2E, false)), // Korean hangul
    (0x213A4A, (0x5B30, false)), // East Asian ideograph
    (0x287855, (0x80EB, false)), // East Asian ideograph
    (0x233A4B, (0x8E69, false)), // East Asian ideograph
    (0x213A4C, (0x5B40, false)), // East Asian ideograph
    (0x6F545F, (0xC43F, false)), // Korean hangul
    (0x213A4D, (0x5B50, false)), // East Asian ideograph
    (0x213A4E, (0x5B51, false)), // East Asian ideograph
    (0x217A4F, (0x59EE, false)), // East Asian ideograph
    (0x225B3F, (0x7485, false)), // East Asian ideograph
    (0x295E7C, (0x9F0B, false)), // East Asian ideograph
    (0x29456F, (0x9538, false)), // East Asian ideograph
    (0x6F5460, (0xC464, false)), // Korean hangul
    (0x233A52, (0x8E83, false)), // East Asian ideograph
    (0x213A45, (0x5AF5, false)), // East Asian ideograph
    (0x334243, (0x52B9, false)), // East Asian ideograph
    (0x233A53, (0x8E84, false)), // East Asian ideograph
    (0x2D592C, (0x8B01, false)), // East Asian ideograph
    (0x294335, (0x94F5, false)), // East Asian ideograph
    (0x4C3A55, (0x6741, false)), // East Asian ideograph
    (0x4B623B, (0x9D12, false)), // East Asian ideograph
    (0x217A56, (0x59FD, false)), // East Asian ideograph
    (0x6F5461, (0xC465, false)), // Korean hangul
    (0x213A57, (0x5B5F, false)), // East Asian ideograph
    (0x6F5A39, (0xCF2C, false)), // Korean hangul
    (0x213A58, (0x5B63, false)), // East Asian ideograph
    (0x692544, (0x30C4, false)), // Katakana letter TU
    (0x225622, (0x728D, false)), // East Asian ideograph
    (0x215E21, (0x937E, false)), // East Asian ideograph
    (0x275E22, (0x9524, false)), // East Asian ideograph
    (0x275E23, (0x9539, false)), // East Asian ideograph
    (0x215E24, (0x935B, false)), // East Asian ideograph
    (0x275E25, (0x9551, false)), // East Asian ideograph
    (0x215E26, (0x9394, false)), // East Asian ideograph
    (0x275E27, (0x9547, false)), // East Asian ideograph
    (0x275E28, (0x9501, false)), // East Asian ideograph
    (0x215E29, (0x93A2, false)), // East Asian ideograph
    (0x275E2A, (0x954D, false)), // East Asian ideograph
    (0x275E2B, (0x955C, false)), // East Asian ideograph
    (0x275E2C, (0x955D, false)), // East Asian ideograph
    (0x215E2D, (0x93D6, false)), // East Asian ideograph
    (0x275E2E, (0x955E, false)), // East Asian ideograph
    (0x215E2F, (0x93DF, false)), // East Asian ideograph
    (0x275E30, (0x94FF, false)), // East Asian ideograph
    (0x275E31, (0x94FE, false)), // East Asian ideograph
    (0x215E32, (0x93E2, false)), // East Asian ideograph
    (0x215E33, (0x93DC, false)), // East Asian ideograph
    (0x215E34, (0x93E4, false)), // East Asian ideograph
    (0x225E35, (0x7598, false)), // East Asian ideograph
    (0x215E36, (0x93CD, false)), // East Asian ideograph
    (0x235E37, (0x9EC8, false)), // East Asian ideograph
    (0x215E39, (0x9403, false)), // East Asian ideograph
    (0x215E3A, (0x942E, false)), // East Asian ideograph
    (0x225E3B, (0x75A3, false)), // East Asian ideograph
    (0x215E3C, (0x9433, false)), // East Asian ideograph
    (0x215E3D, (0x9435, false)), // East Asian ideograph
    (0x215E3E, (0x943A, false)), // East Asian ideograph
    (0x215E3F, (0x9438, false)), // East Asian ideograph
    (0x215E40, (0x9432, false)), // East Asian ideograph
    (0x223A60, (0x675D, false)), // East Asian ideograph
    (0x215E42, (0x9451, false)), // East Asian ideograph
    (0x215E43, (0x9444, false)), // East Asian ideograph
    (0x215E44, (0x9463, false)), // East Asian ideograph
    (0x215E45, (0x9460, false)), // East Asian ideograph
    (0x215E46, (0x9472, false)), // East Asian ideograph
    (0x215E47, (0x9470, false)), // East Asian ideograph
    (0x215E48, (0x947E, false)), // East Asian ideograph
    (0x215E49, (0x947C, false)), // East Asian ideograph
    (0x235E4A, (0x9ED0, false)), // East Asian ideograph
    (0x215E4B, (0x947F, false)), // East Asian ideograph
    (0x215E4C, (0x9577, false)), // East Asian ideograph
    (0x215E4D, (0x9580, false)), // East Asian ideograph
    (0x215E4E, (0x9582, false)), // East Asian ideograph
    (0x215E4F, (0x9583, false)), // East Asian ideograph
    (0x215E50, (0x9589, false)), // East Asian ideograph
    (0x215E51, (0x9594, false)), // East Asian ideograph
    (0x215E52, (0x958F, false)), // East Asian ideograph
    (0x235E53, (0x9EDA, false)), // East Asian ideograph
    (0x215E54, (0x9591, false)), // East Asian ideograph
    (0x235E55, (0x9EDF, false)), // East Asian ideograph
    (0x215E56, (0x9592, false)), // East Asian ideograph
    (0x215E57, (0x9598, false)), // East Asian ideograph
    (0x215E58, (0x95A1, false)), // East Asian ideograph
    (0x235E59, (0x9EE5, false)), // East Asian ideograph
    (0x215E5A, (0x95A9, false)), // East Asian ideograph
    (0x215E5B, (0x95A3, false)), // East Asian ideograph
    (0x215E5C, (0x95A5, false)), // East Asian ideograph
    (0x215E5D, (0x95A4, false)), // East Asian ideograph
    (0x215E5E, (0x95B1, false)), // East Asian ideograph
    (0x215E5F, (0x95AD, false)), // East Asian ideograph
    (0x235E60, (0x9EEE, false)), // East Asian ideograph
    (0x215E61, (0x95CA, false)), // East Asian ideograph
    (0x215E62, (0x95CB, false)), // East Asian ideograph
    (0x215E63, (0x95CC, false)), // East Asian ideograph
    (0x215E64, (0x95C8, false)), // East Asian ideograph
    (0x215E65, (0x95C6, false)), // East Asian ideograph
    (0x235E66, (0x9EF0, false)), // East Asian ideograph
    (0x215E67, (0x95D6, false)), // East Asian ideograph
    (0x215E68, (0x95D0, false)), // East Asian ideograph
    (0x215E69, (0x95DC, false)), // East Asian ideograph
    (0x215E6A, (0x95E1, false)), // East Asian ideograph
    (0x215E6B, (0x95E2, false)), // East Asian ideograph
    (0x215E6C, (0x961C, false)), // East Asian ideograph
    (0x215E6D, (0x9621, false)), // East Asian ideograph
    (0x215E6E, (0x9632, false)), // East Asian ideograph
    (0x215E6F, (0x9631, false)), // East Asian ideograph
    (0x215E70, (0x962E, false)), // East Asian ideograph
    (0x215E71, (0x962A, false)), // East Asian ideograph
    (0x215E72, (0x9640, false)), // East Asian ideograph
    (0x215E73, (0x963F, false)), // East Asian ideograph
    (0x215E74, (0x963B, false)), // East Asian ideograph
    (0x215E75, (0x9644, false)), // East Asian ideograph
    (0x215E76, (0x9650, false)), // East Asian ideograph
    (0x235E77, (0x9EFC, false)), // East Asian ideograph
    (0x215E78, (0x964B, false)), // East Asian ideograph
    (0x215E79, (0x964D, false)), // East Asian ideograph
    (0x235E7A, (0x9EFD, false)), // East Asian ideograph
    (0x215E7B, (0x9663, false)), // East Asian ideograph
    (0x235E7C, (0x9EFF, false)), // East Asian ideograph
    (0x215E7D, (0x9661, false)), // East Asian ideograph
    (0x225E7E, (0x7603, false)), // East Asian ideograph
    (0x6F5465, (0xC479, false)), // Korean hangul
    (0x223A6B, (0x6763, false)), // East Asian ideograph
    (0x223A6E, (0x6753, false)), // East Asian ideograph
    (0x2D355C, (0x5434, false)), // East Asian ideograph
    (0x213A6F, (0x5B98, false)), // East Asian ideograph
    (0x6F5466, (0xC480, false)), // Korean hangul
    (0x6F5440, (0xC328, false)), // Korean hangul
    (0x293A70, (0x8E9C, false)), // East Asian ideograph
    (0x213A4B, (0x5B38, false)), // East Asian ideograph
    (0x294936, (0x95F3, false)), // East Asian ideograph
    (0x215821, (0x896A, false)), // East Asian ideograph
    (0x233A71, (0x8EA9, false)), // East Asian ideograph
    (0x692524, (0x30A4, false)), // Katakana letter I
    (0x213A72, (0x5BA5, false)), // East Asian ideograph
    (0x6F595F, (0xCE04, false)), // Korean hangul
    (0x2D3B52, (0x6EBA, false)), // East Asian ideograph
    (0x223A75, (0x6793, false)), // East Asian ideograph
    (0x23424A, (0x9216, false)), // East Asian ideograph
    (0x6F2521, (0x315C, false)), // Korean hangul
    (0x4B4767, (0x6DB5, false)), // East Asian ideograph (variant of 214767 which maps to 6DB5)
    (0x28342C, (0x63BA, false)), // East Asian ideograph
    (0x295A44, (0x9E32, false)), // East Asian ideograph
    (0x223A78, (0x677C, false)), // East Asian ideograph
    (0x692523, (0x30A3, false)), // Katakana letter small I
    (0x292524, (0x848C, false)), // East Asian ideograph
    (0x29322A, (0x8BB5, false)), // East Asian ideograph
    (0x223A7A, (0x679F, false)), // East Asian ideograph
    (0x226065, (0x76A4, false)), // East Asian ideograph
    (0x23424B, (0x9211, false)), // East Asian ideograph
    (0x4D472C, (0x952A, false)), // East Asian ideograph
    (0x4D5934, (0x9CA6, false)), // East Asian ideograph
    (0x213A7C, (0x5BAE, false)), // East Asian ideograph
    (0x692527, (0x30A7, false)), // Katakana letter small E
    (0x213D2E, (0x5EEC, false)), // East Asian ideograph
    (0x233A7D, (0x8EB6, false)), // East Asian ideograph
    (0x692528, (0x30A8, false)), // Katakana letter E
    (0x6F5469, (0xC4D5, false)), // Korean hangul
    (0x69252A, (0x30AA, false)), // Katakana letter O
    (0x283B22, (0x4E2B, false)), // East Asian ideograph
    (0x22652C, (0x7892, false)), // East Asian ideograph
    (0x28342E, (0x63BC, false)), // East Asian ideograph
    (0x235359, (0x9A2F, false)), // East Asian ideograph
    (0x22252D, (0x5D47, false)), // East Asian ideograph
    (0x2D4829, (0x51CF, false)), // East Asian ideograph
    (0x6F5027, (0xBA4B, false)), // Korean hangul
    (0x23252F, (0x84E7, false)), // East Asian ideograph
    (0x215F21, (0x9664, false)), // East Asian ideograph
    (0x215F22, (0x966A, false)), // East Asian ideograph
    (0x215F23, (0x9673, false)), // East Asian ideograph
    (0x215F24, (0x9678, false)), // East Asian ideograph
    (0x215F25, (0x9675, false)), // East Asian ideograph
    (0x215F26, (0x9672, false)), // East Asian ideograph
    (0x215F27, (0x9676, false)), // East Asian ideograph
    (0x215F28, (0x9677, false)), // East Asian ideograph
    (0x215F29, (0x9674, false)), // East Asian ideograph
    (0x215F2A, (0x9670, false)), // East Asian ideograph
    (0x215F2B, (0x968A, false)), // East Asian ideograph
    (0x215F2C, (0x968E, false)), // East Asian ideograph
    (0x215F2D, (0x968B, false)), // East Asian ideograph
    (0x215F2E, (0x967D, false)), // East Asian ideograph
    (0x235F2F, (0x9F0F, false)), // East Asian ideograph
    (0x215F30, (0x9686, false)), // East Asian ideograph
    (0x235F31, (0x9F10, false)), // East Asian ideograph
    (0x235F32, (0x9F12, false)), // East Asian ideograph
    (0x235F33, (0x9F16, false)), // East Asian ideograph
    (0x235F34, (0x9F17, false)), // East Asian ideograph
    (0x215F35, (0x9695, false)), // East Asian ideograph
    (0x215F36, (0x969C, false)), // East Asian ideograph
    (0x235F37, (0x9F1A, false)), // East Asian ideograph
    (0x215F38, (0x96A7, false)), // East Asian ideograph
    (0x215F39, (0x96A8, false)), // East Asian ideograph
    (0x215F3A, (0x96AA, false)), // East Asian ideograph
    (0x215F3B, (0x96B1, false)), // East Asian ideograph
    (0x215F3C, (0x96B4, false)), // East Asian ideograph
    (0x215F3D, (0x96B8, false)), // East Asian ideograph
    (0x225F3E, (0x7625, false)), // East Asian ideograph
    (0x225F3F, (0x761A, false)), // East Asian ideograph
    (0x215F40, (0x96C7, false)), // East Asian ideograph
    (0x215F41, (0x96C6, false)), // East Asian ideograph
    (0x215F42, (0x96C4, false)), // East Asian ideograph
    (0x215F43, (0x96C1, false)), // East Asian ideograph
    (0x215F44, (0x96C5, false)), // East Asian ideograph
    (0x215F45, (0x96CD, false)), // East Asian ideograph
    (0x215F46, (0x96CB, false)), // East Asian ideograph
    (0x215F47, (0x96C9, false)), // East Asian ideograph
    (0x215F48, (0x96CC, false)), // East Asian ideograph
    (0x215F49, (0x96D5, false)), // East Asian ideograph
    (0x215F4A, (0x96D6, false)), // East Asian ideograph
    (0x215F4B, (0x96DC, false)), // East Asian ideograph
    (0x215F4C, (0x96DE, false)), // East Asian ideograph
    (0x215F4D, (0x96DB, false)), // East Asian ideograph
    (0x215F4E, (0x96D9, false)), // East Asian ideograph
    (0x215F4F, (0x96E2, false)), // East Asian ideograph
    (0x225F50, (0x7622, false)), // East Asian ideograph
    (0x225F51, (0x762F, false)), // East Asian ideograph
    (0x215F52, (0x96EA, false)), // East Asian ideograph
    (0x215F53, (0x96EF, false)), // East Asian ideograph
    (0x215F54, (0x96F2, false)), // East Asian ideograph
    (0x215F55, (0x96FB, false)), // East Asian ideograph
    (0x215F56, (0x96F7, false)), // East Asian ideograph
    (0x215F57, (0x96F9, false)), // East Asian ideograph
    (0x215F58, (0x96F6, false)), // East Asian ideograph
    (0x215F59, (0x9700, false)), // East Asian ideograph
    (0x23424F, (0x92A2, false)), // East Asian ideograph
    (0x215F5B, (0x9704, false)), // East Asian ideograph
    (0x215F5C, (0x9709, false)), // East Asian ideograph
    (0x215F5D, (0x9706, false)), // East Asian ideograph
    (0x225F5E, (0x763B, false)), // East Asian ideograph
    (0x215F5F, (0x970E, false)), // East Asian ideograph
    (0x225F60, (0x763C, false)), // East Asian ideograph
    (0x215F61, (0x970F, false)), // East Asian ideograph
    (0x225F62, (0x7635, false)), // East Asian ideograph
    (0x215F63, (0x9713, false)), // East Asian ideograph
    (0x235F64, (0x9F3D, false)), // East Asian ideograph
    (0x215F65, (0x971E, false)), // East Asian ideograph
    (0x215F66, (0x972A, false)), // East Asian ideograph
    (0x225F67, (0x7648, false)), // East Asian ideograph
    (0x225F68, (0x764E, false)), // East Asian ideograph
    (0x235F69, (0x9F41, false)), // East Asian ideograph
    (0x225F6A, (0x7643, false)), // East Asian ideograph
    (0x215F6B, (0x973D, false)), // East Asian ideograph
    (0x215F6C, (0x973E, false)), // East Asian ideograph
    (0x215F6D, (0x9744, false)), // East Asian ideograph
    (0x215F6E, (0x9742, false)), // East Asian ideograph
    (0x225F6F, (0x7649, false)), // East Asian ideograph
    (0x215F70, (0x9751, false)), // East Asian ideograph
    (0x215F71, (0xFA1C, false)), // East Asian ideograph
    (0x215F72, (0x975B, false)), // East Asian ideograph (variant of 4B5F72 which maps to 975B)
    (0x215F73, (0x975C, false)), // East Asian ideograph
    (0x215F74, (0x975E, false)), // East Asian ideograph
    (0x225F75, (0x7654, false)), // East Asian ideograph
    (0x215F76, (0x9761, false)), // East Asian ideograph
    (0x215F78, (0x9766, false)), // East Asian ideograph
    (0x235F79, (0x9F4E, false)), // East Asian ideograph
    (0x225F7A, (0x765C, false)), // East Asian ideograph
    (0x235F7B, (0x9F4F, false)), // East Asian ideograph
    (0x235F7C, (0x9F54, false)), // East Asian ideograph
    (0x215F7D, (0x977C, false)), // East Asian ideograph
    (0x235F7E, (0x9F55, false)), // East Asian ideograph
    (0x216540, (0x4F66, false)), // East Asian ideograph
    (0x692541, (0x30C1, false)), // Katakana letter TI
    (0x6F5622, (0xC644, false)), // Korean hangul
    (0x6F546E, (0xC500, false)), // Korean hangul
    (0x6F502B, (0xBA54, false)), // Korean hangul
    (0x215829, (0x898F, false)), // East Asian ideograph
    (0x234251, (0x9230, false)), // East Asian ideograph
    (0x225C28, (0x74B5, false)), // East Asian ideograph
    (0x216544, (0x4F67, false)), // East Asian ideograph
    (0x695429, (0x5726, false)), // East Asian ideograph
    (0x6F515C, (0xBD88, false)), // Korean hangul
    (0x292546, (0x8368, false)), // East Asian ideograph
    (0x233145, (0x89DC, false)), // East Asian ideograph
    (0x692547, (0x30C7, false)), // Katakana letter DE
    (0x225C29, (0x74BA, false)), // East Asian ideograph
    (0x213A48, (0x5B2A, false)), // East Asian ideograph
    (0x6F492D, (0xAC85, false)), // Korean hangul
    (0x69254A, (0x30CA, false)), // Katakana letter NA
    (0x29254B, (0x835B, false)), // East Asian ideograph
    (0x2D482F, (0x6E07, false)), // East Asian ideograph
    (0x6F5442, (0xC330, false)), // Korean hangul
    (0x70586F, (0x4EEB, false)), // East Asian ideograph
    (0x274142, (0x6325, false)), // East Asian ideograph
    (0x6F502D, (0xBA58, false)), // Korean hangul
    (0x23254D, (0x8553, false)), // East Asian ideograph
    (0x21654E, (0x4F5A, false)), // East Asian ideograph
    (0x335065, (0x7A45, false)), // East Asian ideograph
    (0x2E3B22, (0x690F, false)), // East Asian ideograph
    (0x224F61, (0x7044, false)), // East Asian ideograph
    (0x692550, (0x30D0, false)), // Katakana letter BA
    (0x6F5C71, (0xD57C, false)), // Korean hangul
    (0x222551, (0x5D8E, false)), // East Asian ideograph
    (0x6F586B, (0xCB58, false)), // Korean hangul
    (0x335834, (0x89E7, false)), // East Asian ideograph
    (0x2D593D, (0x8AE9, false)), // East Asian ideograph
    (0x4B4476, (0x685F, false)), // East Asian ideograph
    (0x692555, (0x30D5, false)), // Katakana letter HU
    (0x216556, (0x4F82, false)), // East Asian ideograph
    (0x6F5A3A, (0xCF2D, false)), // Korean hangul
    (0x6F502F, (0xBA64, false)), // Korean hangul
    (0x692557, (0x30D7, false)), // Katakana letter PU
    (0x294942, (0x9606, false)), // East Asian ideograph
    (0x234255, (0x9248, false)), // East Asian ideograph
    (0x692558, (0x30D8, false)), // Katakana letter HE
    (0x47347B, (0x8C2B, false)), // East Asian ideograph
    (0x6F5262, (0xC0AF, false)), // Korean hangul
    (0x23255A, (0x8546, false)), // East Asian ideograph
    (0x216021, (0x978D, false)), // East Asian ideograph
    (0x226022, (0x7664, false)), // East Asian ideograph
    (0x236023, (0x9F57, false)), // East Asian ideograph
    (0x226024, (0x7659, false)), // East Asian ideograph
    (0x216025, (0x97A0, false)), // East Asian ideograph
    (0x216026, (0x97A3, false)), // East Asian ideograph
    (0x216027, (0x97A6, false)), // East Asian ideograph
    (0x236028, (0x9F60, false)), // East Asian ideograph
    (0x216029, (0x97C3, false)), // East Asian ideograph
    (0x21602A, (0x97C1, false)), // East Asian ideograph
    (0x22602B, (0x765F, false)), // East Asian ideograph
    (0x21602C, (0x97CB, false)), // East Asian ideograph
    (0x21602D, (0x97CC, false)), // East Asian ideograph
    (0x21602E, (0x97D3, false)), // East Asian ideograph
    (0x21602F, (0x97DC, false)), // East Asian ideograph
    (0x216030, (0x97ED, false)), // East Asian ideograph
    (0x216031, (0x97F3, false)), // East Asian ideograph
    (0x226032, (0x7667, false)), // East Asian ideograph
    (0x216033, (0x7ADF, false)), // East Asian ideograph
    (0x216034, (0x97F6, false)), // East Asian ideograph
    (0x226035, (0x766A, false)), // East Asian ideograph
    (0x216036, (0x97FF, false)), // East Asian ideograph (variant of 456036 which maps to 97FF)
    (0x226037, (0x766D, false)), // East Asian ideograph
    (0x226038, (0x766F, false)), // East Asian ideograph
    (0x216039, (0x9803, false)), // East Asian ideograph
    (0x22603A, (0x7670, false)), // East Asian ideograph
    (0x21603B, (0x9806, false)), // East Asian ideograph
    (0x21603C, (0x9808, false)), // East Asian ideograph
    (0x21603D, (0x9810, false)), // East Asian ideograph
    (0x21603E, (0x980A, false)), // East Asian ideograph
    (0x21603F, (0x9811, false)), // East Asian ideograph
    (0x226040, (0x7676, false)), // East Asian ideograph
    (0x226041, (0x7677, false)), // East Asian ideograph
    (0x216042, (0x980C, false)), // East Asian ideograph
    (0x216043, (0x9817, false)), // East Asian ideograph
    (0x216044, (0x9818, false)), // East Asian ideograph (variant of 4B6044 which maps to 9818)
    (0x216045, (0x9821, false)), // East Asian ideograph
    (0x216046, (0x982D, false)), // East Asian ideograph
    (0x216047, (0x9830, false)), // East Asian ideograph
    (0x226048, (0x7680, false)), // East Asian ideograph
    (0x21582F, (0x89B2, false)), // East Asian ideograph
    (0x22604A, (0x768B, false)), // East Asian ideograph
    (0x21604B, (0x9837, false)), // East Asian ideograph
    (0x21604C, (0x9824, false)), // East Asian ideograph
    (0x21604D, (0x9846, false)), // East Asian ideograph
    (0x21604E, (0x9854, false)), // East Asian ideograph
    (0x21604F, (0x984D, false)), // East Asian ideograph
    (0x216050, (0x984C, false)), // East Asian ideograph
    (0x216051, (0x984E, false)), // East Asian ideograph
    (0x226052, (0x7695, false)), // East Asian ideograph
    (0x216053, (0x985E, false)), // East Asian ideograph (variant of 4B6053 which maps to 985E)
    (0x216054, (0x985A, false)), // East Asian ideograph
    (0x226055, (0x656B, false)), // East Asian ideograph
    (0x216056, (0x9867, false)), // East Asian ideograph
    (0x216057, (0x986B, false)), // East Asian ideograph
    (0x216058, (0x986F, false)), // East Asian ideograph
    (0x226059, (0x7699, false)), // East Asian ideograph
    (0x21605A, (0x9870, false)), // East Asian ideograph
    (0x21605B, (0x98A8, false)), // East Asian ideograph
    (0x21605C, (0x98AF, false)), // East Asian ideograph
    (0x22605D, (0x769C, false)), // East Asian ideograph
    (0x21605E, (0x98B3, false)), // East Asian ideograph
    (0x22605F, (0x769D, false)), // East Asian ideograph
    (0x216060, (0x98BA, false)), // East Asian ideograph
    (0x236061, (0x9F93, false)), // East Asian ideograph
    (0x216062, (0x98C4, false)), // East Asian ideograph
    (0x216063, (0x98DB, false)), // East Asian ideograph
    (0x216064, (0x98DF, false)), // East Asian ideograph
    (0x216065, (0x98E2, false)), // East Asian ideograph
    (0x226066, (0x76A5, false)), // East Asian ideograph
    (0x226067, (0x76A6, false)), // East Asian ideograph
    (0x216068, (0x98ED, false)), // East Asian ideograph
    (0x216069, (0x98EA, false)), // East Asian ideograph
    (0x21606A, (0x98EE, false)), // East Asian ideograph
    (0x23606B, (0x9FA0, false)), // East Asian ideograph
    (0x21606C, (0x98FC, false)), // East Asian ideograph
    (0x21606D, (0x98F4, false)), // East Asian ideograph
    (0x21606E, (0x98FD, false)), // East Asian ideograph
    (0x21606F, (0x98FE, false)), // East Asian ideograph
    (0x216070, (0x9903, false)), // East Asian ideograph
    (0x216071, (0x990A, false)), // East Asian ideograph
    (0x236072, (0x9FA4, false)), // East Asian ideograph
    (0x216073, (0x9909, false)), // East Asian ideograph
    (0x226074, (0x76B8, false)), // East Asian ideograph
    (0x216075, (0x9912, false)), // East Asian ideograph
    (0x216076, (0x9918, false)), // East Asian ideograph
    (0x226077, (0x76BD, false)), // East Asian ideograph
    (0x216078, (0x9905, false)), // East Asian ideograph
    (0x216079, (0x9928, false)), // East Asian ideograph
    (0x21607A, (0x991E, false)), // East Asian ideograph
    (0x21607B, (0x991B, false)), // East Asian ideograph
    (0x21607C, (0x9921, false)), // East Asian ideograph
    (0x21607D, (0x9935, false)), // East Asian ideograph
    (0x21607E, (0x993E, false)), // East Asian ideograph
    (0x6F5369, (0xC258, false)), // Korean hangul
    (0x6F5033, (0xBA71, false)), // Korean hangul
    (0x213A5B, (0x5B6B, false)), // East Asian ideograph
    (0x69256B, (0x30EB, false)), // Katakana letter RU
    (0x69256C, (0x30EC, false)), // Katakana letter RE
    (0x293670, (0x8D49, false)), // East Asian ideograph
    (0x69656D, (0x7E83, false)), // East Asian ideograph
    (0x224F67, (0x7047, false)), // East Asian ideograph
    (0x235172, (0x994C, false)), // East Asian ideograph
    (0x69256F, (0x30EF, false)), // Katakana letter WA
    (0x2E4C35, (0x6DE5, false)), // East Asian ideograph
    (0x6F5034, (0xBA74, false)), // Korean hangul
    (0x213A5C, (0x5B70, false)), // East Asian ideograph
    (0x6F5934, (0xCC59, false)), // Korean hangul
    (0x4D4F39, (0x988C, false)), // East Asian ideograph
    (0x292571, (0x835E, false)), // East Asian ideograph
    (0x226573, (0x78E0, false)), // East Asian ideograph
    (0x6F4E4C, (0xB6B1, false)), // Korean hangul
    (0x292574, (0x83B8, false)), // East Asian ideograph
    (0x4B3A47, (0x88CA, false)), // East Asian ideograph
    (0x6F5035, (0xBA78, false)), // Korean hangul
    (0x692575, (0x30F5, false)), // Katakana letter small KA
    (0x294948, (0x960F, false)), // East Asian ideograph
    (0x213378, (0x5275, false)), // East Asian ideograph
    (0x225C32, (0x74CC, false)), // East Asian ideograph
    (0x216576, (0x4F9C, false)), // East Asian ideograph
    (0x215021, (0x7B4D, false)), // East Asian ideograph
    (0x6F5C6C, (0xD56D, false)), // Korean hangul
    (0x232577, (0x858C, false)), // East Asian ideograph
    (0x214B6A, (0x74CA, false)), // East Asian ideograph
    (0x224F69, (0x7049, false)), // East Asian ideograph
    (0x216940, (0x5133, false)), // East Asian ideograph
    (0x692578, (0x309C, false)), // Katakana-hiragana semi-voiced sound mark
    (0x275023, (0x8345, false)), // East Asian ideograph
    (0x2E742E, (0x7516, false)), // East Asian ideograph
    (0x6F5479, (0xC53D, false)), // Korean hangul
    (0x6F5024, (0xBA40, false)), // Korean hangul
    (0x6F5036, (0xBA83, false)), // Korean hangul
    (0x213A5E, (0x5B71, false)), // East Asian ideograph
    (0x294949, (0x9608, false)), // East Asian ideograph
    (0x225025, (0x7066, false)), // East Asian ideograph
    (0x2E257B, (0x5D1F, false)), // East Asian ideograph
    (0x6F5026, (0xBA49, false)), // Korean hangul
    (0x6F4A5F, (0xAECD, false)), // Korean hangul
    (0x225027, (0x7065, false)), // East Asian ideograph
    (0x235369, (0x9A36, false)), // East Asian ideograph
    (0x225028, (0x7068, false)), // East Asian ideograph
    (0x234F26, (0x9816, false)), // East Asian ideograph
    (0x215029, (0x7B95, false)), // East Asian ideograph
    (0x276272, (0x9EE9, false)), // East Asian ideograph
    (0x213A5F, (0x5B75, false)), // East Asian ideograph
    (0x27502A, (0x94B3, false)), // East Asian ideograph
    (0x27502B, (0x7B3A, false)), // East Asian ideograph
    (0x6F502C, (0xBA55, false)), // Korean hangul
    (0x224F6B, (0x7055, false)), // East Asian ideograph
    (0x23536A, (0x9A2E, false)), // East Asian ideograph
    (0x2D502D, (0x7B5D, false)), // East Asian ideograph
    (0x4C4339, (0x69DE, false)), // East Asian ideograph
    (0x6F502E, (0xBA5C, false)), // Korean hangul
    (0x6F5038, (0xBA85, false)), // Korean hangul
    (0x213A60, (0x5B78, false)), // East Asian ideograph
    (0x21502F, (0x7BAD, false)), // East Asian ideograph
    (0x215030, (0x7BC4, false)), // East Asian ideograph
    (0x216122, (0x993D, false)), // East Asian ideograph
    (0x226123, (0x76CB, false)), // East Asian ideograph
    (0x216124, (0x9952, false)), // East Asian ideograph
    (0x216125, (0x9951, false)), // East Asian ideograph
    (0x226126, (0x76CC, false)), // East Asian ideograph
    (0x216127, (0x995E, false)), // East Asian ideograph
    (0x216128, (0x9996, false)), // East Asian ideograph
    (0x216129, (0x9999, false)), // East Asian ideograph
    (0x21612A, (0x99A5, false)), // East Asian ideograph
    (0x21612B, (0x99A8, false)), // East Asian ideograph
    (0x21612C, (0x99AC, false)), // East Asian ideograph
    (0x21612D, (0x99AE, false)), // East Asian ideograph
    (0x21612E, (0x99AD, false)), // East Asian ideograph
    (0x21612F, (0x99B3, false)), // East Asian ideograph
    (0x216130, (0x99B1, false)), // East Asian ideograph
    (0x216131, (0x99B4, false)), // East Asian ideograph
    (0x216132, (0x99C1, false)), // East Asian ideograph
    (0x275033, (0x8282, false)), // East Asian ideograph
    (0x216134, (0x99DD, false)), // East Asian ideograph
    (0x216135, (0x99D5, false)), // East Asian ideograph
    (0x216136, (0x99DF, false)), // East Asian ideograph
    (0x216137, (0x99DB, false)), // East Asian ideograph
    (0x216138, (0x99D2, false)), // East Asian ideograph
    (0x216139, (0x99D9, false)), // East Asian ideograph
    (0x21613A, (0x99D1, false)), // East Asian ideograph
    (0x21613B, (0x99ED, false)), // East Asian ideograph
    (0x21613C, (0x99F1, false)), // East Asian ideograph
    (0x21613D, (0x9A01, false)), // East Asian ideograph
    (0x21613E, (0x99FF, false)), // East Asian ideograph
    (0x21613F, (0x99E2, false)), // East Asian ideograph
    (0x216140, (0x9A0E, false)), // East Asian ideograph
    (0x216141, (0x9A19, false)), // East Asian ideograph
    (0x216142, (0x9A16, false)), // East Asian ideograph
    (0x216143, (0x9A2B, false)), // East Asian ideograph
    (0x226144, (0x76ED, false)), // East Asian ideograph
    (0x216145, (0x9A37, false)), // East Asian ideograph
    (0x216146, (0x9A43, false)), // East Asian ideograph
    (0x216147, (0x9A45, false)), // East Asian ideograph
    (0x226148, (0x76F1, false)), // East Asian ideograph
    (0x216149, (0x9A3E, false)), // East Asian ideograph
    (0x21614A, (0x9A55, false)), // East Asian ideograph
    (0x21614B, (0x9A5A, false)), // East Asian ideograph
    (0x21614C, (0x9A5B, false)), // East Asian ideograph
    (0x21614D, (0x9A57, false)), // East Asian ideograph
    (0x21614E, (0x9A5F, false)), // East Asian ideograph
    (0x22614F, (0x7708, false)), // East Asian ideograph
    (0x226150, (0x7707, false)), // East Asian ideograph
    (0x275038, (0x7BAC, false)), // East Asian ideograph
    (0x216152, (0x9AA8, false)), // East Asian ideograph
    (0x216153, (0x9AAF, false)), // East Asian ideograph
    (0x226154, (0x770A, false)), // East Asian ideograph
    (0x216155, (0x9AB7, false)), // East Asian ideograph
    (0x216156, (0x9AB8, false)), // East Asian ideograph
    (0x215039, (0x7BE4, false)), // East Asian ideograph
    (0x216158, (0x9ACF, false)), // East Asian ideograph
    (0x226159, (0x76FB, false)), // East Asian ideograph
    (0x21615A, (0x9AD4, false)), // East Asian ideograph
    (0x21615B, (0x9AD2, false)), // East Asian ideograph
    (0x21615C, (0x9AD8, false)), // East Asian ideograph
    (0x21615D, (0x9AE5, false)), // East Asian ideograph
    (0x22615E, (0x772B, false)), // East Asian ideograph
    (0x21615F, (0x9AEE, false)), // East Asian ideograph
    (0x216160, (0x9AFB, false)), // East Asian ideograph
    (0x216161, (0x9AED, false)), // East Asian ideograph
    (0x216162, (0x9B03, false)), // East Asian ideograph
    (0x216163, (0x9B06, false)), // East Asian ideograph
    (0x216164, (0x9B0D, false)), // East Asian ideograph
    (0x216165, (0x9B1A, false)), // East Asian ideograph
    (0x216166, (0x9B22, false)), // East Asian ideograph
    (0x216167, (0x9B25, false)), // East Asian ideograph
    (0x216168, (0x9B27, false)), // East Asian ideograph
    (0x27503C, (0x7B5B, false)), // East Asian ideograph
    (0x21616A, (0x9B31, false)), // East Asian ideograph
    (0x21616B, (0x9B32, false)), // East Asian ideograph
    (0x21616C, (0x9B3C, false)), // East Asian ideograph
    (0x21616D, (0x9B41, false)), // East Asian ideograph
    (0x21616E, (0x9B42, false)), // East Asian ideograph
    (0x22616F, (0x7721, false)), // East Asian ideograph
    (0x216170, (0x9B44, false)), // East Asian ideograph
    (0x216171, (0x9B4F, false)), // East Asian ideograph
    (0x216172, (0x9B54, false)), // East Asian ideograph
    (0x216173, (0x9B58, false)), // East Asian ideograph
    (0x216174, (0x9B5A, false)), // East Asian ideograph
    (0x226175, (0x7739, false)), // East Asian ideograph
    (0x226176, (0x772F, false)), // East Asian ideograph
    (0x216177, (0x9B91, false)), // East Asian ideograph
    (0x216178, (0x9BAB, false)), // East Asian ideograph
    (0x216179, (0x9BAE, false)), // East Asian ideograph
    (0x21617A, (0x9BAA, false)), // East Asian ideograph
    (0x21617B, (0x9BCA, false)), // East Asian ideograph
    (0x21617C, (0x9BC9, false)), // East Asian ideograph
    (0x21617D, (0x9BE8, false)), // East Asian ideograph
    (0x21617E, (0x9BE7, false)), // East Asian ideograph
    (0x215040, (0x7BF7, false)), // East Asian ideograph
    (0x275041, (0x7B80, false)), // East Asian ideograph
    (0x225042, (0x7086, false)), // East Asian ideograph
    (0x6F503C, (0xBAAB, false)), // Korean hangul
    (0x29596B, (0x9CA1, false)), // East Asian ideograph
    (0x335F3D, (0x96B7, false)), // East Asian ideograph
    (0x29494F, (0x960A, false)), // East Asian ideograph
    (0x6F5043, (0xBAC3, false)), // Korean hangul
    (0x4B5044, (0x7C27, false)), // East Asian ideograph (variant of 215044 which maps to 7C27)
    (0x275045, (0x7BAA, false)), // East Asian ideograph
    (0x2E3D73, (0x7A1C, false)), // East Asian ideograph
    (0x275046, (0x7BD1, false)), // East Asian ideograph
    (0x6F5047, (0xBB34, false)), // Korean hangul
    (0x6F503D, (0xBAAC, false)), // Korean hangul
    (0x213A65, (0x5B87, false)), // East Asian ideograph
    (0x294950, (0x960C, false)), // East Asian ideograph
    (0x235048, (0x98B8, false)), // East Asian ideograph
    (0x225C3A, (0x74D4, false)), // East Asian ideograph
    (0x27583A, (0x8BA3, false)), // East Asian ideograph
    (0x225049, (0x7084, false)), // East Asian ideograph
    (0x2D594C, (0x8B72, false)), // East Asian ideograph
    (0x6F5960, (0xCE20, false)), // Korean hangul
    (0x22504A, (0x7081, false)), // East Asian ideograph
    (0x235370, (0x9A41, false)), // East Asian ideograph
    (0x21504B, (0x7C3D, false)), // East Asian ideograph
    (0x4D3032, (0x88AE, false)), // East Asian ideograph
    (0x6F5A3D, (0xCF54, false)), // Korean hangul
    (0x27504C, (0x7BEE, false)), // East Asian ideograph
    (0x274153, (0x6363, false)), // East Asian ideograph
    (0x4D5C6B, (0x9D50, false)), // East Asian ideograph
    (0x213A66, (0x5B88, false)), // East Asian ideograph
    (0x21504D, (0x7C4C, false)), // East Asian ideograph
    (0x234264, (0x925E, false)), // East Asian ideograph
    (0x2D3B77, (0x5CE9, false)), // East Asian ideograph
    (0x21504E, (0x7C4D, false)), // East Asian ideograph
    (0x6F5025, (0xBA48, false)), // Korean hangul
    (0x2D504F, (0x7C58, false)), // East Asian ideograph
    (0x69542A, (0x5737, false)), // East Asian ideograph
    (0x293B59, (0x8F82, false)), // East Asian ideograph
    (0x4B4B2B, (0x7363, false)), // East Asian ideograph
    (0x275050, (0x7B3C, false)), // East Asian ideograph
    (0x275051, (0x7C41, false)), // East Asian ideograph
    (0x6F503F, (0xBAB8, false)), // Korean hangul
    (0x213A67, (0x5B89, false)), // East Asian ideograph
    (0x294952, (0x960D, false)), // East Asian ideograph
    (0x275052, (0x7B7E, false)), // East Asian ideograph (duplicate simplified)
    (0x6F5963, (0xCE35, false)), // Korean hangul
    (0x2D3B78, (0x5CEF, false)), // East Asian ideograph
    (0x275053, (0x7BF1, false)), // East Asian ideograph
    (0x4D594E, (0x9BF5, false)), // East Asian ideograph
    (0x3F614C, (0x99C5, false)), // East Asian ideograph
    (0x275054, (0x7BA9, false)), // East Asian ideograph
    (0x4B6258, (0x68BA, false)), // East Asian ideograph
    (0x275055, (0x5401, false)), // East Asian ideograph
    (0x6F245A, (0x3139, false)), // Korean hangul
    (0x225056, (0x7088, false)), // East Asian ideograph
    (0x6F5040, (0xBAB9, false)), // Korean hangul
    (0x213A68, (0x5B85, false)), // East Asian ideograph
    (0x21583E, (0x8A0C, false)), // East Asian ideograph
    (0x2D3B79, (0x5D8B, false)), // East Asian ideograph
    (0x6F5058, (0xBB88, false)), // Korean hangul
    (0x2D594F, (0x8B83, false)), // East Asian ideograph
    (0x225059, (0x708C, false)), // East Asian ideograph
    (0x224B31, (0x6E5D, false)), // East Asian ideograph
    (0x216221, (0x9C13, false)), // East Asian ideograph
    (0x226222, (0x7725, false)), // East Asian ideograph
    (0x216223, (0x9BFD, false)), // East Asian ideograph
    (0x216224, (0x9C2D, false)), // East Asian ideograph
    (0x216225, (0x9C25, false)), // East Asian ideograph
    (0x226226, (0x7734, false)), // East Asian ideograph
    (0x216227, (0x9C3E, false)), // East Asian ideograph
    (0x216228, (0x9C3B, false)), // East Asian ideograph
    (0x216229, (0x9C54, false)), // East Asian ideograph
    (0x21622A, (0x9C57, false)), // East Asian ideograph
    (0x21622B, (0x9C56, false)), // East Asian ideograph
    (0x21622C, (0x9C49, false)), // East Asian ideograph
    (0x22622D, (0x7747, false)), // East Asian ideograph
    (0x21622E, (0x9C78, false)), // East Asian ideograph
    (0x21622F, (0x9CE5, false)), // East Asian ideograph
    (0x216230, (0x9CE9, false)), // East Asian ideograph
    (0x226231, (0x7745, false)), // East Asian ideograph
    (0x226232, (0x774D, false)), // East Asian ideograph
    (0x216233, (0x9CF3, false)), // East Asian ideograph
    (0x216234, (0x9D06, false)), // East Asian ideograph
    (0x216235, (0x9D09, false)), // East Asian ideograph
    (0x216236, (0x9D15, false)), // East Asian ideograph
    (0x226237, (0x774E, false)), // East Asian ideograph
    (0x216238, (0x9D28, false)), // East Asian ideograph
    (0x216239, (0x9D26, false)), // East Asian ideograph
    (0x22623A, (0x775F, false)), // East Asian ideograph
    (0x21505F, (0x7CBD, false)), // East Asian ideograph
    (0x21623C, (0x9D3B, false)), // East Asian ideograph
    (0x21623D, (0x9D3F, false)), // East Asian ideograph
    (0x22623E, (0x7752, false)), // East Asian ideograph
    (0x21623F, (0x9D51, false)), // East Asian ideograph
    (0x216240, (0x9D60, false)), // East Asian ideograph
    (0x215060, (0x7CB9, false)), // East Asian ideograph
    (0x226242, (0x7758, false)), // East Asian ideograph
    (0x216243, (0x9D72, false)), // East Asian ideograph
    (0x226244, (0x7756, false)), // East Asian ideograph
    (0x226245, (0x775A, false)), // East Asian ideograph
    (0x216246, (0x9DB4, false)), // East Asian ideograph
    (0x216247, (0x9DAF, false)), // East Asian ideograph
    (0x216248, (0x9DC2, false)), // East Asian ideograph
    (0x216249, (0x9DD3, false)), // East Asian ideograph
    (0x21624A, (0x9DD7, false)), // East Asian ideograph
    (0x21624B, (0x9DE5, false)), // East Asian ideograph
    (0x21624C, (0x9DF9, false)), // East Asian ideograph
    (0x215062, (0x7CCA, false)), // East Asian ideograph
    (0x21624E, (0x9E1A, false)), // East Asian ideograph
    (0x22624F, (0x7762, false)), // East Asian ideograph
    (0x216250, (0x9E79, false)), // East Asian ideograph
    (0x216251, (0x9E7D, false)), // East Asian ideograph
    (0x226252, (0x7780, false)), // East Asian ideograph
    (0x216253, (0x9E7F, false)), // East Asian ideograph
    (0x216254, (0x9E82, false)), // East Asian ideograph
    (0x216255, (0x9E8B, false)), // East Asian ideograph
    (0x226256, (0x776F, false)), // East Asian ideograph
    (0x216257, (0x9E92, false)), // East Asian ideograph
    (0x216258, (0x9E93, false)), // East Asian ideograph
    (0x224B33, (0x6E30, false)), // East Asian ideograph
    (0x21625A, (0x9E9F, false)), // East Asian ideograph
    (0x21625B, (0x9EA5, false)), // East Asian ideograph
    (0x21625C, (0x9EA9, false)), // East Asian ideograph
    (0x21625D, (0x9EB4, false)), // East Asian ideograph
    (0x21625E, (0x9EB5, false)), // East Asian ideograph
    (0x22625F, (0x7785, false)), // East Asian ideograph
    (0x216260, (0x9EBC, false)), // East Asian ideograph
    (0x216261, (0x9EBE, false)), // East Asian ideograph
    (0x216262, (0x9EC3, false)), // East Asian ideograph
    (0x216263, (0x9ECD, false)), // East Asian ideograph
    (0x216264, (0x9ECE, false)), // East Asian ideograph
    (0x216265, (0x9ECF, false)), // East Asian ideograph
    (0x226266, (0x778B, false)), // East Asian ideograph (variant of 4C6266 which maps to 778B)
    (0x216267, (0x58A8, false)), // East Asian ideograph
    (0x216268, (0x9ED8, false)), // East Asian ideograph
    (0x216269, (0x9ED4, false)), // East Asian ideograph
    (0x22626A, (0x778D, false)), // East Asian ideograph
    (0x21626B, (0x9EDC, false)), // East Asian ideograph
    (0x21626C, (0x9EDB, false)), // East Asian ideograph
    (0x21626D, (0x9EDD, false)), // East Asian ideograph
    (0x21626E, (0x9EE0, false)), // East Asian ideograph
    (0x21626F, (0x9EE8, false)), // East Asian ideograph
    (0x216270, (0x9EEF, false)), // East Asian ideograph
    (0x235068, (0x98F1, false)), // East Asian ideograph
    (0x226272, (0x7798, false)), // East Asian ideograph
    (0x226273, (0x7796, false)), // East Asian ideograph
    (0x216274, (0x9F0E, false)), // East Asian ideograph
    (0x226275, (0x77A2, false)), // East Asian ideograph
    (0x226276, (0x7799, false)), // East Asian ideograph
    (0x216277, (0x9F19, false)), // East Asian ideograph
    (0x216278, (0x9F20, false)), // East Asian ideograph
    (0x216279, (0x9F2C, false)), // East Asian ideograph
    (0x22627A, (0x77B5, false)), // East Asian ideograph
    (0x21627B, (0x9F3B, false)), // East Asian ideograph
    (0x21627C, (0x9F3E, false)), // East Asian ideograph
    (0x22627D, (0x77B7, false)), // East Asian ideograph
    (0x21627E, (0x9F4B, false)), // East Asian ideograph
    (0x6F5044, (0xBAFC, false)), // Korean hangul
    (0x27506B, (0x7CAE, false)), // East Asian ideograph
    (0x6F5964, (0xCE58, false)), // Korean hangul
    (0x225C41, (0x74DB, false)), // East Asian ideograph
    (0x236040, (0x9F6F, false)), // East Asian ideograph
    (0x23506C, (0x98EB, false)), // East Asian ideograph
    (0x6F506D, (0xBC14, false)), // Korean hangul
    (0x23315E, (0x89F5, false)), // East Asian ideograph
    (0x6F506E, (0xBC15, false)), // Korean hangul
    (0x4B4049, (0x62D0, false)), // East Asian ideograph
    (0x234F34, (0x982B, false)), // East Asian ideograph
    (0x22506F, (0x70A7, false)), // East Asian ideograph
    (0x27415A, (0x5C4F, false)), // East Asian ideograph
    (0x696273, (0x78B5, false)), // East Asian ideograph
    (0x275070, (0x7EAA, false)), // East Asian ideograph
    (0x215843, (0x8A13, false)), // East Asian ideograph
    (0x225071, (0x70B5, false)), // East Asian ideograph
    (0x275072, (0x7EA2, false)), // East Asian ideograph
    (0x295A65, (0x9E39, false)), // East Asian ideograph
    (0x215073, (0x7D09, false)), // East Asian ideograph
    (0x396179, (0x5C20, false)), // East Asian ideograph
    (0x275074, (0x7EA6, false)), // East Asian ideograph
    (0x27415B, (0x631A, false)), // East Asian ideograph
    (0x6F5046, (0xBB18, false)), // Korean hangul
    (0x275075, (0x7EA5, false)), // East Asian ideograph
    (0x215844, (0x8A2A, false)), // East Asian ideograph
    (0x275076, (0x7EBA, false)), // East Asian ideograph
    (0x213B21, (0x5BC6, false)), // East Asian ideograph
    (0x275077, (0x7EB9, false)), // East Asian ideograph
    (0x213B22, (0x5BC7, false)), // East Asian ideograph
    (0x235379, (0x9A42, false)), // East Asian ideograph
    (0x215078, (0x7D0A, false)), // East Asian ideograph
    (0x6F5729, (0xC7B0, false)), // Korean hangul
    (0x213B23, (0x5BC5, false)), // East Asian ideograph
    (0x6F5C72, (0xD584, false)), // Korean hangul
    (0x6F5079, (0xBC2D, false)), // Korean hangul
    (0x6F536D, (0xC27C, false)), // Korean hangul
    (0x29495A, (0x9612, false)), // East Asian ideograph
    (0x27507A, (0x7EAD, false)), // East Asian ideograph
    (0x213B25, (0x5BC2, false)), // East Asian ideograph
    (0x22507B, (0x70E5, false)), // East Asian ideograph
    (0x213B26, (0x5BBF, false)), // East Asian ideograph
    (0x21507C, (0x7D15, false)), // East Asian ideograph
    (0x224F7B, (0x705E, false)), // East Asian ideograph
    (0x23537A, (0x9A44, false)), // East Asian ideograph
    (0x22507D, (0x70D3, false)), // East Asian ideograph
    (0x234F37, (0x9820, false)), // East Asian ideograph
    (0x27507E, (0x7EBD, false)), // East Asian ideograph
    (0x335276, (0x8061, false)), // East Asian ideograph
    (0x6F5048, (0xBB35, false)), // Korean hangul
    (0x215846, (0x8A1D, false)), // East Asian ideograph
    (0x2D3B2A, (0x5EBD, false)), // East Asian ideograph
    (0x2D5957, (0x7AEA, false)), // East Asian ideograph
    (0x4B386C, (0x5841, false)), // East Asian ideograph
    (0x295A68, (0x9E3A, false)), // East Asian ideograph
    (0x453336, (0x5B82, false)), // East Asian ideograph
    (0x224B39, (0x6E6B, false)), // East Asian ideograph
    (0x213B2D, (0x5BE8, false)), // East Asian ideograph
    (0x273B2E, (0x5BDD, false)), // East Asian ideograph
    (0x6F5049, (0xBB36, false)), // Korean hangul
    (0x213A71, (0x5B9B, false)), // East Asian ideograph
    (0x6F5965, (0xCE59, false)), // Korean hangul
    (0x213B2F, (0x5BE4, false)), // East Asian ideograph
    (0x4B515A, (0x7E01, false)), // East Asian ideograph
    (0x216321, (0x9F52, false)), // East Asian ideograph
    (0x216322, (0x9F5F, false)), // East Asian ideograph
    (0x216323, (0x9F63, false)), // East Asian ideograph
    (0x216324, (0x9F61, false)), // East Asian ideograph (variant of 456324 which maps to 9F61)
    (0x216325, (0x9F66, false)), // East Asian ideograph
    (0x216326, (0x9F5C, false)), // East Asian ideograph
    (0x233B31, (0x8ECE, false)), // East Asian ideograph
    (0x216328, (0x9F6A, false)), // East Asian ideograph
    (0x216329, (0x9F77, false)), // East Asian ideograph
    (0x21632A, (0x9F72, false)), // East Asian ideograph
    (0x21632B, (0x9F8D, false)), // East Asian ideograph
    (0x22632C, (0x77BC, false)), // East Asian ideograph
    (0x21632D, (0x9F9C, false)), // East Asian ideograph
    (0x216330, (0x8288, false)), // East Asian ideograph
    (0x213B33, (0x5BDF, false)), // East Asian ideograph
    (0x6F504A, (0xBB38, false)), // Korean hangul
    (0x226335, (0x77CD, false)), // East Asian ideograph
    (0x29307D, (0x89CF, false)), // East Asian ideograph
    (0x696733, (0x81A4, false)), // East Asian ideograph
    (0x225C47, (0x74DE, false)), // East Asian ideograph
    (0x6F4C3F, (0xB1D4, false)), // Korean hangul
    (0x213B35, (0x5BEC, false)), // East Asian ideograph
    (0x706340, (0x61B7, false)), // East Asian ideograph
    (0x45462B, (0x7688, false)), // East Asian ideograph
    (0x226345, (0x77DE, false)), // East Asian ideograph
    (0x226346, (0x77DF, false)), // East Asian ideograph
    (0x23537D, (0x9A48, false)), // East Asian ideograph
    (0x224B3B, (0x6E8B, false)), // East Asian ideograph
    (0x213B37, (0x5BEB, false)), // East Asian ideograph
    (0x335738, (0x880F, false)), // East Asian ideograph
    (0x69634E, (0x7A43, false)), // East Asian ideograph
    (0x22634F, (0x77E7, false)), // East Asian ideograph
    (0x274160, (0x63B4, false)), // East Asian ideograph
    (0x226352, (0x77E6, false)), // East Asian ideograph
    (0x226355, (0x77EC, false)), // East Asian ideograph
    (0x273B39, (0x5B9D, false)), // East Asian ideograph
    (0x69254D, (0x30CD, false)), // Katakana letter NE
    (0x226359, (0x77F0, false)), // East Asian ideograph
    (0x22635A, (0x77F1, false)), // East Asian ideograph
    (0x22635C, (0x77F4, false)), // East Asian ideograph
    (0x217B3A, (0x5A38, false)), // East Asian ideograph
    (0x226360, (0x77FC, false)), // East Asian ideograph
    (0x213B3B, (0x5BFA, false)), // East Asian ideograph
    (0x23537E, (0x9A4C, false)), // East Asian ideograph
    (0x226367, (0x77F8, false)), // East Asian ideograph
    (0x226368, (0x77FB, false)), // East Asian ideograph
    (0x277B3C, (0x5A05, false)), // East Asian ideograph
    (0x355739, (0x9C76, false)), // East Asian ideograph
    (0x234944, (0x95AB, false)), // East Asian ideograph
    (0x226370, (0x7809, false)), // East Asian ideograph
    (0x226371, (0x7806, false)), // East Asian ideograph
    (0x226373, (0x7819, false)), // East Asian ideograph
    (0x226374, (0x7811, false)), // East Asian ideograph
    (0x293B3E, (0x8F71, false)), // East Asian ideograph
    (0x4C6376, (0x7839, false)), // East Asian ideograph
    (0x226378, (0x7812, false)), // East Asian ideograph
    (0x223B3F, (0x67A1, false)), // East Asian ideograph
    (0x213B40, (0x5C07, false)), // East Asian ideograph
    (0x4B5E27, (0x93AD, false)), // East Asian ideograph
    (0x273B42, (0x5BFB, false)), // East Asian ideograph
    (0x6F504D, (0xBB3D, false)), // Korean hangul
    (0x6F5935, (0xCC60, false)), // Korean hangul
    (0x294960, (0x9619, false)), // East Asian ideograph
    (0x213B43, (0x5C0D, false)), // East Asian ideograph
    (0x223A31, (0x6710, false)), // East Asian ideograph
    (0x273B44, (0x5BFC, false)), // East Asian ideograph
    (0x224B3E, (0x6E76, false)), // East Asian ideograph
    (0x234F3D, (0x9833, false)), // East Asian ideograph
    (0x2D4850, (0x6EDA, false)), // East Asian ideograph
    (0x293B47, (0x8F77, false)), // East Asian ideograph
    (0x6F504E, (0xBB44, false)), // Korean hangul
    (0x275F39, (0x968F, false)), // East Asian ideograph
    (0x23573F, (0x9BA8, false)), // East Asian ideograph
    (0x274B74, (0x74EF, false)), // East Asian ideograph
    (0x217B48, (0x5A50, false)), // East Asian ideograph
    (0x213B49, (0x5C24, false)), // East Asian ideograph
    (0x234E3B, (0x97C5, false)), // East Asian ideograph
    (0x4B4053, (0x627A, false)), // East Asian ideograph
    (0x213B4B, (0x5C31, false)), // East Asian ideograph
    (0x273B4C, (0x5C34, false)), // East Asian ideograph
    (0x6F504F, (0xBB47, false)), // Korean hangul
    (0x275F3A, (0x9669, false)), // East Asian ideograph
    (0x2D625F, (0x83FB, false)), // East Asian ideograph
    (0x213634, (0x5501, false)), // East Asian ideograph
    (0x213B4E, (0x5C3A, false)), // East Asian ideograph
    (0x394956, (0x792E, false)), // East Asian ideograph
    (0x2D7552, (0x579B, false)), // East Asian ideograph
    (0x213B4F, (0x5C3C, false)), // East Asian ideograph
    (0x69562C, (0x599B, false)), // East Asian ideograph
    (0x294E54, (0x97EA, false)), // East Asian ideograph
    (0x692574, (0x30F4, false)), // Katakana letter VU
    (0x233B51, (0x8EFF, false)), // East Asian ideograph
    (0x6F5050, (0xBB49, false)), // Korean hangul
    (0x275F3B, (0x9690, false)), // East Asian ideograph
    (0x213635, (0x54FC, false)), // East Asian ideograph
    (0x27516C, (0x7F1D, false)), // East Asian ideograph
    (0x4B4537, (0x6804, false)), // East Asian ideograph
    (0x213B54, (0x5C46, false)), // East Asian ideograph
    (0x45304C, (0x69A6, false)), // East Asian ideograph
    (0x234F40, (0x982E, false)), // East Asian ideograph
    (0x2D4853, (0x7001, false)), // East Asian ideograph
    (0x224A3D, (0x6DA4, false)), // East Asian ideograph
    (0x213B56, (0x5C48, false)), // East Asian ideograph
    (0x6F5051, (0xBB4D, false)), // Korean hangul
    (0x275F3C, (0x9647, false)), // East Asian ideograph
    (0x334277, (0x65EF, false)), // East Asian ideograph
    (0x213B58, (0x5C4B, false)), // East Asian ideograph
    (0x213B59, (0x5C4D, false)), // East Asian ideograph
    (0x23316B, (0x89FF, false)), // East Asian ideograph
    (0x213B5A, (0x5C55, false)), // East Asian ideograph
    (0x213B5B, (0x5C51, false)), // East Asian ideograph
    (0x226424, (0x781B, false)), // East Asian ideograph
    (0x216425, (0x5187, false)), // East Asian ideograph
    (0x226426, (0x782C, false)), // East Asian ideograph
    (0x226427, (0x7823, false)), // East Asian ideograph
    (0x226428, (0x782B, false)), // East Asian ideograph
    (0x216429, (0x4E28, false)), // East Asian ideograph
    (0x22642A, (0x7829, false)), // East Asian ideograph
    (0x22642D, (0x7822, false)), // East Asian ideograph
    (0x21642E, (0x4E31, false)), // East Asian ideograph
    (0x2D3748, (0x8B5F, false)), // East Asian ideograph
    (0x226431, (0x7835, false)), // East Asian ideograph
    (0x226432, (0x7833, false)), // East Asian ideograph
    (0x226433, (0x782E, false)), // East Asian ideograph
    (0x216434, (0x4E42, false)), // East Asian ideograph
    (0x226435, (0x7820, false)), // East Asian ideograph
    (0x216437, (0x738D, false)), // East Asian ideograph
    (0x226438, (0x783D, false)), // East Asian ideograph
    (0x22643B, (0x781F, false)), // East Asian ideograph
    (0x21643C, (0x4E5C, false)), // East Asian ideograph
    (0x22643D, (0x7831, false)), // East Asian ideograph
    (0x21643F, (0x6C39, false)), // East Asian ideograph
    (0x274168, (0x6320, false)), // East Asian ideograph
    (0x6F5053, (0xBB50, false)), // Korean hangul
    (0x275F3E, (0x53EA, false)), // East Asian ideograph (duplicate simplified)
    (0x226444, (0x784D, false)), // East Asian ideograph
    (0x216446, (0x4E85, false)), // East Asian ideograph
    (0x273B61, (0x5C42, false)), // East Asian ideograph
    (0x226448, (0x7848, false)), // East Asian ideograph
    (0x226449, (0x7853, false)), // East Asian ideograph
    (0x22644A, (0x7854, false)), // East Asian ideograph
    (0x22644B, (0x7845, false)), // East Asian ideograph
    (0x22644C, (0x7852, false)), // East Asian ideograph
    (0x295938, (0x9CDF, false)), // East Asian ideograph
    (0x22644E, (0x7850, false)), // East Asian ideograph
    (0x22644F, (0x7858, false)), // East Asian ideograph
    (0x216450, (0x4EA0, false)), // East Asian ideograph
    (0x216451, (0x4EA2, false)), // East Asian ideograph
    (0x226452, (0x7847, false)), // East Asian ideograph
    (0x233B63, (0x8F27, false)), // East Asian ideograph
    (0x216455, (0x4EB6, false)), // East Asian ideograph (variant of 4B6455 which maps to 4EB6)
    (0x226456, (0x784C, false)), // East Asian ideograph
    (0x695630, (0x5CBC, false)), // East Asian ideograph
    (0x216458, (0x4EB9, false)), // East Asian ideograph
    (0x223B64, (0x67B7, false)), // East Asian ideograph
    (0x22645A, (0x7868, false)), // East Asian ideograph
    (0x22645B, (0x786D, false)), // East Asian ideograph
    (0x21645E, (0x4EC9, false)), // East Asian ideograph
    (0x226460, (0x7864, false)), // East Asian ideograph
    (0x226461, (0x785C, false)), // East Asian ideograph
    (0x216462, (0x4ECE, false)), // East Asian ideograph (not in Unicode)
    (0x216463, (0x4EE8, false)), // East Asian ideograph
    (0x215852, (0x8A54, false)), // East Asian ideograph
    (0x213639, (0x54FA, false)), // East Asian ideograph
    (0x226466, (0x786A, false)), // East Asian ideograph
    (0x226469, (0x7886, false)), // East Asian ideograph
    (0x21646B, (0x4EE1, false)), // East Asian ideograph
    (0x22646C, (0x787F, false)), // East Asian ideograph
    (0x22646D, (0x7887, false)), // East Asian ideograph
    (0x213C2A, (0x5D84, false)), // East Asian ideograph
    (0x226470, (0x7894, false)), // East Asian ideograph
    (0x696471, (0x7CC0, false)), // East Asian ideograph
    (0x216472, (0x4F08, false)), // East Asian ideograph
    (0x216473, (0x4F0E, false)), // East Asian ideograph
    (0x696474, (0x7CD8, false)), // East Asian ideograph
    (0x216475, (0x4F03, false)), // East Asian ideograph
    (0x226476, (0x788F, false)), // East Asian ideograph
    (0x234F44, (0x982F, false)), // East Asian ideograph
    (0x6F544A, (0xC378, false)), // Korean hangul
    (0x21647C, (0x4F22, false)), // East Asian ideograph
    (0x22647E, (0x7899, false)), // East Asian ideograph
    (0x213B6B, (0x5CB7, false)), // East Asian ideograph
    (0x225C52, (0x74E7, false)), // East Asian ideograph
    (0x27516D, (0x603B, false)), // East Asian ideograph
    (0x223B6D, (0x6802, false)), // East Asian ideograph
    (0x695632, (0x5CC5, false)), // East Asian ideograph
    (0x213B6E, (0x5CA1, false)), // East Asian ideograph
    (0x4C3A5B, (0x6859, false)), // East Asian ideograph
    (0x213B6F, (0x5CAB, false)), // East Asian ideograph
    (0x6F5056, (0xBB61, false)), // Korean hangul
    (0x294969, (0x961A, false)), // East Asian ideograph
    (0x215854, (0x8A50, false)), // East Asian ideograph
    (0x21363B, (0x54EE, false)), // East Asian ideograph
    (0x21762A, (0x57FB, false)), // East Asian ideograph
    (0x27583F, (0x8BAA, false)), // East Asian ideograph
    (0x213B71, (0x5CB1, false)), // East Asian ideograph
    (0x6F5961, (0xCE21, false)), // Korean hangul
    (0x213B72, (0x5CD9, false)), // East Asian ideograph
    (0x275F2C, (0x9636, false)), // East Asian ideograph
    (0x223B74, (0x67DF, false)), // East Asian ideograph
    (0x6F5057, (0xBB63, false)), // Korean hangul
    (0x233B75, (0x8F17, false)), // East Asian ideograph
    (0x23576B, (0x9BBB, false)), // East Asian ideograph
    (0x213B77, (0x5CE8, false)), // East Asian ideograph
    (0x216622, (0x4FE4, false)), // East Asian ideograph
    (0x6F4E53, (0xB729, false)), // Korean hangul
    (0x223B78, (0x6806, false)), // East Asian ideograph
    (0x223B79, (0x67AE, false)), // East Asian ideograph
    (0x213B7A, (0x5CEA, false)), // East Asian ideograph
    (0x336054, (0x985B, false)), // East Asian ideograph
    (0x213B7B, (0x5D07, false)), // East Asian ideograph
    (0x27527A, (0x804B, false)), // East Asian ideograph
    (0x213B7C, (0x5D06, false)), // East Asian ideograph
    (0x216627, (0x4FC5, false)), // East Asian ideograph
    (0x6F773E, (0xCB4C, false)), // Korean hangul
    (0x692435, (0x3055, false)), // Hiragana letter SA
    (0x233B7D, (0x8F2D, false)), // East Asian ideograph
    (0x455746, (0x672F, false)), // East Asian ideograph
    (0x213B7E, (0x5D16, false)), // East Asian ideograph
    (0x6F5059, (0xBB8C, false)), // Korean hangul
    (0x216629, (0x4FC9, false)), // East Asian ideograph
    (0x4B516A, (0x7EF7, false)), // East Asian ideograph
    (0x6F7737, (0xC5AB, false)), // Korean hangul
    (0x6F5A67, (0xD0A5, false)), // Korean hangul
    (0x6F5D23, (0xD5DD, false)), // Korean hangul
    (0x2D485C, (0x6F44, false)), // East Asian ideograph
    (0x6F505A, (0xBBA4, false)), // Korean hangul
    (0x21363F, (0x54E9, false)), // East Asian ideograph
    (0x22262F, (0x5DAE, false)), // East Asian ideograph
    (0x27516E, (0x7EB5, false)), // East Asian ideograph
    (0x283462, (0x6322, false)), // East Asian ideograph
    (0x334C7B, (0x767A, false)), // East Asian ideograph
    (0x216527, (0x4EF5, false)), // East Asian ideograph
    (0x216528, (0x4F07, false)), // East Asian ideograph
    (0x226529, (0x7893, false)), // East Asian ideograph
    (0x21652A, (0x4F00, false)), // East Asian ideograph
    (0x21652C, (0x4F0B, false)), // East Asian ideograph
    (0x22652D, (0x7896, false)), // East Asian ideograph
    (0x22652F, (0x78B2, false)), // East Asian ideograph
    (0x6F5371, (0xC288, false)), // Korean hangul
    (0x226531, (0x78A1, false)), // East Asian ideograph
    (0x216532, (0x4F3B, false)), // East Asian ideograph
    (0x292633, (0x84E3, false)), // East Asian ideograph
    (0x216536, (0x4F58, false)), // East Asian ideograph
    (0x216537, (0x4F62, false)), // East Asian ideograph
    (0x216539, (0x4F64, false)), // East Asian ideograph
    (0x21653A, (0x4F49, false)), // East Asian ideograph
    (0x22653B, (0x78A4, false)), // East Asian ideograph
    (0x22653E, (0x78B4, false)), // East Asian ideograph
    (0x21653F, (0x4F3E, false)), // East Asian ideograph
    (0x226540, (0x78AD, false)), // East Asian ideograph
    (0x226541, (0x78A3, false)), // East Asian ideograph
    (0x226543, (0x789E, false)), // East Asian ideograph
    (0x226544, (0x78A8, false)), // East Asian ideograph
    (0x232636, (0x857B, false)), // East Asian ideograph
    (0x214B5E, (0x746A, false)), // East Asian ideograph
    (0x226548, (0x78AB, false)), // East Asian ideograph
    (0x234F4B, (0x9847, false)), // East Asian ideograph
    (0x4B6637, (0x4FE3, false)), // East Asian ideograph
    (0x21654D, (0x4F68, false)), // East Asian ideograph
    (0x22654E, (0x78BB, false)), // East Asian ideograph
    (0x21654F, (0x4F5F, false)), // East Asian ideograph
    (0x6F505C, (0xBBC4, false)), // Korean hangul
    (0x29496F, (0x95FC, false)), // East Asian ideograph
    (0x226555, (0x78CC, false)), // East Asian ideograph
    (0x226556, (0x78C9, false)), // East Asian ideograph
    (0x216557, (0x4F7C, false)), // East Asian ideograph
    (0x226558, (0x78D1, false)), // East Asian ideograph
    (0x21655A, (0x4F98, false)), // East Asian ideograph
    (0x21655B, (0x4F92, false)), // East Asian ideograph
    (0x21655C, (0x4F7D, false)), // East Asian ideograph
    (0x22655E, (0x78C8, false)), // East Asian ideograph
    (0x226560, (0x78D4, false)), // East Asian ideograph
    (0x274E3B, (0x781A, false)), // East Asian ideograph
    (0x216562, (0x4F76, false)), // East Asian ideograph
    (0x216564, (0x4FA2, false)), // East Asian ideograph
    (0x4C6565, (0x78B9, false)), // East Asian ideograph
    (0x216566, (0x4F91, false)), // East Asian ideograph
    (0x216567, (0x4F95, false)), // East Asian ideograph
    (0x226568, (0x78DF, false)), // East Asian ideograph
    (0x22656A, (0x78E7, false)), // East Asian ideograph
    (0x21656C, (0x4F4C, false)), // East Asian ideograph
    (0x21656D, (0x4F97, false)), // East Asian ideograph
    (0x22656E, (0x78DB, false)), // East Asian ideograph
    (0x22656F, (0x78E1, false)), // East Asian ideograph
    (0x216570, (0x4F79, false)), // East Asian ideograph
    (0x216571, (0x4F9A, false)), // East Asian ideograph
    (0x216572, (0x4F81, false)), // East Asian ideograph
    (0x216573, (0x4F78, false)), // East Asian ideograph
    (0x225C5A, (0x74F0, false)), // East Asian ideograph
    (0x4B516E, (0x7E26, false)), // East Asian ideograph
    (0x226576, (0x78EE, false)), // East Asian ideograph
    (0x226577, (0x78E3, false)), // East Asian ideograph
    (0x226579, (0x78F2, false)), // East Asian ideograph
    (0x21657B, (0x4F7A, false)), // East Asian ideograph
    (0x21657C, (0x4FCD, false)), // East Asian ideograph
    (0x2D5529, (0x830E, false)), // East Asian ideograph (variant of 275529)
    (0x22657E, (0x7905, false)), // East Asian ideograph
    (0x6F5D27, (0xD5EC, false)), // Korean hangul
    (0x6F505E, (0xBBD0, false)), // Korean hangul
    (0x217A75, (0x5A16, false)), // East Asian ideograph
    (0x224F5D, (0x7043, false)), // East Asian ideograph
    (0x216643, (0x4FB9, false)), // East Asian ideograph
    (0x232644, (0x8597, false)), // East Asian ideograph
    (0x283466, (0x63FF, false)), // East Asian ideograph
    (0x6F5D28, (0xD5F4, false)), // Korean hangul
    (0x287269, (0x7EC9, false)), // East Asian ideograph
    (0x2D3C38, (0x9245, false)), // East Asian ideograph
    (0x216646, (0x501E, false)), // East Asian ideograph
    (0x217E25, (0x5B67, false)), // East Asian ideograph
    (0x6F4B42, (0xB059, false)), // Korean hangul
    (0x6F505F, (0xBBF8, false)), // Korean hangul
    (0x282647, (0x5CBF, false)), // East Asian ideograph
    (0x275F4A, (0x867D, false)), // East Asian ideograph
    (0x225C5C, (0x74EE, false)), // East Asian ideograph
    (0x217633, (0x5800, false)), // East Asian ideograph
    (0x23605B, (0x9F8E, false)), // East Asian ideograph
    (0x276649, (0x4F1C, false)), // East Asian ideograph
    (0x274E3E, (0x7815, false)), // East Asian ideograph
    (0x293866, (0x8DB1, false)), // East Asian ideograph
    (0x29563C, (0x9B49, false)), // East Asian ideograph
    (0x6F5C73, (0xD585, false)), // Korean hangul
    (0x4B3C21, (0x5D5C, false)), // East Asian ideograph
    (0x21664C, (0x5007, false)), // East Asian ideograph
    (0x21664D, (0x5013, false)), // East Asian ideograph
    (0x23264E, (0x8586, false)), // East Asian ideograph
    (0x6F5D2A, (0xD5F7, false)), // Korean hangul
    (0x222650, (0x5DDB, false)), // East Asian ideograph
    (0x22606A, (0x76AA, false)), // East Asian ideograph
    (0x292651, (0x84DF, false)), // East Asian ideograph
    (0x275F4C, (0x9E21, false)), // East Asian ideograph
    (0x696466, (0x7CAD, false)), // East Asian ideograph
    (0x217635, (0x57EC, false)), // East Asian ideograph
    (0x6F5264, (0xC0B3, false)), // Korean hangul
    (0x393770, (0x56F2, false)), // East Asian ideograph
    (0x2D552D, (0x8358, false)), // East Asian ideograph
    (0x6F5D2B, (0xD5F9, false)), // Korean hangul
    (0x4B4066, (0x62F4, false)), // East Asian ideograph
    (0x286655, (0x783B, false)), // East Asian ideograph
    (0x4B3C23, (0x5CE5, false)), // East Asian ideograph
    (0x274177, (0x631E, false)), // East Asian ideograph
    (0x222656, (0x5DE4, false)), // East Asian ideograph
    (0x217636, (0x5807, false)), // East Asian ideograph
    (0x292658, (0x83B6, false)), // East Asian ideograph
    (0x282659, (0x5DEF, false)), // East Asian ideograph
    (0x692432, (0x3052, false)), // Hiragana letter GE
    (0x276068, (0x996C, false)), // East Asian ideograph
    (0x706D3B, (0x7818, false)), // East Asian ideograph
    (0x226621, (0x78F9, false)), // East Asian ideograph
    (0x226622, (0x78FD, false)), // East Asian ideograph
    (0x6F5063, (0xBC00, false)), // Korean hangul
    (0x216626, (0x4FB7, false)), // East Asian ideograph
    (0x226627, (0x78FE, false)), // East Asian ideograph
    (0x226629, (0x78FB, false)), // East Asian ideograph
    (0x21662A, (0x4FE5, false)), // East Asian ideograph
    (0x22662B, (0x7904, false)), // East Asian ideograph
    (0x21662C, (0x4FE7, false)), // East Asian ideograph
    (0x22662E, (0x7912, false)), // East Asian ideograph
    (0x226632, (0x790C, false)), // East Asian ideograph
    (0x216633, (0x4FDC, false)), // East Asian ideograph
    (0x226634, (0x7913, false)), // East Asian ideograph
    (0x216635, (0x4FD4, false)), // East Asian ideograph
    (0x216637, (0x4FC1, false)), // East Asian ideograph
    (0x21663B, (0x4FDB, false)), // East Asian ideograph
    (0x21663E, (0x4FC6, false)), // East Asian ideograph
    (0x706640, (0x80EC, false)), // East Asian ideograph
    (0x6F5064, (0xBC08, false)), // Korean hangul
    (0x226643, (0x791E, false)), // East Asian ideograph
    (0x6F4C21, (0xB128, false)), // Korean hangul
    (0x226646, (0x7922, false)), // East Asian ideograph
    (0x292661, (0x8360, false)), // East Asian ideograph
    (0x216648, (0x503F, false)), // East Asian ideograph
    (0x216649, (0x5005, false)), // East Asian ideograph
    (0x4D5973, (0x51EB, false)), // East Asian ideograph
    (0x22664C, (0x7924, false)), // East Asian ideograph
    (0x22664D, (0x7927, false)), // East Asian ideograph
    (0x21664E, (0x5022, false)), // East Asian ideograph
    (0x226650, (0x7929, false)), // East Asian ideograph
    (0x216652, (0x4FF5, false)), // East Asian ideograph
    (0x6F2463, (0x314D, false)), // Korean hangul
    (0x226655, (0x7931, false)), // East Asian ideograph
    (0x393428, (0x5227, false)), // East Asian ideograph
    (0x276235, (0x9E26, false)), // East Asian ideograph
    (0x216659, (0x4FF4, false)), // East Asian ideograph
    (0x21665B, (0x5037, false)), // East Asian ideograph
    (0x22665D, (0x7934, false)), // East Asian ideograph
    (0x21665E, (0x502E, false)), // East Asian ideograph
    (0x6F5065, (0xBC09, false)), // Korean hangul
    (0x226660, (0x7936, false)), // East Asian ideograph
    (0x216661, (0x4FF6, false)), // East Asian ideograph
    (0x216662, (0x501C, false)), // East Asian ideograph
    (0x6F4C22, (0xB12C, false)), // Korean hangul
    (0x226665, (0x793D, false)), // East Asian ideograph
    (0x216666, (0x502C, false)), // East Asian ideograph
    (0x226667, (0x7942, false)), // East Asian ideograph
    (0x226668, (0x793F, false)), // East Asian ideograph
    (0x216669, (0x5010, false)), // East Asian ideograph
    (0x22666A, (0x794A, false)), // East Asian ideograph
    (0x22666B, (0x794D, false)), // East Asian ideograph
    (0x292668, (0x8369, false)), // East Asian ideograph
    (0x226675, (0x7946, false)), // East Asian ideograph
    (0x226677, (0x7958, false)), // East Asian ideograph
    (0x216679, (0x503D, false)), // East Asian ideograph
    (0x22667A, (0x795C, false)), // East Asian ideograph
    (0x22667B, (0x794F, false)), // East Asian ideograph
    (0x22667C, (0x7953, false)), // East Asian ideograph
    (0x22667D, (0x7953, false)), // Unrelated variant of EACC 22667C which maps to 7953
    (0x6F4C23, (0xB134, false)), // Korean hangul
    (0x225C63, (0x74F8, false)), // East Asian ideograph
    (0x236062, (0x9F95, false)), // East Asian ideograph
    (0x2D336B, (0x5C05, false)), // East Asian ideograph
    (0x213F71, (0x6233, false)), // East Asian ideograph
    (0x6F5D30, (0xD610, false)), // Korean hangul
    (0x224B57, (0x6EA8, false)), // East Asian ideograph
    (0x27627D, (0x9F50, false)), // East Asian ideograph
    (0x6F5D77, (0xD774, false)), // Korean hangul
    (0x213B2E, (0x5BE2, false)), // East Asian ideograph
    (0x6F4C24, (0xB135, false)), // Korean hangul
    (0x21763B, (0x580F, false)), // East Asian ideograph
    (0x227A3A, (0x81CA, false)), // East Asian ideograph
    (0x232672, (0x85BF, false)), // East Asian ideograph
    (0x6F5528, (0xC557, false)), // Korean hangul
    (0x6F5068, (0xBC0D, false)), // Korean hangul
    (0x6F4C25, (0xB137, false)), // Korean hangul
    (0x295A48, (0x9E31, false)), // East Asian ideograph
    (0x395A36, (0x983C, false)), // East Asian ideograph
    (0x6F4939, (0xACA1, false)), // Korean hangul
    (0x275121, (0x7EB1, false)), // East Asian ideograph
    (0x222677, (0x5E12, false)), // East Asian ideograph
    (0x225122, (0x70DD, false)), // East Asian ideograph
    (0x225123, (0x70E1, false)), // East Asian ideograph
    (0x27417E, (0x62E9, false)), // East Asian ideograph
    (0x226679, (0x795B, false)), // East Asian ideograph
    (0x275F54, (0x4E91, false)), // East Asian ideograph
    (0x235124, (0x9907, false)), // East Asian ideograph
    (0x6F4C26, (0xB140, false)), // Korean hangul
    (0x225C66, (0x74FB, false)), // East Asian ideograph
    (0x275125, (0x7EB7, false)), // East Asian ideograph
    (0x4B3869, (0x5727, false)), // East Asian ideograph
    (0x235C22, (0x9DC7, false)), // East Asian ideograph
    (0x225126, (0x70E3, false)), // East Asian ideograph
    (0x6F5D33, (0xD614, false)), // Korean hangul
    (0x6F5127, (0xBC85, false)), // Korean hangul
    (0x235128, (0x9902, false)), // East Asian ideograph
    (0x6F5374, (0xC298, false)), // Korean hangul
    (0x6F506A, (0xBC11, false)), // Korean hangul
    (0x275129, (0x624E, false)), // East Asian ideograph
    (0x277D2B, (0x5A06, false)), // East Asian ideograph
    (0x225C67, (0x74FF, false)), // East Asian ideograph
    (0x27512A, (0x7ECD, false)), // East Asian ideograph
    (0x21512B, (0x7D44, false)), // East Asian ideograph
    (0x6F4F31, (0xB82C, false)), // Korean hangul
    (0x27413F, (0x626C, false)), // East Asian ideograph
    (0x6F5D34, (0xD615, false)), // Korean hangul
    (0x27512C, (0x7EC6, false)), // East Asian ideograph
    (0x27512D, (0x7EC5, false)), // East Asian ideograph
    (0x4C7328, (0x5FAD, false)), // East Asian ideograph (variant of 2E7328 which maps to 5FAD)
    (0x22512E, (0x70D1, false)), // East Asian ideograph
    (0x215869, (0x8AAA, false)), // East Asian ideograph
    (0x21512F, (0x7D40, false)), // East Asian ideograph
    (0x215130, (0x7D42, false)), // East Asian ideograph
    (0x216722, (0x506F, false)), // East Asian ideograph
    (0x216723, (0x5050, false)), // East Asian ideograph
    (0x216725, (0x5070, false)), // East Asian ideograph
    (0x215131, (0x7D71, false)), // East Asian ideograph
    (0x216729, (0x5053, false)), // East Asian ideograph
    (0x21672A, (0x506A, false)), // East Asian ideograph
    (0x21672C, (0x5056, false)), // East Asian ideograph
    (0x215132, (0x7D5E, false)), // East Asian ideograph
    (0x226730, (0x7972, false)), // East Asian ideograph
    (0x216731, (0x506D, false)), // East Asian ideograph
    (0x275133, (0x7ED2, false)), // East Asian ideograph
    (0x6F4C29, (0xB150, false)), // Korean hangul
    (0x216738, (0x505D, false)), // East Asian ideograph
    (0x215134, (0x7D50, false)), // East Asian ideograph
    (0x21673B, (0x5058, false)), // East Asian ideograph
    (0x21673C, (0x5072, false)), // East Asian ideograph
    (0x22673E, (0x797C, false)), // East Asian ideograph
    (0x3F6179, (0x5C1F, false)), // East Asian ideograph
    (0x216741, (0x5041, false)), // East Asian ideograph
    (0x6F5D36, (0xD638, false)), // Korean hangul
    (0x275136, (0x7EDA, false)), // East Asian ideograph
    (0x216746, (0x5015, false)), // East Asian ideograph
    (0x293430, (0x8BB4, false)), // East Asian ideograph
    (0x216748, (0x507A, false)), // East Asian ideograph
    (0x21674A, (0x506C, false)), // East Asian ideograph
    (0x275137, (0x7EDD, false)), // East Asian ideograph
    (0x21674D, (0x506B, false)), // East Asian ideograph
    (0x21674E, (0x5094, false)), // East Asian ideograph
    (0x22674F, (0x798B, false)), // East Asian ideograph
    (0x216750, (0x509E, false)), // East Asian ideograph
    (0x235138, (0x9915, false)), // East Asian ideograph
    (0x216752, (0x509B, false)), // East Asian ideograph
    (0x216753, (0x509A, false)), // East Asian ideograph
    (0x226754, (0x7994, false)), // East Asian ideograph
    (0x226755, (0x7993, false)), // East Asian ideograph
    (0x215139, (0x7D66, false)), // East Asian ideograph
    (0x21675A, (0x508C, false)), // East Asian ideograph
    (0x21675C, (0x5088, false)), // East Asian ideograph
    (0x23513A, (0x9924, false)), // East Asian ideograph
    (0x22675F, (0x79A1, false)), // East Asian ideograph
    (0x226760, (0x799B, false)), // East Asian ideograph
    (0x226761, (0x79A3, false)), // East Asian ideograph
    (0x216762, (0x508E, false)), // East Asian ideograph
    (0x23513B, (0x991F, false)), // East Asian ideograph
    (0x224B5E, (0x6E8E, false)), // East Asian ideograph
    (0x216767, (0x50A6, false)), // East Asian ideograph
    (0x274C76, (0x763E, false)), // East Asian ideograph
    (0x21513C, (0x7D93, false)), // East Asian ideograph
    (0x21676A, (0x5092, false)), // East Asian ideograph
    (0x21676C, (0x509C, false)), // East Asian ideograph
    (0x2D442D, (0x6780, false)), // East Asian ideograph
    (0x22676E, (0x79A9, false)), // East Asian ideograph
    (0x27513D, (0x6346, false)), // East Asian ideograph
    (0x226770, (0x79AB, false)), // East Asian ideograph
    (0x216771, (0x50C7, false)), // East Asian ideograph
    (0x216775, (0x50C9, false)), // East Asian ideograph
    (0x22677A, (0x79B3, false)), // East Asian ideograph
    (0x22513F, (0x70FA, false)), // East Asian ideograph
    (0x21677C, (0x50B4, false)), // East Asian ideograph
    (0x6F5D38, (0xD63C, false)), // Korean hangul
    (0x212A23, (0xE8D2, false)), // EACC component character
    (0x215140, (0x7D81, false)), // East Asian ideograph
    (0x334F5E, (0x7A91, false)), // East Asian ideograph
    (0x215141, (0x7D9C, false)), // East Asian ideograph
    (0x6F5375, (0xC29B, false)), // Korean hangul
    (0x213538, (0x53F0, false)), // East Asian ideograph (duplicate simplified)
    (0x6F506F, (0xBC16, false)), // Korean hangul
    (0x215142, (0x7DBB, false)), // East Asian ideograph
    (0x6F4C2C, (0xB154, false)), // Korean hangul
    (0x284140, (0x6861, false)), // East Asian ideograph
    (0x235143, (0x9929, false)), // East Asian ideograph
    (0x2D3765, (0x8086, false)), // East Asian ideograph
    (0x215144, (0x7DCA, false)), // East Asian ideograph
    (0x6F5D39, (0xD640, false)), // Korean hangul
    (0x215145, (0x7DBE, false)), // East Asian ideograph
    (0x224B60, (0x6ED9, false)), // East Asian ideograph
    (0x215146, (0x7DB4, false)), // East Asian ideograph
    (0x2E2D79, (0x6128, false)), // East Asian ideograph
    (0x4B5724, (0x86CD, false)), // East Asian ideograph
    (0x235147, (0x991A, false)), // East Asian ideograph
    (0x275242, (0x4E49, false)), // East Asian ideograph
    (0x6F4C2D, (0xB155, false)), // Korean hangul
    (0x6F5C48, (0xD46F, false)), // Korean hangul
    (0x215148, (0x7DB2, false)), // East Asian ideograph
    (0x215149, (0x7DB1, false)), // East Asian ideograph
    (0x6F5D3A, (0xD648, false)), // Korean hangul
    (0x21514A, (0x7DBD, false)), // East Asian ideograph
    (0x224B61, (0x6EBD, false)), // East Asian ideograph
    (0x234F60, (0x9852, false)), // East Asian ideograph
    (0x4B3C32, (0x5DD3, false)), // East Asian ideograph
    (0x6F5071, (0xBC1B, false)), // Korean hangul
    (0x22514C, (0x7103, false)), // East Asian ideograph
    (0x21586F, (0x8AA3, false)), // East Asian ideograph
    (0x21514D, (0x7DA2, false)), // East Asian ideograph
    (0x22582B, (0x736B, false)), // East Asian ideograph
    (0x4B615F, (0x9AEA, false)), // East Asian ideograph
    (0x21514E, (0x7DAD, false)), // East Asian ideograph
    (0x2D3324, (0x634C, false)), // East Asian ideograph
    (0x6F5D3B, (0xD649, false)), // Korean hangul
    (0x21514F, (0x7DBF, false)), // East Asian ideograph
    (0x2D356A, (0x8A36, false)), // East Asian ideograph
    (0x215150, (0x7DB8, false)), // East Asian ideograph
    (0x6F5072, (0xBC1C, false)), // Korean hangul
    (0x215151, (0x7DC7, false)), // East Asian ideograph
    (0x22482D, (0x6CF2, false)), // East Asian ideograph
    (0x275152, (0x7F14, false)), // East Asian ideograph
    (0x6F493B, (0xACA9, false)), // Korean hangul
    (0x2D3768, (0x56EC, false)), // East Asian ideograph
    (0x215153, (0x7DEF, false)), // East Asian ideograph
    (0x294346, (0x94D1, false)), // East Asian ideograph
    (0x2F5D3C, (0x6EF7, false)), // East Asian ideograph
    (0x215154, (0x7DF4, false)), // East Asian ideograph (variant of 4B5154 which maps to 7DF4)
    (0x224B63, (0x6EC1, false)), // East Asian ideograph
    (0x234F62, (0x984B, false)), // East Asian ideograph
    (0x235155, (0x9932, false)), // East Asian ideograph
    (0x6F5073, (0xBC1D, false)), // Korean hangul
    (0x225156, (0x7112, false)), // East Asian ideograph
    (0x6F4C30, (0xB178, false)), // Korean hangul
    (0x69675C, (0x825D, false)), // East Asian ideograph
    (0x215157, (0x7DEC, false)), // East Asian ideograph
    (0x234179, (0x91E9, false)), // East Asian ideograph
    (0x215158, (0x7DDD, false)), // East Asian ideograph
    (0x213E37, (0x6012, false)), // East Asian ideograph
    (0x6F5D3D, (0xD64D, false)), // Korean hangul
    (0x215159, (0x7DE9, false)), // East Asian ideograph
    (0x21515A, (0x7DE3, false)), // East Asian ideograph
    (0x213539, (0x53E5, false)), // East Asian ideograph
    (0x6F5074, (0xBC1F, false)), // Korean hangul
    (0x216822, (0x50C2, false)), // East Asian ideograph
    (0x27515B, (0x7F16, false)), // East Asian ideograph
    (0x226825, (0x79BC, false)), // East Asian ideograph
    (0x225C71, (0x7505, false)), // East Asian ideograph
    (0x226828, (0x79C6, false)), // East Asian ideograph
    (0x22515C, (0x710C, false)), // East Asian ideograph
    (0x22682A, (0x79C8, false)), // East Asian ideograph
    (0x21682C, (0x50BA, false)), // East Asian ideograph
    (0x22682D, (0x79D4, false)), // East Asian ideograph
    (0x21682E, (0x50CD, false)), // East Asian ideograph
    (0x21515D, (0x7D9E, false)), // East Asian ideograph
    (0x6F4F33, (0xB835, false)), // Korean hangul
    (0x226832, (0x79D6, false)), // East Asian ideograph
    (0x216834, (0x50EF, false)), // East Asian ideograph
    (0x21515E, (0x7DDE, false)), // East Asian ideograph
    (0x293438, (0x8C29, false)), // East Asian ideograph
    (0x21683A, (0x50F4, false)), // East Asian ideograph
    (0x21515F, (0x7E11, false)), // East Asian ideograph
    (0x21683C, (0x50DD, false)), // East Asian ideograph
    (0x22683D, (0x79EC, false)), // East Asian ideograph
    (0x22683E, (0x79EB, false)), // East Asian ideograph (variant of 4C683E which maps to 79EB)
    (0x6F5075, (0xBC24, false)), // Korean hangul
    (0x215160, (0x7E0A, false)), // East Asian ideograph
    (0x226842, (0x79E1, false)), // East Asian ideograph
    (0x6F4C32, (0xB17A, false)), // Korean hangul
    (0x226844, (0x79DD, false)), // East Asian ideograph
    (0x226845, (0x79ED, false)), // East Asian ideograph
    (0x216846, (0x50D9, false)), // East Asian ideograph
    (0x215161, (0x7E08, false)), // East Asian ideograph
    (0x226848, (0x79F8, false)), // East Asian ideograph
    (0x215162, (0x7E1B, false)), // East Asian ideograph
    (0x2E684E, (0x8020, false)), // East Asian ideograph
    (0x22684F, (0x7A02, false)), // East Asian ideograph
    (0x226850, (0x7A0A, false)), // East Asian ideograph
    (0x6F5D3F, (0xD654, false)), // Korean hangul
    (0x6F5625, (0xC651, false)), // Korean hangul
    (0x275163, (0x81F4, false)), // East Asian ideograph
    (0x226854, (0x7A09, false)), // East Asian ideograph
    (0x216855, (0x50EC, false)), // East Asian ideograph
    (0x4B442D, (0x67A9, false)), // East Asian ideograph
    (0x215164, (0x7E23, false)), // East Asian ideograph
    (0x21685B, (0x510E, false)), // East Asian ideograph
    (0x22685C, (0x7A03, false)), // East Asian ideograph
    (0x6F5076, (0xBC25, false)), // Korean hangul
    (0x275165, (0x7F29, false)), // East Asian ideograph
    (0x226861, (0x7A0C, false)), // East Asian ideograph
    (0x293160, (0x89EF, false)), // East Asian ideograph
    (0x225166, (0x7113, false)), // East Asian ideograph
    (0x216866, (0x5107, false)), // East Asian ideograph
    (0x216867, (0x510F, false)), // East Asian ideograph
    (0x216868, (0x50FE, false)), // East Asian ideograph
    (0x216869, (0x510B, false)), // East Asian ideograph
    (0x21686A, (0x50FD, false)), // East Asian ideograph
    (0x22686B, (0x7A11, false)), // East Asian ideograph
    (0x22686C, (0x7A18, false)), // East Asian ideograph
    (0x21686D, (0x5101, false)), // East Asian ideograph
    (0x234E43, (0x97C9, false)), // East Asian ideograph
    (0x22686F, (0x7A19, false)), // East Asian ideograph (variant of 2E686F which maps to 7A19)
    (0x6F5D40, (0xD655, false)), // Korean hangul
    (0x226871, (0x7A1E, false)), // East Asian ideograph
    (0x216872, (0x5113, false)), // East Asian ideograph
    (0x234F66, (0x983F, false)), // East Asian ideograph
    (0x226876, (0x7A17, false)), // East Asian ideograph
    (0x275169, (0x7F27, false)), // East Asian ideograph
    (0x216878, (0x511A, false)), // East Asian ideograph
    (0x216879, (0x9797, false)), // East Asian ideograph
    (0x6F5077, (0xBC27, false)), // Korean hangul
    (0x21516A, (0x7E43, false)), // East Asian ideograph
    (0x21687E, (0x5126, false)), // East Asian ideograph
    (0x6F4C34, (0xB180, false)), // Korean hangul
    (0x223A5B, (0x6745, false)), // East Asian ideograph
    (0x33516B, (0x7DD0, false)), // East Asian ideograph
    (0x4C735D, (0x7D4B, false)), // East Asian ideograph
    (0x22516C, (0x711E, false)), // East Asian ideograph
    (0x2E3729, (0x65B5, false)), // East Asian ideograph
    (0x6F5D41, (0xD658, false)), // Korean hangul
    (0x21516D, (0x7E3D, false)), // East Asian ideograph
    (0x6F5451, (0xC3D8, false)), // Korean hangul
    (0x22516E, (0x7120, false)), // East Asian ideograph
    (0x2D4437, (0x67FE, false)), // East Asian ideograph
    (0x21516F, (0x7E45, false)), // East Asian ideograph
    (0x6F4C35, (0xB188, false)), // Korean hangul
    (0x215170, (0x7E55, false)), // East Asian ideograph
    (0x275174, (0x7F2D, false)), // East Asian ideograph
    (0x235171, (0x994D, false)), // East Asian ideograph
    (0x473539, (0x8B9E, false)), // East Asian ideograph
    (0x29426D, (0x94CD, false)), // East Asian ideograph
    (0x275172, (0x7EE3, false)), // East Asian ideograph
    (0x224B69, (0x6EBB, false)), // East Asian ideograph
    (0x275173, (0x7ED5, false)), // East Asian ideograph
    (0x6F5B23, (0xD0F0, false)), // Korean hangul
    (0x215174, (0x7E5A, false)), // East Asian ideograph
    (0x6F4C36, (0xB189, false)), // Korean hangul
    (0x225175, (0x712D, false)), // East Asian ideograph
    (0x2D376F, (0x5700, false)), // East Asian ideograph
    (0x275176, (0x7EF3, false)), // East Asian ideograph
    (0x213C21, (0x5D0E, false)), // East Asian ideograph
    (0x275177, (0x8327, false)), // East Asian ideograph
    (0x2D3C22, (0x5D10, false)), // East Asian ideograph
    (0x275178, (0x7ECE, false)), // East Asian ideograph
    (0x223C23, (0x67C2, false)), // East Asian ideograph
    (0x6F507A, (0xBC30, false)), // Korean hangul
    (0x275179, (0x7ED8, false)), // East Asian ideograph
    (0x215878, (0x8AD2, false)), // East Asian ideograph
    (0x225C77, (0x7503, false)), // East Asian ideograph
    (0x27517A, (0x8FAB, false)), // East Asian ideograph
    (0x217C25, (0x5A9E, false)), // East Asian ideograph
    (0x45465B, (0x6C2F, false)), // East Asian ideograph
    (0x21517B, (0x7E7D, false)), // East Asian ideograph
    (0x223C26, (0x67CA, false)), // East Asian ideograph
    (0x274E59, (0x7840, false)), // East Asian ideograph
    (0x6F5D44, (0xD667, false)), // Korean hangul
    (0x6F517C, (0xBE61, false)), // Korean hangul
    (0x213C27, (0x5D4C, false)), // East Asian ideograph
    (0x234F6A, (0x985C, false)), // East Asian ideograph
    (0x27517D, (0x7EE7, false)), // East Asian ideograph
    (0x223C28, (0x67CE, false)), // East Asian ideograph
    (0x2D443A, (0x6942, false)), // East Asian ideograph
    (0x23517E, (0x9955, false)), // East Asian ideograph
    (0x213B32, (0x5BE7, false)), // East Asian ideograph
    (0x213C29, (0x5D69, false)), // East Asian ideograph
    (0x223C2A, (0x67F2, false)), // East Asian ideograph
    (0x275E3E, (0x94DB, false)), // East Asian ideograph
    (0x2D5547, (0x837D, false)), // East Asian ideograph
    (0x223C2B, (0x67C3, false)), // East Asian ideograph
    (0x6F5D45, (0xD669, false)), // Korean hangul
    (0x234F6B, (0x9859, false)), // East Asian ideograph
    (0x223C2D, (0x67DD, false)), // East Asian ideograph
    (0x6F507C, (0xBC34, false)), // Korean hangul
    (0x275F67, (0x96FE, false)), // East Asian ideograph
    (0x213C2E, (0x5DBD, false)), // East Asian ideograph
    (0x6F4D43, (0xB3D0, false)), // Korean hangul
    (0x233E5F, (0x90AD, false)), // East Asian ideograph
    (0x213C2F, (0x5DBA, false)), // East Asian ideograph (variant of 4B3C2F which maps to 5DBA)
    (0x6F493D, (0xACAC, false)), // Korean hangul
    (0x233C30, (0x8F46, false)), // East Asian ideograph
    (0x226922, (0x7A2C, false)), // East Asian ideograph
    (0x6F5D46, (0xD670, false)), // Korean hangul
    (0x233C31, (0x8F4A, false)), // East Asian ideograph
    (0x216929, (0x5124, false)), // East Asian ideograph
    (0x6F5452, (0xC3D9, false)), // Korean hangul
    (0x21692B, (0x5129, false)), // East Asian ideograph
    (0x213C32, (0x5DD4, false)), // East Asian ideograph
    (0x6F507D, (0xBC37, false)), // Korean hangul
    (0x216930, (0x5131, false)), // East Asian ideograph
    (0x273C33, (0x5CA9, false)), // East Asian ideograph
    (0x29454D, (0x9534, false)), // East Asian ideograph
    (0x275175, (0x7CFB, false)), // East Asian ideograph (duplicate simplified)
    (0x226939, (0x7A48, false)), // East Asian ideograph
    (0x22693D, (0x7A4B, false)), // East Asian ideograph
    (0x22693E, (0x7A47, false)), // East Asian ideograph
    (0x22693F, (0x7A44, false)), // East Asian ideograph
    (0x274E5C, (0x77FE, false)), // East Asian ideograph
    (0x6F5D47, (0xD671, false)), // Korean hangul
    (0x216944, (0x513A, false)), // East Asian ideograph
    (0x213C36, (0x5DE2, false)), // East Asian ideograph
    (0x696946, (0x8630, false)), // East Asian ideograph
    (0x216947, (0x5139, false)), // East Asian ideograph
    (0x216948, (0x513B, false)), // East Asian ideograph
    (0x213C37, (0x5DE5, false)), // East Asian ideograph
    (0x22694D, (0x7A5F, false)), // East Asian ideograph
    (0x22694F, (0x7A60, false)), // East Asian ideograph
    (0x216951, (0x5159, false)), // East Asian ideograph
    (0x216952, (0x515B, false)), // East Asian ideograph
    (0x213663, (0x55AA, false)), // East Asian ideograph
    (0x29454E, (0x9545, false)), // East Asian ideograph
    (0x216955, (0x515D, false)), // East Asian ideograph
    (0x216956, (0x515E, false)), // East Asian ideograph
    (0x225838, (0x737E, false)), // East Asian ideograph
    (0x216958, (0x515F, false)), // East Asian ideograph
    (0x216959, (0x5161, false)), // East Asian ideograph
    (0x69695B, (0x86AB, false)), // East Asian ideograph
    (0x21695C, (0x5163, false)), // East Asian ideograph
    (0x6F4F35, (0xB838, false)), // Korean hangul
    (0x274E5D, (0x783A, false)), // East Asian ideograph
    (0x22695F, (0x7A70, false)), // East Asian ideograph
    (0x6F5D48, (0xD683, false)), // Korean hangul
    (0x696962, (0x86EF, false)), // East Asian ideograph
    (0x213C3B, (0x5DEB, false)), // East Asian ideograph
    (0x226966, (0x7A75, false)), // East Asian ideograph
    (0x216967, (0x5182, false)), // East Asian ideograph
    (0x216969, (0x5184, false)), // East Asian ideograph
    (0x22696B, (0x7A80, false)), // East Asian ideograph
    (0x21696E, (0x518F, false)), // East Asian ideograph
    (0x213C3D, (0x5DF1, false)), // East Asian ideograph
    (0x216970, (0x5194, false)), // East Asian ideograph
    (0x216971, (0x5193, false)), // East Asian ideograph
    (0x2E403D, (0x6AC1, false)), // East Asian ideograph
    (0x216975, (0x5196, false)), // East Asian ideograph
    (0x226978, (0x7A8A, false)), // East Asian ideograph
    (0x22697A, (0x7A94, false)), // East Asian ideograph
    (0x21697B, (0x51A1, false)), // East Asian ideograph
    (0x21697C, (0x51A3, false)), // East Asian ideograph
    (0x22697E, (0x68A5, false)), // East Asian ideograph
    (0x213C40, (0x5DF4, false)), // East Asian ideograph
    (0x223C41, (0x6832, false)), // East Asian ideograph
    (0x213C42, (0x5DFD, false)), // East Asian ideograph
    (0x275B28, (0x8E0C, false)), // East Asian ideograph
    (0x213C43, (0x5DFE, false)), // East Asian ideograph
    (0x275E3F, (0x94CE, false)), // East Asian ideograph
    (0x213C44, (0x5E02, false)), // East Asian ideograph
    (0x6F5471, (0xC510, false)), // Korean hangul
    (0x29565D, (0x9C82, false)), // East Asian ideograph
    (0x217C45, (0x5AB7, false)), // East Asian ideograph
    (0x2D4440, (0x6822, false)), // East Asian ideograph
    (0x223C47, (0x682B, false)), // East Asian ideograph
    (0x294551, (0x9517, false)), // East Asian ideograph
    (0x223C48, (0x682D, false)), // East Asian ideograph
    (0x6F493E, (0xACAF, false)), // Korean hangul
    (0x235C3A, (0x9DDF, false)), // East Asian ideograph
    (0x233C49, (0x8F57, false)), // East Asian ideograph
    (0x6F5D4B, (0xD68D, false)), // Korean hangul
    (0x213C4A, (0x5E16, false)), // East Asian ideograph
    (0x334F71, (0x54B2, false)), // East Asian ideograph
    (0x213C4B, (0x5E15, false)), // East Asian ideograph
    (0x275F6D, (0x972D, false)), // East Asian ideograph
    (0x213C4C, (0x5E1B, false)), // East Asian ideograph
    (0x4B3321, (0x5185, false)), // East Asian ideograph
    (0x233C4D, (0x8F5C, false)), // East Asian ideograph
    (0x213C4E, (0x5E1D, false)), // East Asian ideograph
    (0x284F7D, (0x704F, false)), // East Asian ideograph
    (0x29426F, (0x94BD, false)), // East Asian ideograph
    (0x223C4F, (0x6844, false)), // East Asian ideograph
    (0x4B5E5D, (0x95D4, false)), // East Asian ideograph
    (0x224730, (0x6C78, false)), // East Asian ideograph
    (0x6F5379, (0xC2A8, false)), // Korean hangul
    (0x217C50, (0x5ABA, false)), // East Asian ideograph
    (0x275F6E, (0x96F3, false)), // East Asian ideograph
    (0x213C51, (0x5E2B, false)), // East Asian ideograph
    (0x6F5B26, (0xD131, false)), // Korean hangul
    (0x213668, (0x55AE, false)), // East Asian ideograph
    (0x232635, (0x8598, false)), // East Asian ideograph
    (0x213C52, (0x5E33, false)), // East Asian ideograph
    (0x233C53, (0x8F5D, false)), // East Asian ideograph
    (0x6F5D4D, (0xD6A1, false)), // Korean hangul
    (0x224731, (0x6C74, false)), // East Asian ideograph
    (0x213C55, (0x5E37, false)), // East Asian ideograph
    (0x275F6F, (0x7075, false)), // East Asian ideograph
    (0x213C56, (0x5E45, false)), // East Asian ideograph
    (0x6F4C41, (0xB1E8, false)), // Korean hangul
    (0x275B2C, (0x8E8F, false)), // East Asian ideograph
    (0x213226, (0x5000, false)), // East Asian ideograph
    (0x223C58, (0x6834, false)), // East Asian ideograph
    (0x334F37, (0x5EE9, false)), // East Asian ideograph
    (0x695D36, (0x6B1F, false)), // East Asian ideograph
    (0x223C59, (0x6812, false)), // East Asian ideograph
    (0x224732, (0x6C86, false)), // East Asian ideograph
    (0x213C5A, (0x5E5B, false)), // East Asian ideograph
    (0x216A22, (0x51AA, false)), // East Asian ideograph
    (0x216A23, (0x51AB, false)), // East Asian ideograph
    (0x6F4C42, (0xB1FD, false)), // Korean hangul
    (0x216A26, (0x51B1, false)), // East Asian ideograph
    (0x29233C, (0x836D, false)), // East Asian ideograph
    (0x226A28, (0x7AA3, false)), // East Asian ideograph
    (0x213227, (0x4FEE, false)), // East Asian ideograph
    (0x226A2B, (0x7A9E, false)), // East Asian ideograph
    (0x226A2C, (0x7AA7, false)), // East Asian ideograph
    (0x226A2E, (0x7AA8, false)), // East Asian ideograph
    (0x46284C, (0x5ED0, false)), // East Asian ideograph
    (0x226A31, (0x7AAC, false)), // East Asian ideograph
    (0x6F5D4F, (0xD6C4, false)), // Korean hangul
    (0x216A35, (0x51BC, false)), // East Asian ideograph
    (0x226A36, (0x7AB3, false)), // East Asian ideograph
    (0x226A3A, (0x7ABD, false)), // East Asian ideograph
    (0x2D3C5F, (0x6A66, false)), // East Asian ideograph
    (0x226A3C, (0x7AB6, false)), // East Asian ideograph
    (0x226A3D, (0x7AB8, false)), // East Asian ideograph
    (0x226A3E, (0x7AB5, false)), // East Asian ideograph
    (0x226A3F, (0x7ABB, false)), // East Asian ideograph
    (0x213C60, (0x5E5F, false)), // East Asian ideograph
    (0x6F4C43, (0xB204, false)), // Korean hangul
    (0x216A43, (0x51CA, false)), // East Asian ideograph
    (0x216A46, (0x51C7, false)), // East Asian ideograph
    (0x213C61, (0x5E6B, false)), // East Asian ideograph
    (0x226A49, (0x7ACD, false)), // East Asian ideograph
    (0x226A4B, (0x7ACF, false)), // East Asian ideograph
    (0x216A4E, (0x51D1, false)), // East Asian ideograph
    (0x216A4F, (0x51D0, false)), // East Asian ideograph
    (0x287271, (0x7EA4, false)), // East Asian ideograph (duplicate simplified)
    (0x226A51, (0x7AD3, false)), // East Asian ideograph
    (0x226A52, (0x7AD4, false)), // East Asian ideograph
    (0x216A54, (0x51D3, false)), // East Asian ideograph
    (0x226A55, (0x7ADA, false)), // East Asian ideograph
    (0x226A5A, (0x7AE1, false)), // East Asian ideograph
    (0x226A5E, (0x7AE6, false)), // East Asian ideograph
    (0x233C65, (0x8FA4, false)), // East Asian ideograph
    (0x277D48, (0x5AD2, false)), // East Asian ideograph
    (0x696A61, (0x88C3, false)), // East Asian ideograph
    (0x21765B, (0x57DD, false)), // East Asian ideograph
    (0x216A63, (0x51D9, false)), // East Asian ideograph
    (0x226A66, (0x7AEB, false)), // East Asian ideograph
    (0x216A68, (0x51E2, false)), // East Asian ideograph
    (0x226A6B, (0x7AF0, false)), // East Asian ideograph
    (0x696A6D, (0x8904, false)), // East Asian ideograph
    (0x6F5D51, (0xD6C8, false)), // Korean hangul
    (0x213C68, (0x5E7B, false)), // East Asian ideograph
    (0x216A73, (0x5160, false)), // East Asian ideograph
    (0x226A76, (0x7AF5, false)), // East Asian ideograph
    (0x213C69, (0x5E7C, false)), // East Asian ideograph
    (0x216A78, (0x51F5, false)), // East Asian ideograph
    (0x216A79, (0x51F7, false)), // East Asian ideograph
    (0x226A7C, (0x7AFE, false)), // East Asian ideograph
    (0x2D3C6A, (0x51FC, false)), // East Asian ideograph
    (0x273C6B, (0x51E0, false)), // East Asian ideograph
    (0x4B4D56, (0x8846, false)), // East Asian ideograph
    (0x2D5554, (0x855A, false)), // East Asian ideograph
    (0x213C6C, (0x5E8F, false)), // East Asian ideograph
    (0x6F5D52, (0xD6CC, false)), // Korean hangul
    (0x233C6D, (0x8FB7, false)), // East Asian ideograph
    (0x295222, (0x98E8, false)), // East Asian ideograph
    (0x234B35, (0x96A9, false)), // East Asian ideograph
    (0x227333, (0x7E50, false)), // East Asian ideograph
    (0x6F4C46, (0xB20B, false)), // Korean hangul
    (0x275B31, (0x8EAF, false)), // East Asian ideograph
    (0x213C70, (0x5E97, false)), // East Asian ideograph
    (0x22326A, (0x63F9, false)), // East Asian ideograph
    (0x223C71, (0x689B, false)), // East Asian ideograph
    (0x6F5D53, (0xD6D1, false)), // Korean hangul
    (0x213C72, (0x5E9C, false)), // East Asian ideograph
    (0x29344D, (0x8C2E, false)), // East Asian ideograph
    (0x6F5972, (0xCE7C, false)), // Korean hangul
    (0x223C74, (0x68B6, false)), // East Asian ideograph
    (0x6F4C47, (0xB20C, false)), // Korean hangul
    (0x275B32, (0x8F66, false)), // East Asian ideograph
    (0x213C75, (0x5EA6, false)), // East Asian ideograph
    (0x223C76, (0x6882, false)), // East Asian ideograph
    (0x226721, (0x7951, false)), // East Asian ideograph
    (0x6F5D54, (0xD6D4, false)), // Korean hangul
    (0x273C77, (0x5750, false)), // East Asian ideograph
    (0x23595C, (0x9C6F, false)), // East Asian ideograph
    (0x234B37, (0x96AE, false)), // East Asian ideograph
    (0x226723, (0x7954, false)), // East Asian ideograph
    (0x28232B, (0x5C66, false)), // East Asian ideograph
    (0x232724, (0x8624, false)), // East Asian ideograph
    (0x223C7A, (0x6890, false)), // East Asian ideograph
    (0x4B4D59, (0x775B, false)), // East Asian ideograph (variant of 214D59 which maps to 775B)
    (0x2F317D, (0x8A7E, false)), // East Asian ideograph
    (0x213C7B, (0x5EB6, false)), // East Asian ideograph
    (0x6F5D55, (0xD6D7, false)), // Korean hangul
    (0x275D67, (0x94DD, false)), // East Asian ideograph
    (0x217C7C, (0x5AEB, false)), // East Asian ideograph
    (0x2D462C, (0x6B7A, false)), // East Asian ideograph
    (0x224739, (0x6C67, false)), // East Asian ideograph
    (0x233C7D, (0x8FCD, false)), // East Asian ideograph
    (0x4B5A23, (0x621D, false)), // East Asian ideograph
    (0x213C7E, (0x5EC1, false)), // East Asian ideograph
    (0x2D4756, (0x6F94, false)), // East Asian ideograph
    (0x275B34, (0x519B, false)), // East Asian ideograph
    (0x4B5C47, (0x9059, false)), // East Asian ideograph
    (0x22672A, (0x7967, false)), // East Asian ideograph
    (0x235C45, (0x9DD6, false)), // East Asian ideograph
    (0x6F4866, (0xAC10, false)), // Korean hangul
    (0x6F5B27, (0xD134, false)), // Korean hangul
    (0x4C6266, (0x778B, false)), // East Asian ideograph
    (0x22672D, (0x796B, false)), // East Asian ideograph
    (0x2D6222, (0x9C0C, false)), // East Asian ideograph
    (0x6F4C4A, (0xB215, false)), // Korean hangul
    (0x275B35, (0x8F68, false)), // East Asian ideograph
    (0x6F4F38, (0xB85C, false)), // Korean hangul
    (0x33476F, (0x6D44, false)), // East Asian ideograph
    (0x6F5D57, (0xD6E4, false)), // Korean hangul
    (0x216B24, (0x5213, false)), // East Asian ideograph
    (0x216B26, (0x5216, false)), // East Asian ideograph
    (0x226B27, (0x7B39, false)), // East Asian ideograph
    (0x22473B, (0x6C84, false)), // East Asian ideograph
    (0x216B2A, (0x521C, false)), // East Asian ideograph
    (0x226B2D, (0x7B0F, false)), // East Asian ideograph
    (0x226B2E, (0x7B08, false)), // East Asian ideograph
    (0x275F79, (0x9765, false)), // East Asian ideograph
    (0x6F4C4B, (0xB217, false)), // Korean hangul
    (0x226B33, (0x7B0A, false)), // East Asian ideograph
    (0x29455E, (0x94E1, false)), // East Asian ideograph
    (0x226B35, (0x7B35, false)), // East Asian ideograph
    (0x226B36, (0x7B25, false)), // East Asian ideograph
    (0x216B37, (0x5232, false)), // East Asian ideograph
    (0x226B39, (0x7B38, false)), // East Asian ideograph
    (0x226B3B, (0x7B3B, false)), // East Asian ideograph
    (0x216B3E, (0x5244, false)), // East Asian ideograph
    (0x226B3F, (0x7B24, false)), // East Asian ideograph
    (0x226B40, (0x7B33, false)), // East Asian ideograph
    (0x226B42, (0x7B2A, false)), // East Asian ideograph
    (0x216B43, (0x5249, false)), // East Asian ideograph
    (0x226B44, (0x7B18, false)), // East Asian ideograph
    (0x282736, (0x5E0F, false)), // East Asian ideograph
    (0x226B47, (0x7B31, false)), // East Asian ideograph
    (0x234B3B, (0x96B0, false)), // East Asian ideograph
    (0x226B4A, (0x7B2B, false)), // East Asian ideograph
    (0x216B4B, (0x525A, false)), // East Asian ideograph
    (0x216B4C, (0x5252, false)), // East Asian ideograph
    (0x226B4D, (0x7B1F, false)), // East Asian ideograph
    (0x213B36, (0x5BE9, false)), // East Asian ideograph
    (0x216B50, (0x525F, false)), // East Asian ideograph
    (0x226B52, (0x7B4A, false)), // East Asian ideograph
    (0x226B53, (0x7B59, false)), // East Asian ideograph (not in Unicode)
    (0x226B54, (0x7B04, false)), // East Asian ideograph (variant of 2E6B54 which maps to 7B04)
    (0x226B55, (0x7B47, false)), // East Asian ideograph
    (0x216739, (0x5048, false)), // East Asian ideograph
    (0x226B59, (0x7B58, false)), // East Asian ideograph
    (0x226B5B, (0x7B6C, false)), // East Asian ideograph
    (0x696B5C, (0x8ADA, false)), // East Asian ideograph
    (0x216B5E, (0x5268, false)), // East Asian ideograph
    (0x216B5F, (0x7B9A, false)), // East Asian ideograph
    (0x226B60, (0x7B48, false)), // East Asian ideograph
    (0x226B61, (0x7B45, false)), // East Asian ideograph
    (0x226B62, (0x7B4C, false)), // East Asian ideograph
    (0x226B63, (0x7B4E, false)), // East Asian ideograph
    (0x234B3C, (0x96B2, false)), // East Asian ideograph
    (0x226B68, (0x7B66, false)), // East Asian ideograph
    (0x706B6A, (0x8159, false)), // East Asian ideograph
    (0x216B6B, (0x5278, false)), // East Asian ideograph
    (0x226B6C, (0x7B64, false)), // East Asian ideograph
    (0x226B6E, (0x7B69, false)), // East Asian ideograph
    (0x275B38, (0x8F6F, false)), // East Asian ideograph
    (0x226B70, (0x7B6D, false)), // East Asian ideograph
    (0x226B74, (0x7B62, false)), // East Asian ideograph
    (0x226B75, (0x7B6E, false)), // East Asian ideograph
    (0x226B76, (0x7B74, false)), // East Asian ideograph
    (0x233A30, (0x8E50, false)), // East Asian ideograph
    (0x216B79, (0x528C, false)), // East Asian ideograph
    (0x216B7A, (0x528A, false)), // East Asian ideograph
    (0x226B7B, (0x7B6F, false)), // East Asian ideograph
    (0x216B7C, (0x5290, false)), // East Asian ideograph
    (0x226B7E, (0x7B65, false)), // East Asian ideograph
    (0x4B3A2F, (0x805F, false)), // East Asian ideograph
    (0x275B39, (0x8F6D, false)), // East Asian ideograph
    (0x6F4867, (0xAC11, false)), // Korean hangul
    (0x27457A, (0x6B20, false)), // East Asian ideograph (duplicate simplified)
    (0x222969, (0x5F54, false)), // East Asian ideograph
    (0x4B3C53, (0x5E2F, false)), // East Asian ideograph
    (0x234B3E, (0x96B3, false)), // East Asian ideograph
    (0x4C6564, (0x78D9, false)), // East Asian ideograph
    (0x282747, (0x5E3B, false)), // East Asian ideograph
    (0x22584C, (0x7393, false)), // East Asian ideograph
    (0x6F4F39, (0xB85D, false)), // Korean hangul
    (0x2F5D5C, (0x730A, false)), // East Asian ideograph
    (0x294944, (0x9603, false)), // East Asian ideograph
    (0x22674A, (0x7998, false)), // East Asian ideograph
    (0x33306C, (0x8B90, false)), // East Asian ideograph
    (0x21674B, (0x505F, false)), // East Asian ideograph
    (0x273D65, (0x540E, false)), // East Asian ideograph
    (0x6F4C50, (0xB25C, false)), // Korean hangul
    (0x27583B, (0x8BA6, false)), // East Asian ideograph
    (0x213235, (0x5074, false)), // East Asian ideograph
    (0x22674D, (0x7999, false)), // East Asian ideograph
    (0x22674E, (0x7995, false)), // East Asian ideograph
    (0x6F5D5D, (0xD711, false)), // Korean hangul
    (0x226750, (0x7996, false)), // East Asian ideograph
    (0x333051, (0x8CB3, false)), // East Asian ideograph
    (0x2D6229, (0x9C53, false)), // East Asian ideograph
    (0x6F4C51, (0xB260, false)), // Korean hangul
    (0x275B3C, (0x8F76, false)), // East Asian ideograph
    (0x294564, (0x9536, false)), // East Asian ideograph
    (0x292752, (0x830F, false)), // East Asian ideograph
    (0x233A34, (0x8E5C, false)), // East Asian ideograph
    (0x6F5A29, (0xCEF5, false)), // Korean hangul
    (0x6F5D5E, (0xD718, false)), // Korean hangul
    (0x274A30, (0x70DB, false)), // East Asian ideograph
    (0x292577, (0x8297, false)), // East Asian ideograph
    (0x4B4A62, (0x72A0, false)), // East Asian ideograph
    (0x273D67, (0x5F84, false)), // East Asian ideograph
    (0x6F4C52, (0xB268, false)), // Korean hangul
    (0x275B3D, (0x8F83, false)), // East Asian ideograph
    (0x217669, (0x5819, false)), // East Asian ideograph
    (0x223636, (0x6549, false)), // East Asian ideograph
    (0x2D5561, (0x76D6, false)), // East Asian ideograph
    (0x6F5D5F, (0xD719, false)), // Korean hangul
    (0x274A31, (0x707F, false)), // East Asian ideograph
    (0x293459, (0x8C2F, false)), // East Asian ideograph
    (0x284E30, (0x6E11, false)), // East Asian ideograph
    (0x29584B, (0x9CBD, false)), // East Asian ideograph
    (0x6F584A, (0xCA0B, false)), // Korean hangul
    (0x34715A, (0x7E1A, false)), // East Asian ideograph
    (0x216C21, (0x5293, false)), // East Asian ideograph
    (0x6F4C53, (0xB269, false)), // Korean hangul
    (0x275B3E, (0x8F7C, false)), // East Asian ideograph
    (0x226C26, (0x7B71, false)), // East Asian ideograph
    (0x226C27, (0x7B70, false)), // East Asian ideograph
    (0x216C29, (0x5298, false)), // East Asian ideograph
    (0x235C4F, (0x9DE9, false)), // East Asian ideograph
    (0x216C2B, (0x529A, false)), // East Asian ideograph
    (0x216C2C, (0x5299, false)), // East Asian ideograph
    (0x226C2D, (0x7B9C, false)), // East Asian ideograph
    (0x216C2E, (0x52A6, false)), // East Asian ideograph
    (0x22275D, (0x5E68, false)), // East Asian ideograph
    (0x212A2B, (0xE8D9, false)), // EACC component character
    (0x216C31, (0x52AD, false)), // East Asian ideograph
    (0x226C33, (0x7B92, false)), // East Asian ideograph
    (0x226C34, (0x7B91, false)), // East Asian ideograph
    (0x226C35, (0x7B90, false)), // East Asian ideograph
    (0x216C37, (0x52BB, false)), // East Asian ideograph
    (0x226C38, (0x7BA3, false)), // East Asian ideograph
    (0x226C3A, (0x7B8D, false)), // East Asian ideograph
    (0x28275F, (0x5E31, false)), // East Asian ideograph
    (0x216C3C, (0x52CA, false)), // East Asian ideograph
    (0x216C3D, (0x52CD, false)), // East Asian ideograph
    (0x2E6C3E, (0x7B59, false)), // East Asian ideograph
    (0x2D622C, (0x9F08, false)), // East Asian ideograph
    (0x216C40, (0x52D0, false)), // East Asian ideograph
    (0x226C41, (0x7B85, false)), // East Asian ideograph
    (0x706C42, (0x70BB, false)), // East Asian ideograph
    (0x226C43, (0x7B8E, false)), // East Asian ideograph
    (0x226C44, (0x7B98, false)), // East Asian ideograph
    (0x213239, (0x504C, false)), // East Asian ideograph
    (0x226C46, (0x7B86, false)), // East Asian ideograph
    (0x226C48, (0x7B99, false)), // East Asian ideograph
    (0x6F4F3A, (0xB860, false)), // Korean hangul
    (0x216C4C, (0x52E3, false)), // East Asian ideograph
    (0x216C4E, (0x52E1, false)), // East Asian ideograph
    (0x6F5D61, (0xD720, false)), // Korean hangul
    (0x216C50, (0x55E7, false)), // East Asian ideograph
    (0x226C52, (0x7BB2, false)), // East Asian ideograph
    (0x216C53, (0x52E9, false)), // East Asian ideograph
    (0x6F5623, (0xC648, false)), // Korean hangul
    (0x226C58, (0x7BCB, false)), // East Asian ideograph
    (0x226C59, (0x7BB8, false)), // East Asian ideograph
    (0x226C5A, (0x7BCF, false)), // East Asian ideograph
    (0x226C5C, (0x7BD0, false)), // East Asian ideograph
    (0x216C5E, (0x52F7, false)), // East Asian ideograph
    (0x292765, (0x82C8, false)), // East Asian ideograph
    (0x226C60, (0x7BBE, false)), // East Asian ideograph
    (0x216C61, (0x52F9, false)), // East Asian ideograph
    (0x216C62, (0x52FA, false)), // East Asian ideograph
    (0x216C64, (0x52FC, false)), // East Asian ideograph
    (0x216C69, (0x5307, false)), // East Asian ideograph
    (0x216C6A, (0x5303, false)), // East Asian ideograph
    (0x216C6B, (0x5306, false)), // East Asian ideograph (not in Unicode)
    (0x6F5D62, (0xD728, false)), // Korean hangul
    (0x216C6E, (0x530A, false)), // East Asian ideograph
    (0x226C6F, (0x7BCC, false)), // East Asian ideograph
    (0x216560, (0x4F80, false)), // East Asian ideograph
    (0x216C77, (0x5311, false)), // East Asian ideograph
    (0x213F6A, (0x6221, false)), // East Asian ideograph
    (0x6F5975, (0xCE87, false)), // Korean hangul
    (0x216C7B, (0x6706, false)), // East Asian ideograph
    (0x234767, (0x93F5, false)), // East Asian ideograph
    (0x21323B, (0x500F, false)), // East Asian ideograph
    (0x343E38, (0x7BDA, false)), // East Asian ideograph
    (0x4B6167, (0x95D8, false)), // East Asian ideograph
    (0x6F5D63, (0xD729, false)), // Korean hangul
    (0x6F5532, (0xC571, false)), // Korean hangul
    (0x216561, (0x4F74, false)), // East Asian ideograph
    (0x4B5A31, (0x8CCE, false)), // East Asian ideograph
    (0x6F4C57, (0xB290, false)), // Korean hangul
    (0x275B42, (0x8F84, false)), // East Asian ideograph
    (0x333E7D, (0x7652, false)), // East Asian ideograph
    (0x4B4925, (0x6FB3, false)), // East Asian ideograph (variant of 214925 which maps to 6FB3)
    (0x226771, (0x79A8, false)), // East Asian ideograph
    (0x225A7E, (0x7488, false)), // East Asian ideograph
    (0x6F5921, (0xCC29, false)), // Korean hangul
    (0x692577, (0x309B, false)), // Katakana-hiragana voiced sound mark
    (0x224B26, (0x6E31, false)), // East Asian ideograph
    (0x6F5A3E, (0xCF55, false)), // Korean hangul
    (0x6F4C58, (0xB291, false)), // Korean hangul
    (0x275B43, (0x8F7B, false)), // East Asian ideograph
    (0x226775, (0x79B0, false)), // East Asian ideograph
    (0x233A3B, (0x8E67, false)), // East Asian ideograph
    (0x275221, (0x7EED, false)), // East Asian ideograph
    (0x6F5922, (0xCC2C, false)), // Korean hangul
    (0x215222, (0x7E93, false)), // East Asian ideograph
    (0x234B48, (0x96B9, false)), // East Asian ideograph
    (0x4D445B, (0x9306, false)), // East Asian ideograph (variant of 23445B which maps to 9306)
    (0x275223, (0x7EA4, false)), // East Asian ideograph
    (0x275224, (0x7F06, false)), // East Asian ideograph
    (0x21323E, (0x50A2, false)), // East Asian ideograph
    (0x6F4F3B, (0xB864, false)), // Korean hangul
    (0x2D334F, (0x5202, false)), // East Asian ideograph
    (0x21677B, (0x50CA, false)), // East Asian ideograph
    (0x4B4B2C, (0x731F, false)), // East Asian ideograph
    (0x213933, (0x5944, false)), // East Asian ideograph
    (0x27677C, (0x4F1B, false)), // East Asian ideograph
    (0x4C6022, (0x7596, false)), // East Asian ideograph
    (0x225227, (0x7139, false)), // East Asian ideograph
    (0x4B3C5E, (0x5E64, false)), // East Asian ideograph
    (0x234B49, (0x96BC, false)), // East Asian ideograph
    (0x215228, (0x7F3D, false)), // East Asian ideograph
    (0x6F4C5A, (0xB298, false)), // Korean hangul
    (0x275B45, (0x8F87, false)), // East Asian ideograph
    (0x215229, (0x7F44, false)), // East Asian ideograph
    (0x22363E, (0x6554, false)), // East Asian ideograph
    (0x23522B, (0x995F, false)), // East Asian ideograph
    (0x6F5924, (0xCC2F, false)), // Korean hangul
    (0x22522C, (0x713B, false)), // East Asian ideograph
    (0x516122, (0x9988, false)), // East Asian ideograph
    (0x6F522D, (0xBE90, false)), // Korean hangul
    (0x6F4C5B, (0xB299, false)), // Korean hangul
    (0x22522E, (0x711C, false)), // East Asian ideograph
    (0x213240, (0x5099, false)), // East Asian ideograph
    (0x23522F, (0x9997, false)), // East Asian ideograph
    (0x225B59, (0x74A0, false)), // East Asian ideograph
    (0x4B6168, (0x9599, false)), // East Asian ideograph
    (0x235230, (0x9998, false)), // East Asian ideograph
    (0x226D22, (0x7BDD, false)), // East Asian ideograph
    (0x216D23, (0x531A, false)), // East Asian ideograph
    (0x226D24, (0x7BE5, false)), // East Asian ideograph
    (0x216D25, (0x531F, false)), // East Asian ideograph
    (0x215231, (0x7F69, false)), // East Asian ideograph
    (0x226D29, (0x7BE8, false)), // East Asian ideograph
    (0x277267, (0x5452, false)), // East Asian ideograph
    (0x225232, (0x713D, false)), // East Asian ideograph
    (0x226D2E, (0x7BF9, false)), // East Asian ideograph
    (0x226D2F, (0x7BD4, false)), // East Asian ideograph
    (0x6F4C5C, (0xB2A0, false)), // Korean hangul
    (0x226D32, (0x7BDF, false)), // East Asian ideograph
    (0x275233, (0x7F5A, false)), // East Asian ideograph
    (0x226D35, (0x7BD8, false)), // East Asian ideograph
    (0x216D36, (0x5335, false)), // East Asian ideograph
    (0x226D37, (0x7BEA, false)), // Unrelated variant of EACC 3A6A7C which maps to 7BEA
    (0x213C2D, (0x5DBC, false)), // East Asian ideograph
    (0x275234, (0x9A82, false)), // East Asian ideograph
    (0x216D3A, (0x5338, false)), // East Asian ideograph
    (0x226D3B, (0x7C06, false)), // East Asian ideograph
    (0x226D3E, (0x7BF0, false)), // East Asian ideograph
    (0x275D6B, (0x952D, false)), // East Asian ideograph
    (0x696D40, (0x8EC5, false)), // East Asian ideograph
    (0x226D41, (0x7C0F, false)), // East Asian ideograph
    (0x216D42, (0x534D, false)), // East Asian ideograph
    (0x6F5926, (0xCC38, false)), // Korean hangul
    (0x706D45, (0x783C, false)), // East Asian ideograph
    (0x226D46, (0x7C0B, false)), // East Asian ideograph
    (0x222534, (0x5D74, false)), // East Asian ideograph
    (0x275237, (0x7F57, false)), // East Asian ideograph
    (0x216D4C, (0x5363, false)), // East Asian ideograph
    (0x2D6235, (0x9D76, false)), // East Asian ideograph
    (0x216D4E, (0x5365, false)), // East Asian ideograph (not in Unicode)
    (0x226D4F, (0x7BF4, false)), // East Asian ideograph
    (0x215238, (0x7F88, false)), // East Asian ideograph
    (0x216D53, (0x536C, false)), // East Asian ideograph
    (0x226D54, (0x7BF3, false)), // East Asian ideograph
    (0x216D57, (0x5372, false)), // East Asian ideograph
    (0x216D58, (0x537A, false)), // East Asian ideograph
    (0x4B492B, (0x6FEB, false)), // East Asian ideograph
    (0x226D5A, (0x7C09, false)), // East Asian ideograph
    (0x226D5B, (0x7C03, false)), // East Asian ideograph
    (0x226D5C, (0x7BFC, false)), // East Asian ideograph
    (0x216D5D, (0x5380, false)), // East Asian ideograph
    (0x226D5F, (0x7C1C, false)), // East Asian ideograph
    (0x226D61, (0x7C26, false)), // East Asian ideograph
    (0x226D62, (0x7C28, false)), // East Asian ideograph
    (0x22523B, (0x7129, false)), // East Asian ideograph
    (0x216D64, (0x538E, false)), // East Asian ideograph
    (0x233D3F, (0x9004, false)), // East Asian ideograph
    (0x226D66, (0x7C1F, false)), // East Asian ideograph
    (0x216D67, (0x5394, false)), // East Asian ideograph
    (0x226D68, (0x7C2F, false)), // East Asian ideograph
    (0x23523C, (0x99A1, false)), // East Asian ideograph
    (0x6F4C5E, (0xB2A5, false)), // Korean hangul
    (0x216D6D, (0x5399, false)), // East Asian ideograph
    (0x6F523D, (0xBF18, false)), // Korean hangul
    (0x285F48, (0x7617, false)), // East Asian ideograph
    (0x213243, (0x5096, false)), // East Asian ideograph
    (0x216D74, (0x8652, false)), // East Asian ideograph
    (0x226D75, (0x7C30, false)), // East Asian ideograph
    (0x6F4F3C, (0xB86C, false)), // Korean hangul
    (0x216D7A, (0x53A4, false)), // East Asian ideograph
    (0x216D7B, (0x53AB, false)), // East Asian ideograph
    (0x2D5941, (0x5629, false)), // East Asian ideograph
    (0x6F5928, (0xCC3B, false)), // Korean hangul
    (0x2D5240, (0x7FA1, false)), // East Asian ideograph
    (0x235241, (0x99A9, false)), // East Asian ideograph
    (0x6F4C5F, (0xB2A6, false)), // Korean hangul
    (0x215242, (0x7FA9, false)), // East Asian ideograph
    (0x283D30, (0x67A7, false)), // East Asian ideograph
    (0x235C5B, (0x9DF8, false)), // East Asian ideograph
    (0x225243, (0x712E, false)), // East Asian ideograph
    (0x6F5244, (0xBF51, false)), // Korean hangul
    (0x6F5929, (0xCC3C, false)), // Korean hangul
    (0x226969, (0x7A78, false)), // East Asian ideograph
    (0x6F523B, (0xBF08, false)), // Korean hangul
    (0x28337B, (0x62A0, false)), // East Asian ideograph
    (0x6F4C60, (0xB2AA, false)), // Korean hangul
    (0x4B5247, (0x7FAE, false)), // East Asian ideograph
    (0x334256, (0x6B5B, false)), // East Asian ideograph
    (0x22585D, (0x73A5, false)), // East Asian ideograph
    (0x235C5C, (0x9DFC, false)), // East Asian ideograph
    (0x225248, (0x7177, false)), // East Asian ideograph
    (0x233A43, (0x8E5D, false)), // East Asian ideograph
    (0x4B492E, (0x6E0B, false)), // East Asian ideograph
    (0x234E4C, (0x97CD, false)), // East Asian ideograph
    (0x2D7345, (0x56D3, false)), // East Asian ideograph
    (0x215249, (0x7FBF, false)), // East Asian ideograph
    (0x284E3E, (0x6CF6, false)), // East Asian ideograph
    (0x212A3A, (0xE8E7, false)), // EACC component character
    (0x2D524A, (0x7FC4, false)), // East Asian ideograph
    (0x39483B, (0x9061, false)), // East Asian ideograph
    (0x276029, (0x9791, false)), // East Asian ideograph
    (0x6F524B, (0xBFD0, false)), // Korean hangul
    (0x2E337B, (0x630E, false)), // East Asian ideograph
    (0x6F4C61, (0xB2AC, false)), // Korean hangul
    (0x6F524C, (0xBFD4, false)), // Korean hangul
    (0x27524D, (0x4E60, false)), // East Asian ideograph
    (0x233A44, (0x8E75, false)), // East Asian ideograph
    (0x23524E, (0x99BC, false)), // East Asian ideograph
    (0x293468, (0x8C35, false)), // East Asian ideograph
    (0x6F545A, (0xC410, false)), // Korean hangul
    (0x23524F, (0x99C3, false)), // East Asian ideograph
    (0x6F5250, (0xC058, false)), // Korean hangul
    (0x6F4C62, (0xB2C8, false)), // Korean hangul
    (0x275B4D, (0x8F91, false)), // East Asian ideograph
    (0x275251, (0x7FC6, false)), // East Asian ideograph
    (0x213247, (0x50B5, false)), // East Asian ideograph
    (0x4B4D73, (0x66B8, false)), // East Asian ideograph
    (0x225252, (0x7152, false)), // East Asian ideograph
    (0x235253, (0x99B9, false)), // East Asian ideograph
    (0x6F592C, (0xCC3F, false)), // Korean hangul
    (0x215254, (0x7FE9, false)), // East Asian ideograph
    (0x274D3A, (0x76CF, false)), // East Asian ideograph
    (0x225255, (0x715D, false)), // East Asian ideograph
    (0x6F4C63, (0xB2C9, false)), // Korean hangul
    (0x225256, (0x7141, false)), // East Asian ideograph
    (0x227636, (0x800F, false)), // East Asian ideograph
    (0x4B4931, (0x6E16, false)), // East Asian ideograph
    (0x4D3359, (0x56AF, false)), // East Asian ideograph
    (0x275258, (0x7FD8, false)), // East Asian ideograph
    (0x21656E, (0x4F94, false)), // East Asian ideograph
    (0x284E41, (0x6F4B, false)), // East Asian ideograph
    (0x225259, (0x7175, false)), // East Asian ideograph
    (0x692546, (0x30C6, false)), // Katakana letter TE
    (0x22525A, (0x7173, false)), // East Asian ideograph
    (0x6F4C64, (0xB2CC, false)), // Korean hangul
    (0x275B4F, (0x8F96, false)), // East Asian ideograph
    (0x33525B, (0x71FF, false)), // East Asian ideograph
    (0x226E27, (0x7C35, false)), // East Asian ideograph
    (0x23347B, (0x8B7E, false)), // East Asian ideograph
    (0x21525C, (0x8001, false)), // East Asian ideograph
    (0x226E2A, (0x7C40, false)), // East Asian ideograph
    (0x2D5573, (0x83D4, false)), // East Asian ideograph
    (0x216E2C, (0x53B5, false)), // East Asian ideograph
    (0x216E2E, (0x53B9, false)), // East Asian ideograph
    (0x22525D, (0x715A, false)), // East Asian ideograph
    (0x226E30, (0x7C39, false)), // East Asian ideograph
    (0x6F592E, (0xCC45, false)), // Korean hangul
    (0x226E34, (0x7C3B, false)), // East Asian ideograph
    (0x226E35, (0x7C34, false)), // East Asian ideograph
    (0x6F5426, (0xC2E3, false)), // Korean hangul
    (0x226E3B, (0x7C42, false)), // East Asian ideograph
    (0x216E3E, (0x53D0, false)), // East Asian ideograph
    (0x70727D, (0x87A8, false)), // East Asian ideograph
    (0x275B50, (0x8F97, false)), // East Asian ideograph
    (0x225260, (0x714B, false)), // East Asian ideograph
    (0x4C6E42, (0x7C31, false)), // East Asian ideograph
    (0x21324A, (0x50BE, false)), // East Asian ideograph
    (0x225862, (0x73A2, false)), // East Asian ideograph
    (0x226E46, (0x7C4E, false)), // East Asian ideograph
    (0x235261, (0x99D3, false)), // East Asian ideograph
    (0x216E48, (0x53DA, false)), // East Asian ideograph
    (0x4D5574, (0x9B2E, false)), // East Asian ideograph
    (0x275E47, (0x94A5, false)), // East Asian ideograph
    (0x225262, (0x7147, false)), // East Asian ideograph
    (0x6F592F, (0xCC48, false)), // Korean hangul
    (0x235263, (0x99D4, false)), // East Asian ideograph
    (0x226E54, (0x7C5D, false)), // East Asian ideograph
    (0x226E56, (0x7C5C, false)), // East Asian ideograph
    (0x226E57, (0x7C5A, false)), // East Asian ideograph
    (0x226E58, (0x7C5B, false)), // East Asian ideograph
    (0x226E59, (0x7C59, false)), // East Asian ideograph
    (0x226E5B, (0x7C5E, false)), // East Asian ideograph
    (0x226E5C, (0x7C67, false)), // East Asian ideograph
    (0x6F4C66, (0xB2D8, false)), // Korean hangul
    (0x226E5E, (0x7C63, false)), // East Asian ideograph
    (0x235265, (0x99C9, false)), // East Asian ideograph
    (0x226E61, (0x7C68, false)), // East Asian ideograph
    (0x226E62, (0x7C65, false)), // East Asian ideograph
    (0x2D3132, (0x4ECF, false)), // East Asian ideograph
    (0x225266, (0x7171, false)), // East Asian ideograph
    (0x216E68, (0x5406, false)), // East Asian ideograph
    (0x286E69, (0x7C16, false)), // East Asian ideograph
    (0x225267, (0x715F, false)), // East Asian ideograph
    (0x216E6C, (0x544C, false)), // East Asian ideograph
    (0x216E6D, (0x5445, false)), // East Asian ideograph
    (0x226E6F, (0x7C6F, false)), // East Asian ideograph
    (0x216E70, (0x5432, false)), // East Asian ideograph
    (0x226970, (0x7A85, false)), // East Asian ideograph
    (0x226E75, (0x7C75, false)), // East Asian ideograph
    (0x216E76, (0x5421, false)), // East Asian ideograph
    (0x215269, (0x8033, false)), // East Asian ideograph
    (0x216E78, (0x5430, false)), // East Asian ideograph
    (0x226E79, (0x7C7E, false)), // East Asian ideograph
    (0x226E7A, (0x7C78, false)), // East Asian ideograph
    (0x6F4C67, (0xB2D9, false)), // Korean hangul
    (0x275B52, (0x6BC2, false)), // East Asian ideograph
    (0x226E7D, (0x7C7D, false)), // East Asian ideograph
    (0x27517E, (0x7F20, false)), // East Asian ideograph
    (0x692560, (0x30E0, false)), // Katakana letter MU
    (0x215022, (0x7B4F, false)), // East Asian ideograph
    (0x6F5323, (0xC11E, false)), // Korean hangul
    (0x225421, (0x71DD, false)), // East Asian ideograph
    (0x6F486C, (0xAC16, false)), // Korean hangul
    (0x2D526C, (0x8EAD, false)), // East Asian ideograph
    (0x274A46, (0x5899, false)), // East Asian ideograph
    (0x6F5931, (0xCC54, false)), // Korean hangul
    (0x225F5F, (0x7630, false)), // East Asian ideograph
    (0x6F5B2D, (0xD145, false)), // Korean hangul
    (0x22253F, (0x5D75, false)), // East Asian ideograph
    (0x234B57, (0x96D2, false)), // East Asian ideograph
    (0x69654F, (0x7E05, false)), // East Asian ideograph
    (0x4B526E, (0x8046, false)), // East Asian ideograph (variant of 21526E which maps to 8046)
    (0x6F4C68, (0xB2DB, false)), // Korean hangul
    (0x27526F, (0x5723, false)), // East Asian ideograph
    (0x22763B, (0x801F, false)), // East Asian ideograph
    (0x225422, (0x71C0, false)), // East Asian ideograph
    (0x335821, (0x97C8, false)), // East Asian ideograph
    (0x275271, (0x95FB, false)), // East Asian ideograph
    (0x6F5932, (0xCC55, false)), // Korean hangul
    (0x6F5272, (0xC0D0, false)), // Korean hangul
    (0x2D446B, (0x6936, false)), // East Asian ideograph
    (0x2D6241, (0x9D5E, false)), // East Asian ideograph
    (0x21346A, (0x536F, false)), // East Asian ideograph
    (0x275B54, (0x8F99, false)), // East Asian ideograph
    (0x235274, (0x99EC, false)), // East Asian ideograph
    (0x2E625F, (0x77C1, false)), // East Asian ideograph
    (0x275275, (0x8038, false)), // East Asian ideograph
    (0x4B4937, (0x56A0, false)), // East Asian ideograph
    (0x225276, (0x7172, false)), // East Asian ideograph
    (0x223D21, (0x6872, false)), // East Asian ideograph
    (0x6F5933, (0xCC58, false)), // Korean hangul
    (0x29486F, (0x9569, false)), // East Asian ideograph
    (0x275277, (0x8054, false)), // East Asian ideograph
    (0x223D22, (0x689C, false)), // East Asian ideograph
    (0x275278, (0x804C, false)), // East Asian ideograph
    (0x6F5979, (0xCE94, false)), // Korean hangul
    (0x2F3C2D, (0x8F3C, false)), // East Asian ideograph
    (0x6F4C6A, (0xB2E2, false)), // Korean hangul
    (0x275B55, (0x8F6C, false)), // East Asian ideograph
    (0x215279, (0x8076, false)), // East Asian ideograph
    (0x2E7D24, (0x83F0, false)), // East Asian ideograph
    (0x225867, (0x73B6, false)), // East Asian ideograph
    (0x235D66, (0x9E9E, false)), // East Asian ideograph
    (0x21527A, (0x807E, false)), // East Asian ideograph
    (0x213D25, (0x5ED3, false)), // East Asian ideograph
    (0x275E48, (0x92AE, false)), // East Asian ideograph
    (0x235823, (0x9BD5, false)), // East Asian ideograph
    (0x21527B, (0x807D, false)), // East Asian ideograph
    (0x217D26, (0x5AFF, false)), // East Asian ideograph
    (0x287061, (0x7EC0, false)), // East Asian ideograph
    (0x23527C, (0x99EA, false)), // East Asian ideograph
    (0x213D27, (0x5EE2, false)), // East Asian ideograph
    (0x284668, (0x6C29, false)), // East Asian ideograph
    (0x6F527D, (0xC0F7, false)), // Korean hangul
    (0x333D28, (0x53A8, false)), // East Asian ideograph
    (0x275B56, (0x8F9A, false)), // East Asian ideograph
    (0x6F527E, (0xC0F9, false)), // Korean hangul
    (0x2D3D29, (0x53AE, false)), // East Asian ideograph
    (0x33417E, (0x629E, false)), // East Asian ideograph
    (0x213D2A, (0x5EE3, false)), // East Asian ideograph (variant of 4B3D2A which maps to 5EE3)
    (0x287279, (0x7F25, false)), // East Asian ideograph
    (0x294568, (0x9518, false)), // East Asian ideograph
    (0x213D2B, (0x5EDF, false)), // East Asian ideograph
    (0x287062, (0x7EC1, false)), // East Asian ideograph
    (0x6F545C, (0xC430, false)), // Korean hangul
    (0x226975, (0x7A86, false)), // East Asian ideograph
    (0x22475C, (0x6CA0, false)), // East Asian ideograph
    (0x216133, (0x99D0, false)), // East Asian ideograph
    (0x226532, (0x78B6, false)), // East Asian ideograph
    (0x213D2D, (0x9F90, false)), // East Asian ideograph
    (0x275B57, (0x8F7F, false)), // East Asian ideograph
    (0x223D2E, (0x68A9, false)), // East Asian ideograph
    (0x213D2F, (0x5EF3, false)), // East Asian ideograph
    (0x212A30, (0xE8DE, false)), // EACC component character
    (0x6F5D79, (0xD789, false)), // Korean hangul
    (0x226F21, (0x7C81, false)), // East Asian ideograph
    (0x216577, (0x4F90, false)), // East Asian ideograph
    (0x216F24, (0x542A, false)), // East Asian ideograph
    (0x33314C, (0x5FA0, false)), // East Asian ideograph
    (0x216F26, (0x5422, false)), // East Asian ideograph
    (0x274D3C, (0x5C3D, false)), // East Asian ideograph
    (0x226F28, (0x7C8E, false)), // East Asian ideograph
    (0x226F29, (0x7C91, false)), // East Asian ideograph
    (0x226F2A, (0x7C83, false)), // East Asian ideograph
    (0x226F2C, (0x7C8D, false)), // East Asian ideograph
    (0x213D32, (0x5EF6, false)), // East Asian ideograph
    (0x216F2E, (0x545F, false)), // East Asian ideograph
    (0x216F2F, (0x549C, false)), // East Asian ideograph
    (0x27393F, (0x5941, false)), // East Asian ideograph
    (0x223D33, (0x68A0, false)), // East Asian ideograph
    (0x213252, (0x50D6, false)), // East Asian ideograph
    (0x216F35, (0x5488, false)), // East Asian ideograph
    (0x216F37, (0x547F, false)), // East Asian ideograph
    (0x216F39, (0x5482, false)), // East Asian ideograph
    (0x226F3A, (0x7C99, false)), // East Asian ideograph
    (0x226F3B, (0x7C98, false)), // East Asian ideograph
    (0x6F5D7A, (0xD78C, false)), // Korean hangul
    (0x226F3E, (0x7C9C, false)), // East Asian ideograph
    (0x226F40, (0x7C95, false)), // East Asian ideograph
    (0x6F5937, (0xCC70, false)), // Korean hangul
    (0x226F42, (0x7CA7, false)), // East Asian ideograph
    (0x226F43, (0x7CA2, false)), // East Asian ideograph
    (0x226F45, (0x7C9E, false)), // East Asian ideograph
    (0x226F46, (0x7CA9, false)), // East Asian ideograph
    (0x6F5939, (0xCC98, false)), // Korean hangul
    (0x226F48, (0x7CA8, false)), // East Asian ideograph
    (0x226F49, (0x7CA1, false)), // East Asian ideograph
    (0x226F4A, (0x7CAC, false)), // East Asian ideograph
    (0x216F4B, (0x5474, false)), // East Asian ideograph
    (0x226F4C, (0x7CA6, false)), // East Asian ideograph
    (0x6F4C6E, (0xB2E5, false)), // Korean hangul
    (0x216F52, (0x5466, false)), // East Asian ideograph
    (0x216F53, (0x5464, false)), // East Asian ideograph
    (0x226F54, (0x7CB2, false)), // East Asian ideograph
    (0x216F55, (0x54A4, false)), // East Asian ideograph
    (0x213D39, (0x5F0F, false)), // East Asian ideograph
    (0x226F58, (0x7CBB, false)), // East Asian ideograph
    (0x226F59, (0x7CBF, false)), // East Asian ideograph
    (0x216F5A, (0x54AD, false)), // East Asian ideograph
    (0x216F5B, (0x54BA, false)), // East Asian ideograph
    (0x216F5C, (0x54CF, false)), // East Asian ideograph
    (0x696F5D, (0x9596, false)), // East Asian ideograph
    (0x226F5E, (0x7CBA, false)), // East Asian ideograph
    (0x226F5F, (0x7CBC, false)), // East Asian ideograph
    (0x216F60, (0x54A5, false)), // East Asian ideograph
    (0x216F63, (0x54A7, false)), // East Asian ideograph
    (0x226F64, (0x7CC2, false)), // East Asian ideograph
    (0x216F66, (0x54A2, false)), // East Asian ideograph
    (0x216F67, (0x5472, false)), // East Asian ideograph
    (0x216F68, (0x5470, false)), // East Asian ideograph
    (0x216F69, (0x54BC, false)), // East Asian ideograph
    (0x216F6A, (0x54B7, false)), // East Asian ideograph
    (0x216F6B, (0x54DE, false)), // East Asian ideograph
    (0x216F6C, (0x54D6, false)), // East Asian ideograph
    (0x226F6D, (0x7CCC, false)), // East Asian ideograph
    (0x226F6F, (0x7CC9, false)), // East Asian ideograph
    (0x226F71, (0x7CD2, false)), // East Asian ideograph
    (0x6F4A22, (0xADC0, false)), // Korean hangul
    (0x29442D, (0x94A1, false)), // East Asian ideograph
    (0x216F74, (0x54C6, false)), // East Asian ideograph
    (0x225429, (0x71CB, false)), // East Asian ideograph
    (0x226F77, (0x7CE1, false)), // East Asian ideograph
    (0x6F5D7C, (0xD798, false)), // Korean hangul
    (0x33467A, (0x6CA1, false)), // East Asian ideograph
    (0x223D3F, (0x6877, false)), // East Asian ideograph
    (0x216F7C, (0x54E2, false)), // East Asian ideograph
    (0x216F7D, (0x5507, false)), // East Asian ideograph
    (0x233D40, (0x9008, false)), // East Asian ideograph
    (0x233D59, (0x9036, false)), // East Asian ideograph
    (0x277D74, (0x5A08, false)), // East Asian ideograph
    (0x333D42, (0x7D43, false)), // East Asian ideograph
    (0x213A63, (0x5B7F, false)), // East Asian ideograph
    (0x213D43, (0x5F27, false)), // East Asian ideograph
    (0x2D3366, (0x5234, false)), // East Asian ideograph
    (0x6F5D7D, (0xD799, false)), // Korean hangul
    (0x223D44, (0x688E, false)), // East Asian ideograph
    (0x6F593A, (0xCC99, false)), // Korean hangul
    (0x233D45, (0x900B, false)), // East Asian ideograph
    (0x277030, (0x5457, false)), // East Asian ideograph
    (0x697023, (0x9666, false)), // East Asian ideograph
    (0x6F4A64, (0xAEDC, false)), // Korean hangul
    (0x213D47, (0x5F35, false)), // East Asian ideograph
    (0x235C6D, (0x9DEE, false)), // East Asian ideograph
    (0x233D48, (0x900C, false)), // East Asian ideograph
    (0x6F5D7E, (0xD79B, false)), // Korean hangul
    (0x213D49, (0x5F3C, false)), // East Asian ideograph
    (0x6F593B, (0xCC9C, false)), // Korean hangul
    (0x394944, (0x6B12, false)), // East Asian ideograph
    (0x6F5B2F, (0xD14D, false)), // Korean hangul
    (0x224762, (0x6CEB, false)), // East Asian ideograph
    (0x4B3B52, (0x8132, false)), // East Asian ideograph
    (0x2D4474, (0x690D, false)), // East Asian ideograph (variant of 214474 which maps to 690D)
    (0x273D4B, (0x5F39, false)), // East Asian ideograph
    (0x2D4031, (0x64A6, false)), // East Asian ideograph
    (0x6F5A32, (0xCF10, false)), // Korean hangul
    (0x213D4C, (0x5F4C, false)), // East Asian ideograph
    (0x213D4D, (0x5F4E, false)), // East Asian ideograph
    (0x4B4940, (0x6F45, false)), // East Asian ideograph
    (0x23582B, (0x9BF1, false)), // East Asian ideograph
    (0x6F5D25, (0xD5E5, false)), // Korean hangul
    (0x213D4E, (0x5F57, false)), // East Asian ideograph
    (0x6F5030, (0xBA65, false)), // Korean hangul
    (0x224763, (0x6CEE, false)), // East Asian ideograph
    (0x226539, (0x78B7, false)), // East Asian ideograph
    (0x213D50, (0x5F5D, false)), // East Asian ideograph
    (0x6F4C73, (0xB2ED, false)), // Korean hangul
    (0x223D51, (0x6917, false)), // East Asian ideograph
    (0x225870, (0x73C8, false)), // East Asian ideograph
    (0x217247, (0x5642, false)), // East Asian ideograph
    (0x4B6247, (0x9D2C, false)), // East Asian ideograph
    (0x217D52, (0x5B2C, false)), // East Asian ideograph
    (0x23582C, (0x9BE1, false)), // East Asian ideograph
    (0x213D53, (0x5F65, false)), // East Asian ideograph
    (0x28706A, (0x7ED0, false)), // East Asian ideograph
    (0x294871, (0x954A, false)), // East Asian ideograph
    (0x6F5D78, (0xD788, false)), // Korean hangul
    (0x224764, (0x6CC0, false)), // East Asian ideograph
    (0x6F597B, (0xCEA0, false)), // Korean hangul
    (0x275B5F, (0x529E, false)), // East Asian ideograph
    (0x285F5E, (0x7618, false)), // East Asian ideograph
    (0x223D56, (0x690B, false)), // East Asian ideograph
    (0x213259, (0x5100, false)), // East Asian ideograph
    (0x233D57, (0x9034, false)), // East Asian ideograph
    (0x455122, (0x7D0D, false)), // East Asian ideograph
    (0x23582D, (0x9BDB, false)), // East Asian ideograph
    (0x233D58, (0x902F, false)), // East Asian ideograph
    (0x6F593E, (0xCCA9, false)), // Korean hangul
    (0x223D59, (0x6904, false)), // East Asian ideograph
    (0x6F5B45, (0xD230, false)), // Korean hangul
    (0x234B64, (0x96DF, false)), // East Asian ideograph
    (0x6F4C75, (0xB2F3, false)), // Korean hangul
    (0x275B60, (0x8F9E, false)), // East Asian ideograph
    (0x227022, (0x7CDD, false)), // East Asian ideograph
    (0x217023, (0x5517, false)), // East Asian ideograph
    (0x217024, (0x54FD, false)), // East Asian ideograph
    (0x217025, (0x54E7, false)), // East Asian ideograph
    (0x217027, (0x54F3, false)), // East Asian ideograph
    (0x227028, (0x7CED, false)), // East Asian ideograph
    (0x213D5C, (0x5F80, false)), // East Asian ideograph
    (0x21702A, (0x54E4, false)), // East Asian ideograph
    (0x21702B, (0x550A, false)), // East Asian ideograph
    (0x21702D, (0x54FF, false)), // East Asian ideograph
    (0x21702E, (0x5518, false)), // East Asian ideograph
    (0x223D5D, (0x6929, false)), // East Asian ideograph
    (0x227030, (0x7CF2, false)), // East Asian ideograph
    (0x6F593F, (0xCCAB, false)), // Korean hangul
    (0x217032, (0x54EF, false)), // East Asian ideograph
    (0x217034, (0x5508, false)), // East Asian ideograph
    (0x227035, (0x7CF4, false)), // East Asian ideograph
    (0x217038, (0x54F6, false)), // East Asian ideograph
    (0x227039, (0x7CF6, false)), // East Asian ideograph
    (0x6F4C76, (0xB2F4, false)), // Korean hangul
    (0x21703E, (0x550E, false)), // East Asian ideograph
    (0x275B61, (0x8FA9, false)), // East Asian ideograph
    (0x4B5C50, (0x9045, false)), // East Asian ideograph
    (0x692563, (0x30E3, false)), // Katakana letter small YA
    (0x21325B, (0x50FB, false)), // East Asian ideograph
    (0x217044, (0x5523, false)), // East Asian ideograph
    (0x227045, (0x7D08, false)), // East Asian ideograph
    (0x217046, (0x550F, false)), // East Asian ideograph
    (0x217047, (0x5511, false)), // East Asian ideograph
    (0x23582F, (0x9BE2, false)), // East Asian ideograph
    (0x22704A, (0x7D13, false)), // East Asian ideograph
    (0x21704B, (0x5575, false)), // East Asian ideograph
    (0x21704D, (0x5573, false)), // East Asian ideograph
    (0x21704E, (0x554C, false)), // East Asian ideograph
    (0x21704F, (0x5576, false)), // East Asian ideograph
    (0x217050, (0x554D, false)), // East Asian ideograph
    (0x217051, (0x555A, false)), // East Asian ideograph
    (0x227052, (0x7D1D, false)), // East Asian ideograph
    (0x217053, (0x553C, false)), // East Asian ideograph
    (0x217055, (0x5550, false)), // East Asian ideograph
    (0x217057, (0x5539, false)), // East Asian ideograph
    (0x217058, (0x5548, false)), // East Asian ideograph
    (0x217059, (0x552D, false)), // East Asian ideograph
    (0x21705A, (0x5551, false)), // East Asian ideograph
    (0x6F4C77, (0xB2F5, false)), // Korean hangul
    (0x21705D, (0x552A, false)), // East Asian ideograph
    (0x213D65, (0x5F8C, false)), // East Asian ideograph
    (0x217060, (0x5562, false)), // East Asian ideograph
    (0x217061, (0x5536, false)), // East Asian ideograph
    (0x227062, (0x7D32, false)), // East Asian ideograph
    (0x217064, (0x5549, false)), // East Asian ideograph
    (0x227065, (0x7D31, false)), // East Asian ideograph
    (0x335830, (0x658D, false)), // East Asian ideograph
    (0x227068, (0x7D45, false)), // East Asian ideograph
    (0x21706A, (0x5540, false)), // East Asian ideograph
    (0x21706B, (0x5535, false)), // East Asian ideograph
    (0x22706C, (0x7D29, false)), // East Asian ideograph
    (0x6F5941, (0xCCB4, false)), // Korean hangul
    (0x22706F, (0x7D41, false)), // East Asian ideograph
    (0x217070, (0x5545, false)), // East Asian ideograph
    (0x227071, (0x7D3E, false)), // East Asian ideograph
    (0x234B67, (0x96DD, false)), // East Asian ideograph
    (0x213D69, (0x5F98, false)), // East Asian ideograph
    (0x217079, (0x553F, false)), // East Asian ideograph
    (0x22707A, (0x7D5C, false)), // East Asian ideograph
    (0x21707B, (0x5541, false)), // East Asian ideograph
    (0x22707C, (0x7D53, false)), // East Asian ideograph
    (0x21707D, (0x5565, false)), // East Asian ideograph
    (0x22707E, (0x7D5A, false)), // East Asian ideograph
    (0x275779, (0x891B, false)), // East Asian ideograph
    (0x225432, (0x71EB, false)), // East Asian ideograph
    (0x235831, (0x9BF0, false)), // East Asian ideograph
    (0x213D6C, (0x5F9E, false)), // East Asian ideograph
    (0x6F5942, (0xCCB5, false)), // Korean hangul
    (0x213F27, (0x614D, false)), // East Asian ideograph
    (0x213B3F, (0x5C08, false)), // East Asian ideograph
    (0x283D6E, (0x67A8, false)), // East Asian ideograph
    (0x2D6251, (0x5869, false)), // East Asian ideograph
    (0x6F4C79, (0xB2F9, false)), // Korean hangul
    (0x275B64, (0x519C, false)), // East Asian ideograph
    (0x273D6F, (0x590D, false)), // East Asian ideograph
    (0x21325E, (0x5102, false)), // East Asian ideograph
    (0x6F4A24, (0xADC8, false)), // Korean hangul
    (0x4B4947, (0x7AC3, false)), // East Asian ideograph
    (0x6F5943, (0xCCB8, false)), // Korean hangul
    (0x213F28, (0x614B, false)), // East Asian ideograph
    (0x213D73, (0x5FAE, false)), // East Asian ideograph
    (0x2D6252, (0x78B1, false)), // East Asian ideograph
    (0x295A59, (0x9E38, false)), // East Asian ideograph
    (0x277328, (0x54DC, false)), // East Asian ideograph
    (0x213D74, (0x5FB9, false)), // East Asian ideograph
    (0x21325F, (0x510D, false)), // East Asian ideograph
    (0x285B21, (0x740F, false)), // East Asian ideograph
    (0x233A5D, (0x8E94, false)), // East Asian ideograph
    (0x213D75, (0x5FB7, false)), // East Asian ideograph
    (0x275D71, (0x94A2, false)), // East Asian ideograph
    (0x217D76, (0x5B4B, false)), // East Asian ideograph
    (0x6F5023, (0xBA3C, false)), // Korean hangul
    (0x226822, (0x79B8, false)), // East Asian ideograph
    (0x223D78, (0x68FD, false)), // East Asian ideograph
    (0x226823, (0x79BA, false)), // East Asian ideograph
    (0x213D79, (0x5FC5, false)), // East Asian ideograph
    (0x4B7E6A, (0x5BC3, false)), // East Asian ideograph (variant of 217E6A which maps to 5BC3)
    (0x223F3E, (0x696F, false)), // East Asian ideograph
    (0x212A33, (0xE8E0, false)), // EACC component character
    (0x223D7B, (0x68F3, false)), // East Asian ideograph
    (0x6F5945, (0xCCC7, false)), // Korean hangul
    (0x6F5B31, (0xD154, false)), // Korean hangul
    (0x213D7C, (0x5FCC, false)), // East Asian ideograph
    (0x27493C, (0x6FD2, false)), // East Asian ideograph
    (0x213261, (0x5109, false)), // East Asian ideograph
    (0x233A5F, (0x8E92, false)), // East Asian ideograph
    (0x29282A, (0x8539, false)), // East Asian ideograph
    (0x29494D, (0x9609, false)), // East Asian ideograph
    (0x6F5342, (0xC190, false)), // Korean hangul
    (0x696D3F, (0x8EBE, false)), // East Asian ideograph
    (0x213F2B, (0x6134, false)), // East Asian ideograph
    (0x334729, (0x6E2B, false)), // East Asian ideograph
    (0x6F4C7D, (0xB300, false)), // Korean hangul
    (0x2F2A64, (0x87B5, false)), // East Asian ideograph
    (0x22682E, (0x79D5, false)), // East Asian ideograph
    (0x287431, (0x575B, false)), // East Asian ideograph (duplicate simplified)
    (0x275E60, (0x960E, false)), // East Asian ideograph
    (0x233A60, (0x8E93, false)), // East Asian ideograph
    (0x22282F, (0x5EA4, false)), // East Asian ideograph
    (0x227122, (0x7D70, false)), // East Asian ideograph
    (0x217123, (0x5591, false)), // East Asian ideograph
    (0x697124, (0x98AA, false)), // East Asian ideograph
    (0x217125, (0x5577, false)), // East Asian ideograph
    (0x217126, (0x55A8, false)), // East Asian ideograph
    (0x217127, (0x55AD, false)), // East Asian ideograph
    (0x227129, (0x7D67, false)), // East Asian ideograph
    (0x21712A, (0x5605, false)), // East Asian ideograph
    (0x22712B, (0x7D6A, false)), // East Asian ideograph
    (0x22712C, (0x7D6B, false)), // East Asian ideograph
    (0x216832, (0x50D4, false)), // East Asian ideograph
    (0x21712F, (0x5586, false)), // East Asian ideograph
    (0x227130, (0x7D73, false)), // East Asian ideograph
    (0x227134, (0x7D4E, false)), // East Asian ideograph
    (0x217136, (0x55B4, false)), // East Asian ideograph
    (0x227137, (0x7D8B, false)), // East Asian ideograph
    (0x227139, (0x7D88, false)), // East Asian ideograph
    (0x22713B, (0x7D85, false)), // East Asian ideograph
    (0x2D514A, (0x6DD6, false)), // East Asian ideograph
    (0x22713D, (0x7D8E, false)), // East Asian ideograph
    (0x216835, (0x50E6, false)), // East Asian ideograph
    (0x6F5948, (0xCD08, false)), // Korean hangul
    (0x227142, (0x7D7F, false)), // East Asian ideograph
    (0x217143, (0x55E2, false)), // East Asian ideograph (variant of 2D7143 which maps to 55E2)
    (0x227144, (0x7D86, false)), // East Asian ideograph
    (0x217145, (0x558E, false)), // East Asian ideograph
    (0x217147, (0x55B5, false)), // East Asian ideograph
    (0x227148, (0x7D8D, false)), // East Asian ideograph
    (0x217149, (0x558F, false)), // East Asian ideograph
    (0x21714B, (0x5559, false)), // East Asian ideograph
    (0x22714D, (0x7D83, false)), // East Asian ideograph
    (0x22714F, (0x7D7D, false)), // East Asian ideograph
    (0x217150, (0x55A4, false)), // East Asian ideograph
    (0x217151, (0x5592, false)), // East Asian ideograph
    (0x217152, (0x5599, false)), // East Asian ideograph
    (0x227154, (0x7D7B, false)), // East Asian ideograph
    (0x233A62, (0x8E90, false)), // East Asian ideograph
    (0x217156, (0x55F4, false)), // East Asian ideograph
    (0x227158, (0x7D7A, false)), // East Asian ideograph
    (0x227159, (0x7D96, false)), // East Asian ideograph
    (0x22715A, (0x7D5B, false)), // East Asian ideograph
    (0x22715B, (0x7D8C, false)), // East Asian ideograph
    (0x21715C, (0x55DE, false)), // East Asian ideograph
    (0x21715D, (0x55D9, false)), // East Asian ideograph
    (0x21715E, (0x55C3, false)), // East Asian ideograph
    (0x21715F, (0x55C9, false)), // East Asian ideograph
    (0x217161, (0x55CA, false)), // East Asian ideograph
    (0x227162, (0x7DAE, false)), // East Asian ideograph
    (0x21683B, (0x50CE, false)), // East Asian ideograph
    (0x217164, (0x55D4, false)), // East Asian ideograph
    (0x217165, (0x55C4, false)), // East Asian ideograph
    (0x227167, (0x7DCB, false)), // East Asian ideograph
    (0x227169, (0x7DAA, false)), // East Asian ideograph
    (0x22716A, (0x7DCE, false)), // East Asian ideograph
    (0x22716B, (0x7DC9, false)), // East Asian ideograph
    (0x692565, (0x30E5, false)), // Katakana letter small YU
    (0x22716E, (0x7DC5, false)), // East Asian ideograph
    (0x22716F, (0x7DA6, false)), // East Asian ideograph
    (0x217170, (0x55D2, false)), // East Asian ideograph
    (0x223664, (0x6585, false)), // East Asian ideograph
    (0x277C36, (0x59AB, false)), // East Asian ideograph
    (0x22543A, (0x71F5, false)), // East Asian ideograph
    (0x227174, (0x7DC4, false)), // East Asian ideograph
    (0x217175, (0x55E5, false)), // East Asian ideograph
    (0x6F4871, (0xAC1C, false)), // Korean hangul
    (0x217177, (0x55D6, false)), // East Asian ideograph
    (0x227178, (0x7DAC, false)), // East Asian ideograph
    (0x217179, (0x55F2, false)), // East Asian ideograph
    (0x6F5C77, (0xD590, false)), // Korean hangul
    (0x2E717C, (0x7D63, false)), // East Asian ideograph
    (0x22717D, (0x7DB9, false)), // East Asian ideograph
    (0x21717E, (0x5627, false)), // East Asian ideograph
    (0x292840, (0x84E0, false)), // East Asian ideograph
    (0x235F5F, (0x9F37, false)), // East Asian ideograph
    (0x216841, (0x50F3, false)), // East Asian ideograph
    (0x216842, (0x50E8, false)), // East Asian ideograph
    (0x217255, (0x5635, false)), // East Asian ideograph
    (0x2D514D, (0x7D2C, false)), // East Asian ideograph
    (0x216844, (0x50F0, false)), // East Asian ideograph
    (0x6F594B, (0xCD10, false)), // Korean hangul
    (0x224772, (0x6CF5, false)), // East Asian ideograph
    (0x6F4E69, (0xB78D, false)), // Korean hangul
    (0x23307D, (0x89AF, false)), // East Asian ideograph
    (0x295B35, (0x9E2B, false)), // East Asian ideograph
    (0x4B7874, (0x590A, false)), // East Asian ideograph
    (0x23284C, (0x8645, false)), // East Asian ideograph
    (0x6F4A26, (0xADD1, false)), // Korean hangul
    (0x22543D, (0x71F3, false)), // East Asian ideograph
    (0x234E53, (0x97D8, false)), // East Asian ideograph
    (0x69684D, (0x8422, false)), // East Asian ideograph
    (0x333623, (0x9F69, false)), // East Asian ideograph
    (0x225B61, (0x74B1, false)), // East Asian ideograph
    (0x6F553B, (0xC58C, false)), // Korean hangul
    (0x706D3F, (0x781C, false)), // East Asian ideograph
    (0x4B587A, (0x8ACB, false)), // East Asian ideograph
    (0x6F7723, (0xE8CA, false)), // Korean hangul
    (0x213F32, (0x615D, false)), // East Asian ideograph
    (0x234730, (0x93E6, false)), // East Asian ideograph
    (0x222851, (0x5ECC, false)), // East Asian ideograph
    (0x6F594E, (0xCD1B, false)), // Korean hangul
    (0x284E62, (0x6F4D, false)), // East Asian ideograph
    (0x6F5732, (0xC7CC, false)), // Korean hangul
    (0x6F5328, (0xC126, false)), // Korean hangul
    (0x213F33, (0x6182, false)), // East Asian ideograph
    (0x285F6F, (0x7605, false)), // East Asian ideograph
    (0x6F4A3D, (0xAE43, false)), // Korean hangul
    (0x6F4E25, (0xB545, false)), // Korean hangul
    (0x295F2B, (0x9F0D, false)), // East Asian ideograph
    (0x212A35, (0xE8E2, false)), // EACC component character
    (0x6F594F, (0xCD1D, false)), // Korean hangul
    (0x6F5B33, (0xD15D, false)), // Korean hangul
    (0x274621, (0x6B22, false)), // East Asian ideograph
    (0x232859, (0x864D, false)), // East Asian ideograph
    (0x224333, (0x6ADF, false)), // East Asian ideograph
    (0x234732, (0x940B, false)), // East Asian ideograph
    (0x21797C, (0x5997, false)), // East Asian ideograph
    (0x23285A, (0x8653, false)), // East Asian ideograph
    (0x6F4E43, (0xB614, false)), // Korean hangul
    (0x227222, (0x7D9F, false)), // East Asian ideograph
    (0x217224, (0x55FB, false)), // East Asian ideograph
    (0x217225, (0x5612, false)), // East Asian ideograph
    (0x217227, (0x55F8, false)), // East Asian ideograph
    (0x217228, (0x560F, false)), // East Asian ideograph
    (0x227229, (0x7DE1, false)), // East Asian ideograph
    (0x22722A, (0x7DD9, false)), // East Asian ideograph
    (0x22722B, (0x7DE4, false)), // East Asian ideograph
    (0x6F4F44, (0xB8E9, false)), // Korean hangul
    (0x21722E, (0x561E, false)), // East Asian ideograph
    (0x217539, (0x5790, false)), // East Asian ideograph
    (0x227231, (0x7DD7, false)), // East Asian ideograph
    (0x295263, (0x9A75, false)), // East Asian ideograph
    (0x217234, (0x561C, false)), // East Asian ideograph
    (0x217235, (0x5610, false)), // East Asian ideograph
    (0x217236, (0x5601, false)), // East Asian ideograph
    (0x217238, (0x5613, false)), // East Asian ideograph
    (0x217239, (0x55F6, false)), // East Asian ideograph
    (0x22723A, (0x7E06, false)), // East Asian ideograph
    (0x21685F, (0x5105, false)), // East Asian ideograph
    (0x21723C, (0x5602, false)), // East Asian ideograph
    (0x22723E, (0x7DE6, false)), // East Asian ideograph
    (0x697240, (0x9BB4, false)), // East Asian ideograph
    (0x217242, (0x561D, false)), // East Asian ideograph
    (0x27577C, (0x88C6, false)), // East Asian ideograph
    (0x217244, (0x55FF, false)), // East Asian ideograph
    (0x697245, (0x9BCF, false)), // East Asian ideograph
    (0x227246, (0x7DDC, false)), // East Asian ideograph
    (0x216861, (0x50FC, false)), // East Asian ideograph
    (0x217248, (0x564C, false)), // East Asian ideograph
    (0x227249, (0x7DE5, false)), // East Asian ideograph
    (0x22724B, (0x7DF5, false)), // East Asian ideograph
    (0x6F4E6A, (0xB78F, false)), // Korean hangul
    (0x295021, (0x98A2, false)), // East Asian ideograph
    (0x2E7328, (0x5FAD, false)), // East Asian ideograph
    (0x227250, (0x7E17, false)), // East Asian ideograph
    (0x227251, (0x7E1E, false)), // East Asian ideograph
    (0x227252, (0x7E21, false)), // East Asian ideograph
    (0x227253, (0x7E0B, false)), // East Asian ideograph
    (0x224335, (0x6ADE, false)), // East Asian ideograph
    (0x227256, (0x7E22, false)), // East Asian ideograph
    (0x217257, (0x5649, false)), // East Asian ideograph
    (0x217258, (0x5641, false)), // East Asian ideograph
    (0x2D5124, (0x5E0B, false)), // East Asian ideograph
    (0x22725B, (0x7E20, false)), // East Asian ideograph
    (0x21725C, (0x5658, false)), // East Asian ideograph
    (0x22725D, (0x7E1D, false)), // East Asian ideograph
    (0x21725E, (0x5654, false)), // East Asian ideograph
    (0x216865, (0x5106, false)), // East Asian ideograph
    (0x217260, (0x562A, false)), // East Asian ideograph
    (0x217261, (0x563D, false)), // East Asian ideograph
    (0x217264, (0x562C, false)), // East Asian ideograph
    (0x227265, (0x7E15, false)), // East Asian ideograph
    (0x6F5474, (0xC52C, false)), // Korean hangul
    (0x217267, (0x5638, false)), // East Asian ideograph
    (0x4D5154, (0x9942, false)), // East Asian ideograph
    (0x217269, (0x564D, false)), // East Asian ideograph
    (0x22726A, (0x7E0F, false)), // East Asian ideograph
    (0x21726B, (0x562B, false)), // East Asian ideograph
    (0x21726C, (0x564F, false)), // East Asian ideograph
    (0x22726D, (0x7E3B, false)), // East Asian ideograph
    (0x21726E, (0x5670, false)), // East Asian ideograph
    (0x21726F, (0x565F, false)), // East Asian ideograph
    (0x217270, (0x567C, false)), // East Asian ideograph
    (0x227271, (0x7E34, false)), // East Asian ideograph
    (0x227272, (0x7E2D, false)), // East Asian ideograph
    (0x227273, (0x7E2F, false)), // East Asian ideograph
    (0x227275, (0x7E36, false)), // East Asian ideograph
    (0x227277, (0x7E3A, false)), // East Asian ideograph
    (0x217278, (0x5676, false)), // East Asian ideograph
    (0x227279, (0x7E39, false)), // East Asian ideograph
    (0x21727A, (0x5666, false)), // East Asian ideograph
    (0x21727B, (0x5673, false)), // East Asian ideograph
    (0x21727C, (0x566D, false)), // East Asian ideograph
    (0x22727D, (0x7E44, false)), // East Asian ideograph
    (0x21727E, (0x5672, false)), // East Asian ideograph
    (0x33537D, (0x9AD5, false)), // East Asian ideograph
    (0x4B3E7E, (0x60A9, false)), // East Asian ideograph
    (0x6F5953, (0xCD94, false)), // Korean hangul
    (0x6F7729, (0xAE0E, false)), // Korean hangul
    (0x224B30, (0x6E36, false)), // East Asian ideograph
    (0x2D6262, (0x9EC4, false)), // East Asian ideograph
    (0x2D4049, (0x67B4, false)), // East Asian ideograph
    (0x4B5C54, (0x8F9F, false)), // East Asian ideograph (duplicate simplified)
    (0x2E686F, (0x7A19, false)), // East Asian ideograph
    (0x6F4873, (0xAC20, false)), // Korean hangul
    (0x33362A, (0x9B28, false)), // East Asian ideograph
    (0x216871, (0x5115, false)), // East Asian ideograph
    (0x6F5954, (0xCD95, false)), // Korean hangul
    (0x6F5B34, (0xD15F, false)), // Korean hangul
    (0x234B7A, (0x96F5, false)), // East Asian ideograph
    (0x6F4F45, (0xB8EC, false)), // Korean hangul
    (0x6F5031, (0xBA67, false)), // Korean hangul
    (0x6F5955, (0xCD98, false)), // Korean hangul
    (0x215321, (0x8085, false)), // East Asian ideograph
    (0x6F772B, (0xAE11, false)), // Korean hangul
    (0x213F3A, (0x615F, false)), // East Asian ideograph
    (0x6F5322, (0xC11D, false)), // Korean hangul
    (0x225323, (0x7192, false)), // East Asian ideograph
    (0x2D5E21, (0x9418, false)), // East Asian ideograph
    (0x29415C, (0x917E, false)), // East Asian ideograph
    (0x215324, (0x808B, false)), // East Asian ideograph
    (0x6F5325, (0xC123, false)), // Korean hangul
    (0x224539, (0x6BAB, false)), // East Asian ideograph
    (0x6F5956, (0xCD9C, false)), // Korean hangul
    (0x695326, (0x54D8, false)), // East Asian ideograph
    (0x22477D, (0x6CC2, false)), // East Asian ideograph
    (0x23287C, (0x866F, false)), // East Asian ideograph
    (0x6F5327, (0xC125, false)), // Korean hangul
    (0x275E35, (0x9558, false)), // East Asian ideograph
    (0x2D404C, (0x6283, false)), // East Asian ideograph
    (0x6F4B6B, (0xB0E0, false)), // Korean hangul
    (0x275735, (0x8681, false)), // East Asian ideograph
    (0x6F4A28, (0xADDC, false)), // Korean hangul
    (0x235329, (0x99F8, false)), // East Asian ideograph
    (0x225447, (0x71E0, false)), // East Asian ideograph
    (0x23532A, (0x99F4, false)), // East Asian ideograph
    (0x6F5957, (0xCDA4, false)), // Korean hangul
    (0x21532B, (0x8096, false)), // East Asian ideograph
    (0x276842, (0x507E, false)), // East Asian ideograph
    (0x274629, (0x5C81, false)), // East Asian ideograph
    (0x213F3C, (0x617E, false)), // East Asian ideograph
    (0x21532C, (0x80B2, false)), // East Asian ideograph
    (0x6F5235, (0xBED8, false)), // Korean hangul
    (0x6F532D, (0xC12F, false)), // Korean hangul
    (0x213273, (0x5147, false)), // East Asian ideograph
    (0x6F532E, (0xC130, false)), // Korean hangul
    (0x275D75, (0x9525, false)), // East Asian ideograph
    (0x216F43, (0x546B, false)), // East Asian ideograph
    (0x6F5958, (0xCDA5, false)), // Korean hangul
    (0x215330, (0x80A2, false)), // East Asian ideograph
    (0x27462A, (0x5386, false)), // East Asian ideograph
    (0x217325, (0x5693, false)), // East Asian ideograph
    (0x227326, (0x7E3F, false)), // East Asian ideograph
    (0x215331, (0x80AB, false)), // East Asian ideograph
    (0x227328, (0x7E47, false)), // East Asian ideograph
    (0x225332, (0x7185, false)), // East Asian ideograph
    (0x22732F, (0x7E51, false)), // East Asian ideograph
    (0x217332, (0x56BA, false)), // East Asian ideograph
    (0x215333, (0x80AF, false)), // East Asian ideograph
    (0x227334, (0x7E67, false)), // East Asian ideograph
    (0x217335, (0x5684, false)), // East Asian ideograph
    (0x217336, (0x5691, false)), // East Asian ideograph
    (0x227337, (0x7E56, false)), // East Asian ideograph
    (0x6F5334, (0xC140, false)), // Korean hangul
    (0x21733E, (0x569E, false)), // East Asian ideograph
    (0x6F5335, (0xC148, false)), // Korean hangul
    (0x27462B, (0x5F52, false)), // East Asian ideograph
    (0x217342, (0x569A, false)), // East Asian ideograph
    (0x213F3E, (0x61B2, false)), // East Asian ideograph
    (0x225336, (0x717C, false)), // East Asian ideograph
    (0x227348, (0x7E68, false)), // East Asian ideograph
    (0x227349, (0x7E6E, false)), // East Asian ideograph
    (0x21734B, (0x56AD, false)), // East Asian ideograph
    (0x21734C, (0x56A6, false)), // East Asian ideograph
    (0x22734E, (0x7E70, false)), // East Asian ideograph
    (0x6F4E66, (0xB780, false)), // Korean hangul
    (0x227351, (0x7E6F, false)), // East Asian ideograph
    (0x227352, (0x7E73, false)), // East Asian ideograph
    (0x217353, (0x56B2, false)), // East Asian ideograph
    (0x235849, (0x9C0A, false)), // East Asian ideograph
    (0x225339, (0x7198, false)), // East Asian ideograph
    (0x227358, (0x7E7B, false)), // East Asian ideograph
    (0x227359, (0x7E7E, false)), // East Asian ideograph
    (0x21735A, (0x56B3, false)), // East Asian ideograph
    (0x22735B, (0x7E81, false)), // East Asian ideograph
    (0x6F595A, (0xCDA9, false)), // Korean hangul
    (0x22735D, (0x7E8A, false)), // East Asian ideograph
    (0x22735E, (0x7E87, false)), // East Asian ideograph
    (0x6F7730, (0xAF50, false)), // Korean hangul
    (0x227360, (0x7E88, false)), // East Asian ideograph
    (0x217362, (0x56CF, false)), // East Asian ideograph
    (0x4B533B, (0x695C, false)), // East Asian ideograph
    (0x227364, (0x7E86, false)), // East Asian ideograph
    (0x23473D, (0x93FB, false)), // East Asian ideograph
    (0x217367, (0x56CD, false)), // East Asian ideograph
    (0x22533C, (0x7197, false)), // East Asian ideograph
    (0x22736A, (0x7E91, false)), // East Asian ideograph
    (0x21736B, (0x56D7, false)), // East Asian ideograph
    (0x22736D, (0x7E94, false)), // East Asian ideograph
    (0x70736E, (0x7BA2, false)), // East Asian ideograph
    (0x23533D, (0x9A0F, false)), // East Asian ideograph
    (0x227370, (0x7E9B, false)), // East Asian ideograph
    (0x227371, (0x7E9A, false)), // East Asian ideograph
    (0x22544B, (0x720C, false)), // East Asian ideograph
    (0x227373, (0x7E99, false)), // East Asian ideograph
    (0x227374, (0x7E98, false)), // East Asian ideograph
    (0x22533E, (0x71B5, false)), // East Asian ideograph
    (0x217376, (0x56EE, false)), // East Asian ideograph
    (0x217377, (0x56E7, false)), // East Asian ideograph
    (0x217379, (0x56FB, false)), // East Asian ideograph
    (0x22533F, (0x71A9, false)), // East Asian ideograph
    (0x21737E, (0x56F7, false)), // East Asian ideograph
    (0x222569, (0x5D97, false)), // East Asian ideograph
    (0x295340, (0x9A92, false)), // East Asian ideograph
    (0x4B3853, (0x586D, false)), // East Asian ideograph (not in Unicode)
    (0x4B5629, (0x85CD, false)), // East Asian ideograph
    (0x6F5341, (0xC18E, false)), // Korean hangul
    (0x6F4A29, (0xADE0, false)), // Korean hangul
    (0x225342, (0x71A5, false)), // East Asian ideograph
    (0x334260, (0x89D4, false)), // East Asian ideograph
    (0x275E50, (0x95ED, false)), // East Asian ideograph
    (0x23584B, (0x9C08, false)), // East Asian ideograph
    (0x2E3936, (0x66CD, false)), // East Asian ideograph
    (0x6F595C, (0xCDC4, false)), // Korean hangul
    (0x235344, (0x9A04, false)), // East Asian ideograph
    (0x6F7732, (0xB060, false)), // Korean hangul
    (0x235345, (0x9A11, false)), // East Asian ideograph
    (0x275B7E, (0x8FDE, false)), // East Asian ideograph
    (0x2D5E28, (0x93C1, false)), // East Asian ideograph
    (0x213A6A, (0x5B8B, false)), // East Asian ideograph
    (0x235347, (0x9A05, false)), // East Asian ideograph
    (0x4B4874, (0x6FF3, false)), // East Asian ideograph
    (0x23584C, (0x9C14, false)), // East Asian ideograph
    (0x215348, (0x80FD, false)), // East Asian ideograph
    (0x6F5349, (0xC1A8, false)), // Korean hangul
    (0x705D5C, (0x8488, false)), // East Asian ideograph
    (0x6F7733, (0xB9C4, false)), // Korean hangul
    (0x224B32, (0x6E72, false)), // East Asian ideograph
    (0x6F5128, (0xBC88, false)), // Korean hangul
    (0x22655A, (0x78D8, false)), // East Asian ideograph
    (0x27534A, (0x8090, false)), // East Asian ideograph
    (0x226D4B, (0x7C0C, false)), // East Asian ideograph
    (0x334740, (0x6D1A, false)), // East Asian ideograph
    (0x4B562B, (0x8535, false)), // East Asian ideograph
    (0x2D534B, (0x80F7, false)), // East Asian ideograph
    (0x27573C, (0x86CE, false)), // East Asian ideograph
    (0x21534C, (0x810A, false)), // East Asian ideograph
    (0x23584D, (0x9C04, false)), // East Asian ideograph
    (0x23534D, (0x9A22, false)), // East Asian ideograph
    (0x6F595E, (0xCDE8, false)), // Korean hangul
    (0x22534E, (0x71AF, false)), // East Asian ideograph
    (0x6F5B36, (0xD161, false)), // Korean hangul
    (0x6F7734, (0xC54D, false)), // Korean hangul
    (0x23534F, (0x9A20, false)), // East Asian ideograph
    (0x6F5350, (0xC1E4, false)), // Korean hangul
    (0x21327A, (0x5152, false)), // East Asian ideograph
    (0x2D6134, (0x99DE, false)), // East Asian ideograph
    (0x233A78, (0x8EB3, false)), // East Asian ideograph
    (0x4C6F7B, (0x7CE8, false)), // East Asian ideograph
    (0x235352, (0x9A27, false)), // East Asian ideograph
    (0x6F5343, (0xC194, false)), // Korean hangul
    (0x6F5353, (0xC1F1, false)), // Korean hangul
    (0x6F7735, (0xC54F, false)), // Korean hangul
    (0x213F44, (0x619A, false)), // East Asian ideograph
    (0x6F5354, (0xC1F3, false)), // Korean hangul
    (0x4C7C45, (0x82AE, false)), // East Asian ideograph
    (0x225355, (0x719A, false)), // East Asian ideograph
    (0x4D4832, (0x953F, false)), // East Asian ideograph
    (0x27573E, (0x8721, false)), // East Asian ideograph
    (0x22367A, (0x65A0, false)), // East Asian ideograph
    (0x223237, (0x63B1, false)), // East Asian ideograph
    (0x6F5629, (0xC65C, false)), // Korean hangul
    (0x225357, (0x71B3, false)), // East Asian ideograph
    (0x27407B, (0x5377, false)), // East Asian ideograph
    (0x275358, (0x80BE, false)), // East Asian ideograph
    (0x213F45, (0x61A9, false)), // East Asian ideograph
    (0x215359, (0x8139, false)), // East Asian ideograph
    (0x2D416E, (0x6534, false)), // East Asian ideograph
    (0x23535A, (0x9A38, false)), // East Asian ideograph
    (0x217421, (0x56F9, false)), // East Asian ideograph
    (0x6F4A2A, (0xADE4, false)), // Korean hangul
    (0x294435, (0x950A, false)), // East Asian ideograph
    (0x217424, (0x56FF, false)), // East Asian ideograph
    (0x227425, (0x7F43, false)), // East Asian ideograph
    (0x275E51, (0x95F5, false)), // East Asian ideograph
    (0x217427, (0x5705, false)), // East Asian ideograph
    (0x227428, (0x7F45, false)), // East Asian ideograph
    (0x217429, (0x5702, false)), // East Asian ideograph
    (0x22742B, (0x7F4B, false)), // East Asian ideograph
    (0x21742C, (0x570A, false)), // East Asian ideograph
    (0x21742D, (0x5709, false)), // East Asian ideograph
    (0x22742E, (0x7F4C, false)), // East Asian ideograph
    (0x22742F, (0x7F4D, false)), // East Asian ideograph
    (0x217430, (0x570C, false)), // East Asian ideograph
    (0x217431, (0x5715, false)), // East Asian ideograph
    (0x227432, (0x7F4F, false)), // East Asian ideograph
    (0x213F46, (0x6194, false)), // East Asian ideograph
    (0x27535E, (0x80A0, false)), // East Asian ideograph
    (0x224345, (0x6AE8, false)), // East Asian ideograph
    (0x217437, (0x571C, false)), // East Asian ideograph
    (0x4B423A, (0x64B9, false)), // East Asian ideograph
    (0x217439, (0x571D, false)), // East Asian ideograph
    (0x21743A, (0x571E, false)), // East Asian ideograph
    (0x6F535F, (0xC220, false)), // Korean hangul
    (0x22743E, (0x7F60, false)), // East Asian ideograph
    (0x22743F, (0x7F61, false)), // East Asian ideograph
    (0x235360, (0x9A2D, false)), // East Asian ideograph
    (0x217442, (0x572E, false)), // East Asian ideograph
    (0x227443, (0x7F5D, false)), // East Asian ideograph
    (0x227445, (0x7F5B, false)), // East Asian ideograph
    (0x235361, (0x9A35, false)), // East Asian ideograph
    (0x217448, (0x5738, false)), // East Asian ideograph
    (0x21744C, (0x572A, false)), // East Asian ideograph
    (0x215362, (0x816B, false)), // East Asian ideograph
    (0x6F4B43, (0xB05D, false)), // Korean hangul
    (0x227450, (0x7F65, false)), // East Asian ideograph
    (0x227451, (0x7F66, false)), // East Asian ideograph
    (0x213F47, (0x618A, false)), // East Asian ideograph
    (0x227453, (0x7F6D, false)), // East Asian ideograph
    (0x227454, (0x7F6B, false)), // East Asian ideograph
    (0x227455, (0x7F67, false)), // East Asian ideograph
    (0x227457, (0x7F68, false)), // East Asian ideograph
    (0x235364, (0x9A32, false)), // East Asian ideograph
    (0x21327E, (0x5165, false)), // East Asian ideograph
    (0x22745E, (0x7F71, false)), // East Asian ideograph
    (0x275365, (0x8111, false)), // East Asian ideograph
    (0x227460, (0x7F73, false)), // East Asian ideograph
    (0x227463, (0x7F76, false)), // East Asian ideograph
    (0x6F5022, (0xBA39, false)), // Korean hangul
    (0x217465, (0x5745, false)), // East Asian ideograph
    (0x6F572F, (0xC7C1, false)), // Korean hangul
    (0x217468, (0x574B, false)), // East Asian ideograph
    (0x217469, (0x574C, false)), // East Asian ideograph
    (0x21746A, (0x573F, false)), // East Asian ideograph
    (0x215367, (0x818F, false)), // East Asian ideograph
    (0x22746C, (0x7F7D, false)), // East Asian ideograph
    (0x6F7739, (0xC61C, false)), // Korean hangul
    (0x217470, (0x5768, false)), // East Asian ideograph
    (0x6F5368, (0xC250, false)), // Korean hangul
    (0x227472, (0x7F86, false)), // East Asian ideograph
    (0x6F4F25, (0xB807, false)), // Korean hangul
    (0x217475, (0x578A, false)), // East Asian ideograph
    (0x225369, (0x71C7, false)), // East Asian ideograph
    (0x345175, (0x7162, false)), // East Asian ideograph
    (0x217479, (0x5774, false)), // East Asian ideograph
    (0x21747A, (0x5767, false)), // East Asian ideograph
    (0x22536A, (0x71B7, false)), // East Asian ideograph
    (0x22747E, (0x7F96, false)), // East Asian ideograph
    (0x6F536B, (0xC270, false)), // Korean hangul
    (0x27536C, (0x80F6, false)), // East Asian ideograph
    (0x274636, (0x6B93, false)), // East Asian ideograph
    (0x6F5521, (0xC549, false)), // Korean hangul
    (0x21536D, (0x819B, false)), // East Asian ideograph
    (0x224348, (0x6AF5, false)), // East Asian ideograph
    (0x4B5632, (0x7C54, false)), // East Asian ideograph
    (0x27536E, (0x80A4, false)), // East Asian ideograph
    (0x454F45, (0x9896, false)), // East Asian ideograph
    (0x22536F, (0x71CF, false)), // East Asian ideograph
    (0x4C5541, (0x4E2C, false)), // East Asian ideograph
    (0x225370, (0x71D6, false)), // East Asian ideograph
    (0x213031, (0x4E1E, false)), // East Asian ideograph
    (0x215371, (0x81A9, false)), // East Asian ideograph
    (0x274637, (0x6BA1, false)), // East Asian ideograph
    (0x6F5522, (0xC54A, false)), // Korean hangul
    (0x213F4A, (0x61C9, false)), // East Asian ideograph
    (0x217463, (0x5749, false)), // East Asian ideograph
    (0x6F5373, (0xC290, false)), // Korean hangul
    (0x335172, (0x7D89, false)), // East Asian ideograph
    (0x212A29, (0xE8D7, false)), // EACC component character
    (0x6F4A2B, (0xADEC, false)), // Korean hangul
    (0x235374, (0x9A3B, false)), // East Asian ideograph
    (0x4B496A, (0x932C, false)), // East Asian ideograph
    (0x225375, (0x71C2, false)), // East Asian ideograph
    (0x6F5376, (0xC29D, false)), // Korean hangul
    (0x233E21, (0x9070, false)), // East Asian ideograph
    (0x213F4B, (0x6190, false)), // East Asian ideograph
    (0x225377, (0x71C5, false)), // East Asian ideograph
    (0x4B385E, (0x5897, false)), // East Asian ideograph
    (0x234749, (0x93FA, false)), // East Asian ideograph
    (0x2D6275, (0x76B7, false)), // East Asian ideograph
    (0x215378, (0x81BF, false)), // East Asian ideograph
    (0x216431, (0x4E36, false)), // East Asian ideograph
    (0x215379, (0x81BD, false)), // East Asian ideograph
    (0x223E24, (0x6919, false)), // East Asian ideograph
    (0x4B496B, (0x83F8, false)), // East Asian ideograph
    (0x21537A, (0x81C9, false)), // East Asian ideograph
    (0x213E25, (0x5FFD, false)), // East Asian ideograph
    (0x21537B, (0x81BE, false)), // East Asian ideograph
    (0x39304C, (0x4E81, false)), // East Asian ideograph
    (0x213E26, (0x5FDD, false)), // East Asian ideograph
    (0x23603F, (0x9F6E, false)), // East Asian ideograph
    (0x27537C, (0x8110, false)), // East Asian ideograph
    (0x33474A, (0x6D1F, false)), // East Asian ideograph
    (0x21537D, (0x81CF, false)), // East Asian ideograph
    (0x213E28, (0x5FFF, false)), // East Asian ideograph
    (0x275746, (0x672E, false)), // East Asian ideograph
    (0x21537E, (0x81D8, false)), // East Asian ideograph
    (0x6F4E26, (0xB54B, false)), // Korean hangul
    (0x227042, (0x7D06, false)), // East Asian ideograph
    (0x294471, (0x951D, false)), // East Asian ideograph
    (0x233E2A, (0x907B, false)), // East Asian ideograph
    (0x6F5968, (0xCE61, false)), // Korean hangul
    (0x6F5B38, (0xD1A0, false)), // Korean hangul
    (0x213E2B, (0x602A, false)), // East Asian ideograph
    (0x213E2C, (0x602F, false)), // East Asian ideograph
    (0x6F585B, (0xCACC, false)), // Korean hangul
    (0x213E2D, (0x6016, false)), // East Asian ideograph
    (0x213E2F, (0x600F, false)), // East Asian ideograph
    (0x6F5969, (0xCE68, false)), // Korean hangul
    (0x227523, (0x7F97, false)), // East Asian ideograph
    (0x227524, (0x7F95, false)), // East Asian ideograph
    (0x51456D, (0x822E, false)), // East Asian ideograph
    (0x217526, (0x5770, false)), // East Asian ideograph
    (0x217528, (0x5771, false)), // East Asian ideograph
    (0x21752A, (0x576E, false)), // East Asian ideograph
    (0x22752C, (0x7FA2, false)), // East Asian ideograph
    (0x21752D, (0x5776, false)), // East Asian ideograph
    (0x21752E, (0x5789, false)), // East Asian ideograph
    (0x217530, (0x577F, false)), // East Asian ideograph
    (0x217531, (0x5775, false)), // East Asian ideograph
    (0x217532, (0x577B, false)), // East Asian ideograph
    (0x227533, (0x7FA7, false)), // East Asian ideograph
    (0x217535, (0x5773, false)), // East Asian ideograph
    (0x223241, (0x636D, false)), // East Asian ideograph
    (0x217538, (0x579F, false)), // East Asian ideograph
    (0x233E34, (0x9083, false)), // East Asian ideograph
    (0x21753A, (0x5793, false)), // East Asian ideograph
    (0x22753B, (0x7FB0, false)), // East Asian ideograph
    (0x22753C, (0x7FAD, false)), // East Asian ideograph
    (0x6F596A, (0xCE69, false)), // Korean hangul
    (0x22753F, (0x7FB1, false)), // East Asian ideograph
    (0x227540, (0x7FB4, false)), // East Asian ideograph
    (0x6F5527, (0xC555, false)), // Korean hangul
    (0x227542, (0x7FB5, false)), // East Asian ideograph
    (0x217543, (0x579A, false)), // East Asian ideograph
    (0x217545, (0x5794, false)), // East Asian ideograph
    (0x217547, (0x57A4, false)), // East Asian ideograph
    (0x217548, (0x5799, false)), // East Asian ideograph
    (0x217549, (0x578C, false)), // East Asian ideograph
    (0x22754A, (0x7FBC, false)), // East Asian ideograph
    (0x21754B, (0x5797, false)), // East Asian ideograph
    (0x22754C, (0x7FBE, false)), // East Asian ideograph
    (0x275749, (0x536B, false)), // East Asian ideograph
    (0x227551, (0x7FC3, false)), // East Asian ideograph
    (0x217552, (0x579C, false)), // East Asian ideograph
    (0x217554, (0x57A7, false)), // East Asian ideograph
    (0x227557, (0x7FCA, false)), // East Asian ideograph
    (0x217559, (0x3013, false)), // East Asian ideograph (not found in unified han)
    (0x21755B, (0x5795, false)), // East Asian ideograph
    (0x213E3A, (0x6068, false)), // East Asian ideograph
    (0x21755F, (0x57B8, false)), // East Asian ideograph
    (0x217560, (0x57C7, false)), // East Asian ideograph
    (0x227567, (0x7FDB, false)), // East Asian ideograph
    (0x227568, (0x7FE3, false)), // East Asian ideograph
    (0x2D3E3C, (0x803B, false)), // East Asian ideograph
    (0x21756A, (0x5809, false)), // East Asian ideograph
    (0x22756C, (0x7FE6, false)), // East Asian ideograph
    (0x22756F, (0x7FE5, false)), // East Asian ideograph
    (0x22545C, (0x5911, false)), // East Asian ideograph
    (0x217571, (0x57DB, false)), // East Asian ideograph
    (0x227572, (0x7FEC, false)), // East Asian ideograph
    (0x227573, (0x7FEB, false)), // East Asian ideograph
    (0x273B60, (0x5C61, false)), // East Asian ideograph
    (0x213E3E, (0x606D, false)), // East Asian ideograph
    (0x227577, (0x7FEF, false)), // East Asian ideograph
    (0x6F596C, (0xCE6D, false)), // Korean hangul
    (0x22757A, (0x7FEE, false)), // East Asian ideograph
    (0x233E3F, (0x9099, false)), // East Asian ideograph
    (0x28632C, (0x7751, false)), // East Asian ideograph
    (0x293066, (0x89C7, false)), // East Asian ideograph
    (0x21757E, (0x57C6, false)), // East Asian ideograph
    (0x233E40, (0x9097, false)), // East Asian ideograph
    (0x4B563A, (0x82A6, false)), // East Asian ideograph (variant of 27563A which maps to 82A6)
    (0x4B3421, (0x5263, false)), // East Asian ideograph
    (0x275936, (0x8C23, false)), // East Asian ideograph
    (0x213E41, (0x604D, false)), // East Asian ideograph
    (0x6F4860, (0xAC01, false)), // Korean hangul
    (0x4C695F, (0x7A63, false)), // East Asian ideograph
    (0x213E42, (0x606B, false)), // East Asian ideograph
    (0x395F49, (0x5F6B, false)), // East Asian ideograph
    (0x23585C, (0x9C09, false)), // East Asian ideograph
    (0x233643, (0x8C9F, false)), // East Asian ideograph
    (0x213E43, (0x6069, false)), // East Asian ideograph
    (0x6F5B39, (0xD1A1, false)), // Korean hangul
    (0x223E44, (0x6911, false)), // East Asian ideograph
    (0x6F552A, (0xC559, false)), // Korean hangul
    (0x4B5A7E, (0x5C69, false)), // East Asian ideograph
    (0x213E46, (0x606A, false)), // East Asian ideograph
    (0x6F4861, (0xAC02, false)), // Korean hangul
    (0x223E47, (0x68EF, false)), // East Asian ideograph
    (0x22545E, (0x720A, false)), // East Asian ideograph
    (0x6F4F4A, (0xB8FD, false)), // Korean hangul
    (0x213E48, (0x6070, false)), // East Asian ideograph
    (0x213E49, (0x6055, false)), // East Asian ideograph
    (0x274640, (0x6BB4, false)), // East Asian ideograph
    (0x234751, (0x9427, false)), // East Asian ideograph
    (0x33433E, (0x95C7, false)), // East Asian ideograph
    (0x213E4B, (0x60A6, false)), // East Asian ideograph
    (0x4D4835, (0x954C, false)), // East Asian ideograph
    (0x2D3C21, (0x57FC, false)), // East Asian ideograph
    (0x275F2E, (0x9633, false)), // East Asian ideograph
    (0x393E4C, (0x6142, false)), // East Asian ideograph
    (0x4B4973, (0x7115, false)), // East Asian ideograph
    (0x213E4D, (0x609F, false)), // East Asian ideograph
    (0x27407E, (0x626A, false)), // East Asian ideograph
    (0x6F596F, (0xCE74, false)), // Korean hangul
    (0x6F552C, (0xC55F, false)), // Korean hangul
    (0x697058, (0x9779, false)), // East Asian ideograph
    (0x275E36, (0x9559, false)), // East Asian ideograph
    (0x2D627E, (0x658B, false)), // East Asian ideograph
    (0x213B48, (0x5C1A, false)), // East Asian ideograph
    (0x2D5E3B, (0x92B9, false)), // East Asian ideograph
    (0x6F4863, (0xAC07, false)), // Korean hangul
    (0x6F4A2D, (0xADF9, false)), // Korean hangul
    (0x393078, (0x9AE3, false)), // East Asian ideograph
    (0x227E51, (0x8423, false)), // East Asian ideograph
    (0x225460, (0x7217, false)), // East Asian ideograph
    (0x223247, (0x63D3, false)), // East Asian ideograph
    (0x213E52, (0x60A3, false)), // East Asian ideograph
    (0x6F5970, (0xCE75, false)), // Korean hangul
    (0x6F5542, (0xC598, false)), // Korean hangul
    (0x223E53, (0x6974, false)), // East Asian ideograph
    (0x6F552D, (0xC560, false)), // Korean hangul
    (0x213E54, (0x6094, false)), // East Asian ideograph
    (0x2D4066, (0x63CE, false)), // East Asian ideograph
    (0x216433, (0x4E3F, false)), // East Asian ideograph
    (0x6F4864, (0xAC08, false)), // Korean hangul
    (0x4B4975, (0x6427, false)), // East Asian ideograph
    (0x275D7A, (0x9532, false)), // East Asian ideograph
    (0x213E57, (0x60B4, false)), // East Asian ideograph
    (0x395D23, (0x91BB, false)), // East Asian ideograph
    (0x4B517E, (0x7E92, false)), // East Asian ideograph
    (0x6F5971, (0xCE78, false)), // Korean hangul
    (0x223E58, (0x6962, false)), // East Asian ideograph
    (0x224B36, (0x6E39, false)), // East Asian ideograph
    (0x6F5468, (0xC4D4, false)), // Korean hangul
    (0x213E59, (0x60CB, false)), // East Asian ideograph
    (0x4B563F, (0x6A98, false)), // East Asian ideograph
    (0x2D4067, (0x62CF, false)), // East Asian ideograph
    (0x6F4865, (0xAC09, false)), // Korean hangul
    (0x6F7621, (0x3181, false)), // Korean hangul
    (0x217622, (0x57C4, false)), // East Asian ideograph
    (0x233E5B, (0x90B6, false)), // East Asian ideograph
    (0x6F7624, (0xE8B0, false)), // Korean hangul
    (0x6F7625, (0xE8B1, false)), // Korean hangul
    (0x4B4556, (0x6A2A, false)), // East Asian ideograph
    (0x217627, (0x70FE, false)), // East Asian ideograph
    (0x227629, (0x7FFD, false)), // East Asian ideograph
    (0x22762A, (0x7FFE, false)), // East Asian ideograph
    (0x21762B, (0x5803, false)), // East Asian ideograph
    (0x22762C, (0x7FFF, false)), // East Asian ideograph
    (0x21762D, (0x57E6, false)), // East Asian ideograph
    (0x22762E, (0x8004, false)), // East Asian ideograph
    (0x233E5D, (0x90B0, false)), // East Asian ideograph
    (0x29332C, (0x8BFC, false)), // East Asian ideograph
    (0x217631, (0x57ED, false)), // East Asian ideograph
    (0x227633, (0x800B, false)), // East Asian ideograph
    (0x227634, (0x800E, false)), // East Asian ideograph
    (0x227635, (0x8011, false)), // East Asian ideograph
    (0x234755, (0x9409, false)), // East Asian ideograph
    (0x227637, (0x8014, false)), // East Asian ideograph
    (0x277638, (0x57AD, false)), // East Asian ideograph
    (0x227639, (0x8016, false)), // East Asian ideograph
    (0x223E5F, (0x6957, false)), // East Asian ideograph
    (0x21763D, (0x57F4, false)), // East Asian ideograph
    (0x22763E, (0x801D, false)), // East Asian ideograph
    (0x294179, (0x9492, false)), // East Asian ideograph
    (0x217640, (0x580D, false)), // East Asian ideograph
    (0x223E60, (0x693F, false)), // East Asian ideograph
    (0x6F7642, (0xE8B4, false)), // Korean hangul
    (0x217643, (0x57EF, false)), // East Asian ideograph
    (0x6F7644, (0xE8B6, false)), // Korean hangul
    (0x6F7645, (0xE8B7, false)), // Korean hangul
    (0x6F4F4B, (0xB904, false)), // Korean hangul
    (0x273E61, (0x6076, false)), // East Asian ideograph
    (0x217648, (0x5801, false)), // East Asian ideograph
    (0x217649, (0x5812, false)), // East Asian ideograph
    (0x6F764A, (0xE8BC, false)), // Korean hangul
    (0x22764B, (0x8025, false)), // East Asian ideograph
    (0x22764C, (0x8026, false)), // East Asian ideograph
    (0x22764D, (0x802A, false)), // East Asian ideograph
    (0x22764E, (0x8029, false)), // East Asian ideograph
    (0x22764F, (0x8028, false)), // East Asian ideograph
    (0x217650, (0x580C, false)), // East Asian ideograph
    (0x217651, (0x5813, false)), // East Asian ideograph
    (0x217652, (0x57F0, false)), // East Asian ideograph
    (0x6F7653, (0xE8C5, false)), // Korean hangul
    (0x6F7654, (0xE8C6, false)), // Korean hangul
    (0x287655, (0x8027, false)), // East Asian ideograph
    (0x217656, (0x580B, false)), // East Asian ideograph
    (0x6F7657, (0xE8C9, false)), // Korean hangul
    (0x217658, (0x57F3, false)), // East Asian ideograph
    (0x217659, (0x5804, false)), // East Asian ideograph
    (0x21765A, (0x57CF, false)), // East Asian ideograph
    (0x22765B, (0x8030, false)), // East Asian ideograph
    (0x22765D, (0x8031, false)), // East Asian ideograph
    (0x21765F, (0x5847, false)), // East Asian ideograph
    (0x227660, (0x8035, false)), // East Asian ideograph
    (0x225021, (0x9E02, false)), // East Asian ideograph
    (0x4D2F5D, (0x8941, false)), // East Asian ideograph (variant of 232F5D which maps to 8941)
    (0x217667, (0x581B, false)), // East Asian ideograph
    (0x227669, (0x8039, false)), // East Asian ideograph
    (0x21766A, (0x5833, false)), // East Asian ideograph
    (0x22766B, (0x8041, false)), // East Asian ideograph
    (0x21766C, (0x581E, false)), // East Asian ideograph
    (0x21766D, (0x583F, false)), // East Asian ideograph
    (0x213F59, (0x61FE, false)), // East Asian ideograph
    (0x227670, (0x8043, false)), // East Asian ideograph
    (0x213E68, (0x60B8, false)), // East Asian ideograph
    (0x217676, (0x5828, false)), // East Asian ideograph
    (0x213E69, (0x60DA, false)), // East Asian ideograph
    (0x217678, (0x582E, false)), // East Asian ideograph
    (0x6F4868, (0xAC12, false)), // Korean hangul
    (0x21767A, (0x581D, false)), // East Asian ideograph
    (0x22767B, (0x8052, false)), // East Asian ideograph
    (0x233E6A, (0x90BD, false)), // East Asian ideograph
    (0x22767E, (0x8062, false)), // East Asian ideograph
    (0x225B69, (0x74AA, false)), // East Asian ideograph
    (0x213E6B, (0x610F, false)), // East Asian ideograph
    (0x2D4D34, (0x76C7, false)), // East Asian ideograph
    (0x4B4046, (0x629C, false)), // East Asian ideograph
    (0x213E6C, (0x611C, false)), // East Asian ideograph
    (0x29306F, (0x89CB, false)), // East Asian ideograph
    (0x213F5A, (0x61FF, false)), // East Asian ideograph
    (0x224832, (0x6CD2, false)), // East Asian ideograph
    (0x234D62, (0x979C, false)), // East Asian ideograph
    (0x6F773D, (0xC733, false)), // Korean hangul
    (0x2D4844, (0x6FD5, false)), // East Asian ideograph
    (0x2D4461, (0x6746, false)), // East Asian ideograph
    (0x213E6E, (0x611F, false)), // East Asian ideograph
    (0x6F4869, (0xAC13, false)), // Korean hangul
    (0x39417C, (0x62E0, false)), // East Asian ideograph
    (0x213E6F, (0x60F0, false)), // East Asian ideograph
    (0x225466, (0x7215, false)), // East Asian ideograph
    (0x22723C, (0x7DF2, false)), // East Asian ideograph
    (0x223E70, (0x696A, false)), // East Asian ideograph
    (0x6F5976, (0xCE89, false)), // Korean hangul
    (0x213E71, (0x60FA, false)), // East Asian ideograph
    (0x224B37, (0x6E71, false)), // East Asian ideograph
    (0x213E72, (0x611A, false)), // East Asian ideograph
    (0x234759, (0x9404, false)), // East Asian ideograph
    (0x333D48, (0x5F3A, false)), // East Asian ideograph
    (0x213E73, (0x6115, false)), // East Asian ideograph
    (0x6F486A, (0xAC14, false)), // Korean hangul
    (0x212A3D, (0xE8EA, false)), // EACC component character
    (0x235866, (0x9C1C, false)), // East Asian ideograph
    (0x233E75, (0x90C7, false)), // East Asian ideograph
    (0x456064, (0x9963, false)), // East Asian ideograph
    (0x6F5977, (0xCE90, false)), // Korean hangul
    (0x6F5B3B, (0xD1A8, false)), // Korean hangul
    (0x2D3B7B, (0x5D08, false)), // East Asian ideograph
    (0x222921, (0x5EF1, false)), // East Asian ideograph
    (0x213F5C, (0x6200, false)), // East Asian ideograph
    (0x697060, (0x9790, false)), // East Asian ideograph
    (0x4B506C, (0x7CAB, false)), // East Asian ideograph
    (0x215D32, (0x91AC, false)), // East Asian ideograph
    (0x225347, (0x71B2, false)), // East Asian ideograph
    (0x213E78, (0x610E, false)), // East Asian ideograph
    (0x2D5E43, (0x92F3, false)), // East Asian ideograph
    (0x6F486B, (0xAC15, false)), // Korean hangul
    (0x213E79, (0x6100, false)), // East Asian ideograph
    (0x23364E, (0x8CB0, false)), // East Asian ideograph
    (0x213E7A, (0x6101, false)), // East Asian ideograph
    (0x4D2925, (0x8770, false)), // East Asian ideograph
    (0x6F5344, (0xC19C, false)), // Korean hangul
    (0x6F5978, (0xCE91, false)), // Korean hangul
    (0x213E7B, (0x60F6, false)), // East Asian ideograph
    (0x6F5535, (0xC575, false)), // Korean hangul
    (0x4B3870, (0x58CC, false)), // East Asian ideograph
    (0x232927, (0x867C, false)), // East Asian ideograph
    (0x223E7D, (0x6980, false)), // East Asian ideograph
    (0x28736D, (0x624D, false)), // East Asian ideograph
    (0x275E62, (0x9615, false)), // East Asian ideograph
    (0x223E7E, (0x6933, false)), // East Asian ideograph
    (0x225469, (0x7213, false)), // East Asian ideograph
    (0x6F4935, (0xAC94, false)), // Korean hangul
    (0x6F4E72, (0xB7A8, false)), // Korean hangul
    (0x2D4D38, (0x76D7, false)), // East Asian ideograph
    (0x6F5536, (0xC57C, false)), // Korean hangul
    (0x4B3871, (0x57BB, false)), // East Asian ideograph
    (0x4B5647, (0x51E6, false)), // East Asian ideograph
    (0x6F486D, (0xAC17, false)), // Korean hangul
    (0x2D5434, (0x64E7, false)), // East Asian ideograph
    (0x234E5C, (0x97DE, false)), // East Asian ideograph
    (0x225B6A, (0x7490, false)), // East Asian ideograph
    (0x23292F, (0x86A8, false)), // East Asian ideograph
    (0x6F597A, (0xCE98, false)), // Korean hangul
    (0x4B5221, (0x7D9A, false)), // East Asian ideograph
    (0x217721, (0x5848, false)), // East Asian ideograph
    (0x6F7722, (0xAD7B, false)), // Korean hangul
    (0x217723, (0x5818, false)), // East Asian ideograph
    (0x6F7724, (0xAD89, false)), // Korean hangul
    (0x6F7725, (0xAD9D, false)), // Korean hangul
    (0x217726, (0x57F5, false)), // East Asian ideograph
    (0x4B5D36, (0x91C6, false)), // East Asian ideograph
    (0x227728, (0x8063, false)), // East Asian ideograph
    (0x21315B, (0x4FBF, false)), // East Asian ideograph
    (0x6F772A, (0xAE0F, false)), // Korean hangul
    (0x21772B, (0x5820, false)), // East Asian ideograph
    (0x6F772C, (0xAE14, false)), // Korean hangul
    (0x6F486E, (0xAC19, false)), // Korean hangul
    (0x6F772E, (0xAEED, false)), // Korean hangul
    (0x6F772F, (0xAF09, false)), // Korean hangul
    (0x217730, (0x584E, false)), // East Asian ideograph
    (0x6F7731, (0xAFBF, false)), // Korean hangul
    (0x227732, (0x806C, false)), // East Asian ideograph
    (0x217733, (0x585D, false)), // East Asian ideograph
    (0x6F4926, (0xAC78, false)), // Korean hangul
    (0x217735, (0x5859, false)), // East Asian ideograph
    (0x6F7736, (0xC552, false)), // Korean hangul
    (0x217737, (0x584B, false)), // East Asian ideograph
    (0x6F7738, (0xC5B1, false)), // Korean hangul
    (0x227739, (0x8075, false)), // East Asian ideograph
    (0x6F773A, (0xC61D, false)), // Korean hangul
    (0x2D3876, (0x58F7, false)), // East Asian ideograph
    (0x6F773C, (0xE8CB, false)), // Korean hangul
    (0x21773D, (0x5865, false)), // East Asian ideograph
    (0x22773E, (0x807B, false)), // East Asian ideograph
    (0x22773F, (0x8079, false)), // East Asian ideograph
    (0x217740, (0x586C, false)), // East Asian ideograph
    (0x213F60, (0x620D, false)), // East Asian ideograph
    (0x217742, (0x5852, false)), // East Asian ideograph
    (0x33475E, (0x6FB9, false)), // East Asian ideograph
    (0x217745, (0x5864, false)), // East Asian ideograph
    (0x227747, (0x808A, false)), // East Asian ideograph
    (0x217748, (0x584F, false)), // East Asian ideograph
    (0x227749, (0x808E, false)), // East Asian ideograph
    (0x213533, (0x53FC, false)), // East Asian ideograph
    (0x6F486F, (0xAC1A, false)), // Korean hangul
    (0x21774D, (0x584D, false)), // East Asian ideograph
    (0x22774E, (0x809F, false)), // East Asian ideograph
    (0x6F5730, (0xC7C8, false)), // Korean hangul
    (0x225029, (0x7054, false)), // East Asian ideograph
    (0x232939, (0x8698, false)), // East Asian ideograph
    (0x217758, (0x5892, false)), // East Asian ideograph
    (0x6F597C, (0xCEA1, false)), // Korean hangul
    (0x21775A, (0x588E, false)), // East Asian ideograph
    (0x22775C, (0x670A, false)), // East Asian ideograph
    (0x70775D, (0x9B0F, false)), // East Asian ideograph
    (0x282632, (0x5D58, false)), // East Asian ideograph
    (0x21775F, (0x5840, false)), // East Asian ideograph
    (0x227760, (0x80A7, false)), // East Asian ideograph
    (0x227761, (0x80B0, false)), // East Asian ideograph
    (0x33475F, (0x60BD, false)), // East Asian ideograph
    (0x21576C, (0x88FD, false)), // East Asian ideograph
    (0x217765, (0x5890, false)), // East Asian ideograph
    (0x217768, (0x5898, false)), // East Asian ideograph
    (0x227769, (0x80B5, false)), // East Asian ideograph
    (0x22776A, (0x80A6, false)), // East Asian ideograph
    (0x21776B, (0x587D, false)), // East Asian ideograph
    (0x6F5C36, (0xD3B8, false)), // Korean hangul
    (0x21776F, (0x587F, false)), // East Asian ideograph
    (0x217770, (0x5881, false)), // East Asian ideograph
    (0x707771, (0x9EE2, false)), // East Asian ideograph (Version J extension)
    (0x227773, (0x80E0, false)), // East Asian ideograph
    (0x21693E, (0x5135, false)), // East Asian ideograph
    (0x235B26, (0x9D5A, false)), // East Asian ideograph
    (0x6F597D, (0xCEA3, false)), // Korean hangul
    (0x213D62, (0x5F8B, false)), // East Asian ideograph
    (0x22777B, (0x80DF, false)), // East Asian ideograph
    (0x22777D, (0x80C2, false)), // East Asian ideograph
    (0x21777E, (0x58A1, false)), // East Asian ideograph
    (0x226940, (0x7A5C, false)), // East Asian ideograph
    (0x4B7421, (0xF9A9, false)), // East Asian ideograph
    (0x4C433F, (0x6A65, false)), // East Asian ideograph
    (0x6F4D21, (0xB304, false)), // Korean hangul
    (0x45606B, (0x98F0, false)), // East Asian ideograph
    (0x27482D, (0x6C64, false)), // East Asian ideograph
    (0x293B6B, (0x8F8B, false)), // East Asian ideograph
    (0x276944, (0x50A9, false)), // East Asian ideograph
    (0x213F63, (0x6212, false)), // East Asian ideograph
    (0x2D5E4A, (0x945A, false)), // East Asian ideograph
    (0x6F4A30, (0xAE00, false)), // Korean hangul
    (0x275E57, (0x95F8, false)), // East Asian ideograph
    (0x6F5321, (0xC11C, false)), // Korean hangul
    (0x276948, (0x50A5, false)), // East Asian ideograph
    (0x6F553C, (0xC58D, false)), // Korean hangul
    (0x213F64, (0x6211, false)), // East Asian ideograph
    (0x215D3A, (0x91CD, false)), // East Asian ideograph
    (0x234762, (0x9423, false)), // East Asian ideograph
    (0x23294B, (0x86BF, false)), // East Asian ideograph
    (0x6F4956, (0xACF6, false)), // Korean hangul
    (0x6F4B41, (0xB057, false)), // Korean hangul
    (0x292C5D, (0x867F, false)), // East Asian ideograph
    (0x4B3435, (0x52B4, false)), // East Asian ideograph
    (0x6F4E27, (0xB54C, false)), // Korean hangul
    (0x222951, (0x5F33, false)), // East Asian ideograph
    (0x223258, (0x63E6, false)), // East Asian ideograph
    (0x235870, (0x9C2E, false)), // East Asian ideograph
    (0x227247, (0x7DF1, false)), // East Asian ideograph
    (0x6F5B3D, (0xD1B1, false)), // Korean hangul
    (0x6F553E, (0xC590, false)), // Korean hangul
    (0x213F66, (0x6215, false)), // East Asian ideograph
    (0x2D3539, (0x52FE, false)), // East Asian ideograph
    (0x22613B, (0x76E6, false)), // East Asian ideograph
    (0x4B3436, (0x52F2, false)), // East Asian ideograph
    (0x6F4E6C, (0xB791, false)), // Korean hangul
    (0x23517A, (0x9958, false)), // East Asian ideograph
    (0x214C30, (0x755A, false)), // East Asian ideograph
    (0x226957, (0x7A6E, false)), // East Asian ideograph
    (0x222958, (0x5F38, false)), // East Asian ideograph
    (0x2D424F, (0x555F, false)), // East Asian ideograph
    (0x215D3D, (0x91D0, false)), // East Asian ideograph
    (0x22613C, (0x76E9, false)), // East Asian ideograph
    (0x334342, (0x7156, false)), // East Asian ideograph
    (0x513D67, (0x8FF3, false)), // East Asian ideograph
    (0x217824, (0x58B1, false)), // East Asian ideograph
    (0x227827, (0x80D9, false)), // East Asian ideograph
    (0x4B4544, (0x69D8, false)), // East Asian ideograph
    (0x4C695C, (0x7A06, false)), // East Asian ideograph
    (0x22782A, (0x80DD, false)), // East Asian ideograph
    (0x21782B, (0x58AD, false)), // East Asian ideograph
    (0x22782D, (0x80CF, false)), // East Asian ideograph
    (0x21782E, (0x58A0, false)), // East Asian ideograph
    (0x22782F, (0x80CD, false)), // East Asian ideograph
    (0x227830, (0x80D7, false)), // East Asian ideograph
    (0x213F68, (0x621A, false)), // East Asian ideograph
    (0x217832, (0x58A6, false)), // East Asian ideograph
    (0x227833, (0x80F2, false)), // East Asian ideograph
    (0x227834, (0x80FA, false)), // East Asian ideograph
    (0x227838, (0x80FE, false)), // East Asian ideograph
    (0x21783A, (0x58C8, false)), // East Asian ideograph
    (0x2D3C36, (0x5DE3, false)), // East Asian ideograph
    (0x22783C, (0x8103, false)), // East Asian ideograph
    (0x275762, (0x8865, false)), // East Asian ideograph
    (0x227840, (0x80F9, false)), // East Asian ideograph
    (0x217841, (0x58BC, false)), // East Asian ideograph
    (0x227842, (0x80D4, false)), // East Asian ideograph
    (0x33365A, (0x5405, false)), // East Asian ideograph
    (0x4B4545, (0x6982, false)), // East Asian ideograph
    (0x217849, (0x58BF, false)), // East Asian ideograph
    (0x22784B, (0x8118, false)), // East Asian ideograph
    (0x21784C, (0x58BA, false)), // East Asian ideograph
    (0x222962, (0x5F4D, false)), // East Asian ideograph
    (0x6F5541, (0xC597, false)), // Korean hangul
    (0x227850, (0x8130, false)), // East Asian ideograph
    (0x232963, (0x86B4, false)), // East Asian ideograph
    (0x227854, (0x8124, false)), // East Asian ideograph
    (0x227855, (0x811B, false)), // East Asian ideograph
    (0x217856, (0x58CE, false)), // East Asian ideograph
    (0x2D5E50, (0x9587, false)), // East Asian ideograph
    (0x6F4878, (0xAC2F, false)), // Korean hangul
    (0x21785A, (0x58E0, false)), // East Asian ideograph
    (0x21785E, (0x58DA, false)), // East Asian ideograph
    (0x227860, (0x812A, false)), // East Asian ideograph
    (0x227861, (0x811E, false)), // East Asian ideograph
    (0x294362, (0x94EA, false)), // East Asian ideograph
    (0x227864, (0x8121, false)), // East Asian ideograph
    (0x212321, (0x3000, false)), // Ideographic space per ANSI Z39.64
    (0x227866, (0x8117, false)), // East Asian ideograph
    (0x227869, (0x813A, false)), // East Asian ideograph
    (0x22786A, (0x815A, false)), // East Asian ideograph
    (0x21786C, (0x58FC, false)), // East Asian ideograph
    (0x22786D, (0x8148, false)), // East Asian ideograph
    (0x28786E, (0x80E8, false)), // East Asian ideograph
    (0x4B387D, (0x591B, false)), // East Asian ideograph
    (0x217870, (0x5902, false)), // East Asian ideograph
    (0x213B27, (0x5BCC, false)), // East Asian ideograph
    (0x217873, (0x5906, false)), // East Asian ideograph
    (0x217874, (0x6535, false)), // East Asian ideograph
    (0x227877, (0x814C, false)), // East Asian ideograph
    (0x21787A, (0x5910, false)), // East Asian ideograph
    (0x21787C, (0x8641, false)), // East Asian ideograph
    (0x22787D, (0x8141, false)), // East Asian ideograph
    (0x22325D, (0x63F6, false)), // East Asian ideograph
    (0x214C34, (0x7570, false)), // East Asian ideograph
    (0x343A5B, (0x572C, false)), // East Asian ideograph
    (0x2E735D, (0x7D56, false)), // East Asian ideograph
    (0x276871, (0x4FAA, false)), // East Asian ideograph
    (0x6F5543, (0xC59C, false)), // Korean hangul
    (0x696464, (0x7C90, false)), // East Asian ideograph
    (0x213B28, (0x5BD2, false)), // East Asian ideograph
    (0x234326, (0x924C, false)), // East Asian ideograph
    (0x275765, (0x88C5, false)), // East Asian ideograph
    (0x23296F, (0x86E9, false)), // East Asian ideograph
    (0x22325E, (0x63F2, false)), // East Asian ideograph
    (0x214C35, (0x7565, false)), // East Asian ideograph
    (0x6F557C, (0xC63A, false)), // Korean hangul
    (0x235433, (0x9A64, false)), // East Asian ideograph
    (0x222971, (0x5F61, false)), // East Asian ideograph
    (0x6F5544, (0xC5B4, false)), // Korean hangul
    (0x2D5E24, (0x7145, false)), // East Asian ideograph
    (0x23476A, (0x9407, false)), // East Asian ideograph
    (0x4B403D, (0x62DD, false)), // East Asian ideograph
    (0x6F487B, (0xAC38, false)), // Korean hangul
    (0x2D4D65, (0x53E1, false)), // East Asian ideograph
    (0x232974, (0x86D5, false)), // East Asian ideograph
    (0x22325F, (0x63F8, false)), // East Asian ideograph
    (0x23365E, (0x8CD8, false)), // East Asian ideograph
    (0x235434, (0x9A66, false)), // East Asian ideograph
    (0x275421, (0x80EA, false)), // East Asian ideograph
    (0x275E37, (0x9535, false)), // East Asian ideograph
    (0x215422, (0x81DF, false)), // East Asian ideograph
    (0x6F5862, (0xCB10, false)), // Korean hangul
    (0x6F487C, (0xAC39, false)), // Korean hangul
    (0x6F4A32, (0xAE08, false)), // Korean hangul
    (0x6F5423, (0xC2DC, false)), // Korean hangul
    (0x27623F, (0x9E43, false)), // East Asian ideograph
    (0x275E59, (0x95FA, false)), // East Asian ideograph
    (0x215424, (0x81E5, false)), // East Asian ideograph
    (0x23365F, (0x8CD5, false)), // East Asian ideograph
    (0x456076, (0x9980, false)), // East Asian ideograph
    (0x215425, (0x81E8, false)), // East Asian ideograph
    (0x6F5B48, (0xD23D, false)), // Korean hangul
    (0x275142, (0x7EFD, false)), // East Asian ideograph
    (0x225426, (0x71D4, false)), // East Asian ideograph
    (0x235427, (0x9A4A, false)), // East Asian ideograph
    (0x6F487D, (0xAC40, false)), // Korean hangul
    (0x4B5428, (0x81ED, false)), // East Asian ideograph (variant of 215428 which maps to 81ED)
    (0x235879, (0x9C24, false)), // East Asian ideograph
    (0x6F5C6A, (0xD56B, false)), // Korean hangul
    (0x23542A, (0x9A58, false)), // East Asian ideograph
    (0x6F546D, (0xC4F8, false)), // Korean hangul
    (0x6F5547, (0xC5B8, false)), // Korean hangul
    (0x27542B, (0x53F0, false)), // East Asian ideograph
    (0x333D4C, (0x7030, false)), // East Asian ideograph
    (0x23542C, (0x9A56, false)), // East Asian ideograph
    (0x6F542D, (0xC2F6, false)), // Korean hangul
    (0x293D4E, (0x8FF8, false)), // East Asian ideograph
    (0x28352A, (0x6448, false)), // East Asian ideograph
    (0x47577A, (0x9BD6, false)), // East Asian ideograph (variant of 23577A which maps to 9BD6)
    (0x6F5B3F, (0xD1B5, false)), // Korean hangul
    (0x6F5430, (0xC2F8, false)), // Korean hangul
    (0x227925, (0x814D, false)), // East Asian ideograph
    (0x6F5431, (0xC2F9, false)), // Korean hangul
    (0x217928, (0x592C, false)), // East Asian ideograph
    (0x21792B, (0x592F, false)), // East Asian ideograph
    (0x275432, (0x4E0E, false)), // East Asian ideograph
    (0x22792E, (0x6720, false)), // East Asian ideograph
    (0x21507D, (0x7D14, false)), // East Asian ideograph
    (0x217930, (0x593C, false)), // East Asian ideograph
    (0x395F68, (0x8987, false)), // East Asian ideograph
    (0x227932, (0x8160, false)), // East Asian ideograph
    (0x215433, (0x8208, false)), // East Asian ideograph
    (0x225039, (0x7074, false)), // East Asian ideograph
    (0x217938, (0x594D, false)), // East Asian ideograph
    (0x215434, (0x8209, false)), // East Asian ideograph
    (0x22793B, (0x8169, false)), // East Asian ideograph
    (0x22793C, (0x817C, false)), // East Asian ideograph
    (0x275435, (0x65E7, false)), // East Asian ideograph
    (0x294E5C, (0x97EB, false)), // East Asian ideograph
    (0x227941, (0x8161, false)), // East Asian ideograph
    (0x6F5A65, (0xD081, false)), // Korean hangul
    (0x217943, (0x5953, false)), // East Asian ideograph
    (0x225436, (0x71E8, false)), // East Asian ideograph
    (0x227946, (0x8176, false)), // East Asian ideograph
    (0x227947, (0x8174, false)), // East Asian ideograph
    (0x227948, (0x8167, false)), // East Asian ideograph
    (0x334633, (0x6B8B, false)), // East Asian ideograph (variant of 274633 which maps to 6B8B)
    (0x22794B, (0x816F, false)), // East Asian ideograph
    (0x22794D, (0x8182, false)), // East Asian ideograph
    (0x4C794E, (0x80B7, false)), // East Asian ideograph
    (0x21794F, (0x5961, false)), // East Asian ideograph
    (0x227951, (0x818B, false)), // East Asian ideograph
    (0x227952, (0x8186, false)), // East Asian ideograph
    (0x217954, (0x596C, false)), // East Asian ideograph
    (0x217955, (0x596D, false)), // East Asian ideograph
    (0x274830, (0x6D4B, false)), // East Asian ideograph
    (0x4B6324, (0x9F62, false)), // East Asian ideograph
    (0x227959, (0x8183, false)), // East Asian ideograph
    (0x21543A, (0x8214, false)), // East Asian ideograph
    (0x334770, (0x5A6C, false)), // East Asian ideograph
    (0x69543B, (0x57B0, false)), // East Asian ideograph
    (0x217965, (0x597C, false)), // East Asian ideograph
    (0x6F4A33, (0xAE09, false)), // Korean hangul
    (0x217969, (0x59A7, false)), // East Asian ideograph
    (0x22796A, (0x819F, false)), // East Asian ideograph
    (0x22796B, (0x81A3, false)), // East Asian ideograph
    (0x287941, (0x8136, false)), // East Asian ideograph
    (0x21796F, (0x599A, false)), // East Asian ideograph
    (0x227970, (0x8198, false)), // East Asian ideograph
    (0x22503B, (0x707A, false)), // East Asian ideograph
    (0x335E3D, (0x9244, false)), // East Asian ideograph
    (0x227975, (0x8195, false)), // East Asian ideograph
    (0x227977, (0x8197, false)), // East Asian ideograph
    (0x22543F, (0x71E1, false)), // East Asian ideograph
    (0x22797C, (0x81AA, false)), // East Asian ideograph
    (0x224372, (0x6B1E, false)), // East Asian ideograph
    (0x22797E, (0x6725, false)), // East Asian ideograph
    (0x213B30, (0x5BDE, false)), // East Asian ideograph
    (0x2D5440, (0x6841, false)), // East Asian ideograph
    (0x6F4862, (0xAC04, false)), // Korean hangul
    (0x215441, (0x822B, false)), // East Asian ideograph
    (0x275068, (0x7CAA, false)), // East Asian ideograph
    (0x695442, (0x57D6, false)), // East Asian ideograph
    (0x227255, (0x7E12, false)), // East Asian ideograph
    (0x235443, (0x9AB1, false)), // East Asian ideograph
    (0x294E79, (0x9878, false)), // East Asian ideograph
    (0x6F5444, (0xC345, false)), // Korean hangul
    (0x213B31, (0x5BE6, false)), // East Asian ideograph
    (0x235445, (0x9AB3, false)), // East Asian ideograph
    (0x33432F, (0x664B, false)), // East Asian ideograph
    (0x225651, (0x72C6, false)), // East Asian ideograph
    (0x2D5446, (0x8229, false)), // East Asian ideograph
    (0x216E57, (0x53FB, false)), // East Asian ideograph
    (0x214C3E, (0x758F, false)), // East Asian ideograph
    (0x276329, (0x9F8C, false)), // East Asian ideograph
    (0x6F554D, (0xC5C4, false)), // Korean hangul
    (0x235449, (0x9AB6, false)), // East Asian ideograph
    (0x227427, (0x7F46, false)), // East Asian ideograph
    (0x224A62, (0x6E4B, false)), // East Asian ideograph
    (0x27544A, (0x8231, false)), // East Asian ideograph
    (0x294161, (0x9487, false)), // East Asian ideograph
    (0x27544B, (0x8230, false)), // East Asian ideograph
    (0x283955, (0x6619, false)), // East Asian ideograph
    (0x23544C, (0x9ABB, false)), // East Asian ideograph
    (0x233667, (0x8CE8, false)), // East Asian ideograph
    (0x4D2F7A, (0x891D, false)), // East Asian ideograph
    (0x6F5345, (0xC19D, false)), // Korean hangul
    (0x6F544D, (0xC37D, false)), // Korean hangul
    (0x235871, (0x9C28, false)), // East Asian ideograph
    (0x6F554E, (0xC5C5, false)), // Korean hangul
    (0x27544E, (0x8270, false)), // East Asian ideograph
    (0x234774, (0x943F, false)), // East Asian ideograph
    (0x22544F, (0x71FC, false)), // East Asian ideograph
    (0x235450, (0x9ABA, false)), // East Asian ideograph
    (0x695451, (0x58B9, false)), // East Asian ideograph
    (0x233668, (0x8CE9, false)), // East Asian ideograph
    (0x4B4553, (0x6955, false)), // East Asian ideograph
    (0x6F4E77, (0xB7B4, false)), // Korean hangul
    (0x274831, (0x6DA1, false)), // East Asian ideograph
    (0x233225, (0x8A22, false)), // East Asian ideograph
    (0x6F554F, (0xC5C6, false)), // Korean hangul
    (0x6F5453, (0xC3DC, false)), // Korean hangul
    (0x235454, (0x9ABD, false)), // East Asian ideograph
    (0x2E6C26, (0x7BE0, false)), // East Asian ideograph
    (0x6F4A34, (0xAE0B, false)), // Korean hangul
    (0x6F5455, (0xC3E0, false)), // Korean hangul
    (0x225456, (0x71F9, false)), // East Asian ideograph
    (0x225040, (0x7093, false)), // East Asian ideograph
    (0x23543F, (0x9AAD, false)), // East Asian ideograph
    (0x235457, (0x9AC1, false)), // East Asian ideograph
    (0x6F5550, (0xC5C7, false)), // Korean hangul
    (0x275458, (0x5DF4, false)), // East Asian ideograph (duplicate simplified)
    (0x274222, (0x62C5, false)), // East Asian ideograph
    (0x4B3B22, (0x51A6, false)), // East Asian ideograph
    (0x235459, (0x9AC0, false)), // East Asian ideograph
    (0x22572C, (0x72FB, false)), // East Asian ideograph
    (0x23545A, (0x9AC2, false)), // East Asian ideograph
    (0x217A21, (0x5990, false)), // East Asian ideograph
    (0x22545B, (0x720E, false)), // East Asian ideograph
    (0x217A24, (0x59C5, false)), // East Asian ideograph
    (0x217A25, (0x59B5, false)), // East Asian ideograph
    (0x217A28, (0x59CF, false)), // East Asian ideograph
    (0x21545C, (0x82BB, false)), // East Asian ideograph
    (0x217A2A, (0x59BA, false)), // East Asian ideograph
    (0x3F377B, (0x784E, false)), // East Asian ideograph (Version J extension)
    (0x217A2C, (0x59B8, false)), // East Asian ideograph
    (0x6F546F, (0xC501, false)), // Korean hangul
    (0x227A2E, (0x81B0, false)), // East Asian ideograph
    (0x227A2F, (0x81B4, false)), // East Asian ideograph
    (0x215D4F, (0x9237, false)), // East Asian ideograph
    (0x227A33, (0x81B7, false)), // East Asian ideograph
    (0x217A35, (0x59B2, false)), // East Asian ideograph
    (0x227A37, (0x81BB, false)), // East Asian ideograph
    (0x227A38, (0x81C1, false)), // East Asian ideograph
    (0x227A39, (0x81CC, false)), // East Asian ideograph
    (0x217A3A, (0x59B7, false)), // East Asian ideograph
    (0x227A3B, (0x81C4, false)), // East Asian ideograph
    (0x217A3E, (0x59C1, false)), // East Asian ideograph
    (0x227A40, (0x81D1, false)), // East Asian ideograph
    (0x227A41, (0x81CE, false)), // East Asian ideograph
    (0x217A43, (0x59F9, false)), // East Asian ideograph
    (0x217A44, (0x59F8, false)), // East Asian ideograph
    (0x212A43, (0xE8F0, false)), // EACC component character
    (0x225461, (0x7207, false)), // East Asian ideograph
    (0x227A4B, (0x81DB, false)), // East Asian ideograph
    (0x6F5552, (0xC5C9, false)), // Korean hangul
    (0x6F5462, (0xC468, false)), // Korean hangul
    (0x227A4F, (0x81DD, false)), // East Asian ideograph
    (0x217A50, (0x59F1, false)), // East Asian ideograph
    (0x217A51, (0x5A00, false)), // East Asian ideograph
    (0x217A52, (0x59DE, false)), // East Asian ideograph
    (0x227A53, (0x81DE, false)), // East Asian ideograph
    (0x227A56, (0x81E0, false)), // East Asian ideograph
    (0x227A57, (0x81E2, false)), // East Asian ideograph
    (0x6F5464, (0xC474, false)), // Korean hangul
    (0x227A5B, (0x81E7, false)), // East Asian ideograph
    (0x217A5D, (0x59F6, false)), // East Asian ideograph
    (0x217A5E, (0x59DD, false)), // East Asian ideograph
    (0x217A5F, (0x59FA, false)), // East Asian ideograph
    (0x227A60, (0x81EF, false)), // East Asian ideograph
    (0x217A61, (0x59E4, false)), // East Asian ideograph
    (0x227A65, (0x81F2, false)), // East Asian ideograph
    (0x227A68, (0x81F6, false)), // East Asian ideograph
    (0x6F5553, (0xC5CA, false)), // Korean hangul
    (0x6F5467, (0xC494, false)), // Korean hangul
    (0x274225, (0x6324, false)), // East Asian ideograph
    (0x217A6E, (0x5A2A, false)), // East Asian ideograph
    (0x213B38, (0x5BF5, false)), // East Asian ideograph
    (0x227A70, (0x8201, false)), // East Asian ideograph
    (0x2D5468, (0x6959, false)), // East Asian ideograph
    (0x227A72, (0x8201, false)), // East Asian ideograph (not in Unicode)
    (0x227A74, (0x8203, false)), // East Asian ideograph
    (0x227A75, (0x8204, false)), // East Asian ideograph
    (0x2D3C49, (0x83F7, false)), // East Asian ideograph
    (0x227A77, (0x820B, false)), // East Asian ideograph
    (0x217A78, (0x5A09, false)), // East Asian ideograph
    (0x23546A, (0x9AD1, false)), // East Asian ideograph
    (0x217A7E, (0x5A12, false)), // East Asian ideograph
    (0x6F4E78, (0xB7B5, false)), // Korean hangul
    (0x6F546B, (0xC4F1, false)), // Korean hangul
    (0x6F5554, (0xC5CC, false)), // Korean hangul
    (0x6F546C, (0xC4F4, false)), // Korean hangul
    (0x274226, (0x62E7, false)), // East Asian ideograph
    (0x213B39, (0x5BF6, false)), // East Asian ideograph
    (0x21546D, (0x82D3, false)), // East Asian ideograph
    (0x234337, (0x927C, false)), // East Asian ideograph
    (0x22546E, (0x7218, false)), // East Asian ideograph
    (0x28395C, (0x6654, false)), // East Asian ideograph
    (0x2D546F, (0x83C0, false)), // East Asian ideograph
    (0x22725E, (0x7E09, false)), // East Asian ideograph
    (0x4B4559, (0x9792, false)), // East Asian ideograph
    (0x6F5470, (0xC50C, false)), // Korean hangul
    (0x4B5227, (0x6B20, false)), // East Asian ideograph
    (0x6F5555, (0xC5CE, false)), // Korean hangul
    (0x225471, (0x720B, false)), // East Asian ideograph
    (0x33477B, (0x904A, false)), // East Asian ideograph
    (0x235472, (0x9ADC, false)), // East Asian ideograph
    (0x4B5223, (0x7E4A, false)), // East Asian ideograph
    (0x275777, (0x4EB5, false)), // East Asian ideograph
    (0x215474, (0x834A, false)), // East Asian ideograph
    (0x22725F, (0x7E1F, false)), // East Asian ideograph
    (0x23366F, (0x8CEB, false)), // East Asian ideograph
    (0x335445, (0x67C1, false)), // East Asian ideograph
    (0x6F5475, (0xC530, false)), // Korean hangul
    (0x6F5556, (0xC5D0, false)), // Korean hangul
    (0x235476, (0x9AE0, false)), // East Asian ideograph
    (0x274228, (0x62DF, false)), // East Asian ideograph
    (0x23477C, (0x943D, false)), // East Asian ideograph
    (0x215477, (0x8350, false)), // East Asian ideograph
    (0x27593F, (0x8BC1, false)), // East Asian ideograph
    (0x233F22, (0x90DD, false)), // East Asian ideograph
    (0x6F5478, (0xC53B, false)), // Korean hangul
    (0x293F23, (0x90CF, false)), // East Asian ideograph
    (0x6F5827, (0xC999, false)), // Korean hangul
    (0x225479, (0x721A, false)), // East Asian ideograph
    (0x233670, (0x8CDA, false)), // East Asian ideograph
    (0x212A44, (0xE8F1, false)), // EACC component character
    (0x335446, (0x8221, false)), // East Asian ideograph
    (0x22437E, (0x6B2C, false)), // East Asian ideograph
    (0x4B5154, (0x7DF4, false)), // East Asian ideograph
    (0x6F547C, (0xC544, false)), // Korean hangul
    (0x233F27, (0x90D8, false)), // East Asian ideograph
    (0x6F5B6D, (0xD30D, false)), // Korean hangul
    (0x22547D, (0x721F, false)), // East Asian ideograph
    (0x273F28, (0x6001, false)), // East Asian ideograph
    (0x223272, (0x63EB, false)), // East Asian ideograph
    (0x6F4F53, (0xB974, false)), // Korean hangul
    (0x213F29, (0x613E, false)), // East Asian ideograph
    (0x225048, (0x7096, false)), // East Asian ideograph (not in Unicode)
    (0x6F5624, (0xC650, false)), // Korean hangul
    (0x213F2A, (0x6127, false)), // East Asian ideograph
    (0x27422A, (0x6269, false)), // East Asian ideograph
    (0x213C2B, (0x5D87, false)), // East Asian ideograph
    (0x213F2C, (0x6147, false)), // East Asian ideograph
    (0x275F37, (0x9645, false)), // East Asian ideograph
    (0x223F2D, (0x6985, false)), // East Asian ideograph
    (0x273F2E, (0x5E86, false)), // East Asian ideograph
    (0x6F4E79, (0xB7C9, false)), // Korean hangul
    (0x274833, (0x6D51, false)), // East Asian ideograph
    (0x213F2F, (0x6167, false)), // East Asian ideograph
    (0x6F5559, (0xC5D8, false)), // Korean hangul
    (0x27422B, (0x63B7, false)), // East Asian ideograph
    (0x2E624F, (0x772D, false)), // East Asian ideograph
    (0x227B27, (0x821D, false)), // East Asian ideograph
    (0x227B29, (0x8220, false)), // East Asian ideograph
    (0x6F4A36, (0xAE30, false)), // Korean hangul
    (0x217B2C, (0x5A60, false)), // East Asian ideograph
    (0x223F32, (0x693D, false)), // East Asian ideograph
    (0x227B2E, (0x822D, false)), // East Asian ideograph
    (0x227B2F, (0x822F, false)), // East Asian ideograph
    (0x6F5477, (0xC539, false)), // Korean hangul
    (0x217B31, (0x5A67, false)), // East Asian ideograph
    (0x227B32, (0x8238, false)), // East Asian ideograph
    (0x273F33, (0x5FE7, false)), // East Asian ideograph
    (0x227B34, (0x823A, false)), // East Asian ideograph
    (0x227B35, (0x8233, false)), // East Asian ideograph
    (0x227B36, (0x8234, false)), // East Asian ideograph
    (0x213F34, (0x617C, false)), // East Asian ideograph
    (0x227B3A, (0x8232, false)), // East Asian ideograph
    (0x217B3B, (0x5A5E, false)), // East Asian ideograph
    (0x217B3C, (0x5A6D, false)), // East Asian ideograph
    (0x217B3D, (0x5A35, false)), // East Asian ideograph
    (0x217B3E, (0x5A55, false)), // East Asian ideograph
    (0x27422C, (0x64B5, false)), // East Asian ideograph
    (0x215D58, (0x9234, false)), // East Asian ideograph (variant of 4B5D58 which maps to 9234)
    (0x217B41, (0x5A2C, false)), // East Asian ideograph
    (0x227B42, (0x8248, false)), // East Asian ideograph
    (0x227B43, (0x8249, false)), // East Asian ideograph
    (0x227B45, (0x8244, false)), // East Asian ideograph
    (0x227B47, (0x8240, false)), // East Asian ideograph
    (0x227B48, (0x8241, false)), // East Asian ideograph
    (0x217B49, (0x5A65, false)), // East Asian ideograph
    (0x227B4A, (0x8245, false)), // East Asian ideograph
    (0x227B4B, (0x824B, false)), // East Asian ideograph
    (0x334E37, (0x784E, false)), // East Asian ideograph
    (0x227B50, (0x824F, false)), // East Asian ideograph
    (0x213F38, (0x6158, false)), // East Asian ideograph
    (0x217B52, (0x5A64, false)), // East Asian ideograph
    (0x227B53, (0x824E, false)), // East Asian ideograph
    (0x227B56, (0x8256, false)), // East Asian ideograph
    (0x227B57, (0x8257, false)), // East Asian ideograph
    (0x2D6B33, (0x5231, false)), // East Asian ideograph (not in Unicode)
    (0x6F555B, (0xC5E1, false)), // Korean hangul
    (0x223F3A, (0x6934, false)), // East Asian ideograph
    (0x227B5E, (0x825A, false)), // East Asian ideograph
    (0x227B62, (0x825F, false)), // East Asian ideograph
    (0x223F3B, (0x6969, false)), // East Asian ideograph
    (0x217B65, (0x5A8A, false)), // East Asian ideograph
    (0x227B67, (0x8262, false)), // East Asian ideograph
    (0x217B69, (0x5ACF, false)), // East Asian ideograph
    (0x217B6A, (0x5A7A, false)), // East Asian ideograph
    (0x227B6B, (0x8268, false)), // East Asian ideograph
    (0x227B6F, (0x826D, false)), // East Asian ideograph
    (0x217B71, (0x5A9F, false)), // East Asian ideograph
    (0x273F3E, (0x5BAA, false)), // East Asian ideograph
    (0x227B77, (0x8278, false)), // East Asian ideograph
    (0x213F3F, (0x6191, false)), // East Asian ideograph
    (0x227B7D, (0x827F, false)), // East Asian ideograph
    (0x23433F, (0x928D, false)), // East Asian ideograph
    (0x213F41, (0x61AB, false)), // East Asian ideograph
    (0x23486C, (0x946F, false)), // East Asian ideograph
    (0x295F7C, (0x9F80, false)), // East Asian ideograph
    (0x6F4F54, (0xB975, false)), // Korean hangul
    (0x213F42, (0x61A4, false)), // East Asian ideograph
    (0x453D53, (0x5F66, false)), // East Asian ideograph
    (0x4B4561, (0x691C, false)), // East Asian ideograph
    (0x4B5061, (0x7CBE, false)), // East Asian ideograph
    (0x2D4D5F, (0x7741, false)), // East Asian ideograph
    (0x6F555D, (0xC5E5, false)), // Korean hangul
    (0x27422F, (0x64DE, false)), // East Asian ideograph
    (0x6F5A69, (0xD0AC, false)), // Korean hangul
    (0x213B42, (0x5C0B, false)), // East Asian ideograph
    (0x294666, (0x933E, false)), // East Asian ideograph
    (0x223F45, (0x69A0, false)), // East Asian ideograph
    (0x234340, (0x92EE, false)), // East Asian ideograph
    (0x4B522B, (0x7F36, false)), // East Asian ideograph
    (0x6F5C50, (0xD48D, false)), // Korean hangul
    (0x223F46, (0x69B1, false)), // East Asian ideograph
    (0x23553C, (0x9B08, false)), // East Asian ideograph
    (0x233F47, (0x90FE, false)), // East Asian ideograph
    (0x295031, (0x98A7, false)), // East Asian ideograph
    (0x213F48, (0x61B6, false)), // East Asian ideograph
    (0x6F555E, (0xC5EC, false)), // Korean hangul
    (0x213F49, (0x61CD, false)), // East Asian ideograph
    (0x213B52, (0x5C3F, false)), // East Asian ideograph
    (0x233F4A, (0x90FF, false)), // East Asian ideograph
    (0x6F4A37, (0xAE31, false)), // Korean hangul
    (0x273F4B, (0x601C, false)), // East Asian ideograph
    (0x213F4C, (0x61BE, false)), // East Asian ideograph
    (0x516A26, (0x51B4, false)), // East Asian ideograph
    (0x4D4D61, (0x7EF1, false)), // East Asian ideograph
    (0x6F5B49, (0xD23F, false)), // Korean hangul
    (0x275143, (0x7EFE, false)), // East Asian ideograph
    (0x274231, (0x62E2, false)), // East Asian ideograph
    (0x295940, (0x9CBC, false)), // East Asian ideograph
    (0x213B44, (0x5C0E, false)), // East Asian ideograph
    (0x234A21, (0x9627, false)), // East Asian ideograph
    (0x223F50, (0x69CE, false)), // East Asian ideograph
    (0x22327A, (0x63DC, false)), // East Asian ideograph
    (0x227269, (0x7E10, false)), // East Asian ideograph
    (0x6F5472, (0xC528, false)), // Korean hangul
    (0x4B3F53, (0x61F2, false)), // East Asian ideograph
    (0x4B5671, (0x873B, false)), // East Asian ideograph (variant of 215671 which maps to 873B)
    (0x223F44, (0x698A, false)), // East Asian ideograph
    (0x213F54, (0x61F7, false)), // East Asian ideograph
    (0x234343, (0x927A, false)), // East Asian ideograph
    (0x213F55, (0x61F6, false)), // East Asian ideograph
    (0x27414F, (0x635F, false)), // East Asian ideograph
    (0x22327B, (0x63D7, false)), // East Asian ideograph
    (0x213F56, (0x61F8, false)), // East Asian ideograph
    (0x223F51, (0x69CA, false)), // East Asian ideograph
    (0x233237, (0x8A57, false)), // East Asian ideograph
    (0x213F57, (0x61F5, false)), // East Asian ideograph
    (0x6F5561, (0xC5F0, false)), // Korean hangul
    (0x21355B, (0x5436, false)), // East Asian ideograph
    (0x233F58, (0x9111, false)), // East Asian ideograph
    (0x215D5F, (0x927B, false)), // East Asian ideograph
    (0x224A66, (0x6E62, false)), // East Asian ideograph
    (0x223F59, (0x698D, false)), // East Asian ideograph
    (0x223F5A, (0x6991, false)), // East Asian ideograph
    (0x217C21, (0x5AA6, false)), // East Asian ideograph
    (0x217C22, (0x5A8C, false)), // East Asian ideograph
    (0x213F5B, (0x61FC, false)), // East Asian ideograph
    (0x227C24, (0x828E, false)), // East Asian ideograph
    (0x227C25, (0x8291, false)), // East Asian ideograph
    (0x217C26, (0x5AA2, false)), // East Asian ideograph
    (0x227C27, (0x828F, false)), // East Asian ideograph
    (0x227C28, (0x8284, false)), // East Asian ideograph
    (0x273F5C, (0x604B, false)), // East Asian ideograph
    (0x227C2D, (0x8283, false)), // East Asian ideograph
    (0x227C2E, (0x828A, false)), // East Asian ideograph
    (0x213F5D, (0x6208, false)), // East Asian ideograph
    (0x225138, (0x70CB, false)), // East Asian ideograph
    (0x274C78, (0x762B, false)), // East Asian ideograph
    (0x227C34, (0x82A7, false)), // East Asian ideograph
    (0x217C35, (0x5A95, false)), // East Asian ideograph
    (0x217C36, (0x5AAF, false)), // East Asian ideograph
    (0x227C38, (0x82AB, false)), // East Asian ideograph
    (0x217C39, (0x5AC8, false)), // East Asian ideograph
    (0x227C3A, (0x82B0, false)), // East Asian ideograph
    (0x227C3C, (0x82A4, false)), // East Asian ideograph
    (0x217C3E, (0x5AB5, false)), // East Asian ideograph
    (0x227C3F, (0x829A, false)), // East Asian ideograph
    (0x233F60, (0x910B, false)), // East Asian ideograph
    (0x227C42, (0x82A3, false)), // East Asian ideograph
    (0x6F4E7B, (0xB7ED, false)), // Korean hangul
    (0x227C44, (0x82B7, false)), // East Asian ideograph
    (0x227C45, (0x82AE, false)), // East Asian ideograph (variant of 4C7C45 which maps to 82AE)
    (0x227C46, (0x82A9, false)), // East Asian ideograph
    (0x213F61, (0x620C, false)), // East Asian ideograph
    (0x217C49, (0x5AD1, false)), // East Asian ideograph
    (0x217C4A, (0x5A90, false)), // East Asian ideograph
    (0x6F5563, (0xC5F6, false)), // Korean hangul
    (0x227C4C, (0x82A8, false)), // East Asian ideograph
    (0x213F62, (0x6210, false)), // East Asian ideograph
    (0x227C4E, (0x82B4, false)), // East Asian ideograph
    (0x217C4F, (0x5AB8, false)), // East Asian ideograph
    (0x227C50, (0x82A1, false)), // East Asian ideograph
    (0x226160, (0x770E, false)), // East Asian ideograph
    (0x217C52, (0x5AAA, false)), // East Asian ideograph
    (0x227C53, (0x82AA, false)), // East Asian ideograph
    (0x227C55, (0x82D9, false)), // East Asian ideograph
    (0x227C57, (0x82FE, false)), // East Asian ideograph
    (0x2D3224, (0x7B87, false)), // East Asian ideograph
    (0x217C59, (0x5AD3, false)), // East Asian ideograph
    (0x227C5A, (0x82E0, false)), // East Asian ideograph
    (0x217C5B, (0x5AB1, false)), // East Asian ideograph
    (0x227C5C, (0x8300, false)), // East Asian ideograph
    (0x227C5F, (0x82EA, false)), // East Asian ideograph
    (0x227C60, (0x82F7, false)), // East Asian ideograph
    (0x227C62, (0x82EF, false)), // East Asian ideograph
    (0x227C63, (0x833A, false)), // East Asian ideograph
    (0x227C64, (0x82E4, false)), // East Asian ideograph
    (0x227C65, (0x82D5, false)), // East Asian ideograph
    (0x227C67, (0x8307, false)), // East Asian ideograph
    (0x227C68, (0x82FA, false)), // East Asian ideograph
    (0x227C69, (0x82F4, false)), // East Asian ideograph
    (0x227C6A, (0x82E2, false)), // East Asian ideograph
    (0x213F67, (0x621B, false)), // East Asian ideograph
    (0x227C6D, (0x82D2, false)), // East Asian ideograph
    (0x217C6E, (0x5AE0, false)), // East Asian ideograph
    (0x227C71, (0x82EB, false)), // East Asian ideograph
    (0x227C72, (0x82D8, false)), // East Asian ideograph
    (0x227C73, (0x82E1, false)), // East Asian ideograph
    (0x227C75, (0x82F6, false)), // East Asian ideograph
    (0x28602B, (0x762A, false)), // East Asian ideograph
    (0x227C7B, (0x8310, false)), // East Asian ideograph
    (0x227C7C, (0x82F3, false)), // East Asian ideograph
    (0x233F6A, (0x911E, false)), // East Asian ideograph
    (0x4B4569, (0x6A71, false)), // East Asian ideograph
    (0x23323B, (0x8A58, false)), // East Asian ideograph
    (0x6F5473, (0xC529, false)), // Korean hangul
    (0x274237, (0x631B, false)), // East Asian ideograph
    (0x6F5122, (0xBC41, false)), // Korean hangul
    (0x226162, (0x771B, false)), // East Asian ideograph
    (0x213F6D, (0x622E, false)), // East Asian ideograph
    (0x213F6E, (0x6230, false)), // East Asian ideograph
    (0x4B5973, (0x8D2E, false)), // East Asian ideograph
    (0x275344, (0x80C1, false)), // East Asian ideograph
    (0x213F6F, (0x6232, false)), // East Asian ideograph
    (0x23323C, (0x8A52, false)), // East Asian ideograph
    (0x6F5566, (0xC5FD, false)), // Korean hangul
    (0x21355C, (0x5433, false)), // East Asian ideograph
    (0x274238, (0x644A, false)), // East Asian ideograph
    (0x6F5123, (0xBC43, false)), // Korean hangul
    (0x226163, (0x7724, false)), // East Asian ideograph
    (0x213F72, (0x6236, false)), // East Asian ideograph
    (0x234349, (0x92AA, false)), // East Asian ideograph
    (0x6F4C38, (0xB18D, false)), // Korean hangul
    (0x22557C, (0x728B, false)), // East Asian ideograph
    (0x213F74, (0x623E, false)), // East Asian ideograph
    (0x225057, (0x7098, false)), // East Asian ideograph
    (0x235B2F, (0x9D7A, false)), // East Asian ideograph
    (0x213F75, (0x6240, false)), // East Asian ideograph
    (0x29325D, (0x8BD4, false)), // East Asian ideograph
    (0x2D3F76, (0x78A5, false)), // East Asian ideograph
    (0x6F5A6B, (0xD0B5, false)), // Korean hangul
    (0x6F5124, (0xBC44, false)), // Korean hangul
    (0x213B4C, (0x5C37, false)), // East Asian ideograph
    (0x223F77, (0x69BE, false)), // East Asian ideograph
    (0x4B6A22, (0x7F83, false)), // East Asian ideograph
    (0x213F78, (0x6248, false)), // East Asian ideograph
    (0x222A23, (0x5F82, false)), // East Asian ideograph
    (0x233F79, (0x912B, false)), // East Asian ideograph
    (0x4B456C, (0x6ADB, false)), // East Asian ideograph (variant of 21456C)
    (0x2D602D, (0x976D, false)), // East Asian ideograph
    (0x213F7A, (0x624B, false)), // East Asian ideograph
    (0x212A25, (0xE8D4, false)), // EACC component character
    (0x6F5568, (0xC5FF, false)), // Korean hangul
    (0x294E7B, (0x9883, false)), // East Asian ideograph
    (0x4B6A26, (0x6C8D, false)), // East Asian ideograph
    (0x6F4A39, (0xAE37, false)), // Korean hangul
    (0x2D5A34, (0x8CAD, false)), // East Asian ideograph
    (0x393D6F, (0x8907, false)), // East Asian ideograph
    (0x213F7E, (0x6254, false)), // East Asian ideograph
    (0x6F5A70, (0xD0C0, false)), // Korean hangul
    (0x4B456D, (0x823B, false)), // East Asian ideograph
    (0x2D403F, (0x6255, false)), // East Asian ideograph
    (0x695F70, (0x7195, false)), // East Asian ideograph
    (0x27423B, (0x63FD, false)), // East Asian ideograph
    (0x6F5126, (0xBC84, false)), // Korean hangul
    (0x225731, (0x731D, false)), // East Asian ideograph
    (0x213A7A, (0x5BB5, false)), // East Asian ideograph
    (0x696A2C, (0x87D0, false)), // East Asian ideograph
    (0x2E5A78, (0x74A2, false)), // East Asian ideograph
    (0x277954, (0x5956, false)), // East Asian ideograph
    (0x212A2E, (0xE8DC, false)), // EACC component character
    (0x333240, (0x4FFB, false)), // East Asian ideograph
    (0x232A2F, (0x86FA, false)), // East Asian ideograph
    (0x227D21, (0x830C, false)), // East Asian ideograph
    (0x227D22, (0x82FB, false)), // East Asian ideograph
    (0x227D24, (0x82FD, false)), // East Asian ideograph
    (0x215925, (0x8AE6, false)), // East Asian ideograph
    (0x227D26, (0x8333, false)), // East Asian ideograph
    (0x4B5238, (0x7F87, false)), // East Asian ideograph
    (0x227D29, (0x8328, false)), // East Asian ideograph
    (0x217D2A, (0x5AFD, false)), // East Asian ideograph
    (0x217D2B, (0x5B08, false)), // East Asian ideograph
    (0x212A32, (0xE8DF, false)), // EACC component character
    (0x227D2E, (0x8351, false)), // East Asian ideograph
    (0x214C5C, (0x75F1, false)), // East Asian ideograph
    (0x6F5C7B, (0xD5CC, false)), // Korean hangul
    (0x4B456F, (0x685C, false)), // East Asian ideograph
    (0x227D35, (0x831B, false)), // East Asian ideograph
    (0x217D38, (0x5B03, false)), // East Asian ideograph
    (0x222A34, (0x3013, false)), // East Asian ideograph (not found in unified han)
    (0x227D3B, (0x8356, false)), // East Asian ideograph
    (0x217D3D, (0x5B17, false)), // East Asian ideograph
    (0x217D3E, (0x5B16, false)), // East Asian ideograph
    (0x227D3F, (0x8322, false)), // East Asian ideograph
    (0x227D40, (0x832C, false)), // East Asian ideograph
    (0x212A36, (0xE8E3, false)), // EACC component character
    (0x217D47, (0x5B1B, false)), // East Asian ideograph
    (0x227D48, (0x833C, false)), // East Asian ideograph
    (0x227D4A, (0x834D, false)), // East Asian ideograph
    (0x334F3A, (0x7A49, false)), // East Asian ideograph
    (0x227D4D, (0x8343, false)), // East Asian ideograph (variant of 4C7D4D which maps to 8343)
    (0x22505C, (0x70B7, false)), // East Asian ideograph
    (0x4B4570, (0x6A29, false)), // East Asian ideograph
    (0x227D52, (0x832F, false)), // East Asian ideograph
    (0x227D53, (0x8348, false)), // East Asian ideograph
    (0x227D54, (0x8312, false)), // East Asian ideograph
    (0x227D56, (0x8316, false)), // East Asian ideograph
    (0x212A39, (0xE8E6, false)), // EACC component character
    (0x227D58, (0x831A, false)), // East Asian ideograph
    (0x217D59, (0x5B32, false)), // East Asian ideograph
    (0x2E6F43, (0x9908, false)), // East Asian ideograph
    (0x6F5A6C, (0xD0B7, false)), // Korean hangul
    (0x6F5129, (0xBC8B, false)), // Korean hangul
    (0x213B51, (0x5C41, false)), // East Asian ideograph
    (0x227D5F, (0x8347, false)), // East Asian ideograph
    (0x227D62, (0x83A8, false)), // East Asian ideograph
    (0x217D63, (0x5B3F, false)), // East Asian ideograph
    (0x4B3021, (0x58F1, false)), // East Asian ideograph
    (0x227D67, (0x83AD, false)), // East Asian ideograph
    (0x6F5121, (0xBC40, false)), // Korean hangul
    (0x286A3C, (0x7AAD, false)), // East Asian ideograph
    (0x4C7D6A, (0x8323, false)), // East Asian ideograph
    (0x227D6D, (0x8373, false)), // East Asian ideograph
    (0x217D6E, (0x5B45, false)), // East Asian ideograph
    (0x6F4E7D, (0xB7F4, false)), // Korean hangul
    (0x227D72, (0x83B0, false)), // East Asian ideograph
    (0x217D74, (0x5B4C, false)), // East Asian ideograph
    (0x212A3E, (0xE8EB, false)), // EACC component character
    (0x227D76, (0x831D, false)), // East Asian ideograph
    (0x6F556D, (0xC608, false)), // Korean hangul
    (0x282868, (0x5E91, false)), // East Asian ideograph
    (0x227D7A, (0x838F, false)), // East Asian ideograph
    (0x6F512A, (0xBC8C, false)), // Korean hangul
    (0x227D7C, (0x8395, false)), // East Asian ideograph
    (0x227D7E, (0x8375, false)), // East Asian ideograph
    (0x215928, (0x8AF1, false)), // East Asian ideograph
    (0x234350, (0x92A6, false)), // East Asian ideograph
    (0x51384D, (0x51C3, false)), // East Asian ideograph
    (0x69545C, (0x58D7, false)), // East Asian ideograph
    (0x212A41, (0xE8EE, false)), // EACC component character
    (0x275E61, (0x9614, false)), // East Asian ideograph
    (0x212A46, (0x3013, false)), // Ideographic geta symbol
    (0x212A42, (0xE8EF, false)), // EACC component character
    (0x23545D, (0x9AC8, false)), // East Asian ideograph
    (0x226A43, (0x7ABF, false)), // East Asian ideograph
    (0x6F556E, (0xC60C, false)), // Korean hangul
    (0x6F512B, (0xBC94, false)), // Korean hangul
    (0x22315C, (0x634B, false)), // East Asian ideograph
    (0x212A45, (0xE8F2, false)), // EACC component character
    (0x6F5574, (0xC62C, false)), // Korean hangul
    (0x2D314C, (0x5008, false)), // East Asian ideograph
    (0x27534D, (0x8109, false)), // East Asian ideograph
    (0x273B6E, (0x5188, false)), // East Asian ideograph
    (0x214C60, (0x760D, false)), // East Asian ideograph
    (0x2D4D71, (0x7719, false)), // East Asian ideograph
    (0x213E6A, (0x60DF, false)), // East Asian ideograph
    (0x6F5C21, (0xD33D, false)), // Korean hangul
    (0x2D3C61, (0x5E47, false)), // East Asian ideograph
    (0x234667, (0x93E7, false)), // East Asian ideograph
    (0x6F512C, (0xBC95, false)), // Korean hangul
    (0x21592A, (0x8ADC, false)), // East Asian ideograph
    (0x2D3C65, (0x79CA, false)), // East Asian ideograph
    (0x6F4E29, (0xB550, false)), // Korean hangul
    (0x393E47, (0x8CC9, false)), // East Asian ideograph
    (0x6F5032, (0xBA70, false)), // Korean hangul
    (0x2F386F, (0x8DD7, false)), // East Asian ideograph
    (0x214C61, (0x7627, false)), // East Asian ideograph
    (0x6F2464, (0x314E, false)), // Korean hangul
    (0x6F5B47, (0xD23C, false)), // Korean hangul
    (0x6F512D, (0xBC97, false)), // Korean hangul
    (0x6F516E, (0xBE45, false)), // Korean hangul
    (0x226A4F, (0x7AD1, false)), // East Asian ideograph
    (0x4B3974, (0x5B22, false)), // East Asian ideograph
    (0x3F5564, (0x61DE, false)), // East Asian ideograph
    (0x214C62, (0x7613, false)), // East Asian ideograph
    (0x4B6130, (0x99C4, false)), // East Asian ideograph
    (0x6F5571, (0xC624, false)), // Korean hangul
    (0x6F512E, (0xBC98, false)), // Korean hangul
    (0x284642, (0x6BF5, false)), // East Asian ideograph
    (0x226A54, (0x7AD5, false)), // East Asian ideograph
    (0x2D5A3D, (0x8CDB, false)), // East Asian ideograph
    (0x225C50, (0x74E4, false)), // East Asian ideograph
    (0x225062, (0x70A1, false)), // East Asian ideograph
    (0x335461, (0x6CD6, false)), // East Asian ideograph
    (0x213041, (0x4E4D, false)), // East Asian ideograph
    (0x6F5572, (0xC625, false)), // Korean hangul
    (0x6F512F, (0xBC99, false)), // Korean hangul
    (0x234355, (0x929A, false)), // East Asian ideograph
    (0x6F4A3B, (0xAE40, false)), // Korean hangul
    (0x217E59, (0x5BC1, false)), // East Asian ideograph
    (0x2F2A5A, (0x868B, false)), // Unrelated variant of EACC 23293D which maps to 868B
    (0x22714B, (0x7D9B, false)), // East Asian ideograph
    (0x227E21, (0x837F, false)), // East Asian ideograph
    (0x227E22, (0x8399, false)), // East Asian ideograph
    (0x225063, (0x70A3, false)), // East Asian ideograph
    (0x217E24, (0x5B65, false)), // East Asian ideograph
    (0x227E25, (0x8387, false)), // East Asian ideograph
    (0x227E26, (0x83B9, false)), // East Asian ideograph
    (0x217E27, (0x5C58, false)), // East Asian ideograph (not in Unicode)
    (0x217E28, (0x5B6C, false)), // East Asian ideograph
    (0x217E2A, (0x5B6E, false)), // East Asian ideograph
    (0x227E2B, (0x83A9, false)), // East Asian ideograph
    (0x227E2F, (0x839B, false)), // East Asian ideograph
    (0x217E30, (0x5B7B, false)), // East Asian ideograph
    (0x217E31, (0x5B7C, false)), // East Asian ideograph
    (0x217E32, (0x5B80, false)), // East Asian ideograph
    (0x227E33, (0x83AA, false)), // East Asian ideograph
    (0x217E34, (0x5B84, false)), // East Asian ideograph
    (0x217E35, (0x5B82, false)), // East Asian ideograph (not in Unicode)
    (0x227E37, (0x839C, false)), // East Asian ideograph
    (0x227E38, (0x839F, false)), // East Asian ideograph
    (0x222A5F, (0x5FD1, false)), // East Asian ideograph
    (0x217E40, (0x5B95, false)), // East Asian ideograph
    (0x227E41, (0x83CF, false)), // East Asian ideograph
    (0x227E43, (0x83F9, false)), // East Asian ideograph
    (0x2F445F, (0x941A, false)), // East Asian ideograph
    (0x227E45, (0x8421, false)), // East Asian ideograph
    (0x6F5476, (0xC538, false)), // Korean hangul
    (0x217E49, (0x5BAC, false)), // East Asian ideograph
    (0x6F5131, (0xBCA0, false)), // Korean hangul
    (0x294A44, (0x9655, false)), // East Asian ideograph
    (0x21592F, (0x8AF7, false)), // East Asian ideograph
    (0x227E52, (0x83EA, false)), // East Asian ideograph
    (0x227E53, (0x8413, false)), // East Asian ideograph
    (0x217E55, (0x5BB7, false)), // East Asian ideograph
    (0x227E56, (0x83FC, false)), // East Asian ideograph
    (0x227E57, (0x83F6, false)), // East Asian ideograph
    (0x227E59, (0x8410, false)), // East Asian ideograph
    (0x227E5A, (0x83E1, false)), // East Asian ideograph
    (0x217E5B, (0x3761, false)), // East Asian ideograph (not found in unified han)
    (0x227E60, (0x83C6, false)), // East Asian ideograph
    (0x227E61, (0x8407, false)), // East Asian ideograph
    (0x227E63, (0x83EB, false)), // East Asian ideograph
    (0x216A66, (0x51DF, false)), // East Asian ideograph
    (0x6F5575, (0xC62D, false)), // Korean hangul
    (0x217E68, (0x5BD4, false)), // East Asian ideograph
    (0x217E6A, (0x5BC3, false)), // East Asian ideograph
    (0x227E6B, (0x83E2, false)), // East Asian ideograph
    (0x227E6D, (0x8401, false)), // East Asian ideograph
    (0x217E6E, (0x5BD6, false)), // East Asian ideograph
    (0x234358, (0x92AB, false)), // East Asian ideograph
    (0x227E71, (0x83D8, false)), // East Asian ideograph
    (0x227E72, (0x83E5, false)), // East Asian ideograph
    (0x227E74, (0x8418, false)), // East Asian ideograph
    (0x217E75, (0x5BD7, false)), // East Asian ideograph
    (0x227E79, (0x83CE, false)), // East Asian ideograph
    (0x227E7B, (0x83D3, false)), // East Asian ideograph
    (0x295B52, (0x9E4E, false)), // East Asian ideograph
    (0x227E7D, (0x83D6, false)), // East Asian ideograph
    (0x217E7E, (0x5BEA, false)), // East Asian ideograph
    (0x6F5576, (0xC62E, false)), // Korean hangul
    (0x6F5133, (0xBCA4, false)), // Korean hangul
    (0x294A46, (0x9649, false)), // East Asian ideograph
    (0x275F3D, (0x96B6, false)), // East Asian ideograph
    (0x4B6260, (0x9EBD, false)), // East Asian ideograph
    (0x227E6A, (0x83BF, false)), // East Asian ideograph
    (0x2D6030, (0x97EE, false)), // East Asian ideograph
    (0x235466, (0x9AD0, false)), // East Asian ideograph
    (0x22454D, (0x6996, false)), // East Asian ideograph
    (0x6F5577, (0xC633, false)), // Korean hangul
    (0x6F5134, (0xBCA7, false)), // Korean hangul
    (0x213B5C, (0x5C50, false)), // East Asian ideograph
    (0x215932, (0x8B19, false)), // East Asian ideograph
    (0x6F4A3C, (0xAE41, false)), // Korean hangul
    (0x2D3C6D, (0x8298, false)), // East Asian ideograph (duplicate simplified)
    (0x222A73, (0x5FF8, false)), // East Asian ideograph
    (0x225068, (0x7551, false)), // East Asian ideograph
    (0x6F5551, (0xC5C8, false)), // Korean hangul
    (0x6F5135, (0xBCA8, false)), // Korean hangul
    (0x215521, (0x5179, false)), // East Asian ideograph
    (0x223F5C, (0x69AA, false)), // East Asian ideograph
    (0x275274, (0x58F0, false)), // East Asian ideograph
    (0x2D3C6E, (0x7240, false)), // East Asian ideograph
    (0x6F5523, (0xC54C, false)), // Korean hangul
    (0x6F5524, (0xC54E, false)), // Korean hangul
    (0x23324F, (0x8A7F, false)), // East Asian ideograph
    (0x233E37, (0x9088, false)), // East Asian ideograph
    (0x4C4446, (0x6B4E, false)), // East Asian ideograph
    (0x6F5525, (0xC553, false)), // Korean hangul
    (0x232A7B, (0x877B, false)), // East Asian ideograph
    (0x6F5526, (0xC554, false)), // Korean hangul
    (0x3A6A7C, (0x7BEA, false)), // East Asian ideograph
    (0x235527, (0x9AEB, false)), // East Asian ideograph
    (0x22763D, (0x801E, false)), // East Asian ideograph
    (0x235528, (0x9AF2, false)), // East Asian ideograph
    (0x215529, (0x8396, false)), // East Asian ideograph
    (0x233250, (0x8A86, false)), // East Asian ideograph
    (0x225424, (0x71C1, false)), // East Asian ideograph
    (0x21552A, (0x83A7, false)), // East Asian ideograph
    (0x6F5137, (0xBCB1, false)), // Korean hangul
    (0x213B5F, (0x5C5C, false)), // East Asian ideograph
    (0x6F552B, (0xC55E, false)), // Korean hangul
    (0x295F7B, (0x9F51, false)), // East Asian ideograph
    (0x2D3C70, (0x576B, false)), // East Asian ideograph
    (0x27552D, (0x5E84, false)), // East Asian ideograph
    (0x2D552E, (0x82FA, false)), // East Asian ideograph (variant of 227C68)
    (0x23316E, (0x8A04, false)), // East Asian ideograph
    (0x2D493A, (0x702C, false)), // East Asian ideograph
    (0x215D79, (0x9375, false)), // East Asian ideograph
    (0x28464C, (0x6BE1, false)), // East Asian ideograph
    (0x213B60, (0x5C62, false)), // East Asian ideograph
    (0x6F5531, (0xC570, false)), // Korean hangul
    (0x293726, (0x8D5C, false)), // East Asian ideograph
    (0x235532, (0x9AF9, false)), // East Asian ideograph
    (0x6F5533, (0xC573, false)), // Korean hangul
    (0x6F5534, (0xC574, false)), // Korean hangul
    (0x2F3363, (0x8B1A, false)), // East Asian ideograph
    (0x6F5139, (0xBCB5, false)), // Korean hangul
    (0x235535, (0x9AFD, false)), // East Asian ideograph
    (0x4B3474, (0x537F, false)), // East Asian ideograph (variant of 213474 which maps to 537F)
    (0x6F582F, (0xC9D0, false)), // Korean hangul
    (0x22385A, (0x6678, false)), // East Asian ideograph
    (0x235536, (0x9B01, false)), // East Asian ideograph
    (0x2D5A48, (0x8D71, false)), // East Asian ideograph
    (0x295B59, (0x9E5C, false)), // East Asian ideograph
    (0x235538, (0x9B02, false)), // East Asian ideograph
    (0x6F5539, (0xC584, false)), // Korean hangul
    (0x6F513A, (0xBCBC, false)), // Korean hangul
    (0x4B553A, (0x83C1, false)), // East Asian ideograph (variant of 21553A which maps to 83C1)
    (0x23553B, (0x9B00, false)), // East Asian ideograph
    (0x2D3830, (0x573B, false)), // East Asian ideograph
    (0x27553C, (0x534E, false)), // East Asian ideograph
    (0x283542, (0x64B7, false)), // East Asian ideograph
    (0x287531, (0x7F9F, false)), // East Asian ideograph
    (0x33502A, (0x9257, false)), // East Asian ideograph
    (0x23553E, (0x9B04, false)), // East Asian ideograph
    (0x6F513B, (0xBCBD, false)), // Korean hangul
    (0x213B63, (0x5C6C, false)), // East Asian ideograph
    (0x22565B, (0x72C1, false)), // East Asian ideograph
    (0x275947, (0x8C34, false)), // East Asian ideograph
    (0x223442, (0x648F, false)), // East Asian ideograph
    (0x213E3F, (0x6062, false)), // East Asian ideograph
    (0x3F4472, (0x7881, false)), // East Asian ideograph
    (0x215541, (0x840A, false)), // East Asian ideograph
    (0x4B5542, (0x8420, false)), // East Asian ideograph
    (0x33502B, (0x724B, false)), // East Asian ideograph
    (0x235543, (0x9B0B, false)), // East Asian ideograph
    (0x6F4B2B, (0xAFCE, false)), // Korean hangul
    (0x2E4D3D, (0x6D38, false)), // East Asian ideograph
    (0x696A5E, (0x88B0, false)), // East Asian ideograph
    (0x227431, (0x7F4E, false)), // East Asian ideograph
    (0x213561, (0x543B, false)), // East Asian ideograph
    (0x225D39, (0x7517, false)), // East Asian ideograph
    (0x6F5545, (0xC5B5, false)), // Korean hangul
    (0x6F5546, (0xC5B6, false)), // Korean hangul
    (0x6F4F5B, (0xB98E, false)), // Korean hangul
    (0x295B5C, (0x9E5B, false)), // East Asian ideograph
    (0x225070, (0x79CC, false)), // East Asian ideograph
    (0x235547, (0x9B0E, false)), // East Asian ideograph
    (0x233256, (0x8A61, false)), // East Asian ideograph
    (0x6F5548, (0xC5B9, false)), // Korean hangul
    (0x274252, (0x654C, false)), // East Asian ideograph
    (0x6F513D, (0xBCC4, false)), // Korean hangul
    (0x284651, (0x6C07, false)), // East Asian ideograph
    (0x6F5549, (0xC5BA, false)), // Korean hangul
    (0x213722, (0x55C6, false)), // East Asian ideograph
    (0x233C2D, (0x8F40, false)), // East Asian ideograph
    (0x6F554A, (0xC5BB, false)), // Korean hangul
    (0x6F554B, (0xC5BC, false)), // Korean hangul
    (0x28575E, (0x72B8, false)), // East Asian ideograph
    (0x227849, (0x811D, false)), // East Asian ideograph
    (0x393944, (0x59B3, false)), // East Asian ideograph
    (0x6F554C, (0xC5BD, false)), // Korean hangul
    (0x355E76, (0x82BE, false)), // East Asian ideograph
    (0x27554D, (0x82C7, false)), // East Asian ideograph
    (0x6F513E, (0xBCCC, false)), // Korean hangul
    (0x23554E, (0x9B11, false)), // East Asian ideograph
    (0x223F65, (0x699E, false)), // East Asian ideograph
    (0x4B5959, (0x8273, false)), // East Asian ideograph
    (0x21554F, (0x8449, false)), // East Asian ideograph
    (0x2D5550, (0x585F, false)), // East Asian ideograph
    (0x28575F, (0x72F2, false)), // East Asian ideograph
    (0x283546, (0x6445, false)), // East Asian ideograph
    (0x225072, (0x70BF, false)), // East Asian ideograph
    (0x215551, (0x846B, false)), // East Asian ideograph
    (0x233258, (0x8A3E, false)), // East Asian ideograph
    (0x225552, (0x7253, false)), // East Asian ideograph
    (0x274254, (0x6570, false)), // East Asian ideograph
    (0x6F513F, (0xBCCD, false)), // Korean hangul
    (0x225553, (0x7255, false)), // East Asian ideograph
    (0x275276, (0x806A, false)), // East Asian ideograph
    (0x6F7726, (0xADB9, false)), // Korean hangul
    (0x235554, (0x9B18, false)), // East Asian ideograph
    (0x6F585A, (0xCAC4, false)), // Korean hangul
    (0x333F22, (0x6168, false)), // East Asian ideograph (variant of 213F22 which maps to 6168)
    (0x275555, (0x83B4, false)), // East Asian ideograph
];

pub const SUPERSCRIPTS: [(u32, (u16, bool)); 14] = [
    (0x28, (0x207D, false)), // SUPERSCRIPT OPENING PARENTHESIS / SUPERSCRIPT LEFT PARENTHESIS
    (0x29, (0x207E, false)), // SUPERSCRIPT CLOSING PARENTHESIS / SUPERSCRIPT RIGHT PARENTHESIS
    (0x2B, (0x207A, false)), // SUPERSCRIPT PLUS SIGN
    (0x2D, (0x207B, false)), // SUPERSCRIPT HYPHEN-MINUS / SUPERSCRIPT MINUS
    (0x30, (0x2070, false)), // SUPERSCRIPT DIGIT ZERO
    (0x31, (0xB9, false)),   // SUPERSCRIPT DIGIT ONE
    (0x32, (0xB2, false)),   // SUPERSCRIPT DIGIT TWO
    (0x33, (0xB3, false)),   // SUPERSCRIPT DIGIT THREE
    (0x34, (0x2074, false)), // SUPERSCRIPT DIGIT FOUR
    (0x35, (0x2075, false)), // SUPERSCRIPT DIGIT FIVE
    (0x36, (0x2076, false)), // SUPERSCRIPT DIGIT SIX
    (0x37, (0x2077, false)), // SUPERSCRIPT DIGIT SEVEN
    (0x38, (0x2078, false)), // SUPERSCRIPT DIGIT EIGHT
    (0x39, (0x2079, false)), // SUPERSCRIPT DIGIT NINE
];

pub const EXTENDED_CYRILLIC: [(u32, (u16, bool)); 42] = [
    (0xC0, (0x491, false)), // LOWERCASE GE WITH UPTURN / CYRILLIC SMALL LETTER GHE WITH UPTURN
    (0xC1, (0x452, false)), // LOWERCASE DJE / CYRILLIC SMALL LETTER DJE (Serbian)
    (0xC2, (0x453, false)), // CYRILLIC SMALL LETTER GJE
    (0xC3, (0x454, false)), // LOWERCASE E / CYRILLIC SMALL LETTER UKRAINIAN IE
    (0xC4, (0x451, false)), // CYRILLIC SMALL LETTER IO
    (0xC5, (0x455, false)), // CYRILLIC SMALL LETTER DZE
    (0xC6, (0x456, false)), // LOWERCASE I / CYRILLIC SMALL LETTER BYELORUSSIAN-UKRANIAN I
    (0xC7, (0x457, false)), // LOWERCASE YI / CYRILLIC SMALL LETTER YI (Ukrainian)
    (0xC8, (0x458, false)), // CYRILLIC SMALL LETTER JE
    (0xC9, (0x459, false)), // CYRILLIC SMALL LETTER LJE
    (0xCA, (0x45A, false)), // CYRILLIC SMALL LETTER NJE
    (0xCB, (0x45B, false)), // LOWERCASE TSHE / CYRILLIC SMALL LETTER TSHE (Serbian)
    (0xCC, (0x45C, false)), // CYRILLIC SMALL LETTER KJE
    (0xCD, (0x45E, false)), // LOWERCASE SHORT U / CYRILLIC SMALL LETTER SHORT U (Byelorussian)
    (0xCE, (0x45F, false)), // CYRILLIC SMALL LETTER DZHE
    (0xD0, (0x463, false)), // CYRILLIC SMALL LETTER YAT
    (0xD1, (0x473, false)), // CYRILLIC SMALL LETTER FITA
    (0xD2, (0x475, false)), // CYRILLIC SMALL LETTER IZHITSA
    (0xD3, (0x46B, false)), // CYRILLIC SMALL LETTER BIG YUS
    (0xDB, (0x5B, false)),  // OPENING SQUARE BRACKET / LEFT SQUARE BRACKET
    (0xDD, (0x5D, false)),  // CLOSING SQUARE BRACKET / RIGHT SQUARE BRACKET
    (0xDF, (0x5F, false)),  // SPACING UNDERSCORE / LOW LINE
    (0xE0, (0x490, false)), // UPPERCASE GE WITH UPTURN / CYRILLIC CAPITAL LETTER GHE WITH UPTURN
    (0xE1, (0x402, false)), // UPPERCASE DJE / CYRILLIC CAPITAL LETTER DJE (Serbian)
    (0xE2, (0x403, false)), // CYRILLIC CAPITAL LETTER GJE
    (0xE3, (0x404, false)), // UPPERCASE E / CYRILLIC CAPITAL LETTER UKRAINIAN IE
    (0xE4, (0x401, false)), // CYRILLIC CAPITAL LETTER IO
    (0xE5, (0x405, false)), // CYRILLIC CAPITAL LETTER DZE
    (0xE6, (0x406, false)), // UPPERCASE I / CYRILLIC CAPITAL LETTER BYELORUSSIAN-UKRANIAN I
    (0xE7, (0x407, false)), // UPPERCASE YI / CYRILLIC CAPITAL LETTER YI (Ukrainian)
    (0xE8, (0x408, false)), // CYRILLIC CAPITAL LETTER JE
    (0xE9, (0x409, false)), // CYRILLIC CAPITAL LETTER LJE
    (0xEA, (0x40A, false)), // CYRILLIC CAPITAL LETTER NJE
    (0xEB, (0x40B, false)), // UPPERCASE TSHE / CYRILLIC CAPITAL LETTER TSHE (Serbian)
    (0xEC, (0x40C, false)), // CYRILLIC CAPITAL LETTER KJE
    (0xED, (0x40E, false)), // UPPERCASE SHORT U / CYRILLIC CAPITAL LETTER SHORT U (Byelorussian)
    (0xEE, (0x40F, false)), // CYRILLIC CAPITAL LETTER DZHE
    (0xEF, (0x42A, false)), // CYRILLIC CAPITAL LETTER HARD SIGN
    (0xF0, (0x462, false)), // CYRILLIC CAPITAL LETTER YAT
    (0xF1, (0x472, false)), // CYRILLIC CAPITAL LETTER FITA
    (0xF2, (0x474, false)), // CYRILLIC CAPITAL LETTER IZHITSA
    (0xF3, (0x46A, false)), // CYRILLIC CAPITAL LETTER BIG YUS
];

pub const BASIC_GREEK: [(u32, (u16, bool)); 73] = [
    (0x21, (0x300, true)),   // COMBINING GRAVE ACCENT
    (0x22, (0x301, true)),   // COMBINING ACUTE ACCENT
    (0x23, (0x308, true)),   // COMBINING DIAERESIS
    (0x24, (0x342, true)),   // COMBINING GREEK PERISPOMENI / CIRCUMFLEX
    (0x25, (0x313, true)),   // COMBINING COMMA ABOVE / SMOOTH BREATHING
    (0x26, (0x314, true)),   // COMBINING REVERSED COMMA ABOVE / ROUGH BREATHING
    (0x27, (0x345, true)),   // COMBINING GREEK YPOGEGRAMMENI / IOTA SUBSCRIPT
    (0x30, (0xAB, false)),   // LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
    (0x31, (0xBB, false)),   // RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
    (0x32, (0x201C, false)), // LEFT DOUBLE QUOTATION MARK
    (0x33, (0x201D, false)), // RIGHT DOUBLE QUOTATION MARK
    (0x34, (0x374, false)),  // GREEK NUMERAL SIGN / UPPER PRIME
    (0x35, (0x375, false)),  // GREEK LOWER NUMERAL SIGN / LOWER PRIME
    (0x3B, (0x387, false)),  // GREEK ANO TELEIA / RAISED DOT, GREEK SEMICOLON
    (0x3F, (0x37E, false)),  // GREEK QUESTION MARK
    (0x41, (0x391, false)),  // GREEK CAPITAL LETTER ALPHA
    (0x42, (0x392, false)),  // GREEK CAPITAL LETTER BETA
    (0x44, (0x393, false)),  // GREEK CAPITAL LETTER GAMMA
    (0x45, (0x394, false)),  // GREEK CAPITAL LETTER DELTA
    (0x46, (0x395, false)),  // GREEK CAPITAL LETTER EPSILON
    (0x47, (0x3DA, false)),  // GREEK LETTER STIGMA
    (0x48, (0x3DC, false)),  // GREEK LETTER DIGAMMA
    (0x49, (0x396, false)),  // GREEK CAPITAL LETTER ZETA
    (0x4A, (0x397, false)),  // GREEK CAPITAL LETTER ETA
    (0x4B, (0x398, false)),  // GREEK CAPITAL LETTER THETA
    (0x4C, (0x399, false)),  // GREEK CAPITAL LETTER IOTA
    (0x4D, (0x39A, false)),  // GREEK CAPITAL LETTER KAPPA
    (0x4E, (0x39B, false)),  // GREEK CAPITAL LETTER LAMDA
    (0x4F, (0x39C, false)),  // GREEK CAPITAL LETTER MU
    (0x50, (0x39D, false)),  // GREEK CAPITAL LETTER NU
    (0x51, (0x39E, false)),  // GREEK CAPITAL LETTER XI
    (0x52, (0x39F, false)),  // GREEK CAPITAL LETTER OMICRON
    (0x53, (0x3A0, false)),  // GREEK CAPITAL LETTER PI
    (0x54, (0x3DE, false)),  // GREEK LETTER KOPPA
    (0x55, (0x3A1, false)),  // GREEK CAPITAL LETTER RHO
    (0x56, (0x3A3, false)),  // GREEK CAPITAL LETTER SIGMA
    (0x58, (0x3A4, false)),  // GREEK CAPITAL LETTER TAU
    (0x59, (0x3A5, false)),  // GREEK CAPITAL LETTER UPSILON
    (0x5A, (0x3A6, false)),  // GREEK CAPITAL LETTER PHI
    (0x5B, (0x3A7, false)),  // GREEK CAPITAL LETTER CHI
    (0x5C, (0x3A8, false)),  // GREEK CAPITAL LETTER PSI
    (0x5D, (0x3A9, false)),  // GREEK CAPITAL LETTER OMEGA
    (0x5E, (0x3E0, false)),  // GREEK LETTER SAMPI
    (0x61, (0x3B1, false)),  // GREEK SMALL LETTER ALPHA
    (0x62, (0x3B2, false)),  // GREEK SMALL LETTER BETA / SMALL LETTER BETA BEGINNING OF WORD
    (0x63, (0x3D0, false)),  // GREEK BETA SYMBOL / SMALL LETTER BETA MIDDLE OF WORD
    (0x64, (0x3B3, false)),  // GREEK SMALL LETTER GAMMA
    (0x65, (0x3B4, false)),  // GREEK SMALL LETTER DELTA
    (0x66, (0x3B5, false)),  // GREEK SMALL LETTER EPSILON
    (0x67, (0x3DB, false)),  // GREEK SMALL LETTER STIGMA
    (0x68, (0x3DD, false)),  // GREEK SMALL LETTER DIGAMMA
    (0x69, (0x3B6, false)),  // GREEK SMALL LETTER ZETA
    (0x6A, (0x3B7, false)),  // GREEK SMALL LETTER ETA
    (0x6B, (0x3B8, false)),  // GREEK SMALL LETTER THETA
    (0x6C, (0x3B9, false)),  // GREEK SMALL LETTER IOTA
    (0x6D, (0x3BA, false)),  // GREEK SMALL LETTER KAPPA
    (0x6E, (0x3BB, false)),  // GREEK SMALL LETTER LAMDA
    (0x6F, (0x3BC, false)),  // GREEK SMALL LETTER MU
    (0x70, (0x3BD, false)),  // GREEK SMALL LETTER NU
    (0x71, (0x3BE, false)),  // GREEK SMALL LETTER XI
    (0x72, (0x3BF, false)),  // GREEK SMALL LETTER OMICRON
    (0x73, (0x3C0, false)),  // GREEK SMALL LETTER PI
    (0x74, (0x3DF, false)),  // GREEK SMALL LETTER KOPPA
    (0x75, (0x3C1, false)),  // GREEK SMALL LETTER RHO
    (0x76, (0x3C3, false)),  // GREEK SMALL LETTER SIGMA
    (0x77, (0x3C2, false)),  // GREEK SMALL LETTER FINAL SIGMA / SMALL LETTER SIGMA END OF WORD
    (0x78, (0x3C4, false)),  // GREEK SMALL LETTER TAU
    (0x79, (0x3C5, false)),  // GREEK SMALL LETTER UPSILON
    (0x7A, (0x3C6, false)),  // GREEK SMALL LETTER PHI
    (0x7B, (0x3C7, false)),  // GREEK SMALL LETTER CHI
    (0x7C, (0x3C8, false)),  // GREEK SMALL LETTER PSI
    (0x7D, (0x3C9, false)),  // GREEK SMALL LETTER OMEGA
    (0x7E, (0x3E1, false)),  // GREEK SMALL LETTER SAMPI
];

pub const ASCII: [(u32, (u16, bool)); 99] = [
    (0x1B, (0x1B, false)), // ESCAPE (Unlikely to occur in UCS/Unicode)
    (0x1D, (0x1D, false)), // RECORD TERMINATOR / GROUP SEPARATOR
    (0x1E, (0x1E, false)), // FIELD TERMINATOR / RECORD SEPARATOR
    (0x1F, (0x1F, false)), // SUBFIELD DELIMITER / UNIT SEPARATOR
    (0x20, (0x20, false)), // SPACE, BLANK / SPACE
    (0x21, (0x21, false)), // EXCLAMATION MARK
    (0x22, (0x22, false)), // QUOTATION MARK
    (0x23, (0x23, false)), // NUMBER SIGN
    (0x24, (0x24, false)), // DOLLAR SIGN
    (0x25, (0x25, false)), // PERCENT SIGN
    (0x26, (0x26, false)), // AMPERSAND
    (0x27, (0x27, false)), // APOSTROPHE
    (0x28, (0x28, false)), // OPENING PARENTHESIS / LEFT PARENTHESIS
    (0x29, (0x29, false)), // CLOSING PARENTHESIS / CLOSING PARENTHESIS
    (0x2A, (0x2A, false)), // ASTERISK
    (0x2B, (0x2B, false)), // PLUS SIGN
    (0x2C, (0x2C, false)), // COMMA
    (0x2D, (0x2D, false)), // HYPHEN-MINUS
    (0x2E, (0x2E, false)), // PERIOD, DECIMAL POINT / FULL STOP
    (0x2F, (0x2F, false)), // SLASH / SOLIDUS
    (0x30, (0x30, false)), // DIGIT ZERO
    (0x31, (0x31, false)), // DIGIT ONE
    (0x32, (0x32, false)), // DIGIT TWO
    (0x33, (0x33, false)), // DIGIT THREE
    (0x34, (0x34, false)), // DIGIT FOUR
    (0x35, (0x35, false)), // DIGIT FIVE
    (0x36, (0x36, false)), // DIGIT SIX
    (0x37, (0x37, false)), // DIGIT SEVEN
    (0x38, (0x38, false)), // DIGIT EIGHT
    (0x39, (0x39, false)), // DIGIT NINE
    (0x3A, (0x3A, false)), // COLON
    (0x3B, (0x3B, false)), // SEMICOLON
    (0x3C, (0x3C, false)), // LESS-THAN SIGN
    (0x3D, (0x3D, false)), // EQUALS SIGN
    (0x3E, (0x3E, false)), // GREATER-THAN SIGN
    (0x3F, (0x3F, false)), // QUESTION MARK
    (0x40, (0x40, false)), // COMMERCIAL AT
    (0x41, (0x41, false)), // LATIN CAPITAL LETTER A
    (0x42, (0x42, false)), // LATIN CAPITAL LETTER B
    (0x43, (0x43, false)), // LATIN CAPITAL LETTER C
    (0x44, (0x44, false)), // LATIN CAPITAL LETTER D
    (0x45, (0x45, false)), // LATIN CAPITAL LETTER E
    (0x46, (0x46, false)), // LATIN CAPITAL LETTER F
    (0x47, (0x47, false)), // LATIN CAPITAL LETTER G
    (0x48, (0x48, false)), // LATIN CAPITAL LETTER H
    (0x49, (0x49, false)), // LATIN CAPITAL LETTER I
    (0x4A, (0x4A, false)), // LATIN CAPITAL LETTER J
    (0x4B, (0x4B, false)), // LATIN CAPITAL LETTER K
    (0x4C, (0x4C, false)), // LATIN CAPITAL LETTER L
    (0x4D, (0x4D, false)), // LATIN CAPITAL LETTER M
    (0x4E, (0x4E, false)), // LATIN CAPITAL LETTER N
    (0x4F, (0x4F, false)), // LATIN CAPITAL LETTER O
    (0x50, (0x50, false)), // LATIN CAPITAL LETTER P
    (0x51, (0x51, false)), // LATIN CAPITAL LETTER Q
    (0x52, (0x52, false)), // LATIN CAPITAL LETTER R
    (0x53, (0x53, false)), // LATIN CAPITAL LETTER S
    (0x54, (0x54, false)), // LATIN CAPITAL LETTER T
    (0x55, (0x55, false)), // LATIN CAPITAL LETTER U
    (0x56, (0x56, false)), // LATIN CAPITAL LETTER V
    (0x57, (0x57, false)), // LATIN CAPITAL LETTER W
    (0x58, (0x58, false)), // LATIN CAPITAL LETTER X
    (0x59, (0x59, false)), // LATIN CAPITAL LETTER Y
    (0x5A, (0x5A, false)), // LATIN CAPITAL LETTER Z
    (0x5B, (0x5B, false)), // OPENING SQUARE BRACKET / LEFT SQUARE BRACKET
    (0x5C, (0x5C, false)), // REVERSE SLASH / REVERSE SOLIDUS
    (0x5D, (0x5D, false)), // CLOSING SQUARE BRACKET / RIGHT SQUARE BRACKET
    (0x5E, (0x5E, false)), // SPACING CIRCUMFLEX / CIRCUMFLEX ACCENT
    (0x5F, (0x5F, false)), // SPACING UNDERSCORE / LOW LINE
    (0x60, (0x60, false)), // SPACING GRAVE / GRAVE ACCENT
    (0x61, (0x61, false)), // LATIN SMALL LETTER A
    (0x62, (0x62, false)), // LATIN SMALL LETTER B
    (0x63, (0x63, false)), // LATIN SMALL LETTER C
    (0x64, (0x64, false)), // LATIN SMALL LETTER D
    (0x65, (0x65, false)), // LATIN SMALL LETTER E
    (0x66, (0x66, false)), // LATIN SMALL LETTER F
    (0x67, (0x67, false)), // LATIN SMALL LETTER G
    (0x68, (0x68, false)), // LATIN SMALL LETTER H
    (0x69, (0x69, false)), // LATIN SMALL LETTER I
    (0x6A, (0x6A, false)), // LATIN SMALL LETTER J
    (0x6B, (0x6B, false)), // LATIN SMALL LETTER K
    (0x6C, (0x6C, false)), // LATIN SMALL LETTER L
    (0x6D, (0x6D, false)), // LATIN SMALL LETTER M
    (0x6E, (0x6E, false)), // LATIN SMALL LETTER N
    (0x6F, (0x6F, false)), // LATIN SMALL LETTER O
    (0x70, (0x70, false)), // LATIN SMALL LETTER P
    (0x71, (0x71, false)), // LATIN SMALL LETTER Q
    (0x72, (0x72, false)), // LATIN SMALL LETTER R
    (0x73, (0x73, false)), // LATIN SMALL LETTER S
    (0x74, (0x74, false)), // LATIN SMALL LETTER T
    (0x75, (0x75, false)), // LATIN SMALL LETTER U
    (0x76, (0x76, false)), // LATIN SMALL LETTER V
    (0x77, (0x77, false)), // LATIN SMALL LETTER W
    (0x78, (0x78, false)), // LATIN SMALL LETTER X
    (0x79, (0x79, false)), // LATIN SMALL LETTER Y
    (0x7A, (0x7A, false)), // LATIN SMALL LETTER Z
    (0x7B, (0x7B, false)), // OPENING CURLY BRACKET / LEFT CURLY BRACKET
    (0x7C, (0x7C, false)), // VERTICAL BAR (FILL) / VERTICAL LINE
    (0x7D, (0x7D, false)), // CLOSING CURLY BRACKET / RIGHT CURLY BRACKET
    (0x7E, (0x7E, false)), // SPACING TILDE / TILDE
];

pub const SUBSCRIPTS: [(u32, (u16, bool)); 14] = [
    // Subscripts
    (0x28, (0x208D, false)), // SUBSCRIPT OPENING PARENTHESIS / SUBSCRIPT LEFT PARENTHESIS
    (0x29, (0x208E, false)), // SUBSCRIPT CLOSING PARENTHESIS / SUBSCRIPT RIGHT PARENTHESIS
    (0x2B, (0x208A, false)), // SUBSCRIPT PLUS SIGN
    (0x2D, (0x208B, false)), // SUBSCRIPT HYPHEN-MINUS / SUBSCRIPT MINUS
    (0x30, (0x2080, false)), // SUBSCRIPT DIGIT ZERO
    (0x31, (0x2081, false)), // SUBSCRIPT DIGIT ONE
    (0x32, (0x2082, false)), // SUBSCRIPT DIGIT TWO
    (0x33, (0x2083, false)), // SUBSCRIPT DIGIT THREE
    (0x34, (0x2084, false)), // SUBSCRIPT DIGIT FOUR
    (0x35, (0x2085, false)), // SUBSCRIPT DIGIT FIVE
    (0x36, (0x2086, false)), // SUBSCRIPT DIGIT SIX
    (0x37, (0x2087, false)), // SUBSCRIPT DIGIT SEVEN
    (0x38, (0x2088, false)), // SUBSCRIPT DIGIT EIGHT
    (0x39, (0x2089, false)), // SUBSCRIPT DIGIT NINE
];

pub const GREEK_SYMBOLS: [(u32, (u16, bool)); 3] = [
    (0x61, (0x3B1, false)), // GREEK SMALL LETTER ALPHA
    (0x62, (0x3B2, false)), // GREEK SMALL LETTER BETA
    (0x63, (0x3B3, false)), // GREEK SMALL LETTER GAMMA
];

pub const BASIC_CYRILLIC: [(u32, (u16, bool)); 94] = [
    (0x21, (0x21, false)),  // EXCLAMATION MARK
    (0x22, (0x22, false)),  // QUOTATION MARK
    (0x23, (0x23, false)),  // NUMBER SIGN
    (0x24, (0x24, false)),  // DOLLAR SIGN
    (0x25, (0x25, false)),  // PERCENT SIGN
    (0x26, (0x26, false)),  // AMPERSAND
    (0x27, (0x27, false)),  // APOSTROPHE
    (0x28, (0x28, false)),  // OPENING PARENTHESIS / LEFT PARENTHESIS
    (0x29, (0x29, false)),  // CLOSING PARENTHESIS / RIGHT PARENTHESIS
    (0x2A, (0x2A, false)),  // ASTERISK
    (0x2B, (0x2B, false)),  // PLUS SIGN
    (0x2C, (0x2C, false)),  // COMMA
    (0x2D, (0x2D, false)),  // HYPHEN-MINUS
    (0x2E, (0x2E, false)),  // PERIOD, DECIMAL POINT / FULL STOP
    (0x2F, (0x2F, false)),  // SLASH / SOLIDUS
    (0x30, (0x30, false)),  // DIGIT ZERO
    (0x31, (0x31, false)),  // DIGIT ONE
    (0x32, (0x32, false)),  // DIGIT TWO
    (0x33, (0x33, false)),  // DIGIT THREE
    (0x34, (0x34, false)),  // DIGIT FOUR
    (0x35, (0x35, false)),  // DIGIT FIVE
    (0x36, (0x36, false)),  // DIGIT SIX
    (0x37, (0x37, false)),  // DIGIT SEVEN
    (0x38, (0x38, false)),  // DIGIT EIGHT
    (0x39, (0x39, false)),  // DIGIT NINE
    (0x3A, (0x3A, false)),  // COLON
    (0x3B, (0x3B, false)),  // SEMICOLON
    (0x3C, (0x3C, false)),  // LESS-THAN SIGN
    (0x3D, (0x3D, false)),  // EQUALS SIGN
    (0x3E, (0x3E, false)),  // GREATER-THAN SIGN
    (0x3F, (0x3F, false)),  // QUESTION MARK
    (0x40, (0x44E, false)), // LOWERCASE IU / CYRILLIC SMALL LETTER YU
    (0x41, (0x430, false)), // CYRILLIC SMALL LETTER A
    (0x42, (0x431, false)), // CYRILLIC SMALL LETTER BE
    (0x43, (0x446, false)), // CYRILLIC SMALL LETTER TSE
    (0x44, (0x434, false)), // CYRILLIC SMALL LETTER DE
    (0x45, (0x435, false)), // CYRILLIC SMALL LETTER IE
    (0x46, (0x444, false)), // CYRILLIC SMALL LETTER EF
    (0x47, (0x433, false)), // LOWERCASE GE / CYRILLIC SMALL LETTER GHE
    (0x48, (0x445, false)), // LOWERCASE KHA / CYRILLIC SMALL LETTER HA
    (0x49, (0x438, false)), // LOWERCASE II / CYRILLIC SMALL LETTER I
    (0x4A, (0x439, false)), // LOWERCASE SHORT II / CYRILLIC SMALL LETTER SHORT I
    (0x4B, (0x43A, false)), // CYRILLIC SMALL LETTER KA
    (0x4C, (0x43B, false)), // CYRILLIC SMALL LETTER EL
    (0x4D, (0x43C, false)), // CYRILLIC SMALL LETTER EM
    (0x4E, (0x43D, false)), // CYRILLIC SMALL LETTER EN
    (0x4F, (0x43E, false)), // CYRILLIC SMALL LETTER O
    (0x50, (0x43F, false)), // CYRILLIC SMALL LETTER PE
    (0x51, (0x44F, false)), // LOWERCASE IA / CYRILLIC SMALL LETTER YA
    (0x52, (0x440, false)), // CYRILLIC SMALL LETTER ER
    (0x53, (0x441, false)), // CYRILLIC SMALL LETTER ES
    (0x54, (0x442, false)), // CYRILLIC SMALL LETTER TE
    (0x55, (0x443, false)), // CYRILLIC SMALL LETTER U
    (0x56, (0x436, false)), // CYRILLIC SMALL LETTER ZHE
    (0x57, (0x432, false)), // CYRILLIC SMALL LETTER VE
    (0x58, (0x44C, false)), // CYRILLIC SMALL LETTER SOFT SIGN
    (0x59, (0x44B, false)), // LOWERCASE YERI / CYRILLIC SMALL LETTER YERI
    (0x5A, (0x437, false)), // CYRILLIC SMALL LETTER ZE
    (0x5B, (0x448, false)), // CYRILLIC SMALL LETTER SHA
    (0x5C, (0x44D, false)), // LOWERCASE REVERSED E / CYRILLIC SMALL LETTER E
    (0x5D, (0x449, false)), // CYRILLIC SMALL LETTER SHCHA
    (0x5E, (0x447, false)), // CYRILLIC SMALL LETTER CHE
    (0x5F, (0x44A, false)), // CYRILLIC SMALL LETTER HARD SIGN
    (0x60, (0x42E, false)), // UPPERCASE IU / CYRILLIC CAPITAL LETTER YU
    (0x61, (0x410, false)), // CYRILLIC CAPITAL LETTER A
    (0x62, (0x411, false)), // CYRILLIC CAPITAL LETTER BE
    (0x63, (0x426, false)), // CYRILLIC CAPITAL LETTER TSE
    (0x64, (0x414, false)), // CYRILLIC CAPITAL LETTER DE
    (0x65, (0x415, false)), // CYRILLIC CAPITAL LETTER IE
    (0x66, (0x424, false)), // CYRILLIC CAPITAL LETTER EF
    (0x67, (0x413, false)), // UPPERCASE GE / CYRILLIC CAPITAL LETTER GHE
    (0x68, (0x425, false)), // UPPERCASE KHA / CYRILLIC CAPITAL LETTER HA
    (0x69, (0x418, false)), // UPPERCASE II / CYRILLIC CAPITAL LETTER I
    (0x6A, (0x419, false)), // UPPERCASE SHORT II / CYRILLIC CAPITAL LETTER SHORT I
    (0x6B, (0x41A, false)), // CYRILLIC CAPITAL LETTER KA
    (0x6C, (0x41B, false)), // CYRILLIC CAPITAL LETTER EL
    (0x6D, (0x41C, false)), // CYRILLIC CAPITAL LETTER EM
    (0x6E, (0x41D, false)), // CYRILLIC CAPITAL LETTER EN
    (0x6F, (0x41E, false)), // CYRILLIC CAPITAL LETTER O
    (0x70, (0x41F, false)), // CYRILLIC CAPITAL LETTER PE
    (0x71, (0x42F, false)), // UPPERCASE IA / CYRILLIC CAPITAL LETTER YA
    (0x72, (0x420, false)), // CYRILLIC CAPITAL LETTER ER
    (0x73, (0x421, false)), // CYRILLIC CAPITAL LETTER ES
    (0x74, (0x422, false)), // CYRILLIC CAPITAL LETTER TE
    (0x75, (0x423, false)), // CYRILLIC CAPITAL LETTER U
    (0x76, (0x416, false)), // CYRILLIC CAPITAL LETTER ZHE
    (0x77, (0x412, false)), // CYRILLIC CAPITAL LETTER VE
    (0x78, (0x42C, false)), // CYRILLIC CAPITAL LETTER SOFT SIGN
    (0x79, (0x42B, false)), // UPPERCASE YERI / CYRILLIC CAPITAL LETTER YERI
    (0x7A, (0x417, false)), // CYRILLIC CAPITAL LETTER ZE
    (0x7B, (0x428, false)), // CYRILLIC CAPITAL LETTER SHA
    (0x7C, (0x42D, false)), // CYRILLIC CAPITAL LETTER E
    (0x7D, (0x429, false)), // CYRILLIC CAPITAL LETTER SHCHA
    (0x7E, (0x427, false)), // CYRILLIC CAPITAL LETTER CHE
];
