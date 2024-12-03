mod charsets;
mod constants;
mod error;
mod mappings;

use crate::constants::*;
use crate::error::EncodingError;
use crate::mappings::{codesets, odd_map};
use core::str;
use pyo3::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;

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

        if marc8_string.iter().all(|ch| ch.is_ascii()) {
            return Ok(Cow::Borrowed(str::from_utf8(marc8_string)?));
        }

        self.uni_list = Some(Vec::new());
        self.combinings = Some(Vec::new());
        let mut pos = 0;
        let mut next_byte: u8;

        while pos < marc8_string.len() {
            if is_escape(marc8_string[pos]) {
                next_byte = marc8_string[pos + 1];

                if G0_SET.contains(&next_byte) {
                    if marc8_string.len() > pos + 2 {
                        if marc8_string[pos + 2] == b',' && next_byte == b'$' {
                            pos += 1;
                        }
                        self.g0 = marc8_string[pos + 2];
                        pos += 3;
                    } else {
                        self.uni_list
                            .as_mut()
                            .map(|v| v.push(char::from(marc8_string[pos])));
                        pos += 1;
                    }
                } else if G1_SET.contains(&next_byte) {
                    if marc8_string[pos + 2] == b'-' && next_byte == b'$' {
                        pos += 1;
                    }
                    self.g1 = marc8_string[pos + 2];
                    pos += 3;
                } else {
                    let charset = next_byte;
                    if codesets().contains_key(&charset) {
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

            let mb_flag = self.g0 == EACC;
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
                    let mut bytes: [u8; 4] = [0, 0, 0, 0];
                    bytes[1] = marc8_string[pos];
                    bytes[2] = marc8_string[pos + 1];
                    bytes[3] = marc8_string[pos + 2];
                    code_point = u32::from_le_bytes(bytes);
                }
            } else {
                code_point = marc8_string[pos] as u32;
                pos += 1;
            }

            if code_point < 0x20 || code_point > 0x80 && code_point < 0xA0 {
                if let Some(uni_list) = self.uni_list.as_mut() {
                    if let Some(ch) = char::from_u32(code_point) {
                        uni_list.push(ch);
                    }
                }
            } else if code_point > 0x80 && !mb_flag {
                if let Some(charset) = codesets().get(&self.g1) {
                    self.handle_codepoint(charset, code_point);
                }
            } else {
                if let Some(charset) = codesets().get(&self.g0) {
                    self.handle_codepoint(charset, code_point);
                } else {
                    if let Some(val) = odd_map().get(&(code_point)) {
                        self.uni_list.as_mut().map(|v| v.push(*val));
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
            }
        }

        if let Some(v) = self.uni_list.to_owned() {
            return Ok(Cow::Owned(String::from_iter(v)));
        } else {
            Err(EncodingError::NoData)
        }
    }

    fn handle_codepoint(&mut self, charset: &HashMap<u32, (char, bool)>, code_point: u32) {
        if let Some((uni, cflag)) = charset.get(&code_point) {
            if *cflag {
                self.combinings.as_mut().map(|v| v.push(*uni));
            } else {
                self.uni_list.as_mut().map(|v| v.push(*uni));
                if let Some(combinings) = self.combinings.as_mut() {
                    if let Some(&combining) = combinings.iter().next() {
                        self.uni_list.as_mut().map(|v| v.push(combining));
                        self.combinings.as_mut().map(|v| v.clear());
                    }
                }
            }
        }
    }
}

#[pyclass]
struct MARC8ToUnicode(Decoder);

#[pymethods]
impl MARC8ToUnicode {
    #[new]
    #[pyo3(signature = (g0 = None, g1 = None, quiet = None))]
    fn new(g0: Option<u8>, g1: Option<u8>, quiet: Option<bool>) -> Self {
        Self(Decoder::new(g0, g1, quiet))
    }

    fn translate(&mut self, marc8_string: String) -> String {
        self.0.decode(marc8_string.as_bytes()).unwrap().to_string()
    }
}

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
        let want = "Conversação".to_string();
        let got = converter.decode(b"Conversa\xF0c\xE4ao").unwrap();
        assert_eq!(got, want);
    }
}
