#[macro_use]
extern crate lazy_static;
extern crate petgraph;

pub use petgraph::graphmap::DiGraphMap;

pub type Keyboard = DiGraphMap<Key, Edge>;

/// Datatype for graph nodes representing a key on the keyboard.
#[derive(Hash, Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Key {
    /// Value of the key
    pub value: char, 
    /// Value when shift is pressed
    pub shifted: char,
}

impl Key {
    pub fn is_shifted(&self, val: char) -> bool {
        self.shifted == val && val != '\0'
    }

    pub fn is_unshifted(&self, val: char) -> bool {
        self.value == val && val != '\0'
    }
}

#[test]
fn is_shifted_test() {
    let t = Key { 
        value: 'a',
        shifted: 'A'
    };
    assert!(t.is_shifted('A'));
    assert!(t.is_unshifted('a'));
    assert!(!t.is_shifted('a'));
    assert!(!t.is_unshifted('A'));
    assert!(!(t.is_shifted('Y') || t.is_unshifted('y')));
}

/// Trait to find a key given a single character from it. This function is 
/// useful when you don't know what the locale of the keyboard is as numbers
/// and symbols on a key can change (i.e. UK vs US)
pub trait KeySearch {
    /// Finds the key given a char from it. 
    /// Returns Some(Key) if a key exists else returns None.
    fn find_key(&self, v: char) -> Option<Key>;
}

/// Implementation of KeySearch for the graph used to hold keys
impl KeySearch for DiGraphMap<Key, Edge> {
    fn find_key(&self, v: char) -> Option<Key> {
        if v == '\0' {
            None
        } else {
            self.nodes().filter(|x| x.value == v || x.shifted == v).nth(0)
        }
    }
}

/// Enum representing a direction relative to a key on either the horizontal or
/// vertical axis
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    /// Previous refers to above or left to the key 
    Previous = -1, 
    /// Next refers to below or to the right of the key
    Next = 1, 
    /// Same refers to the same row or column as the reference key
    Same = 0, 
}

/// Struct to represent the relative positioning of one key to a neighbouring 
/// key
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Edge {
    /// Relative horizontal position
    pub horizontal: Direction, 
    /// Relative Vertical position
    pub vertical: Direction, 
}

/// Keyboard style. The main part of a keyboard normally applies a slant to the
/// rows meaning that a key only has 6 neighbours, however numpads are aligned
/// meaning that they have more neighbours. This enum allows for distinguishing
/// between physical key layouts
#[derive(PartialEq)]
enum KeyboardStyle {
    /// Keys are slanted with a row offset likely applied
    Slanted, 
    /// Keys are aligned in a clear grid
    Aligned, 
}

/// Returns a vector of the relative positions of the neighbours to a key on a
/// slanted keyboard
fn get_slanted_positions() -> Vec<Edge> {
    use Direction::{Previous, Next, Same};
    vec![ 
        Edge{ horizontal: Previous, vertical: Same },
        Edge{ horizontal: Same, vertical: Previous },
        Edge{ horizontal: Next, vertical: Previous },
        Edge{ horizontal: Next, vertical: Same },
        Edge{ horizontal: Same, vertical: Next },
        Edge{ horizontal: Previous, vertical: Next },
    ]
}

/// Returns a vector of the relative positions of the neighbours to a key on an
/// aligned keyboard
fn get_aligned_positions() -> Vec<Edge> {
    use Direction::{Previous, Next, Same};
    vec![
        Edge{ horizontal: Previous, vertical: Same },
        Edge{ horizontal: Previous, vertical: Previous },
        Edge{ horizontal: Same, vertical: Previous },
        Edge{ horizontal: Next, vertical: Previous },
        Edge{ horizontal: Next, vertical: Same },
        Edge{ horizontal: Next, vertical: Next },
        Edge{ horizontal: Same, vertical: Next },
        Edge{ horizontal: Previous, vertical: Next },
    ]
}

/// Keyboards exported to the user.
lazy_static! {
    pub static ref QWERTY_US: Keyboard = generate_qwerty_us();
    pub static ref QWERTY_UK: Keyboard = generate_qwerty_uk();
    pub static ref DVORAK: Keyboard = generate_dvorak(); 
    pub static ref STANDARD_NUMPAD: Keyboard = generate_standard_numpad();
    pub static ref MAC_NUMPAD: Keyboard = generate_mac_numpad();
}


/// Convenience strings to iterate over.
static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";
static NUMBERS: &'static str = "0123456789";


/// Function to add all alphabet characters to keyboard. (a-z & A-Z).
/// With qwerty and dvorak unshifted is lowercase and shifted is uppercase so
/// these keys are common.
///
/// This function takes a graph representing the keyboard as an argument so it
/// can insert the nodes
fn add_alphabetics(graph: &mut Keyboard) {
    for c in ALPHABET.chars() {
        graph.add_node(Key {
            value: c,
            shifted: c.to_uppercase().nth(0).unwrap(),
        });
    }
}

