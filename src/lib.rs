#[macro_use]
extern crate lazy_static;
extern crate petgraph;

use petgraph::graphmap::DiGraphMap;
use petgraph::dot::{Dot, Config};

#[derive(Hash, Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Key {
    pub value: char,
    pub shifted: char,
}


pub trait KeySearch {
    fn find_key(&self, v: char) -> Option<Key>;
}


impl KeySearch for DiGraphMap<Key, Edge> {
    fn find_key(&self, v: char) -> Option<Key> {
        self.nodes().filter(|x| x.value == v || x.shifted == v).nth(0)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Previous = -1,
    Next = 1,
    Same = 0,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Edge {
    pub horizontal: Direction,
    pub vertical: Direction,
}

#[derive(PartialEq)]
enum KeyboardStyle {
    Slanted,
    Aligned,
}

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


lazy_static! {
    pub static ref QWERTY_US: DiGraphMap<Key, Edge> = generate_qwerty_us();
    pub static ref DVORAK: DiGraphMap<Key, Edge> = generate_dvorak(); 
    pub static ref STANDARD_NUMPAD: DiGraphMap<Key, Edge> = generate_standard_numpad();
    pub static ref MAC_NUMPAD: DiGraphMap<Key, Edge> = generate_mac_numpad();
}

#[test]
fn test_qwerty_us() {
    // Printing out graphviz for debugging
    println!("{:?}",
             Dot::with_config(&*QWERTY_US, &[Config::EdgeNoLabel]));
}

static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";
static NUMBERS: &'static str = "0123456789";


/// Function to add all alphabet characters to keyboard. (a-z & A-Z).
/// With qwerty and dvorak unshifted is lowercase and shifted is uppercase so
/// these keys are common.
fn add_alphabetics(graph: &mut DiGraphMap<Key, Edge>) {
    for c in ALPHABET.chars() {
        graph.add_node(Key {
            value: c,
            shifted: c.to_uppercase().nth(0).unwrap(),
        });
    }
}


/// Numpads typically have no shift modifiers so use this function to populate
/// the numeric keys.
fn add_unshifted_number_keys(graph: &mut DiGraphMap<Key, Edge>) {

    for c in NUMBERS.chars() {
        graph.add_node(Key {
            value: c,
            shifted: '\0',
        });
    }
}


fn connect_keyboard_nodes(keyboard: &str,
                          graph: &mut DiGraphMap<Key, Edge>,
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


fn add_remaining_keys(keys: Vec<Key>, graph: &mut DiGraphMap<Key, Edge>) {

    for k in keys.iter() {
        graph.add_node(k.clone());
    }
}


fn generate_qwerty_us() -> DiGraphMap<Key, Edge> {
    let mut result = DiGraphMap::<Key, Edge>::new();
    // This is a bit nasty but I don't see how to do it nicer..
    // Trailing space after \n represents keyboard offset.
    let qwerty_us = "` 1 2 3 4 5 6 7 8 9 0 - =\n\0 q w e r t y u i o p [ ] \\\n\0 a s d f g h j k \
                     l ; '\n\0 z x c v b n m , . /";

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


fn generate_dvorak() -> DiGraphMap<Key, Edge> {
    let mut result = DiGraphMap::<Key, Edge>::new();
    // This is a bit nasty but I don't see how to do it nicer..
    // Trailing space after \n represents keyboard offset.
    let qwerty_us = "` 1 2 3 4 5 6 7 8 9 0 [ ]\n\0 ' , . p y f g c r l / = \\\n\0 a o e u i d h t \
                     n s -\n\0 ; q j k x b m w v z";

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

fn generate_standard_numpad() -> DiGraphMap<Key, Edge> {
    let mut result = DiGraphMap::<Key, Edge>::new();
    let numpad = "\0 / * -\n7 8 9 +\n4 5 6\n1 2 3\n\0 0 .";

    add_unshifted_number_keys(&mut result);

    connect_keyboard_nodes(numpad, &mut result, KeyboardStyle::Aligned, true);
    result
}


#[test]
fn test_standard_numpad() {
    let g = generate_standard_numpad();
    println!("{:?}",
             Dot::with_config(&g.into_graph::<u32>(), &[Config::EdgeNoLabel]));

}

fn generate_mac_numpad() -> DiGraphMap<Key, Edge> {
    let mut result = DiGraphMap::<Key, Edge>::new();
    let numpad = "\0 = / *\n7 8 9 -\n4 5 6 +\n1 2 3\n\0 0 .";

    connect_keyboard_nodes(numpad, &mut result, KeyboardStyle::Aligned, true);
    result
}
