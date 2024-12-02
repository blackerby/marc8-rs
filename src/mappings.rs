use crate::charsets::{
    ASCII, BASIC_ARABIC, BASIC_CYRILLIC, BASIC_GREEK, BASIC_HEBREW, EACC, EXTENDED_ARABIC,
    EXTENDED_CYRILLIC, EXTENDED_LATIN, GREEK_SYMBOLS, SUBSCRIPTS, SUPERSCRIPTS,
};
/// MARC-8 mapping.
/// adapted from: https://gitlab.com/pymarc/pymarc/-/blob/main/pymarc/marc8_mapping.py?ref_type=heads
use std::collections::HashMap;
use std::sync::OnceLock;

macro_rules! charset {
    ($name:ident, $constant_name:expr) => {
        pub fn $name() -> &'static HashMap<u32, (u16, bool)> {
            static HASHMAP: OnceLock<HashMap<u32, (u16, bool)>> = OnceLock::new();

            HASHMAP.get_or_init(|| {
                let mut m = HashMap::new();

                for codepoint in $constant_name {
                    m.insert(codepoint.0, codepoint.1);
                }

                m
            })
        }
    };
}

charset!(extended_arabic, EXTENDED_ARABIC);
charset!(extended_latin, EXTENDED_LATIN);
charset!(basic_arabic, BASIC_ARABIC);
charset!(basic_hebrew, BASIC_HEBREW);
charset!(eacc, EACC);
charset!(superscripts, SUPERSCRIPTS);
charset!(extended_cyrillic, EXTENDED_CYRILLIC);
charset!(basic_greek, BASIC_GREEK);
charset!(ascii, ASCII);
charset!(subscripts, SUBSCRIPTS);
charset!(greek_symbols, GREEK_SYMBOLS);
charset!(basic_cyrillic, BASIC_CYRILLIC);

pub fn codesets() -> &'static HashMap<u8, &'static HashMap<u32, (u16, bool)>> {
    static HASHMAP: OnceLock<HashMap<u8, &'static HashMap<u32, (u16, bool)>>> = OnceLock::new();

    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();

        m.insert(0x31, eacc());
        m.insert(0x32, basic_hebrew());
        m.insert(0x33, basic_arabic());
        m.insert(0x34, extended_arabic());
        m.insert(0x42, ascii());
        m.insert(0x45, extended_latin());
        m.insert(0x4E, basic_cyrillic());
        m.insert(0x51, extended_cyrillic());
        m.insert(0x53, basic_greek());
        m.insert(0x62, subscripts());
        m.insert(0x67, greek_symbols());
        m.insert(0x70, superscripts());

        m
    })
}

// ODD_MAP for odd characters (all from III for now)
pub fn odd_map() -> &'static HashMap<u32, u16> {
    static HASHMAP: OnceLock<HashMap<u32, u16>> = OnceLock::new();

    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();

        m.insert(0x21203D, 0x2026); // HORIZONTAL ELLIPSIS
        m.insert(0x212040, 0x201C); // LEFT DOUBLE QUOTATION MARK
        m.insert(0x7F2014, 0x2014); // EM DASH
        m.insert(0x7F2019, 0x2019); // RIGHT SINGLE QUOTATION MARK
        m.insert(0x7F2020, 0x201D); // RIGHT DOUBLE QUOTATION MARK
        m.insert(0x7F2122, 0x2122); // TRADE MARK SIGN

        m
    })
}