/// Numpads typically have no shift modifiers so use this function to populate
/// the numeric keys.
/// 
/// This function takes a graph representing the keyboard as an argument so it
/// can insert the nodes
fn add_unshifted_number_keys(graph: &mut Keyboard) {

    for c in NUMBERS.chars() {
        graph.add_node(Key {
            value: c,
            shifted: '\0',
        });
    }
}


/// Given string representation of the keyboard and it's rows and a graph of
/// nodes this function connects the edges between the nodes. 
/// 
/// * keyboard - string representation of the keyboard. Use line breaks to 
///     separate rows, spaces to delimit chars and \0 on a row to represent
///     a void area on the keyboard (lines up keys when keys are slanted)
/// * graph - graph storing the keyboard adjacency graph
/// * style - enum representing alignment of keys
/// * add_missing_keys - whether missing keys should be added to the graph or 
///     ignored
fn connect_keyboard_nodes(keyboard: &str,
                          graph: &mut Keyboard,
                          style: KeyboardStyle,
                          add_missing_keys: bool) {

    let relative_positions = if style == KeyboardStyle::Slanted {
        get_slanted_positions()
    } else {
        get_aligned_positions()
    };
    let rows = keyboard.lines()
                       .map(|x| x.chars().filter(|y| y != &' ').collect::<Vec<char>>())
                       .collect::<Vec<Vec<char>>>();

    let rowcount = rows.iter().count() as i32;
    for (i, row) in rows.iter().enumerate() {
        for (j, key) in row.iter().enumerate() {
            // Get the adjacent keys now
            let k = graph.find_key(*key);
            if k.is_none() && !add_missing_keys {
                continue;
            }
            let k = if k.is_some() {
                k.unwrap()
            } else {
                Key {
                    value: *key,
                    shifted: '\0',
                }
            };

            for dir in relative_positions.iter() {
                let y: i32 = i as i32 + dir.vertical as i32;
                let x: i32 = j as i32 + dir.horizontal as i32;
                if y > -1 && y < rowcount && x > -1 {
                    let temp_row = if dir.vertical == Direction::Same {
                        row
                    } else {
                        rows.get(y as usize).unwrap()
                    };

                    if let Some(temp_char) = temp_row.get(x as usize) {

                        let n = graph.find_key(*temp_char);
                        
                        if n.is_none() && !add_missing_keys {
                            continue;
                        }

                        let n = if n.is_some() {
                            n.unwrap()
                        } else {
                            Key {
                                value: *temp_char,
                                shifted: '\0',
                            }
                        };
            
                        graph.add_edge(k, n, *dir);
                    }
                }
            }
        }
    }
}

/// Any keys the user wants to specify that aren't populated by another function
/// should be added here.
fn add_remaining_keys(keys: Vec<Key>, graph: &mut Keyboard) {

    for k in keys.iter() {
        graph.add_node(k.clone());
    }
}

/// Generates the graph for the qwerty US keyboard layout
pub fn generate_qwerty_us() -> Keyboard {
    let mut result = DiGraphMap::<Key, Edge>::new();
    // This is a bit nasty but I don't see how to do it nicer..
    // Trailing space after \n represents keyboard offset.
    let qwerty_us = "` 1 2 3 4 5 6 7 8 9 0 - =\n\
                     \0 q w e r t y u i o p [ ] \\\n\
                     \0 a s d f g h j k l ; '\n\
                     \0 z x c v b n m , . /";

    add_alphabetics(&mut result);

    let remaining_keys = vec![ 
        Key{ value: '`', shifted: '~'},
        Key{ value: '1', shifted: '!'},
        Key{ value: '2', shifted: '@'},
        Key{ value: '3', shifted: '#'},
        Key{ value: '4', shifted: '$'},
        Key{ value: '5', shifted: '%'},
        Key{ value: '6', shifted: '^'},
        Key{ value: '7', shifted: '&'},
        Key{ value: '8', shifted: '*'},
        Key{ value: '9', shifted: '('},
        Key{ value: '0', shifted: ')'},
        Key{ value: '-', shifted: '_'},
        Key{ value: '=', shifted: '+'},
        Key{ value: '[', shifted: '{'},
        Key{ value: ']', shifted: '}'},
        Key{ value: '\\', shifted: '|'},
        Key{ value: ';', shifted: ':'},
        Key{ value: '\'', shifted: '\"'},
        Key{ value: ',', shifted: '<'},
        Key{ value: '.', shifted: '>'},
        Key{ value: '/', shifted: '?'}
    ];
    add_remaining_keys(remaining_keys, &mut result);

    connect_keyboard_nodes(qwerty_us, &mut result, KeyboardStyle::Slanted, false);

    result
}

