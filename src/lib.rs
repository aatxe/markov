//! A generic [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) for almost any type.
//! In particular, elements of the chain must be `Eq`, `Hash`, and `Clone`.
//!
//! # Examples
//!
//! ```
//! use markov::Chain;
//!
//! let mut chain = Chain::new();
//! chain.feed_str("I like cats and I like dogs.");
//! println!("{}", chain.generate_str());
//! ```
//!
//! ```
//! use markov::Chain;
//!
//! let mut chain = Chain::new();
//! chain.feed(vec![1u8, 2, 3, 5]).feed([3u8, 9, 2]);
//! println!("{:?}", chain.generate());
//! ```
#![warn(missing_docs)]

#[cfg(feature = "graph")]
extern crate itertools;
#[cfg(feature = "graph")]
extern crate petgraph;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "yaml")]
extern crate serde_yaml;

use std::borrow::ToOwned;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, Result};
#[cfg(feature = "yaml")]
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::iter::Map;
use std::path::Path;

#[cfg(feature = "graph")]
use itertools::Itertools;
#[cfg(feature = "graph")]
use petgraph::graph::Graph;
use rand::{Rng, thread_rng};
#[cfg(feature = "yaml")]
use serde::Serialize;
#[cfg(feature = "yaml")]
use serde::de::DeserializeOwned;
#[cfg(feature = "yaml")]
use serde_yaml as yaml;

/// The definition of all types that can be used in a `Chain`.
pub trait Chainable: Eq + Hash + Clone {}
impl<T> Chainable for T where T: Eq + Hash + Clone {}

type Token<T> = Option<T>;

/// A generic [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) for almost any type.
/// In particular, elements of the chain must be `Eq`, `Hash`, and `Clone`.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Chain<T> where T: Chainable {
    map: HashMap<Vec<Token<T>>, HashMap<Token<T>, usize>>,
    order: usize,
}

impl<T> Chain<T> where T: Chainable {
    /// Constructs a new Markov chain.
    pub fn new() -> Chain<T> {
        Self::of_order(1)
    }

    /// Creates a new Markov chain of the specified order. The order is the number of previous
    /// tokens to use for each mapping in the chain. Higher orders mean that the generated text
    /// will more closely resemble the training set. Increasing the order can yield more realistic
    /// output, but typically at the cost of requiring more training data.
    pub fn of_order(order: usize) -> Chain<T> {
        assert!(order != 0);
        Chain {
            map: {
                let mut map = HashMap::new();
                map.insert(vec!(None; order), HashMap::new());
                map
            },
            order: order,
        }
    }

    /// Determines whether or not the chain is empty. A chain is considered empty if nothing has
    /// been fed into it.
    pub fn is_empty(&self) -> bool {
        self.map[&vec!(None; self.order)].is_empty()
    }


    /// Feeds the chain a collection of tokens. This operation is `O(n)` where `n` is the number of
    /// tokens to be fed into the chain.
    pub fn feed<S: AsRef<[T]>>(&mut self, tokens: S) -> &mut Chain<T> {
        let tokens = tokens.as_ref();
        if tokens.is_empty() { return self }
        let mut toks = vec!(None; self.order);
        toks.extend(tokens.iter().map(|token| {
            Some(token.clone())
        }));
        toks.push(None);
        for p in toks.windows(self.order + 1) {
            self.map.entry(p[0..self.order].to_vec()).or_insert_with(HashMap::new);
            self.map.get_mut(&p[0..self.order].to_vec()).unwrap().add(p[self.order].clone());
        }
        self
    }

    /// Generates a collection of tokens from the chain. This operation is `O(mn)` where `m` is the
    /// length of the generated collection, and `n` is the number of possible states from a given
    /// state.
    pub fn generate(&self) -> Vec<T> {
        let mut ret = Vec::new();
        let mut curs = vec!(None; self.order);
        loop {
            let next = self.map[&curs].next();
            curs = curs[1..self.order].to_vec();
            curs.push(next.clone());
            if let Some(next) = next { ret.push(next) };
            if curs[self.order - 1].is_none() { break }
        }
        ret
    }

