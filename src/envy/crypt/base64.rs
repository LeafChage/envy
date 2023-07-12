use base64::{engine::general_purpose, DecodeError, Engine as _};

pub fn encode(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

pub fn decode(data: &str) -> Result<Vec<u8>, DecodeError> {
    general_purpose::STANDARD.decode(data)
}

#[test]
fn it_base64() {
    let data = b"hello world";
    assert_eq!(decode(encode(data).as_ref()), Ok(b"hello world".to_vec()));
}
