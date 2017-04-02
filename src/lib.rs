

#[derive(Hash)]
pub struct Key {
    value: char,
    shifted: char,
}

impl PartialEq for Key {

    fn eq(&self, other: &Key) -> bool {
        if other.shifted == '\0' || self.shifted == '\0' {
            self.value == other.value
        } else {
            self.value == other.value && 
                self.shifted == other.shifted
        }
    }
}

