use std::hash::Hasher;

pub trait StringHasher {
    fn hash_string_lc(&mut self, string: &str) -> u64;
    fn write_string_lc(&mut self, string: &str);
}

pub fn hash_string_lc<H: Hasher + Default>(string: &str) -> u64 {
    let mut hasher = H::default();

    string
        .bytes()
        .for_each(|c| hasher.write_u8(c.to_ascii_lowercase()));

    hasher.finish()
}

impl<H: Hasher> StringHasher for H {
    fn hash_string_lc(&mut self, string: &str) -> u64 {
        for c in string.chars() {
            for lc in c.to_lowercase() {
                self.write_u8(lc as u8)
            }
        }

        self.finish()
    }

    fn write_string_lc(&mut self, string: &str) {
        for c in string.chars() {
            for lc in c.to_lowercase() {
                self.write_u8(lc as u8)
            }
        }
    }
}
