#[macro_use]
extern crate lazy_static;
extern crate petgraph;

use std::collections::HashMap;
use petgraph::graph::Graph;
use petgraph::prelude::NodeIndex;
use petgraph::dot::{Dot, Config};

#[derive(Hash, Debug, Clone)]
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


lazy_static! {
    pub static ref QWERTY_US: Graph<Key, Edge> = generate_qwerty_us();
    
    pub static ref STANDARD_NUMPAD: Graph<Key, Edge> = generate_standard_numpad();
}

#[test]
fn test_qwerty_us() {
    // Printing out graphviz for debugging
    println!("{:?}", Dot::with_config(&*QWERTY_US, &[Config::EdgeNoLabel]));
}

static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";
static NUMBERS: &'static str = "0123456789";


/// Function to add all alphabet characters to keyboard. (a-z & A-Z).
/// With qwerty and dvorak unshifted is lowercase and shifted is uppercase so
/// these keys are common.
fn add_alphabetics(graph: &mut Graph<Key, Edge>) -> HashMap<char, NodeIndex> {
    let mut index_map = HashMap::<char, NodeIndex>::new();
    for c in ALPHABET.chars() {
        let index = graph.add_node(Key { 
            value: c, 
            shifted: c.to_uppercase().nth(0).unwrap()
        });
        index_map.insert(c, index);
    }
    index_map
}


/// Numpads typically have no shift modifiers so use this function to populate
/// the numeric keys.
fn add_unshifted_number_keys(graph: &mut Graph<Key, Edge>) -> HashMap<char, NodeIndex> {
    let mut index_map = HashMap::<char, NodeIndex>::new();
    for c in NUMBERS.chars() {
        let index = graph.add_node(Key {
            value: c,
            shifted: '\0'
        });
        index_map.insert(c, index);
    }
    index_map
}


fn join_qwerty_us(graph: &mut Graph<Key, Edge>, 
                  indexes: &HashMap<char, NodeIndex>) {
    // This is a bit nasty but I don't see how to do it nicer..
    // Trailing space after \n represents keyboard offset.
    let qwerty_us = "` 1 2 3 4 5 6 7 8 9 0 - =\n\
                      \0 q w e r t y u i o p [ ] \\\n\
                      \0 a s d f g h j k l ; '\n\
                      \0 z x c v b n m , . /";

    let relative_positions = get_slanted_positions();

    let rows = qwerty_us.lines()
                        .map(|x| x.chars().filter(|y| y != &' ').collect::<Vec<char>>())
                        .collect::<Vec<Vec<char>>>();
    
    let rowcount = rows.iter().count() as i32;
    for (i, row) in rows.iter().enumerate() {
        for (j, key) in row.iter().enumerate() {
            if !indexes.contains_key(key) {
                continue;
            }
            let current_key = indexes.get(key).unwrap();
            // Get the adjacent keys now
            for dir in relative_positions.iter() {
                let y:i32 = i as i32 + dir.vertical as i32;
                let x:i32 = j as i32 + dir.horizontal as i32;
                if y > -1 && y < rowcount && x > -1 {
                    let temp_row = if dir.vertical == Direction::Same {
                        row
                    } else {
                        rows.get(y as usize).unwrap()
                    };
                    if let Some(temp_char) = temp_row.get(x as usize) {
                        match indexes.get(&temp_char) {
                            Some(ind) => {
                                graph.add_edge(*current_key, *ind, *dir);
                            },
                            None => {},
                        }
                    }
                }        
            }
        }
    }
}


fn generate_qwerty_us() -> Graph<Key, Edge> {
    let mut result = Graph::<Key, Edge>::new();
    
    let mut index_map = add_alphabetics(&mut result);
    
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

    for k in remaining_keys.iter() {
        let index = result.add_node(k.clone());
        index_map.insert(k.value, index);
    }
    join_qwerty_us(&mut result, &index_map);

    result
}


fn generate_standard_numpad() -> Graph<Key, Edge> {
    let mut result = Graph::<Key, Edge>::new();
    
    let mut index_map = add_unshifted_number_keys(&mut result);
    
    result
}
