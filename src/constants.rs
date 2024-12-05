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