    /// Generates a collection of tokens from the chain, starting with the given token. This
    /// operation is O(mn) where m is the length of the generated collection, and n is the number
    /// of possible states from a given state. This returns an empty vector if the token is not
    /// found.
    pub fn generate_from_token(&self, token: T) -> Vec<T> {
        let mut curs = vec!(None; self.order - 1);
        curs.push(Some(token.clone()));
        if !self.map.contains_key(&curs) { return Vec::new() }
        let mut ret = vec![token.clone()];
        loop {
            let next = self.map[&curs].next();
            curs = curs[1..self.order].to_vec();
            curs.push(next.clone());
            if let Some(next) = next { ret.push(next) };
            if curs[self.order - 1].is_none() { break }
        }
        ret
    }


    /// Generates a collection of tokens from the chain, starting with the given tokens. This
    /// operation is O(mn) where m is the length of the generated collection, and n is the number
    /// of possible states from a given state. If not all tokens are in the chain, this returns
    /// the first n tokens that are.
    pub fn generate_from_tokens<S: AsRef<[T]>>(&self, tokens: S) -> Vec<T> {
        let tokens = tokens.as_ref();
        if tokens.is_empty() {
            return vec![];
        }

        // the current key in the hashmap
        let mut curs = vec!(None; self.order - 1);
        curs.push(Some(tokens[0].clone()));

        if !self.map.contains_key(&curs) {
            return vec![];
        }

        let mut ret = vec![tokens[0].clone()];

        // walk up to the provided tokens
        for t in tokens[1..].iter() {
            let next_states = &self.map[&curs];

            // if full tokens input not in chain, bail & return what we have so far
            if !next_states.contains_key(&Some(t.to_owned())) {
                return ret;
            }

            ret.push(t.to_owned());

            // shift cursor
            curs = curs[1..curs.len()].to_owned();
            curs.push(Some(t.to_owned()));
        }

        // now cursor should be full of the last (self.order) tokens from our input

        while let Some(next) = self.map[&curs].next() {
            // shift cursor
            curs = curs[1..self.order].to_owned();
            curs.push(Some(next.clone()));
            // add token
            ret.push(next);
        }
        ret
    }

    /// Produces an infinite iterator of generated token collections.
    pub fn iter(&self) -> InfiniteChainIterator<T> {
        InfiniteChainIterator { chain: self }
    }

    /// Produces an iterator for the specified number of generated token collections.
    pub fn iter_for(&self, size: usize) -> SizedChainIterator<T> {
        SizedChainIterator { chain: self, size: size }
    }

    /// Create a graph using `petgraph` from the markov chain.
    #[cfg(feature = "graph")]
    pub fn graph(&self) -> Graph<Vec<Token<T>>, f64> {
        let mut graph = Graph::new();

        // Create all possible node and store indices into hashmap.
        let state_map = self.map.iter().flat_map(|(state, nexts)| {
            let mut states = vec!(state.clone());

            let mut state = state.clone();
            state.remove(0);

            for next in nexts {
                let mut next_state = state.clone();
                next_state.push(next.0.clone());
                states.push(next_state);
            }

            states
        }).unique()
            .map(|state| (state.clone(), graph.add_node(state)))
            .collect::<HashMap<_, _>>();

        // Create all edges, and add them to the graph.
        self.map.iter().flat_map(|(state, nexts)| {
            let sum = nexts.iter().map(|(_, p)| p).sum::<usize>() as f64;

            nexts.iter()
                .map(|(next, p)| (state.clone(), next.clone(), *p as f64 / sum))
                .collect::<Vec<_>>()
        }).for_each(|(state, next, p)| {
            let mut next_state = state.clone();
            next_state.remove(0);
            next_state.push(next.clone());

            graph.add_edge(state_map[&state], state_map[&next_state], p);
        });

        graph
    }
}

