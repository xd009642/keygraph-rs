extern crate keygraph_rs;
extern crate petgraph;

use petgraph::Direction;
use keygraph_rs::*;


#[test]
fn test_keyboard_contents() {
    let characters = "`~1!2\"34$5%^67&8*9(0)-_=+[{]};:'@#~,<.>/?\\|";
    let alphabet = "abcdefghijklmnopqrstuvqwxyz";

    let relevant_keyboards = vec![
        generate_qwerty_us(),
        generate_dvorak()
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
    let qwerty_us = generate_qwerty_us();
    // testing 'g'
    let reference_key = qwerty_us.find_key('g');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        qwerty_us.find_key('f').unwrap(),
        qwerty_us.find_key('h').unwrap(),
        qwerty_us.find_key('t').unwrap(),
        qwerty_us.find_key('y').unwrap(),
        qwerty_us.find_key('v').unwrap(),
        qwerty_us.find_key('b').unwrap()
    ];
    
    let actual = qwerty_us.neighbors_directed(reference_key, Direction::Incoming)
                          .collect::<Vec<_>>();

    test_neighbours(expected, actual);

    // testing '`'
    let reference_key = qwerty_us.find_key('`');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        qwerty_us.find_key('1').unwrap()
    ];
    
    let actual = qwerty_us.neighbors_directed(reference_key, Direction::Incoming)
                          .collect::<Vec<_>>();

    test_neighbours(expected, actual);

    // testing 'c'
    let reference_key = qwerty_us.find_key('c');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        qwerty_us.find_key('x').unwrap(),
        qwerty_us.find_key('v').unwrap(),
        qwerty_us.find_key('d').unwrap(),
        qwerty_us.find_key('f').unwrap(),
    ];
    
    let actual = qwerty_us.neighbors_directed(reference_key, Direction::Incoming)
                          .collect::<Vec<_>>();

    test_neighbours(expected, actual);
}

#[test]
fn test_qwerty_uk() {
    let qwerty_uk = generate_qwerty_uk();
    // testing 'g'
    let reference_key = qwerty_uk.find_key('g');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        qwerty_uk.find_key('f').unwrap(),
        qwerty_uk.find_key('h').unwrap(),
        qwerty_uk.find_key('t').unwrap(),
        qwerty_uk.find_key('y').unwrap(),
        qwerty_uk.find_key('v').unwrap(),
        qwerty_uk.find_key('b').unwrap()
    ];
    
    let actual = qwerty_uk.neighbors_directed(reference_key, Direction::Incoming)
                          .collect::<Vec<_>>();

    test_neighbours(expected, actual);

    // testing '`'
    let reference_key = qwerty_uk.find_key('`');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        qwerty_uk.find_key('1').unwrap()
    ];
    
    let actual = qwerty_uk.neighbors_directed(reference_key, Direction::Incoming)
                          .collect::<Vec<_>>();

    test_neighbours(expected, actual);

    // testing 'c'
    let reference_key = qwerty_uk.find_key('c');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        qwerty_uk.find_key('x').unwrap(),
        qwerty_uk.find_key('v').unwrap(),
        qwerty_uk.find_key('d').unwrap(),
        qwerty_uk.find_key('f').unwrap(),
    ];
    
    let actual = qwerty_uk.neighbors_directed(reference_key, Direction::Incoming)
                          .collect::<Vec<_>>();

    test_neighbours(expected, actual);

    // testing '6'
    let reference_key = qwerty_uk.find_key('6');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        qwerty_uk.find_key('%').unwrap(),
        qwerty_uk.find_key('&').unwrap(),
        qwerty_uk.find_key('t').unwrap(),
        qwerty_uk.find_key('y').unwrap(),
    ];
    
    let actual = qwerty_uk.neighbors_directed(reference_key, Direction::Incoming)
                          .collect::<Vec<_>>();

    test_neighbours(expected, actual);
}

#[test]
fn test_dvorak() {
    let dvorak = generate_dvorak();
    // testing 'y'
    let reference_key = dvorak.find_key('y');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        dvorak.find_key('p').unwrap(),
        dvorak.find_key('5').unwrap(),
        dvorak.find_key('6').unwrap(),
        dvorak.find_key('f').unwrap(),
        dvorak.find_key('i').unwrap(),
        dvorak.find_key('u').unwrap()
    ];
    
    let actual = dvorak.neighbors_directed(reference_key, Direction::Incoming)
                       .collect::<Vec<_>>();

    test_neighbours(expected, actual);

    // testing '~'
    let reference_key = dvorak.find_key('~');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        dvorak.find_key('1').unwrap()
    ];
    
    let actual = dvorak.neighbors_directed(reference_key, Direction::Incoming)
                       .collect::<Vec<_>>();

    test_neighbours(expected, actual);

    // testing 'z'
    let reference_key = QWERTY_US.find_key('z');
    assert!(reference_key.is_some());
    let reference_key = reference_key.unwrap();

    let expected = vec![
        dvorak.find_key('v').unwrap(),
        dvorak.find_key('s').unwrap(),
        dvorak.find_key('-').unwrap()
    ];
    
    let actual = dvorak.neighbors_directed(reference_key, Direction::Incoming)
                       .collect::<Vec<_>>();

    test_neighbours(expected, actual);

}
