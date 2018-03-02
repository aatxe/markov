# markov [![Build Status][ci-badge]][ci] [![Crates.io][cr-badge]][cr] [![Docs][doc-badge]][doc] [![Built with Spacemacs][bws]][sm]

[ci-badge]: https://travis-ci.org/aatxe/markov.svg
[ci]: https://travis-ci.org/aatxe/markov
[cr-badge]: https://img.shields.io/crates/v/markov.svg
[cr]: https://crates.io/crates/markov
[doc-badge]: https://docs.rs/markov/badge.svg
[doc]: https://docs.rs/markov
[bws]: https://cdn.rawgit.com/syl20bnr/spacemacs/442d025779da2f62fc86c2082703697714db6514/assets/spacemacs-badge.svg
[sm]: http://spacemacs.org

A generic implementation of a [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) in Rust.
You can find up-to-date, ready-to-use documentation online [on docs.rs][doc].

## Examples ##

With Strings: 
```rust,no_run
extern crate markov;

use markov::Chain;

fn main() {
    let mut chain = Chain::new();
    chain.feed_str("I like cats and I like dogs.");
    println!("{:?}", chain.generate_str());
}
```

With integers:
```rust,no_run
extern crate markov;

use markov::Chain;

fn main() {
    let mut chain = Chain::new();
    chain.feed(vec![1u8, 2, 3, 5]).feed(vec![3u8, 9, 2]);
    println!("{:?}", chain.generate());
}
```

Chains have iterators (both infinite and sized!):
```rust,no_run
extern crate markov;

use markov::Chain;

fn main() {
    let mut chain = Chain::new();
    chain.feed_str("I like cats and I like dogs.");
    for line in chain.iter_for(5) {
        println!("{:?}", line);
    }
}
```

Chains can be higher-order:
```rust,no_run
extern crate markov;

use markov::Chain;

fn main() {
    let mut chain = Chain::of_order(2);
    chain.feed_str("I like cats and I like dogs.");
    for line in chain.iter_for(5) {
        println!("{:?}", line);
    }
}
```

## Contributing ##
Contributions to this library would be immensely appreciated. It should be noted that as this is a
public domain project, any contributions will thus be released into the public domain as well.
