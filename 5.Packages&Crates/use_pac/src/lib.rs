mod front {
    pub mod hosting {
        pub fn add() {}
    }
}

use crate::front::hosting;

pub fn eat() {
    hosting::add();
}

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

pub use std::io::Result as IoResult;

use ::{ cmp::Ordering, io };

use std::io::{ self, Write };

use std::collections::*;
