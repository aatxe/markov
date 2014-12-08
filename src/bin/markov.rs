extern crate markov;

use markov::Chain;

fn main() {
    let mut chain = Chain::new("START".into_string(), "END".into_string());
    chain.feed_file(&Path::new("corpus"));
    println!("{}", chain.generate_str())
}
