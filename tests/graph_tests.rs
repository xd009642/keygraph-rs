extern crate keygraph_rs;
extern crate petgraph;

use petgraph::Direction;
use keygraph_rs::*;


#[test]
fn test_keyboard_contents() {
    let characters = "`~1!2\"34$5%^67&8*9(0)-_=+[{]};:'@#~,<.>/?\\|";
    let alphabet = "abcdefghijklmnopqrstuvqwxyz";

    let relevant_keyboards = vec![
        &*QWERTY_US,
        &*DVORAK
    ];
    for keyboard in relevant_keyboards.iter() {
        for c in characters.chars() {
            println!("{}", c);
            let key = keyboard.find_key(c);
            assert!(key.is_some());
            let key = key.unwrap();
            assert!(key.value ==  c || key.shifted == c);
        }
        for c in alphabet.chars() {
            let key = keyboard.find_key(c);
            let made_key = Key {
                value: c,
                shifted: c.to_uppercase().nth(0).unwrap()
            };

            assert!(key.is_some());
            let key = key.unwrap();
            assert_eq!(key, made_key);
        }
    }
}

fn test_neighbours(expected: Vec<Key>,
                   actual: Vec<Key>) {
    assert_eq!(expected.iter().count(), actual.iter().count());

    for n in actual.iter() {
        let others = expected.iter().filter(|x| **x != *n).count();
        assert_eq!(others, expected.iter().count()-1);
    }
}


#[test]
fn test_qwerty_us() {
    // testing 'g'
    let reference_key = QWERTY_US.find_key('g');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        QWERTY_US.find_key('f').unwrap(),
        QWERTY_US.find_key('h').unwrap(),
        QWERTY_US.find_key('t').unwrap(),
        QWERTY_US.find_key('y').unwrap(),
        QWERTY_US.find_key('v').unwrap(),
        QWERTY_US.find_key('b').unwrap()
    ];
    
    let actual = QWERTY_US.neighbors_directed(reference_key, Direction::Incoming)
                          .collect::<Vec<_>>();

    test_neighbours(expected, actual);

    // testing '`'
    let reference_key = QWERTY_US.find_key('`');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        QWERTY_US.find_key('1').unwrap()
    ];
    
    let actual = QWERTY_US.neighbors_directed(reference_key, Direction::Incoming)
                          .collect::<Vec<_>>();

    test_neighbours(expected, actual);

    // testing 'c'
    let reference_key = QWERTY_US.find_key('c');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        QWERTY_US.find_key('x').unwrap(),
        QWERTY_US.find_key('v').unwrap(),
        QWERTY_US.find_key('d').unwrap(),
        QWERTY_US.find_key('f').unwrap(),
    ];
    
    let actual = QWERTY_US.neighbors_directed(reference_key, Direction::Incoming)
                          .collect::<Vec<_>>();

    test_neighbours(expected, actual);

}

#[test]
fn test_dvorak() {
    // testing 'y'
    let reference_key = DVORAK.find_key('y');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        QWERTY_US.find_key('p').unwrap(),
        QWERTY_US.find_key('5').unwrap(),
        QWERTY_US.find_key('6').unwrap(),
        QWERTY_US.find_key('f').unwrap(),
        QWERTY_US.find_key('i').unwrap(),
        QWERTY_US.find_key('u').unwrap()
    ];
    
    let actual = DVORAK.neighbors_directed(reference_key, Direction::Incoming)
                       .collect::<Vec<_>>();

    test_neighbours(expected, actual);

    // testing '~'
    let reference_key = DVORAK.find_key('~');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        DVORAK.find_key('1').unwrap()
    ];
    
    let actual = DVORAK.neighbors_directed(reference_key, Direction::Incoming)
                       .collect::<Vec<_>>();

    test_neighbours(expected, actual);

    // testing 'z'
    let reference_key = QWERTY_US.find_key('z');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        QWERTY_US.find_key('v').unwrap(),
        QWERTY_US.find_key('s').unwrap(),
        QWERTY_US.find_key('-').unwrap()
    ];
    
    let actual = DVORAK.neighbors_directed(reference_key, Direction::Incoming)
                       .collect::<Vec<_>>();

    test_neighbours(expected, actual);

}
