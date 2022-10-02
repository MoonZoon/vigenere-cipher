use indexmap::IndexSet;

pub type Result = std::result::Result<String, Error>;

#[derive(Clone, Copy)]
pub enum Error {
    InvalidMessageChar(char),
    InvalidKeyChar(char),
}

pub struct Cipher {
    dictionary: IndexSet<char>,
    key_chars: Vec<char>,
}

impl Cipher {
    pub fn new(dictionary: &str, key: &str) -> Self {
        Self {
            dictionary: dictionary.chars().chain(['\n', '\r']).collect(),
            key_chars: key.chars().collect(),
        }
    }

    pub fn encode(&self, message: &str) -> Result {
        self.code(message, true)
    }

    pub fn decode(&self, encoded_message: &str) -> Result {
        self.code(encoded_message, false)
    }

    fn code(&self, message: &str, encode: bool) -> Result {
        // Alg: https://www.javatpoint.com/vigenere-cipher
        message
            .chars()
            .enumerate()
            .map(|(index, message_char)| {
                let message_char_index = self
                    .dictionary
                    .get_index_of(&message_char)
                    .ok_or(Error::InvalidMessageChar(message_char))?;

                let key_char = self.key_chars[index % self.key_chars.len()];
                let key_char_index = self
                    .dictionary
                    .get_index_of(&key_char)
                    .ok_or(Error::InvalidKeyChar(key_char))?;

                let coded_char_index = {
                    if encode {
                        message_char_index + key_char_index
                    } else if message_char_index >= key_char_index {
                        message_char_index - key_char_index
                    } else {
                        self.dictionary.len() - (key_char_index - message_char_index)
                    }
                } % self.dictionary.len();
                let coded_char = self.dictionary[coded_char_index];
                Ok(coded_char)
            })
            .collect()
    }
}
