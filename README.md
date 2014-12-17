# markov [![Build Status](https://travis-ci.org/aaronweiss74/markov.svg?branch=master)](https://travis-ci.org/aaronweiss74/markov) #

A generic implementation of a [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) in Rust. 
Documentation can be found online [here](http://www.rust-ci.org/aaronweiss74/markov/doc/markov/).

## Examples ##

With Strings: 
```rust
extern crate markov;

use markov::Chain;

fn main() {
    let mut chain = Chain::for_strings();
    chain.feed_str("I like cats and I like dogs.");
    println!("{}", chain.generate_str());
}
```

With integers:
```rust
extern crate markov;

use markov::Chain;

fn main() {
    let mut chain = Chain::new(0u8, 255);
    chain.feed(vec![1u8, 2, 3, 5]).feed(vec![3u8, 9, 2]);
    println!("{}", chain.generate());
}
```

Chains have iterators (both infinite and sized!):
```rust
extern crate markov;

use markov::Chain;

fn main() {
    let mut chain = Chain::for_strings();
    chain.feed_str("I like cats and I like dogs.");
    for line in chain.iter_for(5) {
        println!("{}", line);
    }
}
```

## Contributing ##
Contributions to this library would be immensely appreciated. As this project is public domain, 
all prospective contributors must 
[sign the Contributor License Agreement](https://www.clahub.com/agreements/aaronweiss74/markov), a 
public domain dedication.