#[cfg(feature = "yaml")]
impl<T> Chain<T> where T: Chainable + Serialize {
    /// Saves the current chain to the specified path.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut file = File::create(&path)?;
        let data = yaml::to_string(self).map_err(|e| {
            Error::new(ErrorKind::InvalidData, e)
        })?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

#[cfg(feature = "yaml")]
impl<T> Chain<T> where T: Chainable + DeserializeOwned {
    /// Loads a chain from the specified path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Chain<T>> {
        let mut file = File::open(&path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        yaml::from_str(&data).map_err(|e| {
            Error::new(ErrorKind::InvalidInput, e)
        })
    }
}

impl Chain<String> {
    /// Feeds a string of text into the chain.
    pub fn feed_str(&mut self, string: &str) -> &mut Chain<String> {
        self.feed(&string.split(' ').map(|s| s.to_owned()).collect::<Vec<_>>())
    }

    /// Feeds a properly formatted file into the chain. This file should be formatted such that
    /// each line is a new sentence. Punctuation may be included if it is desired.
    pub fn feed_file<P: AsRef<Path>>(&mut self, path: P) -> Result<&mut Chain<String>> {
        let reader = BufReader::new(File::open(path)?);
        for line in reader.lines() {
            let line = line?;
            let words = line.split_whitespace()
                            .filter(|word| !word.is_empty())
                            .map(|s| s.to_owned())
                            .collect::<Vec<_>>();
            self.feed(&words);
        }
        Ok(self)
    }

    /// Converts the output of `generate(...)` on a String chain to a single String.
    fn vec_to_string(vec: Vec<String>) -> String {
        let mut ret = String::new();
        for s in &vec {
            ret.push_str(&s);
            ret.push_str(" ");
        }
        let len = ret.len();
        if len > 0 {
            ret.truncate(len - 1);
        }
        ret
    }

    /// Generates a random string of text.
    pub fn generate_str(&self) -> String {
        Chain::vec_to_string(self.generate())
    }

    /// Generates a random string of text starting with the desired token. This returns an empty
    /// string if the token is not found.
    pub fn generate_str_from_token(&self, string: &str) -> String {
        Chain::vec_to_string(self.generate_from_token(string.to_owned()))
    }

    /// Generates a random string of text starting with the desired tokens. If the token is not
    /// found, this returns the string with the first n tokens that were found
    // pub fn generate_from_tokens<S: AsRef<[T]>>(&self, tokens: S) -> Vec<T> {
    pub fn generate_str_from_tokens(&self, strings: &[&str]) -> String {
        let tokens = strings.iter().map(|&s| s.to_owned()).collect::<Vec<_>>();
        Chain::vec_to_string(self.generate_from_tokens(tokens))
    }

    /// Produces an infinite iterator of generated strings.
    pub fn str_iter(&self) -> InfiniteChainStringIterator {
        let vec_to_string: fn(Vec<String>) -> String = Chain::vec_to_string;
        self.iter().map(vec_to_string)
    }

    /// Produces a sized iterator of generated strings.
    pub fn str_iter_for(&self, size: usize) -> SizedChainStringIterator {
        let vec_to_string: fn(Vec<String>) -> String = Chain::vec_to_string;
        self.iter_for(size).map(vec_to_string)
    }
}

/// A sized iterator over a Markov chain of strings.
pub type SizedChainStringIterator<'a> =
Map<SizedChainIterator<'a, String>, fn(Vec<String>) -> String>;

/// A sized iterator over a Markov chain.
pub struct SizedChainIterator<'a, T: Chainable + 'a> {
    chain: &'a Chain<T>,
    size: usize,
}

impl<'a, T> Iterator for SizedChainIterator<'a, T> where T: Chainable + 'a {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        if self.size > 0 {
            self.size -= 1;
            Some(self.chain.generate())
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

/// An infinite iterator over a Markov chain of strings.
pub type InfiniteChainStringIterator<'a> =
Map<InfiniteChainIterator<'a, String>, fn(Vec<String>) -> String>;

/// An infinite iterator over a Markov chain.
pub struct InfiniteChainIterator<'a, T: Chainable + 'a> {
    chain: &'a Chain<T>
}

impl<'a, T> Iterator for InfiniteChainIterator<'a, T> where T: Chainable + 'a {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        Some(self.chain.generate())
    }
}

