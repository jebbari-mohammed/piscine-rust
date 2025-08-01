#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut expected = String::new();
    
    for c in original.chars() {
        if c >= 'a' && c <= 'z' {
            let position = c as u8 - b'a';
            let mirror_position = 25 - position;
            let mirror_char = (b'a' + mirror_position) as char;
            expected.push(mirror_char);
        } else if c >= 'A' && c <= 'Z' {
            let position = c as u8 - b'A';
            let mirror_position = 25 - position;
            let mirror_char = (b'A' + mirror_position) as char;
            expected.push(mirror_char);
        } else {
            expected.push(c);
        }
    }
    
    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}