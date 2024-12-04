pub const BASIC_LATIN: u8 = 0x42;
pub const REDESIGNATED_ASCII: u8 = 0x73;
pub const ANSEL: u8 = 0x45;
pub const G0_SET: [u8; 3] = [b'(', b',', b'$'];
pub const G1_SET: [u8; 3] = [b')', b'-', b'$'];
pub const BLANK: char = ' ';
pub const MULTI_BYTE: u8 = 0x31;
const ESCAPE: u8 = 0x1b;

pub const fn is_escape(c: u8) -> bool {
    c == ESCAPE
}
