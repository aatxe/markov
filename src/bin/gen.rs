#![cfg(not(test))] 
#![feature(slicing_syntax)]
extern crate markov;

use std::os::args;
use markov::Chain;

fn main() {
    let mut chain = Chain::for_strings();
    for arg in args().iter().skip(1) {
        chain.feed_file(&Path::new(arg[]));
    }
    println!("{}", chain.generate_str());
}
