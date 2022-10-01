use indexmap::IndexSet;
use zoon::*;

pub type Result = std::result::Result<String, Error>;

#[derive(Clone, Copy)]
pub enum Error {
    InvalidMessageChar(char),
    InvalidKeyChar(char),
}

#[static_ref]
fn dictionary() -> &'static IndexSet<char> {
    super::DICTIONARY.chars().chain(['\n', '\r']).collect()
}

#[static_ref]
fn key_chars() -> &'static Vec<char> {
    super::KEY.chars().collect()
}

pub fn encode(message: &str) -> Result {
    code(message, true)
}

pub fn decode(encoded_message: &str) -> Result {
    code(encoded_message, false)
}

fn code(message: &str, encode: bool) -> Result {
    // Alg: https://www.javatpoint.com/vigenere-cipher
    message
        .chars()
        .enumerate()
        .map(|(index, message_char)| {
            let message_char_index = dictionary()
                .get_index_of(&message_char)
                .ok_or(Error::InvalidMessageChar(message_char))?;

            let key_char = key_chars()[index % key_chars().len()];
            let key_char_index = dictionary()
                .get_index_of(&key_char)
                .ok_or(Error::InvalidKeyChar(key_char))?;

            let coded_char_index = {
                if encode {
                    message_char_index + key_char_index
                } else if message_char_index >= key_char_index {
                    message_char_index - key_char_index
                } else {
                    dictionary().len() - (key_char_index - message_char_index)
                }
            } % dictionary().len();
            let coded_char = dictionary()[coded_char_index];
            Ok(coded_char)
        })
        .collect()
}
