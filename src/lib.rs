mod charsets;
mod error;

use std::borrow::Cow;
use std::char;
use unicode_normalization::UnicodeNormalization;

use crate::error::EncodingError;
#[cfg(feature = "python")]
use pyo3::prelude::*;

pub const BASIC_LATIN: u8 = 0x42;
pub const REDESIGNATED_ASCII: u8 = 0x73;
pub const ANSEL: u8 = 0x45;
pub const G0_SET: [u8; 3] = [b'(', b',', b'$'];
pub const G1_SET: [u8; 3] = [b')', b'-', b'$'];
pub const BLANK: char = ' ';
pub const MULTI_BYTE: u8 = 0x31;
pub const ESCAPE: u8 = 0x1b;
pub const CODESETS: [u8; 12] = [
    0x31, 0x32, 0x33, 0x34, 0x42, 0x45, 0x4E, 0x51, 0x53, 0x62, 0x67, 0x70,
];

pub struct Decoder {
    g0: u8,
    g1: u8,
    quiet: bool,
    uni_list: Option<Vec<char>>,
    combinings: Option<Vec<char>>,
}

impl Decoder {
    pub fn new(g0: Option<u8>, g1: Option<u8>, quiet: Option<bool>) -> Self {
        let g0 = g0.unwrap_or(BASIC_LATIN);
        let g1 = g1.unwrap_or(ANSEL);
        let quiet = quiet.unwrap_or(false);

        Decoder {
            g0,
            g1,
            quiet,
            uni_list: None,
            combinings: None,
        }
    }