/// A collection of states for the Markov chain.
trait States<T: PartialEq> {
    /// Adds a state to this states collection.
    fn add(&mut self, token: Token<T>);
    /// Gets the next state from this collection of states.
    fn next(&self) -> Token<T>;
}

impl<T> States<T> for HashMap<Token<T>, usize> where T: Chainable {
    fn add(&mut self, token: Token<T>) {
        match self.entry(token) {
            Occupied(mut e) => *e.get_mut() += 1,
            Vacant(e) => { e.insert(1); },
        }
    }

    fn next(&self) -> Token<T> {
        let mut sum = 0;
        for &value in self.values() {
            sum += value;
        }
        let mut rng = thread_rng();
        let cap = rng.gen_range(0, sum);
        sum = 0;
        for (key, &value) in self.iter() {
            sum += value;
            if sum > cap {
                return key.clone()
            }
        }
        unreachable!("The random number generator failed.")
    }
}

#[cfg(test)]
mod test {
    use super::Chain;

    #[test]
    fn new() {
        Chain::<u8>::new();
        Chain::<String>::new();
    }

    #[test]
    fn is_empty() {
        let mut chain = Chain::new();
        assert!(chain.is_empty());
        chain.feed(vec![1u8, 2, 3]);
        assert!(!chain.is_empty());
    }

    #[test]
    fn feed() {
        let mut chain = Chain::new();
        chain.feed(vec![3, 5, 10]).feed(vec![5, 12]);
    }

    #[test]
    fn generate() {
        let mut chain = Chain::new();
        chain.feed(vec![3u8, 5, 10]).feed(vec![5, 12]);
        let v = chain.generate();
        assert!([vec![3, 5, 10], vec![3, 5, 12], vec![5, 10], vec![5, 12]].contains(&v));
    }

    #[test]
    fn generate_for_higher_order() {
        let mut chain = Chain::of_order(2);
        chain.feed(vec![3u8, 5, 10]).feed(vec![2, 3, 5, 12]);
        let v = chain.generate();
        assert!([vec![3, 5, 10], vec![3, 5, 12], vec![2, 3, 5, 10], vec![2, 3, 5, 12]].contains(&v));
    }

    #[test]
    fn generate_from_token() {
        let mut chain = Chain::new();
        chain.feed(vec![3u8, 5, 10]).feed(vec![5, 12]);
        let v = chain.generate_from_token(5);
        assert!([vec![5, 10], vec![5, 12]].contains(&v));
    }

    #[test]
    fn generate_from_unfound_token() {
        let mut chain = Chain::new();
        chain.feed(vec![3u8, 5, 10]).feed(vec![5, 12]);
        let v: Vec<_> = chain.generate_from_token(9);
        assert!(v.is_empty());
    }

    #[test]
    fn generate_from_tokens() {
        let mut chain = Chain::new();
        chain.feed(vec![3u8, 5, 10]).feed(vec![5, 12]);
        let v = chain.generate_from_tokens(&vec![3, 5]);
        assert!([vec![3u8, 5, 10], vec![3u8, 5, 12]].contains(&v));
    }

    #[test]
    fn generate_from_tokens_longer() {
        let mut chain = Chain::of_order(2);
        chain.feed(vec![1u8, 2, 3, 4, 5]).feed(vec![2, 3, 5, 6]);
        let v = chain.generate_from_tokens(&vec![1, 2, 3]);
        assert!([vec![1u8, 2, 3, 4, 5], vec![1u8, 2, 3, 5, 6]].contains(&v));
    }


