extern crate keygraph_rs;
extern crate petgraph;

use keygraph_rs::*;

use petgraph::dot::{Dot, Config};

fn simple() {
    
    let ampersand = QWERTY_US.find_key('h');

    match ampersand {
        Some(_) => println!("Key exists"),
        None => println!("Key doesn't exist in layout"),
    }

    if let Some(key) = ampersand {
        for n in QWERTY_US.neighbors(key) {
            println!("{:?}", n);
        }
    }
    println!("{:?}", Dot::with_config(&*QWERTY_US, &[Config::EdgeNoLabel]));
}


fn main() {
    simple();
}