    pub fn decode<'a>(&mut self, marc8_string: &'a [u8]) -> Result<Cow<'a, str>, EncodingError> {
        if marc8_string.is_empty() {
            return Ok(Cow::Borrowed(""));
        }

        self.uni_list = Some(Vec::new());
        self.combinings = Some(Vec::new());
        let mut pos = 0;
        let mut next_byte: u8;

        while pos < marc8_string.len() {
            if marc8_string[pos] == ESCAPE {
                next_byte = marc8_string[pos + 1];

                if G0_SET.contains(&next_byte) {
                    if marc8_string.len() >= pos + 3 {
                        if marc8_string[pos + 2] == b',' && next_byte == b'$' {
                            pos += 1;
                        }
                        self.g0 = marc8_string[pos + 2];
                        pos += 3;
                        continue;
                    } else {
                        self.uni_list
                            .as_mut()
                            .map(|v| v.push(char::from(marc8_string[pos])));
                        pos += 1;
                        continue;
                    }
                } else if G1_SET.contains(&next_byte) {
                    if marc8_string[pos + 2] == b'-' && next_byte == b'$' {
                        pos += 1;
                    }
                    self.g1 = marc8_string[pos + 2];
                    pos += 3;
                    continue;
                } else {
                    let charset = next_byte;
                    if CODESETS.contains(&charset) {
                        self.g0 = charset;
                        pos += 2;
                    } else if charset == REDESIGNATED_ASCII {
                        self.g0 = BASIC_LATIN;
                        pos += 2;
                        if pos == marc8_string.len() {
                            break;
                        }
                    }
                }
            }

            let mb_flag = self.g0 == MULTI_BYTE;
            let code_point: u32;
            if mb_flag {
                if marc8_string.len() < pos + 3 {
                    eprintln!(
                        "Multi-byte position {} exceeds length of marc8 string {}",
                        pos + 3,
                        marc8_string.len()
                    );
                    code_point = BLANK as u32;
                } else {
                    code_point = (marc8_string[pos] as u32) << 16
                        | (marc8_string[pos + 1] as u32) << 8
                        | (marc8_string[pos + 2]) as u32;
                }
                pos += 3;
            } else {
                code_point = marc8_string[pos] as u32;
                pos += 1;
            }

            if code_point < 0x20 || code_point > 0x80 && code_point < 0xA0 {
                continue;
            }

            let codeset = if code_point > 0x80 && !mb_flag {
                &self.g1
            } else {
                &self.g0
            };

            let pair = Self::get_pair(codeset, code_point);
            if let Some((uni, cflag)) = pair {
                if cflag {
                    self.combinings.as_mut().map(|v| v.push(uni));
                } else {
                    self.uni_list.as_mut().map(|v| v.push(uni));
                    if let Some(combinings) = self.combinings.as_mut() {
                        if !combinings.is_empty() {
                            self.uni_list.as_mut().map(|v| v.extend(combinings.iter()));
                            self.combinings.as_mut().map(|v| v.clear());
                        }
                    }
                }
            } else if let Some((uni, _)) = charsets::get_odd_char(code_point) {
                self.uni_list.as_mut().map(|v| v.push(uni));
            } else {
                if !self.quiet {
                    eprintln!(
                        "Unable to parse character 0x{:X} in g0={} g1={}",
                        code_point, self.g0, self.g1
                    );
                }
                self.uni_list.as_mut().map(|v| v.push(BLANK));
            }
        }

        if let Some(v) = self.uni_list.to_owned() {
            let s = v.into_iter().nfc().collect::<String>();
            Ok(Cow::Owned(s))
        } else {
            Err(EncodingError::NoData)
        }
    }

    fn get_pair(codeset: &u8, code_point: u32) -> Option<(char, bool)> {
        match codeset {
            0x31 => charsets::get_eacc(code_point),
            0x32 => charsets::get_basic_hebrew(code_point),
            0x33 => charsets::get_basic_arabic(code_point),
            0x34 => charsets::get_extended_arabic(code_point),
            0x42 => {
                if code_point < 0x80 {
                    let uni = char::from_u32(code_point).unwrap();
                    Some((uni, false))
                } else {
                    None
                }
            }
            0x45 => charsets::get_ansel(code_point),
            0x4E => charsets::get_basic_cyrillic(code_point),
            0x51 => charsets::get_extended_cyrillic(code_point),
            0x53 => charsets::get_basic_greek(code_point),
            0x62 => charsets::get_subscript(code_point),
            0x67 => charsets::get_greek_symbol(code_point),
            0x70 => charsets::get_superscript(code_point),
            _ => None,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass]
struct MARC8ToUnicode(Decoder);

#[cfg(feature = "python")]
#[pymethods]
impl MARC8ToUnicode {
    #[new]
    #[pyo3(signature = (g0 = None, g1 = None, quiet = None))]
    fn new(g0: Option<u8>, g1: Option<u8>, quiet: Option<bool>) -> Self {
        Self(Decoder::new(g0, g1, quiet))
    }

    fn translate(&mut self, marc8_string: &[u8]) -> String {
        self.0.decode(marc8_string).unwrap().to_string()
    }
}

#[cfg(feature = "python")]
#[pymodule]
fn marc8(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MARC8ToUnicode>()?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let mut converter = Decoder::new(None, None, None);
        let got = converter.decode(b"Conversa\xF0c\xE4ao").unwrap();
        let want = "Conversação";
        assert_eq!(got, want);
    }

    #[test]
    fn subscript_works() {
        let mut converter = Decoder::new(None, None, None);
        let want = "CO₂ is a gas";
        let got = converter.decode(b"CO\x1bb2\x1bs is a gas").unwrap();
        assert_eq!(got, want);
    }

    #[test]
    fn combining() {
        let mut converter = Decoder::new(None, None, None);
        let got = converter.decode(b"La Soci\xe2et").unwrap();
        let want = "La Sociét";
        assert_eq!(got, want);
    }

    #[test]
    fn bad_escape() {
        let mut converter = Decoder::new(None, None, None);
        let got = converter.decode(b"La Soci\xe2et\x1b,").unwrap();
        let want = "La Sociét\x1b,";
        assert_eq!(got, want);
    }

    #[test]
    fn odd_chars() {
        let bytes: &[u8] = &[
            0x61, 0x20, 0x1b, 0x24, 0x31, 0x21, 0x20, 0x3d, 0x21, 0x20, 0x40, 0x7f, 0x20, 0x14,
            0x7f, 0x20, 0x19, 0x7f, 0x20, 0x20, 0x7f, 0x21, 0x22, 0x1b, 0x28, 0x42, 0x20, 0x7a,
            0x0a,
        ];
        let want = "a …“—’”™ z";
        let mut converter = Decoder::new(None, None, None);
        let got = converter.decode(bytes).unwrap();
        println!("{:?}", got);
        println!("{:x?}", got);
        assert_eq!(got, want);
    }

    #[test]
    fn blanks_in_expected_places() {
        let mut converter = Decoder::new(None, None, Some(true));
        let got = converter.decode(b"a\xcc\x80").unwrap();
        let want = "a  ";
        assert_eq!(got, want);
    }

    #[test]
    fn marc8_to_unicode() {
        let mut converter = Decoder::new(None, None, None);

        let got = converter
            .decode(b"\x1b(3YhOI,\x1b(B \x1b(3eMeO\x1b(B.")
            .unwrap()
            .to_string()
            .into_bytes();
        let want = b"\xd8\xb9\xd9\x88\xd8\xaf\xd8\xa9\xd8\x8c \xd9\x85\xd8\xad\xd9\x85\xd8\xaf.";
        assert_eq!(got, want);
    }
}
