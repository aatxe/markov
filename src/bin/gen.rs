extern crate markov;

use markov::Chain;

fn main() {
    println!("initializing...");
    let mut chain = Chain::new("START".into_string(), "END".into_string());
    println!("feeding data...");
    chain.feed_file(&Path::new("corpus"));
    println!("generating...");
    println!("{}", chain.generate_str());
}
