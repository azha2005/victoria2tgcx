//! Borrowed native MSVC x86 string layout; first24 bytes only are engine input.
#[repr(C)]
pub struct GameString {
    data: [u8; 16],
    length: u32,
    capacity: u32,
    _bytes: Vec<u8>,
}
impl GameString {
    pub fn new(text: &str) -> Self {
        let mut bytes = text.chars().map(encode_char).take(2048).collect::<Vec<_>>();
        bytes.push(0);
        let len = bytes.len() - 1;
        let mut result = Self {
            data: [0; 16],
            length: len as u32,
            capacity: 15,
            _bytes: bytes,
        };
        if len < 16 {
            result.data[..=len].copy_from_slice(&result._bytes);
        } else {
            result.data[..4].copy_from_slice(&(result._bytes.as_ptr() as u32).to_le_bytes());
            result.capacity = len as u32;
        }
        result
    }
}
fn encode_char(c: char) -> u8 {
    match c {
        '€' => 0x80,
        '‘' => 0x91,
        '’' => 0x92,
        '“' => 0x93,
        '”' => 0x94,
        '–' => 0x96,
        '—' => 0x97,
        _ if c as u32 <= 255 => c as u8,
        _ => b'?',
    }
}
