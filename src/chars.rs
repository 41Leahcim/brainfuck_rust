/// Converts a byte iterator to a char iterator
pub struct Chars<T: Iterator>(T);

impl<T: Iterator<Item = u8>> From<T> for Chars<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Iterator<Item = u8>> Iterator for Chars<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // Stores the value of the char if valid
        let mut value = 0;

        // A utf-8 char can't be larger than 32-bit
        for byte in self.0.by_ref().take(4) {
            // Add the current byte to the char value
            value = (value << 8) + u32::from(byte);

            // Try to convert the value to a character, return it if valid
            if let Some(value) = char::from_u32(value) {
                return Some(value);
            }
        }

        // No character was found
        None
    }
}
