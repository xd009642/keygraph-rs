

#[derive(Hash)]
pub struct Key {
    pub value: char,
    pub shifted: char,
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

pub enum Direction {
    Previous = -1,
    Next = 1,
}

pub struct Edge {
    pub horizontal: Direction,
    pub vertical: Direction,
}
