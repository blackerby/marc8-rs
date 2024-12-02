// Adapted from https://gitlab.com/pymarc/pymarc/-/blob/main/pymarc/marc8_mapping.py
// MARC-8 Code Tables: https://www.loc.gov/marc/specifications/specchartables.html

pub const EXTENDED_ARABIC: [(u32, (char, bool)); 90] = [
    (0xA1, ('\u{6FD}', false)), // DOUBLE ALEF WITH HAMZA ABOVE / ARABIC SIGN SINDHI AMPERSAND
    (0xA2, ('\u{672}', false)), // ARABIC LETTER ALEF WITH WAVY HAMZA ABOVE
    (0xA3, ('\u{673}', false)), // ARABIC LETTER ALEF WITH WAVY HAMZA BELOW
    (0xA4, ('\u{679}', false)), // ARABIC LETTER TTEH
    (0xA5, ('\u{67A}', false)), // ARABIC LETTER TTEHEH
    (0xA6, ('\u{67B}', false)), // ARABIC LETTER BBEH
    (0xA7, ('\u{67C}', false)), // ARABIC LETTER TEH WITH RING
    (0xA8, ('\u{67D}', false)), // ARABIC LETTER TEH WITH THREE DOTS ABOVE DOWNWARDS
    (0xA9, ('\u{67E}', false)), // ARABIC LETTER PEH
    (0xAA, ('\u{67F}', false)), // ARABIC LETTER TEHEH
    (0xAB, ('\u{680}', false)), // ARABIC LETTER BEHEH
    (0xAC, ('\u{681}', false)), // ARABIC LETTER HAH WITH HAMZA ABOVE
    (0xAD, ('\u{682}', false)), // ARABIC LETTER HAH WITH TWO ABOVE DOTS VERTICAL ABOVE
    (0xAE, ('\u{683}', false)), // ARABIC LETTER NYEH
    (0xAF, ('\u{684}', false)), // ARABIC LETTER DYEH
    (0xB0, ('\u{685}', false)), // ARABIC LETTER HAH WITH THREE DOTS ABOVE
    (0xB1, ('\u{686}', false)), // ARABIC LETTER TCHEH
    (0xB2, ('\u{6BF}', false)), // ARABIC LETTER TCHEH WITH DOT ABOVE
    (0xB3, ('\u{687}', false)), // ARABIC LETTER TCHEHEH
    (0xB4, ('\u{688}', false)), // ARABIC LETTER DDAL
    (0xB5, ('\u{689}', false)), // ARABIC LETTER DAL WITH RING
    (0xB6, ('\u{68A}', false)), // ARABIC LETTER DAL WITH DOT BELOW
    (0xB7, ('\u{68B}', false)), // ARABIC LETTER DAL WITH DOT BELOW AND SMALL TAH
    (0xB8, ('\u{68C}', false)), // ARABIC LETTER DAHAL
    (0xB9, ('\u{68D}', false)), // ARABIC LETTER DDAHAL
    (0xBA, ('\u{68E}', false)), // ARABIC LETTER DUL
    (0xBB, ('\u{68F}', false)), // ARABIC LETTER DAL WITH THREE DOTS ABOVE DOWNWARDS
    (0xBC, ('\u{690}', false)), // ARABIC LETTER DAL WITH FOUR DOTS ABOVE
    (0xBD, ('\u{691}', false)), // ARABIC LETTER RREH
    (0xBE, ('\u{692}', false)), // ARABIC LETTER REH WITH SMALL V
    (0xBF, ('\u{693}', false)), // ARABIC LETTER REH WITH RING
    (0xC0, ('\u{694}', false)), // ARABIC LETTER REH WITH DOT BELOW
    (0xC1, ('\u{695}', false)), // ARABIC LETTER REH WITH SMALL V BELOW
    (0xC2, ('\u{696}', false)), // ARABIC LETTER REH WITH DOT BELOW AND DOT ABOVE
    (0xC3, ('\u{697}', false)), // ARABIC LETTER REH WITH TWO DOTS ABOVE
    (0xC4, ('\u{698}', false)), // ARABIC LETTER JEH
    (0xC5, ('\u{699}', false)), // ARABIC LETTER REH WITH FOUR DOTS ABOVE
    (0xC6, ('\u{69A}', false)), // ARABIC LETTER SEEN WITH DOT BELOW AND DOT ABOVE
    (0xC7, ('\u{69B}', false)), // ARABIC LETTER SEEN WITH THREE DOTS BELOW
    (0xC8, ('\u{69C}', false)), // ARABIC LETTER SEEN WITH THREE DOTS BELOW AND THREE DOTS ABOVE
    (0xC9, ('\u{6FA}', false)), // ARABIC LETTER SHEEN WITH DOT BELOW
    (0xCA, ('\u{69D}', false)), // ARABIC LETTER SAD WITH TWO DOTS BELOW
    (0xCB, ('\u{69E}', false)), // ARABIC LETTER SAD WITH THREE DOTS ABOVE
    (0xCC, ('\u{6FB}', false)), // ARABIC LETTER DAD WITH DOT BELOW
    (0xCD, ('\u{69F}', false)), // ARABIC LETTER TAH WITH THREE DOTS ABOVE
    (0xCE, ('\u{6A0}', false)), // ARABIC LETTER AIN WITH THREE DOTS ABOVE
    (0xCF, ('\u{6FC}', false)), // ARABIC LETTER GHAIN WITH DOT BELOW
    (0xD0, ('\u{6A1}', false)), // ARABIC LETTER DOTLESS FEH
    (0xD1, ('\u{6A2}', false)), // ARABIC LETTER FEH WITH DOT MOVED BELOW
    (0xD2, ('\u{6A3}', false)), // ARABIC LETTER FEH WITH DOT BELOW
    (0xD3, ('\u{6A4}', false)), // ARABIC LETTER VEH
    (0xD4, ('\u{6A5}', false)), // ARABIC LETTER FEH WITH THREE DOTS BELOW
    (0xD5, ('\u{6A6}', false)), // ARABIC LETTER PEHEH
    (0xD6, ('\u{6A7}', false)), // ARABIC LETTER QAF WITH DOT ABOVE
    (0xD7, ('\u{6A8}', false)), // ARABIC LETTER QAF WITH THREE DOTS ABOVE
    (0xD8, ('\u{6A9}', false)), // ARABIC LETTER KEHEH
    (0xD9, ('\u{6AA}', false)), // ARABIC LETTER SWASH KAF
    (0xDA, ('\u{6AB}', false)), // ARABIC LETTER KAF WITH RING
    (0xDB, ('\u{6AC}', false)), // ARABIC LETTER KAF WITH DOT ABOVE
    (0xDC, ('\u{6AD}', false)), // ARABIC LETTER NG
    (0xDD, ('\u{6AE}', false)), // ARABIC LETTER KAF WITH THREE DOTS BELOW
    (0xDE, ('\u{6AF}', false)), // ARABIC LETTER GAF
    (0xDF, ('\u{6B0}', false)), // ARABIC LETTER GAF WITH RING
    (0xE0, ('\u{6B1}', false)), // ARABIC LETTER NGOEH
    (0xE1, ('\u{6B2}', false)), // ARABIC LETTER GAF WITH TWO DOTS BELOW
    (0xE2, ('\u{6B3}', false)), // ARABIC LETTER GUEH
    (0xE3, ('\u{6B4}', false)), // ARABIC LETTER GAF WITH THREE DOTS ABOVE
    (0xE4, ('\u{6B5}', false)), // ARABIC LETTER LAM WITH SMALL V
    (0xE5, ('\u{6B6}', false)), // ARABIC LETTER LAM WITH DOT ABOVE
    (0xE6, ('\u{6B7}', false)), // ARABIC LETTER LAM WITH THREE DOTS ABOVE
    (0xE7, ('\u{6B8}', false)), // ARABIC LETTER LAM WITH THREE DOTS BELOW
    (0xE8, ('\u{6BA}', false)), // ARABIC LETTER NOON GHUNNA
    (0xE9, ('\u{6BB}', false)), // ARABIC LETTER RNOON
    (0xEA, ('\u{6BC}', false)), // ARABIC LETTER NOON WITH RING
    (0xEB, ('\u{6BD}', false)), // ARABIC LETTER NOON WITH THREE DOTS ABOVE
    (0xEC, ('\u{6B9}', false)), // ARABIC LETTER NOON WITH DOT BELOW
    (0xED, ('\u{6BE}', false)), // ARABIC LETTER HEH DOACHASHMEE
    (0xEE, ('\u{6C0}', false)), // HEH WITH HAMZA ABOVE / ARABIC LETTER HEH WITH YEH ABOVE
    (0xEF, ('\u{6C4}', false)), // ARABIC LETTER WAW WITH RING
    (0xF0, ('\u{6C5}', false)), // KYRGHYZ OE / ARABIC LETTER KIRGHIZ OE
    (0xF1, ('\u{6C6}', false)), // ARABIC LETTER OE
    (0xF2, ('\u{6CA}', false)), // ARABIC LETTER WAW WITH TWO DOTS ABOVE
    (0xF3, ('\u{6CB}', false)), // ARABIC LETTER VE
    (0xF4, ('\u{6CD}', false)), // ARABIC LETTER YEH WITH TAIL
    (0xF5, ('\u{6CE}', false)), // ARABIC LETTER YEH WITH SMALL V
    (0xF6, ('\u{6D0}', false)), // ARABIC LETTER E
    (0xF7, ('\u{6D2}', false)), // ARABIC LETTER YEH BARREE
    (0xF8, ('\u{6D3}', false)), // ARABIC LETTER YEH BARREE WITH HAMZA ABOVE
    (0xFD, ('\u{306}', true)),  // SHORT E / COMBINING BREVE
    (0xFE, ('\u{30C}', true)),  // SHORT U / COMBINING CARON
];

// Extended Latin (ANSEL)
pub const EXTENDED_LATIN: [(u32, (char, bool)); 69] = [
    (0x88, ('\u{98}', false)),   // NON-SORT BEGIN / START OF STRING
    (0x89, ('\u{9C}', false)),   // NON-SORT END / STRING TERMINATOR
    (0x8D, ('\u{200D}', false)), // JOINER / ZERO WIDTH JOINER
    (0x8E, ('\u{200C}', false)), // NON-JOINER / ZERO WIDTH NON-JOINER
    (0xA1, ('\u{141}', false)),  // UPPERCASE POLISH L / LATIN CAPITAL LETTER L WITH STROKE
    (0xA2, ('\u{D8}', false)),   // UPPERCASE SCANDINAVIAN O / LATIN CAPITAL LETTER O WITH STROKE
    (0xA3, ('\u{110}', false)),  // UPPERCASE D WITH CROSSBAR / LATIN CAPITAL LETTER D WITH STROKE
    (0xA4, ('\u{DE}', false)), // UPPERCASE ICELANDIC THORN / LATIN CAPITAL LETTER THORN (Icelandic)
    (0xA5, ('\u{C6}', false)), // UPPERCASE DIGRAPH AE / LATIN CAPITAL LIGATURE AE
    (0xA6, ('\u{152}', false)), // UPPERCASE DIGRAPH OE / LATIN CAPITAL LIGATURE OE
    (0xA7, ('\u{2B9}', false)), // SOFT SIGN, PRIME / MODIFIER LETTER PRIME
    (0xA8, ('\u{B7}', false)), // MIDDLE DOT
    (0xA9, ('\u{266D}', false)), // MUSIC FLAT SIGN
    (0xAA, ('\u{AE}', false)), // PATENT MARK / REGISTERED SIGN
    (0xAB, ('\u{B1}', false)), // PLUS OR MINUS / PLUS-MINUS SIGN
    (0xAC, ('\u{1A0}', false)), // UPPERCASE O-HOOK / LATIN CAPITAL LETTER O WITH HORN
    (0xAD, ('\u{1AF}', false)), // UPPERCASE U-HOOK / LATIN CAPITAL LETTER U WITH HORN
    (0xAE, ('\u{2BC}', false)), // ALIF / MODIFIER LETTER RIGHT HALF RING
    (0xB0, ('\u{2BB}', false)), // AYN / MODIFIER LETTER TURNED COMMA
    (0xB1, ('\u{142}', false)), // LOWERCASE POLISH L / LATIN SMALL LETTER L WITH STROKE
    (0xB2, ('\u{F8}', false)), // LOWERCASE SCANDINAVIAN O / LATIN SMALL LETTER O WITH STROKE
    (0xB3, ('\u{111}', false)), // LOWERCASE D WITH CROSSBAR / LATIN SMALL LETTER D WITH STROKE
    (0xB4, ('\u{FE}', false)), // LOWERCASE ICELANDIC THORN / LATIN SMALL LETTER THORN (Icelandic)
    (0xB5, ('\u{E6}', false)), // LOWERCASE DIGRAPH AE / LATIN SMALL LIGATURE AE
    (0xB6, ('\u{153}', false)), // LOWERCASE DIGRAPH OE / LATIN SMALL LIGATURE OE
    (0xB7, ('\u{2BA}', false)), // HARD SIGN, DOUBLE PRIME / MODIFIER LETTER DOUBLE PRIME
    (0xB8, ('\u{131}', false)), // LOWERCASE TURKISH I / LATIN SMALL LETTER DOTLESS I
    (0xB9, ('\u{A3}', false)), // BRITISH POUND / POUND SIGN
    (0xBA, ('\u{F0}', false)), // LOWERCASE ETH / LATIN SMALL LETTER ETH (Icelandic)
    (0xBC, ('\u{1A1}', false)), // LOWERCASE O-HOOK / LATIN SMALL LETTER O WITH HORN
    (0xBD, ('\u{1B0}', false)), // LOWERCASE U-HOOK / LATIN SMALL LETTER U WITH HORN
    (0xC0, ('\u{B0}', false)), // DEGREE SIGN
    (0xC1, ('\u{2113}', false)), // SCRIPT SMALL L
    (0xC2, ('\u{2117}', false)), // SOUND RECORDING COPYRIGHT
    (0xC3, ('\u{A9}', false)), // COPYRIGHT SIGN
    (0xC4, ('\u{266F}', false)), // MUSIC SHARP SIGN
    (0xC5, ('\u{BF}', false)), // INVERTED QUESTION MARK
    (0xC6, ('\u{A1}', false)), // INVERTED EXCLAMATION MARK
    (0xC7, ('\u{DF}', false)), // ESZETT SYMBOL
    (0xC8, ('\u{20AC}', false)), // EURO SIGN
    (0xE0, ('\u{309}', true)), // PSEUDO QUESTION MARK / COMBINING HOOK ABOVE
    (0xE1, ('\u{300}', true)), // GRAVE / COMBINING GRAVE ACCENT (Varia)
    (0xE2, ('\u{301}', true)), // ACUTE / COMBINING ACUTE ACCENT (Oxia)
    (0xE3, ('\u{302}', true)), // CIRCUMFLEX / COMBINING CIRCUMFLEX ACCENT
    (0xE4, ('\u{303}', true)), // TILDE / COMBINING TILDE
    (0xE5, ('\u{304}', true)), // MACRON / COMBINING MACRON
    (0xE6, ('\u{306}', true)), // BREVE / COMBINING BREVE (Vrachy)
    (0xE7, ('\u{307}', true)), // SUPERIOR DOT / COMBINING DOT ABOVE
    (0xE8, ('\u{308}', true)), // UMLAUT, DIAERESIS / COMBINING DIAERESIS (Dialytika)
    (0xE9, ('\u{30C}', true)), // HACEK / COMBINING CARON
    (0xEA, ('\u{30A}', true)), // CIRCLE ABOVE, ANGSTROM / COMBINING RING ABOVE
    (0xEB, ('\u{FE20}', true)), // LIGATURE, FIRST HALF / COMBINING LIGATURE LEFT HALF
    (0xEC, ('\u{FE21}', true)), // LIGATURE, SECOND HALF / COMBINING LIGATURE RIGHT HALF
    (0xED, ('\u{315}', true)), // HIGH COMMA, OFF CENTER / COMBINING COMMA ABOVE RIGHT
    (0xEE, ('\u{30B}', true)), // DOUBLE ACUTE / COMBINING DOUBLE ACUTE ACCENT
    (0xEF, ('\u{310}', true)), // CANDRABINDU / COMBINING CANDRABINDU
    (0xF0, ('\u{327}', true)), // CEDILLA / COMBINING CEDILLA
    (0xF1, ('\u{328}', true)), // RIGHT HOOK, OGONEK / COMBINING OGONEK
    (0xF2, ('\u{323}', true)), // DOT BELOW / COMBINING DOT BELOW
    (0xF3, ('\u{324}', true)), // DOUBLE DOT BELOW / COMBINING DIAERESIS BELOW
    (0xF4, ('\u{325}', true)), // CIRCLE BELOW / COMBINING RING BELOW
    (0xF5, ('\u{333}', true)), // DOUBLE UNDERSCORE / COMBINING DOUBLE LOW LINE
    (0xF6, ('\u{332}', true)), // UNDERSCORE / COMBINING LOW LINE
    (0xF7, ('\u{326}', true)), // LEFT HOOK (COMMA BELOW) / COMBINING COMMA BELOW
    (0xF8, ('\u{31C}', true)), // RIGHT CEDILLA / COMBINING LEFT HALF RING BELOW
    (0xF9, ('\u{32E}', true)), // UPADHMANIYA / COMBINING BREVE BELOW
    (0xFA, ('\u{FE22}', true)), // DOUBLE TILDE, FIRST HALF / COMBINING DOUBLE TILDE LEFT HALF
    (0xFB, ('\u{FE23}', true)), // DOUBLE TILDE, SECOND HALF / COMBINING DOUBLE TILDE RIGHT HALF
    (0xFE, ('\u{313}', true)), // HIGH COMMA, CENTERED / COMBINING COMMA ABOVE (Psili)
];

pub const BASIC_ARABIC: [(u32, (char, bool)); 83] = [
    (0x21, ('\u{21}', false)),   // EXCLAMATION MARK
    (0x22, ('\u{22}', false)),   // QUOTATION MARK
    (0x23, ('\u{23}', false)),   // NUMBER SIGN
    (0x24, ('\u{24}', false)),   // DOLLAR SIGN
    (0x25, ('\u{66A}', false)),  // PERCENT SIGN / ARABIC PERCENT SIGN
    (0x26, ('\u{26}', false)),   // AMPERSAND
    (0x27, ('\u{27}', false)),   // APOSTROPHE
    (0x28, ('\u{28}', false)),   // OPENING PARENTHESIS / LEFT PARENTHESIS
    (0x29, ('\u{29}', false)),   // CLOSING PARENTHESIS / RIGHT PARENTHESIS
    (0x2A, ('\u{66D}', false)),  // ASTERISK / ARABIC FIVE POINTED STAR
    (0x2B, ('\u{2B}', false)),   // PLUS SIGN
    (0x2C, ('\u{60C}', false)),  // ARABIC COMMA
    (0x2D, ('\u{2D}', false)),   // HYPHEN-MINUS
    (0x2E, ('\u{2E}', false)),   // PERIOD, DECIMAL POINT / FULL STOP
    (0x2F, ('\u{2F}', false)),   // SLASH / SOLIDUS
    (0x30, ('\u{660}', false)),  // ARABIC-INDIC DIGIT ZERO
    (0x31, ('\u{661}', false)),  // ARABIC-INDIC DIGIT ONE
    (0x32, ('\u{662}', false)),  // ARABIC-INDIC DIGIT TWO
    (0x33, ('\u{663}', false)),  // ARABIC-INDIC DIGIT THREE
    (0x34, ('\u{664}', false)),  // ARABIC-INDIC DIGIT FOUR
    (0x35, ('\u{665}', false)),  // ARABIC-INDIC DIGIT FIVE
    (0x36, ('\u{666}', false)),  // ARABIC-INDIC DIGIT SIX
    (0x37, ('\u{667}', false)),  // ARABIC-INDIC DIGIT SEVEN
    (0x38, ('\u{668}', false)),  // ARABIC-INDIC DIGIT EIGHT
    (0x39, ('\u{669}', false)),  // ARABIC-INDIC DIGIT NINE
    (0x3A, ('\u{3A}', false)),   // COLON
    (0x3B, ('\u{61B}', false)),  // ARABIC SEMICOLON
    (0x3C, ('\u{3C}', false)),   // LESS-THAN SIGN
    (0x3D, ('\u{3D}', false)),   // EQUALS SIGN
    (0x3E, ('\u{3E}', false)),   // GREATER-THAN SIGN
    (0x3F, ('\u{61F}', false)),  // ARABIC QUESTION MARK
    (0x41, ('\u{621}', false)),  // HAMZAH / ARABIC LETTER HAMZA
    (0x42, ('\u{622}', false)),  // ARABIC LETTER ALEF WITH MADDA ABOVE
    (0x43, ('\u{623}', false)),  // ARABIC LETTER ALEF WITH HAMZA ABOVE
    (0x44, ('\u{624}', false)),  // ARABIC LETTER WAW WITH HAMZA ABOVE
    (0x45, ('\u{625}', false)),  // ARABIC LETTER ALEF WITH HAMZA BELOW
    (0x46, ('\u{626}', false)),  // ARABIC LETTER YEH WITH HAMZA ABOVE
    (0x47, ('\u{627}', false)),  // ARABIC LETTER ALEF
    (0x48, ('\u{628}', false)),  // ARABIC LETTER BEH
    (0x49, ('\u{629}', false)),  // ARABIC LETTER TEH MARBUTA
    (0x4A, ('\u{62A}', false)),  // ARABIC LETTER TEH
    (0x4B, ('\u{62B}', false)),  // ARABIC LETTER THEH
    (0x4C, ('\u{62C}', false)),  // ARABIC LETTER JEEM
    (0x4D, ('\u{62D}', false)),  // ARABIC LETTER HAH
    (0x4E, ('\u{62E}', false)),  // ARABIC LETTER KHAH
    (0x4F, ('\u{62F}', false)),  // ARABIC LETTER DAL
    (0x50, ('\u{630}', false)),  // ARABIC LETTER THAL
    (0x51, ('\u{631}', false)),  // ARABIC LETTER REH
    (0x52, ('\u{632}', false)),  // ARABIC LETTER ZAIN
    (0x53, ('\u{633}', false)),  // ARABIC LETTER SEEN
    (0x54, ('\u{634}', false)),  // ARABIC LETTER SHEEN
    (0x55, ('\u{635}', false)),  // ARABIC LETTER SAD
    (0x56, ('\u{636}', false)),  // ARABIC LETTER DAD
    (0x57, ('\u{637}', false)),  // ARABIC LETTER TAH
    (0x58, ('\u{638}', false)),  // ARABIC LETTER ZAH
    (0x59, ('\u{639}', false)),  // ARABIC LETTER AIN
    (0x5A, ('\u{63A}', false)),  // ARABIC LETTER GHAIN
    (0x5B, ('\u{5B}', false)),   // OPENING SQUARE BRACKET / LEFT SQUARE BRACKET
    (0x5D, ('\u{5D}', false)),   // CLOSING SQUARE BRACKET / RIGHT SQUARE BRACKET
    (0x60, ('\u{640}', false)),  // ARABIC TATWEEL
    (0x61, ('\u{641}', false)),  // ARABIC LETTER FEH
    (0x62, ('\u{642}', false)),  // ARABIC LETTER QAF
    (0x63, ('\u{643}', false)),  // ARABIC LETTER KAF
    (0x64, ('\u{644}', false)),  // ARABIC LETTER LAM
    (0x65, ('\u{645}', false)),  // ARABIC LETTER MEEM
    (0x66, ('\u{646}', false)),  // ARABIC LETTER NOON
    (0x67, ('\u{647}', false)),  // ARABIC LETTER HEH
    (0x68, ('\u{648}', false)),  // ARABIC LETTER WAW
    (0x69, ('\u{649}', false)),  // ARABIC LETTER ALEF MAKSURA
    (0x6A, ('\u{64A}', false)),  // ARABIC LETTER YEH
    (0x6B, ('\u{64B}', true)),   // ARABIC FATHATAN
    (0x6C, ('\u{64C}', true)),   // ARABIC DAMMATAN
    (0x6D, ('\u{64D}', true)),   // ARABIC KASRATAN
    (0x6E, ('\u{64E}', true)),   // ARABIC FATHA
    (0x6F, ('\u{64F}', true)),   // ARABIC DAMMA
    (0x70, ('\u{650}', true)),   // ARABIC KASRA
    (0x71, ('\u{651}', true)),   // ARABIC SHADDA
    (0x72, ('\u{652}', true)),   // ARABIC SUKUN
    (0x73, ('\u{671}', false)),  // ARABIC LETTER ALEF WASLA
    (0x74, ('\u{670}', false)),  // ARABIC LETTER SUPERSCRIPT ALEF
    (0x78, ('\u{66C}', false)),  // ARABIC THOUSANDS SEPARATOR
    (0x79, ('\u{201D}', false)), // RIGHT DOUBLE QUOTATION MARK
    (0x7A, ('\u{201C}', false)), // LEFT DOUBLE QUOTATION MARK
];

// Basic Hebrew
pub const BASIC_HEBREW: [(u32, (char, bool)); 78] = [
    (0x21, ('\u{21}', false)),  // EXCLAMATION MARK
    (0x22, ('\u{5F4}', false)), // QUOTATION MARK, GERSHAYIM / HEBREW PUNCTUATION GERSHAYIM
    (0x23, ('\u{23}', false)),  // NUMBER SIGN
    (0x24, ('\u{24}', false)),  // DOLLAR SIGN
    (0x25, ('\u{25}', false)),  // PERCENT SIGN
    (0x26, ('\u{26}', false)),  // AMPERSAND
    (0x27, ('\u{5F3}', false)), // APOSTROPHE, GERESH / HEBREW PUNCTUATION GERESH
    (0x28, ('\u{28}', false)),  // OPENING PARENTHESIS / LEFT PARENTHESIS
    (0x29, ('\u{29}', false)),  // CLOSING PARENTHESIS / RIGHT PARENTHESIS
    (0x2A, ('\u{2A}', false)),  // ASTERISK
    (0x2B, ('\u{2B}', false)),  // PLUS SIGN
    (0x2C, ('\u{2C}', false)),  // COMMA
    (0x2D, ('\u{5BE}', false)), // HYPHEN-MINUS, MAKEF / HEBREW PUNCTUATION MAQAF
    (0x2E, ('\u{2E}', false)),  // PERIOD, DECIMAL POINT / FULL STOP
    (0x2F, ('\u{2F}', false)),  // SLASH / SOLIDUS
    (0x30, ('\u{30}', false)),  // DIGIT ZERO
    (0x31, ('\u{31}', false)),  // DIGIT ONE
    (0x32, ('\u{32}', false)),  // DIGIT TWO
    (0x33, ('\u{33}', false)),  // DIGIT THREE
    (0x34, ('\u{34}', false)),  // DIGIT FOUR
    (0x35, ('\u{35}', false)),  // DIGIT FIVE
    (0x36, ('\u{36}', false)),  // DIGIT SIX
    (0x37, ('\u{37}', false)),  // DIGIT SEVEN
    (0x38, ('\u{38}', false)),  // DIGIT EIGHT
    (0x39, ('\u{39}', false)),  // DIGIT NINE
    (0x3A, ('\u{3A}', false)),  // COLON
    (0x3B, ('\u{3B}', false)),  // SEMICOLON
    (0x3C, ('\u{3C}', false)),  // LESS-THAN SIGN
    (0x3D, ('\u{3D}', false)),  // EQUALS SIGN
    (0x3E, ('\u{3E}', false)),  // GREATER-THAN SIGN
    (0x3F, ('\u{3F}', false)),  // QUESTION MARK
    (0x40, ('\u{5B7}', true)),  // HEBREW POINT PATAH
    (0x41, ('\u{5B8}', true)),  // KAMATS / HEBREW POINT QAMATS
    (0x42, ('\u{5B6}', true)),  // HEBREW POINT SEGOL
    (0x43, ('\u{5B5}', true)),  // TSEREH / HEBREW POINT TSERE
    (0x44, ('\u{5B4}', true)),  // HIRIK / HEBREW POINT HIRIQ
    (0x45, ('\u{5B9}', true)),  // HOLAM, LEFT SIN DOT / HEBREW POINT HOLAM
    (0x46, ('\u{5BB}', true)),  // KUBUTS / HEBREW POINT QUBUTS
    (0x47, ('\u{5B0}', true)),  // HEBREW POINT SHEVA
    (0x48, ('\u{5B2}', true)),  // HEBREW POINT HATAF PATAH
    (0x49, ('\u{5B3}', true)),  // HATAF KAMATS / HEBREW POINT HATAF QAMATS
    (0x4A, ('\u{5B1}', true)),  // HEBREW POINT HATAF SEGOL
    (0x4B, ('\u{5BC}', true)),  // HEBREW POINT DAGESH OR MAPIQ
    (0x4C, ('\u{5BF}', true)),  // RAFEH / HEBREW POINT RAFE
    (0x4D, ('\u{5C1}', true)),  // RIGHT SHIN DOT / HEBREW POINT  SHIN DOT
    (0x4E, ('\u{FB1E}', true)), // VARIKA / HEBREW POINT JUDEO-SPANISH VARIKA
    (0x5B, ('\u{5B}', false)),  // OPENING SQUARE BRACKET / LEFT SQUARE BRACKET
    (0x5D, ('\u{5D}', false)),  // CLOSING SQUARE BRACKET / RIGHT SQUARE BRACKET
    (0x60, ('\u{5D0}', false)), // HEBREW LETTER ALEF
    (0x61, ('\u{5D1}', false)), // HEBREW LETTER BET
    (0x62, ('\u{5D2}', false)), // HEBREW LETTER GIMEL
    (0x63, ('\u{5D3}', false)), // HEBREW LETTER DALET
    (0x64, ('\u{5D4}', false)), // HEBREW LETTER HE
    (0x65, ('\u{5D5}', false)), // HEBREW LETTER VAV
    (0x66, ('\u{5D6}', false)), // HEBREW LETTER ZAYIN
    (0x67, ('\u{5D7}', false)), // HEBREW LETTER HET
    (0x68, ('\u{5D8}', false)), // HEBREW LETTER TET
    (0x69, ('\u{5D9}', false)), // HEBREW LETTER YOD
    (0x6A, ('\u{5DA}', false)), // HEBREW LETTER FINAL KAF
    (0x6B, ('\u{5DB}', false)), // HEBREW LETTER KAF
    (0x6C, ('\u{5DC}', false)), // HEBREW LETTER LAMED
    (0x6D, ('\u{5DD}', false)), // HEBREW LETTER FINAL MEM
    (0x6E, ('\u{5DE}', false)), // HEBREW LETTER MEM
    (0x6F, ('\u{5DF}', false)), // HEBREW LETTER FINAL NUN
    (0x70, ('\u{5E0}', false)), // HEBREW LETTER NUN
    (0x71, ('\u{5E1}', false)), // HEBREW LETTER SAMEKH
    (0x72, ('\u{5E2}', false)), // HEBREW LETTER AYIN
    (0x73, ('\u{5E3}', false)), // HEBREW LETTER FINAL PE
    (0x74, ('\u{5E4}', false)), // HEBREW LETTER PE
    (0x75, ('\u{5E5}', false)), // HEBREW LETTER FINAL TSADI
    (0x76, ('\u{5E6}', false)), // HEBREW LETTER TSADI
    (0x77, ('\u{5E7}', false)), // HEBREW LETTER QOF / KOF
    (0x78, ('\u{5E8}', false)), // HEBREW LETTER RESH
    (0x79, ('\u{5E9}', false)), // HEBREW LETTER SHIN
    (0x7A, ('\u{5EA}', false)), // HEBREW LETTER TAV
    (0x7B, ('\u{5F0}', false)), // HEBREW LIGATURE YIDDISH DOUBLE VAV / TSVEY VOVN
    (0x7C, ('\u{5F1}', false)), // HEBREW LIGATURE YIDDISH VAV YOD / VOV YUD
    (0x7D, ('\u{5F2}', false)), // HEBREW LIGATURE YIDDISH DOUBLE YOD / TSVEY YUDN
];

// Chinese, Japanese, Korean (EACC)
pub const EACC: [(u32, (char, bool)); 15739] = [
    (0x215556, ('\u{8461}', false)), // East Asian ideograph
    (0x6F5557, ('\u{C5D1}', false)), // Korean hangul
    (0x456324, ('\u{9F61}', false)), // East Asian ideograph
    (0x6F5140, ('\u{BCCF}', false)), // Korean hangul
    (0x6F5558, ('\u{C5D4}', false)), // Korean hangul
    (0x213536, ('\u{53EC}', false)), // East Asian ideograph
    (0x6F5D3C, ('\u{D64B}', false)), // Korean hangul
    (0x215559, ('\u{8438}', false)), // East Asian ideograph
    (0x2D555A, ('\u{8386}', false)), // East Asian ideograph
    (0x6F5C7C, ('\u{D5D0}', false)), // Korean hangul
    (0x295B60, ('\u{9E55}', false)), // East Asian ideograph
    (0x2D555B, ('\u{8385}', false)), // East Asian ideograph
    (0x6F555C, ('\u{C5E3}', false)), // Korean hangul
    (0x6F5141, ('\u{BCD0}', false)), // Korean hangul
    (0x27555D, ('\u{5E2D}', false)), // East Asian ideograph
    (0x23555E, ('\u{9B1F}', false)), // East Asian ideograph
    (0x333F24, ('\u{7718}', false)), // East Asian ideograph
    (0x6F555F, ('\u{C5ED}', false)), // Korean hangul
    (0x6F4F5C, ('\u{B9AC}', false)), // Korean hangul
    (0x6F5560, ('\u{C5EE}', false)), // Korean hangul
    (0x6F4E21, ('\u{B540}', false)), // Korean hangul
    (0x4B4146, ('\u{6362}', false)), // East Asian ideograph
    (0x235031, ('\u{9874}', false)), // East Asian ideograph
    (0x225561, ('\u{7273}', false)), // East Asian ideograph
    (0x274257, ('\u{6BD9}', false)), // East Asian ideograph
    (0x295C28, ('\u{9E58}', false)), // East Asian ideograph
    (0x6F5142, ('\u{BCD1}', false)), // Korean hangul
    (0x6F5562, ('\u{C5F4}', false)), // Korean hangul
    (0x213727, ('\u{5616}', false)), // East Asian ideograph
    (0x215563, ('\u{84C0}', false)), // East Asian ideograph
    (0x215564, ('\u{8499}', false)), // East Asian ideograph
    (0x6F562E, ('\u{C679}', false)), // Korean hangul
    (0x2D4674, ('\u{51B2}', false)), // East Asian ideograph
    (0x6F5565, ('\u{C5FC}', false)), // Korean hangul
    (0x4B4147, ('\u{633F}', false)), // East Asian ideograph
    (0x215566, ('\u{8490}', false)), // East Asian ideograph
    (0x6F5143, ('\u{BCD2}', false)), // Korean hangul
    (0x275567, ('\u{82CD}', false)), // East Asian ideograph
    (0x215568, ('\u{853D}', false)), // East Asian ideograph
    (0x6F5569, ('\u{C600}', false)), // Korean hangul
    (0x27314C, ('\u{6765}', false)), // East Asian ideograph
    (0x276071, ('\u{517B}', false)), // East Asian ideograph
    (0x6F556A, ('\u{C601}', false)), // Korean hangul
    (0x33325D, ('\u{4FA1}', false)), // East Asian ideograph
    (0x6F5839, ('\u{C9DD}', false)), // Korean hangul
    (0x2D6B5F, ('\u{5273}', false)), // East Asian ideograph
    (0x21556B, ('\u{851A}', false)), // East Asian ideograph
    (0x6F5144, ('\u{BCD5}', false)), // Korean hangul
    (0x27556C, ('\u{83B2}', false)), // East Asian ideograph
    (0x22556D, ('\u{727C}', false)), // East Asian ideograph
    (0x21556E, ('\u{852D}', false)), // East Asian ideograph
    (0x6F556F, ('\u{C610}', false)), // Korean hangul
    (0x295721, ('\u{9C86}', false)), // East Asian ideograph
    (0x466074, ('\u{76B2}', false)), // East Asian ideograph
    (0x333529, ('\u{53DC}', false)), // East Asian ideograph
    (0x6F5145, ('\u{BCF4}', false)), // Korean hangul
    (0x225571, ('\u{727F}', false)), // East Asian ideograph
    (0x225D42, ('\u{7521}', false)), // East Asian ideograph
    (0x275949, ('\u{8A89}', false)), // East Asian ideograph
    (0x6F5037, ('\u{BA84}', false)), // Korean hangul
    (0x215573, ('\u{8514}', false)), // East Asian ideograph
    (0x215574, ('\u{84EC}', false)), // East Asian ideograph
    (0x4C2330, ('\u{5C53}', false)), // East Asian ideograph
    (0x69656E, ('\u{7DD5}', false)), // East Asian ideograph
    (0x6F5146, ('\u{BCF5}', false)), // Korean hangul
    (0x215576, ('\u{8569}', false)), // East Asian ideograph
    (0x282441, ('\u{5C98}', false)), // East Asian ideograph
    (0x234021, ('\u{9132}', false)), // East Asian ideograph
    (0x4D4176, ('\u{91DB}', false)), // East Asian ideograph
    (0x335577, ('\u{8602}', false)), // East Asian ideograph
    (0x394022, ('\u{6443}', false)), // East Asian ideograph
    (0x6F5578, ('\u{C634}', false)), // Korean hangul
    (0x6F4F5D, ('\u{B9AD}', false)), // Korean hangul
    (0x2D3749, ('\u{5650}', false)), // East Asian ideograph
    (0x287139, ('\u{7EE8}', false)), // East Asian ideograph
    (0x234024, ('\u{9126}', false)), // East Asian ideograph
    (0x6F557A, ('\u{C637}', false)), // Korean hangul
    (0x213C35, ('\u{5DDE}', false)), // East Asian ideograph
    (0x6F5147, ('\u{BCF6}', false)), // Korean hangul
    (0x6F557B, ('\u{C639}', false)), // Korean hangul
    (0x215945, ('\u{8B66}', false)), // East Asian ideograph
    (0x21372C, ('\u{5606}', false)), // East Asian ideograph (variant of 4B372C which maps to 5606)
    (0x27557C, ('\u{829C}', false)), // East Asian ideograph
    (0x224027, ('\u{69BF}', false)), // East Asian ideograph
    (0x23557D, ('\u{9B34}', false)), // East Asian ideograph
    (0x6F557E, ('\u{C640}', false)), // Korean hangul
    (0x2D4029, ('\u{5214}', false)), // East Asian ideograph
    (0x6F5148, ('\u{BCF8}', false)), // Korean hangul
    (0x23402B, ('\u{9134}', false)), // East Asian ideograph
    (0x21372D, ('\u{5609}', false)), // East Asian ideograph
    (0x23402C, ('\u{9136}', false)), // East Asian ideograph
    (0x6F5876, ('\u{CC14}', false)), // Korean hangul
    (0x22402D, ('\u{69A3}', false)), // East Asian ideograph
    (0x22507C, ('\u{70DC}', false)), // East Asian ideograph
    (0x22402E, ('\u{69A4}', false)), // East Asian ideograph
    (0x6F5149, ('\u{BCFC}', false)), // Korean hangul
    (0x6F575F, ('\u{C8B0}', false)), // Korean hangul
    (0x295A75, ('\u{9E41}', false)), // East Asian ideograph
    (0x4B525A, ('\u{7FFA}', false)), // East Asian ideograph
    (0x234031, ('\u{913A}', false)), // East Asian ideograph
    (0x2D383F, ('\u{575A}', false)), // East Asian ideograph
    (0x294371, ('\u{94FD}', false)), // East Asian ideograph
    (0x234032, ('\u{913B}', false)), // East Asian ideograph
    (0x6F5C27, ('\u{D38C}', false)), // Korean hangul
    (0x224034, ('\u{69D4}', false)), // East Asian ideograph
    (0x6F514A, ('\u{BD04}', false)), // Korean hangul
    (0x335F73, ('\u{9759}', false)), // East Asian ideograph
    (0x6F4C33, ('\u{B17C}', false)), // Korean hangul
    (0x4B525B, ('\u{66DC}', false)), // East Asian ideograph (variant of 39525B which maps to 66DC)
    (0x2E7C2E, ('\u{831C}', false)), // East Asian ideograph
    (0x224038, ('\u{69C3}', false)), // East Asian ideograph
    (0x6F5B4D, ('\u{D280}', false)), // Korean hangul
    (0x2D4039, ('\u{67C6}', false)), // East Asian ideograph
    (0x6F514B, ('\u{BD05}', false)), // Korean hangul
    (0x276036, ('\u{54CD}', false)), // East Asian ideograph
    (0x395477, ('\u{85A6}', false)), // East Asian ideograph
    (0x213730, ('\u{5617}', false)), // East Asian ideograph
    (0x4B525C, ('\u{8002}', false)), // East Asian ideograph
    (0x23403B, ('\u{9143}', false)), // East Asian ideograph
    (0x295B6B, ('\u{9E57}', false)), // East Asian ideograph
    (0x2D5963, ('\u{8C98}', false)), // East Asian ideograph
    (0x224C3C, ('\u{6F3B}', false)), // East Asian ideograph
    (0x22403E, ('\u{6A11}', false)), // East Asian ideograph
    (0x6F5A73, ('\u{D0C8}', false)), // Korean hangul
    (0x6F514C, ('\u{BD07}', false)), // Korean hangul
    (0x213B74, ('\u{5CFD}', false)), // East Asian ideograph
    (0x23403F, ('\u{9145}', false)), // East Asian ideograph
    (0x21594A, ('\u{8B80}', false)), // East Asian ideograph
    (0x213731, ('\u{560D}', false)), // East Asian ideograph
    (0x225D49, ('\u{752F}', false)), // East Asian ideograph
    (0x234040, ('\u{9148}', false)), // East Asian ideograph
    (0x224041, ('\u{6A00}', false)), // East Asian ideograph
    (0x295B6C, ('\u{9E4B}', false)), // East Asian ideograph
    (0x234042, ('\u{9150}', false)), // East Asian ideograph
    (0x234043, ('\u{914E}', false)), // East Asian ideograph
    (0x6F514D, ('\u{BD09}', false)), // Korean hangul
    (0x213B75, ('\u{5CED}', false)), // East Asian ideograph
    (0x394A60, ('\u{9AE6}', false)), // East Asian ideograph
    (0x213732, ('\u{562E}', false)), // East Asian ideograph
    (0x334045, ('\u{629B}', false)), // East Asian ideograph
    (0x292A34, ('\u{86AC}', false)), // East Asian ideograph
    (0x224046, ('\u{69E6}', false)), // East Asian ideograph
    (0x2F2D79, ('\u{88B5}', false)), // East Asian ideograph
    (0x234048, ('\u{9159}', false)), // East Asian ideograph
    (0x6F514E, ('\u{BD10}', false)), // Korean hangul
    (0x276039, ('\u{9877}', false)), // East Asian ideograph
    (0x234049, ('\u{915C}', false)), // East Asian ideograph
    (0x33494A, ('\u{70D6}', false)), // East Asian ideograph
    (0x294372, ('\u{9513}', false)), // East Asian ideograph
    (0x22404B, ('\u{6A0B}', false)), // East Asian ideograph
    (0x22404C, ('\u{69E5}', false)), // East Asian ideograph
    (0x2E2F7A, ('\u{6738}', false)), // East Asian ideograph
    (0x22404D, ('\u{69E9}', false)), // East Asian ideograph
    (0x6F514F, ('\u{BD14}', false)), // Korean hangul
    (0x27603A, ('\u{9879}', false)), // East Asian ideograph
    (0x2D4F29, ('\u{9F9D}', false)), // East Asian ideograph
    (0x213734, ('\u{564E}', false)), // East Asian ideograph
    (0x2D404F, ('\u{6294}', false)), // East Asian ideograph
    (0x6F5039, ('\u{BA87}', false)), // Korean hangul
    (0x224050, ('\u{69FC}', false)), // East Asian ideograph
    (0x6F5B4E, ('\u{D284}', false)), // Korean hangul
    (0x234052, ('\u{915A}', false)), // East Asian ideograph
    (0x6F5150, ('\u{BD24}', false)), // Korean hangul
    (0x213B78, ('\u{5CF0}', false)), // East Asian ideograph
    (0x234053, ('\u{9161}', false)), // East Asian ideograph
    (0x6F596B, ('\u{CE6B}', false)), // Korean hangul
    (0x225D4D, ('\u{753A}', false)), // East Asian ideograph
    (0x224054, ('\u{6A17}', false)), // East Asian ideograph
    (0x4B4C3C, ('\u{7573}', false)), // East Asian ideograph
    (0x224056, ('\u{69E7}', false)), // East Asian ideograph
    (0x224057, ('\u{69EB}', false)), // East Asian ideograph
    (0x6F5151, ('\u{BD48}', false)), // Korean hangul
    (0x213B79, ('\u{5CF6}', false)), // East Asian ideograph
    (0x294621, ('\u{9553}', false)), // East Asian ideograph
    (0x4B6266, ('\u{9ED2}', false)), // East Asian ideograph
    (0x6F5631, ('\u{C688}', false)), // Korean hangul
    (0x22405B, ('\u{69F1}', false)), // East Asian ideograph
    (0x6F5152, ('\u{BD49}', false)), // Korean hangul
    (0x27603D, ('\u{9884}', false)), // East Asian ideograph
    (0x22405E, ('\u{6A2B}', false)), // East Asian ideograph
    (0x29444D, ('\u{952B}', false)), // East Asian ideograph
    (0x22405F, ('\u{69FF}', false)), // East Asian ideograph
    (0x224060, ('\u{6A20}', false)), // East Asian ideograph
    (0x234061, ('\u{916F}', false)), // East Asian ideograph
    (0x6F5153, ('\u{BD4C}', false)), // Korean hangul
    (0x27603E, ('\u{987C}', false)), // East Asian ideograph
    (0x234062, ('\u{916E}', false)), // East Asian ideograph
    (0x275D60, ('\u{94E8}', false)), // East Asian ideograph
    (0x6F5A71, ('\u{D0C1}', false)), // Korean hangul
    (0x4B4E21, ('\u{7B36}', false)), // East Asian ideograph
    (0x224064, ('\u{69ED}', false)), // East Asian ideograph
    (0x28355B, ('\u{6484}', false)), // East Asian ideograph
    (0x6F547D, ('\u{C545}', false)), // Korean hangul
    (0x234066, ('\u{917A}', false)), // East Asian ideograph
    (0x6F582E, ('\u{C9CA}', false)), // Korean hangul
    (0x6F5154, ('\u{BD50}', false)), // Korean hangul
    (0x27603F, ('\u{987D}', false)), // East Asian ideograph
    (0x224067, ('\u{6A1B}', false)), // East Asian ideograph
    (0x213739, ('\u{5657}', false)), // East Asian ideograph
    (0x2D7143, ('\u{55E2}', false)), // East Asian ideograph
    (0x234068, ('\u{9172}', false)), // East Asian ideograph
    (0x2D5A63, ('\u{8DE5}', false)), // East Asian ideograph
    (0x2D384A, ('\u{5872}', false)), // East Asian ideograph
    (0x234069, ('\u{9179}', false)), // East Asian ideograph
    (0x27632C, ('\u{9F9A}', false)), // East Asian ideograph
    (0x23406A, ('\u{9176}', false)), // East Asian ideograph
    (0x4C233F, ('\u{5C76}', false)), // East Asian ideograph
    (0x23406B, ('\u{9174}', false)), // East Asian ideograph
    (0x6F5155, ('\u{BD58}', false)), // Korean hangul
    (0x213B7D, ('\u{5D1B}', false)), // East Asian ideograph
    (0x276040, ('\u{987F}', false)), // East Asian ideograph
    (0x23406C, ('\u{9173}', false)), // East Asian ideograph
    (0x23406D, ('\u{9185}', false)), // East Asian ideograph
    (0x22406E, ('\u{6A18}', false)), // East Asian ideograph
    (0x23406F, ('\u{9182}', false)), // East Asian ideograph
    (0x4B5F30, ('\u{9686}', false)), // East Asian ideograph (variant of 215F30 which maps to 9686)
    (0x234070, ('\u{918A}', false)), // East Asian ideograph
    (0x6F5A75, ('\u{D0D0}', false)), // Korean hangul
    (0x213C38, ('\u{5DE8}', false)), // East Asian ideograph
    (0x6F5156, ('\u{BD59}', false)), // Korean hangul
    (0x276041, ('\u{9881}', false)), // East Asian ideograph
    (0x234071, ('\u{9186}', false)), // East Asian ideograph
    (0x21373B, ('\u{5653}', false)), // East Asian ideograph
    (0x234072, ('\u{918C}', false)), // East Asian ideograph
    (0x234073, ('\u{9181}', false)), // East Asian ideograph
    (0x224075, ('\u{6A0C}', false)), // East Asian ideograph
    (0x6F5157, ('\u{BD64}', false)), // Korean hangul
    (0x224076, ('\u{6A0F}', false)), // East Asian ideograph
    (0x21373C, ('\u{563F}', false)), // East Asian ideograph
    (0x4B3F74, ('\u{623B}', false)), // East Asian ideograph
    (0x282F43, ('\u{6206}', false)), // East Asian ideograph
    (0x454738, ('\u{6CFA}', false)), // East Asian ideograph
    (0x275E6A, ('\u{9610}', false)), // East Asian ideograph
    (0x274F36, ('\u{5E0C}', false)), // East Asian ideograph
    (0x6F5E21, ('\u{D79D}', false)), // Korean hangul
    (0x232B24, ('\u{876A}', false)), // East Asian ideograph
    (0x212B25, ('\u{300C}', false)), // Ideographic left corner bracket
    (0x2F5158, ('\u{7CC7}', false)), // East Asian ideograph
    (0x23407B, ('\u{9191}', false)), // East Asian ideograph
    (0x225D55, ('\u{754A}', false)), // East Asian ideograph
    (0x294628, ('\u{9552}', false)), // East Asian ideograph
    (0x4B3050, ('\u{4E8A}', false)), // East Asian ideograph
    (0x22407C, ('\u{69EE}', false)), // East Asian ideograph
    (0x232B27, ('\u{874E}', false)), // East Asian ideograph
    (0x695B37, ('\u{6737}', false)), // East Asian ideograph
    (0x23407D, ('\u{9190}', false)), // East Asian ideograph
    (0x23407E, ('\u{918E}', false)), // East Asian ideograph
    (0x4C6775, ('\u{7962}', false)), // Unrelated variant of EACC 293032 which maps to 7962
    (0x6F5159, ('\u{BD81}', false)), // Korean hangul
    (0x21373E, ('\u{5637}', false)), // East Asian ideograph
    (0x294629, ('\u{84E5}', false)), // East Asian ideograph
    (0x4B3051, ('\u{5F10}', false)), // East Asian ideograph
    (0x6F503B, ('\u{BAA9}', false)), // Korean hangul
    (0x222B2D, ('\u{602B}', false)), // East Asian ideograph
    (0x6F5B50, ('\u{D290}', false)), // Korean hangul
    (0x455847, ('\u{8A25}', false)), // East Asian ideograph (variant of 215847 which maps to 8A25)
    (0x396B2F, ('\u{521F}', false)), // East Asian ideograph
    (0x6F515A, ('\u{BD84}', false)), // Korean hangul
    (0x6F5861, ('\u{CAD9}', false)), // Korean hangul
    (0x222B30, ('\u{6019}', false)), // East Asian ideograph
    (0x225D57, ('\u{754E}', false)), // East Asian ideograph
    (0x4B3052, ('\u{6275}', false)), // East Asian ideograph
    (0x212B31, ('\u{FF3B}', false)), // Ideographic left square bracket
    (0x4B3749, ('\u{5668}', false)), // East Asian ideograph (variant of 213749 which maps to 5668)
    (0x3A3B7D, ('\u{67B1}', false)), // East Asian ideograph
    (0x216B33, ('\u{5231}', false)), // East Asian ideograph
    (0x23504A, ('\u{98BF}', false)), // East Asian ideograph
    (0x4B5F35, ('\u{6B92}', false)), // East Asian ideograph
    (0x474270, ('\u{94BC}', false)), // East Asian ideograph
    (0x6F515B, ('\u{BD87}', false)), // Korean hangul
    (0x212B35, ('\u{3001}', false)), // Ideographic comma
    (0x216B36, ('\u{5235}', false)), // East Asian ideograph
    (0x4B6268, ('\u{9ED9}', false)), // East Asian ideograph
    (0x3F404F, ('\u{638A}', false)), // East Asian ideograph
    (0x695B7B, ('\u{6926}', false)), // East Asian ideograph
    (0x222B38, ('\u{601B}', false)), // East Asian ideograph
    (0x216B39, ('\u{5233}', false)), // East Asian ideograph
    (0x6F485F, ('\u{AC00}', false)), // Korean hangul
    (0x276047, ('\u{988A}', false)), // East Asian ideograph
    (0x212B3A, ('\u{FF1A}', false)), // Ideographic colon
    (0x225D59, ('\u{754B}', false)), // East Asian ideograph
    (0x333F3F, ('\u{51F4}', false)), // East Asian ideograph
    (0x212B3B, ('\u{FF1F}', false)), // Ideographic question mark
    (0x2D3852, ('\u{51A2}', false)), // East Asian ideograph
    (0x295739, ('\u{9C9E}', false)), // East Asian ideograph
    (0x222B3D, ('\u{6033}', false)), // East Asian ideograph
    (0x276B3E, ('\u{522D}', false)), // East Asian ideograph
    (0x6F515D, ('\u{BD89}', false)), // Korean hangul
    (0x276048, ('\u{9888}', false)), // East Asian ideograph
    (0x275A28, ('\u{8D42}', false)), // East Asian ideograph
    (0x225D5A, ('\u{7548}', false)), // East Asian ideograph
    (0x29462D, ('\u{9549}', false)), // East Asian ideograph
    (0x6F4B45, ('\u{B07D}', false)), // Korean hangul
    (0x274F3C, ('\u{79F0}', false)), // East Asian ideograph
    (0x706B42, ('\u{80BC}', false)), // East Asian ideograph
    (0x6F515E, ('\u{BD90}', false)), // Korean hangul
    (0x276049, ('\u{9891}', false)), // East Asian ideograph
    (0x217B75, ('\u{5AA0}', false)), // East Asian ideograph
    (0x706B44, ('\u{80BD}', false)), // East Asian ideograph
    (0x33615A, ('\u{8EB0}', false)), // East Asian ideograph
    (0x222B45, ('\u{600D}', false)), // East Asian ideograph
    (0x2D3854, ('\u{5896}', false)), // East Asian ideograph
    (0x274F3D, ('\u{79CD}', false)), // East Asian ideograph
    (0x28533C, ('\u{709D}', false)), // East Asian ideograph
    (0x69573B, ('\u{5F41}', false)), // East Asian ideograph
    (0x216B47, ('\u{5260}', false)), // East Asian ideograph
    (0x6F5B51, ('\u{D291}', false)), // Korean hangul
    (0x6F515F, ('\u{BD91}', false)), // Korean hangul
    (0x27604A, ('\u{9893}', false)), // East Asian ideograph
    (0x213744, ('\u{5678}', false)), // East Asian ideograph
    (0x4B3057, ('\u{4E99}', false)), // East Asian ideograph
    (0x6F2477, ('\u{3154}', false)), // Korean hangul
    (0x2F4053, ('\u{914F}', false)), // East Asian ideograph
    (0x226B4B, ('\u{7B37}', false)), // East Asian ideograph
    (0x29573C, ('\u{9C91}', false)), // East Asian ideograph
    (0x706B4C, ('\u{80E9}', false)), // East Asian ideograph
    (0x4B5F3A, ('\u{967A}', false)), // East Asian ideograph
    (0x216B4D, ('\u{525E}', false)), // East Asian ideograph
    (0x6F5160, ('\u{BD93}', false)), // Korean hangul
    (0x6F5940, ('\u{CCAD}', false)), // Korean hangul
    (0x29573D, ('\u{9C92}', false)), // East Asian ideograph
    (0x6F5161, ('\u{BD95}', false)), // Korean hangul
    (0x216B53, ('\u{5255}', false)), // East Asian ideograph
    (0x705F50, ('\u{549D}', false)), // East Asian ideograph
    (0x2E6B54, ('\u{7B04}', false)), // East Asian ideograph
    (0x292B55, ('\u{86F3}', false)), // East Asian ideograph
    (0x6F555A, ('\u{C5E0}', false)), // Korean hangul
    (0x70622A, ('\u{7339}', false)), // East Asian ideograph
    (0x6F5162, ('\u{BD99}', false)), // Korean hangul
    (0x212B59, ('\u{FF0F}', false)), // Ideographic solidus
    (0x2D562E, ('\u{8024}', false)), // East Asian ideograph
    (0x213563, ('\u{5439}', false)), // East Asian ideograph
    (0x216B5B, ('\u{526E}', false)), // East Asian ideograph
    (0x6F4B67, ('\u{B0C8}', false)), // Korean hangul
    (0x4B3D24, ('\u{53A6}', false)), // East Asian ideograph
    (0x6F5163, ('\u{BD9C}', false)), // Korean hangul
    (0x225D60, ('\u{755B}', false)), // East Asian ideograph
    (0x276B5F, ('\u{672D}', false)), // East Asian ideograph
    (0x2D433E, ('\u{667B}', false)), // East Asian ideograph
    (0x235053, ('\u{98C6}', false)), // East Asian ideograph
    (0x345452, ('\u{7118}', false)), // East Asian ideograph
    (0x6F5B40, ('\u{D1D8}', false)), // Korean hangul
    (0x213569, ('\u{5462}', false)), // East Asian ideograph
    (0x4C6B62, ('\u{7B4C}', false)), // East Asian ideograph (variant of 226B62 which maps to 7B4C)
    (0x394634, ('\u{6B96}', false)), // East Asian ideograph (variant of 214634 which maps to 6B96)
    (0x274171, ('\u{629A}', false)), // East Asian ideograph
    (0x6F5165, ('\u{BDF0}', false)), // Korean hangul
    (0x28356D, ('\u{6512}', false)), // East Asian ideograph
    (0x274F44, ('\u{79EF}', false)), // East Asian ideograph
    (0x295742, ('\u{9C95}', false)), // East Asian ideograph
    (0x4B3D27, ('\u{5EC3}', false)), // East Asian ideograph
    (0x6F5166, ('\u{BE0C}', false)), // Korean hangul
    (0x6F4D23, ('\u{B310}', false)), // Korean hangul
    (0x213B61, ('\u{5C64}', false)), // East Asian ideograph (variant of 4B3B61 which maps to 5C64)
    (0x4B5277, ('\u{8068}', false)), // East Asian ideograph
    (0x336162, ('\u{9A23}', false)), // East Asian ideograph
    (0x705F51, ('\u{54D0}', false)), // East Asian ideograph
    (0x6F4A46, ('\u{AE50}', false)), // Korean hangul
    (0x292B6E, ('\u{86F0}', false)), // East Asian ideograph
    (0x2D4A60, ('\u{6C02}', false)), // East Asian ideograph
    (0x222B6F, ('\u{604C}', false)), // East Asian ideograph
    (0x6F5167, ('\u{BE0D}', false)), // Korean hangul
    (0x276052, ('\u{989B}', false)), // East Asian ideograph
    (0x6F4D24, ('\u{B311}', false)), // Korean hangul
    (0x232B72, ('\u{87AC}', false)), // East Asian ideograph
    (0x6F496C, ('\u{AD49}', false)), // Korean hangul
    (0x216B74, ('\u{5282}', false)), // East Asian ideograph
    (0x2D5F2E, ('\u{661C}', false)), // East Asian ideograph
    (0x216B75, ('\u{5281}', false)), // East Asian ideograph
    (0x6F5168, ('\u{BE10}', false)), // Korean hangul
    (0x215966, ('\u{8C8C}', false)), // East Asian ideograph
    (0x6F5621, ('\u{C641}', false)), // Korean hangul
    (0x33515C, ('\u{7DAB}', false)), // East Asian ideograph
    (0x275622, ('\u{8427}', false)), // East Asian ideograph
    (0x215623, ('\u{859B}', false)), // East Asian ideograph
    (0x226B79, ('\u{7B72}', false)), // East Asian ideograph
    (0x215624, ('\u{8591}', false)), // East Asian ideograph
    (0x2D496B, ('\u{70DF}', false)), // East Asian ideograph
    (0x226B7A, ('\u{7B78}', false)), // East Asian ideograph
    (0x6F5169, ('\u{BE14}', false)), // Korean hangul
    (0x275626, ('\u{8537}', false)), // East Asian ideograph
    (0x23487C, ('\u{9481}', false)), // East Asian ideograph
    (0x226B7C, ('\u{7B67}', false)), // East Asian ideograph
    (0x6F5627, ('\u{C654}', false)), // Korean hangul
    (0x2D5635, ('\u{846F}', false)), // East Asian ideograph
    (0x215628, ('\u{8587}', false)), // East Asian ideograph
    (0x29352D, ('\u{8C30}', false)), // East Asian ideograph
    (0x275629, ('\u{84DD}', false)), // East Asian ideograph
    (0x21562A, ('\u{85A9}', false)), // East Asian ideograph
    (0x276055, ('\u{613F}', false)), // East Asian ideograph
    (0x6F4D27, ('\u{B315}', false)), // Korean hangul
    (0x225D67, ('\u{7563}', false)), // East Asian ideograph
    (0x273D2F, ('\u{5385}', false)), // East Asian ideograph
    (0x335D23, ('\u{8A76}', false)), // East Asian ideograph
    (0x6F562D, ('\u{C678}', false)), // Korean hangul
    (0x2E2968, ('\u{5F51}', false)), // East Asian ideograph
    (0x21562E, ('\u{85C9}', false)), // East Asian ideograph
    (0x4B3D2C, ('\u{53B0}', false)), // East Asian ideograph
    (0x21562F, ('\u{85B0}', false)), // East Asian ideograph
    (0x4B527C, ('\u{8080}', false)), // East Asian ideograph
    (0x233F4E, ('\u{9100}', false)), // East Asian ideograph
    (0x4B4E39, ('\u{5CFA}', false)), // East Asian ideograph
    (0x275631, ('\u{827A}', false)), // East Asian ideograph
    (0x215632, ('\u{85EA}', false)), // East Asian ideograph
    (0x695633, ('\u{5CBE}', false)), // East Asian ideograph
    (0x6F516C, ('\u{BE1F}', false)), // Korean hangul
    (0x21596A, ('\u{8CA0}', false)), // East Asian ideograph
    (0x213751, ('\u{5687}', false)), // East Asian ideograph
    (0x275635, ('\u{836F}', false)), // East Asian ideograph
    (0x6F5529, ('\u{C558}', false)), // Korean hangul
    (0x235636, ('\u{9B43}', false)), // East Asian ideograph
    (0x275637, ('\u{853C}', false)), // East Asian ideograph
    (0x6F5C2E, ('\u{D39C}', false)), // Korean hangul
    (0x4C5638, ('\u{729F}', false)), // East Asian ideograph
    (0x275639, ('\u{853A}', false)), // East Asian ideograph
    (0x21563A, ('\u{8606}', false)), // East Asian ideograph
    (0x233F50, ('\u{9107}', false)), // East Asian ideograph
    (0x225927, ('\u{73EA}', false)), // East Asian ideograph
    (0x21563B, ('\u{860B}', false)), // East Asian ideograph
    (0x6F5A22, ('\u{CEAC}', false)), // Korean hangul
    (0x21563C, ('\u{8607}', false)), // East Asian ideograph
    (0x224C5E, ('\u{6F36}', false)), // East Asian ideograph
    (0x21563D, ('\u{860A}', false)), // East Asian ideograph
    (0x4B3D2F, ('\u{5EF0}', false)), // East Asian ideograph
    (0x696576, ('\u{7E90}', false)), // East Asian ideograph
    (0x21563E, ('\u{862D}', false)), // East Asian ideograph
    (0x276059, ('\u{9885}', false)), // East Asian ideograph
    (0x21596C, ('\u{8CA1}', false)), // East Asian ideograph
    (0x2D563F, ('\u{6A97}', false)), // East Asian ideograph
    (0x276023, ('\u{5DE9}', false)), // East Asian ideograph
    (0x275640, ('\u{85D3}', false)), // East Asian ideograph
    (0x346126, ('\u{6900}', false)), // East Asian ideograph
    (0x2D3421, ('\u{5294}', false)), // East Asian ideograph
    (0x235641, ('\u{9B4B}', false)), // East Asian ideograph
    (0x215642, ('\u{863F}', false)), // East Asian ideograph
    (0x4B5F49, ('\u{51CB}', false)), // East Asian ideograph
    (0x2D4971, ('\u{70A4}', false)), // East Asian ideograph
    (0x395643, ('\u{4E55}', false)), // East Asian ideograph
    (0x6F4D2C, ('\u{B35C}', false)), // Korean hangul
    (0x213754, ('\u{5695}', false)), // East Asian ideograph
    (0x275644, ('\u{4E47}', false)), // East Asian ideograph
    (0x70602D, ('\u{55B9}', false)), // East Asian ideograph
    (0x692426, ('\u{3046}', false)), // Hiragana letter U
    (0x2D5A7E, ('\u{8E7B}', false)), // East Asian ideograph
    (0x6F5645, ('\u{C6CC}', false)), // Korean hangul
    (0x6F5646, ('\u{C6CD}', false)), // Korean hangul
    (0x2D572B, ('\u{8797}', false)), // East Asian ideograph
    (0x275647, ('\u{5904}', false)), // East Asian ideograph
    (0x215648, ('\u{865C}', false)), // East Asian ideograph
    (0x27605B, ('\u{98CE}', false)), // East Asian ideograph
    (0x28645A, ('\u{7817}', false)), // East Asian ideograph
    (0x6F4D2D, ('\u{B35F}', false)), // Korean hangul
    (0x225D6D, ('\u{7579}', false)), // East Asian ideograph
    (0x21564A, ('\u{865F}', false)), // East Asian ideograph
    (0x2D563C, ('\u{8613}', false)), // East Asian ideograph
    (0x45564B, ('\u{865E}', false)), // East Asian ideograph (variant of 21564B which maps to 865E)
    (0x21564C, ('\u{8667}', false)), // East Asian ideograph
    (0x6F5171, ('\u{BE4C}', false)), // Korean hangul
    (0x27605C, ('\u{98D2}', false)), // East Asian ideograph
    (0x69564E, ('\u{5D76}', false)), // East Asian ideograph
    (0x29302D, ('\u{88E3}', false)), // East Asian ideograph
    (0x22564F, ('\u{72B4}', false)), // East Asian ideograph
    (0x6F496E, ('\u{AD6C}', false)), // Korean hangul
    (0x277169, ('\u{5522}', false)), // East Asian ideograph
    (0x6F5650, ('\u{C6EC}', false)), // Korean hangul
    (0x6F5C2F, ('\u{D3A0}', false)), // Korean hangul
    (0x235061, ('\u{98E4}', false)), // East Asian ideograph
    (0x4B5F4C, ('\u{9D8F}', false)), // East Asian ideograph
    (0x225652, ('\u{72B5}', false)), // East Asian ideograph
    (0x27605D, ('\u{53F0}', false)), // East Asian ideograph (duplicate simplified)
    (0x6F4D2F, ('\u{B365}', false)), // Korean hangul
    (0x6F5653, ('\u{C704}', false)), // Korean hangul
    (0x294642, ('\u{94E0}', false)), // East Asian ideograph
    (0x692429, ('\u{3049}', false)), // Hiragana letter small O
    (0x333F55, ('\u{5B3E}', false)), // East Asian ideograph
    (0x6F5654, ('\u{C705}', false)), // Korean hangul
    (0x6F5655, ('\u{C708}', false)), // Korean hangul
    (0x225656, ('\u{72BC}', false)), // East Asian ideograph
    (0x695657, ('\u{5D90}', false)), // East Asian ideograph
    (0x27605E, ('\u{522E}', false)), // East Asian ideograph
    (0x6F4D30, ('\u{B367}', false)), // Korean hangul
    (0x225658, ('\u{72C3}', false)), // East Asian ideograph
    (0x217747, ('\u{5853}', false)), // East Asian ideograph
    (0x6F5659, ('\u{C719}', false)), // Korean hangul
    (0x21565A, ('\u{86CB}', false)), // East Asian ideograph
    (0x293537, ('\u{8C20}', false)), // East Asian ideograph
    (0x235063, ('\u{98E5}', false)), // East Asian ideograph
    (0x6F5174, ('\u{BE55}', false)), // Korean hangul
    (0x27605F, ('\u{98D3}', false)), // East Asian ideograph
    (0x6F4D31, ('\u{B368}', false)), // Korean hangul
    (0x23565D, ('\u{9B74}', false)), // East Asian ideograph
    (0x6F4B23, ('\u{AFB9}', false)), // Korean hangul
    (0x4B306C, ('\u{96E0}', false)), // East Asian ideograph
    (0x6F565E, ('\u{C730}', false)), // Korean hangul
    (0x6F5638, ('\u{C6A7}', false)), // Korean hangul
    (0x6F565F, ('\u{C735}', false)), // Korean hangul
    (0x4C6074, ('\u{76B9}', false)), // East Asian ideograph
    (0x224C65, ('\u{6F2D}', false)), // East Asian ideograph
    (0x215660, ('\u{86DF}', false)), // East Asian ideograph
    (0x4B5052, ('\u{7C56}', false)), // East Asian ideograph
    (0x6F5175, ('\u{BE57}', false)), // Korean hangul
    (0x6F4D32, ('\u{B369}', false)), // Korean hangul
    (0x213B64, ('\u{5C6F}', false)), // East Asian ideograph
    (0x6F5662, ('\u{C73D}', false)), // Korean hangul
    (0x396223, ('\u{9BFD}', false)), // East Asian ideograph (variant of 216223)
    (0x333F58, ('\u{61F4}', false)), // East Asian ideograph
    (0x235663, ('\u{9B68}', false)), // East Asian ideograph
    (0x2D3428, ('\u{5226}', false)), // East Asian ideograph
    (0x2E5A40, ('\u{73B3}', false)), // East Asian ideograph
    (0x215664, ('\u{86DB}', false)), // East Asian ideograph
    (0x6F583B, ('\u{C9E2}', false)), // Korean hangul
    (0x293539, ('\u{8C33}', false)), // East Asian ideograph
    (0x215665, ('\u{86E4}', false)), // East Asian ideograph
    (0x4B5F50, ('\u{96E3}', false)), // East Asian ideograph
    (0x4B3B37, ('\u{51A9}', false)), // East Asian ideograph
    (0x6F5176, ('\u{BE59}', false)), // Korean hangul
    (0x2F2F5D, ('\u{7E48}', false)), // East Asian ideograph
    (0x276061, ('\u{98D5}', false)), // East Asian ideograph
    (0x286460, ('\u{7856}', false)), // East Asian ideograph
    (0x21375B, ('\u{56B6}', false)), // East Asian ideograph
    (0x215667, ('\u{86F9}', false)), // East Asian ideograph
    (0x225930, ('\u{73DB}', false)), // East Asian ideograph
    (0x6F5668, ('\u{C751}', false)), // Korean hangul
    (0x22403D, ('\u{6A12}', false)), // East Asian ideograph
    (0x6F5669, ('\u{C758}', false)), // Korean hangul
    (0x224C67, ('\u{6F34}', false)), // East Asian ideograph
    (0x4B566A, ('\u{8708}', false)), // East Asian ideograph (variant of 21566A which maps to 8708)
    (0x21566B, ('\u{8700}', false)), // East Asian ideograph
    (0x276062, ('\u{98D8}', false)), // East Asian ideograph
    (0x285E7A, ('\u{75D6}', false)), // East Asian ideograph
    (0x6F4D34, ('\u{B36B}', false)), // Korean hangul
    (0x22566C, ('\u{72CC}', false)), // East Asian ideograph
    (0x294647, ('\u{954F}', false)), // East Asian ideograph
    (0x6F566D, ('\u{C77C}', false)), // Korean hangul
    (0x334730, ('\u{6E5F}', false)), // East Asian ideograph
    (0x6F5041, ('\u{BABB}', false)), // Korean hangul
    (0x22566E, ('\u{72DB}', false)), // East Asian ideograph
    (0x22566F, ('\u{72CD}', false)), // East Asian ideograph
    (0x21356D, ('\u{5496}', false)), // East Asian ideograph
    (0x276063, ('\u{98DE}', false)), // East Asian ideograph
    (0x6F4D35, ('\u{B36E}', false)), // Korean hangul
    (0x21375D, ('\u{56C1}', false)), // East Asian ideograph
    (0x225D75, ('\u{7571}', false)), // East Asian ideograph
    (0x333F5B, ('\u{6133}', false)), // East Asian ideograph
    (0x235672, ('\u{9B80}', false)), // East Asian ideograph
    (0x235673, ('\u{9B8C}', false)), // East Asian ideograph
    (0x2F5E42, ('\u{9EC9}', false)), // East Asian ideograph
    (0x6F5674, ('\u{C789}', false)), // Korean hangul
    (0x6F543C, ('\u{C318}', false)), // Korean hangul
    (0x2D5675, ('\u{9F05}', false)), // East Asian ideograph
    (0x6F4D36, ('\u{B370}', false)), // Korean hangul
    (0x21375E, ('\u{56C2}', false)), // East Asian ideograph
    (0x275676, ('\u{8680}', false)), // East Asian ideograph
    (0x692430, ('\u{3050}', false)), // Hiragana letter GU
    (0x4D386F, ('\u{544B}', false)), // East Asian ideograph
    (0x2D4122, ('\u{6485}', false)), // East Asian ideograph
    (0x224123, ('\u{69F0}', false)), // East Asian ideograph
    (0x295756, ('\u{9CA9}', false)), // East Asian ideograph
    (0x215679, ('\u{8774}', false)), // East Asian ideograph
    (0x224124, ('\u{69F2}', false)), // East Asian ideograph
    (0x21567A, ('\u{8766}', false)), // East Asian ideograph
    (0x234125, ('\u{9193}', false)), // East Asian ideograph
    (0x6F4D37, ('\u{B371}', false)), // Korean hangul
    (0x23567B, ('\u{9B7D}', false)), // East Asian ideograph
    (0x21774E, ('\u{5856}', false)), // East Asian ideograph
    (0x4B3072, ('\u{4EED}', false)), // East Asian ideograph
    (0x6F4A4A, ('\u{AE60}', false)), // Korean hangul
    (0x33567C, ('\u{8671}', false)), // East Asian ideograph
    (0x6F567D, ('\u{C798}', false)), // Korean hangul
    (0x224128, ('\u{6A14}', false)), // East Asian ideograph
    (0x21567E, ('\u{8757}', false)), // East Asian ideograph
    (0x224129, ('\u{6A63}', false)), // East Asian ideograph
    (0x295030, ('\u{989E}', false)), // East Asian ideograph
    (0x6F517B, ('\u{BE60}', false)), // Korean hangul
    (0x4B412A, ('\u{6323}', false)), // East Asian ideograph
    (0x6F4D38, ('\u{B374}', false)), // Korean hangul
    (0x225742, ('\u{731E}', false)), // East Asian ideograph
    (0x23412B, ('\u{919D}', false)), // East Asian ideograph
    (0x212B33, ('\u{3002}', false)), // Ideographic full stop
    (0x23412C, ('\u{919A}', false)), // East Asian ideograph
    (0x2D342E, ('\u{8274}', false)), // East Asian ideograph
    (0x6F5538, ('\u{C580}', false)), // Korean hangul
    (0x276067, ('\u{9968}', false)), // East Asian ideograph
    (0x6F4D39, ('\u{B378}', false)), // Korean hangul
    (0x234130, ('\u{91A2}', false)), // East Asian ideograph
    (0x3F476F, ('\u{51C8}', false)), // East Asian ideograph
    (0x334131, ('\u{6425}', false)), // East Asian ideograph
    (0x6F5042, ('\u{BABD}', false)), // Korean hangul
    (0x2D4132, ('\u{642F}', false)), // East Asian ideograph
    (0x287130, ('\u{7EDB}', false)), // East Asian ideograph
    (0x234C29, ('\u{9708}', false)), // East Asian ideograph
    (0x6F517D, ('\u{BE64}', false)), // Korean hangul
    (0x234134, ('\u{919B}', false)), // East Asian ideograph (variant of 4D4134 which maps to 919B)
    (0x6F4D3A, ('\u{B380}', false)), // Korean hangul
    (0x213762, ('\u{56C8}', false)), // East Asian ideograph
    (0x336179, ('\u{9C7B}', false)), // East Asian ideograph
    (0x4C3474, ('\u{631D}', false)), // East Asian ideograph
    (0x235D36, ('\u{9E15}', false)), // East Asian ideograph
    (0x274136, ('\u{62A1}', false)), // East Asian ideograph
    (0x274F5C, ('\u{6D3C}', false)), // East Asian ideograph
    (0x6F4F68, ('\u{B9CC}', false)), // Korean hangul
    (0x224137, ('\u{6A67}', false)), // East Asian ideograph
    (0x344138, ('\u{8022}', false)), // East Asian ideograph
    (0x6F517E, ('\u{BE68}', false)), // Korean hangul
    (0x224139, ('\u{6A43}', false)), // East Asian ideograph
    (0x6F4D3B, ('\u{B383}', false)), // Korean hangul
    (0x22413A, ('\u{6A33}', false)), // East Asian ideograph
    (0x225938, ('\u{73E3}', false)), // East Asian ideograph
    (0x22413B, ('\u{6A32}', false)), // East Asian ideograph
    (0x274F5D, ('\u{7A9D}', false)), // East Asian ideograph
    (0x27413C, ('\u{62E3}', false)), // East Asian ideograph
    (0x706247, ('\u{9987}', false)), // East Asian ideograph
    (0x23413D, ('\u{91AA}', false)), // East Asian ideograph
    (0x27606A, ('\u{996E}', false)), // East Asian ideograph
    (0x6F4D3C, ('\u{B385}', false)), // Korean hangul
    (0x213B66, ('\u{5C79}', false)), // East Asian ideograph
    (0x213764, ('\u{56D1}', false)), // East Asian ideograph
    (0x22413F, ('\u{6A28}', false)), // East Asian ideograph
    (0x213321, ('\u{5167}', false)), // East Asian ideograph
    (0x4C3F68, ('\u{69C7}', false)), // East Asian ideograph
    (0x224140, ('\u{6A48}', false)), // East Asian ideograph
    (0x224141, ('\u{6A50}', false)), // East Asian ideograph
    (0x224142, ('\u{6A52}', false)), // East Asian ideograph
    (0x334C2C, ('\u{754D}', false)), // East Asian ideograph
    (0x336058, ('\u{9855}', false)), // East Asian ideograph
    (0x224143, ('\u{6A72}', false)), // East Asian ideograph
    (0x6F4D3D, ('\u{B38C}', false)), // Korean hangul
    (0x274570, ('\u{6743}', false)), // East Asian ideograph
    (0x285836, ('\u{7315}', false)), // East Asian ideograph
    (0x224145, ('\u{6A3E}', false)), // East Asian ideograph
    (0x224146, ('\u{6A77}', false)), // East Asian ideograph
    (0x287134, ('\u{7ED7}', false)), // East Asian ideograph
    (0x224147, ('\u{6A5B}', false)), // East Asian ideograph
    (0x214148, ('\u{63EA}', false)), // East Asian ideograph
    (0x6F4D3E, ('\u{B3C4}', false)), // Korean hangul
    (0x225D7E, ('\u{757F}', false)), // East Asian ideograph
    (0x217755, ('\u{589A}', false)), // East Asian ideograph
    (0x2D3877, ('\u{5900}', false)), // East Asian ideograph
    (0x22414A, ('\u{6A5E}', false)), // East Asian ideograph
    (0x21414B, ('\u{643E}', false)), // East Asian ideograph
    (0x21414C, ('\u{6413}', false)), // East Asian ideograph
    (0x23414D, ('\u{91B5}', false)), // East Asian ideograph
    (0x3F4A60, ('\u{7266}', false)), // East Asian ideograph
    (0x6F4D3F, ('\u{B3C5}', false)), // Korean hangul
    (0x6F4C78, ('\u{B2F7}', false)), // Korean hangul
    (0x335D3B, ('\u{57DC}', false)), // East Asian ideograph
    (0x22414F, ('\u{6A51}', false)), // East Asian ideograph
    (0x2D4150, ('\u{6428}', false)), // East Asian ideograph
    (0x29575F, ('\u{9CA0}', false)), // East Asian ideograph
    (0x224151, ('\u{6A56}', false)), // East Asian ideograph
    (0x2D4152, ('\u{6447}', false)), // East Asian ideograph
    (0x6F4D40, ('\u{B3C8}', false)), // Korean hangul
    (0x224153, ('\u{6A36}', false)), // East Asian ideograph
    (0x2D4154, ('\u{635C}', false)), // East Asian ideograph
    (0x2D3436, ('\u{52F3}', false)), // East Asian ideograph
    (0x274155, ('\u{62A2}', false)), // East Asian ideograph
    (0x217C30, ('\u{5A93}', false)), // East Asian ideograph
    (0x224156, ('\u{6A7A}', false)), // East Asian ideograph
    (0x234157, ('\u{91BD}', false)), // East Asian ideograph
    (0x6F4D41, ('\u{B3CB}', false)), // Korean hangul
    (0x213769, ('\u{56E4}', false)), // East Asian ideograph
    (0x224158, ('\u{6A3F}', false)), // East Asian ideograph
    (0x4B4F7B, ('\u{7B7A}', false)), // East Asian ideograph
    (0x21623B, ('\u{9D12}', false)), // East Asian ideograph (variant of 4B623B which maps to 9D12)
    (0x4B523E, ('\u{7F9A}', false)), // East Asian ideograph (variant of 21523E which maps to 7F9A)
    (0x23415A, ('\u{91C2}', false)), // East Asian ideograph
    (0x293651, ('\u{8D36}', false)), // East Asian ideograph
    (0x23415B, ('\u{91C4}', false)), // East Asian ideograph
    (0x33347D, ('\u{53C1}', false)), // East Asian ideograph
    (0x23415C, ('\u{91C3}', false)), // East Asian ideograph
    (0x275A30, ('\u{8D24}', false)), // East Asian ideograph
    (0x29415D, ('\u{917D}', false)), // East Asian ideograph
    (0x29586A, ('\u{9CCB}', false)), // East Asian ideograph
    (0x274F64, ('\u{7A83}', false)), // East Asian ideograph
    (0x27415F, ('\u{6402}', false)), // East Asian ideograph
    (0x70624E, ('\u{9995}', false)), // East Asian ideograph
    (0x224C76, ('\u{6EFA}', false)), // East Asian ideograph
    (0x224833, ('\u{6CB4}', false)), // East Asian ideograph
    (0x4B5164, ('\u{770C}', false)), // East Asian ideograph
    (0x4C4146, ('\u{8538}', false)), // East Asian ideograph
    (0x234161, ('\u{91D4}', false)), // East Asian ideograph
    (0x284257, ('\u{68BC}', false)), // East Asian ideograph
    (0x294656, ('\u{955B}', false)), // East Asian ideograph
    (0x234162, ('\u{91D3}', false)), // East Asian ideograph
    (0x234163, ('\u{91D5}', false)), // East Asian ideograph
    (0x217639, ('\u{580E}', false)), // East Asian ideograph
    (0x234164, ('\u{91D9}', false)), // East Asian ideograph
    (0x274B22, ('\u{72EF}', false)), // East Asian ideograph
    (0x6F5B59, ('\u{D2B8}', false)), // Korean hangul
    (0x274165, ('\u{635E}', false)), // East Asian ideograph
    (0x286272, ('\u{770D}', false)), // East Asian ideograph
    (0x274166, ('\u{62E8}', false)), // East Asian ideograph
    (0x6F4D44, ('\u{B3D4}', false)), // Korean hangul
    (0x234168, ('\u{91E2}', false)), // East Asian ideograph
    (0x6F534A, ('\u{C1A9}', false)), // Korean hangul
    (0x234169, ('\u{91ED}', false)), // East Asian ideograph
    (0x23416A, ('\u{91F7}', false)), // East Asian ideograph
    (0x333D2F, ('\u{5E81}', false)), // East Asian ideograph
    (0x23416B, ('\u{91FA}', false)), // East Asian ideograph
    (0x6F4D45, ('\u{B3D5}', false)), // Korean hangul
    (0x21775C, ('\u{5889}', false)), // East Asian ideograph
    (0x22416C, ('\u{69F9}', false)), // East Asian ideograph
    (0x215543, ('\u{83CC}', false)), // East Asian ideograph
    (0x4B4E56, ('\u{78FA}', false)), // East Asian ideograph
    (0x22416D, ('\u{6A64}', false)), // East Asian ideograph
    (0x295427, ('\u{9A85}', false)), // East Asian ideograph
    (0x27416E, ('\u{6251}', false)), // East Asian ideograph
    (0x217C31, ('\u{5AAC}', false)), // East Asian ideograph
    (0x23416F, ('\u{91F2}', false)), // East Asian ideograph
    (0x234171, ('\u{91E8}', false)), // East Asian ideograph
    (0x294458, ('\u{952C}', false)), // East Asian ideograph
    (0x4B434D, ('\u{663F}', false)), // East Asian ideograph
    (0x234172, ('\u{91F6}', false)), // East Asian ideograph
    (0x2D343C, ('\u{52A2}', false)), // East Asian ideograph
    (0x334C3E, ('\u{8E08}', false)), // East Asian ideograph
    (0x234173, ('\u{91EE}', false)), // East Asian ideograph
    (0x274174, ('\u{62E5}', false)), // East Asian ideograph
    (0x4B3D4B, ('\u{5F3E}', false)), // East Asian ideograph
    (0x334C36, ('\u{753B}', false)), // East Asian ideograph (variant of 274C36 which maps to 753B)
    (0x4B4C67, ('\u{761F}', false)), // East Asian ideograph (variant of 214C67 which maps to 761F)
    (0x224175, ('\u{6AA8}', false)), // East Asian ideograph
    (0x29465A, ('\u{955F}', false)), // East Asian ideograph
    (0x274176, ('\u{51FB}', false)), // East Asian ideograph
    (0x692441, ('\u{3061}', false)), // Hiragana letter TI
    (0x21332C, ('\u{5178}', false)), // East Asian ideograph
    (0x235D43, ('\u{9E7B}', false)), // East Asian ideograph
    (0x224177, ('\u{6AA5}', false)), // East Asian ideograph
    (0x2D343D, ('\u{52E7}', false)), // East Asian ideograph
    (0x224179, ('\u{6A96}', false)), // East Asian ideograph
    (0x4B3D4C, ('\u{5F25}', false)), // East Asian ideograph (variant of 273D4C)
    (0x222C24, ('\u{608A}', false)), // East Asian ideograph
    (0x27417A, ('\u{6321}', false)), // East Asian ideograph
    (0x6F4D48, ('\u{B3DB}', false)), // Korean hangul
    (0x707360, ('\u{7B7B}', false)), // East Asian ideograph
    (0x286032, ('\u{75AC}', false)), // East Asian ideograph
    (0x21332D, ('\u{517C}', false)), // East Asian ideograph
    (0x27417C, ('\u{636E}', false)), // East Asian ideograph
    (0x216C27, ('\u{5296}', false)), // East Asian ideograph
    (0x27417D, ('\u{63B3}', false)), // East Asian ideograph
    (0x284F26, ('\u{6CF7}', false)), // East Asian ideograph
    (0x224A44, ('\u{6E12}', false)), // East Asian ideograph
    (0x22417E, ('\u{6A7D}', false)), // East Asian ideograph
    (0x6F5D6A, ('\u{D750}', false)), // Korean hangul
    (0x226C29, ('\u{7B73}', false)), // East Asian ideograph
    (0x2E2B74, ('\u{609B}', false)), // East Asian ideograph
    (0x276077, ('\u{997F}', false)), // East Asian ideograph
    (0x6F4D49, ('\u{B3FC}', false)), // Korean hangul
    (0x217760, ('\u{589B}', false)), // East Asian ideograph
    (0x223378, ('\u{647D}', false)), // East Asian ideograph
    (0x232C2C, ('\u{87EE}', false)), // East Asian ideograph
    (0x274B28, ('\u{72DE}', false)), // East Asian ideograph
    (0x222C2F, ('\u{609E}', false)), // East Asian ideograph
    (0x6F4B28, ('\u{AFC9}', false)), // Korean hangul
    (0x217761, ('\u{587C}', false)), // East Asian ideograph
    (0x222C30, ('\u{6083}', false)), // East Asian ideograph
    (0x4B4E5B, ('\u{783F}', false)), // East Asian ideograph
    (0x22483B, ('\u{6D28}', false)), // East Asian ideograph
    (0x216C33, ('\u{52AE}', false)), // East Asian ideograph
    (0x4C4345, ('\u{67A6}', false)), // East Asian ideograph
    (0x222C34, ('\u{60A7}', false)), // East Asian ideograph
    (0x6F4D4B, ('\u{B410}', false)), // Korean hangul
    (0x213773, ('\u{5718}', false)), // East Asian ideograph
    (0x233B2E, ('\u{8EC9}', false)), // East Asian ideograph
    (0x216C38, ('\u{52BC}', false)), // East Asian ideograph
    (0x2D454E, ('\u{697D}', false)), // East Asian ideograph
    (0x217763, ('\u{5888}', false)), // East Asian ideograph
    (0x692446, ('\u{3066}', false)), // Hiragana letter TE
    (0x232C3A, ('\u{87D6}', false)), // East Asian ideograph
    (0x235D48, ('\u{9E83}', false)), // East Asian ideograph
    (0x39302D, ('\u{534B}', false)), // East Asian ideograph
    (0x6F4D4D, ('\u{B41C}', false)), // Korean hangul
    (0x213775, ('\u{571F}', false)), // East Asian ideograph
    (0x286037, ('\u{763F}', false)), // East Asian ideograph
    (0x692447, ('\u{3067}', false)), // Hiragana letter DE
    (0x223731, ('\u{65C6}', false)), // East Asian ideograph
    (0x294478, ('\u{9522}', false)), // East Asian ideograph
    (0x274B2C, ('\u{730E}', false)), // East Asian ideograph
    (0x287144, ('\u{7EE0}', false)), // East Asian ideograph
    (0x234C3D, ('\u{971D}', false)), // East Asian ideograph
    (0x706C43, ('\u{70C0}', false)), // East Asian ideograph
    (0x213333, ('\u{5191}', false)), // East Asian ideograph
    (0x333066, ('\u{5FC8}', false)), // East Asian ideograph
    (0x69613A, ('\u{7549}', false)), // East Asian ideograph
    (0x6F4F6C, ('\u{B9D1}', false)), // Korean hangul
    (0x216C46, ('\u{52D4}', false)), // East Asian ideograph
    (0x234C3E, ('\u{9719}', false)), // East Asian ideograph
    (0x453666, ('\u{5AD0}', false)), // East Asian ideograph
    (0x232C48, ('\u{87D3}', false)), // East Asian ideograph
    (0x6F4D4F, ('\u{B428}', false)), // Korean hangul
    (0x6F4B29, ('\u{AFCB}', false)), // Korean hangul
    (0x294662, ('\u{956A}', false)), // East Asian ideograph
    (0x692449, ('\u{3069}', false)), // Hiragana letter DO
    (0x275F50, ('\u{96BE}', false)), // East Asian ideograph
    (0x235D4B, ('\u{9E88}', false)), // East Asian ideograph
    (0x274B2E, ('\u{732E}', false)), // East Asian ideograph
    (0x235A6B, ('\u{9D3D}', false)), // East Asian ideograph
    (0x292C4C, ('\u{866E}', false)), // East Asian ideograph
    (0x4B346B, ('\u{5DF5}', false)), // East Asian ideograph
    (0x69595E, ('\u{63B5}', false)), // East Asian ideograph
    (0x472C4D, ('\u{8801}', false)), // East Asian ideograph (variant of 232C4D which maps to 8801)
    (0x28603A, ('\u{75C8}', false)), // East Asian ideograph
    (0x454774, ('\u{6E15}', false)), // East Asian ideograph
    (0x6F5564, ('\u{C5F7}', false)), // Korean hangul
    (0x694664, ('\u{51EA}', false)), // East Asian ideograph
    (0x294221, ('\u{9495}', false)), // East Asian ideograph
    (0x6F4975, ('\u{AD7C}', false)), // Korean hangul
    (0x292C55, ('\u{86CF}', false)), // East Asian ideograph
    (0x6F5762, ('\u{C8C8}', false)), // Korean hangul
    (0x4B5F6F, ('\u{970A}', false)), // East Asian ideograph
    (0x2D4F37, ('\u{7980}', false)), // East Asian ideograph
    (0x216C58, ('\u{52F0}', false)), // East Asian ideograph
    (0x294222, ('\u{9490}', false)), // East Asian ideograph
    (0x6F5A48, ('\u{CF78}', false)), // Korean hangul
    (0x216C5A, ('\u{52F1}', false)), // East Asian ideograph
    (0x2D632B, ('\u{5C28}', false)), // East Asian ideograph
    (0x4B4135, ('\u{6368}', false)), // East Asian ideograph
    (0x275C3E, ('\u{8FC7}', false)), // East Asian ideograph
    (0x223B7A, ('\u{67F6}', false)), // East Asian ideograph
    (0x69244D, ('\u{306D}', false)), // Hiragana letter NE
    (0x222C5D, ('\u{60C4}', false)), // East Asian ideograph
    (0x3A284C, ('\u{53A9}', false)), // East Asian ideograph (variant of 4C284C)
    (0x294223, ('\u{94AD}', false)), // East Asian ideograph
    (0x235D4F, ('\u{9E87}', false)), // East Asian ideograph
    (0x3F5E60, ('\u{9586}', false)), // East Asian ideograph
    (0x395773, ('\u{7DAF}', false)), // East Asian ideograph
    (0x4B5F71, ('\u{9756}', false)), // East Asian ideograph
    (0x224844, ('\u{6D39}', false)), // East Asian ideograph
    (0x292C61, ('\u{86F4}', false)), // East Asian ideograph
    (0x6F4D54, ('\u{B451}', false)), // Korean hangul
    (0x6F4B2A, ('\u{AFCD}', false)), // Korean hangul
    (0x69544B, ('\u{5870}', false)), // East Asian ideograph
    (0x213339, ('\u{51A5}', false)), // East Asian ideograph
    (0x294224, ('\u{94AA}', false)), // East Asian ideograph
    (0x6F563F, ('\u{C6B9}', false)), // Korean hangul
    (0x292C64, ('\u{877E}', false)), // East Asian ideograph
    (0x4B5F72, ('\u{975B}', false)), // East Asian ideograph
    (0x4B5365, ('\u{8133}', false)), // East Asian ideograph
    (0x222C66, ('\u{60E2}', false)), // East Asian ideograph
    (0x6F4A50, ('\u{AE6C}', false)), // Korean hangul
    (0x294225, ('\u{94AB}', false)), // East Asian ideograph
    (0x2D5664, ('\u{9F04}', false)), // East Asian ideograph
    (0x6F542A, ('\u{C2ED}', false)), // Korean hangul
    (0x4B5F73, ('\u{975C}', false)), // East Asian ideograph (variant of 215F73 which maps to 975C)
    (0x335228, ('\u{94B5}', false)), // East Asian ideograph
    (0x336C6B, ('\u{6031}', false)), // East Asian ideograph
    (0x6F4D56, ('\u{B458}', false)), // Korean hangul
    (0x692450, ('\u{3070}', false)), // Hiragana letter BA
    (0x4B4E67, ('\u{79D8}', false)), // East Asian ideograph
    (0x6F5C37, ('\u{D3BC}', false)), // Korean hangul
    (0x28714D, ('\u{7EE1}', false)), // East Asian ideograph
    (0x216C6F, ('\u{530B}', false)), // East Asian ideograph
    (0x6F4D57, ('\u{B460}', false)), // Korean hangul
    (0x222C73, ('\u{6103}', false)), // East Asian ideograph
    (0x6F5A21, ('\u{CEA5}', false)), // Korean hangul
    (0x4B3D5C, ('\u{5F83}', false)), // East Asian ideograph
    (0x6F4D58, ('\u{B461}', false)), // Korean hangul
    (0x6F5425, ('\u{C2E0}', false)), // Korean hangul
    (0x692452, ('\u{3072}', false)), // Hiragana letter HI
    (0x21333D, ('\u{51B6}', false)), // East Asian ideograph
    (0x215721, ('\u{8759}', false)), // East Asian ideograph
    (0x6F4F6E, ('\u{B9D9}', false)), // Korean hangul
    (0x6F5723, ('\u{C7A4}', false)), // Korean hangul
    (0x225724, ('\u{72F4}', false)), // East Asian ideograph
    (0x6F5D74, ('\u{D769}', false)), // Korean hangul
    (0x6F4D59, ('\u{B463}', false)), // Korean hangul
    (0x215725, ('\u{879E}', false)), // East Asian ideograph
    (0x695438, ('\u{57B3}', false)), // East Asian ideograph
    (0x692453, ('\u{3073}', false)), // Hiragana letter BI
    (0x6F5726, ('\u{C7A7}', false)), // Korean hangul
    (0x6F5727, ('\u{C7AC}', false)), // Korean hangul
    (0x2F5E66, ('\u{9B12}', false)), // East Asian ideograph
    (0x4B4B71, ('\u{7F3E}', false)), // East Asian ideograph (variant of 2D4B71 which maps to 7F3E)
    (0x6F5728, ('\u{C7AD}', false)), // Korean hangul
    (0x225729, ('\u{7302}', false)), // East Asian ideograph
    (0x697323, ('\u{9D64}', false)), // East Asian ideograph
    (0x6F4D5A, ('\u{B465}', false)), // Korean hangul
    (0x6F572A, ('\u{C7B4}', false)), // Korean hangul
    (0x2E4873, ('\u{6FA3}', false)), // East Asian ideograph
    (0x21572B, ('\u{87B3}', false)), // East Asian ideograph
    (0x333330, ('\u{518A}', false)), // East Asian ideograph
    (0x21572C, ('\u{87BB}', false)), // East Asian ideograph
    (0x29577A, ('\u{9CAD}', false)), // East Asian ideograph
    (0x21572D, ('\u{87C8}', false)), // East Asian ideograph
    (0x39563C, ('\u{56CC}', false)), // East Asian ideograph
    (0x222632, ('\u{5DB8}', false)), // East Asian ideograph
    (0x21572E, ('\u{87D2}', false)), // East Asian ideograph
    (0x4B5D58, ('\u{9234}', false)), // East Asian ideograph
    (0x6F4D5B, ('\u{B46C}', false)), // Korean hangul
    (0x21572F, ('\u{87BA}', false)), // East Asian ideograph
    (0x2D5730, ('\u{87C7}', false)), // East Asian ideograph
    (0x235731, ('\u{9B92}', false)), // East Asian ideograph
    (0x6F5C38, ('\u{D3C4}', false)), // Korean hangul
    (0x275732, ('\u{86F2}', false)), // East Asian ideograph
    (0x6F4E75, ('\u{B7AC}', false)), // Korean hangul
    (0x275733, ('\u{866B}', false)), // East Asian ideograph
    (0x6F4D5C, ('\u{B480}', false)), // Korean hangul
    (0x275734, ('\u{8749}', false)), // East Asian ideograph
    (0x215735, ('\u{87FB}', false)), // East Asian ideograph
    (0x215736, ('\u{8805}', false)), // East Asian ideograph
    (0x2D5228, ('\u{9262}', false)), // East Asian ideograph
    (0x29577C, ('\u{9CB0}', false)), // East Asian ideograph
    (0x695737, ('\u{5F16}', false)), // East Asian ideograph
    (0x222634, ('\u{5DBF}', false)), // East Asian ideograph
    (0x213D21, ('\u{5EBE}', false)), // East Asian ideograph
    (0x6F4D5D, ('\u{B488}', false)), // Korean hangul
    (0x235739, ('\u{9B9D}', false)), // East Asian ideograph
    (0x6F573A, ('\u{C811}', false)), // Korean hangul
    (0x2D3453, ('\u{758B}', false)), // East Asian ideograph
    (0x21573B, ('\u{8822}', false)), // East Asian ideograph
    (0x213132, ('\u{4F5B}', false)), // East Asian ideograph
    (0x21573C, ('\u{8823}', false)), // East Asian ideograph
    (0x21573D, ('\u{8821}', false)), // East Asian ideograph
    (0x23417A, ('\u{91F8}', false)), // East Asian ideograph
    (0x6F4D5E, ('\u{B4A4}', false)), // Korean hangul
    (0x21573E, ('\u{881F}', false)), // East Asian ideograph
    (0x275E69, ('\u{5173}', false)), // East Asian ideograph
    (0x6F5B7A, ('\u{D330}', false)), // Korean hangul
    (0x692458, ('\u{3078}', false)), // Hiragana letter HE
    (0x21573F, ('\u{8831}', false)), // East Asian ideograph
    (0x235D5A, ('\u{9E95}', false)), // East Asian ideograph
    (0x4B5740, ('\u{8827}', false)), // East Asian ideograph
    (0x2D572D, ('\u{8748}', false)), // East Asian ideograph
    (0x215741, ('\u{8836}', false)), // East Asian ideograph
    (0x69533B, ('\u{555D}', false)), // East Asian ideograph
    (0x275742, ('\u{86EE}', false)), // East Asian ideograph
    (0x27614F, ('\u{9A74}', false)), // East Asian ideograph
    (0x6F4D5F, ('\u{B4B7}', false)), // Korean hangul
    (0x215743, ('\u{8840}', false)), // East Asian ideograph
    (0x4C5C3A, ('\u{73F1}', false)), // East Asian ideograph
    (0x6F5744, ('\u{C82D}', false)), // Korean hangul
    (0x694823, ('\u{7872}', false)), // East Asian ideograph
    (0x6F5745, ('\u{C82F}', false)), // Korean hangul
    (0x2D522B, ('\u{9475}', false)), // East Asian ideograph
    (0x215746, ('\u{8853}', false)), // East Asian ideograph (variant of 4B5746 which maps to 8853)
    (0x275747, ('\u{4E8D}', false)), // East Asian ideograph
    (0x2D4562, ('\u{681D}', false)), // East Asian ideograph
    (0x275A36, ('\u{8D56}', false)), // East Asian ideograph
    (0x6F4D60, ('\u{B4C0}', false)), // Korean hangul
    (0x69245A, ('\u{307A}', false)), // Hiragana letter PE
    (0x213345, ('\u{51DD}', false)), // East Asian ideograph
    (0x215749, ('\u{885B}', false)), // East Asian ideograph
    (0x235D5C, ('\u{9E91}', false)), // East Asian ideograph
    (0x2D3164, ('\u{7AE2}', false)), // East Asian ideograph
    (0x4B4A2E, ('\u{55B6}', false)), // East Asian ideograph
    (0x21574A, ('\u{885D}', false)), // East Asian ideograph
    (0x474236, ('\u{949A}', false)), // East Asian ideograph
    (0x2D4425, ('\u{686E}', false)), // East Asian ideograph
    (0x29533D, ('\u{9A90}', false)), // East Asian ideograph
    (0x21574C, ('\u{8862}', false)), // East Asian ideograph
    (0x3B3922, ('\u{8DB5}', false)), // East Asian ideograph
    (0x21574D, ('\u{8863}', false)), // East Asian ideograph
    (0x23574E, ('\u{9BA0}', false)), // East Asian ideograph
    (0x2D3457, ('\u{62FE}', false)), // East Asian ideograph
    (0x21574F, ('\u{8868}', false)), // East Asian ideograph
    (0x6F5750, ('\u{C885}', false)), // Korean hangul
    (0x213D22, ('\u{5ECA}', false)), // East Asian ideograph
    (0x2D4564, ('\u{68B9}', false)), // East Asian ideograph
    (0x215752, ('\u{8881}', false)), // East Asian ideograph
    (0x27602E, ('\u{97E9}', false)), // East Asian ideograph
    (0x6F5753, ('\u{C88B}', false)), // Korean hangul
    (0x69613E, ('\u{7569}', false)), // East Asian ideograph
    (0x6F5754, ('\u{C88C}', false)), // Korean hangul
    (0x215755, ('\u{8888}', false)), // East Asian ideograph
    (0x4B3D67, ('\u{5F84}', false)), // East Asian ideograph (variant of 273D67)
    (0x215756, ('\u{88AB}', false)), // East Asian ideograph
    (0x6F4D63, ('\u{B4DD}', false)), // Korean hangul
    (0x29367E, ('\u{8D59}', false)), // East Asian ideograph
    (0x217337, ('\u{568A}', false)), // East Asian ideograph
    (0x215759, ('\u{888D}', false)), // East Asian ideograph
    (0x6F5967, ('\u{CE60}', false)), // Korean hangul
    (0x21575A, ('\u{888B}', false)), // East Asian ideograph
    (0x21575B, ('\u{889E}', false)), // East Asian ideograph
    (0x4D3C6C, ('\u{8FB6}', false)), // East Asian ideograph
    (0x21575C, ('\u{88C1}', false)), // East Asian ideograph
    (0x273A36, ('\u{5988}', false)), // East Asian ideograph
    (0x6F4921, ('\u{AC70}', false)), // Korean hangul
    (0x23575D, ('\u{9BC6}', false)), // East Asian ideograph
    (0x23575E, ('\u{9BBF}', false)), // East Asian ideograph
    (0x22575F, ('\u{733B}', false)), // East Asian ideograph
    (0x2D5760, ('\u{5E2C}', false)), // East Asian ideograph
    (0x6F4D65, ('\u{B4E3}', false)), // Korean hangul
    (0x4C7265, ('\u{7DFC}', false)), // East Asian ideograph
    (0x69245F, ('\u{307F}', false)), // Hiragana letter MI
    (0x6F4922, ('\u{AC71}', false)), // Korean hangul
    (0x225762, ('\u{733A}', false)), // East Asian ideograph
    (0x6F5125, ('\u{BC45}', false)), // Korean hangul
    (0x2E4670, ('\u{6CD0}', false)), // East Asian ideograph
    (0x284539, ('\u{6B9A}', false)), // East Asian ideograph
    (0x2D345B, ('\u{6607}', false)), // East Asian ideograph
    (0x275763, ('\u{91CC}', false)), // East Asian ideograph
    (0x393054, ('\u{4F0D}', false)), // East Asian ideograph
    (0x226260, ('\u{777A}', false)), // East Asian ideograph
    (0x6F5764, ('\u{C8D4}', false)), // Korean hangul
    (0x232739, ('\u{85DA}', false)), // East Asian ideograph
    (0x215765, ('\u{88DD}', false)), // East Asian ideograph
    (0x395564, ('\u{6726}', false)), // East Asian ideograph
    (0x235766, ('\u{9BB9}', false)), // East Asian ideograph
    (0x6F4E2E, ('\u{B560}', false)), // Korean hangul
    (0x6F5767, ('\u{C8E0}', false)), // Korean hangul
    (0x6F504B, ('\u{BB3B}', false)), // Korean hangul
    (0x6F2469, ('\u{3138}', false)), // Korean hangul
    (0x215768, ('\u{88F3}', false)), // East Asian ideograph
    (0x2D5232, ('\u{8FA0}', false)), // East Asian ideograph
    (0x6F5769, ('\u{C8F0}', false)), // Korean hangul
    (0x6F5B60, ('\u{D2CB}', false)), // Korean hangul
    (0x69576A, ('\u{603A}', false)), // East Asian ideograph
    (0x22576B, ('\u{7352}', false)), // East Asian ideograph
    (0x21334C, ('\u{51F9}', false)), // East Asian ideograph
    (0x27576C, ('\u{5236}', false)), // East Asian ideograph
    (0x225521, ('\u{721D}', false)), // East Asian ideograph
    (0x2D345D, ('\u{5349}', false)), // East Asian ideograph
    (0x6F576D, ('\u{C8FD}', false)), // Korean hangul
    (0x2D5233, ('\u{7F78}', false)), // East Asian ideograph
    (0x23576E, ('\u{9BC0}', false)), // East Asian ideograph
    (0x29312B, ('\u{89D1}', false)), // East Asian ideograph
    (0x2D5361, ('\u{811A}', false)), // East Asian ideograph
    (0x4B576F, ('\u{8910}', false)), // East Asian ideograph (variant of 21576F which maps to 8910)
    (0x6F4D68, ('\u{B4ED}', false)), // Korean hangul
    (0x6F4B2E, ('\u{AFE9}', false)), // Korean hangul
    (0x6F4925, ('\u{AC77}', false)), // Korean hangul
    (0x275771, ('\u{8934}', false)), // East Asian ideograph
    (0x215772, ('\u{8912}', false)), // East Asian ideograph
    (0x294164, ('\u{948B}', false)), // East Asian ideograph
    (0x275773, ('\u{88E4}', false)), // East Asian ideograph
    (0x215774, ('\u{892A}', false)), // East Asian ideograph
    (0x6F4D69, ('\u{B4EF}', false)), // Korean hangul
    (0x29467C, ('\u{9546}', false)), // East Asian ideograph
    (0x3F4926, ('\u{6E08}', false)), // East Asian ideograph
    (0x21334E, ('\u{51FD}', false)), // East Asian ideograph
    (0x6F5776, ('\u{C92C}', false)), // Korean hangul
    (0x234221, ('\u{91F9}', false)), // East Asian ideograph
    (0x215777, ('\u{893B}', false)), // East Asian ideograph
    (0x224222, ('\u{6A7F}', false)), // East Asian ideograph
    (0x6F5A33, ('\u{CF11}', false)), // Korean hangul
    (0x234223, ('\u{9204}', false)), // East Asian ideograph
    (0x216231, ('\u{9CF6}', false)), // East Asian ideograph
    (0x215779, ('\u{8938}', false)), // East Asian ideograph
    (0x224224, ('\u{6A91}', false)), // East Asian ideograph
    (0x21577A, ('\u{8944}', false)), // East Asian ideograph
    (0x214225, ('\u{64E0}', false)), // East Asian ideograph
    (0x6F4927, ('\u{AC79}', false)), // Korean hangul
    (0x22577B, ('\u{7358}', false)), // East Asian ideograph
    (0x224226, ('\u{6A9F}', false)), // East Asian ideograph
    (0x4B4A38, ('\u{71D7}', false)), // East Asian ideograph
    (0x21577C, ('\u{8960}', false)), // East Asian ideograph
    (0x234227, ('\u{920A}', false)), // East Asian ideograph
    (0x27577D, ('\u{8884}', false)), // East Asian ideograph
    (0x234228, ('\u{9225}', false)), // East Asian ideograph
    (0x295347, ('\u{9A93}', false)), // East Asian ideograph
    (0x216232, ('\u{9CF4}', false)), // East Asian ideograph
    (0x21577E, ('\u{8964}', false)), // East Asian ideograph
    (0x274229, ('\u{6401}', false)), // East Asian ideograph
    (0x22422A, ('\u{6A92}', false)), // East Asian ideograph
    (0x692465, ('\u{3085}', false)), // Hiragana letter small YU
    (0x22422B, ('\u{6AA3}', false)), // East Asian ideograph
    (0x6F504C, ('\u{BB3C}', false)), // Korean hangul
    (0x6F545B, ('\u{C42C}', false)), // Korean hangul
    (0x23422C, ('\u{9228}', false)), // East Asian ideograph
    (0x6F5A35, ('\u{CF15}', false)), // Korean hangul
    (0x6F5B61, ('\u{D2D4}', false)), // Korean hangul
    (0x29556C, ('\u{960B}', false)), // East Asian ideograph
    (0x4B5B46, ('\u{8F0C}', false)), // East Asian ideograph
    (0x21422E, ('\u{64FE}', false)), // East Asian ideograph
    (0x456260, ('\u{5E7A}', false)), // East Asian ideograph
    (0x275C57, ('\u{8FD8}', false)), // East Asian ideograph
    (0x23422F, ('\u{9203}', false)), // East Asian ideograph
    (0x276030, ('\u{827D}', false)), // East Asian ideograph
    (0x692466, ('\u{3086}', false)), // Hiragana letter YU
    (0x274230, ('\u{6446}', false)), // East Asian ideograph
    (0x2D567B, ('\u{8717}', false)), // East Asian ideograph
    (0x234231, ('\u{9200}', false)), // East Asian ideograph
    (0x393573, ('\u{5611}', false)), // East Asian ideograph
    (0x234232, ('\u{9218}', false)), // East Asian ideograph
    (0x295C3E, ('\u{9E37}', false)), // East Asian ideograph
    (0x274233, ('\u{62E6}', false)), // East Asian ideograph
    (0x6F4D6D, ('\u{B518}', false)), // Korean hangul
    (0x6F4B2F, ('\u{AFF0}', false)), // Korean hangul
    (0x274234, ('\u{6400}', false)), // East Asian ideograph
    (0x274235, ('\u{6444}', false)), // East Asian ideograph
    (0x234236, ('\u{9208}', false)), // East Asian ideograph
    (0x6F5A37, ('\u{CF20}', false)), // Korean hangul
    (0x224237, ('\u{6A9B}', false)), // East Asian ideograph
    (0x234238, ('\u{921C}', false)), // East Asian ideograph
    (0x2D6079, ('\u{8218}', false)), // East Asian ideograph
    (0x6F492B, ('\u{AC83}', false)), // Korean hangul
    (0x213353, ('\u{5206}', false)), // East Asian ideograph
    (0x27423A, ('\u{6405}', false)), // East Asian ideograph
    (0x287349, ('\u{7F30}', false)), // East Asian ideograph
    (0x2D3464, ('\u{613D}', false)), // East Asian ideograph
    (0x23423B, ('\u{9224}', false)), // East Asian ideograph
    (0x2D3021, ('\u{5F0C}', false)), // East Asian ideograph
    (0x6F5A38, ('\u{CF24}', false)), // Korean hangul
    (0x335772, ('\u{8943}', false)), // East Asian ideograph
    (0x293132, ('\u{89CC}', false)), // East Asian ideograph
    (0x226635, ('\u{7911}', false)), // East Asian ideograph
    (0x33423D, ('\u{53CE}', false)), // East Asian ideograph
    (0x284D2B, ('\u{6D54}', false)), // East Asian ideograph
    (0x2D486B, ('\u{6F82}', false)), // East Asian ideograph
    (0x692469, ('\u{3089}', false)), // Hiragana letter RA
    (0x6F576B, ('\u{C8F5}', false)), // Korean hangul
    (0x2D3C7C, ('\u{83F4}', false)), // East Asian ideograph
    (0x4B516D, ('\u{7DCF}', false)), // East Asian ideograph
    (0x216237, ('\u{9D23}', false)), // East Asian ideograph
    (0x224242, ('\u{6AA0}', false)), // East Asian ideograph
    (0x6F4D70, ('\u{B524}', false)), // Korean hangul
    (0x234243, ('\u{9212}', false)), // East Asian ideograph
    (0x69246A, ('\u{308A}', false)), // Hiragana letter RI
    (0x334244, ('\u{6559}', false)), // East Asian ideograph
    (0x4B4A3E, ('\u{7235}', false)), // East Asian ideograph
    (0x2F5E7D, ('\u{6641}', false)), // East Asian ideograph
    (0x393577, ('\u{9FA2}', false)), // East Asian ideograph
    (0x6F5B62, ('\u{D1F8}', false)), // Korean hangul
    (0x214247, ('\u{6557}', false)), // East Asian ideograph
    (0x6F4D71, ('\u{B525}', false)), // Korean hangul
    (0x6F4C7A, ('\u{B2FA}', false)), // Korean hangul
    (0x234248, ('\u{91FF}', false)), // East Asian ideograph
    (0x285323, ('\u{8367}', false)), // East Asian ideograph
    (0x69246B, ('\u{308B}', false)), // Hiragana letter RU
    (0x217345, ('\u{5699}', false)), // East Asian ideograph
    (0x224249, ('\u{6A9E}', false)), // East Asian ideograph
    (0x22424A, ('\u{6A87}', false)), // East Asian ideograph
    (0x6F5A3B, ('\u{CF2F}', false)), // Korean hangul
    (0x22424B, ('\u{6A8E}', false)), // East Asian ideograph
    (0x23424E, ('\u{9206}', false)), // East Asian ideograph
    (0x27424F, ('\u{542F}', false)), // East Asian ideograph
    (0x6F5A3C, ('\u{CF30}', false)), // Korean hangul
    (0x21623A, ('\u{9D1B}', false)), // East Asian ideograph
    (0x224251, ('\u{6AAB}', false)), // East Asian ideograph
    (0x6F4D73, ('\u{B528}', false)), // Korean hangul
    (0x234252, ('\u{9249}', false)), // East Asian ideograph
    (0x6F4930, ('\u{AC89}', false)), // Korean hangul
    (0x394243, ('\u{4FF2}', false)), // East Asian ideograph
    (0x705F61, ('\u{54DA}', false)), // East Asian ideograph
    (0x234254, ('\u{924D}', false)), // East Asian ideograph
    (0x6F556B, ('\u{C606}', false)), // Korean hangul
    (0x224255, ('\u{6AC8}', false)), // East Asian ideograph
    (0x224864, ('\u{6D19}', false)), // East Asian ideograph
    (0x4C4177, ('\u{8223}', false)), // East Asian ideograph
    (0x274256, ('\u{655B}', false)), // East Asian ideograph
    (0x6F4D74, ('\u{B529}', false)), // Korean hangul
    (0x224257, ('\u{6AAE}', false)), // East Asian ideograph
    (0x6F4931, ('\u{AC8A}', false)), // Korean hangul
    (0x6F497C, ('\u{AD90}', false)), // Korean hangul
    (0x234258, ('\u{923A}', false)), // East Asian ideograph
    (0x2D346A, ('\u{5918}', false)), // East Asian ideograph
    (0x6F4879, ('\u{AC30}', false)), // Korean hangul
    (0x2D3C7D, ('\u{53A2}', false)), // East Asian ideograph
    (0x2D4F3E, ('\u{7A3E}', false)), // East Asian ideograph
    (0x4D4134, ('\u{919B}', false)), // East Asian ideograph
    (0x23425C, ('\u{922E}', false)), // East Asian ideograph
    (0x6F4932, ('\u{AC8B}', false)), // Korean hangul
    (0x22425D, ('\u{6ABF}', false)), // East Asian ideograph
    (0x2D5241, ('\u{7FA3}', false)), // East Asian ideograph
    (0x6F5A3F, ('\u{CF58}', false)), // Korean hangul
    (0x23425F, ('\u{9233}', false)), // East Asian ideograph
    (0x294260, ('\u{94B7}', false)), // East Asian ideograph
    (0x234261, ('\u{9266}', false)), // East Asian ideograph
    (0x6F4933, ('\u{AC8C}', false)), // Korean hangul
    (0x214263, ('\u{65AC}', false)), // East Asian ideograph
    (0x29446D, ('\u{951B}', false)), // East Asian ideograph
    (0x6F5A40, ('\u{CF5C}', false)), // Korean hangul
    (0x224264, ('\u{6ACA}', false)), // East Asian ideograph
    (0x224867, ('\u{6D0E}', false)), // East Asian ideograph
    (0x4B3938, ('\u{5942}', false)), // East Asian ideograph
    (0x214266, ('\u{65B7}', false)), // East Asian ideograph
    (0x6F4934, ('\u{AC90}', false)), // Korean hangul
    (0x22375B, ('\u{65FB}', false)), // East Asian ideograph
    (0x4B4A45, ('\u{5C13}', false)), // East Asian ideograph
    (0x234268, ('\u{9235}', false)), // East Asian ideograph
    (0x4B4B77, ('\u{4EC0}', false)), // East Asian ideograph
    (0x6F5A41, ('\u{CF64}', false)), // Korean hangul
    (0x4B5B52, ('\u{8F42}', false)), // East Asian ideograph
    (0x6F4D78, ('\u{B530}', false)), // Korean hangul
    (0x23426B, ('\u{9250}', false)), // East Asian ideograph
    (0x692472, ('\u{3092}', false)), // Hiragana letter WO
    (0x21335D, ('\u{5228}', false)), // East Asian ideograph
    (0x22375C, ('\u{65FC}', false)), // East Asian ideograph
    (0x23426C, ('\u{926B}', false)), // East Asian ideograph
    (0x23426D, ('\u{9239}', false)), // East Asian ideograph
    (0x6F556C, ('\u{C607}', false)), // Korean hangul
    (0x6F5A42, ('\u{CF65}', false)), // Korean hangul
    (0x6F4B6C, ('\u{B0E5}', false)), // Korean hangul
    (0x23426F, ('\u{926D}', false)), // East Asian ideograph
    (0x234270, ('\u{926C}', false)), // East Asian ideograph
    (0x6F4936, ('\u{AC9C}', false)), // Korean hangul
    (0x234271, ('\u{924F}', false)), // East Asian ideograph
    (0x2D4272, ('\u{65E3}', false)), // East Asian ideograph
    (0x6F5C3E, ('\u{D3ED}', false)), // Korean hangul
    (0x2D3C7E, ('\u{53A0}', false)), // East Asian ideograph
    (0x6F5A43, ('\u{CF67}', false)), // Korean hangul
    (0x294274, ('\u{94BF}', false)), // East Asian ideograph
    (0x6F4D7A, ('\u{B532}', false)), // Korean hangul
    (0x6F4937, ('\u{AC9F}', false)), // Korean hangul
    (0x234277, ('\u{9260}', false)), // East Asian ideograph
    (0x2D302D, ('\u{4E17}', false)), // East Asian ideograph
    (0x6F4D62, ('\u{B4DC}', false)), // Korean hangul
    (0x6F5A44, ('\u{CF69}', false)), // Korean hangul
    (0x234C6A, ('\u{9741}', false)), // East Asian ideograph
    (0x4B5B55, ('\u{8EE2}', false)), // East Asian ideograph
    (0x224279, ('\u{6AE6}', false)), // East Asian ideograph
    (0x216D24, ('\u{531C}', false)), // East Asian ideograph
    (0x6F4D7B, ('\u{B534}', false)), // Korean hangul
    (0x69562E, ('\u{5CBB}', false)), // East Asian ideograph
    (0x6F4938, ('\u{ACA0}', false)), // Korean hangul
    (0x6F5B3A, ('\u{D1A4}', false)), // Korean hangul
    (0x275823, ('\u{88AD}', false)), // East Asian ideograph
    (0x29424B, ('\u{94A3}', false)), // East Asian ideograph
    (0x235D77, ('\u{9EAD}', false)), // East Asian ideograph
    (0x6F4F75, ('\u{B9E5}', false)), // Korean hangul
    (0x213D6A, ('\u{5F97}', false)), // East Asian ideograph
    (0x2E3870, ('\u{714A}', false)), // East Asian ideograph
    (0x6F5A45, ('\u{CF70}', false)), // Korean hangul
    (0x22486C, ('\u{6D00}', false)), // East Asian ideograph
    (0x234C6B, ('\u{9747}', false)), // East Asian ideograph
    (0x23427E, ('\u{9236}', false)), // East Asian ideograph
    (0x6F4D7C, ('\u{B537}', false)), // Korean hangul
    (0x6F4B32, ('\u{B00C}', false)), // Korean hangul
    (0x222D2A, ('\u{610A}', false)), // East Asian ideograph
    (0x22442A, ('\u{6B35}', false)), // East Asian ideograph
    (0x216D2E, ('\u{532D}', false)), // East Asian ideograph
    (0x6F4D7D, ('\u{B538}', false)), // Korean hangul
    (0x6F493A, ('\u{ACA8}', false)), // Korean hangul
    (0x213362, ('\u{5230}', false)), // East Asian ideograph
    (0x276822, ('\u{507B}', false)), // East Asian ideograph
    (0x2D3473, ('\u{5374}', false)), // East Asian ideograph
    (0x6F5A47, ('\u{CF74}', false)), // Korean hangul
    (0x4C3F7A, ('\u{6922}', false)), // East Asian ideograph
    (0x29535A, ('\u{9A9F}', false)), // East Asian ideograph
    (0x222D32, ('\u{6112}', false)), // East Asian ideograph
    (0x4B5B58, ('\u{8EE3}', false)), // East Asian ideograph
    (0x4B393F, ('\u{5333}', false)), // East Asian ideograph
    (0x216D33, ('\u{5330}', false)), // East Asian ideograph
    (0x2D486E, ('\u{6F97}', false)), // East Asian ideograph (not in Unicode)
    (0x282D34, ('\u{607D}', false)), // East Asian ideograph
    (0x6F497E, ('\u{ADA4}', false)), // Korean hangul
    (0x224E32, ('\u{6FCA}', false)), // East Asian ideograph
    (0x6F5C3F, ('\u{D3F0}', false)), // Korean hangul
    (0x2F5A48, ('\u{9D44}', false)), // East Asian ideograph
    (0x22486F, ('\u{6D33}', false)), // East Asian ideograph
    (0x4C5F58, ('\u{7640}', false)), // East Asian ideograph
    (0x6F4E2F, ('\u{B561}', false)), // Korean hangul
    (0x6F493C, ('\u{ACAA}', false)), // Korean hangul
    (0x39424F, ('\u{5554}', false)), // East Asian ideograph
    (0x2D3032, ('\u{7ADD}', false)), // East Asian ideograph
    (0x33392F, ('\u{9029}', false)), // East Asian ideograph
    (0x284F5D, ('\u{6CA3}', false)), // East Asian ideograph
    (0x22442D, ('\u{6B3B}', false)), // East Asian ideograph
    (0x216D3E, ('\u{533D}', false)), // East Asian ideograph
    (0x51513B, ('\u{7E9F}', false)), // East Asian ideograph
    (0x222D3F, ('\u{6121}', false)), // East Asian ideograph
    (0x6F4F76, ('\u{B9E8}', false)), // Korean hangul
    (0x292768, ('\u{8572}', false)), // East Asian ideograph
    (0x274B5F, ('\u{7410}', false)), // East Asian ideograph
    (0x6F5A4A, ('\u{CF85}', false)), // Korean hangul
    (0x232D41, ('\u{8841}', false)), // East Asian ideograph
    (0x2D4277, ('\u{65EE}', false)), // East Asian ideograph
    (0x293A6B, ('\u{8E7F}', false)), // East Asian ideograph
    (0x232A57, ('\u{873E}', false)), // East Asian ideograph
    (0x6F4B33, ('\u{B00D}', false)), // Korean hangul
    (0x222D43, ('\u{6106}', false)), // East Asian ideograph
    (0x294251, ('\u{94C8}', false)), // East Asian ideograph
    (0x3B2D44, ('\u{8842}', false)), // East Asian ideograph
    (0x295053, ('\u{98D9}', false)), // East Asian ideograph
    (0x287178, ('\u{7EF6}', false)), // East Asian ideograph
    (0x226D47, ('\u{7C00}', false)), // East Asian ideograph
    (0x2D4141, ('\u{63B2}', false)), // East Asian ideograph
    (0x6F493F, ('\u{ACB0}', false)), // Korean hangul
    (0x6F547E, ('\u{C548}', false)), // Korean hangul
    (0x294252, ('\u{94C9}', false)), // East Asian ideograph
    (0x296028, ('\u{9F86}', false)), // East Asian ideograph
    (0x2F3833, ('\u{8D91}', false)), // East Asian ideograph
    (0x216D4B, ('\u{535D}', false)), // East Asian ideograph
    (0x6F4940, ('\u{ACB8}', false)), // Korean hangul
    (0x284F61, ('\u{6EE0}', false)), // East Asian ideograph
    (0x6F5C40, ('\u{D3F4}', false)), // Korean hangul
    (0x295360, ('\u{9A98}', false)), // East Asian ideograph
    (0x4B5B5E, ('\u{5F01}', false)), // East Asian ideograph
    (0x232D51, ('\u{884A}', false)), // East Asian ideograph
    (0x6F4941, ('\u{ACB9}', false)), // Korean hangul
    (0x294254, ('\u{94CB}', false)), // East Asian ideograph
    (0x286D54, ('\u{7B5A}', false)), // East Asian ideograph
    (0x222D56, ('\u{53AF}', false)), // East Asian ideograph
    (0x232D57, ('\u{8850}', false)), // East Asian ideograph
    (0x21336A, ('\u{524C}', false)), // East Asian ideograph
    (0x294255, ('\u{94CA}', false)), // East Asian ideograph
    (0x29602B, ('\u{9F85}', false)), // East Asian ideograph
    (0x6F4F77, ('\u{B9EC}', false)), // Korean hangul
    (0x21313A, ('\u{4F38}', false)), // East Asian ideograph
    (0x3F462B, ('\u{5E30}', false)), // East Asian ideograph
    (0x274B64, ('\u{7391}', false)), // East Asian ideograph
    (0x6F5A4F, ('\u{CFB0}', false)), // Korean hangul
    (0x696D5A, ('\u{8F4C}', false)), // East Asian ideograph
    (0x29327E, ('\u{8C07}', false)), // East Asian ideograph
    (0x6F4B34, ('\u{B010}', false)), // Korean hangul
    (0x6F4943, ('\u{ACBC}', false)), // Korean hangul
    (0x21336B, ('\u{524B}', false)), // East Asian ideograph
    (0x4D4862, ('\u{9229}', false)), // East Asian ideograph
    (0x222D5E, ('\u{6137}', false)), // East Asian ideograph
    (0x28717D, ('\u{7EFA}', false)), // East Asian ideograph
    (0x6F5A50, ('\u{CFC4}', false)), // Korean hangul
    (0x6F4B75, ('\u{B113}', false)), // Korean hangul
    (0x6F4A5A, ('\u{AEBD}', false)), // Korean hangul
    (0x6F4944, ('\u{ACBD}', false)), // Korean hangul
    (0x21735B, ('\u{56C3}', false)), // East Asian ideograph
    (0x225541, ('\u{723F}', false)), // East Asian ideograph
    (0x226D63, ('\u{7C20}', false)), // East Asian ideograph
    (0x2D4147, ('\u{6271}', false)), // East Asian ideograph
    (0x216D66, ('\u{5393}', false)), // East Asian ideograph
    (0x6F4945, ('\u{ACC1}', false)), // Korean hangul
    (0x21336D, ('\u{5247}', false)), // East Asian ideograph
    (0x294258, ('\u{94B0}', false)), // East Asian ideograph
    (0x6F4B22, ('\u{AFB8}', false)), // Korean hangul
    (0x335941, ('\u{54D7}', false)), // East Asian ideograph
    (0x274B67, ('\u{73AF}', false)), // East Asian ideograph
    (0x234C78, ('\u{975D}', false)), // East Asian ideograph
    (0x234835, ('\u{942B}', false)), // East Asian ideograph
    (0x6F5052, ('\u{BB4F}', false)), // Korean hangul
    (0x276D6D, ('\u{538D}', false)), // East Asian ideograph
    (0x274B68, ('\u{7477}', false)), // East Asian ideograph
    (0x6F5A53, ('\u{CFE4}', false)), // Korean hangul
    (0x225235, ('\u{712F}', false)), // East Asian ideograph
    (0x294F23, ('\u{9880}', false)), // East Asian ideograph
    (0x4B546D, ('\u{82D3}', false)), // East Asian ideograph (variant of 21546D which maps to 82D3)
    (0x6F4C7B, ('\u{B2FB}', false)), // Korean hangul
    (0x6F4947, ('\u{ACC4}', false)), // Korean hangul
    (0x21336F, ('\u{5256}', false)), // East Asian ideograph
    (0x225544, ('\u{7242}', false)), // East Asian ideograph
    (0x6F5724, ('\u{C7A5}', false)), // Korean hangul
    (0x274932, ('\u{6CFB}', false)), // East Asian ideograph
    (0x4B682E, ('\u{4EC2}', false)), // East Asian ideograph
    (0x274B69, ('\u{73BA}', false)), // East Asian ideograph
    (0x6F5A54, ('\u{CFE8}', false)), // Korean hangul
    (0x234837, ('\u{9441}', false)), // East Asian ideograph
    (0x213D3F, ('\u{5F17}', false)), // East Asian ideograph
    (0x4D2D75, ('\u{8872}', false)), // East Asian ideograph (variant of 232D75 which maps to 8872)
    (0x213370, ('\u{525B}', false)), // East Asian ideograph
    (0x69252C, ('\u{30AC}', false)), // Katakana letter GA
    (0x225821, ('\u{734B}', false)), // East Asian ideograph
    (0x282D77, ('\u{60AD}', false)), // East Asian ideograph
    (0x215822, ('\u{896F}', false)), // East Asian ideograph
    (0x6F5A55, ('\u{CFF0}', false)), // Korean hangul
    (0x6F557D, ('\u{C63B}', false)), // Korean hangul
    (0x234C7B, ('\u{975F}', false)), // East Asian ideograph
    (0x222D79, ('\u{6164}', false)), // East Asian ideograph
    (0x4B5824, ('\u{897E}', false)), // East Asian ideograph
    (0x696D7A, ('\u{9027}', false)), // East Asian ideograph
    (0x6F4949, ('\u{ACD7}', false)), // Korean hangul
    (0x215825, ('\u{8981}', false)), // East Asian ideograph
    (0x29425C, ('\u{94CC}', false)), // East Asian ideograph
    (0x695C29, ('\u{6925}', false)), // East Asian ideograph
    (0x4B5826, ('\u{8983}', false)), // East Asian ideograph (variant of 215826 which maps to 8983)
    (0x232D7C, ('\u{8879}', false)), // East Asian ideograph
    (0x295827, ('\u{9CB2}', false)), // East Asian ideograph
    (0x6F5A56, ('\u{CFF3}', false)), // Korean hangul
    (0x295369, ('\u{9A7A}', false)), // East Asian ideograph
    (0x215828, ('\u{898B}', false)), // East Asian ideograph
    (0x225829, ('\u{736C}', false)), // East Asian ideograph
    (0x6F494A, ('\u{ACE0}', false)), // Korean hangul
    (0x21582A, ('\u{8993}', false)), // East Asian ideograph
    (0x21582B, ('\u{8996}', false)), // East Asian ideograph
    (0x2D5259, ('\u{98DC}', false)), // East Asian ideograph
    (0x21582C, ('\u{89AA}', false)), // East Asian ideograph
    (0x284F6B, ('\u{6F13}', false)), // East Asian ideograph
    (0x29536A, ('\u{9A9D}', false)), // East Asian ideograph
    (0x21582D, ('\u{89A6}', false)), // East Asian ideograph
    (0x27582E, ('\u{89CA}', false)), // East Asian ideograph
    (0x6F494B, ('\u{ACE1}', false)), // Korean hangul
    (0x22582F, ('\u{736F}', false)), // East Asian ideograph
    (0x275830, ('\u{89C9}', false)), // East Asian ideograph
    (0x215831, ('\u{89BD}', false)), // East Asian ideograph
    (0x6F5A58, ('\u{CFFC}', false)), // Korean hangul
    (0x275832, ('\u{89C2}', false)), // East Asian ideograph
    (0x222871, ('\u{5EE8}', false)), // East Asian ideograph
    (0x22443C, ('\u{6B43}', false)), // East Asian ideograph
    (0x33483B, ('\u{6CDD}', false)), // East Asian ideograph
    (0x2D5833, ('\u{752A}', false)), // East Asian ideograph
    (0x276037, ('\u{9875}', false)), // East Asian ideograph
    (0x6F494C, ('\u{ACE4}', false)), // Korean hangul
    (0x215834, ('\u{89E3}', false)), // East Asian ideograph
    (0x29425F, ('\u{94B6}', false)), // East Asian ideograph
    (0x275835, ('\u{89DE}', false)), // East Asian ideograph
    (0x274933, ('\u{6E0E}', false)), // East Asian ideograph
    (0x235B52, ('\u{9D6F}', false)), // East Asian ideograph
    (0x215836, ('\u{89F8}', false)), // East Asian ideograph
    (0x455837, ('\u{8BA0}', false)), // East Asian ideograph
    (0x4C5F69, ('\u{75EB}', false)), // East Asian ideograph
    (0x275838, ('\u{8BA1}', false)), // East Asian ideograph
    (0x6F4B36, ('\u{B01C}', false)), // Korean hangul
    (0x6F494D, ('\u{ACE7}', false)), // Korean hangul
    (0x275839, ('\u{8BA2}', false)), // East Asian ideograph
    (0x4D2962, ('\u{86C9}', false)), // East Asian ideograph (variant of 232962 which maps to 86C9)
    (0x223331, ('\u{640C}', false)), // East Asian ideograph
    (0x22583B, ('\u{7381}', false)), // East Asian ideograph
    (0x6F5A5A, ('\u{D018}', false)), // Korean hangul
    (0x2E7431, ('\u{7F48}', false)), // East Asian ideograph
    (0x4B625C, ('\u{9EB8}', false)), // East Asian ideograph (variant of 27625C which maps to 9EB8)
    (0x27583C, ('\u{8BB0}', false)), // East Asian ideograph
    (0x22443E, ('\u{6B48}', false)), // East Asian ideograph
    (0x23483D, ('\u{9467}', false)), // East Asian ideograph
    (0x27583D, ('\u{8BA8}', false)), // East Asian ideograph
    (0x273A63, ('\u{5B6A}', false)), // East Asian ideograph
    (0x6F494E, ('\u{ACE8}', false)), // Korean hangul
    (0x27583E, ('\u{8BA7}', false)), // East Asian ideograph
    (0x294261, ('\u{94B2}', false)), // East Asian ideograph
    (0x22583F, ('\u{7388}', false)), // East Asian ideograph
    (0x2D525D, ('\u{6537}', false)), // East Asian ideograph
    (0x224C3F, ('\u{6F26}', false)), // East Asian ideograph
    (0x6F5A5B, ('\u{D02D}', false)), // Korean hangul
    (0x215841, ('\u{8A16}', false)), // East Asian ideograph
    (0x215842, ('\u{8A17}', false)), // East Asian ideograph
    (0x6F494F, ('\u{ACEA}', false)), // Korean hangul
    (0x275843, ('\u{8BAD}', false)), // East Asian ideograph
    (0x275844, ('\u{8BBF}', false)), // East Asian ideograph
    (0x233732, ('\u{8D0C}', false)), // East Asian ideograph
    (0x2D3045, ('\u{4E57}', false)), // East Asian ideograph
    (0x275845, ('\u{8BC0}', false)), // East Asian ideograph
    (0x6F5A5C, ('\u{D034}', false)), // Korean hangul
    (0x225846, ('\u{7395}', false)), // East Asian ideograph
    (0x294F2C, ('\u{988F}', false)), // East Asian ideograph
    (0x215847, ('\u{8A25}', false)), // East Asian ideograph
    (0x6F4950, ('\u{ACEC}', false)), // Korean hangul
    (0x225848, ('\u{7397}', false)), // East Asian ideograph
    (0x6F5739, ('\u{C810}', false)), // Korean hangul
    (0x285C3A, ('\u{748E}', false)), // East Asian ideograph
    (0x6F5054, ('\u{BB54}', false)), // Korean hangul
    (0x215849, ('\u{8A2D}', false)), // East Asian ideograph
    (0x21584A, ('\u{8A1B}', false)), // East Asian ideograph
    (0x6F5A5D, ('\u{D035}', false)), // Korean hangul
    (0x295370, ('\u{9A9C}', false)), // East Asian ideograph
    (0x286D47, ('\u{7BA6}', false)), // East Asian ideograph
    (0x21584B, ('\u{8A1F}', false)), // East Asian ideograph
    (0x21584C, ('\u{8A3B}', false)), // East Asian ideograph
    (0x2D4153, ('\u{64E3}', false)), // East Asian ideograph
    (0x276038, ('\u{9876}', false)), // East Asian ideograph
    (0x6F4951, ('\u{ACEF}', false)), // Korean hangul
    (0x22584D, ('\u{7394}', false)), // East Asian ideograph
    (0x294264, ('\u{94BA}', false)), // East Asian ideograph
    (0x21584E, ('\u{8A55}', false)), // East Asian ideograph
    (0x21584F, ('\u{8A5E}', false)), // East Asian ideograph
    (0x4B576C, ('\u{523E}', false)), // East Asian ideograph
    (0x27486D, ('\u{6DA6}', false)), // East Asian ideograph
    (0x275851, ('\u{8BC2}', false)), // East Asian ideograph
    (0x6F4B37, ('\u{B01D}', false)), // Korean hangul
    (0x6F4952, ('\u{ACF0}', false)), // Korean hangul
    (0x225852, ('\u{73A6}', false)), // East Asian ideograph
    (0x227768, ('\u{80B8}', false)), // East Asian ideograph
    (0x223336, ('\u{6415}', false)), // East Asian ideograph
    (0x275854, ('\u{8BC8}', false)), // East Asian ideograph
    (0x6F5A5F, ('\u{D050}', false)), // Korean hangul
    (0x275855, ('\u{8BCB}', false)), // East Asian ideograph
    (0x275856, ('\u{8BC9}', false)), // East Asian ideograph
    (0x6F4A5D, ('\u{AEC4}', false)), // Korean hangul
    (0x215857, ('\u{8A3A}', false)), // East Asian ideograph
    (0x21736A, ('\u{56D4}', false)), // East Asian ideograph
    (0x33333C, ('\u{6C37}', false)), // East Asian ideograph
    (0x215858, ('\u{8A6B}', false)), // East Asian ideograph
    (0x4B4621, ('\u{6B53}', false)), // East Asian ideograph
    (0x235859, ('\u{9C12}', false)), // East Asian ideograph
    (0x6F5A60, ('\u{D06C}', false)), // Korean hangul
    (0x27585A, ('\u{8BE6}', false)), // East Asian ideograph
    (0x27585B, ('\u{8BD5}', false)), // East Asian ideograph
    (0x45456D, ('\u{6A10}', false)), // East Asian ideograph
    (0x2D5F2C, ('\u{5826}', false)), // East Asian ideograph
    (0x6F4954, ('\u{ACF3}', false)), // Korean hangul
    (0x21585C, ('\u{8A69}', false)), // East Asian ideograph
    (0x6F4B4A, ('\u{B08F}', false)), // Korean hangul
    (0x225551, ('\u{724F}', false)), // East Asian ideograph
    (0x21585D, ('\u{8A70}', false)), // East Asian ideograph
    (0x29443E, ('\u{94E4}', false)), // East Asian ideograph
    (0x21585E, ('\u{8A63}', false)), // East Asian ideograph
    (0x6F5A61, ('\u{D070}', false)), // Korean hangul
    (0x21625F, ('\u{9EBB}', false)), // East Asian ideograph
    (0x21585F, ('\u{8A7C}', false)), // East Asian ideograph
    (0x215860, ('\u{8AA0}', false)), // East Asian ideograph
    (0x6F4E30, ('\u{B5A0}', false)), // Korean hangul
    (0x2D5F2D, ('\u{964F}', false)), // East Asian ideograph
    (0x275861, ('\u{5938}', false)), // East Asian ideograph
    (0x275840, ('\u{8BAF}', false)), // East Asian ideograph
    (0x6F5055, ('\u{BB58}', false)), // Korean hangul
    (0x215862, ('\u{8A85}', false)), // East Asian ideograph
    (0x6F5441, ('\u{C329}', false)), // Korean hangul
    (0x225863, ('\u{73A0}', false)), // East Asian ideograph
    (0x6F5A62, ('\u{D074}', false)), // Korean hangul
    (0x695375, ('\u{56CE}', false)), // East Asian ideograph
    (0x6F5864, ('\u{CB18}', false)), // Korean hangul
    (0x215865, ('\u{8A62}', false)), // East Asian ideograph
    (0x2D575B, ('\u{886E}', false)), // East Asian ideograph
    (0x3F4956, ('\u{7832}', false)), // East Asian ideograph
    (0x215866, ('\u{8A71}', false)), // East Asian ideograph
    (0x285C40, ('\u{74D2}', false)), // East Asian ideograph
    (0x215867, ('\u{8A6E}', false)), // East Asian ideograph
    (0x295132, ('\u{997D}', false)), // East Asian ideograph
    (0x233739, ('\u{8D11}', false)), // East Asian ideograph
    (0x2D5265, ('\u{79D0}', false)), // East Asian ideograph
    (0x225868, ('\u{73CF}', false)), // East Asian ideograph
    (0x6F5A63, ('\u{D07C}', false)), // Korean hangul
    (0x282D5E, ('\u{607A}', false)), // East Asian ideograph
    (0x6F4A25, ('\u{ADD0}', false)), // Korean hangul
    (0x275869, ('\u{8BF4}', false)), // East Asian ideograph
    (0x21586A, ('\u{8AA6}', false)), // East Asian ideograph
    (0x6F4B38, ('\u{B028}', false)), // Korean hangul
    (0x21586B, ('\u{8AA1}', false)), // East Asian ideograph
    (0x215155, ('\u{7DD6}', false)), // East Asian ideograph
    (0x22333B, ('\u{6422}', false)), // East Asian ideograph
    (0x27586D, ('\u{5FD7}', false)), // East Asian ideograph
    (0x22456F, ('\u{6BE7}', false)), // East Asian ideograph
    (0x23586E, ('\u{9C23}', false)), // East Asian ideograph
    (0x27586F, ('\u{8BEC}', false)), // East Asian ideograph
    (0x6F4F36, ('\u{B839}', false)), // Korean hangul
    (0x6F4A5E, ('\u{AECC}', false)), // Korean hangul
    (0x215870, ('\u{8A8D}', false)), // East Asian ideograph
    (0x21736F, ('\u{56E1}', false)), // East Asian ideograph
    (0x294469, ('\u{94FC}', false)), // East Asian ideograph
    (0x215871, ('\u{8AA4}', false)), // East Asian ideograph (variant of 4B5871 which maps to 8AA4)
    (0x23373B, ('\u{8D12}', false)), // East Asian ideograph
    (0x2D5267, ('\u{79CF}', false)), // East Asian ideograph
    (0x215872, ('\u{8AA8}', false)), // East Asian ideograph
    (0x2D4E24, ('\u{6998}', false)), // East Asian ideograph
    (0x215873, ('\u{8AA5}', false)), // East Asian ideograph
    (0x217E60, ('\u{5BC9}', false)), // East Asian ideograph
    (0x215874, ('\u{8A98}', false)), // East Asian ideograph
    (0x4B5D65, ('\u{8217}', false)), // East Asian ideograph
    (0x6F4959, ('\u{ACFD}', false)), // Korean hangul
    (0x215875, ('\u{8A91}', false)), // East Asian ideograph
    (0x6F5130, ('\u{BC9A}', false)), // Korean hangul
    (0x275876, ('\u{8C0A}', false)), // East Asian ideograph
    (0x6F4B68, ('\u{B0C9}', false)), // Korean hangul
    (0x215877, ('\u{8AC4}', false)), // East Asian ideograph
    (0x6F5A66, ('\u{D0A4}', false)), // Korean hangul
    (0x4B5176, ('\u{7E04}', false)), // East Asian ideograph
    (0x295379, ('\u{9A96}', false)), // East Asian ideograph
    (0x235878, ('\u{9C21}', false)), // East Asian ideograph
    (0x234323, ('\u{924E}', false)), // East Asian ideograph
    (0x275879, ('\u{8C08}', false)), // East Asian ideograph
    (0x6F495A, ('\u{AD00}', false)), // Korean hangul
    (0x27587A, ('\u{8BF7}', false)), // East Asian ideograph
    (0x224325, ('\u{6ACC}', false)), // East Asian ideograph
    (0x21587B, ('\u{8AF8}', false)), // East Asian ideograph
    (0x23373D, ('\u{8D14}', false)), // East Asian ideograph
    (0x21587C, ('\u{8AB2}', false)), // East Asian ideograph
    (0x234327, ('\u{9256}', false)), // East Asian ideograph
    (0x29537A, ('\u{9AA2}', false)), // East Asian ideograph
    (0x21587D, ('\u{8ABF}', false)), // East Asian ideograph
    (0x224328, ('\u{6AD1}', false)), // East Asian ideograph
    (0x21587E, ('\u{8AC9}', false)), // East Asian ideograph
    (0x4C7D4D, ('\u{8343}', false)), // East Asian ideograph
    (0x2D4329, ('\u{668E}', false)), // East Asian ideograph
    (0x6F495B, ('\u{AD04}', false)), // Korean hangul
    (0x6F5D31, ('\u{D611}', false)), // Korean hangul
    (0x6F4F7C, ('\u{B9F9}', false)), // Korean hangul
    (0x23432B, ('\u{925A}', false)), // East Asian ideograph
    (0x2D3051, ('\u{5F0D}', false)), // East Asian ideograph
    (0x6F5A68, ('\u{D0A8}', false)), // Korean hangul
    (0x216266, ('\u{9ED1}', false)), // East Asian ideograph
    (0x27432D, ('\u{65F6}', false)), // East Asian ideograph
    (0x4B5736, ('\u{877F}', false)), // East Asian ideograph
    (0x23432E, ('\u{9241}', false)), // East Asian ideograph
    (0x6F495C, ('\u{AD0C}', false)), // Korean hangul
    (0x285252, ('\u{709C}', false)), // East Asian ideograph
    (0x275847, ('\u{8BB7}', false)), // East Asian ideograph
    (0x23432F, ('\u{9283}', false)), // East Asian ideograph
    (0x335958, ('\u{8C4A}', false)), // East Asian ideograph
    (0x394330, ('\u{6644}', false)), // East Asian ideograph
    (0x2D526B, ('\u{7085}', false)), // East Asian ideograph
    (0x234331, ('\u{92A5}', false)), // East Asian ideograph
    (0x213065, ('\u{4ECA}', false)), // East Asian ideograph
    (0x6F5626, ('\u{C653}', false)), // Korean hangul
    (0x274332, ('\u{663C}', false)), // East Asian ideograph
    (0x234333, ('\u{9282}', false)), // East Asian ideograph
    (0x2D5F35, ('\u{78D2}', false)), // East Asian ideograph
    (0x6F495D, ('\u{AD0D}', false)), // Korean hangul
    (0x275848, ('\u{8BB8}', false)), // East Asian ideograph
    (0x224334, ('\u{6ACD}', false)), // East Asian ideograph
    (0x234335, ('\u{92A8}', false)), // East Asian ideograph
    (0x2D3053, ('\u{4E3C}', false)), // East Asian ideograph
    (0x29572B, ('\u{9C90}', false)), // East Asian ideograph
    (0x6F5A6A, ('\u{D0B4}', false)), // Korean hangul
    (0x224337, ('\u{6AEC}', false)), // East Asian ideograph
    (0x215E25, ('\u{938A}', false)), // East Asian ideograph
    (0x234338, ('\u{92A4}', false)), // East Asian ideograph
    (0x6F495E, ('\u{AD0F}', false)), // Korean hangul
    (0x224339, ('\u{6AF3}', false)), // East Asian ideograph
    (0x22433A, ('\u{6AE7}', false)), // East Asian ideograph
    (0x2D433B, ('\u{6662}', false)), // East Asian ideograph
    (0x226225, ('\u{7735}', false)), // East Asian ideograph
    (0x4B5B29, ('\u{8E8D}', false)), // East Asian ideograph
    (0x6F495F, ('\u{AD11}', false)), // Korean hangul
    (0x23433E, ('\u{9276}', false)), // East Asian ideograph
    (0x227775, ('\u{6711}', false)), // East Asian ideograph
    (0x22433F, ('\u{6AEB}', false)), // East Asian ideograph
    (0x224340, ('\u{6AEA}', false)), // East Asian ideograph
    (0x21626A, ('\u{9EDE}', false)), // East Asian ideograph
    (0x274341, ('\u{6655}', false)), // East Asian ideograph
    (0x6F5428, ('\u{C2EB}', false)), // Korean hangul
    (0x234342, ('\u{9288}', false)), // East Asian ideograph
    (0x27603B, ('\u{987A}', false)), // East Asian ideograph
    (0x6F4960, ('\u{AD18}', false)), // Korean hangul
    (0x274343, ('\u{7545}', false)), // East Asian ideograph
    (0x6F4F7D, ('\u{B9FA}', false)), // Korean hangul
    (0x223344, ('\u{6430}', false)), // East Asian ideograph
    (0x224344, ('\u{6AF1}', false)), // East Asian ideograph
    (0x4C6C46, ('\u{7B9F}', false)), // East Asian ideograph
    (0x234345, ('\u{928E}', false)), // East Asian ideograph
    (0x6F562A, ('\u{C65D}', false)), // Korean hangul
    (0x234346, ('\u{92A0}', false)), // East Asian ideograph
    (0x4B7954, ('\u{5968}', false)), // East Asian ideograph
    (0x6F4B3A, ('\u{B045}', false)), // Korean hangul
    (0x234347, ('\u{9277}', false)), // East Asian ideograph
    (0x6F4961, ('\u{AD19}', false)), // Korean hangul
    (0x274348, ('\u{6653}', false)), // East Asian ideograph
    (0x274349, ('\u{5386}', false)), // East Asian ideograph (duplicate simplified)
    (0x6F564F, ('\u{C6E9}', false)), // Korean hangul
    (0x224571, ('\u{6BE8}', false)), // East Asian ideograph
    (0x213066, ('\u{4EC1}', false)), // East Asian ideograph
    (0x6F562B, ('\u{C660}', false)), // Korean hangul
    (0x27434B, ('\u{66A7}', false)), // East Asian ideograph
    (0x6F4962, ('\u{AD1C}', false)), // Korean hangul
    (0x27434D, ('\u{65F7}', false)), // East Asian ideograph
    (0x6F4A60, ('\u{AECF}', false)), // Korean hangul
    (0x4B335B, ('\u{522B}', false)), // East Asian ideograph
    (0x23595E, ('\u{9C6E}', false)), // East Asian ideograph
    (0x22434E, ('\u{6AFD}', false)), // East Asian ideograph
    (0x2D3058, ('\u{4E9C}', false)), // East Asian ideograph
    (0x29434F, ('\u{94DE}', false)), // East Asian ideograph
    (0x6F562C, ('\u{C671}', false)), // Korean hangul
    (0x224350, ('\u{6AFA}', false)), // East Asian ideograph
    (0x347D24, ('\u{83C7}', false)), // East Asian ideograph
    (0x6F552E, ('\u{C561}', false)), // Korean hangul
    (0x2D5F3B, ('\u{96A0}', false)), // East Asian ideograph
    (0x6F4963, ('\u{AD20}', false)), // Korean hangul
    (0x6F5132, ('\u{BCA1}', false)), // Korean hangul
    (0x224352, ('\u{6B01}', false)), // East Asian ideograph
    (0x4B4A74, ('\u{731C}', false)), // East Asian ideograph (variant of 214A74 which maps to 731C)
    (0x234354, ('\u{927E}', false)), // East Asian ideograph
    (0x6F5C47, ('\u{D45C}', false)), // Korean hangul
    (0x274355, ('\u{4E66}', false)), // East Asian ideograph
    (0x6F4964, ('\u{AD28}', false)), // Korean hangul
    (0x334357, ('\u{6702}', false)), // East Asian ideograph
    (0x223348, ('\u{6435}', false)), // East Asian ideograph
    (0x224358, ('\u{6B03}', false)), // East Asian ideograph
    (0x224359, ('\u{6AF8}', false)), // East Asian ideograph
    (0x21605F, ('\u{98B6}', false)), // East Asian ideograph
    (0x4D6047, ('\u{816D}', false)), // East Asian ideograph
    (0x27435A, ('\u{4F1A}', false)), // East Asian ideograph
    (0x23435B, ('\u{9291}', false)), // East Asian ideograph
    (0x27603C, ('\u{987B}', false)), // East Asian ideograph
    (0x6F4965, ('\u{AD29}', false)), // Korean hangul
    (0x6F4F7E, ('\u{BA00}', false)), // Korean hangul
    (0x23435D, ('\u{929B}', false)), // East Asian ideograph
    (0x233748, ('\u{8D6C}', false)), // East Asian ideograph
    (0x2D305B, ('\u{4EBE}', false)), // East Asian ideograph
    (0x217573, ('\u{57D5}', false)), // East Asian ideograph
    (0x6F562F, ('\u{C67C}', false)), // Korean hangul
    (0x284B43, ('\u{8365}', false)), // East Asian ideograph
    (0x22435F, ('\u{6B0D}', false)), // East Asian ideograph
    (0x6F4B3B, ('\u{B048}', false)), // Korean hangul
    (0x224360, ('\u{6B09}', false)), // East Asian ideograph
    (0x355053, ('\u{98C8}', false)), // East Asian ideograph
    (0x6F4966, ('\u{AD2D}', false)), // Korean hangul
    (0x224361, ('\u{6B0E}', false)), // East Asian ideograph
    (0x225563, ('\u{726E}', false)), // East Asian ideograph
    (0x234362, ('\u{927F}', false)), // East Asian ideograph
    (0x69243C, ('\u{305C}', false)), // Hiragana letter ZE
    (0x6F5630, ('\u{C680}', false)), // Korean hangul
    (0x234364, ('\u{92A3}', false)), // East Asian ideograph
    (0x6F4967, ('\u{AD34}', false)), // Korean hangul
    (0x275852, ('\u{8BCF}', false)), // East Asian ideograph
    (0x214366, ('\u{6727}', false)), // East Asian ideograph
    (0x224367, ('\u{6B11}', false)), // East Asian ideograph
    (0x2D4E33, ('\u{78AA}', false)), // East Asian ideograph
    (0x3F5631, ('\u{517F}', false)), // East Asian ideograph
    (0x334369, ('\u{5932}', false)), // East Asian ideograph
    (0x234857, ('\u{9464}', false)), // East Asian ideograph
    (0x21436A, ('\u{672B}', false)), // East Asian ideograph
    (0x293032, ('\u{7962}', false)), // East Asian ideograph
    (0x6F4968, ('\u{AD38}', false)), // Korean hangul
    (0x275853, ('\u{8BC5}', false)), // East Asian ideograph
    (0x33496A, ('\u{934A}', false)), // East Asian ideograph
    (0x22436D, ('\u{6B19}', false)), // East Asian ideograph
    (0x4B5179, ('\u{7F0B}', false)), // East Asian ideograph
    (0x6F5632, ('\u{C68B}', false)), // Korean hangul
    (0x23436F, ('\u{92D0}', false)), // East Asian ideograph
    (0x6F4969, ('\u{AD3C}', false)), // Korean hangul
    (0x2D4370, ('\u{6736}', false)), // East Asian ideograph
    (0x215167, ('\u{7E46}', false)), // East Asian ideograph
    (0x234371, ('\u{92F1}', false)), // East Asian ideograph
    (0x234372, ('\u{92DF}', false)), // East Asian ideograph
    (0x275528, ('\u{835A}', false)), // East Asian ideograph
    (0x215E31, ('\u{93C8}', false)), // East Asian ideograph
    (0x6F496A, ('\u{AD44}', false)), // Korean hangul
    (0x214375, ('\u{6750}', false)), // East Asian ideograph
    (0x215168, ('\u{7E37}', false)), // East Asian ideograph
    (0x6F572B, ('\u{C7BC}', false)), // Korean hangul
    (0x234376, ('\u{92B6}', false)), // East Asian ideograph
    (0x4B4638, ('\u{6BB1}', false)), // East Asian ideograph
    (0x234377, ('\u{92C0}', false)), // East Asian ideograph
    (0x6F5634, ('\u{C694}', false)), // Korean hangul
    (0x6F4D2B, ('\u{B35B}', false)), // Korean hangul
    (0x6F4B3C, ('\u{B04A}', false)), // Korean hangul
    (0x234379, ('\u{92BE}', false)), // East Asian ideograph
    (0x6F572E, ('\u{C7C0}', false)), // Korean hangul
    (0x2D3061, ('\u{4EB0}', false)), // East Asian ideograph
    (0x6F5A78, ('\u{D0D4}', false)), // Korean hangul
    (0x6F5635, ('\u{C695}', false)), // Korean hangul
    (0x29437D, ('\u{94D8}', false)), // East Asian ideograph
    (0x696E28, ('\u{9056}', false)), // East Asian ideograph
    (0x4B5746, ('\u{8853}', false)), // East Asian ideograph
    (0x23437E, ('\u{92D5}', false)), // East Asian ideograph
    (0x2D3D2B, ('\u{5EBF}', false)), // East Asian ideograph
    (0x6F4A62, ('\u{AED1}', false)), // Korean hangul
    (0x276E2A, ('\u{53A3}', false)), // East Asian ideograph
    (0x2D527B, ('\u{8074}', false)), // East Asian ideograph
    (0x6F5A79, ('\u{D0D5}', false)), // Korean hangul
    (0x282D74, ('\u{6004}', false)), // East Asian ideograph
    (0x6F5636, ('\u{C698}', false)), // Korean hangul
    (0x23485C, ('\u{9465}', false)), // East Asian ideograph
    (0x226233, ('\u{774A}', false)), // East Asian ideograph
    (0x6F496D, ('\u{AD50}', false)), // Korean hangul
    (0x6F5C49, ('\u{D478}', false)), // Korean hangul
    (0x6F5840, ('\u{C9ED}', false)), // Korean hangul
    (0x222E2F, ('\u{615C}', false)), // East Asian ideograph
    (0x223351, ('\u{640A}', false)), // East Asian ideograph
    (0x21317D, ('\u{501A}', false)), // East Asian ideograph
    (0x6F5A7A, ('\u{D0DC}', false)), // Korean hangul
    (0x2E7451, ('\u{7F58}', false)), // East Asian ideograph
    (0x6F5637, ('\u{C6A5}', false)), // Korean hangul
    (0x215E35, ('\u{93DD}', false)), // East Asian ideograph
    (0x23485D, ('\u{9455}', false)), // East Asian ideograph
    (0x6F4E31, ('\u{B5A1}', false)), // Korean hangul
    (0x225E2C, ('\u{7590}', false)), // East Asian ideograph
    (0x2D3D2D, ('\u{5396}', false)), // East Asian ideograph
    (0x275859, ('\u{8BE5}', false)), // East Asian ideograph
    (0x21516C, ('\u{7E2B}', false)), // East Asian ideograph
    (0x225128, ('\u{70E0}', false)), // East Asian ideograph
    (0x6F5A7B, ('\u{D0DD}', false)), // Korean hangul
    (0x275529, ('\u{830E}', false)), // East Asian ideograph
    (0x22445F, ('\u{6B6E}', false)), // East Asian ideograph
    (0x33485E, ('\u{67D2}', false)), // East Asian ideograph
    (0x226235, ('\u{7743}', false)), // East Asian ideograph
    (0x2D4171, ('\u{62CA}', false)), // East Asian ideograph
    (0x6F496F, ('\u{AD6D}', false)), // Korean hangul
    (0x6F572C, ('\u{C7BD}', false)), // Korean hangul
    (0x27493A, ('\u{6FD1}', false)), // East Asian ideograph
    (0x23596B, ('\u{9C7A}', false)), // East Asian ideograph
    (0x235B59, ('\u{9DA9}', false)), // East Asian ideograph
    (0x27474E, ('\u{6CFE}', false)), // East Asian ideograph
    (0x6F5639, ('\u{C6A9}', false)), // Korean hangul
    (0x215E37, ('\u{93D8}', false)), // East Asian ideograph
    (0x222E3D, ('\u{61A2}', false)), // East Asian ideograph
    (0x2D3D2F, ('\u{539B}', false)), // East Asian ideograph
    (0x6F5946, ('\u{CCD0}', false)), // Korean hangul
    (0x345D6B, ('\u{756D}', false)), // East Asian ideograph
    (0x285D6B, ('\u{7572}', false)), // East Asian ideograph
    (0x222E40, ('\u{61A8}', false)), // East Asian ideograph
    (0x6F563A, ('\u{C6B0}', false)), // Korean hangul
    (0x4B3C2B, ('\u{67C3}', false)), // East Asian ideograph (Version J extension)
    (0x6F4F37, ('\u{B840}', false)), // Korean hangul
    (0x6F4971, ('\u{AD73}', false)), // Korean hangul
    (0x6F4A63, ('\u{AED8}', false)), // Korean hangul
    (0x22512B, ('\u{70D4}', false)), // East Asian ideograph
    (0x23552A, ('\u{9AEF}', false)), // East Asian ideograph
    (0x6F5A7E, ('\u{D0EC}', false)), // Korean hangul
    (0x222E45, ('\u{6196}', false)), // East Asian ideograph
    (0x6F563B, ('\u{C6B1}', false)), // Korean hangul
    (0x6F4972, ('\u{AD74}', false)), // Korean hangul
    (0x6F563C, ('\u{C6B4}', false)), // Korean hangul
    (0x234862, ('\u{946A}', false)), // East Asian ideograph
    (0x282E4C, ('\u{6126}', false)), // East Asian ideograph
    (0x2D5F4B, ('\u{96D1}', false)), // East Asian ideograph
    (0x6F4973, ('\u{AD75}', false)), // Korean hangul
    (0x215171, ('\u{7E54}', false)), // East Asian ideograph
    (0x6F563D, ('\u{C6B7}', false)), // Korean hangul
    (0x215E3B, ('\u{93FD}', false)), // East Asian ideograph
    (0x2D4176, ('\u{6483}', false)), // East Asian ideograph
    (0x2D5F4C, ('\u{9DC4}', false)), // East Asian ideograph
    (0x6F4974, ('\u{AD76}', false)), // Korean hangul
    (0x6F5D32, ('\u{D613}', false)), // Korean hangul
    (0x282E52, ('\u{6003}', false)), // East Asian ideograph
    (0x27493B, ('\u{6CA5}', false)), // East Asian ideograph
    (0x233941, ('\u{8DFD}', false)), // East Asian ideograph
    (0x6F563E, ('\u{C6B8}', false)), // Korean hangul
    (0x4B5773, ('\u{7ED4}', false)), // East Asian ideograph
    (0x213C23, ('\u{5D22}', false)), // East Asian ideograph
    (0x286E56, ('\u{7BA8}', false)), // East Asian ideograph
    (0x2D3D34, ('\u{5EFE}', false)), // East Asian ideograph
    (0x215173, ('\u{7E5E}', false)), // East Asian ideograph
    (0x223359, ('\u{6407}', false)), // East Asian ideograph
    (0x216E58, ('\u{535F}', false)), // East Asian ideograph
    (0x707523, ('\u{9170}', false)), // East Asian ideograph
    (0x6F4B77, ('\u{B119}', false)), // Korean hangul
    (0x222E5A, ('\u{61CB}', false)), // East Asian ideograph
    (0x213C24, ('\u{5D29}', false)), // East Asian ideograph
    (0x33355C, ('\u{5449}', false)), // East Asian ideograph
    (0x273648, ('\u{95EE}', false)), // East Asian ideograph
    (0x282E5C, ('\u{603F}', false)), // East Asian ideograph
    (0x27514C, ('\u{7EFF}', false)), // East Asian ideograph
    (0x6F5579, ('\u{C635}', false)), // Korean hangul
    (0x6F5640, ('\u{C6BA}', false)), // Korean hangul
    (0x6F5951, ('\u{CD5C}', false)), // Korean hangul
    (0x4B397B, ('\u{5A2F}', false)), // East Asian ideograph
    (0x29402C, ('\u{90D0}', false)), // East Asian ideograph
    (0x22623D, ('\u{7760}', false)), // East Asian ideograph
    (0x6F4977, ('\u{AD7F}', false)), // Korean hangul
    (0x6F5136, ('\u{BCB0}', false)), // Korean hangul
    (0x216E61, ('\u{5414}', false)), // East Asian ideograph
    (0x22335B, ('\u{643B}', false)), // East Asian ideograph
    (0x6F4A2E, ('\u{ADFC}', false)), // Korean hangul
    (0x6F5641, ('\u{C6C0}', false)), // Korean hangul
    (0x213C26, ('\u{5D19}', false)), // East Asian ideograph
    (0x275863, ('\u{8BE1}', false)), // East Asian ideograph
    (0x695A7E, ('\u{66BC}', false)), // East Asian ideograph
    (0x287E61, ('\u{82CC}', false)), // East Asian ideograph
    (0x232E68, ('\u{88D2}', false)), // East Asian ideograph
    (0x6F5642, ('\u{C6C1}', false)), // Korean hangul
    (0x234868, ('\u{946B}', false)), // East Asian ideograph
    (0x6F5429, ('\u{C2EC}', false)), // Korean hangul
    (0x334425, ('\u{76C3}', false)), // East Asian ideograph
    (0x6F4979, ('\u{AD82}', false)), // Korean hangul
    (0x296062, ('\u{9F9B}', false)), // East Asian ideograph
    (0x22335D, ('\u{643F}', false)), // East Asian ideograph
    (0x23375C, ('\u{8D7A}', false)), // East Asian ideograph
    (0x6F5643, ('\u{C6C3}', false)), // Korean hangul
    (0x213C28, ('\u{5D50}', false)), // East Asian ideograph
    (0x216E6F, ('\u{541A}', false)), // East Asian ideograph
    (0x6F497A, ('\u{AD88}', false)), // Korean hangul
    (0x275422, ('\u{810F}', false)), // East Asian ideograph (duplicate simplified)
    (0x222E71, ('\u{61E0}', false)), // East Asian ideograph
    (0x2D3E40, ('\u{6052}', false)), // East Asian ideograph
    (0x6F5644, ('\u{C6C5}', false)), // Korean hangul
    (0x28702E, ('\u{56E2}', false)), // East Asian ideograph (duplicate simplified)
    (0x6F497B, ('\u{AD8C}', false)), // Korean hangul
    (0x6F4A65, ('\u{AEF4}', false)), // Korean hangul
    (0x455164, ('\u{53BF}', false)), // East Asian ideograph
    (0x215921, ('\u{8AC2}', false)), // East Asian ideograph
    (0x222E77, ('\u{61E5}', false)), // East Asian ideograph
    (0x275922, ('\u{8C01}', false)), // East Asian ideograph
    (0x275923, ('\u{8BDE}', false)), // East Asian ideograph
    (0x232652, ('\u{85A2}', false)), // East Asian ideograph
    (0x6F552F, ('\u{C564}', false)), // Korean hangul
    (0x216E79, ('\u{5454}', false)), // East Asian ideograph
    (0x275924, ('\u{8BBA}', false)), // East Asian ideograph
    (0x235925, ('\u{9C32}', false)), // East Asian ideograph
    (0x215926, ('\u{8AFA}', false)), // East Asian ideograph
    (0x275927, ('\u{8C0F}', false)), // East Asian ideograph
    (0x216E7D, ('\u{543D}', false)), // East Asian ideograph
    (0x235928, ('\u{9C48}', false)), // East Asian ideograph
    (0x282E7E, ('\u{603C}', false)), // East Asian ideograph
    (0x215929, ('\u{8AE7}', false)), // East Asian ideograph
    (0x275868, ('\u{8BDF}', false)), // East Asian ideograph
    (0x6F505D, ('\u{BBC8}', false)), // Korean hangul
    (0x23592A, ('\u{9C33}', false)), // East Asian ideograph
    (0x21592B, ('\u{8B00}', false)), // East Asian ideograph
    (0x6F5B72, ('\u{D31D}', false)), // Korean hangul
    (0x27592C, ('\u{8C12}', false)), // East Asian ideograph
    (0x6F5647, ('\u{C6D0}', false)), // Korean hangul
    (0x29416B, ('\u{948E}', false)), // East Asian ideograph
    (0x213E47, ('\u{6064}', false)), // East Asian ideograph
    (0x23486D, ('\u{9471}', false)), // East Asian ideograph
    (0x27592E, ('\u{8BFA}', false)), // East Asian ideograph
    (0x22592F, ('\u{73D4}', false)), // East Asian ideograph
    (0x27493D, ('\u{6F47}', false)), // East Asian ideograph
    (0x233761, ('\u{8D84}', false)), // East Asian ideograph
    (0x215930, ('\u{8AED}', false)), // East Asian ideograph
    (0x335A7B, ('\u{8E28}', false)), // East Asian ideograph
    (0x275931, ('\u{8C24}', false)), // East Asian ideograph
    (0x6F5648, ('\u{C6D4}', false)), // Korean hangul
    (0x697174, ('\u{9ADE}', false)), // East Asian ideograph
    (0x235932, ('\u{9C35}', false)), // East Asian ideograph
    (0x275933, ('\u{8C1C}', false)), // East Asian ideograph
    (0x27586A, ('\u{8BF5}', false)), // East Asian ideograph
    (0x215934, ('\u{8B1B}', false)), // East Asian ideograph
    (0x215935, ('\u{8B0A}', false)), // East Asian ideograph
    (0x225936, ('\u{73E7}', false)), // East Asian ideograph
    (0x6F5649, ('\u{C6DC}', false)), // Korean hangul
    (0x275937, ('\u{8A8A}', false)), // East Asian ideograph
    (0x215938, ('\u{8B1D}', false)), // East Asian ideograph
    (0x27586B, ('\u{8BEB}', false)), // East Asian ideograph
    (0x6F4A66, ('\u{AF0D}', false)), // Korean hangul
    (0x282F66, ('\u{6217}', false)), // East Asian ideograph
    (0x215939, ('\u{8B39}', false)), // East Asian ideograph
    (0x22513A, ('\u{70D0}', false)), // East Asian ideograph
    (0x21593A, ('\u{8B2C}', false)), // East Asian ideograph
    (0x21593B, ('\u{8B28}', false)), // East Asian ideograph
    (0x6F564A, ('\u{C6DD}', false)), // Korean hangul
    (0x224471, ('\u{6B82}', false)), // East Asian ideograph
    (0x21593C, ('\u{8B58}', false)), // East Asian ideograph
    (0x27593D, ('\u{8C31}', false)), // East Asian ideograph
    (0x6F5138, ('\u{BCB3}', false)), // Korean hangul
    (0x27586C, ('\u{8BED}', false)), // East Asian ideograph
    (0x27593E, ('\u{8C32}', false)), // East Asian ideograph
    (0x22513B, ('\u{70C7}', false)), // East Asian ideograph
    (0x22593F, ('\u{73E9}', false)), // East Asian ideograph
    (0x274E5B, ('\u{77FF}', false)), // East Asian ideograph
    (0x215940, ('\u{8B5A}', false)), // East Asian ideograph
    (0x6F564B, ('\u{C6DF}', false)), // Korean hangul
    (0x395577, ('\u{854B}', false)), // East Asian ideograph
    (0x213C30, ('\u{5DCD}', false)), // East Asian ideograph
    (0x215942, ('\u{8B4F}', false)), // East Asian ideograph
    (0x6F573B, ('\u{C813}', false)), // Korean hangul
    (0x275943, ('\u{8BAE}', false)), // East Asian ideograph
    (0x22472C, ('\u{6C54}', false)), // East Asian ideograph
    (0x22513C, ('\u{70DA}', false)), // East Asian ideograph
    (0x6F5944, ('\u{CCBC}', false)), // Korean hangul
    (0x2D435F, ('\u{6716}', false)), // East Asian ideograph
    (0x6F5B73, ('\u{D31F}', false)), // Korean hangul
    (0x235945, ('\u{9C51}', false)), // East Asian ideograph
    (0x6F564C, ('\u{C6E0}', false)), // Korean hangul
    (0x69255A, ('\u{30DA}', false)), // Katakana letter PE
    (0x213C31, ('\u{5DD2}', false)), // East Asian ideograph
    (0x215947, ('\u{8B74}', false)), // East Asian ideograph
    (0x215948, ('\u{8B77}', false)), // East Asian ideograph
    (0x2E4C7B, ('\u{6E86}', false)), // East Asian ideograph
    (0x22513D, ('\u{70C6}', false)), // East Asian ideograph
    (0x235949, ('\u{9C63}', false)), // East Asian ideograph
    (0x333323, ('\u{4E21}', false)), // East Asian ideograph
    (0x22594A, ('\u{73F8}', false)), // East Asian ideograph
    (0x6F564D, ('\u{C6E1}', false)), // Korean hangul
    (0x6F5275, ('\u{C0DB}', false)), // Korean hangul
    (0x27594B, ('\u{53D8}', false)), // East Asian ideograph
    (0x21594C, ('\u{8B93}', false)), // East Asian ideograph
    (0x27594D, ('\u{8C36}', false)), // East Asian ideograph
    (0x28582B, ('\u{7303}', false)), // East Asian ideograph
    (0x27594E, ('\u{8C17}', false)), // East Asian ideograph
    (0x21594F, ('\u{8B9A}', false)), // East Asian ideograph
    (0x6F564E, ('\u{C6E8}', false)), // Korean hangul
    (0x4B3C2F, ('\u{5DBA}', false)), // East Asian ideograph
    (0x287030, ('\u{7C9D}', false)), // East Asian ideograph
    (0x213C33, ('\u{5DD6}', false)), // East Asian ideograph
    (0x6F5A2C, ('\u{CEF9}', false)), // Korean hangul
    (0x2D3253, ('\u{50E3}', false)), // East Asian ideograph
    (0x6F4A67, ('\u{AF2C}', false)), // Korean hangul
    (0x6F5952, ('\u{CD78}', false)), // Korean hangul
    (0x333768, ('\u{8FF4}', false)), // East Asian ideograph
    (0x21373F, ('\u{5659}', false)), // East Asian ideograph
    (0x393439, ('\u{61C3}', false)), // East Asian ideograph
    (0x215954, ('\u{8C48}', false)), // East Asian ideograph
    (0x395652, ('\u{87A1}', false)), // East Asian ideograph
    (0x215955, ('\u{8C49}', false)), // East Asian ideograph
    (0x215956, ('\u{8C4C}', false)), // East Asian ideograph
    (0x396167, ('\u{9B2A}', false)), // East Asian ideograph
    (0x706B5B, ('\u{810E}', false)), // East Asian ideograph
    (0x275957, ('\u{7AD6}', false)), // East Asian ideograph
    (0x215958, ('\u{8C50}', false)), // East Asian ideograph
    (0x474931, ('\u{95F6}', false)), // East Asian ideograph
    (0x2D5959, ('\u{8277}', false)), // East Asian ideograph
    (0x213665, ('\u{558A}', false)), // East Asian ideograph
    (0x22595A, ('\u{73FD}', false)), // East Asian ideograph
    (0x6F4E32, ('\u{B5A4}', false)), // Korean hangul
    (0x217C24, ('\u{5AA7}', false)), // East Asian ideograph
    (0x2F5973, ('\u{9CEC}', false)), // East Asian ideograph
    (0x6F595B, ('\u{CDB0}', false)), // Korean hangul
    (0x21595C, ('\u{8C62}', false)), // East Asian ideograph
    (0x4B4655, ('\u{6C17}', false)), // East Asian ideograph
    (0x6F595D, ('\u{CDCC}', false)), // Korean hangul
    (0x455D3E, ('\u{9485}', false)), // East Asian ideograph
    (0x6F5B74, ('\u{D320}', false)), // Korean hangul
    (0x21595E, ('\u{8C6B}', false)), // East Asian ideograph
    (0x6F5651, ('\u{C6F0}', false)), // Korean hangul
    (0x69717D, ('\u{9AF1}', false)), // East Asian ideograph
    (0x2D595F, ('\u{732A}', false)), // East Asian ideograph
    (0x2D5960, ('\u{72B2}', false)), // East Asian ideograph
    (0x6F5731, ('\u{C7C9}', false)), // Korean hangul
    (0x513A47, ('\u{8885}', false)), // East Asian ideograph
    (0x6F5962, ('\u{CE30}', false)), // Korean hangul
    (0x39505B, ('\u{9B3B}', false)), // East Asian ideograph
    (0x215963, ('\u{8C8A}', false)), // East Asian ideograph
    (0x6F5652, ('\u{C6F8}', false)), // Korean hangul
    (0x224479, ('\u{6B8D}', false)), // East Asian ideograph
    (0x4B5964, ('\u{72E2}', false)), // East Asian ideograph
    (0x234435, ('\u{92DD}', false)), // East Asian ideograph
    (0x2D5965, ('\u{72F8}', false)), // East Asian ideograph
    (0x2D3D48, ('\u{5F4A}', false)), // East Asian ideograph
    (0x275966, ('\u{7683}', false)), // East Asian ideograph
    (0x213F79, ('\u{6249}', false)), // East Asian ideograph
    (0x215967, ('\u{8C93}', false)), // East Asian ideograph
    (0x215968, ('\u{8C9D}', false)), // East Asian ideograph
    (0x295B77, ('\u{9E63}', false)), // East Asian ideograph
    (0x215969, ('\u{8C9E}', false)), // East Asian ideograph
    (0x217C27, ('\u{5A9C}', false)), // East Asian ideograph
    (0x6F5C2D, ('\u{D399}', false)), // Korean hangul
    (0x27596A, ('\u{8D1F}', false)), // East Asian ideograph
    (0x6F4A68, ('\u{AF2D}', false)), // Korean hangul
    (0x706B5F, ('\u{8112}', false)), // East Asian ideograph
    (0x21596B, ('\u{8CA2}', false)), // East Asian ideograph
    (0x52735D, ('\u{7E8A}', false)), // East Asian ideograph (variant of 22735D which maps to 7E8A)
    (0x695C30, ('\u{6923}', false)), // East Asian ideograph
    (0x225144, ('\u{7104}', false)), // East Asian ideograph
    (0x22596C, ('\u{7430}', false)), // East Asian ideograph
    (0x33332A, ('\u{4E93}', false)), // East Asian ideograph
    (0x21596D, ('\u{8CAC}', false)), // East Asian ideograph
    (0x21596E, ('\u{8CAB}', false)), // East Asian ideograph
    (0x217C28, ('\u{5A7C}', false)), // East Asian ideograph
    (0x697260, ('\u{9C30}', false)), // East Asian ideograph
    (0x27596F, ('\u{8D27}', false)), // East Asian ideograph
    (0x215970, ('\u{8CAA}', false)), // East Asian ideograph
    (0x695C31, ('\u{6921}', false)), // East Asian ideograph
    (0x215971, ('\u{8CA7}', false)), // East Asian ideograph
    (0x6F5C4F, ('\u{D48B}', false)), // Korean hangul
    (0x275E46, ('\u{9576}', false)), // East Asian ideograph
    (0x215972, ('\u{8CA9}', false)), // East Asian ideograph
    (0x215973, ('\u{8CAF}', false)), // East Asian ideograph
    (0x217C29, ('\u{5A96}', false)), // East Asian ideograph
    (0x6F5974, ('\u{CE85}', false)), // Korean hangul
    (0x275975, ('\u{8D39}', false)), // East Asian ideograph
    (0x6F5060, ('\u{BBF9}', false)), // Korean hangul
    (0x4B465A, ('\u{6C32}', false)), // East Asian ideograph
    (0x275976, ('\u{8D32}', false)), // East Asian ideograph
    (0x234421, ('\u{92C6}', false)), // East Asian ideograph
    (0x215977, ('\u{8CC0}', false)), // East Asian ideograph
    (0x6F5656, ('\u{C70C}', false)), // Korean hangul
    (0x277748, ('\u{57B2}', false)), // East Asian ideograph
    (0x215978, ('\u{8CB4}', false)), // East Asian ideograph
    (0x275979, ('\u{8D34}', false)), // East Asian ideograph
    (0x295B2A, ('\u{9E46}', false)), // East Asian ideograph
    (0x6F503E, ('\u{BAB0}', false)), // Korean hangul
    (0x21597A, ('\u{8CB7}', false)), // East Asian ideograph
    (0x234425, ('\u{92F4}', false)), // East Asian ideograph
    (0x27597B, ('\u{8D2C}', false)), // East Asian ideograph
    (0x274426, ('\u{4E1C}', false)), // East Asian ideograph
    (0x27597C, ('\u{8D3B}', false)), // East Asian ideograph
    (0x6F5657, ('\u{C714}', false)), // Korean hangul
    (0x234427, ('\u{92CF}', false)), // East Asian ideograph
    (0x292D51, ('\u{8511}', false)), // East Asian ideograph
    (0x21597D, ('\u{8CB8}', false)), // East Asian ideograph
    (0x23443A, ('\u{92CA}', false)), // East Asian ideograph
    (0x27597E, ('\u{8D38}', false)), // East Asian ideograph
    (0x23442A, ('\u{92B2}', false)), // East Asian ideograph
    (0x225148, ('\u{70F3}', false)), // East Asian ideograph
    (0x29442B, ('\u{9503}', false)), // East Asian ideograph
    (0x6F5324, ('\u{C120}', false)), // Korean hangul
    (0x6F5658, ('\u{C717}', false)), // Korean hangul
    (0x23442C, ('\u{92E7}', false)), // East Asian ideograph
    (0x294F6B, ('\u{98A1}', false)), // East Asian ideograph
    (0x696D41, ('\u{8EC8}', false)), // East Asian ideograph
    (0x23442D, ('\u{92C7}', false)), // East Asian ideograph
    (0x277D40, ('\u{5AF1}', false)), // East Asian ideograph
    (0x2D3D4E, ('\u{7BF2}', false)), // East Asian ideograph
    (0x23442E, ('\u{92F0}', false)), // East Asian ideograph
    (0x6F4A69, ('\u{AF30}', false)), // Korean hangul
    (0x23442F, ('\u{92DB}', false)), // East Asian ideograph
    (0x6F4E5E, ('\u{B768}', false)), // Korean hangul
    (0x234430, ('\u{92DC}', false)), // East Asian ideograph
    (0x2D4E5B, ('\u{945B}', false)), // East Asian ideograph
    (0x234431, ('\u{92D8}', false)), // East Asian ideograph
    (0x224432, ('\u{6B39}', false)), // East Asian ideograph
    (0x234433, ('\u{92E9}', false)), // East Asian ideograph
    (0x224435, ('\u{6B3F}', false)), // East Asian ideograph
    (0x274E5E, ('\u{783E}', false)), // East Asian ideograph
    (0x6F565A, ('\u{C720}', false)), // Korean hangul
    (0x27375A, ('\u{4E25}', false)), // East Asian ideograph
    (0x224437, ('\u{6B46}', false)), // East Asian ideograph
    (0x2D3D50, ('\u{5F5C}', false)), // East Asian ideograph
    (0x224438, ('\u{6B41}', false)), // East Asian ideograph
    (0x234439, ('\u{92D1}', false)), // East Asian ideograph
    (0x6F5061, ('\u{BBFC}', false)), // Korean hangul
    (0x283561, ('\u{64BA}', false)), // East Asian ideograph
    (0x22443A, ('\u{6B40}', false)), // East Asian ideograph
    (0x6F565B, ('\u{C721}', false)), // Korean hangul
    (0x22443B, ('\u{6B42}', false)), // East Asian ideograph
    (0x6F5973, ('\u{CE84}', false)), // Korean hangul
    (0x6F4C7E, ('\u{B301}', false)), // Korean hangul
    (0x23443C, ('\u{92C2}', false)), // East Asian ideograph
    (0x6F4F29, ('\u{B810}', false)), // Korean hangul
    (0x454C3C, ('\u{7589}', false)), // East Asian ideograph
    (0x6F5733, ('\u{C7D8}', false)), // Korean hangul
    (0x23443E, ('\u{92CC}', false)), // East Asian ideograph
    (0x22443F, ('\u{6B4A}', false)), // East Asian ideograph
    (0x235B60, ('\u{9D98}', false)), // East Asian ideograph
    (0x2E525D, ('\u{715B}', false)), // East Asian ideograph
    (0x6F565C, ('\u{C724}', false)), // Korean hangul
    (0x234440, ('\u{92EF}', false)), // East Asian ideograph
    (0x213C41, ('\u{5DF7}', false)), // East Asian ideograph
    (0x234441, ('\u{92E8}', false)), // East Asian ideograph
    (0x6F5D35, ('\u{D61C}', false)), // Korean hangul
    (0x287739, ('\u{8069}', false)), // East Asian ideograph
    (0x27587E, ('\u{8BFF}', false)), // East Asian ideograph
    (0x234443, ('\u{92EB}', false)), // East Asian ideograph
    (0x695C39, ('\u{697E}', false)), // East Asian ideograph
    (0x295D36, ('\u{9E2C}', false)), // East Asian ideograph
    (0x2D4444, ('\u{69C5}', false)), // East Asian ideograph
    (0x6F565D, ('\u{C728}', false)), // Korean hangul
    (0x234445, ('\u{92F5}', false)), // East Asian ideograph
    (0x224446, ('\u{6B4E}', false)), // East Asian ideograph (variant of 4C4446 which maps to 6B4E)
    (0x6F4A6A, ('\u{AF34}', false)), // Korean hangul
    (0x234448, ('\u{92F2}', false)), // East Asian ideograph
    (0x28422B, ('\u{6A2F}', false)), // East Asian ideograph
    (0x334449, ('\u{6144}', false)), // East Asian ideograph
    (0x22444A, ('\u{6B57}', false)), // East Asian ideograph
    (0x2D444B, ('\u{6852}', false)), // East Asian ideograph
    (0x6F5530, ('\u{C568}', false)), // Korean hangul
    (0x6F513C, ('\u{BCC0}', false)), // Korean hangul
    (0x22444C, ('\u{6B54}', false)), // East Asian ideograph
    (0x23444D, ('\u{9307}', false)), // East Asian ideograph
    (0x22444E, ('\u{6B55}', false)), // East Asian ideograph
    (0x6F5C51, ('\u{D4CC}', false)), // Korean hangul
    (0x515E5D, ('\u{9616}', false)), // East Asian ideograph
    (0x2D4450, ('\u{8308}', false)), // East Asian ideograph
    (0x224451, ('\u{6B5C}', false)), // East Asian ideograph
    (0x287A56, ('\u{8114}', false)), // East Asian ideograph
    (0x6F5062, ('\u{BBFF}', false)), // Korean hangul
    (0x212B38, ('\u{FF0C}', false)), // Ideographic variant comma
    (0x225150, ('\u{70F4}', false)), // East Asian ideograph
    (0x23554F, ('\u{9B10}', false)), // East Asian ideograph
    (0x224453, ('\u{6B5E}', false)), // East Asian ideograph
    (0x6F5B77, ('\u{D328}', false)), // Korean hangul
    (0x224454, ('\u{6B60}', false)), // East Asian ideograph
    (0x22625D, ('\u{777E}', false)), // East Asian ideograph
    (0x217C34, ('\u{5AAE}', false)), // East Asian ideograph
    (0x4B4456, ('\u{6813}', false)), // East Asian ideograph
    (0x6F5734, ('\u{C800}', false)), // Korean hangul
    (0x294457, ('\u{9529}', false)), // East Asian ideograph
    (0x212B39, ('\u{FF1B}', false)), // Ideographic semicolon
    (0x234458, ('\u{931F}', false)), // East Asian ideograph
    (0x6F5661, ('\u{C73C}', false)), // Korean hangul
    (0x23445A, ('\u{9331}', false)), // East Asian ideograph
    (0x4D5F70, ('\u{9F44}', false)), // East Asian ideograph
    (0x22445B, ('\u{6B6B}', false)), // East Asian ideograph
    (0x22736B, ('\u{7E95}', false)), // East Asian ideograph
    (0x22445D, ('\u{6B6C}', false)), // East Asian ideograph
    (0x4C284C, ('\u{53A9}', false)), // East Asian ideograph
    (0x4B3C33, ('\u{5DCC}', false)), // East Asian ideograph
    (0x215E60, ('\u{95BB}', false)), // East Asian ideograph
    (0x23445F, ('\u{930F}', false)), // East Asian ideograph
    (0x6F4A6B, ('\u{AF3C}', false)), // Korean hangul
    (0x224461, ('\u{6B71}', false)), // East Asian ideograph
    (0x6F5D2C, ('\u{D600}', false)), // Korean hangul
    (0x234462, ('\u{9302}', false)), // East Asian ideograph
    (0x6F5170, ('\u{BE4B}', false)), // Korean hangul
    (0x274463, ('\u{6761}', false)), // East Asian ideograph
    (0x234464, ('\u{9324}', false)), // East Asian ideograph
    (0x2D5B2F, ('\u{8EB1}', false)), // East Asian ideograph
    (0x214466, ('\u{6885}', false)), // East Asian ideograph
    (0x274468, ('\u{67AD}', false)), // East Asian ideograph
    (0x294F77, ('\u{989F}', false)), // East Asian ideograph
    (0x6F5221, ('\u{BE70}', false)), // Korean hangul
    (0x6F4B7A, ('\u{B11D}', false)), // Korean hangul
    (0x274469, ('\u{6800}', false)), // East Asian ideograph
    (0x2D5F73, ('\u{975A}', false)), // Unrelated variant of EACC 234C76 which maps to 975A
    (0x23446A, ('\u{9323}', false)), // East Asian ideograph
    (0x22446B, ('\u{6B7E}', false)), // East Asian ideograph
    (0x212B3D, ('\u{FF01}', false)), // Ideographic exclamation point
    (0x225155, ('\u{7111}', false)), // East Asian ideograph
    (0x23446C, ('\u{9321}', false)), // East Asian ideograph
    (0x4C683E, ('\u{79EB}', false)), // East Asian ideograph
    (0x27446D, ('\u{5F03}', false)), // East Asian ideograph
    (0x6F5222, ('\u{BE71}', false)), // Korean hangul
    (0x27446E, ('\u{6816}', false)), // East Asian ideograph
    (0x2D4E79, ('\u{5FA1}', false)), // East Asian ideograph
    (0x6F5735, ('\u{C801}', false)), // Korean hangul
    (0x2D4B43, ('\u{746F}', false)), // East Asian ideograph
    (0x274471, ('\u{680B}', false)), // East Asian ideograph
    (0x4B5A68, ('\u{8DF5}', false)), // East Asian ideograph (variant of 275A68 which maps to 8DF5)
    (0x234472, ('\u{9301}', false)), // East Asian ideograph
    (0x6F5223, ('\u{BE73}', false)), // Korean hangul
    (0x6F5326, ('\u{C124}', false)), // Korean hangul
    (0x224473, ('\u{6B84}', false)), // East Asian ideograph
    (0x2D3332, ('\u{5190}', false)), // East Asian ideograph
    (0x234474, ('\u{9315}', false)), // East Asian ideograph
    (0x47366F, ('\u{8D4D}', false)), // East Asian ideograph
    (0x294475, ('\u{9494}', false)), // East Asian ideograph
    (0x235556, ('\u{9B1D}', false)), // East Asian ideograph
    (0x234476, ('\u{9329}', false)), // East Asian ideograph
    (0x23386F, ('\u{8DBA}', false)), // East Asian ideograph
    (0x232F21, ('\u{88FC}', false)), // East Asian ideograph
    (0x2D4A26, ('\u{713C}', false)), // East Asian ideograph
    (0x287035, ('\u{7C74}', false)), // East Asian ideograph
    (0x6F5224, ('\u{BE74}', false)), // Korean hangul
    (0x234478, ('\u{932E}', false)), // East Asian ideograph
    (0x234479, ('\u{932A}', false)), // East Asian ideograph
    (0x6F4A6C, ('\u{AF3D}', false)), // Korean hangul
    (0x27447A, ('\u{67A3}', false)), // East Asian ideograph
    (0x6F5D2D, ('\u{D601}', false)), // Korean hangul
    (0x22447B, ('\u{6B95}', false)), // East Asian ideograph
    (0x21447C, ('\u{6912}', false)), // East Asian ideograph
    (0x216F27, ('\u{5423}', false)), // East Asian ideograph
    (0x213C4D, ('\u{5E11}', false)), // East Asian ideograph
    (0x2D447D, ('\u{684C}', false)), // East Asian ideograph
    (0x2D3D5E, ('\u{9AF4}', false)), // East Asian ideograph
    (0x23447E, ('\u{9335}', false)), // East Asian ideograph
    (0x706058, ('\u{562D}', false)), // East Asian ideograph
    (0x273671, ('\u{54DF}', false)), // East Asian ideograph
    (0x6F5226, ('\u{BE7B}', false)), // Korean hangul
    (0x232F2D, ('\u{8909}', false)), // East Asian ideograph
    (0x23444C, ('\u{9303}', false)), // East Asian ideograph
    (0x4D5B35, ('\u{9DAB}', false)), // East Asian ideograph
    (0x232F2F, ('\u{8918}', false)), // East Asian ideograph
    (0x6F5B79, ('\u{D32C}', false)), // Korean hangul
    (0x213D3C, ('\u{5F15}', false)), // East Asian ideograph
    (0x6F566A, ('\u{C774}', false)), // Korean hangul
    (0x6F5227, ('\u{BE7C}', false)), // Korean hangul
    (0x213C4F, ('\u{5E25}', false)), // East Asian ideograph
    (0x2D3632, ('\u{8A7B}', false)), // East Asian ideograph
    (0x346622, ('\u{589D}', false)), // East Asian ideograph
    (0x295C47, ('\u{9E68}', false)), // East Asian ideograph
    (0x293A2E, ('\u{8DC4}', false)), // East Asian ideograph
    (0x2E6F35, ('\u{6CD4}', false)), // East Asian ideograph
    (0x6F566B, ('\u{C775}', false)), // Korean hangul
    (0x6F5228, ('\u{BE7D}', false)), // Korean hangul
    (0x22516D, ('\u{7134}', false)), // East Asian ideograph
    (0x23444E, ('\u{931E}', false)), // East Asian ideograph
    (0x2F5D49, ('\u{9EA4}', false)), // East Asian ideograph
    (0x2E313A, ('\u{6332}', false)), // East Asian ideograph
    (0x216F3A, ('\u{546D}', false)), // East Asian ideograph
    (0x6F566C, ('\u{C778}', false)), // Korean hangul
    (0x282458, ('\u{5D03}', false)), // East Asian ideograph
    (0x216F3B, ('\u{5491}', false)), // East Asian ideograph
    (0x6F5229, ('\u{BE80}', false)), // Korean hangul
    (0x4D5F7B, ('\u{97F2}', false)), // East Asian ideograph
    (0x222F3D, ('\u{6201}', false)), // East Asian ideograph
    (0x295C49, ('\u{9E47}', false)), // East Asian ideograph
    (0x4B577E, ('\u{7E7F}', false)), // East Asian ideograph
    (0x217C41, ('\u{5AC4}', false)), // East Asian ideograph
    (0x215A28, ('\u{8CC2}', false)), // East Asian ideograph
    (0x697265, ('\u{9C5A}', false)), // East Asian ideograph
    (0x216F42, ('\u{5494}', false)), // East Asian ideograph
    (0x232F43, ('\u{8915}', false)), // East Asian ideograph
    (0x333344, ('\u{51DB}', false)), // East Asian ideograph
    (0x6F566E, ('\u{C77D}', false)), // Korean hangul
    (0x274340, ('\u{6656}', false)), // East Asian ideograph
    (0x6F522B, ('\u{BE8C}', false)), // Korean hangul
    (0x213C53, ('\u{5E36}', false)), // East Asian ideograph
    (0x4C5175, ('\u{8315}', false)), // East Asian ideograph
    (0x225E37, ('\u{75A2}', false)), // East Asian ideograph
    (0x222F47, ('\u{6214}', false)), // East Asian ideograph
    (0x2D3921, ('\u{591F}', false)), // East Asian ideograph
    (0x233345, ('\u{8AE2}', false)), // East Asian ideograph
    (0x216F49, ('\u{548D}', false)), // East Asian ideograph
    (0x355D5C, ('\u{8C8E}', false)), // East Asian ideograph
    (0x6F566F, ('\u{C783}', false)), // Korean hangul
    (0x216F4A, ('\u{5463}', false)), // East Asian ideograph
    (0x6F522C, ('\u{BE8F}', false)), // Korean hangul
    (0x6F5737, ('\u{C808}', false)), // Korean hangul
    (0x225160, ('\u{70F6}', false)), // East Asian ideograph
    (0x6F5670, ('\u{C784}', false)), // Korean hangul
    (0x234453, ('\u{931D}', false)), // East Asian ideograph
    (0x216F7B, ('\u{551A}', false)), // East Asian ideograph
    (0x6F5D68, ('\u{D744}', false)), // Korean hangul
    (0x6F5671, ('\u{C785}', false)), // Korean hangul
    (0x6F522E, ('\u{BE91}', false)), // Korean hangul
    (0x294427, ('\u{94D7}', false)), // East Asian ideograph
    (0x234454, ('\u{92FA}', false)), // East Asian ideograph
    (0x2D3D67, ('\u{9015}', false)), // East Asian ideograph
    (0x6F4A6E, ('\u{AF41}', false)), // Korean hangul
    (0x222F56, ('\u{6223}', false)), // East Asian ideograph
    (0x6F4F43, ('\u{B8E8}', false)), // Korean hangul
    (0x6F5D2F, ('\u{D608}', false)), // Korean hangul
    (0x335561, ('\u{8462}', false)), // East Asian ideograph
    (0x216F58, ('\u{54A1}', false)), // East Asian ideograph
    (0x6F5672, ('\u{C787}', false)), // Korean hangul
    (0x274344, ('\u{6682}', false)), // East Asian ideograph
    (0x6F522F, ('\u{BE98}', false)), // Korean hangul
    (0x213C57, ('\u{5E3D}', false)), // East Asian ideograph
    (0x4B356A, ('\u{55EC}', false)), // East Asian ideograph
    (0x23223C, ('\u{83F3}', false)), // East Asian ideograph
    (0x696F5B, ('\u{958A}', false)), // East Asian ideograph
    (0x273238, ('\u{4FA6}', false)), // East Asian ideograph
    (0x695C4F, ('\u{69DD}', false)), // East Asian ideograph
    (0x222F5D, ('\u{6224}', false)), // East Asian ideograph
    (0x6F5673, ('\u{C788}', false)), // Korean hangul
    (0x216F5E, ('\u{54BE}', false)), // East Asian ideograph
    (0x6F5230, ('\u{BEA8}', false)), // Korean hangul
    (0x213C58, ('\u{5E40}', false)), // East Asian ideograph
    (0x692433, ('\u{3053}', false)), // Hiragana letter KO
    (0x292F60, ('\u{88E2}', false)), // East Asian ideograph
    (0x6F5066, ('\u{BC0B}', false)), // Korean hangul
    (0x4C2F61, ('\u{622C}', false)), // East Asian ideograph
    (0x4B4235, ('\u{6442}', false)), // East Asian ideograph
    (0x6F5B7B, ('\u{D338}', false)), // Korean hangul
    (0x6F5C32, ('\u{D3AB}', false)), // Korean hangul
    (0x6F5231, ('\u{BED0}', false)), // Korean hangul
    (0x213C59, ('\u{5E4C}', false)), // East Asian ideograph
    (0x216F64, ('\u{54B5}', false)), // East Asian ideograph
    (0x225E2E, ('\u{7594}', false)), // East Asian ideograph
    (0x6F5738, ('\u{C80A}', false)), // Korean hangul
    (0x2D3F24, ('\u{661A}', false)), // East Asian ideograph
    (0x226F66, ('\u{7CCE}', false)), // East Asian ideograph
    (0x4B4236, ('\u{643A}', false)), // East Asian ideograph
    (0x6F5A30, ('\u{CF04}', false)), // Korean hangul
    (0x2D4A34, ('\u{718F}', false)), // East Asian ideograph
    (0x226F68, ('\u{7CC8}', false)), // East Asian ideograph
    (0x6F5A28, ('\u{CEF4}', false)), // Korean hangul
    (0x6F5232, ('\u{BED1}', false)), // Korean hangul
    (0x222F69, ('\u{97EF}', false)), // East Asian ideograph
    (0x333428, ('\u{523C}', false)), // East Asian ideograph
    (0x6F5676, ('\u{C78E}', false)), // Korean hangul
    (0x216F6D, ('\u{54AE}', false)), // East Asian ideograph
    (0x6F5233, ('\u{BED4}', false)), // Korean hangul
    (0x2D3D6C, ('\u{5F93}', false)), // East Asian ideograph
    (0x6F4A6F, ('\u{AF42}', false)), // Korean hangul
    (0x232F6F, ('\u{894F}', false)), // East Asian ideograph
    (0x2D5B42, ('\u{8F19}', false)), // East Asian ideograph
    (0x393E7D, ('\u{7609}', false)), // East Asian ideograph
    (0x695C53, ('\u{6A2E}', false)), // East Asian ideograph
    (0x225167, ('\u{70EF}', false)), // East Asian ideograph
    (0x235566, ('\u{9B23}', false)), // East Asian ideograph
    (0x216F71, ('\u{54BF}', false)), // East Asian ideograph
    (0x6F5677, ('\u{C655}', false)), // Korean hangul
    (0x292F72, ('\u{88E5}', false)), // East Asian ideograph
    (0x6F5234, ('\u{BED7}', false)), // Korean hangul
    (0x213C5C, ('\u{5E57}', false)), // East Asian ideograph
    (0x4D2F73, ('\u{7E5D}', false)), // East Asian ideograph
    (0x6F4F4C, ('\u{B93C}', false)), // Korean hangul
    (0x2D5B43, ('\u{8EFD}', false)), // East Asian ideograph
    (0x226F75, ('\u{7CD7}', false)), // East Asian ideograph
    (0x225168, ('\u{7100}', false)), // East Asian ideograph
    (0x33334E, ('\u{51FE}', false)), // East Asian ideograph
    (0x225A21, ('\u{7428}', false)), // East Asian ideograph
    (0x215A22, ('\u{8CC7}', false)), // East Asian ideograph
    (0x23445B, ('\u{9306}', false)), // East Asian ideograph
    (0x215A23, ('\u{8CCA}', false)), // East Asian ideograph
    (0x6F536C, ('\u{C274}', false)), // Korean hangul
    (0x4B312D, ('\u{4F2B}', false)), // East Asian ideograph
    (0x235A24, ('\u{9D04}', false)), // East Asian ideograph
    (0x21322A, ('\u{5003}', false)), // East Asian ideograph
    (0x222F7A, ('\u{6250}', false)), // East Asian ideograph
    (0x215A25, ('\u{8CC4}', false)), // East Asian ideograph
    (0x6F5D60, ('\u{D71C}', false)), // Korean hangul
    (0x335568, ('\u{8406}', false)), // East Asian ideograph
    (0x226F7B, ('\u{7CE8}', false)), // East Asian ideograph (variant of 4C6F7B which maps to 7CE8)
    (0x6F5B7C, ('\u{D339}', false)), // Korean hangul
    (0x275A26, ('\u{8D40}', false)), // East Asian ideograph
    (0x6F5679, ('\u{C790}', false)), // Korean hangul
    (0x2E2F7C, ('\u{634D}', false)), // East Asian ideograph
    (0x215A27, ('\u{8CC3}', false)), // East Asian ideograph
    (0x213C5E, ('\u{5E63}', false)), // East Asian ideograph
    (0x276121, ('\u{998A}', false)), // East Asian ideograph
    (0x2D6F7D, ('\u{8123}', false)), // East Asian ideograph
    (0x235A28, ('\u{9D07}', false)), // East Asian ideograph
    (0x2D4472, ('\u{68CA}', false)), // East Asian ideograph
    (0x215A29, ('\u{8CD3}', false)), // East Asian ideograph
    (0x215A2A, ('\u{8CD1}', false)), // East Asian ideograph
    (0x217D2E, ('\u{5B0D}', false)), // East Asian ideograph
    (0x215A2B, ('\u{8CD2}', false)), // East Asian ideograph
    (0x395063, ('\u{9939}', false)), // East Asian ideograph
    (0x6F567A, ('\u{C791}', false)), // Korean hangul
    (0x275A2C, ('\u{8D54}', false)), // East Asian ideograph
    (0x23445D, ('\u{92F9}', false)), // East Asian ideograph
    (0x215A2D, ('\u{8CE6}', false)), // East Asian ideograph
    (0x295C57, ('\u{9E6B}', false)), // East Asian ideograph
    (0x215A2F, ('\u{8CE3}', false)), // East Asian ideograph
    (0x213076, ('\u{4ED9}', false)), // East Asian ideograph
    (0x215A30, ('\u{8CE2}', false)), // East Asian ideograph
    (0x4B3C38, ('\u{949C}', false)), // East Asian ideograph
    (0x275A31, ('\u{8D31}', false)), // East Asian ideograph
    (0x276123, ('\u{9992}', false)), // East Asian ideograph
    (0x225A32, ('\u{743B}', false)), // East Asian ideograph
    (0x4B3130, ('\u{4FAB}', false)), // East Asian ideograph
    (0x6F4A70, ('\u{AF43}', false)), // Korean hangul
    (0x215A33, ('\u{8CDC}', false)), // East Asian ideograph
    (0x275A34, ('\u{8D28}', false)), // East Asian ideograph
    (0x215A35, ('\u{8CED}', false)), // East Asian ideograph
    (0x2D4A3B, ('\u{4E89}', false)), // East Asian ideograph
    (0x225A36, ('\u{7444}', false)), // East Asian ideograph
    (0x275A37, ('\u{8D5B}', false)), // East Asian ideograph
    (0x215A38, ('\u{8CFA}', false)), // East Asian ideograph
    (0x215A39, ('\u{8D05}', false)), // East Asian ideograph
    (0x293A40, ('\u{8DF8}', false)), // East Asian ideograph
    (0x23556C, ('\u{9B29}', false)), // East Asian ideograph
    (0x215A3A, ('\u{8CFC}', false)), // East Asian ideograph
    (0x215A3B, ('\u{8D08}', false)), // East Asian ideograph (variant of 4B5A3B which maps to 8D08)
    (0x6F5352, ('\u{C1F0}', false)), // Korean hangul
    (0x215A3C, ('\u{8D0B}', false)), // East Asian ideograph
    (0x215A3D, ('\u{8D0A}', false)), // East Asian ideograph
    (0x215A3E, ('\u{8D0F}', false)), // East Asian ideograph
    (0x6F5B7D, ('\u{D33B}', false)), // Korean hangul
    (0x215A3F, ('\u{8D0D}', false)), // East Asian ideograph
    (0x6F567E, ('\u{C7A0}', false)), // Korean hangul
    (0x213D40, ('\u{5F1B}', false)), // East Asian ideograph
    (0x215A40, ('\u{8D13}', false)), // East Asian ideograph
    (0x276126, ('\u{990D}', false)), // East Asian ideograph
    (0x215A41, ('\u{8D16}', false)), // East Asian ideograph
    (0x215A42, ('\u{8D1B}', false)), // East Asian ideograph
    (0x295C5B, ('\u{9E6C}', false)), // East Asian ideograph
    (0x225A43, ('\u{7458}', false)), // East Asian ideograph
    (0x235A44, ('\u{9D1D}', false)), // East Asian ideograph
    (0x225A45, ('\u{7442}', false)), // East Asian ideograph
    (0x6F5A46, ('\u{CF71}', false)), // Korean hangul
    (0x2D3D75, ('\u{60EA}', false)), // East Asian ideograph
    (0x2E4174, ('\u{6AA9}', false)), // East Asian ideograph
    (0x225A47, ('\u{744B}', false)), // East Asian ideograph
    (0x215A48, ('\u{8D70}', false)), // East Asian ideograph
    (0x6F5660, ('\u{C737}', false)), // Korean hangul
    (0x337345, ('\u{9F67}', false)), // East Asian ideograph
    (0x6F5A49, ('\u{CF80}', false)), // Korean hangul
    (0x225A4A, ('\u{744A}', false)), // East Asian ideograph
    (0x213C65, ('\u{5E74}', false)), // East Asian ideograph
    (0x6F5A4B, ('\u{CF8C}', false)), // Korean hangul
    (0x6F5938, ('\u{CC71}', false)), // Korean hangul
    (0x2D3D76, ('\u{5FB4}', false)), // East Asian ideograph
    (0x2D5476, ('\u{8318}', false)), // East Asian ideograph
    (0x6F5A4C, ('\u{CF8D}', false)), // Korean hangul
    (0x6F5573, ('\u{C628}', false)), // Korean hangul
    (0x6F5A4D, ('\u{CFA1}', false)), // Korean hangul
    (0x2D5A4E, ('\u{8D82}', false)), // East Asian ideograph
    (0x215A4F, ('\u{8D99}', false)), // East Asian ideograph
    (0x4B3864, ('\u{58C7}', false)), // East Asian ideograph
    (0x275A50, ('\u{8D76}', false)), // East Asian ideograph
    (0x2D392F, ('\u{7287}', false)), // East Asian ideograph
    (0x6F5A51, ('\u{CFE0}', false)), // Korean hangul
    (0x704C2A, ('\u{915E}', false)), // East Asian ideograph
    (0x224E37, ('\u{6FAF}', false)), // East Asian ideograph
    (0x6F5A52, ('\u{CFE1}', false)), // Korean hangul
    (0x6F5C58, ('\u{D515}', false)), // Korean hangul
    (0x215A53, ('\u{8DA8}', false)), // East Asian ideograph
    (0x6F492E, ('\u{AC86}', false)), // Korean hangul
    (0x6F537D, ('\u{C2B4}', false)), // Korean hangul
    (0x6F523F, ('\u{BF40}', false)), // Korean hangul
    (0x225A55, ('\u{7457}', false)), // East Asian ideograph
    (0x225A56, ('\u{7451}', false)), // East Asian ideograph
    (0x6F5069, ('\u{BC0F}', false)), // Korean hangul
    (0x6F5A57, ('\u{CFF5}', false)), // Korean hangul
    (0x293A46, ('\u{8E70}', false)), // East Asian ideograph
    (0x23512F, ('\u{9916}', false)), // East Asian ideograph
    (0x4B3642, ('\u{8BF6}', false)), // East Asian ideograph
    (0x2E4E41, ('\u{7032}', false)), // East Asian ideograph
    (0x215A59, ('\u{8DDB}', false)), // East Asian ideograph
    (0x6F5240, ('\u{BF41}', false)), // Korean hangul
    (0x706131, ('\u{5C9C}', false)), // East Asian ideograph
    (0x4B357B, ('\u{54CC}', false)), // East Asian ideograph
    (0x225A5A, ('\u{745D}', false)), // East Asian ideograph
    (0x225A5B, ('\u{7454}', false)), // East Asian ideograph
    (0x225174, ('\u{7131}', false)), // East Asian ideograph
    (0x235573, ('\u{9B2D}', false)), // East Asian ideograph
    (0x235130, ('\u{9914}', false)), // East Asian ideograph
    (0x33386E, ('\u{576F}', false)), // East Asian ideograph
    (0x6F5A5E, ('\u{D038}', false)), // Korean hangul
    (0x6F5241, ('\u{BF44}', false)), // Korean hangul
    (0x2D5A5F, ('\u{8E5F}', false)), // East Asian ideograph
    (0x6F5949, ('\u{CD09}', false)), // Korean hangul
    (0x225A60, ('\u{746D}', false)), // East Asian ideograph
    (0x277239, ('\u{54D4}', false)), // East Asian ideograph
    (0x225A61, ('\u{7462}', false)), // East Asian ideograph
    (0x235574, ('\u{9B2E}', false)), // East Asian ideograph (not in Unicode)
    (0x29506C, ('\u{996B}', false)), // East Asian ideograph
    (0x6F532F, ('\u{C131}', false)), // Korean hangul
    (0x215A62, ('\u{8DF3}', false)), // East Asian ideograph
    (0x215A63, ('\u{8DFA}', false)), // East Asian ideograph
    (0x6F5242, ('\u{BF48}', false)), // Korean hangul
    (0x27612D, ('\u{51AF}', false)), // East Asian ideograph
    (0x6F5A64, ('\u{D07D}', false)), // Korean hangul
    (0x4C796B, ('\u{815F}', false)), // East Asian ideograph
    (0x235A65, ('\u{9D30}', false)), // East Asian ideograph
    (0x275021, ('\u{7B0B}', false)), // East Asian ideograph
    (0x235132, ('\u{9911}', false)), // East Asian ideograph
    (0x2F3B63, ('\u{5E32}', false)), // East Asian ideograph (not in Unicode)
    (0x2D4A45, ('\u{5C12}', false)), // East Asian ideograph
    (0x275A68, ('\u{8DF5}', false)), // East Asian ideograph
    (0x6F5243, ('\u{BF50}', false)), // Korean hangul
    (0x213C6B, ('\u{5E7E}', false)), // East Asian ideograph
    (0x215A69, ('\u{8E22}', false)), // East Asian ideograph
    (0x225A6A, ('\u{7471}', false)), // East Asian ideograph
    (0x225A6B, ('\u{7468}', false)), // East Asian ideograph
    (0x4B5936, ('\u{8B20}', false)), // East Asian ideograph
    (0x4D5A6C, ('\u{9D46}', false)), // East Asian ideograph
    (0x216035, ('\u{97FB}', false)), // East Asian ideograph
    (0x2D4A46, ('\u{58BB}', false)), // East Asian ideograph
    (0x6F5A6D, ('\u{D0B9}', false)), // Korean hangul
    (0x4B4857, ('\u{6F22}', false)), // East Asian ideograph
    (0x235A70, ('\u{9D5C}', false)), // East Asian ideograph
    (0x224D35, ('\u{6F90}', false)), // East Asian ideograph
    (0x282951, ('\u{5F2A}', false)), // East Asian ideograph
    (0x275A71, ('\u{8E0A}', false)), // East Asian ideograph
    (0x6F5A72, ('\u{D0C4}', false)), // Korean hangul
    (0x6F5245, ('\u{BF55}', false)), // Korean hangul
    (0x276130, ('\u{9A6E}', false)), // East Asian ideograph
    (0x6F4C27, ('\u{B141}', false)), // Korean hangul
    (0x695A73, ('\u{6683}', false)), // East Asian ideograph
    (0x454B7A, ('\u{7523}', false)), // East Asian ideograph
    (0x6F5A74, ('\u{D0C9}', false)), // Korean hangul
    (0x2D377C, ('\u{962C}', false)), // East Asian ideograph
    (0x215A75, ('\u{8E4B}', false)), // East Asian ideograph
    (0x295822, ('\u{9CAE}', false)), // East Asian ideograph
    (0x6F5A76, ('\u{D0D1}', false)), // Korean hangul
    (0x6F5A77, ('\u{D0D3}', false)), // Korean hangul
    (0x6F5246, ('\u{BFB0}', false)), // Korean hangul
    (0x234522, ('\u{9314}', false)), // East Asian ideograph
    (0x225A78, ('\u{7460}', false)), // East Asian ideograph
    (0x225A79, ('\u{7472}', false)), // East Asian ideograph
    (0x225A7A, ('\u{7484}', false)), // East Asian ideograph
    (0x224525, ('\u{6B99}', false)), // East Asian ideograph
    (0x224D37, ('\u{6F8D}', false)), // East Asian ideograph
    (0x225A7B, ('\u{7487}', false)), // East Asian ideograph
    (0x274526, ('\u{6781}', false)), // East Asian ideograph
    (0x6F5A7C, ('\u{D0E0}', false)), // Korean hangul
    (0x6F5247, ('\u{BFC0}', false)), // Korean hangul
    (0x276132, ('\u{9A73}', false)), // East Asian ideograph
    (0x6F5A7D, ('\u{D0E4}', false)), // Korean hangul
    (0x234528, ('\u{92FE}', false)), // East Asian ideograph
    (0x215A7E, ('\u{8E7A}', false)), // East Asian ideograph
    (0x224529, ('\u{6B9B}', false)), // East Asian ideograph
    (0x27452A, ('\u{6768}', false)), // East Asian ideograph
    (0x3F4C3C, ('\u{7582}', false)), // East Asian ideograph
    (0x27452B, ('\u{6862}', false)), // East Asian ideograph
    (0x2E7062, ('\u{7D4F}', false)), // East Asian ideograph
    (0x6F5248, ('\u{BFC5}', false)), // Korean hangul
    (0x276133, ('\u{9A7B}', false)), // East Asian ideograph
    (0x4B3B61, ('\u{5C64}', false)), // East Asian ideograph
    (0x69242B, ('\u{304B}', false)), // Hiragana letter KA
    (0x27452D, ('\u{4E1A}', false)), // East Asian ideograph
    (0x2D3931, ('\u{67F0}', false)), // East Asian ideograph
    (0x2D7E6A, ('\u{51A4}', false)), // East Asian ideograph
    (0x27452F, ('\u{67AB}', false)), // East Asian ideograph
    (0x6F5C5A, ('\u{D53D}', false)), // Korean hangul
    (0x224D39, ('\u{6F92}', false)), // East Asian ideograph
    (0x692434, ('\u{3054}', false)), // Hiragana letter GO
    (0x6F5249, ('\u{BFCC}', false)), // Korean hangul
    (0x234531, ('\u{9341}', false)), // East Asian ideograph
    (0x217C60, ('\u{5ADC}', false)), // East Asian ideograph
    (0x273764, ('\u{5631}', false)), // East Asian ideograph
    (0x234532, ('\u{9319}', false)), // East Asian ideograph
    (0x6F506B, ('\u{BBB4}', false)), // Korean hangul
    (0x4B4534, ('\u{6994}', false)), // East Asian ideograph (variant of 214534)
    (0x224D3A, ('\u{6F89}', false)), // East Asian ideograph
    (0x234535, ('\u{934C}', false)), // East Asian ideograph
    (0x6F524A, ('\u{BFCD}', false)), // Korean hangul
    (0x224536, ('\u{6BA2}', false)), // East Asian ideograph
    (0x6F4C28, ('\u{B144}', false)), // Korean hangul
    (0x21382F, ('\u{577C}', false)), // East Asian ideograph
    (0x274537, ('\u{8363}', false)), // East Asian ideograph
    (0x224538, ('\u{6BAA}', false)), // East Asian ideograph
    (0x274539, ('\u{6784}', false)), // East Asian ideograph
    (0x235B6A, ('\u{9DA1}', false)), // East Asian ideograph
    (0x2D453A, ('\u{6760}', false)), // East Asian ideograph
    (0x22453B, ('\u{6BAD}', false)), // East Asian ideograph
    (0x234471, ('\u{9340}', false)), // East Asian ideograph
    (0x692531, ('\u{30B1}', false)), // Katakana letter KE
    (0x22453D, ('\u{6BB0}', false)), // East Asian ideograph
    (0x6F4F66, ('\u{B9C8}', false)), // Korean hangul
    (0x6F5663, ('\u{C740}', false)), // Korean hangul
    (0x274871, ('\u{6D47}', false)), // East Asian ideograph
    (0x224D3C, ('\u{6F8C}', false)), // East Asian ideograph
    (0x22453F, ('\u{6BB3}', false)), // East Asian ideograph
    (0x274540, ('\u{67AA}', false)), // East Asian ideograph
    (0x234541, ('\u{9379}', false)), // East Asian ideograph
    (0x4B3144, ('\u{4F36}', false)), // East Asian ideograph (variant of 213144 which maps to 4F36)
    (0x295C6C, ('\u{9E6A}', false)), // East Asian ideograph
    (0x2D4543, ('\u{6901}', false)), // East Asian ideograph
    (0x224D3D, ('\u{6F62}', false)), // East Asian ideograph (variant of 4C4D3D which maps to 6F62)
    (0x33513C, ('\u{7D4C}', false)), // East Asian ideograph
    (0x214544, ('\u{6A23}', false)), // East Asian ideograph
    (0x2D5036, ('\u{84D1}', false)), // East Asian ideograph
    (0x6F524D, ('\u{BFDC}', false)), // Korean hangul
    (0x234A5E, ('\u{967F}', false)), // East Asian ideograph
    (0x223553, ('\u{6509}', false)), // East Asian ideograph
    (0x4B7577, ('\u{57D3}', false)), // East Asian ideograph
    (0x225E4A, ('\u{75C2}', false)), // East Asian ideograph
    (0x214546, ('\u{6A01}', false)), // East Asian ideograph
    (0x2D3932, ('\u{7AD2}', false)), // East Asian ideograph
    (0x274547, ('\u{6807}', false)), // East Asian ideograph
    (0x234548, ('\u{935C}', false)), // East Asian ideograph
    (0x6F5C5B, ('\u{D540}', false)), // Korean hangul
    (0x216037, ('\u{9801}', false)), // East Asian ideograph
    (0x274549, ('\u{67A2}', false)), // East Asian ideograph
    (0x6F524E, ('\u{BFDD}', false)), // Korean hangul
    (0x27454A, ('\u{697C}', false)), // East Asian ideograph
    (0x213833, ('\u{57AE}', false)), // East Asian ideograph
    (0x4B602D, ('\u{9771}', false)), // East Asian ideograph
    (0x2D5B5D, ('\u{8FA2}', false)), // East Asian ideograph
    (0x2D3944, ('\u{511E}', false)), // East Asian ideograph
    (0x27454C, ('\u{6868}', false)), // East Asian ideograph
    (0x23454D, ('\u{9347}', false)), // East Asian ideograph
    (0x293373, ('\u{8C21}', false)), // East Asian ideograph
    (0x27454E, ('\u{4E50}', false)), // East Asian ideograph
    (0x6F524F, ('\u{BFE1}', false)), // Korean hangul
    (0x27454F, ('\u{679E}', false)), // East Asian ideograph
    (0x2D3F2A, ('\u{5ABF}', false)), // East Asian ideograph
    (0x2D4550, ('\u{58AB}', false)), // East Asian ideograph
    (0x2D5B5E, ('\u{8FA7}', false)), // East Asian ideograph
    (0x234551, ('\u{937A}', false)), // East Asian ideograph
    (0x29582C, ('\u{9CB1}', false)), // East Asian ideograph
    (0x274553, ('\u{692D}', false)), // East Asian ideograph
    (0x27767A, ('\u{57DA}', false)), // East Asian ideograph
    (0x224554, ('\u{6BC8}', false)), // East Asian ideograph
    (0x274555, ('\u{6811}', false)), // East Asian ideograph
    (0x234556, ('\u{937C}', false)), // East Asian ideograph
    (0x293A57, ('\u{8DFB}', false)), // East Asian ideograph
    (0x274557, ('\u{6866}', false)), // East Asian ideograph
    (0x29582D, ('\u{9CB7}', false)), // East Asian ideograph
    (0x235140, ('\u{991C}', false)), // East Asian ideograph
    (0x274558, ('\u{6734}', false)), // East Asian ideograph
    (0x474E5C, ('\u{97DE}', false)), // East Asian ideograph (variant of 234E5C which maps to 97DE)
    (0x274366, ('\u{80E7}', false)), // East Asian ideograph
    (0x6F5251, ('\u{C059}', false)), // Korean hangul
    (0x27613C, ('\u{9A86}', false)), // East Asian ideograph
    (0x2D3261, ('\u{5039}', false)), // East Asian ideograph
    (0x27455B, ('\u{6865}', false)), // East Asian ideograph
    (0x275030, ('\u{8303}', false)), // East Asian ideograph
    (0x23455C, ('\u{9377}', false)), // East Asian ideograph
    (0x6F5172, ('\u{BE4E}', false)), // Korean hangul
    (0x27455D, ('\u{673A}', false)), // East Asian ideograph
    (0x213F50, ('\u{61CA}', false)), // East Asian ideograph
    (0x6F5252, ('\u{C05C}', false)), // Korean hangul
    (0x23455E, ('\u{9358}', false)), // East Asian ideograph
    (0x2D5F63, ('\u{873A}', false)), // East Asian ideograph
    (0x27455F, ('\u{6863}', false)), // East Asian ideograph
    (0x224560, ('\u{6BDA}', false)), // East Asian ideograph
    (0x33327A, ('\u{5150}', false)), // East Asian ideograph
    (0x274561, ('\u{68C0}', false)), // East Asian ideograph
    (0x29582F, ('\u{9CB5}', false)), // East Asian ideograph
    (0x274562, ('\u{6867}', false)), // East Asian ideograph
    (0x3F347D, ('\u{84E1}', false)), // East Asian ideograph
    (0x6F5253, ('\u{C060}', false)), // Korean hangul
    (0x274563, ('\u{67E0}', false)), // East Asian ideograph
    (0x235131, ('\u{9917}', false)), // East Asian ideograph
    (0x234564, ('\u{9376}', false)), // East Asian ideograph
    (0x274565, ('\u{67DC}', false)), // East Asian ideograph
    (0x213230, ('\u{5065}', false)), // East Asian ideograph
    (0x224A4C, ('\u{6E1F}', false)), // East Asian ideograph
    (0x274566, ('\u{69DB}', false)), // East Asian ideograph
    (0x294567, ('\u{9537}', false)), // East Asian ideograph
    (0x6F5254, ('\u{C068}', false)), // Korean hangul
    (0x27613F, ('\u{9A88}', false)), // East Asian ideograph
    (0x6F4C2A, ('\u{B151}', false)), // Korean hangul
    (0x2D4569, ('\u{6AC9}', false)), // East Asian ideograph
    (0x4B314C, ('\u{5F95}', false)), // East Asian ideograph
    (0x6F573F, ('\u{C81C}', false)), // Korean hangul
    (0x23456A, ('\u{9348}', false)), // East Asian ideograph
    (0x27325D, ('\u{4EF7}', false)), // East Asian ideograph
    (0x27456B, ('\u{691F}', false)), // East Asian ideograph
    (0x213F3B, ('\u{616B}', false)), // East Asian ideograph
    (0x295831, ('\u{9CB6}', false)), // East Asian ideograph
    (0x27456C, ('\u{6809}', false)), // East Asian ideograph
    (0x2E4E56, ('\u{9800}', false)), // East Asian ideograph
    (0x6F5255, ('\u{C069}', false)), // Korean hangul
    (0x27456D, ('\u{6A79}', false)), // East Asian ideograph
    (0x276140, ('\u{9A91}', false)), // East Asian ideograph
    (0x23447B, ('\u{933F}', false)), // East Asian ideograph
    (0x27456E, ('\u{680F}', false)), // East Asian ideograph
    (0x27456F, ('\u{6A31}', false)), // East Asian ideograph
    (0x224570, ('\u{6BEA}', false)), // East Asian ideograph
    (0x3F456D, ('\u{8263}', false)), // East Asian ideograph
    (0x235145, ('\u{9927}', false)), // East Asian ideograph
    (0x274571, ('\u{6984}', false)), // East Asian ideograph
    (0x2D4A58, ('\u{7F9D}', false)), // East Asian ideograph
    (0x6F5256, ('\u{C090}', false)), // Korean hangul
    (0x217C6D, ('\u{5AE5}', false)), // East Asian ideograph
    (0x286540, ('\u{7800}', false)), // East Asian ideograph
    (0x23447C, ('\u{933A}', false)), // East Asian ideograph
    (0x234573, ('\u{9360}', false)), // East Asian ideograph
    (0x2D4574, ('\u{5FFB}', false)), // East Asian ideograph
    (0x6F5D37, ('\u{D639}', false)), // Korean hangul
    (0x233021, ('\u{894D}', false)), // East Asian ideograph
    (0x6F5257, ('\u{C091}', false)), // Korean hangul
    (0x234577, ('\u{936E}', false)), // East Asian ideograph
    (0x287022, ('\u{7CC1}', false)), // East Asian ideograph
    (0x274578, ('\u{94A6}', false)), // East Asian ideograph
    (0x6F5844, ('\u{C9F8}', false)), // Korean hangul
    (0x213023, ('\u{4E03}', false)), // East Asian ideograph
    (0x225B2D, ('\u{7486}', false)), // East Asian ideograph
    (0x234579, ('\u{938F}', false)), // East Asian ideograph
    (0x233024, ('\u{895A}', false)), // East Asian ideograph
    (0x293A5E, ('\u{8DF9}', false)), // East Asian ideograph
    (0x23457A, ('\u{93AC}', false)), // East Asian ideograph
    (0x6F5C5D, ('\u{D54C}', false)), // Korean hangul
    (0x233025, ('\u{895E}', false)), // East Asian ideograph
    (0x335147, ('\u{7EEE}', false)), // East Asian ideograph
    (0x23457B, ('\u{9395}', false)), // East Asian ideograph
    (0x213026, ('\u{4E0A}', false)), // East Asian ideograph
    (0x4B4E7B, ('\u{7985}', false)), // East Asian ideograph (variant of 274E7B which maps to 7985)
    (0x6F5258, ('\u{C094}', false)), // Korean hangul
    (0x27457C, ('\u{6B27}', false)), // East Asian ideograph
    (0x295175, ('\u{9993}', false)), // East Asian ideograph
    (0x21383D, ('\u{57DF}', false)), // East Asian ideograph
    (0x2E3028, ('\u{640B}', false)), // East Asian ideograph
    (0x27457E, ('\u{6B24}', false)), // East Asian ideograph
    (0x213029, ('\u{4E10}', false)), // East Asian ideograph
    (0x293A5F, ('\u{8DDE}', false)), // East Asian ideograph
    (0x2D4A5B, ('\u{7282}', false)), // East Asian ideograph
    (0x22302B, ('\u{625A}', false)), // East Asian ideograph
    (0x6F5259, ('\u{C098}', false)), // Korean hangul
    (0x276144, ('\u{817E}', false)), // East Asian ideograph
    (0x21302C, ('\u{4E19}', false)), // East Asian ideograph
    (0x21383E, ('\u{580A}', false)), // East Asian ideograph
    (0x22302D, ('\u{6266}', false)), // East Asian ideograph
    (0x22702E, ('\u{7CF0}', false)), // East Asian ideograph
    (0x6F5664, ('\u{C744}', false)), // Korean hangul
    (0x293A60, ('\u{8E2C}', false)), // East Asian ideograph
    (0x21302F, ('\u{4E18}', false)), // East Asian ideograph
    (0x217030, ('\u{5504}', false)), // East Asian ideograph
    (0x6F525A, ('\u{C0A0}', false)), // Korean hangul
    (0x234469, ('\u{9338}', false)), // East Asian ideograph
    (0x692126, ('\u{30FB}', false)), // Ideographic centered point
    (0x223031, ('\u{6286}', false)), // East Asian ideograph
    (0x21383F, ('\u{5805}', false)), // East Asian ideograph
    (0x273032, ('\u{5E76}', false)), // East Asian ideograph
    (0x6F594A, ('\u{CD0C}', false)), // Korean hangul
    (0x2D5B69, ('\u{5EF5}', false)), // East Asian ideograph
    (0x6F4C55, ('\u{B284}', false)), // Korean hangul
    (0x275039, ('\u{7B03}', false)), // East Asian ideograph
    (0x6F5666, ('\u{C74C}', false)), // Korean hangul
    (0x692139, ('\u{3005}', false)), // Ideographic iteration mark
    (0x69213C, ('\u{30FC}', false)), // Vowel elongation mark for kana
    (0x213035, ('\u{4E32}', false)), // East Asian ideograph
    (0x695130, ('\u{5116}', false)), // East Asian ideograph
    (0x6F525B, ('\u{C0A3}', false)), // Korean hangul
    (0x276146, ('\u{9AA0}', false)), // East Asian ideograph
    (0x217C72, ('\u{5AEA}', false)), // East Asian ideograph
    (0x6F4A77, ('\u{AF64}', false)), // Korean hangul
    (0x23403E, ('\u{9146}', false)), // East Asian ideograph
    (0x2D3263, ('\u{4FAD}', false)), // East Asian ideograph
    (0x4B4F29, ('\u{7A50}', false)), // East Asian ideograph
    (0x692152, ('\u{3008}', false)), // Ideographic less than sign
    (0x27503A, ('\u{7B51}', false)), // East Asian ideograph
    (0x692154, ('\u{300A}', false)), // Ideographic left double angle bracket
    (0x692155, ('\u{300B}', false)), // Ideographic right double angle bracket
    (0x217039, ('\u{54F7}', false)), // East Asian ideograph
    (0x21303A, ('\u{4E43}', false)), // East Asian ideograph
    (0x2E4E5D, ('\u{6DE0}', false)), // East Asian ideograph
    (0x6F525C, ('\u{C0A5}', false)), // Korean hangul
    (0x21303B, ('\u{4E45}', false)), // East Asian ideograph
    (0x213841, ('\u{5806}', false)), // East Asian ideograph
    (0x217830, ('\u{58A3}', false)), // East Asian ideograph
    (0x6F576A, ('\u{C8F1}', false)), // Korean hangul
    (0x233376, ('\u{8B1F}', false)), // East Asian ideograph
    (0x33514C, ('\u{7DD1}', false)), // East Asian ideograph
    (0x213E21, ('\u{5FCD}', false)), // East Asian ideograph
    (0x276148, ('\u{84E6}', false)), // East Asian ideograph
    (0x2D4B71, ('\u{7F3E}', false)), // East Asian ideograph
    (0x223041, ('\u{62A3}', false)), // East Asian ideograph
    (0x213042, ('\u{4E52}', false)), // East Asian ideograph
    (0x277255, ('\u{54D3}', false)), // East Asian ideograph
    (0x213043, ('\u{4E53}', false)), // East Asian ideograph
    (0x227044, ('\u{7D03}', false)), // East Asian ideograph
    (0x234544, ('\u{9386}', false)), // East Asian ideograph
    (0x287045, ('\u{7EA8}', false)), // East Asian ideograph
    (0x227C31, ('\u{82AF}', false)), // East Asian ideograph
    (0x234041, ('\u{9147}', false)), // East Asian ideograph
    (0x2D3954, ('\u{7385}', false)), // East Asian ideograph
    (0x213047, ('\u{4E5D}', false)), // East Asian ideograph
    (0x213049, ('\u{4E5E}', false)), // East Asian ideograph
    (0x6F525F, ('\u{C0AC}', false)), // Korean hangul
    (0x28255A, ('\u{5D5D}', false)), // East Asian ideograph
    (0x273F31, ('\u{60ED}', false)), // East Asian ideograph
    (0x225E5C, ('\u{75CF}', false)), // East Asian ideograph
    (0x29472F, ('\u{94E9}', false)), // East Asian ideograph
    (0x21304B, ('\u{4E73}', false)), // East Asian ideograph
    (0x21304C, ('\u{4E7E}', false)), // East Asian ideograph
    (0x27503E, ('\u{7BD3}', false)), // East Asian ideograph
    (0x6F5667, ('\u{C74D}', false)), // Korean hangul
    (0x27304D, ('\u{4E71}', false)), // East Asian ideograph
    (0x23514F, ('\u{992E}', false)), // East Asian ideograph
    (0x6F5870, ('\u{CBD4}', false)), // Korean hangul
    (0x275B36, ('\u{8F69}', false)), // East Asian ideograph
    (0x6F5260, ('\u{C0AD}', false)), // Korean hangul
    (0x27614B, ('\u{60CA}', false)), // East Asian ideograph
    (0x6F4A78, ('\u{AF65}', false)), // Korean hangul
    (0x227050, ('\u{7D18}', false)), // East Asian ideograph
    (0x227051, ('\u{7D1E}', false)), // East Asian ideograph
    (0x277258, ('\u{6076}', false)), // East Asian ideograph (duplicate simplified)
    (0x393052, ('\u{65BC}', false)), // East Asian ideograph
    (0x235150, ('\u{992C}', false)), // East Asian ideograph
    (0x213053, ('\u{4E95}', false)), // East Asian ideograph
    (0x6F5261, ('\u{C0AE}', false)), // Korean hangul
    (0x294040, ('\u{90E6}', false)), // East Asian ideograph
    (0x213055, ('\u{4E92}', false)), // East Asian ideograph
    (0x2E5F6F, ('\u{75B8}', false)), // East Asian ideograph
    (0x27326A, ('\u{4FEA}', false)), // East Asian ideograph
    (0x223057, ('\u{62D1}', false)), // East Asian ideograph
    (0x235151, ('\u{992A}', false)), // East Asian ideograph
    (0x213058, ('\u{4E9E}', false)), // East Asian ideograph
    (0x2D4621, ('\u{61FD}', false)), // East Asian ideograph
    (0x213059, ('\u{4E9B}', false)), // East Asian ideograph
    (0x284333, ('\u{680E}', false)), // East Asian ideograph
    (0x294732, ('\u{94F4}', false)), // East Asian ideograph
    (0x33625E, ('\u{9EAA}', false)), // East Asian ideograph
    (0x22705A, ('\u{7D3D}', false)), // East Asian ideograph
    (0x6F5070, ('\u{BC18}', false)), // Korean hangul
    (0x232223, ('\u{83FD}', false)), // East Asian ideograph
    (0x222224, ('\u{5BF0}', false)), // East Asian ideograph
    (0x232225, ('\u{841E}', false)), // East Asian ideograph
    (0x693C36, ('\u{96EB}', false)), // East Asian ideograph
    (0x232229, ('\u{83C9}', false)), // East Asian ideograph
    (0x23222A, ('\u{83DF}', false)), // East Asian ideograph
    (0x23222C, ('\u{841F}', false)), // East Asian ideograph
    (0x23222E, ('\u{840F}', false)), // East Asian ideograph
    (0x69705D, ('\u{9786}', false)), // East Asian ideograph
    (0x232230, ('\u{8411}', false)), // East Asian ideograph
    (0x222233, ('\u{5C00}', false)), // East Asian ideograph
    (0x222235, ('\u{5C57}', false)), // East Asian ideograph
    (0x232236, ('\u{839A}', false)), // East Asian ideograph
    (0x707438, ('\u{823E}', false)), // East Asian ideograph
    (0x33625F, ('\u{8534}', false)), // East Asian ideograph
    (0x21305F, ('\u{4EA8}', false)), // East Asian ideograph
    (0x22223C, ('\u{5C15}', false)), // East Asian ideograph
    (0x333060, ('\u{4EAF}', false)), // East Asian ideograph
    (0x6F5742, ('\u{C824}', false)), // Korean hangul
    (0x232243, ('\u{83D1}', false)), // East Asian ideograph
    (0x275042, ('\u{7BAB}', false)), // East Asian ideograph
    (0x222246, ('\u{5C22}', false)), // East Asian ideograph
    (0x223061, ('\u{62E4}', false)), // East Asian ideograph
    (0x222248, ('\u{5C25}', false)), // East Asian ideograph
    (0x23224A, ('\u{848E}', false)), // East Asian ideograph
    (0x22224B, ('\u{5C2A}', false)), // East Asian ideograph
    (0x23224C, ('\u{8439}', false)), // East Asian ideograph
    (0x23224D, ('\u{8476}', false)), // East Asian ideograph
    (0x23224E, ('\u{8479}', false)), // East Asian ideograph
    (0x213C6E, ('\u{5E8A}', false)), // East Asian ideograph
    (0x222252, ('\u{5C2F}', false)), // East Asian ideograph
    (0x217C7B, ('\u{5ADA}', false)), // East Asian ideograph
    (0x284335, ('\u{6A7C}', false)), // East Asian ideograph
    (0x4C3B22, ('\u{6860}', false)), // East Asian ideograph
    (0x294734, ('\u{9566}', false)), // East Asian ideograph
    (0x213064, ('\u{4EBA}', false)), // East Asian ideograph
    (0x22225B, ('\u{5C32}', false)), // East Asian ideograph
    (0x23225C, ('\u{8451}', false)), // East Asian ideograph
    (0x23225F, ('\u{847D}', false)), // East Asian ideograph
    (0x232262, ('\u{845A}', false)), // East Asian ideograph
    (0x222263, ('\u{5C3B}', false)), // East Asian ideograph
    (0x222265, ('\u{5C44}', false)), // East Asian ideograph
    (0x232266, ('\u{8459}', false)), // East Asian ideograph
    (0x222267, ('\u{5C49}', false)), // East Asian ideograph
    (0x232269, ('\u{8473}', false)), // East Asian ideograph
    (0x23226E, ('\u{843E}', false)), // East Asian ideograph
    (0x6F5265, ('\u{C0B4}', false)), // Korean hangul
    (0x276150, ('\u{9AA5}', false)), // East Asian ideograph
    (0x232271, ('\u{846D}', false)), // East Asian ideograph
    (0x394735, ('\u{6C3E}', false)), // East Asian ideograph
    (0x223069, ('\u{62B6}', false)), // East Asian ideograph
    (0x232278, ('\u{847A}', false)), // East Asian ideograph
    (0x222279, ('\u{5C59}', false)), // East Asian ideograph
    (0x22227B, ('\u{5C5D}', false)), // East Asian ideograph
    (0x22227C, ('\u{5C5F}', false)), // East Asian ideograph
    (0x22706A, ('\u{7D3F}', false)), // East Asian ideograph
    (0x21306B, ('\u{4ECD}', false)), // East Asian ideograph
    (0x2D306C, ('\u{8B8E}', false)), // East Asian ideograph
    (0x6F5266, ('\u{C0B5}', false)), // Korean hangul
    (0x276151, ('\u{9A8A}', false)), // East Asian ideograph
    (0x284337, ('\u{6987}', false)), // East Asian ideograph
    (0x225E63, ('\u{75E7}', false)), // East Asian ideograph
    (0x33512E, ('\u{7E8D}', false)), // East Asian ideograph
    (0x4B306E, ('\u{4EE4}', false)), // East Asian ideograph (variant of 21306E which maps to 4EE4)
    (0x21306F, ('\u{4ED8}', false)), // East Asian ideograph
    (0x235156, ('\u{992B}', false)), // East Asian ideograph
    (0x454146, ('\u{63DB}', false)), // East Asian ideograph
    (0x213071, ('\u{4ED6}', false)), // East Asian ideograph
    (0x6F5267, ('\u{C0B6}', false)), // Korean hangul
    (0x213072, ('\u{4EDE}', false)), // East Asian ideograph
    (0x6F4E24, ('\u{B544}', false)), // Korean hangul
    (0x6F5A24, ('\u{CEE4}', false)), // Korean hangul
    (0x225254, ('\u{714F}', false)), // East Asian ideograph
    (0x215B21, ('\u{8E76}', false)), // East Asian ideograph
    (0x6F5268, ('\u{C0BC}', false)), // Korean hangul
    (0x276153, ('\u{80AE}', false)), // East Asian ideograph
    (0x213077, ('\u{4EE5}', false)), // East Asian ideograph
    (0x215B22, ('\u{8E7C}', false)), // East Asian ideograph
    (0x21384D, ('\u{5857}', false)), // East Asian ideograph
    (0x333078, ('\u{5F77}', false)), // East Asian ideograph
    (0x225D69, ('\u{756F}', false)), // East Asian ideograph
    (0x335E21, ('\u{9221}', false)), // East Asian ideograph
    (0x213079, ('\u{4F09}', false)), // East Asian ideograph
    (0x6F5B24, ('\u{D0F1}', false)), // Korean hangul
    (0x6F5B25, ('\u{D130}', false)), // Korean hangul
    (0x235158, ('\u{9931}', false)), // East Asian ideograph
    (0x4B3E2A, ('\u{6035}', false)), // East Asian ideograph
    (0x275B26, ('\u{8DB8}', false)), // East Asian ideograph
    (0x6F5269, ('\u{C0BD}', false)), // Korean hangul
    (0x6F4B2C, ('\u{AFD4}', false)), // Korean hangul
    (0x225B27, ('\u{7482}', false)), // East Asian ideograph
    (0x21384E, ('\u{5858}', false)), // East Asian ideograph
    (0x21307D, ('\u{4F0A}', false)), // East Asian ideograph
    (0x215B28, ('\u{8E8A}', false)), // East Asian ideograph
    (0x215B29, ('\u{8E8D}', false)), // East Asian ideograph (variant of 4B5B29 which maps to 8E8D)
    (0x275048, ('\u{5E18}', false)), // East Asian ideograph
    (0x275B2A, ('\u{8E2F}', false)), // East Asian ideograph
    (0x4B6044, ('\u{9818}', false)), // East Asian ideograph
    (0x4D4A6C, ('\u{9667}', false)), // East Asian ideograph
    (0x275B2B, ('\u{8E51}', false)), // East Asian ideograph
    (0x6F526A, ('\u{C0BF}', false)), // Korean hangul
    (0x293B7A, ('\u{8F8F}', false)), // East Asian ideograph
    (0x215A68, ('\u{8E10}', false)), // East Asian ideograph
    (0x23404D, ('\u{9156}', false)), // East Asian ideograph
    (0x215B2D, ('\u{8EAB}', false)), // East Asian ideograph
    (0x235B2E, ('\u{9D8A}', false)), // East Asian ideograph
    (0x3F5F34, ('\u{9699}', false)), // East Asian ideograph (not in Unicode)
    (0x274224, ('\u{6361}', false)), // East Asian ideograph
    (0x212320, ('\u{3000}', false)), // Ideographic space in some implementations
    (0x215B30, ('\u{8EBA}', false)), // East Asian ideograph
    (0x222323, ('\u{5C63}', false)), // East Asian ideograph
    (0x232324, ('\u{8432}', false)), // East Asian ideograph
    (0x225772, ('\u{735E}', false)), // East Asian ideograph
    (0x212328, ('\u{FF08}', false)), // Ideographic left parenthesis
    (0x222329, ('\u{5C67}', false)), // East Asian ideograph
    (0x22232B, ('\u{5C68}', false)), // East Asian ideograph
    (0x23232D, ('\u{842A}', false)), // East Asian ideograph
    (0x23232E, ('\u{8429}', false)), // East Asian ideograph
    (0x222330, ('\u{5C6D}', false)), // East Asian ideograph
    (0x222331, ('\u{5C6E}', false)), // East Asian ideograph
    (0x232332, ('\u{8471}', false)), // East Asian ideograph
    (0x215B33, ('\u{8ECB}', false)), // East Asian ideograph
    (0x232335, ('\u{845F}', false)), // East Asian ideograph
    (0x232336, ('\u{8460}', false)), // East Asian ideograph
    (0x222337, ('\u{5C74}', false)), // East Asian ideograph
    (0x222339, ('\u{5C73}', false)), // East Asian ideograph
    (0x23233A, ('\u{8446}', false)), // East Asian ideograph
    (0x22233B, ('\u{5C77}', false)), // East Asian ideograph
    (0x22233C, ('\u{5C7A}', false)), // East Asian ideograph
    (0x29233D, ('\u{836E}', false)), // East Asian ideograph
    (0x235B35, ('\u{9D87}', false)), // East Asian ideograph
    (0x222340, ('\u{5C7C}', false)), // East Asian ideograph
    (0x6F526C, ('\u{C0C1}', false)), // Korean hangul
    (0x232345, ('\u{844E}', false)), // East Asian ideograph
    (0x222346, ('\u{5C8F}', false)), // East Asian ideograph
    (0x29473C, ('\u{9568}', false)), // East Asian ideograph
    (0x222349, ('\u{5C88}', false)), // East Asian ideograph
    (0x275B37, ('\u{8F6B}', false)), // East Asian ideograph
    (0x22234D, ('\u{5C99}', false)), // East Asian ideograph
    (0x232350, ('\u{84A1}', false)), // East Asian ideograph
    (0x225B38, ('\u{7480}', false)), // East Asian ideograph
    (0x232353, ('\u{849F}', false)), // East Asian ideograph
    (0x222355, ('\u{5CA6}', false)), // East Asian ideograph
    (0x232356, ('\u{84BA}', false)), // East Asian ideograph
    (0x222357, ('\u{5CA0}', false)), // East Asian ideograph
    (0x23515C, ('\u{993B}', false)), // East Asian ideograph
    (0x69255E, ('\u{30DE}', false)), // Katakana letter MA
    (0x22235C, ('\u{5CA2}', false)), // East Asian ideograph
    (0x275B3A, ('\u{8F72}', false)), // East Asian ideograph
    (0x23235E, ('\u{84C1}', false)), // East Asian ideograph
    (0x23235F, ('\u{84BB}', false)), // East Asian ideograph
    (0x222360, ('\u{5CB5}', false)), // East Asian ideograph
    (0x222361, ('\u{5CA7}', false)), // East Asian ideograph
    (0x215B3B, ('\u{8EF8}', false)), // East Asian ideograph
    (0x222366, ('\u{5CA8}', false)), // East Asian ideograph
    (0x222367, ('\u{5CAC}', false)), // East Asian ideograph
    (0x234050, ('\u{9158}', false)), // East Asian ideograph
    (0x225B3C, ('\u{7481}', false)), // East Asian ideograph
    (0x22236B, ('\u{5CA3}', false)), // East Asian ideograph
    (0x22236C, ('\u{5CB6}', false)), // East Asian ideograph
    (0x22236D, ('\u{5CC1}', false)), // East Asian ideograph
    (0x23456E, ('\u{9351}', false)), // East Asian ideograph
    (0x22236F, ('\u{5CAD}', false)), // East Asian ideograph
    (0x232370, ('\u{84B1}', false)), // East Asian ideograph
    (0x232371, ('\u{849D}', false)), // East Asian ideograph
    (0x232372, ('\u{84D0}', false)), // East Asian ideograph
    (0x224666, ('\u{6C2D}', false)), // East Asian ideograph
    (0x232375, ('\u{8494}', false)), // East Asian ideograph
    (0x222378, ('\u{5CD3}', false)), // East Asian ideograph
    (0x232379, ('\u{84C7}', false)), // East Asian ideograph
    (0x23237A, ('\u{84BD}', false)), // East Asian ideograph
    (0x215B3F, ('\u{8F09}', false)), // East Asian ideograph
    (0x23237C, ('\u{84C2}', false)), // East Asian ideograph
    (0x6F526E, ('\u{C0C8}', false)), // Korean hangul
    (0x282569, ('\u{5D02}', false)), // East Asian ideograph
    (0x225B40, ('\u{7497}', false)), // East Asian ideograph
    (0x29473E, ('\u{94F9}', false)), // East Asian ideograph
    (0x23572E, ('\u{9B93}', false)), // East Asian ideograph
    (0x275B41, ('\u{8F85}', false)), // East Asian ideograph
    (0x215B42, ('\u{8F12}', false)), // East Asian ideograph
    (0x27504D, ('\u{7B79}', false)), // East Asian ideograph
    (0x224D5F, ('\u{6F72}', false)), // East Asian ideograph
    (0x225B43, ('\u{7498}', false)), // East Asian ideograph
    (0x6F5B44, ('\u{D22D}', false)), // Korean hangul
    (0x6F526F, ('\u{C0C9}', false)), // Korean hangul
    (0x2E6C46, ('\u{7BE6}', false)), // East Asian ideograph
    (0x225B45, ('\u{749A}', false)), // East Asian ideograph
    (0x284340, ('\u{67A5}', false)), // East Asian ideograph
    (0x692526, ('\u{30A6}', false)), // Katakana letter U
    (0x275B46, ('\u{8F86}', false)), // East Asian ideograph
    (0x2D573B, ('\u{60F7}', false)), // East Asian ideograph
    (0x215B47, ('\u{8F1F}', false)), // East Asian ideograph
    (0x275B48, ('\u{8F89}', false)), // East Asian ideograph
    (0x275B49, ('\u{8F88}', false)), // East Asian ideograph
    (0x6F5270, ('\u{C0CC}', false)), // Korean hangul
    (0x27615B, ('\u{810F}', false)), // East Asian ideograph
    (0x275B4A, ('\u{8F6E}', false)), // East Asian ideograph
    (0x234A65, ('\u{9688}', false)), // East Asian ideograph
    (0x6F5845, ('\u{C9F9}', false)), // Korean hangul
    (0x215B4B, ('\u{8F1C}', false)), // East Asian ideograph
    (0x33422A, ('\u{62E1}', false)), // East Asian ideograph
    (0x275B4C, ('\u{8F90}', false)), // East Asian ideograph
    (0x225B4D, ('\u{74A4}', false)), // East Asian ideograph
    (0x235160, ('\u{993A}', false)), // East Asian ideograph
    (0x275B4E, ('\u{8F93}', false)), // East Asian ideograph
    (0x6F516B, ('\u{BE1D}', false)), // Korean hangul
    (0x276131, ('\u{9A6F}', false)), // East Asian ideograph
    (0x215B4F, ('\u{8F44}', false)), // East Asian ideograph
    (0x225A2B, ('\u{7424}', false)), // East Asian ideograph
    (0x2D3B40, ('\u{5C06}', false)), // East Asian ideograph (variant of 273B40 which maps to 5C06)
    (0x275B51, ('\u{8F95}', false)), // East Asian ideograph
    (0x28544F, ('\u{70EC}', false)), // East Asian ideograph
    (0x224D62, ('\u{6F57}', false)), // East Asian ideograph
    (0x29337A, ('\u{8BCC}', false)), // East Asian ideograph
    (0x235161, ('\u{9941}', false)), // East Asian ideograph
    (0x275B53, ('\u{8206}', false)), // East Asian ideograph
    (0x215B54, ('\u{8F4D}', false)), // East Asian ideograph
    (0x692533, ('\u{30B3}', false)), // Katakana letter KO
    (0x4B316A, ('\u{723C}', false)), // East Asian ideograph
    (0x215B55, ('\u{8F49}', false)), // East Asian ideograph
    (0x2D3F31, ('\u{6159}', false)), // East Asian ideograph
    (0x6F5665, ('\u{C74A}', false)), // Korean hangul
    (0x225B56, ('\u{748D}', false)), // East Asian ideograph
    (0x224667, ('\u{6C30}', false)), // East Asian ideograph
    (0x215B57, ('\u{8F4E}', false)), // East Asian ideograph
    (0x275B58, ('\u{8F70}', false)), // East Asian ideograph
    (0x213C71, ('\u{5E96}', false)), // East Asian ideograph
    (0x275B59, ('\u{8F94}', false)), // East Asian ideograph
    (0x234056, ('\u{9164}', false)), // East Asian ideograph
    (0x225A2D, ('\u{742D}', false)), // East Asian ideograph
    (0x6F4C56, ('\u{B289}', false)), // Korean hangul
    (0x232421, ('\u{8495}', false)), // East Asian ideograph
    (0x692422, ('\u{3042}', false)), // Hiragana letter A
    (0x692423, ('\u{3043}', false)), // Hiragana letter small I
    (0x692424, ('\u{3044}', false)), // Hiragana letter I
    (0x692425, ('\u{3045}', false)), // Hiragana letter small U
    (0x222426, ('\u{5CE0}', false)), // East Asian ideograph
    (0x232427, ('\u{84AF}', false)), // East Asian ideograph
    (0x222428, ('\u{5CD2}', false)), // East Asian ideograph
    (0x232429, ('\u{84AD}', false)), // East Asian ideograph
    (0x69242A, ('\u{304A}', false)), // Hiragana letter O
    (0x22242B, ('\u{5CCB}', false)), // East Asian ideograph
    (0x69242C, ('\u{304C}', false)), // Hiragana letter GA
    (0x69242D, ('\u{304D}', false)), // Hiragana letter KI
    (0x69242E, ('\u{304E}', false)), // Hiragana letter GI
    (0x215B5D, ('\u{8FA3}', false)), // East Asian ideograph
    (0x222430, ('\u{5CC7}', false)), // East Asian ideograph
    (0x222431, ('\u{5CDC}', false)), // East Asian ideograph
    (0x232432, ('\u{84A8}', false)), // East Asian ideograph
    (0x232433, ('\u{84D6}', false)), // East Asian ideograph
    (0x222434, ('\u{5D00}', false)), // East Asian ideograph
    (0x215B5E, ('\u{8FA8}', false)), // East Asian ideograph
    (0x213859, ('\u{5875}', false)), // East Asian ideograph
    (0x692437, ('\u{3057}', false)), // Hiragana letter SI
    (0x692438, ('\u{3058}', false)), // Hiragana letter ZI
    (0x692439, ('\u{3059}', false)), // Hiragana letter SU
    (0x23243A, ('\u{8493}', false)), // East Asian ideograph
    (0x22243B, ('\u{5CFF}', false)), // East Asian ideograph
    (0x22243C, ('\u{5CEB}', false)), // East Asian ideograph
    (0x69243D, ('\u{305D}', false)), // Hiragana letter SO
    (0x69243E, ('\u{305E}', false)), // Hiragana letter ZO
    (0x23243F, ('\u{84CF}', false)), // East Asian ideograph
    (0x692440, ('\u{3060}', false)), // Hiragana letter DA
    (0x232441, ('\u{84CA}', false)), // East Asian ideograph
    (0x692442, ('\u{3062}', false)), // Hiragana letter DI
    (0x692443, ('\u{3063}', false)), // Hiragana letter small TU
    (0x692444, ('\u{3064}', false)), // Hiragana letter TU
    (0x692445, ('\u{3065}', false)), // Hiragana letter DU
    (0x232446, ('\u{8506}', false)), // East Asian ideograph
    (0x232447, ('\u{850B}', false)), // East Asian ideograph
    (0x692448, ('\u{3068}', false)), // Hiragana letter TO
    (0x222449, ('\u{5D1E}', false)), // East Asian ideograph
    (0x22244A, ('\u{5D12}', false)), // East Asian ideograph
    (0x69244B, ('\u{306B}', false)), // Hiragana letter NI
    (0x69244C, ('\u{306C}', false)), // Hiragana letter NU
    (0x23244D, ('\u{8500}', false)), // East Asian ideograph
    (0x69244E, ('\u{306E}', false)), // Hiragana letter NO
    (0x69244F, ('\u{306F}', false)), // Hiragana letter HA
    (0x222450, ('\u{5D1A}', false)), // East Asian ideograph
    (0x692451, ('\u{3071}', false)), // Hiragana letter PA
    (0x222452, ('\u{5D0C}', false)), // East Asian ideograph
    (0x222453, ('\u{5D20}', false)), // East Asian ideograph
    (0x222454, ('\u{5D21}', false)), // East Asian ideograph
    (0x692455, ('\u{3075}', false)), // Hiragana letter HU
    (0x692456, ('\u{3076}', false)), // Hiragana letter BU
    (0x222457, ('\u{5D27}', false)), // East Asian ideograph
    (0x222458, ('\u{5D0D}', false)), // East Asian ideograph
    (0x232459, ('\u{851F}', false)), // East Asian ideograph
    (0x22245A, ('\u{5D26}', false)), // East Asian ideograph
    (0x69245B, ('\u{307B}', false)), // Hiragana letter HO
    (0x23245C, ('\u{853B}', false)), // East Asian ideograph
    (0x22245D, ('\u{5D2E}', false)), // East Asian ideograph
    (0x69245E, ('\u{307E}', false)), // Hiragana letter MA
    (0x23245F, ('\u{84EA}', false)), // East Asian ideograph
    (0x692460, ('\u{3080}', false)), // Hiragana letter MU
    (0x692461, ('\u{3081}', false)), // Hiragana letter ME
    (0x692462, ('\u{3082}', false)), // Hiragana letter MO
    (0x692463, ('\u{3083}', false)), // Hiragana letter small YA
    (0x692464, ('\u{3084}', false)), // Hiragana letter YA
    (0x235B66, ('\u{9DA4}', false)), // East Asian ideograph
    (0x232466, ('\u{84F4}', false)), // East Asian ideograph
    (0x692467, ('\u{3087}', false)), // Hiragana letter small YO
    (0x692468, ('\u{3088}', false)), // Hiragana letter YO
    (0x222469, ('\u{5D24}', false)), // East Asian ideograph
    (0x23246A, ('\u{850C}', false)), // East Asian ideograph
    (0x215B67, ('\u{8FC5}', false)), // East Asian ideograph
    (0x69246C, ('\u{308C}', false)), // Hiragana letter RE
    (0x69246D, ('\u{308D}', false)), // Hiragana letter RO
    (0x69246E, ('\u{308E}', false)), // Hiragana letter small WA
    (0x69246F, ('\u{308F}', false)), // Hiragana letter WA
    (0x692470, ('\u{3090}', false)), // Hiragana letter WI
    (0x222471, ('\u{5D36}', false)), // East Asian ideograph
    (0x222472, ('\u{5D3E}', false)), // East Asian ideograph
    (0x692473, ('\u{3093}', false)), // Hiragana letter N
    (0x222474, ('\u{5D4B}', false)), // East Asian ideograph
    (0x232475, ('\u{8515}', false)), // East Asian ideograph
    (0x222476, ('\u{5D57}', false)), // East Asian ideograph
    (0x222477, ('\u{5D34}', false)), // East Asian ideograph
    (0x6F2478, ('\u{3155}', false)), // Korean hangul
    (0x335E2F, ('\u{5257}', false)), // East Asian ideograph
    (0x23247A, ('\u{84FC}', false)), // East Asian ideograph
    (0x6F247B, ('\u{3158}', false)), // Korean hangul
    (0x23247C, ('\u{84EB}', false)), // East Asian ideograph
    (0x23247D, ('\u{84FD}', false)), // East Asian ideograph
    (0x6F247E, ('\u{315B}', false)), // Korean hangul
    (0x235B6B, ('\u{9D9A}', false)), // East Asian ideograph
    (0x235166, ('\u{993C}', false)), // East Asian ideograph
    (0x225B6C, ('\u{74A5}', false)), // East Asian ideograph
    (0x6F5277, ('\u{C0DD}', false)), // Korean hangul
    (0x6F4C31, ('\u{B179}', false)), // Korean hangul
    (0x215B6D, ('\u{8FF0}', false)), // East Asian ideograph (variant of 275B6D which maps to 8FF0)
    (0x21385C, ('\u{5885}', false)), // East Asian ideograph
    (0x69252E, ('\u{30AE}', false)), // Katakana letter GI
    (0x225B6E, ('\u{74A8}', false)), // East Asian ideograph
    (0x235E30, ('\u{9EC1}', false)), // East Asian ideograph
    (0x295921, ('\u{9CD9}', false)), // East Asian ideograph
    (0x6F5B6F, ('\u{D310}', false)), // Korean hangul
    (0x295854, ('\u{9CC4}', false)), // East Asian ideograph
    (0x215B70, ('\u{8FEA}', false)), // East Asian ideograph
    (0x705B71, ('\u{57B4}', false)), // East Asian ideograph
    (0x6F5278, ('\u{C0E4}', false)), // Korean hangul
    (0x276163, ('\u{677E}', false)), // East Asian ideograph (duplicate simplified)
    (0x6F4E35, ('\u{B5B0}', false)), // Korean hangul
    (0x69252F, ('\u{30AF}', false)), // Katakana letter KU
    (0x234B66, ('\u{96D8}', false)), // East Asian ideograph
    (0x235B74, ('\u{9DB1}', false)), // East Asian ideograph
    (0x6F4B7C, ('\u{B123}', false)), // Korean hangul
    (0x4B6053, ('\u{985E}', false)), // East Asian ideograph
    (0x224926, ('\u{6D6F}', false)), // East Asian ideograph
    (0x235B76, ('\u{9DB6}', false)), // East Asian ideograph
    (0x234621, ('\u{93B5}', false)), // East Asian ideograph
    (0x276164, ('\u{80E1}', false)), // East Asian ideograph (duplicate simplified)
    (0x235B77, ('\u{9DBC}', false)), // East Asian ideograph
    (0x336275, ('\u{76BC}', false)), // East Asian ideograph
    (0x234623, ('\u{9388}', false)), // East Asian ideograph
    (0x295B79, ('\u{9E5A}', false)), // East Asian ideograph
    (0x51496B, ('\u{852B}', false)), // East Asian ideograph
    (0x215B7A, ('\u{9003}', false)), // East Asian ideograph
    (0x234625, ('\u{93B9}', false)), // East Asian ideograph
    (0x215B7B, ('\u{8FFD}', false)), // East Asian ideograph
    (0x6F5173, ('\u{BE54}', false)), // Korean hangul
    (0x6F527A, ('\u{C0E8}', false)), // Korean hangul
    (0x276165, ('\u{987B}', false)), // East Asian ideograph (duplicate simplified)
    (0x235B7C, ('\u{9DBA}', false)), // East Asian ideograph
    (0x21385F, ('\u{5880}', false)), // East Asian ideograph
    (0x234627, ('\u{93A1}', false)), // East Asian ideograph
    (0x215635, ('\u{85E5}', false)), // East Asian ideograph
    (0x275B7D, ('\u{8FD9}', false)), // East Asian ideograph
    (0x234628, ('\u{93B0}', false)), // East Asian ideograph
    (0x235B7E, ('\u{9DCF}', false)), // East Asian ideograph
    (0x234629, ('\u{93A3}', false)), // East Asian ideograph
    (0x21462A, ('\u{6B77}', false)), // East Asian ideograph
    (0x224928, ('\u{6D61}', false)), // East Asian ideograph
    (0x23462B, ('\u{939B}', false)), // East Asian ideograph
    (0x276166, ('\u{9B13}', false)), // East Asian ideograph
    (0x4B3F50, ('\u{61CA}', false)), // East Asian ideograph (variant of 213F50)
    (0x22462C, ('\u{6BF3}', false)), // East Asian ideograph
    (0x692532, ('\u{30B2}', false)), // Katakana letter GE
    (0x23462D, ('\u{9398}', false)), // East Asian ideograph
    (0x213238, ('\u{5075}', false)), // East Asian ideograph
    (0x282626, ('\u{5CC4}', false)), // East Asian ideograph
    (0x4B462E, ('\u{6B81}', false)), // East Asian ideograph
    (0x2F5F45, ('\u{86A1}', false)), // East Asian ideograph
    (0x2D4B22, ('\u{736A}', false)), // East Asian ideograph
    (0x29576E, ('\u{9CA7}', false)), // East Asian ideograph
    (0x692521, ('\u{30A1}', false)), // Katakana letter small A
    (0x692522, ('\u{30A2}', false)), // Katakana letter A
    (0x276167, ('\u{6597}', false)), // East Asian ideograph
    (0x232524, ('\u{851E}', false)), // East Asian ideograph
    (0x222525, ('\u{5D3F}', false)), // East Asian ideograph
    (0x222526, ('\u{5D52}', false)), // East Asian ideograph
    (0x222527, ('\u{5D3D}', false)), // East Asian ideograph
    (0x222528, ('\u{5D4E}', false)), // East Asian ideograph
    (0x692529, ('\u{30A9}', false)), // Katakana letter small O
    (0x23252A, ('\u{8518}', false)), // East Asian ideograph
    (0x69252B, ('\u{30AB}', false)), // Katakana letter KA
    (0x22252C, ('\u{5D59}', false)), // East Asian ideograph
    (0x23252D, ('\u{8526}', false)), // East Asian ideograph
    (0x23252E, ('\u{8507}', false)), // East Asian ideograph (variant of 2F252E which maps to 8507)
    (0x22252F, ('\u{5D32}', false)), // East Asian ideograph
    (0x692530, ('\u{30B0}', false)), // Katakana letter GU
    (0x222531, ('\u{5D42}', false)), // East Asian ideograph
    (0x4C2532, ('\u{5D5B}', false)), // East Asian ideograph
    (0x224633, ('\u{6BF8}', false)), // East Asian ideograph
    (0x232534, ('\u{84F0}', false)), // East Asian ideograph
    (0x232535, ('\u{84EF}', false)), // East Asian ideograph
    (0x232536, ('\u{8556}', false)), // East Asian ideograph
    (0x692537, ('\u{30B7}', false)), // Katakana letter SI
    (0x692538, ('\u{30B8}', false)), // Katakana letter ZI
    (0x222539, ('\u{5D6F}', false)), // East Asian ideograph
    (0x22253A, ('\u{5D6B}', false)), // East Asian ideograph
    (0x696136, ('\u{753C}', false)), // East Asian ideograph
    (0x69253C, ('\u{30BC}', false)), // Katakana letter ZE
    (0x69253D, ('\u{30BD}', false)), // Katakana letter SO
    (0x69253E, ('\u{30BE}', false)), // Katakana letter ZO
    (0x274635, ('\u{6B87}', false)), // East Asian ideograph
    (0x692540, ('\u{30C0}', false)), // Katakana letter DA
    (0x276168, ('\u{95F9}', false)), // East Asian ideograph
    (0x692542, ('\u{30C2}', false)), // Katakana letter DI
    (0x692543, ('\u{30C3}', false)), // Katakana letter small TU
    (0x222544, ('\u{5D4A}', false)), // East Asian ideograph
    (0x225E7A, ('\u{7602}', false)), // East Asian ideograph
    (0x232546, ('\u{8541}', false)), // East Asian ideograph
    (0x692534, ('\u{30B4}', false)), // Katakana letter GO
    (0x692548, ('\u{30C8}', false)), // Katakana letter TO
    (0x222549, ('\u{5D6C}', false)), // East Asian ideograph
    (0x22254A, ('\u{5D62}', false)), // East Asian ideograph
    (0x23254B, ('\u{8558}', false)), // East Asian ideograph
    (0x69254C, ('\u{30CC}', false)), // Katakana letter NU
    (0x22254D, ('\u{5D82}', false)), // East Asian ideograph
    (0x23254E, ('\u{8561}', false)), // East Asian ideograph
    (0x23254F, ('\u{8540}', false)), // East Asian ideograph
    (0x222550, ('\u{5D79}', false)), // East Asian ideograph
    (0x224638, ('\u{6BF9}', false)), // East Asian ideograph
    (0x692552, ('\u{30D2}', false)), // Katakana letter HI
    (0x692553, ('\u{30D3}', false)), // Katakana letter BI
    (0x692554, ('\u{30D4}', false)), // Katakana letter PI
    (0x287231, ('\u{7F03}', false)), // East Asian ideograph
    (0x692556, ('\u{30D6}', false)), // Katakana letter BU
    (0x33516D, ('\u{6374}', false)), // East Asian ideograph
    (0x4D222A, ('\u{83B5}', false)), // East Asian ideograph
    (0x692559, ('\u{30D9}', false)), // Katakana letter BE
    (0x22255A, ('\u{5D81}', false)), // East Asian ideograph
    (0x69255B, ('\u{30DB}', false)), // Katakana letter HO
    (0x23255C, ('\u{8564}', false)), // East Asian ideograph
    (0x23255D, ('\u{855E}', false)), // East Asian ideograph
    (0x23255E, ('\u{8573}', false)), // East Asian ideograph
    (0x23255F, ('\u{8551}', false)), // East Asian ideograph
    (0x222560, ('\u{5D7E}', false)), // East Asian ideograph
    (0x692561, ('\u{30E1}', false)), // Katakana letter ME
    (0x692562, ('\u{30E2}', false)), // Katakana letter MO
    (0x27463B, ('\u{6740}', false)), // East Asian ideograph
    (0x232564, ('\u{8562}', false)), // East Asian ideograph
    (0x292535, ('\u{82C1}', false)), // East Asian ideograph
    (0x222566, ('\u{5D92}', false)), // East Asian ideograph
    (0x292567, ('\u{836C}', false)), // East Asian ideograph
    (0x222568, ('\u{5D99}', false)), // East Asian ideograph
    (0x27463C, ('\u{58F3}', false)), // East Asian ideograph
    (0x22256A, ('\u{5DA2}', false)), // East Asian ideograph
    (0x23256B, ('\u{8563}', false)), // East Asian ideograph
    (0x23256C, ('\u{848D}', false)), // East Asian ideograph
    (0x23256D, ('\u{8542}', false)), // East Asian ideograph
    (0x69256E, ('\u{30EE}', false)), // Katakana letter small WA
    (0x23463D, ('\u{93A4}', false)), // East Asian ideograph
    (0x692570, ('\u{30F0}', false)), // Katakana letter WI
    (0x232571, ('\u{854E}', false)), // East Asian ideograph
    (0x692572, ('\u{30F2}', false)), // Katakana letter WO
    (0x222573, ('\u{5DA1}', false)), // East Asian ideograph
    (0x232574, ('\u{8555}', false)), // East Asian ideograph
    (0x222575, ('\u{5D93}', false)), // East Asian ideograph
    (0x232576, ('\u{855D}', false)), // East Asian ideograph
    (0x222577, ('\u{5DA0}', false)), // East Asian ideograph
    (0x4B3E40, ('\u{6046}', false)), // East Asian ideograph
    (0x22257B, ('\u{5D94}', false)), // East Asian ideograph
    (0x27616A, ('\u{90C1}', false)), // East Asian ideograph
    (0x22257E, ('\u{5DAC}', false)), // East Asian ideograph
    (0x6F4E3C, ('\u{B5C0}', false)), // Korean hangul
    (0x284350, ('\u{68C2}', false)), // East Asian ideograph
    (0x234640, ('\u{93BC}', false)), // East Asian ideograph
    (0x692536, ('\u{30B6}', false)), // Katakana letter ZA
    (0x513421, ('\u{91D6}', false)), // East Asian ideograph
    (0x224642, ('\u{6BFF}', false)), // East Asian ideograph
    (0x3F5F49, ('\u{7431}', false)), // East Asian ideograph
    (0x29585C, ('\u{9CC7}', false)), // East Asian ideograph
    (0x6F5C65, ('\u{D55C}', false)), // Korean hangul
    (0x224644, ('\u{6C06}', false)), // East Asian ideograph
    (0x276134, ('\u{9A7C}', false)), // East Asian ideograph
    (0x28656A, ('\u{789B}', false)), // East Asian ideograph
    (0x213865, ('\u{58C5}', false)), // East Asian ideograph
    (0x294750, ('\u{950E}', false)), // East Asian ideograph
    (0x4B3178, ('\u{5029}', false)), // East Asian ideograph (variant of 213178 which maps to 5029)
    (0x234647, ('\u{93A6}', false)), // East Asian ideograph
    (0x29337D, ('\u{8C27}', false)), // East Asian ideograph
    (0x224648, ('\u{6C04}', false)), // East Asian ideograph
    (0x22492E, ('\u{6D8A}', false)), // East Asian ideograph
    (0x453755, ('\u{56AE}', false)), // East Asian ideograph
    (0x2D516A, ('\u{7DB3}', false)), // East Asian ideograph
    (0x6F4E3E, ('\u{B5CC}', false)), // Korean hangul
    (0x23464A, ('\u{93AA}', false)), // East Asian ideograph
    (0x294751, ('\u{950F}', false)), // East Asian ideograph
    (0x33627D, ('\u{6589}', false)), // East Asian ideograph
    (0x213423, ('\u{5291}', false)), // East Asian ideograph
    (0x235060, ('\u{98E3}', false)), // East Asian ideograph
    (0x333C21, ('\u{7895}', false)), // East Asian ideograph
    (0x6F5748, ('\u{C84C}', false)), // Korean hangul
    (0x295153, ('\u{9967}', false)), // East Asian ideograph
    (0x22464C, ('\u{6C08}', false)), // East Asian ideograph
    (0x23464D, ('\u{939E}', false)), // East Asian ideograph
    (0x213C74, ('\u{5EA0}', false)), // East Asian ideograph
    (0x6F4E3F, ('\u{B5CF}', false)), // Korean hangul
    (0x23464F, ('\u{9397}', false)), // East Asian ideograph
    (0x692539, ('\u{30B9}', false)), // Katakana letter SU
    (0x27727A, ('\u{54D5}', false)), // East Asian ideograph
    (0x234651, ('\u{93BB}', false)), // East Asian ideograph
    (0x295825, ('\u{9CBA}', false)), // East Asian ideograph
    (0x224D73, ('\u{6FB6}', false)), // East Asian ideograph
    (0x224652, ('\u{6C0D}', false)), // East Asian ideograph
    (0x234653, ('\u{93F1}', false)), // East Asian ideograph
    (0x225851, ('\u{739E}', false)), // East Asian ideograph
    (0x6F4E40, ('\u{B5D1}', false)), // Korean hangul
    (0x213868, ('\u{58D5}', false)), // East Asian ideograph
    (0x69253A, ('\u{30BA}', false)), // Katakana letter ZU
    (0x274655, ('\u{6C14}', false)), // East Asian ideograph
    (0x234656, ('\u{93DE}', false)), // East Asian ideograph
    (0x234657, ('\u{93EE}', false)), // East Asian ideograph
    (0x234D30, ('\u{976E}', false)), // East Asian ideograph
    (0x274658, ('\u{6C22}', false)), // East Asian ideograph
    (0x27384A, ('\u{573A}', false)), // East Asian ideograph
    (0x223244, ('\u{63E5}', false)), // East Asian ideograph
    (0x224659, ('\u{6C15}', false)), // East Asian ideograph
    (0x51563F, ('\u{8616}', false)), // East Asian ideograph
    (0x23465A, ('\u{93C7}', false)), // East Asian ideograph
    (0x23465B, ('\u{93F2}', false)), // East Asian ideograph
    (0x232625, ('\u{8580}', false)), // East Asian ideograph
    (0x222626, ('\u{5DA7}', false)), // East Asian ideograph
    (0x232628, ('\u{858F}', false)), // East Asian ideograph
    (0x22465C, ('\u{6C1A}', false)), // East Asian ideograph
    (0x22262A, ('\u{5DB0}', false)), // East Asian ideograph
    (0x23262D, ('\u{8579}', false)), // East Asian ideograph
    (0x22262E, ('\u{5DB4}', false)), // East Asian ideograph
    (0x23465D, ('\u{93D4}', false)), // East Asian ideograph
    (0x222630, ('\u{5DB6}', false)), // East Asian ideograph
    (0x232632, ('\u{857F}', false)), // East Asian ideograph
    (0x232633, ('\u{8577}', false)), // East Asian ideograph
    (0x232634, ('\u{8578}', false)), // East Asian ideograph
    (0x22465E, ('\u{6C1D}', false)), // East Asian ideograph
    (0x222636, ('\u{5DB7}', false)), // East Asian ideograph
    (0x6F4C37, ('\u{B18B}', false)), // Korean hangul
    (0x2D6132, ('\u{99EE}', false)), // East Asian ideograph
    (0x23263D, ('\u{85A4}', false)), // East Asian ideograph
    (0x22263E, ('\u{5DC3}', false)), // East Asian ideograph
    (0x224660, ('\u{6C20}', false)), // East Asian ideograph
    (0x232642, ('\u{857A}', false)), // East Asian ideograph
    (0x222644, ('\u{5DC7}', false)), // East Asian ideograph
    (0x232645, ('\u{8557}', false)), // East Asian ideograph
    (0x222646, ('\u{5DC9}', false)), // East Asian ideograph
    (0x222647, ('\u{5DCB}', false)), // East Asian ideograph
    (0x232649, ('\u{85A8}', false)), // East Asian ideograph
    (0x213D4F, ('\u{5F59}', false)), // East Asian ideograph
    (0x234D32, ('\u{9778}', false)), // East Asian ideograph
    (0x224662, ('\u{6C21}', false)), // East Asian ideograph
    (0x22264E, ('\u{5DD8}', false)), // East Asian ideograph
    (0x232650, ('\u{8599}', false)), // East Asian ideograph
    (0x232651, ('\u{858A}', false)), // East Asian ideograph
    (0x222652, ('\u{5DDC}', false)), // East Asian ideograph
    (0x234663, ('\u{93CA}', false)), // East Asian ideograph
    (0x232654, ('\u{8590}', false)), // East Asian ideograph
    (0x232656, ('\u{8585}', false)), // East Asian ideograph
    (0x232657, ('\u{8588}', false)), // East Asian ideograph
    (0x225A40, ('\u{7447}', false)), // East Asian ideograph
    (0x224664, ('\u{6C2A}', false)), // East Asian ideograph
    (0x23265A, ('\u{85B8}', false)), // East Asian ideograph
    (0x6F5749, ('\u{C870}', false)), // Korean hangul
    (0x23265D, ('\u{85C1}', false)), // East Asian ideograph
    (0x334665, ('\u{6C61}', false)), // East Asian ideograph
    (0x232661, ('\u{85BA}', false)), // East Asian ideograph
    (0x222662, ('\u{5E00}', false)), // East Asian ideograph
    (0x222664, ('\u{51E7}', false)), // East Asian ideograph
    (0x234666, ('\u{93E8}', false)), // East Asian ideograph
    (0x224934, ('\u{6D79}', false)), // East Asian ideograph
    (0x232668, ('\u{85CE}', false)), // East Asian ideograph
    (0x23266A, ('\u{85C2}', false)), // East Asian ideograph
    (0x23266B, ('\u{85B7}', false)), // East Asian ideograph
    (0x23266C, ('\u{85B9}', false)), // East Asian ideograph
    (0x23266E, ('\u{85B3}', false)), // East Asian ideograph
    (0x23266F, ('\u{85BD}', false)), // East Asian ideograph
    (0x232670, ('\u{85C4}', false)), // East Asian ideograph
    (0x224668, ('\u{6C2C}', false)), // East Asian ideograph
    (0x222672, ('\u{5E14}', false)), // East Asian ideograph
    (0x222673, ('\u{5E17}', false)), // East Asian ideograph
    (0x232675, ('\u{85BE}', false)), // East Asian ideograph
    (0x222676, ('\u{5E19}', false)), // East Asian ideograph
    (0x224669, ('\u{6C31}', false)), // East Asian ideograph (not in Unicode)
    (0x222678, ('\u{5E1F}', false)), // East Asian ideograph
    (0x22267A, ('\u{5E23}', false)), // East Asian ideograph
    (0x22267B, ('\u{5E21}', false)), // East Asian ideograph
    (0x23267E, ('\u{85B6}', false)), // East Asian ideograph
    (0x295421, ('\u{9AA3}', false)), // East Asian ideograph
    (0x2D4647, ('\u{6BD8}', false)), // East Asian ideograph
    (0x284359, ('\u{6989}', false)), // East Asian ideograph
    (0x2D466D, ('\u{51B3}', false)), // East Asian ideograph
    (0x294758, ('\u{9561}', false)), // East Asian ideograph
    (0x69253F, ('\u{30BF}', false)), // Katakana letter TA
    (0x227C5B, ('\u{82D0}', false)), // East Asian ideograph
    (0x28723C, ('\u{7F08}', false)), // East Asian ideograph
    (0x224670, ('\u{6C3B}', false)), // East Asian ideograph
    (0x295422, ('\u{9A81}', false)), // East Asian ideograph
    (0x234D35, ('\u{9773}', false)), // East Asian ideograph
    (0x276174, ('\u{9C7C}', false)), // East Asian ideograph
    (0x234672, ('\u{93DA}', false)), // East Asian ideograph
    (0x234673, ('\u{93D0}', false)), // East Asian ideograph
    (0x335E42, ('\u{9452}', false)), // East Asian ideograph
    (0x2D353C, ('\u{6B62}', false)), // East Asian ideograph
    (0x234674, ('\u{93EF}', false)), // East Asian ideograph
    (0x6F4E37, ('\u{B5B3}', false)), // Korean hangul
    (0x4B4676, ('\u{6C89}', false)), // East Asian ideograph
    (0x213121, ('\u{4F11}', false)), // East Asian ideograph
    (0x276136, ('\u{9A77}', false)), // East Asian ideograph
    (0x21386F, ('\u{58E2}', false)), // East Asian ideograph
    (0x223C6E, ('\u{68B2}', false)), // East Asian ideograph
    (0x6F2472, ('\u{314F}', false)), // Korean hangul
    (0x224678, ('\u{6C46}', false)), // East Asian ideograph
    (0x6F5078, ('\u{BC29}', false)), // Korean hangul
    (0x28723E, ('\u{7F0C}', false)), // East Asian ideograph
    (0x29364E, ('\u{8D33}', false)), // East Asian ideograph
    (0x22467A, ('\u{6C52}', false)), // East Asian ideograph
    (0x213125, ('\u{4F01}', false)), // East Asian ideograph
    (0x234D37, ('\u{9783}', false)), // East Asian ideograph
    (0x215F69, ('\u{9739}', false)), // East Asian ideograph
    (0x276176, ('\u{9C81}', false)), // East Asian ideograph
    (0x6F4E48, ('\u{B69D}', false)), // Korean hangul
    (0x23467C, ('\u{93CC}', false)), // East Asian ideograph
    (0x6F574A, ('\u{C871}', false)), // Korean hangul
    (0x224D7C, ('\u{6FC6}', false)), // East Asian ideograph
    (0x23517B, ('\u{9954}', false)), // East Asian ideograph
    (0x21312A, ('\u{4F4F}', false)), // East Asian ideograph
    (0x234D38, ('\u{977A}', false)), // East Asian ideograph
    (0x213C76, ('\u{5EAB}', false)), // East Asian ideograph
    (0x21312B, ('\u{4F4D}', false)), // East Asian ideograph
    (0x6F4E49, ('\u{B6A4}', false)), // Korean hangul
    (0x213871, ('\u{58E9}', false)), // East Asian ideograph
    (0x21312C, ('\u{4F34}', false)), // East Asian ideograph
    (0x6F594C, ('\u{CD18}', false)), // Korean hangul
    (0x21342E, ('\u{52C3}', false)), // East Asian ideograph
    (0x21312D, ('\u{4F47}', false)), // East Asian ideograph
    (0x2D5758, ('\u{890E}', false)), // East Asian ideograph
    (0x21312F, ('\u{4F3A}', false)), // East Asian ideograph
    (0x275B3F, ('\u{8F7D}', false)), // East Asian ideograph
    (0x6F4F3D, ('\u{B86D}', false)), // Korean hangul
    (0x28704A, ('\u{7EBE}', false)), // East Asian ideograph
    (0x222722, ('\u{5E22}', false)), // East Asian ideograph
    (0x286577, ('\u{789C}', false)), // East Asian ideograph
    (0x222724, ('\u{5E28}', false)), // East Asian ideograph
    (0x213872, ('\u{58EB}', false)), // East Asian ideograph
    (0x232728, ('\u{85F7}', false)), // East Asian ideograph
    (0x6F5424, ('\u{C2DD}', false)), // Korean hangul
    (0x23272C, ('\u{85E6}', false)), // East Asian ideograph
    (0x223132, ('\u{6360}', false)), // East Asian ideograph
    (0x23272E, ('\u{85D4}', false)), // East Asian ideograph
    (0x232731, ('\u{85ED}', false)), // East Asian ideograph
    (0x6F5D42, ('\u{D65C}', false)), // Korean hangul
    (0x222735, ('\u{5E44}', false)), // East Asian ideograph
    (0x222736, ('\u{5E43}', false)), // East Asian ideograph
    (0x222739, ('\u{5E42}', false)), // East Asian ideograph
    (0x22273F, ('\u{5E4E}', false)), // East Asian ideograph
    (0x6F4E4B, ('\u{B6AC}', false)), // Korean hangul
    (0x232743, ('\u{85DF}', false)), // East Asian ideograph
    (0x232745, ('\u{85D8}', false)), // East Asian ideograph
    (0x692545, ('\u{30C5}', false)), // Katakana letter DU
    (0x222747, ('\u{5E58}', false)), // East Asian ideograph
    (0x222748, ('\u{5E48}', false)), // East Asian ideograph
    (0x513B52, ('\u{6C3D}', false)), // East Asian ideograph
    (0x213137, ('\u{4F3D}', false)), // East Asian ideograph
    (0x23274C, ('\u{85DC}', false)), // East Asian ideograph
    (0x23274E, ('\u{85F5}', false)), // East Asian ideograph
    (0x273138, ('\u{5E03}', false)), // East Asian ideograph
    (0x232752, ('\u{8622}', false)), // East Asian ideograph
    (0x232754, ('\u{8610}', false)), // East Asian ideograph
    (0x285029, ('\u{6EDF}', false)), // East Asian ideograph
    (0x232757, ('\u{85FC}', false)), // East Asian ideograph
    (0x222758, ('\u{5E61}', false)), // East Asian ideograph
    (0x23275B, ('\u{85FF}', false)), // East Asian ideograph
    (0x23313A, ('\u{89D6}', false)), // East Asian ideograph
    (0x23275E, ('\u{85FE}', false)), // East Asian ideograph
    (0x22275F, ('\u{5E6C}', false)), // East Asian ideograph
    (0x222760, ('\u{5E6A}', false)), // East Asian ideograph
    (0x222763, ('\u{5E6E}', false)), // East Asian ideograph
    (0x222764, ('\u{5E6D}', false)), // East Asian ideograph
    (0x222765, ('\u{5E70}', false)), // East Asian ideograph
    (0x232768, ('\u{8604}', false)), // East Asian ideograph
    (0x27313C, ('\u{5360}', false)), // East Asian ideograph
    (0x227C6E, ('\u{8314}', false)), // East Asian ideograph
    (0x22276D, ('\u{5E75}', false)), // East Asian ideograph
    (0x232771, ('\u{8605}', false)), // East Asian ideograph
    (0x216757, ('\u{50A3}', false)), // East Asian ideograph
    (0x232775, ('\u{862B}', false)), // East Asian ideograph
    (0x213D51, ('\u{5F62}', false)), // East Asian ideograph
    (0x222777, ('\u{5E80}', false)), // East Asian ideograph
    (0x21313F, ('\u{4F5C}', false)), // East Asian ideograph
    (0x22277E, ('\u{5E8B}', false)), // East Asian ideograph
    (0x275D38, ('\u{91CA}', false)), // East Asian ideograph
    (0x294760, ('\u{9563}', false)), // East Asian ideograph
    (0x213432, ('\u{52D2}', false)), // East Asian ideograph
    (0x33572E, ('\u{880E}', false)), // East Asian ideograph
    (0x2D3543, ('\u{4EDD}', false)), // East Asian ideograph
    (0x27506F, ('\u{7EA0}', false)), // East Asian ideograph
    (0x215B27, ('\u{8E85}', false)), // East Asian ideograph
    (0x213142, ('\u{4F4E}', false)), // East Asian ideograph
    (0x217D40, ('\u{5B19}', false)), // East Asian ideograph
    (0x213143, ('\u{4F5D}', false)), // East Asian ideograph
    (0x213C77, ('\u{5EA7}', false)), // East Asian ideograph
    (0x213144, ('\u{4F36}', false)), // East Asian ideograph
    (0x6F4B5C, ('\u{B0AF}', false)), // Korean hangul
    (0x6F4E4E, ('\u{B6F4}', false)), // Korean hangul
    (0x213876, ('\u{58FA}', false)), // East Asian ideograph
    (0x223145, ('\u{6335}', false)), // East Asian ideograph
    (0x706067, ('\u{567B}', false)), // East Asian ideograph
    (0x223832, ('\u{665F}', false)), // East Asian ideograph
    (0x295828, ('\u{9CB4}', false)), // East Asian ideograph
    (0x233147, ('\u{89E1}', false)), // East Asian ideograph
    (0x29586E, ('\u{9CA5}', false)), // East Asian ideograph
    (0x213148, ('\u{4F8D}', false)), // East Asian ideograph
    (0x275B40, ('\u{8F7E}', false)), // East Asian ideograph
    (0x6F4E4F, ('\u{B6F8}', false)), // Korean hangul
    (0x275736, ('\u{8747}', false)), // East Asian ideograph
    (0x21314A, ('\u{4F7F}', false)), // East Asian ideograph
    (0x692549, ('\u{30C9}', false)), // Katakana letter DO
    (0x2F5476, ('\u{9AE1}', false)), // East Asian ideograph
    (0x21314B, ('\u{4F9B}', false)), // East Asian ideograph
    (0x275071, ('\u{7EA3}', false)), // East Asian ideograph
    (0x21314C, ('\u{4F86}', false)), // East Asian ideograph
    (0x2D3730, ('\u{751E}', false)), // East Asian ideograph
    (0x21314D, ('\u{4F6C}', false)), // East Asian ideograph
    (0x6F4E50, ('\u{B700}', false)), // Korean hangul
    (0x21314F, ('\u{4F96}', false)), // East Asian ideograph
    (0x213435, ('\u{52DE}', false)), // East Asian ideograph
    (0x233C33, ('\u{8F47}', false)), // East Asian ideograph
    (0x213151, ('\u{4F83}', false)), // East Asian ideograph
    (0x287247, ('\u{7F11}', false)), // East Asian ideograph
    (0x697152, ('\u{99F2}', false)), // East Asian ideograph
    (0x453768, ('\u{5EFB}', false)), // East Asian ideograph
    (0x213153, ('\u{4F88}', false)), // East Asian ideograph
    (0x276138, ('\u{9A79}', false)), // East Asian ideograph
    (0x4B3F51, ('\u{61D1}', false)), // East Asian ideograph
    (0x6F4E51, ('\u{B701}', false)), // Korean hangul
    (0x213154, ('\u{4F69}', false)), // East Asian ideograph
    (0x69254B, ('\u{30CB}', false)), // Katakana letter NI
    (0x213436, ('\u{52DB}', false)), // East Asian ideograph
    (0x6F5826, ('\u{C998}', false)), // Korean hangul
    (0x275073, ('\u{7EAB}', false)), // East Asian ideograph
    (0x354156, ('\u{91BE}', false)), // East Asian ideograph
    (0x295871, ('\u{9CCE}', false)), // East Asian ideograph
    (0x287248, ('\u{7F0F}', false)), // East Asian ideograph
    (0x4B606F, ('\u{991D}', false)), // East Asian ideograph
    (0x233158, ('\u{89F1}', false)), // East Asian ideograph
    (0x294531, ('\u{9528}', false)), // East Asian ideograph
    (0x6F4E52, ('\u{B728}', false)), // Korean hangul
    (0x284366, ('\u{6924}', false)), // East Asian ideograph
    (0x217159, ('\u{55D0}', false)), // East Asian ideograph
    (0x225A4F, ('\u{7452}', false)), // East Asian ideograph
    (0x21315A, ('\u{4FAF}', false)), // East Asian ideograph
    (0x232822, ('\u{8627}', false)), // East Asian ideograph
    (0x21715B, ('\u{55CD}', false)), // East Asian ideograph
    (0x274C31, ('\u{7544}', false)), // East Asian ideograph
    (0x232826, ('\u{8629}', false)), // East Asian ideograph
    (0x23315C, ('\u{89F3}', false)), // East Asian ideograph
    (0x224943, ('\u{6D94}', false)), // East Asian ideograph
    (0x213A61, ('\u{5B7A}', false)), // East Asian ideograph
    (0x21315D, ('\u{4FE0}', false)), // East Asian ideograph
    (0x6F4B5D, ('\u{B0B1}', false)), // Korean hangul
    (0x232832, ('\u{8637}', false)), // East Asian ideograph
    (0x395230, ('\u{5BD8}', false)), // East Asian ideograph
    (0x222835, ('\u{5EA5}', false)), // East Asian ideograph
    (0x222836, ('\u{5EAF}', false)), // East Asian ideograph
    (0x213438, ('\u{52E2}', false)), // East Asian ideograph
    (0x232838, ('\u{8636}', false)), // East Asian ideograph
    (0x21315F, ('\u{4FB6}', false)), // East Asian ideograph
    (0x23283E, ('\u{863C}', false)), // East Asian ideograph
    (0x23283F, ('\u{8640}', false)), // East Asian ideograph
    (0x232840, ('\u{863A}', false)), // East Asian ideograph
    (0x233160, ('\u{89F6}', false)), // East Asian ideograph
    (0x222842, ('\u{5EB9}', false)), // East Asian ideograph
    (0x39365A, ('\u{8AE0}', false)), // East Asian ideograph
    (0x227161, ('\u{7DA3}', false)), // East Asian ideograph
    (0x22284B, ('\u{5EB3}', false)), // East Asian ideograph
    (0x22284C, ('\u{5EC4}', false)), // East Asian ideograph
    (0x217162, ('\u{55DD}', false)), // East Asian ideograph
    (0x6F4E54, ('\u{B72C}', false)), // Korean hangul
    (0x275D3F, ('\u{9488}', false)), // East Asian ideograph
    (0x294767, ('\u{94E7}', false)), // East Asian ideograph
    (0x69254E, ('\u{30CE}', false)), // Katakana letter NO
    (0x222855, ('\u{5ECB}', false)), // East Asian ideograph
    (0x222857, ('\u{5ECD}', false)), // East Asian ideograph
    (0x213164, ('\u{4FDF}', false)), // East Asian ideograph
    (0x22285A, ('\u{5ED2}', false)), // East Asian ideograph
    (0x22285B, ('\u{5ED1}', false)), // East Asian ideograph
    (0x22285C, ('\u{5ED5}', false)), // East Asian ideograph
    (0x23285E, ('\u{8659}', false)), // East Asian ideograph
    (0x22285F, ('\u{5ED4}', false)), // East Asian ideograph
    (0x222860, ('\u{5ED9}', false)), // East Asian ideograph
    (0x222861, ('\u{5ECE}', false)), // East Asian ideograph
    (0x21232D, ('\u{FF0D}', false)), // Ideographic hyphen minus
    (0x232866, ('\u{8661}', false)), // East Asian ideograph
    (0x4C2867, ('\u{5EDB}', false)), // East Asian ideograph
    (0x222868, ('\u{5EE1}', false)), // East Asian ideograph
    (0x232869, ('\u{8662}', false)), // East Asian ideograph
    (0x23286A, ('\u{8663}', false)), // East Asian ideograph
    (0x287167, ('\u{7EEF}', false)), // East Asian ideograph
    (0x22286D, ('\u{5EE7}', false)), // East Asian ideograph
    (0x232871, ('\u{8669}', false)), // East Asian ideograph
    (0x69254F, ('\u{30CF}', false)), // Katakana letter HA
    (0x2D5B7A, ('\u{8FEF}', false)), // East Asian ideograph
    (0x217169, ('\u{55E9}', false)), // East Asian ideograph
    (0x232878, ('\u{866C}', false)), // East Asian ideograph
    (0x23287B, ('\u{8672}', false)), // East Asian ideograph
    (0x22287C, ('\u{5EED}', false)), // East Asian ideograph
    (0x21316A, ('\u{4FCE}', false)), // East Asian ideograph
    (0x23287E, ('\u{867B}', false)), // East Asian ideograph
    (0x274C34, ('\u{5F02}', false)), // East Asian ideograph
    (0x23462A, ('\u{93B7}', false)), // East Asian ideograph
    (0x21316B, ('\u{4FD7}', false)), // East Asian ideograph
    (0x23316C, ('\u{8A06}', false)), // East Asian ideograph
    (0x276139, ('\u{9A78}', false)), // East Asian ideograph
    (0x6F4E56, ('\u{B730}', false)), // Korean hangul
    (0x294769, ('\u{9564}', false)), // East Asian ideograph
    (0x6F507B, ('\u{BC31}', false)), // Korean hangul
    (0x21316E, ('\u{500D}', false)), // East Asian ideograph
    (0x4B5861, ('\u{4F89}', false)), // East Asian ideograph
    (0x21716F, ('\u{55CF}', false)), // East Asian ideograph
    (0x213170, ('\u{5026}', false)), // East Asian ideograph
    (0x4B3E5B, ('\u{60C5}', false)), // East Asian ideograph
    (0x213171, ('\u{500C}', false)), // East Asian ideograph
    (0x6F4E57, ('\u{B738}', false)), // Korean hangul
    (0x223172, ('\u{639E}', false)), // East Asian ideograph
    (0x692551, ('\u{30D1}', false)), // Katakana letter PA
    (0x21343C, ('\u{52F5}', false)), // East Asian ideograph
    (0x273173, ('\u{4EEC}', false)), // East Asian ideograph
    (0x4C4F24, ('\u{6F46}', false)), // East Asian ideograph
    (0x235B7A, ('\u{9DC1}', false)), // East Asian ideograph
    (0x287174, ('\u{7EF2}', false)), // East Asian ideograph
    (0x274C36, ('\u{753B}', false)), // East Asian ideograph
    (0x39365E, ('\u{559E}', false)), // East Asian ideograph
    (0x6F5B21, ('\u{D0ED}', false)), // Korean hangul
    (0x4B5C32, ('\u{9038}', false)), // East Asian ideograph
    (0x6F4B5E, ('\u{B0B3}', false)), // Korean hangul
    (0x6F4E58, ('\u{B739}', false)), // Korean hangul
    (0x2D3177, ('\u{5E78}', false)), // East Asian ideograph
    (0x21343D, ('\u{52F8}', false)), // East Asian ideograph
    (0x217178, ('\u{55C1}', false)), // East Asian ideograph
    (0x6F5C23, ('\u{D37C}', false)), // Korean hangul
    (0x273179, ('\u{4FE9}', false)), // East Asian ideograph
    (0x6F5C24, ('\u{D37D}', false)), // Korean hangul
    (0x29365F, ('\u{8D47}', false)), // East Asian ideograph
    (0x6F5B22, ('\u{D0EF}', false)), // Korean hangul
    (0x6F5C25, ('\u{D380}', false)), // Korean hangul
    (0x21317B, ('\u{5012}', false)), // East Asian ideograph
    (0x6F5C26, ('\u{D384}', false)), // Korean hangul
    (0x6F4E59, ('\u{B73B}', false)), // Korean hangul
    (0x4B317C, ('\u{5024}', false)), // East Asian ideograph
    (0x235C27, ('\u{9DC3}', false)), // East Asian ideograph
    (0x22317D, ('\u{63AB}', false)), // East Asian ideograph
    (0x215C28, ('\u{901A}', false)), // East Asian ideograph
    (0x4C4F26, ('\u{6EDD}', false)), // East Asian ideograph
    (0x69717E, ('\u{9AF7}', false)), // East Asian ideograph
    (0x215C29, ('\u{9020}', false)), // East Asian ideograph
    (0x216764, ('\u{5095}', false)), // East Asian ideograph
    (0x6F5C2A, ('\u{D390}', false)), // Korean hangul
    (0x6F5C2B, ('\u{D391}', false)), // Korean hangul
    (0x6F4E5A, ('\u{B744}', false)), // Korean hangul
    (0x6F5C2C, ('\u{D398}', false)), // Korean hangul
    (0x695C2D, ('\u{6928}', false)), // East Asian ideograph
    (0x4B372F, ('\u{5C1C}', false)), // East Asian ideograph
    (0x274C39, ('\u{5F53}', false)), // East Asian ideograph
    (0x287251, ('\u{7F1F}', false)), // East Asian ideograph
    (0x6F5C6B, ('\u{D56C}', false)), // Korean hangul
    (0x215C2F, ('\u{902E}', false)), // East Asian ideograph
    (0x27613A, ('\u{9A7D}', false)), // East Asian ideograph
    (0x225C30, ('\u{74C8}', false)), // East Asian ideograph
    (0x6F4E5B, ('\u{B748}', false)), // Korean hangul
    (0x222923, ('\u{5EF4}', false)), // East Asian ideograph
    (0x232925, ('\u{867A}', false)), // East Asian ideograph
    (0x232926, ('\u{8673}', false)), // East Asian ideograph
    (0x225C31, ('\u{74C5}', false)), // East Asian ideograph
    (0x29432B, ('\u{94C6}', false)), // East Asian ideograph
    (0x6F5828, ('\u{C99D}', false)), // Korean hangul
    (0x215C32, ('\u{FA25}', false)), // East Asian ideograph
    (0x23292E, ('\u{8696}', false)), // East Asian ideograph
    (0x27507D, ('\u{7EAF}', false)), // East Asian ideograph
    (0x217671, ('\u{5827}', false)), // East Asian ideograph
    (0x29333B, ('\u{8C00}', false)), // East Asian ideograph
    (0x215C33, ('\u{9032}', false)), // East Asian ideograph
    (0x222935, ('\u{5F07}', false)), // East Asian ideograph
    (0x232936, ('\u{8691}', false)), // East Asian ideograph
    (0x232937, ('\u{869C}', false)), // East Asian ideograph
    (0x235C34, ('\u{9DAC}', false)), // East Asian ideograph
    (0x22293A, ('\u{5F0B}', false)), // East Asian ideograph
    (0x23293C, ('\u{868D}', false)), // East Asian ideograph
    (0x23293D, ('\u{868B}', false)), // East Asian ideograph
    (0x6F5C35, ('\u{D3B5}', false)), // Korean hangul
    (0x232940, ('\u{86A6}', false)), // East Asian ideograph
    (0x232942, ('\u{869D}', false)), // East Asian ideograph
    (0x39476F, ('\u{51C0}', false)), // East Asian ideograph
    (0x235C36, ('\u{9DB2}', false)), // East Asian ideograph
    (0x232946, ('\u{86A0}', false)), // East Asian ideograph
    (0x2D3F3A, ('\u{6185}', false)), // East Asian ideograph
    (0x232948, ('\u{86A7}', false)), // East Asian ideograph
    (0x22294A, ('\u{5F28}', false)), // East Asian ideograph
    (0x22294B, ('\u{5F22}', false)), // East Asian ideograph
    (0x22294C, ('\u{5F23}', false)), // East Asian ideograph
    (0x22294D, ('\u{5F24}', false)), // East Asian ideograph
    (0x235B7B, ('\u{9DB8}', false)), // East Asian ideograph
    (0x225C38, ('\u{74D6}', false)), // East Asian ideograph
    (0x222952, ('\u{5F30}', false)), // East Asian ideograph
    (0x6F5A27, ('\u{CEEC}', false)), // Korean hangul
    (0x215C39, ('\u{9054}', false)), // East Asian ideograph
    (0x232958, ('\u{86BA}', false)), // East Asian ideograph
    (0x232959, ('\u{86B0}', false)), // East Asian ideograph
    (0x22295C, ('\u{5F40}', false)), // East Asian ideograph
    (0x215C3A, ('\u{9055}', false)), // East Asian ideograph
    (0x6F4E5D, ('\u{B764}', false)), // Korean hangul
    (0x22295F, ('\u{5F44}', false)), // East Asian ideograph
    (0x232960, ('\u{86B3}', false)), // East Asian ideograph
    (0x232962, ('\u{86C9}', false)), // East Asian ideograph
    (0x215C3B, ('\u{903C}', false)), // East Asian ideograph
    (0x232967, ('\u{86D8}', false)), // East Asian ideograph
    (0x222968, ('\u{5F50}', false)), // East Asian ideograph
    (0x215C3C, ('\u{9047}', false)), // East Asian ideograph
    (0x22296A, ('\u{5F56}', false)), // East Asian ideograph
    (0x22296C, ('\u{5F58}', false)), // East Asian ideograph
    (0x2D3E60, ('\u{6075}', false)), // East Asian ideograph
    (0x23296E, ('\u{86E3}', false)), // East Asian ideograph
    (0x225C3D, ('\u{74D8}', false)), // East Asian ideograph
    (0x222970, ('\u{5F60}', false)), // East Asian ideograph
    (0x232971, ('\u{86EC}', false)), // East Asian ideograph
    (0x222972, ('\u{5F63}', false)), // East Asian ideograph
    (0x222973, ('\u{809C}', false)), // East Asian ideograph
    (0x222974, ('\u{5F67}', false)), // East Asian ideograph
    (0x215C3E, ('\u{904E}', false)), // East Asian ideograph
    (0x232977, ('\u{86D0}', false)), // East Asian ideograph
    (0x222978, ('\u{5F72}', false)), // East Asian ideograph
    (0x222979, ('\u{5F73}', false)), // East Asian ideograph
    (0x23297A, ('\u{86D1}', false)), // East Asian ideograph
    (0x2D5C3F, ('\u{5FA7}', false)), // East Asian ideograph
    (0x22297C, ('\u{5F74}', false)), // East Asian ideograph
    (0x23297E, ('\u{86DE}', false)), // East Asian ideograph
    (0x225C40, ('\u{74DA}', false)), // East Asian ideograph
    (0x213443, ('\u{5308}', false)), // East Asian ideograph
    (0x215C41, ('\u{9041}', false)), // East Asian ideograph
    (0x4C4F2B, ('\u{701E}', false)), // East Asian ideograph
    (0x6F5C42, ('\u{D3FD}', false)), // Korean hangul
    (0x333251, ('\u{5FBA}', false)), // East Asian ideograph
    (0x6F5B28, ('\u{D138}', false)), // Korean hangul
    (0x22494F, ('\u{6D96}', false)), // East Asian ideograph
    (0x695C43, ('\u{6981}', false)), // East Asian ideograph
    (0x4B5C39, ('\u{9039}', false)), // East Asian ideograph
    (0x215C44, ('\u{9060}', false)), // East Asian ideograph
    (0x6F4E5F, ('\u{B770}', false)), // Korean hangul
    (0x273B31, ('\u{5B9E}', false)), // East Asian ideograph
    (0x215C45, ('\u{905C}', false)), // East Asian ideograph
    (0x29432F, ('\u{94F3}', false)), // East Asian ideograph
    (0x696325, ('\u{7907}', false)), // East Asian ideograph
    (0x235C46, ('\u{9DDE}', false)), // East Asian ideograph
    (0x215C47, ('\u{9065}', false)), // East Asian ideograph
    (0x6F5B29, ('\u{D140}', false)), // Korean hangul
    (0x215C48, ('\u{905E}', false)), // East Asian ideograph
    (0x6F4E38, ('\u{B5B4}', false)), // Korean hangul
    (0x27613B, ('\u{9A87}', false)), // East Asian ideograph
    (0x275C49, ('\u{9002}', false)), // East Asian ideograph
    (0x6F4E60, ('\u{B771}', false)), // Korean hangul
    (0x273B32, ('\u{5B81}', false)), // East Asian ideograph
    (0x29255A, ('\u{8487}', false)), // East Asian ideograph
    (0x6F5C4A, ('\u{D479}', false)), // Korean hangul
    (0x225A5D, ('\u{7440}', false)), // East Asian ideograph
    (0x235E5C, ('\u{9EE7}', false)), // East Asian ideograph
    (0x6F5C4B, ('\u{D47C}', false)), // Korean hangul
    (0x2D3556, ('\u{7343}', false)), // East Asian ideograph
    (0x2D532C, ('\u{6BD3}', false)), // East Asian ideograph
    (0x6F5C4C, ('\u{D480}', false)), // Korean hangul
    (0x336321, ('\u{6B6F}', false)), // East Asian ideograph
    (0x6F5B2A, ('\u{D141}', false)), // Korean hangul
    (0x215C4D, ('\u{9075}', false)), // East Asian ideograph
    (0x6F4C3A, ('\u{B193}', false)), // Korean hangul
    (0x6F5C4E, ('\u{D489}', false)), // Korean hangul
    (0x6F4E61, ('\u{B775}', false)), // Korean hangul
    (0x294774, ('\u{9571}', false)), // East Asian ideograph
    (0x215C4F, ('\u{9078}', false)), // East Asian ideograph
    (0x217435, ('\u{571A}', false)), // East Asian ideograph
    (0x215C50, ('\u{9072}', false)), // East Asian ideograph
    (0x4B3231, ('\u{4EEE}', false)), // East Asian ideograph
    (0x275C51, ('\u{8FC1}', false)), // East Asian ideograph
    (0x6F5B2B, ('\u{D143}', false)), // Korean hangul
    (0x275C52, ('\u{8FBD}', false)), // East Asian ideograph
    (0x215C53, ('\u{907A}', false)), // East Asian ideograph
    (0x4D503A, ('\u{98D1}', false)), // East Asian ideograph
    (0x69255C, ('\u{30DC}', false)), // Katakana letter BO
    (0x225C54, ('\u{74E9}', false)), // East Asian ideograph
    (0x217436, ('\u{571B}', false)), // East Asian ideograph
    (0x6F5C55, ('\u{D508}', false)), // Korean hangul
    (0x213A36, ('\u{5ABD}', false)), // East Asian ideograph
    (0x215C56, ('\u{9081}', false)), // East Asian ideograph
    (0x6F5B2C, ('\u{D144}', false)), // Korean hangul
    (0x455F35, ('\u{9668}', false)), // East Asian ideograph (Version J extension)
    (0x215C57, ('\u{9084}', false)), // East Asian ideograph
    (0x6F4F3E, ('\u{B86F}', false)), // Korean hangul
    (0x6F5C33, ('\u{D3AD}', false)), // Korean hangul
    (0x225C58, ('\u{74F1}', false)), // East Asian ideograph
    (0x69255D, ('\u{30DD}', false)), // Katakana letter PO
    (0x6F5C59, ('\u{D53C}', false)), // Korean hangul
    (0x215C5A, ('\u{9087}', false)), // East Asian ideograph
    (0x212A21, ('\u{E8D0}', false)), // EACC component character
    (0x212A22, ('\u{E8D1}', false)), // EACC component character
    (0x215C5B, ('\u{908A}', false)), // East Asian ideograph
    (0x212A24, ('\u{E8D3}', false)), // EACC component character
    (0x232A25, ('\u{870B}', false)), // East Asian ideograph
    (0x212A26, ('\u{E8D5}', false)), // EACC component character
    (0x222A27, ('\u{5F89}', false)), // East Asian ideograph
    (0x212A28, ('\u{E8D6}', false)), // EACC component character
    (0x215C5C, ('\u{9090}', false)), // East Asian ideograph
    (0x212A2A, ('\u{E8D8}', false)), // EACC component character
    (0x222A2B, ('\u{5F94}', false)), // East Asian ideograph
    (0x212A2C, ('\u{E8DA}', false)), // EACC component character
    (0x212A2D, ('\u{E8DB}', false)), // EACC component character
    (0x69727E, ('\u{9D48}', false)), // East Asian ideograph
    (0x215C5D, ('\u{908F}', false)), // East Asian ideograph
    (0x2E284C, ('\u{5ECF}', false)), // East Asian ideograph
    (0x6F4E64, ('\u{B77C}', false)), // Korean hangul
    (0x275D4F, ('\u{94B4}', false)), // East Asian ideograph
    (0x232A33, ('\u{86F8}', false)), // East Asian ideograph
    (0x232A34, ('\u{8706}', false)), // East Asian ideograph
    (0x4B5C5E, ('\u{961D}', false)), // East Asian ideograph
    (0x232A36, ('\u{870E}', false)), // East Asian ideograph
    (0x212A37, ('\u{E8E4}', false)), // EACC component character
    (0x232A38, ('\u{8709}', false)), // East Asian ideograph
    (0x222A39, ('\u{5F9C}', false)), // East Asian ideograph
    (0x232A3A, ('\u{870A}', false)), // East Asian ideograph
    (0x235C5F, ('\u{9DEB}', false)), // East Asian ideograph
    (0x212A3C, ('\u{E8E9}', false)), // EACC component character
    (0x222A3D, ('\u{5F9A}', false)), // East Asian ideograph
    (0x232A3E, ('\u{870D}', false)), // East Asian ideograph
    (0x212A3F, ('\u{E8EC}', false)), // EACC component character
    (0x212A40, ('\u{E8ED}', false)), // EACC component character
    (0x6F5C60, ('\u{D551}', false)), // Korean hangul
    (0x232A42, ('\u{874A}', false)), // East Asian ideograph
    (0x232A43, ('\u{8723}', false)), // East Asian ideograph
    (0x232A44, ('\u{8737}', false)), // East Asian ideograph
    (0x232A45, ('\u{8728}', false)), // East Asian ideograph
    (0x222A46, ('\u{5FAF}', false)), // East Asian ideograph
    (0x225C61, ('\u{74F4}', false)), // East Asian ideograph
    (0x232A49, ('\u{8740}', false)), // East Asian ideograph
    (0x232A4B, ('\u{872E}', false)), // East Asian ideograph
    (0x232A4C, ('\u{873D}', false)), // East Asian ideograph
    (0x232A4E, ('\u{871E}', false)), // East Asian ideograph
    (0x6F4E65, ('\u{B77D}', false)), // Korean hangul
    (0x222A50, ('\u{5FBC}', false)), // East Asian ideograph
    (0x69255F, ('\u{30DF}', false)), // Katakana letter MI
    (0x232A53, ('\u{8743}', false)), // East Asian ideograph
    (0x232A55, ('\u{8744}', false)), // East Asian ideograph
    (0x6F507E, ('\u{BC38}', false)), // Korean hangul
    (0x222A57, ('\u{5FC9}', false)), // East Asian ideograph
    (0x232A59, ('\u{8729}', false)), // East Asian ideograph
    (0x232A5A, ('\u{8739}', false)), // East Asian ideograph
    (0x213241, ('\u{5091}', false)), // East Asian ideograph
    (0x232A5F, ('\u{871A}', false)), // East Asian ideograph
    (0x222A61, ('\u{5FD2}', false)), // East Asian ideograph
    (0x222A63, ('\u{5FD0}', false)), // East Asian ideograph
    (0x232A64, ('\u{8731}', false)), // East Asian ideograph
    (0x232A65, ('\u{8711}', false)), // East Asian ideograph
    (0x232A66, ('\u{8712}', false)), // East Asian ideograph
    (0x222A67, ('\u{5FCE}', false)), // East Asian ideograph
    (0x222A68, ('\u{5FED}', false)), // East Asian ideograph
    (0x6F4C3B, ('\u{B194}', false)), // Korean hangul
    (0x232A6B, ('\u{874F}', false)), // East Asian ideograph
    (0x232A6C, ('\u{8771}', false)), // East Asian ideograph
    (0x232A6D, ('\u{8763}', false)), // East Asian ideograph
    (0x275D51, ('\u{94B8}', false)), // East Asian ideograph
    (0x232A71, ('\u{8764}', false)), // East Asian ideograph
    (0x222A72, ('\u{5FEE}', false)), // East Asian ideograph
    (0x232A73, ('\u{8765}', false)), // East Asian ideograph
    (0x232A74, ('\u{877D}', false)), // East Asian ideograph
    (0x6F5C69, ('\u{D569}', false)), // Korean hangul
    (0x222A78, ('\u{5FE1}', false)), // East Asian ideograph
    (0x232A79, ('\u{8758}', false)), // East Asian ideograph
    (0x222A7B, ('\u{5FE4}', false)), // East Asian ideograph
    (0x215C6A, ('\u{90E1}', false)), // East Asian ideograph
    (0x28725D, ('\u{7F1C}', false)), // East Asian ideograph
    (0x6F5B30, ('\u{D150}', false)), // Korean hangul
    (0x275C6B, ('\u{5369}', false)), // East Asian ideograph
    (0x3F516D, ('\u{6403}', false)), // East Asian ideograph
    (0x235C6C, ('\u{9DE6}', false)), // East Asian ideograph
    (0x6F4E67, ('\u{B784}', false)), // Korean hangul
    (0x275D52, ('\u{94C0}', false)), // East Asian ideograph
    (0x215C6D, ('\u{90F5}', false)), // East Asian ideograph
    (0x33303A, ('\u{8FFA}', false)), // East Asian ideograph
    (0x6F5C6E, ('\u{D574}', false)), // Korean hangul
    (0x6F5C6F, ('\u{D575}', false)), // Korean hangul
    (0x28725E, ('\u{7F19}', false)), // East Asian ideograph
    (0x6F5873, ('\u{CC0C}', false)), // Korean hangul
    (0x705F30, ('\u{7519}', false)), // East Asian ideograph
    (0x215C70, ('\u{9109}', false)), // East Asian ideograph
    (0x215C71, ('\u{9112}', false)), // East Asian ideograph
    (0x6F4E68, ('\u{B78C}', false)), // Korean hangul
    (0x275E68, ('\u{9617}', false)), // East Asian ideograph
    (0x4B5C72, ('\u{9119}', false)), // East Asian ideograph (variant of 215C72 which maps to 9119)
    (0x275153, ('\u{7EAC}', false)), // East Asian ideograph
    (0x215C73, ('\u{912D}', false)), // East Asian ideograph
    (0x235A21, ('\u{9D02}', false)), // East Asian ideograph
    (0x215C74, ('\u{9130}', false)), // East Asian ideograph
    (0x28725F, ('\u{7F1B}', false)), // East Asian ideograph
    (0x6F5B32, ('\u{D15C}', false)), // Korean hangul
    (0x224959, ('\u{6DAB}', false)), // East Asian ideograph
    (0x215C75, ('\u{9127}', false)), // East Asian ideograph
    (0x6F5C76, ('\u{D589}', false)), // Korean hangul
    (0x2D4228, ('\u{5117}', false)), // East Asian ideograph
    (0x215C77, ('\u{9139}', false)), // East Asian ideograph (variant of 4B5C77 which maps to 9139)
    (0x455E60, ('\u{95EB}', false)), // East Asian ideograph (Version J extension)
    (0x6F5C78, ('\u{D5A5}', false)), // Korean hangul
    (0x235A22, ('\u{9D03}', false)), // East Asian ideograph
    (0x2F3D5D, ('\u{900E}', false)), // East Asian ideograph
    (0x6F5C79, ('\u{D5C8}', false)), // Korean hangul
    (0x224724, ('\u{6C5C}', false)), // East Asian ideograph
    (0x6F5C7A, ('\u{D5C9}', false)), // Korean hangul
    (0x27613D, ('\u{9A8B}', false)), // East Asian ideograph
    (0x2E4A6B, ('\u{6EA6}', false)), // East Asian ideograph
    (0x224726, ('\u{6C5B}', false)), // East Asian ideograph
    (0x275D55, ('\u{94C5}', false)), // East Asian ideograph
    (0x292564, ('\u{8489}', false)), // East Asian ideograph
    (0x224727, ('\u{6C4D}', false)), // East Asian ideograph
    (0x225C7D, ('\u{7507}', false)), // East Asian ideograph
    (0x22474D, ('\u{6C93}', false)), // East Asian ideograph
    (0x235A23, ('\u{9CF7}', false)), // East Asian ideograph
    (0x235C7E, ('\u{9DFD}', false)), // East Asian ideograph
    (0x2D4729, ('\u{6D29}', false)), // East Asian ideograph
    (0x213D57, ('\u{5F6D}', false)), // East Asian ideograph
    (0x234D5A, ('\u{979F}', false)), // East Asian ideograph
    (0x6F4C3C, ('\u{B1A8}', false)), // Korean hangul
    (0x294532, ('\u{9531}', false)), // East Asian ideograph
    (0x22472B, ('\u{6C4B}', false)), // East Asian ideograph
    (0x3F4A28, ('\u{9DF0}', false)), // East Asian ideograph
    (0x225A68, ('\u{7474}', false)), // East Asian ideograph
    (0x6F7649, ('\u{E8BB}', false)), // Korean hangul
    (0x6F5751, ('\u{C886}', false)), // Korean hangul
    (0x22472D, ('\u{6C63}', false)), // East Asian ideograph
    (0x6F5B35, ('\u{D160}', false)), // Korean hangul
    (0x4B3D2A, ('\u{5EE3}', false)), // East Asian ideograph
    (0x23472F, ('\u{93A9}', false)), // East Asian ideograph
    (0x28773F, ('\u{804D}', false)), // East Asian ideograph
    (0x232B21, ('\u{8761}', false)), // East Asian ideograph
    (0x692535, ('\u{30B5}', false)), // Katakana letter SA
    (0x222B24, ('\u{5FEA}', false)), // East Asian ideograph
    (0x692566, ('\u{30E6}', false)), // Katakana letter YU
    (0x212B26, ('\u{300D}', false)), // Ideographic right corner bracket
    (0x225A69, ('\u{746E}', false)), // East Asian ideograph
    (0x232B28, ('\u{875F}', false)), // East Asian ideograph
    (0x222B2A, ('\u{6026}', false)), // East Asian ideograph
    (0x222B2C, ('\u{6029}', false)), // East Asian ideograph
    (0x232B2D, ('\u{876F}', false)), // East Asian ideograph
    (0x232B2E, ('\u{875D}', false)), // East Asian ideograph
    (0x232B30, ('\u{876E}', false)), // East Asian ideograph
    (0x222B31, ('\u{6008}', false)), // East Asian ideograph
    (0x212B32, ('\u{FF3D}', false)), // Ideographic right square bracket
    (0x224733, ('\u{6C76}', false)), // East Asian ideograph
    (0x212B34, ('\u{FF0E}', false)), // Ideographic variant full stop
    (0x232B35, ('\u{8753}', false)), // East Asian ideograph
    (0x222B36, ('\u{600A}', false)), // East Asian ideograph
    (0x222B37, ('\u{600C}', false)), // East Asian ideograph
    (0x234D5C, ('\u{979A}', false)), // East Asian ideograph
    (0x214734, ('\u{6CBB}', false)), // East Asian ideograph
    (0x232B3A, ('\u{87A3}', false)), // East Asian ideograph
    (0x4B5E69, ('\u{95A2}', false)), // East Asian ideograph
    (0x222B3C, ('\u{6017}', false)), // East Asian ideograph
    (0x232B3D, ('\u{8793}', false)), // East Asian ideograph
    (0x6F245F, ('\u{3148}', false)), // Korean hangul
    (0x2D4735, ('\u{6C4E}', false)), // East Asian ideograph
    (0x275D58, ('\u{94C3}', false)), // East Asian ideograph
    (0x273B3F, ('\u{4E13}', false)), // East Asian ideograph
    (0x692567, ('\u{30E7}', false)), // Katakana letter small YO
    (0x213452, ('\u{5331}', false)), // East Asian ideograph
    (0x232B45, ('\u{8799}', false)), // East Asian ideograph
    (0x222B46, ('\u{6010}', false)), // East Asian ideograph
    (0x232B48, ('\u{8788}', false)), // East Asian ideograph
    (0x222B4B, ('\u{6039}', false)), // East Asian ideograph
    (0x232B4C, ('\u{8798}', false)), // East Asian ideograph
    (0x222B50, ('\u{6013}', false)), // East Asian ideograph
    (0x224738, ('\u{6C6C}', false)), // East Asian ideograph
    (0x222B53, ('\u{6054}', false)), // East Asian ideograph
    (0x232B54, ('\u{878B}', false)), // East Asian ideograph
    (0x232B55, ('\u{8784}', false)), // East Asian ideograph
    (0x222B57, ('\u{605D}', false)), // East Asian ideograph
    (0x232B58, ('\u{87A9}', false)), // East Asian ideograph
    (0x336B33, ('\u{524F}', false)), // East Asian ideograph
    (0x222B5A, ('\u{6047}', false)), // East Asian ideograph
    (0x2E2B5B, ('\u{605A}', false)), // East Asian ideograph
    (0x232B5D, ('\u{8789}', false)), // East Asian ideograph
    (0x222B5E, ('\u{6049}', false)), // East Asian ideograph
    (0x222B5F, ('\u{6053}', false)), // East Asian ideograph
    (0x232B60, ('\u{87AD}', false)), // East Asian ideograph
    (0x4B4E37, ('\u{7814}', false)), // East Asian ideograph
    (0x23473B, ('\u{940F}', false)), // East Asian ideograph
    (0x232B66, ('\u{87BE}', false)), // East Asian ideograph
    (0x222B68, ('\u{6067}', false)), // East Asian ideograph
    (0x23473C, ('\u{9420}', false)), // East Asian ideograph (not in Unicode)
    (0x232B6E, ('\u{87C4}', false)), // East Asian ideograph
    (0x232B6F, ('\u{87AF}', false)), // East Asian ideograph
    (0x222B71, ('\u{6041}', false)), // East Asian ideograph
    (0x222B72, ('\u{6077}', false)), // East Asian ideograph
    (0x222B74, ('\u{6042}', false)), // East Asian ideograph
    (0x22473E, ('\u{6C94}', false)), // East Asian ideograph
    (0x222B76, ('\u{605F}', false)), // East Asian ideograph
    (0x232B78, ('\u{87AE}', false)), // East Asian ideograph
    (0x2E6C27, ('\u{7B2E}', false)), // East Asian ideograph
    (0x222B7A, ('\u{6061}', false)), // East Asian ideograph
    (0x6F4E6F, ('\u{B799}', false)), // Korean hangul
    (0x232B7E, ('\u{87BF}', false)), // East Asian ideograph
    (0x692569, ('\u{30E9}', false)), // Katakana letter RA
    (0x224740, ('\u{6C8F}', false)), // East Asian ideograph
    (0x333C52, ('\u{8CEC}', false)), // East Asian ideograph
    (0x4B4741, ('\u{51BD}', false)), // East Asian ideograph
    (0x23233C, ('\u{8452}', false)), // East Asian ideograph
    (0x224742, ('\u{6C65}', false)), // East Asian ideograph
    (0x213D58, ('\u{5F70}', false)), // East Asian ideograph
    (0x2D5E61, ('\u{6FF6}', false)), // East Asian ideograph
    (0x6F4C3D, ('\u{B1CC}', false)), // Korean hangul
    (0x69256A, ('\u{30EA}', false)), // Katakana letter RI
    (0x294340, ('\u{94D6}', false)), // East Asian ideograph
    (0x6F5752, ('\u{C887}', false)), // Korean hangul
    (0x4B4B3E, ('\u{F9AD}', false)), // East Asian ideograph
    (0x2D4746, ('\u{6C79}', false)), // East Asian ideograph
    (0x2D6147, ('\u{99C8}', false)), // East Asian ideograph
    (0x224747, ('\u{6C6F}', false)), // East Asian ideograph
    (0x705F39, ('\u{5416}', false)), // East Asian ideograph
    (0x224749, ('\u{6C9D}', false)), // East Asian ideograph
    (0x275D5C, ('\u{94DC}', false)), // East Asian ideograph
    (0x295433, ('\u{9AA7}', false)), // East Asian ideograph
    (0x2F4A2E, ('\u{90B4}', false)), // East Asian ideograph
    (0x22474A, ('\u{6C69}', false)), // East Asian ideograph
    (0x22474B, ('\u{6C9A}', false)), // East Asian ideograph
    (0x21383B, ('\u{57F7}', false)), // East Asian ideograph
    (0x22474C, ('\u{6C6D}', false)), // East Asian ideograph
    (0x23474D, ('\u{9419}', false)), // East Asian ideograph
    (0x275B47, ('\u{8F8D}', false)), // East Asian ideograph
    (0x23474E, ('\u{940D}', false)), // East Asian ideograph
    (0x275D5D, ('\u{94ED}', false)), // East Asian ideograph
    (0x6F4A2F, ('\u{ADFF}', false)), // Korean hangul
    (0x234750, ('\u{9426}', false)), // East Asian ideograph
    (0x6F5D4A, ('\u{D68C}', false)), // Korean hangul
    (0x224751, ('\u{6C87}', false)), // East Asian ideograph
    (0x39345B, ('\u{965E}', false)), // East Asian ideograph
    (0x224752, ('\u{6C6E}', false)), // East Asian ideograph
    (0x6F4E73, ('\u{B7A9}', false)), // Korean hangul
    (0x69256D, ('\u{30ED}', false)), // Katakana letter RO
    (0x4D4754, ('\u{9544}', false)), // East Asian ideograph
    (0x294343, ('\u{94D2}', false)), // East Asian ideograph
    (0x276D2E, ('\u{5326}', false)), // East Asian ideograph
    (0x235A2C, ('\u{9CF8}', false)), // East Asian ideograph
    (0x224756, ('\u{6C95}', false)), // East Asian ideograph
    (0x234758, ('\u{9414}', false)), // East Asian ideograph
    (0x275D5F, ('\u{94EC}', false)), // East Asian ideograph
    (0x6F4A31, ('\u{AE01}', false)), // Korean hangul
    (0x274759, ('\u{6CEA}', false)), // East Asian ideograph
    (0x6F582D, ('\u{C9C8}', false)), // Korean hangul
    (0x4C715A, ('\u{7EE6}', false)), // East Asian ideograph
    (0x22475A, ('\u{6C82}', false)), // East Asian ideograph
    (0x2D5340, ('\u{812C}', false)), // East Asian ideograph
    (0x2D3B6E, ('\u{5D17}', false)), // East Asian ideograph
    (0x232C24, ('\u{87BD}', false)), // East Asian ideograph
    (0x23475C, ('\u{9422}', false)), // East Asian ideograph
    (0x222C2B, ('\u{6092}', false)), // East Asian ideograph
    (0x222C2C, ('\u{609D}', false)), // East Asian ideograph
    (0x222C2D, ('\u{6081}', false)), // East Asian ideograph
    (0x23475D, ('\u{9406}', false)), // East Asian ideograph
    (0x232C30, ('\u{87F3}', false)), // East Asian ideograph
    (0x232C31, ('\u{87F0}', false)), // East Asian ideograph
    (0x222C32, ('\u{6097}', false)), // East Asian ideograph
    (0x215673, ('\u{8725}', false)), // East Asian ideograph
    (0x232C34, ('\u{87EA}', false)), // East Asian ideograph
    (0x29475E, ('\u{9562}', false)), // East Asian ideograph
    (0x232C36, ('\u{87DB}', false)), // East Asian ideograph
    (0x232C37, ('\u{87E2}', false)), // East Asian ideograph
    (0x232C39, ('\u{87EB}', false)), // East Asian ideograph
    (0x222C3A, ('\u{6095}', false)), // East Asian ideograph
    (0x2D475F, ('\u{51C4}', false)), // East Asian ideograph
    (0x4D2C3C, ('\u{87E5}', false)), // East Asian ideograph
    (0x222C3E, ('\u{60C7}', false)), // East Asian ideograph
    (0x232C3F, ('\u{87F5}', false)), // East Asian ideograph
    (0x217D48, ('\u{5B21}', false)), // East Asian ideograph
    (0x234760, ('\u{9410}', false)), // East Asian ideograph
    (0x222C42, ('\u{60B0}', false)), // East Asian ideograph
    (0x6F4D33, ('\u{B36A}', false)), // Korean hangul
    (0x222C46, ('\u{60BE}', false)), // East Asian ideograph
    (0x232C47, ('\u{87E0}', false)), // East Asian ideograph
    (0x222C48, ('\u{60D4}', false)), // East Asian ideograph
    (0x232C49, ('\u{87DC}', false)), // East Asian ideograph
    (0x232C4C, ('\u{87E3}', false)), // East Asian ideograph
    (0x232C4D, ('\u{8801}', false)), // East Asian ideograph
    (0x222C4E, ('\u{60CE}', false)), // East Asian ideograph
    (0x232C4F, ('\u{8803}', false)), // East Asian ideograph
    (0x232C50, ('\u{880A}', false)), // East Asian ideograph
    (0x222C51, ('\u{60CF}', false)), // East Asian ideograph
    (0x222C53, ('\u{60D9}', false)), // East Asian ideograph
    (0x222C54, ('\u{60B3}', false)), // East Asian ideograph
    (0x232C55, ('\u{87F6}', false)), // East Asian ideograph
    (0x222C56, ('\u{60DD}', false)), // East Asian ideograph
    (0x232C57, ('\u{87F7}', false)), // East Asian ideograph
    (0x235A2F, ('\u{9D2A}', false)), // East Asian ideograph
    (0x232C5C, ('\u{880B}', false)), // East Asian ideograph
    (0x232C5D, ('\u{8806}', false)), // East Asian ideograph
    (0x232C5F, ('\u{87FE}', false)), // East Asian ideograph
    (0x222C60, ('\u{60B1}', false)), // East Asian ideograph
    (0x232C61, ('\u{8810}', false)), // East Asian ideograph
    (0x222C62, ('\u{60E3}', false)), // East Asian ideograph
    (0x232C63, ('\u{8819}', false)), // East Asian ideograph
    (0x232C64, ('\u{8811}', false)), // East Asian ideograph
    (0x224766, ('\u{6CEF}', false)), // East Asian ideograph
    (0x232C66, ('\u{8818}', false)), // East Asian ideograph
    (0x222C67, ('\u{60E5}', false)), // East Asian ideograph
    (0x222C69, ('\u{60DB}', false)), // East Asian ideograph
    (0x232C6A, ('\u{8813}', false)), // East Asian ideograph
    (0x232C6B, ('\u{8816}', false)), // East Asian ideograph
    (0x6F5236, ('\u{BEE0}', false)), // Korean hangul
    (0x275D62, ('\u{950C}', false)), // East Asian ideograph
    (0x222C6E, ('\u{60E9}', false)), // East Asian ideograph
    (0x692571, ('\u{30F1}', false)), // Katakana letter WE
    (0x222C70, ('\u{6114}', false)), // East Asian ideograph
    (0x274768, ('\u{6D45}', false)), // East Asian ideograph
    (0x232C72, ('\u{8834}', false)), // East Asian ideograph
    (0x232C73, ('\u{881C}', false)), // East Asian ideograph
    (0x222C75, ('\u{6119}', false)), // East Asian ideograph
    (0x234769, ('\u{93F7}', false)), // East Asian ideograph
    (0x232C7A, ('\u{881B}', false)), // East Asian ideograph
    (0x222C7C, ('\u{60FD}', false)), // East Asian ideograph
    (0x222C7D, ('\u{610D}', false)), // East Asian ideograph
    (0x6F5B41, ('\u{D1F4}', false)), // Korean hangul
    (0x29323B, ('\u{8BCE}', false)), // East Asian ideograph
    (0x287042, ('\u{7EA1}', false)), // East Asian ideograph
    (0x4B476C, ('\u{51C5}', false)), // East Asian ideograph
    (0x275D63, ('\u{9511}', false)), // East Asian ideograph
    (0x6F4A35, ('\u{AE0D}', false)), // Korean hangul
    (0x223E61, ('\u{6971}', false)), // East Asian ideograph
    (0x22476E, ('\u{6CAD}', false)), // East Asian ideograph (variant of 4C476E which maps to 6CAD)
    (0x2D5344, ('\u{8107}', false)), // East Asian ideograph
    (0x23476F, ('\u{940E}', false)), // East Asian ideograph
    (0x6F5B2E, ('\u{D14C}', false)), // Korean hangul
    (0x29323C, ('\u{8BD2}', false)), // East Asian ideograph
    (0x6F4E39, ('\u{B5B5}', false)), // Korean hangul
    (0x224770, ('\u{6CAF}', false)), // East Asian ideograph
    (0x295925, ('\u{9CCC}', false)), // East Asian ideograph
    (0x234771, ('\u{9411}', false)), // East Asian ideograph
    (0x692573, ('\u{30F3}', false)), // Katakana letter N
    (0x275921, ('\u{8C04}', false)), // East Asian ideograph
    (0x294349, ('\u{94D5}', false)), // East Asian ideograph
    (0x2D386E, ('\u{58CA}', false)), // East Asian ideograph
    (0x217E23, ('\u{5B62}', false)), // East Asian ideograph
    (0x274774, ('\u{6E0A}', false)), // East Asian ideograph
    (0x6F5B43, ('\u{D22C}', false)), // Korean hangul
    (0x275551, ('\u{80E1}', false)), // East Asian ideograph (duplicate simplified)
    (0x22496A, ('\u{6DAC}', false)), // East Asian ideograph
    (0x6F5B3E, ('\u{D1B3}', false)), // Korean hangul
    (0x225265, ('\u{7168}', false)), // East Asian ideograph
    (0x2D467C, ('\u{6CB2}', false)), // East Asian ideograph
    (0x29464A, ('\u{953C}', false)), // East Asian ideograph
    (0x3F3E47, ('\u{5379}', false)), // East Asian ideograph
    (0x21345F, ('\u{5352}', false)), // East Asian ideograph
    (0x274777, ('\u{6CA6}', false)), // East Asian ideograph
    (0x2D6159, ('\u{9AC4}', false)), // East Asian ideograph
    (0x213223, ('\u{5021}', false)), // East Asian ideograph
    (0x234779, ('\u{9429}', false)), // East Asian ideograph
    (0x213224, ('\u{500B}', false)), // East Asian ideograph
    (0x695457, ('\u{58B8}', false)), // East Asian ideograph
    (0x22477A, ('\u{6CBA}', false)), // East Asian ideograph
    (0x223225, ('\u{6387}', false)), // East Asian ideograph
    (0x22477B, ('\u{7553}', false)), // East Asian ideograph
    (0x223226, ('\u{637A}', false)), // East Asian ideograph
    (0x6F4B65, ('\u{B0C5}', false)), // Korean hangul
    (0x6F4A38, ('\u{AE34}', false)), // Korean hangul
    (0x213460, ('\u{5354}', false)), // East Asian ideograph
    (0x233227, ('\u{8A51}', false)), // East Asian ideograph
    (0x27477D, ('\u{6D8C}', false)), // East Asian ideograph
    (0x213228, ('\u{4FF3}', false)), // East Asian ideograph
    (0x6F5330, ('\u{C136}', false)), // Korean hangul
    (0x3F3D6F, ('\u{8986}', false)), // East Asian ideograph
    (0x287272, ('\u{7F21}', false)), // East Asian ideograph
    (0x6F4C44, ('\u{B205}', false)), // Korean hangul
    (0x213229, ('\u{502D}', false)), // East Asian ideograph
    (0x28742E, ('\u{7F42}', false)), // East Asian ideograph
    (0x6F4F3F, ('\u{B871}', false)), // Korean hangul
    (0x22322A, ('\u{6386}', false)), // East Asian ideograph
    (0x4C5C61, ('\u{74F4}', false)), // East Asian ideograph (variant of 225C61 which maps to 74F4)
    (0x6F4E7C, ('\u{B7F0}', false)), // Korean hangul
    (0x6F5237, ('\u{BEE3}', false)), // Korean hangul
    (0x21722B, ('\u{55F9}', false)), // East Asian ideograph
    (0x692576, ('\u{30F6}', false)), // Katakana letter small KE
    (0x223860, ('\u{6673}', false)), // East Asian ideograph
    (0x21322D, ('\u{502B}', false)), // East Asian ideograph
    (0x6F5D4C, ('\u{D69F}', false)), // Korean hangul
    (0x6F5B46, ('\u{D234}', false)), // Korean hangul
    (0x21322E, ('\u{505C}', false)), // East Asian ideograph
    (0x22496D, ('\u{6DD5}', false)), // East Asian ideograph
    (0x232B53, ('\u{8785}', false)), // East Asian ideograph
    (0x21322F, ('\u{504F}', false)), // East Asian ideograph
    (0x225F2F, ('\u{760F}', false)), // East Asian ideograph
    (0x292657, ('\u{835F}', false)), // East Asian ideograph
    (0x233230, ('\u{8A56}', false)), // East Asian ideograph
    (0x232D23, ('\u{8828}', false)), // East Asian ideograph
    (0x217231, ('\u{560C}', false)), // East Asian ideograph
    (0x232D2A, ('\u{8832}', false)), // East Asian ideograph
    (0x222D2C, ('\u{6110}', false)), // East Asian ideograph
    (0x232D2E, ('\u{882E}', false)), // East Asian ideograph
    (0x213E35, ('\u{600E}', false)), // East Asian ideograph
    (0x232D32, ('\u{882D}', false)), // East Asian ideograph
    (0x213233, ('\u{5049}', false)), // East Asian ideograph
    (0x222D34, ('\u{60F2}', false)), // East Asian ideograph
    (0x222D37, ('\u{6125}', false)), // East Asian ideograph
    (0x277234, ('\u{551B}', false)), // East Asian ideograph
    (0x222D3B, ('\u{60F8}', false)), // East Asian ideograph
    (0x232D3C, ('\u{883C}', false)), // East Asian ideograph
    (0x6F4E7E, ('\u{B7FC}', false)), // Korean hangul
    (0x273235, ('\u{4FA7}', false)), // East Asian ideograph
    (0x222D41, ('\u{60FC}', false)), // East Asian ideograph
    (0x232D42, ('\u{4610}', false)), // East Asian ideograph (not in Unicode)
    (0x275926, ('\u{8C1A}', false)), // East Asian ideograph
    (0x232D44, ('\u{8844}', false)), // East Asian ideograph
    (0x227236, ('\u{7DF9}', false)), // East Asian ideograph
    (0x222D48, ('\u{6149}', false)), // East Asian ideograph
    (0x222D4A, ('\u{614A}', false)), // East Asian ideograph
    (0x232D4B, ('\u{8847}', false)), // East Asian ideograph
    (0x222D4E, ('\u{612B}', false)), // East Asian ideograph
    (0x287275, ('\u{7D77}', false)), // East Asian ideograph
    (0x222D50, ('\u{6129}', false)), // East Asian ideograph
    (0x222D51, ('\u{6150}', false)), // East Asian ideograph
    (0x232D53, ('\u{884E}', false)), // East Asian ideograph
    (0x232D56, ('\u{8852}', false)), // East Asian ideograph
    (0x233239, ('\u{8A48}', false)), // East Asian ideograph
    (0x222D58, ('\u{6130}', false)), // East Asian ideograph
    (0x232D59, ('\u{8856}', false)), // East Asian ideograph
    (0x232D5A, ('\u{8855}', false)), // East Asian ideograph
    (0x222D5B, ('\u{6141}', false)), // East Asian ideograph
    (0x21323A, ('\u{5055}', false)), // East Asian ideograph
    (0x232D5E, ('\u{885C}', false)), // East Asian ideograph
    (0x232D5F, ('\u{885A}', false)), // East Asian ideograph
    (0x222D61, ('\u{6146}', false)), // East Asian ideograph
    (0x22323B, ('\u{6390}', false)), // East Asian ideograph
    (0x222D66, ('\u{615E}', false)), // East Asian ideograph
    (0x222D67, ('\u{6175}', false)), // East Asian ideograph
    (0x222D68, ('\u{6174}', false)), // East Asian ideograph
    (0x232D69, ('\u{8869}', false)), // East Asian ideograph
    (0x21316C, ('\u{5009}', false)), // East Asian ideograph
    (0x222D6B, ('\u{6183}', false)), // East Asian ideograph
    (0x232D6D, ('\u{886D}', false)), // East Asian ideograph
    (0x232D6E, ('\u{887A}', false)), // East Asian ideograph
    (0x21323D, ('\u{508D}', false)), // East Asian ideograph
    (0x222D70, ('\u{6171}', false)), // East Asian ideograph
    (0x232D71, ('\u{8875}', false)), // East Asian ideograph
    (0x222757, ('\u{5E5E}', false)), // East Asian ideograph
    (0x222D74, ('\u{616A}', false)), // East Asian ideograph
    (0x232D75, ('\u{8872}', false)), // East Asian ideograph
    (0x6F5045, ('\u{BB0F}', false)), // Korean hangul
    (0x222D77, ('\u{6173}', false)), // East Asian ideograph
    (0x232D79, ('\u{887D}', false)), // East Asian ideograph
    (0x455564, ('\u{6FDB}', false)), // East Asian ideograph (Version J extension)
    (0x222D7B, ('\u{6153}', false)), // East Asian ideograph
    (0x224234, ('\u{6A99}', false)), // East Asian ideograph
    (0x232D7D, ('\u{887F}', false)), // East Asian ideograph
    (0x232D7E, ('\u{887E}', false)), // East Asian ideograph
    (0x275928, ('\u{8BB3}', false)), // East Asian ideograph
    (0x294350, ('\u{94DF}', false)), // East Asian ideograph
    (0x233240, ('\u{8A3D}', false)), // East Asian ideograph
    (0x696126, ('\u{74F2}', false)), // East Asian ideograph
    (0x6F567B, ('\u{C794}', false)), // Korean hangul
    (0x273241, ('\u{6770}', false)), // East Asian ideograph
    (0x6F5874, ('\u{CC0D}', false)), // Korean hangul
    (0x6F5B4A, ('\u{D241}', false)), // Korean hangul
    (0x213242, ('\u{5080}', false)), // East Asian ideograph
    (0x4B5C5B, ('\u{8FBA}', false)), // East Asian ideograph
    (0x223243, ('\u{63DE}', false)), // East Asian ideograph
    (0x6F5238, ('\u{BEE4}', false)), // Korean hangul
    (0x213244, ('\u{5098}', false)), // East Asian ideograph
    (0x6F4A3E, ('\u{AE44}', false)), // Korean hangul
    (0x275929, ('\u{8C10}', false)), // East Asian ideograph
    (0x2D4539, ('\u{6406}', false)), // East Asian ideograph
    (0x213246, ('\u{50B3}', false)), // East Asian ideograph
    (0x274C60, ('\u{75A1}', false)), // East Asian ideograph
    (0x6F5B4B, ('\u{D264}', false)), // Korean hangul
    (0x273247, ('\u{503A}', false)), // East Asian ideograph
    (0x227248, ('\u{7DF6}', false)), // East Asian ideograph
    (0x23492E, ('\u{9585}', false)), // East Asian ideograph
    (0x213249, ('\u{50C5}', false)), // East Asian ideograph
    (0x6F4A3F, ('\u{AE45}', false)), // Korean hangul
    (0x27592A, ('\u{8C0D}', false)), // East Asian ideograph
    (0x223866, ('\u{666D}', false)), // East Asian ideograph
    (0x21724B, ('\u{564B}', false)), // East Asian ideograph
    (0x274C61, ('\u{759F}', false)), // East Asian ideograph
    (0x6F5B4C, ('\u{D277}', false)), // Korean hangul
    (0x21324C, ('\u{50B7}', false)), // East Asian ideograph
    (0x393246, ('\u{4F1D}', false)), // East Asian ideograph
    (0x21324D, ('\u{50AF}', false)), // East Asian ideograph
    (0x275D6E, ('\u{9530}', false)), // East Asian ideograph
    (0x6F4A40, ('\u{AE4A}', false)), // Korean hangul
    (0x27592B, ('\u{8C0B}', false)), // East Asian ideograph
    (0x6F5830, ('\u{C9D1}', false)), // Korean hangul
    (0x21324F, ('\u{50EE}', false)), // East Asian ideograph
    (0x213250, ('\u{50F1}', false)), // East Asian ideograph
    (0x274C62, ('\u{75EA}', false)), // East Asian ideograph
    (0x333963, ('\u{59C9}', false)), // East Asian ideograph
    (0x213251, ('\u{50E5}', false)), // East Asian ideograph
    (0x217252, ('\u{5640}', false)), // East Asian ideograph
    (0x292433, ('\u{8298}', false)), // East Asian ideograph (duplicate simplified)
    (0x69515E, ('\u{51E9}', false)), // East Asian ideograph
    (0x287253, ('\u{7F12}', false)), // East Asian ideograph
    (0x6F575A, ('\u{C89F}', false)), // Korean hangul
    (0x223B63, ('\u{67D8}', false)), // East Asian ideograph
    (0x213255, ('\u{50D5}', false)), // East Asian ideograph
    (0x274C63, ('\u{75AF}', false)), // East Asian ideograph
    (0x4C523A, ('\u{717A}', false)), // East Asian ideograph
    (0x213256, ('\u{507D}', false)), // East Asian ideograph
    (0x234D74, ('\u{97AB}', false)), // East Asian ideograph
    (0x22674B, ('\u{798A}', false)), // East Asian ideograph
    (0x213257, ('\u{50CF}', false)), // East Asian ideograph
    (0x234931, ('\u{958C}', false)), // East Asian ideograph
    (0x213258, ('\u{50D1}', false)), // East Asian ideograph
    (0x224235, ('\u{6A9D}', false)), // East Asian ideograph
    (0x27592D, ('\u{8C13}', false)), // East Asian ideograph
    (0x294355, ('\u{94EB}', false)), // East Asian ideograph
    (0x273259, ('\u{4EEA}', false)), // East Asian ideograph
    (0x6F567C, ('\u{C796}', false)), // Korean hangul
    (0x21325A, ('\u{5104}', false)), // East Asian ideograph
    (0x6F5B4F, ('\u{D288}', false)), // Korean hangul
    (0x222E23, ('\u{618B}', false)), // East Asian ideograph
    (0x2D5129, ('\u{7D25}', false)), // East Asian ideograph
    (0x232E28, ('\u{88A2}', false)), // East Asian ideograph
    (0x21325C, ('\u{50F5}', false)), // East Asian ideograph
    (0x232E2A, ('\u{88A4}', false)), // East Asian ideograph
    (0x222E2C, ('\u{616F}', false)), // East Asian ideograph
    (0x222E2D, ('\u{6165}', false)), // East Asian ideograph
    (0x6F5239, ('\u{BEE5}', false)), // Korean hangul
    (0x232E2F, ('\u{88AA}', false)), // East Asian ideograph
    (0x276135, ('\u{9A7E}', false)), // East Asian ideograph
    (0x222E32, ('\u{619D}', false)), // East Asian ideograph
    (0x222E33, ('\u{61A6}', false)), // East Asian ideograph
    (0x232E34, ('\u{889A}', false)), // East Asian ideograph
    (0x27325E, ('\u{4FAC}', false)), // East Asian ideograph
    (0x235A3F, ('\u{9D1E}', false)), // East Asian ideograph
    (0x232E3A, ('\u{8890}', false)), // East Asian ideograph
    (0x232E3B, ('\u{888C}', false)), // East Asian ideograph
    (0x232E3D, ('\u{88A0}', false)), // East Asian ideograph
    (0x232E40, ('\u{8899}', false)), // East Asian ideograph
    (0x213260, ('\u{5108}', false)), // East Asian ideograph
    (0x222E42, ('\u{619C}', false)), // East Asian ideograph
    (0x222E43, ('\u{61AF}', false)), // East Asian ideograph
    (0x232E45, ('\u{8897}', false)), // East Asian ideograph
    (0x222E46, ('\u{6197}', false)), // East Asian ideograph
    (0x222E47, ('\u{61AD}', false)), // East Asian ideograph
    (0x232E48, ('\u{88C9}', false)), // East Asian ideograph
    (0x232E49, ('\u{88BF}', false)), // East Asian ideograph
    (0x232E4A, ('\u{88BA}', false)), // East Asian ideograph
    (0x222E4C, ('\u{6192}', false)), // East Asian ideograph
    (0x213262, ('\u{5110}', false)), // East Asian ideograph
    (0x232E4F, ('\u{88C0}', false)), // East Asian ideograph
    (0x6F4A44, ('\u{AE4D}', false)), // Korean hangul
    (0x232E51, ('\u{88B2}', false)), // East Asian ideograph
    (0x222E52, ('\u{61AE}', false)), // East Asian ideograph
    (0x213263, ('\u{5118}', false)), // East Asian ideograph
    (0x232E54, ('\u{88BC}', false)), // East Asian ideograph
    (0x222E55, ('\u{618D}', false)), // East Asian ideograph
    (0x232E57, ('\u{88B7}', false)), // East Asian ideograph
    (0x232E59, ('\u{88BD}', false)), // East Asian ideograph
    (0x232E5A, ('\u{88C4}', false)), // East Asian ideograph
    (0x2D313A, ('\u{62BB}', false)), // East Asian ideograph
    (0x222E5C, ('\u{61CC}', false)), // East Asian ideograph
    (0x222E5D, ('\u{61C6}', false)), // East Asian ideograph
    (0x232E5E, ('\u{88CB}', false)), // East Asian ideograph
    (0x213265, ('\u{5114}', false)), // East Asian ideograph
    (0x232E60, ('\u{88CC}', false)), // East Asian ideograph
    (0x232E62, ('\u{88DB}', false)), // East Asian ideograph
    (0x232E64, ('\u{88CE}', false)), // East Asian ideograph
    (0x224535, ('\u{6BA3}', false)), // East Asian ideograph
    (0x234934, ('\u{9597}', false)), // East Asian ideograph
    (0x222E68, ('\u{61BA}', false)), // East Asian ideograph
    (0x222E6A, ('\u{61B8}', false)), // East Asian ideograph
    (0x273267, ('\u{507F}', false)), // East Asian ideograph
    (0x6F4A45, ('\u{AE4E}', false)), // Korean hangul
    (0x275930, ('\u{8C15}', false)), // East Asian ideograph
    (0x294358, ('\u{94EF}', false)), // East Asian ideograph
    (0x232E71, ('\u{88F1}', false)), // East Asian ideograph
    (0x232E72, ('\u{88FE}', false)), // East Asian ideograph
    (0x6F5D66, ('\u{D734}', false)), // Korean hangul
    (0x232E75, ('\u{88F2}', false)), // East Asian ideograph
    (0x233269, ('\u{8A8F}', false)), // East Asian ideograph
    (0x232E78, ('\u{8900}', false)), // East Asian ideograph
    (0x213F2E, ('\u{6176}', false)), // East Asian ideograph
    (0x232E7A, ('\u{88F0}', false)), // East Asian ideograph
    (0x6F5B52, ('\u{D293}', false)), // Korean hangul
    (0x222E7D, ('\u{61DC}', false)), // East Asian ideograph
    (0x222E7E, ('\u{61DF}', false)), // East Asian ideograph
    (0x69723B, ('\u{9B96}', false)), // East Asian ideograph
    (0x21326B, ('\u{513C}', false)), // East Asian ideograph
    (0x276069, ('\u{996A}', false)), // East Asian ideograph
    (0x294359, ('\u{94E5}', false)), // East Asian ideograph
    (0x4B5434, ('\u{6319}', false)), // East Asian ideograph
    (0x6F5B53, ('\u{D295}', false)), // Korean hangul
    (0x21326F, ('\u{5145}', false)), // East Asian ideograph
    (0x223270, ('\u{63BE}', false)), // East Asian ideograph
    (0x234936, ('\u{958E}', false)), // East Asian ideograph
    (0x2D4249, ('\u{53D9}', false)), // East Asian ideograph
    (0x213271, ('\u{5146}', false)), // East Asian ideograph
    (0x224236, ('\u{6A7E}', false)), // East Asian ideograph
    (0x6F4A47, ('\u{AE54}', false)), // Korean hangul
    (0x275932, ('\u{8C26}', false)), // East Asian ideograph
    (0x217272, ('\u{5660}', false)), // East Asian ideograph
    (0x295834, ('\u{9CBB}', false)), // East Asian ideograph
    (0x223273, ('\u{63DD}', false)), // East Asian ideograph
    (0x6F5B54, ('\u{D29C}', false)), // Korean hangul
    (0x22497B, ('\u{6DBF}', false)), // East Asian ideograph
    (0x213275, ('\u{514C}', false)), // East Asian ideograph
    (0x6F523A, ('\u{BEEC}', false)), // Korean hangul
    (0x2D3A26, ('\u{5A3F}', false)), // East Asian ideograph
    (0x6F5D21, ('\u{D5D9}', false)), // Korean hangul
    (0x283F5C, ('\u{6769}', false)), // East Asian ideograph
    (0x29435B, ('\u{94E3}', false)), // East Asian ideograph
    (0x213277, ('\u{514D}', false)), // East Asian ideograph
    (0x6F5D22, ('\u{D5DB}', false)), // Korean hangul
    (0x2D5D23, ('\u{9167}', false)), // East Asian ideograph
    (0x274C6A, ('\u{75AE}', false)), // East Asian ideograph
    (0x6F4C2E, ('\u{B158}', false)), // Korean hangul
    (0x213279, ('\u{5154}', false)), // East Asian ideograph
    (0x705F54, ('\u{54B4}', false)), // East Asian ideograph
    (0x6F5D24, ('\u{D5E4}', false)), // Korean hangul
    (0x29324F, ('\u{8BD6}', false)), // East Asian ideograph
    (0x393460, ('\u{604A}', false)), // East Asian ideograph
    (0x273859, ('\u{5C18}', false)), // East Asian ideograph
    (0x225D25, ('\u{750E}', false)), // East Asian ideograph
    (0x21327B, ('\u{5157}', false)), // East Asian ideograph
    (0x6F5D26, ('\u{D5E8}', false)), // Korean hangul
    (0x275934, ('\u{8BB2}', false)), // East Asian ideograph
    (0x223870, ('\u{6684}', false)), // East Asian ideograph
    (0x235D27, ('\u{9E0E}', false)), // East Asian ideograph
    (0x6F577C, ('\u{C961}', false)), // Korean hangul
    (0x21327D, ('\u{5162}', false)), // East Asian ideograph
    (0x225D28, ('\u{750D}', false)), // East Asian ideograph
    (0x213E38, ('\u{6059}', false)), // East Asian ideograph
    (0x23327E, ('\u{8AB6}', false)), // East Asian ideograph
    (0x295D29, ('\u{9E71}', false)), // East Asian ideograph
    (0x293250, ('\u{8BD3}', false)), // East Asian ideograph
    (0x275D2A, ('\u{9154}', false)), // East Asian ideograph
    (0x235D2B, ('\u{9E11}', false)), // East Asian ideograph
    (0x2F4A4A, ('\u{5F8F}', false)), // East Asian ideograph
    (0x275935, ('\u{8C0E}', false)), // East Asian ideograph
    (0x225D2C, ('\u{7511}', false)), // East Asian ideograph
    (0x225D2D, ('\u{750F}', false)), // East Asian ideograph
    (0x2D3140, ('\u{4F32}', false)), // East Asian ideograph
    (0x6F5B57, ('\u{D2AC}', false)), // Korean hangul
    (0x6F5D2E, ('\u{D604}', false)), // Korean hangul
    (0x215D2F, ('\u{919E}', false)), // East Asian ideograph
    (0x275D79, ('\u{952E}', false)), // East Asian ideograph
    (0x275D30, ('\u{4E11}', false)), // East Asian ideograph
    (0x6F4A4B, ('\u{AE61}', false)), // Korean hangul
    (0x232F23, ('\u{88EF}', false)), // East Asian ideograph
    (0x232F24, ('\u{8903}', false)), // East Asian ideograph
    (0x39456D, ('\u{826B}', false)), // East Asian ideograph
    (0x6F5758, ('\u{C89C}', false)), // Korean hangul
    (0x215D31, ('\u{91AB}', false)), // East Asian ideograph
    (0x225648, ('\u{72A8}', false)), // East Asian ideograph
    (0x222F29, ('\u{61F3}', false)), // East Asian ideograph
    (0x275D32, ('\u{9171}', false)), // East Asian ideograph
    (0x274C6D, ('\u{75E8}', false)), // East Asian ideograph
    (0x212F30, ('\u{3007}', false)), // East Asian ideograph (number zero)
    (0x225D33, ('\u{7513}', false)), // East Asian ideograph
    (0x232F35, ('\u{8906}', false)), // East Asian ideograph
    (0x232F36, ('\u{890C}', false)), // East Asian ideograph
    (0x232F37, ('\u{8919}', false)), // East Asian ideograph
    (0x215D34, ('\u{91C0}', false)), // East Asian ideograph
    (0x232F3D, ('\u{890A}', false)), // East Asian ideograph
    (0x6F4B69, ('\u{B0D0}', false)), // Korean hangul
    (0x215D35, ('\u{91C1}', false)), // East Asian ideograph
    (0x6F4A4C, ('\u{AE62}', false)), // Korean hangul
    (0x222F41, ('\u{6204}', false)), // East Asian ideograph
    (0x235742, ('\u{9B9E}', false)), // East Asian ideograph
    (0x222F43, ('\u{6207}', false)), // East Asian ideograph
    (0x222F44, ('\u{6209}', false)), // East Asian ideograph
    (0x232F45, ('\u{892F}', false)), // East Asian ideograph
    (0x232F47, ('\u{8930}', false)), // East Asian ideograph
    (0x345E47, ('\u{75FE}', false)), // East Asian ideograph
    (0x235D37, ('\u{9E18}', false)), // East Asian ideograph
    (0x274C6E, ('\u{7597}', false)), // East Asian ideograph
    (0x232F4E, ('\u{8921}', false)), // East Asian ideograph
    (0x232F4F, ('\u{8927}', false)), // East Asian ideograph
    (0x232F51, ('\u{891F}', false)), // East Asian ideograph
    (0x232F53, ('\u{8931}', false)), // East Asian ideograph
    (0x232F54, ('\u{891E}', false)), // East Asian ideograph
    (0x295029, ('\u{98A5}', false)), // East Asian ideograph
    (0x232F56, ('\u{8926}', false)), // East Asian ideograph
    (0x232F57, ('\u{8922}', false)), // East Asian ideograph
    (0x232F5A, ('\u{8935}', false)), // East Asian ideograph
    (0x222F5B, ('\u{6225}', false)), // East Asian ideograph
    (0x232F5D, ('\u{8941}', false)), // East Asian ideograph
    (0x275938, ('\u{8C22}', false)), // East Asian ideograph
    (0x232F60, ('\u{8933}', false)), // East Asian ideograph
    (0x222F61, ('\u{6229}', false)), // East Asian ideograph
    (0x235D3B, ('\u{9E1D}', false)), // East Asian ideograph
    (0x232F66, ('\u{8954}', false)), // East Asian ideograph
    (0x222F67, ('\u{622D}', false)), // East Asian ideograph
    (0x6F5D50, ('\u{D6C5}', false)), // Korean hangul
    (0x215D3C, ('\u{91CF}', false)), // East Asian ideograph
    (0x6F5B5A, ('\u{D2B9}', false)), // Korean hangul
    (0x222F6E, ('\u{6239}', false)), // East Asian ideograph
    (0x222F6F, ('\u{623A}', false)), // East Asian ideograph
    (0x222F70, ('\u{623D}', false)), // East Asian ideograph
    (0x232F72, ('\u{8947}', false)), // East Asian ideograph
    (0x27385A, ('\u{57AB}', false)), // East Asian ideograph
    (0x222F75, ('\u{6243}', false)), // East Asian ideograph
    (0x222F77, ('\u{6246}', false)), // East Asian ideograph
    (0x222F78, ('\u{6245}', false)), // East Asian ideograph
    (0x222F79, ('\u{624A}', false)), // East Asian ideograph
    (0x232F7A, ('\u{894C}', false)), // East Asian ideograph
    (0x232F7B, ('\u{8946}', false)), // East Asian ideograph
    (0x222F7C, ('\u{625E}', false)), // East Asian ideograph
    (0x295859, ('\u{9CC6}', false)), // East Asian ideograph
    (0x275D40, ('\u{9489}', false)), // East Asian ideograph
    (0x275D41, ('\u{948A}', false)), // East Asian ideograph
    (0x6F5B5B, ('\u{D2BC}', false)), // Korean hangul
    (0x215D42, ('\u{91DC}', false)), // East Asian ideograph
    (0x6F4E3A, ('\u{B5BB}', false)), // Korean hangul
    (0x275D43, ('\u{9497}', false)), // East Asian ideograph
    (0x45604E, ('\u{984F}', false)), // East Asian ideograph
    (0x215D44, ('\u{91E6}', false)), // East Asian ideograph
    (0x6F4A4F, ('\u{AE69}', false)), // Korean hangul
    (0x27593A, ('\u{8C2C}', false)), // East Asian ideograph
    (0x273721, ('\u{545C}', false)), // East Asian ideograph
    (0x275D45, ('\u{9493}', false)), // East Asian ideograph
    (0x2D535E, ('\u{8193}', false)), // East Asian ideograph
    (0x275D46, ('\u{948F}', false)), // East Asian ideograph
    (0x33632B, ('\u{7ADC}', false)), // East Asian ideograph
    (0x6F5B5C, ('\u{D2BF}', false)), // Korean hangul
    (0x705F5B, ('\u{54A3}', false)), // East Asian ideograph
    (0x215D47, ('\u{9223}', false)), // East Asian ideograph
    (0x293256, ('\u{8BE9}', false)), // East Asian ideograph
    (0x2E493B, ('\u{6E7C}', false)), // East Asian ideograph
    (0x275D48, ('\u{949D}', false)), // East Asian ideograph
    (0x4B4921, ('\u{6CA2}', false)), // East Asian ideograph
    (0x235D49, ('\u{9E84}', false)), // East Asian ideograph
    (0x27606B, ('\u{996D}', false)), // East Asian ideograph
    (0x27593B, ('\u{8C1F}', false)), // East Asian ideograph
    (0x6F5759, ('\u{C89D}', false)), // Korean hangul
    (0x275D4A, ('\u{94A0}', false)), // East Asian ideograph
    (0x215D4B, ('\u{9214}', false)), // East Asian ideograph
    (0x6F5B5D, ('\u{D2C0}', false)), // Korean hangul
    (0x275D4C, ('\u{94A7}', false)), // East Asian ideograph
    (0x284C2E, ('\u{6D52}', false)), // East Asian ideograph
    (0x697246, ('\u{9BD1}', false)), // East Asian ideograph
    (0x275D4D, ('\u{94A4}', false)), // East Asian ideograph
    (0x2D3356, ('\u{5211}', false)), // East Asian ideograph (not in Unicode)
    (0x6F4B6A, ('\u{B0D1}', false)), // Korean hangul
    (0x6F5D4E, ('\u{D6A8}', false)), // Korean hangul
    (0x6F4A51, ('\u{AE70}', false)), // Korean hangul
    (0x27593C, ('\u{8BC6}', false)), // East Asian ideograph
    (0x294364, ('\u{94F7}', false)), // East Asian ideograph
    (0x233C77, ('\u{8FCB}', false)), // East Asian ideograph
    (0x213A38, ('\u{5AC2}', false)), // East Asian ideograph
    (0x275D50, ('\u{94B9}', false)), // East Asian ideograph
    (0x2D3147, ('\u{5002}', false)), // East Asian ideograph
    (0x6F5B5E, ('\u{D2C8}', false)), // Korean hangul
    (0x215D51, ('\u{923D}', false)), // East Asian ideograph
    (0x396074, ('\u{55B0}', false)), // East Asian ideograph
    (0x215D52, ('\u{923E}', false)), // East Asian ideograph
    (0x6F523C, ('\u{BF09}', false)), // Korean hangul
    (0x275D53, ('\u{94BE}', false)), // East Asian ideograph
    (0x21347A, ('\u{53AD}', false)), // East Asian ideograph
    (0x213037, ('\u{4E38}', false)), // East Asian ideograph
    (0x4B4B63, ('\u{749C}', false)), // East Asian ideograph
    (0x215D55, ('\u{925B}', false)), // East Asian ideograph
    (0x6F5B5F, ('\u{D2C9}', false)), // Korean hangul
    (0x275D56, ('\u{94A9}', false)), // East Asian ideograph
    (0x22675C, ('\u{799A}', false)), // East Asian ideograph
    (0x275D57, ('\u{94C2}', false)), // East Asian ideograph
    (0x234942, ('\u{95AC}', false)), // East Asian ideograph
    (0x225D58, ('\u{7547}', false)), // East Asian ideograph
    (0x6F4A53, ('\u{AE79}', false)), // Korean hangul
    (0x224830, ('\u{6CD1}', false)), // East Asian ideograph
    (0x275D59, ('\u{94F0}', false)), // East Asian ideograph
    (0x235A4F, ('\u{9D41}', false)), // East Asian ideograph
    (0x275D5A, ('\u{94F6}', false)), // East Asian ideograph
    (0x274C75, ('\u{765E}', false)), // East Asian ideograph
    (0x213021, ('\u{4E00}', false)), // East Asian ideograph
    (0x213022, ('\u{4E01}', false)), // East Asian ideograph
    (0x215D5B, ('\u{92AC}', false)), // East Asian ideograph
    (0x213024, ('\u{4E09}', false)), // East Asian ideograph
    (0x213025, ('\u{4E0B}', false)), // East Asian ideograph
    (0x223026, ('\u{6268}', false)), // East Asian ideograph
    (0x213027, ('\u{4E08}', false)), // East Asian ideograph
    (0x223028, ('\u{6260}', false)), // East Asian ideograph
    (0x233029, ('\u{895B}', false)), // East Asian ideograph
    (0x21302A, ('\u{4E0D}', false)), // East Asian ideograph
    (0x21302B, ('\u{4E14}', false)), // East Asian ideograph
    (0x22302C, ('\u{6262}', false)), // East Asian ideograph
    (0x21302D, ('\u{4E16}', false)), // East Asian ideograph
    (0x21302E, ('\u{4E15}', false)), // East Asian ideograph
    (0x215D5D, ('\u{9298}', false)), // East Asian ideograph
    (0x213030, ('\u{4E22}', false)), // East Asian ideograph
    (0x233031, ('\u{8966}', false)), // East Asian ideograph
    (0x223032, ('\u{628E}', false)), // East Asian ideograph
    (0x213034, ('\u{4E2D}', false)), // East Asian ideograph
    (0x275D5E, ('\u{94E2}', false)), // East Asian ideograph
    (0x213036, ('\u{51E1}', false)), // East Asian ideograph
    (0x233037, ('\u{896D}', false)), // East Asian ideograph
    (0x213038, ('\u{4E39}', false)), // East Asian ideograph
    (0x213039, ('\u{4E3B}', false)), // East Asian ideograph
    (0x23303A, ('\u{896B}', false)), // East Asian ideograph
    (0x23303B, ('\u{896E}', false)), // East Asian ideograph
    (0x23303C, ('\u{896C}', false)), // East Asian ideograph
    (0x21303D, ('\u{4E4B}', false)), // East Asian ideograph
    (0x21303E, ('\u{5C39}', false)), // East Asian ideograph
    (0x21303F, ('\u{4E4F}', false)), // East Asian ideograph
    (0x213040, ('\u{4E4E}', false)), // East Asian ideograph
    (0x233041, ('\u{8976}', false)), // East Asian ideograph
    (0x233042, ('\u{8974}', false)), // East Asian ideograph
    (0x223043, ('\u{6282}', false)), // East Asian ideograph
    (0x213044, ('\u{4E56}', false)), // East Asian ideograph
    (0x213045, ('\u{4E58}', false)), // East Asian ideograph
    (0x213046, ('\u{4E59}', false)), // East Asian ideograph
    (0x215D61, ('\u{929C}', false)), // East Asian ideograph
    (0x213048, ('\u{4E5F}', false)), // East Asian ideograph
    (0x233049, ('\u{897B}', false)), // East Asian ideograph
    (0x23304A, ('\u{897C}', false)), // East Asian ideograph
    (0x22304B, ('\u{629D}', false)), // East Asian ideograph
    (0x27304C, ('\u{5E72}', false)), // East Asian ideograph
    (0x225D62, ('\u{7564}', false)), // East Asian ideograph
    (0x6F4A55, ('\u{AE7C}', false)), // Korean hangul
    (0x213050, ('\u{4E8B}', false)), // East Asian ideograph
    (0x213051, ('\u{4E8C}', false)), // East Asian ideograph
    (0x213052, ('\u{4E8E}', false)), // East Asian ideograph
    (0x233053, ('\u{8984}', false)), // East Asian ideograph
    (0x213054, ('\u{4E94}', false)), // East Asian ideograph
    (0x233055, ('\u{8985}', false)), // East Asian ideograph
    (0x223056, ('\u{62A6}', false)), // East Asian ideograph
    (0x213057, ('\u{4E99}', false)), // East Asian ideograph (variant of 4B3057 which maps to 4E99)
    (0x273058, ('\u{4E9A}', false)), // East Asian ideograph
    (0x215D64, ('\u{92B3}', false)), // East Asian ideograph
    (0x21305A, ('\u{4E9F}', false)), // East Asian ideograph
    (0x274C77, ('\u{7663}', false)), // East Asian ideograph
    (0x21305C, ('\u{4EA6}', false)), // East Asian ideograph
    (0x21305D, ('\u{4EA5}', false)), // East Asian ideograph
    (0x21305E, ('\u{4EA4}', false)), // East Asian ideograph
    (0x215D65, ('\u{92EA}', false)), // East Asian ideograph
    (0x213060, ('\u{4EAB}', false)), // East Asian ideograph
    (0x213061, ('\u{4EAC}', false)), // East Asian ideograph
    (0x233062, ('\u{8991}', false)), // East Asian ideograph
    (0x213063, ('\u{4EAE}', false)), // East Asian ideograph
    (0x233064, ('\u{8997}', false)), // East Asian ideograph
    (0x215D66, ('\u{92B7}', false)), // East Asian ideograph
    (0x233066, ('\u{8998}', false)), // East Asian ideograph
    (0x4B5830, ('\u{899A}', false)), // East Asian ideograph
    (0x213068, ('\u{4EC3}', false)), // East Asian ideograph
    (0x213069, ('\u{4EC4}', false)), // East Asian ideograph
    (0x22306A, ('\u{62C3}', false)), // East Asian ideograph
    (0x23306B, ('\u{899C}', false)), // East Asian ideograph
    (0x21306C, ('\u{4EC7}', false)), // East Asian ideograph
    (0x21306D, ('\u{4ECB}', false)), // East Asian ideograph
    (0x21306E, ('\u{4EE4}', false)), // East Asian ideograph
    (0x23306F, ('\u{89A1}', false)), // East Asian ideograph
    (0x213070, ('\u{4ED5}', false)), // East Asian ideograph
    (0x275D68, ('\u{9504}', false)), // East Asian ideograph
    (0x223072, ('\u{630D}', false)), // East Asian ideograph
    (0x213073, ('\u{4EE3}', false)), // East Asian ideograph
    (0x213074, ('\u{4ED4}', false)), // East Asian ideograph
    (0x213075, ('\u{4ED7}', false)), // East Asian ideograph
    (0x233076, ('\u{89A5}', false)), // East Asian ideograph
    (0x275D69, ('\u{9509}', false)), // East Asian ideograph
    (0x213078, ('\u{4EFF}', false)), // East Asian ideograph
    (0x233079, ('\u{89A9}', false)), // East Asian ideograph
    (0x6F5B63, ('\u{D2F0}', false)), // Korean hangul
    (0x21307C, ('\u{4EFB}', false)), // East Asian ideograph
    (0x275D6A, ('\u{950B}', false)), // East Asian ideograph
    (0x21307E, ('\u{4F15}', false)), // East Asian ideograph
    (0x224547, ('\u{6BBD}', false)), // East Asian ideograph
    (0x215D6B, ('\u{9320}', false)), // East Asian ideograph
    (0x292A2F, ('\u{86F1}', false)), // East Asian ideograph
    (0x6F5D6C, ('\u{D754}', false)), // Korean hangul
    (0x29436A, ('\u{9512}', false)), // East Asian ideograph
    (0x215D6D, ('\u{92F8}', false)), // East Asian ideograph
    (0x235A53, ('\u{9D36}', false)), // East Asian ideograph
    (0x225D6E, ('\u{757A}', false)), // East Asian ideograph
    (0x6F535B, ('\u{C218}', false)), // Korean hangul
    (0x274C79, ('\u{766B}', false)), // East Asian ideograph
    (0x6F5B64, ('\u{D2F1}', false)), // Korean hangul
    (0x275D6F, ('\u{9519}', false)), // East Asian ideograph
    (0x29325E, ('\u{8BDC}', false)), // East Asian ideograph
    (0x6F5721, ('\u{C7A1}', false)), // Korean hangul
    (0x2F575F, ('\u{9ABE}', false)), // East Asian ideograph
    (0x275D70, ('\u{94B1}', false)), // East Asian ideograph
    (0x4B5832, ('\u{89B3}', false)), // East Asian ideograph
    (0x225D71, ('\u{7577}', false)), // East Asian ideograph
    (0x6F4A58, ('\u{AE85}', false)), // Korean hangul
    (0x275D72, ('\u{9521}', false)), // East Asian ideograph
    (0x22343C, ('\u{649D}', false)), // East Asian ideograph
    (0x275D73, ('\u{94EE}', false)), // East Asian ideograph
    (0x6F5B65, ('\u{D2F4}', false)), // Korean hangul
    (0x275D74, ('\u{5F55}', false)), // East Asian ideograph
    (0x6F5722, ('\u{C7A3}', false)), // Korean hangul
    (0x69724E, ('\u{9BF2}', false)), // East Asian ideograph
    (0x215D75, ('\u{9310}', false)), // East Asian ideograph
    (0x234948, ('\u{95BC}', false)), // East Asian ideograph
    (0x4B325F, ('\u{50BB}', false)), // East Asian ideograph
    (0x215D76, ('\u{9326}', false)), // East Asian ideograph
    (0x6F5835, ('\u{C9DA}', false)), // Korean hangul
    (0x692153, ('\u{3009}', false)), // Ideographic greater than sign
    (0x215D77, ('\u{934D}', false)), // East Asian ideograph
    (0x2D632D, ('\u{4E80}', false)), // East Asian ideograph
    (0x215D78, ('\u{9382}', false)), // East Asian ideograph
    (0x274C7B, ('\u{53D1}', false)), // East Asian ideograph
    (0x6F5B66, ('\u{D2F8}', false)), // Korean hangul
    (0x225D79, ('\u{757D}', false)), // East Asian ideograph
    (0x6F5432, ('\u{C2FC}', false)), // Korean hangul
    (0x224824, ('\u{6CD8}', false)), // East Asian ideograph
    (0x4B5C77, ('\u{9139}', false)), // East Asian ideograph
    (0x235D7A, ('\u{9EB0}', false)), // East Asian ideograph
    (0x6F5D7B, ('\u{D790}', false)), // Korean hangul
    (0x27606D, ('\u{9974}', false)), // East Asian ideograph
    (0x224826, ('\u{6CC6}', false)), // East Asian ideograph
    (0x6F575B, ('\u{C8A0}', false)), // Korean hangul
    (0x275D7C, ('\u{9505}', false)), // East Asian ideograph
    (0x2E5452, ('\u{71FE}', false)), // East Asian ideograph
    (0x234827, ('\u{93F4}', false)), // East Asian ideograph
    (0x275D7D, ('\u{951A}', false)), // East Asian ideograph
    (0x234828, ('\u{9436}', false)), // East Asian ideograph
    (0x6F5B67, ('\u{D300}', false)), // Korean hangul
    (0x275D7E, ('\u{953E}', false)), // East Asian ideograph
    (0x224829, ('\u{6CE9}', false)), // East Asian ideograph
    (0x226764, ('\u{799D}', false)), // East Asian ideograph
    (0x23494A, ('\u{95CD}', false)), // East Asian ideograph
    (0x395E3D, ('\u{9295}', false)), // East Asian ideograph
    (0x6F4A5B, ('\u{AEBE}', false)), // Korean hangul
    (0x23482B, ('\u{943B}', false)), // East Asian ideograph
    (0x275946, ('\u{8BD1}', false)), // East Asian ideograph
    (0x22527C, ('\u{717B}', false)), // East Asian ideograph
    (0x23482D, ('\u{9424}', false)), // East Asian ideograph
    (0x6F5B68, ('\u{D301}', false)), // Korean hangul
    (0x6F5725, ('\u{C7A6}', false)), // Korean hangul
    (0x284E42, ('\u{6D4D}', false)), // East Asian ideograph
    (0x21482F, ('\u{6E34}', false)), // East Asian ideograph
    (0x6F523E, ('\u{BF1D}', false)), // Korean hangul
    (0x6F4A5C, ('\u{AEC0}', false)), // Korean hangul
    (0x234830, ('\u{9437}', false)), // East Asian ideograph
    (0x213122, ('\u{4F10}', false)), // East Asian ideograph
    (0x213123, ('\u{4F0F}', false)), // East Asian ideograph
    (0x213124, ('\u{4EF2}', false)), // East Asian ideograph
    (0x223125, ('\u{62F5}', false)), // East Asian ideograph
    (0x213126, ('\u{4EF3}', false)), // East Asian ideograph
    (0x213127, ('\u{4EF6}', false)), // East Asian ideograph
    (0x213128, ('\u{4EF0}', false)), // East Asian ideograph
    (0x23312A, ('\u{89B8}', false)), // East Asian ideograph
    (0x23312B, ('\u{89B7}', false)), // East Asian ideograph
    (0x23312C, ('\u{89B6}', false)), // East Asian ideograph
    (0x234832, ('\u{9440}', false)), // East Asian ideograph
    (0x21312E, ('\u{4F57}', false)), // East Asian ideograph
    (0x23312F, ('\u{89BC}', false)), // East Asian ideograph
    (0x213130, ('\u{4F5E}', false)), // East Asian ideograph
    (0x223131, ('\u{630C}', false)), // East Asian ideograph
    (0x233132, ('\u{89BF}', false)), // East Asian ideograph
    (0x213133, ('\u{4F55}', false)), // East Asian ideograph
    (0x213134, ('\u{4F30}', false)), // East Asian ideograph
    (0x213135, ('\u{4F50}', false)), // East Asian ideograph
    (0x213136, ('\u{4F51}', false)), // East Asian ideograph
    (0x223137, ('\u{62F6}', false)), // East Asian ideograph
    (0x213138, ('\u{4F48}', false)), // East Asian ideograph
    (0x213139, ('\u{4F46}', false)), // East Asian ideograph
    (0x22313A, ('\u{6331}', false)), // East Asian ideograph
    (0x23313B, ('\u{89D5}', false)), // East Asian ideograph
    (0x21313C, ('\u{4F54}', false)), // East Asian ideograph
    (0x21313D, ('\u{4F3C}', false)), // East Asian ideograph
    (0x21313E, ('\u{4F63}', false)), // East Asian ideograph
    (0x23313F, ('\u{89DA}', false)), // East Asian ideograph
    (0x213140, ('\u{4F60}', false)), // East Asian ideograph
    (0x213141, ('\u{4F2F}', false)), // East Asian ideograph
    (0x223142, ('\u{6345}', false)), // East Asian ideograph
    (0x233143, ('\u{89E5}', false)), // East Asian ideograph
    (0x223144, ('\u{6343}', false)), // East Asian ideograph
    (0x234836, ('\u{942D}', false)), // East Asian ideograph
    (0x213146, ('\u{4F6F}', false)), // East Asian ideograph
    (0x223147, ('\u{6353}', false)), // East Asian ideograph
    (0x223148, ('\u{6364}', false)), // East Asian ideograph
    (0x223149, ('\u{6336}', false)), // East Asian ideograph
    (0x22314A, ('\u{6344}', false)), // East Asian ideograph
    (0x224837, ('\u{6D1D}', false)), // East Asian ideograph
    (0x23314C, ('\u{89E9}', false)), // East Asian ideograph
    (0x23314D, ('\u{89EB}', false)), // East Asian ideograph
    (0x21314E, ('\u{4F8B}', false)), // East Asian ideograph
    (0x27314F, ('\u{4ED1}', false)), // East Asian ideograph
    (0x234838, ('\u{9431}', false)), // East Asian ideograph
    (0x213152, ('\u{4F7B}', false)), // East Asian ideograph
    (0x233153, ('\u{89ED}', false)), // East Asian ideograph
    (0x223154, ('\u{6339}', false)), // East Asian ideograph
    (0x213155, ('\u{4F8F}', false)), // East Asian ideograph
    (0x213156, ('\u{4F7E}', false)), // East Asian ideograph
    (0x213157, ('\u{4FE1}', false)), // East Asian ideograph
    (0x223158, ('\u{6357}', false)), // East Asian ideograph
    (0x213159, ('\u{4FB5}', false)), // East Asian ideograph
    (0x22315A, ('\u{633C}', false)), // East Asian ideograph
    (0x22315B, ('\u{6358}', false)), // East Asian ideograph
    (0x21315C, ('\u{4FDE}', false)), // East Asian ideograph
    (0x27315D, ('\u{4FA0}', false)), // East Asian ideograph
    (0x21315E, ('\u{4FCF}', false)), // East Asian ideograph
    (0x22315F, ('\u{6354}', false)), // East Asian ideograph
    (0x213160, ('\u{4FDA}', false)), // East Asian ideograph
    (0x213161, ('\u{4FDD}', false)), // East Asian ideograph
    (0x213162, ('\u{4FC3}', false)), // East Asian ideograph
    (0x213163, ('\u{4FD8}', false)), // East Asian ideograph
    (0x233164, ('\u{89F7}', false)), // East Asian ideograph
    (0x213165, ('\u{4FCA}', false)), // East Asian ideograph
    (0x213166, ('\u{4FAE}', false)), // East Asian ideograph
    (0x213167, ('\u{4FD0}', false)), // East Asian ideograph
    (0x223168, ('\u{637D}', false)), // East Asian ideograph
    (0x273169, ('\u{7CFB}', false)), // East Asian ideograph (duplicate simplified)
    (0x22316A, ('\u{63B6}', false)), // East Asian ideograph
    (0x22316B, ('\u{6382}', false)), // East Asian ideograph
    (0x27316C, ('\u{4ED3}', false)), // East Asian ideograph
    (0x23316D, ('\u{8A07}', false)), // East Asian ideograph
    (0x22316E, ('\u{639F}', false)), // East Asian ideograph
    (0x21483D, ('\u{6E9D}', false)), // East Asian ideograph
    (0x233170, ('\u{8A0F}', false)), // East Asian ideograph
    (0x233171, ('\u{8A11}', false)), // East Asian ideograph
    (0x233172, ('\u{8A12}', false)), // East Asian ideograph
    (0x233173, ('\u{8A0D}', false)), // East Asian ideograph
    (0x213174, ('\u{4FF8}', false)), // East Asian ideograph
    (0x213175, ('\u{5028}', false)), // East Asian ideograph
    (0x213176, ('\u{5014}', false)), // East Asian ideograph
    (0x213177, ('\u{5016}', false)), // East Asian ideograph
    (0x213178, ('\u{5029}', false)), // East Asian ideograph
    (0x223179, ('\u{6381}', false)), // East Asian ideograph
    (0x23317A, ('\u{8A27}', false)), // East Asian ideograph
    (0x22317B, ('\u{6397}', false)), // East Asian ideograph
    (0x21317C, ('\u{503C}', false)), // East Asian ideograph
    (0x23317D, ('\u{8A29}', false)), // East Asian ideograph
    (0x21317E, ('\u{4FFA}', false)), // East Asian ideograph
    (0x234840, ('\u{9445}', false)), // East Asian ideograph
    (0x274841, ('\u{6C85}', false)), // East Asian ideograph
    (0x6F5B6C, ('\u{D30C}', false)), // Korean hangul
    (0x234842, ('\u{9450}', false)), // East Asian ideograph
    (0x273266, ('\u{4F18}', false)), // East Asian ideograph
    (0x4B513B, ('\u{7CF8}', false)), // East Asian ideograph
    (0x6F4B6D, ('\u{B0EC}', false)), // Korean hangul
    (0x274844, ('\u{6E7F}', false)), // East Asian ideograph
    (0x2D4845, ('\u{6E29}', false)), // East Asian ideograph
    (0x4B4846, ('\u{78C6}', false)), // East Asian ideograph
    (0x226F69, ('\u{7CC5}', false)), // East Asian ideograph
    (0x274848, ('\u{6CA7}', false)), // East Asian ideograph
    (0x4B3622, ('\u{8C18}', false)), // East Asian ideograph
    (0x6F4A61, ('\u{AED0}', false)), // Korean hangul
    (0x273733, ('\u{5578}', false)), // East Asian ideograph
    (0x23484A, ('\u{944A}', false)), // East Asian ideograph
    (0x27484B, ('\u{51C6}', false)), // East Asian ideograph
    (0x6F5B6E, ('\u{D30E}', false)), // Korean hangul
    (0x2F3639, ('\u{8C7C}', false)), // East Asian ideograph
    (0x4B484C, ('\u{6F91}', false)), // East Asian ideograph
    (0x22484D, ('\u{6D26}', false)), // East Asian ideograph
    (0x22484E, ('\u{6D27}', false)), // East Asian ideograph
    (0x294375, ('\u{9514}', false)), // East Asian ideograph
    (0x22484F, ('\u{6D0F}', false)), // East Asian ideograph
    (0x224850, ('\u{6D0A}', false)), // East Asian ideograph
    (0x2D4466, ('\u{6973}', false)), // East Asian ideograph
    (0x224851, ('\u{6D3F}', false)), // East Asian ideograph
    (0x226329, ('\u{77BE}', false)), // East Asian ideograph
    (0x234853, ('\u{9466}', false)), // East Asian ideograph
    (0x47594E, ('\u{9C3A}', false)), // East Asian ideograph
    (0x274854, ('\u{6E0D}', false)), // East Asian ideograph
    (0x514E5B, ('\u{9271}', false)), // East Asian ideograph
    (0x274855, ('\u{6DA8}', false)), // East Asian ideograph
    (0x6F5B70, ('\u{D314}', false)), // Korean hangul
    (0x274842, ('\u{706D}', false)), // East Asian ideograph
    (0x6F572D, ('\u{C7BF}', false)), // Korean hangul
    (0x284C41, ('\u{6CA4}', false)), // East Asian ideograph
    (0x234560, ('\u{93BE}', false)), // East Asian ideograph (not in Unicode)
    (0x29243A, ('\u{83BC}', false)), // East Asian ideograph
    (0x274857, ('\u{6C49}', false)), // East Asian ideograph
    (0x273B79, ('\u{5C9B}', false)), // East Asian ideograph
    (0x234858, ('\u{9462}', false)), // East Asian ideograph
    (0x2F252D, ('\u{6A22}', false)), // East Asian ideograph
    (0x6F575D, ('\u{C8A8}', false)), // Korean hangul
    (0x3F4621, ('\u{9A69}', false)), // East Asian ideograph
    (0x274859, ('\u{6D9F}', false)), // East Asian ideograph
    (0x286622, ('\u{7857}', false)), // East Asian ideograph
    (0x22485A, ('\u{6D07}', false)), // East Asian ideograph
    (0x4B4D7B, ('\u{77D7}', false)), // East Asian ideograph (variant of 214D7B which maps to 77D7)
    (0x6F5B71, ('\u{D31C}', false)), // Korean hangul
    (0x213221, ('\u{5018}', false)), // East Asian ideograph
    (0x213222, ('\u{4FF1}', false)), // East Asian ideograph
    (0x22485B, ('\u{6D04}', false)), // East Asian ideograph
    (0x273224, ('\u{4E2A}', false)), // East Asian ideograph
    (0x213225, ('\u{5019}', false)), // East Asian ideograph
    (0x273226, ('\u{4F25}', false)), // East Asian ideograph
    (0x223227, ('\u{638E}', false)), // East Asian ideograph
    (0x233228, ('\u{8A4A}', false)), // East Asian ideograph
    (0x22485C, ('\u{6CDA}', false)), // East Asian ideograph
    (0x23322A, ('\u{8A4E}', false)), // East Asian ideograph
    (0x21322B, ('\u{4FFE}', false)), // East Asian ideograph
    (0x21322C, ('\u{502A}', false)), // East Asian ideograph
    (0x27322D, ('\u{4F26}', false)), // East Asian ideograph
    (0x27322E, ('\u{4EC3}', false)), // East Asian ideograph (duplicate simplified)
    (0x22322F, ('\u{6375}', false)), // East Asian ideograph
    (0x223230, ('\u{63AF}', false)), // East Asian ideograph
    (0x213231, ('\u{5047}', false)), // East Asian ideograph
    (0x213232, ('\u{505A}', false)), // East Asian ideograph
    (0x273233, ('\u{4F1F}', false)), // East Asian ideograph
    (0x213234, ('\u{5043}', false)), // East Asian ideograph
    (0x23485E, ('\u{945E}', false)), // East Asian ideograph
    (0x213236, ('\u{5076}', false)), // East Asian ideograph
    (0x213237, ('\u{504E}', false)), // East Asian ideograph
    (0x223238, ('\u{63B0}', false)), // East Asian ideograph
    (0x223239, ('\u{63AE}', false)), // East Asian ideograph
    (0x22323A, ('\u{637C}', false)), // East Asian ideograph
    (0x27485F, ('\u{6EDE}', false)), // East Asian ideograph
    (0x21323C, ('\u{5077}', false)), // East Asian ideograph
    (0x22323D, ('\u{63AD}', false)), // East Asian ideograph
    (0x27323E, ('\u{5BB6}', false)), // East Asian ideograph
    (0x21323F, ('\u{5085}', false)), // East Asian ideograph
    (0x273240, ('\u{5907}', false)), // East Asian ideograph
    (0x224860, ('\u{6D2E}', false)), // East Asian ideograph
    (0x233242, ('\u{8A45}', false)), // East Asian ideograph
    (0x273243, ('\u{4F27}', false)), // East Asian ideograph
    (0x273244, ('\u{4F1E}', false)), // East Asian ideograph
    (0x213245, ('\u{50AD}', false)), // East Asian ideograph
    (0x273246, ('\u{4F20}', false)), // East Asian ideograph
    (0x224861, ('\u{6D35}', false)), // East Asian ideograph
    (0x213248, ('\u{50B2}', false)), // East Asian ideograph
    (0x273249, ('\u{4EC5}', false)), // East Asian ideograph
    (0x27324A, ('\u{503E}', false)), // East Asian ideograph
    (0x21324B, ('\u{50AC}', false)), // East Asian ideograph
    (0x27324C, ('\u{4F24}', false)), // East Asian ideograph
    (0x224862, ('\u{6D3A}', false)), // East Asian ideograph
    (0x21324E, ('\u{50E7}', false)), // East Asian ideograph
    (0x22324F, ('\u{63BD}', false)), // East Asian ideograph
    (0x223250, ('\u{63C3}', false)), // East Asian ideograph
    (0x273251, ('\u{4FA5}', false)), // East Asian ideograph
    (0x223252, ('\u{63F5}', false)), // East Asian ideograph
    (0x213253, ('\u{50ED}', false)), // East Asian ideograph
    (0x213254, ('\u{50DA}', false)), // East Asian ideograph
    (0x273255, ('\u{4EC6}', false)), // East Asian ideograph
    (0x273256, ('\u{4F2A}', false)), // East Asian ideograph
    (0x273257, ('\u{8C61}', false)), // East Asian ideograph
    (0x273258, ('\u{4FA8}', false)), // East Asian ideograph
    (0x233259, ('\u{8A82}', false)), // East Asian ideograph
    (0x27325A, ('\u{4EBF}', false)), // East Asian ideograph
    (0x22325B, ('\u{63E0}', false)), // East Asian ideograph
    (0x22325C, ('\u{63D5}', false)), // East Asian ideograph
    (0x23325D, ('\u{8A84}', false)), // East Asian ideograph
    (0x23325E, ('\u{8A75}', false)), // East Asian ideograph
    (0x274865, ('\u{6E14}', false)), // East Asian ideograph
    (0x273260, ('\u{4FA9}', false)), // East Asian ideograph
    (0x273261, ('\u{4FED}', false)), // East Asian ideograph
    (0x273262, ('\u{50A7}', false)), // East Asian ideograph
    (0x273263, ('\u{5C3D}', false)), // East Asian ideograph (duplicate simplified)
    (0x213264, ('\u{5112}', false)), // East Asian ideograph
    (0x273265, ('\u{4FE6}', false)), // East Asian ideograph
    (0x223266, ('\u{63C5}', false)), // East Asian ideograph (not in Unicode)
    (0x213267, ('\u{511F}', false)), // East Asian ideograph
    (0x213268, ('\u{5121}', false)), // East Asian ideograph
    (0x273269, ('\u{50A8}', false)), // East Asian ideograph
    (0x21326A, ('\u{5137}', false)), // East Asian ideograph
    (0x27326B, ('\u{4FE8}', false)), // East Asian ideograph
    (0x21326C, ('\u{5140}', false)), // East Asian ideograph
    (0x21326D, ('\u{5143}', false)), // East Asian ideograph
    (0x21326E, ('\u{5141}', false)), // East Asian ideograph
    (0x23326F, ('\u{8A96}', false)), // East Asian ideograph
    (0x213270, ('\u{5144}', false)), // East Asian ideograph
    (0x233271, ('\u{8A9A}', false)), // East Asian ideograph
    (0x213272, ('\u{5149}', false)), // East Asian ideograph
    (0x273273, ('\u{51F6}', false)), // East Asian ideograph
    (0x213274, ('\u{5148}', false)), // East Asian ideograph
    (0x274C3C, ('\u{8FED}', false)), // East Asian ideograph
    (0x223276, ('\u{63D1}', false)), // East Asian ideograph (not in Unicode)
    (0x234869, ('\u{946D}', false)), // East Asian ideograph
    (0x213278, ('\u{5155}', false)), // East Asian ideograph
    (0x223279, ('\u{63C4}', false)), // East Asian ideograph
    (0x27327A, ('\u{513F}', false)), // East Asian ideograph
    (0x27327B, ('\u{5156}', false)), // East Asian ideograph
    (0x21327C, ('\u{515C}', false)), // East Asian ideograph
    (0x22486A, ('\u{6D2B}', false)), // East Asian ideograph
    (0x22327E, ('\u{6412}', false)), // East Asian ideograph
    (0x2D4F7C, ('\u{7B5E}', false)), // East Asian ideograph
    (0x22486B, ('\u{6D11}', false)), // East Asian ideograph
    (0x27486C, ('\u{6CFC}', false)), // East Asian ideograph
    (0x6F5838, ('\u{C9DC}', false)), // Korean hangul
    (0x21304D, ('\u{4E82}', false)), // East Asian ideograph
    (0x22486D, ('\u{6D24}', false)), // East Asian ideograph
    (0x6F4E5C, ('\u{B760}', false)), // Korean hangul
    (0x235C65, ('\u{9DEF}', false)), // East Asian ideograph
    (0x293B3F, ('\u{8F7A}', false)), // East Asian ideograph
    (0x27486E, ('\u{6DA7}', false)), // East Asian ideograph
    (0x6F5B75, ('\u{D321}', false)), // Korean hangul
    (0x27486F, ('\u{6D01}', false)), // East Asian ideograph
    (0x6F4870, ('\u{AC1B}', false)), // Korean hangul
    (0x6F4C49, ('\u{B214}', false)), // Korean hangul
    (0x234871, ('\u{9477}', false)), // East Asian ideograph
    (0x275954, ('\u{5C82}', false)), // East Asian ideograph
    (0x2F252E, ('\u{8507}', false)), // East Asian ideograph
    (0x6F4872, ('\u{AC1D}', false)), // Korean hangul
    (0x2D315F, ('\u{4FA3}', false)), // East Asian ideograph
    (0x235622, ('\u{9B35}', false)), // East Asian ideograph
    (0x6F5B76, ('\u{D325}', false)), // Korean hangul
    (0x2D4874, ('\u{6F5C}', false)), // East Asian ideograph
    (0x6F4875, ('\u{AC24}', false)), // Korean hangul
    (0x29457A, ('\u{9550}', false)), // East Asian ideograph
    (0x6F4876, ('\u{AC2C}', false)), // Korean hangul
    (0x227321, ('\u{7E35}', false)), // East Asian ideograph
    (0x224877, ('\u{6DA5}', false)), // East Asian ideograph
    (0x213322, ('\u{5168}', false)), // East Asian ideograph
    (0x4B5361, ('\u{89D2}', false)), // East Asian ideograph (duplicate simplified)
    (0x274878, ('\u{6E83}', false)), // East Asian ideograph
    (0x213323, ('\u{5169}', false)), // East Asian ideograph
    (0x455E21, ('\u{953A}', false)), // East Asian ideograph
    (0x293271, ('\u{8BEE}', false)), // East Asian ideograph
    (0x213324, ('\u{516B}', false)), // East Asian ideograph
    (0x22455B, ('\u{6BD6}', false)), // East Asian ideograph
    (0x6F487A, ('\u{AC31}', false)), // Korean hangul
    (0x213325, ('\u{516D}', false)), // East Asian ideograph
    (0x23487B, ('\u{9482}', false)), // East Asian ideograph
    (0x27373D, ('\u{5480}', false)), // East Asian ideograph
    (0x27487C, ('\u{6D53}', false)), // East Asian ideograph
    (0x213327, ('\u{516C}', false)), // East Asian ideograph
    (0x6F5D56, ('\u{D6E0}', false)), // Korean hangul
    (0x22487D, ('\u{6D92}', false)), // East Asian ideograph
    (0x217328, ('\u{568C}', false)), // East Asian ideograph
    (0x6F487E, ('\u{AC54}', false)), // Korean hangul
    (0x213329, ('\u{5175}', false)), // East Asian ideograph
    (0x215F33, ('\u{9694}', false)), // East Asian ideograph
    (0x276225, ('\u{9CCF}', false)), // East Asian ideograph
    (0x2D332A, ('\u{4E0C}', false)), // East Asian ideograph
    (0x2D3E2B, ('\u{6060}', false)), // East Asian ideograph
    (0x21332B, ('\u{5177}', false)), // East Asian ideograph
    (0x4B5F62, ('\u{7668}', false)), // East Asian ideograph
    (0x3F4629, ('\u{4E97}', false)), // East Asian ideograph
    (0x513051, ('\u{8CAE}', false)), // East Asian ideograph
    (0x22332C, ('\u{6424}', false)), // East Asian ideograph
    (0x274C3B, ('\u{7574}', false)), // East Asian ideograph
    (0x23463C, ('\u{9389}', false)), // East Asian ideograph
    (0x22732D, ('\u{7E52}', false)), // East Asian ideograph
    (0x22534A, ('\u{719B}', false)), // East Asian ideograph
    (0x6F5736, ('\u{C804}', false)), // Korean hangul
    (0x215F34, ('\u{9699}', false)), // East Asian ideograph
    (0x21332F, ('\u{5189}', false)), // East Asian ideograph
    (0x6F4A6D, ('\u{AF3F}', false)), // Korean hangul
    (0x275958, ('\u{4E30}', false)), // East Asian ideograph
    (0x233321, ('\u{8ABE}', false)), // East Asian ideograph
    (0x223322, ('\u{6410}', false)), // East Asian ideograph
    (0x273323, ('\u{4E24}', false)), // East Asian ideograph
    (0x223324, ('\u{6434}', false)), // East Asian ideograph
    (0x233325, ('\u{8ACF}', false)), // East Asian ideograph
    (0x213326, ('\u{516E}', false)), // East Asian ideograph
    (0x233327, ('\u{8AC6}', false)), // East Asian ideograph
    (0x213328, ('\u{5171}', false)), // East Asian ideograph
    (0x223329, ('\u{641B}', false)), // East Asian ideograph
    (0x21332A, ('\u{5176}', false)), // East Asian ideograph
    (0x22332B, ('\u{6420}', false)), // East Asian ideograph
    (0x23332C, ('\u{8AD1}', false)), // East Asian ideograph
    (0x23332D, ('\u{8AD3}', false)), // East Asian ideograph
    (0x21332E, ('\u{5180}', false)), // East Asian ideograph
    (0x22332F, ('\u{6426}', false)), // East Asian ideograph
    (0x213330, ('\u{518C}', false)), // East Asian ideograph
    (0x233331, ('\u{8AAF}', false)), // East Asian ideograph
    (0x213332, ('\u{5192}', false)), // East Asian ideograph
    (0x233333, ('\u{8AD4}', false)), // East Asian ideograph
    (0x213334, ('\u{5195}', false)), // East Asian ideograph
    (0x213335, ('\u{6700}', false)), // East Asian ideograph
    (0x213336, ('\u{5197}', false)), // East Asian ideograph
    (0x213337, ('\u{51A0}', false)), // East Asian ideograph
    (0x233338, ('\u{8AB9}', false)), // East Asian ideograph
    (0x223339, ('\u{3013}', false)), // East Asian ideograph (not found in unified han)
    (0x23333B, ('\u{8ADB}', false)), // East Asian ideograph
    (0x21333C, ('\u{51B0}', false)), // East Asian ideograph
    (0x22333D, ('\u{6421}', false)), // East Asian ideograph
    (0x21333E, ('\u{51B7}', false)), // East Asian ideograph
    (0x23333F, ('\u{8AD0}', false)), // East Asian ideograph
    (0x233340, ('\u{8AD7}', false)), // East Asian ideograph
    (0x213341, ('\u{51CC}', false)), // East Asian ideograph
    (0x233344, ('\u{8AF3}', false)), // East Asian ideograph
    (0x233336, ('\u{8ACD}', false)), // East Asian ideograph
    (0x213347, ('\u{51F0}', false)), // East Asian ideograph
    (0x273348, ('\u{51EF}', false)), // East Asian ideograph
    (0x233349, ('\u{8B4C}', false)), // East Asian ideograph
    (0x223337, ('\u{6418}', false)), // East Asian ideograph
    (0x22334C, ('\u{6409}', false)), // East Asian ideograph
    (0x21334D, ('\u{51F8}', false)), // East Asian ideograph
    (0x23334E, ('\u{8AF6}', false)), // East Asian ideograph
    (0x21334F, ('\u{5200}', false)), // East Asian ideograph
    (0x213350, ('\u{5201}', false)), // East Asian ideograph
    (0x223338, ('\u{640E}', false)), // East Asian ideograph
    (0x213352, ('\u{5207}', false)), // East Asian ideograph
    (0x223353, ('\u{6440}', false)), // East Asian ideograph
    (0x213354, ('\u{5208}', false)), // East Asian ideograph
    (0x213355, ('\u{520A}', false)), // East Asian ideograph
    (0x233356, ('\u{8B03}', false)), // East Asian ideograph
    (0x233357, ('\u{8AE4}', false)), // East Asian ideograph
    (0x233359, ('\u{8B14}', false)), // East Asian ideograph
    (0x21335A, ('\u{5224}', false)), // East Asian ideograph
    (0x21335B, ('\u{5225}', false)), // East Asian ideograph
    (0x235749, ('\u{9B86}', false)), // East Asian ideograph
    (0x23335D, ('\u{8AFC}', false)), // East Asian ideograph
    (0x21335E, ('\u{5229}', false)), // East Asian ideograph
    (0x21335F, ('\u{5238}', false)), // East Asian ideograph
    (0x213360, ('\u{523B}', false)), // East Asian ideograph
    (0x213361, ('\u{5237}', false)), // East Asian ideograph
    (0x233362, ('\u{8ADE}', false)), // East Asian ideograph
    (0x233363, ('\u{8AE1}', false)), // East Asian ideograph
    (0x233364, ('\u{8B07}', false)), // East Asian ideograph
    (0x2D537E, ('\u{81C8}', false)), // East Asian ideograph
    (0x213366, ('\u{5241}', false)), // East Asian ideograph
    (0x213367, ('\u{5239}', false)), // East Asian ideograph
    (0x223368, ('\u{645B}', false)), // East Asian ideograph
    (0x213369, ('\u{524D}', false)), // East Asian ideograph
    (0x22336A, ('\u{644F}', false)), // East Asian ideograph
    (0x27336B, ('\u{514B}', false)), // East Asian ideograph
    (0x21336C, ('\u{524A}', false)), // East Asian ideograph
    (0x27336D, ('\u{5219}', false)), // East Asian ideograph
    (0x21336E, ('\u{525C}', false)), // East Asian ideograph
    (0x22336F, ('\u{6476}', false)), // East Asian ideograph
    (0x273370, ('\u{521A}', false)), // East Asian ideograph
    (0x215F37, ('\u{969B}', false)), // East Asian ideograph
    (0x213372, ('\u{525D}', false)), // East Asian ideograph
    (0x233373, ('\u{8B16}', false)), // East Asian ideograph
    (0x213374, ('\u{526F}', false)), // East Asian ideograph
    (0x213375, ('\u{5272}', false)), // East Asian ideograph
    (0x223376, ('\u{6474}', false)), // East Asian ideograph
    (0x213377, ('\u{5269}', false)), // East Asian ideograph
    (0x273378, ('\u{521B}', false)), // East Asian ideograph
    (0x233379, ('\u{8B06}', false)), // East Asian ideograph
    (0x23337A, ('\u{8B05}', false)), // East Asian ideograph
    (0x21337B, ('\u{527F}', false)), // East Asian ideograph
    (0x27337C, ('\u{5212}', false)), // East Asian ideograph
    (0x21337D, ('\u{5288}', false)), // East Asian ideograph
    (0x27337E, ('\u{5267}', false)), // East Asian ideograph
    (0x213340, ('\u{51CD}', false)), // East Asian ideograph
    (0x217341, ('\u{569C}', false)), // East Asian ideograph
    (0x27484F, ('\u{6CAA}', false)), // East Asian ideograph
    (0x276226, ('\u{9CA2}', false)), // East Asian ideograph
    (0x234960, ('\u{95D5}', false)), // East Asian ideograph
    (0x6F773B, ('\u{C6FD}', false)), // Korean hangul
    (0x213344, ('\u{51DC}', false)), // East Asian ideograph
    (0x23337C, ('\u{8B0F}', false)), // East Asian ideograph
    (0x223345, ('\u{6441}', false)), // East Asian ideograph
    (0x6F5B7E, ('\u{D33C}', false)), // Korean hangul
    (0x282E79, ('\u{6079}', false)), // East Asian ideograph
    (0x213E40, ('\u{6046}', false)), // East Asian ideograph (variant of 4B3E40 which maps to 6046)
    (0x286E68, ('\u{7B3E}', false)), // East Asian ideograph
    (0x22677B, ('\u{79B4}', false)), // East Asian ideograph
    (0x224562, ('\u{6BDC}', false)), // East Asian ideograph
    (0x213348, ('\u{51F1}', false)), // East Asian ideograph
    (0x695626, ('\u{4E62}', false)), // East Asian ideograph
    (0x6F4A72, ('\u{AF49}', false)), // Korean hangul
    (0x213349, ('\u{51F3}', false)), // East Asian ideograph
    (0x513057, ('\u{4E98}', false)), // East Asian ideograph
    (0x21334B, ('\u{51FA}', false)), // East Asian ideograph
    (0x6F573C, ('\u{C814}', false)), // Korean hangul
    (0x23334C, ('\u{8ADD}', false)), // East Asian ideograph
    (0x224563, ('\u{6BDD}', false)), // East Asian ideograph
    (0x234962, ('\u{95D2}', false)), // East Asian ideograph
    (0x6F2525, ('\u{3160}', false)), // Korean hangul
    (0x4C3A33, ('\u{80AD}', false)), // East Asian ideograph (variant of 2E3A33 which maps to 80AD)
    (0x276072, ('\u{9975}', false)), // East Asian ideograph
    (0x27595E, ('\u{4E88}', false)), // East Asian ideograph
    (0x21734E, ('\u{56AC}', false)), // East Asian ideograph
    (0x273745, ('\u{55B7}', false)), // East Asian ideograph
    (0x23334F, ('\u{8AF4}', false)), // East Asian ideograph
    (0x233350, ('\u{8AF5}', false)), // East Asian ideograph
    (0x6F573D, ('\u{C815}', false)), // Korean hangul
    (0x213351, ('\u{5203}', false)), // East Asian ideograph
    (0x395050, ('\u{7BED}', false)), // East Asian ideograph
    (0x22633A, ('\u{77D1}', false)), // East Asian ideograph
    (0x287352, ('\u{7F34}', false)), // East Asian ideograph
    (0x6F4B71, ('\u{B10B}', false)), // Korean hangul
    (0x6F4A74, ('\u{AF58}', false)), // Korean hangul
    (0x233353, ('\u{8ADF}', false)), // East Asian ideograph
    (0x4B3354, ('\u{82C5}', false)), // East Asian ideograph
    (0x4B3355, ('\u{520B}', false)), // East Asian ideograph
    (0x6F573E, ('\u{C816}', false)), // Korean hangul
    (0x213356, ('\u{5211}', false)), // East Asian ideograph
    (0x224565, ('\u{6BDF}', false)), // East Asian ideograph
    (0x213357, ('\u{5217}', false)), // East Asian ideograph
    (0x2D5C48, ('\u{9013}', false)), // East Asian ideograph
    (0x273747, ('\u{54DD}', false)), // East Asian ideograph
    (0x213359, ('\u{520E}', false)), // East Asian ideograph
    (0x6F5D58, ('\u{D6E8}', false)), // Korean hangul
    (0x27735A, ('\u{55BE}', false)), // East Asian ideograph
    (0x2D4F41, ('\u{4E69}', false)), // East Asian ideograph
    (0x4B5D2B, ('\u{9162}', false)), // East Asian ideograph
    (0x273421, ('\u{5251}', false)), // East Asian ideograph
    (0x213422, ('\u{5289}', false)), // East Asian ideograph
    (0x273423, ('\u{5242}', false)), // East Asian ideograph
    (0x223424, ('\u{6464}', false)), // East Asian ideograph
    (0x213425, ('\u{529F}', false)), // East Asian ideograph
    (0x213426, ('\u{52A0}', false)), // East Asian ideograph
    (0x223427, ('\u{6482}', false)), // East Asian ideograph
    (0x223428, ('\u{645E}', false)), // East Asian ideograph
    (0x21335C, ('\u{5220}', false)), // East Asian ideograph
    (0x21342A, ('\u{52AC}', false)), // East Asian ideograph
    (0x21342B, ('\u{52AA}', false)), // East Asian ideograph
    (0x22342C, ('\u{647B}', false)), // East Asian ideograph
    (0x23342D, ('\u{8B26}', false)), // East Asian ideograph
    (0x22342E, ('\u{645C}', false)), // East Asian ideograph
    (0x27342F, ('\u{52B2}', false)), // East Asian ideograph
    (0x233430, ('\u{8B33}', false)), // East Asian ideograph
    (0x213431, ('\u{52D8}', false)), // East Asian ideograph
    (0x21305B, ('\u{4EA1}', false)), // East Asian ideograph
    (0x273433, ('\u{52A1}', false)), // East Asian ideograph
    (0x273434, ('\u{52A8}', false)), // East Asian ideograph
    (0x273435, ('\u{52B3}', false)), // East Asian ideograph
    (0x273436, ('\u{52CB}', false)), // East Asian ideograph
    (0x213437, ('\u{52DD}', false)), // East Asian ideograph
    (0x273438, ('\u{52BF}', false)), // East Asian ideograph
    (0x213439, ('\u{52E4}', false)), // East Asian ideograph
    (0x23343A, ('\u{8B29}', false)), // East Asian ideograph
    (0x2D335F, ('\u{52B5}', false)), // East Asian ideograph
    (0x27343C, ('\u{52B1}', false)), // East Asian ideograph
    (0x27343D, ('\u{529D}', false)), // East Asian ideograph
    (0x21343E, ('\u{52FB}', false)), // East Asian ideograph
    (0x22343F, ('\u{6499}', false)), // East Asian ideograph
    (0x213440, ('\u{52FF}', false)), // East Asian ideograph
    (0x217360, ('\u{56C5}', false)), // East Asian ideograph
    (0x233442, ('\u{8B48}', false)), // East Asian ideograph
    (0x215F3E, ('\u{96BB}', false)), // East Asian ideograph
    (0x213444, ('\u{530D}', false)), // East Asian ideograph
    (0x234966, ('\u{95DA}', false)), // East Asian ideograph
    (0x213446, ('\u{530F}', false)), // East Asian ideograph
    (0x213447, ('\u{5315}', false)), // East Asian ideograph
    (0x213448, ('\u{5316}', false)), // East Asian ideograph
    (0x213449, ('\u{5317}', false)), // East Asian ideograph
    (0x23344A, ('\u{8B46}', false)), // East Asian ideograph
    (0x21344B, ('\u{53F5}', false)), // East Asian ideograph
    (0x21344C, ('\u{531D}', false)), // East Asian ideograph
    (0x22344D, ('\u{6496}', false)), // East Asian ideograph
    (0x21344E, ('\u{5320}', false)), // East Asian ideograph
    (0x21344F, ('\u{5323}', false)), // East Asian ideograph
    (0x213450, ('\u{532A}', false)), // East Asian ideograph
    (0x273451, ('\u{6C47}', false)), // East Asian ideograph
    (0x273452, ('\u{532E}', false)), // East Asian ideograph
    (0x213363, ('\u{523A}', false)), // East Asian ideograph
    (0x213454, ('\u{533E}', false)), // East Asian ideograph
    (0x273455, ('\u{533A}', false)), // East Asian ideograph
    (0x213456, ('\u{533F}', false)), // East Asian ideograph
    (0x213457, ('\u{5341}', false)), // East Asian ideograph
    (0x213458, ('\u{5343}', false)), // East Asian ideograph
    (0x213459, ('\u{5345}', false)), // East Asian ideograph
    (0x21345A, ('\u{5348}', false)), // East Asian ideograph
    (0x22345B, ('\u{64B6}', false)), // East Asian ideograph
    (0x21345C, ('\u{534A}', false)), // East Asian ideograph
    (0x21345D, ('\u{5349}', false)), // East Asian ideograph (variant of 2D345D which maps to 5349)
    (0x6F5741, ('\u{C820}', false)), // Korean hangul
    (0x27345F, ('\u{5346}', false)), // East Asian ideograph
    (0x273460, ('\u{534F}', false)), // East Asian ideograph
    (0x213461, ('\u{5353}', false)), // East Asian ideograph
    (0x223462, ('\u{649F}', false)), // East Asian ideograph
    (0x213463, ('\u{5357}', false)), // East Asian ideograph
    (0x213464, ('\u{535A}', false)), // East Asian ideograph
    (0x223465, ('\u{64A7}', false)), // East Asian ideograph (not in Unicode)
    (0x213466, ('\u{535E}', false)), // East Asian ideograph
    (0x213467, ('\u{5361}', false)), // East Asian ideograph
    (0x233468, ('\u{8B6B}', false)), // East Asian ideograph
    (0x213469, ('\u{5366}', false)), // East Asian ideograph
    (0x22346A, ('\u{64D7}', false)), // East Asian ideograph
    (0x21346B, ('\u{536E}', false)), // East Asian ideograph
    (0x21346C, ('\u{5370}', false)), // East Asian ideograph
    (0x21346D, ('\u{5371}', false)), // East Asian ideograph
    (0x21346E, ('\u{537D}', false)), // East Asian ideograph
    (0x21346F, ('\u{5375}', false)), // East Asian ideograph
    (0x233470, ('\u{8B78}', false)), // East Asian ideograph
    (0x213368, ('\u{5243}', false)), // East Asian ideograph
    (0x213473, ('\u{537B}', false)), // East Asian ideograph
    (0x223474, ('\u{64BE}', false)), // East Asian ideograph
    (0x223475, ('\u{64D0}', false)), // East Asian ideograph
    (0x213476, ('\u{539A}', false)), // East Asian ideograph
    (0x213477, ('\u{539D}', false)), // East Asian ideograph
    (0x213478, ('\u{539F}', false)), // East Asian ideograph
    (0x233479, ('\u{8B81}', false)), // East Asian ideograph
    (0x27347A, ('\u{538C}', false)), // East Asian ideograph
    (0x27347B, ('\u{5389}', false)), // East Asian ideograph
    (0x21347C, ('\u{53BB}', false)), // East Asian ideograph
    (0x27347D, ('\u{53C2}', false)), // East Asian ideograph
    (0x21347E, ('\u{53C8}', false)), // East Asian ideograph
    (0x334968, ('\u{7133}', false)), // East Asian ideograph
    (0x23336B, ('\u{8B0C}', false)), // East Asian ideograph
    (0x6F4B72, ('\u{B10C}', false)), // Korean hangul
    (0x6F4A79, ('\u{AF79}', false)), // Korean hangul
    (0x22336C, ('\u{646B}', false)), // East Asian ideograph
    (0x225676, ('\u{72EB}', false)), // East Asian ideograph
    (0x29583E, ('\u{9CCA}', false)), // East Asian ideograph
    (0x21736D, ('\u{56DD}', false)), // East Asian ideograph
    (0x2D4F45, ('\u{9834}', false)), // East Asian ideograph
    (0x274858, ('\u{6EE1}', false)), // East Asian ideograph
    (0x6F5743, ('\u{C82C}', false)), // Korean hangul
    (0x23336F, ('\u{8B1C}', false)), // East Asian ideograph
    (0x234969, ('\u{95DE}', false)), // East Asian ideograph
    (0x694677, ('\u{5302}', false)), // East Asian ideograph
    (0x217370, ('\u{56DF}', false)), // East Asian ideograph
    (0x6F4A7A, ('\u{AF80}', false)), // Korean hangul
    (0x213371, ('\u{5254}', false)), // East Asian ideograph
    (0x333377, ('\u{5270}', false)), // East Asian ideograph
    (0x2D3372, ('\u{5265}', false)), // East Asian ideograph
    (0x6F5D59, ('\u{D6F0}', false)), // Korean hangul
    (0x6F502A, ('\u{BA53}', false)), // Korean hangul
    (0x696F27, ('\u{933B}', false)), // East Asian ideograph
    (0x295C65, ('\u{9E69}', false)), // East Asian ideograph
    (0x213373, ('\u{526A}', false)), // East Asian ideograph
    (0x395E2F, ('\u{5277}', false)), // East Asian ideograph
    (0x287374, ('\u{7F35}', false)), // East Asian ideograph
    (0x225F3C, ('\u{760A}', false)), // East Asian ideograph
    (0x23496A, ('\u{95E0}', false)), // East Asian ideograph
    (0x217375, ('\u{56EB}', false)), // East Asian ideograph
    (0x334527, ('\u{6918}', false)), // East Asian ideograph
    (0x273376, ('\u{5240}', false)), // East Asian ideograph
    (0x6F516A, ('\u{BE1C}', false)), // Korean hangul
    (0x275E21, ('\u{949F}', false)), // East Asian ideograph
    (0x2D3377, ('\u{8CF8}', false)), // East Asian ideograph
    (0x215E22, ('\u{9318}', false)), // East Asian ideograph
    (0x27615F, ('\u{53D1}', false)), // East Asian ideograph (duplicate simplified)
    (0x233378, ('\u{8B0B}', false)), // East Asian ideograph
    (0x215E23, ('\u{936C}', false)), // East Asian ideograph
    (0x27485A, ('\u{6E10}', false)), // East Asian ideograph
    (0x275E24, ('\u{953B}', false)), // East Asian ideograph
    (0x21337A, ('\u{527D}', false)), // East Asian ideograph
    (0x225E25, ('\u{7583}', false)), // East Asian ideograph
    (0x6F583C, ('\u{C9E4}', false)), // Korean hangul
    (0x22337B, ('\u{6473}', false)), // East Asian ideograph
    (0x27374E, ('\u{549B}', false)), // East Asian ideograph
    (0x2D5E26, ('\u{7194}', false)), // East Asian ideograph
    (0x293F4C, ('\u{90D3}', false)), // East Asian ideograph
    (0x21337C, ('\u{5283}', false)), // East Asian ideograph
    (0x215E27, ('\u{93AE}', false)), // East Asian ideograph
    (0x335635, ('\u{85AC}', false)), // East Asian ideograph
    (0x3F3F24, ('\u{614E}', false)), // East Asian ideograph
    (0x23337D, ('\u{8B10}', false)), // East Asian ideograph
    (0x215E28, ('\u{9396}', false)), // East Asian ideograph
    (0x6F5746, ('\u{C838}', false)), // Korean hangul
    (0x21337E, ('\u{5287}', false)), // East Asian ideograph
    (0x275E29, ('\u{94A8}', false)), // East Asian ideograph
    (0x6F4C4D, ('\u{B233}', false)), // Korean hangul
    (0x215E2A, ('\u{93B3}', false)), // East Asian ideograph
    (0x215E2B, ('\u{93E1}', false)), // East Asian ideograph
    (0x213062, ('\u{4EAD}', false)), // East Asian ideograph
    (0x692564, ('\u{30E4}', false)), // Katakana letter YA
    (0x223461, ('\u{6498}', false)), // East Asian ideograph
    (0x29516D, ('\u{9991}', false)), // East Asian ideograph
    (0x215E2C, ('\u{93D1}', false)), // East Asian ideograph
    (0x225E2D, ('\u{7592}', false)), // East Asian ideograph
    (0x4C4D63, ('\u{6F99}', false)), // East Asian ideograph
    (0x6F5747, ('\u{C83C}', false)), // Korean hangul
    (0x215E2E, ('\u{93C3}', false)), // East Asian ideograph
    (0x213D2C, ('\u{5EE0}', false)), // East Asian ideograph
    (0x275E2F, ('\u{94F2}', false)), // East Asian ideograph
    (0x2D6056, ('\u{980B}', false)), // East Asian ideograph
    (0x6F4A7E, ('\u{AF95}', false)), // Korean hangul
    (0x275969, ('\u{8D1E}', false)), // East Asian ideograph
    (0x215E30, ('\u{93D7}', false)), // East Asian ideograph
    (0x213522, ('\u{53CB}', false)), // East Asian ideograph
    (0x233523, ('\u{8B8B}', false)), // East Asian ideograph
    (0x213524, ('\u{53CD}', false)), // East Asian ideograph
    (0x213525, ('\u{53D6}', false)), // East Asian ideograph
    (0x233526, ('\u{8B87}', false)), // East Asian ideograph
    (0x225E31, ('\u{7595}', false)), // East Asian ideograph
    (0x213528, ('\u{53DB}', false)), // East Asian ideograph
    (0x213529, ('\u{53DF}', false)), // East Asian ideograph
    (0x22352A, ('\u{64EF}', false)), // East Asian ideograph
    (0x27352B, ('\u{4E1B}', false)), // East Asian ideograph
    (0x21352C, ('\u{53E3}', false)), // East Asian ideograph
    (0x22352D, ('\u{64E1}', false)), // East Asian ideograph
    (0x22352E, ('\u{64E5}', false)), // East Asian ideograph
    (0x224873, ('\u{6D63}', false)), // East Asian ideograph
    (0x213530, ('\u{53EF}', false)), // East Asian ideograph
    (0x213531, ('\u{53E9}', false)), // East Asian ideograph
    (0x213532, ('\u{53F3}', false)), // East Asian ideograph
    (0x223533, ('\u{64E2}', false)), // East Asian ideograph
    (0x213534, ('\u{53E8}', false)), // East Asian ideograph
    (0x213535, ('\u{53E6}', false)), // East Asian ideograph
    (0x223536, ('\u{64ED}', false)), // East Asian ideograph
    (0x233537, ('\u{8B9C}', false)), // East Asian ideograph
    (0x223538, ('\u{64E4}', false)), // East Asian ideograph
    (0x275E34, ('\u{9542}', false)), // East Asian ideograph
    (0x21353A, ('\u{53F1}', false)), // East Asian ideograph
    (0x21353B, ('\u{53ED}', false)), // East Asian ideograph
    (0x21353C, ('\u{53EA}', false)), // East Asian ideograph
    (0x23353D, ('\u{8C3A}', false)), // East Asian ideograph
    (0x235E35, ('\u{9EC6}', false)), // East Asian ideograph
    (0x213540, ('\u{5409}', false)), // East Asian ideograph
    (0x213541, ('\u{5410}', false)), // East Asian ideograph
    (0x213542, ('\u{540F}', false)), // East Asian ideograph
    (0x235A7B, ('\u{9D59}', false)), // East Asian ideograph
    (0x233544, ('\u{8C40}', false)), // East Asian ideograph
    (0x233545, ('\u{8C42}', false)), // East Asian ideograph
    (0x213546, ('\u{5404}', false)), // East Asian ideograph
    (0x213547, ('\u{5403}', false)), // East Asian ideograph
    (0x213548, ('\u{5412}', false)), // East Asian ideograph
    (0x2D7164, ('\u{55D4}', false)), // East Asian ideograph (variant of 217164 which maps to 55D4)
    (0x21354A, ('\u{5406}', false)), // East Asian ideograph (not in Unicode)
    (0x23354B, ('\u{8C47}', false)), // East Asian ideograph
    (0x21354D, ('\u{542D}', false)), // East Asian ideograph
    (0x21354E, ('\u{541D}', false)), // East Asian ideograph
    (0x21354F, ('\u{541E}', false)), // East Asian ideograph
    (0x213550, ('\u{541B}', false)), // East Asian ideograph
    (0x213551, ('\u{544E}', false)), // East Asian ideograph
    (0x233552, ('\u{8C55}', false)), // East Asian ideograph
    (0x23496F, ('\u{95E5}', false)), // East Asian ideograph
    (0x233554, ('\u{8C57}', false)), // East Asian ideograph
    (0x213555, ('\u{5431}', false)), // East Asian ideograph
    (0x233556, ('\u{8C5D}', false)), // East Asian ideograph
    (0x213557, ('\u{543C}', false)), // East Asian ideograph
    (0x213558, ('\u{5443}', false)), // East Asian ideograph
    (0x213559, ('\u{5426}', false)), // East Asian ideograph
    (0x21355A, ('\u{5420}', false)), // East Asian ideograph
    (0x22355B, ('\u{6516}', false)), // East Asian ideograph
    (0x23355C, ('\u{86C3}', false)), // East Asian ideograph
    (0x21355D, ('\u{5435}', false)), // East Asian ideograph
    (0x234E26, ('\u{97B5}', false)), // East Asian ideograph
    (0x21355F, ('\u{544A}', false)), // East Asian ideograph
    (0x213560, ('\u{5448}', false)), // East Asian ideograph
    (0x223561, ('\u{651B}', false)), // East Asian ideograph
    (0x213562, ('\u{5438}', false)), // East Asian ideograph
    (0x233563, ('\u{8C68}', false)), // East Asian ideograph
    (0x213564, ('\u{5442}', false)), // East Asian ideograph
    (0x233565, ('\u{8C6D}', false)), // East Asian ideograph
    (0x213566, ('\u{541F}', false)), // East Asian ideograph
    (0x213567, ('\u{5429}', false)), // East Asian ideograph
    (0x213568, ('\u{5473}', false)), // East Asian ideograph
    (0x223569, ('\u{6527}', false)), // East Asian ideograph
    (0x21356A, ('\u{5475}', false)), // East Asian ideograph
    (0x21356B, ('\u{5495}', false)), // East Asian ideograph
    (0x21356C, ('\u{5478}', false)), // East Asian ideograph
    (0x22356D, ('\u{6522}', false)), // East Asian ideograph
    (0x21356E, ('\u{5477}', false)), // East Asian ideograph
    (0x22356F, ('\u{6529}', false)), // East Asian ideograph
    (0x213571, ('\u{5492}', false)), // East Asian ideograph
    (0x223572, ('\u{6525}', false)), // East Asian ideograph
    (0x213573, ('\u{547C}', false)), // East Asian ideograph
    (0x233574, ('\u{8C76}', false)), // East Asian ideograph
    (0x225E3E, ('\u{75BA}', false)), // East Asian ideograph
    (0x213576, ('\u{548B}', false)), // East Asian ideograph
    (0x213577, ('\u{548C}', false)), // East Asian ideograph
    (0x213578, ('\u{5490}', false)), // East Asian ideograph
    (0x213579, ('\u{547D}', false)), // East Asian ideograph
    (0x21357A, ('\u{5476}', false)), // East Asian ideograph
    (0x23357B, ('\u{8C78}', false)), // East Asian ideograph
    (0x22357C, ('\u{6541}', false)), // East Asian ideograph
    (0x23357D, ('\u{8C7B}', false)), // East Asian ideograph
    (0x21357E, ('\u{54A9}', false)), // East Asian ideograph
    (0x275E40, ('\u{956F}', false)), // East Asian ideograph
    (0x23563A, ('\u{9B48}', false)), // East Asian ideograph
    (0x333421, ('\u{91FC}', false)), // East Asian ideograph
    (0x6F574B, ('\u{C874}', false)), // Korean hangul
    (0x234566, ('\u{9355}', false)), // East Asian ideograph
    (0x235E42, ('\u{9ECC}', false)), // East Asian ideograph
    (0x213D30, ('\u{5EF7}', false)), // East Asian ideograph
    (0x6F4C4E, ('\u{B234}', false)), // Korean hangul
    (0x225E43, ('\u{75B0}', false)), // East Asian ideograph
    (0x225E44, ('\u{75C3}', false)), // East Asian ideograph
    (0x27552A, ('\u{82CB}', false)), // East Asian ideograph
    (0x6F5763, ('\u{C8CC}', false)), // Korean hangul
    (0x275E45, ('\u{94C4}', false)), // East Asian ideograph
    (0x225E46, ('\u{75BF}', false)), // East Asian ideograph
    (0x2D5927, ('\u{8ACC}', false)), // East Asian ideograph
    (0x234C38, ('\u{971B}', false)), // East Asian ideograph
    (0x6F574C, ('\u{C878}', false)), // Korean hangul
    (0x225E47, ('\u{75B4}', false)), // East Asian ideograph
    (0x6F5D29, ('\u{D5F5}', false)), // Korean hangul
    (0x6F4B74, ('\u{B110}', false)), // Korean hangul
    (0x23452F, ('\u{9342}', false)), // East Asian ideograph
    (0x27596E, ('\u{8D2F}', false)), // East Asian ideograph
    (0x273755, ('\u{5411}', false)), // East Asian ideograph
    (0x275E49, ('\u{9523}', false)), // East Asian ideograph
    (0x215E4A, ('\u{947D}', false)), // East Asian ideograph
    (0x23563C, ('\u{9B4E}', false)), // East Asian ideograph
    (0x333423, ('\u{5264}', false)), // East Asian ideograph
    (0x275E4B, ('\u{51FF}', false)), // East Asian ideograph
    (0x6F574D, ('\u{C87A}', false)), // Korean hangul
    (0x235E4C, ('\u{9ED3}', false)), // East Asian ideograph
    (0x275E4D, ('\u{95E8}', false)), // East Asian ideograph
    (0x34492F, ('\u{6D34}', false)), // East Asian ideograph
    (0x225E4E, ('\u{75C1}', false)), // East Asian ideograph
    (0x277745, ('\u{57D9}', false)), // East Asian ideograph
    (0x275E4F, ('\u{95EA}', false)), // East Asian ideograph
    (0x4C6F43, ('\u{7CCD}', false)), // East Asian ideograph
    (0x225E50, ('\u{75B1}', false)), // East Asian ideograph
    (0x274863, ('\u{6D46}', false)), // East Asian ideograph
    (0x6F574E, ('\u{C880}', false)), // Korean hangul
    (0x284C62, ('\u{988D}', false)), // East Asian ideograph
    (0x225E51, ('\u{75C4}', false)), // East Asian ideograph
    (0x213D33, ('\u{5EFA}', false)), // East Asian ideograph
    (0x275E52, ('\u{95F0}', false)), // East Asian ideograph
    (0x275970, ('\u{8D2A}', false)), // East Asian ideograph
    (0x215E53, ('\u{958B}', false)), // East Asian ideograph
    (0x275E54, ('\u{95F2}', false)), // East Asian ideograph
    (0x23563E, ('\u{9B4D}', false)), // East Asian ideograph
    (0x215E55, ('\u{9593}', false)), // East Asian ideograph
    (0x274864, ('\u{6E17}', false)), // East Asian ideograph
    (0x6F574F, ('\u{C881}', false)), // Korean hangul
    (0x6F4D29, ('\u{B355}', false)), // Korean hangul
    (0x235E57, ('\u{9EE3}', false)), // East Asian ideograph
    (0x275971, ('\u{8D2B}', false)), // East Asian ideograph
    (0x225E58, ('\u{75CD}', false)), // East Asian ideograph
    (0x215E59, ('\u{95A8}', false)), // East Asian ideograph
    (0x275E5A, ('\u{95FD}', false)), // East Asian ideograph
    (0x6F7727, ('\u{ADD5}', false)), // Korean hangul
    (0x213621, ('\u{54AA}', false)), // East Asian ideograph
    (0x213622, ('\u{54A8}', false)), // East Asian ideograph
    (0x213623, ('\u{54AC}', false)), // East Asian ideograph
    (0x213624, ('\u{54C0}', false)), // East Asian ideograph
    (0x213625, ('\u{54B3}', false)), // East Asian ideograph
    (0x213626, ('\u{54A6}', false)), // East Asian ideograph
    (0x213627, ('\u{54AB}', false)), // East Asian ideograph
    (0x213628, ('\u{54C7}', false)), // East Asian ideograph
    (0x213629, ('\u{54C9}', false)), // East Asian ideograph
    (0x21362A, ('\u{54C4}', false)), // East Asian ideograph
    (0x21362B, ('\u{54C2}', false)), // East Asian ideograph
    (0x22362C, ('\u{6538}', false)), // East Asian ideograph
    (0x21362D, ('\u{54C1}', false)), // East Asian ideograph
    (0x23362E, ('\u{8C88}', false)), // East Asian ideograph
    (0x21362F, ('\u{54CE}', false)), // East Asian ideograph
    (0x213630, ('\u{54B1}', false)), // East Asian ideograph
    (0x213631, ('\u{54BB}', false)), // East Asian ideograph
    (0x213632, ('\u{54AF}', false)), // East Asian ideograph
    (0x213633, ('\u{54C8}', false)), // East Asian ideograph
    (0x223634, ('\u{6542}', false)), // East Asian ideograph
    (0x225E5E, ('\u{75CC}', false)), // East Asian ideograph
    (0x213636, ('\u{5510}', false)), // East Asian ideograph
    (0x213637, ('\u{54EA}', false)), // East Asian ideograph
    (0x213638, ('\u{5514}', false)), // East Asian ideograph
    (0x233639, ('\u{8C94}', false)), // East Asian ideograph
    (0x21363A, ('\u{54E5}', false)), // East Asian ideograph
    (0x225E5F, ('\u{75D0}', false)), // East Asian ideograph
    (0x21363C, ('\u{54F2}', false)), // East Asian ideograph
    (0x21363D, ('\u{54E8}', false)), // East Asian ideograph
    (0x21363E, ('\u{54E1}', false)), // East Asian ideograph
    (0x22363F, ('\u{6555}', false)), // East Asian ideograph
    (0x213640, ('\u{54ED}', false)), // East Asian ideograph
    (0x233641, ('\u{8C9B}', false)), // East Asian ideograph
    (0x213642, ('\u{5509}', false)), // East Asian ideograph
    (0x213643, ('\u{54E6}', false)), // East Asian ideograph
    (0x233644, ('\u{8CA4}', false)), // East Asian ideograph
    (0x223645, ('\u{6567}', false)), // East Asian ideograph
    (0x213646, ('\u{5546}', false)), // East Asian ideograph
    (0x223647, ('\u{6561}', false)), // East Asian ideograph
    (0x213648, ('\u{554F}', false)), // East Asian ideograph
    (0x273649, ('\u{54D1}', false)), // East Asian ideograph
    (0x21364A, ('\u{5566}', false)), // East Asian ideograph
    (0x21364B, ('\u{556A}', false)), // East Asian ideograph
    (0x21364C, ('\u{554A}', false)), // East Asian ideograph
    (0x21364D, ('\u{5544}', false)), // East Asian ideograph
    (0x21364E, ('\u{555C}', false)), // East Asian ideograph
    (0x22364F, ('\u{656D}', false)), // East Asian ideograph
    (0x213650, ('\u{5543}', false)), // East Asian ideograph
    (0x213651, ('\u{552C}', false)), // East Asian ideograph
    (0x213652, ('\u{5561}', false)), // East Asian ideograph
    (0x233653, ('\u{8CB9}', false)), // East Asian ideograph
    (0x223654, ('\u{657A}', false)), // East Asian ideograph
    (0x213655, ('\u{5555}', false)), // East Asian ideograph
    (0x213656, ('\u{552F}', false)), // East Asian ideograph
    (0x233657, ('\u{8CCD}', false)), // East Asian ideograph
    (0x213658, ('\u{5564}', false)), // East Asian ideograph
    (0x213659, ('\u{5538}', false)), // East Asian ideograph
    (0x21365A, ('\u{55A7}', false)), // East Asian ideograph
    (0x21365B, ('\u{5580}', false)), // East Asian ideograph
    (0x21365C, ('\u{557B}', false)), // East Asian ideograph
    (0x21365D, ('\u{557C}', false)), // East Asian ideograph
    (0x21365E, ('\u{5527}', false)), // East Asian ideograph
    (0x21365F, ('\u{5594}', false)), // East Asian ideograph
    (0x213660, ('\u{5587}', false)), // East Asian ideograph
    (0x213661, ('\u{559C}', false)), // East Asian ideograph
    (0x213662, ('\u{558B}', false)), // East Asian ideograph
    (0x273663, ('\u{4E27}', false)), // East Asian ideograph
    (0x213664, ('\u{55B3}', false)), // East Asian ideograph
    (0x225E66, ('\u{75E1}', false)), // East Asian ideograph
    (0x213666, ('\u{5583}', false)), // East Asian ideograph
    (0x213667, ('\u{55B1}', false)), // East Asian ideograph
    (0x273668, ('\u{5355}', false)), // East Asian ideograph
    (0x213669, ('\u{5582}', false)), // East Asian ideograph
    (0x21366A, ('\u{559F}', false)), // East Asian ideograph
    (0x225E67, ('\u{75E6}', false)), // East Asian ideograph
    (0x21366C, ('\u{5598}', false)), // East Asian ideograph
    (0x21366D, ('\u{559A}', false)), // East Asian ideograph
    (0x22366E, ('\u{658C}', false)), // East Asian ideograph
    (0x27366F, ('\u{4E54}', false)), // East Asian ideograph
    (0x223670, ('\u{6592}', false)), // East Asian ideograph
    (0x213671, ('\u{55B2}', false)), // East Asian ideograph
    (0x233672, ('\u{8CDD}', false)), // East Asian ideograph
    (0x213673, ('\u{55E8}', false)), // East Asian ideograph
    (0x233674, ('\u{8CD9}', false)), // East Asian ideograph
    (0x223675, ('\u{659B}', false)), // East Asian ideograph
    (0x213676, ('\u{55DC}', false)), // East Asian ideograph
    (0x223677, ('\u{659D}', false)), // East Asian ideograph
    (0x213678, ('\u{55C7}', false)), // East Asian ideograph
    (0x213679, ('\u{55D3}', false)), // East Asian ideograph
    (0x21367A, ('\u{55CE}', false)), // East Asian ideograph
    (0x21367B, ('\u{55E3}', false)), // East Asian ideograph
    (0x23367C, ('\u{8CF5}', false)), // East Asian ideograph
    (0x21367D, ('\u{55E4}', false)), // East Asian ideograph
    (0x23367E, ('\u{8CFB}', false)), // East Asian ideograph
    (0x232760, ('\u{8600}', false)), // East Asian ideograph
    (0x275E6B, ('\u{8F9F}', false)), // East Asian ideograph (duplicate simplified)
    (0x285424, ('\u{70E8}', false)), // East Asian ideograph
    (0x27375C, ('\u{556D}', false)), // East Asian ideograph
    (0x4B5E6C, ('\u{961D}', false)), // East Asian ideograph (duplicate simplified)
    (0x293F5A, ('\u{90E7}', false)), // East Asian ideograph
    (0x235E6F, ('\u{9EF6}', false)), // East Asian ideograph
    (0x4B5422, ('\u{81D3}', false)), // East Asian ideograph
    (0x6F583F, ('\u{C9EC}', false)), // Korean hangul
    (0x27375D, ('\u{55EB}', false)), // East Asian ideograph
    (0x225E71, ('\u{75E4}', false)), // East Asian ideograph
    (0x225E72, ('\u{75E0}', false)), // East Asian ideograph
    (0x4B4759, ('\u{6D99}', false)), // East Asian ideograph
    (0x6F4B66, ('\u{B0C7}', false)), // Korean hangul
    (0x225E73, ('\u{75D7}', false)), // East Asian ideograph
    (0x6F5755, ('\u{C88D}', false)), // Korean hangul
    (0x235E74, ('\u{9EF9}', false)), // East Asian ideograph
    (0x275977, ('\u{8D3A}', false)), // East Asian ideograph
    (0x27375E, ('\u{56A3}', false)), // East Asian ideograph
    (0x235E76, ('\u{9EFB}', false)), // East Asian ideograph
    (0x274921, ('\u{6CFD}', false)), // East Asian ideograph
    (0x293F5C, ('\u{90AC}', false)), // East Asian ideograph
    (0x2D616A, ('\u{6B1D}', false)), // East Asian ideograph
    (0x215E77, ('\u{964C}', false)), // East Asian ideograph
    (0x274922, ('\u{6D4A}', false)), // East Asian ideograph
    (0x6F5756, ('\u{C890}', false)), // Korean hangul
    (0x6F4924, ('\u{AC74}', false)), // Korean hangul
    (0x6F4B76, ('\u{B118}', false)), // Korean hangul
    (0x215E7A, ('\u{9662}', false)), // East Asian ideograph
    (0x224925, ('\u{6D6D}', false)), // East Asian ideograph
    (0x275978, ('\u{8D35}', false)), // East Asian ideograph
    (0x235E7B, ('\u{9EFE}', false)), // East Asian ideograph (not in Unicode)
    (0x274926, ('\u{6D4E}', false)), // East Asian ideograph
    (0x215E7C, ('\u{965B}', false)), // East Asian ideograph
    (0x274927, ('\u{6CDE}', false)), // East Asian ideograph
    (0x235E7D, ('\u{9F02}', false)), // East Asian ideograph
    (0x274928, ('\u{6EE8}', false)), // East Asian ideograph
    (0x6F5757, ('\u{C894}', false)), // Korean hangul
    (0x215E7E, ('\u{965D}', false)), // East Asian ideograph
    (0x224929, ('\u{6D91}', false)), // East Asian ideograph
    (0x287065, ('\u{7EC2}', false)), // East Asian ideograph
    (0x2F4231, ('\u{8019}', false)), // Unrelated variant of EACC 215266 which maps to 8019
    (0x6F492A, ('\u{AC81}', false)), // Korean hangul
    (0x33492E, ('\u{6F81}', false)), // East Asian ideograph
    (0x27492B, ('\u{6EE5}', false)), // East Asian ideograph
    (0x33337B, ('\u{52E6}', false)), // East Asian ideograph
    (0x233871, ('\u{8DC2}', false)), // East Asian ideograph
    (0x22492C, ('\u{6D81}', false)), // East Asian ideograph
    (0x235647, ('\u{9B51}', false)), // East Asian ideograph
    (0x33463C, ('\u{6BBB}', false)), // East Asian ideograph
    (0x27492D, ('\u{6D9B}', false)), // East Asian ideograph
    (0x6F553A, ('\u{C587}', false)), // Korean hangul
    (0x27492E, ('\u{6DA9}', false)), // East Asian ideograph
    (0x22413C, ('\u{6A5A}', false)), // East Asian ideograph
    (0x2E492F, ('\u{6CD9}', false)), // East Asian ideograph
    (0x27597A, ('\u{4E70}', false)), // East Asian ideograph
    (0x213331, ('\u{518D}', false)), // East Asian ideograph
    (0x273761, ('\u{7F57}', false)), // East Asian ideograph (duplicate simplified)
    (0x213721, ('\u{55DA}', false)), // East Asian ideograph
    (0x223722, ('\u{65A8}', false)), // East Asian ideograph
    (0x223723, ('\u{65A6}', false)), // East Asian ideograph
    (0x213724, ('\u{5600}', false)), // East Asian ideograph
    (0x233725, ('\u{8D04}', false)), // East Asian ideograph
    (0x213726, ('\u{55FE}', false)), // East Asian ideograph
    (0x273727, ('\u{5567}', false)), // East Asian ideograph
    (0x213728, ('\u{55F7}', false)), // East Asian ideograph
    (0x213729, ('\u{5608}', false)), // East Asian ideograph
    (0x22372A, ('\u{65B6}', false)), // East Asian ideograph
    (0x21372B, ('\u{55FD}', false)), // East Asian ideograph
    (0x22372C, ('\u{65B8}', false)), // East Asian ideograph
    (0x23372D, ('\u{8D09}', false)), // East Asian ideograph
    (0x21372E, ('\u{5614}', false)), // East Asian ideograph
    (0x22372F, ('\u{65BF}', false)), // East Asian ideograph
    (0x273730, ('\u{5C1D}', false)), // East Asian ideograph
    (0x273731, ('\u{55BD}', false)), // East Asian ideograph
    (0x273732, ('\u{5520}', false)), // East Asian ideograph
    (0x213733, ('\u{562F}', false)), // East Asian ideograph
    (0x223734, ('\u{65C2}', false)), // East Asian ideograph
    (0x213735, ('\u{5636}', false)), // East Asian ideograph
    (0x213736, ('\u{5632}', false)), // East Asian ideograph
    (0x213737, ('\u{563B}', false)), // East Asian ideograph
    (0x213738, ('\u{5639}', false)), // East Asian ideograph
    (0x274934, ('\u{6E85}', false)), // East Asian ideograph
    (0x23373A, ('\u{8D10}', false)), // East Asian ideograph
    (0x22373B, ('\u{65D0}', false)), // East Asian ideograph
    (0x22373C, ('\u{65D2}', false)), // East Asian ideograph
    (0x21373D, ('\u{5634}', false)), // East Asian ideograph
    (0x23373E, ('\u{8D18}', false)), // East Asian ideograph
    (0x224935, ('\u{6DEF}', false)), // East Asian ideograph
    (0x213740, ('\u{5630}', false)), // East Asian ideograph
    (0x213741, ('\u{566B}', false)), // East Asian ideograph
    (0x213742, ('\u{5664}', false)), // East Asian ideograph
    (0x213743, ('\u{5669}', false)), // East Asian ideograph
    (0x223744, ('\u{65DB}', false)), // East Asian ideograph
    (0x213745, ('\u{5674}', false)), // East Asian ideograph
    (0x273746, ('\u{5F53}', false)), // East Asian ideograph (duplicate simplified)
    (0x213747, ('\u{5665}', false)), // East Asian ideograph
    (0x213748, ('\u{566A}', false)), // East Asian ideograph
    (0x213749, ('\u{5668}', false)), // East Asian ideograph
    (0x22374A, ('\u{65E1}', false)), // East Asian ideograph
    (0x27374B, ('\u{55F3}', false)), // East Asian ideograph
    (0x225A23, ('\u{7429}', false)), // East Asian ideograph
    (0x21374D, ('\u{566C}', false)), // East Asian ideograph
    (0x21374E, ('\u{5680}', false)), // East Asian ideograph
    (0x21374F, ('\u{568E}', false)), // East Asian ideograph
    (0x213750, ('\u{5685}', false)), // East Asian ideograph
    (0x214938, ('\u{701B}', false)), // East Asian ideograph
    (0x233752, ('\u{8D78}', false)), // East Asian ideograph
    (0x213753, ('\u{568F}', false)), // East Asian ideograph
    (0x223754, ('\u{65F4}', false)), // East Asian ideograph
    (0x213755, ('\u{56AE}', false)), // East Asian ideograph (variant of 453755 which maps to 56AE)
    (0x273756, ('\u{5499}', false)), // East Asian ideograph
    (0x224939, ('\u{6D7F}', false)), // East Asian ideograph
    (0x213758, ('\u{56A5}', false)), // East Asian ideograph
    (0x213759, ('\u{56B7}', false)), // East Asian ideograph
    (0x22375A, ('\u{6609}', false)), // East Asian ideograph
    (0x27375B, ('\u{5624}', false)), // East Asian ideograph
    (0x21375C, ('\u{56C0}', false)), // East Asian ideograph
    (0x21493A, ('\u{7028}', false)), // East Asian ideograph
    (0x22375E, ('\u{660A}', false)), // East Asian ideograph
    (0x21375F, ('\u{56BC}', false)), // East Asian ideograph
    (0x213760, ('\u{56CA}', false)), // East Asian ideograph
    (0x213761, ('\u{56C9}', false)), // East Asian ideograph
    (0x273762, ('\u{5453}', false)), // East Asian ideograph
    (0x22493B, ('\u{6D85}', false)), // East Asian ideograph
    (0x223764, ('\u{6603}', false)), // East Asian ideograph
    (0x213765, ('\u{56DB}', false)), // East Asian ideograph
    (0x213766, ('\u{56DA}', false)), // East Asian ideograph
    (0x213767, ('\u{56E0}', false)), // East Asian ideograph
    (0x213768, ('\u{56DE}', false)), // East Asian ideograph
    (0x21493C, ('\u{7015}', false)), // East Asian ideograph
    (0x22376A, ('\u{6611}', false)), // East Asian ideograph
    (0x22376B, ('\u{6615}', false)), // East Asian ideograph
    (0x21376C, ('\u{56FA}', false)), // East Asian ideograph
    (0x22376D, ('\u{6604}', false)), // East Asian ideograph
    (0x22376E, ('\u{6631}', false)), // East Asian ideograph
    (0x21376F, ('\u{570B}', false)), // East Asian ideograph
    (0x213770, ('\u{570D}', false)), // East Asian ideograph
    (0x233771, ('\u{8D94}', false)), // East Asian ideograph
    (0x223772, ('\u{6621}', false)), // East Asian ideograph
    (0x273773, ('\u{56E2}', false)), // East Asian ideograph
    (0x273774, ('\u{56FE}', false)), // East Asian ideograph
    (0x223775, ('\u{662C}', false)), // East Asian ideograph
    (0x223777, ('\u{6635}', false)), // East Asian ideograph
    (0x213778, ('\u{572F}', false)), // East Asian ideograph
    (0x213779, ('\u{5730}', false)), // East Asian ideograph
    (0x21377A, ('\u{5728}', false)), // East Asian ideograph
    (0x21377B, ('\u{5733}', false)), // East Asian ideograph
    (0x22377C, ('\u{661E}', false)), // East Asian ideograph
    (0x22377D, ('\u{663A}', false)), // East Asian ideograph
    (0x224940, ('\u{6D67}', false)), // East Asian ideograph
    (0x274941, ('\u{6D12}', false)), // East Asian ideograph
    (0x2E3144, ('\u{651F}', false)), // East Asian ideograph
    (0x274942, ('\u{6EE9}', false)), // East Asian ideograph
    (0x212A34, ('\u{E8E1}', false)), // EACC component character
    (0x214943, ('\u{7063}', false)), // East Asian ideograph
    (0x274944, ('\u{6EE6}', false)), // East Asian ideograph
    (0x4B5A3B, ('\u{8D08}', false)), // East Asian ideograph
    (0x4B4761, ('\u{6E05}', false)), // East Asian ideograph
    (0x213F21, ('\u{6148}', false)), // East Asian ideograph
    (0x224946, ('\u{6D60}', false)), // East Asian ideograph
    (0x2D4B35, ('\u{73C9}', false)), // East Asian ideograph
    (0x2D4947, ('\u{7AC8}', false)), // East Asian ideograph
    (0x394C2D, ('\u{7546}', false)), // East Asian ideograph
    (0x224948, ('\u{6D98}', false)), // East Asian ideograph
    (0x6F516F, ('\u{BE48}', false)), // Korean hangul
    (0x234949, ('\u{95BE}', false)), // East Asian ideograph
    (0x217068, ('\u{5530}', false)), // East Asian ideograph
    (0x295D3A, ('\u{9E73}', false)), // East Asian ideograph
    (0x27494A, ('\u{707E}', false)), // East Asian ideograph
    (0x225352, ('\u{71A0}', false)), // East Asian ideograph
    (0x234644, ('\u{93BD}', false)), // East Asian ideograph
    (0x22494B, ('\u{6D7C}', false)), // East Asian ideograph
    (0x6F575E, ('\u{C8AC}', false)), // Korean hangul
    (0x22494C, ('\u{6D70}', false)), // East Asian ideograph
    (0x21494D, ('\u{7092}', false)), // East Asian ideograph
    (0x23494E, ('\u{95BA}', false)), // East Asian ideograph
    (0x295D3B, ('\u{9E42}', false)), // East Asian ideograph
    (0x23494F, ('\u{95B6}', false)), // East Asian ideograph
    (0x2E3E3F, ('\u{7BA0}', false)), // East Asian ideograph
    (0x234950, ('\u{95BF}', false)), // East Asian ideograph
    (0x225278, ('\u{7178}', false)), // East Asian ideograph
    (0x274951, ('\u{4E3A}', false)), // East Asian ideograph
    (0x213D44, ('\u{5F29}', false)), // East Asian ideograph
    (0x334674, ('\u{76C5}', false)), // East Asian ideograph
    (0x234952, ('\u{95BD}', false)), // East Asian ideograph
    (0x6F4953, ('\u{ACF1}', false)), // Korean hangul
    (0x21392A, ('\u{592E}', false)), // East Asian ideograph
    (0x295D3C, ('\u{5364}', false)), // East Asian ideograph
    (0x2D4954, ('\u{70F1}', false)), // East Asian ideograph
    (0x6F4955, ('\u{ACF5}', false)), // Korean hangul
    (0x22526B, ('\u{7153}', false)), // East Asian ideograph
    (0x6F5760, ('\u{C8B8}', false)), // Korean hangul
    (0x2D4956, ('\u{70B0}', false)), // East Asian ideograph
    (0x213D45, ('\u{5F2D}', false)), // East Asian ideograph
    (0x4B5871, ('\u{8AA4}', false)), // East Asian ideograph
    (0x6F4B78, ('\u{B11B}', false)), // Korean hangul
    (0x6F4957, ('\u{ACFA}', false)), // Korean hangul
    (0x6F4958, ('\u{ACFC}', false)), // Korean hangul
    (0x284339, ('\u{680A}', false)), // East Asian ideograph
    (0x22746A, ('\u{7F7F}', false)), // East Asian ideograph
    (0x225251, ('\u{7146}', false)), // East Asian ideograph
    (0x234959, ('\u{95C9}', false)), // East Asian ideograph
    (0x22495A, ('\u{6DB4}', false)), // East Asian ideograph
    (0x6F5761, ('\u{C8C4}', false)), // Korean hangul
    (0x213821, ('\u{5740}', false)), // East Asian ideograph
    (0x233822, ('\u{8D96}', false)), // East Asian ideograph
    (0x213823, ('\u{574D}', false)), // East Asian ideograph
    (0x213824, ('\u{573E}', false)), // East Asian ideograph
    (0x213825, ('\u{574E}', false)), // East Asian ideograph
    (0x223827, ('\u{6633}', false)), // East Asian ideograph
    (0x223828, ('\u{662B}', false)), // East Asian ideograph
    (0x22495C, ('\u{6DAA}', false)), // East Asian ideograph
    (0x21382A, ('\u{5777}', false)), // East Asian ideograph
    (0x22382B, ('\u{6634}', false)), // East Asian ideograph
    (0x22382C, ('\u{6624}', false)), // East Asian ideograph
    (0x21382D, ('\u{5766}', false)), // East Asian ideograph
    (0x21382E, ('\u{5782}', false)), // East Asian ideograph
    (0x21495D, ('\u{70CF}', false)), // East Asian ideograph
    (0x213830, ('\u{57A0}', false)), // East Asian ideograph
    (0x223831, ('\u{6645}', false)), // East Asian ideograph
    (0x213832, ('\u{57A3}', false)), // East Asian ideograph
    (0x233833, ('\u{8DA6}', false)), // East Asian ideograph
    (0x213834, ('\u{57A2}', false)), // East Asian ideograph
    (0x213835, ('\u{57D4}', false)), // East Asian ideograph
    (0x213836, ('\u{57C2}', false)), // East Asian ideograph
    (0x213837, ('\u{57CE}', false)), // East Asian ideograph
    (0x213838, ('\u{57CB}', false)), // East Asian ideograph
    (0x213839, ('\u{57C3}', false)), // East Asian ideograph
    (0x21383A, ('\u{57F9}', false)), // East Asian ideograph
    (0x27383B, ('\u{6267}', false)), // East Asian ideograph
    (0x21383C, ('\u{57FA}', false)), // East Asian ideograph
    (0x22383D, ('\u{6665}', false)), // East Asian ideograph
    (0x22383E, ('\u{665C}', false)), // East Asian ideograph
    (0x22383F, ('\u{6661}', false)), // East Asian ideograph
    (0x213840, ('\u{5802}', false)), // East Asian ideograph
    (0x224960, ('\u{6DEC}', false)), // East Asian ideograph
    (0x213842, ('\u{57E4}', false)), // East Asian ideograph
    (0x213843, ('\u{57E0}', false)), // East Asian ideograph
    (0x273844, ('\u{62A5}', false)), // East Asian ideograph
    (0x273845, ('\u{5C27}', false)), // East Asian ideograph
    (0x213846, ('\u{5835}', false)), // East Asian ideograph
    (0x213847, ('\u{582A}', false)), // East Asian ideograph
    (0x223848, ('\u{665B}', false)), // East Asian ideograph
    (0x223849, ('\u{6659}', false)), // East Asian ideograph
    (0x22384A, ('\u{6667}', false)), // East Asian ideograph
    (0x21384B, ('\u{5821}', false)), // East Asian ideograph
    (0x21384C, ('\u{585E}', false)), // East Asian ideograph
    (0x22384D, ('\u{6657}', false)), // East Asian ideograph
    (0x275541, ('\u{83B1}', false)), // East Asian ideograph
    (0x21384F, ('\u{5851}', false)), // East Asian ideograph
    (0x213850, ('\u{586B}', false)), // East Asian ideograph
    (0x223851, ('\u{666C}', false)), // East Asian ideograph
    (0x233852, ('\u{8DAB}', false)), // East Asian ideograph
    (0x234963, ('\u{95D3}', false)), // East Asian ideograph
    (0x213854, ('\u{5854}', false)), // East Asian ideograph
    (0x273855, ('\u{575E}', false)), // East Asian ideograph
    (0x213856, ('\u{584A}', false)), // East Asian ideograph
    (0x213857, ('\u{5883}', false)), // East Asian ideograph
    (0x213858, ('\u{587E}', false)), // East Asian ideograph
    (0x234964, ('\u{95D1}', false)), // East Asian ideograph
    (0x23385A, ('\u{8DB0}', false)), // East Asian ideograph
    (0x27385B, ('\u{5811}', false)), // East Asian ideograph
    (0x216061, ('\u{98BC}', false)), // East Asian ideograph
    (0x21385D, ('\u{5893}', false)), // East Asian ideograph
    (0x21385E, ('\u{589E}', false)), // East Asian ideograph
    (0x234965, ('\u{95C3}', false)), // East Asian ideograph
    (0x273860, ('\u{575F}', false)), // East Asian ideograph
    (0x273861, ('\u{5760}', false)), // East Asian ideograph
    (0x273862, ('\u{5815}', false)), // East Asian ideograph
    (0x213863, ('\u{589F}', false)), // East Asian ideograph
    (0x273864, ('\u{575B}', false)), // East Asian ideograph
    (0x274966, ('\u{65E0}', false)), // East Asian ideograph
    (0x233866, ('\u{8DB2}', false)), // East Asian ideograph
    (0x273867, ('\u{57A6}', false)), // East Asian ideograph
    (0x223868, ('\u{6677}', false)), // East Asian ideograph
    (0x273869, ('\u{538B}', false)), // East Asian ideograph
    (0x21386A, ('\u{58D1}', false)), // East Asian ideograph
    (0x27386B, ('\u{5739}', false)), // East Asian ideograph
    (0x21386C, ('\u{58D8}', false)), // East Asian ideograph
    (0x27386D, ('\u{5784}', false)), // East Asian ideograph
    (0x23386E, ('\u{8DBC}', false)), // East Asian ideograph
    (0x27386F, ('\u{575C}', false)), // East Asian ideograph
    (0x233870, ('\u{8DB9}', false)), // East Asian ideograph
    (0x223871, ('\u{668C}', false)), // East Asian ideograph
    (0x233872, ('\u{8DC1}', false)), // East Asian ideograph
    (0x213873, ('\u{58EC}', false)), // East Asian ideograph
    (0x213874, ('\u{58EF}', false)), // East Asian ideograph
    (0x223875, ('\u{668B}', false)), // East Asian ideograph
    (0x273876, ('\u{58F6}', false)), // East Asian ideograph
    (0x273877, ('\u{5BFF}', false)), // East Asian ideograph
    (0x213878, ('\u{590F}', false)), // East Asian ideograph
    (0x223879, ('\u{6694}', false)), // East Asian ideograph
    (0x22387A, ('\u{668A}', false)), // East Asian ideograph
    (0x21387B, ('\u{5916}', false)), // East Asian ideograph
    (0x22387C, ('\u{6698}', false)), // East Asian ideograph
    (0x22387D, ('\u{668D}', false)), // East Asian ideograph
    (0x21387E, ('\u{591C}', false)), // East Asian ideograph
    (0x234547, ('\u{936A}', false)), // East Asian ideograph
    (0x22496B, ('\u{6DB7}', false)), // East Asian ideograph
    (0x2D3F54, ('\u{61D0}', false)), // East Asian ideograph
    (0x22496C, ('\u{6DE2}', false)), // East Asian ideograph
    (0x6F5768, ('\u{C8E4}', false)), // Korean hangul
    (0x4B393E, ('\u{5965}', false)), // East Asian ideograph
    (0x27496D, ('\u{70E6}', false)), // East Asian ideograph
    (0x22496E, ('\u{6DE9}', false)), // East Asian ideograph
    (0x6F5765, ('\u{C8D5}', false)), // Korean hangul
    (0x27496F, ('\u{7080}', false)), // East Asian ideograph
    (0x21763E, ('\u{5810}', false)), // East Asian ideograph
    (0x6F4B79, ('\u{B11C}', false)), // Korean hangul
    (0x4D5053, ('\u{98DA}', false)), // East Asian ideograph
    (0x6F4970, ('\u{AD70}', false)), // Korean hangul
    (0x224247, ('\u{6A90}', false)), // East Asian ideograph
    (0x224971, ('\u{6DF6}', false)), // East Asian ideograph
    (0x28433A, ('\u{69E0}', false)), // East Asian ideograph
    (0x295D42, ('\u{9E7E}', false)), // East Asian ideograph
    (0x234972, ('\u{95E4}', false)), // East Asian ideograph
    (0x6F7622, ('\u{3186}', false)), // Korean hangul
    (0x27487B, ('\u{6DC0}', false)), // East Asian ideograph
    (0x6F5766, ('\u{C8D7}', false)), // Korean hangul
    (0x215F64, ('\u{971C}', false)), // East Asian ideograph
    (0x213D4B, ('\u{5F48}', false)), // East Asian ideograph
    (0x274975, ('\u{6247}', false)), // East Asian ideograph
    (0x6F4976, ('\u{AD7D}', false)), // Korean hangul
    (0x213421, ('\u{528D}', false)), // East Asian ideograph
    (0x225257, ('\u{7160}', false)), // East Asian ideograph
    (0x4B4977, ('\u{7188}', false)), // East Asian ideograph
    (0x233422, ('\u{8B2B}', false)), // East Asian ideograph
    (0x6F4978, ('\u{AD81}', false)), // Korean hangul
    (0x4B4339, ('\u{6674}', false)), // East Asian ideograph
    (0x223423, ('\u{644E}', false)), // East Asian ideograph
    (0x223D6A, ('\u{6910}', false)), // East Asian ideograph
    (0x224979, ('\u{6E0F}', false)), // East Asian ideograph
    (0x213424, ('\u{529B}', false)), // East Asian ideograph
    (0x22414B, ('\u{6A5C}', false)), // East Asian ideograph
    (0x23497A, ('\u{961E}', false)), // East Asian ideograph
    (0x283671, ('\u{6593}', false)), // East Asian ideograph
    (0x23497B, ('\u{9624}', false)), // East Asian ideograph
    (0x23497C, ('\u{9622}', false)), // East Asian ideograph
    (0x213427, ('\u{52A3}', false)), // East Asian ideograph
    (0x4B5963, ('\u{734F}', false)), // East Asian ideograph
    (0x27497D, ('\u{70ED}', false)), // East Asian ideograph
    (0x213428, ('\u{52AB}', false)), // East Asian ideograph
    (0x27497E, ('\u{70EB}', false)), // East Asian ideograph
    (0x213429, ('\u{52A9}', false)), // East Asian ideograph
    (0x275D47, ('\u{9499}', false)), // East Asian ideograph
    (0x23342A, ('\u{8B37}', false)), // East Asian ideograph
    (0x273771, ('\u{56ED}', false)), // East Asian ideograph
    (0x6F5843, ('\u{C9F1}', false)), // Korean hangul
    (0x21342C, ('\u{52BE}', false)), // East Asian ideograph
    (0x2D4F6B, ('\u{7AF8}', false)), // East Asian ideograph
    (0x4C2962, ('\u{5F4D}', false)), // East Asian ideograph (variant of 222962 which maps to 5F4D)
    (0x21342D, ('\u{52C7}', false)), // East Asian ideograph
    (0x334C37, ('\u{8E6F}', false)), // East Asian ideograph
    (0x215F67, ('\u{9727}', false)), // East Asian ideograph
    (0x21742E, ('\u{5707}', false)), // East Asian ideograph
    (0x23454C, ('\u{934F}', false)), // East Asian ideograph
    (0x2D6078, ('\u{9920}', false)), // East Asian ideograph
    (0x21342F, ('\u{52C1}', false)), // East Asian ideograph
    (0x273772, ('\u{5706}', false)), // East Asian ideograph
    (0x233921, ('\u{8DCF}', false)), // East Asian ideograph
    (0x233922, ('\u{8DD6}', false)), // East Asian ideograph
    (0x273923, ('\u{4F19}', false)), // East Asian ideograph
    (0x223924, ('\u{7A25}', false)), // East Asian ideograph
    (0x213925, ('\u{5927}', false)), // East Asian ideograph
    (0x213926, ('\u{592A}', false)), // East Asian ideograph
    (0x233927, ('\u{8DD0}', false)), // East Asian ideograph
    (0x213928, ('\u{5929}', false)), // East Asian ideograph
    (0x213929, ('\u{592D}', false)), // East Asian ideograph
    (0x22392A, ('\u{66A0}', false)), // East Asian ideograph
    (0x23392B, ('\u{8DC5}', false)), // East Asian ideograph
    (0x21392C, ('\u{5937}', false)), // East Asian ideograph
    (0x217432, ('\u{5714}', false)), // East Asian ideograph
    (0x27392E, ('\u{5939}', false)), // East Asian ideograph
    (0x23392F, ('\u{8DE4}', false)), // East Asian ideograph
    (0x223930, ('\u{5C21}', false)), // East Asian ideograph
    (0x213931, ('\u{5948}', false)), // East Asian ideograph
    (0x223932, ('\u{669D}', false)), // East Asian ideograph
    (0x213433, ('\u{52D9}', false)), // East Asian ideograph
    (0x213934, ('\u{5955}', false)), // East Asian ideograph
    (0x233935, ('\u{8DEB}', false)), // East Asian ideograph
    (0x233936, ('\u{8DF4}', false)), // East Asian ideograph
    (0x213937, ('\u{594F}', false)), // East Asian ideograph
    (0x233938, ('\u{8DE9}', false)), // East Asian ideograph
    (0x213434, ('\u{52D5}', false)), // East Asian ideograph
    (0x22393A, ('\u{66B2}', false)), // East Asian ideograph
    (0x23393B, ('\u{8DE3}', false)), // East Asian ideograph
    (0x21393C, ('\u{5960}', false)), // East Asian ideograph
    (0x23393D, ('\u{8DE7}', false)), // East Asian ideograph
    (0x21393E, ('\u{5967}', false)), // East Asian ideograph
    (0x23393F, ('\u{8E09}', false)), // East Asian ideograph
    (0x223940, ('\u{66B5}', false)), // East Asian ideograph
    (0x273941, ('\u{594B}', false)), // East Asian ideograph
    (0x213942, ('\u{5973}', false)), // East Asian ideograph
    (0x223943, ('\u{66AC}', false)), // East Asian ideograph
    (0x233944, ('\u{8DFF}', false)), // East Asian ideograph
    (0x213945, ('\u{5984}', false)), // East Asian ideograph
    (0x233946, ('\u{8E05}', false)), // East Asian ideograph
    (0x223947, ('\u{66B1}', false)), // East Asian ideograph
    (0x213948, ('\u{597D}', false)), // East Asian ideograph
    (0x233949, ('\u{8E01}', false)), // East Asian ideograph
    (0x21394A, ('\u{5982}', false)), // East Asian ideograph
    (0x21394B, ('\u{5981}', false)), // East Asian ideograph
    (0x21394C, ('\u{59A8}', false)), // East Asian ideograph
    (0x21394D, ('\u{5992}', false)), // East Asian ideograph
    (0x23394E, ('\u{8E04}', false)), // East Asian ideograph
    (0x22394F, ('\u{66BE}', false)), // East Asian ideograph
    (0x233950, ('\u{8E06}', false)), // East Asian ideograph
    (0x233438, ('\u{8B3E}', false)), // East Asian ideograph
    (0x233952, ('\u{8E2A}', false)), // East Asian ideograph
    (0x273953, ('\u{5986}', false)), // East Asian ideograph
    (0x223954, ('\u{66C0}', false)), // East Asian ideograph
    (0x223955, ('\u{66C7}', false)), // East Asian ideograph
    (0x213956, ('\u{598A}', false)), // East Asian ideograph
    (0x233957, ('\u{8E2E}', false)), // East Asian ideograph
    (0x233958, ('\u{8E21}', false)), // East Asian ideograph
    (0x213959, ('\u{59BB}', false)), // East Asian ideograph
    (0x22395A, ('\u{66BB}', false)), // East Asian ideograph
    (0x21395B, ('\u{59D1}', false)), // East Asian ideograph
    (0x22395C, ('\u{66C4}', false)), // East Asian ideograph
    (0x21343A, ('\u{52DF}', false)), // East Asian ideograph
    (0x21395E, ('\u{59D0}', false)), // East Asian ideograph
    (0x21395F, ('\u{59D7}', false)), // East Asian ideograph
    (0x223960, ('\u{66CF}', false)), // East Asian ideograph
    (0x213961, ('\u{59D2}', false)), // East Asian ideograph
    (0x213962, ('\u{59D3}', false)), // East Asian ideograph
    (0x213963, ('\u{59CA}', false)), // East Asian ideograph
    (0x233964, ('\u{8E16}', false)), // East Asian ideograph
    (0x213965, ('\u{59CB}', false)), // East Asian ideograph
    (0x233966, ('\u{8E26}', false)), // East Asian ideograph
    (0x213967, ('\u{59E3}', false)), // East Asian ideograph
    (0x233968, ('\u{8E14}', false)), // East Asian ideograph
    (0x213969, ('\u{59FF}', false)), // East Asian ideograph
    (0x21396A, ('\u{59D8}', false)), // East Asian ideograph
    (0x21396B, ('\u{5A03}', false)), // East Asian ideograph
    (0x21396C, ('\u{59E8}', false)), // East Asian ideograph
    (0x21396D, ('\u{59E5}', false)), // East Asian ideograph
    (0x21396E, ('\u{59EA}', false)), // East Asian ideograph
    (0x23396F, ('\u{8E41}', false)), // East Asian ideograph
    (0x213970, ('\u{59FB}', false)), // East Asian ideograph
    (0x223971, ('\u{66DA}', false)), // East Asian ideograph
    (0x223972, ('\u{66DB}', false)), // East Asian ideograph
    (0x223973, ('\u{66E2}', false)), // East Asian ideograph
    (0x213974, ('\u{5A18}', false)), // East Asian ideograph
    (0x213975, ('\u{5A23}', false)), // East Asian ideograph
    (0x223976, ('\u{66E1}', false)), // East Asian ideograph
    (0x233977, ('\u{8E40}', false)), // East Asian ideograph
    (0x223978, ('\u{66E8}', false)), // East Asian ideograph
    (0x233979, ('\u{8E36}', false)), // East Asian ideograph
    (0x21397A, ('\u{5A1F}', false)), // East Asian ideograph
    (0x21397B, ('\u{5A1B}', false)), // East Asian ideograph
    (0x22397C, ('\u{66E9}', false)), // East Asian ideograph
    (0x21397D, ('\u{5A29}', false)), // East Asian ideograph
    (0x23397E, ('\u{8E3D}', false)), // East Asian ideograph
    (0x27785A, ('\u{5785}', false)), // East Asian ideograph
    (0x217441, ('\u{5724}', false)), // East Asian ideograph
    (0x6F532A, ('\u{C12A}', false)), // Korean hangul
    (0x213442, ('\u{5306}', false)), // East Asian ideograph
    (0x334550, ('\u{7F47}', false)), // East Asian ideograph
    (0x232337, ('\u{846E}', false)), // East Asian ideograph
    (0x217443, ('\u{5729}', false)), // East Asian ideograph
    (0x4B5521, ('\u{8332}', false)), // East Asian ideograph
    (0x233444, ('\u{8B54}', false)), // East Asian ideograph
    (0x235C71, ('\u{9E07}', false)), // East Asian ideograph
    (0x22525E, ('\u{7176}', false)), // East Asian ideograph
    (0x213445, ('\u{5310}', false)), // East Asian ideograph
    (0x2D4B45, ('\u{6BEC}', false)), // East Asian ideograph
    (0x6F5435, ('\u{C309}', false)), // Korean hangul
    (0x22527B, ('\u{7187}', false)), // East Asian ideograph
    (0x6F576E, ('\u{C900}', false)), // Korean hangul
    (0x6F532B, ('\u{C12C}', false)), // Korean hangul
    (0x6F2527, ('\u{3162}', false)), // Korean hangul
    (0x227447, ('\u{7F63}', false)), // East Asian ideograph
    (0x4B3666, ('\u{5A1A}', false)), // East Asian ideograph
    (0x233448, ('\u{8B53}', false)), // East Asian ideograph
    (0x233449, ('\u{8B4A}', false)), // East Asian ideograph
    (0x275124, ('\u{7EB8}', false)), // East Asian ideograph
    (0x223046, ('\u{6285}', false)), // East Asian ideograph
    (0x21344A, ('\u{5319}', false)), // East Asian ideograph
    (0x6F576F, ('\u{C904}', false)), // Korean hangul
    (0x6F532C, ('\u{C12D}', false)), // Korean hangul
    (0x23356F, ('\u{8C74}', false)), // East Asian ideograph
    (0x6F4B7B, ('\u{B11E}', false)), // Korean hangul
    (0x215B2A, ('\u{8E91}', false)), // East Asian ideograph
    (0x6F4F6F, ('\u{B9DB}', false)), // Korean hangul
    (0x21344D, ('\u{5321}', false)), // East Asian ideograph
    (0x22344E, ('\u{64A2}', false)), // East Asian ideograph
    (0x23344F, ('\u{8B3F}', false)), // East Asian ideograph
    (0x2E7450, ('\u{7F82}', false)), // East Asian ideograph
    (0x213451, ('\u{532F}', false)), // East Asian ideograph
    (0x335230, ('\u{7F6E}', false)), // East Asian ideograph (variant of 215230 which maps to 7F6E)
    (0x4B3668, ('\u{5358}', false)), // East Asian ideograph
    (0x234553, ('\u{9356}', false)), // East Asian ideograph
    (0x21725D, ('\u{5620}', false)), // East Asian ideograph
    (0x27554F, ('\u{53F6}', false)), // East Asian ideograph
    (0x213453, ('\u{5339}', false)), // East Asian ideograph
    (0x223454, ('\u{6490}', false)), // East Asian ideograph
    (0x213455, ('\u{5340}', false)), // East Asian ideograph
    (0x215F6F, ('\u{9748}', false)), // East Asian ideograph
    (0x215B2C, ('\u{8EAA}', false)), // East Asian ideograph
    (0x234554, ('\u{9371}', false)), // East Asian ideograph
    (0x6F5028, ('\u{BA4D}', false)), // Korean hangul
    (0x283457, ('\u{63B8}', false)), // East Asian ideograph
    (0x274B2D, ('\u{736D}', false)), // East Asian ideograph
    (0x2D3458, ('\u{4EDF}', false)), // East Asian ideograph
    (0x6F4C65, ('\u{B2D0}', false)), // Korean hangul
    (0x284934, ('\u{6D43}', false)), // East Asian ideograph
    (0x233459, ('\u{8B59}', false)), // East Asian ideograph
    (0x6F5772, ('\u{C90D}', false)), // Korean hangul
    (0x233A21, ('\u{8E30}', false)), // East Asian ideograph
    (0x213A22, ('\u{5A49}', false)), // East Asian ideograph
    (0x21345B, ('\u{5347}', false)), // East Asian ideograph
    (0x233A24, ('\u{8E47}', false)), // East Asian ideograph
    (0x213A25, ('\u{5A4A}', false)), // East Asian ideograph
    (0x233A26, ('\u{8E46}', false)), // East Asian ideograph
    (0x273A27, ('\u{5987}', false)), // East Asian ideograph
    (0x273A28, ('\u{5A04}', false)), // East Asian ideograph
    (0x213A29, ('\u{5A3C}', false)), // East Asian ideograph
    (0x213A2A, ('\u{5A62}', false)), // East Asian ideograph
    (0x213A2B, ('\u{5A5A}', false)), // East Asian ideograph
    (0x213A2C, ('\u{5A77}', false)), // East Asian ideograph
    (0x213A2D, ('\u{5A9A}', false)), // East Asian ideograph
    (0x233A2E, ('\u{8E4C}', false)), // East Asian ideograph
    (0x213A2F, ('\u{5A7F}', false)), // East Asian ideograph
    (0x223A30, ('\u{670F}', false)), // East Asian ideograph
    (0x224767, ('\u{6CAC}', false)), // East Asian ideograph
    (0x233A32, ('\u{8E4F}', false)), // East Asian ideograph
    (0x223A33, ('\u{6712}', false)), // East Asian ideograph
    (0x223A34, ('\u{6713}', false)), // East Asian ideograph
    (0x233A35, ('\u{8E62}', false)), // East Asian ideograph
    (0x233A36, ('\u{8E60}', false)), // East Asian ideograph
    (0x213A37, ('\u{5AB2}', false)), // East Asian ideograph
    (0x223A38, ('\u{6719}', false)), // East Asian ideograph
    (0x223A39, ('\u{6718}', false)), // East Asian ideograph
    (0x233A3A, ('\u{8E54}', false)), // East Asian ideograph
    (0x273A3B, ('\u{59AA}', false)), // East Asian ideograph
    (0x213A3C, ('\u{5AD6}', false)), // East Asian ideograph
    (0x213A3D, ('\u{5AE3}', false)), // East Asian ideograph
    (0x233A3E, ('\u{8E5A}', false)), // East Asian ideograph
    (0x233A3F, ('\u{8E5E}', false)), // East Asian ideograph
    (0x233A40, ('\u{8E55}', false)), // East Asian ideograph
    (0x273A41, ('\u{5A34}', false)), // East Asian ideograph
    (0x213A42, ('\u{5B09}', false)), // East Asian ideograph
    (0x273A43, ('\u{5A75}', false)), // East Asian ideograph
    (0x273A44, ('\u{5A07}', false)), // East Asian ideograph
    (0x273A45, ('\u{59A9}', false)), // East Asian ideograph
    (0x233A46, ('\u{8E95}', false)), // East Asian ideograph
    (0x223A47, ('\u{6723}', false)), // East Asian ideograph
    (0x233A48, ('\u{8E6D}', false)), // East Asian ideograph
    (0x213A49, ('\u{5B24}', false)), // East Asian ideograph
    (0x273A4A, ('\u{5A74}', false)), // East Asian ideograph
    (0x273A4B, ('\u{5A76}', false)), // East Asian ideograph
    (0x223A4C, ('\u{673E}', false)), // East Asian ideograph
    (0x213462, ('\u{5351}', false)), // East Asian ideograph
    (0x223A4E, ('\u{673F}', false)), // East Asian ideograph
    (0x213A4F, ('\u{5B53}', false)), // East Asian ideograph
    (0x213A50, ('\u{5B54}', false)), // East Asian ideograph
    (0x213A51, ('\u{5B55}', false)), // East Asian ideograph
    (0x213A52, ('\u{5B57}', false)), // East Asian ideograph
    (0x213A53, ('\u{5B58}', false)), // East Asian ideograph
    (0x213A54, ('\u{5B5D}', false)), // East Asian ideograph
    (0x213A55, ('\u{5B5C}', false)), // East Asian ideograph
    (0x233A57, ('\u{8E8B}', false)), // East Asian ideograph
    (0x223A58, ('\u{6757}', false)), // East Asian ideograph
    (0x213A59, ('\u{5B64}', false)), // East Asian ideograph
    (0x213A5A, ('\u{5B69}', false)), // East Asian ideograph
    (0x273A5B, ('\u{5B59}', false)), // East Asian ideograph
    (0x223A5C, ('\u{6747}', false)), // East Asian ideograph
    (0x213A5D, ('\u{5B73}', false)), // East Asian ideograph
    (0x233A5E, ('\u{8E9A}', false)), // East Asian ideograph
    (0x273A5F, ('\u{5B5A}', false)), // East Asian ideograph
    (0x273A60, ('\u{5B66}', false)), // East Asian ideograph
    (0x223A61, ('\u{6755}', false)), // East Asian ideograph
    (0x213A62, ('\u{5B7D}', false)), // East Asian ideograph
    (0x233A63, ('\u{8E98}', false)), // East Asian ideograph
    (0x233A64, ('\u{8E9E}', false)), // East Asian ideograph
    (0x223466, ('\u{64B3}', false)), // East Asian ideograph
    (0x223A66, ('\u{674C}', false)), // East Asian ideograph
    (0x223A67, ('\u{6759}', false)), // East Asian ideograph
    (0x223A68, ('\u{6748}', false)), // East Asian ideograph
    (0x213A69, ('\u{5B8C}', false)), // East Asian ideograph
    (0x275553, ('\u{8364}', false)), // East Asian ideograph
    (0x233A6B, ('\u{8EA5}', false)), // East Asian ideograph
    (0x213A6C, ('\u{5B97}', false)), // East Asian ideograph
    (0x213A6D, ('\u{5B9A}', false)), // East Asian ideograph
    (0x213A6E, ('\u{5B9C}', false)), // East Asian ideograph
    (0x233A6F, ('\u{8EA7}', false)), // East Asian ideograph
    (0x213A70, ('\u{5B99}', false)), // East Asian ideograph
    (0x223A71, ('\u{674A}', false)), // East Asian ideograph
    (0x233A72, ('\u{8E99}', false)), // East Asian ideograph
    (0x213A73, ('\u{5BA3}', false)), // East Asian ideograph
    (0x213A74, ('\u{5BA6}', false)), // East Asian ideograph
    (0x213A75, ('\u{5BA4}', false)), // East Asian ideograph
    (0x213A76, ('\u{5BA2}', false)), // East Asian ideograph
    (0x213A77, ('\u{5BB0}', false)), // East Asian ideograph
    (0x213A78, ('\u{5BB8}', false)), // East Asian ideograph
    (0x233A7A, ('\u{8EBC}', false)), // East Asian ideograph
    (0x213A7B, ('\u{5BB4}', false)), // East Asian ideograph
    (0x223A7C, ('\u{6785}', false)), // East Asian ideograph
    (0x213A7D, ('\u{5BB9}', false)), // East Asian ideograph
    (0x213A7E, ('\u{5BB3}', false)), // East Asian ideograph
    (0x23233F, ('\u{844A}', false)), // East Asian ideograph
    (0x4B763D, ('\u{57F4}', false)), // East Asian ideograph (variant of 21763D which maps to 57F4)
    (0x22746B, ('\u{7F7E}', false)), // East Asian ideograph
    (0x283B7D, ('\u{53F0}', false)), // East Asian ideograph (duplicate simplified)
    (0x22346C, ('\u{64D3}', false)), // East Asian ideograph
    (0x6F5927, ('\u{CC39}', false)), // Korean hangul
    (0x393B39, ('\u{5BF3}', false)), // East Asian ideograph
    (0x213F26, ('\u{614C}', false)), // East Asian ideograph
    (0x235222, ('\u{9957}', false)), // East Asian ideograph (variant of 475222 which maps to 9957)
    (0x2D346E, ('\u{5373}', false)), // East Asian ideograph
    (0x276232, ('\u{9E23}', false)), // East Asian ideograph
    (0x6F5333, ('\u{C13C}', false)), // Korean hangul
    (0x213D5B, ('\u{5F79}', false)), // East Asian ideograph
    (0x213471, ('\u{5378}', false)), // East Asian ideograph
    (0x287472, ('\u{7F74}', false)), // East Asian ideograph
    (0x23344D, ('\u{8B56}', false)), // East Asian ideograph
    (0x335223, ('\u{7E8E}', false)), // East Asian ideograph
    (0x233473, ('\u{8B45}', false)), // East Asian ideograph
    (0x273F3F, ('\u{51ED}', false)), // East Asian ideograph
    (0x213474, ('\u{537F}', false)), // East Asian ideograph
    (0x213475, ('\u{5384}', false)), // East Asian ideograph
    (0x21325D, ('\u{50F9}', false)), // East Asian ideograph
    (0x225F21, ('\u{75F9}', false)), // East Asian ideograph
    (0x217477, ('\u{576D}', false)), // East Asian ideograph
    (0x225F22, ('\u{75FC}', false)), // East Asian ideograph
    (0x23456F, ('\u{9364}', false)), // East Asian ideograph
    (0x275F23, ('\u{9648}', false)), // East Asian ideograph
    (0x213479, ('\u{53A5}', false)), // East Asian ideograph
    (0x275F24, ('\u{9646}', false)), // East Asian ideograph
    (0x22747A, ('\u{7F91}', false)), // East Asian ideograph
    (0x21347B, ('\u{53B2}', false)), // East Asian ideograph
    (0x21392F, ('\u{5954}', false)), // East Asian ideograph
    (0x225269, ('\u{7150}', false)), // East Asian ideograph
    (0x4B4835, ('\u{6DA3}', false)), // East Asian ideograph
    (0x21347D, ('\u{53C3}', false)), // East Asian ideograph
    (0x2D5F28, ('\u{9665}', false)), // East Asian ideograph
    (0x6F5336, ('\u{C149}', false)), // Korean hangul
    (0x6F5329, ('\u{C127}', false)), // Korean hangul
    (0x225F29, ('\u{7616}', false)), // East Asian ideograph
    (0x275F2A, ('\u{9634}', false)), // East Asian ideograph
    (0x275F2B, ('\u{961F}', false)), // East Asian ideograph
    (0x216C41, ('\u{52D1}', false)), // East Asian ideograph
    (0x225F2C, ('\u{7608}', false)), // East Asian ideograph
    (0x6F577A, ('\u{C958}', false)), // Korean hangul
    (0x225F2D, ('\u{7615}', false)), // East Asian ideograph
    (0x295731, ('\u{9C8B}', false)), // East Asian ideograph
    (0x276222, ('\u{9CC5}', false)), // East Asian ideograph
    (0x225F2E, ('\u{760C}', false)), // East Asian ideograph
    (0x23455D, ('\u{9349}', false)), // East Asian ideograph
    (0x6F5567, ('\u{C5FE}', false)), // Korean hangul
    (0x215F2F, ('\u{9685}', false)), // East Asian ideograph
    (0x273340, ('\u{51BB}', false)), // East Asian ideograph
    (0x223B21, ('\u{677B}', false)), // East Asian ideograph
    (0x223B22, ('\u{6792}', false)), // East Asian ideograph
    (0x223B23, ('\u{6776}', false)), // East Asian ideograph
    (0x213B24, ('\u{5BC4}', false)), // East Asian ideograph
    (0x223B25, ('\u{6791}', false)), // East Asian ideograph
    (0x223B26, ('\u{6799}', false)), // East Asian ideograph
    (0x215F31, ('\u{968D}', false)), // East Asian ideograph
    (0x223B28, ('\u{67A4}', false)), // East Asian ideograph
    (0x213B29, ('\u{5BD0}', false)), // East Asian ideograph
    (0x213B2A, ('\u{5BD3}', false)), // East Asian ideograph
    (0x213B2B, ('\u{5BE1}', false)), // East Asian ideograph
    (0x213B2C, ('\u{5BE5}', false)), // East Asian ideograph
    (0x215F32, ('\u{9698}', false)), // East Asian ideograph
    (0x223B2E, ('\u{678F}', false)), // East Asian ideograph
    (0x233B2F, ('\u{8ECF}', false)), // East Asian ideograph
    (0x223B30, ('\u{6772}', false)), // East Asian ideograph
    (0x223B31, ('\u{6798}', false)), // East Asian ideograph (variant of 4C3B31 which maps to 6798)
    (0x223B32, ('\u{676A}', false)), // East Asian ideograph
    (0x233B33, ('\u{8ED5}', false)), // East Asian ideograph
    (0x213B34, ('\u{5BEE}', false)), // East Asian ideograph
    (0x273B35, ('\u{5BBD}', false)), // East Asian ideograph
    (0x273B36, ('\u{5BA1}', false)), // East Asian ideograph
    (0x273B37, ('\u{5199}', false)), // East Asian ideograph
    (0x273B38, ('\u{5BA0}', false)), // East Asian ideograph
    (0x223B39, ('\u{67AC}', false)), // East Asian ideograph
    (0x213B3A, ('\u{5BF8}', false)), // East Asian ideograph
    (0x223B3B, ('\u{67A0}', false)), // East Asian ideograph
    (0x213B3C, ('\u{5C01}', false)), // East Asian ideograph
    (0x213B3D, ('\u{5C04}', false)), // East Asian ideograph
    (0x213B3E, ('\u{5C09}', false)), // East Asian ideograph
    (0x233B3F, ('\u{8EFA}', false)), // East Asian ideograph
    (0x273B40, ('\u{5C06}', false)), // East Asian ideograph
    (0x213B41, ('\u{5C0A}', false)), // East Asian ideograph
    (0x233B42, ('\u{8EF9}', false)), // East Asian ideograph
    (0x273B43, ('\u{5BF9}', false)), // East Asian ideograph
    (0x223B44, ('\u{67F9}', false)), // East Asian ideograph
    (0x213B45, ('\u{5C0F}', false)), // East Asian ideograph
    (0x213B46, ('\u{5C11}', false)), // East Asian ideograph
    (0x213B47, ('\u{5C16}', false)), // East Asian ideograph
    (0x223B48, ('\u{678D}', false)), // East Asian ideograph
    (0x223B49, ('\u{678C}', false)), // East Asian ideograph
    (0x213B4A, ('\u{5C2C}', false)), // East Asian ideograph
    (0x233B4B, ('\u{8EE8}', false)), // East Asian ideograph
    (0x223B4C, ('\u{67FC}', false)), // East Asian ideograph
    (0x213B4D, ('\u{5C38}', false)), // East Asian ideograph
    (0x223B4E, ('\u{6810}', false)), // East Asian ideograph
    (0x233B4F, ('\u{8EEB}', false)), // East Asian ideograph
    (0x213B50, ('\u{5C40}', false)), // East Asian ideograph
    (0x223B51, ('\u{67C8}', false)), // East Asian ideograph
    (0x23455F, ('\u{935A}', false)), // East Asian ideograph
    (0x213B53, ('\u{5C3E}', false)), // East Asian ideograph
    (0x223B54, ('\u{67CC}', false)), // East Asian ideograph
    (0x213B55, ('\u{5C45}', false)), // East Asian ideograph
    (0x233B56, ('\u{8F00}', false)), // East Asian ideograph
    (0x213B57, ('\u{5C4E}', false)), // East Asian ideograph
    (0x223B58, ('\u{67C5}', false)), // East Asian ideograph
    (0x233B59, ('\u{8F05}', false)), // East Asian ideograph
    (0x233B5A, ('\u{8F08}', false)), // East Asian ideograph
    (0x233B5B, ('\u{8F07}', false)), // East Asian ideograph
    (0x223B5C, ('\u{67BB}', false)), // East Asian ideograph
    (0x213B5D, ('\u{5C5B}', false)), // East Asian ideograph (not in Unicode)
    (0x213B5E, ('\u{5C60}', false)), // East Asian ideograph
    (0x223B5F, ('\u{67B0}', false)), // East Asian ideograph
    (0x223B60, ('\u{6803}', false)), // East Asian ideograph
    (0x223B61, ('\u{67F8}', false)), // East Asian ideograph
    (0x213B62, ('\u{5C65}', false)), // East Asian ideograph
    (0x273B63, ('\u{5C5E}', false)), // East Asian ideograph
    (0x233B64, ('\u{8F2C}', false)), // East Asian ideograph
    (0x213B65, ('\u{5C71}', false)), // East Asian ideograph
    (0x225A2A, ('\u{741B}', false)), // East Asian ideograph
    (0x213B67, ('\u{5C90}', false)), // East Asian ideograph
    (0x213B68, ('\u{5C8C}', false)), // East Asian ideograph
    (0x213B69, ('\u{5C91}', false)), // East Asian ideograph
    (0x213B6A, ('\u{5C94}', false)), // East Asian ideograph
    (0x233B6B, ('\u{8F1E}', false)), // East Asian ideograph
    (0x213B6C, ('\u{5CB8}', false)), // East Asian ideograph
    (0x233B6D, ('\u{8F25}', false)), // East Asian ideograph
    (0x233B6E, ('\u{8F20}', false)), // East Asian ideograph
    (0x223B6F, ('\u{67E4}', false)), // East Asian ideograph
    (0x223B70, ('\u{67D9}', false)), // East Asian ideograph
    (0x223B71, ('\u{67DB}', false)), // East Asian ideograph
    (0x223B72, ('\u{67B5}', false)), // East Asian ideograph
    (0x213B73, ('\u{5D01}', false)), // East Asian ideograph
    (0x273B74, ('\u{5CE1}', false)), // East Asian ideograph
    (0x223B75, ('\u{67F7}', false)), // East Asian ideograph
    (0x213B76, ('\u{5CFB}', false)), // East Asian ideograph
    (0x223B77, ('\u{67B3}', false)), // East Asian ideograph
    (0x233B78, ('\u{8F36}', false)), // East Asian ideograph
    (0x233B79, ('\u{8F2E}', false)), // East Asian ideograph
    (0x233B7A, ('\u{8F33}', false)), // East Asian ideograph
    (0x215F3F, ('\u{96C0}', false)), // East Asian ideograph
    (0x223B7C, ('\u{67EE}', false)), // East Asian ideograph
    (0x223B7D, ('\u{6AAF}', false)), // East Asian ideograph
    (0x223B7E, ('\u{67B2}', false)), // East Asian ideograph
    (0x225F40, ('\u{761B}', false)), // East Asian ideograph
    (0x6F577E, ('\u{C970}', false)), // Korean hangul
    (0x6F533B, ('\u{C158}', false)), // Korean hangul
    (0x213D63, ('\u{5F8A}', false)), // East Asian ideograph
    (0x6F4B7E, ('\u{B125}', false)), // Korean hangul
    (0x2D5D2F, ('\u{9196}', false)), // East Asian ideograph
    (0x2D5F43, ('\u{9CEB}', false)), // East Asian ideograph
    (0x6F2459, ('\u{3137}', false)), // Korean hangul
    (0x293B42, ('\u{8F75}', false)), // East Asian ideograph
    (0x235F45, ('\u{9F22}', false)), // East Asian ideograph
    (0x2D5F46, ('\u{96BD}', false)), // East Asian ideograph
    (0x6F533C, ('\u{C167}', false)), // Korean hangul
    (0x213D64, ('\u{5F87}', false)), // East Asian ideograph
    (0x225F47, ('\u{7619}', false)), // East Asian ideograph
    (0x234562, ('\u{935F}', false)), // East Asian ideograph
    (0x235F48, ('\u{9F2B}', false)), // East Asian ideograph
    (0x2E604A, ('\u{7690}', false)), // East Asian ideograph
    (0x235F49, ('\u{9F26}', false)), // East Asian ideograph
    (0x225270, ('\u{7144}', false)), // East Asian ideograph
    (0x6F5D65, ('\u{D72D}', false)), // Korean hangul
    (0x224E2D, ('\u{6FC9}', false)), // East Asian ideograph
    (0x2D4B3F, ('\u{73CE}', false)), // East Asian ideograph
    (0x275F4B, ('\u{6742}', false)), // East Asian ideograph
    (0x276234, ('\u{9E29}', false)), // East Asian ideograph
    (0x6F533D, ('\u{C168}', false)), // Korean hangul
    (0x225F4C, ('\u{761D}', false)), // East Asian ideograph
    (0x275F4D, ('\u{96CF}', false)), // East Asian ideograph
    (0x6F5773, ('\u{C90F}', false)), // Korean hangul
    (0x6F245B, ('\u{3141}', false)), // Korean hangul
    (0x275F4E, ('\u{53CC}', false)), // East Asian ideograph
    (0x216C48, ('\u{52D6}', false)), // East Asian ideograph
    (0x275F4F, ('\u{79BB}', false)), // East Asian ideograph
    (0x215F50, ('\u{96E3}', false)), // East Asian ideograph (variant of 4B5F50 which maps to 96E3)
    (0x6F533E, ('\u{C170}', false)), // Korean hangul
    (0x213D66, ('\u{5F92}', false)), // East Asian ideograph
    (0x215F51, ('\u{96E8}', false)), // East Asian ideograph
    (0x225F3B, ('\u{7610}', false)), // East Asian ideograph
    (0x284345, ('\u{680C}', false)), // East Asian ideograph
    (0x6F5848, ('\u{CA08}', false)), // Korean hangul
    (0x6F245C, ('\u{3142}', false)), // Korean hangul
    (0x235F53, ('\u{9F2F}', false)), // East Asian ideograph
    (0x225F54, ('\u{762D}', false)), // East Asian ideograph
    (0x234571, ('\u{936B}', false)), // East Asian ideograph
    (0x6F505B, ('\u{BBC0}', false)), // Korean hangul
    (0x275F55, ('\u{7535}', false)), // East Asian ideograph
    (0x6F533F, ('\u{C18C}', false)), // Korean hangul
    (0x213D67, ('\u{5F91}', false)), // East Asian ideograph
    (0x6F4D64, ('\u{B4E0}', false)), // Korean hangul
    (0x213924, ('\u{5922}', false)), // East Asian ideograph
    (0x6F245D, ('\u{3145}', false)), // Korean hangul
    (0x275128, ('\u{7ECB}', false)), // East Asian ideograph
    (0x4B5F58, ('\u{F9B2}', false)), // East Asian ideograph
    (0x227049, ('\u{7D0F}', false)), // East Asian ideograph
    (0x224E30, ('\u{6FA0}', false)), // East Asian ideograph
    (0x293338, ('\u{8BFD}', false)), // East Asian ideograph
    (0x2E715A, ('\u{7E27}', false)), // East Asian ideograph
    (0x215F5A, ('\u{9707}', false)), // East Asian ideograph
    (0x6F5340, ('\u{C18D}', false)), // Korean hangul
    (0x223C21, ('\u{67B9}', false)), // East Asian ideograph
    (0x213C22, ('\u{5D11}', false)), // East Asian ideograph
    (0x215B3E, ('\u{8EFE}', false)), // East Asian ideograph
    (0x223C24, ('\u{67E3}', false)), // East Asian ideograph
    (0x213C25, ('\u{5D14}', false)), // East Asian ideograph
    (0x233C26, ('\u{8F39}', false)), // East Asian ideograph
    (0x233C27, ('\u{8F34}', false)), // East Asian ideograph
    (0x273C28, ('\u{5C9A}', false)), // East Asian ideograph
    (0x223C29, ('\u{67E2}', false)), // East Asian ideograph
    (0x273C2A, ('\u{5D2D}', false)), // East Asian ideograph
    (0x273C2B, ('\u{5C96}', false)), // East Asian ideograph
    (0x213C2C, ('\u{5D9D}', false)), // East Asian ideograph
    (0x273C2D, ('\u{5C7F}', false)), // East Asian ideograph
    (0x273C2E, ('\u{5CB3}', false)), // East Asian ideograph
    (0x223C2F, ('\u{67E7}', false)), // East Asian ideograph
    (0x223C30, ('\u{6849}', false)), // East Asian ideograph
    (0x223C31, ('\u{683E}', false)), // East Asian ideograph
    (0x273C32, ('\u{5DC5}', false)), // East Asian ideograph
    (0x214A32, ('\u{71EC}', false)), // East Asian ideograph
    (0x213C34, ('\u{5DDD}', false)), // East Asian ideograph
    (0x215F5E, ('\u{9711}', false)), // East Asian ideograph
    (0x223C36, ('\u{6814}', false)), // East Asian ideograph
    (0x223C37, ('\u{684B}', false)), // East Asian ideograph
    (0x223C38, ('\u{681E}', false)), // East Asian ideograph
    (0x213C39, ('\u{5DE7}', false)), // East Asian ideograph
    (0x213C3A, ('\u{5DE6}', false)), // East Asian ideograph
    (0x223C3B, ('\u{6833}', false)), // East Asian ideograph
    (0x213C3C, ('\u{5DEE}', false)), // East Asian ideograph
    (0x233C3D, ('\u{8F52}', false)), // East Asian ideograph
    (0x213C3E, ('\u{5DF2}', false)), // East Asian ideograph
    (0x213C3F, ('\u{5DF3}', false)), // East Asian ideograph
    (0x223C40, ('\u{6831}', false)), // East Asian ideograph
    (0x215F60, ('\u{9716}', false)), // East Asian ideograph
    (0x223C42, ('\u{6835}', false)), // East Asian ideograph
    (0x223C43, ('\u{683B}', false)), // East Asian ideograph
    (0x223C44, ('\u{684E}', false)), // East Asian ideograph
    (0x213C46, ('\u{5E06}', false)), // East Asian ideograph
    (0x234124, ('\u{918D}', false)), // East Asian ideograph
    (0x233C48, ('\u{8F56}', false)), // East Asian ideograph
    (0x213C49, ('\u{5E1A}', false)), // East Asian ideograph
    (0x223C4A, ('\u{684D}', false)), // East Asian ideograph
    (0x233C4B, ('\u{8F55}', false)), // East Asian ideograph
    (0x233C4C, ('\u{8F58}', false)), // East Asian ideograph
    (0x215F62, ('\u{970D}', false)), // East Asian ideograph
    (0x233C4E, ('\u{8F5E}', false)), // East Asian ideograph
    (0x273C4F, ('\u{5E05}', false)), // East Asian ideograph
    (0x273C51, ('\u{5E08}', false)), // East Asian ideograph
    (0x273C52, ('\u{5E10}', false)), // East Asian ideograph
    (0x273C53, ('\u{5E26}', false)), // East Asian ideograph
    (0x213C54, ('\u{5E38}', false)), // East Asian ideograph
    (0x223C55, ('\u{685D}', false)), // East Asian ideograph
    (0x223C56, ('\u{685E}', false)), // East Asian ideograph
    (0x233C57, ('\u{8F62}', false)), // East Asian ideograph
    (0x273C58, ('\u{5E27}', false)), // East Asian ideograph
    (0x233C59, ('\u{8F63}', false)), // East Asian ideograph
    (0x233C5A, ('\u{8F64}', false)), // East Asian ideograph
    (0x213C5B, ('\u{5E54}', false)), // East Asian ideograph
    (0x273C5C, ('\u{5E3C}', false)), // East Asian ideograph
    (0x213C5D, ('\u{5E55}', false)), // East Asian ideograph
    (0x273C5E, ('\u{5E01}', false)), // East Asian ideograph
    (0x213C5F, ('\u{5E62}', false)), // East Asian ideograph
    (0x273C60, ('\u{5E1C}', false)), // East Asian ideograph
    (0x273C61, ('\u{5E2E}', false)), // East Asian ideograph
    (0x213C63, ('\u{5E73}', false)), // East Asian ideograph
    (0x6F5177, ('\u{BE5A}', false)), // Korean hangul
    (0x223C65, ('\u{685A}', false)), // East Asian ideograph
    (0x233C66, ('\u{8FA5}', false)), // East Asian ideograph
    (0x273C67, ('\u{5E72}', false)), // East Asian ideograph (Version J extension)
    (0x223C68, ('\u{686B}', false)), // East Asian ideograph
    (0x223C69, ('\u{686C}', false)), // East Asian ideograph
    (0x213C6A, ('\u{5E7D}', false)), // East Asian ideograph
    (0x223C6B, ('\u{6879}', false)), // East Asian ideograph
    (0x233C6C, ('\u{8FB5}', false)), // East Asian ideograph
    (0x213C6D, ('\u{5E87}', false)), // East Asian ideograph
    (0x233C6E, ('\u{8FBB}', false)), // East Asian ideograph
    (0x213C6F, ('\u{5E9A}', false)), // East Asian ideograph
    (0x233C70, ('\u{8FBC}', false)), // East Asian ideograph
    (0x215F68, ('\u{9738}', false)), // East Asian ideograph
    (0x223C72, ('\u{687E}', false)), // East Asian ideograph
    (0x213C73, ('\u{5E95}', false)), // East Asian ideograph
    (0x233C74, ('\u{8FBF}', false)), // East Asian ideograph
    (0x233C75, ('\u{8FD2}', false)), // East Asian ideograph
    (0x273C76, ('\u{5E93}', false)), // East Asian ideograph
    (0x225F69, ('\u{7647}', false)), // East Asian ideograph
    (0x213C78, ('\u{5EAD}', false)), // East Asian ideograph
    (0x213C79, ('\u{5EB7}', false)), // East Asian ideograph
    (0x233C7A, ('\u{8FCA}', false)), // East Asian ideograph
    (0x233C7B, ('\u{8FD3}', false)), // East Asian ideograph
    (0x213C7C, ('\u{5EB5}', false)), // East Asian ideograph
    (0x215F6A, ('\u{9732}', false)), // East Asian ideograph
    (0x273C7E, ('\u{5395}', false)), // East Asian ideograph
    (0x275F6B, ('\u{9701}', false)), // East Asian ideograph
    (0x6F5849, ('\u{CA09}', false)), // Korean hangul
    (0x6F2461, ('\u{314B}', false)), // Korean hangul
    (0x275725, ('\u{8682}', false)), // East Asian ideograph
    (0x275122, ('\u{7EB3}', false)), // East Asian ideograph
    (0x6F5273, ('\u{C0D8}', false)), // Korean hangul
    (0x235F6D, ('\u{9F45}', false)), // East Asian ideograph
    (0x4C476E, ('\u{6CAD}', false)), // East Asian ideograph
    (0x225A2C, ('\u{7432}', false)), // East Asian ideograph
    (0x225F6E, ('\u{764D}', false)), // East Asian ideograph
    (0x6F2528, ('\u{3163}', false)), // Korean hangul
    (0x2F312B, ('\u{89BB}', false)), // East Asian ideograph
    (0x235F6F, ('\u{9F46}', false)), // East Asian ideograph
    (0x4B5F70, ('\u{9752}', false)), // East Asian ideograph
    (0x6F2462, ('\u{314C}', false)), // Korean hangul
    (0x235F71, ('\u{9F48}', false)), // East Asian ideograph
    (0x275123, ('\u{7EA7}', false)), // East Asian ideograph
    (0x214A36, ('\u{720D}', false)), // East Asian ideograph
    (0x224E35, ('\u{6FB4}', false)), // East Asian ideograph
    (0x335234, ('\u{99E1}', false)), // East Asian ideograph
    (0x2E3D62, ('\u{684A}', false)), // East Asian ideograph
    (0x235F73, ('\u{9F49}', false)), // East Asian ideograph
    (0x69253B, ('\u{30BB}', false)), // Katakana letter SE
    (0x276230, ('\u{9E20}', false)), // East Asian ideograph
    (0x23456B, ('\u{9374}', false)), // East Asian ideograph
    (0x215F75, ('\u{9760}', false)), // East Asian ideograph
    (0x692431, ('\u{3051}', false)), // Hiragana letter KE
    (0x274A21, ('\u{70BD}', false)), // East Asian ideograph
    (0x23345F, ('\u{8B4D}', false)), // East Asian ideograph
    (0x224D63, ('\u{6F5F}', false)), // East Asian ideograph
    (0x274A22, ('\u{7096}', false)), // East Asian ideograph
    (0x4D446B, ('\u{954E}', false)), // East Asian ideograph
    (0x6F4A23, ('\u{ADC4}', false)), // Korean hangul
    (0x6F5346, ('\u{C19F}', false)), // Korean hangul
    (0x276231, ('\u{9E22}', false)), // East Asian ideograph
    (0x215F79, ('\u{9768}', false)), // East Asian ideograph
    (0x274A24, ('\u{706F}', false)), // East Asian ideograph
    (0x345E3B, ('\u{80AC}', false)), // East Asian ideograph
    (0x215F7A, ('\u{9769}', false)), // East Asian ideograph
    (0x274A25, ('\u{7116}', false)), // East Asian ideograph
    (0x275568, ('\u{8298}', false)), // East Asian ideograph
    (0x215F7B, ('\u{9776}', false)), // East Asian ideograph
    (0x274A26, ('\u{70E7}', false)), // East Asian ideograph
    (0x6F5D67, ('\u{D73C}', false)), // Korean hangul
    (0x215F7C, ('\u{9774}', false)), // East Asian ideograph
    (0x6F4A27, ('\u{ADD3}', false)), // Korean hangul
    (0x2E3172, ('\u{5261}', false)), // East Asian ideograph
    (0x276236, ('\u{9E35}', false)), // East Asian ideograph
    (0x2D4A28, ('\u{8B8C}', false)), // East Asian ideograph
    (0x6F5347, ('\u{C1A1}', false)), // Korean hangul
    (0x213D6F, ('\u{5FA9}', false)), // East Asian ideograph
    (0x215F7E, ('\u{9785}', false)), // East Asian ideograph
    (0x33456D, ('\u{826A}', false)), // East Asian ideograph
    (0x6F5178, ('\u{BE5B}', false)), // Korean hangul
    (0x224A2A, ('\u{6DDF}', false)), // East Asian ideograph
    (0x6F2465, ('\u{3132}', false)), // Korean hangul
    (0x275126, ('\u{7ECA}', false)), // East Asian ideograph
    (0x23567A, ('\u{9B95}', false)), // East Asian ideograph
    (0x6F4A2C, ('\u{ADF8}', false)), // Korean hangul
    (0x224A2D, ('\u{6DD3}', false)), // East Asian ideograph
    (0x6F5348, ('\u{C1A5}', false)), // Korean hangul
    (0x276233, ('\u{51E4}', false)), // East Asian ideograph
    (0x274A2E, ('\u{8425}', false)), // East Asian ideograph
    (0x2E3328, ('\u{6528}', false)), // East Asian ideograph
    (0x6F5D6B, ('\u{D751}', false)), // Korean hangul
    (0x234A2F, ('\u{9642}', false)), // East Asian ideograph
    (0x4B462A, ('\u{6B74}', false)), // East Asian ideograph
    (0x233D21, ('\u{8FDA}', false)), // East Asian ideograph
    (0x233D22, ('\u{8FD5}', false)), // East Asian ideograph
    (0x213D23, ('\u{5EC9}', false)), // East Asian ideograph
    (0x213D24, ('\u{5EC8}', false)), // East Asian ideograph
    (0x223D25, ('\u{686D}', false)), // East Asian ideograph
    (0x213D26, ('\u{5ED6}', false)), // East Asian ideograph
    (0x273D27, ('\u{5E9F}', false)), // East Asian ideograph
    (0x213D28, ('\u{5EDA}', false)), // East Asian ideograph
    (0x213D29, ('\u{5EDD}', false)), // East Asian ideograph
    (0x273D2A, ('\u{5E7F}', false)), // East Asian ideograph
    (0x273D2B, ('\u{5E99}', false)), // East Asian ideograph
    (0x273D2C, ('\u{5382}', false)), // East Asian ideograph
    (0x273D2D, ('\u{5E9E}', false)), // East Asian ideograph
    (0x273D2E, ('\u{5E90}', false)), // East Asian ideograph
    (0x233D2F, ('\u{8FE4}', false)), // East Asian ideograph
    (0x233D30, ('\u{8FEE}', false)), // East Asian ideograph
    (0x223D32, ('\u{688B}', false)), // East Asian ideograph
    (0x274A33, ('\u{70E9}', false)), // East Asian ideograph
    (0x213D34, ('\u{5EFF}', false)), // East Asian ideograph
    (0x233D35, ('\u{8FF9}', false)), // East Asian ideograph
    (0x213D36, ('\u{5F04}', false)), // East Asian ideograph
    (0x213D37, ('\u{5F08}', false)), // East Asian ideograph
    (0x213D38, ('\u{5F0A}', false)), // East Asian ideograph
    (0x223D39, ('\u{68A3}', false)), // East Asian ideograph
    (0x213D3A, ('\u{5F12}', false)), // East Asian ideograph
    (0x213D3B, ('\u{5F13}', false)), // East Asian ideograph
    (0x233D3C, ('\u{8FFB}', false)), // East Asian ideograph
    (0x213D3D, ('\u{5F14}', false)), // East Asian ideograph
    (0x213D3E, ('\u{5F18}', false)), // East Asian ideograph
    (0x214A35, ('\u{7206}', false)), // East Asian ideograph
    (0x223D40, ('\u{688F}', false)), // East Asian ideograph
    (0x213D41, ('\u{5F1F}', false)), // East Asian ideograph
    (0x213D42, ('\u{5F26}', false)), // East Asian ideograph
    (0x223D43, ('\u{687B}', false)), // East Asian ideograph
    (0x233D44, ('\u{9011}', false)), // East Asian ideograph
    (0x224A36, ('\u{6DDC}', false)), // East Asian ideograph
    (0x213D46, ('\u{5F31}', false)), // East Asian ideograph
    (0x273D47, ('\u{5F20}', false)), // East Asian ideograph
    (0x213D48, ('\u{5F37}', false)), // East Asian ideograph
    (0x233D49, ('\u{9021}', false)), // East Asian ideograph
    (0x233D4A, ('\u{902D}', false)), // East Asian ideograph
    (0x274A37, ('\u{7089}', false)), // East Asian ideograph
    (0x273D4C, ('\u{5F25}', false)), // East Asian ideograph
    (0x273D4D, ('\u{5F2F}', false)), // East Asian ideograph
    (0x233D4E, ('\u{902C}', false)), // East Asian ideograph
    (0x273D4F, ('\u{6C47}', false)), // East Asian ideograph (duplicate simplified)
    (0x223D50, ('\u{692C}', false)), // East Asian ideograph
    (0x274A38, ('\u{70C2}', false)), // East Asian ideograph
    (0x213D52, ('\u{5F64}', false)), // East Asian ideograph
    (0x223D53, ('\u{690C}', false)), // East Asian ideograph
    (0x213D54, ('\u{5F6C}', false)), // East Asian ideograph
    (0x213D55, ('\u{5F69}', false)), // East Asian ideograph
    (0x233D56, ('\u{9037}', false)), // East Asian ideograph
    (0x214A39, ('\u{7228}', false)), // East Asian ideograph
    (0x223D58, ('\u{68D3}', false)), // East Asian ideograph
    (0x213D59, ('\u{5F71}', false)), // East Asian ideograph
    (0x223D5B, ('\u{690A}', false)), // East Asian ideograph
    (0x223D5C, ('\u{6909}', false)), // East Asian ideograph
    (0x233D5D, ('\u{9052}', false)), // East Asian ideograph
    (0x213D5E, ('\u{5F7F}', false)), // East Asian ideograph
    (0x213D5F, ('\u{5F7C}', false)), // East Asian ideograph
    (0x213D60, ('\u{5F85}', false)), // East Asian ideograph
    (0x213D61, ('\u{5F88}', false)), // East Asian ideograph
    (0x223D62, ('\u{68EC}', false)), // East Asian ideograph
    (0x223D63, ('\u{692A}', false)), // East Asian ideograph
    (0x223D64, ('\u{68EA}', false)), // East Asian ideograph
    (0x223D65, ('\u{681F}', false)), // East Asian ideograph
    (0x223D66, ('\u{7439}', false)), // East Asian ideograph
    (0x233D67, ('\u{9049}', false)), // East Asian ideograph
    (0x213D68, ('\u{5F90}', false)), // East Asian ideograph
    (0x234A3C, ('\u{9651}', false)), // East Asian ideograph
    (0x233D6A, ('\u{9044}', false)), // East Asian ideograph
    (0x213D6B, ('\u{5F99}', false)), // East Asian ideograph
    (0x273D6C, ('\u{4ECE}', false)), // East Asian ideograph
    (0x223D6E, ('\u{68D6}', false)), // East Asian ideograph
    (0x223D6F, ('\u{68EB}', false)), // East Asian ideograph
    (0x225F48, ('\u{761E}', false)), // East Asian ideograph
    (0x213D71, ('\u{5FAA}', false)), // East Asian ideograph
    (0x213D72, ('\u{5FAC}', false)), // East Asian ideograph
    (0x223D73, ('\u{68F1}', false)), // East Asian ideograph
    (0x273D74, ('\u{5F7B}', false)), // East Asian ideograph
    (0x233D75, ('\u{905D}', false)), // East Asian ideograph
    (0x273D76, ('\u{5F81}', false)), // East Asian ideograph
    (0x213D77, ('\u{5FBD}', false)), // East Asian ideograph
    (0x233D78, ('\u{905B}', false)), // East Asian ideograph
    (0x223D79, ('\u{68FC}', false)), // East Asian ideograph
    (0x213D7A, ('\u{5FD9}', false)), // East Asian ideograph
    (0x233D7B, ('\u{906B}', false)), // East Asian ideograph
    (0x223D7C, ('\u{6913}', false)), // East Asian ideograph
    (0x213D7D, ('\u{5FD6}', false)), // East Asian ideograph
    (0x224E3C, ('\u{6FA8}', false)), // East Asian ideograph
    (0x23523B, ('\u{99A3}', false)), // East Asian ideograph
    (0x6F534C, ('\u{C1C4}', false)), // Korean hangul
    (0x276237, ('\u{9E2A}', false)), // East Asian ideograph
    (0x224173, ('\u{6A8D}', false)), // East Asian ideograph
    (0x274A42, ('\u{7237}', false)), // East Asian ideograph
    (0x223D30, ('\u{6898}', false)), // East Asian ideograph
    (0x6F5925, ('\u{CC30}', false)), // Korean hangul
    (0x2D5C5B, ('\u{9089}', false)), // East Asian ideograph
    (0x6F4A43, ('\u{AE4C}', false)), // Korean hangul
    (0x234A44, ('\u{965C}', false)), // East Asian ideograph
    (0x232435, ('\u{84DA}', false)), // East Asian ideograph
    (0x696E5C, ('\u{91DF}', false)), // East Asian ideograph
    (0x295929, ('\u{9CA3}', false)), // East Asian ideograph
    (0x213E51, ('\u{608D}', false)), // East Asian ideograph
    (0x274A45, ('\u{5C14}', false)), // East Asian ideograph
    (0x233023, ('\u{8962}', false)), // East Asian ideograph
    (0x214A46, ('\u{7246}', false)), // East Asian ideograph
    (0x6F534D, ('\u{C1C8}', false)), // Korean hangul
    (0x276238, ('\u{9E2D}', false)), // East Asian ideograph
    (0x6F5740, ('\u{C81D}', false)), // Korean hangul
    (0x213932, ('\u{5947}', false)), // East Asian ideograph
    (0x295574, ('\u{9604}', false)), // East Asian ideograph
    (0x6F4A48, ('\u{AE5C}', false)), // Korean hangul
    (0x277345, ('\u{556E}', false)), // East Asian ideograph
    (0x6F4A49, ('\u{AE5D}', false)), // Korean hangul
    (0x29592A, ('\u{9CD3}', false)), // East Asian ideograph
    (0x4B4352, ('\u{66F5}', false)), // East Asian ideograph
    (0x234A4A, ('\u{965F}', false)), // East Asian ideograph
    (0x234A4B, ('\u{9656}', false)), // East Asian ideograph
    (0x6F534E, ('\u{C1D7}', false)), // Korean hangul
    (0x213D76, ('\u{5FB5}', false)), // East Asian ideograph
    (0x274A4C, ('\u{724D}', false)), // East Asian ideograph
    (0x6F4A4D, ('\u{AE65}', false)), // Korean hangul
    (0x6F5771, ('\u{C90C}', false)), // Korean hangul
    (0x295928, ('\u{9CD5}', false)), // East Asian ideograph
    (0x6F4A4E, ('\u{AE68}', false)), // Korean hangul
    (0x334050, ('\u{62D5}', false)), // East Asian ideograph
    (0x23523E, ('\u{99A6}', false)), // East Asian ideograph
    (0x4C2539, ('\u{5D73}', false)), // East Asian ideograph
    (0x4D3363, ('\u{8C25}', false)), // East Asian ideograph
    (0x224A50, ('\u{6E27}', false)), // East Asian ideograph
    (0x6F534F, ('\u{C1E0}', false)), // Korean hangul
    (0x27623A, ('\u{9E33}', false)), // East Asian ideograph
    (0x4B485F, ('\u{6EDE}', false)), // East Asian ideograph (variant of 27485F which maps to 6EDE)
    (0x234A51, ('\u{966C}', false)), // East Asian ideograph
    (0x23235C, ('\u{84B4}', false)), // East Asian ideograph
    (0x285A47, ('\u{73AE}', false)), // East Asian ideograph
    (0x2D3165, ('\u{349E}', false)), // East Asian ideograph (not found in unified han)
    (0x6F4A52, ('\u{AE78}', false)), // Korean hangul
    (0x275571, ('\u{848B}', false)), // East Asian ideograph
    (0x274A53, ('\u{5B83}', false)), // East Asian ideograph
    (0x227059, ('\u{7D35}', false)), // East Asian ideograph
    (0x33523F, ('\u{8B71}', false)), // East Asian ideograph
    (0x473422, ('\u{8C2A}', false)), // East Asian ideograph
    (0x224A55, ('\u{6E49}', false)), // East Asian ideograph
    (0x213D78, ('\u{5FC3}', false)), // East Asian ideograph
    (0x213935, ('\u{5951}', false)), // East Asian ideograph
    (0x223D34, ('\u{686F}', false)), // East Asian ideograph
    (0x4B3248, ('\u{50B2}', false)), // East Asian ideograph (not in Unicode)
    (0x6F4A57, ('\u{AE84}', false)), // Korean hangul
    (0x224A58, ('\u{6E3C}', false)), // East Asian ideograph
    (0x6F5D69, ('\u{D749}', false)), // Korean hangul
    (0x6F4A59, ('\u{AEBC}', false)), // Korean hangul
    (0x274A5A, ('\u{7275}', false)), // East Asian ideograph
    (0x6F5351, ('\u{C1E8}', false)), // Korean hangul
    (0x27623C, ('\u{9E3F}', false)), // East Asian ideograph
    (0x223E21, ('\u{6907}', false)), // East Asian ideograph
    (0x213E22, ('\u{5FEB}', false)), // East Asian ideograph
    (0x213E23, ('\u{5FE0}', false)), // East Asian ideograph
    (0x213E24, ('\u{5FF1}', false)), // East Asian ideograph
    (0x233E25, ('\u{906F}', false)), // East Asian ideograph
    (0x233E26, ('\u{9079}', false)), // East Asian ideograph
    (0x213E27, ('\u{5FF5}', false)), // East Asian ideograph
    (0x233E28, ('\u{9076}', false)), // East Asian ideograph
    (0x213E29, ('\u{6014}', false)), // East Asian ideograph
    (0x223E2A, ('\u{68DE}', false)), // East Asian ideograph
    (0x223E2B, ('\u{691B}', false)), // East Asian ideograph
    (0x233E2C, ('\u{9085}', false)), // East Asian ideograph
    (0x223E2D, ('\u{68FB}', false)), // East Asian ideograph
    (0x213E2E, ('\u{601D}', false)), // East Asian ideograph
    (0x234A5D, ('\u{967B}', false)), // East Asian ideograph
    (0x213E30, ('\u{6021}', false)), // East Asian ideograph
    (0x213E31, ('\u{6020}', false)), // East Asian ideograph
    (0x213E32, ('\u{6028}', false)), // East Asian ideograph
    (0x223E33, ('\u{68E1}', false)), // East Asian ideograph
    (0x213E34, ('\u{6027}', false)), // East Asian ideograph
    (0x214A5E, ('\u{7296}', false)), // East Asian ideograph
    (0x213E36, ('\u{6015}', false)), // East Asian ideograph
    (0x223E37, ('\u{68D1}', false)), // East Asian ideograph
    (0x223E38, ('\u{68D0}', false)), // East Asian ideograph
    (0x223E39, ('\u{6908}', false)), // East Asian ideograph
    (0x233E3A, ('\u{908B}', false)), // East Asian ideograph
    (0x213E3B, ('\u{6043}', false)), // East Asian ideograph
    (0x213E3C, ('\u{6065}', false)), // East Asian ideograph
    (0x213E3D, ('\u{6050}', false)), // East Asian ideograph
    (0x223E3E, ('\u{68E8}', false)), // East Asian ideograph
    (0x223E3F, ('\u{68F0}', false)), // East Asian ideograph
    (0x223E40, ('\u{68C3}', false)), // East Asian ideograph
    (0x214A60, ('\u{729B}', false)), // East Asian ideograph
    (0x235164, ('\u{9940}', false)), // East Asian ideograph
    (0x233E43, ('\u{909B}', false)), // East Asian ideograph
    (0x233E44, ('\u{909C}', false)), // East Asian ideograph
    (0x213E45, ('\u{606F}', false)), // East Asian ideograph
    (0x223E46, ('\u{68D4}', false)), // East Asian ideograph
    (0x274A61, ('\u{728A}', false)), // East Asian ideograph
    (0x233E48, ('\u{90A1}', false)), // East Asian ideograph
    (0x223E49, ('\u{68C6}', false)), // East Asian ideograph
    (0x213E4A, ('\u{608C}', false)), // East Asian ideograph
    (0x223E4B, ('\u{68C7}', false)), // East Asian ideograph
    (0x213E4C, ('\u{607F}', false)), // East Asian ideograph
    (0x214A62, ('\u{72A7}', false)), // East Asian ideograph
    (0x213E4E, ('\u{609A}', false)), // East Asian ideograph
    (0x213E4F, ('\u{6096}', false)), // East Asian ideograph
    (0x213E50, ('\u{6084}', false)), // East Asian ideograph
    (0x233E51, ('\u{90A8}', false)), // East Asian ideograph
    (0x224828, ('\u{6CCE}', false)), // East Asian ideograph
    (0x234A63, ('\u{9684}', false)), // East Asian ideograph
    (0x233E54, ('\u{90A0}', false)), // East Asian ideograph
    (0x223E55, ('\u{6938}', false)), // East Asian ideograph
    (0x213E56, ('\u{60A8}', false)), // East Asian ideograph
    (0x273E57, ('\u{5FF0}', false)), // East Asian ideograph
    (0x233E58, ('\u{90AF}', false)), // East Asian ideograph
    (0x233E59, ('\u{90B3}', false)), // East Asian ideograph
    (0x6F4C5D, ('\u{B2A1}', false)), // Korean hangul
    (0x213E5B, ('\u{60C5}', false)), // East Asian ideograph (variant of 4B3E5B which maps to 60C5)
    (0x273E5C, ('\u{95F7}', false)), // East Asian ideograph
    (0x223E5D, ('\u{6958}', false)), // East Asian ideograph
    (0x273E5E, ('\u{6005}', false)), // East Asian ideograph
    (0x213E5F, ('\u{60BB}', false)), // East Asian ideograph
    (0x213E60, ('\u{60E0}', false)), // East Asian ideograph
    (0x233E61, ('\u{90B2}', false)), // East Asian ideograph
    (0x213E62, ('\u{60DC}', false)), // East Asian ideograph
    (0x213E63, ('\u{60D8}', false)), // East Asian ideograph
    (0x223E64, ('\u{6945}', false)), // East Asian ideograph
    (0x223E65, ('\u{695D}', false)), // East Asian ideograph
    (0x223E66, ('\u{6932}', false)), // East Asian ideograph
    (0x213E67, ('\u{60C6}', false)), // East Asian ideograph
    (0x233E68, ('\u{90C9}', false)), // East Asian ideograph
    (0x223E69, ('\u{696E}', false)), // East Asian ideograph
    (0x223E6A, ('\u{6963}', false)), // East Asian ideograph
    (0x223E6B, ('\u{6948}', false)), // East Asian ideograph
    (0x273E6C, ('\u{60EC}', false)), // East Asian ideograph
    (0x213E6D, ('\u{60F3}', false)), // East Asian ideograph
    (0x223E6E, ('\u{6939}', false)), // East Asian ideograph
    (0x233E6F, ('\u{90D5}', false)), // East Asian ideograph
    (0x273E70, ('\u{607B}', false)), // East Asian ideograph
    (0x214A68, ('\u{72C0}', false)), // East Asian ideograph
    (0x233E72, ('\u{90BE}', false)), // East Asian ideograph
    (0x223E73, ('\u{6937}', false)), // East Asian ideograph
    (0x213E74, ('\u{60F9}', false)), // East Asian ideograph
    (0x213E75, ('\u{6123}', false)), // East Asian ideograph
    (0x213E76, ('\u{60F4}', false)), // East Asian ideograph
    (0x273E77, ('\u{7231}', false)), // East Asian ideograph
    (0x233E78, ('\u{90C8}', false)), // East Asian ideograph
    (0x233E79, ('\u{90C3}', false)), // East Asian ideograph
    (0x223E7A, ('\u{696C}', false)), // East Asian ideograph
    (0x223E7B, ('\u{694E}', false)), // East Asian ideograph
    (0x213E7C, ('\u{6109}', false)), // East Asian ideograph
    (0x224A6A, ('\u{6E51}', false)), // East Asian ideograph
    (0x273E7E, ('\u{607C}', false)), // East Asian ideograph
    (0x234137, ('\u{91A8}', false)), // East Asian ideograph
    (0x224A6B, ('\u{6E44}', false)), // East Asian ideograph
    (0x275576, ('\u{8361}', false)), // East Asian ideograph
    (0x27734C, ('\u{5456}', false)), // East Asian ideograph
    (0x21385B, ('\u{5879}', false)), // East Asian ideograph
    (0x293B5B, ('\u{8F81}', false)), // East Asian ideograph
    (0x234A6D, ('\u{9682}', false)), // East Asian ideograph
    (0x234A6E, ('\u{9683}', false)), // East Asian ideograph
    (0x6F5355, ('\u{C1FC}', false)), // Korean hangul
    (0x335238, ('\u{8989}', false)), // East Asian ideograph
    (0x694C68, ('\u{5301}', false)), // East Asian ideograph
    (0x21393A, ('\u{5958}', false)), // East Asian ideograph
    (0x2D5C5A, ('\u{8FE9}', false)), // East Asian ideograph
    (0x2D3A41, ('\u{5AFA}', false)), // East Asian ideograph
    (0x214A70, ('\u{72F9}', false)), // East Asian ideograph
    (0x6F4A71, ('\u{AF48}', false)), // Korean hangul
    (0x213F2D, ('\u{6177}', false)), // East Asian ideograph
    (0x295932, ('\u{9CD8}', false)), // East Asian ideograph
    (0x274A72, ('\u{72C8}', false)), // East Asian ideograph
    (0x23302C, ('\u{895C}', false)), // East Asian ideograph
    (0x276239, ('\u{9E2F}', false)), // East Asian ideograph
    (0x6F4A73, ('\u{AF4C}', false)), // Korean hangul
    (0x6F5356, ('\u{C1FD}', false)), // Korean hangul
    (0x276241, ('\u{9E45}', false)), // East Asian ideograph
    (0x21393B, ('\u{595A}', false)), // East Asian ideograph
    (0x4B324E, ('\u{50E7}', false)), // East Asian ideograph (variant of 21324E which maps to 50E7)
    (0x2E4E72, ('\u{6F74}', false)), // East Asian ideograph
    (0x6F5774, ('\u{C911}', false)), // Korean hangul
    (0x6F4A75, ('\u{AF5C}', false)), // Korean hangul
    (0x6F4A76, ('\u{AF5D}', false)), // Korean hangul
    (0x213521, ('\u{53C9}', false)), // East Asian ideograph
    (0x224A77, ('\u{6E4E}', false)), // East Asian ideograph
    (0x23302D, ('\u{895D}', false)), // East Asian ideograph
    (0x4B4A78, ('\u{72F0}', false)), // East Asian ideograph
    (0x6F5357, ('\u{C200}', false)), // Korean hangul
    (0x213523, ('\u{53CA}', false)), // East Asian ideograph
    (0x276242, ('\u{9E51}', false)), // East Asian ideograph
    (0x275D49, ('\u{94AE}', false)), // East Asian ideograph
    (0x274A79, ('\u{72B9}', false)), // East Asian ideograph
    (0x223D3B, ('\u{6874}', false)), // East Asian ideograph
    (0x234A7A, ('\u{9697}', false)), // East Asian ideograph
    (0x224D68, ('\u{6F5D}', false)), // East Asian ideograph
    (0x6F4A7B, ('\u{AF84}', false)), // Korean hangul
    (0x213526, ('\u{53D4}', false)), // East Asian ideograph
    (0x227061, ('\u{7D3A}', false)), // East Asian ideograph
    (0x6F4A7C, ('\u{AF88}', false)), // Korean hangul
    (0x225A30, ('\u{7415}', false)), // East Asian ideograph
    (0x213527, ('\u{53D7}', false)), // East Asian ideograph
    (0x227C49, ('\u{82BC}', false)), // East Asian ideograph
    (0x6F4A7D, ('\u{AF90}', false)), // Korean hangul
    (0x6F5358, ('\u{C204}', false)), // Korean hangul
    (0x6F7623, ('\u{317F}', false)), // Korean hangul
    (0x274A7E, ('\u{72F1}', false)), // East Asian ideograph
    (0x223D3C, ('\u{6875}', false)), // East Asian ideograph
    (0x227D2B, ('\u{8344}', false)), // East Asian ideograph
    (0x21352A, ('\u{66FC}', false)), // East Asian ideograph
    (0x213936, ('\u{594E}', false)), // East Asian ideograph
    (0x21352B, ('\u{53E2}', false)), // East Asian ideograph
    (0x2D4B5B, ('\u{78AF}', false)), // East Asian ideograph
    (0x6F5359, ('\u{C20D}', false)), // Korean hangul
    (0x23352D, ('\u{8B95}', false)), // East Asian ideograph
    (0x276244, ('\u{9E4C}', false)), // East Asian ideograph
    (0x23352E, ('\u{8B94}', false)), // East Asian ideograph
    (0x3A7970, ('\u{81D5}', false)), // East Asian ideograph
    (0x21352F, ('\u{53EE}', false)), // East Asian ideograph
    (0x6F5331, ('\u{C138}', false)), // Korean hangul
    (0x275138, ('\u{7EDC}', false)), // East Asian ideograph
    (0x223F21, ('\u{6952}', false)), // East Asian ideograph
    (0x213F22, ('\u{6168}', false)), // East Asian ideograph
    (0x233F23, ('\u{90DF}', false)), // East Asian ideograph
    (0x213F24, ('\u{613C}', false)), // East Asian ideograph
    (0x223F25, ('\u{695B}', false)), // East Asian ideograph
    (0x233F26, ('\u{90E2}', false)), // East Asian ideograph
    (0x223531, ('\u{64EB}', false)), // East Asian ideograph
    (0x233F28, ('\u{90DB}', false)), // East Asian ideograph
    (0x273F29, ('\u{5FFE}', false)), // East Asian ideograph
    (0x233F2A, ('\u{90DC}', false)), // East Asian ideograph
    (0x273F2B, ('\u{6006}', false)), // East Asian ideograph
    (0x233F2C, ('\u{90D7}', false)), // East Asian ideograph
    (0x233F2D, ('\u{90E4}', false)), // East Asian ideograph
    (0x233F2E, ('\u{90EF}', false)), // East Asian ideograph
    (0x233F2F, ('\u{90EA}', false)), // East Asian ideograph
    (0x213F30, ('\u{6170}', false)), // East Asian ideograph
    (0x213F31, ('\u{615A}', false)), // East Asian ideograph
    (0x233F32, ('\u{90F0}', false)), // East Asian ideograph
    (0x233F33, ('\u{90F4}', false)), // East Asian ideograph
    (0x233F34, ('\u{90F2}', false)), // East Asian ideograph
    (0x223F35, ('\u{6978}', false)), // East Asian ideograph
    (0x273F36, ('\u{8651}', false)), // East Asian ideograph
    (0x223F37, ('\u{697B}', false)), // East Asian ideograph
    (0x273F38, ('\u{60E8}', false)), // East Asian ideograph
    (0x273F39, ('\u{60EF}', false)), // East Asian ideograph
    (0x273F3A, ('\u{6078}', false)), // East Asian ideograph
    (0x273F3B, ('\u{6002}', false)), // East Asian ideograph
    (0x273F3C, ('\u{6B32}', false)), // East Asian ideograph
    (0x223F3D, ('\u{6944}', false)), // East Asian ideograph
    (0x233F3E, ('\u{90EB}', false)), // East Asian ideograph
    (0x233F3F, ('\u{90F3}', false)), // East Asian ideograph
    (0x213F40, ('\u{618E}', false)), // East Asian ideograph
    (0x273F41, ('\u{60AF}', false)), // East Asian ideograph
    (0x273F42, ('\u{6124}', false)), // East Asian ideograph
    (0x213F43, ('\u{61AC}', false)), // East Asian ideograph
    (0x273F44, ('\u{60EE}', false)), // East Asian ideograph
    (0x273F45, ('\u{6187}', false)), // East Asian ideograph
    (0x233F46, ('\u{90FC}', false)), // East Asian ideograph
    (0x273F47, ('\u{60EB}', false)), // East Asian ideograph
    (0x273F48, ('\u{5FC6}', false)), // East Asian ideograph
    (0x233F49, ('\u{9104}', false)), // East Asian ideograph
    (0x273F4A, ('\u{5E94}', false)), // East Asian ideograph
    (0x213537, ('\u{53EB}', false)), // East Asian ideograph
    (0x233F4C, ('\u{9106}', false)), // East Asian ideograph
    (0x213F4D, ('\u{61C2}', false)), // East Asian ideograph
    (0x273F4E, ('\u{6073}', false)), // East Asian ideograph
    (0x213F4F, ('\u{61C8}', false)), // East Asian ideograph
    (0x213940, ('\u{596A}', false)), // East Asian ideograph
    (0x232368, ('\u{84CD}', false)), // East Asian ideograph
    (0x213F52, ('\u{61E6}', false)), // East Asian ideograph
    (0x213F53, ('\u{61F2}', false)), // East Asian ideograph (variant of 4B3F53 which maps to 61F2)
    (0x273F54, ('\u{6000}', false)), // East Asian ideograph
    (0x273F55, ('\u{61D2}', false)), // East Asian ideograph
    (0x273F56, ('\u{60AC}', false)), // East Asian ideograph
    (0x233F57, ('\u{910F}', false)), // East Asian ideograph
    (0x273F58, ('\u{5FCF}', false)), // East Asian ideograph
    (0x273F59, ('\u{6151}', false)), // East Asian ideograph
    (0x233F5A, ('\u{9116}', false)), // East Asian ideograph
    (0x273F5B, ('\u{60E7}', false)), // East Asian ideograph
    (0x233F5C, ('\u{9114}', false)), // East Asian ideograph
    (0x23353A, ('\u{8B9F}', false)), // East Asian ideograph
    (0x213F5E, ('\u{620A}', false)), // East Asian ideograph
    (0x213F5F, ('\u{620E}', false)), // East Asian ideograph
    (0x223F60, ('\u{69BC}', false)), // East Asian ideograph
    (0x223F61, ('\u{69A7}', false)), // East Asian ideograph
    (0x233F62, ('\u{9123}', false)), // East Asian ideograph (Version J extension)
    (0x233F63, ('\u{9118}', false)), // East Asian ideograph
    (0x233F64, ('\u{911C}', false)), // East Asian ideograph
    (0x213F65, ('\u{6216}', false)), // East Asian ideograph
    (0x233F66, ('\u{9120}', false)), // East Asian ideograph
    (0x233F67, ('\u{9122}', false)), // East Asian ideograph
    (0x223F68, ('\u{69D9}', false)), // East Asian ideograph
    (0x213F69, ('\u{621F}', false)), // East Asian ideograph
    (0x223F6A, ('\u{698E}', false)), // East Asian ideograph
    (0x213F6B, ('\u{6222}', false)), // East Asian ideograph
    (0x213F6C, ('\u{622A}', false)), // East Asian ideograph
    (0x223F6D, ('\u{69D6}', false)), // East Asian ideograph
    (0x273F6E, ('\u{6218}', false)), // East Asian ideograph
    (0x273F6F, ('\u{620F}', false)), // East Asian ideograph
    (0x213F70, ('\u{6234}', false)), // East Asian ideograph
    (0x233F71, ('\u{9124}', false)), // East Asian ideograph
    (0x233F72, ('\u{911A}', false)), // East Asian ideograph
    (0x213F73, ('\u{623F}', false)), // East Asian ideograph
    (0x233F74, ('\u{9125}', false)), // East Asian ideograph
    (0x223F75, ('\u{69A5}', false)), // East Asian ideograph
    (0x213F76, ('\u{6241}', false)), // East Asian ideograph
    (0x233F77, ('\u{912F}', false)), // East Asian ideograph
    (0x223F78, ('\u{69D1}', false)), // East Asian ideograph
    (0x27513B, ('\u{4E1D}', false)), // East Asian ideograph
    (0x223F7A, ('\u{69F6}', false)), // East Asian ideograph
    (0x21753F, ('\u{579E}', false)), // East Asian ideograph
    (0x213F7D, ('\u{6253}', false)), // East Asian ideograph
    (0x223F7E, ('\u{69D5}', false)), // East Asian ideograph
    (0x217540, ('\u{57B5}', false)), // East Asian ideograph
    (0x6F535D, ('\u{C21C}', false)), // Korean hangul
    (0x276248, ('\u{9E5E}', false)), // East Asian ideograph
    (0x223542, ('\u{64F7}', false)), // East Asian ideograph
    (0x213543, ('\u{540C}', false)), // East Asian ideograph
    (0x213544, ('\u{540A}', false)), // East Asian ideograph
    (0x29593A, ('\u{9C85}', false)), // East Asian ideograph
    (0x23524D, ('\u{99BF}', false)), // East Asian ideograph
    (0x213545, ('\u{540D}', false)), // East Asian ideograph
    (0x6F535E, ('\u{C21F}', false)), // Korean hangul
    (0x223546, ('\u{6504}', false)), // East Asian ideograph
    (0x234E5E, ('\u{97E0}', false)), // East Asian ideograph
    (0x2D3547, ('\u{55AB}', false)), // East Asian ideograph
    (0x6F5C66, ('\u{D560}', false)), // Korean hangul
    (0x234141, ('\u{91AF}', false)), // East Asian ideograph
    (0x692436, ('\u{3056}', false)), // Hiragana letter ZA
    (0x696868, ('\u{84D9}', false)), // East Asian ideograph
    (0x233478, ('\u{8B85}', false)), // East Asian ideograph
    (0x4C354A, ('\u{64B8}', false)), // East Asian ideograph
    (0x22587D, ('\u{73CC}', false)), // East Asian ideograph
    (0x22354B, ('\u{64FD}', false)), // East Asian ideograph
    (0x333021, ('\u{58F9}', false)), // East Asian ideograph
    (0x225F5C, ('\u{7633}', false)), // East Asian ideograph
    (0x217933, ('\u{5940}', false)), // East Asian ideograph
    (0x234142, ('\u{91B1}', false)), // East Asian ideograph
    (0x23346B, ('\u{8B6D}', false)), // East Asian ideograph
    (0x23354D, ('\u{8C4B}', false)), // East Asian ideograph
    (0x2D7A44, ('\u{598D}', false)), // East Asian ideograph
    (0x27513E, ('\u{7EE2}', false)), // East Asian ideograph
    (0x23472C, ('\u{93D3}', false)), // East Asian ideograph
    (0x22354F, ('\u{6508}', false)), // East Asian ideograph
    (0x395E42, ('\u{9274}', false)), // East Asian ideograph
    (0x233550, ('\u{8C4F}', false)), // East Asian ideograph
    (0x3F5F35, ('\u{6B9E}', false)), // East Asian ideograph
    (0x39303A, ('\u{5EFC}', false)), // East Asian ideograph
    (0x213552, ('\u{543E}', false)), // East Asian ideograph
    (0x27513F, ('\u{7EE5}', false)), // East Asian ideograph
    (0x213553, ('\u{5427}', false)), // East Asian ideograph
    (0x213554, ('\u{5440}', false)), // East Asian ideograph
    (0x274476, ('\u{6808}', false)), // East Asian ideograph
    (0x233555, ('\u{8C5C}', false)), // East Asian ideograph
    (0x6F5A26, ('\u{CEE8}', false)), // Korean hangul
    (0x213556, ('\u{5446}', false)), // East Asian ideograph
    (0x217557, ('\u{57A1}', false)), // East Asian ideograph
    (0x213266, ('\u{512A}', false)), // East Asian ideograph
    (0x293C30, ('\u{8F98}', false)), // East Asian ideograph
    (0x224B38, ('\u{6E69}', false)), // East Asian ideograph
    (0x6F5332, ('\u{C139}', false)), // Korean hangul
    (0x293725, ('\u{8D3D}', false)), // East Asian ideograph
    (0x287229, ('\u{7F17}', false)), // East Asian ideograph
    (0x223559, ('\u{651A}', false)), // East Asian ideograph
    (0x6F5362, ('\u{C22B}', false)), // Korean hangul
    (0x27624D, ('\u{9E6D}', false)), // East Asian ideograph
    (0x214021, ('\u{6252}', false)), // East Asian ideograph
    (0x214022, ('\u{625B}', false)), // East Asian ideograph
    (0x214023, ('\u{6263}', false)), // East Asian ideograph
    (0x214024, ('\u{6258}', false)), // East Asian ideograph
    (0x214025, ('\u{6296}', false)), // East Asian ideograph
    (0x214026, ('\u{6297}', false)), // East Asian ideograph
    (0x214027, ('\u{6292}', false)), // East Asian ideograph
    (0x214028, ('\u{6276}', false)), // East Asian ideograph
    (0x214029, ('\u{6289}', false)), // East Asian ideograph
    (0x21402A, ('\u{627F}', false)), // East Asian ideograph
    (0x21402B, ('\u{6279}', false)), // East Asian ideograph
    (0x21402C, ('\u{6280}', false)), // East Asian ideograph
    (0x21402D, ('\u{628A}', false)), // East Asian ideograph
    (0x21402E, ('\u{626D}', false)), // East Asian ideograph
    (0x21402F, ('\u{627C}', false)), // East Asian ideograph
    (0x214030, ('\u{627E}', false)), // East Asian ideograph
    (0x214031, ('\u{626F}', false)), // East Asian ideograph
    (0x214032, ('\u{6284}', false)), // East Asian ideograph
    (0x214033, ('\u{6295}', false)), // East Asian ideograph
    (0x214034, ('\u{6291}', false)), // East Asian ideograph
    (0x214035, ('\u{6298}', false)), // East Asian ideograph
    (0x214036, ('\u{626E}', false)), // East Asian ideograph
    (0x214037, ('\u{6273}', false)), // East Asian ideograph
    (0x214038, ('\u{6293}', false)), // East Asian ideograph
    (0x214039, ('\u{62C9}', false)), // East Asian ideograph
    (0x21403A, ('\u{62C4}', false)), // East Asian ideograph
    (0x21403B, ('\u{62CC}', false)), // East Asian ideograph
    (0x21403C, ('\u{62A8}', false)), // East Asian ideograph
    (0x21403D, ('\u{62DC}', false)), // East Asian ideograph
    (0x21403E, ('\u{62BF}', false)), // East Asian ideograph
    (0x21403F, ('\u{62C2}', false)), // East Asian ideograph
    (0x214040, ('\u{62B9}', false)), // East Asian ideograph
    (0x214041, ('\u{62D2}', false)), // East Asian ideograph
    (0x214042, ('\u{62D3}', false)), // East Asian ideograph
    (0x214043, ('\u{62DB}', false)), // East Asian ideograph
    (0x214044, ('\u{62AB}', false)), // East Asian ideograph
    (0x214045, ('\u{62CB}', false)), // East Asian ideograph
    (0x214046, ('\u{62D4}', false)), // East Asian ideograph
    (0x214047, ('\u{62BD}', false)), // East Asian ideograph
    (0x214048, ('\u{62BC}', false)), // East Asian ideograph
    (0x214049, ('\u{62D0}', false)), // East Asian ideograph (variant of 4B4049 which maps to 62D0)
    (0x21404A, ('\u{62C8}', false)), // East Asian ideograph
    (0x21404B, ('\u{62D9}', false)), // East Asian ideograph
    (0x21404C, ('\u{62DA}', false)), // East Asian ideograph
    (0x21404D, ('\u{62AC}', false)), // East Asian ideograph
    (0x21404E, ('\u{62C7}', false)), // East Asian ideograph
    (0x21404F, ('\u{62B1}', false)), // East Asian ideograph
    (0x214050, ('\u{62D6}', false)), // East Asian ideograph
    (0x214051, ('\u{62D8}', false)), // East Asian ideograph
    (0x214052, ('\u{62CD}', false)), // East Asian ideograph
    (0x214053, ('\u{62B5}', false)), // East Asian ideograph
    (0x214054, ('\u{62CE}', false)), // East Asian ideograph
    (0x214055, ('\u{62D7}', false)), // East Asian ideograph
    (0x214056, ('\u{62C6}', false)), // East Asian ideograph
    (0x214057, ('\u{6309}', false)), // East Asian ideograph
    (0x214058, ('\u{6316}', false)), // East Asian ideograph
    (0x214059, ('\u{62FC}', false)), // East Asian ideograph
    (0x21405A, ('\u{62F3}', false)), // East Asian ideograph
    (0x21405B, ('\u{6308}', false)), // East Asian ideograph
    (0x21405C, ('\u{62ED}', false)), // East Asian ideograph
    (0x21405D, ('\u{6301}', false)), // East Asian ideograph
    (0x21405E, ('\u{62EE}', false)), // East Asian ideograph
    (0x21405F, ('\u{62EF}', false)), // East Asian ideograph
    (0x214060, ('\u{62F7}', false)), // East Asian ideograph
    (0x214061, ('\u{6307}', false)), // East Asian ideograph
    (0x214062, ('\u{62F1}', false)), // East Asian ideograph
    (0x214063, ('\u{62FD}', false)), // East Asian ideograph
    (0x214064, ('\u{6311}', false)), // East Asian ideograph
    (0x214065, ('\u{62EC}', false)), // East Asian ideograph
    (0x214066, ('\u{62F4}', false)), // East Asian ideograph (variant of 4B4066 which maps to 62F4)
    (0x214067, ('\u{62FF}', false)), // East Asian ideograph
    (0x224068, ('\u{6A2D}', false)), // East Asian ideograph
    (0x214069, ('\u{6342}', false)), // East Asian ideograph
    (0x21406A, ('\u{632A}', false)), // East Asian ideograph
    (0x21406B, ('\u{6355}', false)), // East Asian ideograph
    (0x21406C, ('\u{633E}', false)), // East Asian ideograph
    (0x21406D, ('\u{632F}', false)), // East Asian ideograph
    (0x21406E, ('\u{634E}', false)), // East Asian ideograph
    (0x21406F, ('\u{634F}', false)), // East Asian ideograph
    (0x214070, ('\u{6350}', false)), // East Asian ideograph
    (0x214071, ('\u{6349}', false)), // East Asian ideograph
    (0x224072, ('\u{6A1D}', false)), // East Asian ideograph
    (0x214073, ('\u{632B}', false)), // East Asian ideograph
    (0x214074, ('\u{6328}', false)), // East Asian ideograph
    (0x214075, ('\u{633A}', false)), // East Asian ideograph
    (0x214076, ('\u{63A5}', false)), // East Asian ideograph
    (0x214077, ('\u{6369}', false)), // East Asian ideograph
    (0x214078, ('\u{63A0}', false)), // East Asian ideograph
    (0x214079, ('\u{6396}', false)), // East Asian ideograph
    (0x21407A, ('\u{63A7}', false)), // East Asian ideograph
    (0x21407B, ('\u{6372}', false)), // East Asian ideograph
    (0x21407C, ('\u{6377}', false)), // East Asian ideograph
    (0x21407D, ('\u{6383}', false)), // East Asian ideograph
    (0x21407E, ('\u{636B}', false)), // East Asian ideograph
    (0x2D5C74, ('\u{96A3}', false)), // East Asian ideograph
    (0x2D5831, ('\u{89A7}', false)), // East Asian ideograph
    (0x21756C, ('\u{57BE}', false)), // East Asian ideograph
    (0x693729, ('\u{7C82}', false)), // East Asian ideograph
    (0x23356D, ('\u{8C73}', false)), // East Asian ideograph
    (0x6F5966, ('\u{CE5C}', false)), // Korean hangul
    (0x29252D, ('\u{8311}', false)), // East Asian ideograph
    (0x6F5366, ('\u{C232}', false)), // Korean hangul
    (0x276251, ('\u{76D0}', false)), // East Asian ideograph
    (0x6F5463, ('\u{C46C}', false)), // Korean hangul
    (0x6F4F23, ('\u{B800}', false)), // Korean hangul
    (0x21356F, ('\u{547B}', false)), // East Asian ideograph
    (0x6F4C71, ('\u{B2EB}', false)), // Korean hangul
    (0x233571, ('\u{8C75}', false)), // East Asian ideograph
    (0x28722A, ('\u{7F02}', false)), // East Asian ideograph
    (0x213572, ('\u{5484}', false)), // East Asian ideograph
    (0x6F4A54, ('\u{AE7B}', false)), // Korean hangul
    (0x39593F, ('\u{8A3C}', false)), // East Asian ideograph
    (0x233573, ('\u{8C77}', false)), // East Asian ideograph
    (0x276252, ('\u{7877}', false)), // East Asian ideograph
    (0x213D7B, ('\u{5FD8}', false)), // East Asian ideograph
    (0x6F4F24, ('\u{B801}', false)), // Korean hangul
    (0x213574, ('\u{5468}', false)), // East Asian ideograph
    (0x223D4B, ('\u{68B4}', false)), // East Asian ideograph
    (0x692568, ('\u{30E8}', false)), // Katakana letter YO
    (0x213575, ('\u{5486}', false)), // East Asian ideograph
    (0x213939, ('\u{5957}', false)), // East Asian ideograph
    (0x6F5A2F, ('\u{CF01}', false)), // Korean hangul
    (0x393B6E, ('\u{5C97}', false)), // East Asian ideograph
    (0x2D6021, ('\u{978C}', false)), // East Asian ideograph
    (0x335652, ('\u{87C1}', false)), // East Asian ideograph
    (0x223577, ('\u{652E}', false)), // East Asian ideograph
    (0x216022, ('\u{978B}', false)), // East Asian ideograph
    (0x216023, ('\u{978F}', false)), // East Asian ideograph
    (0x215B66, ('\u{8FC6}', false)), // East Asian ideograph
    (0x694838, ('\u{567A}', false)), // East Asian ideograph
    (0x216024, ('\u{9798}', false)), // East Asian ideograph
    (0x4B5036, ('\u{7C14}', false)), // East Asian ideograph
    (0x277360, ('\u{5181}', false)), // East Asian ideograph
    (0x21357B, ('\u{5471}', false)), // East Asian ideograph
    (0x21357C, ('\u{549A}', false)), // East Asian ideograph
    (0x454E43, ('\u{788C}', false)), // East Asian ideograph (variant of 214E43 which maps to 788C)
    (0x21357D, ('\u{548E}', false)), // East Asian ideograph
    (0x216028, ('\u{97AD}', false)), // East Asian ideograph
    (0x6F4F26, ('\u{B808}', false)), // Korean hangul
    (0x215724, ('\u{87A2}', false)), // East Asian ideograph
    (0x275E7B, ('\u{9635}', false)), // East Asian ideograph
    (0x6F5D6E, ('\u{D758}', false)), // Korean hangul
    (0x21602B, ('\u{97C6}', false)), // East Asian ideograph
    (0x335259, ('\u{7E59}', false)), // East Asian ideograph
    (0x27602C, ('\u{97E6}', false)), // East Asian ideograph
    (0x27623D, ('\u{9E3D}', false)), // East Asian ideograph
    (0x4B4347, ('\u{66A8}', false)), // East Asian ideograph
    (0x6F536A, ('\u{C26C}', false)), // Korean hangul
    (0x27602D, ('\u{97E7}', false)), // East Asian ideograph
    (0x6F4F27, ('\u{B809}', false)), // Korean hangul
    (0x6F592B, ('\u{CC3E}', false)), // Korean hangul
    (0x213938, ('\u{5950}', false)), // East Asian ideograph
    (0x2D3A60, ('\u{6588}', false)), // East Asian ideograph
    (0x27602F, ('\u{97EC}', false)), // East Asian ideograph
    (0x214121, ('\u{6367}', false)), // East Asian ideograph
    (0x214122, ('\u{6398}', false)), // East Asian ideograph
    (0x214123, ('\u{639B}', false)), // East Asian ideograph
    (0x214124, ('\u{63AA}', false)), // East Asian ideograph
    (0x214125, ('\u{6371}', false)), // East Asian ideograph
    (0x214126, ('\u{63A9}', false)), // East Asian ideograph
    (0x214127, ('\u{638C}', false)), // East Asian ideograph
    (0x214128, ('\u{6389}', false)), // East Asian ideograph
    (0x214129, ('\u{63A2}', false)), // East Asian ideograph
    (0x21412A, ('\u{6399}', false)), // East Asian ideograph
    (0x21412B, ('\u{63A1}', false)), // East Asian ideograph
    (0x21412C, ('\u{6388}', false)), // East Asian ideograph
    (0x21412D, ('\u{63AC}', false)), // East Asian ideograph
    (0x21412E, ('\u{633D}', false)), // East Asian ideograph
    (0x21412F, ('\u{6392}', false)), // East Asian ideograph
    (0x214130, ('\u{63A3}', false)), // East Asian ideograph
    (0x214131, ('\u{6376}', false)), // East Asian ideograph
    (0x214132, ('\u{638F}', false)), // East Asian ideograph
    (0x214133, ('\u{63A8}', false)), // East Asian ideograph
    (0x214134, ('\u{637B}', false)), // East Asian ideograph
    (0x214135, ('\u{6368}', false)), // East Asian ideograph (variant of 4B4135 which maps to 6368)
    (0x214136, ('\u{6384}', false)), // East Asian ideograph
    (0x214137, ('\u{6380}', false)), // East Asian ideograph
    (0x214138, ('\u{63C6}', false)), // East Asian ideograph
    (0x214139, ('\u{63C9}', false)), // East Asian ideograph
    (0x21413A, ('\u{63CD}', false)), // East Asian ideograph
    (0x21413B, ('\u{63E1}', false)), // East Asian ideograph
    (0x21413C, ('\u{63C0}', false)), // East Asian ideograph
    (0x21413D, ('\u{63E9}', false)), // East Asian ideograph
    (0x21413E, ('\u{63D0}', false)), // East Asian ideograph
    (0x21413F, ('\u{63DA}', false)), // East Asian ideograph
    (0x214140, ('\u{63D6}', false)), // East Asian ideograph
    (0x214141, ('\u{63ED}', false)), // East Asian ideograph
    (0x214142, ('\u{63EE}', false)), // East Asian ideograph
    (0x214143, ('\u{63CF}', false)), // East Asian ideograph
    (0x214144, ('\u{63E3}', false)), // East Asian ideograph
    (0x214145, ('\u{63F4}', false)), // East Asian ideograph
    (0x214146, ('\u{63DB}', false)), // East Asian ideograph (variant of 454146 which maps to 63DB)
    (0x214147, ('\u{63D2}', false)), // East Asian ideograph
    (0x234148, ('\u{91AE}', false)), // East Asian ideograph
    (0x214149, ('\u{641E}', false)), // East Asian ideograph
    (0x21414A, ('\u{642A}', false)), // East Asian ideograph
    (0x23414B, ('\u{91B4}', false)), // East Asian ideograph
    (0x23414C, ('\u{91B2}', false)), // East Asian ideograph
    (0x21414D, ('\u{640F}', false)), // East Asian ideograph
    (0x21414E, ('\u{6414}', false)), // East Asian ideograph
    (0x21414F, ('\u{640D}', false)), // East Asian ideograph
    (0x214150, ('\u{642D}', false)), // East Asian ideograph
    (0x214151, ('\u{643D}', false)), // East Asian ideograph
    (0x214152, ('\u{6416}', false)), // East Asian ideograph
    (0x214153, ('\u{6417}', false)), // East Asian ideograph
    (0x214154, ('\u{641C}', false)), // East Asian ideograph
    (0x214155, ('\u{6436}', false)), // East Asian ideograph
    (0x214156, ('\u{642C}', false)), // East Asian ideograph
    (0x214157, ('\u{6458}', false)), // East Asian ideograph
    (0x214158, ('\u{6469}', false)), // East Asian ideograph
    (0x214159, ('\u{6454}', false)), // East Asian ideograph
    (0x21415A, ('\u{6452}', false)), // East Asian ideograph
    (0x21415B, ('\u{646F}', false)), // East Asian ideograph
    (0x21415C, ('\u{6478}', false)), // East Asian ideograph
    (0x21415D, ('\u{6479}', false)), // East Asian ideograph
    (0x21415E, ('\u{647A}', false)), // East Asian ideograph
    (0x21415F, ('\u{645F}', false)), // East Asian ideograph
    (0x214160, ('\u{6451}', false)), // East Asian ideograph
    (0x214161, ('\u{6467}', false)), // East Asian ideograph
    (0x214162, ('\u{649E}', false)), // East Asian ideograph
    (0x214163, ('\u{64A4}', false)), // East Asian ideograph
    (0x214164, ('\u{6487}', false)), // East Asian ideograph
    (0x214165, ('\u{6488}', false)), // East Asian ideograph
    (0x214166, ('\u{64A5}', false)), // East Asian ideograph
    (0x214167, ('\u{64B0}', false)), // East Asian ideograph
    (0x214168, ('\u{6493}', false)), // East Asian ideograph
    (0x214169, ('\u{6495}', false)), // East Asian ideograph
    (0x21416A, ('\u{6492}', false)), // East Asian ideograph
    (0x21416B, ('\u{64A9}', false)), // East Asian ideograph
    (0x21416C, ('\u{6491}', false)), // East Asian ideograph
    (0x21416D, ('\u{64AE}', false)), // East Asian ideograph
    (0x21416E, ('\u{64B2}', false)), // East Asian ideograph
    (0x21416F, ('\u{64AD}', false)), // East Asian ideograph
    (0x214170, ('\u{649A}', false)), // East Asian ideograph
    (0x214171, ('\u{64AB}', false)), // East Asian ideograph
    (0x214172, ('\u{64AC}', false)), // East Asian ideograph
    (0x214173, ('\u{64C5}', false)), // East Asian ideograph
    (0x214174, ('\u{64C1}', false)), // East Asian ideograph
    (0x214175, ('\u{64D8}', false)), // East Asian ideograph
    (0x214176, ('\u{64CA}', false)), // East Asian ideograph
    (0x214177, ('\u{64BB}', false)), // East Asian ideograph
    (0x214178, ('\u{64C2}', false)), // East Asian ideograph
    (0x214179, ('\u{64BC}', false)), // East Asian ideograph
    (0x21417A, ('\u{64CB}', false)), // East Asian ideograph
    (0x21417B, ('\u{64CD}', false)), // East Asian ideograph
    (0x21417C, ('\u{64DA}', false)), // East Asian ideograph
    (0x21417D, ('\u{64C4}', false)), // East Asian ideograph
    (0x21417E, ('\u{64C7}', false)), // East Asian ideograph
    (0x705C50, ('\u{82C4}', false)), // East Asian ideograph
    (0x216040, ('\u{9813}', false)), // East Asian ideograph
    (0x393E61, ('\u{60AA}', false)), // East Asian ideograph
    (0x6F536E, ('\u{C27D}', false)), // Korean hangul
    (0x216041, ('\u{9812}', false)), // East Asian ideograph
    (0x2D3571, ('\u{546A}', false)), // East Asian ideograph
    (0x6F4F2B, ('\u{B819}', false)), // Korean hangul
    (0x29483E, ('\u{9554}', false)), // East Asian ideograph
    (0x276042, ('\u{9882}', false)), // East Asian ideograph
    (0x276043, ('\u{9887}', false)), // East Asian ideograph
    (0x6F5D6F, ('\u{D759}', false)), // Korean hangul
    (0x276044, ('\u{9886}', false)), // East Asian ideograph
    (0x69594B, ('\u{6327}', false)), // East Asian ideograph
    (0x276045, ('\u{9889}', false)), // East Asian ideograph
    (0x27623E, ('\u{9E49}', false)), // East Asian ideograph
    (0x6F536F, ('\u{C27F}', false)), // Korean hangul
    (0x276046, ('\u{5934}', false)), // East Asian ideograph
    (0x6F4F2C, ('\u{B81B}', false)), // Korean hangul
    (0x213954, ('\u{5999}', false)), // East Asian ideograph
    (0x29483F, ('\u{9572}', false)), // East Asian ideograph
    (0x236047, ('\u{9F76}', false)), // East Asian ideograph
    (0x6F5775, ('\u{C918}', false)), // Korean hangul
    (0x216048, ('\u{9838}', false)), // East Asian ideograph
    (0x6F503A, ('\u{BAA8}', false)), // Korean hangul
    (0x216049, ('\u{983B}', false)), // East Asian ideograph
    (0x213E58, ('\u{60E6}', false)), // East Asian ideograph
    (0x222C47, ('\u{60D3}', false)), // East Asian ideograph
    (0x21604A, ('\u{9839}', false)), // East Asian ideograph
    (0x6F5370, ('\u{C281}', false)), // Korean hangul
    (0x27625B, ('\u{9EA6}', false)), // East Asian ideograph
    (0x27604B, ('\u{9894}', false)), // East Asian ideograph
    (0x6F4F2D, ('\u{B81D}', false)), // Korean hangul
    (0x27604C, ('\u{9890}', false)), // East Asian ideograph
    (0x225B2A, ('\u{748A}', false)), // East Asian ideograph
    (0x27604D, ('\u{9897}', false)), // East Asian ideograph
    (0x213269, ('\u{5132}', false)), // East Asian ideograph
    (0x4C725D, ('\u{7A39}', false)), // East Asian ideograph
    (0x27604E, ('\u{989C}', false)), // East Asian ideograph
    (0x27604F, ('\u{989D}', false)), // East Asian ideograph
    (0x2D4730, ('\u{51B5}', false)), // East Asian ideograph
    (0x27625C, ('\u{9EB8}', false)), // East Asian ideograph
    (0x276050, ('\u{9898}', false)), // East Asian ideograph
    (0x276051, ('\u{989A}', false)), // East Asian ideograph
    (0x216052, ('\u{9853}', false)), // East Asian ideograph
    (0x393B78, ('\u{5CC4}', false)), // East Asian ideograph (duplicate simplified)
    (0x276053, ('\u{7C7B}', false)), // East Asian ideograph
    (0x284F39, ('\u{6CF8}', false)), // East Asian ideograph
    (0x276054, ('\u{98A0}', false)), // East Asian ideograph
    (0x6F5372, ('\u{C289}', false)), // Korean hangul
    (0x216055, ('\u{9858}', false)), // East Asian ideograph
    (0x6F4F2F, ('\u{B825}', false)), // Korean hangul
    (0x4B4866, ('\u{6E89}', false)), // East Asian ideograph
    (0x276056, ('\u{987E}', false)), // East Asian ideograph
    (0x69243A, ('\u{305A}', false)), // Hiragana letter ZU
    (0x276057, ('\u{98A4}', false)), // East Asian ideograph
    (0x276058, ('\u{663E}', false)), // East Asian ideograph
    (0x213861, ('\u{589C}', false)), // East Asian ideograph
    (0x4B614D, ('\u{9A13}', false)), // East Asian ideograph
    (0x216059, ('\u{9871}', false)), // East Asian ideograph
    (0x4C4333, ('\u{6AAA}', false)), // East Asian ideograph
    (0x27625E, ('\u{9762}', false)), // East Asian ideograph
    (0x27605A, ('\u{98A6}', false)), // East Asian ideograph
    (0x6F4F30, ('\u{B828}', false)), // Korean hangul
    (0x214221, ('\u{64CE}', false)), // East Asian ideograph
    (0x214222, ('\u{64D4}', false)), // East Asian ideograph
    (0x214223, ('\u{64D2}', false)), // East Asian ideograph
    (0x214224, ('\u{64BF}', false)), // East Asian ideograph
    (0x234225, ('\u{9201}', false)), // East Asian ideograph
    (0x214226, ('\u{64F0}', false)), // East Asian ideograph
    (0x214227, ('\u{64E6}', false)), // East Asian ideograph
    (0x214228, ('\u{64EC}', false)), // East Asian ideograph
    (0x214229, ('\u{64F1}', false)), // East Asian ideograph
    (0x21422A, ('\u{64F4}', false)), // East Asian ideograph
    (0x21422B, ('\u{64F2}', false)), // East Asian ideograph
    (0x21422C, ('\u{6506}', false)), // East Asian ideograph
    (0x21422D, ('\u{6500}', false)), // East Asian ideograph
    (0x27422E, ('\u{6270}', false)), // East Asian ideograph
    (0x21422F, ('\u{64FB}', false)), // East Asian ideograph
    (0x214230, ('\u{64FA}', false)), // East Asian ideograph
    (0x214231, ('\u{650F}', false)), // East Asian ideograph
    (0x214232, ('\u{6518}', false)), // East Asian ideograph
    (0x214233, ('\u{6514}', false)), // East Asian ideograph
    (0x214234, ('\u{6519}', false)), // East Asian ideograph
    (0x214235, ('\u{651D}', false)), // East Asian ideograph
    (0x214236, ('\u{651C}', false)), // East Asian ideograph
    (0x214237, ('\u{6523}', false)), // East Asian ideograph
    (0x214238, ('\u{6524}', false)), // East Asian ideograph
    (0x214239, ('\u{652B}', false)), // East Asian ideograph
    (0x21423A, ('\u{652A}', false)), // East Asian ideograph
    (0x21423B, ('\u{652C}', false)), // East Asian ideograph
    (0x21423C, ('\u{652F}', false)), // East Asian ideograph
    (0x21423D, ('\u{6536}', false)), // East Asian ideograph
    (0x21423E, ('\u{6539}', false)), // East Asian ideograph
    (0x21423F, ('\u{653B}', false)), // East Asian ideograph
    (0x214240, ('\u{653E}', false)), // East Asian ideograph
    (0x214241, ('\u{653F}', false)), // East Asian ideograph
    (0x214242, ('\u{6545}', false)), // East Asian ideograph
    (0x214243, ('\u{6548}', false)), // East Asian ideograph
    (0x214244, ('\u{654E}', false)), // East Asian ideograph
    (0x214245, ('\u{6556}', false)), // East Asian ideograph
    (0x214246, ('\u{6551}', false)), // East Asian ideograph
    (0x274247, ('\u{8D25}', false)), // East Asian ideograph
    (0x214248, ('\u{655D}', false)), // East Asian ideograph
    (0x214249, ('\u{6558}', false)), // East Asian ideograph
    (0x21424A, ('\u{654F}', false)), // East Asian ideograph
    (0x21424B, ('\u{6566}', false)), // East Asian ideograph
    (0x21424C, ('\u{6562}', false)), // East Asian ideograph
    (0x21424D, ('\u{6563}', false)), // East Asian ideograph
    (0x21424E, ('\u{655E}', false)), // East Asian ideograph
    (0x21424F, ('\u{5553}', false)), // East Asian ideograph
    (0x214250, ('\u{656C}', false)), // East Asian ideograph
    (0x214251, ('\u{6572}', false)), // East Asian ideograph
    (0x214252, ('\u{6575}', false)), // East Asian ideograph
    (0x214253, ('\u{6577}', false)), // East Asian ideograph
    (0x214254, ('\u{6578}', false)), // East Asian ideograph
    (0x214255, ('\u{6574}', false)), // East Asian ideograph
    (0x214256, ('\u{6582}', false)), // East Asian ideograph
    (0x214257, ('\u{6583}', false)), // East Asian ideograph
    (0x214258, ('\u{6587}', false)), // East Asian ideograph
    (0x214259, ('\u{6591}', false)), // East Asian ideograph
    (0x21425A, ('\u{6590}', false)), // East Asian ideograph
    (0x6F4F32, ('\u{B834}', false)), // Korean hangul
    (0x21425C, ('\u{6599}', false)), // East Asian ideograph
    (0x21425D, ('\u{659C}', false)), // East Asian ideograph
    (0x21425E, ('\u{659F}', false)), // East Asian ideograph
    (0x21425F, ('\u{65A1}', false)), // East Asian ideograph
    (0x214260, ('\u{65A4}', false)), // East Asian ideograph
    (0x214261, ('\u{65A5}', false)), // East Asian ideograph
    (0x214262, ('\u{65A7}', false)), // East Asian ideograph
    (0x274263, ('\u{65A9}', false)), // East Asian ideograph
    (0x214264, ('\u{65AF}', false)), // East Asian ideograph
    (0x214265, ('\u{65B0}', false)), // East Asian ideograph
    (0x274266, ('\u{65AD}', false)), // East Asian ideograph
    (0x214267, ('\u{65B9}', false)), // East Asian ideograph
    (0x224268, ('\u{6AB4}', false)), // East Asian ideograph
    (0x214269, ('\u{65BD}', false)), // East Asian ideograph
    (0x21426A, ('\u{65C1}', false)), // East Asian ideograph
    (0x21426B, ('\u{65C5}', false)), // East Asian ideograph
    (0x21426C, ('\u{65CE}', false)), // East Asian ideograph
    (0x21426D, ('\u{65CB}', false)), // East Asian ideograph
    (0x21426E, ('\u{65CC}', false)), // East Asian ideograph
    (0x21426F, ('\u{65CF}', false)), // East Asian ideograph
    (0x214270, ('\u{65D7}', false)), // East Asian ideograph
    (0x214271, ('\u{65D6}', false)), // East Asian ideograph
    (0x214272, ('\u{65E2}', false)), // East Asian ideograph
    (0x214273, ('\u{65E5}', false)), // East Asian ideograph
    (0x234274, ('\u{923F}', false)), // East Asian ideograph
    (0x214275, ('\u{65E9}', false)), // East Asian ideograph
    (0x214276, ('\u{65EC}', false)), // East Asian ideograph
    (0x214277, ('\u{65ED}', false)), // East Asian ideograph
    (0x214278, ('\u{65E8}', false)), // East Asian ideograph
    (0x214279, ('\u{65F1}', false)), // East Asian ideograph
    (0x21427A, ('\u{65FA}', false)), // East Asian ideograph
    (0x21427B, ('\u{6606}', false)), // East Asian ideograph
    (0x21427C, ('\u{6614}', false)), // East Asian ideograph
    (0x21427D, ('\u{660C}', false)), // East Asian ideograph
    (0x21427E, ('\u{6600}', false)), // East Asian ideograph
    (0x6F5779, ('\u{C954}', false)), // Korean hangul
    (0x21606B, ('\u{98EF}', false)), // East Asian ideograph
    (0x27606C, ('\u{9972}', false)), // East Asian ideograph
    (0x453F6D, ('\u{52E0}', false)), // East Asian ideograph
    (0x29373A, ('\u{8D46}', false)), // East Asian ideograph
    (0x22606D, ('\u{76AD}', false)), // East Asian ideograph
    (0x6F5377, ('\u{C2A4}', false)), // Korean hangul
    (0x27606E, ('\u{9971}', false)), // East Asian ideograph
    (0x6F4F34, ('\u{B837}', false)), // Korean hangul
    (0x21395C, ('\u{59B9}', false)), // East Asian ideograph
    (0x27606F, ('\u{9970}', false)), // East Asian ideograph
    (0x69243B, ('\u{305B}', false)), // Hiragana letter SE
    (0x276070, ('\u{997A}', false)), // East Asian ideograph
    (0x2D362A, ('\u{95A7}', false)), // East Asian ideograph
    (0x275156, ('\u{7F04}', false)), // East Asian ideograph
    (0x277272, ('\u{54D2}', false)), // East Asian ideograph
    (0x236071, ('\u{9FA5}', false)), // East Asian ideograph
    (0x213862, ('\u{58AE}', false)), // East Asian ideograph
    (0x216072, ('\u{990C}', false)), // East Asian ideograph
    (0x6F5378, ('\u{C2A5}', false)), // Korean hangul
    (0x276073, ('\u{9977}', false)), // East Asian ideograph
    (0x215B76, ('\u{8FF7}', false)), // East Asian ideograph
    (0x21395D, ('\u{59C6}', false)), // East Asian ideograph
    (0x216074, ('\u{9910}', false)), // East Asian ideograph
    (0x276075, ('\u{9981}', false)), // East Asian ideograph
    (0x6F5D71, ('\u{D761}', false)), // Korean hangul
    (0x276076, ('\u{4F59}', false)), // East Asian ideograph
    (0x295955, ('\u{9C8E}', false)), // East Asian ideograph
    (0x6F4B21, ('\u{AF9C}', false)), // Korean hangul
    (0x216077, ('\u{9913}', false)), // East Asian ideograph
    (0x214B22, ('\u{733E}', false)), // East Asian ideograph
    (0x2D4738, ('\u{6FFC}', false)), // East Asian ideograph
    (0x276078, ('\u{997C}', false)), // East Asian ideograph
    (0x214B23, ('\u{7345}', false)), // East Asian ideograph
    (0x276079, ('\u{9986}', false)), // East Asian ideograph
    (0x4B553F, ('\u{83BD}', false)), // East Asian ideograph (variant of 21553F which maps to 83BD)
    (0x224B24, ('\u{6E5C}', false)), // East Asian ideograph
    (0x27607A, ('\u{996F}', false)), // East Asian ideograph
    (0x27607B, ('\u{9984}', false)), // East Asian ideograph
    (0x224E6A, ('\u{700D}', false)), // East Asian ideograph
    (0x27607C, ('\u{9985}', false)), // East Asian ideograph
    (0x214B27, ('\u{7368}', false)), // East Asian ideograph
    (0x6F537A, ('\u{C2AC}', false)), // Korean hangul
    (0x217334, ('\u{5686}', false)), // East Asian ideograph
    (0x214B28, ('\u{7370}', false)), // East Asian ideograph
    (0x29484A, ('\u{956C}', false)), // East Asian ideograph
    (0x27607E, ('\u{998F}', false)), // East Asian ideograph
    (0x6F5D6D, ('\u{D757}', false)), // Korean hangul
    (0x214B29, ('\u{7372}', false)), // East Asian ideograph
    (0x4C5447, ('\u{71E0}', false)), // East Asian ideograph (variant of 225447 which maps to 71E0)
    (0x214B2A, ('\u{7377}', false)), // East Asian ideograph
    (0x2E7374, ('\u{7E89}', false)), // East Asian ideograph
    (0x214B2B, ('\u{7378}', false)), // East Asian ideograph
    (0x2E6060, ('\u{76A1}', false)), // East Asian ideograph
    (0x214B2C, ('\u{7375}', false)), // East Asian ideograph
    (0x6F537B, ('\u{C2AD}', false)), // Korean hangul
    (0x214B2D, ('\u{737A}', false)), // East Asian ideograph
    (0x213960, ('\u{59AF}', false)), // East Asian ideograph
    (0x286222, ('\u{7726}', false)), // East Asian ideograph
    (0x214B2E, ('\u{737B}', false)), // East Asian ideograph
    (0x335F34, ('\u{90C4}', false)), // East Asian ideograph
    (0x21393D, ('\u{5962}', false)), // East Asian ideograph
    (0x274B2F, ('\u{7321}', false)), // East Asian ideograph
    (0x295958, ('\u{9C9A}', false)), // East Asian ideograph
    (0x214321, ('\u{660E}', false)), // East Asian ideograph
    (0x214322, ('\u{6613}', false)), // East Asian ideograph
    (0x214323, ('\u{6602}', false)), // East Asian ideograph
    (0x214324, ('\u{660F}', false)), // East Asian ideograph
    (0x214325, ('\u{6625}', false)), // East Asian ideograph
    (0x214326, ('\u{6627}', false)), // East Asian ideograph
    (0x214327, ('\u{662F}', false)), // East Asian ideograph
    (0x214328, ('\u{662D}', false)), // East Asian ideograph
    (0x214329, ('\u{6620}', false)), // East Asian ideograph
    (0x21432A, ('\u{661F}', false)), // East Asian ideograph
    (0x21432B, ('\u{6628}', false)), // East Asian ideograph
    (0x21432C, ('\u{664F}', false)), // East Asian ideograph
    (0x21432D, ('\u{6642}', false)), // East Asian ideograph
    (0x21432E, ('\u{6652}', false)), // East Asian ideograph
    (0x21432F, ('\u{6649}', false)), // East Asian ideograph
    (0x214330, ('\u{6643}', false)), // East Asian ideograph
    (0x214331, ('\u{664C}', false)), // East Asian ideograph
    (0x214332, ('\u{665D}', false)), // East Asian ideograph
    (0x214333, ('\u{6664}', false)), // East Asian ideograph
    (0x214334, ('\u{6668}', false)), // East Asian ideograph
    (0x214335, ('\u{6666}', false)), // East Asian ideograph
    (0x214336, ('\u{665A}', false)), // East Asian ideograph
    (0x214337, ('\u{666F}', false)), // East Asian ideograph
    (0x214338, ('\u{666E}', false)), // East Asian ideograph
    (0x214339, ('\u{FA12}', false)), // East Asian ideograph
    (0x21433A, ('\u{6691}', false)), // East Asian ideograph
    (0x21433B, ('\u{6670}', false)), // East Asian ideograph
    (0x21433C, ('\u{6676}', false)), // East Asian ideograph
    (0x21433D, ('\u{667A}', false)), // East Asian ideograph
    (0x21433E, ('\u{6697}', false)), // East Asian ideograph
    (0x21433F, ('\u{6687}', false)), // East Asian ideograph
    (0x214340, ('\u{6689}', false)), // East Asian ideograph
    (0x214341, ('\u{6688}', false)), // East Asian ideograph
    (0x214342, ('\u{6696}', false)), // East Asian ideograph
    (0x214343, ('\u{66A2}', false)), // East Asian ideograph
    (0x214344, ('\u{66AB}', false)), // East Asian ideograph
    (0x214345, ('\u{66B4}', false)), // East Asian ideograph
    (0x214346, ('\u{66AE}', false)), // East Asian ideograph
    (0x214347, ('\u{66C1}', false)), // East Asian ideograph
    (0x214348, ('\u{66C9}', false)), // East Asian ideograph
    (0x214349, ('\u{66C6}', false)), // East Asian ideograph
    (0x21434A, ('\u{66B9}', false)), // East Asian ideograph
    (0x21434B, ('\u{66D6}', false)), // East Asian ideograph
    (0x21434C, ('\u{66D9}', false)), // East Asian ideograph
    (0x21434D, ('\u{66E0}', false)), // East Asian ideograph
    (0x21434E, ('\u{66DD}', false)), // East Asian ideograph
    (0x21434F, ('\u{66E6}', false)), // East Asian ideograph
    (0x214350, ('\u{66F0}', false)), // East Asian ideograph
    (0x214351, ('\u{66F2}', false)), // East Asian ideograph
    (0x214352, ('\u{66F3}', false)), // East Asian ideograph
    (0x214353, ('\u{66F4}', false)), // East Asian ideograph
    (0x214354, ('\u{66F7}', false)), // East Asian ideograph
    (0x214355, ('\u{66F8}', false)), // East Asian ideograph
    (0x214356, ('\u{66F9}', false)), // East Asian ideograph
    (0x214357, ('\u{52D7}', false)), // East Asian ideograph
    (0x214358, ('\u{66FE}', false)), // East Asian ideograph
    (0x214359, ('\u{66FF}', false)), // East Asian ideograph
    (0x21435A, ('\u{6703}', false)), // East Asian ideograph
    (0x21435B, ('\u{6708}', false)), // East Asian ideograph
    (0x21435C, ('\u{6709}', false)), // East Asian ideograph
    (0x21435D, ('\u{670D}', false)), // East Asian ideograph
    (0x21435E, ('\u{670B}', false)), // East Asian ideograph
    (0x21435F, ('\u{6717}', false)), // East Asian ideograph
    (0x214360, ('\u{6715}', false)), // East Asian ideograph
    (0x214361, ('\u{6714}', false)), // East Asian ideograph
    (0x214362, ('\u{671B}', false)), // East Asian ideograph
    (0x214363, ('\u{671D}', false)), // East Asian ideograph
    (0x214364, ('\u{671F}', false)), // East Asian ideograph
    (0x6F537E, ('\u{C2B5}', false)), // Korean hangul
    (0x234366, ('\u{92C8}', false)), // East Asian ideograph
    (0x214367, ('\u{6728}', false)), // East Asian ideograph
    (0x214369, ('\u{672C}', false)), // East Asian ideograph
    (0x23436A, ('\u{92C3}', false)), // East Asian ideograph
    (0x21436B, ('\u{672A}', false)), // East Asian ideograph
    (0x29436C, ('\u{950D}', false)), // East Asian ideograph
    (0x21436D, ('\u{673D}', false)), // East Asian ideograph
    (0x22436E, ('\u{6B17}', false)), // East Asian ideograph
    (0x21436F, ('\u{6731}', false)), // East Asian ideograph
    (0x214370, ('\u{6735}', false)), // East Asian ideograph
    (0x214371, ('\u{675E}', false)), // East Asian ideograph
    (0x214372, ('\u{6751}', false)), // East Asian ideograph
    (0x214373, ('\u{674E}', false)), // East Asian ideograph
    (0x214374, ('\u{675C}', false)), // East Asian ideograph
    (0x234375, ('\u{92E6}', false)), // East Asian ideograph
    (0x214376, ('\u{6756}', false)), // East Asian ideograph
    (0x214377, ('\u{675F}', false)), // East Asian ideograph
    (0x214378, ('\u{674F}', false)), // East Asian ideograph
    (0x214379, ('\u{6749}', false)), // East Asian ideograph
    (0x23437A, ('\u{92D9}', false)), // East Asian ideograph
    (0x21437B, ('\u{676D}', false)), // East Asian ideograph
    (0x21437C, ('\u{678B}', false)), // East Asian ideograph
    (0x21437D, ('\u{6795}', false)), // East Asian ideograph
    (0x21437E, ('\u{6789}', false)), // East Asian ideograph
    (0x21777B, ('\u{58A9}', false)), // East Asian ideograph
    (0x4B3F40, ('\u{618E}', false)), // East Asian ideograph (variant of 213F40)
    (0x214B40, ('\u{73ED}', false)), // East Asian ideograph
    (0x27626A, ('\u{70B9}', false)), // East Asian ideograph
    (0x214B41, ('\u{73EE}', false)), // East Asian ideograph
    (0x214B42, ('\u{73E0}', false)), // East Asian ideograph
    (0x214B43, ('\u{7405}', false)), // East Asian ideograph
    (0x6F4B44, ('\u{B07C}', false)), // Korean hangul
    (0x23457E, ('\u{938B}', false)), // East Asian ideograph
    (0x214B45, ('\u{7403}', false)), // East Asian ideograph
    (0x336062, ('\u{98C3}', false)), // East Asian ideograph
    (0x214B46, ('\u{740A}', false)), // East Asian ideograph
    (0x517954, ('\u{734E}', false)), // East Asian ideograph
    (0x274B47, ('\u{73B0}', false)), // East Asian ideograph
    (0x6F577B, ('\u{C960}', false)), // Korean hangul
    (0x214B48, ('\u{7406}', false)), // East Asian ideograph
    (0x214B49, ('\u{740D}', false)), // East Asian ideograph
    (0x282577, ('\u{5CE4}', false)), // East Asian ideograph
    (0x214B4A, ('\u{743A}', false)), // East Asian ideograph
    (0x6F5338, ('\u{C14D}', false)), // Korean hangul
    (0x6F4B4B, ('\u{B090}', false)), // Korean hangul
    (0x213966, ('\u{59D4}', false)), // East Asian ideograph
    (0x6F4B4C, ('\u{B091}', false)), // Korean hangul
    (0x2D584D, ('\u{548F}', false)), // East Asian ideograph
    (0x214B4D, ('\u{7434}', false)), // East Asian ideograph
    (0x2E3A33, ('\u{80AD}', false)), // East Asian ideograph
    (0x213864, ('\u{58C7}', false)), // East Asian ideograph (variant of 4B3864 which maps to 58C7)
    (0x214B4F, ('\u{7433}', false)), // East Asian ideograph
    (0x6F4B50, ('\u{B099}', false)), // Korean hangul
    (0x6F5021, ('\u{BA38}', false)), // Korean hangul
    (0x214B51, ('\u{7425}', false)), // East Asian ideograph
    (0x6F4B52, ('\u{B09C}', false)), // Korean hangul
    (0x213F36, ('\u{616E}', false)), // East Asian ideograph
    (0x234B53, ('\u{96CA}', false)), // East Asian ideograph
    (0x6F4B54, ('\u{B0A0}', false)), // Korean hangul
    (0x6F4B55, ('\u{B0A1}', false)), // Korean hangul
    (0x6F4F40, ('\u{B8B0}', false)), // Korean hangul
    (0x6F5930, ('\u{CC4C}', false)), // Korean hangul
    (0x6F4B56, ('\u{B0A8}', false)), // Korean hangul
    (0x274B57, ('\u{73F2}', false)), // East Asian ideograph
    (0x6F4B58, ('\u{B0AB}', false)), // Korean hangul
    (0x214B59, ('\u{745E}', false)), // East Asian ideograph
    (0x214B5A, ('\u{745C}', false)), // East Asian ideograph
    (0x6F4F41, ('\u{B8CC}', false)), // Korean hangul
    (0x214421, ('\u{6787}', false)), // East Asian ideograph
    (0x214422, ('\u{6777}', false)), // East Asian ideograph
    (0x214423, ('\u{679D}', false)), // East Asian ideograph
    (0x214424, ('\u{6797}', false)), // East Asian ideograph
    (0x214425, ('\u{676F}', false)), // East Asian ideograph
    (0x214426, ('\u{6771}', false)), // East Asian ideograph
    (0x214427, ('\u{6773}', false)), // East Asian ideograph
    (0x214428, ('\u{679C}', false)), // East Asian ideograph
    (0x214429, ('\u{6775}', false)), // East Asian ideograph
    (0x21442A, ('\u{679A}', false)), // East Asian ideograph
    (0x21442B, ('\u{6790}', false)), // East Asian ideograph
    (0x22442C, ('\u{6B37}', false)), // East Asian ideograph
    (0x21442D, ('\u{677E}', false)), // East Asian ideograph
    (0x21442E, ('\u{67D3}', false)), // East Asian ideograph
    (0x21442F, ('\u{67F1}', false)), // East Asian ideograph
    (0x214430, ('\u{67FF}', false)), // East Asian ideograph
    (0x214431, ('\u{67D4}', false)), // East Asian ideograph
    (0x214432, ('\u{67C4}', false)), // East Asian ideograph
    (0x214433, ('\u{67AF}', false)), // East Asian ideograph
    (0x214434, ('\u{67D0}', false)), // East Asian ideograph
    (0x214435, ('\u{67D1}', false)), // East Asian ideograph
    (0x214436, ('\u{67EF}', false)), // East Asian ideograph
    (0x214437, ('\u{67E9}', false)), // East Asian ideograph
    (0x214438, ('\u{67B6}', false)), // East Asian ideograph
    (0x214439, ('\u{67EC}', false)), // East Asian ideograph
    (0x21443A, ('\u{67E5}', false)), // East Asian ideograph
    (0x21443B, ('\u{67FA}', false)), // East Asian ideograph
    (0x21443C, ('\u{67DA}', false)), // East Asian ideograph
    (0x21443D, ('\u{6805}', false)), // East Asian ideograph
    (0x21443E, ('\u{67DE}', false)), // East Asian ideograph
    (0x21443F, ('\u{67B8}', false)), // East Asian ideograph
    (0x214440, ('\u{67CF}', false)), // East Asian ideograph
    (0x214441, ('\u{67F3}', false)), // East Asian ideograph
    (0x214442, ('\u{6848}', false)), // East Asian ideograph
    (0x214443, ('\u{6821}', false)), // East Asian ideograph
    (0x214444, ('\u{6838}', false)), // East Asian ideograph
    (0x214445, ('\u{6853}', false)), // East Asian ideograph
    (0x214446, ('\u{6846}', false)), // East Asian ideograph
    (0x214447, ('\u{6842}', false)), // East Asian ideograph
    (0x214448, ('\u{6854}', false)), // East Asian ideograph
    (0x214449, ('\u{6817}', false)), // East Asian ideograph
    (0x21444A, ('\u{683D}', false)), // East Asian ideograph
    (0x21444B, ('\u{6851}', false)), // East Asian ideograph
    (0x21444C, ('\u{6829}', false)), // East Asian ideograph
    (0x21444D, ('\u{6850}', false)), // East Asian ideograph
    (0x21444E, ('\u{6839}', false)), // East Asian ideograph
    (0x23444F, ('\u{9344}', false)), // East Asian ideograph
    (0x214450, ('\u{67F4}', false)), // East Asian ideograph
    (0x214451, ('\u{6843}', false)), // East Asian ideograph
    (0x214452, ('\u{6840}', false)), // East Asian ideograph
    (0x214453, ('\u{682A}', false)), // East Asian ideograph
    (0x214454, ('\u{6845}', false)), // East Asian ideograph
    (0x214455, ('\u{683C}', false)), // East Asian ideograph
    (0x214456, ('\u{6813}', false)), // East Asian ideograph (variant of 4B4456 which maps to 6813)
    (0x214457, ('\u{6881}', false)), // East Asian ideograph
    (0x214458, ('\u{6893}', false)), // East Asian ideograph
    (0x214459, ('\u{68AF}', false)), // East Asian ideograph
    (0x21445A, ('\u{6876}', false)), // East Asian ideograph
    (0x21445B, ('\u{68B0}', false)), // East Asian ideograph
    (0x21445C, ('\u{68A7}', false)), // East Asian ideograph
    (0x21445D, ('\u{6897}', false)), // East Asian ideograph
    (0x21445E, ('\u{68B5}', false)), // East Asian ideograph
    (0x21445F, ('\u{68B3}', false)), // East Asian ideograph
    (0x214460, ('\u{68A2}', false)), // East Asian ideograph
    (0x214461, ('\u{687F}', false)), // East Asian ideograph
    (0x214462, ('\u{68B1}', false)), // East Asian ideograph
    (0x214463, ('\u{689D}', false)), // East Asian ideograph
    (0x214464, ('\u{68AD}', false)), // East Asian ideograph
    (0x214465, ('\u{6886}', false)), // East Asian ideograph
    (0x234466, ('\u{9312}', false)), // East Asian ideograph
    (0x214467, ('\u{68A8}', false)), // East Asian ideograph
    (0x214468, ('\u{689F}', false)), // East Asian ideograph
    (0x214469, ('\u{6894}', false)), // East Asian ideograph
    (0x21446A, ('\u{6883}', false)), // East Asian ideograph
    (0x21446B, ('\u{68D5}', false)), // East Asian ideograph
    (0x21446C, ('\u{68FA}', false)), // East Asian ideograph
    (0x21446D, ('\u{68C4}', false)), // East Asian ideograph
    (0x21446E, ('\u{68F2}', false)), // East Asian ideograph
    (0x21446F, ('\u{68D2}', false)), // East Asian ideograph
    (0x214470, ('\u{68E3}', false)), // East Asian ideograph
    (0x214471, ('\u{68DF}', false)), // East Asian ideograph
    (0x214472, ('\u{68CB}', false)), // East Asian ideograph
    (0x214473, ('\u{68EE}', false)), // East Asian ideograph
    (0x214474, ('\u{690D}', false)), // East Asian ideograph
    (0x214475, ('\u{6905}', false)), // East Asian ideograph
    (0x214476, ('\u{68E7}', false)), // East Asian ideograph
    (0x214477, ('\u{68E0}', false)), // East Asian ideograph
    (0x214478, ('\u{68F5}', false)), // East Asian ideograph
    (0x214479, ('\u{68CD}', false)), // East Asian ideograph
    (0x21447A, ('\u{68D7}', false)), // East Asian ideograph
    (0x21447B, ('\u{68D8}', false)), // East Asian ideograph
    (0x27447C, ('\u{832D}', false)), // East Asian ideograph
    (0x21447D, ('\u{68F9}', false)), // East Asian ideograph
    (0x21447E, ('\u{68DA}', false)), // East Asian ideograph
    (0x214B6B, ('\u{74CF}', false)), // East Asian ideograph
    (0x275166, ('\u{7EE9}', false)), // East Asian ideograph
    (0x214B6C, ('\u{74DC}', false)), // East Asian ideograph
    (0x6F2457, ('\u{3131}', false)), // Korean hangul
    (0x6F576C, ('\u{C8FC}', false)), // Korean hangul
    (0x214B6D, ('\u{74E0}', false)), // East Asian ideograph
    (0x6F4B6E, ('\u{B108}', false)), // Korean hangul
    (0x70755D, ('\u{8E3A}', false)), // East Asian ideograph
    (0x6F4B6F, ('\u{B109}', false)), // Korean hangul
    (0x39525B, ('\u{66DC}', false)), // East Asian ideograph
    (0x214B71, ('\u{74F6}', false)), // East Asian ideograph
    (0x4B3F4A, ('\u{5FDC}', false)), // East Asian ideograph
    (0x234E35, ('\u{97BE}', false)), // East Asian ideograph
    (0x29474D, ('\u{956B}', false)), // East Asian ideograph
    (0x6F4B73, ('\u{B10F}', false)), // Korean hangul
    (0x6F4F46, ('\u{B8F0}', false)), // Korean hangul
    (0x214B74, ('\u{750C}', false)), // East Asian ideograph
    (0x224B75, ('\u{6EA4}', false)), // East Asian ideograph
    (0x214B76, ('\u{7518}', false)), // East Asian ideograph
    (0x4B3F4B, ('\u{601C}', false)), // East Asian ideograph (variant of 273F4B)
    (0x234B77, ('\u{96F4}', false)), // East Asian ideograph
    (0x2D3622, ('\u{8AEE}', false)), // East Asian ideograph
    (0x6F4D67, ('\u{B4EC}', false)), // Korean hangul
    (0x214B78, ('\u{751C}', false)), // East Asian ideograph
    (0x275E32, ('\u{9556}', false)), // East Asian ideograph
    (0x214B79, ('\u{751F}', false)), // East Asian ideograph
    (0x6F577D, ('\u{C96C}', false)), // Korean hangul
    (0x335F43, ('\u{9D08}', false)), // East Asian ideograph
    (0x333D2A, ('\u{5E83}', false)), // East Asian ideograph
    (0x2D5856, ('\u{612C}', false)), // East Asian ideograph
    (0x274B7A, ('\u{4EA7}', false)), // East Asian ideograph
    (0x4B5437, ('\u{820E}', false)), // East Asian ideograph
    (0x694B7B, ('\u{9EBF}', false)), // East Asian ideograph
    (0x213032, ('\u{4E26}', false)), // East Asian ideograph
    (0x214B7C, ('\u{7525}', false)), // East Asian ideograph
    (0x6F533A, ('\u{C154}', false)), // Korean hangul
    (0x276276, ('\u{51AC}', false)), // East Asian ideograph
    (0x214B7D, ('\u{7528}', false)), // East Asian ideograph
    (0x6F4F48, ('\u{B8F9}', false)), // Korean hangul
    (0x275E33, ('\u{9557}', false)), // East Asian ideograph
    (0x21352D, ('\u{53F8}', false)), // East Asian ideograph
    (0x217629, ('\u{57E3}', false)), // East Asian ideograph
    (0x23362A, ('\u{8C86}', false)), // East Asian ideograph
    (0x213866, ('\u{58C1}', false)), // East Asian ideograph
    (0x23527B, ('\u{99E3}', false)), // East Asian ideograph
    (0x21762C, ('\u{57F6}', false)), // East Asian ideograph
    (0x6F4F49, ('\u{B8FB}', false)), // Korean hangul
    (0x23362D, ('\u{8C85}', false)), // East Asian ideograph
    (0x29485C, ('\u{9565}', false)), // East Asian ideograph
    (0x21352E, ('\u{53E4}', false)), // East Asian ideograph
    (0x4D5858, ('\u{9BE3}', false)), // East Asian ideograph
    (0x6F5D75, ('\u{D76C}', false)), // Korean hangul
    (0x214521, ('\u{690E}', false)), // East Asian ideograph
    (0x214522, ('\u{68C9}', false)), // East Asian ideograph
    (0x214523, ('\u{6954}', false)), // East Asian ideograph
    (0x214524, ('\u{6930}', false)), // East Asian ideograph
    (0x214525, ('\u{6977}', false)), // East Asian ideograph
    (0x214526, ('\u{6975}', false)), // East Asian ideograph
    (0x214527, ('\u{695A}', false)), // East Asian ideograph
    (0x214528, ('\u{6960}', false)), // East Asian ideograph
    (0x214529, ('\u{696B}', false)), // East Asian ideograph
    (0x21452A, ('\u{694A}', false)), // East Asian ideograph
    (0x21452B, ('\u{6968}', false)), // East Asian ideograph
    (0x21452C, ('\u{695E}', false)), // East Asian ideograph
    (0x21452D, ('\u{696D}', false)), // East Asian ideograph
    (0x21452E, ('\u{6979}', false)), // East Asian ideograph
    (0x21452F, ('\u{6953}', false)), // East Asian ideograph
    (0x214530, ('\u{6986}', false)), // East Asian ideograph
    (0x214531, ('\u{69A8}', false)), // East Asian ideograph
    (0x214532, ('\u{6995}', false)), // East Asian ideograph
    (0x214533, ('\u{699C}', false)), // East Asian ideograph
    (0x214534, ('\u{6994}', false)), // East Asian ideograph
    (0x214535, ('\u{69C1}', false)), // East Asian ideograph
    (0x214536, ('\u{69B7}', false)), // East Asian ideograph
    (0x214537, ('\u{69AE}', false)), // East Asian ideograph
    (0x214538, ('\u{699B}', false)), // East Asian ideograph
    (0x214539, ('\u{69CB}', false)), // East Asian ideograph
    (0x21453A, ('\u{69D3}', false)), // East Asian ideograph
    (0x21453B, ('\u{69BB}', false)), // East Asian ideograph
    (0x21453C, ('\u{69AB}', false)), // East Asian ideograph
    (0x21453D, ('\u{69CC}', false)), // East Asian ideograph
    (0x21453E, ('\u{69AD}', false)), // East Asian ideograph
    (0x21453F, ('\u{69D0}', false)), // East Asian ideograph
    (0x214540, ('\u{69CD}', false)), // East Asian ideograph
    (0x214541, ('\u{69B4}', false)), // East Asian ideograph
    (0x214542, ('\u{6A1F}', false)), // East Asian ideograph
    (0x214543, ('\u{69E8}', false)), // East Asian ideograph
    (0x274544, ('\u{6837}', false)), // East Asian ideograph
    (0x214545, ('\u{69EA}', false)), // East Asian ideograph
    (0x274546, ('\u{6869}', false)), // East Asian ideograph
    (0x214547, ('\u{6A19}', false)), // East Asian ideograph
    (0x214548, ('\u{69FD}', false)), // East Asian ideograph
    (0x214549, ('\u{6A1E}', false)), // East Asian ideograph
    (0x21454A, ('\u{6A13}', false)), // East Asian ideograph
    (0x21454B, ('\u{6A21}', false)), // East Asian ideograph
    (0x21454C, ('\u{69F3}', false)), // East Asian ideograph
    (0x21454D, ('\u{6A0A}', false)), // East Asian ideograph
    (0x21454E, ('\u{6A02}', false)), // East Asian ideograph
    (0x21454F, ('\u{6A05}', false)), // East Asian ideograph
    (0x214550, ('\u{6A3D}', false)), // East Asian ideograph
    (0x214551, ('\u{6A58}', false)), // East Asian ideograph
    (0x214552, ('\u{6A59}', false)), // East Asian ideograph
    (0x214553, ('\u{6A62}', false)), // East Asian ideograph
    (0x214554, ('\u{6A44}', false)), // East Asian ideograph
    (0x214555, ('\u{6A39}', false)), // East Asian ideograph
    (0x214556, ('\u{6A6B}', false)), // East Asian ideograph
    (0x214557, ('\u{6A3A}', false)), // East Asian ideograph
    (0x214558, ('\u{6A38}', false)), // East Asian ideograph
    (0x214559, ('\u{6A47}', false)), // East Asian ideograph
    (0x21455A, ('\u{6A61}', false)), // East Asian ideograph
    (0x21455B, ('\u{6A4B}', false)), // East Asian ideograph
    (0x21455C, ('\u{6A35}', false)), // East Asian ideograph
    (0x21455D, ('\u{6A5F}', false)), // East Asian ideograph
    (0x21455E, ('\u{6A80}', false)), // East Asian ideograph
    (0x21455F, ('\u{6A94}', false)), // East Asian ideograph
    (0x214560, ('\u{6A84}', false)), // East Asian ideograph
    (0x214561, ('\u{6AA2}', false)), // East Asian ideograph
    (0x214562, ('\u{6A9C}', false)), // East Asian ideograph
    (0x214563, ('\u{6AB8}', false)), // East Asian ideograph
    (0x214564, ('\u{6AB3}', false)), // East Asian ideograph
    (0x214565, ('\u{6AC3}', false)), // East Asian ideograph
    (0x214566, ('\u{6ABB}', false)), // East Asian ideograph
    (0x234567, ('\u{9354}', false)), // East Asian ideograph
    (0x214568, ('\u{6AAC}', false)), // East Asian ideograph
    (0x214569, ('\u{6AE5}', false)), // East Asian ideograph
    (0x21456A, ('\u{6ADA}', false)), // East Asian ideograph
    (0x21456B, ('\u{6ADD}', false)), // East Asian ideograph
    (0x21456C, ('\u{6ADB}', false)), // East Asian ideograph
    (0x21456D, ('\u{6AD3}', false)), // East Asian ideograph
    (0x21456E, ('\u{6B04}', false)), // East Asian ideograph
    (0x21456F, ('\u{6AFB}', false)), // East Asian ideograph
    (0x214570, ('\u{6B0A}', false)), // East Asian ideograph
    (0x214571, ('\u{6B16}', false)), // East Asian ideograph
    (0x234572, ('\u{936D}', false)), // East Asian ideograph
    (0x214573, ('\u{6B21}', false)), // East Asian ideograph
    (0x214574, ('\u{6B23}', false)), // East Asian ideograph
    (0x27363E, ('\u{5458}', false)), // East Asian ideograph
    (0x214576, ('\u{6B3E}', false)), // East Asian ideograph
    (0x214577, ('\u{6B3A}', false)), // East Asian ideograph
    (0x214578, ('\u{6B3D}', false)), // East Asian ideograph
    (0x214579, ('\u{6B47}', false)), // East Asian ideograph
    (0x21457A, ('\u{6B49}', false)), // East Asian ideograph
    (0x21457B, ('\u{6B4C}', false)), // East Asian ideograph
    (0x21457C, ('\u{6B50}', false)), // East Asian ideograph
    (0x21457D, ('\u{6B59}', false)), // East Asian ideograph
    (0x21457E, ('\u{6B5F}', false)), // East Asian ideograph
    (0x6F7640, ('\u{E8B2}', false)), // Korean hangul
    (0x2D3B27, ('\u{51A8}', false)), // East Asian ideograph
    (0x453421, ('\u{5271}', false)), // East Asian ideograph
    (0x213641, ('\u{5506}', false)), // East Asian ideograph
    (0x4C4D3D, ('\u{6F62}', false)), // East Asian ideograph
    (0x2D3642, ('\u{6B38}', false)), // East Asian ideograph
    (0x335F49, ('\u{9D70}', false)), // East Asian ideograph
    (0x4D5B7E, ('\u{9DC6}', false)), // East Asian ideograph
    (0x27516F, ('\u{7F2B}', false)), // East Asian ideograph
    (0x213867, ('\u{58BE}', false)), // East Asian ideograph
    (0x213644, ('\u{5556}', false)), // East Asian ideograph
    (0x213645, ('\u{5533}', false)), // East Asian ideograph
    (0x6F4F4E, ('\u{B959}', false)), // Korean hangul
    (0x275E39, ('\u{94D9}', false)), // East Asian ideograph
    (0x6F5449, ('\u{C372}', false)), // Korean hangul
    (0x234174, ('\u{91F4}', false)), // East Asian ideograph
    (0x213647, ('\u{5537}', false)), // East Asian ideograph (Version J extension)
    (0x2D3644, ('\u{5557}', false)), // East Asian ideograph
    (0x275170, ('\u{7F2E}', false)), // East Asian ideograph
    (0x6F553F, ('\u{C591}', false)), // Korean hangul
    (0x213649, ('\u{555E}', false)), // East Asian ideograph
    (0x276245, ('\u{9E4F}', false)), // East Asian ideograph
    (0x275E3A, ('\u{9570}', false)), // East Asian ideograph
    (0x6F764C, ('\u{E8BE}', false)), // Korean hangul
    (0x21764D, ('\u{57FD}', false)), // East Asian ideograph
    (0x21764E, ('\u{57F8}', false)), // East Asian ideograph
    (0x21364F, ('\u{5531}', false)), // East Asian ideograph
    (0x2D5749, ('\u{885E}', false)), // East Asian ideograph
    (0x275E3B, ('\u{9508}', false)), // East Asian ideograph
    (0x21574E, ('\u{521D}', false)), // East Asian ideograph
    (0x6F5859, ('\u{CAC0}', false)), // Korean hangul
    (0x233651, ('\u{8CBA}', false)), // East Asian ideograph
    (0x233652, ('\u{8CB5}', false)), // East Asian ideograph
    (0x213653, ('\u{553E}', false)), // East Asian ideograph
    (0x213654, ('\u{5563}', false)), // East Asian ideograph
    (0x6F4F51, ('\u{B968}', false)), // Korean hangul
    (0x275E3C, ('\u{956D}', false)), // East Asian ideograph
    (0x234177, ('\u{91F1}', false)), // East Asian ideograph
    (0x6F7656, ('\u{E8C8}', false)), // Korean hangul
    (0x213657, ('\u{552E}', false)), // East Asian ideograph
    (0x6F4A3A, ('\u{AE38}', false)), // Korean hangul
    (0x34682A, ('\u{7C7C}', false)), // East Asian ideograph
    (0x275E3D, ('\u{94C1}', false)), // East Asian ideograph
    (0x214621, ('\u{6B61}', false)), // East Asian ideograph
    (0x234622, ('\u{938C}', false)), // East Asian ideograph
    (0x214623, ('\u{6B63}', false)), // East Asian ideograph
    (0x214624, ('\u{6B64}', false)), // East Asian ideograph
    (0x214625, ('\u{6B65}', false)), // East Asian ideograph
    (0x214627, ('\u{6B66}', false)), // East Asian ideograph
    (0x214628, ('\u{6B6A}', false)), // East Asian ideograph
    (0x214629, ('\u{6B72}', false)), // East Asian ideograph
    (0x22462A, ('\u{6BF6}', false)), // East Asian ideograph
    (0x21462B, ('\u{6B78}', false)), // East Asian ideograph
    (0x21462C, ('\u{6B79}', false)), // East Asian ideograph
    (0x21462D, ('\u{6B7B}', false)), // East Asian ideograph
    (0x21462E, ('\u{6B7F}', false)), // East Asian ideograph
    (0x21462F, ('\u{6B83}', false)), // East Asian ideograph
    (0x214630, ('\u{6B86}', false)), // East Asian ideograph
    (0x214631, ('\u{6B8A}', false)), // East Asian ideograph
    (0x214632, ('\u{6B89}', false)), // East Asian ideograph
    (0x214633, ('\u{6B98}', false)), // East Asian ideograph
    (0x214634, ('\u{6B96}', false)), // East Asian ideograph
    (0x214635, ('\u{6BA4}', false)), // East Asian ideograph
    (0x214636, ('\u{6BAE}', false)), // East Asian ideograph
    (0x214637, ('\u{6BAF}', false)), // East Asian ideograph
    (0x214638, ('\u{6BB2}', false)), // East Asian ideograph
    (0x214639, ('\u{6BB5}', false)), // East Asian ideograph
    (0x21463A, ('\u{6BB7}', false)), // East Asian ideograph
    (0x21463B, ('\u{6BBA}', false)), // East Asian ideograph
    (0x21463C, ('\u{6BBC}', false)), // East Asian ideograph
    (0x21463D, ('\u{6BC0}', false)), // East Asian ideograph
    (0x21463E, ('\u{6BBF}', false)), // East Asian ideograph
    (0x21463F, ('\u{6BC5}', false)), // East Asian ideograph
    (0x214640, ('\u{6BC6}', false)), // East Asian ideograph
    (0x214641, ('\u{6BCB}', false)), // East Asian ideograph
    (0x214642, ('\u{6BCD}', false)), // East Asian ideograph
    (0x214643, ('\u{6BCF}', false)), // East Asian ideograph
    (0x214644, ('\u{6BD2}', false)), // East Asian ideograph
    (0x214646, ('\u{6BD4}', false)), // East Asian ideograph
    (0x214647, ('\u{6BD7}', false)), // East Asian ideograph
    (0x214648, ('\u{6BDB}', false)), // East Asian ideograph
    (0x214649, ('\u{6BEB}', false)), // East Asian ideograph
    (0x21464A, ('\u{6BEF}', false)), // East Asian ideograph
    (0x21464B, ('\u{6BFD}', false)), // East Asian ideograph
    (0x21464C, ('\u{6C0F}', false)), // East Asian ideograph
    (0x21464D, ('\u{6C11}', false)), // East Asian ideograph
    (0x21464E, ('\u{6C10}', false)), // East Asian ideograph
    (0x21464F, ('\u{6C13}', false)), // East Asian ideograph
    (0x214650, ('\u{6C16}', false)), // East Asian ideograph
    (0x214651, ('\u{6C1B}', false)), // East Asian ideograph
    (0x214652, ('\u{6C1F}', false)), // East Asian ideograph
    (0x214653, ('\u{6C27}', false)), // East Asian ideograph
    (0x214654, ('\u{6C26}', false)), // East Asian ideograph
    (0x214655, ('\u{6C23}', false)), // East Asian ideograph
    (0x214656, ('\u{6C28}', false)), // East Asian ideograph
    (0x214657, ('\u{6C24}', false)), // East Asian ideograph
    (0x214658, ('\u{6C2B}', false)), // East Asian ideograph
    (0x214659, ('\u{6C2E}', false)), // East Asian ideograph
    (0x21465A, ('\u{6C33}', false)), // East Asian ideograph
    (0x21465B, ('\u{6C2F}', false)), // East Asian ideograph (variant of 45465B which maps to 6C2F)
    (0x21465C, ('\u{6C34}', false)), // East Asian ideograph
    (0x21465D, ('\u{6C38}', false)), // East Asian ideograph
    (0x21465E, ('\u{6C41}', false)), // East Asian ideograph
    (0x23465F, ('\u{93E5}', false)), // East Asian ideograph
    (0x214660, ('\u{6C40}', false)), // East Asian ideograph
    (0x214661, ('\u{6C42}', false)), // East Asian ideograph
    (0x214662, ('\u{6C5E}', false)), // East Asian ideograph
    (0x214663, ('\u{6C57}', false)), // East Asian ideograph
    (0x214664, ('\u{6C5F}', false)), // East Asian ideograph
    (0x214665, ('\u{6C59}', false)), // East Asian ideograph
    (0x214666, ('\u{6C60}', false)), // East Asian ideograph
    (0x214667, ('\u{6C55}', false)), // East Asian ideograph
    (0x214668, ('\u{6C50}', false)), // East Asian ideograph
    (0x214669, ('\u{6C5D}', false)), // East Asian ideograph
    (0x21466A, ('\u{6C9B}', false)), // East Asian ideograph
    (0x21466B, ('\u{6C81}', false)), // East Asian ideograph
    (0x21466D, ('\u{6C7A}', false)), // East Asian ideograph
    (0x21466E, ('\u{6C6A}', false)), // East Asian ideograph
    (0x21466F, ('\u{6C8C}', false)), // East Asian ideograph
    (0x214670, ('\u{6C90}', false)), // East Asian ideograph
    (0x214671, ('\u{6C72}', false)), // East Asian ideograph
    (0x214672, ('\u{6C70}', false)), // East Asian ideograph
    (0x214673, ('\u{6C68}', false)), // East Asian ideograph
    (0x214674, ('\u{6C96}', false)), // East Asian ideograph
    (0x234675, ('\u{93DB}', false)), // East Asian ideograph
    (0x214676, ('\u{6C89}', false)), // East Asian ideograph (variant of 4B4676 which maps to 6C89)
    (0x214677, ('\u{6C99}', false)), // East Asian ideograph
    (0x214678, ('\u{6C7E}', false)), // East Asian ideograph
    (0x214679, ('\u{6C7D}', false)), // East Asian ideograph
    (0x21467A, ('\u{6C92}', false)), // East Asian ideograph
    (0x21467B, ('\u{6C83}', false)), // East Asian ideograph
    (0x21467C, ('\u{6CB1}', false)), // East Asian ideograph
    (0x23366A, ('\u{8CE1}', false)), // East Asian ideograph
    (0x21467E, ('\u{6CF3}', false)), // East Asian ideograph
    (0x21366B, ('\u{559D}', false)), // East Asian ideograph
    (0x2D5421, ('\u{9AD7}', false)), // East Asian ideograph
    (0x6F4A56, ('\u{AE7D}', false)), // Korean hangul
    (0x4C4359, ('\u{6B05}', false)), // East Asian ideograph
    (0x27366D, ('\u{5524}', false)), // East Asian ideograph
    (0x21366E, ('\u{557E}', false)), // East Asian ideograph
    (0x294869, ('\u{9567}', false)), // East Asian ideograph
    (0x284027, ('\u{6864}', false)), // East Asian ideograph
    (0x21366F, ('\u{55AC}', false)), // East Asian ideograph
    (0x213670, ('\u{5589}', false)), // East Asian ideograph
    (0x223671, ('\u{6595}', false)), // East Asian ideograph
    (0x213672, ('\u{55BB}', false)), // East Asian ideograph
    (0x27406C, ('\u{631F}', false)), // East Asian ideograph
    (0x4C3B60, ('\u{6764}', false)), // East Asian ideograph
    (0x294228, ('\u{94AC}', false)), // East Asian ideograph
    (0x213674, ('\u{55DF}', false)), // East Asian ideograph
    (0x213675, ('\u{55D1}', false)), // East Asian ideograph
    (0x213869, ('\u{58D3}', false)), // East Asian ideograph
    (0x28734E, ('\u{7F32}', false)), // East Asian ideograph
    (0x233676, ('\u{8CEE}', false)), // East Asian ideograph
    (0x216121, ('\u{993F}', false)), // East Asian ideograph
    (0x213677, ('\u{55E6}', false)), // East Asian ideograph
    (0x4B6122, ('\u{994B}', false)), // East Asian ideograph
    (0x6F4F58, ('\u{B985}', false)), // Korean hangul
    (0x273678, ('\u{556C}', false)), // East Asian ideograph
    (0x275E43, ('\u{94F8}', false)), // East Asian ideograph
    (0x216123, ('\u{9945}', false)), // East Asian ideograph
    (0x6F5263, ('\u{C0B0}', false)), // Korean hangul
    (0x21353D, ('\u{53F2}', false)), // East Asian ideograph
    (0x276124, ('\u{9976}', false)), // East Asian ideograph
    (0x27367A, ('\u{5417}', false)), // East Asian ideograph
    (0x2D5424, ('\u{5367}', false)), // East Asian ideograph
    (0x23367B, ('\u{8CF1}', false)), // East Asian ideograph
    (0x216126, ('\u{995C}', false)), // East Asian ideograph
    (0x6F5851, ('\u{CA54}', false)), // Korean hangul
    (0x21367C, ('\u{55EF}', false)), // East Asian ideograph
    (0x2D475B, ('\u{51C9}', false)), // East Asian ideograph
    (0x276127, ('\u{998B}', false)), // East Asian ideograph
    (0x6F4F59, ('\u{B987}', false)), // Korean hangul
    (0x21767D, ('\u{5844}', false)), // East Asian ideograph
    (0x275E44, ('\u{9573}', false)), // East Asian ideograph
    (0x21367E, ('\u{55C5}', false)), // East Asian ideograph
    (0x396C6B, ('\u{60A4}', false)), // East Asian ideograph
    (0x6F5B37, ('\u{D168}', false)), // Korean hangul
    (0x213E61, ('\u{60E1}', false)), // East Asian ideograph
    (0x224A4A, ('\u{6DE6}', false)), // East Asian ideograph
    (0x4B5D34, ('\u{91B8}', false)), // East Asian ideograph
    (0x27612C, ('\u{9A6C}', false)), // East Asian ideograph
    (0x217971, ('\u{59A0}', false)), // East Asian ideograph
    (0x21353F, ('\u{540B}', false)), // East Asian ideograph
    (0x27612E, ('\u{9A6D}', false)), // East Asian ideograph
    (0x2D6260, ('\u{5E85}', false)), // East Asian ideograph
    (0x27612F, ('\u{9A70}', false)), // East Asian ideograph
    (0x287351, ('\u{7F33}', false)), // East Asian ideograph
    (0x214721, ('\u{6CE3}', false)), // East Asian ideograph
    (0x214722, ('\u{6CF0}', false)), // East Asian ideograph
    (0x214723, ('\u{6CB8}', false)), // East Asian ideograph
    (0x214724, ('\u{6CD3}', false)), // East Asian ideograph
    (0x214725, ('\u{6CAB}', false)), // East Asian ideograph
    (0x214726, ('\u{6CE5}', false)), // East Asian ideograph
    (0x214727, ('\u{6CBD}', false)), // East Asian ideograph
    (0x214728, ('\u{6CB3}', false)), // East Asian ideograph
    (0x214729, ('\u{6CC4}', false)), // East Asian ideograph
    (0x21472A, ('\u{6CD5}', false)), // East Asian ideograph
    (0x21472B, ('\u{6CE2}', false)), // East Asian ideograph
    (0x21472C, ('\u{6CBC}', false)), // East Asian ideograph
    (0x21472D, ('\u{6CAE}', false)), // East Asian ideograph
    (0x21472E, ('\u{6CB9}', false)), // East Asian ideograph
    (0x21472F, ('\u{6CF1}', false)), // East Asian ideograph
    (0x214730, ('\u{6CC1}', false)), // East Asian ideograph
    (0x214731, ('\u{6CBE}', false)), // East Asian ideograph
    (0x214732, ('\u{6CC5}', false)), // East Asian ideograph
    (0x214733, ('\u{6CD7}', false)), // East Asian ideograph
    (0x234734, ('\u{9413}', false)), // East Asian ideograph
    (0x214735, ('\u{6CDB}', false)), // East Asian ideograph
    (0x214736, ('\u{6CE1}', false)), // East Asian ideograph
    (0x214737, ('\u{6CBF}', false)), // East Asian ideograph
    (0x214738, ('\u{6CCA}', false)), // East Asian ideograph
    (0x214739, ('\u{6CCC}', false)), // East Asian ideograph
    (0x21473A, ('\u{6CC9}', false)), // East Asian ideograph
    (0x21473B, ('\u{6D41}', false)), // East Asian ideograph
    (0x21473C, ('\u{6D0B}', false)), // East Asian ideograph
    (0x21473D, ('\u{6D32}', false)), // East Asian ideograph
    (0x21473E, ('\u{6D25}', false)), // East Asian ideograph
    (0x21473F, ('\u{6D31}', false)), // East Asian ideograph
    (0x214740, ('\u{6D2A}', false)), // East Asian ideograph
    (0x214741, ('\u{6D0C}', false)), // East Asian ideograph
    (0x214742, ('\u{6D1E}', false)), // East Asian ideograph
    (0x214743, ('\u{6D17}', false)), // East Asian ideograph
    (0x214744, ('\u{6D3B}', false)), // East Asian ideograph
    (0x214745, ('\u{6D1B}', false)), // East Asian ideograph
    (0x214746, ('\u{6D36}', false)), // East Asian ideograph
    (0x214747, ('\u{6D3D}', false)), // East Asian ideograph
    (0x214748, ('\u{6D3E}', false)), // East Asian ideograph
    (0x214749, ('\u{6D6A}', false)), // East Asian ideograph
    (0x21474A, ('\u{6D95}', false)), // East Asian ideograph
    (0x21474B, ('\u{6D78}', false)), // East Asian ideograph
    (0x21474C, ('\u{6D66}', false)), // East Asian ideograph
    (0x21474D, ('\u{6D59}', false)), // East Asian ideograph
    (0x21474E, ('\u{6D87}', false)), // East Asian ideograph
    (0x21474F, ('\u{6D88}', false)), // East Asian ideograph
    (0x214750, ('\u{6D6C}', false)), // East Asian ideograph
    (0x214751, ('\u{6D93}', false)), // East Asian ideograph
    (0x214752, ('\u{6D89}', false)), // East Asian ideograph
    (0x214753, ('\u{6D6E}', false)), // East Asian ideograph
    (0x214754, ('\u{6D74}', false)), // East Asian ideograph
    (0x214755, ('\u{6D5A}', false)), // East Asian ideograph
    (0x214756, ('\u{6D69}', false)), // East Asian ideograph
    (0x214757, ('\u{6D77}', false)), // East Asian ideograph
    (0x214758, ('\u{6DD9}', false)), // East Asian ideograph
    (0x214759, ('\u{6DDA}', false)), // East Asian ideograph
    (0x21475A, ('\u{6DF3}', false)), // East Asian ideograph
    (0x21475B, ('\u{6DBC}', false)), // East Asian ideograph
    (0x21475C, ('\u{6DE4}', false)), // East Asian ideograph
    (0x21475D, ('\u{6DB2}', false)), // East Asian ideograph
    (0x21475E, ('\u{6DE1}', false)), // East Asian ideograph
    (0x21475F, ('\u{6DD2}', false)), // East Asian ideograph
    (0x214760, ('\u{6DAE}', false)), // East Asian ideograph
    (0x214761, ('\u{6DF8}', false)), // East Asian ideograph
    (0x214762, ('\u{6DC7}', false)), // East Asian ideograph
    (0x214763, ('\u{6DCB}', false)), // East Asian ideograph
    (0x214764, ('\u{6DC5}', false)), // East Asian ideograph
    (0x214765, ('\u{6DDE}', false)), // East Asian ideograph
    (0x214766, ('\u{6DAF}', false)), // East Asian ideograph
    (0x214767, ('\u{6DB5}', false)), // East Asian ideograph
    (0x214768, ('\u{6DFA}', false)), // East Asian ideograph
    (0x214769, ('\u{6DF9}', false)), // East Asian ideograph
    (0x21476A, ('\u{6DCC}', false)), // East Asian ideograph
    (0x21476B, ('\u{6DF7}', false)), // East Asian ideograph
    (0x21476C, ('\u{6DB8}', false)), // East Asian ideograph
    (0x21476D, ('\u{6DD1}', false)), // East Asian ideograph
    (0x21476E, ('\u{6DF1}', false)), // East Asian ideograph
    (0x21476F, ('\u{6DE8}', false)), // East Asian ideograph
    (0x214770, ('\u{6DEB}', false)), // East Asian ideograph
    (0x214771, ('\u{6DD8}', false)), // East Asian ideograph
    (0x214772, ('\u{6DFB}', false)), // East Asian ideograph
    (0x214773, ('\u{6DEE}', false)), // East Asian ideograph
    (0x214774, ('\u{6DF5}', false)), // East Asian ideograph
    (0x214775, ('\u{6D8E}', false)), // East Asian ideograph
    (0x214776, ('\u{6DC6}', false)), // East Asian ideograph
    (0x214777, ('\u{6DEA}', false)), // East Asian ideograph
    (0x214778, ('\u{6DC4}', false)), // East Asian ideograph
    (0x214779, ('\u{6E54}', false)), // East Asian ideograph
    (0x21477A, ('\u{6E21}', false)), // East Asian ideograph
    (0x21477B, ('\u{6E38}', false)), // East Asian ideograph
    (0x21477C, ('\u{6E32}', false)), // East Asian ideograph
    (0x21477D, ('\u{6E67}', false)), // East Asian ideograph
    (0x21477E, ('\u{6E20}', false)), // East Asian ideograph
    (0x4B5D38, ('\u{91C8}', false)), // East Asian ideograph
    (0x226140, ('\u{76EC}', false)), // East Asian ideograph
    (0x6F4F5E, ('\u{B9B0}', false)), // Korean hangul
    (0x6F5936, ('\u{CC64}', false)), // Korean hangul
    (0x276141, ('\u{9A97}', false)), // East Asian ideograph
    (0x6F5777, ('\u{C950}', false)), // Korean hangul
    (0x29442E, ('\u{9502}', false)), // East Asian ideograph
    (0x276142, ('\u{9A9B}', false)), // East Asian ideograph
    (0x276143, ('\u{9A9E}', false)), // East Asian ideograph
    (0x274D3D, ('\u{76D1}', false)), // East Asian ideograph
    (0x6F5C28, ('\u{D38D}', false)), // Korean hangul
    (0x216144, ('\u{9A30}', false)), // East Asian ideograph
    (0x4B624F, ('\u{9D49}', false)), // East Asian ideograph
    (0x276145, ('\u{9A9A}', false)), // East Asian ideograph
    (0x6F4F5F, ('\u{B9B4}', false)), // Korean hangul
    (0x275E4A, ('\u{94BB}', false)), // East Asian ideograph
    (0x273C31, ('\u{5CE6}', false)), // East Asian ideograph
    (0x21575D, ('\u{88C2}', false)), // East Asian ideograph
    (0x6F585C, ('\u{CACD}', false)), // Korean hangul
    (0x217533, ('\u{5788}', false)), // East Asian ideograph
    (0x276147, ('\u{9A71}', false)), // East Asian ideograph
    (0x213860, ('\u{58B3}', false)), // East Asian ideograph
    (0x216148, ('\u{9A40}', false)), // East Asian ideograph
    (0x6F772D, ('\u{AE5F}', false)), // Korean hangul
    (0x287236, ('\u{7F07}', false)), // East Asian ideograph
    (0x6F5C29, ('\u{D38F}', false)), // Korean hangul
    (0x276149, ('\u{9AA1}', false)), // East Asian ideograph
    (0x27614A, ('\u{9A84}', false)), // East Asian ideograph
    (0x6F4C6D, ('\u{B2E4}', false)), // Korean hangul
    (0x6F4F60, ('\u{B9BC}', false)), // Korean hangul
    (0x22614B, ('\u{7704}', false)), // East Asian ideograph
    (0x225B5D, ('\u{74A1}', false)), // East Asian ideograph
    (0x27614C, ('\u{9A7F}', false)), // East Asian ideograph
    (0x27614D, ('\u{9A8C}', false)), // East Asian ideograph
    (0x27614E, ('\u{9AA4}', false)), // East Asian ideograph
    (0x21614F, ('\u{9A62}', false)), // East Asian ideograph
    (0x6F4F61, ('\u{B9BD}', false)), // Korean hangul
    (0x275E4C, ('\u{957F}', false)), // East Asian ideograph
    (0x216150, ('\u{9A65}', false)), // East Asian ideograph
    (0x21575F, ('\u{88DF}', false)), // East Asian ideograph
    (0x216151, ('\u{9A6A}', false)), // East Asian ideograph
    (0x226153, ('\u{76F7}', false)), // East Asian ideograph
    (0x293325, ('\u{8BF9}', false)), // East Asian ideograph
    (0x216154, ('\u{9AB0}', false)), // East Asian ideograph
    (0x6F4F62, ('\u{B9BF}', false)), // Korean hangul
    (0x235F5E, ('\u{9F39}', false)), // East Asian ideograph
    (0x213F3D, ('\u{61A7}', false)), // East Asian ideograph
    (0x216157, ('\u{9ABC}', false)), // East Asian ideograph
    (0x287359, ('\u{7F31}', false)), // East Asian ideograph
    (0x276158, ('\u{9AC5}', false)), // East Asian ideograph
    (0x216159, ('\u{9AD3}', false)), // East Asian ideograph
    (0x6F4F63, ('\u{B9C1}', false)), // Korean hangul
    (0x275E4E, ('\u{95E9}', false)), // East Asian ideograph
    (0x27615A, ('\u{4F53}', false)), // East Asian ideograph
    (0x277C24, ('\u{5A32}', false)), // East Asian ideograph
    (0x456036, ('\u{97FF}', false)), // East Asian ideograph
    (0x214821, ('\u{6E5B}', false)), // East Asian ideograph
    (0x214822, ('\u{6E1A}', false)), // East Asian ideograph
    (0x214823, ('\u{6E56}', false)), // East Asian ideograph
    (0x214824, ('\u{6E2F}', false)), // East Asian ideograph
    (0x214825, ('\u{6E6E}', false)), // East Asian ideograph
    (0x214826, ('\u{6E58}', false)), // East Asian ideograph
    (0x214827, ('\u{6E23}', false)), // East Asian ideograph
    (0x214828, ('\u{6E24}', false)), // East Asian ideograph
    (0x214829, ('\u{6E1B}', false)), // East Asian ideograph
    (0x21482A, ('\u{6E25}', false)), // East Asian ideograph
    (0x21482B, ('\u{6E4A}', false)), // East Asian ideograph
    (0x21482C, ('\u{6E3A}', false)), // East Asian ideograph
    (0x21482D, ('\u{6E6F}', false)), // East Asian ideograph
    (0x21482E, ('\u{6E2D}', false)), // East Asian ideograph
    (0x22482F, ('\u{6CE0}', false)), // East Asian ideograph
    (0x214830, ('\u{6E2C}', false)), // East Asian ideograph
    (0x214831, ('\u{6E26}', false)), // East Asian ideograph
    (0x214832, ('\u{6E4D}', false)), // East Asian ideograph
    (0x214833, ('\u{6E3E}', false)), // East Asian ideograph
    (0x214834, ('\u{6E43}', false)), // East Asian ideograph
    (0x214835, ('\u{6E19}', false)), // East Asian ideograph
    (0x214836, ('\u{6E1D}', false)), // East Asian ideograph
    (0x214837, ('\u{6ED3}', false)), // East Asian ideograph
    (0x214838, ('\u{6EB6}', false)), // East Asian ideograph
    (0x214839, ('\u{6EC2}', false)), // East Asian ideograph
    (0x21483B, ('\u{6EAF}', false)), // East Asian ideograph
    (0x21483C, ('\u{6EA2}', false)), // East Asian ideograph
    (0x27483D, ('\u{6C9F}', false)), // East Asian ideograph
    (0x23483E, ('\u{944C}', false)), // East Asian ideograph
    (0x21483F, ('\u{6EA5}', false)), // East Asian ideograph
    (0x214840, ('\u{6E98}', false)), // East Asian ideograph
    (0x214841, ('\u{6E90}', false)), // East Asian ideograph
    (0x214842, ('\u{6EC5}', false)), // East Asian ideograph
    (0x214843, ('\u{6EC7}', false)), // East Asian ideograph
    (0x214844, ('\u{6EBC}', false)), // East Asian ideograph
    (0x214845, ('\u{6EAB}', false)), // East Asian ideograph
    (0x214846, ('\u{6ED1}', false)), // East Asian ideograph
    (0x214847, ('\u{6ECB}', false)), // East Asian ideograph
    (0x214848, ('\u{6EC4}', false)), // East Asian ideograph
    (0x214849, ('\u{6ED4}', false)), // East Asian ideograph
    (0x21484A, ('\u{6EAA}', false)), // East Asian ideograph
    (0x21484B, ('\u{6E96}', false)), // East Asian ideograph
    (0x21484C, ('\u{6E9C}', false)), // East Asian ideograph
    (0x21484D, ('\u{6F33}', false)), // East Asian ideograph
    (0x21484E, ('\u{6EF4}', false)), // East Asian ideograph
    (0x21484F, ('\u{6EEC}', false)), // East Asian ideograph
    (0x214850, ('\u{6EFE}', false)), // East Asian ideograph
    (0x214851, ('\u{6F29}', false)), // East Asian ideograph
    (0x214852, ('\u{6F14}', false)), // East Asian ideograph
    (0x214853, ('\u{6F3E}', false)), // East Asian ideograph
    (0x214854, ('\u{6F2C}', false)), // East Asian ideograph
    (0x214855, ('\u{6F32}', false)), // East Asian ideograph
    (0x214856, ('\u{6F0F}', false)), // East Asian ideograph
    (0x214857, ('\u{6F22}', false)), // East Asian ideograph (variant of 4B4857 which maps to 6F22)
    (0x214858, ('\u{6EFF}', false)), // East Asian ideograph
    (0x214859, ('\u{6F23}', false)), // East Asian ideograph
    (0x21485A, ('\u{6F38}', false)), // East Asian ideograph
    (0x21485B, ('\u{6F15}', false)), // East Asian ideograph
    (0x21485C, ('\u{6F31}', false)), // East Asian ideograph
    (0x21485D, ('\u{6F02}', false)), // East Asian ideograph
    (0x21485E, ('\u{6F06}', false)), // East Asian ideograph
    (0x21485F, ('\u{6EEF}', false)), // East Asian ideograph
    (0x214860, ('\u{6F2B}', false)), // East Asian ideograph
    (0x214861, ('\u{6F2F}', false)), // East Asian ideograph
    (0x214862, ('\u{6F20}', false)), // East Asian ideograph
    (0x214863, ('\u{6F3F}', false)), // East Asian ideograph
    (0x214864, ('\u{6EF2}', false)), // East Asian ideograph
    (0x214865, ('\u{6F01}', false)), // East Asian ideograph
    (0x214866, ('\u{6F11}', false)), // East Asian ideograph
    (0x214867, ('\u{6ECC}', false)), // East Asian ideograph
    (0x214868, ('\u{6F2A}', false)), // East Asian ideograph
    (0x214869, ('\u{6F7C}', false)), // East Asian ideograph
    (0x21486A, ('\u{6F88}', false)), // East Asian ideograph
    (0x21486B, ('\u{6F84}', false)), // East Asian ideograph
    (0x21486C, ('\u{6F51}', false)), // East Asian ideograph
    (0x21486D, ('\u{6F64}', false)), // East Asian ideograph
    (0x21486E, ('\u{6F97}', false)), // East Asian ideograph
    (0x21486F, ('\u{6F54}', false)), // East Asian ideograph
    (0x214870, ('\u{6F7A}', false)), // East Asian ideograph
    (0x214871, ('\u{6F86}', false)), // East Asian ideograph
    (0x214872, ('\u{6F8E}', false)), // East Asian ideograph
    (0x214873, ('\u{6F6D}', false)), // East Asian ideograph
    (0x214874, ('\u{6F5B}', false)), // East Asian ideograph
    (0x214875, ('\u{6F6E}', false)), // East Asian ideograph
    (0x214876, ('\u{6F78}', false)), // East Asian ideograph
    (0x214877, ('\u{6F66}', false)), // East Asian ideograph
    (0x214878, ('\u{6F70}', false)), // East Asian ideograph
    (0x214879, ('\u{6F58}', false)), // East Asian ideograph
    (0x21487A, ('\u{6FC2}', false)), // East Asian ideograph
    (0x21487B, ('\u{6FB1}', false)), // East Asian ideograph
    (0x21487C, ('\u{6FC3}', false)), // East Asian ideograph
    (0x21487D, ('\u{6FA7}', false)), // East Asian ideograph
    (0x21487E, ('\u{6FA1}', false)), // East Asian ideograph
    (0x4D5875, ('\u{9CD0}', false)), // East Asian ideograph
    (0x2D5D65, ('\u{8216}', false)), // East Asian ideograph
    (0x28735D, ('\u{7EA9}', false)), // East Asian ideograph
    (0x6F5C30, ('\u{D3A8}', false)), // Korean hangul
    (0x22616C, ('\u{7722}', false)), // East Asian ideograph
    (0x22616D, ('\u{771A}', false)), // East Asian ideograph
    (0x6F4F67, ('\u{B9C9}', false)), // Korean hangul
    (0x6F4B24, ('\u{AFBC}', false)), // Korean hangul
    (0x21616F, ('\u{9B45}', false)), // East Asian ideograph
    (0x28336F, ('\u{629F}', false)), // East Asian ideograph
    (0x6F5C31, ('\u{D3A9}', false)), // Korean hangul
    (0x6F245E, ('\u{3147}', false)), // Korean hangul
    (0x4B5D42, ('\u{91E1}', false)), // East Asian ideograph
    (0x27407D, ('\u{626B}', false)), // East Asian ideograph
    (0x6F5029, ('\u{BA4E}', false)), // Korean hangul
    (0x2D4327, ('\u{6630}', false)), // East Asian ideograph
    (0x275E53, ('\u{5F00}', false)), // East Asian ideograph
    (0x276173, ('\u{9B47}', false)), // East Asian ideograph
    (0x3F3573, ('\u{8B3C}', false)), // East Asian ideograph
    (0x226174, ('\u{7740}', false)), // East Asian ideograph
    (0x6F4E41, ('\u{B610}', false)), // Korean hangul
    (0x4B4C36, ('\u{7575}', false)), // East Asian ideograph
    (0x216175, ('\u{9B77}', false)), // East Asian ideograph
    (0x224B34, ('\u{6E53}', false)), // East Asian ideograph
    (0x216176, ('\u{9B6F}', false)), // East Asian ideograph
    (0x214C21, ('\u{752C}', false)), // East Asian ideograph
    (0x226177, ('\u{7731}', false)), // East Asian ideograph
    (0x214C22, ('\u{752B}', false)), // East Asian ideograph
    (0x6F4F69, ('\u{B9CE}', false)), // Korean hangul
    (0x276178, ('\u{9C9B}', false)), // East Asian ideograph
    (0x6F4B26, ('\u{AFC7}', false)), // Korean hangul
    (0x276179, ('\u{9C9C}', false)), // East Asian ideograph
    (0x295269, ('\u{9A80}', false)), // East Asian ideograph
    (0x214C24, ('\u{7530}', false)), // East Asian ideograph
    (0x27617A, ('\u{9C94}', false)), // East Asian ideograph
    (0x2F3143, ('\u{89F5}', false)), // Unrelated variant of EACC 23315E which maps to 89F5
    (0x213A6B, ('\u{5B8F}', false)), // East Asian ideograph
    (0x27617B, ('\u{9CA8}', false)), // East Asian ideograph
    (0x214C26, ('\u{7531}', false)), // East Asian ideograph
    (0x27617C, ('\u{9CA4}', false)), // East Asian ideograph
    (0x214C27, ('\u{7533}', false)), // East Asian ideograph
    (0x6F4F6A, ('\u{B9CF}', false)), // Korean hangul
    (0x275E55, ('\u{95F4}', false)), // East Asian ideograph
    (0x27617D, ('\u{9CB8}', false)), // East Asian ideograph
    (0x6F4B27, ('\u{AFC8}', false)), // Korean hangul
    (0x29593B, ('\u{9C9F}', false)), // East Asian ideograph
    (0x27617E, ('\u{9CB3}', false)), // East Asian ideograph
    (0x214C29, ('\u{7538}', false)), // East Asian ideograph
    (0x215B60, ('\u{8FAD}', false)), // East Asian ideograph
    (0x214C2A, ('\u{753D}', false)), // East Asian ideograph
    (0x294237, ('\u{949B}', false)), // East Asian ideograph
    (0x6F5C34, ('\u{D3B4}', false)), // Korean hangul
    (0x39553C, ('\u{5D0B}', false)), // East Asian ideograph
    (0x6F5360, ('\u{C228}', false)), // Korean hangul
    (0x6F4C2B, ('\u{B153}', false)), // Korean hangul
    (0x2D4C2C, ('\u{583A}', false)), // East Asian ideograph
    (0x6F4F6B, ('\u{B9D0}', false)), // Korean hangul
    (0x274C2D, ('\u{4EA9}', false)), // East Asian ideograph
    (0x214C2E, ('\u{755C}', false)), // East Asian ideograph
    (0x2D3661, ('\u{6199}', false)), // East Asian ideograph
    (0x6F4C2F, ('\u{B15C}', false)), // Korean hangul
    (0x214921, ('\u{6FA4}', false)), // East Asian ideograph
    (0x214922, ('\u{6FC1}', false)), // East Asian ideograph
    (0x214924, ('\u{6FC0}', false)), // East Asian ideograph
    (0x214925, ('\u{6FB3}', false)), // East Asian ideograph
    (0x214926, ('\u{6FDF}', false)), // East Asian ideograph
    (0x214927, ('\u{6FD8}', false)), // East Asian ideograph
    (0x214928, ('\u{6FF1}', false)), // East Asian ideograph
    (0x214929, ('\u{6FE0}', false)), // East Asian ideograph
    (0x21492A, ('\u{6FEF}', false)), // East Asian ideograph
    (0x21492B, ('\u{6FEB}', false)), // East Asian ideograph (variant of 4B492B which maps to 6FEB)
    (0x21492C, ('\u{6FE1}', false)), // East Asian ideograph
    (0x21492D, ('\u{6FE4}', false)), // East Asian ideograph
    (0x21492E, ('\u{6F80}', false)), // East Asian ideograph
    (0x22492F, ('\u{6D34}', false)), // East Asian ideograph (variant of 34492F which maps to 6D34)
    (0x234930, ('\u{9588}', false)), // East Asian ideograph
    (0x214931, ('\u{700B}', false)), // East Asian ideograph
    (0x214932, ('\u{7009}', false)), // East Asian ideograph
    (0x214933, ('\u{7006}', false)), // East Asian ideograph
    (0x214934, ('\u{6FFA}', false)), // East Asian ideograph
    (0x214935, ('\u{7011}', false)), // East Asian ideograph
    (0x214936, ('\u{6FFE}', false)), // East Asian ideograph
    (0x214937, ('\u{700F}', false)), // East Asian ideograph
    (0x234938, ('\u{959F}', false)), // East Asian ideograph
    (0x214939, ('\u{701A}', false)), // East Asian ideograph
    (0x23493A, ('\u{95A0}', false)), // East Asian ideograph
    (0x21493B, ('\u{701D}', false)), // East Asian ideograph
    (0x22493C, ('\u{6D65}', false)), // East Asian ideograph
    (0x21493D, ('\u{701F}', false)), // East Asian ideograph
    (0x22493E, ('\u{6D5E}', false)), // East Asian ideograph
    (0x21493F, ('\u{703E}', false)), // East Asian ideograph
    (0x214940, ('\u{704C}', false)), // East Asian ideograph
    (0x214941, ('\u{7051}', false)), // East Asian ideograph
    (0x214942, ('\u{7058}', false)), // East Asian ideograph
    (0x274943, ('\u{6E7E}', false)), // East Asian ideograph
    (0x214944, ('\u{7064}', false)), // East Asian ideograph
    (0x214945, ('\u{706B}', false)), // East Asian ideograph
    (0x214946, ('\u{7070}', false)), // East Asian ideograph
    (0x214947, ('\u{7076}', false)), // East Asian ideograph
    (0x214948, ('\u{707C}', false)), // East Asian ideograph
    (0x214949, ('\u{7078}', false)), // East Asian ideograph
    (0x21494A, ('\u{707D}', false)), // East Asian ideograph
    (0x21494B, ('\u{7095}', false)), // East Asian ideograph
    (0x21494C, ('\u{708E}', false)), // East Asian ideograph
    (0x23494D, ('\u{95B9}', false)), // East Asian ideograph
    (0x21494E, ('\u{7099}', false)), // East Asian ideograph
    (0x21494F, ('\u{708A}', false)), // East Asian ideograph
    (0x214950, ('\u{70AB}', false)), // East Asian ideograph
    (0x214951, ('\u{70BA}', false)), // East Asian ideograph
    (0x214952, ('\u{70AC}', false)), // East Asian ideograph
    (0x214953, ('\u{70B3}', false)), // East Asian ideograph
    (0x214954, ('\u{70AF}', false)), // East Asian ideograph
    (0x214955, ('\u{70AD}', false)), // East Asian ideograph
    (0x214956, ('\u{70AE}', false)), // East Asian ideograph
    (0x214957, ('\u{70B8}', false)), // East Asian ideograph
    (0x214958, ('\u{70CA}', false)), // East Asian ideograph
    (0x214959, ('\u{70E4}', false)), // East Asian ideograph
    (0x21495A, ('\u{70D8}', false)), // East Asian ideograph
    (0x21495B, ('\u{70C8}', false)), // East Asian ideograph
    (0x21495C, ('\u{70D9}', false)), // East Asian ideograph
    (0x23495D, ('\u{95CE}', false)), // East Asian ideograph
    (0x21495E, ('\u{70F9}', false)), // East Asian ideograph
    (0x21495F, ('\u{7109}', false)), // East Asian ideograph
    (0x214960, ('\u{710A}', false)), // East Asian ideograph
    (0x214961, ('\u{70FD}', false)), // East Asian ideograph
    (0x214962, ('\u{7119}', false)), // East Asian ideograph
    (0x214963, ('\u{716E}', false)), // East Asian ideograph
    (0x214964, ('\u{711A}', false)), // East Asian ideograph
    (0x214965, ('\u{7136}', false)), // East Asian ideograph
    (0x214966, ('\u{7121}', false)), // East Asian ideograph
    (0x214967, ('\u{7130}', false)), // East Asian ideograph
    (0x214968, ('\u{7126}', false)), // East Asian ideograph
    (0x214969, ('\u{714E}', false)), // East Asian ideograph
    (0x21496A, ('\u{7149}', false)), // East Asian ideograph
    (0x21496B, ('\u{7159}', false)), // East Asian ideograph
    (0x21496C, ('\u{7164}', false)), // East Asian ideograph
    (0x21496D, ('\u{7169}', false)), // East Asian ideograph
    (0x21496E, ('\u{715C}', false)), // East Asian ideograph
    (0x21496F, ('\u{716C}', false)), // East Asian ideograph
    (0x214970, ('\u{7166}', false)), // East Asian ideograph
    (0x214971, ('\u{7167}', false)), // East Asian ideograph
    (0x214972, ('\u{715E}', false)), // East Asian ideograph
    (0x214973, ('\u{7165}', false)), // East Asian ideograph
    (0x214974, ('\u{714C}', false)), // East Asian ideograph
    (0x214975, ('\u{717D}', false)), // East Asian ideograph
    (0x234976, ('\u{95E7}', false)), // East Asian ideograph
    (0x214977, ('\u{7199}', false)), // East Asian ideograph
    (0x214978, ('\u{718A}', false)), // East Asian ideograph
    (0x214979, ('\u{7184}', false)), // East Asian ideograph
    (0x21497A, ('\u{719F}', false)), // East Asian ideograph
    (0x21497B, ('\u{71A8}', false)), // East Asian ideograph
    (0x21497C, ('\u{71AC}', false)), // East Asian ideograph
    (0x21497D, ('\u{71B1}', false)), // East Asian ideograph
    (0x21497E, ('\u{71D9}', false)), // East Asian ideograph
    (0x6F4C40, ('\u{B1DC}', false)), // Korean hangul
    (0x2D432E, ('\u{66EC}', false)), // East Asian ideograph
    (0x214C41, ('\u{7599}', false)), // East Asian ideograph
    (0x395E71, ('\u{5742}', false)), // East Asian ideograph
    (0x214C42, ('\u{759A}', false)), // East Asian ideograph
    (0x6F586E, ('\u{CB64}', false)), // Korean hangul
    (0x214C43, ('\u{75A4}', false)), // East Asian ideograph
    (0x6F5C39, ('\u{D3C5}', false)), // Korean hangul
    (0x224C44, ('\u{6F00}', false)), // East Asian ideograph
    (0x4B3B31, ('\u{5B9F}', false)), // East Asian ideograph
    (0x6F4C45, ('\u{B208}', false)), // Korean hangul
    (0x6F4F70, ('\u{B9DD}', false)), // Korean hangul
    (0x275E5B, ('\u{9601}', false)), // East Asian ideograph
    (0x6F4B2D, ('\u{AFD8}', false)), // Korean hangul
    (0x294440, ('\u{9506}', false)), // East Asian ideograph
    (0x333475, ('\u{9628}', false)), // East Asian ideograph
    (0x234C47, ('\u{9723}', false)), // East Asian ideograph
    (0x27727E, ('\u{54D9}', false)), // East Asian ideograph
    (0x21386E, ('\u{58DE}', false)), // East Asian ideograph
    (0x6F4C48, ('\u{B213}', false)), // Korean hangul
    (0x6F5C3A, ('\u{D3C8}', false)), // Korean hangul
    (0x214C49, ('\u{75B2}', false)), // East Asian ideograph
    (0x234E60, ('\u{97E1}', false)), // East Asian ideograph
    (0x214C4A, ('\u{75BD}', false)), // East Asian ideograph
    (0x6F4F71, ('\u{B9DE}', false)), // Korean hangul
    (0x275E5C, ('\u{9600}', false)), // East Asian ideograph
    (0x6F4877, ('\u{AC2D}', false)), // Korean hangul
    (0x214C4B, ('\u{75BE}', false)), // East Asian ideograph
    (0x294441, ('\u{9507}', false)), // East Asian ideograph
    (0x333D54, ('\u{4EFD}', false)), // East Asian ideograph
    (0x695C71, ('\u{6A78}', false)), // East Asian ideograph
    (0x276F69, ('\u{5459}', false)), // East Asian ideograph
    (0x6F5C3B, ('\u{D3C9}', false)), // Korean hangul
    (0x70603A, ('\u{55EA}', false)), // East Asian ideograph
    (0x69554E, ('\u{5B36}', false)), // East Asian ideograph
    (0x214C4E, ('\u{75D5}', false)), // East Asian ideograph
    (0x6F5852, ('\u{CA5C}', false)), // Korean hangul
    (0x6F2460, ('\u{314A}', false)), // Korean hangul
    (0x6F4C4F, ('\u{B258}', false)), // Korean hangul
    (0x6F4F72, ('\u{B9E1}', false)), // Korean hangul
    (0x275E5D, ('\u{5408}', false)), // East Asian ideograph
    (0x214C50, ('\u{75B5}', false)), // East Asian ideograph
    (0x214C51, ('\u{75CA}', false)), // East Asian ideograph (variant of 4B4C51 which maps to 75CA)
    (0x2E3F2D, ('\u{69B2}', false)), // East Asian ideograph
    (0x2F2A73, ('\u{87CA}', false)), // East Asian ideograph
    (0x214C52, ('\u{75DB}', false)), // East Asian ideograph
    (0x6F5C3C, ('\u{D3D0}', false)), // Korean hangul
    (0x285150, ('\u{70C3}', false)), // East Asian ideograph
    (0x213E66, ('\u{60B2}', false)), // East Asian ideograph
    (0x293336, ('\u{8BE4}', false)), // East Asian ideograph
    (0x6F4C54, ('\u{B274}', false)), // Korean hangul
    (0x6F4F73, ('\u{B9E3}', false)), // Korean hangul
    (0x275E5E, ('\u{9605}', false)), // East Asian ideograph
    (0x2E4731, ('\u{6C73}', false)), // East Asian ideograph
    (0x6F4B30, ('\u{B000}', false)), // Korean hangul
    (0x214C56, ('\u{75D9}', false)), // East Asian ideograph
    (0x214C57, ('\u{75E2}', false)), // East Asian ideograph
    (0x6F5C3D, ('\u{D3EC}', false)), // Korean hangul
    (0x234C58, ('\u{9730}', false)), // East Asian ideograph
    (0x6F4C59, ('\u{B294}', false)), // Korean hangul
    (0x275E5F, ('\u{95FE}', false)), // East Asian ideograph
    (0x214C5A, ('\u{75F0}', false)), // East Asian ideograph
    (0x394444, ('\u{8988}', false)), // East Asian ideograph
    (0x214A21, ('\u{71BE}', false)), // East Asian ideograph
    (0x214A22, ('\u{71C9}', false)), // East Asian ideograph
    (0x214A23, ('\u{71D0}', false)), // East Asian ideograph
    (0x214A24, ('\u{71C8}', false)), // East Asian ideograph
    (0x214A25, ('\u{71DC}', false)), // East Asian ideograph
    (0x214A26, ('\u{71D2}', false)), // East Asian ideograph
    (0x214A27, ('\u{71B9}', false)), // East Asian ideograph
    (0x214A28, ('\u{71D5}', false)), // East Asian ideograph
    (0x214A29, ('\u{71CE}', false)), // East Asian ideograph
    (0x214A2A, ('\u{71C3}', false)), // East Asian ideograph
    (0x214A2B, ('\u{71C4}', false)), // East Asian ideograph
    (0x214A2C, ('\u{71EE}', false)), // East Asian ideograph
    (0x214A2D, ('\u{71E7}', false)), // East Asian ideograph
    (0x214A2E, ('\u{71DF}', false)), // East Asian ideograph
    (0x214A2F, ('\u{71E5}', false)), // East Asian ideograph
    (0x214A30, ('\u{71ED}', false)), // East Asian ideograph
    (0x214A31, ('\u{71E6}', false)), // East Asian ideograph
    (0x234A32, ('\u{963C}', false)), // East Asian ideograph
    (0x214A33, ('\u{71F4}', false)), // East Asian ideograph
    (0x214A34, ('\u{71FB}', false)), // East Asian ideograph
    (0x224A35, ('\u{6DDD}', false)), // East Asian ideograph
    (0x274A36, ('\u{70C1}', false)), // East Asian ideograph
    (0x214A37, ('\u{7210}', false)), // East Asian ideograph
    (0x214A38, ('\u{721B}', false)), // East Asian ideograph
    (0x224A39, ('\u{6DDB}', false)), // East Asian ideograph
    (0x214A3A, ('\u{722A}', false)), // East Asian ideograph
    (0x214A3B, ('\u{722D}', false)), // East Asian ideograph
    (0x214A3C, ('\u{722C}', false)), // East Asian ideograph
    (0x214A3D, ('\u{7230}', false)), // East Asian ideograph
    (0x214A3E, ('\u{7235}', false)), // East Asian ideograph (variant of 4B4A3E which maps to 7235)
    (0x214A3F, ('\u{7236}', false)), // East Asian ideograph
    (0x214A40, ('\u{7238}', false)), // East Asian ideograph
    (0x214A41, ('\u{7239}', false)), // East Asian ideograph
    (0x214A42, ('\u{723A}', false)), // East Asian ideograph
    (0x214A43, ('\u{723B}', false)), // East Asian ideograph
    (0x214A44, ('\u{723D}', false)), // East Asian ideograph
    (0x214A45, ('\u{723E}', false)), // East Asian ideograph
    (0x224A46, ('\u{6DF0}', false)), // East Asian ideograph
    (0x214A47, ('\u{7247}', false)), // East Asian ideograph
    (0x214A48, ('\u{7248}', false)), // East Asian ideograph
    (0x214A49, ('\u{724C}', false)), // East Asian ideograph
    (0x214A4A, ('\u{7252}', false)), // East Asian ideograph
    (0x214A4B, ('\u{7256}', false)), // East Asian ideograph
    (0x214A4C, ('\u{7258}', false)), // East Asian ideograph
    (0x214A4D, ('\u{7259}', false)), // East Asian ideograph
    (0x214A4E, ('\u{725B}', false)), // East Asian ideograph
    (0x214A4F, ('\u{725F}', false)), // East Asian ideograph
    (0x214A50, ('\u{725D}', false)), // East Asian ideograph
    (0x214A51, ('\u{7262}', false)), // East Asian ideograph
    (0x214A52, ('\u{7261}', false)), // East Asian ideograph
    (0x214A53, ('\u{7260}', false)), // East Asian ideograph
    (0x214A54, ('\u{7267}', false)), // East Asian ideograph
    (0x214A55, ('\u{7269}', false)), // East Asian ideograph
    (0x214A56, ('\u{726F}', false)), // East Asian ideograph
    (0x214A57, ('\u{7272}', false)), // East Asian ideograph
    (0x214A58, ('\u{7274}', false)), // East Asian ideograph
    (0x214A59, ('\u{7279}', false)), // East Asian ideograph
    (0x214A5A, ('\u{727D}', false)), // East Asian ideograph
    (0x214A5B, ('\u{7281}', false)), // East Asian ideograph
    (0x214A5C, ('\u{7280}', false)), // East Asian ideograph
    (0x214A5D, ('\u{7284}', false)), // East Asian ideograph
    (0x274A5E, ('\u{8366}', false)), // East Asian ideograph
    (0x214A5F, ('\u{7292}', false)), // East Asian ideograph
    (0x224A60, ('\u{6E8A}', false)), // East Asian ideograph
    (0x214A61, ('\u{72A2}', false)), // East Asian ideograph
    (0x274A62, ('\u{727A}', false)), // East Asian ideograph
    (0x214A63, ('\u{72AC}', false)), // East Asian ideograph
    (0x214A64, ('\u{72AF}', false)), // East Asian ideograph
    (0x214A65, ('\u{72C4}', false)), // East Asian ideograph
    (0x214A66, ('\u{72C2}', false)), // East Asian ideograph
    (0x214A67, ('\u{72D9}', false)), // East Asian ideograph
    (0x274A68, ('\u{72B6}', false)), // East Asian ideograph
    (0x214A69, ('\u{72CE}', false)), // East Asian ideograph
    (0x214A6A, ('\u{72D7}', false)), // East Asian ideograph
    (0x214A6B, ('\u{72D0}', false)), // East Asian ideograph
    (0x214A6C, ('\u{72E1}', false)), // East Asian ideograph
    (0x214A6D, ('\u{72E9}', false)), // East Asian ideograph
    (0x214A6E, ('\u{72E0}', false)), // East Asian ideograph
    (0x214A6F, ('\u{72FC}', false)), // East Asian ideograph
    (0x274A70, ('\u{72ED}', false)), // East Asian ideograph
    (0x224A71, ('\u{6E73}', false)), // East Asian ideograph
    (0x214A72, ('\u{72FD}', false)), // East Asian ideograph
    (0x214A73, ('\u{72F7}', false)), // East Asian ideograph
    (0x214A74, ('\u{731C}', false)), // East Asian ideograph
    (0x214A75, ('\u{731B}', false)), // East Asian ideograph
    (0x214A76, ('\u{7313}', false)), // East Asian ideograph
    (0x214A77, ('\u{7316}', false)), // East Asian ideograph
    (0x214A78, ('\u{7319}', false)), // East Asian ideograph
    (0x214A79, ('\u{7336}', false)), // East Asian ideograph
    (0x214A7A, ('\u{7337}', false)), // East Asian ideograph
    (0x214A7B, ('\u{7329}', false)), // East Asian ideograph
    (0x214A7C, ('\u{7325}', false)), // East Asian ideograph
    (0x214A7D, ('\u{7334}', false)), // East Asian ideograph
    (0x214A7E, ('\u{7344}', false)), // East Asian ideograph
    (0x2D4C3C, ('\u{53E0}', false)), // East Asian ideograph
    (0x214C6B, ('\u{7634}', false)), // East Asian ideograph
    (0x6F5C41, ('\u{D3FC}', false)), // Korean hangul
    (0x213D4A, ('\u{5F46}', false)), // East Asian ideograph
    (0x214C6C, ('\u{7638}', false)), // East Asian ideograph
    (0x214C6D, ('\u{7646}', false)), // East Asian ideograph
    (0x6F4F78, ('\u{B9F4}', false)), // Korean hangul
    (0x275E63, ('\u{9611}', false)), // East Asian ideograph
    (0x234C6E, ('\u{9749}', false)), // East Asian ideograph
    (0x233D5B, ('\u{9046}', false)), // East Asian ideograph
    (0x6F4C6F, ('\u{B2E6}', false)), // Korean hangul
    (0x6F4C70, ('\u{B2E8}', false)), // Korean hangul
    (0x214C71, ('\u{7658}', false)), // East Asian ideograph
    (0x4D477B, ('\u{943E}', false)), // East Asian ideograph
    (0x35344D, ('\u{8B5B}', false)), // East Asian ideograph
    (0x6F4F79, ('\u{B9F5}', false)), // Korean hangul
    (0x275E64, ('\u{95F1}', false)), // East Asian ideograph
    (0x274C73, ('\u{75D2}', false)), // East Asian ideograph
    (0x21355E, ('\u{542E}', false)), // East Asian ideograph
    (0x275A21, ('\u{8D45}', false)), // East Asian ideograph
    (0x6F4C74, ('\u{B2EE}', false)), // Korean hangul
    (0x234147, ('\u{91AD}', false)), // East Asian ideograph
    (0x217D7C, ('\u{5B56}', false)), // East Asian ideograph
    (0x214C75, ('\u{7669}', false)), // East Asian ideograph
    (0x6F5C43, ('\u{D3FF}', false)), // Korean hangul
    (0x234C76, ('\u{975A}', false)), // East Asian ideograph
    (0x233721, ('\u{8CF7}', false)), // East Asian ideograph
    (0x287161, ('\u{7EFB}', false)), // East Asian ideograph
    (0x3A787D, ('\u{80FC}', false)), // East Asian ideograph
    (0x214C77, ('\u{766C}', false)), // East Asian ideograph
    (0x273722, ('\u{545B}', false)), // East Asian ideograph
    (0x275E65, ('\u{677F}', false)), // East Asian ideograph
    (0x214C78, ('\u{7671}', false)), // East Asian ideograph
    (0x213723, ('\u{55E1}', false)), // East Asian ideograph
    (0x21754E, ('\u{579D}', false)), // East Asian ideograph
    (0x214C79, ('\u{7672}', false)), // East Asian ideograph (variant of 4B4C79 which maps to 7672)
    (0x694C7A, ('\u{9453}', false)), // East Asian ideograph
    (0x213725, ('\u{561B}', false)), // East Asian ideograph
    (0x6F5C44, ('\u{D401}', false)), // Korean hangul
    (0x214C7B, ('\u{767C}', false)), // East Asian ideograph
    (0x233726, ('\u{8CFE}', false)), // East Asian ideograph
    (0x6F4C7C, ('\u{B2FF}', false)), // Korean hangul
    (0x223727, ('\u{65AE}', false)), // East Asian ideograph
    (0x2E4739, ('\u{6C67}', false)), // East Asian ideograph (variant of 224739 which maps to 6C67)
    (0x214C7D, ('\u{767D}', false)), // East Asian ideograph
    (0x6F7728, ('\u{AE07}', false)), // Korean hangul
    (0x215336, ('\u{80B4}', false)), // East Asian ideograph
    (0x2D4C7E, ('\u{4F70}', false)), // East Asian ideograph
    (0x217729, ('\u{582D}', false)), // East Asian ideograph
    (0x2D5447, ('\u{824A}', false)), // East Asian ideograph
    (0x21372A, ('\u{561F}', false)), // East Asian ideograph
    (0x6F5C45, ('\u{D440}', false)), // Korean hangul
    (0x2D3A47, ('\u{5ACB}', false)), // East Asian ideograph
    (0x4B4358, ('\u{66FD}', false)), // East Asian ideograph
    (0x23372B, ('\u{8D07}', false)), // East Asian ideograph
    (0x217850, ('\u{58D6}', false)), // East Asian ideograph
    (0x334A28, ('\u{91BC}', false)), // East Asian ideograph
    (0x27372C, ('\u{53F9}', false)), // East Asian ideograph
    (0x6F593C, ('\u{CCA0}', false)), // Korean hangul
    (0x275E67, ('\u{95EF}', false)), // East Asian ideograph
    (0x6F4B39, ('\u{B044}', false)), // Korean hangul
    (0x275A24, ('\u{8D3E}', false)), // East Asian ideograph
    (0x27372E, ('\u{5455}', false)), // East Asian ideograph
    (0x21372F, ('\u{560E}', false)), // East Asian ideograph
    (0x6F5C46, ('\u{D444}', false)), // Korean hangul
    (0x224A6D, ('\u{6E63}', false)), // East Asian ideograph
    (0x293340, ('\u{8C02}', false)), // East Asian ideograph
    (0x214B21, ('\u{733F}', false)), // East Asian ideograph
    (0x224B22, ('\u{6E28}', false)), // East Asian ideograph
    (0x274B23, ('\u{72EE}', false)), // East Asian ideograph
    (0x214B24, ('\u{7350}', false)), // East Asian ideograph
    (0x6F4B25, ('\u{AFC0}', false)), // Korean hangul
    (0x214B26, ('\u{7357}', false)), // East Asian ideograph
    (0x274B27, ('\u{72EC}', false)), // East Asian ideograph
    (0x224B28, ('\u{6E5E}', false)), // East Asian ideograph
    (0x274B29, ('\u{83B7}', false)), // East Asian ideograph
    (0x274B2A, ('\u{72B7}', false)), // East Asian ideograph
    (0x274B2B, ('\u{517D}', false)), // East Asian ideograph
    (0x224B2C, ('\u{6E84}', false)), // East Asian ideograph
    (0x223732, ('\u{65C3}', false)), // East Asian ideograph
    (0x224B2E, ('\u{6E2E}', false)), // East Asian ideograph
    (0x234B2F, ('\u{96A4}', false)), // East Asian ideograph
    (0x214B30, ('\u{7384}', false)), // East Asian ideograph
    (0x214B31, ('\u{7387}', false)), // East Asian ideograph
    (0x214B32, ('\u{7389}', false)), // East Asian ideograph
    (0x223733, ('\u{65C4}', false)), // East Asian ideograph
    (0x214B34, ('\u{7396}', false)), // East Asian ideograph
    (0x214B35, ('\u{739F}', false)), // East Asian ideograph
    (0x214B36, ('\u{73A8}', false)), // East Asian ideograph
    (0x214B37, ('\u{73A9}', false)), // East Asian ideograph
    (0x214B38, ('\u{73AB}', false)), // East Asian ideograph
    (0x214B39, ('\u{73BB}', false)), // East Asian ideograph
    (0x214B3A, ('\u{73CA}', false)), // East Asian ideograph
    (0x214B3B, ('\u{73B7}', false)), // East Asian ideograph
    (0x214B3C, ('\u{73C0}', false)), // East Asian ideograph
    (0x6F4B3D, ('\u{B04C}', false)), // Korean hangul
    (0x214B3E, ('\u{73B2}', false)), // East Asian ideograph
    (0x214B3F, ('\u{73CD}', false)), // East Asian ideograph
    (0x224B40, ('\u{6E2A}', false)), // East Asian ideograph
    (0x224B41, ('\u{6E4C}', false)), // East Asian ideograph
    (0x224B42, ('\u{6E22}', false)), // East Asian ideograph
    (0x224B43, ('\u{6ECE}', false)), // East Asian ideograph
    (0x214B44, ('\u{7409}', false)), // East Asian ideograph
    (0x224B45, ('\u{6E9B}', false)), // East Asian ideograph
    (0x224B46, ('\u{6E9F}', false)), // East Asian ideograph
    (0x214B47, ('\u{73FE}', false)), // East Asian ideograph
    (0x224B48, ('\u{6EC8}', false)), // East Asian ideograph
    (0x224B49, ('\u{6ED8}', false)), // East Asian ideograph
    (0x224B4A, ('\u{6E8F}', false)), // East Asian ideograph
    (0x214B4B, ('\u{7435}', false)), // East Asian ideograph
    (0x214B4C, ('\u{7436}', false)), // East Asian ideograph
    (0x224B4D, ('\u{6E93}', false)), // East Asian ideograph
    (0x214B4E, ('\u{742A}', false)), // East Asian ideograph
    (0x224B4F, ('\u{6EA0}', false)), // East Asian ideograph
    (0x214B50, ('\u{7422}', false)), // East Asian ideograph
    (0x224B51, ('\u{6EB1}', false)), // East Asian ideograph
    (0x234B52, ('\u{96CE}', false)), // East Asian ideograph
    (0x214B53, ('\u{7455}', false)), // East Asian ideograph
    (0x214B54, ('\u{745F}', false)), // East Asian ideograph
    (0x214B55, ('\u{745A}', false)), // East Asian ideograph
    (0x214B56, ('\u{7441}', false)), // East Asian ideograph
    (0x214B57, ('\u{743F}', false)), // East Asian ideograph
    (0x214B58, ('\u{745B}', false)), // East Asian ideograph
    (0x224B59, ('\u{6E92}', false)), // East Asian ideograph
    (0x224B5A, ('\u{6EA7}', false)), // East Asian ideograph
    (0x214B5B, ('\u{7459}', false)), // East Asian ideograph
    (0x214B5C, ('\u{7483}', false)), // East Asian ideograph
    (0x214B5D, ('\u{7469}', false)), // East Asian ideograph
    (0x274B5E, ('\u{739B}', false)), // East Asian ideograph
    (0x214B5F, ('\u{7463}', false)), // East Asian ideograph
    (0x214B60, ('\u{7464}', false)), // East Asian ideograph
    (0x214B61, ('\u{7470}', false)), // East Asian ideograph
    (0x214B62, ('\u{748B}', false)), // East Asian ideograph
    (0x214B63, ('\u{749C}', false)), // East Asian ideograph (variant of 4B4B63 which maps to 749C)
    (0x214B64, ('\u{74A3}', false)), // East Asian ideograph
    (0x214B65, ('\u{74A7}', false)), // East Asian ideograph
    (0x214B66, ('\u{74A9}', false)), // East Asian ideograph
    (0x214B67, ('\u{74B0}', false)), // East Asian ideograph
    (0x214B68, ('\u{74A6}', false)), // East Asian ideograph
    (0x214B69, ('\u{74BD}', false)), // East Asian ideograph
    (0x224B6A, ('\u{6EC9}', false)), // East Asian ideograph
    (0x274B6B, ('\u{73D1}', false)), // East Asian ideograph
    (0x224B6C, ('\u{6EB3}', false)), // East Asian ideograph
    (0x224B6D, ('\u{6EB7}', false)), // East Asian ideograph
    (0x214B6E, ('\u{74E2}', false)), // East Asian ideograph
    (0x214B6F, ('\u{74E3}', false)), // East Asian ideograph
    (0x214B70, ('\u{74E6}', false)), // East Asian ideograph
    (0x234B71, ('\u{96E9}', false)), // East Asian ideograph
    (0x214B72, ('\u{74F7}', false)), // East Asian ideograph
    (0x214B73, ('\u{7504}', false)), // East Asian ideograph
    (0x234B74, ('\u{96F1}', false)), // East Asian ideograph
    (0x214B75, ('\u{7515}', false)), // East Asian ideograph
    (0x234B76, ('\u{96F0}', false)), // East Asian ideograph
    (0x214B77, ('\u{751A}', false)), // East Asian ideograph
    (0x234B78, ('\u{96FA}', false)), // East Asian ideograph
    (0x224B79, ('\u{6ECF}', false)), // East Asian ideograph
    (0x214B7A, ('\u{7522}', false)), // East Asian ideograph
    (0x214B7B, ('\u{7526}', false)), // East Asian ideograph
    (0x224B7C, ('\u{6ECA}', false)), // East Asian ideograph
    (0x224B7D, ('\u{6ED5}', false)), // East Asian ideograph
    (0x214B7E, ('\u{7529}', false)), // East Asian ideograph
    (0x273740, ('\u{53FD}', false)), // East Asian ideograph
    (0x6F526B, ('\u{C0C0}', false)), // Korean hangul
    (0x294C76, ('\u{9753}', false)), // East Asian ideograph
    (0x213565, ('\u{542B}', false)), // East Asian ideograph
    (0x277742, ('\u{57D8}', false)), // East Asian ideograph
    (0x293344, ('\u{8C19}', false)), // East Asian ideograph
    (0x273744, ('\u{5428}', false)), // East Asian ideograph
    (0x27624F, ('\u{9E3E}', false)), // East Asian ideograph
    (0x223745, ('\u{65DC}', false)), // East Asian ideograph
    (0x6F593D, ('\u{CCA8}', false)), // Korean hangul
    (0x6F4B3E, ('\u{B053}', false)), // Korean hangul
    (0x213746, ('\u{5679}', false)), // East Asian ideograph
    (0x223747, ('\u{65DD}', false)), // East Asian ideograph
    (0x223748, ('\u{65DF}', false)), // East Asian ideograph
    (0x293345, ('\u{8BE8}', false)), // East Asian ideograph
    (0x217749, ('\u{583D}', false)), // East Asian ideograph
    (0x4B3B43, ('\u{5BFE}', false)), // East Asian ideograph
    (0x276175, ('\u{9C7F}', false)), // East Asian ideograph
    (0x21374A, ('\u{5671}', false)), // East Asian ideograph
    (0x6F5D70, ('\u{D760}', false)), // Korean hangul
    (0x6F4B3F, ('\u{B054}', false)), // Korean hangul
    (0x21374B, ('\u{566F}', false)), // East Asian ideograph
    (0x6F5863, ('\u{CB14}', false)), // Korean hangul
    (0x21374C, ('\u{5662}', false)), // East Asian ideograph (variant of 4B374C which maps to 5662)
    (0x282F47, ('\u{620B}', false)), // East Asian ideograph
    (0x22374E, ('\u{65E4}', false)), // East Asian ideograph
    (0x6F543A, ('\u{C314}', false)), // Korean hangul
    (0x6F4B40, ('\u{B055}', false)), // Korean hangul
    (0x692525, ('\u{30A5}', false)), // Katakana letter small U
    (0x4B4C51, ('\u{75CA}', false)), // East Asian ideograph
    (0x273751, ('\u{5413}', false)), // East Asian ideograph
    (0x213752, ('\u{5690}', false)), // East Asian ideograph
    (0x6F5C4D, ('\u{D488}', false)), // Korean hangul
    (0x70604C, ('\u{55F5}', false)), // East Asian ideograph
    (0x224A74, ('\u{6E4F}', false)), // East Asian ideograph
    (0x334E73, ('\u{79A5}', false)), // East Asian ideograph
    (0x234A30, ('\u{963D}', false)), // East Asian ideograph
    (0x273754, ('\u{565C}', false)), // East Asian ideograph
    (0x2D4343, ('\u{6636}', false)), // East Asian ideograph
    (0x2D4768, ('\u{6D45}', false)), // East Asian ideograph (variant of 274768 which maps to 6D45)
    (0x223755, ('\u{65F0}', false)), // East Asian ideograph
    (0x213756, ('\u{56A8}', false)), // East Asian ideograph
    (0x4B375A, ('\u{53B3}', false)), // East Asian ideograph
    (0x213757, ('\u{56B0}', false)), // East Asian ideograph
    (0x2D3758, ('\u{54BD}', false)), // East Asian ideograph
    (0x294162, ('\u{9486}', false)), // East Asian ideograph
    (0x212A3B, ('\u{E8E8}', false)), // EACC component character
    (0x284056, ('\u{6920}', false)), // East Asian ideograph
    (0x21375A, ('\u{56B4}', false)), // East Asian ideograph
    (0x6F592D, ('\u{CC44}', false)), // Korean hangul
    (0x224C21, ('\u{6EC3}', false)), // East Asian ideograph
    (0x234C22, ('\u{96FF}', false)), // East Asian ideograph
    (0x214C23, ('\u{752D}', false)), // East Asian ideograph
    (0x224C24, ('\u{6EB4}', false)), // East Asian ideograph
    (0x214C25, ('\u{7532}', false)), // East Asian ideograph
    (0x224C26, ('\u{6EB2}', false)), // East Asian ideograph
    (0x234C27, ('\u{9702}', false)), // East Asian ideograph
    (0x214C28, ('\u{7537}', false)), // East Asian ideograph
    (0x224C29, ('\u{6EB5}', false)), // East Asian ideograph
    (0x234C2A, ('\u{9705}', false)), // East Asian ideograph
    (0x214C2B, ('\u{754F}', false)), // East Asian ideograph
    (0x214C2C, ('\u{754C}', false)), // East Asian ideograph
    (0x214C2D, ('\u{755D}', false)), // East Asian ideograph
    (0x224C2E, ('\u{6EF8}', false)), // East Asian ideograph
    (0x214C2F, ('\u{7554}', false)), // East Asian ideograph
    (0x224C30, ('\u{6F37}', false)), // East Asian ideograph
    (0x214C31, ('\u{7559}', false)), // East Asian ideograph
    (0x214C32, ('\u{7566}', false)), // East Asian ideograph
    (0x214C33, ('\u{7562}', false)), // East Asian ideograph
    (0x224C34, ('\u{6EFD}', false)), // East Asian ideograph
    (0x224C35, ('\u{6F09}', false)), // East Asian ideograph
    (0x214C36, ('\u{756B}', false)), // East Asian ideograph
    (0x214C37, ('\u{756A}', false)), // East Asian ideograph
    (0x214C38, ('\u{7578}', false)), // East Asian ideograph
    (0x214C39, ('\u{7576}', false)), // East Asian ideograph
    (0x214C3A, ('\u{7586}', false)), // East Asian ideograph
    (0x214C3B, ('\u{7587}', false)), // East Asian ideograph
    (0x214C3C, ('\u{758A}', false)), // East Asian ideograph
    (0x224C3D, ('\u{6F63}', false)), // East Asian ideograph
    (0x224C3E, ('\u{6F12}', false)), // East Asian ideograph
    (0x214C3F, ('\u{7591}', false)), // East Asian ideograph
    (0x214C40, ('\u{759D}', false)), // East Asian ideograph
    (0x224C41, ('\u{6F1A}', false)), // East Asian ideograph
    (0x224C42, ('\u{6EF6}', false)), // East Asian ideograph
    (0x224C43, ('\u{6F19}', false)), // East Asian ideograph
    (0x214C44, ('\u{75AB}', false)), // East Asian ideograph
    (0x214C45, ('\u{75A5}', false)), // East Asian ideograph
    (0x214C46, ('\u{75C7}', false)), // East Asian ideograph
    (0x214C47, ('\u{75C5}', false)), // East Asian ideograph
    (0x214C48, ('\u{75B3}', false)), // East Asian ideograph
    (0x234C49, ('\u{9722}', false)), // East Asian ideograph
    (0x234C4A, ('\u{9724}', false)), // East Asian ideograph
    (0x224C4B, ('\u{6F24}', false)), // East Asian ideograph
    (0x214C4C, ('\u{75BC}', false)), // East Asian ideograph
    (0x214C4D, ('\u{75B9}', false)), // East Asian ideograph
    (0x234C4E, ('\u{9728}', false)), // East Asian ideograph
    (0x214C4F, ('\u{75D4}', false)), // East Asian ideograph
    (0x234C50, ('\u{9726}', false)), // East Asian ideograph
    (0x224C51, ('\u{6F18}', false)), // East Asian ideograph
    (0x234C52, ('\u{9731}', false)), // East Asian ideograph
    (0x214C53, ('\u{75E3}', false)), // East Asian ideograph
    (0x214C54, ('\u{75D8}', false)), // East Asian ideograph
    (0x214C55, ('\u{75DE}', false)), // East Asian ideograph
    (0x274C56, ('\u{75C9}', false)), // East Asian ideograph
    (0x224C57, ('\u{6F1F}', false)), // East Asian ideograph
    (0x214C58, ('\u{7601}', false)), // East Asian ideograph
    (0x214C59, ('\u{7600}', false)), // East Asian ideograph
    (0x224C5A, ('\u{6F0A}', false)), // East Asian ideograph
    (0x214C5B, ('\u{75F2}', false)), // East Asian ideograph
    (0x234C5C, ('\u{9736}', false)), // East Asian ideograph
    (0x214C5D, ('\u{75F4}', false)), // East Asian ideograph
    (0x214C5E, ('\u{75FF}', false)), // East Asian ideograph
    (0x214C5F, ('\u{75FA}', false)), // East Asian ideograph
    (0x224C60, ('\u{6EF9}', false)), // East Asian ideograph
    (0x224C61, ('\u{6EEE}', false)), // East Asian ideograph
    (0x224C62, ('\u{6F41}', false)), // East Asian ideograph
    (0x214C63, ('\u{760B}', false)), // East Asian ideograph
    (0x224C64, ('\u{6F95}', false)), // East Asian ideograph
    (0x214C65, ('\u{7620}', false)), // East Asian ideograph
    (0x214C66, ('\u{7629}', false)), // East Asian ideograph
    (0x214C67, ('\u{761F}', false)), // East Asian ideograph
    (0x214C68, ('\u{7624}', false)), // East Asian ideograph
    (0x214C69, ('\u{7626}', false)), // East Asian ideograph
    (0x214C6A, ('\u{7621}', false)), // East Asian ideograph
    (0x224C6B, ('\u{6F49}', false)), // East Asian ideograph
    (0x234C6C, ('\u{9746}', false)), // East Asian ideograph
    (0x224C6D, ('\u{6F30}', false)), // East Asian ideograph
    (0x214C6E, ('\u{7642}', false)), // East Asian ideograph
    (0x214C6F, ('\u{764C}', false)), // East Asian ideograph
    (0x214C70, ('\u{7656}', false)), // East Asian ideograph
    (0x274C71, ('\u{75A0}', false)), // East Asian ideograph
    (0x6F4C72, ('\u{B2EC}', false)), // Korean hangul
    (0x214C73, ('\u{7662}', false)), // East Asian ideograph
    (0x214C74, ('\u{7665}', false)), // East Asian ideograph
    (0x234C75, ('\u{9758}', false)), // East Asian ideograph
    (0x214C76, ('\u{766E}', false)), // East Asian ideograph
    (0x224C77, ('\u{6EEB}', false)), // East Asian ideograph
    (0x224C78, ('\u{6F08}', false)), // East Asian ideograph
    (0x224C79, ('\u{6F0E}', false)), // East Asian ideograph
    (0x214C7A, ('\u{7678}', false)), // East Asian ideograph
    (0x224C7B, ('\u{6F35}', false)), // East Asian ideograph
    (0x214C7C, ('\u{767B}', false)), // East Asian ideograph
    (0x234C7D, ('\u{9764}', false)), // East Asian ideograph
    (0x214C7E, ('\u{767E}', false)), // East Asian ideograph
    (0x21376B, ('\u{56F1}', false)), // East Asian ideograph
    (0x6F5C52, ('\u{D4E8}', false)), // Korean hangul
    (0x21376D, ('\u{5703}', false)), // East Asian ideograph
    (0x2D4348, ('\u{6681}', false)), // East Asian ideograph
    (0x2E4747, ('\u{6D64}', false)), // East Asian ideograph
    (0x3F424F, ('\u{542F}', false)), // East Asian ideograph (variant of 27424F which maps to 542F)
    (0x6F4B46, ('\u{B080}', false)), // Korean hangul
    (0x21376E, ('\u{5708}', false)), // East Asian ideograph
    (0x212A2F, ('\u{E8DD}', false)), // EACC component character
    (0x27376F, ('\u{56EF}', false)), // East Asian ideograph
    (0x273770, ('\u{56F4}', false)), // East Asian ideograph
    (0x6F5C53, ('\u{D504}', false)), // Korean hangul
    (0x27632B, ('\u{9F99}', false)), // East Asian ideograph
    (0x213771, ('\u{5712}', false)), // East Asian ideograph
    (0x294163, ('\u{948C}', false)), // East Asian ideograph
    (0x224637, ('\u{6BFA}', false)), // East Asian ideograph
    (0x213772, ('\u{5713}', false)), // East Asian ideograph
    (0x2D4349, ('\u{66A6}', false)), // East Asian ideograph
    (0x6F526D, ('\u{C0C5}', false)), // Korean hangul
    (0x213430, ('\u{52C9}', false)), // East Asian ideograph
    (0x6F4B47, ('\u{B084}', false)), // Korean hangul
    (0x275A32, ('\u{8D4F}', false)), // East Asian ideograph
    (0x216F21, ('\u{544F}', false)), // East Asian ideograph
    (0x213774, ('\u{5716}', false)), // East Asian ideograph
    (0x233775, ('\u{8D8D}', false)), // East Asian ideograph
    (0x6F4E2A, ('\u{B554}', false)), // Korean hangul
    (0x29334E, ('\u{8C0C}', false)), // East Asian ideograph
    (0x276221, ('\u{9CC3}', false)), // East Asian ideograph
    (0x213777, ('\u{572D}', false)), // East Asian ideograph
    (0x216222, ('\u{9C0D}', false)), // East Asian ideograph
    (0x6F4B48, ('\u{B08C}', false)), // Korean hangul
    (0x29445B, ('\u{9516}', false)), // East Asian ideograph
    (0x276223, ('\u{9CAB}', false)), // East Asian ideograph
    (0x276224, ('\u{9CCD}', false)), // East Asian ideograph
    (0x694C5D, ('\u{6762}', false)), // East Asian ideograph
    (0x222225, ('\u{5BEF}', false)), // East Asian ideograph
    (0x706054, ('\u{5623}', false)), // East Asian ideograph
    (0x395568, ('\u{83DD}', false)), // East Asian ideograph
    (0x234E7B, ('\u{980F}', false)), // East Asian ideograph
    (0x216226, ('\u{9C31}', false)), // East Asian ideograph
    (0x276177, ('\u{9C8D}', false)), // East Asian ideograph
    (0x21377C, ('\u{5751}', false)), // East Asian ideograph
    (0x276227, ('\u{9CD4}', false)), // East Asian ideograph
    (0x6F4B49, ('\u{B08D}', false)), // Korean hangul
    (0x21377D, ('\u{574A}', false)), // East Asian ideograph
    (0x276228, ('\u{9CD7}', false)), // East Asian ideograph
    (0x276229, ('\u{9CDD}', false)), // East Asian ideograph
    (0x6F5C56, ('\u{D50C}', false)), // Korean hangul
    (0x27622A, ('\u{9CDE}', false)), // East Asian ideograph
    (0x284D27, ('\u{6D9D}', false)), // East Asian ideograph
    (0x27622B, ('\u{9CDC}', false)), // East Asian ideograph
    (0x27622C, ('\u{9CD6}', false)), // East Asian ideograph
    (0x28405E, ('\u{67FD}', false)), // East Asian ideograph
    (0x21622D, ('\u{9C77}', false)), // East Asian ideograph
    (0x4B4C5B, ('\u{75F3}', false)), // East Asian ideograph
    (0x27622E, ('\u{9C88}', false)), // East Asian ideograph
    (0x2D5238, ('\u{898A}', false)), // East Asian ideograph
    (0x2E363F, ('\u{52C5}', false)), // East Asian ideograph
    (0x6F5C57, ('\u{D514}', false)), // Korean hangul
    (0x27622F, ('\u{9E1F}', false)), // East Asian ideograph
    (0x6F5959, ('\u{CDA7}', false)), // Korean hangul
    (0x214D21, ('\u{7682}', false)), // East Asian ideograph
    (0x214D22, ('\u{7684}', false)), // East Asian ideograph
    (0x214D23, ('\u{7687}', false)), // East Asian ideograph
    (0x214D24, ('\u{7686}', false)), // East Asian ideograph
    (0x234D25, ('\u{9767}', false)), // East Asian ideograph
    (0x214D26, ('\u{768E}', false)), // East Asian ideograph
    (0x214D27, ('\u{7696}', false)), // East Asian ideograph
    (0x214D28, ('\u{7693}', false)), // East Asian ideograph
    (0x214D29, ('\u{769A}', false)), // East Asian ideograph
    (0x214D2A, ('\u{76AE}', false)), // East Asian ideograph
    (0x214D2B, ('\u{76B0}', false)), // East Asian ideograph
    (0x214D2C, ('\u{76B4}', false)), // East Asian ideograph
    (0x274D2D, ('\u{76B1}', false)), // East Asian ideograph
    (0x214D2E, ('\u{76BF}', false)), // East Asian ideograph
    (0x214D2F, ('\u{76C2}', false)), // East Asian ideograph
    (0x224D30, ('\u{6F60}', false)), // East Asian ideograph
    (0x234D31, ('\u{9777}', false)), // East Asian ideograph
    (0x214D32, ('\u{76C6}', false)), // East Asian ideograph
    (0x214D33, ('\u{76CA}', false)), // East Asian ideograph
    (0x214D34, ('\u{76CD}', false)), // East Asian ideograph
    (0x214D35, ('\u{76CE}', false)), // East Asian ideograph
    (0x214D36, ('\u{76D4}', false)), // East Asian ideograph
    (0x214D37, ('\u{76D2}', false)), // East Asian ideograph
    (0x214D38, ('\u{76DC}', false)), // East Asian ideograph
    (0x214D39, ('\u{76DB}', false)), // East Asian ideograph
    (0x234D3A, ('\u{9780}', false)), // East Asian ideograph
    (0x214D3B, ('\u{76DF}', false)), // East Asian ideograph
    (0x234D3C, ('\u{9781}', false)), // East Asian ideograph
    (0x214D3D, ('\u{76E3}', false)), // East Asian ideograph
    (0x274D3E, ('\u{76D8}', false)), // East Asian ideograph
    (0x274D3F, ('\u{5362}', false)), // East Asian ideograph
    (0x214D40, ('\u{76E5}', false)), // East Asian ideograph
    (0x214D41, ('\u{76EA}', false)), // East Asian ideograph
    (0x214D42, ('\u{76EE}', false)), // East Asian ideograph
    (0x214D43, ('\u{76EF}', false)), // East Asian ideograph
    (0x214D44, ('\u{76F2}', false)), // East Asian ideograph
    (0x214D45, ('\u{76F4}', false)), // East Asian ideograph
    (0x214D46, ('\u{7709}', false)), // East Asian ideograph
    (0x214D47, ('\u{76F9}', false)), // East Asian ideograph
    (0x214D48, ('\u{76F8}', false)), // East Asian ideograph
    (0x214D49, ('\u{7701}', false)), // East Asian ideograph
    (0x214D4A, ('\u{770B}', false)), // East Asian ideograph
    (0x214D4B, ('\u{76FC}', false)), // East Asian ideograph
    (0x214D4C, ('\u{76FE}', false)), // East Asian ideograph
    (0x214D4D, ('\u{7729}', false)), // East Asian ideograph
    (0x214D4E, ('\u{7720}', false)), // East Asian ideograph
    (0x214D4F, ('\u{771E}', false)), // East Asian ideograph
    (0x214D50, ('\u{7728}', false)), // East Asian ideograph
    (0x214D51, ('\u{7737}', false)), // East Asian ideograph
    (0x214D52, ('\u{773C}', false)), // East Asian ideograph
    (0x214D53, ('\u{7736}', false)), // East Asian ideograph
    (0x214D54, ('\u{7738}', false)), // East Asian ideograph
    (0x214D55, ('\u{773A}', false)), // East Asian ideograph
    (0x274D56, ('\u{4F17}', false)), // East Asian ideograph
    (0x274D57, ('\u{56F0}', false)), // East Asian ideograph
    (0x214D58, ('\u{776B}', false)), // East Asian ideograph
    (0x214D59, ('\u{775B}', false)), // East Asian ideograph
    (0x214D5A, ('\u{776A}', false)), // East Asian ideograph
    (0x214D5B, ('\u{7766}', false)), // East Asian ideograph
    (0x214D5C, ('\u{7779}', false)), // East Asian ideograph
    (0x274D5D, ('\u{7750}', false)), // East Asian ideograph
    (0x214D5E, ('\u{7763}', false)), // East Asian ideograph
    (0x214D5F, ('\u{775C}', false)), // East Asian ideograph
    (0x214D60, ('\u{776C}', false)), // East Asian ideograph
    (0x214D61, ('\u{7768}', false)), // East Asian ideograph
    (0x214D62, ('\u{7765}', false)), // East Asian ideograph
    (0x214D63, ('\u{777D}', false)), // East Asian ideograph
    (0x214D64, ('\u{7771}', false)), // East Asian ideograph
    (0x214D65, ('\u{777F}', false)), // East Asian ideograph
    (0x214D66, ('\u{7784}', false)), // East Asian ideograph
    (0x214D67, ('\u{7761}', false)), // East Asian ideograph
    (0x214D68, ('\u{7787}', false)), // East Asian ideograph
    (0x214D69, ('\u{778E}', false)), // East Asian ideograph
    (0x214D6A, ('\u{778C}', false)), // East Asian ideograph
    (0x214D6B, ('\u{7791}', false)), // East Asian ideograph
    (0x214D6C, ('\u{779F}', false)), // East Asian ideograph
    (0x214D6D, ('\u{779E}', false)), // East Asian ideograph
    (0x214D6E, ('\u{77A0}', false)), // East Asian ideograph
    (0x214D6F, ('\u{77A5}', false)), // East Asian ideograph
    (0x214D70, ('\u{77B3}', false)), // East Asian ideograph
    (0x214D71, ('\u{77AA}', false)), // East Asian ideograph
    (0x214D72, ('\u{77B0}', false)), // East Asian ideograph
    (0x214D73, ('\u{77AD}', false)), // East Asian ideograph
    (0x214D74, ('\u{77AC}', false)), // East Asian ideograph
    (0x214D75, ('\u{77A7}', false)), // East Asian ideograph
    (0x214D76, ('\u{77BD}', false)), // East Asian ideograph
    (0x214D77, ('\u{77BF}', false)), // East Asian ideograph
    (0x214D78, ('\u{77BB}', false)), // East Asian ideograph
    (0x224D79, ('\u{6FA6}', false)), // East Asian ideograph
    (0x214D7A, ('\u{77D3}', false)), // East Asian ideograph
    (0x214D7B, ('\u{77D7}', false)), // East Asian ideograph
    (0x214D7C, ('\u{77DA}', false)), // East Asian ideograph
    (0x214D7D, ('\u{77DB}', false)), // East Asian ideograph
    (0x214D7E, ('\u{77DC}', false)), // East Asian ideograph
    (0x276240, ('\u{9E44}', false)), // East Asian ideograph
    (0x216241, ('\u{9D5D}', false)), // East Asian ideograph
    (0x233D74, ('\u{9062}', false)), // East Asian ideograph
    (0x216242, ('\u{9D89}', false)), // East Asian ideograph
    (0x293B6D, ('\u{8F8A}', false)), // East Asian ideograph
    (0x276243, ('\u{9E4A}', false)), // East Asian ideograph
    (0x6F4D6A, ('\u{B4F1}', false)), // Korean hangul
    (0x216244, ('\u{9D6A}', false)), // East Asian ideograph
    (0x216245, ('\u{9D6C}', false)), // East Asian ideograph
    (0x6F4B4F, ('\u{B098}', false)), // Korean hangul
    (0x276246, ('\u{9E64}', false)), // East Asian ideograph
    (0x333D75, ('\u{5FB3}', false)), // East Asian ideograph
    (0x276247, ('\u{83BA}', false)), // East Asian ideograph
    (0x6F5C5C, ('\u{D544}', false)), // Korean hangul
    (0x232248, ('\u{8453}', false)), // East Asian ideograph
    (0x276249, ('\u{9E67}', false)), // East Asian ideograph
    (0x27624A, ('\u{9E25}', false)), // East Asian ideograph
    (0x27624B, ('\u{9E36}', false)), // East Asian ideograph
    (0x27624C, ('\u{9E70}', false)), // East Asian ideograph
    (0x2E3645, ('\u{69E3}', false)), // East Asian ideograph
    (0x21624D, ('\u{9DFA}', false)), // East Asian ideograph
    (0x293357, ('\u{8C14}', false)), // East Asian ideograph
    (0x27624E, ('\u{9E66}', false)), // East Asian ideograph
    (0x21624F, ('\u{9E1E}', false)), // East Asian ideograph
    (0x4B4F4C, ('\u{7A4F}', false)), // East Asian ideograph
    (0x6F4B51, ('\u{B09A}', false)), // Korean hangul
    (0x276250, ('\u{54B8}', false)), // East Asian ideograph
    (0x294021, ('\u{90F8}', false)), // East Asian ideograph
    (0x235B4D, ('\u{9D7B}', false)), // East Asian ideograph
    (0x233934, ('\u{8DEC}', false)), // East Asian ideograph
    (0x6F5C5E, ('\u{D54D}', false)), // Korean hangul
    (0x216252, ('\u{9E7C}', false)), // East Asian ideograph
    (0x344177, ('\u{8264}', false)), // East Asian ideograph
    (0x39526B, ('\u{7094}', false)), // East Asian ideograph
    (0x216256, ('\u{9E97}', false)), // East Asian ideograph
    (0x2D5461, ('\u{8306}', false)), // East Asian ideograph
    (0x6F5C5F, ('\u{D54F}', false)), // Korean hangul
    (0x274931, ('\u{6C88}', false)), // East Asian ideograph
    (0x293359, ('\u{8C11}', false)), // East Asian ideograph
    (0x4B5D70, ('\u{92AD}', false)), // East Asian ideograph
    (0x234A42, ('\u{9660}', false)), // East Asian ideograph
    (0x216259, ('\u{9E9D}', false)), // East Asian ideograph
    (0x6F4B53, ('\u{B09F}', false)), // Korean hangul
    (0x294466, ('\u{9515}', false)), // East Asian ideograph
    (0x214E21, ('\u{77E2}', false)), // East Asian ideograph
    (0x214E22, ('\u{77E3}', false)), // East Asian ideograph
    (0x214E23, ('\u{77E5}', false)), // East Asian ideograph
    (0x214E24, ('\u{77E9}', false)), // East Asian ideograph
    (0x214E25, ('\u{77ED}', false)), // East Asian ideograph
    (0x214E26, ('\u{77EE}', false)), // East Asian ideograph
    (0x214E27, ('\u{77EF}', false)), // East Asian ideograph
    (0x214E28, ('\u{77F3}', false)), // East Asian ideograph
    (0x214E29, ('\u{77FD}', false)), // East Asian ideograph
    (0x214E2A, ('\u{7802}', false)), // East Asian ideograph
    (0x214E2B, ('\u{780D}', false)), // East Asian ideograph
    (0x214E2C, ('\u{780C}', false)), // East Asian ideograph
    (0x234E2D, ('\u{97B8}', false)), // East Asian ideograph
    (0x214E2E, ('\u{7830}', false)), // East Asian ideograph
    (0x214E2F, ('\u{781D}', false)), // East Asian ideograph
    (0x214E30, ('\u{7834}', false)), // East Asian ideograph
    (0x214E31, ('\u{7838}', false)), // East Asian ideograph
    (0x214E32, ('\u{7837}', false)), // East Asian ideograph
    (0x214E33, ('\u{7827}', false)), // East Asian ideograph
    (0x214E34, ('\u{782D}', false)), // East Asian ideograph
    (0x214E35, ('\u{7825}', false)), // East Asian ideograph
    (0x214E36, ('\u{786B}', false)), // East Asian ideograph
    (0x214E37, ('\u{784F}', false)), // East Asian ideograph
    (0x234E38, ('\u{97C0}', false)), // East Asian ideograph
    (0x214E39, ('\u{786C}', false)), // East Asian ideograph
    (0x214E3A, ('\u{785D}', false)), // East Asian ideograph
    (0x214E3B, ('\u{786F}', false)), // East Asian ideograph
    (0x214E3C, ('\u{78B0}', false)), // East Asian ideograph
    (0x214E3D, ('\u{7897}', false)), // East Asian ideograph
    (0x214E3E, ('\u{788E}', false)), // East Asian ideograph
    (0x214E3F, ('\u{7898}', false)), // East Asian ideograph
    (0x214E40, ('\u{7889}', false)), // East Asian ideograph
    (0x214E41, ('\u{7891}', false)), // East Asian ideograph
    (0x214E42, ('\u{787C}', false)), // East Asian ideograph
    (0x214E43, ('\u{788C}', false)), // East Asian ideograph
    (0x214E44, ('\u{78A7}', false)), // East Asian ideograph
    (0x214E45, ('\u{78A9}', false)), // East Asian ideograph
    (0x214E46, ('\u{789F}', false)), // East Asian ideograph
    (0x214E47, ('\u{78B3}', false)), // East Asian ideograph
    (0x214E48, ('\u{78CB}', false)), // East Asian ideograph
    (0x214E49, ('\u{78BA}', false)), // East Asian ideograph
    (0x214E4A, ('\u{78C1}', false)), // East Asian ideograph
    (0x214E4B, ('\u{78C5}', false)), // East Asian ideograph
    (0x214E4C, ('\u{78BC}', false)), // East Asian ideograph
    (0x214E4D, ('\u{78D5}', false)), // East Asian ideograph
    (0x214E4E, ('\u{78BE}', false)), // East Asian ideograph
    (0x214E4F, ('\u{78CA}', false)), // East Asian ideograph
    (0x214E50, ('\u{78D0}', false)), // East Asian ideograph
    (0x214E51, ('\u{78E8}', false)), // East Asian ideograph
    (0x214E52, ('\u{78EC}', false)), // East Asian ideograph
    (0x214E53, ('\u{78DA}', false)), // East Asian ideograph
    (0x214E54, ('\u{78F7}', false)), // East Asian ideograph
    (0x214E55, ('\u{78F4}', false)), // East Asian ideograph
    (0x214E56, ('\u{78FA}', false)), // East Asian ideograph (variant of 4B4E56 which maps to 78FA)
    (0x214E57, ('\u{7901}', false)), // East Asian ideograph
    (0x214E58, ('\u{78EF}', false)), // East Asian ideograph
    (0x234E59, ('\u{97DD}', false)), // East Asian ideograph
    (0x214E5A, ('\u{7919}', false)), // East Asian ideograph
    (0x214E5B, ('\u{7926}', false)), // East Asian ideograph
    (0x214E5C, ('\u{792C}', false)), // East Asian ideograph
    (0x224E5D, ('\u{6FDE}', false)), // East Asian ideograph
    (0x214E5E, ('\u{792B}', false)), // East Asian ideograph
    (0x214E5F, ('\u{793A}', false)), // East Asian ideograph
    (0x214E60, ('\u{7940}', false)), // East Asian ideograph
    (0x214E61, ('\u{793E}', false)), // East Asian ideograph
    (0x214E62, ('\u{7941}', false)), // East Asian ideograph
    (0x214E63, ('\u{7945}', false)), // East Asian ideograph
    (0x214E64, ('\u{7949}', false)), // East Asian ideograph
    (0x214E65, ('\u{7948}', false)), // East Asian ideograph
    (0x214E66, ('\u{7947}', false)), // East Asian ideograph
    (0x224E67, ('\u{700C}', false)), // East Asian ideograph
    (0x214E68, ('\u{7960}', false)), // East Asian ideograph
    (0x214E69, ('\u{7950}', false)), // East Asian ideograph
    (0x214E6A, ('\u{7956}', false)), // East Asian ideograph
    (0x214E6B, ('\u{795E}', false)), // East Asian ideograph
    (0x214E6C, ('\u{795D}', false)), // East Asian ideograph
    (0x214E6D, ('\u{795F}', false)), // East Asian ideograph
    (0x214E6E, ('\u{795A}', false)), // East Asian ideograph
    (0x214E6F, ('\u{7957}', false)), // East Asian ideograph
    (0x214E70, ('\u{7965}', false)), // East Asian ideograph
    (0x214E71, ('\u{7968}', false)), // East Asian ideograph
    (0x214E72, ('\u{796D}', false)), // East Asian ideograph
    (0x234E73, ('\u{97FA}', false)), // East Asian ideograph
    (0x214E74, ('\u{7981}', false)), // East Asian ideograph
    (0x214E75, ('\u{797F}', false)), // East Asian ideograph
    (0x214E76, ('\u{798F}', false)), // East Asian ideograph
    (0x214E77, ('\u{798D}', false)), // East Asian ideograph
    (0x214E78, ('\u{798E}', false)), // East Asian ideograph
    (0x214E79, ('\u{79A6}', false)), // East Asian ideograph
    (0x214E7A, ('\u{79A7}', false)), // East Asian ideograph
    (0x214E7B, ('\u{79AA}', false)), // East Asian ideograph
    (0x214E7C, ('\u{79AE}', false)), // East Asian ideograph
    (0x214E7D, ('\u{79B1}', false)), // East Asian ideograph
    (0x214E7E, ('\u{79B9}', false)), // East Asian ideograph
    (0x6F5C63, ('\u{D558}', false)), // Korean hangul
    (0x6F4E2D, ('\u{B55F}', false)), // Korean hangul
    (0x29335D, ('\u{8C16}', false)), // East Asian ideograph
    (0x216461, ('\u{4EC8}', false)), // East Asian ideograph
    (0x234A46, ('\u{9658}', false)), // East Asian ideograph
    (0x69626D, ('\u{7874}', false)), // East Asian ideograph
    (0x6F4B57, ('\u{B0A9}', false)), // Korean hangul
    (0x27626F, ('\u{515A}', false)), // East Asian ideograph
    (0x6F5C64, ('\u{D559}', false)), // Korean hangul
    (0x274936, ('\u{6EE4}', false)), // East Asian ideograph
    (0x6F5821, ('\u{C974}', false)), // Korean hangul
    (0x6F4D53, ('\u{B450}', false)), // Korean hangul
    (0x216271, ('\u{9EF4}', false)), // East Asian ideograph
    (0x216272, ('\u{9EF7}', false)), // East Asian ideograph
    (0x4B6159, ('\u{81B8}', false)), // East Asian ideograph
    (0x216273, ('\u{9F07}', false)), // East Asian ideograph
    (0x216275, ('\u{9F13}', false)), // East Asian ideograph
    (0x274937, ('\u{6D4F}', false)), // East Asian ideograph
    (0x6F5822, ('\u{C988}', false)), // Korean hangul
    (0x216276, ('\u{9F15}', false)), // East Asian ideograph
    (0x274633, ('\u{6B8B}', false)), // East Asian ideograph
    (0x6F4D22, ('\u{B308}', false)), // Korean hangul
    (0x6F4B59, ('\u{B0AC}', false)), // Korean hangul
    (0x4B6278, ('\u{9F21}', false)), // East Asian ideograph
    (0x224D23, ('\u{6F7E}', false)), // East Asian ideograph
    (0x21712D, ('\u{5593}', false)), // East Asian ideograph
    (0x224D24, ('\u{6F9D}', false)), // East Asian ideograph
    (0x21627A, ('\u{9F34}', false)), // East Asian ideograph
    (0x6F4D25, ('\u{B313}', false)), // Korean hangul
    (0x6F5823, ('\u{C989}', false)), // Korean hangul
    (0x23227B, ('\u{8484}', false)), // East Asian ideograph
    (0x22464A, ('\u{6C05}', false)), // East Asian ideograph
    (0x6F4D26, ('\u{B314}', false)), // Korean hangul
    (0x6F534B, ('\u{C1B0}', false)), // Korean hangul
    (0x23227C, ('\u{8478}', false)), // East Asian ideograph
    (0x224D27, ('\u{6F87}', false)), // East Asian ideograph
    (0x6F4B5A, ('\u{B0AD}', false)), // Korean hangul
    (0x21627D, ('\u{9F4A}', false)), // East Asian ideograph
    (0x6F4D28, ('\u{B354}', false)), // Korean hangul
    (0x27627E, ('\u{658E}', false)), // East Asian ideograph
    (0x274D29, ('\u{7691}', false)), // East Asian ideograph
    (0x274D7C, ('\u{77A9}', false)), // East Asian ideograph
    (0x6F5C67, ('\u{D565}', false)), // Korean hangul
    (0x6F4D2A, ('\u{B358}', false)), // Korean hangul
    (0x6F5824, ('\u{C98C}', false)), // Korean hangul
    (0x213C7A, ('\u{5EB8}', false)), // East Asian ideograph
    (0x224D2B, ('\u{6F6F}', false)), // East Asian ideograph
    (0x6F5271, ('\u{C0CF}', false)), // Korean hangul
    (0x234D2C, ('\u{976B}', false)), // East Asian ideograph
    (0x6F4B5B, ('\u{B0AE}', false)), // Korean hangul
    (0x214D2D, ('\u{76BA}', false)), // East Asian ideograph
    (0x6F4C39, ('\u{B192}', false)), // Korean hangul
    (0x29402B, ('\u{90BA}', false)), // East Asian ideograph
    (0x23393E, ('\u{8DF2}', false)), // East Asian ideograph
    (0x282D79, ('\u{60AB}', false)), // East Asian ideograph
    (0x6F4D2E, ('\u{B364}', false)), // Korean hangul
    (0x2D3251, ('\u{510C}', false)), // East Asian ideograph
    (0x6F492C, ('\u{AC84}', false)), // Korean hangul
    (0x6F5C68, ('\u{D568}', false)), // Korean hangul
    (0x224D2F, ('\u{6F5A}', false)), // East Asian ideograph
    (0x293362, ('\u{8C1D}', false)), // East Asian ideograph
    (0x214F21, ('\u{79BD}', false)), // East Asian ideograph
    (0x214F22, ('\u{842C}', false)), // East Asian ideograph
    (0x214F23, ('\u{79BE}', false)), // East Asian ideograph
    (0x214F24, ('\u{79C0}', false)), // East Asian ideograph
    (0x214F25, ('\u{79C1}', false)), // East Asian ideograph
    (0x214F26, ('\u{79BF}', false)), // East Asian ideograph
    (0x214D31, ('\u{76C8}', false)), // East Asian ideograph
    (0x214F28, ('\u{79D1}', false)), // East Asian ideograph
    (0x214F29, ('\u{79CB}', false)), // East Asian ideograph
    (0x214F2A, ('\u{79D2}', false)), // East Asian ideograph
    (0x214F2B, ('\u{79E4}', false)), // East Asian ideograph
    (0x214F2C, ('\u{79E6}', false)), // East Asian ideograph
    (0x214F2D, ('\u{79E3}', false)), // East Asian ideograph
    (0x214F2E, ('\u{79DF}', false)), // East Asian ideograph
    (0x214F2F, ('\u{79E7}', false)), // East Asian ideograph
    (0x214F30, ('\u{79E9}', false)), // East Asian ideograph
    (0x224F31, ('\u{702D}', false)), // East Asian ideograph
    (0x214F32, ('\u{7A05}', false)), // East Asian ideograph
    (0x214F33, ('\u{7A0D}', false)), // East Asian ideograph
    (0x214F34, ('\u{7A08}', false)), // East Asian ideograph
    (0x214F35, ('\u{7A0B}', false)), // East Asian ideograph
    (0x214F36, ('\u{7A00}', false)), // East Asian ideograph
    (0x214F37, ('\u{7A1F}', false)), // East Asian ideograph
    (0x234F38, ('\u{981F}', false)), // East Asian ideograph
    (0x214F39, ('\u{7A20}', false)), // East Asian ideograph
    (0x214F3A, ('\u{7A1A}', false)), // East Asian ideograph
    (0x214F3B, ('\u{7A14}', false)), // East Asian ideograph
    (0x214F3C, ('\u{7A31}', false)), // East Asian ideograph
    (0x214F3D, ('\u{7A2E}', false)), // East Asian ideograph
    (0x214F3E, ('\u{7A3F}', false)), // East Asian ideograph
    (0x214F3F, ('\u{7A3C}', false)), // East Asian ideograph
    (0x274F40, ('\u{8C37}', false)), // East Asian ideograph
    (0x214F41, ('\u{7A3D}', false)), // East Asian ideograph
    (0x214F42, ('\u{7A37}', false)), // East Asian ideograph
    (0x214F43, ('\u{7A3B}', false)), // East Asian ideograph
    (0x214F44, ('\u{7A4D}', false)), // East Asian ideograph
    (0x214F45, ('\u{7A4E}', false)), // East Asian ideograph
    (0x214F46, ('\u{7A4C}', false)), // East Asian ideograph
    (0x214F47, ('\u{7A46}', false)), // East Asian ideograph
    (0x214F48, ('\u{7A57}', false)), // East Asian ideograph
    (0x274F49, ('\u{7A51}', false)), // East Asian ideograph
    (0x214F4A, ('\u{7A62}', false)), // East Asian ideograph
    (0x274F4B, ('\u{83B7}', false)), // East Asian ideograph (duplicate simplified)
    (0x214F4C, ('\u{7A69}', false)), // East Asian ideograph
    (0x214F4D, ('\u{7A74}', false)), // East Asian ideograph
    (0x214F4E, ('\u{7A76}', false)), // East Asian ideograph
    (0x214F4F, ('\u{7A79}', false)), // East Asian ideograph
    (0x214F50, ('\u{7A7A}', false)), // East Asian ideograph
    (0x214F51, ('\u{7A7F}', false)), // East Asian ideograph
    (0x214F52, ('\u{7A81}', false)), // East Asian ideograph
    (0x214F53, ('\u{7A84}', false)), // East Asian ideograph
    (0x214F54, ('\u{7A88}', false)), // East Asian ideograph
    (0x214F55, ('\u{7A92}', false)), // East Asian ideograph
    (0x214F56, ('\u{7A95}', false)), // East Asian ideograph
    (0x214F57, ('\u{7A98}', false)), // East Asian ideograph
    (0x214F58, ('\u{7A96}', false)), // East Asian ideograph
    (0x214F59, ('\u{7A97}', false)), // East Asian ideograph
    (0x214F5A, ('\u{7A9F}', false)), // East Asian ideograph
    (0x214F5B, ('\u{7AA0}', false)), // East Asian ideograph
    (0x214F5C, ('\u{7AAA}', false)), // East Asian ideograph
    (0x214D3A, ('\u{76DE}', false)), // East Asian ideograph
    (0x214F5E, ('\u{7AAF}', false)), // East Asian ideograph
    (0x214F5F, ('\u{7AAE}', false)), // East Asian ideograph
    (0x274F60, ('\u{7AA5}', false)), // East Asian ideograph
    (0x274F61, ('\u{7A8D}', false)), // East Asian ideograph
    (0x274F62, ('\u{7A9C}', false)), // East Asian ideograph
    (0x274F63, ('\u{7AA6}', false)), // East Asian ideograph
    (0x214F64, ('\u{7ACA}', false)), // East Asian ideograph
    (0x214F65, ('\u{7ACB}', false)), // East Asian ideograph
    (0x214F66, ('\u{7AD9}', false)), // East Asian ideograph
    (0x214F67, ('\u{7AE5}', false)), // East Asian ideograph
    (0x214F68, ('\u{7AE3}', false)), // East Asian ideograph
    (0x214D3C, ('\u{76E1}', false)), // East Asian ideograph
    (0x214F6A, ('\u{7AEF}', false)), // East Asian ideograph
    (0x274F6B, ('\u{7ADE}', false)), // East Asian ideograph
    (0x214F6C, ('\u{7AF9}', false)), // East Asian ideograph
    (0x214F6D, ('\u{7AFA}', false)), // East Asian ideograph
    (0x214F6E, ('\u{7AFF}', false)), // East Asian ideograph
    (0x214F6F, ('\u{7AFD}', false)), // East Asian ideograph
    (0x214F70, ('\u{7B06}', false)), // East Asian ideograph
    (0x214F71, ('\u{7B11}', false)), // East Asian ideograph
    (0x214F72, ('\u{7B20}', false)), // East Asian ideograph
    (0x214F73, ('\u{7B2C}', false)), // East Asian ideograph
    (0x214F74, ('\u{7B28}', false)), // East Asian ideograph
    (0x214D3E, ('\u{76E4}', false)), // East Asian ideograph
    (0x214F76, ('\u{7B1E}', false)), // East Asian ideograph
    (0x214F77, ('\u{7B19}', false)), // East Asian ideograph
    (0x214F78, ('\u{7B26}', false)), // East Asian ideograph
    (0x214F79, ('\u{7B46}', false)), // East Asian ideograph
    (0x214F7A, ('\u{7B49}', false)), // East Asian ideograph
    (0x214D3F, ('\u{76E7}', false)), // East Asian ideograph
    (0x214F7C, ('\u{7B56}', false)), // East Asian ideograph
    (0x214F7D, ('\u{7B52}', false)), // East Asian ideograph
    (0x214F7E, ('\u{7B4B}', false)), // East Asian ideograph
    (0x234D40, ('\u{9784}', false)), // East Asian ideograph
    (0x6F4B5F, ('\u{B0B4}', false)), // Korean hangul
    (0x294472, ('\u{951E}', false)), // East Asian ideograph
    (0x4B4D41, ('\u{862F}', false)), // East Asian ideograph
    (0x6F4D42, ('\u{B3CC}', false)), // Korean hangul
    (0x2E3654, ('\u{657F}', false)), // East Asian ideograph
    (0x2D502B, ('\u{693E}', false)), // East Asian ideograph
    (0x234D43, ('\u{977F}', false)), // East Asian ideograph
    (0x6F5829, ('\u{C9C0}', false)), // Korean hangul
    (0x224D44, ('\u{6F0B}', false)), // East Asian ideograph
    (0x2D4362, ('\u{6722}', false)), // East Asian ideograph
    (0x4B4D45, ('\u{76F4}', false)), // East Asian ideograph (variant of 214D45 which maps to 76F4)
    (0x6F4B60, ('\u{B0B5}', false)), // Korean hangul
    (0x217577, ('\u{57D2}', false)), // East Asian ideograph
    (0x6F4D46, ('\u{B3D7}', false)), // Korean hangul
    (0x213145, ('\u{4F9D}', false)), // East Asian ideograph
    (0x217134, ('\u{5588}', false)), // East Asian ideograph
    (0x6F4D47, ('\u{B3D9}', false)), // Korean hangul
    (0x6F5C6D, ('\u{D571}', false)), // Korean hangul
    (0x27493F, ('\u{6F9C}', false)), // East Asian ideograph
    (0x6F582A, ('\u{C9C1}', false)), // Korean hangul
    (0x276256, ('\u{4E3D}', false)), // East Asian ideograph
    (0x224651, ('\u{6C0C}', false)), // East Asian ideograph
    (0x234D49, ('\u{9789}', false)), // East Asian ideograph
    (0x6F4D4A, ('\u{B400}', false)), // Korean hangul
    (0x6F4B61, ('\u{B0B8}', false)), // Korean hangul
    (0x294474, ('\u{951F}', false)), // East Asian ideograph
    (0x224D4B, ('\u{6F6C}', false)), // East Asian ideograph
    (0x294031, ('\u{909D}', false)), // East Asian ideograph
    (0x333944, ('\u{5B2D}', false)), // East Asian ideograph
    (0x6F4D4C, ('\u{B418}', false)), // Korean hangul
    (0x283F30, ('\u{6966}', false)), // East Asian ideograph
    (0x224D4D, ('\u{6F8B}', false)), // East Asian ideograph
    (0x6F582B, ('\u{C9C4}', false)), // Korean hangul
    (0x6F4D4E, ('\u{B420}', false)), // Korean hangul
    (0x2D4364, ('\u{671E}', false)), // East Asian ideograph
    (0x2D4D4F, ('\u{771F}', false)), // East Asian ideograph
    (0x276327, ('\u{9F89}', false)), // East Asian ideograph
    (0x6F586A, ('\u{CB50}', false)), // Korean hangul
    (0x6F4D50, ('\u{B429}', false)), // Korean hangul
    (0x213147, ('\u{4F75}', false)), // East Asian ideograph
    (0x6F4D51, ('\u{B42B}', false)), // Korean hangul
    (0x212329, ('\u{FF09}', false)), // Ideographic right parenthesis
    (0x6F4D52, ('\u{B42C}', false)), // Korean hangul
    (0x6F582C, ('\u{C9C7}', false)), // Korean hangul
    (0x4B3B67, ('\u{6B67}', false)), // East Asian ideograph
    (0x234D54, ('\u{9794}', false)), // East Asian ideograph
    (0x6F4B63, ('\u{B0BC}', false)), // Korean hangul
    (0x335773, ('\u{88B4}', false)), // East Asian ideograph
    (0x6F4D55, ('\u{B454}', false)), // Korean hangul
    (0x27514A, ('\u{7EF0}', false)), // East Asian ideograph
    (0x214D56, ('\u{773E}', false)), // East Asian ideograph
    (0x6F5C70, ('\u{D578}', false)), // Korean hangul
    (0x276B5B, ('\u{5250}', false)), // East Asian ideograph
    (0x214D57, ('\u{774F}', false)), // East Asian ideograph
    (0x3F5959, ('\u{8276}', false)), // East Asian ideograph
    (0x224D58, ('\u{6E88}', false)), // East Asian ideograph
    (0x35347B, ('\u{8B2D}', false)), // East Asian ideograph
    (0x234D59, ('\u{979B}', false)), // East Asian ideograph
    (0x6F4B64, ('\u{B0C4}', false)), // Korean hangul
    (0x224D5A, ('\u{6F55}', false)), // East Asian ideograph
    (0x213149, ('\u{4F73}', false)), // East Asian ideograph
    (0x21623E, ('\u{9D61}', false)), // East Asian ideograph
    (0x235021, ('\u{9865}', false)), // East Asian ideograph
    (0x235022, ('\u{9866}', false)), // East Asian ideograph
    (0x215023, ('\u{7B54}', false)), // East Asian ideograph
    (0x215024, ('\u{7B60}', false)), // East Asian ideograph
    (0x215025, ('\u{7B77}', false)), // East Asian ideograph
    (0x215026, ('\u{7B75}', false)), // East Asian ideograph
    (0x215027, ('\u{7BA1}', false)), // East Asian ideograph
    (0x215028, ('\u{7B94}', false)), // East Asian ideograph
    (0x235029, ('\u{986C}', false)), // East Asian ideograph
    (0x21502A, ('\u{7B9D}', false)), // East Asian ideograph
    (0x21502B, ('\u{7B8B}', false)), // East Asian ideograph
    (0x21502C, ('\u{7B97}', false)), // East Asian ideograph
    (0x21502D, ('\u{7B8F}', false)), // East Asian ideograph
    (0x21502E, ('\u{7BC7}', false)), // East Asian ideograph
    (0x214D5D, ('\u{775E}', false)), // East Asian ideograph
    (0x235030, ('\u{9873}', false)), // East Asian ideograph
    (0x215031, ('\u{7BB1}', false)), // East Asian ideograph
    (0x215032, ('\u{7BB4}', false)), // East Asian ideograph
    (0x215033, ('\u{7BC0}', false)), // East Asian ideograph
    (0x215034, ('\u{7BC6}', false)), // East Asian ideograph
    (0x215035, ('\u{7BC1}', false)), // East Asian ideograph
    (0x215036, ('\u{7C11}', false)), // East Asian ideograph
    (0x215037, ('\u{7BD9}', false)), // East Asian ideograph
    (0x215038, ('\u{7BDB}', false)), // East Asian ideograph
    (0x235039, ('\u{98AD}', false)), // East Asian ideograph
    (0x21503A, ('\u{7BC9}', false)), // East Asian ideograph
    (0x21503B, ('\u{7BE1}', false)), // East Asian ideograph
    (0x21503C, ('\u{7BE9}', false)), // East Asian ideograph
    (0x21503D, ('\u{7C07}', false)), // East Asian ideograph
    (0x21503E, ('\u{7C0D}', false)), // East Asian ideograph
    (0x21503F, ('\u{7BFE}', false)), // East Asian ideograph
    (0x235040, ('\u{98B4}', false)), // East Asian ideograph
    (0x215041, ('\u{7C21}', false)), // East Asian ideograph
    (0x215042, ('\u{7C2B}', false)), // East Asian ideograph
    (0x215043, ('\u{7C2A}', false)), // East Asian ideograph
    (0x215044, ('\u{7C27}', false)), // East Asian ideograph
    (0x215045, ('\u{7C1E}', false)), // East Asian ideograph
    (0x215046, ('\u{7C23}', false)), // East Asian ideograph
    (0x215047, ('\u{7C3F}', false)), // East Asian ideograph
    (0x215048, ('\u{7C3E}', false)), // East Asian ideograph
    (0x215049, ('\u{7C38}', false)), // East Asian ideograph
    (0x21504A, ('\u{7C37}', false)), // East Asian ideograph
    (0x27504B, ('\u{7B7E}', false)), // East Asian ideograph
    (0x21504C, ('\u{7C43}', false)), // East Asian ideograph
    (0x23504D, ('\u{98BB}', false)), // East Asian ideograph
    (0x23504E, ('\u{98C0}', false)), // East Asian ideograph
    (0x21504F, ('\u{7C50}', false)), // East Asian ideograph
    (0x215050, ('\u{7C60}', false)), // East Asian ideograph
    (0x215051, ('\u{7C5F}', false)), // East Asian ideograph
    (0x215052, ('\u{7C64}', false)), // East Asian ideograph
    (0x215053, ('\u{7C6C}', false)), // East Asian ideograph
    (0x215054, ('\u{7C6E}', false)), // East Asian ideograph
    (0x215055, ('\u{7C72}', false)), // East Asian ideograph
    (0x215056, ('\u{7C73}', false)), // East Asian ideograph
    (0x215057, ('\u{7C89}', false)), // East Asian ideograph
    (0x215058, ('\u{7C92}', false)), // East Asian ideograph
    (0x215059, ('\u{7C97}', false)), // East Asian ideograph
    (0x21505A, ('\u{7C9F}', false)), // East Asian ideograph
    (0x21505B, ('\u{7CA5}', false)), // East Asian ideograph
    (0x21505C, ('\u{7CA4}', false)), // East Asian ideograph
    (0x21505D, ('\u{7CB1}', false)), // East Asian ideograph
    (0x21505E, ('\u{7CB3}', false)), // East Asian ideograph
    (0x23505F, ('\u{98E1}', false)), // East Asian ideograph
    (0x275060, ('\u{7C8B}', false)), // East Asian ideograph
    (0x215061, ('\u{FA1D}', false)), // East Asian ideograph
    (0x275062, ('\u{80E1}', false)), // East Asian ideograph (duplicate simplified)
    (0x215063, ('\u{7CD6}', false)), // East Asian ideograph
    (0x215064, ('\u{7CD5}', false)), // East Asian ideograph
    (0x215065, ('\u{7CE0}', false)), // East Asian ideograph
    (0x215066, ('\u{7CDC}', false)), // East Asian ideograph
    (0x215067, ('\u{7CDF}', false)), // East Asian ideograph
    (0x215068, ('\u{7CDE}', false)), // East Asian ideograph
    (0x215069, ('\u{7CE2}', false)), // East Asian ideograph
    (0x21506A, ('\u{7CD9}', false)), // East Asian ideograph
    (0x21506B, ('\u{7CE7}', false)), // East Asian ideograph
    (0x21506C, ('\u{7CEF}', false)), // East Asian ideograph
    (0x2E506D, ('\u{70B1}', false)), // East Asian ideograph
    (0x21506E, ('\u{7CFB}', false)), // East Asian ideograph
    (0x21506F, ('\u{7CFE}', false)), // East Asian ideograph
    (0x215070, ('\u{7D00}', false)), // East Asian ideograph
    (0x215071, ('\u{7D02}', false)), // East Asian ideograph
    (0x215072, ('\u{7D05}', false)), // East Asian ideograph
    (0x225073, ('\u{70A9}', false)), // East Asian ideograph
    (0x215074, ('\u{7D04}', false)), // East Asian ideograph
    (0x215075, ('\u{7D07}', false)), // East Asian ideograph
    (0x215076, ('\u{7D21}', false)), // East Asian ideograph
    (0x215077, ('\u{7D0B}', false)), // East Asian ideograph
    (0x225078, ('\u{70EA}', false)), // East Asian ideograph
    (0x215079, ('\u{7D20}', false)), // East Asian ideograph
    (0x21507A, ('\u{7D1C}', false)), // East Asian ideograph
    (0x21507B, ('\u{7D22}', false)), // East Asian ideograph
    (0x27507C, ('\u{7EB0}', false)), // East Asian ideograph
    (0x234D6A, ('\u{97AC}', false)), // East Asian ideograph
    (0x21507E, ('\u{7D10}', false)), // East Asian ideograph
    (0x6F5C74, ('\u{D587}', false)), // Korean hangul
    (0x6F4D6B, ('\u{B514}', false)), // Korean hangul
    (0x6F5831, ('\u{C9D3}', false)), // Korean hangul
    (0x6F4D6C, ('\u{B515}', false)), // Korean hangul
    (0x6F7641, ('\u{E8B3}', false)), // Korean hangul
    (0x2D4D6D, ('\u{7792}', false)), // East Asian ideograph
    (0x2D3F27, ('\u{6120}', false)), // East Asian ideograph
    (0x69252D, ('\u{30AD}', false)), // Katakana letter KI
    (0x6F4D6E, ('\u{B51B}', false)), // Korean hangul
    (0x4B4C79, ('\u{7672}', false)), // East Asian ideograph
    (0x6F4D6F, ('\u{B51C}', false)), // Korean hangul
    (0x4C4C35, ('\u{6E0C}', false)), // East Asian ideograph
    (0x6F5C75, ('\u{D588}', false)), // Korean hangul
    (0x234D70, ('\u{97AE}', false)), // East Asian ideograph
    (0x6F5832, ('\u{C9D5}', false)), // Korean hangul
    (0x234D71, ('\u{97A8}', false)), // East Asian ideograph
    (0x334A58, ('\u{89DD}', false)), // East Asian ideograph
    (0x6F4D72, ('\u{B527}', false)), // Korean hangul
    (0x4B537D, ('\u{9ACC}', false)), // East Asian ideograph
    (0x6F592A, ('\u{CC3D}', false)), // Korean hangul
    (0x6F5337, ('\u{C14B}', false)), // Korean hangul
    (0x274D73, ('\u{4E86}', false)), // East Asian ideograph
    (0x224D74, ('\u{6F9F}', false)), // East Asian ideograph
    (0x2D325F, ('\u{50BB}', false)), // East Asian ideograph (variant of 4B325F which maps to 50BB)
    (0x6F4D75, ('\u{B52A}', false)), // Korean hangul
    (0x6F5833, ('\u{C9D6}', false)), // Korean hangul
    (0x29416A, ('\u{948D}', false)), // East Asian ideograph
    (0x22465A, ('\u{6C18}', false)), // East Asian ideograph
    (0x2D3821, ('\u{962F}', false)), // East Asian ideograph
    (0x6F5274, ('\u{C0D9}', false)), // Korean hangul
    (0x213822, ('\u{5747}', false)), // East Asian ideograph
    (0x39447D, ('\u{6AC2}', false)), // East Asian ideograph
    (0x234D78, ('\u{97A5}', false)), // East Asian ideograph
    (0x2D4D21, ('\u{7681}', false)), // East Asian ideograph
    (0x6F4D79, ('\u{B531}', false)), // Korean hangul
    (0x287360, ('\u{7F2C}', false)), // East Asian ideograph
    (0x2F3A5E, ('\u{8E6E}', false)), // East Asian ideograph
    (0x234D7A, ('\u{97B2}', false)), // East Asian ideograph
    (0x2D5836, ('\u{89E6}', false)), // East Asian ideograph
    (0x6F5834, ('\u{C9D9}', false)), // Korean hangul
    (0x22465B, ('\u{6C19}', false)), // East Asian ideograph
    (0x216032, ('\u{7AE0}', false)), // East Asian ideograph
    (0x4B372C, ('\u{5606}', false)), // East Asian ideograph
    (0x234D7C, ('\u{97B4}', false)), // East Asian ideograph
    (0x213827, ('\u{5783}', false)), // East Asian ideograph
    (0x224D7D, ('\u{6FBC}', false)), // East Asian ideograph
    (0x213828, ('\u{576A}', false)), // East Asian ideograph
    (0x235B67, ('\u{9DAA}', false)), // East Asian ideograph
    (0x4B3773, ('\u{56E3}', false)), // East Asian ideograph
    (0x213829, ('\u{5769}', false)), // East Asian ideograph
    (0x34782A, ('\u{90C5}', false)), // East Asian ideograph
    (0x284D49, ('\u{6DA0}', false)), // East Asian ideograph
    (0x213F35, ('\u{6162}', false)), // East Asian ideograph
    (0x21382B, ('\u{5761}', false)), // East Asian ideograph
    (0x4B5946, ('\u{8A33}', false)), // East Asian ideograph
    (0x21382C, ('\u{5764}', false)), // East Asian ideograph
    (0x27383E, ('\u{57A9}', false)), // East Asian ideograph
    (0x6F586C, ('\u{CB59}', false)), // Korean hangul
    (0x6F543D, ('\u{C31C}', false)), // Korean hangul
    (0x4B382E, ('\u{57C0}', false)), // East Asian ideograph
    (0x23382F, ('\u{8DA1}', false)), // East Asian ideograph
    (0x693C32, ('\u{9D2B}', false)), // East Asian ideograph
    (0x215121, ('\u{7D17}', false)), // East Asian ideograph
    (0x215122, ('\u{7D0D}', false)), // East Asian ideograph (variant of 455122 which maps to 7D0D)
    (0x215123, ('\u{7D1A}', false)), // East Asian ideograph
    (0x215124, ('\u{7D19}', false)), // East Asian ideograph
    (0x215125, ('\u{7D1B}', false)), // East Asian ideograph
    (0x215126, ('\u{7D46}', false)), // East Asian ideograph
    (0x213831, ('\u{578B}', false)), // East Asian ideograph
    (0x215128, ('\u{7D3C}', false)), // East Asian ideograph
    (0x215129, ('\u{7D2E}', false)), // East Asian ideograph
    (0x21512A, ('\u{7D39}', false)), // East Asian ideograph
    (0x27512B, ('\u{7EC4}', false)), // East Asian ideograph
    (0x21512C, ('\u{7D30}', false)), // East Asian ideograph
    (0x21512D, ('\u{7D33}', false)), // East Asian ideograph
    (0x21512E, ('\u{7D2F}', false)), // East Asian ideograph
    (0x27512F, ('\u{7ECC}', false)), // East Asian ideograph
    (0x275130, ('\u{7EC8}', false)), // East Asian ideograph
    (0x275131, ('\u{7EDF}', false)), // East Asian ideograph
    (0x275132, ('\u{7EDE}', false)), // East Asian ideograph
    (0x215133, ('\u{7D68}', false)), // East Asian ideograph
    (0x275134, ('\u{7ED3}', false)), // East Asian ideograph
    (0x215135, ('\u{7D2B}', false)), // East Asian ideograph
    (0x215136, ('\u{7D62}', false)), // East Asian ideograph
    (0x215137, ('\u{7D76}', false)), // East Asian ideograph
    (0x215138, ('\u{7D61}', false)), // East Asian ideograph
    (0x275139, ('\u{7ED9}', false)), // East Asian ideograph
    (0x21513A, ('\u{7D6E}', false)), // East Asian ideograph
    (0x21513B, ('\u{7D72}', false)), // East Asian ideograph
    (0x27513C, ('\u{7ECF}', false)), // East Asian ideograph
    (0x21513D, ('\u{7D91}', false)), // East Asian ideograph
    (0x21513E, ('\u{7D79}', false)), // East Asian ideograph
    (0x21513F, ('\u{7D8F}', false)), // East Asian ideograph
    (0x275140, ('\u{7ED1}', false)), // East Asian ideograph
    (0x275141, ('\u{7EFC}', false)), // East Asian ideograph
    (0x225142, ('\u{70F7}', false)), // East Asian ideograph
    (0x215143, ('\u{7DB0}', false)), // East Asian ideograph
    (0x275144, ('\u{7D27}', false)), // East Asian ideograph
    (0x275145, ('\u{7EEB}', false)), // East Asian ideograph
    (0x275146, ('\u{7F00}', false)), // East Asian ideograph
    (0x215147, ('\u{7DBA}', false)), // East Asian ideograph
    (0x275148, ('\u{7F51}', false)), // East Asian ideograph
    (0x275149, ('\u{7EB2}', false)), // East Asian ideograph
    (0x22514A, ('\u{7110}', false)), // East Asian ideograph
    (0x21514B, ('\u{7DB5}', false)), // East Asian ideograph
    (0x21514C, ('\u{7DA0}', false)), // East Asian ideograph
    (0x27514D, ('\u{7EF8}', false)), // East Asian ideograph
    (0x27514E, ('\u{7EF4}', false)), // East Asian ideograph
    (0x27514F, ('\u{7EF5}', false)), // East Asian ideograph
    (0x275150, ('\u{7EB6}', false)), // East Asian ideograph
    (0x275151, ('\u{7F01}', false)), // East Asian ideograph
    (0x215152, ('\u{7DE0}', false)), // East Asian ideograph
    (0x235153, ('\u{9933}', false)), // East Asian ideograph
    (0x235154, ('\u{9942}', false)), // East Asian ideograph (variant of 4D5154 which maps to 9942)
    (0x275155, ('\u{7EEA}', false)), // East Asian ideograph
    (0x215156, ('\u{7DD8}', false)), // East Asian ideograph
    (0x275157, ('\u{7F05}', false)), // East Asian ideograph
    (0x275158, ('\u{7F09}', false)), // East Asian ideograph
    (0x275159, ('\u{7F13}', false)), // East Asian ideograph
    (0x27515A, ('\u{7F18}', false)), // East Asian ideograph
    (0x21515B, ('\u{7DE8}', false)), // East Asian ideograph
    (0x21515C, ('\u{7DDA}', false)), // East Asian ideograph
    (0x27515D, ('\u{7F0D}', false)), // East Asian ideograph
    (0x27515E, ('\u{7F0E}', false)), // East Asian ideograph
    (0x27515F, ('\u{7F23}', false)), // East Asian ideograph
    (0x275160, ('\u{7F22}', false)), // East Asian ideograph
    (0x275161, ('\u{8426}', false)), // East Asian ideograph
    (0x275162, ('\u{7F1A}', false)), // East Asian ideograph
    (0x215163, ('\u{7DFB}', false)), // East Asian ideograph
    (0x275164, ('\u{53BF}', false)), // East Asian ideograph (variant of 455164 which maps to 53BF)
    (0x215165, ('\u{7E2E}', false)), // East Asian ideograph
    (0x215166, ('\u{7E3E}', false)), // East Asian ideograph
    (0x275167, ('\u{7F2A}', false)), // East Asian ideograph
    (0x275168, ('\u{7F15}', false)), // East Asian ideograph
    (0x215169, ('\u{7E32}', false)), // East Asian ideograph
    (0x23516A, ('\u{9948}', false)), // East Asian ideograph
    (0x21516B, ('\u{7E41}', false)), // East Asian ideograph
    (0x23516C, ('\u{9947}', false)), // East Asian ideograph
    (0x23516D, ('\u{9949}', false)), // East Asian ideograph
    (0x21516E, ('\u{7E31}', false)), // East Asian ideograph
    (0x22516F, ('\u{713F}', false)), // East Asian ideograph
    (0x235170, ('\u{9943}', false)), // East Asian ideograph
    (0x275171, ('\u{7EC7}', false)), // East Asian ideograph
    (0x215172, ('\u{7E61}', false)), // East Asian ideograph
    (0x235173, ('\u{994E}', false)), // East Asian ideograph
    (0x235174, ('\u{9950}', false)), // East Asian ideograph
    (0x215175, ('\u{7E6B}', false)), // East Asian ideograph
    (0x215176, ('\u{7E69}', false)), // East Asian ideograph
    (0x215177, ('\u{7E6D}', false)), // East Asian ideograph
    (0x215178, ('\u{7E79}', false)), // East Asian ideograph
    (0x215179, ('\u{7E6A}', false)), // East Asian ideograph
    (0x21517A, ('\u{8FAE}', false)), // East Asian ideograph
    (0x27517B, ('\u{7F24}', false)), // East Asian ideograph
    (0x21517C, ('\u{7E82}', false)), // East Asian ideograph
    (0x21517D, ('\u{7E7C}', false)), // East Asian ideograph
    (0x21517E, ('\u{7E8F}', false)), // East Asian ideograph
    (0x6F5947, ('\u{CCE4}', false)), // Korean hangul
    (0x227841, ('\u{80EF}', false)), // East Asian ideograph
    (0x217144, ('\u{5581}', false)), // East Asian ideograph
    (0x4B3774, ('\u{56F3}', false)), // East Asian ideograph
    (0x235729, ('\u{9B8E}', false)), // East Asian ideograph
    (0x287321, ('\u{7F26}', false)), // East Asian ideograph
    (0x6F5C7D, ('\u{D5D2}', false)), // Korean hangul
    (0x6F583A, ('\u{C9E0}', false)), // Korean hangul
    (0x216038, ('\u{9802}', false)), // East Asian ideograph
    (0x213844, ('\u{5831}', false)), // East Asian ideograph
    (0x4B594B, ('\u{5909}', false)), // East Asian ideograph
    (0x6F5D72, ('\u{D763}', false)), // Korean hangul
    (0x4C3B31, ('\u{6798}', false)), // East Asian ideograph
    (0x213845, ('\u{582F}', false)), // East Asian ideograph
    (0x213A30, ('\u{5ABC}', false)), // East Asian ideograph
    (0x6F5C7E, ('\u{D5D8}', false)), // Korean hangul
    (0x213848, ('\u{5830}', false)), // East Asian ideograph
    (0x274638, ('\u{6B7C}', false)), // East Asian ideograph
    (0x213849, ('\u{5824}', false)), // East Asian ideograph
    (0x21384A, ('\u{5834}', false)), // East Asian ideograph
    (0x21784B, ('\u{58C6}', false)), // East Asian ideograph
    (0x394042, ('\u{646D}', false)), // East Asian ideograph
    (0x284B28, ('\u{6D48}', false)), // East Asian ideograph
    (0x22384C, ('\u{665E}', false)), // East Asian ideograph
    (0x69525D, ('\u{53FA}', false)), // East Asian ideograph
    (0x705D46, ('\u{841C}', false)), // East Asian ideograph
    (0x27384D, ('\u{6D82}', false)), // East Asian ideograph
    (0x21603A, ('\u{9805}', false)), // East Asian ideograph
    (0x234A62, ('\u{967E}', false)), // East Asian ideograph
    (0x213158, ('\u{4FD1}', false)), // East Asian ideograph
    (0x223850, ('\u{667E}', false)), // East Asian ideograph
    (0x21387C, ('\u{5919}', false)), // East Asian ideograph
    (0x213851, ('\u{584C}', false)), // East Asian ideograph
    (0x213852, ('\u{585A}', false)), // East Asian ideograph
    (0x213853, ('\u{586D}', false)), // East Asian ideograph
    (0x6F5276, ('\u{C0DC}', false)), // Korean hangul
    (0x217854, ('\u{58D2}', false)), // East Asian ideograph
    (0x213855, ('\u{5862}', false)), // East Asian ideograph
    (0x335B70, ('\u{5EF8}', false)), // East Asian ideograph
    (0x213F4E, ('\u{61C7}', false)), // East Asian ideograph
    (0x273856, ('\u{5757}', false)), // East Asian ideograph
    (0x6F4E33, ('\u{B5A8}', false)), // Korean hangul
    (0x6F583E, ('\u{C9E7}', false)), // Korean hangul
    (0x22687E, ('\u{7A2D}', false)), // East Asian ideograph
    (0x4B3B79, ('\u{5D8C}', false)), // East Asian ideograph
    (0x217428, ('\u{5704}', false)), // East Asian ideograph
    (0x334621, ('\u{8B99}', false)), // East Asian ideograph
    (0x233859, ('\u{8DAF}', false)), // East Asian ideograph
    (0x21385A, ('\u{588A}', false)), // East Asian ideograph
    (0x215221, ('\u{7E8C}', false)), // East Asian ideograph
    (0x275222, ('\u{7F28}', false)), // East Asian ideograph
    (0x215223, ('\u{7E96}', false)), // East Asian ideograph
    (0x215224, ('\u{7E9C}', false)), // East Asian ideograph
    (0x6F5225, ('\u{BE75}', false)), // Korean hangul
    (0x215226, ('\u{7F38}', false)), // East Asian ideograph
    (0x215227, ('\u{7F3A}', false)), // East Asian ideograph
    (0x225228, ('\u{7135}', false)), // East Asian ideograph
    (0x235229, ('\u{995D}', false)), // East Asian ideograph
    (0x6F522A, ('\u{BE84}', false)), // Korean hangul
    (0x21522B, ('\u{7F50}', false)), // East Asian ideograph
    (0x21522C, ('\u{7F55}', false)), // East Asian ideograph
    (0x21522D, ('\u{7F54}', false)), // East Asian ideograph
    (0x21522E, ('\u{7F5F}', false)), // East Asian ideograph
    (0x21522F, ('\u{7F72}', false)), // East Asian ideograph
    (0x215230, ('\u{7F6E}', false)), // East Asian ideograph
    (0x224223, ('\u{6A89}', false)), // East Asian ideograph
    (0x215232, ('\u{7F6A}', false)), // East Asian ideograph
    (0x215233, ('\u{7F70}', false)), // East Asian ideograph
    (0x215234, ('\u{7F75}', false)), // East Asian ideograph
    (0x215235, ('\u{7F77}', false)), // East Asian ideograph
    (0x215236, ('\u{7F79}', false)), // East Asian ideograph
    (0x215237, ('\u{7F85}', false)), // East Asian ideograph
    (0x275238, ('\u{7F81}', false)), // East Asian ideograph
    (0x215239, ('\u{7F8A}', false)), // East Asian ideograph
    (0x21523A, ('\u{7F8C}', false)), // East Asian ideograph
    (0x21523B, ('\u{7F8E}', false)), // East Asian ideograph
    (0x21523C, ('\u{7F94}', false)), // East Asian ideograph
    (0x21523D, ('\u{7F9E}', false)), // East Asian ideograph
    (0x21523E, ('\u{7F9A}', false)), // East Asian ideograph
    (0x21523F, ('\u{5584}', false)), // East Asian ideograph
    (0x215240, ('\u{7FA8}', false)), // East Asian ideograph
    (0x215241, ('\u{7FA4}', false)), // East Asian ideograph
    (0x235242, ('\u{99AA}', false)), // East Asian ideograph
    (0x215243, ('\u{7FAF}', false)), // East Asian ideograph
    (0x215244, ('\u{7FB2}', false)), // East Asian ideograph
    (0x215245, ('\u{7FB6}', false)), // East Asian ideograph
    (0x215246, ('\u{7FB8}', false)), // East Asian ideograph
    (0x215247, ('\u{7FB9}', false)), // East Asian ideograph
    (0x215248, ('\u{7FBD}', false)), // East Asian ideograph
    (0x235249, ('\u{99B5}', false)), // East Asian ideograph
    (0x21524A, ('\u{7FC5}', false)), // East Asian ideograph
    (0x21524B, ('\u{7FC1}', false)), // East Asian ideograph
    (0x21524C, ('\u{7FCC}', false)), // East Asian ideograph
    (0x21524D, ('\u{7FD2}', false)), // East Asian ideograph
    (0x21524E, ('\u{7FCE}', false)), // East Asian ideograph (variant of 4B524E which maps to 7FCE)
    (0x21524F, ('\u{7FD4}', false)), // East Asian ideograph
    (0x215250, ('\u{7FD5}', false)), // East Asian ideograph
    (0x215251, ('\u{7FE0}', false)), // East Asian ideograph
    (0x215252, ('\u{7FE1}', false)), // East Asian ideograph
    (0x215253, ('\u{7FDF}', false)), // East Asian ideograph
    (0x235254, ('\u{99BD}', false)), // East Asian ideograph
    (0x215255, ('\u{7FF0}', false)), // East Asian ideograph
    (0x215256, ('\u{7FF3}', false)), // East Asian ideograph
    (0x215257, ('\u{7FFC}', false)), // East Asian ideograph
    (0x215258, ('\u{7FF9}', false)), // East Asian ideograph
    (0x215259, ('\u{7FFB}', false)), // East Asian ideograph
    (0x21525A, ('\u{7FF1}', false)), // East Asian ideograph
    (0x21525B, ('\u{8000}', false)), // East Asian ideograph
    (0x22525C, ('\u{7143}', false)), // East Asian ideograph
    (0x21525D, ('\u{8003}', false)), // East Asian ideograph
    (0x21525E, ('\u{8006}', false)), // East Asian ideograph
    (0x21525F, ('\u{8005}', false)), // East Asian ideograph
    (0x215260, ('\u{800C}', false)), // East Asian ideograph
    (0x215261, ('\u{8010}', false)), // East Asian ideograph
    (0x215262, ('\u{800D}', false)), // East Asian ideograph
    (0x215263, ('\u{8012}', false)), // East Asian ideograph
    (0x215264, ('\u{8015}', false)), // East Asian ideograph
    (0x215265, ('\u{8018}', false)), // East Asian ideograph
    (0x215266, ('\u{8019}', false)), // East Asian ideograph
    (0x215267, ('\u{8017}', false)), // East Asian ideograph
    (0x215268, ('\u{801C}', false)), // East Asian ideograph
    (0x235269, ('\u{99D8}', false)), // East Asian ideograph
    (0x21526A, ('\u{8036}', false)), // East Asian ideograph
    (0x21526B, ('\u{803F}', false)), // East Asian ideograph
    (0x21526C, ('\u{803D}', false)), // East Asian ideograph
    (0x21526D, ('\u{804A}', false)), // East Asian ideograph
    (0x21526E, ('\u{8046}', false)), // East Asian ideograph
    (0x21526F, ('\u{8056}', false)), // East Asian ideograph
    (0x215270, ('\u{8058}', false)), // East Asian ideograph
    (0x215271, ('\u{805E}', false)), // East Asian ideograph
    (0x215272, ('\u{805A}', false)), // East Asian ideograph
    (0x215273, ('\u{8071}', false)), // East Asian ideograph
    (0x215274, ('\u{8072}', false)), // East Asian ideograph
    (0x215275, ('\u{8073}', false)), // East Asian ideograph
    (0x215276, ('\u{8070}', false)), // East Asian ideograph
    (0x215277, ('\u{806F}', false)), // East Asian ideograph
    (0x215278, ('\u{8077}', false)), // East Asian ideograph
    (0x275279, ('\u{8042}', false)), // East Asian ideograph
    (0x23527A, ('\u{99F0}', false)), // East Asian ideograph
    (0x27527B, ('\u{542C}', false)), // East Asian ideograph
    (0x21527C, ('\u{807F}', false)), // East Asian ideograph
    (0x6F527C, ('\u{C0F4}', false)), // Korean hangul
    (0x21527E, ('\u{8084}', false)), // East Asian ideograph
    (0x21386B, ('\u{58D9}', false)), // East Asian ideograph
    (0x6F5842, ('\u{C9F0}', false)), // Korean hangul
    (0x27386C, ('\u{5792}', false)), // East Asian ideograph
    (0x6F5A23, ('\u{CEAD}', false)), // Korean hangul
    (0x21386D, ('\u{58DF}', false)), // East Asian ideograph
    (0x27386E, ('\u{574F}', false)), // East Asian ideograph
    (0x4B3850, ('\u{5861}', false)), // East Asian ideograph (variant of 213850)
    (0x23395C, ('\u{8E1E}', false)), // East Asian ideograph
    (0x235732, ('\u{9B98}', false)), // East Asian ideograph
    (0x6F4E34, ('\u{B5AB}', false)), // Korean hangul
    (0x213870, ('\u{58E4}', false)), // East Asian ideograph
    (0x276065, ('\u{9965}', false)), // East Asian ideograph
    (0x4B3B7E, ('\u{5D15}', false)), // East Asian ideograph
    (0x273871, ('\u{575D}', false)), // East Asian ideograph
    (0x6F5D76, ('\u{D770}', false)), // Korean hangul
    (0x223872, ('\u{6693}', false)), // East Asian ideograph
    (0x233873, ('\u{8DBF}', false)), // East Asian ideograph
    (0x273874, ('\u{58EE}', false)), // East Asian ideograph
    (0x343875, ('\u{5FDE}', false)), // East Asian ideograph
    (0x284D58, ('\u{6CA9}', false)), // East Asian ideograph
    (0x705C43, ('\u{82CA}', false)), // East Asian ideograph
    (0x223876, ('\u{6690}', false)), // East Asian ideograph
    (0x276321, ('\u{9F7F}', false)), // East Asian ideograph
    (0x213877, ('\u{58FD}', false)), // East Asian ideograph
    (0x276322, ('\u{9F83}', false)), // East Asian ideograph
    (0x4D5D49, ('\u{9E81}', false)), // East Asian ideograph
    (0x226323, ('\u{77B6}', false)), // East Asian ideograph
    (0x213879, ('\u{5914}', false)), // East Asian ideograph
    (0x276324, ('\u{9F84}', false)), // East Asian ideograph
    (0x2E765F, ('\u{8037}', false)), // East Asian ideograph
    (0x21387A, ('\u{5915}', false)), // East Asian ideograph
    (0x284D59, ('\u{6ED7}', false)), // East Asian ideograph
    (0x276325, ('\u{9F88}', false)), // East Asian ideograph
    (0x6F542C, ('\u{C2F1}', false)), // Korean hangul
    (0x213E2A, ('\u{6035}', false)), // East Asian ideograph (variant of 4B3E2A which maps to 6035)
    (0x6F5675, ('\u{C78A}', false)), // Korean hangul
    (0x276326, ('\u{9F87}', false)), // East Asian ideograph
    (0x22787C, ('\u{8153}', false)), // East Asian ideograph
    (0x216327, ('\u{9F6C}', false)), // East Asian ideograph
    (0x21387D, ('\u{591A}', false)), // East Asian ideograph
    (0x276328, ('\u{9F8A}', false)), // East Asian ideograph
    (0x2D517D, ('\u{7D99}', false)), // East Asian ideograph
    (0x4B484A, ('\u{6E13}', false)), // East Asian ideograph
    (0x2D3272, ('\u{706E}', false)), // East Asian ideograph
    (0x232329, ('\u{845C}', false)), // East Asian ideograph
    (0x6F5846, ('\u{C9FC}', false)), // Korean hangul
    (0x27632A, ('\u{9F8B}', false)), // East Asian ideograph
    (0x234A6C, ('\u{9689}', false)), // East Asian ideograph
    (0x22632B, ('\u{77B9}', false)), // East Asian ideograph
    (0x453051, ('\u{8D30}', false)), // East Asian ideograph
    (0x6F4B7D, ('\u{B124}', false)), // Korean hangul
    (0x21632C, ('\u{9F94}', false)), // East Asian ideograph
    (0x27632D, ('\u{9F9F}', false)), // East Asian ideograph
    (0x4B484B, ('\u{51D6}', false)), // East Asian ideograph
    (0x235736, ('\u{9B9F}', false)), // East Asian ideograph
    (0x6F5847, ('\u{CA00}', false)), // Korean hangul
    (0x6F5427, ('\u{C2E4}', false)), // Korean hangul
    (0x275321, ('\u{8083}', false)), // East Asian ideograph
    (0x215322, ('\u{8087}', false)), // East Asian ideograph
    (0x215323, ('\u{8089}', false)), // East Asian ideograph
    (0x235324, ('\u{9A02}', false)), // East Asian ideograph
    (0x215325, ('\u{808C}', false)), // East Asian ideograph
    (0x215326, ('\u{8093}', false)), // East Asian ideograph
    (0x215327, ('\u{809D}', false)), // East Asian ideograph
    (0x215328, ('\u{8098}', false)), // East Asian ideograph
    (0x215329, ('\u{809B}', false)), // East Asian ideograph
    (0x21532A, ('\u{809A}', false)), // East Asian ideograph
    (0x22532B, ('\u{7180}', false)), // East Asian ideograph
    (0x22532C, ('\u{7189}', false)), // East Asian ideograph
    (0x21532D, ('\u{80AA}', false)), // East Asian ideograph
    (0x21532E, ('\u{80BA}', false)), // East Asian ideograph
    (0x21532F, ('\u{80A5}', false)), // East Asian ideograph
    (0x235330, ('\u{99FB}', false)), // East Asian ideograph
    (0x235331, ('\u{99FD}', false)), // East Asian ideograph
    (0x215332, ('\u{80B1}', false)), // East Asian ideograph
    (0x225333, ('\u{7196}', false)), // East Asian ideograph
    (0x215334, ('\u{80A1}', false)), // East Asian ideograph
    (0x215335, ('\u{80A9}', false)), // East Asian ideograph
    (0x27495D, ('\u{4E4C}', false)), // East Asian ideograph
    (0x215337, ('\u{80D6}', false)), // East Asian ideograph
    (0x215338, ('\u{80CC}', false)), // East Asian ideograph
    (0x215339, ('\u{80E5}', false)), // East Asian ideograph
    (0x21533A, ('\u{80DA}', false)), // East Asian ideograph
    (0x21533B, ('\u{80E1}', false)), // East Asian ideograph
    (0x21533C, ('\u{80C3}', false)), // East Asian ideograph
    (0x21533D, ('\u{80DB}', false)), // East Asian ideograph
    (0x21533E, ('\u{80C4}', false)), // East Asian ideograph
    (0x21533F, ('\u{80CE}', false)), // East Asian ideograph
    (0x215340, ('\u{80DE}', false)), // East Asian ideograph
    (0x215341, ('\u{80E4}', false)), // East Asian ideograph
    (0x215342, ('\u{80F0}', false)), // East Asian ideograph
    (0x215343, ('\u{8102}', false)), // East Asian ideograph
    (0x215344, ('\u{8105}', false)), // East Asian ideograph
    (0x215345, ('\u{80F1}', false)), // East Asian ideograph
    (0x215346, ('\u{80F4}', false)), // East Asian ideograph
    (0x215347, ('\u{80ED}', false)), // East Asian ideograph
    (0x235348, ('\u{9A10}', false)), // East Asian ideograph
    (0x215349, ('\u{8106}', false)), // East Asian ideograph
    (0x21534A, ('\u{80F3}', false)), // East Asian ideograph
    (0x21534B, ('\u{80F8}', false)), // East Asian ideograph
    (0x23534C, ('\u{9A24}', false)), // East Asian ideograph
    (0x21534D, ('\u{8108}', false)), // East Asian ideograph
    (0x21534E, ('\u{812B}', false)), // East Asian ideograph
    (0x21534F, ('\u{812F}', false)), // East Asian ideograph
    (0x215350, ('\u{8116}', false)), // East Asian ideograph
    (0x225351, ('\u{71A4}', false)), // East Asian ideograph
    (0x215352, ('\u{8129}', false)), // East Asian ideograph
    (0x215353, ('\u{8155}', false)), // East Asian ideograph
    (0x215354, ('\u{8154}', false)), // East Asian ideograph
    (0x215355, ('\u{814B}', false)), // East Asian ideograph
    (0x215356, ('\u{8151}', false)), // East Asian ideograph
    (0x215357, ('\u{8150}', false)), // East Asian ideograph
    (0x215358, ('\u{814E}', false)), // East Asian ideograph
    (0x275359, ('\u{80C0}', false)), // East Asian ideograph
    (0x21535A, ('\u{8146}', false)), // East Asian ideograph
    (0x21535B, ('\u{813E}', false)), // East Asian ideograph
    (0x21535C, ('\u{8171}', false)), // East Asian ideograph
    (0x21535D, ('\u{8170}', false)), // East Asian ideograph
    (0x21535E, ('\u{8178}', false)), // East Asian ideograph
    (0x21535F, ('\u{8165}', false)), // East Asian ideograph
    (0x215360, ('\u{816E}', false)), // East Asian ideograph
    (0x215361, ('\u{8173}', false)), // East Asian ideograph
    (0x275362, ('\u{80BF}', false)), // East Asian ideograph
    (0x215363, ('\u{8179}', false)), // East Asian ideograph
    (0x215364, ('\u{817A}', false)), // East Asian ideograph
    (0x215365, ('\u{8166}', false)), // East Asian ideograph
    (0x215366, ('\u{8180}', false)), // East Asian ideograph
    (0x225367, ('\u{71D1}', false)), // East Asian ideograph
    (0x215368, ('\u{817F}', false)), // East Asian ideograph
    (0x215369, ('\u{818A}', false)), // East Asian ideograph
    (0x21536A, ('\u{8188}', false)), // East Asian ideograph
    (0x21536B, ('\u{819D}', false)), // East Asian ideograph
    (0x21536C, ('\u{81A0}', false)), // East Asian ideograph
    (0x22536D, ('\u{71CA}', false)), // East Asian ideograph
    (0x21536E, ('\u{819A}', false)), // East Asian ideograph
    (0x21536F, ('\u{819C}', false)), // East Asian ideograph
    (0x215370, ('\u{81B3}', false)), // East Asian ideograph
    (0x275371, ('\u{817B}', false)), // East Asian ideograph
    (0x215372, ('\u{81A8}', false)), // East Asian ideograph
    (0x215373, ('\u{81C6}', false)), // East Asian ideograph
    (0x215374, ('\u{81BA}', false)), // East Asian ideograph
    (0x215375, ('\u{81C3}', false)), // East Asian ideograph
    (0x215376, ('\u{81C0}', false)), // East Asian ideograph
    (0x215377, ('\u{81C2}', false)), // East Asian ideograph
    (0x275378, ('\u{8113}', false)), // East Asian ideograph
    (0x275379, ('\u{80C6}', false)), // East Asian ideograph
    (0x27537A, ('\u{8138}', false)), // East Asian ideograph
    (0x27537B, ('\u{810D}', false)), // East Asian ideograph
    (0x21537C, ('\u{81CD}', false)), // East Asian ideograph
    (0x27537D, ('\u{8191}', false)), // East Asian ideograph
    (0x27537E, ('\u{814A}', false)), // East Asian ideograph
    (0x234156, ('\u{91BF}', false)), // East Asian ideograph
    (0x276B79, ('\u{523F}', false)), // East Asian ideograph
    (0x6F584B, ('\u{CA0C}', false)), // Korean hangul
    (0x2D3B3F, ('\u{5C02}', false)), // East Asian ideograph
    (0x21313B, ('\u{4F43}', false)), // East Asian ideograph
    (0x2D615A, ('\u{8EC6}', false)), // East Asian ideograph
    (0x6F2526, ('\u{3161}', false)), // Korean hangul
    (0x276B7A, ('\u{523D}', false)), // East Asian ideograph
    (0x6F584C, ('\u{CA0D}', false)), // Korean hangul
    (0x69543A, ('\u{57AA}', false)), // East Asian ideograph
    (0x232349, ('\u{8497}', false)), // East Asian ideograph
    (0x6F5279, ('\u{C0E5}', false)), // Korean hangul
    (0x274C33, ('\u{6BD5}', false)), // East Asian ideograph
    (0x213168, ('\u{4FC4}', false)), // East Asian ideograph
    (0x22234B, ('\u{5C8D}', false)), // East Asian ideograph
    (0x213F51, ('\u{61E3}', false)), // East Asian ideograph
    (0x2D3279, ('\u{514E}', false)), // East Asian ideograph
    (0x6F4E36, ('\u{B5B1}', false)), // Korean hangul
    (0x6F2471, ('\u{3149}', false)), // Korean hangul
    (0x6F584D, ('\u{CA18}', false)), // Korean hangul
    (0x224674, ('\u{6C3F}', false)), // East Asian ideograph
    (0x6F4929, ('\u{AC80}', false)), // Korean hangul
    (0x6F5164, ('\u{BDD4}', false)), // Korean hangul
    (0x217E21, ('\u{5B5B}', false)), // East Asian ideograph
    (0x213169, ('\u{4FC2}', false)), // East Asian ideograph
    (0x217158, ('\u{55CC}', false)), // East Asian ideograph
    (0x233967, ('\u{8E27}', false)), // East Asian ideograph
    (0x4B594A, ('\u{8AAD}', false)), // East Asian ideograph
    (0x6F5158, ('\u{BD80}', false)), // Korean hangul
    (0x6F584E, ('\u{CA4C}', false)), // Korean hangul
    (0x6F7721, ('\u{AD35}', false)), // Korean hangul
    (0x213E33, ('\u{6025}', false)), // East Asian ideograph
    (0x295A28, ('\u{9E28}', false)), // East Asian ideograph
    (0x2D4B72, ('\u{7506}', false)), // East Asian ideograph
    (0x6F584F, ('\u{CA4D}', false)), // Korean hangul
    (0x232358, ('\u{84B9}', false)), // East Asian ideograph
    (0x347431, ('\u{58DC}', false)), // East Asian ideograph
    (0x21715A, ('\u{55DB}', false)), // East Asian ideograph
    (0x233969, ('\u{8E18}', false)), // East Asian ideograph
    (0x215421, ('\u{81DA}', false)), // East Asian ideograph
    (0x235422, ('\u{9A4D}', false)), // East Asian ideograph
    (0x215423, ('\u{81E3}', false)), // East Asian ideograph
    (0x235424, ('\u{9A52}', false)), // East Asian ideograph
    (0x275425, ('\u{4E34}', false)), // East Asian ideograph
    (0x215426, ('\u{81EA}', false)), // East Asian ideograph
    (0x215427, ('\u{81EC}', false)), // East Asian ideograph
    (0x215428, ('\u{81ED}', false)), // East Asian ideograph
    (0x215429, ('\u{81F3}', false)), // East Asian ideograph
    (0x22542A, ('\u{71DE}', false)), // East Asian ideograph
    (0x21542B, ('\u{81FA}', false)), // East Asian ideograph
    (0x21542C, ('\u{81FB}', false)), // East Asian ideograph
    (0x21542D, ('\u{81FC}', false)), // East Asian ideograph
    (0x21542E, ('\u{81FE}', false)), // East Asian ideograph
    (0x21542F, ('\u{8200}', false)), // East Asian ideograph
    (0x215430, ('\u{8202}', false)), // East Asian ideograph
    (0x215431, ('\u{8205}', false)), // East Asian ideograph
    (0x215432, ('\u{8207}', false)), // East Asian ideograph
    (0x275433, ('\u{5174}', false)), // East Asian ideograph
    (0x275434, ('\u{4E3E}', false)), // East Asian ideograph
    (0x215435, ('\u{820A}', false)), // East Asian ideograph
    (0x215436, ('\u{820C}', false)), // East Asian ideograph
    (0x215437, ('\u{820D}', false)), // East Asian ideograph
    (0x215438, ('\u{8210}', false)), // East Asian ideograph
    (0x215439, ('\u{8212}', false)), // East Asian ideograph
    (0x23543A, ('\u{9A6B}', false)), // East Asian ideograph
    (0x21543B, ('\u{821B}', false)), // East Asian ideograph
    (0x21543C, ('\u{821C}', false)), // East Asian ideograph
    (0x21543D, ('\u{821E}', false)), // East Asian ideograph
    (0x21543E, ('\u{821F}', false)), // East Asian ideograph
    (0x21543F, ('\u{8222}', false)), // East Asian ideograph
    (0x215440, ('\u{822A}', false)), // East Asian ideograph
    (0x235441, ('\u{9AAB}', false)), // East Asian ideograph
    (0x215442, ('\u{822C}', false)), // East Asian ideograph
    (0x215443, ('\u{8228}', false)), // East Asian ideograph
    (0x215444, ('\u{8237}', false)), // East Asian ideograph
    (0x215445, ('\u{8235}', false)), // East Asian ideograph
    (0x215446, ('\u{8239}', false)), // East Asian ideograph
    (0x215447, ('\u{8236}', false)), // East Asian ideograph
    (0x215448, ('\u{8247}', false)), // East Asian ideograph
    (0x215449, ('\u{8258}', false)), // East Asian ideograph
    (0x21544A, ('\u{8259}', false)), // East Asian ideograph
    (0x21544B, ('\u{8266}', false)), // East Asian ideograph
    (0x21544C, ('\u{826E}', false)), // East Asian ideograph
    (0x21544D, ('\u{826F}', false)), // East Asian ideograph
    (0x21544E, ('\u{8271}', false)), // East Asian ideograph
    (0x21544F, ('\u{8272}', false)), // East Asian ideograph
    (0x215450, ('\u{827E}', false)), // East Asian ideograph
    (0x215451, ('\u{8292}', false)), // East Asian ideograph
    (0x215452, ('\u{828B}', false)), // East Asian ideograph
    (0x215453, ('\u{828D}', false)), // East Asian ideograph
    (0x215454, ('\u{82B3}', false)), // East Asian ideograph
    (0x215455, ('\u{829D}', false)), // East Asian ideograph
    (0x215456, ('\u{8299}', false)), // East Asian ideograph
    (0x215457, ('\u{82BD}', false)), // East Asian ideograph
    (0x215458, ('\u{82AD}', false)), // East Asian ideograph
    (0x215459, ('\u{82AC}', false)), // East Asian ideograph
    (0x21545A, ('\u{82A5}', false)), // East Asian ideograph
    (0x21545B, ('\u{829F}', false)), // East Asian ideograph
    (0x27545C, ('\u{520D}', false)), // East Asian ideograph
    (0x21545D, ('\u{82B1}', false)), // East Asian ideograph
    (0x21545E, ('\u{82B9}', false)), // East Asian ideograph
    (0x69545F, ('\u{58E5}', false)), // East Asian ideograph
    (0x215460, ('\u{82E7}', false)), // East Asian ideograph
    (0x215461, ('\u{8305}', false)), // East Asian ideograph
    (0x215462, ('\u{8309}', false)), // East Asian ideograph
    (0x215463, ('\u{82E3}', false)), // East Asian ideograph
    (0x215464, ('\u{82DB}', false)), // East Asian ideograph
    (0x215465, ('\u{82E6}', false)), // East Asian ideograph
    (0x215466, ('\u{8304}', false)), // East Asian ideograph
    (0x215467, ('\u{82E5}', false)), // East Asian ideograph
    (0x215468, ('\u{8302}', false)), // East Asian ideograph
    (0x215469, ('\u{82DC}', false)), // East Asian ideograph
    (0x21546A, ('\u{82D7}', false)), // East Asian ideograph
    (0x21546B, ('\u{82F1}', false)), // East Asian ideograph
    (0x21546C, ('\u{8301}', false)), // East Asian ideograph
    (0x23546D, ('\u{9AD6}', false)), // East Asian ideograph
    (0x21546E, ('\u{82D4}', false)), // East Asian ideograph
    (0x21546F, ('\u{82D1}', false)), // East Asian ideograph
    (0x215470, ('\u{82DE}', false)), // East Asian ideograph
    (0x215471, ('\u{82DF}', false)), // East Asian ideograph
    (0x215472, ('\u{832B}', false)), // East Asian ideograph
    (0x215473, ('\u{8352}', false)), // East Asian ideograph
    (0x235474, ('\u{9ADF}', false)), // East Asian ideograph
    (0x215475, ('\u{8338}', false)), // East Asian ideograph
    (0x215476, ('\u{8354}', false)), // East Asian ideograph
    (0x235477, ('\u{9AE2}', false)), // East Asian ideograph
    (0x215478, ('\u{8349}', false)), // East Asian ideograph
    (0x215479, ('\u{8335}', false)), // East Asian ideograph
    (0x21547A, ('\u{8334}', false)), // East Asian ideograph
    (0x21547B, ('\u{8336}', false)), // East Asian ideograph
    (0x21547C, ('\u{8331}', false)), // East Asian ideograph
    (0x21547D, ('\u{8340}', false)), // East Asian ideograph
    (0x21547E, ('\u{8317}', false)), // East Asian ideograph
    (0x6F5853, ('\u{CA5D}', false)), // Korean hangul
    (0x295166, ('\u{9969}', false)), // East Asian ideograph
    (0x234A79, ('\u{9696}', false)), // East Asian ideograph
    (0x226450, ('\u{7826}', false)), // East Asian ideograph
    (0x6F5D73, ('\u{D765}', false)), // Korean hangul
    (0x6F4B35, ('\u{B014}', false)), // Korean hangul
    (0x2D6162, ('\u{9A0C}', false)), // East Asian ideograph
    (0x6F5872, ('\u{CBE7}', false)), // Korean hangul
    (0x21316F, ('\u{4FEF}', false)), // East Asian ideograph
    (0x4B4858, ('\u{6E80}', false)), // East Asian ideograph
    (0x6F5854, ('\u{CA61}', false)), // Korean hangul
    (0x222370, ('\u{5CD5}', false)), // East Asian ideograph
    (0x22467B, ('\u{6C62}', false)), // East Asian ideograph
    (0x213E39, ('\u{6063}', false)), // East Asian ideograph
    (0x6F7648, ('\u{E8BA}', false)), // Korean hangul
    (0x4B374C, ('\u{5662}', false)), // East Asian ideograph
    (0x29594F, ('\u{9CE2}', false)), // East Asian ideograph
    (0x696373, ('\u{7B02}', false)), // East Asian ideograph
    (0x295A70, ('\u{9E48}', false)), // East Asian ideograph
    (0x6F5364, ('\u{C22F}', false)), // Korean hangul
    (0x27496A, ('\u{70BC}', false)), // East Asian ideograph
    (0x6F5855, ('\u{CA84}', false)), // Korean hangul
    (0x292375, ('\u{83B3}', false)), // East Asian ideograph
    (0x22467C, ('\u{6C4A}', false)), // East Asian ideograph
    (0x224E21, ('\u{6FAA}', false)), // East Asian ideograph
    (0x6F4E22, ('\u{B541}', false)), // Korean hangul
    (0x6F4E23, ('\u{B543}', false)), // Korean hangul
    (0x225346, ('\u{719E}', false)), // East Asian ideograph
    (0x222379, ('\u{5C8D}', false)), // East Asian ideograph (not in Unicode)
    (0x234E24, ('\u{97B3}', false)), // East Asian ideograph
    (0x6F5856, ('\u{CA98}', false)), // Korean hangul
    (0x224E25, ('\u{6FBF}', false)), // East Asian ideograph
    (0x6F527B, ('\u{C0EC}', false)), // Korean hangul
    (0x224E26, ('\u{6FC7}', false)), // East Asian ideograph
    (0x275A78, ('\u{8E52}', false)), // East Asian ideograph
    (0x274E27, ('\u{77EB}', false)), // East Asian ideograph
    (0x213172, ('\u{5025}', false)), // East Asian ideograph
    (0x6F4E28, ('\u{B54D}', false)), // Korean hangul
    (0x27574A, ('\u{51B2}', false)), // East Asian ideograph (duplicate simplified)
    (0x227E23, ('\u{83A6}', false)), // East Asian ideograph
    (0x234E29, ('\u{97B9}', false)), // East Asian ideograph
    (0x6F5857, ('\u{CABC}', false)), // Korean hangul
    (0x29516A, ('\u{9990}', false)), // East Asian ideograph
    (0x51356A, ('\u{8BC3}', false)), // East Asian ideograph
    (0x6F4E2B, ('\u{B55C}', false)), // Korean hangul
    (0x6F594D, ('\u{CD19}', false)), // Korean hangul
    (0x6F5D5C, ('\u{D700}', false)), // Korean hangul
    (0x6F4E2C, ('\u{B55D}', false)), // Korean hangul
    (0x213173, ('\u{5011}', false)), // East Asian ideograph
    (0x4B613F, ('\u{9A08}', false)), // East Asian ideograph
    (0x214E2D, ('\u{65AB}', false)), // East Asian ideograph
    (0x224E2E, ('\u{6F5E}', false)), // East Asian ideograph
    (0x6F5858, ('\u{CABD}', false)), // Korean hangul
    (0x6F546A, ('\u{C4F0}', false)), // Korean hangul
    (0x224E2F, ('\u{6FC8}', false)), // East Asian ideograph
    (0x2D5763, ('\u{88E1}', false)), // East Asian ideograph
    (0x235521, ('\u{9AE7}', false)), // East Asian ideograph
    (0x215522, ('\u{834F}', false)), // East Asian ideograph
    (0x215523, ('\u{8339}', false)), // East Asian ideograph
    (0x215524, ('\u{838E}', false)), // East Asian ideograph
    (0x215525, ('\u{8398}', false)), // East Asian ideograph
    (0x215526, ('\u{839E}', false)), // East Asian ideograph
    (0x215527, ('\u{8378}', false)), // East Asian ideograph
    (0x215528, ('\u{83A2}', false)), // East Asian ideograph
    (0x225529, ('\u{7225}', false)), // East Asian ideograph
    (0x22552A, ('\u{7226}', false)), // East Asian ideograph
    (0x21552B, ('\u{83AB}', false)), // East Asian ideograph
    (0x21552C, ('\u{8392}', false)), // East Asian ideograph (variant of 4B552C which maps to 8392)
    (0x21552D, ('\u{838A}', false)), // East Asian ideograph
    (0x21552E, ('\u{8393}', false)), // East Asian ideograph
    (0x21552F, ('\u{83A0}', false)), // East Asian ideograph
    (0x215530, ('\u{8389}', false)), // East Asian ideograph
    (0x215531, ('\u{8377}', false)), // East Asian ideograph
    (0x215532, ('\u{837C}', false)), // East Asian ideograph
    (0x215533, ('\u{837B}', false)), // East Asian ideograph
    (0x215534, ('\u{840D}', false)), // East Asian ideograph
    (0x215535, ('\u{83E0}', false)), // East Asian ideograph
    (0x215536, ('\u{83E9}', false)), // East Asian ideograph
    (0x6F5537, ('\u{C57D}', false)), // Korean hangul
    (0x215538, ('\u{8403}', false)), // East Asian ideograph
    (0x215539, ('\u{83C5}', false)), // East Asian ideograph
    (0x21553A, ('\u{83C1}', false)), // East Asian ideograph
    (0x21553B, ('\u{840B}', false)), // East Asian ideograph
    (0x21553C, ('\u{83EF}', false)), // East Asian ideograph
    (0x6F553D, ('\u{C58F}', false)), // Korean hangul
    (0x21553E, ('\u{83F1}', false)), // East Asian ideograph
    (0x21553F, ('\u{83BD}', false)), // East Asian ideograph
    (0x6F5540, ('\u{C595}', false)), // Korean hangul
    (0x235541, ('\u{9B05}', false)), // East Asian ideograph
    (0x215542, ('\u{840C}', false)), // East Asian ideograph
    (0x225543, ('\u{7241}', false)), // East Asian ideograph
    (0x215544, ('\u{83DC}', false)), // East Asian ideograph
    (0x215545, ('\u{83CA}', false)), // East Asian ideograph
    (0x215546, ('\u{83F2}', false)), // East Asian ideograph
    (0x215547, ('\u{840E}', false)), // East Asian ideograph
    (0x215548, ('\u{8404}', false)), // East Asian ideograph
    (0x215549, ('\u{843D}', false)), // East Asian ideograph
    (0x21554A, ('\u{8482}', false)), // East Asian ideograph
    (0x21554B, ('\u{8431}', false)), // East Asian ideograph
    (0x21554C, ('\u{8475}', false)), // East Asian ideograph
    (0x21554D, ('\u{8466}', false)), // East Asian ideograph
    (0x21554E, ('\u{8457}', false)), // East Asian ideograph
    (0x22554F, ('\u{7250}', false)), // East Asian ideograph
    (0x215550, ('\u{846C}', false)), // East Asian ideograph
    (0x214E38, ('\u{7843}', false)), // East Asian ideograph
    (0x215552, ('\u{845B}', false)), // East Asian ideograph
    (0x215553, ('\u{8477}', false)), // East Asian ideograph
    (0x215554, ('\u{843C}', false)), // East Asian ideograph
    (0x215555, ('\u{8435}', false)), // East Asian ideograph
    (0x225556, ('\u{725A}', false)), // East Asian ideograph
    (0x215557, ('\u{8463}', false)), // East Asian ideograph
    (0x215558, ('\u{8469}', false)), // East Asian ideograph
    (0x225559, ('\u{7263}', false)), // East Asian ideograph
    (0x21555A, ('\u{84B2}', false)), // East Asian ideograph
    (0x21555B, ('\u{849E}', false)), // East Asian ideograph
    (0x21555C, ('\u{84BF}', false)), // East Asian ideograph
    (0x21555D, ('\u{84C6}', false)), // East Asian ideograph
    (0x21555E, ('\u{84C4}', false)), // East Asian ideograph
    (0x21555F, ('\u{84C9}', false)), // East Asian ideograph
    (0x215560, ('\u{849C}', false)), // East Asian ideograph
    (0x215561, ('\u{84CB}', false)), // East Asian ideograph
    (0x215562, ('\u{84B8}', false)), // East Asian ideograph
    (0x275563, ('\u{836A}', false)), // East Asian ideograph
    (0x275564, ('\u{82CE}', false)), // East Asian ideograph
    (0x215565, ('\u{84D3}', false)), // East Asian ideograph
    (0x225566, ('\u{7276}', false)), // East Asian ideograph
    (0x215567, ('\u{84BC}', false)), // East Asian ideograph
    (0x225568, ('\u{7277}', false)), // East Asian ideograph
    (0x215569, ('\u{84FF}', false)), // East Asian ideograph
    (0x21556A, ('\u{8517}', false)), // East Asian ideograph
    (0x22556B, ('\u{727E}', false)), // East Asian ideograph
    (0x21556C, ('\u{84EE}', false)), // East Asian ideograph
    (0x21556D, ('\u{852C}', false)), // East Asian ideograph
    (0x27556E, ('\u{836B}', false)), // East Asian ideograph
    (0x21556F, ('\u{8513}', false)), // East Asian ideograph
    (0x6F5570, ('\u{C61B}', false)), // Korean hangul
    (0x215571, ('\u{8523}', false)), // East Asian ideograph
    (0x215572, ('\u{8521}', false)), // East Asian ideograph
    (0x275573, ('\u{535C}', false)), // East Asian ideograph
    (0x225574, ('\u{7289}', false)), // East Asian ideograph
    (0x215575, ('\u{8525}', false)), // East Asian ideograph
    (0x235576, ('\u{9B2F}', false)), // East Asian ideograph
    (0x215577, ('\u{854A}', false)), // East Asian ideograph
    (0x215578, ('\u{8559}', false)), // East Asian ideograph
    (0x215579, ('\u{8548}', false)), // East Asian ideograph
    (0x21557A, ('\u{8568}', false)), // East Asian ideograph
    (0x21557B, ('\u{8543}', false)), // East Asian ideograph
    (0x21557C, ('\u{856A}', false)), // East Asian ideograph
    (0x21557D, ('\u{8549}', false)), // East Asian ideograph
    (0x21557E, ('\u{8584}', false)), // East Asian ideograph
    (0x224E40, ('\u{6FA5}', false)), // East Asian ideograph
    (0x224E41, ('\u{6FB0}', false)), // East Asian ideograph
    (0x4B6048, ('\u{981A}', false)), // East Asian ideograph
    (0x224E42, ('\u{6FAE}', false)), // East Asian ideograph
    (0x2F585C, ('\u{9C51}', false)), // Unrelated variant of EACC 235945 which maps to 9C51
    (0x224E43, ('\u{6FD9}', false)), // East Asian ideograph
    (0x276260, ('\u{4E48}', false)), // East Asian ideograph
    (0x21393F, ('\u{5969}', false)), // East Asian ideograph
    (0x224E44, ('\u{6FDA}', false)), // East Asian ideograph
    (0x274E45, ('\u{7855}', false)), // East Asian ideograph
    (0x6F5B3C, ('\u{D1B0}', false)), // Korean hangul
    (0x273422, ('\u{5218}', false)), // East Asian ideograph
    (0x6F4E46, ('\u{B664}', false)), // Korean hangul
    (0x286B7C, ('\u{7B15}', false)), // East Asian ideograph
    (0x22316C, ('\u{636C}', false)), // East Asian ideograph
    (0x6F4E47, ('\u{B69C}', false)), // Korean hangul
    (0x6F585D, ('\u{CAD1}', false)), // Korean hangul
    (0x295170, ('\u{998D}', false)), // East Asian ideograph
    (0x274E49, ('\u{786E}', false)), // East Asian ideograph
    (0x6F4C4C, ('\u{B220}', false)), // Korean hangul
    (0x6F4E4A, ('\u{B6AB}', false)), // Korean hangul
    (0x213179, ('\u{5006}', false)), // East Asian ideograph
    (0x234E4B, ('\u{97CE}', false)), // East Asian ideograph
    (0x2D753A, ('\u{9654}', false)), // East Asian ideograph
    (0x2D5321, ('\u{7C9B}', false)), // East Asian ideograph
    (0x274E4C, ('\u{7801}', false)), // East Asian ideograph
    (0x3F3078, ('\u{5023}', false)), // East Asian ideograph
    (0x6F585E, ('\u{CAD2}', false)), // Korean hangul
    (0x6F4E4D, ('\u{B6F0}', false)), // Korean hangul
    (0x234E4E, ('\u{97D0}', false)), // East Asian ideograph
    (0x4B552C, ('\u{8392}', false)), // East Asian ideograph
    (0x29546D, ('\u{9ACB}', false)), // East Asian ideograph
    (0x333564, ('\u{5415}', false)), // East Asian ideograph
    (0x275154, ('\u{7EC3}', false)), // East Asian ideograph
    (0x224E50, ('\u{6FD4}', false)), // East Asian ideograph
    (0x213930, ('\u{5949}', false)), // East Asian ideograph
    (0x234E51, ('\u{97D4}', false)), // East Asian ideograph
    (0x6F585F, ('\u{CAD3}', false)), // Korean hangul
    (0x295172, ('\u{9994}', false)), // East Asian ideograph
    (0x21605D, ('\u{98B1}', false)), // East Asian ideograph
    (0x213E44, ('\u{606C}', false)), // East Asian ideograph
    (0x274E53, ('\u{7816}', false)), // East Asian ideograph
    (0x234642, ('\u{93A7}', false)), // East Asian ideograph
    (0x234E54, ('\u{97D9}', false)), // East Asian ideograph
    (0x69245C, ('\u{307C}', false)), // Hiragana letter BO
    (0x6F4E55, ('\u{B72F}', false)), // Korean hangul
    (0x224E56, ('\u{6FE9}', false)), // East Asian ideograph
    (0x6F5860, ('\u{CAD8}', false)), // Korean hangul
    (0x224E57, ('\u{6FF8}', false)), // East Asian ideograph
    (0x4B3758, ('\u{56A5}', false)), // East Asian ideograph (variant of 213758 which maps to 56A5)
    (0x274E58, ('\u{77F6}', false)), // East Asian ideograph
    (0x214E59, ('\u{790E}', false)), // East Asian ideograph
    (0x274E5A, ('\u{788D}', false)), // East Asian ideograph
    (0x2D5C40, ('\u{5FA8}', false)), // East Asian ideograph
    (0x215621, ('\u{85AA}', false)), // East Asian ideograph
    (0x215622, ('\u{856D}', false)), // East Asian ideograph
    (0x235623, ('\u{9B37}', false)), // East Asian ideograph
    (0x275624, ('\u{59DC}', false)), // East Asian ideograph
    (0x215625, ('\u{857E}', false)), // East Asian ideograph
    (0x215626, ('\u{8594}', false)), // East Asian ideograph
    (0x215627, ('\u{859C}', false)), // East Asian ideograph
    (0x225628, ('\u{728F}', false)), // East Asian ideograph
    (0x215629, ('\u{85CD}', false)), // East Asian ideograph (variant of 4B5629 which maps to 85CD)
    (0x27562A, ('\u{8428}', false)), // East Asian ideograph
    (0x21562B, ('\u{85CF}', false)), // East Asian ideograph
    (0x21562C, ('\u{85AF}', false)), // East Asian ideograph
    (0x21562D, ('\u{85D0}', false)), // East Asian ideograph
    (0x27562E, ('\u{501F}', false)), // East Asian ideograph
    (0x214E5D, ('\u{792A}', false)), // East Asian ideograph
    (0x215630, ('\u{85E9}', false)), // East Asian ideograph
    (0x215631, ('\u{85DD}', false)), // East Asian ideograph
    (0x275632, ('\u{85AE}', false)), // East Asian ideograph
    (0x215633, ('\u{85E4}', false)), // East Asian ideograph
    (0x215634, ('\u{85D5}', false)), // East Asian ideograph
    (0x224E5E, ('\u{6FEE}', false)), // East Asian ideograph
    (0x215636, ('\u{85FB}', false)), // East Asian ideograph
    (0x215637, ('\u{85F9}', false)), // East Asian ideograph
    (0x215638, ('\u{8611}', false)), // East Asian ideograph
    (0x215639, ('\u{85FA}', false)), // East Asian ideograph
    (0x27563A, ('\u{82A6}', false)), // East Asian ideograph
    (0x27563B, ('\u{82F9}', false)), // East Asian ideograph
    (0x27563C, ('\u{82CF}', false)), // East Asian ideograph
    (0x27563D, ('\u{8574}', false)), // East Asian ideograph
    (0x27563E, ('\u{5170}', false)), // East Asian ideograph
    (0x21563F, ('\u{8617}', false)), // East Asian ideograph
    (0x215640, ('\u{861A}', false)), // East Asian ideograph
    (0x215641, ('\u{8638}', false)), // East Asian ideograph
    (0x275642, ('\u{841D}', false)), // East Asian ideograph
    (0x215643, ('\u{864E}', false)), // East Asian ideograph
    (0x215644, ('\u{8650}', false)), // East Asian ideograph
    (0x215645, ('\u{8654}', false)), // East Asian ideograph
    (0x215646, ('\u{5F6A}', false)), // East Asian ideograph
    (0x215647, ('\u{8655}', false)), // East Asian ideograph
    (0x275648, ('\u{864F}', false)), // East Asian ideograph
    (0x215649, ('\u{865B}', false)), // East Asian ideograph
    (0x27564A, ('\u{53F7}', false)), // East Asian ideograph
    (0x21564B, ('\u{865E}', false)), // East Asian ideograph
    (0x22564C, ('\u{72AB}', false)), // East Asian ideograph
    (0x224E62, ('\u{6FF0}', false)), // East Asian ideograph
    (0x22564E, ('\u{72B0}', false)), // East Asian ideograph
    (0x21564F, ('\u{8679}', false)), // East Asian ideograph
    (0x215650, ('\u{86A9}', false)), // East Asian ideograph
    (0x215651, ('\u{86AA}', false)), // East Asian ideograph
    (0x215652, ('\u{868A}', false)), // East Asian ideograph
    (0x215653, ('\u{8693}', false)), // East Asian ideograph
    (0x215654, ('\u{86A4}', false)), // East Asian ideograph
    (0x215655, ('\u{868C}', false)), // East Asian ideograph
    (0x215656, ('\u{86A3}', false)), // East Asian ideograph
    (0x215657, ('\u{86C0}', false)), // East Asian ideograph
    (0x215658, ('\u{86C7}', false)), // East Asian ideograph
    (0x215659, ('\u{86B5}', false)), // East Asian ideograph
    (0x27565A, ('\u{65E6}', false)), // East Asian ideograph
    (0x21565B, ('\u{86B6}', false)), // East Asian ideograph
    (0x21565C, ('\u{86C4}', false)), // East Asian ideograph
    (0x21565D, ('\u{86C6}', false)), // East Asian ideograph
    (0x21565E, ('\u{86B1}', false)), // East Asian ideograph
    (0x21565F, ('\u{86AF}', false)), // East Asian ideograph
    (0x225660, ('\u{72D6}', false)), // East Asian ideograph
    (0x215661, ('\u{86D9}', false)), // East Asian ideograph
    (0x215662, ('\u{86ED}', false)), // East Asian ideograph
    (0x215663, ('\u{86D4}', false)), // East Asian ideograph
    (0x225664, ('\u{72D2}', false)), // East Asian ideograph
    (0x224E66, ('\u{7005}', false)), // East Asian ideograph
    (0x215666, ('\u{86FB}', false)), // East Asian ideograph
    (0x225667, ('\u{72C9}', false)), // East Asian ideograph
    (0x215668, ('\u{8707}', false)), // East Asian ideograph
    (0x215669, ('\u{8703}', false)), // East Asian ideograph
    (0x21566A, ('\u{8708}', false)), // East Asian ideograph
    (0x214E67, ('\u{7955}', false)), // East Asian ideograph
    (0x21566C, ('\u{86FE}', false)), // East Asian ideograph
    (0x21566D, ('\u{8713}', false)), // East Asian ideograph
    (0x21566E, ('\u{8702}', false)), // East Asian ideograph
    (0x21566F, ('\u{871C}', false)), // East Asian ideograph
    (0x215670, ('\u{873F}', false)), // East Asian ideograph
    (0x215671, ('\u{873B}', false)), // East Asian ideograph
    (0x215672, ('\u{8722}', false)), // East Asian ideograph
    (0x225673, ('\u{72E8}', false)), // East Asian ideograph
    (0x215674, ('\u{8734}', false)), // East Asian ideograph
    (0x215675, ('\u{8718}', false)), // East Asian ideograph
    (0x215676, ('\u{8755}', false)), // East Asian ideograph
    (0x215677, ('\u{8760}', false)), // East Asian ideograph
    (0x215678, ('\u{8776}', false)), // East Asian ideograph
    (0x225679, ('\u{72E5}', false)), // East Asian ideograph
    (0x27567A, ('\u{867E}', false)), // East Asian ideograph
    (0x21567B, ('\u{8778}', false)), // East Asian ideograph
    (0x21567C, ('\u{8768}', false)), // East Asian ideograph
    (0x21567D, ('\u{874C}', false)), // East Asian ideograph
    (0x22567E, ('\u{72FA}', false)), // East Asian ideograph
    (0x6F4E6B, ('\u{B790}', false)), // Korean hangul
    (0x6F5421, ('\u{C2B7}', false)), // Korean hangul
    (0x234E6C, ('\u{97F5}', false)), // East Asian ideograph
    (0x6F5339, ('\u{C151}', false)), // Korean hangul
    (0x6F4E6D, ('\u{B797}', false)), // Korean hangul
    (0x69245D, ('\u{307D}', false)), // Hiragana letter PO
    (0x6F4E6E, ('\u{B798}', false)), // Korean hangul
    (0x274E6F, ('\u{53EA}', false)), // East Asian ideograph (duplicate simplified)
    (0x6F5865, ('\u{CB20}', false)), // Korean hangul
    (0x6F4E70, ('\u{B79C}', false)), // Korean hangul
    (0x6F5422, ('\u{C2B9}', false)), // Korean hangul
    (0x6F5A2A, ('\u{CEF7}', false)), // Korean hangul
    (0x6F4E71, ('\u{B7A0}', false)), // Korean hangul
    (0x234648, ('\u{939A}', false)), // East Asian ideograph
    (0x213441, ('\u{5305}', false)), // East Asian ideograph
    (0x224E72, ('\u{7026}', false)), // East Asian ideograph
    (0x214E73, ('\u{797A}', false)), // East Asian ideograph
    (0x286C58, ('\u{7BA7}', false)), // East Asian ideograph
    (0x6F5633, ('\u{C68D}', false)), // Korean hangul
    (0x6F4E3B, ('\u{B5BC}', false)), // Korean hangul
    (0x6F5866, ('\u{CB21}', false)), // Korean hangul
    (0x395179, ('\u{7D75}', false)), // East Asian ideograph
    (0x6F4E76, ('\u{B7AD}', false)), // Korean hangul
    (0x213921, ('\u{5920}', false)), // East Asian ideograph
    (0x274E77, ('\u{7978}', false)), // East Asian ideograph
    (0x27785E, ('\u{5786}', false)), // East Asian ideograph
    (0x213922, ('\u{5924}', false)), // East Asian ideograph
    (0x274E78, ('\u{796F}', false)), // East Asian ideograph
    (0x213923, ('\u{5925}', false)), // East Asian ideograph
    (0x234E79, ('\u{9807}', false)), // East Asian ideograph
    (0x273924, ('\u{68A6}', false)), // East Asian ideograph
    (0x6F5867, ('\u{CB41}', false)), // Korean hangul
    (0x213F37, ('\u{6155}', false)), // East Asian ideograph
    (0x6F4E7A, ('\u{B7EC}', false)), // Korean hangul
    (0x6F4D61, ('\u{B4D0}', false)), // Korean hangul
    (0x226464, ('\u{7876}', false)), // East Asian ideograph
    (0x274E7B, ('\u{7985}', false)), // East Asian ideograph
    (0x69482B, ('\u{7560}', false)), // East Asian ideograph
    (0x274E7C, ('\u{793C}', false)), // East Asian ideograph
    (0x4B4F3C, ('\u{79F0}', false)), // East Asian ideograph (variant of 274F3C which maps to 79F0)
    (0x213927, ('\u{592B}', false)), // East Asian ideograph
    (0x274E7D, ('\u{7977}', false)), // East Asian ideograph
    (0x2D5323, ('\u{5B8D}', false)), // East Asian ideograph
    (0x234E7E, ('\u{980D}', false)), // East Asian ideograph
    (0x2D3929, ('\u{6B80}', false)), // East Asian ideograph
    (0x213376, ('\u{5274}', false)), // East Asian ideograph
    (0x6F5868, ('\u{CB48}', false)), // Korean hangul
    (0x6F5433, ('\u{C300}', false)), // Korean hangul
    (0x216066, ('\u{98E7}', false)), // East Asian ideograph
    (0x21392B, ('\u{5931}', false)), // East Asian ideograph
    (0x3F304C, ('\u{5E79}', false)), // East Asian ideograph
    (0x2E3A26, ('\u{661D}', false)), // East Asian ideograph
    (0x225359, ('\u{71B4}', false)), // East Asian ideograph
    (0x21392E, ('\u{593E}', false)), // East Asian ideograph
    (0x6F5869, ('\u{CB49}', false)), // Korean hangul
    (0x396B33, ('\u{5259}', false)), // East Asian ideograph (not in Unicode)
    (0x216067, ('\u{98E9}', false)), // East Asian ideograph
    (0x235721, ('\u{9B83}', false)), // East Asian ideograph
    (0x215722, ('\u{8783}', false)), // East Asian ideograph
    (0x215723, ('\u{8782}', false)), // East Asian ideograph
    (0x275724, ('\u{8424}', false)), // East Asian ideograph
    (0x225725, ('\u{72FE}', false)), // East Asian ideograph
    (0x215726, ('\u{878D}', false)), // East Asian ideograph
    (0x215727, ('\u{879F}', false)), // East Asian ideograph
    (0x215728, ('\u{87D1}', false)), // East Asian ideograph
    (0x215729, ('\u{87C0}', false)), // East Asian ideograph
    (0x21572A, ('\u{87AB}', false)), // East Asian ideograph
    (0x23572B, ('\u{9B90}', false)), // East Asian ideograph
    (0x27572C, ('\u{877C}', false)), // East Asian ideograph
    (0x22572D, ('\u{7301}', false)), // East Asian ideograph
    (0x22572E, ('\u{72F3}', false)), // East Asian ideograph
    (0x23572F, ('\u{9B97}', false)), // East Asian ideograph
    (0x215730, ('\u{87C6}', false)), // East Asian ideograph
    (0x215731, ('\u{87CB}', false)), // East Asian ideograph
    (0x215732, ('\u{87EF}', false)), // East Asian ideograph
    (0x215733, ('\u{87F2}', false)), // East Asian ideograph
    (0x215734, ('\u{87EC}', false)), // East Asian ideograph
    (0x225735, ('\u{730B}', false)), // East Asian ideograph
    (0x225736, ('\u{7317}', false)), // East Asian ideograph
    (0x215737, ('\u{880D}', false)), // East Asian ideograph
    (0x215738, ('\u{87F9}', false)), // East Asian ideograph
    (0x215739, ('\u{8814}', false)), // East Asian ideograph
    (0x21573A, ('\u{8815}', false)), // East Asian ideograph
    (0x22573B, ('\u{7307}', false)), // East Asian ideograph
    (0x23573C, ('\u{9BAD}', false)), // East Asian ideograph
    (0x23573D, ('\u{9B9A}', false)), // East Asian ideograph
    (0x22573E, ('\u{7318}', false)), // East Asian ideograph
    (0x27573F, ('\u{86CA}', false)), // East Asian ideograph
    (0x215740, ('\u{8839}', false)), // East Asian ideograph
    (0x275741, ('\u{8695}', false)), // East Asian ideograph
    (0x215742, ('\u{883B}', false)), // East Asian ideograph
    (0x235743, ('\u{9B99}', false)), // East Asian ideograph
    (0x215744, ('\u{884C}', false)), // East Asian ideograph
    (0x215745, ('\u{884D}', false)), // East Asian ideograph
    (0x225746, ('\u{7331}', false)), // East Asian ideograph
    (0x215747, ('\u{8857}', false)), // East Asian ideograph
    (0x215748, ('\u{8859}', false)), // East Asian ideograph
    (0x225749, ('\u{7338}', false)), // East Asian ideograph
    (0x22574A, ('\u{7322}', false)), // East Asian ideograph
    (0x21574B, ('\u{8861}', false)), // East Asian ideograph
    (0x22574C, ('\u{7332}', false)), // East Asian ideograph
    (0x22574D, ('\u{732C}', false)), // East Asian ideograph
    (0x22574E, ('\u{7327}', false)), // East Asian ideograph
    (0x22574F, ('\u{732B}', false)), // East Asian ideograph
    (0x215750, ('\u{886B}', false)), // East Asian ideograph
    (0x215751, ('\u{8882}', false)), // East Asian ideograph
    (0x225752, ('\u{732F}', false)), // East Asian ideograph
    (0x215753, ('\u{8870}', false)), // East Asian ideograph
    (0x215754, ('\u{8877}', false)), // East Asian ideograph
    (0x225755, ('\u{7328}', false)), // East Asian ideograph
    (0x235756, ('\u{9BC7}', false)), // East Asian ideograph
    (0x215757, ('\u{8892}', false)), // East Asian ideograph
    (0x215758, ('\u{8896}', false)), // East Asian ideograph
    (0x235759, ('\u{9BD2}', false)), // East Asian ideograph
    (0x22575A, ('\u{7347}', false)), // East Asian ideograph
    (0x22575B, ('\u{7348}', false)), // East Asian ideograph
    (0x22575C, ('\u{7349}', false)), // East Asian ideograph
    (0x23393A, ('\u{8DE6}', false)), // East Asian ideograph
    (0x21575E, ('\u{88B1}', false)), // East Asian ideograph
    (0x23575F, ('\u{9BC1}', false)), // East Asian ideograph
    (0x215760, ('\u{88D9}', false)), // East Asian ideograph
    (0x215761, ('\u{88D8}', false)), // East Asian ideograph
    (0x215762, ('\u{88DC}', false)), // East Asian ideograph
    (0x215763, ('\u{88CF}', false)), // East Asian ideograph
    (0x215764, ('\u{88D4}', false)), // East Asian ideograph
    (0x225765, ('\u{7340}', false)), // East Asian ideograph
    (0x215766, ('\u{88D5}', false)), // East Asian ideograph
    (0x215767, ('\u{8902}', false)), // East Asian ideograph
    (0x225768, ('\u{734D}', false)), // East Asian ideograph
    (0x215769, ('\u{88F8}', false)), // East Asian ideograph
    (0x21576A, ('\u{88F9}', false)), // East Asian ideograph
    (0x21576B, ('\u{88F4}', false)), // East Asian ideograph
    (0x23576C, ('\u{9BD3}', false)), // East Asian ideograph
    (0x21576D, ('\u{88E8}', false)), // East Asian ideograph
    (0x21576E, ('\u{891A}', false)), // East Asian ideograph
    (0x21576F, ('\u{8910}', false)), // East Asian ideograph
    (0x6F5770, ('\u{C906}', false)), // Korean hangul
    (0x215771, ('\u{8913}', false)), // East Asian ideograph
    (0x235772, ('\u{9BC8}', false)), // East Asian ideograph
    (0x215773, ('\u{8932}', false)), // East Asian ideograph
    (0x225774, ('\u{735D}', false)), // East Asian ideograph
    (0x215775, ('\u{8925}', false)), // East Asian ideograph
    (0x215776, ('\u{892B}', false)), // East Asian ideograph
    (0x235777, ('\u{9BD7}', false)), // East Asian ideograph
    (0x215778, ('\u{8936}', false)), // East Asian ideograph
    (0x225779, ('\u{7360}', false)), // East Asian ideograph
    (0x23577A, ('\u{9BD6}', false)), // East Asian ideograph
    (0x21577B, ('\u{895F}', false)), // East Asian ideograph
    (0x23577C, ('\u{9BEB}', false)), // East Asian ideograph
    (0x21577D, ('\u{8956}', false)), // East Asian ideograph
    (0x22577E, ('\u{7362}', false)), // East Asian ideograph
    (0x273940, ('\u{593A}', false)), // East Asian ideograph
    (0x223941, ('\u{66AA}', false)), // East Asian ideograph
    (0x232B33, ('\u{874D}', false)), // East Asian ideograph
    (0x2D506F, ('\u{7CFA}', false)), // East Asian ideograph
    (0x6F586D, ('\u{CB5D}', false)), // Korean hangul
    (0x6F7643, ('\u{E8B5}', false)), // Korean hangul
    (0x213943, ('\u{5974}', false)), // East Asian ideograph
    (0x213944, ('\u{5976}', false)), // East Asian ideograph
    (0x4B3322, ('\u{5168}', false)), // East Asian ideograph (variant of 213322 which maps to 5168)
    (0x27564C, ('\u{4E8F}', false)), // East Asian ideograph
    (0x4B5E3D, ('\u{9421}', false)), // East Asian ideograph
    (0x213946, ('\u{5983}', false)), // East Asian ideograph
    (0x6F5365, ('\u{C231}', false)), // Korean hangul
    (0x6F516D, ('\u{BE44}', false)), // Korean hangul
    (0x213947, ('\u{5978}', false)), // East Asian ideograph
    (0x2D4C2D, ('\u{756E}', false)), // East Asian ideograph
    (0x6F542B, ('\u{C2EF}', false)), // Korean hangul
    (0x213E53, ('\u{6089}', false)), // East Asian ideograph
    (0x213949, ('\u{5979}', false)), // East Asian ideograph
    (0x21794B, ('\u{595C}', false)), // East Asian ideograph
    (0x454E75, ('\u{7984}', false)), // East Asian ideograph
    (0x213C7D, ('\u{5EC2}', false)), // East Asian ideograph
    (0x6F586F, ('\u{CBB8}', false)), // Korean hangul
    (0x2D394D, ('\u{59AC}', false)), // East Asian ideograph
    (0x217E43, ('\u{5B93}', false)), // East Asian ideograph
    (0x22394E, ('\u{66C8}', false)), // East Asian ideograph
    (0x4B3324, ('\u{634C}', false)), // East Asian ideograph (variant of 2D3324 which maps to 634C)
    (0x21394F, ('\u{59A4}', false)), // East Asian ideograph
    (0x213F58, ('\u{61FA}', false)), // East Asian ideograph
    (0x213950, ('\u{59A3}', false)), // East Asian ideograph
    (0x6F4E3D, ('\u{B5C4}', false)), // Korean hangul
    (0x213951, ('\u{5993}', false)), // East Asian ideograph
    (0x2F5870, ('\u{9C1B}', false)), // East Asian ideograph
    (0x6F7646, ('\u{E8B8}', false)), // Korean hangul
    (0x213952, ('\u{599E}', false)), // East Asian ideograph
    (0x213E55, ('\u{60A0}', false)), // East Asian ideograph
    (0x4B3768, ('\u{56D8}', false)), // East Asian ideograph
    (0x213953, ('\u{599D}', false)), // East Asian ideograph
    (0x233954, ('\u{8E23}', false)), // East Asian ideograph
    (0x213955, ('\u{59A5}', false)), // East Asian ideograph
    (0x335760, ('\u{88E0}', false)), // East Asian ideograph
    (0x2D3956, ('\u{59D9}', false)), // East Asian ideograph
    (0x6F4F22, ('\u{B7FF}', false)), // Korean hangul
    (0x6F5871, ('\u{CBE4}', false)), // Korean hangul
    (0x6F7647, ('\u{E8B9}', false)), // Korean hangul
    (0x213957, ('\u{5996}', false)), // East Asian ideograph
    (0x213958, ('\u{59BE}', false)), // East Asian ideograph
    (0x333642, ('\u{8A92}', false)), // East Asian ideograph
    (0x2D3F67, ('\u{621E}', false)), // East Asian ideograph
    (0x6F5878, ('\u{CC1D}', false)), // Korean hangul
    (0x4C7959, ('\u{817D}', false)), // East Asian ideograph
    (0x273437, ('\u{80DC}', false)), // East Asian ideograph
    (0x214F63, ('\u{7AC7}', false)), // East Asian ideograph
    (0x4B524E, ('\u{7FCE}', false)), // East Asian ideograph
    (0x21395A, ('\u{59AE}', false)), // East Asian ideograph
    (0x3A4034, ('\u{6855}', false)), // East Asian ideograph
    (0x275821, ('\u{889C}', false)), // East Asian ideograph
    (0x275822, ('\u{886C}', false)), // East Asian ideograph
    (0x215823, ('\u{8972}', false)), // East Asian ideograph
    (0x215824, ('\u{897F}', false)), // East Asian ideograph
    (0x225825, ('\u{7367}', false)), // East Asian ideograph
    (0x215826, ('\u{8983}', false)), // East Asian ideograph
    (0x235827, ('\u{9BE4}', false)), // East Asian ideograph
    (0x275828, ('\u{89C1}', false)), // East Asian ideograph
    (0x275829, ('\u{89C4}', false)), // East Asian ideograph
    (0x27582A, ('\u{89C5}', false)), // East Asian ideograph
    (0x27582B, ('\u{89C6}', false)), // East Asian ideograph
    (0x27582C, ('\u{4EB2}', false)), // East Asian ideograph
    (0x27582D, ('\u{89CE}', false)), // East Asian ideograph
    (0x21582E, ('\u{89AC}', false)), // East Asian ideograph
    (0x27582F, ('\u{89D0}', false)), // East Asian ideograph
    (0x215830, ('\u{89BA}', false)), // East Asian ideograph
    (0x275831, ('\u{89C8}', false)), // East Asian ideograph
    (0x215832, ('\u{89C0}', false)), // East Asian ideograph
    (0x215833, ('\u{89D2}', false)), // East Asian ideograph
    (0x235834, ('\u{9BD4}', false)), // East Asian ideograph
    (0x215835, ('\u{89F4}', false)), // East Asian ideograph
    (0x225836, ('\u{737C}', false)), // East Asian ideograph
    (0x215837, ('\u{8A00}', false)), // East Asian ideograph
    (0x215838, ('\u{8A08}', false)), // East Asian ideograph
    (0x215839, ('\u{8A02}', false)), // East Asian ideograph
    (0x21583A, ('\u{8A03}', false)), // East Asian ideograph
    (0x21583B, ('\u{8A10}', false)), // East Asian ideograph
    (0x21583C, ('\u{8A18}', false)), // East Asian ideograph
    (0x21583D, ('\u{8A0E}', false)), // East Asian ideograph
    (0x23583E, ('\u{9BFF}', false)), // East Asian ideograph
    (0x21583F, ('\u{8A15}', false)), // East Asian ideograph
    (0x215840, ('\u{8A0A}', false)), // East Asian ideograph
    (0x275841, ('\u{8BAB}', false)), // East Asian ideograph
    (0x225842, ('\u{738E}', false)), // East Asian ideograph
    (0x235843, ('\u{9C06}', false)), // East Asian ideograph
    (0x235844, ('\u{9C15}', false)), // East Asian ideograph
    (0x215845, ('\u{8A23}', false)), // East Asian ideograph
    (0x275846, ('\u{8BB6}', false)), // East Asian ideograph
    (0x225847, ('\u{7392}', false)), // East Asian ideograph
    (0x215848, ('\u{8A31}', false)), // East Asian ideograph
    (0x275849, ('\u{8BBE}', false)), // East Asian ideograph
    (0x27584A, ('\u{8BB9}', false)), // East Asian ideograph
    (0x27584B, ('\u{8BBC}', false)), // East Asian ideograph
    (0x27584C, ('\u{6CE8}', false)), // East Asian ideograph
    (0x21584D, ('\u{8A60}', false)), // East Asian ideograph
    (0x27584E, ('\u{8BC4}', false)), // East Asian ideograph
    (0x27584F, ('\u{8BCD}', false)), // East Asian ideograph
    (0x6F5850, ('\u{CA50}', false)), // Korean hangul
    (0x215851, ('\u{8A41}', false)), // East Asian ideograph
    (0x235852, ('\u{9C02}', false)), // East Asian ideograph
    (0x215853, ('\u{8A5B}', false)), // East Asian ideograph
    (0x235854, ('\u{9C10}', false)), // East Asian ideograph
    (0x215855, ('\u{8A46}', false)), // East Asian ideograph
    (0x215856, ('\u{8A34}', false)), // East Asian ideograph
    (0x275857, ('\u{8BCA}', false)), // East Asian ideograph
    (0x275858, ('\u{8BE7}', false)), // East Asian ideograph
    (0x215859, ('\u{8A72}', false)), // East Asian ideograph
    (0x21585A, ('\u{8A73}', false)), // East Asian ideograph
    (0x21585B, ('\u{8A66}', false)), // East Asian ideograph
    (0x27585C, ('\u{8BD7}', false)), // East Asian ideograph
    (0x27585D, ('\u{8BD8}', false)), // East Asian ideograph
    (0x27585E, ('\u{8BE3}', false)), // East Asian ideograph
    (0x27585F, ('\u{8BD9}', false)), // East Asian ideograph
    (0x275860, ('\u{8BDA}', false)), // East Asian ideograph
    (0x215861, ('\u{8A87}', false)), // East Asian ideograph
    (0x275862, ('\u{8BDB}', false)), // East Asian ideograph
    (0x215863, ('\u{8A6D}', false)), // East Asian ideograph
    (0x215864, ('\u{8A79}', false)), // East Asian ideograph
    (0x275865, ('\u{8BE2}', false)), // East Asian ideograph
    (0x275866, ('\u{8BDD}', false)), // East Asian ideograph
    (0x275867, ('\u{8BE0}', false)), // East Asian ideograph
    (0x215868, ('\u{8A6C}', false)), // East Asian ideograph
    (0x235869, ('\u{9C2F}', false)), // East Asian ideograph
    (0x22586A, ('\u{73C2}', false)), // East Asian ideograph
    (0x22586B, ('\u{73D0}', false)), // East Asian ideograph
    (0x21586C, ('\u{8A9E}', false)), // East Asian ideograph
    (0x21586D, ('\u{8A8C}', false)), // East Asian ideograph
    (0x21586E, ('\u{8A93}', false)), // East Asian ideograph
    (0x22586F, ('\u{73BF}', false)), // East Asian ideograph
    (0x275870, ('\u{8BA4}', false)), // East Asian ideograph
    (0x275871, ('\u{8BEF}', false)), // East Asian ideograph
    (0x275872, ('\u{8BF2}', false)), // East Asian ideograph
    (0x275873, ('\u{8BF0}', false)), // East Asian ideograph
    (0x275874, ('\u{8BF1}', false)), // East Asian ideograph
    (0x275875, ('\u{8BF3}', false)), // East Asian ideograph
    (0x215876, ('\u{8ABC}', false)), // East Asian ideograph
    (0x275877, ('\u{8C06}', false)), // East Asian ideograph
    (0x275878, ('\u{8C05}', false)), // East Asian ideograph
    (0x215879, ('\u{8AC7}', false)), // East Asian ideograph
    (0x21587A, ('\u{8ACB}', false)), // East Asian ideograph (variant of 4B587A which maps to 8ACB)
    (0x27587B, ('\u{8BF8}', false)), // East Asian ideograph
    (0x27587C, ('\u{8BFE}', false)), // East Asian ideograph
    (0x27587D, ('\u{8C03}', false)), // East Asian ideograph
    (0x23587E, ('\u{9C46}', false)), // East Asian ideograph
    (0x6F5875, ('\u{CC10}', false)), // Korean hangul
    (0x6F764B, ('\u{E8BD}', false)), // Korean hangul
    (0x21796B, ('\u{5998}', false)), // East Asian ideograph
    (0x6F4B62, ('\u{B0BB}', false)), // Korean hangul
    (0x293C5A, ('\u{8F73}', false)), // East Asian ideograph
    (0x2D396E, ('\u{4F84}', false)), // East Asian ideograph
    (0x28732D, ('\u{7F2F}', false)), // East Asian ideograph
    (0x4B6145, ('\u{9A12}', false)), // East Asian ideograph
    (0x21396F, ('\u{5A01}', false)), // East Asian ideograph
    (0x2D4C35, ('\u{7567}', false)), // East Asian ideograph
    (0x2D3970, ('\u{5A63}', false)), // East Asian ideograph
    (0x213971, ('\u{59E6}', false)), // East Asian ideograph
    (0x6F5879, ('\u{CC21}', false)), // Korean hangul
    (0x213972, ('\u{59DA}', false)), // East Asian ideograph
    (0x213973, ('\u{5A11}', false)), // East Asian ideograph
    (0x2D3974, ('\u{5B43}', false)), // East Asian ideograph
    (0x6F5877, ('\u{CC1C}', false)), // Korean hangul
    (0x6F764D, ('\u{E8BF}', false)), // Korean hangul
    (0x6F5434, ('\u{C308}', false)), // Korean hangul
    (0x213E5C, ('\u{60B6}', false)), // East Asian ideograph
    (0x4B376F, ('\u{56FD}', false)), // East Asian ideograph
    (0x213976, ('\u{5A1C}', false)), // East Asian ideograph
    (0x692421, ('\u{3041}', false)), // Hiragana letter small A
    (0x213977, ('\u{5A13}', false)), // East Asian ideograph
    (0x2D5773, ('\u{7D5D}', false)), // East Asian ideograph
    (0x213978, ('\u{59EC}', false)), // East Asian ideograph
    (0x33354E, ('\u{608B}', false)), // East Asian ideograph
    (0x213979, ('\u{5A20}', false)), // East Asian ideograph
    (0x216424, ('\u{4E0F}', false)), // East Asian ideograph
    (0x6F764E, ('\u{E8C0}', false)), // Korean hangul
    (0x6F535C, ('\u{C219}', false)), // Korean hangul
    (0x213E5D, ('\u{60D1}', false)), // East Asian ideograph
    (0x2D397B, ('\u{5A31}', false)), // East Asian ideograph
    (0x2D3F6E, ('\u{6226}', false)), // East Asian ideograph
    (0x21397C, ('\u{5A0C}', false)), // East Asian ideograph
    (0x692427, ('\u{3047}', false)), // Hiragana letter small E
    (0x6F5841, ('\u{C9EF}', false)), // Korean hangul
    (0x22797D, ('\u{81A6}', false)), // East Asian ideograph
    (0x692428, ('\u{3048}', false)), // Hiragana letter E
    (0x21397E, ('\u{5A25}', false)), // East Asian ideograph
    (0x222429, ('\u{5CDD}', false)), // East Asian ideograph
    (0x6F764F, ('\u{E8C1}', false)), // Korean hangul
    (0x6F5436, ('\u{C30B}', false)), // Korean hangul
    (0x213E5E, ('\u{60B5}', false)), // East Asian ideograph
    (0x33304C, ('\u{4E79}', false)), // East Asian ideograph
    (0x215C34, ('\u{904B}', false)), // East Asian ideograph
    (0x2D3F6F, ('\u{622F}', false)), // East Asian ideograph
    (0x27742E, ('\u{56F5}', false)), // East Asian ideograph
    (0x216D41, ('\u{534C}', false)), // East Asian ideograph
    (0x6F587A, ('\u{CC22}', false)), // Korean hangul
    (0x6F7650, ('\u{E8C2}', false)), // Korean hangul
    (0x6F5437, ('\u{C30C}', false)), // Korean hangul
    (0x69242F, ('\u{304F}', false)), // Hiragana letter KU
    (0x4B3772, ('\u{5186}', false)), // East Asian ideograph
    (0x4B5631, ('\u{82B8}', false)), // East Asian ideograph
    (0x225921, ('\u{73D3}', false)), // East Asian ideograph
    (0x215922, ('\u{8AB0}', false)), // East Asian ideograph
    (0x215923, ('\u{8A95}', false)), // East Asian ideograph
    (0x215924, ('\u{8AD6}', false)), // East Asian ideograph
    (0x275925, ('\u{8C1B}', false)), // East Asian ideograph
    (0x235926, ('\u{9C44}', false)), // East Asian ideograph
    (0x215927, ('\u{8AEB}', false)), // East Asian ideograph
    (0x225928, ('\u{73E5}', false)), // East Asian ideograph
    (0x235929, ('\u{9C39}', false)), // East Asian ideograph
    (0x22592A, ('\u{73D9}', false)), // East Asian ideograph
    (0x22592B, ('\u{73EF}', false)), // East Asian ideograph
    (0x21592C, ('\u{8B01}', false)), // East Asian ideograph (variant of 2D592C which maps to 8B01)
    (0x21592D, ('\u{8B02}', false)), // East Asian ideograph
    (0x21592E, ('\u{8AFE}', false)), // East Asian ideograph
    (0x27592F, ('\u{8BBD}', false)), // East Asian ideograph
    (0x235930, ('\u{9C47}', false)), // East Asian ideograph
    (0x215931, ('\u{8B17}', false)), // East Asian ideograph
    (0x225932, ('\u{73D6}', false)), // East Asian ideograph
    (0x215933, ('\u{8B0E}', false)), // East Asian ideograph
    (0x235934, ('\u{9C37}', false)), // East Asian ideograph
    (0x225935, ('\u{73BC}', false)), // East Asian ideograph
    (0x215936, ('\u{8B21}', false)), // East Asian ideograph
    (0x215937, ('\u{8B04}', false)), // East Asian ideograph
    (0x235938, ('\u{9C52}', false)), // East Asian ideograph
    (0x275939, ('\u{8C28}', false)), // East Asian ideograph
    (0x22593A, ('\u{73DE}', false)), // East Asian ideograph
    (0x23593B, ('\u{9C58}', false)), // East Asian ideograph
    (0x22593C, ('\u{73E6}', false)), // East Asian ideograph
    (0x21593D, ('\u{8B5C}', false)), // East Asian ideograph
    (0x21593E, ('\u{8B4E}', false)), // East Asian ideograph
    (0x21593F, ('\u{8B49}', false)), // East Asian ideograph
    (0x275940, ('\u{8C2D}', false)), // East Asian ideograph
    (0x215941, ('\u{8B41}', false)), // East Asian ideograph
    (0x275942, ('\u{8BA5}', false)), // East Asian ideograph
    (0x215943, ('\u{8B70}', false)), // East Asian ideograph
    (0x215944, ('\u{8B6C}', false)), // East Asian ideograph
    (0x225945, ('\u{73F6}', false)), // East Asian ideograph
    (0x215946, ('\u{8B6F}', false)), // East Asian ideograph
    (0x225947, ('\u{73FA}', false)), // East Asian ideograph
    (0x275948, ('\u{62A4}', false)), // East Asian ideograph
    (0x215949, ('\u{8B7D}', false)), // East Asian ideograph
    (0x27594A, ('\u{8BFB}', false)), // East Asian ideograph
    (0x21594B, ('\u{8B8A}', false)), // East Asian ideograph
    (0x27594C, ('\u{8BA9}', false)), // East Asian ideograph
    (0x21594D, ('\u{8B96}', false)), // East Asian ideograph
    (0x21594E, ('\u{8B92}', false)), // East Asian ideograph
    (0x23594F, ('\u{9C67}', false)), // East Asian ideograph
    (0x6F5950, ('\u{CD2C}', false)), // Korean hangul
    (0x215951, ('\u{8C41}', false)), // East Asian ideograph
    (0x215952, ('\u{8C3F}', false)), // East Asian ideograph
    (0x215953, ('\u{8C46}', false)), // East Asian ideograph
    (0x225954, ('\u{73F5}', false)), // East Asian ideograph
    (0x235955, ('\u{9C5F}', false)), // East Asian ideograph
    (0x235956, ('\u{9C60}', false)), // East Asian ideograph
    (0x215957, ('\u{8C4E}', false)), // East Asian ideograph
    (0x235958, ('\u{9C6D}', false)), // East Asian ideograph
    (0x215959, ('\u{8C54}', false)), // East Asian ideograph
    (0x21595A, ('\u{8C5A}', false)), // East Asian ideograph
    (0x23595B, ('\u{9C68}', false)), // East Asian ideograph
    (0x22595C, ('\u{7407}', false)), // East Asian ideograph
    (0x21595D, ('\u{8C6A}', false)), // East Asian ideograph
    (0x22595E, ('\u{7412}', false)), // East Asian ideograph
    (0x21595F, ('\u{8C6C}', false)), // East Asian ideograph
    (0x215960, ('\u{8C7A}', false)), // East Asian ideograph
    (0x215961, ('\u{8C79}', false)), // East Asian ideograph
    (0x215962, ('\u{8C82}', false)), // East Asian ideograph
    (0x225963, ('\u{743C}', false)), // East Asian ideograph
    (0x215964, ('\u{8C89}', false)), // East Asian ideograph
    (0x215965, ('\u{8C8D}', false)), // East Asian ideograph
    (0x225966, ('\u{742E}', false)), // East Asian ideograph
    (0x225967, ('\u{742F}', false)), // East Asian ideograph
    (0x275968, ('\u{8D1D}', false)), // East Asian ideograph
    (0x225969, ('\u{7414}', false)), // East Asian ideograph
    (0x22596A, ('\u{742C}', false)), // East Asian ideograph
    (0x27596B, ('\u{8D21}', false)), // East Asian ideograph
    (0x27596C, ('\u{8D22}', false)), // East Asian ideograph
    (0x27596D, ('\u{8D23}', false)), // East Asian ideograph
    (0x22596E, ('\u{742B}', false)), // East Asian ideograph
    (0x21596F, ('\u{8CA8}', false)), // East Asian ideograph
    (0x225970, ('\u{73F7}', false)), // East Asian ideograph
    (0x225971, ('\u{741A}', false)), // East Asian ideograph
    (0x275972, ('\u{8D29}', false)), // East Asian ideograph
    (0x235973, ('\u{9CE7}', false)), // East Asian ideograph
    (0x235974, ('\u{9CF0}', false)), // East Asian ideograph
    (0x215975, ('\u{8CBB}', false)), // East Asian ideograph
    (0x215976, ('\u{8CC1}', false)), // East Asian ideograph
    (0x235977, ('\u{9CF2}', false)), // East Asian ideograph
    (0x225978, ('\u{7416}', false)), // East Asian ideograph
    (0x215979, ('\u{8CBC}', false)), // East Asian ideograph
    (0x22597A, ('\u{7426}', false)), // East Asian ideograph
    (0x21597B, ('\u{8CB6}', false)), // East Asian ideograph
    (0x21597C, ('\u{8CBD}', false)), // East Asian ideograph
    (0x27597D, ('\u{8D37}', false)), // East Asian ideograph
    (0x21597E, ('\u{8CBF}', false)), // East Asian ideograph
    (0x222441, ('\u{5CF4}', false)), // East Asian ideograph
    (0x224F2B, ('\u{701E}', false)), // East Asian ideograph (variant of 4C4F2B which maps to 701E)
    (0x6F587E, ('\u{CC28}', false)), // Korean hangul
    (0x275E58, ('\u{9602}', false)), // East Asian ideograph
    (0x6F543B, ('\u{C315}', false)), // Korean hangul
    (0x217E52, ('\u{5BA7}', false)), // East Asian ideograph
    (0x333573, ('\u{8656}', false)), // East Asian ideograph
    (0x222446, ('\u{5CF1}', false)), // East Asian ideograph
    (0x69243F, ('\u{305F}', false)), // Hiragana letter TA
    (0x27613E, ('\u{9A8F}', false)), // East Asian ideograph
    (0x6F5B69, ('\u{D305}', false)), // Korean hangul
    (0x2D5C2F, ('\u{8FE8}', false)), // East Asian ideograph
    (0x2D4C3E, ('\u{758E}', false)), // East Asian ideograph
    (0x6F4E74, ('\u{B7AB}', false)), // Korean hangul
    (0x6F7655, ('\u{E8C7}', false)), // Korean hangul
    (0x225F7B, ('\u{7657}', false)), // East Asian ideograph
    (0x213E64, ('\u{60D5}', false)), // East Asian ideograph
    (0x234662, ('\u{93F9}', false)), // East Asian ideograph
    (0x696449, ('\u{7C13}', false)), // East Asian ideograph
    (0x276137, ('\u{9A76}', false)), // East Asian ideograph
    (0x69244A, ('\u{306A}', false)), // Hiragana letter NA
    (0x4B6147, ('\u{99C6}', false)), // East Asian ideograph
    (0x333556, ('\u{9A03}', false)), // East Asian ideograph
    (0x69644C, ('\u{7C17}', false)), // East Asian ideograph
    (0x6F4D66, ('\u{B4E4}', false)), // Korean hangul
    (0x213E65, ('\u{60BC}', false)), // East Asian ideograph
    (0x69644E, ('\u{7BF6}', false)), // East Asian ideograph
    (0x6F587B, ('\u{CC27}', false)), // Korean hangul
    (0x2D3B33, ('\u{8A67}', false)), // East Asian ideograph
    (0x214B2F, ('\u{7380}', false)), // East Asian ideograph
    (0x6F5438, ('\u{C30D}', false)), // Korean hangul
    (0x6F543E, ('\u{C324}', false)), // Korean hangul
    (0x6F7651, ('\u{E8C3}', false)), // Korean hangul
    (0x216452, ('\u{4EB3}', false)), // East Asian ideograph
    (0x294E43, ('\u{97AF}', false)), // East Asian ideograph
    (0x234664, ('\u{93C4}', false)), // East Asian ideograph
    (0x342453, ('\u{5CBD}', false)), // East Asian ideograph
    (0x692454, ('\u{3074}', false)), // Hiragana letter PI
    (0x225372, ('\u{71BA}', false)), // East Asian ideograph
    (0x4B6455, ('\u{4EB6}', false)), // East Asian ideograph
    (0x6F4F7A, ('\u{B9F7}', false)), // Korean hangul
    (0x6F543F, ('\u{C327}', false)), // Korean hangul
    (0x4B503B, ('\u{7C12}', false)), // East Asian ideograph
    (0x692457, ('\u{3077}', false)), // Hiragana letter PU
    (0x223E23, ('\u{691A}', false)), // East Asian ideograph
    (0x234222, ('\u{91E4}', false)), // East Asian ideograph
    (0x692459, ('\u{3079}', false)), // Hiragana letter BE
    (0x21645A, ('\u{4EBC}', false)), // East Asian ideograph
    (0x4B4444, ('\u{8988}', false)), // East Asian ideograph (Version J extension)
    (0x215A21, ('\u{8CC5}', false)), // East Asian ideograph
    (0x275A22, ('\u{8D44}', false)), // East Asian ideograph
    (0x275A23, ('\u{8D3C}', false)), // East Asian ideograph
    (0x215A24, ('\u{8CC8}', false)), // East Asian ideograph
    (0x275A25, ('\u{8D3F}', false)), // East Asian ideograph
    (0x215A26, ('\u{8CB2}', false)), // East Asian ideograph
    (0x275A27, ('\u{8D41}', false)), // East Asian ideograph
    (0x225A28, ('\u{7420}', false)), // East Asian ideograph
    (0x275A29, ('\u{5BBE}', false)), // East Asian ideograph
    (0x275A2A, ('\u{8D48}', false)), // East Asian ideograph
    (0x275A2B, ('\u{8D4A}', false)), // East Asian ideograph
    (0x215A2C, ('\u{8CE0}', false)), // East Asian ideograph
    (0x275A2D, ('\u{8D4B}', false)), // East Asian ideograph
    (0x6F5A2E, ('\u{CF00}', false)), // Korean hangul
    (0x275A2F, ('\u{5356}', false)), // East Asian ideograph
    (0x235A30, ('\u{9D25}', false)), // East Asian ideograph
    (0x215A31, ('\u{8CE4}', false)), // East Asian ideograph
    (0x215A32, ('\u{8CDE}', false)), // East Asian ideograph
    (0x275A33, ('\u{8D50}', false)), // East Asian ideograph
    (0x215A34, ('\u{8CEA}', false)), // East Asian ideograph
    (0x275A35, ('\u{8D4C}', false)), // East Asian ideograph
    (0x215A36, ('\u{8CF4}', false)), // East Asian ideograph
    (0x215A37, ('\u{8CFD}', false)), // East Asian ideograph
    (0x275A38, ('\u{8D5A}', false)), // East Asian ideograph
    (0x275A39, ('\u{8D58}', false)), // East Asian ideograph
    (0x275A3A, ('\u{8D2D}', false)), // East Asian ideograph
    (0x275A3B, ('\u{8D60}', false)), // East Asian ideograph
    (0x275A3C, ('\u{8D5D}', false)), // East Asian ideograph
    (0x275A3D, ('\u{8D5E}', false)), // East Asian ideograph
    (0x275A3E, ('\u{8D62}', false)), // East Asian ideograph
    (0x275A3F, ('\u{8D61}', false)), // East Asian ideograph
    (0x275A40, ('\u{8D43}', false)), // East Asian ideograph
    (0x275A41, ('\u{8D4E}', false)), // East Asian ideograph
    (0x275A42, ('\u{8D63}', false)), // East Asian ideograph
    (0x215A43, ('\u{8D64}', false)), // East Asian ideograph
    (0x215A44, ('\u{8D67}', false)), // East Asian ideograph
    (0x215A45, ('\u{8D66}', false)), // East Asian ideograph
    (0x215A46, ('\u{8D6B}', false)), // East Asian ideograph
    (0x215A47, ('\u{8D6D}', false)), // East Asian ideograph
    (0x235A48, ('\u{9D1F}', false)), // East Asian ideograph
    (0x215A49, ('\u{8D74}', false)), // East Asian ideograph
    (0x215A4A, ('\u{8D73}', false)), // East Asian ideograph
    (0x215A4B, ('\u{8D77}', false)), // East Asian ideograph
    (0x215A4C, ('\u{8D85}', false)), // East Asian ideograph
    (0x215A4D, ('\u{8D8A}', false)), // East Asian ideograph
    (0x215A4E, ('\u{8D81}', false)), // East Asian ideograph
    (0x275A4F, ('\u{8D75}', false)), // East Asian ideograph
    (0x215A50, ('\u{8D95}', false)), // East Asian ideograph
    (0x215A51, ('\u{8DA3}', false)), // East Asian ideograph
    (0x215A52, ('\u{8D9F}', false)), // East Asian ideograph
    (0x275A53, ('\u{8D8B}', false)), // East Asian ideograph
    (0x215A54, ('\u{8DB3}', false)), // East Asian ideograph
    (0x215A55, ('\u{8DB4}', false)), // East Asian ideograph
    (0x215A56, ('\u{8DBE}', false)), // East Asian ideograph
    (0x215A57, ('\u{8DCE}', false)), // East Asian ideograph
    (0x215A58, ('\u{8DDD}', false)), // East Asian ideograph
    (0x214B33, ('\u{738B}', false)), // East Asian ideograph
    (0x215A5A, ('\u{8DCB}', false)), // East Asian ideograph
    (0x215A5B, ('\u{8DDA}', false)), // East Asian ideograph
    (0x215A5C, ('\u{8DC6}', false)), // East Asian ideograph
    (0x215A5D, ('\u{8DD1}', false)), // East Asian ideograph
    (0x215A5E, ('\u{8DCC}', false)), // East Asian ideograph
    (0x215A5F, ('\u{8DE1}', false)), // East Asian ideograph
    (0x215A60, ('\u{8DDF}', false)), // East Asian ideograph
    (0x215A61, ('\u{8DE8}', false)), // East Asian ideograph
    (0x225A62, ('\u{7473}', false)), // East Asian ideograph
    (0x235A63, ('\u{9D3E}', false)), // East Asian ideograph
    (0x215A64, ('\u{8DEA}', false)), // East Asian ideograph
    (0x215A65, ('\u{8DEF}', false)), // East Asian ideograph
    (0x215A66, ('\u{8DFC}', false)), // East Asian ideograph
    (0x215A67, ('\u{8E2B}', false)), // East Asian ideograph
    (0x235A68, ('\u{9D42}', false)), // East Asian ideograph
    (0x235A69, ('\u{9D40}', false)), // East Asian ideograph
    (0x215A6A, ('\u{8E1D}', false)), // East Asian ideograph
    (0x215A6B, ('\u{8E0F}', false)), // East Asian ideograph
    (0x215A6C, ('\u{8E29}', false)), // East Asian ideograph
    (0x215A6D, ('\u{8E1F}', false)), // East Asian ideograph
    (0x215A6E, ('\u{8E44}', false)), // East Asian ideograph
    (0x215A6F, ('\u{8E31}', false)), // East Asian ideograph
    (0x215A70, ('\u{8E42}', false)), // East Asian ideograph
    (0x215A71, ('\u{8E34}', false)), // East Asian ideograph
    (0x215A72, ('\u{8E39}', false)), // East Asian ideograph
    (0x215A73, ('\u{8E35}', false)), // East Asian ideograph
    (0x215A74, ('\u{8E49}', false)), // East Asian ideograph
    (0x235A75, ('\u{9D53}', false)), // East Asian ideograph
    (0x215A76, ('\u{8E48}', false)), // East Asian ideograph
    (0x215A77, ('\u{8E4A}', false)), // East Asian ideograph
    (0x215A78, ('\u{8E63}', false)), // East Asian ideograph
    (0x215A79, ('\u{8E59}', false)), // East Asian ideograph
    (0x215A7A, ('\u{8E66}', false)), // East Asian ideograph
    (0x215A7B, ('\u{8E64}', false)), // East Asian ideograph
    (0x215A7C, ('\u{8E72}', false)), // East Asian ideograph
    (0x215A7D, ('\u{8E6C}', false)), // East Asian ideograph
    (0x275A7E, ('\u{8DF7}', false)), // East Asian ideograph
    (0x6F5439, ('\u{C313}', false)), // Korean hangul
    (0x6F5443, ('\u{C343}', false)), // Korean hangul
    (0x22646B, ('\u{789A}', false)), // East Asian ideograph
    (0x213A28, ('\u{5A41}', false)), // East Asian ideograph
    (0x6F2458, ('\u{3134}', false)), // Korean hangul
    (0x234226, ('\u{922B}', false)), // East Asian ideograph
    (0x27515C, ('\u{7EBF}', false)), // East Asian ideograph
    (0x475222, ('\u{9957}', false)), // East Asian ideograph
    (0x6F246E, ('\u{3143}', false)), // Korean hangul
    (0x335333, ('\u{80BB}', false)), // East Asian ideograph
    (0x6F2470, ('\u{3146}', false)), // Korean hangul
    (0x692471, ('\u{3091}', false)), // Hiragana letter WE
    (0x232472, ('\u{852F}', false)), // East Asian ideograph
    (0x6F2473, ('\u{3150}', false)), // Korean hangul
    (0x224F35, ('\u{7021}', false)), // East Asian ideograph
    (0x6F2474, ('\u{3151}', false)), // Korean hangul
    (0x6F5445, ('\u{C368}', false)), // Korean hangul
    (0x6F5A31, ('\u{CF08}', false)), // Korean hangul
    (0x6F2476, ('\u{3153}', false)), // Korean hangul
    (0x295729, ('\u{9C87}', false)), // East Asian ideograph
    (0x6F4F21, ('\u{B7FD}', false)), // Korean hangul
    (0x232477, ('\u{84F7}', false)), // East Asian ideograph
    (0x274F22, ('\u{4E07}', false)), // East Asian ideograph
    (0x234F23, ('\u{980E}', false)), // East Asian ideograph
    (0x6F4E42, ('\u{B611}', false)), // Korean hangul
    (0x224F24, ('\u{7020}', false)), // East Asian ideograph
    (0x6F5446, ('\u{C369}', false)), // Korean hangul
    (0x6F247A, ('\u{3157}', false)), // Korean hangul
    (0x274F25, ('\u{53B6}', false)), // East Asian ideograph
    (0x4B333E, ('\u{F92E}', false)), // East Asian ideograph
    (0x224F26, ('\u{7027}', false)), // East Asian ideograph
    (0x214F27, ('\u{79C9}', false)), // East Asian ideograph
    (0x22537A, ('\u{71BF}', false)), // East Asian ideograph
    (0x22537C, ('\u{71B8}', false)), // East Asian ideograph
    (0x29247D, ('\u{835C}', false)), // East Asian ideograph
    (0x6F4F28, ('\u{B80C}', false)), // Korean hangul
    (0x212A38, ('\u{E8E5}', false)), // EACC component character
    (0x276775, ('\u{4F65}', false)), // East Asian ideograph
    (0x6F5447, ('\u{C36C}', false)), // Korean hangul
    (0x6F4F2A, ('\u{B818}', false)), // Korean hangul
    (0x23422A, ('\u{9292}', false)), // East Asian ideograph
    (0x2D5D56, ('\u{920E}', false)), // East Asian ideograph
    (0x234F2C, ('\u{9826}', false)), // East Asian ideograph
    (0x6F5D5A, ('\u{D6FC}', false)), // Korean hangul
    (0x234F2D, ('\u{981E}', false)), // East Asian ideograph
    (0x6F7652, ('\u{E8C4}', false)), // Korean hangul
    (0x6F4F2E, ('\u{B824}', false)), // Korean hangul
    (0x2D3C26, ('\u{5D18}', false)), // East Asian ideograph
    (0x6F5448, ('\u{C370}', false)), // Korean hangul
    (0x213E70, ('\u{60FB}', false)), // East Asian ideograph
    (0x224F2F, ('\u{702E}', false)), // East Asian ideograph
    (0x225B21, ('\u{7489}', false)), // East Asian ideograph
    (0x225B22, ('\u{747C}', false)), // East Asian ideograph
    (0x215B23, ('\u{8E82}', false)), // East Asian ideograph
    (0x215B24, ('\u{8E81}', false)), // East Asian ideograph
    (0x215B25, ('\u{8E87}', false)), // East Asian ideograph
    (0x215B26, ('\u{8E89}', false)), // East Asian ideograph
    (0x214F31, ('\u{79FB}', false)), // East Asian ideograph
    (0x225B28, ('\u{747E}', false)), // East Asian ideograph
    (0x275B29, ('\u{8DC3}', false)), // East Asian ideograph
    (0x235B2A, ('\u{9D52}', false)), // East Asian ideograph
    (0x215B2B, ('\u{8EA1}', false)), // East Asian ideograph
    (0x235B2C, ('\u{9D77}', false)), // East Asian ideograph
    (0x224F39, ('\u{7018}', false)), // East Asian ideograph
    (0x215B2E, ('\u{8EAC}', false)), // East Asian ideograph
    (0x215B2F, ('\u{8EB2}', false)), // East Asian ideograph
    (0x225B30, ('\u{747A}', false)), // East Asian ideograph
    (0x215B31, ('\u{8EC0}', false)), // East Asian ideograph
    (0x215B32, ('\u{8ECA}', false)), // East Asian ideograph
    (0x275B33, ('\u{8F67}', false)), // East Asian ideograph
    (0x215B34, ('\u{8ECD}', false)), // East Asian ideograph
    (0x215B35, ('\u{8ECC}', false)), // East Asian ideograph
    (0x215B36, ('\u{8ED2}', false)), // East Asian ideograph
    (0x215B37, ('\u{8ED4}', false)), // East Asian ideograph
    (0x215B38, ('\u{8EDF}', false)), // East Asian ideograph
    (0x215B39, ('\u{8EDB}', false)), // East Asian ideograph
    (0x215B3A, ('\u{8EFB}', false)), // East Asian ideograph
    (0x275B3B, ('\u{8F74}', false)), // East Asian ideograph
    (0x215B3C, ('\u{8EFC}', false)), // East Asian ideograph
    (0x215B3D, ('\u{8F03}', false)), // East Asian ideograph
    (0x225B3E, ('\u{747D}', false)), // East Asian ideograph
    (0x235B3F, ('\u{9D78}', false)), // East Asian ideograph
    (0x215B40, ('\u{8F0A}', false)), // East Asian ideograph
    (0x215B41, ('\u{8F14}', false)), // East Asian ideograph
    (0x235B42, ('\u{9D7E}', false)), // East Asian ideograph
    (0x215B43, ('\u{8F15}', false)), // East Asian ideograph
    (0x215B44, ('\u{8F13}', false)), // East Asian ideograph
    (0x215B45, ('\u{8F26}', false)), // East Asian ideograph
    (0x215B46, ('\u{8F1B}', false)), // East Asian ideograph
    (0x235B47, ('\u{9D69}', false)), // East Asian ideograph
    (0x215B48, ('\u{8F1D}', false)), // East Asian ideograph
    (0x215B49, ('\u{8F29}', false)), // East Asian ideograph
    (0x215B4A, ('\u{8F2A}', false)), // East Asian ideograph
    (0x275B4B, ('\u{8F8E}', false)), // East Asian ideograph
    (0x215B4C, ('\u{8F3B}', false)), // East Asian ideograph
    (0x215B4D, ('\u{8F2F}', false)), // East Asian ideograph
    (0x215B4E, ('\u{8F38}', false)), // East Asian ideograph
    (0x235B4F, ('\u{9D83}', false)), // East Asian ideograph
    (0x215B50, ('\u{8F3E}', false)), // East Asian ideograph
    (0x215B51, ('\u{8F45}', false)), // East Asian ideograph
    (0x215B52, ('\u{8F42}', false)), // East Asian ideograph (variant of 4B5B52 which maps to 8F42)
    (0x215B53, ('\u{8F3F}', false)), // East Asian ideograph
    (0x225B54, ('\u{749F}', false)), // East Asian ideograph
    (0x225B55, ('\u{749D}', false)), // East Asian ideograph
    (0x215B56, ('\u{8F54}', false)), // East Asian ideograph
    (0x225B57, ('\u{749E}', false)), // East Asian ideograph
    (0x215B58, ('\u{8F5F}', false)), // East Asian ideograph
    (0x215B59, ('\u{8F61}', false)), // East Asian ideograph
    (0x215B5A, ('\u{8F9B}', false)), // East Asian ideograph
    (0x215B5B, ('\u{8F9C}', false)), // East Asian ideograph
    (0x215B5C, ('\u{8F9F}', false)), // East Asian ideograph
    (0x224F3A, ('\u{7023}', false)), // East Asian ideograph
    (0x235B5E, ('\u{9D92}', false)), // East Asian ideograph
    (0x215B5F, ('\u{8FA6}', false)), // East Asian ideograph
    (0x225B60, ('\u{74B2}', false)), // East Asian ideograph
    (0x215B61, ('\u{8FAF}', false)), // East Asian ideograph
    (0x215B62, ('\u{8FB0}', false)), // East Asian ideograph
    (0x215B63, ('\u{8FB1}', false)), // East Asian ideograph
    (0x215B64, ('\u{8FB2}', false)), // East Asian ideograph
    (0x295E6A, ('\u{9EEA}', false)), // East Asian ideograph
    (0x225B66, ('\u{74B4}', false)), // East Asian ideograph
    (0x225B67, ('\u{74AB}', false)), // East Asian ideograph
    (0x215B68, ('\u{8FC4}', false)), // East Asian ideograph
    (0x215B69, ('\u{5DE1}', false)), // East Asian ideograph
    (0x215B6A, ('\u{8FCE}', false)), // East Asian ideograph
    (0x215B6B, ('\u{8FD1}', false)), // East Asian ideograph
    (0x215B6C, ('\u{8FD4}', false)), // East Asian ideograph
    (0x275B6D, ('\u{8FF0}', false)), // East Asian ideograph
    (0x215B6E, ('\u{8FE6}', false)), // East Asian ideograph
    (0x215B6F, ('\u{8FE2}', false)), // East Asian ideograph
    (0x235B70, ('\u{9D96}', false)), // East Asian ideograph
    (0x215B71, ('\u{8FE5}', false)), // East Asian ideograph
    (0x6F544B, ('\u{C379}', false)), // Korean hangul
    (0x215B73, ('\u{8FEB}', false)), // East Asian ideograph
    (0x215B74, ('\u{9001}', false)), // East Asian ideograph
    (0x215B75, ('\u{9006}', false)), // East Asian ideograph
    (0x225B76, ('\u{74B8}', false)), // East Asian ideograph
    (0x215B77, ('\u{9000}', false)), // East Asian ideograph
    (0x6F5B78, ('\u{D329}', false)), // Korean hangul
    (0x235B79, ('\u{9DC0}', false)), // East Asian ideograph
    (0x225B7A, ('\u{74C0}', false)), // East Asian ideograph
    (0x23422E, ('\u{9207}', false)), // East Asian ideograph
    (0x215B7C, ('\u{9005}', false)), // East Asian ideograph
    (0x215B7D, ('\u{9019}', false)), // East Asian ideograph
    (0x215B7E, ('\u{9023}', false)), // East Asian ideograph
    (0x214F40, ('\u{7A40}', false)), // East Asian ideograph
    (0x393C52, ('\u{8D26}', false)), // East Asian ideograph
    (0x224F41, ('\u{703C}', false)), // East Asian ideograph
    (0x213941, ('\u{596E}', false)), // East Asian ideograph
    (0x6F4F42, ('\u{B8E1}', false)), // Korean hangul
    (0x6F544C, ('\u{C37C}', false)), // Korean hangul
    (0x4B4F43, ('\u{7A32}', false)), // East Asian ideograph
    (0x213A31, ('\u{5A9B}', false)), // East Asian ideograph
    (0x224F44, ('\u{7035}', false)), // East Asian ideograph
    (0x213A41, ('\u{5AFB}', false)), // East Asian ideograph
    (0x234F45, ('\u{9832}', false)), // East Asian ideograph
    (0x294331, ('\u{94F1}', false)), // East Asian ideograph
    (0x274F46, ('\u{7A23}', false)), // East Asian ideograph
    (0x6F4F47, ('\u{B8F8}', false)), // Korean hangul
    (0x2D4F48, ('\u{7A42}', false)), // East Asian ideograph
    (0x213A32, ('\u{5ACC}', false)), // East Asian ideograph
    (0x294231, ('\u{94AF}', false)), // East Asian ideograph
    (0x214F49, ('\u{7A61}', false)), // East Asian ideograph
    (0x4C3744, ('\u{65D9}', false)), // East Asian ideograph
    (0x274F4A, ('\u{79FD}', false)), // East Asian ideograph
    (0x214F4B, ('\u{7A6B}', false)), // East Asian ideograph
    (0x227631, ('\u{8008}', false)), // East Asian ideograph
    (0x274F4C, ('\u{7A33}', false)), // East Asian ideograph
    (0x6F544E, ('\u{C384}', false)), // Korean hangul
    (0x6F4F4D, ('\u{B958}', false)), // Korean hangul
    (0x6F5361, ('\u{C229}', false)), // Korean hangul
    (0x213A33, ('\u{5AC1}', false)), // East Asian ideograph
    (0x234F4E, ('\u{9844}', false)), // East Asian ideograph
    (0x4B393A, ('\u{5F09}', false)), // East Asian ideograph
    (0x6F4F4F, ('\u{B95C}', false)), // Korean hangul
    (0x334755, ('\u{6FEC}', false)), // East Asian ideograph
    (0x6F4F50, ('\u{B960}', false)), // Korean hangul
    (0x6F4C69, ('\u{B2DD}', false)), // Korean hangul
    (0x23533E, ('\u{9A0B}', false)), // East Asian ideograph
    (0x224F51, ('\u{7034}', false)), // East Asian ideograph
    (0x274564, ('\u{69DF}', false)), // East Asian ideograph
    (0x6F544F, ('\u{C388}', false)), // Korean hangul
    (0x213E77, ('\u{611B}', false)), // East Asian ideograph
    (0x6F4F52, ('\u{B96D}', false)), // Korean hangul
    (0x213A34, ('\u{5AC9}', false)), // East Asian ideograph
    (0x217E79, ('\u{5BE0}', false)), // East Asian ideograph
    (0x224F53, ('\u{7039}', false)), // East Asian ideograph
    (0x224F54, ('\u{703A}', false)), // East Asian ideograph
    (0x395E6F, ('\u{7A7D}', false)), // East Asian ideograph
    (0x6F4F55, ('\u{B978}', false)), // Korean hangul
    (0x23533F, ('\u{9A09}', false)), // East Asian ideograph
    (0x6F4E44, ('\u{B618}', false)), // Korean hangul
    (0x6F4F56, ('\u{B97C}', false)), // Korean hangul
    (0x6F5450, ('\u{C399}', false)), // Korean hangul
    (0x6F4F57, ('\u{B984}', false)), // Korean hangul
    (0x213A35, ('\u{5ABE}', false)), // East Asian ideograph
    (0x696B27, ('\u{8977}', false)), // East Asian ideograph
    (0x234233, ('\u{91FE}', false)), // East Asian ideograph
    (0x3A2F7C, ('\u{64C0}', false)), // East Asian ideograph
    (0x395A2F, ('\u{58F2}', false)), // East Asian ideograph
    (0x334F59, ('\u{7A93}', false)), // East Asian ideograph
    (0x293C57, ('\u{8F79}', false)), // East Asian ideograph
    (0x6F4F5A, ('\u{B989}', false)), // Korean hangul
    (0x215C21, ('\u{901F}', false)), // East Asian ideograph
    (0x215C22, ('\u{9017}', false)), // East Asian ideograph
    (0x215C23, ('\u{901D}', false)), // East Asian ideograph
    (0x215C24, ('\u{9010}', false)), // East Asian ideograph
    (0x225C25, ('\u{74BF}', false)), // East Asian ideograph
    (0x215C26, ('\u{900D}', false)), // East Asian ideograph
    (0x215C27, ('\u{901E}', false)), // East Asian ideograph
    (0x235C28, ('\u{9DBB}', false)), // East Asian ideograph
    (0x274123, ('\u{6302}', false)), // East Asian ideograph
    (0x215C2A, ('\u{900F}', false)), // East Asian ideograph
    (0x215C2B, ('\u{9022}', false)), // East Asian ideograph
    (0x215C2C, ('\u{9016}', false)), // East Asian ideograph
    (0x215C2D, ('\u{901B}', false)), // East Asian ideograph
    (0x215C2E, ('\u{9014}', false)), // East Asian ideograph
    (0x214F5D, ('\u{7AA9}', false)), // East Asian ideograph
    (0x215C30, ('\u{9035}', false)), // East Asian ideograph
    (0x215C31, ('\u{9031}', false)), // East Asian ideograph
    (0x235C32, ('\u{9DB9}', false)), // East Asian ideograph
    (0x275C33, ('\u{8FDB}', false)), // East Asian ideograph
    (0x275C34, ('\u{8FD0}', false)), // East Asian ideograph
    (0x2D4F5E, ('\u{7AB0}', false)), // East Asian ideograph
    (0x215C36, ('\u{9053}', false)), // East Asian ideograph
    (0x215C37, ('\u{9042}', false)), // East Asian ideograph
    (0x215C38, ('\u{9050}', false)), // East Asian ideograph
    (0x275C39, ('\u{8FBE}', false)), // East Asian ideograph
    (0x275C3A, ('\u{8FDD}', false)), // East Asian ideograph
    (0x274F5F, ('\u{7A77}', false)), // East Asian ideograph
    (0x275C3C, ('\u{8FC2}', false)), // East Asian ideograph
    (0x215C3D, ('\u{904F}', false)), // East Asian ideograph
    (0x235C3E, ('\u{9DD9}', false)), // East Asian ideograph
    (0x215C3F, ('\u{904D}', false)), // East Asian ideograph
    (0x215C40, ('\u{9051}', false)), // East Asian ideograph
    (0x214F60, ('\u{7ABA}', false)), // East Asian ideograph
    (0x215C42, ('\u{903E}', false)), // East Asian ideograph
    (0x215C43, ('\u{9058}', false)), // East Asian ideograph
    (0x275C44, ('\u{8FDC}', false)), // East Asian ideograph
    (0x275C45, ('\u{900A}', false)), // East Asian ideograph
    (0x215C46, ('\u{9063}', false)), // East Asian ideograph
    (0x214F61, ('\u{7AC5}', false)), // East Asian ideograph
    (0x275C48, ('\u{9012}', false)), // East Asian ideograph
    (0x215C49, ('\u{9069}', false)), // East Asian ideograph
    (0x215C4A, ('\u{906E}', false)), // East Asian ideograph
    (0x215C4B, ('\u{9068}', false)), // East Asian ideograph
    (0x215C4C, ('\u{906D}', false)), // East Asian ideograph
    (0x214F62, ('\u{7AC4}', false)), // East Asian ideograph
    (0x215C4E, ('\u{9074}', false)), // East Asian ideograph
    (0x275C4F, ('\u{9009}', false)), // East Asian ideograph
    (0x275C50, ('\u{8FDF}', false)), // East Asian ideograph
    (0x215C51, ('\u{9077}', false)), // East Asian ideograph
    (0x215C52, ('\u{907C}', false)), // East Asian ideograph
    (0x275C53, ('\u{9057}', false)), // East Asian ideograph
    (0x215C54, ('\u{907F}', false)), // East Asian ideograph
    (0x215C55, ('\u{907D}', false)), // East Asian ideograph
    (0x275C56, ('\u{8FC8}', false)), // East Asian ideograph
    (0x235C57, ('\u{9DF2}', false)), // East Asian ideograph
    (0x215C58, ('\u{9082}', false)), // East Asian ideograph
    (0x215C59, ('\u{9080}', false)), // East Asian ideograph
    (0x275C5A, ('\u{8FE9}', false)), // East Asian ideograph (variant of 2D5C5A which maps to 8FE9)
    (0x275C5B, ('\u{8FB9}', false)), // East Asian ideograph
    (0x275C5C, ('\u{9026}', false)), // East Asian ideograph
    (0x275C5D, ('\u{903B}', false)), // East Asian ideograph
    (0x215C5E, ('\u{9091}', false)), // East Asian ideograph
    (0x215C5F, ('\u{9095}', false)), // East Asian ideograph
    (0x215C60, ('\u{90A3}', false)), // East Asian ideograph
    (0x215C61, ('\u{90A2}', false)), // East Asian ideograph
    (0x215C62, ('\u{90AA}', false)), // East Asian ideograph
    (0x215C63, ('\u{90A6}', false)), // East Asian ideograph
    (0x215C64, ('\u{90B5}', false)), // East Asian ideograph
    (0x215C65, ('\u{90B1}', false)), // East Asian ideograph
    (0x215C66, ('\u{90B8}', false)), // East Asian ideograph
    (0x215C67, ('\u{90CE}', false)), // East Asian ideograph
    (0x215C68, ('\u{90CA}', false)), // East Asian ideograph
    (0x4B5564, ('\u{77C7}', false)), // East Asian ideograph
    (0x235C6A, ('\u{9DED}', false)), // East Asian ideograph
    (0x215C6B, ('\u{90E8}', false)), // East Asian ideograph
    (0x215C6C, ('\u{90ED}', false)), // East Asian ideograph
    (0x275C6D, ('\u{90AE}', false)), // East Asian ideograph
    (0x215C6E, ('\u{90FD}', false)), // East Asian ideograph
    (0x215C6F, ('\u{9102}', false)), // East Asian ideograph
    (0x275C70, ('\u{4E61}', false)), // East Asian ideograph
    (0x275C71, ('\u{90B9}', false)), // East Asian ideograph
    (0x215C72, ('\u{9119}', false)), // East Asian ideograph
    (0x275C73, ('\u{90D1}', false)), // East Asian ideograph
    (0x275C74, ('\u{90BB}', false)), // East Asian ideograph
    (0x275C75, ('\u{9093}', false)), // East Asian ideograph
    (0x215C76, ('\u{9131}', false)), // East Asian ideograph
    (0x214F69, ('\u{7AED}', false)), // East Asian ideograph
    (0x215C78, ('\u{9149}', false)), // East Asian ideograph
    (0x215C79, ('\u{914B}', false)), // East Asian ideograph
    (0x215C7A, ('\u{914A}', false)), // East Asian ideograph
    (0x215C7B, ('\u{9152}', false)), // East Asian ideograph
    (0x215C7C, ('\u{914D}', false)), // East Asian ideograph
    (0x215C7D, ('\u{914C}', false)), // East Asian ideograph
    (0x215C7E, ('\u{9157}', false)), // East Asian ideograph
    (0x6F5454, ('\u{C3DF}', false)), // Korean hangul
    (0x6F5A34, ('\u{CF13}', false)), // Korean hangul
    (0x214F6B, ('\u{7AF6}', false)), // East Asian ideograph
    (0x213A39, ('\u{5AB3}', false)), // East Asian ideograph
    (0x234237, ('\u{9226}', false)), // East Asian ideograph
    (0x6F4F6D, ('\u{B9D8}', false)), // Korean hangul
    (0x695A31, ('\u{64F6}', false)), // East Asian ideograph
    (0x6F4E45, ('\u{B625}', false)), // Korean hangul
    (0x234F6F, ('\u{9857}', false)), // East Asian ideograph
    (0x27456A, ('\u{6988}', false)), // East Asian ideograph
    (0x213E7D, ('\u{6108}', false)), // East Asian ideograph
    (0x395821, ('\u{97E4}', false)), // East Asian ideograph
    (0x274F70, ('\u{5DF4}', false)), // East Asian ideograph (duplicate simplified)
    (0x213A3A, ('\u{5AE1}', false)), // East Asian ideograph
    (0x224F71, ('\u{7052}', false)), // East Asian ideograph
    (0x234F72, ('\u{9856}', false)), // East Asian ideograph
    (0x295E7A, ('\u{9EFE}', false)), // East Asian ideograph
    (0x224F73, ('\u{705C}', false)), // East Asian ideograph
    (0x213F39, ('\u{6163}', false)), // East Asian ideograph
    (0x6F4F74, ('\u{B9E4}', false)), // Korean hangul
    (0x6F5456, ('\u{C3E8}', false)), // Korean hangul
    (0x213E7E, ('\u{60F1}', false)), // East Asian ideograph
    (0x214F75, ('\u{7B1B}', false)), // East Asian ideograph
    (0x213A3B, ('\u{5AD7}', false)), // East Asian ideograph
    (0x284E66, ('\u{6EE2}', false)), // East Asian ideograph
    (0x213A21, ('\u{5A46}', false)), // East Asian ideograph
    (0x234F77, ('\u{9862}', false)), // East Asian ideograph
    (0x275235, ('\u{7F62}', false)), // East Asian ideograph
    (0x224F78, ('\u{7059}', false)), // East Asian ideograph
    (0x213A23, ('\u{5A6A}', false)), // East Asian ideograph
    (0x274F79, ('\u{7B14}', false)), // East Asian ideograph
    (0x213A24, ('\u{5A36}', false)), // East Asian ideograph
    (0x6F5457, ('\u{C3ED}', false)), // Korean hangul
    (0x22427E, ('\u{6AED}', false)), // East Asian ideograph
    (0x214F7B, ('\u{7B50}', false)), // East Asian ideograph
    (0x213A26, ('\u{5A40}', false)), // East Asian ideograph
    (0x695E63, ('\u{6E82}', false)), // East Asian ideograph
    (0x275679, ('\u{80E1}', false)), // East Asian ideograph (duplicate simplified)
    (0x224F7C, ('\u{7061}', false)), // East Asian ideograph
    (0x213A27, ('\u{5A66}', false)), // East Asian ideograph
    (0x224F7D, ('\u{705D}', false)), // East Asian ideograph
    (0x223A28, ('\u{6705}', false)), // East Asian ideograph
    (0x335347, ('\u{81D9}', false)), // East Asian ideograph
    (0x293B4F, ('\u{8F78}', false)), // East Asian ideograph
    (0x234F7E, ('\u{9868}', false)), // East Asian ideograph
    (0x6F4F7B, ('\u{B9F8}', false)), // Korean hangul
    (0x6F5458, ('\u{C3F4}', false)), // Korean hangul
    (0x6F5363, ('\u{C22D}', false)), // Korean hangul
    (0x2D5D68, ('\u{8021}', false)), // East Asian ideograph
    (0x394928, ('\u{6D5C}', false)), // East Asian ideograph
    (0x29366A, ('\u{8D53}', false)), // East Asian ideograph
    (0x227A2C, ('\u{81B5}', false)), // East Asian ideograph
    (0x223173, ('\u{637F}', false)), // East Asian ideograph
    (0x2D5179, ('\u{7E62}', false)), // East Asian ideograph
    (0x213A2E, ('\u{5A92}', false)), // East Asian ideograph
    (0x6F5459, ('\u{C3F5}', false)), // Korean hangul
    (0x2D3A2F, ('\u{58FB}', false)), // East Asian ideograph
    (0x4B3351, ('\u{5204}', false)), // East Asian ideograph
    (0x215D21, ('\u{9163}', false)), // East Asian ideograph
    (0x215D22, ('\u{9165}', false)), // East Asian ideograph
    (0x215D23, ('\u{916C}', false)), // East Asian ideograph
    (0x215D24, ('\u{9169}', false)), // East Asian ideograph
    (0x215D25, ('\u{916A}', false)), // East Asian ideograph
    (0x215D26, ('\u{9175}', false)), // East Asian ideograph
    (0x215D27, ('\u{9178}', false)), // East Asian ideograph
    (0x215D28, ('\u{9177}', false)), // East Asian ideograph
    (0x215D29, ('\u{9187}', false)), // East Asian ideograph
    (0x215D2A, ('\u{9189}', false)), // East Asian ideograph
    (0x215D2B, ('\u{918B}', false)), // East Asian ideograph
    (0x215D2C, ('\u{9183}', false)), // East Asian ideograph
    (0x215D2D, ('\u{9192}', false)), // East Asian ideograph
    (0x215D2E, ('\u{91A3}', false)), // East Asian ideograph
    (0x275D2F, ('\u{915D}', false)), // East Asian ideograph
    (0x215D30, ('\u{919C}', false)), // East Asian ideograph
    (0x275D31, ('\u{533B}', false)), // East Asian ideograph
    (0x225D32, ('\u{7512}', false)), // East Asian ideograph
    (0x215D33, ('\u{91BA}', false)), // East Asian ideograph
    (0x275D34, ('\u{917F}', false)), // East Asian ideograph
    (0x275D35, ('\u{8845}', false)), // East Asian ideograph
    (0x215D36, ('\u{91C7}', false)), // East Asian ideograph
    (0x215D37, ('\u{91C9}', false)), // East Asian ideograph
    (0x215D38, ('\u{91CB}', false)), // East Asian ideograph
    (0x235D39, ('\u{9E1C}', false)), // East Asian ideograph
    (0x235D3A, ('\u{9E1B}', false)), // East Asian ideograph
    (0x215D3B, ('\u{91CE}', false)), // East Asian ideograph
    (0x235D3C, ('\u{9E75}', false)), // East Asian ideograph
    (0x275D3D, ('\u{5398}', false)), // East Asian ideograph
    (0x215D3E, ('\u{91D1}', false)), // East Asian ideograph
    (0x215D3F, ('\u{91DD}', false)), // East Asian ideograph
    (0x215D40, ('\u{91D8}', false)), // East Asian ideograph
    (0x215D41, ('\u{91D7}', false)), // East Asian ideograph
    (0x235D42, ('\u{9E7A}', false)), // East Asian ideograph
    (0x215D43, ('\u{91F5}', false)), // East Asian ideograph
    (0x225D44, ('\u{7524}', false)), // East Asian ideograph
    (0x215D45, ('\u{91E3}', false)), // East Asian ideograph
    (0x215D46, ('\u{91E7}', false)), // East Asian ideograph
    (0x235D47, ('\u{9E80}', false)), // East Asian ideograph
    (0x215D48, ('\u{920D}', false)), // East Asian ideograph
    (0x215D49, ('\u{9215}', false)), // East Asian ideograph
    (0x215D4A, ('\u{9209}', false)), // East Asian ideograph
    (0x275D4B, ('\u{949E}', false)), // East Asian ideograph
    (0x215D4C, ('\u{921E}', false)), // East Asian ideograph
    (0x215D4D, ('\u{9210}', false)), // East Asian ideograph
    (0x2D4C5D, ('\u{7661}', false)), // East Asian ideograph
    (0x225D4F, ('\u{753F}', false)), // East Asian ideograph
    (0x215D50, ('\u{9238}', false)), // East Asian ideograph
    (0x225D51, ('\u{7540}', false)), // East Asian ideograph
    (0x225D52, ('\u{753E}', false)), // East Asian ideograph
    (0x215D53, ('\u{9240}', false)), // East Asian ideograph
    (0x215D54, ('\u{924B}', false)), // East Asian ideograph
    (0x235D55, ('\u{9E90}', false)), // East Asian ideograph
    (0x215D56, ('\u{9264}', false)), // East Asian ideograph
    (0x215D57, ('\u{9251}', false)), // East Asian ideograph
    (0x235D58, ('\u{9E8C}', false)), // East Asian ideograph
    (0x215D59, ('\u{9278}', false)), // East Asian ideograph
    (0x215D5A, ('\u{9280}', false)), // East Asian ideograph
    (0x275D5B, ('\u{94D0}', false)), // East Asian ideograph
    (0x215D5C, ('\u{9285}', false)), // East Asian ideograph
    (0x235D5D, ('\u{9E9B}', false)), // East Asian ideograph
    (0x215D5E, ('\u{9296}', false)), // East Asian ideograph
    (0x225D5F, ('\u{755F}', false)), // East Asian ideograph
    (0x215D60, ('\u{9293}', false)), // East Asian ideograph
    (0x275D61, ('\u{8854}', false)), // East Asian ideograph
    (0x215D62, ('\u{92C5}', false)), // East Asian ideograph
    (0x215D63, ('\u{92BB}', false)), // East Asian ideograph
    (0x275D64, ('\u{9510}', false)), // East Asian ideograph
    (0x275D65, ('\u{94FA}', false)), // East Asian ideograph
    (0x275D66, ('\u{9500}', false)), // East Asian ideograph
    (0x215D67, ('\u{92C1}', false)), // East Asian ideograph
    (0x215D68, ('\u{92E4}', false)), // East Asian ideograph
    (0x215D69, ('\u{92BC}', false)), // East Asian ideograph
    (0x215D6A, ('\u{92D2}', false)), // East Asian ideograph
    (0x225D6B, ('\u{756C}', false)), // East Asian ideograph
    (0x215D6C, ('\u{9336}', false)), // East Asian ideograph
    (0x275D6D, ('\u{952F}', false)), // East Asian ideograph
    (0x215D6E, ('\u{9333}', false)), // East Asian ideograph
    (0x215D6F, ('\u{932F}', false)), // East Asian ideograph
    (0x215D70, ('\u{9322}', false)), // East Asian ideograph
    (0x215D71, ('\u{92FC}', false)), // East Asian ideograph
    (0x215D72, ('\u{932B}', false)), // East Asian ideograph
    (0x215D73, ('\u{931A}', false)), // East Asian ideograph
    (0x215D74, ('\u{9304}', false)), // East Asian ideograph
    (0x213A3E, ('\u{5AE9}', false)), // East Asian ideograph
    (0x275D76, ('\u{9526}', false)), // East Asian ideograph
    (0x275D77, ('\u{9540}', false)), // East Asian ideograph
    (0x275D78, ('\u{9541}', false)), // East Asian ideograph
    (0x235D79, ('\u{9EAF}', false)), // East Asian ideograph
    (0x215D7A, ('\u{9365}', false)), // East Asian ideograph
    (0x213A3F, ('\u{5AD8}', false)), // East Asian ideograph
    (0x215D7C, ('\u{934B}', false)), // East Asian ideograph
    (0x215D7D, ('\u{9328}', false)), // East Asian ideograph
    (0x215D7E, ('\u{9370}', false)), // East Asian ideograph
    (0x4B5E3F, ('\u{922C}', false)), // East Asian ideograph
    (0x213A40, ('\u{5AE6}', false)), // East Asian ideograph
    (0x6F5367, ('\u{C234}', false)), // Korean hangul
    (0x233A41, ('\u{8E61}', false)), // East Asian ideograph
    (0x6F5825, ('\u{C990}', false)), // Korean hangul
    (0x6F545D, ('\u{C434}', false)), // Korean hangul
    (0x284971, ('\u{6D9E}', false)), // East Asian ideograph
    (0x224A32, ('\u{6DFC}', false)), // East Asian ideograph
    (0x227A43, ('\u{81D0}', false)), // East Asian ideograph
    (0x213A44, ('\u{5B0C}', false)), // East Asian ideograph
    (0x29366B, ('\u{8D55}', false)), // East Asian ideograph
    (0x233A45, ('\u{8E74}', false)), // East Asian ideograph
    (0x213A46, ('\u{5B34}', false)), // East Asian ideograph
    (0x213A47, ('\u{5B1D}', false)), // East Asian ideograph
    (0x6F545E, ('\u{C43C}', false)), // Korean hangul
    (0x6F5A36, ('\u{CF1C}', false)), // Korean hangul
    (0x273A48, ('\u{5AD4}', false)), // East Asian ideograph
    (0x213A43, ('\u{5B0B}', false)), // East Asian ideograph
    (0x395829, ('\u{69FB}', false)), // East Asian ideograph
    (0x6F4C3E, ('\u{B1D0}', false)), // Korean hangul
    (0x4B3A49, ('\u{5B37}', false)), // East Asian ideograph
    (0x2D3B54, ('\u{5C4A}', false)), // East Asian ideograph
    (0x6F5923, ('\u{CC2E}', false)), // Korean hangul
    (0x213A4A, ('\u{5B30}', false)), // East Asian ideograph
    (0x287855, ('\u{80EB}', false)), // East Asian ideograph
    (0x233A4B, ('\u{8E69}', false)), // East Asian ideograph
    (0x213A4C, ('\u{5B40}', false)), // East Asian ideograph
    (0x6F545F, ('\u{C43F}', false)), // Korean hangul
    (0x213A4D, ('\u{5B50}', false)), // East Asian ideograph
    (0x213A4E, ('\u{5B51}', false)), // East Asian ideograph
    (0x217A4F, ('\u{59EE}', false)), // East Asian ideograph
    (0x225B3F, ('\u{7485}', false)), // East Asian ideograph
    (0x295E7C, ('\u{9F0B}', false)), // East Asian ideograph
    (0x29456F, ('\u{9538}', false)), // East Asian ideograph
    (0x6F5460, ('\u{C464}', false)), // Korean hangul
    (0x233A52, ('\u{8E83}', false)), // East Asian ideograph
    (0x213A45, ('\u{5AF5}', false)), // East Asian ideograph
    (0x334243, ('\u{52B9}', false)), // East Asian ideograph
    (0x233A53, ('\u{8E84}', false)), // East Asian ideograph
    (0x2D592C, ('\u{8B01}', false)), // East Asian ideograph
    (0x294335, ('\u{94F5}', false)), // East Asian ideograph
    (0x4C3A55, ('\u{6741}', false)), // East Asian ideograph
    (0x4B623B, ('\u{9D12}', false)), // East Asian ideograph
    (0x217A56, ('\u{59FD}', false)), // East Asian ideograph
    (0x6F5461, ('\u{C465}', false)), // Korean hangul
    (0x213A57, ('\u{5B5F}', false)), // East Asian ideograph
    (0x6F5A39, ('\u{CF2C}', false)), // Korean hangul
    (0x213A58, ('\u{5B63}', false)), // East Asian ideograph
    (0x692544, ('\u{30C4}', false)), // Katakana letter TU
    (0x225622, ('\u{728D}', false)), // East Asian ideograph
    (0x215E21, ('\u{937E}', false)), // East Asian ideograph
    (0x275E22, ('\u{9524}', false)), // East Asian ideograph
    (0x275E23, ('\u{9539}', false)), // East Asian ideograph
    (0x215E24, ('\u{935B}', false)), // East Asian ideograph
    (0x275E25, ('\u{9551}', false)), // East Asian ideograph
    (0x215E26, ('\u{9394}', false)), // East Asian ideograph
    (0x275E27, ('\u{9547}', false)), // East Asian ideograph
    (0x275E28, ('\u{9501}', false)), // East Asian ideograph
    (0x215E29, ('\u{93A2}', false)), // East Asian ideograph
    (0x275E2A, ('\u{954D}', false)), // East Asian ideograph
    (0x275E2B, ('\u{955C}', false)), // East Asian ideograph
    (0x275E2C, ('\u{955D}', false)), // East Asian ideograph
    (0x215E2D, ('\u{93D6}', false)), // East Asian ideograph
    (0x275E2E, ('\u{955E}', false)), // East Asian ideograph
    (0x215E2F, ('\u{93DF}', false)), // East Asian ideograph
    (0x275E30, ('\u{94FF}', false)), // East Asian ideograph
    (0x275E31, ('\u{94FE}', false)), // East Asian ideograph
    (0x215E32, ('\u{93E2}', false)), // East Asian ideograph
    (0x215E33, ('\u{93DC}', false)), // East Asian ideograph
    (0x215E34, ('\u{93E4}', false)), // East Asian ideograph
    (0x225E35, ('\u{7598}', false)), // East Asian ideograph
    (0x215E36, ('\u{93CD}', false)), // East Asian ideograph
    (0x235E37, ('\u{9EC8}', false)), // East Asian ideograph
    (0x215E39, ('\u{9403}', false)), // East Asian ideograph
    (0x215E3A, ('\u{942E}', false)), // East Asian ideograph
    (0x225E3B, ('\u{75A3}', false)), // East Asian ideograph
    (0x215E3C, ('\u{9433}', false)), // East Asian ideograph
    (0x215E3D, ('\u{9435}', false)), // East Asian ideograph
    (0x215E3E, ('\u{943A}', false)), // East Asian ideograph
    (0x215E3F, ('\u{9438}', false)), // East Asian ideograph
    (0x215E40, ('\u{9432}', false)), // East Asian ideograph
    (0x223A60, ('\u{675D}', false)), // East Asian ideograph
    (0x215E42, ('\u{9451}', false)), // East Asian ideograph
    (0x215E43, ('\u{9444}', false)), // East Asian ideograph
    (0x215E44, ('\u{9463}', false)), // East Asian ideograph
    (0x215E45, ('\u{9460}', false)), // East Asian ideograph
    (0x215E46, ('\u{9472}', false)), // East Asian ideograph
    (0x215E47, ('\u{9470}', false)), // East Asian ideograph
    (0x215E48, ('\u{947E}', false)), // East Asian ideograph
    (0x215E49, ('\u{947C}', false)), // East Asian ideograph
    (0x235E4A, ('\u{9ED0}', false)), // East Asian ideograph
    (0x215E4B, ('\u{947F}', false)), // East Asian ideograph
    (0x215E4C, ('\u{9577}', false)), // East Asian ideograph
    (0x215E4D, ('\u{9580}', false)), // East Asian ideograph
    (0x215E4E, ('\u{9582}', false)), // East Asian ideograph
    (0x215E4F, ('\u{9583}', false)), // East Asian ideograph
    (0x215E50, ('\u{9589}', false)), // East Asian ideograph
    (0x215E51, ('\u{9594}', false)), // East Asian ideograph
    (0x215E52, ('\u{958F}', false)), // East Asian ideograph
    (0x235E53, ('\u{9EDA}', false)), // East Asian ideograph
    (0x215E54, ('\u{9591}', false)), // East Asian ideograph
    (0x235E55, ('\u{9EDF}', false)), // East Asian ideograph
    (0x215E56, ('\u{9592}', false)), // East Asian ideograph
    (0x215E57, ('\u{9598}', false)), // East Asian ideograph
    (0x215E58, ('\u{95A1}', false)), // East Asian ideograph
    (0x235E59, ('\u{9EE5}', false)), // East Asian ideograph
    (0x215E5A, ('\u{95A9}', false)), // East Asian ideograph
    (0x215E5B, ('\u{95A3}', false)), // East Asian ideograph
    (0x215E5C, ('\u{95A5}', false)), // East Asian ideograph
    (0x215E5D, ('\u{95A4}', false)), // East Asian ideograph
    (0x215E5E, ('\u{95B1}', false)), // East Asian ideograph
    (0x215E5F, ('\u{95AD}', false)), // East Asian ideograph
    (0x235E60, ('\u{9EEE}', false)), // East Asian ideograph
    (0x215E61, ('\u{95CA}', false)), // East Asian ideograph
    (0x215E62, ('\u{95CB}', false)), // East Asian ideograph
    (0x215E63, ('\u{95CC}', false)), // East Asian ideograph
    (0x215E64, ('\u{95C8}', false)), // East Asian ideograph
    (0x215E65, ('\u{95C6}', false)), // East Asian ideograph
    (0x235E66, ('\u{9EF0}', false)), // East Asian ideograph
    (0x215E67, ('\u{95D6}', false)), // East Asian ideograph
    (0x215E68, ('\u{95D0}', false)), // East Asian ideograph
    (0x215E69, ('\u{95DC}', false)), // East Asian ideograph
    (0x215E6A, ('\u{95E1}', false)), // East Asian ideograph
    (0x215E6B, ('\u{95E2}', false)), // East Asian ideograph
    (0x215E6C, ('\u{961C}', false)), // East Asian ideograph
    (0x215E6D, ('\u{9621}', false)), // East Asian ideograph
    (0x215E6E, ('\u{9632}', false)), // East Asian ideograph
    (0x215E6F, ('\u{9631}', false)), // East Asian ideograph
    (0x215E70, ('\u{962E}', false)), // East Asian ideograph
    (0x215E71, ('\u{962A}', false)), // East Asian ideograph
    (0x215E72, ('\u{9640}', false)), // East Asian ideograph
    (0x215E73, ('\u{963F}', false)), // East Asian ideograph
    (0x215E74, ('\u{963B}', false)), // East Asian ideograph
    (0x215E75, ('\u{9644}', false)), // East Asian ideograph
    (0x215E76, ('\u{9650}', false)), // East Asian ideograph
    (0x235E77, ('\u{9EFC}', false)), // East Asian ideograph
    (0x215E78, ('\u{964B}', false)), // East Asian ideograph
    (0x215E79, ('\u{964D}', false)), // East Asian ideograph
    (0x235E7A, ('\u{9EFD}', false)), // East Asian ideograph
    (0x215E7B, ('\u{9663}', false)), // East Asian ideograph
    (0x235E7C, ('\u{9EFF}', false)), // East Asian ideograph
    (0x215E7D, ('\u{9661}', false)), // East Asian ideograph
    (0x225E7E, ('\u{7603}', false)), // East Asian ideograph
    (0x6F5465, ('\u{C479}', false)), // Korean hangul
    (0x223A6B, ('\u{6763}', false)), // East Asian ideograph
    (0x223A6E, ('\u{6753}', false)), // East Asian ideograph
    (0x2D355C, ('\u{5434}', false)), // East Asian ideograph
    (0x213A6F, ('\u{5B98}', false)), // East Asian ideograph
    (0x6F5466, ('\u{C480}', false)), // Korean hangul
    (0x6F5440, ('\u{C328}', false)), // Korean hangul
    (0x293A70, ('\u{8E9C}', false)), // East Asian ideograph
    (0x213A4B, ('\u{5B38}', false)), // East Asian ideograph
    (0x294936, ('\u{95F3}', false)), // East Asian ideograph
    (0x215821, ('\u{896A}', false)), // East Asian ideograph
    (0x233A71, ('\u{8EA9}', false)), // East Asian ideograph
    (0x692524, ('\u{30A4}', false)), // Katakana letter I
    (0x213A72, ('\u{5BA5}', false)), // East Asian ideograph
    (0x6F595F, ('\u{CE04}', false)), // Korean hangul
    (0x2D3B52, ('\u{6EBA}', false)), // East Asian ideograph
    (0x223A75, ('\u{6793}', false)), // East Asian ideograph
    (0x23424A, ('\u{9216}', false)), // East Asian ideograph
    (0x6F2521, ('\u{315C}', false)), // Korean hangul
    (0x4B4767, ('\u{6DB5}', false)), // East Asian ideograph (variant of 214767 which maps to 6DB5)
    (0x28342C, ('\u{63BA}', false)), // East Asian ideograph
    (0x295A44, ('\u{9E32}', false)), // East Asian ideograph
    (0x223A78, ('\u{677C}', false)), // East Asian ideograph
    (0x692523, ('\u{30A3}', false)), // Katakana letter small I
    (0x292524, ('\u{848C}', false)), // East Asian ideograph
    (0x29322A, ('\u{8BB5}', false)), // East Asian ideograph
    (0x223A7A, ('\u{679F}', false)), // East Asian ideograph
    (0x226065, ('\u{76A4}', false)), // East Asian ideograph
    (0x23424B, ('\u{9211}', false)), // East Asian ideograph
    (0x4D472C, ('\u{952A}', false)), // East Asian ideograph
    (0x4D5934, ('\u{9CA6}', false)), // East Asian ideograph
    (0x213A7C, ('\u{5BAE}', false)), // East Asian ideograph
    (0x692527, ('\u{30A7}', false)), // Katakana letter small E
    (0x213D2E, ('\u{5EEC}', false)), // East Asian ideograph
    (0x233A7D, ('\u{8EB6}', false)), // East Asian ideograph
    (0x692528, ('\u{30A8}', false)), // Katakana letter E
    (0x6F5469, ('\u{C4D5}', false)), // Korean hangul
    (0x69252A, ('\u{30AA}', false)), // Katakana letter O
    (0x283B22, ('\u{4E2B}', false)), // East Asian ideograph
    (0x22652C, ('\u{7892}', false)), // East Asian ideograph
    (0x28342E, ('\u{63BC}', false)), // East Asian ideograph
    (0x235359, ('\u{9A2F}', false)), // East Asian ideograph
    (0x22252D, ('\u{5D47}', false)), // East Asian ideograph
    (0x2D4829, ('\u{51CF}', false)), // East Asian ideograph
    (0x6F5027, ('\u{BA4B}', false)), // Korean hangul
    (0x23252F, ('\u{84E7}', false)), // East Asian ideograph
    (0x215F21, ('\u{9664}', false)), // East Asian ideograph
    (0x215F22, ('\u{966A}', false)), // East Asian ideograph
    (0x215F23, ('\u{9673}', false)), // East Asian ideograph
    (0x215F24, ('\u{9678}', false)), // East Asian ideograph
    (0x215F25, ('\u{9675}', false)), // East Asian ideograph
    (0x215F26, ('\u{9672}', false)), // East Asian ideograph
    (0x215F27, ('\u{9676}', false)), // East Asian ideograph
    (0x215F28, ('\u{9677}', false)), // East Asian ideograph
    (0x215F29, ('\u{9674}', false)), // East Asian ideograph
    (0x215F2A, ('\u{9670}', false)), // East Asian ideograph
    (0x215F2B, ('\u{968A}', false)), // East Asian ideograph
    (0x215F2C, ('\u{968E}', false)), // East Asian ideograph
    (0x215F2D, ('\u{968B}', false)), // East Asian ideograph
    (0x215F2E, ('\u{967D}', false)), // East Asian ideograph
    (0x235F2F, ('\u{9F0F}', false)), // East Asian ideograph
    (0x215F30, ('\u{9686}', false)), // East Asian ideograph
    (0x235F31, ('\u{9F10}', false)), // East Asian ideograph
    (0x235F32, ('\u{9F12}', false)), // East Asian ideograph
    (0x235F33, ('\u{9F16}', false)), // East Asian ideograph
    (0x235F34, ('\u{9F17}', false)), // East Asian ideograph
    (0x215F35, ('\u{9695}', false)), // East Asian ideograph
    (0x215F36, ('\u{969C}', false)), // East Asian ideograph
    (0x235F37, ('\u{9F1A}', false)), // East Asian ideograph
    (0x215F38, ('\u{96A7}', false)), // East Asian ideograph
    (0x215F39, ('\u{96A8}', false)), // East Asian ideograph
    (0x215F3A, ('\u{96AA}', false)), // East Asian ideograph
    (0x215F3B, ('\u{96B1}', false)), // East Asian ideograph
    (0x215F3C, ('\u{96B4}', false)), // East Asian ideograph
    (0x215F3D, ('\u{96B8}', false)), // East Asian ideograph
    (0x225F3E, ('\u{7625}', false)), // East Asian ideograph
    (0x225F3F, ('\u{761A}', false)), // East Asian ideograph
    (0x215F40, ('\u{96C7}', false)), // East Asian ideograph
    (0x215F41, ('\u{96C6}', false)), // East Asian ideograph
    (0x215F42, ('\u{96C4}', false)), // East Asian ideograph
    (0x215F43, ('\u{96C1}', false)), // East Asian ideograph
    (0x215F44, ('\u{96C5}', false)), // East Asian ideograph
    (0x215F45, ('\u{96CD}', false)), // East Asian ideograph
    (0x215F46, ('\u{96CB}', false)), // East Asian ideograph
    (0x215F47, ('\u{96C9}', false)), // East Asian ideograph
    (0x215F48, ('\u{96CC}', false)), // East Asian ideograph
    (0x215F49, ('\u{96D5}', false)), // East Asian ideograph
    (0x215F4A, ('\u{96D6}', false)), // East Asian ideograph
    (0x215F4B, ('\u{96DC}', false)), // East Asian ideograph
    (0x215F4C, ('\u{96DE}', false)), // East Asian ideograph
    (0x215F4D, ('\u{96DB}', false)), // East Asian ideograph
    (0x215F4E, ('\u{96D9}', false)), // East Asian ideograph
    (0x215F4F, ('\u{96E2}', false)), // East Asian ideograph
    (0x225F50, ('\u{7622}', false)), // East Asian ideograph
    (0x225F51, ('\u{762F}', false)), // East Asian ideograph
    (0x215F52, ('\u{96EA}', false)), // East Asian ideograph
    (0x215F53, ('\u{96EF}', false)), // East Asian ideograph
    (0x215F54, ('\u{96F2}', false)), // East Asian ideograph
    (0x215F55, ('\u{96FB}', false)), // East Asian ideograph
    (0x215F56, ('\u{96F7}', false)), // East Asian ideograph
    (0x215F57, ('\u{96F9}', false)), // East Asian ideograph
    (0x215F58, ('\u{96F6}', false)), // East Asian ideograph
    (0x215F59, ('\u{9700}', false)), // East Asian ideograph
    (0x23424F, ('\u{92A2}', false)), // East Asian ideograph
    (0x215F5B, ('\u{9704}', false)), // East Asian ideograph
    (0x215F5C, ('\u{9709}', false)), // East Asian ideograph
    (0x215F5D, ('\u{9706}', false)), // East Asian ideograph
    (0x225F5E, ('\u{763B}', false)), // East Asian ideograph
    (0x215F5F, ('\u{970E}', false)), // East Asian ideograph
    (0x225F60, ('\u{763C}', false)), // East Asian ideograph
    (0x215F61, ('\u{970F}', false)), // East Asian ideograph
    (0x225F62, ('\u{7635}', false)), // East Asian ideograph
    (0x215F63, ('\u{9713}', false)), // East Asian ideograph
    (0x235F64, ('\u{9F3D}', false)), // East Asian ideograph
    (0x215F65, ('\u{971E}', false)), // East Asian ideograph
    (0x215F66, ('\u{972A}', false)), // East Asian ideograph
    (0x225F67, ('\u{7648}', false)), // East Asian ideograph
    (0x225F68, ('\u{764E}', false)), // East Asian ideograph
    (0x235F69, ('\u{9F41}', false)), // East Asian ideograph
    (0x225F6A, ('\u{7643}', false)), // East Asian ideograph
    (0x215F6B, ('\u{973D}', false)), // East Asian ideograph
    (0x215F6C, ('\u{973E}', false)), // East Asian ideograph
    (0x215F6D, ('\u{9744}', false)), // East Asian ideograph
    (0x215F6E, ('\u{9742}', false)), // East Asian ideograph
    (0x225F6F, ('\u{7649}', false)), // East Asian ideograph
    (0x215F70, ('\u{9751}', false)), // East Asian ideograph
    (0x215F71, ('\u{FA1C}', false)), // East Asian ideograph
    (0x215F72, ('\u{975B}', false)), // East Asian ideograph (variant of 4B5F72 which maps to 975B)
    (0x215F73, ('\u{975C}', false)), // East Asian ideograph
    (0x215F74, ('\u{975E}', false)), // East Asian ideograph
    (0x225F75, ('\u{7654}', false)), // East Asian ideograph
    (0x215F76, ('\u{9761}', false)), // East Asian ideograph
    (0x215F78, ('\u{9766}', false)), // East Asian ideograph
    (0x235F79, ('\u{9F4E}', false)), // East Asian ideograph
    (0x225F7A, ('\u{765C}', false)), // East Asian ideograph
    (0x235F7B, ('\u{9F4F}', false)), // East Asian ideograph
    (0x235F7C, ('\u{9F54}', false)), // East Asian ideograph
    (0x215F7D, ('\u{977C}', false)), // East Asian ideograph
    (0x235F7E, ('\u{9F55}', false)), // East Asian ideograph
    (0x216540, ('\u{4F66}', false)), // East Asian ideograph
    (0x692541, ('\u{30C1}', false)), // Katakana letter TI
    (0x6F5622, ('\u{C644}', false)), // Korean hangul
    (0x6F546E, ('\u{C500}', false)), // Korean hangul
    (0x6F502B, ('\u{BA54}', false)), // Korean hangul
    (0x215829, ('\u{898F}', false)), // East Asian ideograph
    (0x234251, ('\u{9230}', false)), // East Asian ideograph
    (0x225C28, ('\u{74B5}', false)), // East Asian ideograph
    (0x216544, ('\u{4F67}', false)), // East Asian ideograph
    (0x695429, ('\u{5726}', false)), // East Asian ideograph
    (0x6F515C, ('\u{BD88}', false)), // Korean hangul
    (0x292546, ('\u{8368}', false)), // East Asian ideograph
    (0x233145, ('\u{89DC}', false)), // East Asian ideograph
    (0x692547, ('\u{30C7}', false)), // Katakana letter DE
    (0x225C29, ('\u{74BA}', false)), // East Asian ideograph
    (0x213A48, ('\u{5B2A}', false)), // East Asian ideograph
    (0x6F492D, ('\u{AC85}', false)), // Korean hangul
    (0x69254A, ('\u{30CA}', false)), // Katakana letter NA
    (0x29254B, ('\u{835B}', false)), // East Asian ideograph
    (0x2D482F, ('\u{6E07}', false)), // East Asian ideograph
    (0x6F5442, ('\u{C330}', false)), // Korean hangul
    (0x70586F, ('\u{4EEB}', false)), // East Asian ideograph
    (0x274142, ('\u{6325}', false)), // East Asian ideograph
    (0x6F502D, ('\u{BA58}', false)), // Korean hangul
    (0x23254D, ('\u{8553}', false)), // East Asian ideograph
    (0x21654E, ('\u{4F5A}', false)), // East Asian ideograph
    (0x335065, ('\u{7A45}', false)), // East Asian ideograph
    (0x2E3B22, ('\u{690F}', false)), // East Asian ideograph
    (0x224F61, ('\u{7044}', false)), // East Asian ideograph
    (0x692550, ('\u{30D0}', false)), // Katakana letter BA
    (0x6F5C71, ('\u{D57C}', false)), // Korean hangul
    (0x222551, ('\u{5D8E}', false)), // East Asian ideograph
    (0x6F586B, ('\u{CB58}', false)), // Korean hangul
    (0x335834, ('\u{89E7}', false)), // East Asian ideograph
    (0x2D593D, ('\u{8AE9}', false)), // East Asian ideograph
    (0x4B4476, ('\u{685F}', false)), // East Asian ideograph
    (0x692555, ('\u{30D5}', false)), // Katakana letter HU
    (0x216556, ('\u{4F82}', false)), // East Asian ideograph
    (0x6F5A3A, ('\u{CF2D}', false)), // Korean hangul
    (0x6F502F, ('\u{BA64}', false)), // Korean hangul
    (0x692557, ('\u{30D7}', false)), // Katakana letter PU
    (0x294942, ('\u{9606}', false)), // East Asian ideograph
    (0x234255, ('\u{9248}', false)), // East Asian ideograph
    (0x692558, ('\u{30D8}', false)), // Katakana letter HE
    (0x47347B, ('\u{8C2B}', false)), // East Asian ideograph
    (0x6F5262, ('\u{C0AF}', false)), // Korean hangul
    (0x23255A, ('\u{8546}', false)), // East Asian ideograph
    (0x216021, ('\u{978D}', false)), // East Asian ideograph
    (0x226022, ('\u{7664}', false)), // East Asian ideograph
    (0x236023, ('\u{9F57}', false)), // East Asian ideograph
    (0x226024, ('\u{7659}', false)), // East Asian ideograph
    (0x216025, ('\u{97A0}', false)), // East Asian ideograph
    (0x216026, ('\u{97A3}', false)), // East Asian ideograph
    (0x216027, ('\u{97A6}', false)), // East Asian ideograph
    (0x236028, ('\u{9F60}', false)), // East Asian ideograph
    (0x216029, ('\u{97C3}', false)), // East Asian ideograph
    (0x21602A, ('\u{97C1}', false)), // East Asian ideograph
    (0x22602B, ('\u{765F}', false)), // East Asian ideograph
    (0x21602C, ('\u{97CB}', false)), // East Asian ideograph
    (0x21602D, ('\u{97CC}', false)), // East Asian ideograph
    (0x21602E, ('\u{97D3}', false)), // East Asian ideograph
    (0x21602F, ('\u{97DC}', false)), // East Asian ideograph
    (0x216030, ('\u{97ED}', false)), // East Asian ideograph
    (0x216031, ('\u{97F3}', false)), // East Asian ideograph
    (0x226032, ('\u{7667}', false)), // East Asian ideograph
    (0x216033, ('\u{7ADF}', false)), // East Asian ideograph
    (0x216034, ('\u{97F6}', false)), // East Asian ideograph
    (0x226035, ('\u{766A}', false)), // East Asian ideograph
    (0x216036, ('\u{97FF}', false)), // East Asian ideograph (variant of 456036 which maps to 97FF)
    (0x226037, ('\u{766D}', false)), // East Asian ideograph
    (0x226038, ('\u{766F}', false)), // East Asian ideograph
    (0x216039, ('\u{9803}', false)), // East Asian ideograph
    (0x22603A, ('\u{7670}', false)), // East Asian ideograph
    (0x21603B, ('\u{9806}', false)), // East Asian ideograph
    (0x21603C, ('\u{9808}', false)), // East Asian ideograph
    (0x21603D, ('\u{9810}', false)), // East Asian ideograph
    (0x21603E, ('\u{980A}', false)), // East Asian ideograph
    (0x21603F, ('\u{9811}', false)), // East Asian ideograph
    (0x226040, ('\u{7676}', false)), // East Asian ideograph
    (0x226041, ('\u{7677}', false)), // East Asian ideograph
    (0x216042, ('\u{980C}', false)), // East Asian ideograph
    (0x216043, ('\u{9817}', false)), // East Asian ideograph
    (0x216044, ('\u{9818}', false)), // East Asian ideograph (variant of 4B6044 which maps to 9818)
    (0x216045, ('\u{9821}', false)), // East Asian ideograph
    (0x216046, ('\u{982D}', false)), // East Asian ideograph
    (0x216047, ('\u{9830}', false)), // East Asian ideograph
    (0x226048, ('\u{7680}', false)), // East Asian ideograph
    (0x21582F, ('\u{89B2}', false)), // East Asian ideograph
    (0x22604A, ('\u{768B}', false)), // East Asian ideograph
    (0x21604B, ('\u{9837}', false)), // East Asian ideograph
    (0x21604C, ('\u{9824}', false)), // East Asian ideograph
    (0x21604D, ('\u{9846}', false)), // East Asian ideograph
    (0x21604E, ('\u{9854}', false)), // East Asian ideograph
    (0x21604F, ('\u{984D}', false)), // East Asian ideograph
    (0x216050, ('\u{984C}', false)), // East Asian ideograph
    (0x216051, ('\u{984E}', false)), // East Asian ideograph
    (0x226052, ('\u{7695}', false)), // East Asian ideograph
    (0x216053, ('\u{985E}', false)), // East Asian ideograph (variant of 4B6053 which maps to 985E)
    (0x216054, ('\u{985A}', false)), // East Asian ideograph
    (0x226055, ('\u{656B}', false)), // East Asian ideograph
    (0x216056, ('\u{9867}', false)), // East Asian ideograph
    (0x216057, ('\u{986B}', false)), // East Asian ideograph
    (0x216058, ('\u{986F}', false)), // East Asian ideograph
    (0x226059, ('\u{7699}', false)), // East Asian ideograph
    (0x21605A, ('\u{9870}', false)), // East Asian ideograph
    (0x21605B, ('\u{98A8}', false)), // East Asian ideograph
    (0x21605C, ('\u{98AF}', false)), // East Asian ideograph
    (0x22605D, ('\u{769C}', false)), // East Asian ideograph
    (0x21605E, ('\u{98B3}', false)), // East Asian ideograph
    (0x22605F, ('\u{769D}', false)), // East Asian ideograph
    (0x216060, ('\u{98BA}', false)), // East Asian ideograph
    (0x236061, ('\u{9F93}', false)), // East Asian ideograph
    (0x216062, ('\u{98C4}', false)), // East Asian ideograph
    (0x216063, ('\u{98DB}', false)), // East Asian ideograph
    (0x216064, ('\u{98DF}', false)), // East Asian ideograph
    (0x216065, ('\u{98E2}', false)), // East Asian ideograph
    (0x226066, ('\u{76A5}', false)), // East Asian ideograph
    (0x226067, ('\u{76A6}', false)), // East Asian ideograph
    (0x216068, ('\u{98ED}', false)), // East Asian ideograph
    (0x216069, ('\u{98EA}', false)), // East Asian ideograph
    (0x21606A, ('\u{98EE}', false)), // East Asian ideograph
    (0x23606B, ('\u{9FA0}', false)), // East Asian ideograph
    (0x21606C, ('\u{98FC}', false)), // East Asian ideograph
    (0x21606D, ('\u{98F4}', false)), // East Asian ideograph
    (0x21606E, ('\u{98FD}', false)), // East Asian ideograph
    (0x21606F, ('\u{98FE}', false)), // East Asian ideograph
    (0x216070, ('\u{9903}', false)), // East Asian ideograph
    (0x216071, ('\u{990A}', false)), // East Asian ideograph
    (0x236072, ('\u{9FA4}', false)), // East Asian ideograph
    (0x216073, ('\u{9909}', false)), // East Asian ideograph
    (0x226074, ('\u{76B8}', false)), // East Asian ideograph
    (0x216075, ('\u{9912}', false)), // East Asian ideograph
    (0x216076, ('\u{9918}', false)), // East Asian ideograph
    (0x226077, ('\u{76BD}', false)), // East Asian ideograph
    (0x216078, ('\u{9905}', false)), // East Asian ideograph
    (0x216079, ('\u{9928}', false)), // East Asian ideograph
    (0x21607A, ('\u{991E}', false)), // East Asian ideograph
    (0x21607B, ('\u{991B}', false)), // East Asian ideograph
    (0x21607C, ('\u{9921}', false)), // East Asian ideograph
    (0x21607D, ('\u{9935}', false)), // East Asian ideograph
    (0x21607E, ('\u{993E}', false)), // East Asian ideograph
    (0x6F5369, ('\u{C258}', false)), // Korean hangul
    (0x6F5033, ('\u{BA71}', false)), // Korean hangul
    (0x213A5B, ('\u{5B6B}', false)), // East Asian ideograph
    (0x69256B, ('\u{30EB}', false)), // Katakana letter RU
    (0x69256C, ('\u{30EC}', false)), // Katakana letter RE
    (0x293670, ('\u{8D49}', false)), // East Asian ideograph
    (0x69656D, ('\u{7E83}', false)), // East Asian ideograph
    (0x224F67, ('\u{7047}', false)), // East Asian ideograph
    (0x235172, ('\u{994C}', false)), // East Asian ideograph
    (0x69256F, ('\u{30EF}', false)), // Katakana letter WA
    (0x2E4C35, ('\u{6DE5}', false)), // East Asian ideograph
    (0x6F5034, ('\u{BA74}', false)), // Korean hangul
    (0x213A5C, ('\u{5B70}', false)), // East Asian ideograph
    (0x6F5934, ('\u{CC59}', false)), // Korean hangul
    (0x4D4F39, ('\u{988C}', false)), // East Asian ideograph
    (0x292571, ('\u{835E}', false)), // East Asian ideograph
    (0x226573, ('\u{78E0}', false)), // East Asian ideograph
    (0x6F4E4C, ('\u{B6B1}', false)), // Korean hangul
    (0x292574, ('\u{83B8}', false)), // East Asian ideograph
    (0x4B3A47, ('\u{88CA}', false)), // East Asian ideograph
    (0x6F5035, ('\u{BA78}', false)), // Korean hangul
    (0x692575, ('\u{30F5}', false)), // Katakana letter small KA
    (0x294948, ('\u{960F}', false)), // East Asian ideograph
    (0x213378, ('\u{5275}', false)), // East Asian ideograph
    (0x225C32, ('\u{74CC}', false)), // East Asian ideograph
    (0x216576, ('\u{4F9C}', false)), // East Asian ideograph
    (0x215021, ('\u{7B4D}', false)), // East Asian ideograph
    (0x6F5C6C, ('\u{D56D}', false)), // Korean hangul
    (0x232577, ('\u{858C}', false)), // East Asian ideograph
    (0x214B6A, ('\u{74CA}', false)), // East Asian ideograph
    (0x224F69, ('\u{7049}', false)), // East Asian ideograph
    (0x216940, ('\u{5133}', false)), // East Asian ideograph
    (0x692578, ('\u{309C}', false)), // Katakana-hiragana semi-voiced sound mark
    (0x275023, ('\u{8345}', false)), // East Asian ideograph
    (0x2E742E, ('\u{7516}', false)), // East Asian ideograph
    (0x6F5479, ('\u{C53D}', false)), // Korean hangul
    (0x6F5024, ('\u{BA40}', false)), // Korean hangul
    (0x6F5036, ('\u{BA83}', false)), // Korean hangul
    (0x213A5E, ('\u{5B71}', false)), // East Asian ideograph
    (0x294949, ('\u{9608}', false)), // East Asian ideograph
    (0x225025, ('\u{7066}', false)), // East Asian ideograph
    (0x2E257B, ('\u{5D1F}', false)), // East Asian ideograph
    (0x6F5026, ('\u{BA49}', false)), // Korean hangul
    (0x6F4A5F, ('\u{AECD}', false)), // Korean hangul
    (0x225027, ('\u{7065}', false)), // East Asian ideograph
    (0x235369, ('\u{9A36}', false)), // East Asian ideograph
    (0x225028, ('\u{7068}', false)), // East Asian ideograph
    (0x234F26, ('\u{9816}', false)), // East Asian ideograph
    (0x215029, ('\u{7B95}', false)), // East Asian ideograph
    (0x276272, ('\u{9EE9}', false)), // East Asian ideograph
    (0x213A5F, ('\u{5B75}', false)), // East Asian ideograph
    (0x27502A, ('\u{94B3}', false)), // East Asian ideograph
    (0x27502B, ('\u{7B3A}', false)), // East Asian ideograph
    (0x6F502C, ('\u{BA55}', false)), // Korean hangul
    (0x224F6B, ('\u{7055}', false)), // East Asian ideograph
    (0x23536A, ('\u{9A2E}', false)), // East Asian ideograph
    (0x2D502D, ('\u{7B5D}', false)), // East Asian ideograph
    (0x4C4339, ('\u{69DE}', false)), // East Asian ideograph
    (0x6F502E, ('\u{BA5C}', false)), // Korean hangul
    (0x6F5038, ('\u{BA85}', false)), // Korean hangul
    (0x213A60, ('\u{5B78}', false)), // East Asian ideograph
    (0x21502F, ('\u{7BAD}', false)), // East Asian ideograph
    (0x215030, ('\u{7BC4}', false)), // East Asian ideograph
    (0x216122, ('\u{993D}', false)), // East Asian ideograph
    (0x226123, ('\u{76CB}', false)), // East Asian ideograph
    (0x216124, ('\u{9952}', false)), // East Asian ideograph
    (0x216125, ('\u{9951}', false)), // East Asian ideograph
    (0x226126, ('\u{76CC}', false)), // East Asian ideograph
    (0x216127, ('\u{995E}', false)), // East Asian ideograph
    (0x216128, ('\u{9996}', false)), // East Asian ideograph
    (0x216129, ('\u{9999}', false)), // East Asian ideograph
    (0x21612A, ('\u{99A5}', false)), // East Asian ideograph
    (0x21612B, ('\u{99A8}', false)), // East Asian ideograph
    (0x21612C, ('\u{99AC}', false)), // East Asian ideograph
    (0x21612D, ('\u{99AE}', false)), // East Asian ideograph
    (0x21612E, ('\u{99AD}', false)), // East Asian ideograph
    (0x21612F, ('\u{99B3}', false)), // East Asian ideograph
    (0x216130, ('\u{99B1}', false)), // East Asian ideograph
    (0x216131, ('\u{99B4}', false)), // East Asian ideograph
    (0x216132, ('\u{99C1}', false)), // East Asian ideograph
    (0x275033, ('\u{8282}', false)), // East Asian ideograph
    (0x216134, ('\u{99DD}', false)), // East Asian ideograph
    (0x216135, ('\u{99D5}', false)), // East Asian ideograph
    (0x216136, ('\u{99DF}', false)), // East Asian ideograph
    (0x216137, ('\u{99DB}', false)), // East Asian ideograph
    (0x216138, ('\u{99D2}', false)), // East Asian ideograph
    (0x216139, ('\u{99D9}', false)), // East Asian ideograph
    (0x21613A, ('\u{99D1}', false)), // East Asian ideograph
    (0x21613B, ('\u{99ED}', false)), // East Asian ideograph
    (0x21613C, ('\u{99F1}', false)), // East Asian ideograph
    (0x21613D, ('\u{9A01}', false)), // East Asian ideograph
    (0x21613E, ('\u{99FF}', false)), // East Asian ideograph
    (0x21613F, ('\u{99E2}', false)), // East Asian ideograph
    (0x216140, ('\u{9A0E}', false)), // East Asian ideograph
    (0x216141, ('\u{9A19}', false)), // East Asian ideograph
    (0x216142, ('\u{9A16}', false)), // East Asian ideograph
    (0x216143, ('\u{9A2B}', false)), // East Asian ideograph
    (0x226144, ('\u{76ED}', false)), // East Asian ideograph
    (0x216145, ('\u{9A37}', false)), // East Asian ideograph
    (0x216146, ('\u{9A43}', false)), // East Asian ideograph
    (0x216147, ('\u{9A45}', false)), // East Asian ideograph
    (0x226148, ('\u{76F1}', false)), // East Asian ideograph
    (0x216149, ('\u{9A3E}', false)), // East Asian ideograph
    (0x21614A, ('\u{9A55}', false)), // East Asian ideograph
    (0x21614B, ('\u{9A5A}', false)), // East Asian ideograph
    (0x21614C, ('\u{9A5B}', false)), // East Asian ideograph
    (0x21614D, ('\u{9A57}', false)), // East Asian ideograph
    (0x21614E, ('\u{9A5F}', false)), // East Asian ideograph
    (0x22614F, ('\u{7708}', false)), // East Asian ideograph
    (0x226150, ('\u{7707}', false)), // East Asian ideograph
    (0x275038, ('\u{7BAC}', false)), // East Asian ideograph
    (0x216152, ('\u{9AA8}', false)), // East Asian ideograph
    (0x216153, ('\u{9AAF}', false)), // East Asian ideograph
    (0x226154, ('\u{770A}', false)), // East Asian ideograph
    (0x216155, ('\u{9AB7}', false)), // East Asian ideograph
    (0x216156, ('\u{9AB8}', false)), // East Asian ideograph
    (0x215039, ('\u{7BE4}', false)), // East Asian ideograph
    (0x216158, ('\u{9ACF}', false)), // East Asian ideograph
    (0x226159, ('\u{76FB}', false)), // East Asian ideograph
    (0x21615A, ('\u{9AD4}', false)), // East Asian ideograph
    (0x21615B, ('\u{9AD2}', false)), // East Asian ideograph
    (0x21615C, ('\u{9AD8}', false)), // East Asian ideograph
    (0x21615D, ('\u{9AE5}', false)), // East Asian ideograph
    (0x22615E, ('\u{772B}', false)), // East Asian ideograph
    (0x21615F, ('\u{9AEE}', false)), // East Asian ideograph
    (0x216160, ('\u{9AFB}', false)), // East Asian ideograph
    (0x216161, ('\u{9AED}', false)), // East Asian ideograph
    (0x216162, ('\u{9B03}', false)), // East Asian ideograph
    (0x216163, ('\u{9B06}', false)), // East Asian ideograph
    (0x216164, ('\u{9B0D}', false)), // East Asian ideograph
    (0x216165, ('\u{9B1A}', false)), // East Asian ideograph
    (0x216166, ('\u{9B22}', false)), // East Asian ideograph
    (0x216167, ('\u{9B25}', false)), // East Asian ideograph
    (0x216168, ('\u{9B27}', false)), // East Asian ideograph
    (0x27503C, ('\u{7B5B}', false)), // East Asian ideograph
    (0x21616A, ('\u{9B31}', false)), // East Asian ideograph
    (0x21616B, ('\u{9B32}', false)), // East Asian ideograph
    (0x21616C, ('\u{9B3C}', false)), // East Asian ideograph
    (0x21616D, ('\u{9B41}', false)), // East Asian ideograph
    (0x21616E, ('\u{9B42}', false)), // East Asian ideograph
    (0x22616F, ('\u{7721}', false)), // East Asian ideograph
    (0x216170, ('\u{9B44}', false)), // East Asian ideograph
    (0x216171, ('\u{9B4F}', false)), // East Asian ideograph
    (0x216172, ('\u{9B54}', false)), // East Asian ideograph
    (0x216173, ('\u{9B58}', false)), // East Asian ideograph
    (0x216174, ('\u{9B5A}', false)), // East Asian ideograph
    (0x226175, ('\u{7739}', false)), // East Asian ideograph
    (0x226176, ('\u{772F}', false)), // East Asian ideograph
    (0x216177, ('\u{9B91}', false)), // East Asian ideograph
    (0x216178, ('\u{9BAB}', false)), // East Asian ideograph
    (0x216179, ('\u{9BAE}', false)), // East Asian ideograph
    (0x21617A, ('\u{9BAA}', false)), // East Asian ideograph
    (0x21617B, ('\u{9BCA}', false)), // East Asian ideograph
    (0x21617C, ('\u{9BC9}', false)), // East Asian ideograph
    (0x21617D, ('\u{9BE8}', false)), // East Asian ideograph
    (0x21617E, ('\u{9BE7}', false)), // East Asian ideograph
    (0x215040, ('\u{7BF7}', false)), // East Asian ideograph
    (0x275041, ('\u{7B80}', false)), // East Asian ideograph
    (0x225042, ('\u{7086}', false)), // East Asian ideograph
    (0x6F503C, ('\u{BAAB}', false)), // Korean hangul
    (0x29596B, ('\u{9CA1}', false)), // East Asian ideograph
    (0x335F3D, ('\u{96B7}', false)), // East Asian ideograph
    (0x29494F, ('\u{960A}', false)), // East Asian ideograph
    (0x6F5043, ('\u{BAC3}', false)), // Korean hangul
    (0x4B5044, ('\u{7C27}', false)), // East Asian ideograph (variant of 215044 which maps to 7C27)
    (0x275045, ('\u{7BAA}', false)), // East Asian ideograph
    (0x2E3D73, ('\u{7A1C}', false)), // East Asian ideograph
    (0x275046, ('\u{7BD1}', false)), // East Asian ideograph
    (0x6F5047, ('\u{BB34}', false)), // Korean hangul
    (0x6F503D, ('\u{BAAC}', false)), // Korean hangul
    (0x213A65, ('\u{5B87}', false)), // East Asian ideograph
    (0x294950, ('\u{960C}', false)), // East Asian ideograph
    (0x235048, ('\u{98B8}', false)), // East Asian ideograph
    (0x225C3A, ('\u{74D4}', false)), // East Asian ideograph
    (0x27583A, ('\u{8BA3}', false)), // East Asian ideograph
    (0x225049, ('\u{7084}', false)), // East Asian ideograph
    (0x2D594C, ('\u{8B72}', false)), // East Asian ideograph
    (0x6F5960, ('\u{CE20}', false)), // Korean hangul
    (0x22504A, ('\u{7081}', false)), // East Asian ideograph
    (0x235370, ('\u{9A41}', false)), // East Asian ideograph
    (0x21504B, ('\u{7C3D}', false)), // East Asian ideograph
    (0x4D3032, ('\u{88AE}', false)), // East Asian ideograph
    (0x6F5A3D, ('\u{CF54}', false)), // Korean hangul
    (0x27504C, ('\u{7BEE}', false)), // East Asian ideograph
    (0x274153, ('\u{6363}', false)), // East Asian ideograph
    (0x4D5C6B, ('\u{9D50}', false)), // East Asian ideograph
    (0x213A66, ('\u{5B88}', false)), // East Asian ideograph
    (0x21504D, ('\u{7C4C}', false)), // East Asian ideograph
    (0x234264, ('\u{925E}', false)), // East Asian ideograph
    (0x2D3B77, ('\u{5CE9}', false)), // East Asian ideograph
    (0x21504E, ('\u{7C4D}', false)), // East Asian ideograph
    (0x6F5025, ('\u{BA48}', false)), // Korean hangul
    (0x2D504F, ('\u{7C58}', false)), // East Asian ideograph
    (0x69542A, ('\u{5737}', false)), // East Asian ideograph
    (0x293B59, ('\u{8F82}', false)), // East Asian ideograph
    (0x4B4B2B, ('\u{7363}', false)), // East Asian ideograph
    (0x275050, ('\u{7B3C}', false)), // East Asian ideograph
    (0x275051, ('\u{7C41}', false)), // East Asian ideograph
    (0x6F503F, ('\u{BAB8}', false)), // Korean hangul
    (0x213A67, ('\u{5B89}', false)), // East Asian ideograph
    (0x294952, ('\u{960D}', false)), // East Asian ideograph
    (0x275052, ('\u{7B7E}', false)), // East Asian ideograph (duplicate simplified)
    (0x6F5963, ('\u{CE35}', false)), // Korean hangul
    (0x2D3B78, ('\u{5CEF}', false)), // East Asian ideograph
    (0x275053, ('\u{7BF1}', false)), // East Asian ideograph
    (0x4D594E, ('\u{9BF5}', false)), // East Asian ideograph
    (0x3F614C, ('\u{99C5}', false)), // East Asian ideograph
    (0x275054, ('\u{7BA9}', false)), // East Asian ideograph
    (0x4B6258, ('\u{68BA}', false)), // East Asian ideograph
    (0x275055, ('\u{5401}', false)), // East Asian ideograph
    (0x6F245A, ('\u{3139}', false)), // Korean hangul
    (0x225056, ('\u{7088}', false)), // East Asian ideograph
    (0x6F5040, ('\u{BAB9}', false)), // Korean hangul
    (0x213A68, ('\u{5B85}', false)), // East Asian ideograph
    (0x21583E, ('\u{8A0C}', false)), // East Asian ideograph
    (0x2D3B79, ('\u{5D8B}', false)), // East Asian ideograph
    (0x6F5058, ('\u{BB88}', false)), // Korean hangul
    (0x2D594F, ('\u{8B83}', false)), // East Asian ideograph
    (0x225059, ('\u{708C}', false)), // East Asian ideograph
    (0x224B31, ('\u{6E5D}', false)), // East Asian ideograph
    (0x216221, ('\u{9C13}', false)), // East Asian ideograph
    (0x226222, ('\u{7725}', false)), // East Asian ideograph
    (0x216223, ('\u{9BFD}', false)), // East Asian ideograph
    (0x216224, ('\u{9C2D}', false)), // East Asian ideograph
    (0x216225, ('\u{9C25}', false)), // East Asian ideograph
    (0x226226, ('\u{7734}', false)), // East Asian ideograph
    (0x216227, ('\u{9C3E}', false)), // East Asian ideograph
    (0x216228, ('\u{9C3B}', false)), // East Asian ideograph
    (0x216229, ('\u{9C54}', false)), // East Asian ideograph
    (0x21622A, ('\u{9C57}', false)), // East Asian ideograph
    (0x21622B, ('\u{9C56}', false)), // East Asian ideograph
    (0x21622C, ('\u{9C49}', false)), // East Asian ideograph
    (0x22622D, ('\u{7747}', false)), // East Asian ideograph
    (0x21622E, ('\u{9C78}', false)), // East Asian ideograph
    (0x21622F, ('\u{9CE5}', false)), // East Asian ideograph
    (0x216230, ('\u{9CE9}', false)), // East Asian ideograph
    (0x226231, ('\u{7745}', false)), // East Asian ideograph
    (0x226232, ('\u{774D}', false)), // East Asian ideograph
    (0x216233, ('\u{9CF3}', false)), // East Asian ideograph
    (0x216234, ('\u{9D06}', false)), // East Asian ideograph
    (0x216235, ('\u{9D09}', false)), // East Asian ideograph
    (0x216236, ('\u{9D15}', false)), // East Asian ideograph
    (0x226237, ('\u{774E}', false)), // East Asian ideograph
    (0x216238, ('\u{9D28}', false)), // East Asian ideograph
    (0x216239, ('\u{9D26}', false)), // East Asian ideograph
    (0x22623A, ('\u{775F}', false)), // East Asian ideograph
    (0x21505F, ('\u{7CBD}', false)), // East Asian ideograph
    (0x21623C, ('\u{9D3B}', false)), // East Asian ideograph
    (0x21623D, ('\u{9D3F}', false)), // East Asian ideograph
    (0x22623E, ('\u{7752}', false)), // East Asian ideograph
    (0x21623F, ('\u{9D51}', false)), // East Asian ideograph
    (0x216240, ('\u{9D60}', false)), // East Asian ideograph
    (0x215060, ('\u{7CB9}', false)), // East Asian ideograph
    (0x226242, ('\u{7758}', false)), // East Asian ideograph
    (0x216243, ('\u{9D72}', false)), // East Asian ideograph
    (0x226244, ('\u{7756}', false)), // East Asian ideograph
    (0x226245, ('\u{775A}', false)), // East Asian ideograph
    (0x216246, ('\u{9DB4}', false)), // East Asian ideograph
    (0x216247, ('\u{9DAF}', false)), // East Asian ideograph
    (0x216248, ('\u{9DC2}', false)), // East Asian ideograph
    (0x216249, ('\u{9DD3}', false)), // East Asian ideograph
    (0x21624A, ('\u{9DD7}', false)), // East Asian ideograph
    (0x21624B, ('\u{9DE5}', false)), // East Asian ideograph
    (0x21624C, ('\u{9DF9}', false)), // East Asian ideograph
    (0x215062, ('\u{7CCA}', false)), // East Asian ideograph
    (0x21624E, ('\u{9E1A}', false)), // East Asian ideograph
    (0x22624F, ('\u{7762}', false)), // East Asian ideograph
    (0x216250, ('\u{9E79}', false)), // East Asian ideograph
    (0x216251, ('\u{9E7D}', false)), // East Asian ideograph
    (0x226252, ('\u{7780}', false)), // East Asian ideograph
    (0x216253, ('\u{9E7F}', false)), // East Asian ideograph
    (0x216254, ('\u{9E82}', false)), // East Asian ideograph
    (0x216255, ('\u{9E8B}', false)), // East Asian ideograph
    (0x226256, ('\u{776F}', false)), // East Asian ideograph
    (0x216257, ('\u{9E92}', false)), // East Asian ideograph
    (0x216258, ('\u{9E93}', false)), // East Asian ideograph
    (0x224B33, ('\u{6E30}', false)), // East Asian ideograph
    (0x21625A, ('\u{9E9F}', false)), // East Asian ideograph
    (0x21625B, ('\u{9EA5}', false)), // East Asian ideograph
    (0x21625C, ('\u{9EA9}', false)), // East Asian ideograph
    (0x21625D, ('\u{9EB4}', false)), // East Asian ideograph
    (0x21625E, ('\u{9EB5}', false)), // East Asian ideograph
    (0x22625F, ('\u{7785}', false)), // East Asian ideograph
    (0x216260, ('\u{9EBC}', false)), // East Asian ideograph
    (0x216261, ('\u{9EBE}', false)), // East Asian ideograph
    (0x216262, ('\u{9EC3}', false)), // East Asian ideograph
    (0x216263, ('\u{9ECD}', false)), // East Asian ideograph
    (0x216264, ('\u{9ECE}', false)), // East Asian ideograph
    (0x216265, ('\u{9ECF}', false)), // East Asian ideograph
    (0x226266, ('\u{778B}', false)), // East Asian ideograph (variant of 4C6266 which maps to 778B)
    (0x216267, ('\u{58A8}', false)), // East Asian ideograph
    (0x216268, ('\u{9ED8}', false)), // East Asian ideograph
    (0x216269, ('\u{9ED4}', false)), // East Asian ideograph
    (0x22626A, ('\u{778D}', false)), // East Asian ideograph
    (0x21626B, ('\u{9EDC}', false)), // East Asian ideograph
    (0x21626C, ('\u{9EDB}', false)), // East Asian ideograph
    (0x21626D, ('\u{9EDD}', false)), // East Asian ideograph
    (0x21626E, ('\u{9EE0}', false)), // East Asian ideograph
    (0x21626F, ('\u{9EE8}', false)), // East Asian ideograph
    (0x216270, ('\u{9EEF}', false)), // East Asian ideograph
    (0x235068, ('\u{98F1}', false)), // East Asian ideograph
    (0x226272, ('\u{7798}', false)), // East Asian ideograph
    (0x226273, ('\u{7796}', false)), // East Asian ideograph
    (0x216274, ('\u{9F0E}', false)), // East Asian ideograph
    (0x226275, ('\u{77A2}', false)), // East Asian ideograph
    (0x226276, ('\u{7799}', false)), // East Asian ideograph
    (0x216277, ('\u{9F19}', false)), // East Asian ideograph
    (0x216278, ('\u{9F20}', false)), // East Asian ideograph
    (0x216279, ('\u{9F2C}', false)), // East Asian ideograph
    (0x22627A, ('\u{77B5}', false)), // East Asian ideograph
    (0x21627B, ('\u{9F3B}', false)), // East Asian ideograph
    (0x21627C, ('\u{9F3E}', false)), // East Asian ideograph
    (0x22627D, ('\u{77B7}', false)), // East Asian ideograph
    (0x21627E, ('\u{9F4B}', false)), // East Asian ideograph
    (0x6F5044, ('\u{BAFC}', false)), // Korean hangul
    (0x27506B, ('\u{7CAE}', false)), // East Asian ideograph
    (0x6F5964, ('\u{CE58}', false)), // Korean hangul
    (0x225C41, ('\u{74DB}', false)), // East Asian ideograph
    (0x236040, ('\u{9F6F}', false)), // East Asian ideograph
    (0x23506C, ('\u{98EB}', false)), // East Asian ideograph
    (0x6F506D, ('\u{BC14}', false)), // Korean hangul
    (0x23315E, ('\u{89F5}', false)), // East Asian ideograph
    (0x6F506E, ('\u{BC15}', false)), // Korean hangul
    (0x4B4049, ('\u{62D0}', false)), // East Asian ideograph
    (0x234F34, ('\u{982B}', false)), // East Asian ideograph
    (0x22506F, ('\u{70A7}', false)), // East Asian ideograph
    (0x27415A, ('\u{5C4F}', false)), // East Asian ideograph
    (0x696273, ('\u{78B5}', false)), // East Asian ideograph
    (0x275070, ('\u{7EAA}', false)), // East Asian ideograph
    (0x215843, ('\u{8A13}', false)), // East Asian ideograph
    (0x225071, ('\u{70B5}', false)), // East Asian ideograph
    (0x275072, ('\u{7EA2}', false)), // East Asian ideograph
    (0x295A65, ('\u{9E39}', false)), // East Asian ideograph
    (0x215073, ('\u{7D09}', false)), // East Asian ideograph
    (0x396179, ('\u{5C20}', false)), // East Asian ideograph
    (0x275074, ('\u{7EA6}', false)), // East Asian ideograph
    (0x27415B, ('\u{631A}', false)), // East Asian ideograph
    (0x6F5046, ('\u{BB18}', false)), // Korean hangul
    (0x275075, ('\u{7EA5}', false)), // East Asian ideograph
    (0x215844, ('\u{8A2A}', false)), // East Asian ideograph
    (0x275076, ('\u{7EBA}', false)), // East Asian ideograph
    (0x213B21, ('\u{5BC6}', false)), // East Asian ideograph
    (0x275077, ('\u{7EB9}', false)), // East Asian ideograph
    (0x213B22, ('\u{5BC7}', false)), // East Asian ideograph
    (0x235379, ('\u{9A42}', false)), // East Asian ideograph
    (0x215078, ('\u{7D0A}', false)), // East Asian ideograph
    (0x6F5729, ('\u{C7B0}', false)), // Korean hangul
    (0x213B23, ('\u{5BC5}', false)), // East Asian ideograph
    (0x6F5C72, ('\u{D584}', false)), // Korean hangul
    (0x6F5079, ('\u{BC2D}', false)), // Korean hangul
    (0x6F536D, ('\u{C27C}', false)), // Korean hangul
    (0x29495A, ('\u{9612}', false)), // East Asian ideograph
    (0x27507A, ('\u{7EAD}', false)), // East Asian ideograph
    (0x213B25, ('\u{5BC2}', false)), // East Asian ideograph
    (0x22507B, ('\u{70E5}', false)), // East Asian ideograph
    (0x213B26, ('\u{5BBF}', false)), // East Asian ideograph
    (0x21507C, ('\u{7D15}', false)), // East Asian ideograph
    (0x224F7B, ('\u{705E}', false)), // East Asian ideograph
    (0x23537A, ('\u{9A44}', false)), // East Asian ideograph
    (0x22507D, ('\u{70D3}', false)), // East Asian ideograph
    (0x234F37, ('\u{9820}', false)), // East Asian ideograph
    (0x27507E, ('\u{7EBD}', false)), // East Asian ideograph
    (0x335276, ('\u{8061}', false)), // East Asian ideograph
    (0x6F5048, ('\u{BB35}', false)), // Korean hangul
    (0x215846, ('\u{8A1D}', false)), // East Asian ideograph
    (0x2D3B2A, ('\u{5EBD}', false)), // East Asian ideograph
    (0x2D5957, ('\u{7AEA}', false)), // East Asian ideograph
    (0x4B386C, ('\u{5841}', false)), // East Asian ideograph
    (0x295A68, ('\u{9E3A}', false)), // East Asian ideograph
    (0x453336, ('\u{5B82}', false)), // East Asian ideograph
    (0x224B39, ('\u{6E6B}', false)), // East Asian ideograph
    (0x213B2D, ('\u{5BE8}', false)), // East Asian ideograph
    (0x273B2E, ('\u{5BDD}', false)), // East Asian ideograph
    (0x6F5049, ('\u{BB36}', false)), // Korean hangul
    (0x213A71, ('\u{5B9B}', false)), // East Asian ideograph
    (0x6F5965, ('\u{CE59}', false)), // Korean hangul
    (0x213B2F, ('\u{5BE4}', false)), // East Asian ideograph
    (0x4B515A, ('\u{7E01}', false)), // East Asian ideograph
    (0x216321, ('\u{9F52}', false)), // East Asian ideograph
    (0x216322, ('\u{9F5F}', false)), // East Asian ideograph
    (0x216323, ('\u{9F63}', false)), // East Asian ideograph
    (0x216324, ('\u{9F61}', false)), // East Asian ideograph (variant of 456324 which maps to 9F61)
    (0x216325, ('\u{9F66}', false)), // East Asian ideograph
    (0x216326, ('\u{9F5C}', false)), // East Asian ideograph
    (0x233B31, ('\u{8ECE}', false)), // East Asian ideograph
    (0x216328, ('\u{9F6A}', false)), // East Asian ideograph
    (0x216329, ('\u{9F77}', false)), // East Asian ideograph
    (0x21632A, ('\u{9F72}', false)), // East Asian ideograph
    (0x21632B, ('\u{9F8D}', false)), // East Asian ideograph
    (0x22632C, ('\u{77BC}', false)), // East Asian ideograph
    (0x21632D, ('\u{9F9C}', false)), // East Asian ideograph
    (0x216330, ('\u{8288}', false)), // East Asian ideograph
    (0x213B33, ('\u{5BDF}', false)), // East Asian ideograph
    (0x6F504A, ('\u{BB38}', false)), // Korean hangul
    (0x226335, ('\u{77CD}', false)), // East Asian ideograph
    (0x29307D, ('\u{89CF}', false)), // East Asian ideograph
    (0x696733, ('\u{81A4}', false)), // East Asian ideograph
    (0x225C47, ('\u{74DE}', false)), // East Asian ideograph
    (0x6F4C3F, ('\u{B1D4}', false)), // Korean hangul
    (0x213B35, ('\u{5BEC}', false)), // East Asian ideograph
    (0x706340, ('\u{61B7}', false)), // East Asian ideograph
    (0x45462B, ('\u{7688}', false)), // East Asian ideograph
    (0x226345, ('\u{77DE}', false)), // East Asian ideograph
    (0x226346, ('\u{77DF}', false)), // East Asian ideograph
    (0x23537D, ('\u{9A48}', false)), // East Asian ideograph
    (0x224B3B, ('\u{6E8B}', false)), // East Asian ideograph
    (0x213B37, ('\u{5BEB}', false)), // East Asian ideograph
    (0x335738, ('\u{880F}', false)), // East Asian ideograph
    (0x69634E, ('\u{7A43}', false)), // East Asian ideograph
    (0x22634F, ('\u{77E7}', false)), // East Asian ideograph
    (0x274160, ('\u{63B4}', false)), // East Asian ideograph
    (0x226352, ('\u{77E6}', false)), // East Asian ideograph
    (0x226355, ('\u{77EC}', false)), // East Asian ideograph
    (0x273B39, ('\u{5B9D}', false)), // East Asian ideograph
    (0x69254D, ('\u{30CD}', false)), // Katakana letter NE
    (0x226359, ('\u{77F0}', false)), // East Asian ideograph
    (0x22635A, ('\u{77F1}', false)), // East Asian ideograph
    (0x22635C, ('\u{77F4}', false)), // East Asian ideograph
    (0x217B3A, ('\u{5A38}', false)), // East Asian ideograph
    (0x226360, ('\u{77FC}', false)), // East Asian ideograph
    (0x213B3B, ('\u{5BFA}', false)), // East Asian ideograph
    (0x23537E, ('\u{9A4C}', false)), // East Asian ideograph
    (0x226367, ('\u{77F8}', false)), // East Asian ideograph
    (0x226368, ('\u{77FB}', false)), // East Asian ideograph
    (0x277B3C, ('\u{5A05}', false)), // East Asian ideograph
    (0x355739, ('\u{9C76}', false)), // East Asian ideograph
    (0x234944, ('\u{95AB}', false)), // East Asian ideograph
    (0x226370, ('\u{7809}', false)), // East Asian ideograph
    (0x226371, ('\u{7806}', false)), // East Asian ideograph
    (0x226373, ('\u{7819}', false)), // East Asian ideograph
    (0x226374, ('\u{7811}', false)), // East Asian ideograph
    (0x293B3E, ('\u{8F71}', false)), // East Asian ideograph
    (0x4C6376, ('\u{7839}', false)), // East Asian ideograph
    (0x226378, ('\u{7812}', false)), // East Asian ideograph
    (0x223B3F, ('\u{67A1}', false)), // East Asian ideograph
    (0x213B40, ('\u{5C07}', false)), // East Asian ideograph
    (0x4B5E27, ('\u{93AD}', false)), // East Asian ideograph
    (0x273B42, ('\u{5BFB}', false)), // East Asian ideograph
    (0x6F504D, ('\u{BB3D}', false)), // Korean hangul
    (0x6F5935, ('\u{CC60}', false)), // Korean hangul
    (0x294960, ('\u{9619}', false)), // East Asian ideograph
    (0x213B43, ('\u{5C0D}', false)), // East Asian ideograph
    (0x223A31, ('\u{6710}', false)), // East Asian ideograph
    (0x273B44, ('\u{5BFC}', false)), // East Asian ideograph
    (0x224B3E, ('\u{6E76}', false)), // East Asian ideograph
    (0x234F3D, ('\u{9833}', false)), // East Asian ideograph
    (0x2D4850, ('\u{6EDA}', false)), // East Asian ideograph
    (0x293B47, ('\u{8F77}', false)), // East Asian ideograph
    (0x6F504E, ('\u{BB44}', false)), // Korean hangul
    (0x275F39, ('\u{968F}', false)), // East Asian ideograph
    (0x23573F, ('\u{9BA8}', false)), // East Asian ideograph
    (0x274B74, ('\u{74EF}', false)), // East Asian ideograph
    (0x217B48, ('\u{5A50}', false)), // East Asian ideograph
    (0x213B49, ('\u{5C24}', false)), // East Asian ideograph
    (0x234E3B, ('\u{97C5}', false)), // East Asian ideograph
    (0x4B4053, ('\u{627A}', false)), // East Asian ideograph
    (0x213B4B, ('\u{5C31}', false)), // East Asian ideograph
    (0x273B4C, ('\u{5C34}', false)), // East Asian ideograph
    (0x6F504F, ('\u{BB47}', false)), // Korean hangul
    (0x275F3A, ('\u{9669}', false)), // East Asian ideograph
    (0x2D625F, ('\u{83FB}', false)), // East Asian ideograph
    (0x213634, ('\u{5501}', false)), // East Asian ideograph
    (0x213B4E, ('\u{5C3A}', false)), // East Asian ideograph
    (0x394956, ('\u{792E}', false)), // East Asian ideograph
    (0x2D7552, ('\u{579B}', false)), // East Asian ideograph
    (0x213B4F, ('\u{5C3C}', false)), // East Asian ideograph
    (0x69562C, ('\u{599B}', false)), // East Asian ideograph
    (0x294E54, ('\u{97EA}', false)), // East Asian ideograph
    (0x692574, ('\u{30F4}', false)), // Katakana letter VU
    (0x233B51, ('\u{8EFF}', false)), // East Asian ideograph
    (0x6F5050, ('\u{BB49}', false)), // Korean hangul
    (0x275F3B, ('\u{9690}', false)), // East Asian ideograph
    (0x213635, ('\u{54FC}', false)), // East Asian ideograph
    (0x27516C, ('\u{7F1D}', false)), // East Asian ideograph
    (0x4B4537, ('\u{6804}', false)), // East Asian ideograph
    (0x213B54, ('\u{5C46}', false)), // East Asian ideograph
    (0x45304C, ('\u{69A6}', false)), // East Asian ideograph
    (0x234F40, ('\u{982E}', false)), // East Asian ideograph
    (0x2D4853, ('\u{7001}', false)), // East Asian ideograph
    (0x224A3D, ('\u{6DA4}', false)), // East Asian ideograph
    (0x213B56, ('\u{5C48}', false)), // East Asian ideograph
    (0x6F5051, ('\u{BB4D}', false)), // Korean hangul
    (0x275F3C, ('\u{9647}', false)), // East Asian ideograph
    (0x334277, ('\u{65EF}', false)), // East Asian ideograph
    (0x213B58, ('\u{5C4B}', false)), // East Asian ideograph
    (0x213B59, ('\u{5C4D}', false)), // East Asian ideograph
    (0x23316B, ('\u{89FF}', false)), // East Asian ideograph
    (0x213B5A, ('\u{5C55}', false)), // East Asian ideograph
    (0x213B5B, ('\u{5C51}', false)), // East Asian ideograph
    (0x226424, ('\u{781B}', false)), // East Asian ideograph
    (0x216425, ('\u{5187}', false)), // East Asian ideograph
    (0x226426, ('\u{782C}', false)), // East Asian ideograph
    (0x226427, ('\u{7823}', false)), // East Asian ideograph
    (0x226428, ('\u{782B}', false)), // East Asian ideograph
    (0x216429, ('\u{4E28}', false)), // East Asian ideograph
    (0x22642A, ('\u{7829}', false)), // East Asian ideograph
    (0x22642D, ('\u{7822}', false)), // East Asian ideograph
    (0x21642E, ('\u{4E31}', false)), // East Asian ideograph
    (0x2D3748, ('\u{8B5F}', false)), // East Asian ideograph
    (0x226431, ('\u{7835}', false)), // East Asian ideograph
    (0x226432, ('\u{7833}', false)), // East Asian ideograph
    (0x226433, ('\u{782E}', false)), // East Asian ideograph
    (0x216434, ('\u{4E42}', false)), // East Asian ideograph
    (0x226435, ('\u{7820}', false)), // East Asian ideograph
    (0x216437, ('\u{738D}', false)), // East Asian ideograph
    (0x226438, ('\u{783D}', false)), // East Asian ideograph
    (0x22643B, ('\u{781F}', false)), // East Asian ideograph
    (0x21643C, ('\u{4E5C}', false)), // East Asian ideograph
    (0x22643D, ('\u{7831}', false)), // East Asian ideograph
    (0x21643F, ('\u{6C39}', false)), // East Asian ideograph
    (0x274168, ('\u{6320}', false)), // East Asian ideograph
    (0x6F5053, ('\u{BB50}', false)), // Korean hangul
    (0x275F3E, ('\u{53EA}', false)), // East Asian ideograph (duplicate simplified)
    (0x226444, ('\u{784D}', false)), // East Asian ideograph
    (0x216446, ('\u{4E85}', false)), // East Asian ideograph
    (0x273B61, ('\u{5C42}', false)), // East Asian ideograph
    (0x226448, ('\u{7848}', false)), // East Asian ideograph
    (0x226449, ('\u{7853}', false)), // East Asian ideograph
    (0x22644A, ('\u{7854}', false)), // East Asian ideograph
    (0x22644B, ('\u{7845}', false)), // East Asian ideograph
    (0x22644C, ('\u{7852}', false)), // East Asian ideograph
    (0x295938, ('\u{9CDF}', false)), // East Asian ideograph
    (0x22644E, ('\u{7850}', false)), // East Asian ideograph
    (0x22644F, ('\u{7858}', false)), // East Asian ideograph
    (0x216450, ('\u{4EA0}', false)), // East Asian ideograph
    (0x216451, ('\u{4EA2}', false)), // East Asian ideograph
    (0x226452, ('\u{7847}', false)), // East Asian ideograph
    (0x233B63, ('\u{8F27}', false)), // East Asian ideograph
    (0x216455, ('\u{4EB6}', false)), // East Asian ideograph (variant of 4B6455 which maps to 4EB6)
    (0x226456, ('\u{784C}', false)), // East Asian ideograph
    (0x695630, ('\u{5CBC}', false)), // East Asian ideograph
    (0x216458, ('\u{4EB9}', false)), // East Asian ideograph
    (0x223B64, ('\u{67B7}', false)), // East Asian ideograph
    (0x22645A, ('\u{7868}', false)), // East Asian ideograph
    (0x22645B, ('\u{786D}', false)), // East Asian ideograph
    (0x21645E, ('\u{4EC9}', false)), // East Asian ideograph
    (0x226460, ('\u{7864}', false)), // East Asian ideograph
    (0x226461, ('\u{785C}', false)), // East Asian ideograph
    (0x216462, ('\u{4ECE}', false)), // East Asian ideograph (not in Unicode)
    (0x216463, ('\u{4EE8}', false)), // East Asian ideograph
    (0x215852, ('\u{8A54}', false)), // East Asian ideograph
    (0x213639, ('\u{54FA}', false)), // East Asian ideograph
    (0x226466, ('\u{786A}', false)), // East Asian ideograph
    (0x226469, ('\u{7886}', false)), // East Asian ideograph
    (0x21646B, ('\u{4EE1}', false)), // East Asian ideograph
    (0x22646C, ('\u{787F}', false)), // East Asian ideograph
    (0x22646D, ('\u{7887}', false)), // East Asian ideograph
    (0x213C2A, ('\u{5D84}', false)), // East Asian ideograph
    (0x226470, ('\u{7894}', false)), // East Asian ideograph
    (0x696471, ('\u{7CC0}', false)), // East Asian ideograph
    (0x216472, ('\u{4F08}', false)), // East Asian ideograph
    (0x216473, ('\u{4F0E}', false)), // East Asian ideograph
    (0x696474, ('\u{7CD8}', false)), // East Asian ideograph
    (0x216475, ('\u{4F03}', false)), // East Asian ideograph
    (0x226476, ('\u{788F}', false)), // East Asian ideograph
    (0x234F44, ('\u{982F}', false)), // East Asian ideograph
    (0x6F544A, ('\u{C378}', false)), // Korean hangul
    (0x21647C, ('\u{4F22}', false)), // East Asian ideograph
    (0x22647E, ('\u{7899}', false)), // East Asian ideograph
    (0x213B6B, ('\u{5CB7}', false)), // East Asian ideograph
    (0x225C52, ('\u{74E7}', false)), // East Asian ideograph
    (0x27516D, ('\u{603B}', false)), // East Asian ideograph
    (0x223B6D, ('\u{6802}', false)), // East Asian ideograph
    (0x695632, ('\u{5CC5}', false)), // East Asian ideograph
    (0x213B6E, ('\u{5CA1}', false)), // East Asian ideograph
    (0x4C3A5B, ('\u{6859}', false)), // East Asian ideograph
    (0x213B6F, ('\u{5CAB}', false)), // East Asian ideograph
    (0x6F5056, ('\u{BB61}', false)), // Korean hangul
    (0x294969, ('\u{961A}', false)), // East Asian ideograph
    (0x215854, ('\u{8A50}', false)), // East Asian ideograph
    (0x21363B, ('\u{54EE}', false)), // East Asian ideograph
    (0x21762A, ('\u{57FB}', false)), // East Asian ideograph
    (0x27583F, ('\u{8BAA}', false)), // East Asian ideograph
    (0x213B71, ('\u{5CB1}', false)), // East Asian ideograph
    (0x6F5961, ('\u{CE21}', false)), // Korean hangul
    (0x213B72, ('\u{5CD9}', false)), // East Asian ideograph
    (0x275F2C, ('\u{9636}', false)), // East Asian ideograph
    (0x223B74, ('\u{67DF}', false)), // East Asian ideograph
    (0x6F5057, ('\u{BB63}', false)), // Korean hangul
    (0x233B75, ('\u{8F17}', false)), // East Asian ideograph
    (0x23576B, ('\u{9BBB}', false)), // East Asian ideograph
    (0x213B77, ('\u{5CE8}', false)), // East Asian ideograph
    (0x216622, ('\u{4FE4}', false)), // East Asian ideograph
    (0x6F4E53, ('\u{B729}', false)), // Korean hangul
    (0x223B78, ('\u{6806}', false)), // East Asian ideograph
    (0x223B79, ('\u{67AE}', false)), // East Asian ideograph
    (0x213B7A, ('\u{5CEA}', false)), // East Asian ideograph
    (0x336054, ('\u{985B}', false)), // East Asian ideograph
    (0x213B7B, ('\u{5D07}', false)), // East Asian ideograph
    (0x27527A, ('\u{804B}', false)), // East Asian ideograph
    (0x213B7C, ('\u{5D06}', false)), // East Asian ideograph
    (0x216627, ('\u{4FC5}', false)), // East Asian ideograph
    (0x6F773E, ('\u{CB4C}', false)), // Korean hangul
    (0x692435, ('\u{3055}', false)), // Hiragana letter SA
    (0x233B7D, ('\u{8F2D}', false)), // East Asian ideograph
    (0x455746, ('\u{672F}', false)), // East Asian ideograph
    (0x213B7E, ('\u{5D16}', false)), // East Asian ideograph
    (0x6F5059, ('\u{BB8C}', false)), // Korean hangul
    (0x216629, ('\u{4FC9}', false)), // East Asian ideograph
    (0x4B516A, ('\u{7EF7}', false)), // East Asian ideograph
    (0x6F7737, ('\u{C5AB}', false)), // Korean hangul
    (0x6F5A67, ('\u{D0A5}', false)), // Korean hangul
    (0x6F5D23, ('\u{D5DD}', false)), // Korean hangul
    (0x2D485C, ('\u{6F44}', false)), // East Asian ideograph
    (0x6F505A, ('\u{BBA4}', false)), // Korean hangul
    (0x21363F, ('\u{54E9}', false)), // East Asian ideograph
    (0x22262F, ('\u{5DAE}', false)), // East Asian ideograph
    (0x27516E, ('\u{7EB5}', false)), // East Asian ideograph
    (0x283462, ('\u{6322}', false)), // East Asian ideograph
    (0x334C7B, ('\u{767A}', false)), // East Asian ideograph
    (0x216527, ('\u{4EF5}', false)), // East Asian ideograph
    (0x216528, ('\u{4F07}', false)), // East Asian ideograph
    (0x226529, ('\u{7893}', false)), // East Asian ideograph
    (0x21652A, ('\u{4F00}', false)), // East Asian ideograph
    (0x21652C, ('\u{4F0B}', false)), // East Asian ideograph
    (0x22652D, ('\u{7896}', false)), // East Asian ideograph
    (0x22652F, ('\u{78B2}', false)), // East Asian ideograph
    (0x6F5371, ('\u{C288}', false)), // Korean hangul
    (0x226531, ('\u{78A1}', false)), // East Asian ideograph
    (0x216532, ('\u{4F3B}', false)), // East Asian ideograph
    (0x292633, ('\u{84E3}', false)), // East Asian ideograph
    (0x216536, ('\u{4F58}', false)), // East Asian ideograph
    (0x216537, ('\u{4F62}', false)), // East Asian ideograph
    (0x216539, ('\u{4F64}', false)), // East Asian ideograph
    (0x21653A, ('\u{4F49}', false)), // East Asian ideograph
    (0x22653B, ('\u{78A4}', false)), // East Asian ideograph
    (0x22653E, ('\u{78B4}', false)), // East Asian ideograph
    (0x21653F, ('\u{4F3E}', false)), // East Asian ideograph
    (0x226540, ('\u{78AD}', false)), // East Asian ideograph
    (0x226541, ('\u{78A3}', false)), // East Asian ideograph
    (0x226543, ('\u{789E}', false)), // East Asian ideograph
    (0x226544, ('\u{78A8}', false)), // East Asian ideograph
    (0x232636, ('\u{857B}', false)), // East Asian ideograph
    (0x214B5E, ('\u{746A}', false)), // East Asian ideograph
    (0x226548, ('\u{78AB}', false)), // East Asian ideograph
    (0x234F4B, ('\u{9847}', false)), // East Asian ideograph
    (0x4B6637, ('\u{4FE3}', false)), // East Asian ideograph
    (0x21654D, ('\u{4F68}', false)), // East Asian ideograph
    (0x22654E, ('\u{78BB}', false)), // East Asian ideograph
    (0x21654F, ('\u{4F5F}', false)), // East Asian ideograph
    (0x6F505C, ('\u{BBC4}', false)), // Korean hangul
    (0x29496F, ('\u{95FC}', false)), // East Asian ideograph
    (0x226555, ('\u{78CC}', false)), // East Asian ideograph
    (0x226556, ('\u{78C9}', false)), // East Asian ideograph
    (0x216557, ('\u{4F7C}', false)), // East Asian ideograph
    (0x226558, ('\u{78D1}', false)), // East Asian ideograph
    (0x21655A, ('\u{4F98}', false)), // East Asian ideograph
    (0x21655B, ('\u{4F92}', false)), // East Asian ideograph
    (0x21655C, ('\u{4F7D}', false)), // East Asian ideograph
    (0x22655E, ('\u{78C8}', false)), // East Asian ideograph
    (0x226560, ('\u{78D4}', false)), // East Asian ideograph
    (0x274E3B, ('\u{781A}', false)), // East Asian ideograph
    (0x216562, ('\u{4F76}', false)), // East Asian ideograph
    (0x216564, ('\u{4FA2}', false)), // East Asian ideograph
    (0x4C6565, ('\u{78B9}', false)), // East Asian ideograph
    (0x216566, ('\u{4F91}', false)), // East Asian ideograph
    (0x216567, ('\u{4F95}', false)), // East Asian ideograph
    (0x226568, ('\u{78DF}', false)), // East Asian ideograph
    (0x22656A, ('\u{78E7}', false)), // East Asian ideograph
    (0x21656C, ('\u{4F4C}', false)), // East Asian ideograph
    (0x21656D, ('\u{4F97}', false)), // East Asian ideograph
    (0x22656E, ('\u{78DB}', false)), // East Asian ideograph
    (0x22656F, ('\u{78E1}', false)), // East Asian ideograph
    (0x216570, ('\u{4F79}', false)), // East Asian ideograph
    (0x216571, ('\u{4F9A}', false)), // East Asian ideograph
    (0x216572, ('\u{4F81}', false)), // East Asian ideograph
    (0x216573, ('\u{4F78}', false)), // East Asian ideograph
    (0x225C5A, ('\u{74F0}', false)), // East Asian ideograph
    (0x4B516E, ('\u{7E26}', false)), // East Asian ideograph
    (0x226576, ('\u{78EE}', false)), // East Asian ideograph
    (0x226577, ('\u{78E3}', false)), // East Asian ideograph
    (0x226579, ('\u{78F2}', false)), // East Asian ideograph
    (0x21657B, ('\u{4F7A}', false)), // East Asian ideograph
    (0x21657C, ('\u{4FCD}', false)), // East Asian ideograph
    (0x2D5529, ('\u{830E}', false)), // East Asian ideograph (variant of 275529)
    (0x22657E, ('\u{7905}', false)), // East Asian ideograph
    (0x6F5D27, ('\u{D5EC}', false)), // Korean hangul
    (0x6F505E, ('\u{BBD0}', false)), // Korean hangul
    (0x217A75, ('\u{5A16}', false)), // East Asian ideograph
    (0x224F5D, ('\u{7043}', false)), // East Asian ideograph
    (0x216643, ('\u{4FB9}', false)), // East Asian ideograph
    (0x232644, ('\u{8597}', false)), // East Asian ideograph
    (0x283466, ('\u{63FF}', false)), // East Asian ideograph
    (0x6F5D28, ('\u{D5F4}', false)), // Korean hangul
    (0x287269, ('\u{7EC9}', false)), // East Asian ideograph
    (0x2D3C38, ('\u{9245}', false)), // East Asian ideograph
    (0x216646, ('\u{501E}', false)), // East Asian ideograph
    (0x217E25, ('\u{5B67}', false)), // East Asian ideograph
    (0x6F4B42, ('\u{B059}', false)), // Korean hangul
    (0x6F505F, ('\u{BBF8}', false)), // Korean hangul
    (0x282647, ('\u{5CBF}', false)), // East Asian ideograph
    (0x275F4A, ('\u{867D}', false)), // East Asian ideograph
    (0x225C5C, ('\u{74EE}', false)), // East Asian ideograph
    (0x217633, ('\u{5800}', false)), // East Asian ideograph
    (0x23605B, ('\u{9F8E}', false)), // East Asian ideograph
    (0x276649, ('\u{4F1C}', false)), // East Asian ideograph
    (0x274E3E, ('\u{7815}', false)), // East Asian ideograph
    (0x293866, ('\u{8DB1}', false)), // East Asian ideograph
    (0x29563C, ('\u{9B49}', false)), // East Asian ideograph
    (0x6F5C73, ('\u{D585}', false)), // Korean hangul
    (0x4B3C21, ('\u{5D5C}', false)), // East Asian ideograph
    (0x21664C, ('\u{5007}', false)), // East Asian ideograph
    (0x21664D, ('\u{5013}', false)), // East Asian ideograph
    (0x23264E, ('\u{8586}', false)), // East Asian ideograph
    (0x6F5D2A, ('\u{D5F7}', false)), // Korean hangul
    (0x222650, ('\u{5DDB}', false)), // East Asian ideograph
    (0x22606A, ('\u{76AA}', false)), // East Asian ideograph
    (0x292651, ('\u{84DF}', false)), // East Asian ideograph
    (0x275F4C, ('\u{9E21}', false)), // East Asian ideograph
    (0x696466, ('\u{7CAD}', false)), // East Asian ideograph
    (0x217635, ('\u{57EC}', false)), // East Asian ideograph
    (0x6F5264, ('\u{C0B3}', false)), // Korean hangul
    (0x393770, ('\u{56F2}', false)), // East Asian ideograph
    (0x2D552D, ('\u{8358}', false)), // East Asian ideograph
    (0x6F5D2B, ('\u{D5F9}', false)), // Korean hangul
    (0x4B4066, ('\u{62F4}', false)), // East Asian ideograph
    (0x286655, ('\u{783B}', false)), // East Asian ideograph
    (0x4B3C23, ('\u{5CE5}', false)), // East Asian ideograph
    (0x274177, ('\u{631E}', false)), // East Asian ideograph
    (0x222656, ('\u{5DE4}', false)), // East Asian ideograph
    (0x217636, ('\u{5807}', false)), // East Asian ideograph
    (0x292658, ('\u{83B6}', false)), // East Asian ideograph
    (0x282659, ('\u{5DEF}', false)), // East Asian ideograph
    (0x692432, ('\u{3052}', false)), // Hiragana letter GE
    (0x276068, ('\u{996C}', false)), // East Asian ideograph
    (0x706D3B, ('\u{7818}', false)), // East Asian ideograph
    (0x226621, ('\u{78F9}', false)), // East Asian ideograph
    (0x226622, ('\u{78FD}', false)), // East Asian ideograph
    (0x6F5063, ('\u{BC00}', false)), // Korean hangul
    (0x216626, ('\u{4FB7}', false)), // East Asian ideograph
    (0x226627, ('\u{78FE}', false)), // East Asian ideograph
    (0x226629, ('\u{78FB}', false)), // East Asian ideograph
    (0x21662A, ('\u{4FE5}', false)), // East Asian ideograph
    (0x22662B, ('\u{7904}', false)), // East Asian ideograph
    (0x21662C, ('\u{4FE7}', false)), // East Asian ideograph
    (0x22662E, ('\u{7912}', false)), // East Asian ideograph
    (0x226632, ('\u{790C}', false)), // East Asian ideograph
    (0x216633, ('\u{4FDC}', false)), // East Asian ideograph
    (0x226634, ('\u{7913}', false)), // East Asian ideograph
    (0x216635, ('\u{4FD4}', false)), // East Asian ideograph
    (0x216637, ('\u{4FC1}', false)), // East Asian ideograph
    (0x21663B, ('\u{4FDB}', false)), // East Asian ideograph
    (0x21663E, ('\u{4FC6}', false)), // East Asian ideograph
    (0x706640, ('\u{80EC}', false)), // East Asian ideograph
    (0x6F5064, ('\u{BC08}', false)), // Korean hangul
    (0x226643, ('\u{791E}', false)), // East Asian ideograph
    (0x6F4C21, ('\u{B128}', false)), // Korean hangul
    (0x226646, ('\u{7922}', false)), // East Asian ideograph
    (0x292661, ('\u{8360}', false)), // East Asian ideograph
    (0x216648, ('\u{503F}', false)), // East Asian ideograph
    (0x216649, ('\u{5005}', false)), // East Asian ideograph
    (0x4D5973, ('\u{51EB}', false)), // East Asian ideograph
    (0x22664C, ('\u{7924}', false)), // East Asian ideograph
    (0x22664D, ('\u{7927}', false)), // East Asian ideograph
    (0x21664E, ('\u{5022}', false)), // East Asian ideograph
    (0x226650, ('\u{7929}', false)), // East Asian ideograph
    (0x216652, ('\u{4FF5}', false)), // East Asian ideograph
    (0x6F2463, ('\u{314D}', false)), // Korean hangul
    (0x226655, ('\u{7931}', false)), // East Asian ideograph
    (0x393428, ('\u{5227}', false)), // East Asian ideograph
    (0x276235, ('\u{9E26}', false)), // East Asian ideograph
    (0x216659, ('\u{4FF4}', false)), // East Asian ideograph
    (0x21665B, ('\u{5037}', false)), // East Asian ideograph
    (0x22665D, ('\u{7934}', false)), // East Asian ideograph
    (0x21665E, ('\u{502E}', false)), // East Asian ideograph
    (0x6F5065, ('\u{BC09}', false)), // Korean hangul
    (0x226660, ('\u{7936}', false)), // East Asian ideograph
    (0x216661, ('\u{4FF6}', false)), // East Asian ideograph
    (0x216662, ('\u{501C}', false)), // East Asian ideograph
    (0x6F4C22, ('\u{B12C}', false)), // Korean hangul
    (0x226665, ('\u{793D}', false)), // East Asian ideograph
    (0x216666, ('\u{502C}', false)), // East Asian ideograph
    (0x226667, ('\u{7942}', false)), // East Asian ideograph
    (0x226668, ('\u{793F}', false)), // East Asian ideograph
    (0x216669, ('\u{5010}', false)), // East Asian ideograph
    (0x22666A, ('\u{794A}', false)), // East Asian ideograph
    (0x22666B, ('\u{794D}', false)), // East Asian ideograph
    (0x292668, ('\u{8369}', false)), // East Asian ideograph
    (0x226675, ('\u{7946}', false)), // East Asian ideograph
    (0x226677, ('\u{7958}', false)), // East Asian ideograph
    (0x216679, ('\u{503D}', false)), // East Asian ideograph
    (0x22667A, ('\u{795C}', false)), // East Asian ideograph
    (0x22667B, ('\u{794F}', false)), // East Asian ideograph
    (0x22667C, ('\u{7953}', false)), // East Asian ideograph
    (0x22667D, ('\u{7953}', false)), // Unrelated variant of EACC 22667C which maps to 7953
    (0x6F4C23, ('\u{B134}', false)), // Korean hangul
    (0x225C63, ('\u{74F8}', false)), // East Asian ideograph
    (0x236062, ('\u{9F95}', false)), // East Asian ideograph
    (0x2D336B, ('\u{5C05}', false)), // East Asian ideograph
    (0x213F71, ('\u{6233}', false)), // East Asian ideograph
    (0x6F5D30, ('\u{D610}', false)), // Korean hangul
    (0x224B57, ('\u{6EA8}', false)), // East Asian ideograph
    (0x27627D, ('\u{9F50}', false)), // East Asian ideograph
    (0x6F5D77, ('\u{D774}', false)), // Korean hangul
    (0x213B2E, ('\u{5BE2}', false)), // East Asian ideograph
    (0x6F4C24, ('\u{B135}', false)), // Korean hangul
    (0x21763B, ('\u{580F}', false)), // East Asian ideograph
    (0x227A3A, ('\u{81CA}', false)), // East Asian ideograph
    (0x232672, ('\u{85BF}', false)), // East Asian ideograph
    (0x6F5528, ('\u{C557}', false)), // Korean hangul
    (0x6F5068, ('\u{BC0D}', false)), // Korean hangul
    (0x6F4C25, ('\u{B137}', false)), // Korean hangul
    (0x295A48, ('\u{9E31}', false)), // East Asian ideograph
    (0x395A36, ('\u{983C}', false)), // East Asian ideograph
    (0x6F4939, ('\u{ACA1}', false)), // Korean hangul
    (0x275121, ('\u{7EB1}', false)), // East Asian ideograph
    (0x222677, ('\u{5E12}', false)), // East Asian ideograph
    (0x225122, ('\u{70DD}', false)), // East Asian ideograph
    (0x225123, ('\u{70E1}', false)), // East Asian ideograph
    (0x27417E, ('\u{62E9}', false)), // East Asian ideograph
    (0x226679, ('\u{795B}', false)), // East Asian ideograph
    (0x275F54, ('\u{4E91}', false)), // East Asian ideograph
    (0x235124, ('\u{9907}', false)), // East Asian ideograph
    (0x6F4C26, ('\u{B140}', false)), // Korean hangul
    (0x225C66, ('\u{74FB}', false)), // East Asian ideograph
    (0x275125, ('\u{7EB7}', false)), // East Asian ideograph
    (0x4B3869, ('\u{5727}', false)), // East Asian ideograph
    (0x235C22, ('\u{9DC7}', false)), // East Asian ideograph
    (0x225126, ('\u{70E3}', false)), // East Asian ideograph
    (0x6F5D33, ('\u{D614}', false)), // Korean hangul
    (0x6F5127, ('\u{BC85}', false)), // Korean hangul
    (0x235128, ('\u{9902}', false)), // East Asian ideograph
    (0x6F5374, ('\u{C298}', false)), // Korean hangul
    (0x6F506A, ('\u{BC11}', false)), // Korean hangul
    (0x275129, ('\u{624E}', false)), // East Asian ideograph
    (0x277D2B, ('\u{5A06}', false)), // East Asian ideograph
    (0x225C67, ('\u{74FF}', false)), // East Asian ideograph
    (0x27512A, ('\u{7ECD}', false)), // East Asian ideograph
    (0x21512B, ('\u{7D44}', false)), // East Asian ideograph
    (0x6F4F31, ('\u{B82C}', false)), // Korean hangul
    (0x27413F, ('\u{626C}', false)), // East Asian ideograph
    (0x6F5D34, ('\u{D615}', false)), // Korean hangul
    (0x27512C, ('\u{7EC6}', false)), // East Asian ideograph
    (0x27512D, ('\u{7EC5}', false)), // East Asian ideograph
    (0x4C7328, ('\u{5FAD}', false)), // East Asian ideograph (variant of 2E7328 which maps to 5FAD)
    (0x22512E, ('\u{70D1}', false)), // East Asian ideograph
    (0x215869, ('\u{8AAA}', false)), // East Asian ideograph
    (0x21512F, ('\u{7D40}', false)), // East Asian ideograph
    (0x215130, ('\u{7D42}', false)), // East Asian ideograph
    (0x216722, ('\u{506F}', false)), // East Asian ideograph
    (0x216723, ('\u{5050}', false)), // East Asian ideograph
    (0x216725, ('\u{5070}', false)), // East Asian ideograph
    (0x215131, ('\u{7D71}', false)), // East Asian ideograph
    (0x216729, ('\u{5053}', false)), // East Asian ideograph
    (0x21672A, ('\u{506A}', false)), // East Asian ideograph
    (0x21672C, ('\u{5056}', false)), // East Asian ideograph
    (0x215132, ('\u{7D5E}', false)), // East Asian ideograph
    (0x226730, ('\u{7972}', false)), // East Asian ideograph
    (0x216731, ('\u{506D}', false)), // East Asian ideograph
    (0x275133, ('\u{7ED2}', false)), // East Asian ideograph
    (0x6F4C29, ('\u{B150}', false)), // Korean hangul
    (0x216738, ('\u{505D}', false)), // East Asian ideograph
    (0x215134, ('\u{7D50}', false)), // East Asian ideograph
    (0x21673B, ('\u{5058}', false)), // East Asian ideograph
    (0x21673C, ('\u{5072}', false)), // East Asian ideograph
    (0x22673E, ('\u{797C}', false)), // East Asian ideograph
    (0x3F6179, ('\u{5C1F}', false)), // East Asian ideograph
    (0x216741, ('\u{5041}', false)), // East Asian ideograph
    (0x6F5D36, ('\u{D638}', false)), // Korean hangul
    (0x275136, ('\u{7EDA}', false)), // East Asian ideograph
    (0x216746, ('\u{5015}', false)), // East Asian ideograph
    (0x293430, ('\u{8BB4}', false)), // East Asian ideograph
    (0x216748, ('\u{507A}', false)), // East Asian ideograph
    (0x21674A, ('\u{506C}', false)), // East Asian ideograph
    (0x275137, ('\u{7EDD}', false)), // East Asian ideograph
    (0x21674D, ('\u{506B}', false)), // East Asian ideograph
    (0x21674E, ('\u{5094}', false)), // East Asian ideograph
    (0x22674F, ('\u{798B}', false)), // East Asian ideograph
    (0x216750, ('\u{509E}', false)), // East Asian ideograph
    (0x235138, ('\u{9915}', false)), // East Asian ideograph
    (0x216752, ('\u{509B}', false)), // East Asian ideograph
    (0x216753, ('\u{509A}', false)), // East Asian ideograph
    (0x226754, ('\u{7994}', false)), // East Asian ideograph
    (0x226755, ('\u{7993}', false)), // East Asian ideograph
    (0x215139, ('\u{7D66}', false)), // East Asian ideograph
    (0x21675A, ('\u{508C}', false)), // East Asian ideograph
    (0x21675C, ('\u{5088}', false)), // East Asian ideograph
    (0x23513A, ('\u{9924}', false)), // East Asian ideograph
    (0x22675F, ('\u{79A1}', false)), // East Asian ideograph
    (0x226760, ('\u{799B}', false)), // East Asian ideograph
    (0x226761, ('\u{79A3}', false)), // East Asian ideograph
    (0x216762, ('\u{508E}', false)), // East Asian ideograph
    (0x23513B, ('\u{991F}', false)), // East Asian ideograph
    (0x224B5E, ('\u{6E8E}', false)), // East Asian ideograph
    (0x216767, ('\u{50A6}', false)), // East Asian ideograph
    (0x274C76, ('\u{763E}', false)), // East Asian ideograph
    (0x21513C, ('\u{7D93}', false)), // East Asian ideograph
    (0x21676A, ('\u{5092}', false)), // East Asian ideograph
    (0x21676C, ('\u{509C}', false)), // East Asian ideograph
    (0x2D442D, ('\u{6780}', false)), // East Asian ideograph
    (0x22676E, ('\u{79A9}', false)), // East Asian ideograph
    (0x27513D, ('\u{6346}', false)), // East Asian ideograph
    (0x226770, ('\u{79AB}', false)), // East Asian ideograph
    (0x216771, ('\u{50C7}', false)), // East Asian ideograph
    (0x216775, ('\u{50C9}', false)), // East Asian ideograph
    (0x22677A, ('\u{79B3}', false)), // East Asian ideograph
    (0x22513F, ('\u{70FA}', false)), // East Asian ideograph
    (0x21677C, ('\u{50B4}', false)), // East Asian ideograph
    (0x6F5D38, ('\u{D63C}', false)), // Korean hangul
    (0x212A23, ('\u{E8D2}', false)), // EACC component character
    (0x215140, ('\u{7D81}', false)), // East Asian ideograph
    (0x334F5E, ('\u{7A91}', false)), // East Asian ideograph
    (0x215141, ('\u{7D9C}', false)), // East Asian ideograph
    (0x6F5375, ('\u{C29B}', false)), // Korean hangul
    (0x213538, ('\u{53F0}', false)), // East Asian ideograph (duplicate simplified)
    (0x6F506F, ('\u{BC16}', false)), // Korean hangul
    (0x215142, ('\u{7DBB}', false)), // East Asian ideograph
    (0x6F4C2C, ('\u{B154}', false)), // Korean hangul
    (0x284140, ('\u{6861}', false)), // East Asian ideograph
    (0x235143, ('\u{9929}', false)), // East Asian ideograph
    (0x2D3765, ('\u{8086}', false)), // East Asian ideograph
    (0x215144, ('\u{7DCA}', false)), // East Asian ideograph
    (0x6F5D39, ('\u{D640}', false)), // Korean hangul
    (0x215145, ('\u{7DBE}', false)), // East Asian ideograph
    (0x224B60, ('\u{6ED9}', false)), // East Asian ideograph
    (0x215146, ('\u{7DB4}', false)), // East Asian ideograph
    (0x2E2D79, ('\u{6128}', false)), // East Asian ideograph
    (0x4B5724, ('\u{86CD}', false)), // East Asian ideograph
    (0x235147, ('\u{991A}', false)), // East Asian ideograph
    (0x275242, ('\u{4E49}', false)), // East Asian ideograph
    (0x6F4C2D, ('\u{B155}', false)), // Korean hangul
    (0x6F5C48, ('\u{D46F}', false)), // Korean hangul
    (0x215148, ('\u{7DB2}', false)), // East Asian ideograph
    (0x215149, ('\u{7DB1}', false)), // East Asian ideograph
    (0x6F5D3A, ('\u{D648}', false)), // Korean hangul
    (0x21514A, ('\u{7DBD}', false)), // East Asian ideograph
    (0x224B61, ('\u{6EBD}', false)), // East Asian ideograph
    (0x234F60, ('\u{9852}', false)), // East Asian ideograph
    (0x4B3C32, ('\u{5DD3}', false)), // East Asian ideograph
    (0x6F5071, ('\u{BC1B}', false)), // Korean hangul
    (0x22514C, ('\u{7103}', false)), // East Asian ideograph
    (0x21586F, ('\u{8AA3}', false)), // East Asian ideograph
    (0x21514D, ('\u{7DA2}', false)), // East Asian ideograph
    (0x22582B, ('\u{736B}', false)), // East Asian ideograph
    (0x4B615F, ('\u{9AEA}', false)), // East Asian ideograph
    (0x21514E, ('\u{7DAD}', false)), // East Asian ideograph
    (0x2D3324, ('\u{634C}', false)), // East Asian ideograph
    (0x6F5D3B, ('\u{D649}', false)), // Korean hangul
    (0x21514F, ('\u{7DBF}', false)), // East Asian ideograph
    (0x2D356A, ('\u{8A36}', false)), // East Asian ideograph
    (0x215150, ('\u{7DB8}', false)), // East Asian ideograph
    (0x6F5072, ('\u{BC1C}', false)), // Korean hangul
    (0x215151, ('\u{7DC7}', false)), // East Asian ideograph
    (0x22482D, ('\u{6CF2}', false)), // East Asian ideograph
    (0x275152, ('\u{7F14}', false)), // East Asian ideograph
    (0x6F493B, ('\u{ACA9}', false)), // Korean hangul
    (0x2D3768, ('\u{56EC}', false)), // East Asian ideograph
    (0x215153, ('\u{7DEF}', false)), // East Asian ideograph
    (0x294346, ('\u{94D1}', false)), // East Asian ideograph
    (0x2F5D3C, ('\u{6EF7}', false)), // East Asian ideograph
    (0x215154, ('\u{7DF4}', false)), // East Asian ideograph (variant of 4B5154 which maps to 7DF4)
    (0x224B63, ('\u{6EC1}', false)), // East Asian ideograph
    (0x234F62, ('\u{984B}', false)), // East Asian ideograph
    (0x235155, ('\u{9932}', false)), // East Asian ideograph
    (0x6F5073, ('\u{BC1D}', false)), // Korean hangul
    (0x225156, ('\u{7112}', false)), // East Asian ideograph
    (0x6F4C30, ('\u{B178}', false)), // Korean hangul
    (0x69675C, ('\u{825D}', false)), // East Asian ideograph
    (0x215157, ('\u{7DEC}', false)), // East Asian ideograph
    (0x234179, ('\u{91E9}', false)), // East Asian ideograph
    (0x215158, ('\u{7DDD}', false)), // East Asian ideograph
    (0x213E37, ('\u{6012}', false)), // East Asian ideograph
    (0x6F5D3D, ('\u{D64D}', false)), // Korean hangul
    (0x215159, ('\u{7DE9}', false)), // East Asian ideograph
    (0x21515A, ('\u{7DE3}', false)), // East Asian ideograph
    (0x213539, ('\u{53E5}', false)), // East Asian ideograph
    (0x6F5074, ('\u{BC1F}', false)), // Korean hangul
    (0x216822, ('\u{50C2}', false)), // East Asian ideograph
    (0x27515B, ('\u{7F16}', false)), // East Asian ideograph
    (0x226825, ('\u{79BC}', false)), // East Asian ideograph
    (0x225C71, ('\u{7505}', false)), // East Asian ideograph
    (0x226828, ('\u{79C6}', false)), // East Asian ideograph
    (0x22515C, ('\u{710C}', false)), // East Asian ideograph
    (0x22682A, ('\u{79C8}', false)), // East Asian ideograph
    (0x21682C, ('\u{50BA}', false)), // East Asian ideograph
    (0x22682D, ('\u{79D4}', false)), // East Asian ideograph
    (0x21682E, ('\u{50CD}', false)), // East Asian ideograph
    (0x21515D, ('\u{7D9E}', false)), // East Asian ideograph
    (0x6F4F33, ('\u{B835}', false)), // Korean hangul
    (0x226832, ('\u{79D6}', false)), // East Asian ideograph
    (0x216834, ('\u{50EF}', false)), // East Asian ideograph
    (0x21515E, ('\u{7DDE}', false)), // East Asian ideograph
    (0x293438, ('\u{8C29}', false)), // East Asian ideograph
    (0x21683A, ('\u{50F4}', false)), // East Asian ideograph
    (0x21515F, ('\u{7E11}', false)), // East Asian ideograph
    (0x21683C, ('\u{50DD}', false)), // East Asian ideograph
    (0x22683D, ('\u{79EC}', false)), // East Asian ideograph
    (0x22683E, ('\u{79EB}', false)), // East Asian ideograph (variant of 4C683E which maps to 79EB)
    (0x6F5075, ('\u{BC24}', false)), // Korean hangul
    (0x215160, ('\u{7E0A}', false)), // East Asian ideograph
    (0x226842, ('\u{79E1}', false)), // East Asian ideograph
    (0x6F4C32, ('\u{B17A}', false)), // Korean hangul
    (0x226844, ('\u{79DD}', false)), // East Asian ideograph
    (0x226845, ('\u{79ED}', false)), // East Asian ideograph
    (0x216846, ('\u{50D9}', false)), // East Asian ideograph
    (0x215161, ('\u{7E08}', false)), // East Asian ideograph
    (0x226848, ('\u{79F8}', false)), // East Asian ideograph
    (0x215162, ('\u{7E1B}', false)), // East Asian ideograph
    (0x2E684E, ('\u{8020}', false)), // East Asian ideograph
    (0x22684F, ('\u{7A02}', false)), // East Asian ideograph
    (0x226850, ('\u{7A0A}', false)), // East Asian ideograph
    (0x6F5D3F, ('\u{D654}', false)), // Korean hangul
    (0x6F5625, ('\u{C651}', false)), // Korean hangul
    (0x275163, ('\u{81F4}', false)), // East Asian ideograph
    (0x226854, ('\u{7A09}', false)), // East Asian ideograph
    (0x216855, ('\u{50EC}', false)), // East Asian ideograph
    (0x4B442D, ('\u{67A9}', false)), // East Asian ideograph
    (0x215164, ('\u{7E23}', false)), // East Asian ideograph
    (0x21685B, ('\u{510E}', false)), // East Asian ideograph
    (0x22685C, ('\u{7A03}', false)), // East Asian ideograph
    (0x6F5076, ('\u{BC25}', false)), // Korean hangul
    (0x275165, ('\u{7F29}', false)), // East Asian ideograph
    (0x226861, ('\u{7A0C}', false)), // East Asian ideograph
    (0x293160, ('\u{89EF}', false)), // East Asian ideograph
    (0x225166, ('\u{7113}', false)), // East Asian ideograph
    (0x216866, ('\u{5107}', false)), // East Asian ideograph
    (0x216867, ('\u{510F}', false)), // East Asian ideograph
    (0x216868, ('\u{50FE}', false)), // East Asian ideograph
    (0x216869, ('\u{510B}', false)), // East Asian ideograph
    (0x21686A, ('\u{50FD}', false)), // East Asian ideograph
    (0x22686B, ('\u{7A11}', false)), // East Asian ideograph
    (0x22686C, ('\u{7A18}', false)), // East Asian ideograph
    (0x21686D, ('\u{5101}', false)), // East Asian ideograph
    (0x234E43, ('\u{97C9}', false)), // East Asian ideograph
    (0x22686F, ('\u{7A19}', false)), // East Asian ideograph (variant of 2E686F which maps to 7A19)
    (0x6F5D40, ('\u{D655}', false)), // Korean hangul
    (0x226871, ('\u{7A1E}', false)), // East Asian ideograph
    (0x216872, ('\u{5113}', false)), // East Asian ideograph
    (0x234F66, ('\u{983F}', false)), // East Asian ideograph
    (0x226876, ('\u{7A17}', false)), // East Asian ideograph
    (0x275169, ('\u{7F27}', false)), // East Asian ideograph
    (0x216878, ('\u{511A}', false)), // East Asian ideograph
    (0x216879, ('\u{9797}', false)), // East Asian ideograph
    (0x6F5077, ('\u{BC27}', false)), // Korean hangul
    (0x21516A, ('\u{7E43}', false)), // East Asian ideograph
    (0x21687E, ('\u{5126}', false)), // East Asian ideograph
    (0x6F4C34, ('\u{B180}', false)), // Korean hangul
    (0x223A5B, ('\u{6745}', false)), // East Asian ideograph
    (0x33516B, ('\u{7DD0}', false)), // East Asian ideograph
    (0x4C735D, ('\u{7D4B}', false)), // East Asian ideograph
    (0x22516C, ('\u{711E}', false)), // East Asian ideograph
    (0x2E3729, ('\u{65B5}', false)), // East Asian ideograph
    (0x6F5D41, ('\u{D658}', false)), // Korean hangul
    (0x21516D, ('\u{7E3D}', false)), // East Asian ideograph
    (0x6F5451, ('\u{C3D8}', false)), // Korean hangul
    (0x22516E, ('\u{7120}', false)), // East Asian ideograph
    (0x2D4437, ('\u{67FE}', false)), // East Asian ideograph
    (0x21516F, ('\u{7E45}', false)), // East Asian ideograph
    (0x6F4C35, ('\u{B188}', false)), // Korean hangul
    (0x215170, ('\u{7E55}', false)), // East Asian ideograph
    (0x275174, ('\u{7F2D}', false)), // East Asian ideograph
    (0x235171, ('\u{994D}', false)), // East Asian ideograph
    (0x473539, ('\u{8B9E}', false)), // East Asian ideograph
    (0x29426D, ('\u{94CD}', false)), // East Asian ideograph
    (0x275172, ('\u{7EE3}', false)), // East Asian ideograph
    (0x224B69, ('\u{6EBB}', false)), // East Asian ideograph
    (0x275173, ('\u{7ED5}', false)), // East Asian ideograph
    (0x6F5B23, ('\u{D0F0}', false)), // Korean hangul
    (0x215174, ('\u{7E5A}', false)), // East Asian ideograph
    (0x6F4C36, ('\u{B189}', false)), // Korean hangul
    (0x225175, ('\u{712D}', false)), // East Asian ideograph
    (0x2D376F, ('\u{5700}', false)), // East Asian ideograph
    (0x275176, ('\u{7EF3}', false)), // East Asian ideograph
    (0x213C21, ('\u{5D0E}', false)), // East Asian ideograph
    (0x275177, ('\u{8327}', false)), // East Asian ideograph
    (0x2D3C22, ('\u{5D10}', false)), // East Asian ideograph
    (0x275178, ('\u{7ECE}', false)), // East Asian ideograph
    (0x223C23, ('\u{67C2}', false)), // East Asian ideograph
    (0x6F507A, ('\u{BC30}', false)), // Korean hangul
    (0x275179, ('\u{7ED8}', false)), // East Asian ideograph
    (0x215878, ('\u{8AD2}', false)), // East Asian ideograph
    (0x225C77, ('\u{7503}', false)), // East Asian ideograph
    (0x27517A, ('\u{8FAB}', false)), // East Asian ideograph
    (0x217C25, ('\u{5A9E}', false)), // East Asian ideograph
    (0x45465B, ('\u{6C2F}', false)), // East Asian ideograph
    (0x21517B, ('\u{7E7D}', false)), // East Asian ideograph
    (0x223C26, ('\u{67CA}', false)), // East Asian ideograph
    (0x274E59, ('\u{7840}', false)), // East Asian ideograph
    (0x6F5D44, ('\u{D667}', false)), // Korean hangul
    (0x6F517C, ('\u{BE61}', false)), // Korean hangul
    (0x213C27, ('\u{5D4C}', false)), // East Asian ideograph
    (0x234F6A, ('\u{985C}', false)), // East Asian ideograph
    (0x27517D, ('\u{7EE7}', false)), // East Asian ideograph
    (0x223C28, ('\u{67CE}', false)), // East Asian ideograph
    (0x2D443A, ('\u{6942}', false)), // East Asian ideograph
    (0x23517E, ('\u{9955}', false)), // East Asian ideograph
    (0x213B32, ('\u{5BE7}', false)), // East Asian ideograph
    (0x213C29, ('\u{5D69}', false)), // East Asian ideograph
    (0x223C2A, ('\u{67F2}', false)), // East Asian ideograph
    (0x275E3E, ('\u{94DB}', false)), // East Asian ideograph
    (0x2D5547, ('\u{837D}', false)), // East Asian ideograph
    (0x223C2B, ('\u{67C3}', false)), // East Asian ideograph
    (0x6F5D45, ('\u{D669}', false)), // Korean hangul
    (0x234F6B, ('\u{9859}', false)), // East Asian ideograph
    (0x223C2D, ('\u{67DD}', false)), // East Asian ideograph
    (0x6F507C, ('\u{BC34}', false)), // Korean hangul
    (0x275F67, ('\u{96FE}', false)), // East Asian ideograph
    (0x213C2E, ('\u{5DBD}', false)), // East Asian ideograph
    (0x6F4D43, ('\u{B3D0}', false)), // Korean hangul
    (0x233E5F, ('\u{90AD}', false)), // East Asian ideograph
    (0x213C2F, ('\u{5DBA}', false)), // East Asian ideograph (variant of 4B3C2F which maps to 5DBA)
    (0x6F493D, ('\u{ACAC}', false)), // Korean hangul
    (0x233C30, ('\u{8F46}', false)), // East Asian ideograph
    (0x226922, ('\u{7A2C}', false)), // East Asian ideograph
    (0x6F5D46, ('\u{D670}', false)), // Korean hangul
    (0x233C31, ('\u{8F4A}', false)), // East Asian ideograph
    (0x216929, ('\u{5124}', false)), // East Asian ideograph
    (0x6F5452, ('\u{C3D9}', false)), // Korean hangul
    (0x21692B, ('\u{5129}', false)), // East Asian ideograph
    (0x213C32, ('\u{5DD4}', false)), // East Asian ideograph
    (0x6F507D, ('\u{BC37}', false)), // Korean hangul
    (0x216930, ('\u{5131}', false)), // East Asian ideograph
    (0x273C33, ('\u{5CA9}', false)), // East Asian ideograph
    (0x29454D, ('\u{9534}', false)), // East Asian ideograph
    (0x275175, ('\u{7CFB}', false)), // East Asian ideograph (duplicate simplified)
    (0x226939, ('\u{7A48}', false)), // East Asian ideograph
    (0x22693D, ('\u{7A4B}', false)), // East Asian ideograph
    (0x22693E, ('\u{7A47}', false)), // East Asian ideograph
    (0x22693F, ('\u{7A44}', false)), // East Asian ideograph
    (0x274E5C, ('\u{77FE}', false)), // East Asian ideograph
    (0x6F5D47, ('\u{D671}', false)), // Korean hangul
    (0x216944, ('\u{513A}', false)), // East Asian ideograph
    (0x213C36, ('\u{5DE2}', false)), // East Asian ideograph
    (0x696946, ('\u{8630}', false)), // East Asian ideograph
    (0x216947, ('\u{5139}', false)), // East Asian ideograph
    (0x216948, ('\u{513B}', false)), // East Asian ideograph
    (0x213C37, ('\u{5DE5}', false)), // East Asian ideograph
    (0x22694D, ('\u{7A5F}', false)), // East Asian ideograph
    (0x22694F, ('\u{7A60}', false)), // East Asian ideograph
    (0x216951, ('\u{5159}', false)), // East Asian ideograph
    (0x216952, ('\u{515B}', false)), // East Asian ideograph
    (0x213663, ('\u{55AA}', false)), // East Asian ideograph
    (0x29454E, ('\u{9545}', false)), // East Asian ideograph
    (0x216955, ('\u{515D}', false)), // East Asian ideograph
    (0x216956, ('\u{515E}', false)), // East Asian ideograph
    (0x225838, ('\u{737E}', false)), // East Asian ideograph
    (0x216958, ('\u{515F}', false)), // East Asian ideograph
    (0x216959, ('\u{5161}', false)), // East Asian ideograph
    (0x69695B, ('\u{86AB}', false)), // East Asian ideograph
    (0x21695C, ('\u{5163}', false)), // East Asian ideograph
    (0x6F4F35, ('\u{B838}', false)), // Korean hangul
    (0x274E5D, ('\u{783A}', false)), // East Asian ideograph
    (0x22695F, ('\u{7A70}', false)), // East Asian ideograph
    (0x6F5D48, ('\u{D683}', false)), // Korean hangul
    (0x696962, ('\u{86EF}', false)), // East Asian ideograph
    (0x213C3B, ('\u{5DEB}', false)), // East Asian ideograph
    (0x226966, ('\u{7A75}', false)), // East Asian ideograph
    (0x216967, ('\u{5182}', false)), // East Asian ideograph
    (0x216969, ('\u{5184}', false)), // East Asian ideograph
    (0x22696B, ('\u{7A80}', false)), // East Asian ideograph
    (0x21696E, ('\u{518F}', false)), // East Asian ideograph
    (0x213C3D, ('\u{5DF1}', false)), // East Asian ideograph
    (0x216970, ('\u{5194}', false)), // East Asian ideograph
    (0x216971, ('\u{5193}', false)), // East Asian ideograph
    (0x2E403D, ('\u{6AC1}', false)), // East Asian ideograph
    (0x216975, ('\u{5196}', false)), // East Asian ideograph
    (0x226978, ('\u{7A8A}', false)), // East Asian ideograph
    (0x22697A, ('\u{7A94}', false)), // East Asian ideograph
    (0x21697B, ('\u{51A1}', false)), // East Asian ideograph
    (0x21697C, ('\u{51A3}', false)), // East Asian ideograph
    (0x22697E, ('\u{68A5}', false)), // East Asian ideograph
    (0x213C40, ('\u{5DF4}', false)), // East Asian ideograph
    (0x223C41, ('\u{6832}', false)), // East Asian ideograph
    (0x213C42, ('\u{5DFD}', false)), // East Asian ideograph
    (0x275B28, ('\u{8E0C}', false)), // East Asian ideograph
    (0x213C43, ('\u{5DFE}', false)), // East Asian ideograph
    (0x275E3F, ('\u{94CE}', false)), // East Asian ideograph
    (0x213C44, ('\u{5E02}', false)), // East Asian ideograph
    (0x6F5471, ('\u{C510}', false)), // Korean hangul
    (0x29565D, ('\u{9C82}', false)), // East Asian ideograph
    (0x217C45, ('\u{5AB7}', false)), // East Asian ideograph
    (0x2D4440, ('\u{6822}', false)), // East Asian ideograph
    (0x223C47, ('\u{682B}', false)), // East Asian ideograph
    (0x294551, ('\u{9517}', false)), // East Asian ideograph
    (0x223C48, ('\u{682D}', false)), // East Asian ideograph
    (0x6F493E, ('\u{ACAF}', false)), // Korean hangul
    (0x235C3A, ('\u{9DDF}', false)), // East Asian ideograph
    (0x233C49, ('\u{8F57}', false)), // East Asian ideograph
    (0x6F5D4B, ('\u{D68D}', false)), // Korean hangul
    (0x213C4A, ('\u{5E16}', false)), // East Asian ideograph
    (0x334F71, ('\u{54B2}', false)), // East Asian ideograph
    (0x213C4B, ('\u{5E15}', false)), // East Asian ideograph
    (0x275F6D, ('\u{972D}', false)), // East Asian ideograph
    (0x213C4C, ('\u{5E1B}', false)), // East Asian ideograph
    (0x4B3321, ('\u{5185}', false)), // East Asian ideograph
    (0x233C4D, ('\u{8F5C}', false)), // East Asian ideograph
    (0x213C4E, ('\u{5E1D}', false)), // East Asian ideograph
    (0x284F7D, ('\u{704F}', false)), // East Asian ideograph
    (0x29426F, ('\u{94BD}', false)), // East Asian ideograph
    (0x223C4F, ('\u{6844}', false)), // East Asian ideograph
    (0x4B5E5D, ('\u{95D4}', false)), // East Asian ideograph
    (0x224730, ('\u{6C78}', false)), // East Asian ideograph
    (0x6F5379, ('\u{C2A8}', false)), // Korean hangul
    (0x217C50, ('\u{5ABA}', false)), // East Asian ideograph
    (0x275F6E, ('\u{96F3}', false)), // East Asian ideograph
    (0x213C51, ('\u{5E2B}', false)), // East Asian ideograph
    (0x6F5B26, ('\u{D131}', false)), // Korean hangul
    (0x213668, ('\u{55AE}', false)), // East Asian ideograph
    (0x232635, ('\u{8598}', false)), // East Asian ideograph
    (0x213C52, ('\u{5E33}', false)), // East Asian ideograph
    (0x233C53, ('\u{8F5D}', false)), // East Asian ideograph
    (0x6F5D4D, ('\u{D6A1}', false)), // Korean hangul
    (0x224731, ('\u{6C74}', false)), // East Asian ideograph
    (0x213C55, ('\u{5E37}', false)), // East Asian ideograph
    (0x275F6F, ('\u{7075}', false)), // East Asian ideograph
    (0x213C56, ('\u{5E45}', false)), // East Asian ideograph
    (0x6F4C41, ('\u{B1E8}', false)), // Korean hangul
    (0x275B2C, ('\u{8E8F}', false)), // East Asian ideograph
    (0x213226, ('\u{5000}', false)), // East Asian ideograph
    (0x223C58, ('\u{6834}', false)), // East Asian ideograph
    (0x334F37, ('\u{5EE9}', false)), // East Asian ideograph
    (0x695D36, ('\u{6B1F}', false)), // East Asian ideograph
    (0x223C59, ('\u{6812}', false)), // East Asian ideograph
    (0x224732, ('\u{6C86}', false)), // East Asian ideograph
    (0x213C5A, ('\u{5E5B}', false)), // East Asian ideograph
    (0x216A22, ('\u{51AA}', false)), // East Asian ideograph
    (0x216A23, ('\u{51AB}', false)), // East Asian ideograph
    (0x6F4C42, ('\u{B1FD}', false)), // Korean hangul
    (0x216A26, ('\u{51B1}', false)), // East Asian ideograph
    (0x29233C, ('\u{836D}', false)), // East Asian ideograph
    (0x226A28, ('\u{7AA3}', false)), // East Asian ideograph
    (0x213227, ('\u{4FEE}', false)), // East Asian ideograph
    (0x226A2B, ('\u{7A9E}', false)), // East Asian ideograph
    (0x226A2C, ('\u{7AA7}', false)), // East Asian ideograph
    (0x226A2E, ('\u{7AA8}', false)), // East Asian ideograph
    (0x46284C, ('\u{5ED0}', false)), // East Asian ideograph
    (0x226A31, ('\u{7AAC}', false)), // East Asian ideograph
    (0x6F5D4F, ('\u{D6C4}', false)), // Korean hangul
    (0x216A35, ('\u{51BC}', false)), // East Asian ideograph
    (0x226A36, ('\u{7AB3}', false)), // East Asian ideograph
    (0x226A3A, ('\u{7ABD}', false)), // East Asian ideograph
    (0x2D3C5F, ('\u{6A66}', false)), // East Asian ideograph
    (0x226A3C, ('\u{7AB6}', false)), // East Asian ideograph
    (0x226A3D, ('\u{7AB8}', false)), // East Asian ideograph
    (0x226A3E, ('\u{7AB5}', false)), // East Asian ideograph
    (0x226A3F, ('\u{7ABB}', false)), // East Asian ideograph
    (0x213C60, ('\u{5E5F}', false)), // East Asian ideograph
    (0x6F4C43, ('\u{B204}', false)), // Korean hangul
    (0x216A43, ('\u{51CA}', false)), // East Asian ideograph
    (0x216A46, ('\u{51C7}', false)), // East Asian ideograph
    (0x213C61, ('\u{5E6B}', false)), // East Asian ideograph
    (0x226A49, ('\u{7ACD}', false)), // East Asian ideograph
    (0x226A4B, ('\u{7ACF}', false)), // East Asian ideograph
    (0x216A4E, ('\u{51D1}', false)), // East Asian ideograph
    (0x216A4F, ('\u{51D0}', false)), // East Asian ideograph
    (0x287271, ('\u{7EA4}', false)), // East Asian ideograph (duplicate simplified)
    (0x226A51, ('\u{7AD3}', false)), // East Asian ideograph
    (0x226A52, ('\u{7AD4}', false)), // East Asian ideograph
    (0x216A54, ('\u{51D3}', false)), // East Asian ideograph
    (0x226A55, ('\u{7ADA}', false)), // East Asian ideograph
    (0x226A5A, ('\u{7AE1}', false)), // East Asian ideograph
    (0x226A5E, ('\u{7AE6}', false)), // East Asian ideograph
    (0x233C65, ('\u{8FA4}', false)), // East Asian ideograph
    (0x277D48, ('\u{5AD2}', false)), // East Asian ideograph
    (0x696A61, ('\u{88C3}', false)), // East Asian ideograph
    (0x21765B, ('\u{57DD}', false)), // East Asian ideograph
    (0x216A63, ('\u{51D9}', false)), // East Asian ideograph
    (0x226A66, ('\u{7AEB}', false)), // East Asian ideograph
    (0x216A68, ('\u{51E2}', false)), // East Asian ideograph
    (0x226A6B, ('\u{7AF0}', false)), // East Asian ideograph
    (0x696A6D, ('\u{8904}', false)), // East Asian ideograph
    (0x6F5D51, ('\u{D6C8}', false)), // Korean hangul
    (0x213C68, ('\u{5E7B}', false)), // East Asian ideograph
    (0x216A73, ('\u{5160}', false)), // East Asian ideograph
    (0x226A76, ('\u{7AF5}', false)), // East Asian ideograph
    (0x213C69, ('\u{5E7C}', false)), // East Asian ideograph
    (0x216A78, ('\u{51F5}', false)), // East Asian ideograph
    (0x216A79, ('\u{51F7}', false)), // East Asian ideograph
    (0x226A7C, ('\u{7AFE}', false)), // East Asian ideograph
    (0x2D3C6A, ('\u{51FC}', false)), // East Asian ideograph
    (0x273C6B, ('\u{51E0}', false)), // East Asian ideograph
    (0x4B4D56, ('\u{8846}', false)), // East Asian ideograph
    (0x2D5554, ('\u{855A}', false)), // East Asian ideograph
    (0x213C6C, ('\u{5E8F}', false)), // East Asian ideograph
    (0x6F5D52, ('\u{D6CC}', false)), // Korean hangul
    (0x233C6D, ('\u{8FB7}', false)), // East Asian ideograph
    (0x295222, ('\u{98E8}', false)), // East Asian ideograph
    (0x234B35, ('\u{96A9}', false)), // East Asian ideograph
    (0x227333, ('\u{7E50}', false)), // East Asian ideograph
    (0x6F4C46, ('\u{B20B}', false)), // Korean hangul
    (0x275B31, ('\u{8EAF}', false)), // East Asian ideograph
    (0x213C70, ('\u{5E97}', false)), // East Asian ideograph
    (0x22326A, ('\u{63F9}', false)), // East Asian ideograph
    (0x223C71, ('\u{689B}', false)), // East Asian ideograph
    (0x6F5D53, ('\u{D6D1}', false)), // Korean hangul
    (0x213C72, ('\u{5E9C}', false)), // East Asian ideograph
    (0x29344D, ('\u{8C2E}', false)), // East Asian ideograph
    (0x6F5972, ('\u{CE7C}', false)), // Korean hangul
    (0x223C74, ('\u{68B6}', false)), // East Asian ideograph
    (0x6F4C47, ('\u{B20C}', false)), // Korean hangul
    (0x275B32, ('\u{8F66}', false)), // East Asian ideograph
    (0x213C75, ('\u{5EA6}', false)), // East Asian ideograph
    (0x223C76, ('\u{6882}', false)), // East Asian ideograph
    (0x226721, ('\u{7951}', false)), // East Asian ideograph
    (0x6F5D54, ('\u{D6D4}', false)), // Korean hangul
    (0x273C77, ('\u{5750}', false)), // East Asian ideograph
    (0x23595C, ('\u{9C6F}', false)), // East Asian ideograph
    (0x234B37, ('\u{96AE}', false)), // East Asian ideograph
    (0x226723, ('\u{7954}', false)), // East Asian ideograph
    (0x28232B, ('\u{5C66}', false)), // East Asian ideograph
    (0x232724, ('\u{8624}', false)), // East Asian ideograph
    (0x223C7A, ('\u{6890}', false)), // East Asian ideograph
    (0x4B4D59, ('\u{775B}', false)), // East Asian ideograph (variant of 214D59 which maps to 775B)
    (0x2F317D, ('\u{8A7E}', false)), // East Asian ideograph
    (0x213C7B, ('\u{5EB6}', false)), // East Asian ideograph
    (0x6F5D55, ('\u{D6D7}', false)), // Korean hangul
    (0x275D67, ('\u{94DD}', false)), // East Asian ideograph
    (0x217C7C, ('\u{5AEB}', false)), // East Asian ideograph
    (0x2D462C, ('\u{6B7A}', false)), // East Asian ideograph
    (0x224739, ('\u{6C67}', false)), // East Asian ideograph
    (0x233C7D, ('\u{8FCD}', false)), // East Asian ideograph
    (0x4B5A23, ('\u{621D}', false)), // East Asian ideograph
    (0x213C7E, ('\u{5EC1}', false)), // East Asian ideograph
    (0x2D4756, ('\u{6F94}', false)), // East Asian ideograph
    (0x275B34, ('\u{519B}', false)), // East Asian ideograph
    (0x4B5C47, ('\u{9059}', false)), // East Asian ideograph
    (0x22672A, ('\u{7967}', false)), // East Asian ideograph
    (0x235C45, ('\u{9DD6}', false)), // East Asian ideograph
    (0x6F4866, ('\u{AC10}', false)), // Korean hangul
    (0x6F5B27, ('\u{D134}', false)), // Korean hangul
    (0x4C6266, ('\u{778B}', false)), // East Asian ideograph
    (0x22672D, ('\u{796B}', false)), // East Asian ideograph
    (0x2D6222, ('\u{9C0C}', false)), // East Asian ideograph
    (0x6F4C4A, ('\u{B215}', false)), // Korean hangul
    (0x275B35, ('\u{8F68}', false)), // East Asian ideograph
    (0x6F4F38, ('\u{B85C}', false)), // Korean hangul
    (0x33476F, ('\u{6D44}', false)), // East Asian ideograph
    (0x6F5D57, ('\u{D6E4}', false)), // Korean hangul
    (0x216B24, ('\u{5213}', false)), // East Asian ideograph
    (0x216B26, ('\u{5216}', false)), // East Asian ideograph
    (0x226B27, ('\u{7B39}', false)), // East Asian ideograph
    (0x22473B, ('\u{6C84}', false)), // East Asian ideograph
    (0x216B2A, ('\u{521C}', false)), // East Asian ideograph
    (0x226B2D, ('\u{7B0F}', false)), // East Asian ideograph
    (0x226B2E, ('\u{7B08}', false)), // East Asian ideograph
    (0x275F79, ('\u{9765}', false)), // East Asian ideograph
    (0x6F4C4B, ('\u{B217}', false)), // Korean hangul
    (0x226B33, ('\u{7B0A}', false)), // East Asian ideograph
    (0x29455E, ('\u{94E1}', false)), // East Asian ideograph
    (0x226B35, ('\u{7B35}', false)), // East Asian ideograph
    (0x226B36, ('\u{7B25}', false)), // East Asian ideograph
    (0x216B37, ('\u{5232}', false)), // East Asian ideograph
    (0x226B39, ('\u{7B38}', false)), // East Asian ideograph
    (0x226B3B, ('\u{7B3B}', false)), // East Asian ideograph
    (0x216B3E, ('\u{5244}', false)), // East Asian ideograph
    (0x226B3F, ('\u{7B24}', false)), // East Asian ideograph
    (0x226B40, ('\u{7B33}', false)), // East Asian ideograph
    (0x226B42, ('\u{7B2A}', false)), // East Asian ideograph
    (0x216B43, ('\u{5249}', false)), // East Asian ideograph
    (0x226B44, ('\u{7B18}', false)), // East Asian ideograph
    (0x282736, ('\u{5E0F}', false)), // East Asian ideograph
    (0x226B47, ('\u{7B31}', false)), // East Asian ideograph
    (0x234B3B, ('\u{96B0}', false)), // East Asian ideograph
    (0x226B4A, ('\u{7B2B}', false)), // East Asian ideograph
    (0x216B4B, ('\u{525A}', false)), // East Asian ideograph
    (0x216B4C, ('\u{5252}', false)), // East Asian ideograph
    (0x226B4D, ('\u{7B1F}', false)), // East Asian ideograph
    (0x213B36, ('\u{5BE9}', false)), // East Asian ideograph
    (0x216B50, ('\u{525F}', false)), // East Asian ideograph
    (0x226B52, ('\u{7B4A}', false)), // East Asian ideograph
    (0x226B53, ('\u{7B59}', false)), // East Asian ideograph (not in Unicode)
    (0x226B54, ('\u{7B04}', false)), // East Asian ideograph (variant of 2E6B54 which maps to 7B04)
    (0x226B55, ('\u{7B47}', false)), // East Asian ideograph
    (0x216739, ('\u{5048}', false)), // East Asian ideograph
    (0x226B59, ('\u{7B58}', false)), // East Asian ideograph
    (0x226B5B, ('\u{7B6C}', false)), // East Asian ideograph
    (0x696B5C, ('\u{8ADA}', false)), // East Asian ideograph
    (0x216B5E, ('\u{5268}', false)), // East Asian ideograph
    (0x216B5F, ('\u{7B9A}', false)), // East Asian ideograph
    (0x226B60, ('\u{7B48}', false)), // East Asian ideograph
    (0x226B61, ('\u{7B45}', false)), // East Asian ideograph
    (0x226B62, ('\u{7B4C}', false)), // East Asian ideograph
    (0x226B63, ('\u{7B4E}', false)), // East Asian ideograph
    (0x234B3C, ('\u{96B2}', false)), // East Asian ideograph
    (0x226B68, ('\u{7B66}', false)), // East Asian ideograph
    (0x706B6A, ('\u{8159}', false)), // East Asian ideograph
    (0x216B6B, ('\u{5278}', false)), // East Asian ideograph
    (0x226B6C, ('\u{7B64}', false)), // East Asian ideograph
    (0x226B6E, ('\u{7B69}', false)), // East Asian ideograph
    (0x275B38, ('\u{8F6F}', false)), // East Asian ideograph
    (0x226B70, ('\u{7B6D}', false)), // East Asian ideograph
    (0x226B74, ('\u{7B62}', false)), // East Asian ideograph
    (0x226B75, ('\u{7B6E}', false)), // East Asian ideograph
    (0x226B76, ('\u{7B74}', false)), // East Asian ideograph
    (0x233A30, ('\u{8E50}', false)), // East Asian ideograph
    (0x216B79, ('\u{528C}', false)), // East Asian ideograph
    (0x216B7A, ('\u{528A}', false)), // East Asian ideograph
    (0x226B7B, ('\u{7B6F}', false)), // East Asian ideograph
    (0x216B7C, ('\u{5290}', false)), // East Asian ideograph
    (0x226B7E, ('\u{7B65}', false)), // East Asian ideograph
    (0x4B3A2F, ('\u{805F}', false)), // East Asian ideograph
    (0x275B39, ('\u{8F6D}', false)), // East Asian ideograph
    (0x6F4867, ('\u{AC11}', false)), // Korean hangul
    (0x27457A, ('\u{6B20}', false)), // East Asian ideograph (duplicate simplified)
    (0x222969, ('\u{5F54}', false)), // East Asian ideograph
    (0x4B3C53, ('\u{5E2F}', false)), // East Asian ideograph
    (0x234B3E, ('\u{96B3}', false)), // East Asian ideograph
    (0x4C6564, ('\u{78D9}', false)), // East Asian ideograph
    (0x282747, ('\u{5E3B}', false)), // East Asian ideograph
    (0x22584C, ('\u{7393}', false)), // East Asian ideograph
    (0x6F4F39, ('\u{B85D}', false)), // Korean hangul
    (0x2F5D5C, ('\u{730A}', false)), // East Asian ideograph
    (0x294944, ('\u{9603}', false)), // East Asian ideograph
    (0x22674A, ('\u{7998}', false)), // East Asian ideograph
    (0x33306C, ('\u{8B90}', false)), // East Asian ideograph
    (0x21674B, ('\u{505F}', false)), // East Asian ideograph
    (0x273D65, ('\u{540E}', false)), // East Asian ideograph
    (0x6F4C50, ('\u{B25C}', false)), // Korean hangul
    (0x27583B, ('\u{8BA6}', false)), // East Asian ideograph
    (0x213235, ('\u{5074}', false)), // East Asian ideograph
    (0x22674D, ('\u{7999}', false)), // East Asian ideograph
    (0x22674E, ('\u{7995}', false)), // East Asian ideograph
    (0x6F5D5D, ('\u{D711}', false)), // Korean hangul
    (0x226750, ('\u{7996}', false)), // East Asian ideograph
    (0x333051, ('\u{8CB3}', false)), // East Asian ideograph
    (0x2D6229, ('\u{9C53}', false)), // East Asian ideograph
    (0x6F4C51, ('\u{B260}', false)), // Korean hangul
    (0x275B3C, ('\u{8F76}', false)), // East Asian ideograph
    (0x294564, ('\u{9536}', false)), // East Asian ideograph
    (0x292752, ('\u{830F}', false)), // East Asian ideograph
    (0x233A34, ('\u{8E5C}', false)), // East Asian ideograph
    (0x6F5A29, ('\u{CEF5}', false)), // Korean hangul
    (0x6F5D5E, ('\u{D718}', false)), // Korean hangul
    (0x274A30, ('\u{70DB}', false)), // East Asian ideograph
    (0x292577, ('\u{8297}', false)), // East Asian ideograph
    (0x4B4A62, ('\u{72A0}', false)), // East Asian ideograph
    (0x273D67, ('\u{5F84}', false)), // East Asian ideograph
    (0x6F4C52, ('\u{B268}', false)), // Korean hangul
    (0x275B3D, ('\u{8F83}', false)), // East Asian ideograph
    (0x217669, ('\u{5819}', false)), // East Asian ideograph
    (0x223636, ('\u{6549}', false)), // East Asian ideograph
    (0x2D5561, ('\u{76D6}', false)), // East Asian ideograph
    (0x6F5D5F, ('\u{D719}', false)), // Korean hangul
    (0x274A31, ('\u{707F}', false)), // East Asian ideograph
    (0x293459, ('\u{8C2F}', false)), // East Asian ideograph
    (0x284E30, ('\u{6E11}', false)), // East Asian ideograph
    (0x29584B, ('\u{9CBD}', false)), // East Asian ideograph
    (0x6F584A, ('\u{CA0B}', false)), // Korean hangul
    (0x34715A, ('\u{7E1A}', false)), // East Asian ideograph
    (0x216C21, ('\u{5293}', false)), // East Asian ideograph
    (0x6F4C53, ('\u{B269}', false)), // Korean hangul
    (0x275B3E, ('\u{8F7C}', false)), // East Asian ideograph
    (0x226C26, ('\u{7B71}', false)), // East Asian ideograph
    (0x226C27, ('\u{7B70}', false)), // East Asian ideograph
    (0x216C29, ('\u{5298}', false)), // East Asian ideograph
    (0x235C4F, ('\u{9DE9}', false)), // East Asian ideograph
    (0x216C2B, ('\u{529A}', false)), // East Asian ideograph
    (0x216C2C, ('\u{5299}', false)), // East Asian ideograph
    (0x226C2D, ('\u{7B9C}', false)), // East Asian ideograph
    (0x216C2E, ('\u{52A6}', false)), // East Asian ideograph
    (0x22275D, ('\u{5E68}', false)), // East Asian ideograph
    (0x212A2B, ('\u{E8D9}', false)), // EACC component character
    (0x216C31, ('\u{52AD}', false)), // East Asian ideograph
    (0x226C33, ('\u{7B92}', false)), // East Asian ideograph
    (0x226C34, ('\u{7B91}', false)), // East Asian ideograph
    (0x226C35, ('\u{7B90}', false)), // East Asian ideograph
    (0x216C37, ('\u{52BB}', false)), // East Asian ideograph
    (0x226C38, ('\u{7BA3}', false)), // East Asian ideograph
    (0x226C3A, ('\u{7B8D}', false)), // East Asian ideograph
    (0x28275F, ('\u{5E31}', false)), // East Asian ideograph
    (0x216C3C, ('\u{52CA}', false)), // East Asian ideograph
    (0x216C3D, ('\u{52CD}', false)), // East Asian ideograph
    (0x2E6C3E, ('\u{7B59}', false)), // East Asian ideograph
    (0x2D622C, ('\u{9F08}', false)), // East Asian ideograph
    (0x216C40, ('\u{52D0}', false)), // East Asian ideograph
    (0x226C41, ('\u{7B85}', false)), // East Asian ideograph
    (0x706C42, ('\u{70BB}', false)), // East Asian ideograph
    (0x226C43, ('\u{7B8E}', false)), // East Asian ideograph
    (0x226C44, ('\u{7B98}', false)), // East Asian ideograph
    (0x213239, ('\u{504C}', false)), // East Asian ideograph
    (0x226C46, ('\u{7B86}', false)), // East Asian ideograph
    (0x226C48, ('\u{7B99}', false)), // East Asian ideograph
    (0x6F4F3A, ('\u{B860}', false)), // Korean hangul
    (0x216C4C, ('\u{52E3}', false)), // East Asian ideograph
    (0x216C4E, ('\u{52E1}', false)), // East Asian ideograph
    (0x6F5D61, ('\u{D720}', false)), // Korean hangul
    (0x216C50, ('\u{55E7}', false)), // East Asian ideograph
    (0x226C52, ('\u{7BB2}', false)), // East Asian ideograph
    (0x216C53, ('\u{52E9}', false)), // East Asian ideograph
    (0x6F5623, ('\u{C648}', false)), // Korean hangul
    (0x226C58, ('\u{7BCB}', false)), // East Asian ideograph
    (0x226C59, ('\u{7BB8}', false)), // East Asian ideograph
    (0x226C5A, ('\u{7BCF}', false)), // East Asian ideograph
    (0x226C5C, ('\u{7BD0}', false)), // East Asian ideograph
    (0x216C5E, ('\u{52F7}', false)), // East Asian ideograph
    (0x292765, ('\u{82C8}', false)), // East Asian ideograph
    (0x226C60, ('\u{7BBE}', false)), // East Asian ideograph
    (0x216C61, ('\u{52F9}', false)), // East Asian ideograph
    (0x216C62, ('\u{52FA}', false)), // East Asian ideograph
    (0x216C64, ('\u{52FC}', false)), // East Asian ideograph
    (0x216C69, ('\u{5307}', false)), // East Asian ideograph
    (0x216C6A, ('\u{5303}', false)), // East Asian ideograph
    (0x216C6B, ('\u{5306}', false)), // East Asian ideograph (not in Unicode)
    (0x6F5D62, ('\u{D728}', false)), // Korean hangul
    (0x216C6E, ('\u{530A}', false)), // East Asian ideograph
    (0x226C6F, ('\u{7BCC}', false)), // East Asian ideograph
    (0x216560, ('\u{4F80}', false)), // East Asian ideograph
    (0x216C77, ('\u{5311}', false)), // East Asian ideograph
    (0x213F6A, ('\u{6221}', false)), // East Asian ideograph
    (0x6F5975, ('\u{CE87}', false)), // Korean hangul
    (0x216C7B, ('\u{6706}', false)), // East Asian ideograph
    (0x234767, ('\u{93F5}', false)), // East Asian ideograph
    (0x21323B, ('\u{500F}', false)), // East Asian ideograph
    (0x343E38, ('\u{7BDA}', false)), // East Asian ideograph
    (0x4B6167, ('\u{95D8}', false)), // East Asian ideograph
    (0x6F5D63, ('\u{D729}', false)), // Korean hangul
    (0x6F5532, ('\u{C571}', false)), // Korean hangul
    (0x216561, ('\u{4F74}', false)), // East Asian ideograph
    (0x4B5A31, ('\u{8CCE}', false)), // East Asian ideograph
    (0x6F4C57, ('\u{B290}', false)), // Korean hangul
    (0x275B42, ('\u{8F84}', false)), // East Asian ideograph
    (0x333E7D, ('\u{7652}', false)), // East Asian ideograph
    (0x4B4925, ('\u{6FB3}', false)), // East Asian ideograph (variant of 214925 which maps to 6FB3)
    (0x226771, ('\u{79A8}', false)), // East Asian ideograph
    (0x225A7E, ('\u{7488}', false)), // East Asian ideograph
    (0x6F5921, ('\u{CC29}', false)), // Korean hangul
    (0x692577, ('\u{309B}', false)), // Katakana-hiragana voiced sound mark
    (0x224B26, ('\u{6E31}', false)), // East Asian ideograph
    (0x6F5A3E, ('\u{CF55}', false)), // Korean hangul
    (0x6F4C58, ('\u{B291}', false)), // Korean hangul
    (0x275B43, ('\u{8F7B}', false)), // East Asian ideograph
    (0x226775, ('\u{79B0}', false)), // East Asian ideograph
    (0x233A3B, ('\u{8E67}', false)), // East Asian ideograph
    (0x275221, ('\u{7EED}', false)), // East Asian ideograph
    (0x6F5922, ('\u{CC2C}', false)), // Korean hangul
    (0x215222, ('\u{7E93}', false)), // East Asian ideograph
    (0x234B48, ('\u{96B9}', false)), // East Asian ideograph
    (0x4D445B, ('\u{9306}', false)), // East Asian ideograph (variant of 23445B which maps to 9306)
    (0x275223, ('\u{7EA4}', false)), // East Asian ideograph
    (0x275224, ('\u{7F06}', false)), // East Asian ideograph
    (0x21323E, ('\u{50A2}', false)), // East Asian ideograph
    (0x6F4F3B, ('\u{B864}', false)), // Korean hangul
    (0x2D334F, ('\u{5202}', false)), // East Asian ideograph
    (0x21677B, ('\u{50CA}', false)), // East Asian ideograph
    (0x4B4B2C, ('\u{731F}', false)), // East Asian ideograph
    (0x213933, ('\u{5944}', false)), // East Asian ideograph
    (0x27677C, ('\u{4F1B}', false)), // East Asian ideograph
    (0x4C6022, ('\u{7596}', false)), // East Asian ideograph
    (0x225227, ('\u{7139}', false)), // East Asian ideograph
    (0x4B3C5E, ('\u{5E64}', false)), // East Asian ideograph
    (0x234B49, ('\u{96BC}', false)), // East Asian ideograph
    (0x215228, ('\u{7F3D}', false)), // East Asian ideograph
    (0x6F4C5A, ('\u{B298}', false)), // Korean hangul
    (0x275B45, ('\u{8F87}', false)), // East Asian ideograph
    (0x215229, ('\u{7F44}', false)), // East Asian ideograph
    (0x22363E, ('\u{6554}', false)), // East Asian ideograph
    (0x23522B, ('\u{995F}', false)), // East Asian ideograph
    (0x6F5924, ('\u{CC2F}', false)), // Korean hangul
    (0x22522C, ('\u{713B}', false)), // East Asian ideograph
    (0x516122, ('\u{9988}', false)), // East Asian ideograph
    (0x6F522D, ('\u{BE90}', false)), // Korean hangul
    (0x6F4C5B, ('\u{B299}', false)), // Korean hangul
    (0x22522E, ('\u{711C}', false)), // East Asian ideograph
    (0x213240, ('\u{5099}', false)), // East Asian ideograph
    (0x23522F, ('\u{9997}', false)), // East Asian ideograph
    (0x225B59, ('\u{74A0}', false)), // East Asian ideograph
    (0x4B6168, ('\u{9599}', false)), // East Asian ideograph
    (0x235230, ('\u{9998}', false)), // East Asian ideograph
    (0x226D22, ('\u{7BDD}', false)), // East Asian ideograph
    (0x216D23, ('\u{531A}', false)), // East Asian ideograph
    (0x226D24, ('\u{7BE5}', false)), // East Asian ideograph
    (0x216D25, ('\u{531F}', false)), // East Asian ideograph
    (0x215231, ('\u{7F69}', false)), // East Asian ideograph
    (0x226D29, ('\u{7BE8}', false)), // East Asian ideograph
    (0x277267, ('\u{5452}', false)), // East Asian ideograph
    (0x225232, ('\u{713D}', false)), // East Asian ideograph
    (0x226D2E, ('\u{7BF9}', false)), // East Asian ideograph
    (0x226D2F, ('\u{7BD4}', false)), // East Asian ideograph
    (0x6F4C5C, ('\u{B2A0}', false)), // Korean hangul
    (0x226D32, ('\u{7BDF}', false)), // East Asian ideograph
    (0x275233, ('\u{7F5A}', false)), // East Asian ideograph
    (0x226D35, ('\u{7BD8}', false)), // East Asian ideograph
    (0x216D36, ('\u{5335}', false)), // East Asian ideograph
    (0x226D37, ('\u{7BEA}', false)), // Unrelated variant of EACC 3A6A7C which maps to 7BEA
    (0x213C2D, ('\u{5DBC}', false)), // East Asian ideograph
    (0x275234, ('\u{9A82}', false)), // East Asian ideograph
    (0x216D3A, ('\u{5338}', false)), // East Asian ideograph
    (0x226D3B, ('\u{7C06}', false)), // East Asian ideograph
    (0x226D3E, ('\u{7BF0}', false)), // East Asian ideograph
    (0x275D6B, ('\u{952D}', false)), // East Asian ideograph
    (0x696D40, ('\u{8EC5}', false)), // East Asian ideograph
    (0x226D41, ('\u{7C0F}', false)), // East Asian ideograph
    (0x216D42, ('\u{534D}', false)), // East Asian ideograph
    (0x6F5926, ('\u{CC38}', false)), // Korean hangul
    (0x706D45, ('\u{783C}', false)), // East Asian ideograph
    (0x226D46, ('\u{7C0B}', false)), // East Asian ideograph
    (0x222534, ('\u{5D74}', false)), // East Asian ideograph
    (0x275237, ('\u{7F57}', false)), // East Asian ideograph
    (0x216D4C, ('\u{5363}', false)), // East Asian ideograph
    (0x2D6235, ('\u{9D76}', false)), // East Asian ideograph
    (0x216D4E, ('\u{5365}', false)), // East Asian ideograph (not in Unicode)
    (0x226D4F, ('\u{7BF4}', false)), // East Asian ideograph
    (0x215238, ('\u{7F88}', false)), // East Asian ideograph
    (0x216D53, ('\u{536C}', false)), // East Asian ideograph
    (0x226D54, ('\u{7BF3}', false)), // East Asian ideograph
    (0x216D57, ('\u{5372}', false)), // East Asian ideograph
    (0x216D58, ('\u{537A}', false)), // East Asian ideograph
    (0x4B492B, ('\u{6FEB}', false)), // East Asian ideograph
    (0x226D5A, ('\u{7C09}', false)), // East Asian ideograph
    (0x226D5B, ('\u{7C03}', false)), // East Asian ideograph
    (0x226D5C, ('\u{7BFC}', false)), // East Asian ideograph
    (0x216D5D, ('\u{5380}', false)), // East Asian ideograph
    (0x226D5F, ('\u{7C1C}', false)), // East Asian ideograph
    (0x226D61, ('\u{7C26}', false)), // East Asian ideograph
    (0x226D62, ('\u{7C28}', false)), // East Asian ideograph
    (0x22523B, ('\u{7129}', false)), // East Asian ideograph
    (0x216D64, ('\u{538E}', false)), // East Asian ideograph
    (0x233D3F, ('\u{9004}', false)), // East Asian ideograph
    (0x226D66, ('\u{7C1F}', false)), // East Asian ideograph
    (0x216D67, ('\u{5394}', false)), // East Asian ideograph
    (0x226D68, ('\u{7C2F}', false)), // East Asian ideograph
    (0x23523C, ('\u{99A1}', false)), // East Asian ideograph
    (0x6F4C5E, ('\u{B2A5}', false)), // Korean hangul
    (0x216D6D, ('\u{5399}', false)), // East Asian ideograph
    (0x6F523D, ('\u{BF18}', false)), // Korean hangul
    (0x285F48, ('\u{7617}', false)), // East Asian ideograph
    (0x213243, ('\u{5096}', false)), // East Asian ideograph
    (0x216D74, ('\u{8652}', false)), // East Asian ideograph
    (0x226D75, ('\u{7C30}', false)), // East Asian ideograph
    (0x6F4F3C, ('\u{B86C}', false)), // Korean hangul
    (0x216D7A, ('\u{53A4}', false)), // East Asian ideograph
    (0x216D7B, ('\u{53AB}', false)), // East Asian ideograph
    (0x2D5941, ('\u{5629}', false)), // East Asian ideograph
    (0x6F5928, ('\u{CC3B}', false)), // Korean hangul
    (0x2D5240, ('\u{7FA1}', false)), // East Asian ideograph
    (0x235241, ('\u{99A9}', false)), // East Asian ideograph
    (0x6F4C5F, ('\u{B2A6}', false)), // Korean hangul
    (0x215242, ('\u{7FA9}', false)), // East Asian ideograph
    (0x283D30, ('\u{67A7}', false)), // East Asian ideograph
    (0x235C5B, ('\u{9DF8}', false)), // East Asian ideograph
    (0x225243, ('\u{712E}', false)), // East Asian ideograph
    (0x6F5244, ('\u{BF51}', false)), // Korean hangul
    (0x6F5929, ('\u{CC3C}', false)), // Korean hangul
    (0x226969, ('\u{7A78}', false)), // East Asian ideograph
    (0x6F523B, ('\u{BF08}', false)), // Korean hangul
    (0x28337B, ('\u{62A0}', false)), // East Asian ideograph
    (0x6F4C60, ('\u{B2AA}', false)), // Korean hangul
    (0x4B5247, ('\u{7FAE}', false)), // East Asian ideograph
    (0x334256, ('\u{6B5B}', false)), // East Asian ideograph
    (0x22585D, ('\u{73A5}', false)), // East Asian ideograph
    (0x235C5C, ('\u{9DFC}', false)), // East Asian ideograph
    (0x225248, ('\u{7177}', false)), // East Asian ideograph
    (0x233A43, ('\u{8E5D}', false)), // East Asian ideograph
    (0x4B492E, ('\u{6E0B}', false)), // East Asian ideograph
    (0x234E4C, ('\u{97CD}', false)), // East Asian ideograph
    (0x2D7345, ('\u{56D3}', false)), // East Asian ideograph
    (0x215249, ('\u{7FBF}', false)), // East Asian ideograph
    (0x284E3E, ('\u{6CF6}', false)), // East Asian ideograph
    (0x212A3A, ('\u{E8E7}', false)), // EACC component character
    (0x2D524A, ('\u{7FC4}', false)), // East Asian ideograph
    (0x39483B, ('\u{9061}', false)), // East Asian ideograph
    (0x276029, ('\u{9791}', false)), // East Asian ideograph
    (0x6F524B, ('\u{BFD0}', false)), // Korean hangul
    (0x2E337B, ('\u{630E}', false)), // East Asian ideograph
    (0x6F4C61, ('\u{B2AC}', false)), // Korean hangul
    (0x6F524C, ('\u{BFD4}', false)), // Korean hangul
    (0x27524D, ('\u{4E60}', false)), // East Asian ideograph
    (0x233A44, ('\u{8E75}', false)), // East Asian ideograph
    (0x23524E, ('\u{99BC}', false)), // East Asian ideograph
    (0x293468, ('\u{8C35}', false)), // East Asian ideograph
    (0x6F545A, ('\u{C410}', false)), // Korean hangul
    (0x23524F, ('\u{99C3}', false)), // East Asian ideograph
    (0x6F5250, ('\u{C058}', false)), // Korean hangul
    (0x6F4C62, ('\u{B2C8}', false)), // Korean hangul
    (0x275B4D, ('\u{8F91}', false)), // East Asian ideograph
    (0x275251, ('\u{7FC6}', false)), // East Asian ideograph
    (0x213247, ('\u{50B5}', false)), // East Asian ideograph
    (0x4B4D73, ('\u{66B8}', false)), // East Asian ideograph
    (0x225252, ('\u{7152}', false)), // East Asian ideograph
    (0x235253, ('\u{99B9}', false)), // East Asian ideograph
    (0x6F592C, ('\u{CC3F}', false)), // Korean hangul
    (0x215254, ('\u{7FE9}', false)), // East Asian ideograph
    (0x274D3A, ('\u{76CF}', false)), // East Asian ideograph
    (0x225255, ('\u{715D}', false)), // East Asian ideograph
    (0x6F4C63, ('\u{B2C9}', false)), // Korean hangul
    (0x225256, ('\u{7141}', false)), // East Asian ideograph
    (0x227636, ('\u{800F}', false)), // East Asian ideograph
    (0x4B4931, ('\u{6E16}', false)), // East Asian ideograph
    (0x4D3359, ('\u{56AF}', false)), // East Asian ideograph
    (0x275258, ('\u{7FD8}', false)), // East Asian ideograph
    (0x21656E, ('\u{4F94}', false)), // East Asian ideograph
    (0x284E41, ('\u{6F4B}', false)), // East Asian ideograph
    (0x225259, ('\u{7175}', false)), // East Asian ideograph
    (0x692546, ('\u{30C6}', false)), // Katakana letter TE
    (0x22525A, ('\u{7173}', false)), // East Asian ideograph
    (0x6F4C64, ('\u{B2CC}', false)), // Korean hangul
    (0x275B4F, ('\u{8F96}', false)), // East Asian ideograph
    (0x33525B, ('\u{71FF}', false)), // East Asian ideograph
    (0x226E27, ('\u{7C35}', false)), // East Asian ideograph
    (0x23347B, ('\u{8B7E}', false)), // East Asian ideograph
    (0x21525C, ('\u{8001}', false)), // East Asian ideograph
    (0x226E2A, ('\u{7C40}', false)), // East Asian ideograph
    (0x2D5573, ('\u{83D4}', false)), // East Asian ideograph
    (0x216E2C, ('\u{53B5}', false)), // East Asian ideograph
    (0x216E2E, ('\u{53B9}', false)), // East Asian ideograph
    (0x22525D, ('\u{715A}', false)), // East Asian ideograph
    (0x226E30, ('\u{7C39}', false)), // East Asian ideograph
    (0x6F592E, ('\u{CC45}', false)), // Korean hangul
    (0x226E34, ('\u{7C3B}', false)), // East Asian ideograph
    (0x226E35, ('\u{7C34}', false)), // East Asian ideograph
    (0x6F5426, ('\u{C2E3}', false)), // Korean hangul
    (0x226E3B, ('\u{7C42}', false)), // East Asian ideograph
    (0x216E3E, ('\u{53D0}', false)), // East Asian ideograph
    (0x70727D, ('\u{87A8}', false)), // East Asian ideograph
    (0x275B50, ('\u{8F97}', false)), // East Asian ideograph
    (0x225260, ('\u{714B}', false)), // East Asian ideograph
    (0x4C6E42, ('\u{7C31}', false)), // East Asian ideograph
    (0x21324A, ('\u{50BE}', false)), // East Asian ideograph
    (0x225862, ('\u{73A2}', false)), // East Asian ideograph
    (0x226E46, ('\u{7C4E}', false)), // East Asian ideograph
    (0x235261, ('\u{99D3}', false)), // East Asian ideograph
    (0x216E48, ('\u{53DA}', false)), // East Asian ideograph
    (0x4D5574, ('\u{9B2E}', false)), // East Asian ideograph
    (0x275E47, ('\u{94A5}', false)), // East Asian ideograph
    (0x225262, ('\u{7147}', false)), // East Asian ideograph
    (0x6F592F, ('\u{CC48}', false)), // Korean hangul
    (0x235263, ('\u{99D4}', false)), // East Asian ideograph
    (0x226E54, ('\u{7C5D}', false)), // East Asian ideograph
    (0x226E56, ('\u{7C5C}', false)), // East Asian ideograph
    (0x226E57, ('\u{7C5A}', false)), // East Asian ideograph
    (0x226E58, ('\u{7C5B}', false)), // East Asian ideograph
    (0x226E59, ('\u{7C59}', false)), // East Asian ideograph
    (0x226E5B, ('\u{7C5E}', false)), // East Asian ideograph
    (0x226E5C, ('\u{7C67}', false)), // East Asian ideograph
    (0x6F4C66, ('\u{B2D8}', false)), // Korean hangul
    (0x226E5E, ('\u{7C63}', false)), // East Asian ideograph
    (0x235265, ('\u{99C9}', false)), // East Asian ideograph
    (0x226E61, ('\u{7C68}', false)), // East Asian ideograph
    (0x226E62, ('\u{7C65}', false)), // East Asian ideograph
    (0x2D3132, ('\u{4ECF}', false)), // East Asian ideograph
    (0x225266, ('\u{7171}', false)), // East Asian ideograph
    (0x216E68, ('\u{5406}', false)), // East Asian ideograph
    (0x286E69, ('\u{7C16}', false)), // East Asian ideograph
    (0x225267, ('\u{715F}', false)), // East Asian ideograph
    (0x216E6C, ('\u{544C}', false)), // East Asian ideograph
    (0x216E6D, ('\u{5445}', false)), // East Asian ideograph
    (0x226E6F, ('\u{7C6F}', false)), // East Asian ideograph
    (0x216E70, ('\u{5432}', false)), // East Asian ideograph
    (0x226970, ('\u{7A85}', false)), // East Asian ideograph
    (0x226E75, ('\u{7C75}', false)), // East Asian ideograph
    (0x216E76, ('\u{5421}', false)), // East Asian ideograph
    (0x215269, ('\u{8033}', false)), // East Asian ideograph
    (0x216E78, ('\u{5430}', false)), // East Asian ideograph
    (0x226E79, ('\u{7C7E}', false)), // East Asian ideograph
    (0x226E7A, ('\u{7C78}', false)), // East Asian ideograph
    (0x6F4C67, ('\u{B2D9}', false)), // Korean hangul
    (0x275B52, ('\u{6BC2}', false)), // East Asian ideograph
    (0x226E7D, ('\u{7C7D}', false)), // East Asian ideograph
    (0x27517E, ('\u{7F20}', false)), // East Asian ideograph
    (0x692560, ('\u{30E0}', false)), // Katakana letter MU
    (0x215022, ('\u{7B4F}', false)), // East Asian ideograph
    (0x6F5323, ('\u{C11E}', false)), // Korean hangul
    (0x225421, ('\u{71DD}', false)), // East Asian ideograph
    (0x6F486C, ('\u{AC16}', false)), // Korean hangul
    (0x2D526C, ('\u{8EAD}', false)), // East Asian ideograph
    (0x274A46, ('\u{5899}', false)), // East Asian ideograph
    (0x6F5931, ('\u{CC54}', false)), // Korean hangul
    (0x225F5F, ('\u{7630}', false)), // East Asian ideograph
    (0x6F5B2D, ('\u{D145}', false)), // Korean hangul
    (0x22253F, ('\u{5D75}', false)), // East Asian ideograph
    (0x234B57, ('\u{96D2}', false)), // East Asian ideograph
    (0x69654F, ('\u{7E05}', false)), // East Asian ideograph
    (0x4B526E, ('\u{8046}', false)), // East Asian ideograph (variant of 21526E which maps to 8046)
    (0x6F4C68, ('\u{B2DB}', false)), // Korean hangul
    (0x27526F, ('\u{5723}', false)), // East Asian ideograph
    (0x22763B, ('\u{801F}', false)), // East Asian ideograph
    (0x225422, ('\u{71C0}', false)), // East Asian ideograph
    (0x335821, ('\u{97C8}', false)), // East Asian ideograph
    (0x275271, ('\u{95FB}', false)), // East Asian ideograph
    (0x6F5932, ('\u{CC55}', false)), // Korean hangul
    (0x6F5272, ('\u{C0D0}', false)), // Korean hangul
    (0x2D446B, ('\u{6936}', false)), // East Asian ideograph
    (0x2D6241, ('\u{9D5E}', false)), // East Asian ideograph
    (0x21346A, ('\u{536F}', false)), // East Asian ideograph
    (0x275B54, ('\u{8F99}', false)), // East Asian ideograph
    (0x235274, ('\u{99EC}', false)), // East Asian ideograph
    (0x2E625F, ('\u{77C1}', false)), // East Asian ideograph
    (0x275275, ('\u{8038}', false)), // East Asian ideograph
    (0x4B4937, ('\u{56A0}', false)), // East Asian ideograph
    (0x225276, ('\u{7172}', false)), // East Asian ideograph
    (0x223D21, ('\u{6872}', false)), // East Asian ideograph
    (0x6F5933, ('\u{CC58}', false)), // Korean hangul
    (0x29486F, ('\u{9569}', false)), // East Asian ideograph
    (0x275277, ('\u{8054}', false)), // East Asian ideograph
    (0x223D22, ('\u{689C}', false)), // East Asian ideograph
    (0x275278, ('\u{804C}', false)), // East Asian ideograph
    (0x6F5979, ('\u{CE94}', false)), // Korean hangul
    (0x2F3C2D, ('\u{8F3C}', false)), // East Asian ideograph
    (0x6F4C6A, ('\u{B2E2}', false)), // Korean hangul
    (0x275B55, ('\u{8F6C}', false)), // East Asian ideograph
    (0x215279, ('\u{8076}', false)), // East Asian ideograph
    (0x2E7D24, ('\u{83F0}', false)), // East Asian ideograph
    (0x225867, ('\u{73B6}', false)), // East Asian ideograph
    (0x235D66, ('\u{9E9E}', false)), // East Asian ideograph
    (0x21527A, ('\u{807E}', false)), // East Asian ideograph
    (0x213D25, ('\u{5ED3}', false)), // East Asian ideograph
    (0x275E48, ('\u{92AE}', false)), // East Asian ideograph
    (0x235823, ('\u{9BD5}', false)), // East Asian ideograph
    (0x21527B, ('\u{807D}', false)), // East Asian ideograph
    (0x217D26, ('\u{5AFF}', false)), // East Asian ideograph
    (0x287061, ('\u{7EC0}', false)), // East Asian ideograph
    (0x23527C, ('\u{99EA}', false)), // East Asian ideograph
    (0x213D27, ('\u{5EE2}', false)), // East Asian ideograph
    (0x284668, ('\u{6C29}', false)), // East Asian ideograph
    (0x6F527D, ('\u{C0F7}', false)), // Korean hangul
    (0x333D28, ('\u{53A8}', false)), // East Asian ideograph
    (0x275B56, ('\u{8F9A}', false)), // East Asian ideograph
    (0x6F527E, ('\u{C0F9}', false)), // Korean hangul
    (0x2D3D29, ('\u{53AE}', false)), // East Asian ideograph
    (0x33417E, ('\u{629E}', false)), // East Asian ideograph
    (0x213D2A, ('\u{5EE3}', false)), // East Asian ideograph (variant of 4B3D2A which maps to 5EE3)
    (0x287279, ('\u{7F25}', false)), // East Asian ideograph
    (0x294568, ('\u{9518}', false)), // East Asian ideograph
    (0x213D2B, ('\u{5EDF}', false)), // East Asian ideograph
    (0x287062, ('\u{7EC1}', false)), // East Asian ideograph
    (0x6F545C, ('\u{C430}', false)), // Korean hangul
    (0x226975, ('\u{7A86}', false)), // East Asian ideograph
    (0x22475C, ('\u{6CA0}', false)), // East Asian ideograph
    (0x216133, ('\u{99D0}', false)), // East Asian ideograph
    (0x226532, ('\u{78B6}', false)), // East Asian ideograph
    (0x213D2D, ('\u{9F90}', false)), // East Asian ideograph
    (0x275B57, ('\u{8F7F}', false)), // East Asian ideograph
    (0x223D2E, ('\u{68A9}', false)), // East Asian ideograph
    (0x213D2F, ('\u{5EF3}', false)), // East Asian ideograph
    (0x212A30, ('\u{E8DE}', false)), // EACC component character
    (0x6F5D79, ('\u{D789}', false)), // Korean hangul
    (0x226F21, ('\u{7C81}', false)), // East Asian ideograph
    (0x216577, ('\u{4F90}', false)), // East Asian ideograph
    (0x216F24, ('\u{542A}', false)), // East Asian ideograph
    (0x33314C, ('\u{5FA0}', false)), // East Asian ideograph
    (0x216F26, ('\u{5422}', false)), // East Asian ideograph
    (0x274D3C, ('\u{5C3D}', false)), // East Asian ideograph
    (0x226F28, ('\u{7C8E}', false)), // East Asian ideograph
    (0x226F29, ('\u{7C91}', false)), // East Asian ideograph
    (0x226F2A, ('\u{7C83}', false)), // East Asian ideograph
    (0x226F2C, ('\u{7C8D}', false)), // East Asian ideograph
    (0x213D32, ('\u{5EF6}', false)), // East Asian ideograph
    (0x216F2E, ('\u{545F}', false)), // East Asian ideograph
    (0x216F2F, ('\u{549C}', false)), // East Asian ideograph
    (0x27393F, ('\u{5941}', false)), // East Asian ideograph
    (0x223D33, ('\u{68A0}', false)), // East Asian ideograph
    (0x213252, ('\u{50D6}', false)), // East Asian ideograph
    (0x216F35, ('\u{5488}', false)), // East Asian ideograph
    (0x216F37, ('\u{547F}', false)), // East Asian ideograph
    (0x216F39, ('\u{5482}', false)), // East Asian ideograph
    (0x226F3A, ('\u{7C99}', false)), // East Asian ideograph
    (0x226F3B, ('\u{7C98}', false)), // East Asian ideograph
    (0x6F5D7A, ('\u{D78C}', false)), // Korean hangul
    (0x226F3E, ('\u{7C9C}', false)), // East Asian ideograph
    (0x226F40, ('\u{7C95}', false)), // East Asian ideograph
    (0x6F5937, ('\u{CC70}', false)), // Korean hangul
    (0x226F42, ('\u{7CA7}', false)), // East Asian ideograph
    (0x226F43, ('\u{7CA2}', false)), // East Asian ideograph
    (0x226F45, ('\u{7C9E}', false)), // East Asian ideograph
    (0x226F46, ('\u{7CA9}', false)), // East Asian ideograph
    (0x6F5939, ('\u{CC98}', false)), // Korean hangul
    (0x226F48, ('\u{7CA8}', false)), // East Asian ideograph
    (0x226F49, ('\u{7CA1}', false)), // East Asian ideograph
    (0x226F4A, ('\u{7CAC}', false)), // East Asian ideograph
    (0x216F4B, ('\u{5474}', false)), // East Asian ideograph
    (0x226F4C, ('\u{7CA6}', false)), // East Asian ideograph
    (0x6F4C6E, ('\u{B2E5}', false)), // Korean hangul
    (0x216F52, ('\u{5466}', false)), // East Asian ideograph
    (0x216F53, ('\u{5464}', false)), // East Asian ideograph
    (0x226F54, ('\u{7CB2}', false)), // East Asian ideograph
    (0x216F55, ('\u{54A4}', false)), // East Asian ideograph
    (0x213D39, ('\u{5F0F}', false)), // East Asian ideograph
    (0x226F58, ('\u{7CBB}', false)), // East Asian ideograph
    (0x226F59, ('\u{7CBF}', false)), // East Asian ideograph
    (0x216F5A, ('\u{54AD}', false)), // East Asian ideograph
    (0x216F5B, ('\u{54BA}', false)), // East Asian ideograph
    (0x216F5C, ('\u{54CF}', false)), // East Asian ideograph
    (0x696F5D, ('\u{9596}', false)), // East Asian ideograph
    (0x226F5E, ('\u{7CBA}', false)), // East Asian ideograph
    (0x226F5F, ('\u{7CBC}', false)), // East Asian ideograph
    (0x216F60, ('\u{54A5}', false)), // East Asian ideograph
    (0x216F63, ('\u{54A7}', false)), // East Asian ideograph
    (0x226F64, ('\u{7CC2}', false)), // East Asian ideograph
    (0x216F66, ('\u{54A2}', false)), // East Asian ideograph
    (0x216F67, ('\u{5472}', false)), // East Asian ideograph
    (0x216F68, ('\u{5470}', false)), // East Asian ideograph
    (0x216F69, ('\u{54BC}', false)), // East Asian ideograph
    (0x216F6A, ('\u{54B7}', false)), // East Asian ideograph
    (0x216F6B, ('\u{54DE}', false)), // East Asian ideograph
    (0x216F6C, ('\u{54D6}', false)), // East Asian ideograph
    (0x226F6D, ('\u{7CCC}', false)), // East Asian ideograph
    (0x226F6F, ('\u{7CC9}', false)), // East Asian ideograph
    (0x226F71, ('\u{7CD2}', false)), // East Asian ideograph
    (0x6F4A22, ('\u{ADC0}', false)), // Korean hangul
    (0x29442D, ('\u{94A1}', false)), // East Asian ideograph
    (0x216F74, ('\u{54C6}', false)), // East Asian ideograph
    (0x225429, ('\u{71CB}', false)), // East Asian ideograph
    (0x226F77, ('\u{7CE1}', false)), // East Asian ideograph
    (0x6F5D7C, ('\u{D798}', false)), // Korean hangul
    (0x33467A, ('\u{6CA1}', false)), // East Asian ideograph
    (0x223D3F, ('\u{6877}', false)), // East Asian ideograph
    (0x216F7C, ('\u{54E2}', false)), // East Asian ideograph
    (0x216F7D, ('\u{5507}', false)), // East Asian ideograph
    (0x233D40, ('\u{9008}', false)), // East Asian ideograph
    (0x233D59, ('\u{9036}', false)), // East Asian ideograph
    (0x277D74, ('\u{5A08}', false)), // East Asian ideograph
    (0x333D42, ('\u{7D43}', false)), // East Asian ideograph
    (0x213A63, ('\u{5B7F}', false)), // East Asian ideograph
    (0x213D43, ('\u{5F27}', false)), // East Asian ideograph
    (0x2D3366, ('\u{5234}', false)), // East Asian ideograph
    (0x6F5D7D, ('\u{D799}', false)), // Korean hangul
    (0x223D44, ('\u{688E}', false)), // East Asian ideograph
    (0x6F593A, ('\u{CC99}', false)), // Korean hangul
    (0x233D45, ('\u{900B}', false)), // East Asian ideograph
    (0x277030, ('\u{5457}', false)), // East Asian ideograph
    (0x697023, ('\u{9666}', false)), // East Asian ideograph
    (0x6F4A64, ('\u{AEDC}', false)), // Korean hangul
    (0x213D47, ('\u{5F35}', false)), // East Asian ideograph
    (0x235C6D, ('\u{9DEE}', false)), // East Asian ideograph
    (0x233D48, ('\u{900C}', false)), // East Asian ideograph
    (0x6F5D7E, ('\u{D79B}', false)), // Korean hangul
    (0x213D49, ('\u{5F3C}', false)), // East Asian ideograph
    (0x6F593B, ('\u{CC9C}', false)), // Korean hangul
    (0x394944, ('\u{6B12}', false)), // East Asian ideograph
    (0x6F5B2F, ('\u{D14D}', false)), // Korean hangul
    (0x224762, ('\u{6CEB}', false)), // East Asian ideograph
    (0x4B3B52, ('\u{8132}', false)), // East Asian ideograph
    (0x2D4474, ('\u{690D}', false)), // East Asian ideograph (variant of 214474 which maps to 690D)
    (0x273D4B, ('\u{5F39}', false)), // East Asian ideograph
    (0x2D4031, ('\u{64A6}', false)), // East Asian ideograph
    (0x6F5A32, ('\u{CF10}', false)), // Korean hangul
    (0x213D4C, ('\u{5F4C}', false)), // East Asian ideograph
    (0x213D4D, ('\u{5F4E}', false)), // East Asian ideograph
    (0x4B4940, ('\u{6F45}', false)), // East Asian ideograph
    (0x23582B, ('\u{9BF1}', false)), // East Asian ideograph
    (0x6F5D25, ('\u{D5E5}', false)), // Korean hangul
    (0x213D4E, ('\u{5F57}', false)), // East Asian ideograph
    (0x6F5030, ('\u{BA65}', false)), // Korean hangul
    (0x224763, ('\u{6CEE}', false)), // East Asian ideograph
    (0x226539, ('\u{78B7}', false)), // East Asian ideograph
    (0x213D50, ('\u{5F5D}', false)), // East Asian ideograph
    (0x6F4C73, ('\u{B2ED}', false)), // Korean hangul
    (0x223D51, ('\u{6917}', false)), // East Asian ideograph
    (0x225870, ('\u{73C8}', false)), // East Asian ideograph
    (0x217247, ('\u{5642}', false)), // East Asian ideograph
    (0x4B6247, ('\u{9D2C}', false)), // East Asian ideograph
    (0x217D52, ('\u{5B2C}', false)), // East Asian ideograph
    (0x23582C, ('\u{9BE1}', false)), // East Asian ideograph
    (0x213D53, ('\u{5F65}', false)), // East Asian ideograph
    (0x28706A, ('\u{7ED0}', false)), // East Asian ideograph
    (0x294871, ('\u{954A}', false)), // East Asian ideograph
    (0x6F5D78, ('\u{D788}', false)), // Korean hangul
    (0x224764, ('\u{6CC0}', false)), // East Asian ideograph
    (0x6F597B, ('\u{CEA0}', false)), // Korean hangul
    (0x275B5F, ('\u{529E}', false)), // East Asian ideograph
    (0x285F5E, ('\u{7618}', false)), // East Asian ideograph
    (0x223D56, ('\u{690B}', false)), // East Asian ideograph
    (0x213259, ('\u{5100}', false)), // East Asian ideograph
    (0x233D57, ('\u{9034}', false)), // East Asian ideograph
    (0x455122, ('\u{7D0D}', false)), // East Asian ideograph
    (0x23582D, ('\u{9BDB}', false)), // East Asian ideograph
    (0x233D58, ('\u{902F}', false)), // East Asian ideograph
    (0x6F593E, ('\u{CCA9}', false)), // Korean hangul
    (0x223D59, ('\u{6904}', false)), // East Asian ideograph
    (0x6F5B45, ('\u{D230}', false)), // Korean hangul
    (0x234B64, ('\u{96DF}', false)), // East Asian ideograph
    (0x6F4C75, ('\u{B2F3}', false)), // Korean hangul
    (0x275B60, ('\u{8F9E}', false)), // East Asian ideograph
    (0x227022, ('\u{7CDD}', false)), // East Asian ideograph
    (0x217023, ('\u{5517}', false)), // East Asian ideograph
    (0x217024, ('\u{54FD}', false)), // East Asian ideograph
    (0x217025, ('\u{54E7}', false)), // East Asian ideograph
    (0x217027, ('\u{54F3}', false)), // East Asian ideograph
    (0x227028, ('\u{7CED}', false)), // East Asian ideograph
    (0x213D5C, ('\u{5F80}', false)), // East Asian ideograph
    (0x21702A, ('\u{54E4}', false)), // East Asian ideograph
    (0x21702B, ('\u{550A}', false)), // East Asian ideograph
    (0x21702D, ('\u{54FF}', false)), // East Asian ideograph
    (0x21702E, ('\u{5518}', false)), // East Asian ideograph
    (0x223D5D, ('\u{6929}', false)), // East Asian ideograph
    (0x227030, ('\u{7CF2}', false)), // East Asian ideograph
    (0x6F593F, ('\u{CCAB}', false)), // Korean hangul
    (0x217032, ('\u{54EF}', false)), // East Asian ideograph
    (0x217034, ('\u{5508}', false)), // East Asian ideograph
    (0x227035, ('\u{7CF4}', false)), // East Asian ideograph
    (0x217038, ('\u{54F6}', false)), // East Asian ideograph
    (0x227039, ('\u{7CF6}', false)), // East Asian ideograph
    (0x6F4C76, ('\u{B2F4}', false)), // Korean hangul
    (0x21703E, ('\u{550E}', false)), // East Asian ideograph
    (0x275B61, ('\u{8FA9}', false)), // East Asian ideograph
    (0x4B5C50, ('\u{9045}', false)), // East Asian ideograph
    (0x692563, ('\u{30E3}', false)), // Katakana letter small YA
    (0x21325B, ('\u{50FB}', false)), // East Asian ideograph
    (0x217044, ('\u{5523}', false)), // East Asian ideograph
    (0x227045, ('\u{7D08}', false)), // East Asian ideograph
    (0x217046, ('\u{550F}', false)), // East Asian ideograph
    (0x217047, ('\u{5511}', false)), // East Asian ideograph
    (0x23582F, ('\u{9BE2}', false)), // East Asian ideograph
    (0x22704A, ('\u{7D13}', false)), // East Asian ideograph
    (0x21704B, ('\u{5575}', false)), // East Asian ideograph
    (0x21704D, ('\u{5573}', false)), // East Asian ideograph
    (0x21704E, ('\u{554C}', false)), // East Asian ideograph
    (0x21704F, ('\u{5576}', false)), // East Asian ideograph
    (0x217050, ('\u{554D}', false)), // East Asian ideograph
    (0x217051, ('\u{555A}', false)), // East Asian ideograph
    (0x227052, ('\u{7D1D}', false)), // East Asian ideograph
    (0x217053, ('\u{553C}', false)), // East Asian ideograph
    (0x217055, ('\u{5550}', false)), // East Asian ideograph
    (0x217057, ('\u{5539}', false)), // East Asian ideograph
    (0x217058, ('\u{5548}', false)), // East Asian ideograph
    (0x217059, ('\u{552D}', false)), // East Asian ideograph
    (0x21705A, ('\u{5551}', false)), // East Asian ideograph
    (0x6F4C77, ('\u{B2F5}', false)), // Korean hangul
    (0x21705D, ('\u{552A}', false)), // East Asian ideograph
    (0x213D65, ('\u{5F8C}', false)), // East Asian ideograph
    (0x217060, ('\u{5562}', false)), // East Asian ideograph
    (0x217061, ('\u{5536}', false)), // East Asian ideograph
    (0x227062, ('\u{7D32}', false)), // East Asian ideograph
    (0x217064, ('\u{5549}', false)), // East Asian ideograph
    (0x227065, ('\u{7D31}', false)), // East Asian ideograph
    (0x335830, ('\u{658D}', false)), // East Asian ideograph
    (0x227068, ('\u{7D45}', false)), // East Asian ideograph
    (0x21706A, ('\u{5540}', false)), // East Asian ideograph
    (0x21706B, ('\u{5535}', false)), // East Asian ideograph
    (0x22706C, ('\u{7D29}', false)), // East Asian ideograph
    (0x6F5941, ('\u{CCB4}', false)), // Korean hangul
    (0x22706F, ('\u{7D41}', false)), // East Asian ideograph
    (0x217070, ('\u{5545}', false)), // East Asian ideograph
    (0x227071, ('\u{7D3E}', false)), // East Asian ideograph
    (0x234B67, ('\u{96DD}', false)), // East Asian ideograph
    (0x213D69, ('\u{5F98}', false)), // East Asian ideograph
    (0x217079, ('\u{553F}', false)), // East Asian ideograph
    (0x22707A, ('\u{7D5C}', false)), // East Asian ideograph
    (0x21707B, ('\u{5541}', false)), // East Asian ideograph
    (0x22707C, ('\u{7D53}', false)), // East Asian ideograph
    (0x21707D, ('\u{5565}', false)), // East Asian ideograph
    (0x22707E, ('\u{7D5A}', false)), // East Asian ideograph
    (0x275779, ('\u{891B}', false)), // East Asian ideograph
    (0x225432, ('\u{71EB}', false)), // East Asian ideograph
    (0x235831, ('\u{9BF0}', false)), // East Asian ideograph
    (0x213D6C, ('\u{5F9E}', false)), // East Asian ideograph
    (0x6F5942, ('\u{CCB5}', false)), // Korean hangul
    (0x213F27, ('\u{614D}', false)), // East Asian ideograph
    (0x213B3F, ('\u{5C08}', false)), // East Asian ideograph
    (0x283D6E, ('\u{67A8}', false)), // East Asian ideograph
    (0x2D6251, ('\u{5869}', false)), // East Asian ideograph
    (0x6F4C79, ('\u{B2F9}', false)), // Korean hangul
    (0x275B64, ('\u{519C}', false)), // East Asian ideograph
    (0x273D6F, ('\u{590D}', false)), // East Asian ideograph
    (0x21325E, ('\u{5102}', false)), // East Asian ideograph
    (0x6F4A24, ('\u{ADC8}', false)), // Korean hangul
    (0x4B4947, ('\u{7AC3}', false)), // East Asian ideograph
    (0x6F5943, ('\u{CCB8}', false)), // Korean hangul
    (0x213F28, ('\u{614B}', false)), // East Asian ideograph
    (0x213D73, ('\u{5FAE}', false)), // East Asian ideograph
    (0x2D6252, ('\u{78B1}', false)), // East Asian ideograph
    (0x295A59, ('\u{9E38}', false)), // East Asian ideograph
    (0x277328, ('\u{54DC}', false)), // East Asian ideograph
    (0x213D74, ('\u{5FB9}', false)), // East Asian ideograph
    (0x21325F, ('\u{510D}', false)), // East Asian ideograph
    (0x285B21, ('\u{740F}', false)), // East Asian ideograph
    (0x233A5D, ('\u{8E94}', false)), // East Asian ideograph
    (0x213D75, ('\u{5FB7}', false)), // East Asian ideograph
    (0x275D71, ('\u{94A2}', false)), // East Asian ideograph
    (0x217D76, ('\u{5B4B}', false)), // East Asian ideograph
    (0x6F5023, ('\u{BA3C}', false)), // Korean hangul
    (0x226822, ('\u{79B8}', false)), // East Asian ideograph
    (0x223D78, ('\u{68FD}', false)), // East Asian ideograph
    (0x226823, ('\u{79BA}', false)), // East Asian ideograph
    (0x213D79, ('\u{5FC5}', false)), // East Asian ideograph
    (0x4B7E6A, ('\u{5BC3}', false)), // East Asian ideograph (variant of 217E6A which maps to 5BC3)
    (0x223F3E, ('\u{696F}', false)), // East Asian ideograph
    (0x212A33, ('\u{E8E0}', false)), // EACC component character
    (0x223D7B, ('\u{68F3}', false)), // East Asian ideograph
    (0x6F5945, ('\u{CCC7}', false)), // Korean hangul
    (0x6F5B31, ('\u{D154}', false)), // Korean hangul
    (0x213D7C, ('\u{5FCC}', false)), // East Asian ideograph
    (0x27493C, ('\u{6FD2}', false)), // East Asian ideograph
    (0x213261, ('\u{5109}', false)), // East Asian ideograph
    (0x233A5F, ('\u{8E92}', false)), // East Asian ideograph
    (0x29282A, ('\u{8539}', false)), // East Asian ideograph
    (0x29494D, ('\u{9609}', false)), // East Asian ideograph
    (0x6F5342, ('\u{C190}', false)), // Korean hangul
    (0x696D3F, ('\u{8EBE}', false)), // East Asian ideograph
    (0x213F2B, ('\u{6134}', false)), // East Asian ideograph
    (0x334729, ('\u{6E2B}', false)), // East Asian ideograph
    (0x6F4C7D, ('\u{B300}', false)), // Korean hangul
    (0x2F2A64, ('\u{87B5}', false)), // East Asian ideograph
    (0x22682E, ('\u{79D5}', false)), // East Asian ideograph
    (0x287431, ('\u{575B}', false)), // East Asian ideograph (duplicate simplified)
    (0x275E60, ('\u{960E}', false)), // East Asian ideograph
    (0x233A60, ('\u{8E93}', false)), // East Asian ideograph
    (0x22282F, ('\u{5EA4}', false)), // East Asian ideograph
    (0x227122, ('\u{7D70}', false)), // East Asian ideograph
    (0x217123, ('\u{5591}', false)), // East Asian ideograph
    (0x697124, ('\u{98AA}', false)), // East Asian ideograph
    (0x217125, ('\u{5577}', false)), // East Asian ideograph
    (0x217126, ('\u{55A8}', false)), // East Asian ideograph
    (0x217127, ('\u{55AD}', false)), // East Asian ideograph
    (0x227129, ('\u{7D67}', false)), // East Asian ideograph
    (0x21712A, ('\u{5605}', false)), // East Asian ideograph
    (0x22712B, ('\u{7D6A}', false)), // East Asian ideograph
    (0x22712C, ('\u{7D6B}', false)), // East Asian ideograph
    (0x216832, ('\u{50D4}', false)), // East Asian ideograph
    (0x21712F, ('\u{5586}', false)), // East Asian ideograph
    (0x227130, ('\u{7D73}', false)), // East Asian ideograph
    (0x227134, ('\u{7D4E}', false)), // East Asian ideograph
    (0x217136, ('\u{55B4}', false)), // East Asian ideograph
    (0x227137, ('\u{7D8B}', false)), // East Asian ideograph
    (0x227139, ('\u{7D88}', false)), // East Asian ideograph
    (0x22713B, ('\u{7D85}', false)), // East Asian ideograph
    (0x2D514A, ('\u{6DD6}', false)), // East Asian ideograph
    (0x22713D, ('\u{7D8E}', false)), // East Asian ideograph
    (0x216835, ('\u{50E6}', false)), // East Asian ideograph
    (0x6F5948, ('\u{CD08}', false)), // Korean hangul
    (0x227142, ('\u{7D7F}', false)), // East Asian ideograph
    (0x217143, ('\u{55E2}', false)), // East Asian ideograph (variant of 2D7143 which maps to 55E2)
    (0x227144, ('\u{7D86}', false)), // East Asian ideograph
    (0x217145, ('\u{558E}', false)), // East Asian ideograph
    (0x217147, ('\u{55B5}', false)), // East Asian ideograph
    (0x227148, ('\u{7D8D}', false)), // East Asian ideograph
    (0x217149, ('\u{558F}', false)), // East Asian ideograph
    (0x21714B, ('\u{5559}', false)), // East Asian ideograph
    (0x22714D, ('\u{7D83}', false)), // East Asian ideograph
    (0x22714F, ('\u{7D7D}', false)), // East Asian ideograph
    (0x217150, ('\u{55A4}', false)), // East Asian ideograph
    (0x217151, ('\u{5592}', false)), // East Asian ideograph
    (0x217152, ('\u{5599}', false)), // East Asian ideograph
    (0x227154, ('\u{7D7B}', false)), // East Asian ideograph
    (0x233A62, ('\u{8E90}', false)), // East Asian ideograph
    (0x217156, ('\u{55F4}', false)), // East Asian ideograph
    (0x227158, ('\u{7D7A}', false)), // East Asian ideograph
    (0x227159, ('\u{7D96}', false)), // East Asian ideograph
    (0x22715A, ('\u{7D5B}', false)), // East Asian ideograph
    (0x22715B, ('\u{7D8C}', false)), // East Asian ideograph
    (0x21715C, ('\u{55DE}', false)), // East Asian ideograph
    (0x21715D, ('\u{55D9}', false)), // East Asian ideograph
    (0x21715E, ('\u{55C3}', false)), // East Asian ideograph
    (0x21715F, ('\u{55C9}', false)), // East Asian ideograph
    (0x217161, ('\u{55CA}', false)), // East Asian ideograph
    (0x227162, ('\u{7DAE}', false)), // East Asian ideograph
    (0x21683B, ('\u{50CE}', false)), // East Asian ideograph
    (0x217164, ('\u{55D4}', false)), // East Asian ideograph
    (0x217165, ('\u{55C4}', false)), // East Asian ideograph
    (0x227167, ('\u{7DCB}', false)), // East Asian ideograph
    (0x227169, ('\u{7DAA}', false)), // East Asian ideograph
    (0x22716A, ('\u{7DCE}', false)), // East Asian ideograph
    (0x22716B, ('\u{7DC9}', false)), // East Asian ideograph
    (0x692565, ('\u{30E5}', false)), // Katakana letter small YU
    (0x22716E, ('\u{7DC5}', false)), // East Asian ideograph
    (0x22716F, ('\u{7DA6}', false)), // East Asian ideograph
    (0x217170, ('\u{55D2}', false)), // East Asian ideograph
    (0x223664, ('\u{6585}', false)), // East Asian ideograph
    (0x277C36, ('\u{59AB}', false)), // East Asian ideograph
    (0x22543A, ('\u{71F5}', false)), // East Asian ideograph
    (0x227174, ('\u{7DC4}', false)), // East Asian ideograph
    (0x217175, ('\u{55E5}', false)), // East Asian ideograph
    (0x6F4871, ('\u{AC1C}', false)), // Korean hangul
    (0x217177, ('\u{55D6}', false)), // East Asian ideograph
    (0x227178, ('\u{7DAC}', false)), // East Asian ideograph
    (0x217179, ('\u{55F2}', false)), // East Asian ideograph
    (0x6F5C77, ('\u{D590}', false)), // Korean hangul
    (0x2E717C, ('\u{7D63}', false)), // East Asian ideograph
    (0x22717D, ('\u{7DB9}', false)), // East Asian ideograph
    (0x21717E, ('\u{5627}', false)), // East Asian ideograph
    (0x292840, ('\u{84E0}', false)), // East Asian ideograph
    (0x235F5F, ('\u{9F37}', false)), // East Asian ideograph
    (0x216841, ('\u{50F3}', false)), // East Asian ideograph
    (0x216842, ('\u{50E8}', false)), // East Asian ideograph
    (0x217255, ('\u{5635}', false)), // East Asian ideograph
    (0x2D514D, ('\u{7D2C}', false)), // East Asian ideograph
    (0x216844, ('\u{50F0}', false)), // East Asian ideograph
    (0x6F594B, ('\u{CD10}', false)), // Korean hangul
    (0x224772, ('\u{6CF5}', false)), // East Asian ideograph
    (0x6F4E69, ('\u{B78D}', false)), // Korean hangul
    (0x23307D, ('\u{89AF}', false)), // East Asian ideograph
    (0x295B35, ('\u{9E2B}', false)), // East Asian ideograph
    (0x4B7874, ('\u{590A}', false)), // East Asian ideograph
    (0x23284C, ('\u{8645}', false)), // East Asian ideograph
    (0x6F4A26, ('\u{ADD1}', false)), // Korean hangul
    (0x22543D, ('\u{71F3}', false)), // East Asian ideograph
    (0x234E53, ('\u{97D8}', false)), // East Asian ideograph
    (0x69684D, ('\u{8422}', false)), // East Asian ideograph
    (0x333623, ('\u{9F69}', false)), // East Asian ideograph
    (0x225B61, ('\u{74B1}', false)), // East Asian ideograph
    (0x6F553B, ('\u{C58C}', false)), // Korean hangul
    (0x706D3F, ('\u{781C}', false)), // East Asian ideograph
    (0x4B587A, ('\u{8ACB}', false)), // East Asian ideograph
    (0x6F7723, ('\u{E8CA}', false)), // Korean hangul
    (0x213F32, ('\u{615D}', false)), // East Asian ideograph
    (0x234730, ('\u{93E6}', false)), // East Asian ideograph
    (0x222851, ('\u{5ECC}', false)), // East Asian ideograph
    (0x6F594E, ('\u{CD1B}', false)), // Korean hangul
    (0x284E62, ('\u{6F4D}', false)), // East Asian ideograph
    (0x6F5732, ('\u{C7CC}', false)), // Korean hangul
    (0x6F5328, ('\u{C126}', false)), // Korean hangul
    (0x213F33, ('\u{6182}', false)), // East Asian ideograph
    (0x285F6F, ('\u{7605}', false)), // East Asian ideograph
    (0x6F4A3D, ('\u{AE43}', false)), // Korean hangul
    (0x6F4E25, ('\u{B545}', false)), // Korean hangul
    (0x295F2B, ('\u{9F0D}', false)), // East Asian ideograph
    (0x212A35, ('\u{E8E2}', false)), // EACC component character
    (0x6F594F, ('\u{CD1D}', false)), // Korean hangul
    (0x6F5B33, ('\u{D15D}', false)), // Korean hangul
    (0x274621, ('\u{6B22}', false)), // East Asian ideograph
    (0x232859, ('\u{864D}', false)), // East Asian ideograph
    (0x224333, ('\u{6ADF}', false)), // East Asian ideograph
    (0x234732, ('\u{940B}', false)), // East Asian ideograph
    (0x21797C, ('\u{5997}', false)), // East Asian ideograph
    (0x23285A, ('\u{8653}', false)), // East Asian ideograph
    (0x6F4E43, ('\u{B614}', false)), // Korean hangul
    (0x227222, ('\u{7D9F}', false)), // East Asian ideograph
    (0x217224, ('\u{55FB}', false)), // East Asian ideograph
    (0x217225, ('\u{5612}', false)), // East Asian ideograph
    (0x217227, ('\u{55F8}', false)), // East Asian ideograph
    (0x217228, ('\u{560F}', false)), // East Asian ideograph
    (0x227229, ('\u{7DE1}', false)), // East Asian ideograph
    (0x22722A, ('\u{7DD9}', false)), // East Asian ideograph
    (0x22722B, ('\u{7DE4}', false)), // East Asian ideograph
    (0x6F4F44, ('\u{B8E9}', false)), // Korean hangul
    (0x21722E, ('\u{561E}', false)), // East Asian ideograph
    (0x217539, ('\u{5790}', false)), // East Asian ideograph
    (0x227231, ('\u{7DD7}', false)), // East Asian ideograph
    (0x295263, ('\u{9A75}', false)), // East Asian ideograph
    (0x217234, ('\u{561C}', false)), // East Asian ideograph
    (0x217235, ('\u{5610}', false)), // East Asian ideograph
    (0x217236, ('\u{5601}', false)), // East Asian ideograph
    (0x217238, ('\u{5613}', false)), // East Asian ideograph
    (0x217239, ('\u{55F6}', false)), // East Asian ideograph
    (0x22723A, ('\u{7E06}', false)), // East Asian ideograph
    (0x21685F, ('\u{5105}', false)), // East Asian ideograph
    (0x21723C, ('\u{5602}', false)), // East Asian ideograph
    (0x22723E, ('\u{7DE6}', false)), // East Asian ideograph
    (0x697240, ('\u{9BB4}', false)), // East Asian ideograph
    (0x217242, ('\u{561D}', false)), // East Asian ideograph
    (0x27577C, ('\u{88C6}', false)), // East Asian ideograph
    (0x217244, ('\u{55FF}', false)), // East Asian ideograph
    (0x697245, ('\u{9BCF}', false)), // East Asian ideograph
    (0x227246, ('\u{7DDC}', false)), // East Asian ideograph
    (0x216861, ('\u{50FC}', false)), // East Asian ideograph
    (0x217248, ('\u{564C}', false)), // East Asian ideograph
    (0x227249, ('\u{7DE5}', false)), // East Asian ideograph
    (0x22724B, ('\u{7DF5}', false)), // East Asian ideograph
    (0x6F4E6A, ('\u{B78F}', false)), // Korean hangul
    (0x295021, ('\u{98A2}', false)), // East Asian ideograph
    (0x2E7328, ('\u{5FAD}', false)), // East Asian ideograph
    (0x227250, ('\u{7E17}', false)), // East Asian ideograph
    (0x227251, ('\u{7E1E}', false)), // East Asian ideograph
    (0x227252, ('\u{7E21}', false)), // East Asian ideograph
    (0x227253, ('\u{7E0B}', false)), // East Asian ideograph
    (0x224335, ('\u{6ADE}', false)), // East Asian ideograph
    (0x227256, ('\u{7E22}', false)), // East Asian ideograph
    (0x217257, ('\u{5649}', false)), // East Asian ideograph
    (0x217258, ('\u{5641}', false)), // East Asian ideograph
    (0x2D5124, ('\u{5E0B}', false)), // East Asian ideograph
    (0x22725B, ('\u{7E20}', false)), // East Asian ideograph
    (0x21725C, ('\u{5658}', false)), // East Asian ideograph
    (0x22725D, ('\u{7E1D}', false)), // East Asian ideograph
    (0x21725E, ('\u{5654}', false)), // East Asian ideograph
    (0x216865, ('\u{5106}', false)), // East Asian ideograph
    (0x217260, ('\u{562A}', false)), // East Asian ideograph
    (0x217261, ('\u{563D}', false)), // East Asian ideograph
    (0x217264, ('\u{562C}', false)), // East Asian ideograph
    (0x227265, ('\u{7E15}', false)), // East Asian ideograph
    (0x6F5474, ('\u{C52C}', false)), // Korean hangul
    (0x217267, ('\u{5638}', false)), // East Asian ideograph
    (0x4D5154, ('\u{9942}', false)), // East Asian ideograph
    (0x217269, ('\u{564D}', false)), // East Asian ideograph
    (0x22726A, ('\u{7E0F}', false)), // East Asian ideograph
    (0x21726B, ('\u{562B}', false)), // East Asian ideograph
    (0x21726C, ('\u{564F}', false)), // East Asian ideograph
    (0x22726D, ('\u{7E3B}', false)), // East Asian ideograph
    (0x21726E, ('\u{5670}', false)), // East Asian ideograph
    (0x21726F, ('\u{565F}', false)), // East Asian ideograph
    (0x217270, ('\u{567C}', false)), // East Asian ideograph
    (0x227271, ('\u{7E34}', false)), // East Asian ideograph
    (0x227272, ('\u{7E2D}', false)), // East Asian ideograph
    (0x227273, ('\u{7E2F}', false)), // East Asian ideograph
    (0x227275, ('\u{7E36}', false)), // East Asian ideograph
    (0x227277, ('\u{7E3A}', false)), // East Asian ideograph
    (0x217278, ('\u{5676}', false)), // East Asian ideograph
    (0x227279, ('\u{7E39}', false)), // East Asian ideograph
    (0x21727A, ('\u{5666}', false)), // East Asian ideograph
    (0x21727B, ('\u{5673}', false)), // East Asian ideograph
    (0x21727C, ('\u{566D}', false)), // East Asian ideograph
    (0x22727D, ('\u{7E44}', false)), // East Asian ideograph
    (0x21727E, ('\u{5672}', false)), // East Asian ideograph
    (0x33537D, ('\u{9AD5}', false)), // East Asian ideograph
    (0x4B3E7E, ('\u{60A9}', false)), // East Asian ideograph
    (0x6F5953, ('\u{CD94}', false)), // Korean hangul
    (0x6F7729, ('\u{AE0E}', false)), // Korean hangul
    (0x224B30, ('\u{6E36}', false)), // East Asian ideograph
    (0x2D6262, ('\u{9EC4}', false)), // East Asian ideograph
    (0x2D4049, ('\u{67B4}', false)), // East Asian ideograph
    (0x4B5C54, ('\u{8F9F}', false)), // East Asian ideograph (duplicate simplified)
    (0x2E686F, ('\u{7A19}', false)), // East Asian ideograph
    (0x6F4873, ('\u{AC20}', false)), // Korean hangul
    (0x33362A, ('\u{9B28}', false)), // East Asian ideograph
    (0x216871, ('\u{5115}', false)), // East Asian ideograph
    (0x6F5954, ('\u{CD95}', false)), // Korean hangul
    (0x6F5B34, ('\u{D15F}', false)), // Korean hangul
    (0x234B7A, ('\u{96F5}', false)), // East Asian ideograph
    (0x6F4F45, ('\u{B8EC}', false)), // Korean hangul
    (0x6F5031, ('\u{BA67}', false)), // Korean hangul
    (0x6F5955, ('\u{CD98}', false)), // Korean hangul
    (0x215321, ('\u{8085}', false)), // East Asian ideograph
    (0x6F772B, ('\u{AE11}', false)), // Korean hangul
    (0x213F3A, ('\u{615F}', false)), // East Asian ideograph
    (0x6F5322, ('\u{C11D}', false)), // Korean hangul
    (0x225323, ('\u{7192}', false)), // East Asian ideograph
    (0x2D5E21, ('\u{9418}', false)), // East Asian ideograph
    (0x29415C, ('\u{917E}', false)), // East Asian ideograph
    (0x215324, ('\u{808B}', false)), // East Asian ideograph
    (0x6F5325, ('\u{C123}', false)), // Korean hangul
    (0x224539, ('\u{6BAB}', false)), // East Asian ideograph
    (0x6F5956, ('\u{CD9C}', false)), // Korean hangul
    (0x695326, ('\u{54D8}', false)), // East Asian ideograph
    (0x22477D, ('\u{6CC2}', false)), // East Asian ideograph
    (0x23287C, ('\u{866F}', false)), // East Asian ideograph
    (0x6F5327, ('\u{C125}', false)), // Korean hangul
    (0x275E35, ('\u{9558}', false)), // East Asian ideograph
    (0x2D404C, ('\u{6283}', false)), // East Asian ideograph
    (0x6F4B6B, ('\u{B0E0}', false)), // Korean hangul
    (0x275735, ('\u{8681}', false)), // East Asian ideograph
    (0x6F4A28, ('\u{ADDC}', false)), // Korean hangul
    (0x235329, ('\u{99F8}', false)), // East Asian ideograph
    (0x225447, ('\u{71E0}', false)), // East Asian ideograph
    (0x23532A, ('\u{99F4}', false)), // East Asian ideograph
    (0x6F5957, ('\u{CDA4}', false)), // Korean hangul
    (0x21532B, ('\u{8096}', false)), // East Asian ideograph
    (0x276842, ('\u{507E}', false)), // East Asian ideograph
    (0x274629, ('\u{5C81}', false)), // East Asian ideograph
    (0x213F3C, ('\u{617E}', false)), // East Asian ideograph
    (0x21532C, ('\u{80B2}', false)), // East Asian ideograph
    (0x6F5235, ('\u{BED8}', false)), // Korean hangul
    (0x6F532D, ('\u{C12F}', false)), // Korean hangul
    (0x213273, ('\u{5147}', false)), // East Asian ideograph
    (0x6F532E, ('\u{C130}', false)), // Korean hangul
    (0x275D75, ('\u{9525}', false)), // East Asian ideograph
    (0x216F43, ('\u{546B}', false)), // East Asian ideograph
    (0x6F5958, ('\u{CDA5}', false)), // Korean hangul
    (0x215330, ('\u{80A2}', false)), // East Asian ideograph
    (0x27462A, ('\u{5386}', false)), // East Asian ideograph
    (0x217325, ('\u{5693}', false)), // East Asian ideograph
    (0x227326, ('\u{7E3F}', false)), // East Asian ideograph
    (0x215331, ('\u{80AB}', false)), // East Asian ideograph
    (0x227328, ('\u{7E47}', false)), // East Asian ideograph
    (0x225332, ('\u{7185}', false)), // East Asian ideograph
    (0x22732F, ('\u{7E51}', false)), // East Asian ideograph
    (0x217332, ('\u{56BA}', false)), // East Asian ideograph
    (0x215333, ('\u{80AF}', false)), // East Asian ideograph
    (0x227334, ('\u{7E67}', false)), // East Asian ideograph
    (0x217335, ('\u{5684}', false)), // East Asian ideograph
    (0x217336, ('\u{5691}', false)), // East Asian ideograph
    (0x227337, ('\u{7E56}', false)), // East Asian ideograph
    (0x6F5334, ('\u{C140}', false)), // Korean hangul
    (0x21733E, ('\u{569E}', false)), // East Asian ideograph
    (0x6F5335, ('\u{C148}', false)), // Korean hangul
    (0x27462B, ('\u{5F52}', false)), // East Asian ideograph
    (0x217342, ('\u{569A}', false)), // East Asian ideograph
    (0x213F3E, ('\u{61B2}', false)), // East Asian ideograph
    (0x225336, ('\u{717C}', false)), // East Asian ideograph
    (0x227348, ('\u{7E68}', false)), // East Asian ideograph
    (0x227349, ('\u{7E6E}', false)), // East Asian ideograph
    (0x21734B, ('\u{56AD}', false)), // East Asian ideograph
    (0x21734C, ('\u{56A6}', false)), // East Asian ideograph
    (0x22734E, ('\u{7E70}', false)), // East Asian ideograph
    (0x6F4E66, ('\u{B780}', false)), // Korean hangul
    (0x227351, ('\u{7E6F}', false)), // East Asian ideograph
    (0x227352, ('\u{7E73}', false)), // East Asian ideograph
    (0x217353, ('\u{56B2}', false)), // East Asian ideograph
    (0x235849, ('\u{9C0A}', false)), // East Asian ideograph
    (0x225339, ('\u{7198}', false)), // East Asian ideograph
    (0x227358, ('\u{7E7B}', false)), // East Asian ideograph
    (0x227359, ('\u{7E7E}', false)), // East Asian ideograph
    (0x21735A, ('\u{56B3}', false)), // East Asian ideograph
    (0x22735B, ('\u{7E81}', false)), // East Asian ideograph
    (0x6F595A, ('\u{CDA9}', false)), // Korean hangul
    (0x22735D, ('\u{7E8A}', false)), // East Asian ideograph
    (0x22735E, ('\u{7E87}', false)), // East Asian ideograph
    (0x6F7730, ('\u{AF50}', false)), // Korean hangul
    (0x227360, ('\u{7E88}', false)), // East Asian ideograph
    (0x217362, ('\u{56CF}', false)), // East Asian ideograph
    (0x4B533B, ('\u{695C}', false)), // East Asian ideograph
    (0x227364, ('\u{7E86}', false)), // East Asian ideograph
    (0x23473D, ('\u{93FB}', false)), // East Asian ideograph
    (0x217367, ('\u{56CD}', false)), // East Asian ideograph
    (0x22533C, ('\u{7197}', false)), // East Asian ideograph
    (0x22736A, ('\u{7E91}', false)), // East Asian ideograph
    (0x21736B, ('\u{56D7}', false)), // East Asian ideograph
    (0x22736D, ('\u{7E94}', false)), // East Asian ideograph
    (0x70736E, ('\u{7BA2}', false)), // East Asian ideograph
    (0x23533D, ('\u{9A0F}', false)), // East Asian ideograph
    (0x227370, ('\u{7E9B}', false)), // East Asian ideograph
    (0x227371, ('\u{7E9A}', false)), // East Asian ideograph
    (0x22544B, ('\u{720C}', false)), // East Asian ideograph
    (0x227373, ('\u{7E99}', false)), // East Asian ideograph
    (0x227374, ('\u{7E98}', false)), // East Asian ideograph
    (0x22533E, ('\u{71B5}', false)), // East Asian ideograph
    (0x217376, ('\u{56EE}', false)), // East Asian ideograph
    (0x217377, ('\u{56E7}', false)), // East Asian ideograph
    (0x217379, ('\u{56FB}', false)), // East Asian ideograph
    (0x22533F, ('\u{71A9}', false)), // East Asian ideograph
    (0x21737E, ('\u{56F7}', false)), // East Asian ideograph
    (0x222569, ('\u{5D97}', false)), // East Asian ideograph
    (0x295340, ('\u{9A92}', false)), // East Asian ideograph
    (0x4B3853, ('\u{586D}', false)), // East Asian ideograph (not in Unicode)
    (0x4B5629, ('\u{85CD}', false)), // East Asian ideograph
    (0x6F5341, ('\u{C18E}', false)), // Korean hangul
    (0x6F4A29, ('\u{ADE0}', false)), // Korean hangul
    (0x225342, ('\u{71A5}', false)), // East Asian ideograph
    (0x334260, ('\u{89D4}', false)), // East Asian ideograph
    (0x275E50, ('\u{95ED}', false)), // East Asian ideograph
    (0x23584B, ('\u{9C08}', false)), // East Asian ideograph
    (0x2E3936, ('\u{66CD}', false)), // East Asian ideograph
    (0x6F595C, ('\u{CDC4}', false)), // Korean hangul
    (0x235344, ('\u{9A04}', false)), // East Asian ideograph
    (0x6F7732, ('\u{B060}', false)), // Korean hangul
    (0x235345, ('\u{9A11}', false)), // East Asian ideograph
    (0x275B7E, ('\u{8FDE}', false)), // East Asian ideograph
    (0x2D5E28, ('\u{93C1}', false)), // East Asian ideograph
    (0x213A6A, ('\u{5B8B}', false)), // East Asian ideograph
    (0x235347, ('\u{9A05}', false)), // East Asian ideograph
    (0x4B4874, ('\u{6FF3}', false)), // East Asian ideograph
    (0x23584C, ('\u{9C14}', false)), // East Asian ideograph
    (0x215348, ('\u{80FD}', false)), // East Asian ideograph
    (0x6F5349, ('\u{C1A8}', false)), // Korean hangul
    (0x705D5C, ('\u{8488}', false)), // East Asian ideograph
    (0x6F7733, ('\u{B9C4}', false)), // Korean hangul
    (0x224B32, ('\u{6E72}', false)), // East Asian ideograph
    (0x6F5128, ('\u{BC88}', false)), // Korean hangul
    (0x22655A, ('\u{78D8}', false)), // East Asian ideograph
    (0x27534A, ('\u{8090}', false)), // East Asian ideograph
    (0x226D4B, ('\u{7C0C}', false)), // East Asian ideograph
    (0x334740, ('\u{6D1A}', false)), // East Asian ideograph
    (0x4B562B, ('\u{8535}', false)), // East Asian ideograph
    (0x2D534B, ('\u{80F7}', false)), // East Asian ideograph
    (0x27573C, ('\u{86CE}', false)), // East Asian ideograph
    (0x21534C, ('\u{810A}', false)), // East Asian ideograph
    (0x23584D, ('\u{9C04}', false)), // East Asian ideograph
    (0x23534D, ('\u{9A22}', false)), // East Asian ideograph
    (0x6F595E, ('\u{CDE8}', false)), // Korean hangul
    (0x22534E, ('\u{71AF}', false)), // East Asian ideograph
    (0x6F5B36, ('\u{D161}', false)), // Korean hangul
    (0x6F7734, ('\u{C54D}', false)), // Korean hangul
    (0x23534F, ('\u{9A20}', false)), // East Asian ideograph
    (0x6F5350, ('\u{C1E4}', false)), // Korean hangul
    (0x21327A, ('\u{5152}', false)), // East Asian ideograph
    (0x2D6134, ('\u{99DE}', false)), // East Asian ideograph
    (0x233A78, ('\u{8EB3}', false)), // East Asian ideograph
    (0x4C6F7B, ('\u{7CE8}', false)), // East Asian ideograph
    (0x235352, ('\u{9A27}', false)), // East Asian ideograph
    (0x6F5343, ('\u{C194}', false)), // Korean hangul
    (0x6F5353, ('\u{C1F1}', false)), // Korean hangul
    (0x6F7735, ('\u{C54F}', false)), // Korean hangul
    (0x213F44, ('\u{619A}', false)), // East Asian ideograph
    (0x6F5354, ('\u{C1F3}', false)), // Korean hangul
    (0x4C7C45, ('\u{82AE}', false)), // East Asian ideograph
    (0x225355, ('\u{719A}', false)), // East Asian ideograph
    (0x4D4832, ('\u{953F}', false)), // East Asian ideograph
    (0x27573E, ('\u{8721}', false)), // East Asian ideograph
    (0x22367A, ('\u{65A0}', false)), // East Asian ideograph
    (0x223237, ('\u{63B1}', false)), // East Asian ideograph
    (0x6F5629, ('\u{C65C}', false)), // Korean hangul
    (0x225357, ('\u{71B3}', false)), // East Asian ideograph
    (0x27407B, ('\u{5377}', false)), // East Asian ideograph
    (0x275358, ('\u{80BE}', false)), // East Asian ideograph
    (0x213F45, ('\u{61A9}', false)), // East Asian ideograph
    (0x215359, ('\u{8139}', false)), // East Asian ideograph
    (0x2D416E, ('\u{6534}', false)), // East Asian ideograph
    (0x23535A, ('\u{9A38}', false)), // East Asian ideograph
    (0x217421, ('\u{56F9}', false)), // East Asian ideograph
    (0x6F4A2A, ('\u{ADE4}', false)), // Korean hangul
    (0x294435, ('\u{950A}', false)), // East Asian ideograph
    (0x217424, ('\u{56FF}', false)), // East Asian ideograph
    (0x227425, ('\u{7F43}', false)), // East Asian ideograph
    (0x275E51, ('\u{95F5}', false)), // East Asian ideograph
    (0x217427, ('\u{5705}', false)), // East Asian ideograph
    (0x227428, ('\u{7F45}', false)), // East Asian ideograph
    (0x217429, ('\u{5702}', false)), // East Asian ideograph
    (0x22742B, ('\u{7F4B}', false)), // East Asian ideograph
    (0x21742C, ('\u{570A}', false)), // East Asian ideograph
    (0x21742D, ('\u{5709}', false)), // East Asian ideograph
    (0x22742E, ('\u{7F4C}', false)), // East Asian ideograph
    (0x22742F, ('\u{7F4D}', false)), // East Asian ideograph
    (0x217430, ('\u{570C}', false)), // East Asian ideograph
    (0x217431, ('\u{5715}', false)), // East Asian ideograph
    (0x227432, ('\u{7F4F}', false)), // East Asian ideograph
    (0x213F46, ('\u{6194}', false)), // East Asian ideograph
    (0x27535E, ('\u{80A0}', false)), // East Asian ideograph
    (0x224345, ('\u{6AE8}', false)), // East Asian ideograph
    (0x217437, ('\u{571C}', false)), // East Asian ideograph
    (0x4B423A, ('\u{64B9}', false)), // East Asian ideograph
    (0x217439, ('\u{571D}', false)), // East Asian ideograph
    (0x21743A, ('\u{571E}', false)), // East Asian ideograph
    (0x6F535F, ('\u{C220}', false)), // Korean hangul
    (0x22743E, ('\u{7F60}', false)), // East Asian ideograph
    (0x22743F, ('\u{7F61}', false)), // East Asian ideograph
    (0x235360, ('\u{9A2D}', false)), // East Asian ideograph
    (0x217442, ('\u{572E}', false)), // East Asian ideograph
    (0x227443, ('\u{7F5D}', false)), // East Asian ideograph
    (0x227445, ('\u{7F5B}', false)), // East Asian ideograph
    (0x235361, ('\u{9A35}', false)), // East Asian ideograph
    (0x217448, ('\u{5738}', false)), // East Asian ideograph
    (0x21744C, ('\u{572A}', false)), // East Asian ideograph
    (0x215362, ('\u{816B}', false)), // East Asian ideograph
    (0x6F4B43, ('\u{B05D}', false)), // Korean hangul
    (0x227450, ('\u{7F65}', false)), // East Asian ideograph
    (0x227451, ('\u{7F66}', false)), // East Asian ideograph
    (0x213F47, ('\u{618A}', false)), // East Asian ideograph
    (0x227453, ('\u{7F6D}', false)), // East Asian ideograph
    (0x227454, ('\u{7F6B}', false)), // East Asian ideograph
    (0x227455, ('\u{7F67}', false)), // East Asian ideograph
    (0x227457, ('\u{7F68}', false)), // East Asian ideograph
    (0x235364, ('\u{9A32}', false)), // East Asian ideograph
    (0x21327E, ('\u{5165}', false)), // East Asian ideograph
    (0x22745E, ('\u{7F71}', false)), // East Asian ideograph
    (0x275365, ('\u{8111}', false)), // East Asian ideograph
    (0x227460, ('\u{7F73}', false)), // East Asian ideograph
    (0x227463, ('\u{7F76}', false)), // East Asian ideograph
    (0x6F5022, ('\u{BA39}', false)), // Korean hangul
    (0x217465, ('\u{5745}', false)), // East Asian ideograph
    (0x6F572F, ('\u{C7C1}', false)), // Korean hangul
    (0x217468, ('\u{574B}', false)), // East Asian ideograph
    (0x217469, ('\u{574C}', false)), // East Asian ideograph
    (0x21746A, ('\u{573F}', false)), // East Asian ideograph
    (0x215367, ('\u{818F}', false)), // East Asian ideograph
    (0x22746C, ('\u{7F7D}', false)), // East Asian ideograph
    (0x6F7739, ('\u{C61C}', false)), // Korean hangul
    (0x217470, ('\u{5768}', false)), // East Asian ideograph
    (0x6F5368, ('\u{C250}', false)), // Korean hangul
    (0x227472, ('\u{7F86}', false)), // East Asian ideograph
    (0x6F4F25, ('\u{B807}', false)), // Korean hangul
    (0x217475, ('\u{578A}', false)), // East Asian ideograph
    (0x225369, ('\u{71C7}', false)), // East Asian ideograph
    (0x345175, ('\u{7162}', false)), // East Asian ideograph
    (0x217479, ('\u{5774}', false)), // East Asian ideograph
    (0x21747A, ('\u{5767}', false)), // East Asian ideograph
    (0x22536A, ('\u{71B7}', false)), // East Asian ideograph
    (0x22747E, ('\u{7F96}', false)), // East Asian ideograph
    (0x6F536B, ('\u{C270}', false)), // Korean hangul
    (0x27536C, ('\u{80F6}', false)), // East Asian ideograph
    (0x274636, ('\u{6B93}', false)), // East Asian ideograph
    (0x6F5521, ('\u{C549}', false)), // Korean hangul
    (0x21536D, ('\u{819B}', false)), // East Asian ideograph
    (0x224348, ('\u{6AF5}', false)), // East Asian ideograph
    (0x4B5632, ('\u{7C54}', false)), // East Asian ideograph
    (0x27536E, ('\u{80A4}', false)), // East Asian ideograph
    (0x454F45, ('\u{9896}', false)), // East Asian ideograph
    (0x22536F, ('\u{71CF}', false)), // East Asian ideograph
    (0x4C5541, ('\u{4E2C}', false)), // East Asian ideograph
    (0x225370, ('\u{71D6}', false)), // East Asian ideograph
    (0x213031, ('\u{4E1E}', false)), // East Asian ideograph
    (0x215371, ('\u{81A9}', false)), // East Asian ideograph
    (0x274637, ('\u{6BA1}', false)), // East Asian ideograph
    (0x6F5522, ('\u{C54A}', false)), // Korean hangul
    (0x213F4A, ('\u{61C9}', false)), // East Asian ideograph
    (0x217463, ('\u{5749}', false)), // East Asian ideograph
    (0x6F5373, ('\u{C290}', false)), // Korean hangul
    (0x335172, ('\u{7D89}', false)), // East Asian ideograph
    (0x212A29, ('\u{E8D7}', false)), // EACC component character
    (0x6F4A2B, ('\u{ADEC}', false)), // Korean hangul
    (0x235374, ('\u{9A3B}', false)), // East Asian ideograph
    (0x4B496A, ('\u{932C}', false)), // East Asian ideograph
    (0x225375, ('\u{71C2}', false)), // East Asian ideograph
    (0x6F5376, ('\u{C29D}', false)), // Korean hangul
    (0x233E21, ('\u{9070}', false)), // East Asian ideograph
    (0x213F4B, ('\u{6190}', false)), // East Asian ideograph
    (0x225377, ('\u{71C5}', false)), // East Asian ideograph
    (0x4B385E, ('\u{5897}', false)), // East Asian ideograph
    (0x234749, ('\u{93FA}', false)), // East Asian ideograph
    (0x2D6275, ('\u{76B7}', false)), // East Asian ideograph
    (0x215378, ('\u{81BF}', false)), // East Asian ideograph
    (0x216431, ('\u{4E36}', false)), // East Asian ideograph
    (0x215379, ('\u{81BD}', false)), // East Asian ideograph
    (0x223E24, ('\u{6919}', false)), // East Asian ideograph
    (0x4B496B, ('\u{83F8}', false)), // East Asian ideograph
    (0x21537A, ('\u{81C9}', false)), // East Asian ideograph
    (0x213E25, ('\u{5FFD}', false)), // East Asian ideograph
    (0x21537B, ('\u{81BE}', false)), // East Asian ideograph
    (0x39304C, ('\u{4E81}', false)), // East Asian ideograph
    (0x213E26, ('\u{5FDD}', false)), // East Asian ideograph
    (0x23603F, ('\u{9F6E}', false)), // East Asian ideograph
    (0x27537C, ('\u{8110}', false)), // East Asian ideograph
    (0x33474A, ('\u{6D1F}', false)), // East Asian ideograph
    (0x21537D, ('\u{81CF}', false)), // East Asian ideograph
    (0x213E28, ('\u{5FFF}', false)), // East Asian ideograph
    (0x275746, ('\u{672E}', false)), // East Asian ideograph
    (0x21537E, ('\u{81D8}', false)), // East Asian ideograph
    (0x6F4E26, ('\u{B54B}', false)), // Korean hangul
    (0x227042, ('\u{7D06}', false)), // East Asian ideograph
    (0x294471, ('\u{951D}', false)), // East Asian ideograph
    (0x233E2A, ('\u{907B}', false)), // East Asian ideograph
    (0x6F5968, ('\u{CE61}', false)), // Korean hangul
    (0x6F5B38, ('\u{D1A0}', false)), // Korean hangul
    (0x213E2B, ('\u{602A}', false)), // East Asian ideograph
    (0x213E2C, ('\u{602F}', false)), // East Asian ideograph
    (0x6F585B, ('\u{CACC}', false)), // Korean hangul
    (0x213E2D, ('\u{6016}', false)), // East Asian ideograph
    (0x213E2F, ('\u{600F}', false)), // East Asian ideograph
    (0x6F5969, ('\u{CE68}', false)), // Korean hangul
    (0x227523, ('\u{7F97}', false)), // East Asian ideograph
    (0x227524, ('\u{7F95}', false)), // East Asian ideograph
    (0x51456D, ('\u{822E}', false)), // East Asian ideograph
    (0x217526, ('\u{5770}', false)), // East Asian ideograph
    (0x217528, ('\u{5771}', false)), // East Asian ideograph
    (0x21752A, ('\u{576E}', false)), // East Asian ideograph
    (0x22752C, ('\u{7FA2}', false)), // East Asian ideograph
    (0x21752D, ('\u{5776}', false)), // East Asian ideograph
    (0x21752E, ('\u{5789}', false)), // East Asian ideograph
    (0x217530, ('\u{577F}', false)), // East Asian ideograph
    (0x217531, ('\u{5775}', false)), // East Asian ideograph
    (0x217532, ('\u{577B}', false)), // East Asian ideograph
    (0x227533, ('\u{7FA7}', false)), // East Asian ideograph
    (0x217535, ('\u{5773}', false)), // East Asian ideograph
    (0x223241, ('\u{636D}', false)), // East Asian ideograph
    (0x217538, ('\u{579F}', false)), // East Asian ideograph
    (0x233E34, ('\u{9083}', false)), // East Asian ideograph
    (0x21753A, ('\u{5793}', false)), // East Asian ideograph
    (0x22753B, ('\u{7FB0}', false)), // East Asian ideograph
    (0x22753C, ('\u{7FAD}', false)), // East Asian ideograph
    (0x6F596A, ('\u{CE69}', false)), // Korean hangul
    (0x22753F, ('\u{7FB1}', false)), // East Asian ideograph
    (0x227540, ('\u{7FB4}', false)), // East Asian ideograph
    (0x6F5527, ('\u{C555}', false)), // Korean hangul
    (0x227542, ('\u{7FB5}', false)), // East Asian ideograph
    (0x217543, ('\u{579A}', false)), // East Asian ideograph
    (0x217545, ('\u{5794}', false)), // East Asian ideograph
    (0x217547, ('\u{57A4}', false)), // East Asian ideograph
    (0x217548, ('\u{5799}', false)), // East Asian ideograph
    (0x217549, ('\u{578C}', false)), // East Asian ideograph
    (0x22754A, ('\u{7FBC}', false)), // East Asian ideograph
    (0x21754B, ('\u{5797}', false)), // East Asian ideograph
    (0x22754C, ('\u{7FBE}', false)), // East Asian ideograph
    (0x275749, ('\u{536B}', false)), // East Asian ideograph
    (0x227551, ('\u{7FC3}', false)), // East Asian ideograph
    (0x217552, ('\u{579C}', false)), // East Asian ideograph
    (0x217554, ('\u{57A7}', false)), // East Asian ideograph
    (0x227557, ('\u{7FCA}', false)), // East Asian ideograph
    (0x217559, ('\u{3013}', false)), // East Asian ideograph (not found in unified han)
    (0x21755B, ('\u{5795}', false)), // East Asian ideograph
    (0x213E3A, ('\u{6068}', false)), // East Asian ideograph
    (0x21755F, ('\u{57B8}', false)), // East Asian ideograph
    (0x217560, ('\u{57C7}', false)), // East Asian ideograph
    (0x227567, ('\u{7FDB}', false)), // East Asian ideograph
    (0x227568, ('\u{7FE3}', false)), // East Asian ideograph
    (0x2D3E3C, ('\u{803B}', false)), // East Asian ideograph
    (0x21756A, ('\u{5809}', false)), // East Asian ideograph
    (0x22756C, ('\u{7FE6}', false)), // East Asian ideograph
    (0x22756F, ('\u{7FE5}', false)), // East Asian ideograph
    (0x22545C, ('\u{5911}', false)), // East Asian ideograph
    (0x217571, ('\u{57DB}', false)), // East Asian ideograph
    (0x227572, ('\u{7FEC}', false)), // East Asian ideograph
    (0x227573, ('\u{7FEB}', false)), // East Asian ideograph
    (0x273B60, ('\u{5C61}', false)), // East Asian ideograph
    (0x213E3E, ('\u{606D}', false)), // East Asian ideograph
    (0x227577, ('\u{7FEF}', false)), // East Asian ideograph
    (0x6F596C, ('\u{CE6D}', false)), // Korean hangul
    (0x22757A, ('\u{7FEE}', false)), // East Asian ideograph
    (0x233E3F, ('\u{9099}', false)), // East Asian ideograph
    (0x28632C, ('\u{7751}', false)), // East Asian ideograph
    (0x293066, ('\u{89C7}', false)), // East Asian ideograph
    (0x21757E, ('\u{57C6}', false)), // East Asian ideograph
    (0x233E40, ('\u{9097}', false)), // East Asian ideograph
    (0x4B563A, ('\u{82A6}', false)), // East Asian ideograph (variant of 27563A which maps to 82A6)
    (0x4B3421, ('\u{5263}', false)), // East Asian ideograph
    (0x275936, ('\u{8C23}', false)), // East Asian ideograph
    (0x213E41, ('\u{604D}', false)), // East Asian ideograph
    (0x6F4860, ('\u{AC01}', false)), // Korean hangul
    (0x4C695F, ('\u{7A63}', false)), // East Asian ideograph
    (0x213E42, ('\u{606B}', false)), // East Asian ideograph
    (0x395F49, ('\u{5F6B}', false)), // East Asian ideograph
    (0x23585C, ('\u{9C09}', false)), // East Asian ideograph
    (0x233643, ('\u{8C9F}', false)), // East Asian ideograph
    (0x213E43, ('\u{6069}', false)), // East Asian ideograph
    (0x6F5B39, ('\u{D1A1}', false)), // Korean hangul
    (0x223E44, ('\u{6911}', false)), // East Asian ideograph
    (0x6F552A, ('\u{C559}', false)), // Korean hangul
    (0x4B5A7E, ('\u{5C69}', false)), // East Asian ideograph
    (0x213E46, ('\u{606A}', false)), // East Asian ideograph
    (0x6F4861, ('\u{AC02}', false)), // Korean hangul
    (0x223E47, ('\u{68EF}', false)), // East Asian ideograph
    (0x22545E, ('\u{720A}', false)), // East Asian ideograph
    (0x6F4F4A, ('\u{B8FD}', false)), // Korean hangul
    (0x213E48, ('\u{6070}', false)), // East Asian ideograph
    (0x213E49, ('\u{6055}', false)), // East Asian ideograph
    (0x274640, ('\u{6BB4}', false)), // East Asian ideograph
    (0x234751, ('\u{9427}', false)), // East Asian ideograph
    (0x33433E, ('\u{95C7}', false)), // East Asian ideograph
    (0x213E4B, ('\u{60A6}', false)), // East Asian ideograph
    (0x4D4835, ('\u{954C}', false)), // East Asian ideograph
    (0x2D3C21, ('\u{57FC}', false)), // East Asian ideograph
    (0x275F2E, ('\u{9633}', false)), // East Asian ideograph
    (0x393E4C, ('\u{6142}', false)), // East Asian ideograph
    (0x4B4973, ('\u{7115}', false)), // East Asian ideograph
    (0x213E4D, ('\u{609F}', false)), // East Asian ideograph
    (0x27407E, ('\u{626A}', false)), // East Asian ideograph
    (0x6F596F, ('\u{CE74}', false)), // Korean hangul
    (0x6F552C, ('\u{C55F}', false)), // Korean hangul
    (0x697058, ('\u{9779}', false)), // East Asian ideograph
    (0x275E36, ('\u{9559}', false)), // East Asian ideograph
    (0x2D627E, ('\u{658B}', false)), // East Asian ideograph
    (0x213B48, ('\u{5C1A}', false)), // East Asian ideograph
    (0x2D5E3B, ('\u{92B9}', false)), // East Asian ideograph
    (0x6F4863, ('\u{AC07}', false)), // Korean hangul
    (0x6F4A2D, ('\u{ADF9}', false)), // Korean hangul
    (0x393078, ('\u{9AE3}', false)), // East Asian ideograph
    (0x227E51, ('\u{8423}', false)), // East Asian ideograph
    (0x225460, ('\u{7217}', false)), // East Asian ideograph
    (0x223247, ('\u{63D3}', false)), // East Asian ideograph
    (0x213E52, ('\u{60A3}', false)), // East Asian ideograph
    (0x6F5970, ('\u{CE75}', false)), // Korean hangul
    (0x6F5542, ('\u{C598}', false)), // Korean hangul
    (0x223E53, ('\u{6974}', false)), // East Asian ideograph
    (0x6F552D, ('\u{C560}', false)), // Korean hangul
    (0x213E54, ('\u{6094}', false)), // East Asian ideograph
    (0x2D4066, ('\u{63CE}', false)), // East Asian ideograph
    (0x216433, ('\u{4E3F}', false)), // East Asian ideograph
    (0x6F4864, ('\u{AC08}', false)), // Korean hangul
    (0x4B4975, ('\u{6427}', false)), // East Asian ideograph
    (0x275D7A, ('\u{9532}', false)), // East Asian ideograph
    (0x213E57, ('\u{60B4}', false)), // East Asian ideograph
    (0x395D23, ('\u{91BB}', false)), // East Asian ideograph
    (0x4B517E, ('\u{7E92}', false)), // East Asian ideograph
    (0x6F5971, ('\u{CE78}', false)), // Korean hangul
    (0x223E58, ('\u{6962}', false)), // East Asian ideograph
    (0x224B36, ('\u{6E39}', false)), // East Asian ideograph
    (0x6F5468, ('\u{C4D4}', false)), // Korean hangul
    (0x213E59, ('\u{60CB}', false)), // East Asian ideograph
    (0x4B563F, ('\u{6A98}', false)), // East Asian ideograph
    (0x2D4067, ('\u{62CF}', false)), // East Asian ideograph
    (0x6F4865, ('\u{AC09}', false)), // Korean hangul
    (0x6F7621, ('\u{3181}', false)), // Korean hangul
    (0x217622, ('\u{57C4}', false)), // East Asian ideograph
    (0x233E5B, ('\u{90B6}', false)), // East Asian ideograph
    (0x6F7624, ('\u{E8B0}', false)), // Korean hangul
    (0x6F7625, ('\u{E8B1}', false)), // Korean hangul
    (0x4B4556, ('\u{6A2A}', false)), // East Asian ideograph
    (0x217627, ('\u{70FE}', false)), // East Asian ideograph
    (0x227629, ('\u{7FFD}', false)), // East Asian ideograph
    (0x22762A, ('\u{7FFE}', false)), // East Asian ideograph
    (0x21762B, ('\u{5803}', false)), // East Asian ideograph
    (0x22762C, ('\u{7FFF}', false)), // East Asian ideograph
    (0x21762D, ('\u{57E6}', false)), // East Asian ideograph
    (0x22762E, ('\u{8004}', false)), // East Asian ideograph
    (0x233E5D, ('\u{90B0}', false)), // East Asian ideograph
    (0x29332C, ('\u{8BFC}', false)), // East Asian ideograph
    (0x217631, ('\u{57ED}', false)), // East Asian ideograph
    (0x227633, ('\u{800B}', false)), // East Asian ideograph
    (0x227634, ('\u{800E}', false)), // East Asian ideograph
    (0x227635, ('\u{8011}', false)), // East Asian ideograph
    (0x234755, ('\u{9409}', false)), // East Asian ideograph
    (0x227637, ('\u{8014}', false)), // East Asian ideograph
    (0x277638, ('\u{57AD}', false)), // East Asian ideograph
    (0x227639, ('\u{8016}', false)), // East Asian ideograph
    (0x223E5F, ('\u{6957}', false)), // East Asian ideograph
    (0x21763D, ('\u{57F4}', false)), // East Asian ideograph
    (0x22763E, ('\u{801D}', false)), // East Asian ideograph
    (0x294179, ('\u{9492}', false)), // East Asian ideograph
    (0x217640, ('\u{580D}', false)), // East Asian ideograph
    (0x223E60, ('\u{693F}', false)), // East Asian ideograph
    (0x6F7642, ('\u{E8B4}', false)), // Korean hangul
    (0x217643, ('\u{57EF}', false)), // East Asian ideograph
    (0x6F7644, ('\u{E8B6}', false)), // Korean hangul
    (0x6F7645, ('\u{E8B7}', false)), // Korean hangul
    (0x6F4F4B, ('\u{B904}', false)), // Korean hangul
    (0x273E61, ('\u{6076}', false)), // East Asian ideograph
    (0x217648, ('\u{5801}', false)), // East Asian ideograph
    (0x217649, ('\u{5812}', false)), // East Asian ideograph
    (0x6F764A, ('\u{E8BC}', false)), // Korean hangul
    (0x22764B, ('\u{8025}', false)), // East Asian ideograph
    (0x22764C, ('\u{8026}', false)), // East Asian ideograph
    (0x22764D, ('\u{802A}', false)), // East Asian ideograph
    (0x22764E, ('\u{8029}', false)), // East Asian ideograph
    (0x22764F, ('\u{8028}', false)), // East Asian ideograph
    (0x217650, ('\u{580C}', false)), // East Asian ideograph
    (0x217651, ('\u{5813}', false)), // East Asian ideograph
    (0x217652, ('\u{57F0}', false)), // East Asian ideograph
    (0x6F7653, ('\u{E8C5}', false)), // Korean hangul
    (0x6F7654, ('\u{E8C6}', false)), // Korean hangul
    (0x287655, ('\u{8027}', false)), // East Asian ideograph
    (0x217656, ('\u{580B}', false)), // East Asian ideograph
    (0x6F7657, ('\u{E8C9}', false)), // Korean hangul
    (0x217658, ('\u{57F3}', false)), // East Asian ideograph
    (0x217659, ('\u{5804}', false)), // East Asian ideograph
    (0x21765A, ('\u{57CF}', false)), // East Asian ideograph
    (0x22765B, ('\u{8030}', false)), // East Asian ideograph
    (0x22765D, ('\u{8031}', false)), // East Asian ideograph
    (0x21765F, ('\u{5847}', false)), // East Asian ideograph
    (0x227660, ('\u{8035}', false)), // East Asian ideograph
    (0x225021, ('\u{9E02}', false)), // East Asian ideograph
    (0x4D2F5D, ('\u{8941}', false)), // East Asian ideograph (variant of 232F5D which maps to 8941)
    (0x217667, ('\u{581B}', false)), // East Asian ideograph
    (0x227669, ('\u{8039}', false)), // East Asian ideograph
    (0x21766A, ('\u{5833}', false)), // East Asian ideograph
    (0x22766B, ('\u{8041}', false)), // East Asian ideograph
    (0x21766C, ('\u{581E}', false)), // East Asian ideograph
    (0x21766D, ('\u{583F}', false)), // East Asian ideograph
    (0x213F59, ('\u{61FE}', false)), // East Asian ideograph
    (0x227670, ('\u{8043}', false)), // East Asian ideograph
    (0x213E68, ('\u{60B8}', false)), // East Asian ideograph
    (0x217676, ('\u{5828}', false)), // East Asian ideograph
    (0x213E69, ('\u{60DA}', false)), // East Asian ideograph
    (0x217678, ('\u{582E}', false)), // East Asian ideograph
    (0x6F4868, ('\u{AC12}', false)), // Korean hangul
    (0x21767A, ('\u{581D}', false)), // East Asian ideograph
    (0x22767B, ('\u{8052}', false)), // East Asian ideograph
    (0x233E6A, ('\u{90BD}', false)), // East Asian ideograph
    (0x22767E, ('\u{8062}', false)), // East Asian ideograph
    (0x225B69, ('\u{74AA}', false)), // East Asian ideograph
    (0x213E6B, ('\u{610F}', false)), // East Asian ideograph
    (0x2D4D34, ('\u{76C7}', false)), // East Asian ideograph
    (0x4B4046, ('\u{629C}', false)), // East Asian ideograph
    (0x213E6C, ('\u{611C}', false)), // East Asian ideograph
    (0x29306F, ('\u{89CB}', false)), // East Asian ideograph
    (0x213F5A, ('\u{61FF}', false)), // East Asian ideograph
    (0x224832, ('\u{6CD2}', false)), // East Asian ideograph
    (0x234D62, ('\u{979C}', false)), // East Asian ideograph
    (0x6F773D, ('\u{C733}', false)), // Korean hangul
    (0x2D4844, ('\u{6FD5}', false)), // East Asian ideograph
    (0x2D4461, ('\u{6746}', false)), // East Asian ideograph
    (0x213E6E, ('\u{611F}', false)), // East Asian ideograph
    (0x6F4869, ('\u{AC13}', false)), // Korean hangul
    (0x39417C, ('\u{62E0}', false)), // East Asian ideograph
    (0x213E6F, ('\u{60F0}', false)), // East Asian ideograph
    (0x225466, ('\u{7215}', false)), // East Asian ideograph
    (0x22723C, ('\u{7DF2}', false)), // East Asian ideograph
    (0x223E70, ('\u{696A}', false)), // East Asian ideograph
    (0x6F5976, ('\u{CE89}', false)), // Korean hangul
    (0x213E71, ('\u{60FA}', false)), // East Asian ideograph
    (0x224B37, ('\u{6E71}', false)), // East Asian ideograph
    (0x213E72, ('\u{611A}', false)), // East Asian ideograph
    (0x234759, ('\u{9404}', false)), // East Asian ideograph
    (0x333D48, ('\u{5F3A}', false)), // East Asian ideograph
    (0x213E73, ('\u{6115}', false)), // East Asian ideograph
    (0x6F486A, ('\u{AC14}', false)), // Korean hangul
    (0x212A3D, ('\u{E8EA}', false)), // EACC component character
    (0x235866, ('\u{9C1C}', false)), // East Asian ideograph
    (0x233E75, ('\u{90C7}', false)), // East Asian ideograph
    (0x456064, ('\u{9963}', false)), // East Asian ideograph
    (0x6F5977, ('\u{CE90}', false)), // Korean hangul
    (0x6F5B3B, ('\u{D1A8}', false)), // Korean hangul
    (0x2D3B7B, ('\u{5D08}', false)), // East Asian ideograph
    (0x222921, ('\u{5EF1}', false)), // East Asian ideograph
    (0x213F5C, ('\u{6200}', false)), // East Asian ideograph
    (0x697060, ('\u{9790}', false)), // East Asian ideograph
    (0x4B506C, ('\u{7CAB}', false)), // East Asian ideograph
    (0x215D32, ('\u{91AC}', false)), // East Asian ideograph
    (0x225347, ('\u{71B2}', false)), // East Asian ideograph
    (0x213E78, ('\u{610E}', false)), // East Asian ideograph
    (0x2D5E43, ('\u{92F3}', false)), // East Asian ideograph
    (0x6F486B, ('\u{AC15}', false)), // Korean hangul
    (0x213E79, ('\u{6100}', false)), // East Asian ideograph
    (0x23364E, ('\u{8CB0}', false)), // East Asian ideograph
    (0x213E7A, ('\u{6101}', false)), // East Asian ideograph
    (0x4D2925, ('\u{8770}', false)), // East Asian ideograph
    (0x6F5344, ('\u{C19C}', false)), // Korean hangul
    (0x6F5978, ('\u{CE91}', false)), // Korean hangul
    (0x213E7B, ('\u{60F6}', false)), // East Asian ideograph
    (0x6F5535, ('\u{C575}', false)), // Korean hangul
    (0x4B3870, ('\u{58CC}', false)), // East Asian ideograph
    (0x232927, ('\u{867C}', false)), // East Asian ideograph
    (0x223E7D, ('\u{6980}', false)), // East Asian ideograph
    (0x28736D, ('\u{624D}', false)), // East Asian ideograph
    (0x275E62, ('\u{9615}', false)), // East Asian ideograph
    (0x223E7E, ('\u{6933}', false)), // East Asian ideograph
    (0x225469, ('\u{7213}', false)), // East Asian ideograph
    (0x6F4935, ('\u{AC94}', false)), // Korean hangul
    (0x6F4E72, ('\u{B7A8}', false)), // Korean hangul
    (0x2D4D38, ('\u{76D7}', false)), // East Asian ideograph
    (0x6F5536, ('\u{C57C}', false)), // Korean hangul
    (0x4B3871, ('\u{57BB}', false)), // East Asian ideograph
    (0x4B5647, ('\u{51E6}', false)), // East Asian ideograph
    (0x6F486D, ('\u{AC17}', false)), // Korean hangul
    (0x2D5434, ('\u{64E7}', false)), // East Asian ideograph
    (0x234E5C, ('\u{97DE}', false)), // East Asian ideograph
    (0x225B6A, ('\u{7490}', false)), // East Asian ideograph
    (0x23292F, ('\u{86A8}', false)), // East Asian ideograph
    (0x6F597A, ('\u{CE98}', false)), // Korean hangul
    (0x4B5221, ('\u{7D9A}', false)), // East Asian ideograph
    (0x217721, ('\u{5848}', false)), // East Asian ideograph
    (0x6F7722, ('\u{AD7B}', false)), // Korean hangul
    (0x217723, ('\u{5818}', false)), // East Asian ideograph
    (0x6F7724, ('\u{AD89}', false)), // Korean hangul
    (0x6F7725, ('\u{AD9D}', false)), // Korean hangul
    (0x217726, ('\u{57F5}', false)), // East Asian ideograph
    (0x4B5D36, ('\u{91C6}', false)), // East Asian ideograph
    (0x227728, ('\u{8063}', false)), // East Asian ideograph
    (0x21315B, ('\u{4FBF}', false)), // East Asian ideograph
    (0x6F772A, ('\u{AE0F}', false)), // Korean hangul
    (0x21772B, ('\u{5820}', false)), // East Asian ideograph
    (0x6F772C, ('\u{AE14}', false)), // Korean hangul
    (0x6F486E, ('\u{AC19}', false)), // Korean hangul
    (0x6F772E, ('\u{AEED}', false)), // Korean hangul
    (0x6F772F, ('\u{AF09}', false)), // Korean hangul
    (0x217730, ('\u{584E}', false)), // East Asian ideograph
    (0x6F7731, ('\u{AFBF}', false)), // Korean hangul
    (0x227732, ('\u{806C}', false)), // East Asian ideograph
    (0x217733, ('\u{585D}', false)), // East Asian ideograph
    (0x6F4926, ('\u{AC78}', false)), // Korean hangul
    (0x217735, ('\u{5859}', false)), // East Asian ideograph
    (0x6F7736, ('\u{C552}', false)), // Korean hangul
    (0x217737, ('\u{584B}', false)), // East Asian ideograph
    (0x6F7738, ('\u{C5B1}', false)), // Korean hangul
    (0x227739, ('\u{8075}', false)), // East Asian ideograph
    (0x6F773A, ('\u{C61D}', false)), // Korean hangul
    (0x2D3876, ('\u{58F7}', false)), // East Asian ideograph
    (0x6F773C, ('\u{E8CB}', false)), // Korean hangul
    (0x21773D, ('\u{5865}', false)), // East Asian ideograph
    (0x22773E, ('\u{807B}', false)), // East Asian ideograph
    (0x22773F, ('\u{8079}', false)), // East Asian ideograph
    (0x217740, ('\u{586C}', false)), // East Asian ideograph
    (0x213F60, ('\u{620D}', false)), // East Asian ideograph
    (0x217742, ('\u{5852}', false)), // East Asian ideograph
    (0x33475E, ('\u{6FB9}', false)), // East Asian ideograph
    (0x217745, ('\u{5864}', false)), // East Asian ideograph
    (0x227747, ('\u{808A}', false)), // East Asian ideograph
    (0x217748, ('\u{584F}', false)), // East Asian ideograph
    (0x227749, ('\u{808E}', false)), // East Asian ideograph
    (0x213533, ('\u{53FC}', false)), // East Asian ideograph
    (0x6F486F, ('\u{AC1A}', false)), // Korean hangul
    (0x21774D, ('\u{584D}', false)), // East Asian ideograph
    (0x22774E, ('\u{809F}', false)), // East Asian ideograph
    (0x6F5730, ('\u{C7C8}', false)), // Korean hangul
    (0x225029, ('\u{7054}', false)), // East Asian ideograph
    (0x232939, ('\u{8698}', false)), // East Asian ideograph
    (0x217758, ('\u{5892}', false)), // East Asian ideograph
    (0x6F597C, ('\u{CEA1}', false)), // Korean hangul
    (0x21775A, ('\u{588E}', false)), // East Asian ideograph
    (0x22775C, ('\u{670A}', false)), // East Asian ideograph
    (0x70775D, ('\u{9B0F}', false)), // East Asian ideograph
    (0x282632, ('\u{5D58}', false)), // East Asian ideograph
    (0x21775F, ('\u{5840}', false)), // East Asian ideograph
    (0x227760, ('\u{80A7}', false)), // East Asian ideograph
    (0x227761, ('\u{80B0}', false)), // East Asian ideograph
    (0x33475F, ('\u{60BD}', false)), // East Asian ideograph
    (0x21576C, ('\u{88FD}', false)), // East Asian ideograph
    (0x217765, ('\u{5890}', false)), // East Asian ideograph
    (0x217768, ('\u{5898}', false)), // East Asian ideograph
    (0x227769, ('\u{80B5}', false)), // East Asian ideograph
    (0x22776A, ('\u{80A6}', false)), // East Asian ideograph
    (0x21776B, ('\u{587D}', false)), // East Asian ideograph
    (0x6F5C36, ('\u{D3B8}', false)), // Korean hangul
    (0x21776F, ('\u{587F}', false)), // East Asian ideograph
    (0x217770, ('\u{5881}', false)), // East Asian ideograph
    (0x707771, ('\u{9EE2}', false)), // East Asian ideograph (Version J extension)
    (0x227773, ('\u{80E0}', false)), // East Asian ideograph
    (0x21693E, ('\u{5135}', false)), // East Asian ideograph
    (0x235B26, ('\u{9D5A}', false)), // East Asian ideograph
    (0x6F597D, ('\u{CEA3}', false)), // Korean hangul
    (0x213D62, ('\u{5F8B}', false)), // East Asian ideograph
    (0x22777B, ('\u{80DF}', false)), // East Asian ideograph
    (0x22777D, ('\u{80C2}', false)), // East Asian ideograph
    (0x21777E, ('\u{58A1}', false)), // East Asian ideograph
    (0x226940, ('\u{7A5C}', false)), // East Asian ideograph
    (0x4B7421, ('\u{F9A9}', false)), // East Asian ideograph
    (0x4C433F, ('\u{6A65}', false)), // East Asian ideograph
    (0x6F4D21, ('\u{B304}', false)), // Korean hangul
    (0x45606B, ('\u{98F0}', false)), // East Asian ideograph
    (0x27482D, ('\u{6C64}', false)), // East Asian ideograph
    (0x293B6B, ('\u{8F8B}', false)), // East Asian ideograph
    (0x276944, ('\u{50A9}', false)), // East Asian ideograph
    (0x213F63, ('\u{6212}', false)), // East Asian ideograph
    (0x2D5E4A, ('\u{945A}', false)), // East Asian ideograph
    (0x6F4A30, ('\u{AE00}', false)), // Korean hangul
    (0x275E57, ('\u{95F8}', false)), // East Asian ideograph
    (0x6F5321, ('\u{C11C}', false)), // Korean hangul
    (0x276948, ('\u{50A5}', false)), // East Asian ideograph
    (0x6F553C, ('\u{C58D}', false)), // Korean hangul
    (0x213F64, ('\u{6211}', false)), // East Asian ideograph
    (0x215D3A, ('\u{91CD}', false)), // East Asian ideograph
    (0x234762, ('\u{9423}', false)), // East Asian ideograph
    (0x23294B, ('\u{86BF}', false)), // East Asian ideograph
    (0x6F4956, ('\u{ACF6}', false)), // Korean hangul
    (0x6F4B41, ('\u{B057}', false)), // Korean hangul
    (0x292C5D, ('\u{867F}', false)), // East Asian ideograph
    (0x4B3435, ('\u{52B4}', false)), // East Asian ideograph
    (0x6F4E27, ('\u{B54C}', false)), // Korean hangul
    (0x222951, ('\u{5F33}', false)), // East Asian ideograph
    (0x223258, ('\u{63E6}', false)), // East Asian ideograph
    (0x235870, ('\u{9C2E}', false)), // East Asian ideograph
    (0x227247, ('\u{7DF1}', false)), // East Asian ideograph
    (0x6F5B3D, ('\u{D1B1}', false)), // Korean hangul
    (0x6F553E, ('\u{C590}', false)), // Korean hangul
    (0x213F66, ('\u{6215}', false)), // East Asian ideograph
    (0x2D3539, ('\u{52FE}', false)), // East Asian ideograph
    (0x22613B, ('\u{76E6}', false)), // East Asian ideograph
    (0x4B3436, ('\u{52F2}', false)), // East Asian ideograph
    (0x6F4E6C, ('\u{B791}', false)), // Korean hangul
    (0x23517A, ('\u{9958}', false)), // East Asian ideograph
    (0x214C30, ('\u{755A}', false)), // East Asian ideograph
    (0x226957, ('\u{7A6E}', false)), // East Asian ideograph
    (0x222958, ('\u{5F38}', false)), // East Asian ideograph
    (0x2D424F, ('\u{555F}', false)), // East Asian ideograph
    (0x215D3D, ('\u{91D0}', false)), // East Asian ideograph
    (0x22613C, ('\u{76E9}', false)), // East Asian ideograph
    (0x334342, ('\u{7156}', false)), // East Asian ideograph
    (0x513D67, ('\u{8FF3}', false)), // East Asian ideograph
    (0x217824, ('\u{58B1}', false)), // East Asian ideograph
    (0x227827, ('\u{80D9}', false)), // East Asian ideograph
    (0x4B4544, ('\u{69D8}', false)), // East Asian ideograph
    (0x4C695C, ('\u{7A06}', false)), // East Asian ideograph
    (0x22782A, ('\u{80DD}', false)), // East Asian ideograph
    (0x21782B, ('\u{58AD}', false)), // East Asian ideograph
    (0x22782D, ('\u{80CF}', false)), // East Asian ideograph
    (0x21782E, ('\u{58A0}', false)), // East Asian ideograph
    (0x22782F, ('\u{80CD}', false)), // East Asian ideograph
    (0x227830, ('\u{80D7}', false)), // East Asian ideograph
    (0x213F68, ('\u{621A}', false)), // East Asian ideograph
    (0x217832, ('\u{58A6}', false)), // East Asian ideograph
    (0x227833, ('\u{80F2}', false)), // East Asian ideograph
    (0x227834, ('\u{80FA}', false)), // East Asian ideograph
    (0x227838, ('\u{80FE}', false)), // East Asian ideograph
    (0x21783A, ('\u{58C8}', false)), // East Asian ideograph
    (0x2D3C36, ('\u{5DE3}', false)), // East Asian ideograph
    (0x22783C, ('\u{8103}', false)), // East Asian ideograph
    (0x275762, ('\u{8865}', false)), // East Asian ideograph
    (0x227840, ('\u{80F9}', false)), // East Asian ideograph
    (0x217841, ('\u{58BC}', false)), // East Asian ideograph
    (0x227842, ('\u{80D4}', false)), // East Asian ideograph
    (0x33365A, ('\u{5405}', false)), // East Asian ideograph
    (0x4B4545, ('\u{6982}', false)), // East Asian ideograph
    (0x217849, ('\u{58BF}', false)), // East Asian ideograph
    (0x22784B, ('\u{8118}', false)), // East Asian ideograph
    (0x21784C, ('\u{58BA}', false)), // East Asian ideograph
    (0x222962, ('\u{5F4D}', false)), // East Asian ideograph
    (0x6F5541, ('\u{C597}', false)), // Korean hangul
    (0x227850, ('\u{8130}', false)), // East Asian ideograph
    (0x232963, ('\u{86B4}', false)), // East Asian ideograph
    (0x227854, ('\u{8124}', false)), // East Asian ideograph
    (0x227855, ('\u{811B}', false)), // East Asian ideograph
    (0x217856, ('\u{58CE}', false)), // East Asian ideograph
    (0x2D5E50, ('\u{9587}', false)), // East Asian ideograph
    (0x6F4878, ('\u{AC2F}', false)), // Korean hangul
    (0x21785A, ('\u{58E0}', false)), // East Asian ideograph
    (0x21785E, ('\u{58DA}', false)), // East Asian ideograph
    (0x227860, ('\u{812A}', false)), // East Asian ideograph
    (0x227861, ('\u{811E}', false)), // East Asian ideograph
    (0x294362, ('\u{94EA}', false)), // East Asian ideograph
    (0x227864, ('\u{8121}', false)), // East Asian ideograph
    (0x212321, ('\u{3000}', false)), // Ideographic space per ANSI Z39.64
    (0x227866, ('\u{8117}', false)), // East Asian ideograph
    (0x227869, ('\u{813A}', false)), // East Asian ideograph
    (0x22786A, ('\u{815A}', false)), // East Asian ideograph
    (0x21786C, ('\u{58FC}', false)), // East Asian ideograph
    (0x22786D, ('\u{8148}', false)), // East Asian ideograph
    (0x28786E, ('\u{80E8}', false)), // East Asian ideograph
    (0x4B387D, ('\u{591B}', false)), // East Asian ideograph
    (0x217870, ('\u{5902}', false)), // East Asian ideograph
    (0x213B27, ('\u{5BCC}', false)), // East Asian ideograph
    (0x217873, ('\u{5906}', false)), // East Asian ideograph
    (0x217874, ('\u{6535}', false)), // East Asian ideograph
    (0x227877, ('\u{814C}', false)), // East Asian ideograph
    (0x21787A, ('\u{5910}', false)), // East Asian ideograph
    (0x21787C, ('\u{8641}', false)), // East Asian ideograph
    (0x22787D, ('\u{8141}', false)), // East Asian ideograph
    (0x22325D, ('\u{63F6}', false)), // East Asian ideograph
    (0x214C34, ('\u{7570}', false)), // East Asian ideograph
    (0x343A5B, ('\u{572C}', false)), // East Asian ideograph
    (0x2E735D, ('\u{7D56}', false)), // East Asian ideograph
    (0x276871, ('\u{4FAA}', false)), // East Asian ideograph
    (0x6F5543, ('\u{C59C}', false)), // Korean hangul
    (0x696464, ('\u{7C90}', false)), // East Asian ideograph
    (0x213B28, ('\u{5BD2}', false)), // East Asian ideograph
    (0x234326, ('\u{924C}', false)), // East Asian ideograph
    (0x275765, ('\u{88C5}', false)), // East Asian ideograph
    (0x23296F, ('\u{86E9}', false)), // East Asian ideograph
    (0x22325E, ('\u{63F2}', false)), // East Asian ideograph
    (0x214C35, ('\u{7565}', false)), // East Asian ideograph
    (0x6F557C, ('\u{C63A}', false)), // Korean hangul
    (0x235433, ('\u{9A64}', false)), // East Asian ideograph
    (0x222971, ('\u{5F61}', false)), // East Asian ideograph
    (0x6F5544, ('\u{C5B4}', false)), // Korean hangul
    (0x2D5E24, ('\u{7145}', false)), // East Asian ideograph
    (0x23476A, ('\u{9407}', false)), // East Asian ideograph
    (0x4B403D, ('\u{62DD}', false)), // East Asian ideograph
    (0x6F487B, ('\u{AC38}', false)), // Korean hangul
    (0x2D4D65, ('\u{53E1}', false)), // East Asian ideograph
    (0x232974, ('\u{86D5}', false)), // East Asian ideograph
    (0x22325F, ('\u{63F8}', false)), // East Asian ideograph
    (0x23365E, ('\u{8CD8}', false)), // East Asian ideograph
    (0x235434, ('\u{9A66}', false)), // East Asian ideograph
    (0x275421, ('\u{80EA}', false)), // East Asian ideograph
    (0x275E37, ('\u{9535}', false)), // East Asian ideograph
    (0x215422, ('\u{81DF}', false)), // East Asian ideograph
    (0x6F5862, ('\u{CB10}', false)), // Korean hangul
    (0x6F487C, ('\u{AC39}', false)), // Korean hangul
    (0x6F4A32, ('\u{AE08}', false)), // Korean hangul
    (0x6F5423, ('\u{C2DC}', false)), // Korean hangul
    (0x27623F, ('\u{9E43}', false)), // East Asian ideograph
    (0x275E59, ('\u{95FA}', false)), // East Asian ideograph
    (0x215424, ('\u{81E5}', false)), // East Asian ideograph
    (0x23365F, ('\u{8CD5}', false)), // East Asian ideograph
    (0x456076, ('\u{9980}', false)), // East Asian ideograph
    (0x215425, ('\u{81E8}', false)), // East Asian ideograph
    (0x6F5B48, ('\u{D23D}', false)), // Korean hangul
    (0x275142, ('\u{7EFD}', false)), // East Asian ideograph
    (0x225426, ('\u{71D4}', false)), // East Asian ideograph
    (0x235427, ('\u{9A4A}', false)), // East Asian ideograph
    (0x6F487D, ('\u{AC40}', false)), // Korean hangul
    (0x4B5428, ('\u{81ED}', false)), // East Asian ideograph (variant of 215428 which maps to 81ED)
    (0x235879, ('\u{9C24}', false)), // East Asian ideograph
    (0x6F5C6A, ('\u{D56B}', false)), // Korean hangul
    (0x23542A, ('\u{9A58}', false)), // East Asian ideograph
    (0x6F546D, ('\u{C4F8}', false)), // Korean hangul
    (0x6F5547, ('\u{C5B8}', false)), // Korean hangul
    (0x27542B, ('\u{53F0}', false)), // East Asian ideograph
    (0x333D4C, ('\u{7030}', false)), // East Asian ideograph
    (0x23542C, ('\u{9A56}', false)), // East Asian ideograph
    (0x6F542D, ('\u{C2F6}', false)), // Korean hangul
    (0x293D4E, ('\u{8FF8}', false)), // East Asian ideograph
    (0x28352A, ('\u{6448}', false)), // East Asian ideograph
    (0x47577A, ('\u{9BD6}', false)), // East Asian ideograph (variant of 23577A which maps to 9BD6)
    (0x6F5B3F, ('\u{D1B5}', false)), // Korean hangul
    (0x6F5430, ('\u{C2F8}', false)), // Korean hangul
    (0x227925, ('\u{814D}', false)), // East Asian ideograph
    (0x6F5431, ('\u{C2F9}', false)), // Korean hangul
    (0x217928, ('\u{592C}', false)), // East Asian ideograph
    (0x21792B, ('\u{592F}', false)), // East Asian ideograph
    (0x275432, ('\u{4E0E}', false)), // East Asian ideograph
    (0x22792E, ('\u{6720}', false)), // East Asian ideograph
    (0x21507D, ('\u{7D14}', false)), // East Asian ideograph
    (0x217930, ('\u{593C}', false)), // East Asian ideograph
    (0x395F68, ('\u{8987}', false)), // East Asian ideograph
    (0x227932, ('\u{8160}', false)), // East Asian ideograph
    (0x215433, ('\u{8208}', false)), // East Asian ideograph
    (0x225039, ('\u{7074}', false)), // East Asian ideograph
    (0x217938, ('\u{594D}', false)), // East Asian ideograph
    (0x215434, ('\u{8209}', false)), // East Asian ideograph
    (0x22793B, ('\u{8169}', false)), // East Asian ideograph
    (0x22793C, ('\u{817C}', false)), // East Asian ideograph
    (0x275435, ('\u{65E7}', false)), // East Asian ideograph
    (0x294E5C, ('\u{97EB}', false)), // East Asian ideograph
    (0x227941, ('\u{8161}', false)), // East Asian ideograph
    (0x6F5A65, ('\u{D081}', false)), // Korean hangul
    (0x217943, ('\u{5953}', false)), // East Asian ideograph
    (0x225436, ('\u{71E8}', false)), // East Asian ideograph
    (0x227946, ('\u{8176}', false)), // East Asian ideograph
    (0x227947, ('\u{8174}', false)), // East Asian ideograph
    (0x227948, ('\u{8167}', false)), // East Asian ideograph
    (0x334633, ('\u{6B8B}', false)), // East Asian ideograph (variant of 274633 which maps to 6B8B)
    (0x22794B, ('\u{816F}', false)), // East Asian ideograph
    (0x22794D, ('\u{8182}', false)), // East Asian ideograph
    (0x4C794E, ('\u{80B7}', false)), // East Asian ideograph
    (0x21794F, ('\u{5961}', false)), // East Asian ideograph
    (0x227951, ('\u{818B}', false)), // East Asian ideograph
    (0x227952, ('\u{8186}', false)), // East Asian ideograph
    (0x217954, ('\u{596C}', false)), // East Asian ideograph
    (0x217955, ('\u{596D}', false)), // East Asian ideograph
    (0x274830, ('\u{6D4B}', false)), // East Asian ideograph
    (0x4B6324, ('\u{9F62}', false)), // East Asian ideograph
    (0x227959, ('\u{8183}', false)), // East Asian ideograph
    (0x21543A, ('\u{8214}', false)), // East Asian ideograph
    (0x334770, ('\u{5A6C}', false)), // East Asian ideograph
    (0x69543B, ('\u{57B0}', false)), // East Asian ideograph
    (0x217965, ('\u{597C}', false)), // East Asian ideograph
    (0x6F4A33, ('\u{AE09}', false)), // Korean hangul
    (0x217969, ('\u{59A7}', false)), // East Asian ideograph
    (0x22796A, ('\u{819F}', false)), // East Asian ideograph
    (0x22796B, ('\u{81A3}', false)), // East Asian ideograph
    (0x287941, ('\u{8136}', false)), // East Asian ideograph
    (0x21796F, ('\u{599A}', false)), // East Asian ideograph
    (0x227970, ('\u{8198}', false)), // East Asian ideograph
    (0x22503B, ('\u{707A}', false)), // East Asian ideograph
    (0x335E3D, ('\u{9244}', false)), // East Asian ideograph
    (0x227975, ('\u{8195}', false)), // East Asian ideograph
    (0x227977, ('\u{8197}', false)), // East Asian ideograph
    (0x22543F, ('\u{71E1}', false)), // East Asian ideograph
    (0x22797C, ('\u{81AA}', false)), // East Asian ideograph
    (0x224372, ('\u{6B1E}', false)), // East Asian ideograph
    (0x22797E, ('\u{6725}', false)), // East Asian ideograph
    (0x213B30, ('\u{5BDE}', false)), // East Asian ideograph
    (0x2D5440, ('\u{6841}', false)), // East Asian ideograph
    (0x6F4862, ('\u{AC04}', false)), // Korean hangul
    (0x215441, ('\u{822B}', false)), // East Asian ideograph
    (0x275068, ('\u{7CAA}', false)), // East Asian ideograph
    (0x695442, ('\u{57D6}', false)), // East Asian ideograph
    (0x227255, ('\u{7E12}', false)), // East Asian ideograph
    (0x235443, ('\u{9AB1}', false)), // East Asian ideograph
    (0x294E79, ('\u{9878}', false)), // East Asian ideograph
    (0x6F5444, ('\u{C345}', false)), // Korean hangul
    (0x213B31, ('\u{5BE6}', false)), // East Asian ideograph
    (0x235445, ('\u{9AB3}', false)), // East Asian ideograph
    (0x33432F, ('\u{664B}', false)), // East Asian ideograph
    (0x225651, ('\u{72C6}', false)), // East Asian ideograph
    (0x2D5446, ('\u{8229}', false)), // East Asian ideograph
    (0x216E57, ('\u{53FB}', false)), // East Asian ideograph
    (0x214C3E, ('\u{758F}', false)), // East Asian ideograph
    (0x276329, ('\u{9F8C}', false)), // East Asian ideograph
    (0x6F554D, ('\u{C5C4}', false)), // Korean hangul
    (0x235449, ('\u{9AB6}', false)), // East Asian ideograph
    (0x227427, ('\u{7F46}', false)), // East Asian ideograph
    (0x224A62, ('\u{6E4B}', false)), // East Asian ideograph
    (0x27544A, ('\u{8231}', false)), // East Asian ideograph
    (0x294161, ('\u{9487}', false)), // East Asian ideograph
    (0x27544B, ('\u{8230}', false)), // East Asian ideograph
    (0x283955, ('\u{6619}', false)), // East Asian ideograph
    (0x23544C, ('\u{9ABB}', false)), // East Asian ideograph
    (0x233667, ('\u{8CE8}', false)), // East Asian ideograph
    (0x4D2F7A, ('\u{891D}', false)), // East Asian ideograph
    (0x6F5345, ('\u{C19D}', false)), // Korean hangul
    (0x6F544D, ('\u{C37D}', false)), // Korean hangul
    (0x235871, ('\u{9C28}', false)), // East Asian ideograph
    (0x6F554E, ('\u{C5C5}', false)), // Korean hangul
    (0x27544E, ('\u{8270}', false)), // East Asian ideograph
    (0x234774, ('\u{943F}', false)), // East Asian ideograph
    (0x22544F, ('\u{71FC}', false)), // East Asian ideograph
    (0x235450, ('\u{9ABA}', false)), // East Asian ideograph
    (0x695451, ('\u{58B9}', false)), // East Asian ideograph
    (0x233668, ('\u{8CE9}', false)), // East Asian ideograph
    (0x4B4553, ('\u{6955}', false)), // East Asian ideograph
    (0x6F4E77, ('\u{B7B4}', false)), // Korean hangul
    (0x274831, ('\u{6DA1}', false)), // East Asian ideograph
    (0x233225, ('\u{8A22}', false)), // East Asian ideograph
    (0x6F554F, ('\u{C5C6}', false)), // Korean hangul
    (0x6F5453, ('\u{C3DC}', false)), // Korean hangul
    (0x235454, ('\u{9ABD}', false)), // East Asian ideograph
    (0x2E6C26, ('\u{7BE0}', false)), // East Asian ideograph
    (0x6F4A34, ('\u{AE0B}', false)), // Korean hangul
    (0x6F5455, ('\u{C3E0}', false)), // Korean hangul
    (0x225456, ('\u{71F9}', false)), // East Asian ideograph
    (0x225040, ('\u{7093}', false)), // East Asian ideograph
    (0x23543F, ('\u{9AAD}', false)), // East Asian ideograph
    (0x235457, ('\u{9AC1}', false)), // East Asian ideograph
    (0x6F5550, ('\u{C5C7}', false)), // Korean hangul
    (0x275458, ('\u{5DF4}', false)), // East Asian ideograph (duplicate simplified)
    (0x274222, ('\u{62C5}', false)), // East Asian ideograph
    (0x4B3B22, ('\u{51A6}', false)), // East Asian ideograph
    (0x235459, ('\u{9AC0}', false)), // East Asian ideograph
    (0x22572C, ('\u{72FB}', false)), // East Asian ideograph
    (0x23545A, ('\u{9AC2}', false)), // East Asian ideograph
    (0x217A21, ('\u{5990}', false)), // East Asian ideograph
    (0x22545B, ('\u{720E}', false)), // East Asian ideograph
    (0x217A24, ('\u{59C5}', false)), // East Asian ideograph
    (0x217A25, ('\u{59B5}', false)), // East Asian ideograph
    (0x217A28, ('\u{59CF}', false)), // East Asian ideograph
    (0x21545C, ('\u{82BB}', false)), // East Asian ideograph
    (0x217A2A, ('\u{59BA}', false)), // East Asian ideograph
    (0x3F377B, ('\u{784E}', false)), // East Asian ideograph (Version J extension)
    (0x217A2C, ('\u{59B8}', false)), // East Asian ideograph
    (0x6F546F, ('\u{C501}', false)), // Korean hangul
    (0x227A2E, ('\u{81B0}', false)), // East Asian ideograph
    (0x227A2F, ('\u{81B4}', false)), // East Asian ideograph
    (0x215D4F, ('\u{9237}', false)), // East Asian ideograph
    (0x227A33, ('\u{81B7}', false)), // East Asian ideograph
    (0x217A35, ('\u{59B2}', false)), // East Asian ideograph
    (0x227A37, ('\u{81BB}', false)), // East Asian ideograph
    (0x227A38, ('\u{81C1}', false)), // East Asian ideograph
    (0x227A39, ('\u{81CC}', false)), // East Asian ideograph
    (0x217A3A, ('\u{59B7}', false)), // East Asian ideograph
    (0x227A3B, ('\u{81C4}', false)), // East Asian ideograph
    (0x217A3E, ('\u{59C1}', false)), // East Asian ideograph
    (0x227A40, ('\u{81D1}', false)), // East Asian ideograph
    (0x227A41, ('\u{81CE}', false)), // East Asian ideograph
    (0x217A43, ('\u{59F9}', false)), // East Asian ideograph
    (0x217A44, ('\u{59F8}', false)), // East Asian ideograph
    (0x212A43, ('\u{E8F0}', false)), // EACC component character
    (0x225461, ('\u{7207}', false)), // East Asian ideograph
    (0x227A4B, ('\u{81DB}', false)), // East Asian ideograph
    (0x6F5552, ('\u{C5C9}', false)), // Korean hangul
    (0x6F5462, ('\u{C468}', false)), // Korean hangul
    (0x227A4F, ('\u{81DD}', false)), // East Asian ideograph
    (0x217A50, ('\u{59F1}', false)), // East Asian ideograph
    (0x217A51, ('\u{5A00}', false)), // East Asian ideograph
    (0x217A52, ('\u{59DE}', false)), // East Asian ideograph
    (0x227A53, ('\u{81DE}', false)), // East Asian ideograph
    (0x227A56, ('\u{81E0}', false)), // East Asian ideograph
    (0x227A57, ('\u{81E2}', false)), // East Asian ideograph
    (0x6F5464, ('\u{C474}', false)), // Korean hangul
    (0x227A5B, ('\u{81E7}', false)), // East Asian ideograph
    (0x217A5D, ('\u{59F6}', false)), // East Asian ideograph
    (0x217A5E, ('\u{59DD}', false)), // East Asian ideograph
    (0x217A5F, ('\u{59FA}', false)), // East Asian ideograph
    (0x227A60, ('\u{81EF}', false)), // East Asian ideograph
    (0x217A61, ('\u{59E4}', false)), // East Asian ideograph
    (0x227A65, ('\u{81F2}', false)), // East Asian ideograph
    (0x227A68, ('\u{81F6}', false)), // East Asian ideograph
    (0x6F5553, ('\u{C5CA}', false)), // Korean hangul
    (0x6F5467, ('\u{C494}', false)), // Korean hangul
    (0x274225, ('\u{6324}', false)), // East Asian ideograph
    (0x217A6E, ('\u{5A2A}', false)), // East Asian ideograph
    (0x213B38, ('\u{5BF5}', false)), // East Asian ideograph
    (0x227A70, ('\u{8201}', false)), // East Asian ideograph
    (0x2D5468, ('\u{6959}', false)), // East Asian ideograph
    (0x227A72, ('\u{8201}', false)), // East Asian ideograph (not in Unicode)
    (0x227A74, ('\u{8203}', false)), // East Asian ideograph
    (0x227A75, ('\u{8204}', false)), // East Asian ideograph
    (0x2D3C49, ('\u{83F7}', false)), // East Asian ideograph
    (0x227A77, ('\u{820B}', false)), // East Asian ideograph
    (0x217A78, ('\u{5A09}', false)), // East Asian ideograph
    (0x23546A, ('\u{9AD1}', false)), // East Asian ideograph
    (0x217A7E, ('\u{5A12}', false)), // East Asian ideograph
    (0x6F4E78, ('\u{B7B5}', false)), // Korean hangul
    (0x6F546B, ('\u{C4F1}', false)), // Korean hangul
    (0x6F5554, ('\u{C5CC}', false)), // Korean hangul
    (0x6F546C, ('\u{C4F4}', false)), // Korean hangul
    (0x274226, ('\u{62E7}', false)), // East Asian ideograph
    (0x213B39, ('\u{5BF6}', false)), // East Asian ideograph
    (0x21546D, ('\u{82D3}', false)), // East Asian ideograph
    (0x234337, ('\u{927C}', false)), // East Asian ideograph
    (0x22546E, ('\u{7218}', false)), // East Asian ideograph
    (0x28395C, ('\u{6654}', false)), // East Asian ideograph
    (0x2D546F, ('\u{83C0}', false)), // East Asian ideograph
    (0x22725E, ('\u{7E09}', false)), // East Asian ideograph
    (0x4B4559, ('\u{9792}', false)), // East Asian ideograph
    (0x6F5470, ('\u{C50C}', false)), // Korean hangul
    (0x4B5227, ('\u{6B20}', false)), // East Asian ideograph
    (0x6F5555, ('\u{C5CE}', false)), // Korean hangul
    (0x225471, ('\u{720B}', false)), // East Asian ideograph
    (0x33477B, ('\u{904A}', false)), // East Asian ideograph
    (0x235472, ('\u{9ADC}', false)), // East Asian ideograph
    (0x4B5223, ('\u{7E4A}', false)), // East Asian ideograph
    (0x275777, ('\u{4EB5}', false)), // East Asian ideograph
    (0x215474, ('\u{834A}', false)), // East Asian ideograph
    (0x22725F, ('\u{7E1F}', false)), // East Asian ideograph
    (0x23366F, ('\u{8CEB}', false)), // East Asian ideograph
    (0x335445, ('\u{67C1}', false)), // East Asian ideograph
    (0x6F5475, ('\u{C530}', false)), // Korean hangul
    (0x6F5556, ('\u{C5D0}', false)), // Korean hangul
    (0x235476, ('\u{9AE0}', false)), // East Asian ideograph
    (0x274228, ('\u{62DF}', false)), // East Asian ideograph
    (0x23477C, ('\u{943D}', false)), // East Asian ideograph
    (0x215477, ('\u{8350}', false)), // East Asian ideograph
    (0x27593F, ('\u{8BC1}', false)), // East Asian ideograph
    (0x233F22, ('\u{90DD}', false)), // East Asian ideograph
    (0x6F5478, ('\u{C53B}', false)), // Korean hangul
    (0x293F23, ('\u{90CF}', false)), // East Asian ideograph
    (0x6F5827, ('\u{C999}', false)), // Korean hangul
    (0x225479, ('\u{721A}', false)), // East Asian ideograph
    (0x233670, ('\u{8CDA}', false)), // East Asian ideograph
    (0x212A44, ('\u{E8F1}', false)), // EACC component character
    (0x335446, ('\u{8221}', false)), // East Asian ideograph
    (0x22437E, ('\u{6B2C}', false)), // East Asian ideograph
    (0x4B5154, ('\u{7DF4}', false)), // East Asian ideograph
    (0x6F547C, ('\u{C544}', false)), // Korean hangul
    (0x233F27, ('\u{90D8}', false)), // East Asian ideograph
    (0x6F5B6D, ('\u{D30D}', false)), // Korean hangul
    (0x22547D, ('\u{721F}', false)), // East Asian ideograph
    (0x273F28, ('\u{6001}', false)), // East Asian ideograph
    (0x223272, ('\u{63EB}', false)), // East Asian ideograph
    (0x6F4F53, ('\u{B974}', false)), // Korean hangul
    (0x213F29, ('\u{613E}', false)), // East Asian ideograph
    (0x225048, ('\u{7096}', false)), // East Asian ideograph (not in Unicode)
    (0x6F5624, ('\u{C650}', false)), // Korean hangul
    (0x213F2A, ('\u{6127}', false)), // East Asian ideograph
    (0x27422A, ('\u{6269}', false)), // East Asian ideograph
    (0x213C2B, ('\u{5D87}', false)), // East Asian ideograph
    (0x213F2C, ('\u{6147}', false)), // East Asian ideograph
    (0x275F37, ('\u{9645}', false)), // East Asian ideograph
    (0x223F2D, ('\u{6985}', false)), // East Asian ideograph
    (0x273F2E, ('\u{5E86}', false)), // East Asian ideograph
    (0x6F4E79, ('\u{B7C9}', false)), // Korean hangul
    (0x274833, ('\u{6D51}', false)), // East Asian ideograph
    (0x213F2F, ('\u{6167}', false)), // East Asian ideograph
    (0x6F5559, ('\u{C5D8}', false)), // Korean hangul
    (0x27422B, ('\u{63B7}', false)), // East Asian ideograph
    (0x2E624F, ('\u{772D}', false)), // East Asian ideograph
    (0x227B27, ('\u{821D}', false)), // East Asian ideograph
    (0x227B29, ('\u{8220}', false)), // East Asian ideograph
    (0x6F4A36, ('\u{AE30}', false)), // Korean hangul
    (0x217B2C, ('\u{5A60}', false)), // East Asian ideograph
    (0x223F32, ('\u{693D}', false)), // East Asian ideograph
    (0x227B2E, ('\u{822D}', false)), // East Asian ideograph
    (0x227B2F, ('\u{822F}', false)), // East Asian ideograph
    (0x6F5477, ('\u{C539}', false)), // Korean hangul
    (0x217B31, ('\u{5A67}', false)), // East Asian ideograph
    (0x227B32, ('\u{8238}', false)), // East Asian ideograph
    (0x273F33, ('\u{5FE7}', false)), // East Asian ideograph
    (0x227B34, ('\u{823A}', false)), // East Asian ideograph
    (0x227B35, ('\u{8233}', false)), // East Asian ideograph
    (0x227B36, ('\u{8234}', false)), // East Asian ideograph
    (0x213F34, ('\u{617C}', false)), // East Asian ideograph
    (0x227B3A, ('\u{8232}', false)), // East Asian ideograph
    (0x217B3B, ('\u{5A5E}', false)), // East Asian ideograph
    (0x217B3C, ('\u{5A6D}', false)), // East Asian ideograph
    (0x217B3D, ('\u{5A35}', false)), // East Asian ideograph
    (0x217B3E, ('\u{5A55}', false)), // East Asian ideograph
    (0x27422C, ('\u{64B5}', false)), // East Asian ideograph
    (0x215D58, ('\u{9234}', false)), // East Asian ideograph (variant of 4B5D58 which maps to 9234)
    (0x217B41, ('\u{5A2C}', false)), // East Asian ideograph
    (0x227B42, ('\u{8248}', false)), // East Asian ideograph
    (0x227B43, ('\u{8249}', false)), // East Asian ideograph
    (0x227B45, ('\u{8244}', false)), // East Asian ideograph
    (0x227B47, ('\u{8240}', false)), // East Asian ideograph
    (0x227B48, ('\u{8241}', false)), // East Asian ideograph
    (0x217B49, ('\u{5A65}', false)), // East Asian ideograph
    (0x227B4A, ('\u{8245}', false)), // East Asian ideograph
    (0x227B4B, ('\u{824B}', false)), // East Asian ideograph
    (0x334E37, ('\u{784E}', false)), // East Asian ideograph
    (0x227B50, ('\u{824F}', false)), // East Asian ideograph
    (0x213F38, ('\u{6158}', false)), // East Asian ideograph
    (0x217B52, ('\u{5A64}', false)), // East Asian ideograph
    (0x227B53, ('\u{824E}', false)), // East Asian ideograph
    (0x227B56, ('\u{8256}', false)), // East Asian ideograph
    (0x227B57, ('\u{8257}', false)), // East Asian ideograph
    (0x2D6B33, ('\u{5231}', false)), // East Asian ideograph (not in Unicode)
    (0x6F555B, ('\u{C5E1}', false)), // Korean hangul
    (0x223F3A, ('\u{6934}', false)), // East Asian ideograph
    (0x227B5E, ('\u{825A}', false)), // East Asian ideograph
    (0x227B62, ('\u{825F}', false)), // East Asian ideograph
    (0x223F3B, ('\u{6969}', false)), // East Asian ideograph
    (0x217B65, ('\u{5A8A}', false)), // East Asian ideograph
    (0x227B67, ('\u{8262}', false)), // East Asian ideograph
    (0x217B69, ('\u{5ACF}', false)), // East Asian ideograph
    (0x217B6A, ('\u{5A7A}', false)), // East Asian ideograph
    (0x227B6B, ('\u{8268}', false)), // East Asian ideograph
    (0x227B6F, ('\u{826D}', false)), // East Asian ideograph
    (0x217B71, ('\u{5A9F}', false)), // East Asian ideograph
    (0x273F3E, ('\u{5BAA}', false)), // East Asian ideograph
    (0x227B77, ('\u{8278}', false)), // East Asian ideograph
    (0x213F3F, ('\u{6191}', false)), // East Asian ideograph
    (0x227B7D, ('\u{827F}', false)), // East Asian ideograph
    (0x23433F, ('\u{928D}', false)), // East Asian ideograph
    (0x213F41, ('\u{61AB}', false)), // East Asian ideograph
    (0x23486C, ('\u{946F}', false)), // East Asian ideograph
    (0x295F7C, ('\u{9F80}', false)), // East Asian ideograph
    (0x6F4F54, ('\u{B975}', false)), // Korean hangul
    (0x213F42, ('\u{61A4}', false)), // East Asian ideograph
    (0x453D53, ('\u{5F66}', false)), // East Asian ideograph
    (0x4B4561, ('\u{691C}', false)), // East Asian ideograph
    (0x4B5061, ('\u{7CBE}', false)), // East Asian ideograph
    (0x2D4D5F, ('\u{7741}', false)), // East Asian ideograph
    (0x6F555D, ('\u{C5E5}', false)), // Korean hangul
    (0x27422F, ('\u{64DE}', false)), // East Asian ideograph
    (0x6F5A69, ('\u{D0AC}', false)), // Korean hangul
    (0x213B42, ('\u{5C0B}', false)), // East Asian ideograph
    (0x294666, ('\u{933E}', false)), // East Asian ideograph
    (0x223F45, ('\u{69A0}', false)), // East Asian ideograph
    (0x234340, ('\u{92EE}', false)), // East Asian ideograph
    (0x4B522B, ('\u{7F36}', false)), // East Asian ideograph
    (0x6F5C50, ('\u{D48D}', false)), // Korean hangul
    (0x223F46, ('\u{69B1}', false)), // East Asian ideograph
    (0x23553C, ('\u{9B08}', false)), // East Asian ideograph
    (0x233F47, ('\u{90FE}', false)), // East Asian ideograph
    (0x295031, ('\u{98A7}', false)), // East Asian ideograph
    (0x213F48, ('\u{61B6}', false)), // East Asian ideograph
    (0x6F555E, ('\u{C5EC}', false)), // Korean hangul
    (0x213F49, ('\u{61CD}', false)), // East Asian ideograph
    (0x213B52, ('\u{5C3F}', false)), // East Asian ideograph
    (0x233F4A, ('\u{90FF}', false)), // East Asian ideograph
    (0x6F4A37, ('\u{AE31}', false)), // Korean hangul
    (0x273F4B, ('\u{601C}', false)), // East Asian ideograph
    (0x213F4C, ('\u{61BE}', false)), // East Asian ideograph
    (0x516A26, ('\u{51B4}', false)), // East Asian ideograph
    (0x4D4D61, ('\u{7EF1}', false)), // East Asian ideograph
    (0x6F5B49, ('\u{D23F}', false)), // Korean hangul
    (0x275143, ('\u{7EFE}', false)), // East Asian ideograph
    (0x274231, ('\u{62E2}', false)), // East Asian ideograph
    (0x295940, ('\u{9CBC}', false)), // East Asian ideograph
    (0x213B44, ('\u{5C0E}', false)), // East Asian ideograph
    (0x234A21, ('\u{9627}', false)), // East Asian ideograph
    (0x223F50, ('\u{69CE}', false)), // East Asian ideograph
    (0x22327A, ('\u{63DC}', false)), // East Asian ideograph
    (0x227269, ('\u{7E10}', false)), // East Asian ideograph
    (0x6F5472, ('\u{C528}', false)), // Korean hangul
    (0x4B3F53, ('\u{61F2}', false)), // East Asian ideograph
    (0x4B5671, ('\u{873B}', false)), // East Asian ideograph (variant of 215671 which maps to 873B)
    (0x223F44, ('\u{698A}', false)), // East Asian ideograph
    (0x213F54, ('\u{61F7}', false)), // East Asian ideograph
    (0x234343, ('\u{927A}', false)), // East Asian ideograph
    (0x213F55, ('\u{61F6}', false)), // East Asian ideograph
    (0x27414F, ('\u{635F}', false)), // East Asian ideograph
    (0x22327B, ('\u{63D7}', false)), // East Asian ideograph
    (0x213F56, ('\u{61F8}', false)), // East Asian ideograph
    (0x223F51, ('\u{69CA}', false)), // East Asian ideograph
    (0x233237, ('\u{8A57}', false)), // East Asian ideograph
    (0x213F57, ('\u{61F5}', false)), // East Asian ideograph
    (0x6F5561, ('\u{C5F0}', false)), // Korean hangul
    (0x21355B, ('\u{5436}', false)), // East Asian ideograph
    (0x233F58, ('\u{9111}', false)), // East Asian ideograph
    (0x215D5F, ('\u{927B}', false)), // East Asian ideograph
    (0x224A66, ('\u{6E62}', false)), // East Asian ideograph
    (0x223F59, ('\u{698D}', false)), // East Asian ideograph
    (0x223F5A, ('\u{6991}', false)), // East Asian ideograph
    (0x217C21, ('\u{5AA6}', false)), // East Asian ideograph
    (0x217C22, ('\u{5A8C}', false)), // East Asian ideograph
    (0x213F5B, ('\u{61FC}', false)), // East Asian ideograph
    (0x227C24, ('\u{828E}', false)), // East Asian ideograph
    (0x227C25, ('\u{8291}', false)), // East Asian ideograph
    (0x217C26, ('\u{5AA2}', false)), // East Asian ideograph
    (0x227C27, ('\u{828F}', false)), // East Asian ideograph
    (0x227C28, ('\u{8284}', false)), // East Asian ideograph
    (0x273F5C, ('\u{604B}', false)), // East Asian ideograph
    (0x227C2D, ('\u{8283}', false)), // East Asian ideograph
    (0x227C2E, ('\u{828A}', false)), // East Asian ideograph
    (0x213F5D, ('\u{6208}', false)), // East Asian ideograph
    (0x225138, ('\u{70CB}', false)), // East Asian ideograph
    (0x274C78, ('\u{762B}', false)), // East Asian ideograph
    (0x227C34, ('\u{82A7}', false)), // East Asian ideograph
    (0x217C35, ('\u{5A95}', false)), // East Asian ideograph
    (0x217C36, ('\u{5AAF}', false)), // East Asian ideograph
    (0x227C38, ('\u{82AB}', false)), // East Asian ideograph
    (0x217C39, ('\u{5AC8}', false)), // East Asian ideograph
    (0x227C3A, ('\u{82B0}', false)), // East Asian ideograph
    (0x227C3C, ('\u{82A4}', false)), // East Asian ideograph
    (0x217C3E, ('\u{5AB5}', false)), // East Asian ideograph
    (0x227C3F, ('\u{829A}', false)), // East Asian ideograph
    (0x233F60, ('\u{910B}', false)), // East Asian ideograph
    (0x227C42, ('\u{82A3}', false)), // East Asian ideograph
    (0x6F4E7B, ('\u{B7ED}', false)), // Korean hangul
    (0x227C44, ('\u{82B7}', false)), // East Asian ideograph
    (0x227C45, ('\u{82AE}', false)), // East Asian ideograph (variant of 4C7C45 which maps to 82AE)
    (0x227C46, ('\u{82A9}', false)), // East Asian ideograph
    (0x213F61, ('\u{620C}', false)), // East Asian ideograph
    (0x217C49, ('\u{5AD1}', false)), // East Asian ideograph
    (0x217C4A, ('\u{5A90}', false)), // East Asian ideograph
    (0x6F5563, ('\u{C5F6}', false)), // Korean hangul
    (0x227C4C, ('\u{82A8}', false)), // East Asian ideograph
    (0x213F62, ('\u{6210}', false)), // East Asian ideograph
    (0x227C4E, ('\u{82B4}', false)), // East Asian ideograph
    (0x217C4F, ('\u{5AB8}', false)), // East Asian ideograph
    (0x227C50, ('\u{82A1}', false)), // East Asian ideograph
    (0x226160, ('\u{770E}', false)), // East Asian ideograph
    (0x217C52, ('\u{5AAA}', false)), // East Asian ideograph
    (0x227C53, ('\u{82AA}', false)), // East Asian ideograph
    (0x227C55, ('\u{82D9}', false)), // East Asian ideograph
    (0x227C57, ('\u{82FE}', false)), // East Asian ideograph
    (0x2D3224, ('\u{7B87}', false)), // East Asian ideograph
    (0x217C59, ('\u{5AD3}', false)), // East Asian ideograph
    (0x227C5A, ('\u{82E0}', false)), // East Asian ideograph
    (0x217C5B, ('\u{5AB1}', false)), // East Asian ideograph
    (0x227C5C, ('\u{8300}', false)), // East Asian ideograph
    (0x227C5F, ('\u{82EA}', false)), // East Asian ideograph
    (0x227C60, ('\u{82F7}', false)), // East Asian ideograph
    (0x227C62, ('\u{82EF}', false)), // East Asian ideograph
    (0x227C63, ('\u{833A}', false)), // East Asian ideograph
    (0x227C64, ('\u{82E4}', false)), // East Asian ideograph
    (0x227C65, ('\u{82D5}', false)), // East Asian ideograph
    (0x227C67, ('\u{8307}', false)), // East Asian ideograph
    (0x227C68, ('\u{82FA}', false)), // East Asian ideograph
    (0x227C69, ('\u{82F4}', false)), // East Asian ideograph
    (0x227C6A, ('\u{82E2}', false)), // East Asian ideograph
    (0x213F67, ('\u{621B}', false)), // East Asian ideograph
    (0x227C6D, ('\u{82D2}', false)), // East Asian ideograph
    (0x217C6E, ('\u{5AE0}', false)), // East Asian ideograph
    (0x227C71, ('\u{82EB}', false)), // East Asian ideograph
    (0x227C72, ('\u{82D8}', false)), // East Asian ideograph
    (0x227C73, ('\u{82E1}', false)), // East Asian ideograph
    (0x227C75, ('\u{82F6}', false)), // East Asian ideograph
    (0x28602B, ('\u{762A}', false)), // East Asian ideograph
    (0x227C7B, ('\u{8310}', false)), // East Asian ideograph
    (0x227C7C, ('\u{82F3}', false)), // East Asian ideograph
    (0x233F6A, ('\u{911E}', false)), // East Asian ideograph
    (0x4B4569, ('\u{6A71}', false)), // East Asian ideograph
    (0x23323B, ('\u{8A58}', false)), // East Asian ideograph
    (0x6F5473, ('\u{C529}', false)), // Korean hangul
    (0x274237, ('\u{631B}', false)), // East Asian ideograph
    (0x6F5122, ('\u{BC41}', false)), // Korean hangul
    (0x226162, ('\u{771B}', false)), // East Asian ideograph
    (0x213F6D, ('\u{622E}', false)), // East Asian ideograph
    (0x213F6E, ('\u{6230}', false)), // East Asian ideograph
    (0x4B5973, ('\u{8D2E}', false)), // East Asian ideograph
    (0x275344, ('\u{80C1}', false)), // East Asian ideograph
    (0x213F6F, ('\u{6232}', false)), // East Asian ideograph
    (0x23323C, ('\u{8A52}', false)), // East Asian ideograph
    (0x6F5566, ('\u{C5FD}', false)), // Korean hangul
    (0x21355C, ('\u{5433}', false)), // East Asian ideograph
    (0x274238, ('\u{644A}', false)), // East Asian ideograph
    (0x6F5123, ('\u{BC43}', false)), // Korean hangul
    (0x226163, ('\u{7724}', false)), // East Asian ideograph
    (0x213F72, ('\u{6236}', false)), // East Asian ideograph
    (0x234349, ('\u{92AA}', false)), // East Asian ideograph
    (0x6F4C38, ('\u{B18D}', false)), // Korean hangul
    (0x22557C, ('\u{728B}', false)), // East Asian ideograph
    (0x213F74, ('\u{623E}', false)), // East Asian ideograph
    (0x225057, ('\u{7098}', false)), // East Asian ideograph
    (0x235B2F, ('\u{9D7A}', false)), // East Asian ideograph
    (0x213F75, ('\u{6240}', false)), // East Asian ideograph
    (0x29325D, ('\u{8BD4}', false)), // East Asian ideograph
    (0x2D3F76, ('\u{78A5}', false)), // East Asian ideograph
    (0x6F5A6B, ('\u{D0B5}', false)), // Korean hangul
    (0x6F5124, ('\u{BC44}', false)), // Korean hangul
    (0x213B4C, ('\u{5C37}', false)), // East Asian ideograph
    (0x223F77, ('\u{69BE}', false)), // East Asian ideograph
    (0x4B6A22, ('\u{7F83}', false)), // East Asian ideograph
    (0x213F78, ('\u{6248}', false)), // East Asian ideograph
    (0x222A23, ('\u{5F82}', false)), // East Asian ideograph
    (0x233F79, ('\u{912B}', false)), // East Asian ideograph
    (0x4B456C, ('\u{6ADB}', false)), // East Asian ideograph (variant of 21456C)
    (0x2D602D, ('\u{976D}', false)), // East Asian ideograph
    (0x213F7A, ('\u{624B}', false)), // East Asian ideograph
    (0x212A25, ('\u{E8D4}', false)), // EACC component character
    (0x6F5568, ('\u{C5FF}', false)), // Korean hangul
    (0x294E7B, ('\u{9883}', false)), // East Asian ideograph
    (0x4B6A26, ('\u{6C8D}', false)), // East Asian ideograph
    (0x6F4A39, ('\u{AE37}', false)), // Korean hangul
    (0x2D5A34, ('\u{8CAD}', false)), // East Asian ideograph
    (0x393D6F, ('\u{8907}', false)), // East Asian ideograph
    (0x213F7E, ('\u{6254}', false)), // East Asian ideograph
    (0x6F5A70, ('\u{D0C0}', false)), // Korean hangul
    (0x4B456D, ('\u{823B}', false)), // East Asian ideograph
    (0x2D403F, ('\u{6255}', false)), // East Asian ideograph
    (0x695F70, ('\u{7195}', false)), // East Asian ideograph
    (0x27423B, ('\u{63FD}', false)), // East Asian ideograph
    (0x6F5126, ('\u{BC84}', false)), // Korean hangul
    (0x225731, ('\u{731D}', false)), // East Asian ideograph
    (0x213A7A, ('\u{5BB5}', false)), // East Asian ideograph
    (0x696A2C, ('\u{87D0}', false)), // East Asian ideograph
    (0x2E5A78, ('\u{74A2}', false)), // East Asian ideograph
    (0x277954, ('\u{5956}', false)), // East Asian ideograph
    (0x212A2E, ('\u{E8DC}', false)), // EACC component character
    (0x333240, ('\u{4FFB}', false)), // East Asian ideograph
    (0x232A2F, ('\u{86FA}', false)), // East Asian ideograph
    (0x227D21, ('\u{830C}', false)), // East Asian ideograph
    (0x227D22, ('\u{82FB}', false)), // East Asian ideograph
    (0x227D24, ('\u{82FD}', false)), // East Asian ideograph
    (0x215925, ('\u{8AE6}', false)), // East Asian ideograph
    (0x227D26, ('\u{8333}', false)), // East Asian ideograph
    (0x4B5238, ('\u{7F87}', false)), // East Asian ideograph
    (0x227D29, ('\u{8328}', false)), // East Asian ideograph
    (0x217D2A, ('\u{5AFD}', false)), // East Asian ideograph
    (0x217D2B, ('\u{5B08}', false)), // East Asian ideograph
    (0x212A32, ('\u{E8DF}', false)), // EACC component character
    (0x227D2E, ('\u{8351}', false)), // East Asian ideograph
    (0x214C5C, ('\u{75F1}', false)), // East Asian ideograph
    (0x6F5C7B, ('\u{D5CC}', false)), // Korean hangul
    (0x4B456F, ('\u{685C}', false)), // East Asian ideograph
    (0x227D35, ('\u{831B}', false)), // East Asian ideograph
    (0x217D38, ('\u{5B03}', false)), // East Asian ideograph
    (0x222A34, ('\u{3013}', false)), // East Asian ideograph (not found in unified han)
    (0x227D3B, ('\u{8356}', false)), // East Asian ideograph
    (0x217D3D, ('\u{5B17}', false)), // East Asian ideograph
    (0x217D3E, ('\u{5B16}', false)), // East Asian ideograph
    (0x227D3F, ('\u{8322}', false)), // East Asian ideograph
    (0x227D40, ('\u{832C}', false)), // East Asian ideograph
    (0x212A36, ('\u{E8E3}', false)), // EACC component character
    (0x217D47, ('\u{5B1B}', false)), // East Asian ideograph
    (0x227D48, ('\u{833C}', false)), // East Asian ideograph
    (0x227D4A, ('\u{834D}', false)), // East Asian ideograph
    (0x334F3A, ('\u{7A49}', false)), // East Asian ideograph
    (0x227D4D, ('\u{8343}', false)), // East Asian ideograph (variant of 4C7D4D which maps to 8343)
    (0x22505C, ('\u{70B7}', false)), // East Asian ideograph
    (0x4B4570, ('\u{6A29}', false)), // East Asian ideograph
    (0x227D52, ('\u{832F}', false)), // East Asian ideograph
    (0x227D53, ('\u{8348}', false)), // East Asian ideograph
    (0x227D54, ('\u{8312}', false)), // East Asian ideograph
    (0x227D56, ('\u{8316}', false)), // East Asian ideograph
    (0x212A39, ('\u{E8E6}', false)), // EACC component character
    (0x227D58, ('\u{831A}', false)), // East Asian ideograph
    (0x217D59, ('\u{5B32}', false)), // East Asian ideograph
    (0x2E6F43, ('\u{9908}', false)), // East Asian ideograph
    (0x6F5A6C, ('\u{D0B7}', false)), // Korean hangul
    (0x6F5129, ('\u{BC8B}', false)), // Korean hangul
    (0x213B51, ('\u{5C41}', false)), // East Asian ideograph
    (0x227D5F, ('\u{8347}', false)), // East Asian ideograph
    (0x227D62, ('\u{83A8}', false)), // East Asian ideograph
    (0x217D63, ('\u{5B3F}', false)), // East Asian ideograph
    (0x4B3021, ('\u{58F1}', false)), // East Asian ideograph
    (0x227D67, ('\u{83AD}', false)), // East Asian ideograph
    (0x6F5121, ('\u{BC40}', false)), // Korean hangul
    (0x286A3C, ('\u{7AAD}', false)), // East Asian ideograph
    (0x4C7D6A, ('\u{8323}', false)), // East Asian ideograph
    (0x227D6D, ('\u{8373}', false)), // East Asian ideograph
    (0x217D6E, ('\u{5B45}', false)), // East Asian ideograph
    (0x6F4E7D, ('\u{B7F4}', false)), // Korean hangul
    (0x227D72, ('\u{83B0}', false)), // East Asian ideograph
    (0x217D74, ('\u{5B4C}', false)), // East Asian ideograph
    (0x212A3E, ('\u{E8EB}', false)), // EACC component character
    (0x227D76, ('\u{831D}', false)), // East Asian ideograph
    (0x6F556D, ('\u{C608}', false)), // Korean hangul
    (0x282868, ('\u{5E91}', false)), // East Asian ideograph
    (0x227D7A, ('\u{838F}', false)), // East Asian ideograph
    (0x6F512A, ('\u{BC8C}', false)), // Korean hangul
    (0x227D7C, ('\u{8395}', false)), // East Asian ideograph
    (0x227D7E, ('\u{8375}', false)), // East Asian ideograph
    (0x215928, ('\u{8AF1}', false)), // East Asian ideograph
    (0x234350, ('\u{92A6}', false)), // East Asian ideograph
    (0x51384D, ('\u{51C3}', false)), // East Asian ideograph
    (0x69545C, ('\u{58D7}', false)), // East Asian ideograph
    (0x212A41, ('\u{E8EE}', false)), // EACC component character
    (0x275E61, ('\u{9614}', false)), // East Asian ideograph
    (0x212A46, ('\u{3013}', false)), // Ideographic geta symbol
    (0x212A42, ('\u{E8EF}', false)), // EACC component character
    (0x23545D, ('\u{9AC8}', false)), // East Asian ideograph
    (0x226A43, ('\u{7ABF}', false)), // East Asian ideograph
    (0x6F556E, ('\u{C60C}', false)), // Korean hangul
    (0x6F512B, ('\u{BC94}', false)), // Korean hangul
    (0x22315C, ('\u{634B}', false)), // East Asian ideograph
    (0x212A45, ('\u{E8F2}', false)), // EACC component character
    (0x6F5574, ('\u{C62C}', false)), // Korean hangul
    (0x2D314C, ('\u{5008}', false)), // East Asian ideograph
    (0x27534D, ('\u{8109}', false)), // East Asian ideograph
    (0x273B6E, ('\u{5188}', false)), // East Asian ideograph
    (0x214C60, ('\u{760D}', false)), // East Asian ideograph
    (0x2D4D71, ('\u{7719}', false)), // East Asian ideograph
    (0x213E6A, ('\u{60DF}', false)), // East Asian ideograph
    (0x6F5C21, ('\u{D33D}', false)), // Korean hangul
    (0x2D3C61, ('\u{5E47}', false)), // East Asian ideograph
    (0x234667, ('\u{93E7}', false)), // East Asian ideograph
    (0x6F512C, ('\u{BC95}', false)), // Korean hangul
    (0x21592A, ('\u{8ADC}', false)), // East Asian ideograph
    (0x2D3C65, ('\u{79CA}', false)), // East Asian ideograph
    (0x6F4E29, ('\u{B550}', false)), // Korean hangul
    (0x393E47, ('\u{8CC9}', false)), // East Asian ideograph
    (0x6F5032, ('\u{BA70}', false)), // Korean hangul
    (0x2F386F, ('\u{8DD7}', false)), // East Asian ideograph
    (0x214C61, ('\u{7627}', false)), // East Asian ideograph
    (0x6F2464, ('\u{314E}', false)), // Korean hangul
    (0x6F5B47, ('\u{D23C}', false)), // Korean hangul
    (0x6F512D, ('\u{BC97}', false)), // Korean hangul
    (0x6F516E, ('\u{BE45}', false)), // Korean hangul
    (0x226A4F, ('\u{7AD1}', false)), // East Asian ideograph
    (0x4B3974, ('\u{5B22}', false)), // East Asian ideograph
    (0x3F5564, ('\u{61DE}', false)), // East Asian ideograph
    (0x214C62, ('\u{7613}', false)), // East Asian ideograph
    (0x4B6130, ('\u{99C4}', false)), // East Asian ideograph
    (0x6F5571, ('\u{C624}', false)), // Korean hangul
    (0x6F512E, ('\u{BC98}', false)), // Korean hangul
    (0x284642, ('\u{6BF5}', false)), // East Asian ideograph
    (0x226A54, ('\u{7AD5}', false)), // East Asian ideograph
    (0x2D5A3D, ('\u{8CDB}', false)), // East Asian ideograph
    (0x225C50, ('\u{74E4}', false)), // East Asian ideograph
    (0x225062, ('\u{70A1}', false)), // East Asian ideograph
    (0x335461, ('\u{6CD6}', false)), // East Asian ideograph
    (0x213041, ('\u{4E4D}', false)), // East Asian ideograph
    (0x6F5572, ('\u{C625}', false)), // Korean hangul
    (0x6F512F, ('\u{BC99}', false)), // Korean hangul
    (0x234355, ('\u{929A}', false)), // East Asian ideograph
    (0x6F4A3B, ('\u{AE40}', false)), // Korean hangul
    (0x217E59, ('\u{5BC1}', false)), // East Asian ideograph
    (0x2F2A5A, ('\u{868B}', false)), // Unrelated variant of EACC 23293D which maps to 868B
    (0x22714B, ('\u{7D9B}', false)), // East Asian ideograph
    (0x227E21, ('\u{837F}', false)), // East Asian ideograph
    (0x227E22, ('\u{8399}', false)), // East Asian ideograph
    (0x225063, ('\u{70A3}', false)), // East Asian ideograph
    (0x217E24, ('\u{5B65}', false)), // East Asian ideograph
    (0x227E25, ('\u{8387}', false)), // East Asian ideograph
    (0x227E26, ('\u{83B9}', false)), // East Asian ideograph
    (0x217E27, ('\u{5C58}', false)), // East Asian ideograph (not in Unicode)
    (0x217E28, ('\u{5B6C}', false)), // East Asian ideograph
    (0x217E2A, ('\u{5B6E}', false)), // East Asian ideograph
    (0x227E2B, ('\u{83A9}', false)), // East Asian ideograph
    (0x227E2F, ('\u{839B}', false)), // East Asian ideograph
    (0x217E30, ('\u{5B7B}', false)), // East Asian ideograph
    (0x217E31, ('\u{5B7C}', false)), // East Asian ideograph
    (0x217E32, ('\u{5B80}', false)), // East Asian ideograph
    (0x227E33, ('\u{83AA}', false)), // East Asian ideograph
    (0x217E34, ('\u{5B84}', false)), // East Asian ideograph
    (0x217E35, ('\u{5B82}', false)), // East Asian ideograph (not in Unicode)
    (0x227E37, ('\u{839C}', false)), // East Asian ideograph
    (0x227E38, ('\u{839F}', false)), // East Asian ideograph
    (0x222A5F, ('\u{5FD1}', false)), // East Asian ideograph
    (0x217E40, ('\u{5B95}', false)), // East Asian ideograph
    (0x227E41, ('\u{83CF}', false)), // East Asian ideograph
    (0x227E43, ('\u{83F9}', false)), // East Asian ideograph
    (0x2F445F, ('\u{941A}', false)), // East Asian ideograph
    (0x227E45, ('\u{8421}', false)), // East Asian ideograph
    (0x6F5476, ('\u{C538}', false)), // Korean hangul
    (0x217E49, ('\u{5BAC}', false)), // East Asian ideograph
    (0x6F5131, ('\u{BCA0}', false)), // Korean hangul
    (0x294A44, ('\u{9655}', false)), // East Asian ideograph
    (0x21592F, ('\u{8AF7}', false)), // East Asian ideograph
    (0x227E52, ('\u{83EA}', false)), // East Asian ideograph
    (0x227E53, ('\u{8413}', false)), // East Asian ideograph
    (0x217E55, ('\u{5BB7}', false)), // East Asian ideograph
    (0x227E56, ('\u{83FC}', false)), // East Asian ideograph
    (0x227E57, ('\u{83F6}', false)), // East Asian ideograph
    (0x227E59, ('\u{8410}', false)), // East Asian ideograph
    (0x227E5A, ('\u{83E1}', false)), // East Asian ideograph
    (0x217E5B, ('\u{3761}', false)), // East Asian ideograph (not found in unified han)
    (0x227E60, ('\u{83C6}', false)), // East Asian ideograph
    (0x227E61, ('\u{8407}', false)), // East Asian ideograph
    (0x227E63, ('\u{83EB}', false)), // East Asian ideograph
    (0x216A66, ('\u{51DF}', false)), // East Asian ideograph
    (0x6F5575, ('\u{C62D}', false)), // Korean hangul
    (0x217E68, ('\u{5BD4}', false)), // East Asian ideograph
    (0x217E6A, ('\u{5BC3}', false)), // East Asian ideograph
    (0x227E6B, ('\u{83E2}', false)), // East Asian ideograph
    (0x227E6D, ('\u{8401}', false)), // East Asian ideograph
    (0x217E6E, ('\u{5BD6}', false)), // East Asian ideograph
    (0x234358, ('\u{92AB}', false)), // East Asian ideograph
    (0x227E71, ('\u{83D8}', false)), // East Asian ideograph
    (0x227E72, ('\u{83E5}', false)), // East Asian ideograph
    (0x227E74, ('\u{8418}', false)), // East Asian ideograph
    (0x217E75, ('\u{5BD7}', false)), // East Asian ideograph
    (0x227E79, ('\u{83CE}', false)), // East Asian ideograph
    (0x227E7B, ('\u{83D3}', false)), // East Asian ideograph
    (0x295B52, ('\u{9E4E}', false)), // East Asian ideograph
    (0x227E7D, ('\u{83D6}', false)), // East Asian ideograph
    (0x217E7E, ('\u{5BEA}', false)), // East Asian ideograph
    (0x6F5576, ('\u{C62E}', false)), // Korean hangul
    (0x6F5133, ('\u{BCA4}', false)), // Korean hangul
    (0x294A46, ('\u{9649}', false)), // East Asian ideograph
    (0x275F3D, ('\u{96B6}', false)), // East Asian ideograph
    (0x4B6260, ('\u{9EBD}', false)), // East Asian ideograph
    (0x227E6A, ('\u{83BF}', false)), // East Asian ideograph
    (0x2D6030, ('\u{97EE}', false)), // East Asian ideograph
    (0x235466, ('\u{9AD0}', false)), // East Asian ideograph
    (0x22454D, ('\u{6996}', false)), // East Asian ideograph
    (0x6F5577, ('\u{C633}', false)), // Korean hangul
    (0x6F5134, ('\u{BCA7}', false)), // Korean hangul
    (0x213B5C, ('\u{5C50}', false)), // East Asian ideograph
    (0x215932, ('\u{8B19}', false)), // East Asian ideograph
    (0x6F4A3C, ('\u{AE41}', false)), // Korean hangul
    (0x2D3C6D, ('\u{8298}', false)), // East Asian ideograph (duplicate simplified)
    (0x222A73, ('\u{5FF8}', false)), // East Asian ideograph
    (0x225068, ('\u{7551}', false)), // East Asian ideograph
    (0x6F5551, ('\u{C5C8}', false)), // Korean hangul
    (0x6F5135, ('\u{BCA8}', false)), // Korean hangul
    (0x215521, ('\u{5179}', false)), // East Asian ideograph
    (0x223F5C, ('\u{69AA}', false)), // East Asian ideograph
    (0x275274, ('\u{58F0}', false)), // East Asian ideograph
    (0x2D3C6E, ('\u{7240}', false)), // East Asian ideograph
    (0x6F5523, ('\u{C54C}', false)), // Korean hangul
    (0x6F5524, ('\u{C54E}', false)), // Korean hangul
    (0x23324F, ('\u{8A7F}', false)), // East Asian ideograph
    (0x233E37, ('\u{9088}', false)), // East Asian ideograph
    (0x4C4446, ('\u{6B4E}', false)), // East Asian ideograph
    (0x6F5525, ('\u{C553}', false)), // Korean hangul
    (0x232A7B, ('\u{877B}', false)), // East Asian ideograph
    (0x6F5526, ('\u{C554}', false)), // Korean hangul
    (0x3A6A7C, ('\u{7BEA}', false)), // East Asian ideograph
    (0x235527, ('\u{9AEB}', false)), // East Asian ideograph
    (0x22763D, ('\u{801E}', false)), // East Asian ideograph
    (0x235528, ('\u{9AF2}', false)), // East Asian ideograph
    (0x215529, ('\u{8396}', false)), // East Asian ideograph
    (0x233250, ('\u{8A86}', false)), // East Asian ideograph
    (0x225424, ('\u{71C1}', false)), // East Asian ideograph
    (0x21552A, ('\u{83A7}', false)), // East Asian ideograph
    (0x6F5137, ('\u{BCB1}', false)), // Korean hangul
    (0x213B5F, ('\u{5C5C}', false)), // East Asian ideograph
    (0x6F552B, ('\u{C55E}', false)), // Korean hangul
    (0x295F7B, ('\u{9F51}', false)), // East Asian ideograph
    (0x2D3C70, ('\u{576B}', false)), // East Asian ideograph
    (0x27552D, ('\u{5E84}', false)), // East Asian ideograph
    (0x2D552E, ('\u{82FA}', false)), // East Asian ideograph (variant of 227C68)
    (0x23316E, ('\u{8A04}', false)), // East Asian ideograph
    (0x2D493A, ('\u{702C}', false)), // East Asian ideograph
    (0x215D79, ('\u{9375}', false)), // East Asian ideograph
    (0x28464C, ('\u{6BE1}', false)), // East Asian ideograph
    (0x213B60, ('\u{5C62}', false)), // East Asian ideograph
    (0x6F5531, ('\u{C570}', false)), // Korean hangul
    (0x293726, ('\u{8D5C}', false)), // East Asian ideograph
    (0x235532, ('\u{9AF9}', false)), // East Asian ideograph
    (0x6F5533, ('\u{C573}', false)), // Korean hangul
    (0x6F5534, ('\u{C574}', false)), // Korean hangul
    (0x2F3363, ('\u{8B1A}', false)), // East Asian ideograph
    (0x6F5139, ('\u{BCB5}', false)), // Korean hangul
    (0x235535, ('\u{9AFD}', false)), // East Asian ideograph
    (0x4B3474, ('\u{537F}', false)), // East Asian ideograph (variant of 213474 which maps to 537F)
    (0x6F582F, ('\u{C9D0}', false)), // Korean hangul
    (0x22385A, ('\u{6678}', false)), // East Asian ideograph
    (0x235536, ('\u{9B01}', false)), // East Asian ideograph
    (0x2D5A48, ('\u{8D71}', false)), // East Asian ideograph
    (0x295B59, ('\u{9E5C}', false)), // East Asian ideograph
    (0x235538, ('\u{9B02}', false)), // East Asian ideograph
    (0x6F5539, ('\u{C584}', false)), // Korean hangul
    (0x6F513A, ('\u{BCBC}', false)), // Korean hangul
    (0x4B553A, ('\u{83C1}', false)), // East Asian ideograph (variant of 21553A which maps to 83C1)
    (0x23553B, ('\u{9B00}', false)), // East Asian ideograph
    (0x2D3830, ('\u{573B}', false)), // East Asian ideograph
    (0x27553C, ('\u{534E}', false)), // East Asian ideograph
    (0x283542, ('\u{64B7}', false)), // East Asian ideograph
    (0x287531, ('\u{7F9F}', false)), // East Asian ideograph
    (0x33502A, ('\u{9257}', false)), // East Asian ideograph
    (0x23553E, ('\u{9B04}', false)), // East Asian ideograph
    (0x6F513B, ('\u{BCBD}', false)), // Korean hangul
    (0x213B63, ('\u{5C6C}', false)), // East Asian ideograph
    (0x22565B, ('\u{72C1}', false)), // East Asian ideograph
    (0x275947, ('\u{8C34}', false)), // East Asian ideograph
    (0x223442, ('\u{648F}', false)), // East Asian ideograph
    (0x213E3F, ('\u{6062}', false)), // East Asian ideograph
    (0x3F4472, ('\u{7881}', false)), // East Asian ideograph
    (0x215541, ('\u{840A}', false)), // East Asian ideograph
    (0x4B5542, ('\u{8420}', false)), // East Asian ideograph
    (0x33502B, ('\u{724B}', false)), // East Asian ideograph
    (0x235543, ('\u{9B0B}', false)), // East Asian ideograph
    (0x6F4B2B, ('\u{AFCE}', false)), // Korean hangul
    (0x2E4D3D, ('\u{6D38}', false)), // East Asian ideograph
    (0x696A5E, ('\u{88B0}', false)), // East Asian ideograph
    (0x227431, ('\u{7F4E}', false)), // East Asian ideograph
    (0x213561, ('\u{543B}', false)), // East Asian ideograph
    (0x225D39, ('\u{7517}', false)), // East Asian ideograph
    (0x6F5545, ('\u{C5B5}', false)), // Korean hangul
    (0x6F5546, ('\u{C5B6}', false)), // Korean hangul
    (0x6F4F5B, ('\u{B98E}', false)), // Korean hangul
    (0x295B5C, ('\u{9E5B}', false)), // East Asian ideograph
    (0x225070, ('\u{79CC}', false)), // East Asian ideograph
    (0x235547, ('\u{9B0E}', false)), // East Asian ideograph
    (0x233256, ('\u{8A61}', false)), // East Asian ideograph
    (0x6F5548, ('\u{C5B9}', false)), // Korean hangul
    (0x274252, ('\u{654C}', false)), // East Asian ideograph
    (0x6F513D, ('\u{BCC4}', false)), // Korean hangul
    (0x284651, ('\u{6C07}', false)), // East Asian ideograph
    (0x6F5549, ('\u{C5BA}', false)), // Korean hangul
    (0x213722, ('\u{55C6}', false)), // East Asian ideograph
    (0x233C2D, ('\u{8F40}', false)), // East Asian ideograph
    (0x6F554A, ('\u{C5BB}', false)), // Korean hangul
    (0x6F554B, ('\u{C5BC}', false)), // Korean hangul
    (0x28575E, ('\u{72B8}', false)), // East Asian ideograph
    (0x227849, ('\u{811D}', false)), // East Asian ideograph
    (0x393944, ('\u{59B3}', false)), // East Asian ideograph
    (0x6F554C, ('\u{C5BD}', false)), // Korean hangul
    (0x355E76, ('\u{82BE}', false)), // East Asian ideograph
    (0x27554D, ('\u{82C7}', false)), // East Asian ideograph
    (0x6F513E, ('\u{BCCC}', false)), // Korean hangul
    (0x23554E, ('\u{9B11}', false)), // East Asian ideograph
    (0x223F65, ('\u{699E}', false)), // East Asian ideograph
    (0x4B5959, ('\u{8273}', false)), // East Asian ideograph
    (0x21554F, ('\u{8449}', false)), // East Asian ideograph
    (0x2D5550, ('\u{585F}', false)), // East Asian ideograph
    (0x28575F, ('\u{72F2}', false)), // East Asian ideograph
    (0x283546, ('\u{6445}', false)), // East Asian ideograph
    (0x225072, ('\u{70BF}', false)), // East Asian ideograph
    (0x215551, ('\u{846B}', false)), // East Asian ideograph
    (0x233258, ('\u{8A3E}', false)), // East Asian ideograph
    (0x225552, ('\u{7253}', false)), // East Asian ideograph
    (0x274254, ('\u{6570}', false)), // East Asian ideograph
    (0x6F513F, ('\u{BCCD}', false)), // Korean hangul
    (0x225553, ('\u{7255}', false)), // East Asian ideograph
    (0x275276, ('\u{806A}', false)), // East Asian ideograph
    (0x6F7726, ('\u{ADB9}', false)), // Korean hangul
    (0x235554, ('\u{9B18}', false)), // East Asian ideograph
    (0x6F585A, ('\u{CAC4}', false)), // Korean hangul
    (0x333F22, ('\u{6168}', false)), // East Asian ideograph (variant of 213F22 which maps to 6168)
    (0x275555, ('\u{83B4}', false)), // East Asian ideograph
];

pub const SUPERSCRIPTS: [(u32, (char, bool)); 14] = [
    (0x28, ('\u{207D}', false)), // SUPERSCRIPT OPENING PARENTHESIS / SUPERSCRIPT LEFT PARENTHESIS
    (0x29, ('\u{207E}', false)), // SUPERSCRIPT CLOSING PARENTHESIS / SUPERSCRIPT RIGHT PARENTHESIS
    (0x2B, ('\u{207A}', false)), // SUPERSCRIPT PLUS SIGN
    (0x2D, ('\u{207B}', false)), // SUPERSCRIPT HYPHEN-MINUS / SUPERSCRIPT MINUS
    (0x30, ('\u{2070}', false)), // SUPERSCRIPT DIGIT ZERO
    (0x31, ('\u{B9}', false)),   // SUPERSCRIPT DIGIT ONE
    (0x32, ('\u{B2}', false)),   // SUPERSCRIPT DIGIT TWO
    (0x33, ('\u{B3}', false)),   // SUPERSCRIPT DIGIT THREE
    (0x34, ('\u{2074}', false)), // SUPERSCRIPT DIGIT FOUR
    (0x35, ('\u{2075}', false)), // SUPERSCRIPT DIGIT FIVE
    (0x36, ('\u{2076}', false)), // SUPERSCRIPT DIGIT SIX
    (0x37, ('\u{2077}', false)), // SUPERSCRIPT DIGIT SEVEN
    (0x38, ('\u{2078}', false)), // SUPERSCRIPT DIGIT EIGHT
    (0x39, ('\u{2079}', false)), // SUPERSCRIPT DIGIT NINE
];

pub const EXTENDED_CYRILLIC: [(u32, (char, bool)); 42] = [
    (0xC0, ('\u{491}', false)), // LOWERCASE GE WITH UPTURN / CYRILLIC SMALL LETTER GHE WITH UPTURN
    (0xC1, ('\u{452}', false)), // LOWERCASE DJE / CYRILLIC SMALL LETTER DJE (Serbian)
    (0xC2, ('\u{453}', false)), // CYRILLIC SMALL LETTER GJE
    (0xC3, ('\u{454}', false)), // LOWERCASE E / CYRILLIC SMALL LETTER UKRAINIAN IE
    (0xC4, ('\u{451}', false)), // CYRILLIC SMALL LETTER IO
    (0xC5, ('\u{455}', false)), // CYRILLIC SMALL LETTER DZE
    (0xC6, ('\u{456}', false)), // LOWERCASE I / CYRILLIC SMALL LETTER BYELORUSSIAN-UKRANIAN I
    (0xC7, ('\u{457}', false)), // LOWERCASE YI / CYRILLIC SMALL LETTER YI (Ukrainian)
    (0xC8, ('\u{458}', false)), // CYRILLIC SMALL LETTER JE
    (0xC9, ('\u{459}', false)), // CYRILLIC SMALL LETTER LJE
    (0xCA, ('\u{45A}', false)), // CYRILLIC SMALL LETTER NJE
    (0xCB, ('\u{45B}', false)), // LOWERCASE TSHE / CYRILLIC SMALL LETTER TSHE (Serbian)
    (0xCC, ('\u{45C}', false)), // CYRILLIC SMALL LETTER KJE
    (0xCD, ('\u{45E}', false)), // LOWERCASE SHORT U / CYRILLIC SMALL LETTER SHORT U (Byelorussian)
    (0xCE, ('\u{45F}', false)), // CYRILLIC SMALL LETTER DZHE
    (0xD0, ('\u{463}', false)), // CYRILLIC SMALL LETTER YAT
    (0xD1, ('\u{473}', false)), // CYRILLIC SMALL LETTER FITA
    (0xD2, ('\u{475}', false)), // CYRILLIC SMALL LETTER IZHITSA
    (0xD3, ('\u{46B}', false)), // CYRILLIC SMALL LETTER BIG YUS
    (0xDB, ('\u{5B}', false)),  // OPENING SQUARE BRACKET / LEFT SQUARE BRACKET
    (0xDD, ('\u{5D}', false)),  // CLOSING SQUARE BRACKET / RIGHT SQUARE BRACKET
    (0xDF, ('\u{5F}', false)),  // SPACING UNDERSCORE / LOW LINE
    (0xE0, ('\u{490}', false)), // UPPERCASE GE WITH UPTURN / CYRILLIC CAPITAL LETTER GHE WITH UPTURN
    (0xE1, ('\u{402}', false)), // UPPERCASE DJE / CYRILLIC CAPITAL LETTER DJE (Serbian)
    (0xE2, ('\u{403}', false)), // CYRILLIC CAPITAL LETTER GJE
    (0xE3, ('\u{404}', false)), // UPPERCASE E / CYRILLIC CAPITAL LETTER UKRAINIAN IE
    (0xE4, ('\u{401}', false)), // CYRILLIC CAPITAL LETTER IO
    (0xE5, ('\u{405}', false)), // CYRILLIC CAPITAL LETTER DZE
    (0xE6, ('\u{406}', false)), // UPPERCASE I / CYRILLIC CAPITAL LETTER BYELORUSSIAN-UKRANIAN I
    (0xE7, ('\u{407}', false)), // UPPERCASE YI / CYRILLIC CAPITAL LETTER YI (Ukrainian)
    (0xE8, ('\u{408}', false)), // CYRILLIC CAPITAL LETTER JE
    (0xE9, ('\u{409}', false)), // CYRILLIC CAPITAL LETTER LJE
    (0xEA, ('\u{40A}', false)), // CYRILLIC CAPITAL LETTER NJE
    (0xEB, ('\u{40B}', false)), // UPPERCASE TSHE / CYRILLIC CAPITAL LETTER TSHE (Serbian)
    (0xEC, ('\u{40C}', false)), // CYRILLIC CAPITAL LETTER KJE
    (0xED, ('\u{40E}', false)), // UPPERCASE SHORT U / CYRILLIC CAPITAL LETTER SHORT U (Byelorussian)
    (0xEE, ('\u{40F}', false)), // CYRILLIC CAPITAL LETTER DZHE
    (0xEF, ('\u{42A}', false)), // CYRILLIC CAPITAL LETTER HARD SIGN
    (0xF0, ('\u{462}', false)), // CYRILLIC CAPITAL LETTER YAT
    (0xF1, ('\u{472}', false)), // CYRILLIC CAPITAL LETTER FITA
    (0xF2, ('\u{474}', false)), // CYRILLIC CAPITAL LETTER IZHITSA
    (0xF3, ('\u{46A}', false)), // CYRILLIC CAPITAL LETTER BIG YUS
];

pub const BASIC_GREEK: [(u32, (char, bool)); 73] = [
    (0x21, ('\u{300}', true)),   // COMBINING GRAVE ACCENT
    (0x22, ('\u{301}', true)),   // COMBINING ACUTE ACCENT
    (0x23, ('\u{308}', true)),   // COMBINING DIAERESIS
    (0x24, ('\u{342}', true)),   // COMBINING GREEK PERISPOMENI / CIRCUMFLEX
    (0x25, ('\u{313}', true)),   // COMBINING COMMA ABOVE / SMOOTH BREATHING
    (0x26, ('\u{314}', true)),   // COMBINING REVERSED COMMA ABOVE / ROUGH BREATHING
    (0x27, ('\u{345}', true)),   // COMBINING GREEK YPOGEGRAMMENI / IOTA SUBSCRIPT
    (0x30, ('\u{AB}', false)),   // LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
    (0x31, ('\u{BB}', false)),   // RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
    (0x32, ('\u{201C}', false)), // LEFT DOUBLE QUOTATION MARK
    (0x33, ('\u{201D}', false)), // RIGHT DOUBLE QUOTATION MARK
    (0x34, ('\u{374}', false)),  // GREEK NUMERAL SIGN / UPPER PRIME
    (0x35, ('\u{375}', false)),  // GREEK LOWER NUMERAL SIGN / LOWER PRIME
    (0x3B, ('\u{387}', false)),  // GREEK ANO TELEIA / RAISED DOT, GREEK SEMICOLON
    (0x3F, ('\u{37E}', false)),  // GREEK QUESTION MARK
    (0x41, ('\u{391}', false)),  // GREEK CAPITAL LETTER ALPHA
    (0x42, ('\u{392}', false)),  // GREEK CAPITAL LETTER BETA
    (0x44, ('\u{393}', false)),  // GREEK CAPITAL LETTER GAMMA
    (0x45, ('\u{394}', false)),  // GREEK CAPITAL LETTER DELTA
    (0x46, ('\u{395}', false)),  // GREEK CAPITAL LETTER EPSILON
    (0x47, ('\u{3DA}', false)),  // GREEK LETTER STIGMA
    (0x48, ('\u{3DC}', false)),  // GREEK LETTER DIGAMMA
    (0x49, ('\u{396}', false)),  // GREEK CAPITAL LETTER ZETA
    (0x4A, ('\u{397}', false)),  // GREEK CAPITAL LETTER ETA
    (0x4B, ('\u{398}', false)),  // GREEK CAPITAL LETTER THETA
    (0x4C, ('\u{399}', false)),  // GREEK CAPITAL LETTER IOTA
    (0x4D, ('\u{39A}', false)),  // GREEK CAPITAL LETTER KAPPA
    (0x4E, ('\u{39B}', false)),  // GREEK CAPITAL LETTER LAMDA
    (0x4F, ('\u{39C}', false)),  // GREEK CAPITAL LETTER MU
    (0x50, ('\u{39D}', false)),  // GREEK CAPITAL LETTER NU
    (0x51, ('\u{39E}', false)),  // GREEK CAPITAL LETTER XI
    (0x52, ('\u{39F}', false)),  // GREEK CAPITAL LETTER OMICRON
    (0x53, ('\u{3A0}', false)),  // GREEK CAPITAL LETTER PI
    (0x54, ('\u{3DE}', false)),  // GREEK LETTER KOPPA
    (0x55, ('\u{3A1}', false)),  // GREEK CAPITAL LETTER RHO
    (0x56, ('\u{3A3}', false)),  // GREEK CAPITAL LETTER SIGMA
    (0x58, ('\u{3A4}', false)),  // GREEK CAPITAL LETTER TAU
    (0x59, ('\u{3A5}', false)),  // GREEK CAPITAL LETTER UPSILON
    (0x5A, ('\u{3A6}', false)),  // GREEK CAPITAL LETTER PHI
    (0x5B, ('\u{3A7}', false)),  // GREEK CAPITAL LETTER CHI
    (0x5C, ('\u{3A8}', false)),  // GREEK CAPITAL LETTER PSI
    (0x5D, ('\u{3A9}', false)),  // GREEK CAPITAL LETTER OMEGA
    (0x5E, ('\u{3E0}', false)),  // GREEK LETTER SAMPI
    (0x61, ('\u{3B1}', false)),  // GREEK SMALL LETTER ALPHA
    (0x62, ('\u{3B2}', false)),  // GREEK SMALL LETTER BETA / SMALL LETTER BETA BEGINNING OF WORD
    (0x63, ('\u{3D0}', false)),  // GREEK BETA SYMBOL / SMALL LETTER BETA MIDDLE OF WORD
    (0x64, ('\u{3B3}', false)),  // GREEK SMALL LETTER GAMMA
    (0x65, ('\u{3B4}', false)),  // GREEK SMALL LETTER DELTA
    (0x66, ('\u{3B5}', false)),  // GREEK SMALL LETTER EPSILON
    (0x67, ('\u{3DB}', false)),  // GREEK SMALL LETTER STIGMA
    (0x68, ('\u{3DD}', false)),  // GREEK SMALL LETTER DIGAMMA
    (0x69, ('\u{3B6}', false)),  // GREEK SMALL LETTER ZETA
    (0x6A, ('\u{3B7}', false)),  // GREEK SMALL LETTER ETA
    (0x6B, ('\u{3B8}', false)),  // GREEK SMALL LETTER THETA
    (0x6C, ('\u{3B9}', false)),  // GREEK SMALL LETTER IOTA
    (0x6D, ('\u{3BA}', false)),  // GREEK SMALL LETTER KAPPA
    (0x6E, ('\u{3BB}', false)),  // GREEK SMALL LETTER LAMDA
    (0x6F, ('\u{3BC}', false)),  // GREEK SMALL LETTER MU
    (0x70, ('\u{3BD}', false)),  // GREEK SMALL LETTER NU
    (0x71, ('\u{3BE}', false)),  // GREEK SMALL LETTER XI
    (0x72, ('\u{3BF}', false)),  // GREEK SMALL LETTER OMICRON
    (0x73, ('\u{3C0}', false)),  // GREEK SMALL LETTER PI
    (0x74, ('\u{3DF}', false)),  // GREEK SMALL LETTER KOPPA
    (0x75, ('\u{3C1}', false)),  // GREEK SMALL LETTER RHO
    (0x76, ('\u{3C3}', false)),  // GREEK SMALL LETTER SIGMA
    (0x77, ('\u{3C2}', false)),  // GREEK SMALL LETTER FINAL SIGMA / SMALL LETTER SIGMA END OF WORD
    (0x78, ('\u{3C4}', false)),  // GREEK SMALL LETTER TAU
    (0x79, ('\u{3C5}', false)),  // GREEK SMALL LETTER UPSILON
    (0x7A, ('\u{3C6}', false)),  // GREEK SMALL LETTER PHI
    (0x7B, ('\u{3C7}', false)),  // GREEK SMALL LETTER CHI
    (0x7C, ('\u{3C8}', false)),  // GREEK SMALL LETTER PSI
    (0x7D, ('\u{3C9}', false)),  // GREEK SMALL LETTER OMEGA
    (0x7E, ('\u{3E1}', false)),  // GREEK SMALL LETTER SAMPI
];

pub const ASCII: [(u32, (char, bool)); 99] = [
    (0x1B, ('\u{1B}', false)), // ESCAPE (Unlikely to occur in UCS/Unicode)
    (0x1D, ('\u{1D}', false)), // RECORD TERMINATOR / GROUP SEPARATOR
    (0x1E, ('\u{1E}', false)), // FIELD TERMINATOR / RECORD SEPARATOR
    (0x1F, ('\u{1F}', false)), // SUBFIELD DELIMITER / UNIT SEPARATOR
    (0x20, ('\u{20}', false)), // SPACE, BLANK / SPACE
    (0x21, ('\u{21}', false)), // EXCLAMATION MARK
    (0x22, ('\u{22}', false)), // QUOTATION MARK
    (0x23, ('\u{23}', false)), // NUMBER SIGN
    (0x24, ('\u{24}', false)), // DOLLAR SIGN
    (0x25, ('\u{25}', false)), // PERCENT SIGN
    (0x26, ('\u{26}', false)), // AMPERSAND
    (0x27, ('\u{27}', false)), // APOSTROPHE
    (0x28, ('\u{28}', false)), // OPENING PARENTHESIS / LEFT PARENTHESIS
    (0x29, ('\u{29}', false)), // CLOSING PARENTHESIS / CLOSING PARENTHESIS
    (0x2A, ('\u{2A}', false)), // ASTERISK
    (0x2B, ('\u{2B}', false)), // PLUS SIGN
    (0x2C, ('\u{2C}', false)), // COMMA
    (0x2D, ('\u{2D}', false)), // HYPHEN-MINUS
    (0x2E, ('\u{2E}', false)), // PERIOD, DECIMAL POINT / FULL STOP
    (0x2F, ('\u{2F}', false)), // SLASH / SOLIDUS
    (0x30, ('\u{30}', false)), // DIGIT ZERO
    (0x31, ('\u{31}', false)), // DIGIT ONE
    (0x32, ('\u{32}', false)), // DIGIT TWO
    (0x33, ('\u{33}', false)), // DIGIT THREE
    (0x34, ('\u{34}', false)), // DIGIT FOUR
    (0x35, ('\u{35}', false)), // DIGIT FIVE
    (0x36, ('\u{36}', false)), // DIGIT SIX
    (0x37, ('\u{37}', false)), // DIGIT SEVEN
    (0x38, ('\u{38}', false)), // DIGIT EIGHT
    (0x39, ('\u{39}', false)), // DIGIT NINE
    (0x3A, ('\u{3A}', false)), // COLON
    (0x3B, ('\u{3B}', false)), // SEMICOLON
    (0x3C, ('\u{3C}', false)), // LESS-THAN SIGN
    (0x3D, ('\u{3D}', false)), // EQUALS SIGN
    (0x3E, ('\u{3E}', false)), // GREATER-THAN SIGN
    (0x3F, ('\u{3F}', false)), // QUESTION MARK
    (0x40, ('\u{40}', false)), // COMMERCIAL AT
    (0x41, ('\u{41}', false)), // LATIN CAPITAL LETTER A
    (0x42, ('\u{42}', false)), // LATIN CAPITAL LETTER B
    (0x43, ('\u{43}', false)), // LATIN CAPITAL LETTER C
    (0x44, ('\u{44}', false)), // LATIN CAPITAL LETTER D
    (0x45, ('\u{45}', false)), // LATIN CAPITAL LETTER E
    (0x46, ('\u{46}', false)), // LATIN CAPITAL LETTER F
    (0x47, ('\u{47}', false)), // LATIN CAPITAL LETTER G
    (0x48, ('\u{48}', false)), // LATIN CAPITAL LETTER H
    (0x49, ('\u{49}', false)), // LATIN CAPITAL LETTER I
    (0x4A, ('\u{4A}', false)), // LATIN CAPITAL LETTER J
    (0x4B, ('\u{4B}', false)), // LATIN CAPITAL LETTER K
    (0x4C, ('\u{4C}', false)), // LATIN CAPITAL LETTER L
    (0x4D, ('\u{4D}', false)), // LATIN CAPITAL LETTER M
    (0x4E, ('\u{4E}', false)), // LATIN CAPITAL LETTER N
    (0x4F, ('\u{4F}', false)), // LATIN CAPITAL LETTER O
    (0x50, ('\u{50}', false)), // LATIN CAPITAL LETTER P
    (0x51, ('\u{51}', false)), // LATIN CAPITAL LETTER Q
    (0x52, ('\u{52}', false)), // LATIN CAPITAL LETTER R
    (0x53, ('\u{53}', false)), // LATIN CAPITAL LETTER S
    (0x54, ('\u{54}', false)), // LATIN CAPITAL LETTER T
    (0x55, ('\u{55}', false)), // LATIN CAPITAL LETTER U
    (0x56, ('\u{56}', false)), // LATIN CAPITAL LETTER V
    (0x57, ('\u{57}', false)), // LATIN CAPITAL LETTER W
    (0x58, ('\u{58}', false)), // LATIN CAPITAL LETTER X
    (0x59, ('\u{59}', false)), // LATIN CAPITAL LETTER Y
    (0x5A, ('\u{5A}', false)), // LATIN CAPITAL LETTER Z
    (0x5B, ('\u{5B}', false)), // OPENING SQUARE BRACKET / LEFT SQUARE BRACKET
    (0x5C, ('\u{5C}', false)), // REVERSE SLASH / REVERSE SOLIDUS
    (0x5D, ('\u{5D}', false)), // CLOSING SQUARE BRACKET / RIGHT SQUARE BRACKET
    (0x5E, ('\u{5E}', false)), // SPACING CIRCUMFLEX / CIRCUMFLEX ACCENT
    (0x5F, ('\u{5F}', false)), // SPACING UNDERSCORE / LOW LINE
    (0x60, ('\u{60}', false)), // SPACING GRAVE / GRAVE ACCENT
    (0x61, ('\u{61}', false)), // LATIN SMALL LETTER A
    (0x62, ('\u{62}', false)), // LATIN SMALL LETTER B
    (0x63, ('\u{63}', false)), // LATIN SMALL LETTER C
    (0x64, ('\u{64}', false)), // LATIN SMALL LETTER D
    (0x65, ('\u{65}', false)), // LATIN SMALL LETTER E
    (0x66, ('\u{66}', false)), // LATIN SMALL LETTER F
    (0x67, ('\u{67}', false)), // LATIN SMALL LETTER G
    (0x68, ('\u{68}', false)), // LATIN SMALL LETTER H
    (0x69, ('\u{69}', false)), // LATIN SMALL LETTER I
    (0x6A, ('\u{6A}', false)), // LATIN SMALL LETTER J
    (0x6B, ('\u{6B}', false)), // LATIN SMALL LETTER K
    (0x6C, ('\u{6C}', false)), // LATIN SMALL LETTER L
    (0x6D, ('\u{6D}', false)), // LATIN SMALL LETTER M
    (0x6E, ('\u{6E}', false)), // LATIN SMALL LETTER N
    (0x6F, ('\u{6F}', false)), // LATIN SMALL LETTER O
    (0x70, ('\u{70}', false)), // LATIN SMALL LETTER P
    (0x71, ('\u{71}', false)), // LATIN SMALL LETTER Q
    (0x72, ('\u{72}', false)), // LATIN SMALL LETTER R
    (0x73, ('\u{73}', false)), // LATIN SMALL LETTER S
    (0x74, ('\u{74}', false)), // LATIN SMALL LETTER T
    (0x75, ('\u{75}', false)), // LATIN SMALL LETTER U
    (0x76, ('\u{76}', false)), // LATIN SMALL LETTER V
    (0x77, ('\u{77}', false)), // LATIN SMALL LETTER W
    (0x78, ('\u{78}', false)), // LATIN SMALL LETTER X
    (0x79, ('\u{79}', false)), // LATIN SMALL LETTER Y
    (0x7A, ('\u{7A}', false)), // LATIN SMALL LETTER Z
    (0x7B, ('\u{7B}', false)), // OPENING CURLY BRACKET / LEFT CURLY BRACKET
    (0x7C, ('\u{7C}', false)), // VERTICAL BAR (FILL) / VERTICAL LINE
    (0x7D, ('\u{7D}', false)), // CLOSING CURLY BRACKET / RIGHT CURLY BRACKET
    (0x7E, ('\u{7E}', false)), // SPACING TILDE / TILDE
];

pub const SUBSCRIPTS: [(u32, (char, bool)); 14] = [
    // Subscripts
    (0x28, ('\u{208D}', false)), // SUBSCRIPT OPENING PARENTHESIS / SUBSCRIPT LEFT PARENTHESIS
    (0x29, ('\u{208E}', false)), // SUBSCRIPT CLOSING PARENTHESIS / SUBSCRIPT RIGHT PARENTHESIS
    (0x2B, ('\u{208A}', false)), // SUBSCRIPT PLUS SIGN
    (0x2D, ('\u{208B}', false)), // SUBSCRIPT HYPHEN-MINUS / SUBSCRIPT MINUS
    (0x30, ('\u{2080}', false)), // SUBSCRIPT DIGIT ZERO
    (0x31, ('\u{2081}', false)), // SUBSCRIPT DIGIT ONE
    (0x32, ('\u{2082}', false)), // SUBSCRIPT DIGIT TWO
    (0x33, ('\u{2083}', false)), // SUBSCRIPT DIGIT THREE
    (0x34, ('\u{2084}', false)), // SUBSCRIPT DIGIT FOUR
    (0x35, ('\u{2085}', false)), // SUBSCRIPT DIGIT FIVE
    (0x36, ('\u{2086}', false)), // SUBSCRIPT DIGIT SIX
    (0x37, ('\u{2087}', false)), // SUBSCRIPT DIGIT SEVEN
    (0x38, ('\u{2088}', false)), // SUBSCRIPT DIGIT EIGHT
    (0x39, ('\u{2089}', false)), // SUBSCRIPT DIGIT NINE
];

pub const GREEK_SYMBOLS: [(u32, (char, bool)); 3] = [
    (0x61, ('\u{3B1}', false)), // GREEK SMALL LETTER ALPHA
    (0x62, ('\u{3B2}', false)), // GREEK SMALL LETTER BETA
    (0x63, ('\u{3B3}', false)), // GREEK SMALL LETTER GAMMA
];

pub const BASIC_CYRILLIC: [(u32, (char, bool)); 94] = [
    (0x21, ('\u{21}', false)),  // EXCLAMATION MARK
    (0x22, ('\u{22}', false)),  // QUOTATION MARK
    (0x23, ('\u{23}', false)),  // NUMBER SIGN
    (0x24, ('\u{24}', false)),  // DOLLAR SIGN
    (0x25, ('\u{25}', false)),  // PERCENT SIGN
    (0x26, ('\u{26}', false)),  // AMPERSAND
    (0x27, ('\u{27}', false)),  // APOSTROPHE
    (0x28, ('\u{28}', false)),  // OPENING PARENTHESIS / LEFT PARENTHESIS
    (0x29, ('\u{29}', false)),  // CLOSING PARENTHESIS / RIGHT PARENTHESIS
    (0x2A, ('\u{2A}', false)),  // ASTERISK
    (0x2B, ('\u{2B}', false)),  // PLUS SIGN
    (0x2C, ('\u{2C}', false)),  // COMMA
    (0x2D, ('\u{2D}', false)),  // HYPHEN-MINUS
    (0x2E, ('\u{2E}', false)),  // PERIOD, DECIMAL POINT / FULL STOP
    (0x2F, ('\u{2F}', false)),  // SLASH / SOLIDUS
    (0x30, ('\u{30}', false)),  // DIGIT ZERO
    (0x31, ('\u{31}', false)),  // DIGIT ONE
    (0x32, ('\u{32}', false)),  // DIGIT TWO
    (0x33, ('\u{33}', false)),  // DIGIT THREE
    (0x34, ('\u{34}', false)),  // DIGIT FOUR
    (0x35, ('\u{35}', false)),  // DIGIT FIVE
    (0x36, ('\u{36}', false)),  // DIGIT SIX
    (0x37, ('\u{37}', false)),  // DIGIT SEVEN
    (0x38, ('\u{38}', false)),  // DIGIT EIGHT
    (0x39, ('\u{39}', false)),  // DIGIT NINE
    (0x3A, ('\u{3A}', false)),  // COLON
    (0x3B, ('\u{3B}', false)),  // SEMICOLON
    (0x3C, ('\u{3C}', false)),  // LESS-THAN SIGN
    (0x3D, ('\u{3D}', false)),  // EQUALS SIGN
    (0x3E, ('\u{3E}', false)),  // GREATER-THAN SIGN
    (0x3F, ('\u{3F}', false)),  // QUESTION MARK
    (0x40, ('\u{44E}', false)), // LOWERCASE IU / CYRILLIC SMALL LETTER YU
    (0x41, ('\u{430}', false)), // CYRILLIC SMALL LETTER A
    (0x42, ('\u{431}', false)), // CYRILLIC SMALL LETTER BE
    (0x43, ('\u{446}', false)), // CYRILLIC SMALL LETTER TSE
    (0x44, ('\u{434}', false)), // CYRILLIC SMALL LETTER DE
    (0x45, ('\u{435}', false)), // CYRILLIC SMALL LETTER IE
    (0x46, ('\u{444}', false)), // CYRILLIC SMALL LETTER EF
    (0x47, ('\u{433}', false)), // LOWERCASE GE / CYRILLIC SMALL LETTER GHE
    (0x48, ('\u{445}', false)), // LOWERCASE KHA / CYRILLIC SMALL LETTER HA
    (0x49, ('\u{438}', false)), // LOWERCASE II / CYRILLIC SMALL LETTER I
    (0x4A, ('\u{439}', false)), // LOWERCASE SHORT II / CYRILLIC SMALL LETTER SHORT I
    (0x4B, ('\u{43A}', false)), // CYRILLIC SMALL LETTER KA
    (0x4C, ('\u{43B}', false)), // CYRILLIC SMALL LETTER EL
    (0x4D, ('\u{43C}', false)), // CYRILLIC SMALL LETTER EM
    (0x4E, ('\u{43D}', false)), // CYRILLIC SMALL LETTER EN
    (0x4F, ('\u{43E}', false)), // CYRILLIC SMALL LETTER O
    (0x50, ('\u{43F}', false)), // CYRILLIC SMALL LETTER PE
    (0x51, ('\u{44F}', false)), // LOWERCASE IA / CYRILLIC SMALL LETTER YA
    (0x52, ('\u{440}', false)), // CYRILLIC SMALL LETTER ER
    (0x53, ('\u{441}', false)), // CYRILLIC SMALL LETTER ES
    (0x54, ('\u{442}', false)), // CYRILLIC SMALL LETTER TE
    (0x55, ('\u{443}', false)), // CYRILLIC SMALL LETTER U
    (0x56, ('\u{436}', false)), // CYRILLIC SMALL LETTER ZHE
    (0x57, ('\u{432}', false)), // CYRILLIC SMALL LETTER VE
    (0x58, ('\u{44C}', false)), // CYRILLIC SMALL LETTER SOFT SIGN
    (0x59, ('\u{44B}', false)), // LOWERCASE YERI / CYRILLIC SMALL LETTER YERI
    (0x5A, ('\u{437}', false)), // CYRILLIC SMALL LETTER ZE
    (0x5B, ('\u{448}', false)), // CYRILLIC SMALL LETTER SHA
    (0x5C, ('\u{44D}', false)), // LOWERCASE REVERSED E / CYRILLIC SMALL LETTER E
    (0x5D, ('\u{449}', false)), // CYRILLIC SMALL LETTER SHCHA
    (0x5E, ('\u{447}', false)), // CYRILLIC SMALL LETTER CHE
    (0x5F, ('\u{44A}', false)), // CYRILLIC SMALL LETTER HARD SIGN
    (0x60, ('\u{42E}', false)), // UPPERCASE IU / CYRILLIC CAPITAL LETTER YU
    (0x61, ('\u{410}', false)), // CYRILLIC CAPITAL LETTER A
    (0x62, ('\u{411}', false)), // CYRILLIC CAPITAL LETTER BE
    (0x63, ('\u{426}', false)), // CYRILLIC CAPITAL LETTER TSE
    (0x64, ('\u{414}', false)), // CYRILLIC CAPITAL LETTER DE
    (0x65, ('\u{415}', false)), // CYRILLIC CAPITAL LETTER IE
    (0x66, ('\u{424}', false)), // CYRILLIC CAPITAL LETTER EF
    (0x67, ('\u{413}', false)), // UPPERCASE GE / CYRILLIC CAPITAL LETTER GHE
    (0x68, ('\u{425}', false)), // UPPERCASE KHA / CYRILLIC CAPITAL LETTER HA
    (0x69, ('\u{418}', false)), // UPPERCASE II / CYRILLIC CAPITAL LETTER I
    (0x6A, ('\u{419}', false)), // UPPERCASE SHORT II / CYRILLIC CAPITAL LETTER SHORT I
    (0x6B, ('\u{41A}', false)), // CYRILLIC CAPITAL LETTER KA
    (0x6C, ('\u{41B}', false)), // CYRILLIC CAPITAL LETTER EL
    (0x6D, ('\u{41C}', false)), // CYRILLIC CAPITAL LETTER EM
    (0x6E, ('\u{41D}', false)), // CYRILLIC CAPITAL LETTER EN
    (0x6F, ('\u{41E}', false)), // CYRILLIC CAPITAL LETTER O
    (0x70, ('\u{41F}', false)), // CYRILLIC CAPITAL LETTER PE
    (0x71, ('\u{42F}', false)), // UPPERCASE IA / CYRILLIC CAPITAL LETTER YA
    (0x72, ('\u{420}', false)), // CYRILLIC CAPITAL LETTER ER
    (0x73, ('\u{421}', false)), // CYRILLIC CAPITAL LETTER ES
    (0x74, ('\u{422}', false)), // CYRILLIC CAPITAL LETTER TE
    (0x75, ('\u{423}', false)), // CYRILLIC CAPITAL LETTER U
    (0x76, ('\u{416}', false)), // CYRILLIC CAPITAL LETTER ZHE
    (0x77, ('\u{412}', false)), // CYRILLIC CAPITAL LETTER VE
    (0x78, ('\u{42C}', false)), // CYRILLIC CAPITAL LETTER SOFT SIGN
    (0x79, ('\u{42B}', false)), // UPPERCASE YERI / CYRILLIC CAPITAL LETTER YERI
    (0x7A, ('\u{417}', false)), // CYRILLIC CAPITAL LETTER ZE
    (0x7B, ('\u{428}', false)), // CYRILLIC CAPITAL LETTER SHA
    (0x7C, ('\u{42D}', false)), // CYRILLIC CAPITAL LETTER E
    (0x7D, ('\u{429}', false)), // CYRILLIC CAPITAL LETTER SHCHA
    (0x7E, ('\u{427}', false)), // CYRILLIC CAPITAL LETTER CHE
];