/// Generates the graph for the qwerty US keyboard layout
pub fn generate_qwerty_uk() -> Keyboard {
    let mut result = DiGraphMap::<Key, Edge>::new();
    // This is a bit nasty but I don't see how to do it nicer..
    // Trailing space after \n represents keyboard offset.
    let qwerty_uk = "` 1 2 3 4 5 6 7 8 9 0 - =\n\
                     \0 q w e r t y u i o p [ ] \\\n\
                     \0 a s d f g h j k l ; ' #\n\
                     \0 z x c v b n m , . /";

    add_alphabetics(&mut result);

    let remaining_keys = vec![ 
        Key{ value: '`', shifted: '¬'},
        Key{ value: '1', shifted: '!'},
        Key{ value: '2', shifted: '\"'},
        Key{ value: '3', shifted: '£'},
        Key{ value: '4', shifted: '$'},
        Key{ value: '5', shifted: '%'},
        Key{ value: '6', shifted: '^'},
        Key{ value: '7', shifted: '&'},
        Key{ value: '8', shifted: '*'},
        Key{ value: '9', shifted: '('},
        Key{ value: '0', shifted: ')'},
        Key{ value: '-', shifted: '_'},
        Key{ value: '=', shifted: '+'},
        Key{ value: '[', shifted: '{'},
        Key{ value: ']', shifted: '}'},
        Key{ value: '\\', shifted: '|'},
        Key{ value: ';', shifted: ':'},
        Key{ value: '\'', shifted: '@'},
        Key{ value: ',', shifted: '<'},
        Key{ value: '.', shifted: '>'},
        Key{ value: '/', shifted: '?'},
        Key{ value: '#', shifted: '~'}
    ];
    add_remaining_keys(remaining_keys, &mut result);

    connect_keyboard_nodes(qwerty_uk, &mut result, KeyboardStyle::Slanted, false);

    result
}

/// Generates a graph for the dvorak keyboard layout
pub fn generate_dvorak() -> Keyboard {
    let mut result = DiGraphMap::<Key, Edge>::new();
    // This is a bit nasty but I don't see how to do it nicer..
    // Trailing space after \n represents keyboard offset.
    let qwerty_us = "` 1 2 3 4 5 6 7 8 9 0 [ ]\n\
                      \0 ' , . p y f g c r l / = \\\n\
                      \0 a o e u i d h t n s -\n\
                      \0 ; q j k x b m w v z";

    add_alphabetics(&mut result);

    let remaining_keys = vec![ 
        Key{ value: '`', shifted: '~'},
        Key{ value: '1', shifted: '!'},
        Key{ value: '2', shifted: '@'},
        Key{ value: '3', shifted: '#'},
        Key{ value: '4', shifted: '$'},
        Key{ value: '5', shifted: '%'},
        Key{ value: '6', shifted: '^'},
        Key{ value: '7', shifted: '&'},
        Key{ value: '8', shifted: '*'},
        Key{ value: '9', shifted: '('},
        Key{ value: '0', shifted: ')'},
        Key{ value: '-', shifted: '_'},
        Key{ value: '=', shifted: '+'},
        Key{ value: '[', shifted: '{'},
        Key{ value: ']', shifted: '}'},
        Key{ value: '\\', shifted: '|'},
        Key{ value: ';', shifted: ':'},
        Key{ value: '\'', shifted: '\"'},
        Key{ value: ',', shifted: '<'},
        Key{ value: '.', shifted: '>'},
        Key{ value: '/', shifted: '?'}
    ];
    add_remaining_keys(remaining_keys, &mut result);

    connect_keyboard_nodes(qwerty_us, &mut result, KeyboardStyle::Slanted, false);

    result
}

/// Generates a standard numpad.
pub fn generate_standard_numpad() -> Keyboard {
    let mut result = DiGraphMap::<Key, Edge>::new();
    let numpad = "\0 / * -\n7 8 9 +\n4 5 6\n1 2 3\n\0 0 .";

    add_unshifted_number_keys(&mut result);

    connect_keyboard_nodes(numpad, &mut result, KeyboardStyle::Aligned, true);
    result
}


/// Generates the Apple Mac style numpad
pub fn generate_mac_numpad() -> Keyboard {
    let mut result = DiGraphMap::<Key, Edge>::new();
    let numpad = "\0 = / *\n7 8 9 -\n4 5 6 +\n1 2 3\n\0 0 .";

    connect_keyboard_nodes(numpad, &mut result, KeyboardStyle::Aligned, true);
    result
}


#[test]
fn test_alphabetics() {
    assert_eq!(ALPHABET.chars().count(), 26);
    
    let mut result = DiGraphMap::<Key, Edge>::new();
    add_alphabetics(&mut result);

    let uppercase = ALPHABET.to_uppercase();
    for (l, u) in ALPHABET.chars().zip(uppercase.chars()) {
        let test = Key {
            value: l,
            shifted: u
        };
        assert!(result.contains_node(test));
        // Get testing of trait for free
        assert!(result.find_key(l).is_some());
        assert!(result.find_key(u).is_some());
    }
}

#[test]
fn test_add_number_keys() {
    assert_eq!(NUMBERS.chars().count(), 10);
    
    let mut result = DiGraphMap::<Key, Edge>::new();
    add_unshifted_number_keys(&mut result);
    for c in NUMBERS.chars() {
        let test = Key {
            value: c,
            shifted: '\0'
        };
        assert!(result.contains_node(test));
        assert!(result.find_key(c).is_some());
    }
    assert!(result.find_key('\0').is_none());
}
