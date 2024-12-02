mod charsets;
mod constants;
mod error;
mod mappings;

use crate::constants::*;
use crate::error::EncodingError;
use crate::mappings::{codesets, odd_map};
use core::str;
use std::borrow::Cow;

pub struct Decoder {
    g0: u8,
    g1: u8,
    quiet: bool,
}

impl Decoder {
    pub fn new(g0: Option<u8>, g1: Option<u8>, quiet: Option<bool>) -> Self {
        let g0 = g0.unwrap_or(BASIC_LATIN);
        let g1 = g1.unwrap_or(ANSEL);
        let quiet = quiet.unwrap_or(false);

        Decoder { g0, g1, quiet }
    }

    #[allow(unused_assignments)]
    pub fn decode<'a>(&mut self, marc8_string: &'a [u8]) -> Result<Cow<'a, str>, EncodingError> {
        if marc8_string.is_empty() {
            return Ok(Cow::Borrowed(""));
        }

        if marc8_string.iter().all(|ch| ch.is_ascii()) {
            return Ok(Cow::Borrowed(str::from_utf8(marc8_string)?));
        }

        let mut uni_list: Vec<u16> = Vec::new();
        let mut combinings: Vec<u16> = Vec::new();
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
                        continue;
                    } else {
                        uni_list.push(marc8_string[pos] as u16);
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
            let mut code_point: u32 = 0;
            if mb_flag {
                if marc8_string.len() < pos + 3 {
                    eprintln!(
                        "Multi-byte position {} exceeds length of marc8 string {}",
                        pos + 3,
                        marc8_string.len()
                    );
                    code_point = BLANK as u32;
                }
            } else {
                code_point = marc8_string[pos] as u32;
                pos += 1;
            }

            let mut uni: u16 = 0;
            let mut cflag = false;
            if code_point < 0x20 || code_point > 0x80 && code_point < 0xA0 {
                uni = code_point as u16;
                continue;
            } else if code_point > 0x80 && !mb_flag {
                if let Some(charset) = codesets().get(&self.g1) {
                    if let Some((uni_ref, cflag_ref)) = charset.get(&code_point) {
                        uni = *uni_ref;
                        cflag = *cflag_ref;
                    }
                }
            } else {
                if let Some(charset) = codesets().get(&self.g0) {
                    if let Some((uni_ref, cflag_ref)) = charset.get(&code_point) {
                        uni = *uni_ref;
                        cflag = *cflag_ref;
                    }
                } else {
                    if let Some(val) = odd_map().get(&(code_point as u32)) {
                        uni_list.push(*val as u16);
                        continue;
                    } else {
                        if !self.quiet {
                            eprintln!(
                                "Unable to parse character 0x{:X} in g0={} g1={}",
                                code_point, self.g0, self.g1
                            );
                        }
                        uni = BLANK as u16;
                        cflag = false;
                    }
                }
            }

            if cflag {
                combinings.push(uni);
            } else {
                uni_list.push(uni);
                if combinings.len() > 0 {
                    uni_list.extend(combinings.iter());
                    combinings.clear();
                }
            }
        }

        let uni_string = Cow::Owned(String::from_utf16(&uni_list)?);
        Ok(uni_string)
    }
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