    #[test]
    fn generate_from_tokens_unfound() {
        let mut chain = Chain::new();
        chain.feed(vec![3u8, 5, 10]).feed(vec![5, 12]);
        let v = chain.generate_from_tokens(&vec![12, 37]);
        assert_eq!(vec![12], v);
    }

    #[test]
    fn iter() {
        let mut chain = Chain::new();
        chain.feed(vec![3u8, 5, 10]).feed(vec![5, 12]);
        assert_eq!(chain.iter().size_hint().1, None);
    }

    #[test]
    fn iter_for() {
        let mut chain = Chain::new();
        chain.feed(vec![3u8, 5, 10]).feed(vec![5, 12]);
        assert_eq!(chain.iter_for(5).collect::<Vec<_>>().len(), 5);
    }

    #[test]
    fn feed_str() {
        let mut chain = Chain::new();
        chain.feed_str("I like cats and dogs");
    }

    #[test]
    fn generate_str() {
        let mut chain = Chain::new();
        chain.feed_str("I like cats").feed_str("I hate cats");
        assert!(["I like cats", "I hate cats"].contains(&&chain.generate_str()[..]));
    }

    #[test]
    fn generate_str_from_token() {
        let mut chain = Chain::new();
        chain.feed_str("I like cats").feed_str("cats are cute");
        assert!(["cats", "cats are cute"].contains(&&chain.generate_str_from_token("cats")[..]));
    }

    #[test]
    fn generate_str_from_token_higher_order() {
        let mut chain = Chain::of_order(2);
        chain.feed_str("I like cats").feed_str("cats are cute");
        println!("{:?}", chain.generate_str_from_token("cats"));
        assert!(["cats", "cats are cute"].contains(&&chain.generate_str_from_token("cats")[..]));
    }

    #[test]
    fn generate_str_from_unfound_token() {
        let mut chain = Chain::new();
        chain.feed_str("I like cats").feed_str("cats are cute");
        assert_eq!(chain.generate_str_from_token("test"), "");
    }

    #[test]
    fn generate_str_from_tokens() {
        let mut chain = Chain::new();
        chain.feed_str("I like cats").feed_str("cats are cute").feed_str("cats are nice");
        assert!(["cats are cute", "cats are nice"].contains(&&chain.generate_str_from_tokens(&["cats", "are"])[..]));
    }

    #[test]
    fn generate_str_from_tokens_higher_order() {
        let mut chain = Chain::of_order(2);
        chain.feed_str("I like cats").feed_str("cats are cute").feed_str("cats are nice");
        assert!(["cats are cute", "cats are nice"].contains(&&chain.generate_str_from_tokens(&["cats", "are"])[..]));
    }

    #[test]
    fn generate_str_from_tokens_unfound() {
        let mut chain = Chain::new();
        chain.feed_str("I like cats").feed_str("cats are cute");
        assert_eq!(chain.generate_str_from_tokens(&["test"]), "");
        assert_eq!(chain.generate_str_from_tokens(&["cats", "lick"]), "cats");
    }

    #[test]
    fn str_iter() {
        let mut chain = Chain::new();
        chain.feed_str("I like cats and I like dogs");
        assert_eq!(chain.str_iter().size_hint().1, None);
    }

    #[test]
    fn str_iter_for() {
        let mut chain = Chain::new();
        chain.feed_str("I like cats and I like dogs");
        assert_eq!(chain.str_iter_for(5).collect::<Vec<_>>().len(), 5);
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn save_then_load() {
        let mut chain = Chain::of_order(2);
        chain.feed_str("I like cats and I like dogs");
        chain.save("test.yaml").unwrap();

        let new_chain = Chain::load("test.yaml").unwrap();
        assert_eq!(chain, new_chain);
    }
}
