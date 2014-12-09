#![cfg(not(test))] 
extern crate markov;

use markov::Chain;

fn main() {
    println!("initializing...");
    let mut chain = Chain::for_strings();
    println!("feeding data...");
    chain.feed_file(&Path::new("corpus"));
    println!("generating...");
    println!("{}", chain.generate_str());
}
