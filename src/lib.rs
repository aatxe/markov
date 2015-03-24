//! A generic [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) for almost any type. This 
//! uses HashMaps internally, and so Eq and Hash are both required.
//!
//! # Examples
//!
//! ```
//! use markov::Chain;
//! 
//! let mut chain = Chain::for_strings();
//! chain.feed_str("I like cats and I like dogs.");
//! println!("{}", chain.generate_str());
//! ```
//!
//! ```
//! use markov::Chain;
//!
//! let mut chain = Chain::new(0u8, 255);
//! chain.feed(vec![1u8, 2, 3, 5]).feed(vec![3u8, 9, 2]);
//! println!("{:?}", chain.generate());
//! ```
#![unstable]
#![feature(core, io, std_misc)]
#![warn(missing_docs)]

extern crate rand;
extern crate "rustc-serialize" as rustc_serialize;

use std::borrow::ToOwned;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::error::Error as StdError;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, Error, ErrorKind, Result};
use std::io::prelude::*;
use std::iter::Map;
use std::path::Path;
use std::rc::Rc;
use rand::{Rng, thread_rng};
use rustc_serialize::{Decodable, Encodable};
use rustc_serialize::json::{decode, encode};

/// The definition of all types that can be used in a Chain.
pub trait Chainable: Eq + Hash {}
impl<T> Chainable for T where T: Eq + Hash {}

/// A generic [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) for almost any type. This 
/// uses HashMaps internally, and so Eq and Hash are both required.
#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
pub struct Chain<T> where T: Chainable {
    map: HashMap<Rc<T>, HashMap<Rc<T>, usize>>,
    start: Rc<T>,
    end: Rc<T>,
}

impl<T> Chain<T> where T: Chainable {
    /// Constructs a new Markov chain using the given tokens as the marked starting and ending
    /// points for generation.
    pub fn new(start: T, end: T) -> Chain<T> {
        let start = Rc::new(start);
        let end = Rc::new(end);
        Chain {
            map: {
                let mut map = HashMap::new();
                map.insert(start.clone(), HashMap::new());
                map
            },
            start: start, end: end
        }
    }

    /// Determines whether or not the chain is empty. A chain is considered empty if nothing has
    /// been fed into it.
    pub fn is_empty(&self) -> bool {
        self.map[self.start.clone()].is_empty()
    }


    /// Feeds the chain a collection of tokens. This operation is O(n) where n is the number of
    /// tokens to be fed into the chain.
    pub fn feed(&mut self, tokens: Vec<T>) -> &mut Chain<T> {
        if tokens.len() == 0 { return self }
        let mut toks = Vec::new();
        toks.push(self.start.clone());
        toks.extend(tokens.into_iter().map(|token| {
            let rc = Rc::new(token);
            if !self.map.contains_key(&rc) {
                self.map.insert(rc.clone(), HashMap::new());
            }
            rc
        }));
        toks.push(self.end.clone());
        for p in toks.windows(2) {
            self.map.get_mut(&p[0]).unwrap().add(p[1].clone());
        }
        self
    }

    /// Generates a collection of tokens from the chain. This operation is O(mn) where m is the
    /// length of the generated collection, and n is the number of possible states from a given
    /// state.
    pub fn generate(&self) -> Vec<Rc<T>> {
        let mut ret = Vec::new();
        let mut curs = self.start.clone();
        while curs != self.end {
            curs = self.map[curs].next();
            ret.push(curs.clone());
        }
        ret.pop();
        ret
    }

    /// Generates a collection of tokens from the chain, starting with the given token. This
    /// operation is O(mn) where m is the length of the generated collection, and n is the number
    /// of possible states from a given state. This returns an empty vector if the token is not
    /// found.
    pub fn generate_from_token(&self, token: T) -> Vec<Rc<T>> {
        let token = Rc::new(token);
        if !self.map.contains_key(&token) { return Vec::new() }
        let mut ret = vec![token.clone()];
        let mut curs = token;
        while curs != self.end {
            curs = self.map[curs].next();
            ret.push(curs.clone());
        }
        ret.pop();
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
}

impl<T> Chain<T> where T: Decodable + Chainable {
    /// Loads a chain from a JSON file at the specified path.
    pub fn load(path: &Path) -> Result<Chain<T>> {
        let mut file = try!(File::open(path));
        let mut data = String::new();
        try!(file.read_to_string(&mut data));
        decode(&data).map_err(|e| 
            Error::new(ErrorKind::InvalidInput, "Failed to decode markov chain.", 
                       Some(e.description().to_owned()))
        )
    }

    /// Loads a chain from a JSON file using a string path.
    pub fn load_utf8(path: &str) -> Result<Chain<T>> {
        Chain::load(&Path::new(path))
    }
}

impl<T> Chain<T> where T: for<'a> Encodable + Chainable {
    /// Saves a chain to a JSON file at the specified path.
    pub fn save(&self, path: &Path) -> Result<()> {
        let mut f = try!(File::create(path));
        try!(f.write_all(&try!(encode(self).map_err(|e| 
            Error::new(ErrorKind::InvalidInput, "Failed to encode markov chain.", 
                       Some(e.description().to_owned()))                                
        )).as_bytes()));
        f.flush()
    }

    /// Saves a chain to a JSON file using a string path.
    pub fn save_utf8(&self, path: &str) -> Result<()> {
        self.save(&Path::new(path))
    }
}

impl Chain<String> {
    /// Creates a new Chain intended specifically for strings. This uses the Unicode start of text
    /// and end of text control characters as the starting and ending tokens for the chain.
    pub fn for_strings() -> Chain<String> {
        Chain::new("\u{0002}".to_owned(), "\u{0003}".to_owned())
    }

    /// Feeds a string of text into the chain.     
    pub fn feed_str(&mut self, string: &str) -> &mut Chain<String> {
        self.feed(string.split(" ").map(|s| s.to_owned()).collect())
    }

    /// Feeds a properly formatted file into the chain. This file should be formatted such that
    /// each line is a new sentence. Punctuation may be included if it is desired.
    pub fn feed_file(&mut self, path: &Path) -> &mut Chain<String> {
        let reader = BufReader::new(File::open(path).unwrap());
        for line in reader.lines() {
            let line = line.unwrap();
            let words: Vec<_> = line.split(&[' ', '\t', '\n', '\r'][..])
                                    .filter(|word| !word.is_empty())
                                    .collect();
            self.feed(words.iter().map(|&s| s.to_owned()).collect());
        }
        self
    }

    /// Converts the output of generate(...) on a String chain to a single String.
    fn vec_to_string(vec: Vec<Rc<String>>) -> String {
        let mut ret = String::new();
        for s in vec.iter() {
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

    /// Produces an infinite iterator of generated strings.
    pub fn str_iter(&self) -> InfiniteChainStringIterator {
        let vec_to_string: fn(Vec<Rc<String>>) -> String = Chain::vec_to_string;
        self.iter().map(vec_to_string) 
    }

    /// Produces a sized iterator of generated strings.
    pub fn str_iter_for(&self, size: usize) -> SizedChainStringIterator {
        let vec_to_string: fn(Vec<Rc<String>>) -> String = Chain::vec_to_string;
        self.iter_for(size).map(vec_to_string)
    }
}

/// A sized iterator over a Markov chain of strings.
pub type SizedChainStringIterator<'a> =
Map<SizedChainIterator<'a, String>, fn(Vec<Rc<String>>) -> String>;

/// A sized iterator over a Markov chain.
pub struct SizedChainIterator<'a, T: Chainable + 'a> {
    chain: &'a Chain<T>,
    size: usize,
}

impl<'a, T> Iterator for SizedChainIterator<'a, T> where T: Chainable + 'a {
    type Item = Vec<Rc<T>>;
    fn next(&mut self) -> Option<Vec<Rc<T>>> {
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
Map<InfiniteChainIterator<'a, String>, fn(Vec<Rc<String>>) -> String>;

/// An infinite iterator over a Markov chain.
pub struct InfiniteChainIterator<'a, T: Chainable + 'a> {
    chain: &'a Chain<T>
}

impl<'a, T> Iterator for InfiniteChainIterator<'a, T> where T: Chainable + 'a {
    type Item = Vec<Rc<T>>;
    fn next(&mut self) -> Option<Vec<Rc<T>>> {
        Some(self.chain.generate())
    }
}

/// A collection of states for the Markov chain.
trait States<T: PartialEq> {
    /// Adds a state to this states collection.
    fn add(&mut self, token: Rc<T>);
    /// Gets the next state from this collection of states.
    fn next(&self) -> Rc<T>;
}

impl<T> States<T> for HashMap<Rc<T>, usize> where T: Chainable {
    fn add(&mut self, token: Rc<T>) {
        match self.entry(token) {
            Occupied(mut e) => *e.get_mut() += 1,
            Vacant(e) => { e.insert(1); },
        }
    }

    fn next(&self) -> Rc<T> {
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
        Chain::new(0u8, 100);
        Chain::for_strings();
    }

    #[test]
    fn is_empty() {
        let mut chain = Chain::new(0u8, 100);
        assert!(chain.is_empty());
        chain.feed(vec![1, 2, 3]);
        assert!(!chain.is_empty());
    }

    #[test]
    fn feed() {
        let mut chain = Chain::new(0u8, 100);
        chain.feed(vec![3, 5, 10]).feed(vec![5, 12]);
    }

    #[test]
    fn generate() {
        let mut chain = Chain::new(0u8, 100);
        chain.feed(vec![3, 5, 10]).feed(vec![5, 12]);
        let v = chain.generate().into_iter().map(|v| *v).collect();
        assert!([vec![3, 5, 10], vec![3, 5, 12], vec![5, 10], vec![5, 12]].contains(&v));
    }

    #[test]
    fn generate_from_token() {
        let mut chain = Chain::new(0u8, 100);
        chain.feed(vec![3, 5, 10]).feed(vec![5, 12]);
        let v = chain.generate_from_token(5).into_iter().map(|v| *v).collect();
        assert!([vec![5, 10], vec![5, 12]].contains(&v));
    }

    #[test]
    fn generate_from_unfound_token() {
        let mut chain = Chain::new(0u8, 100);
        chain.feed(vec![3, 5, 10]).feed(vec![5, 12]);
        let v: Vec<_> = chain.generate_from_token(9).into_iter().map(|v| *v).collect();
        assert_eq!(v, vec![]);
    }

    #[test]
    fn iter() {    
        let mut chain = Chain::new(0u8, 100);
        chain.feed(vec![3, 5, 10]).feed(vec![5, 12]);
        assert_eq!(chain.iter().size_hint().1, None);
    }

    #[test]
    fn iter_for() {
        let mut chain = Chain::new(0u8, 100);
        chain.feed(vec![3, 5, 10]).feed(vec![5, 12]);
        assert_eq!(chain.iter_for(5).collect::<Vec<_>>().len(), 5);
    }

    #[test]
    fn feed_str() {
        let mut chain = Chain::for_strings();
        chain.feed_str("I like cats and dogs");
    }

    #[test]
    fn generate_str() {
        let mut chain = Chain::for_strings();
        chain.feed_str("I like cats").feed_str("I hate cats");
        assert!(["I like cats", "I hate cats"].contains(&&chain.generate_str()[..]));
    }

    #[test]
    fn generate_str_from_token() {
        let mut chain = Chain::for_strings();
        chain.feed_str("I like cats").feed_str("cats are cute");
        assert!(["cats", "cats are cute"].contains(&&chain.generate_str_from_token("cats")[..]));
    }

    #[test]
    fn generate_str_from_unfound_token() {
        let mut chain = Chain::for_strings();
        chain.feed_str("I like cats").feed_str("cats are cute");
        assert_eq!(chain.generate_str_from_token("test"), "");
    }
    
    #[test]
    fn str_iter() {    
        let mut chain = Chain::for_strings();
        chain.feed_str("I like cats and I like dogs");
        assert_eq!(chain.str_iter().size_hint().1, None);
    }

    #[test]
    fn str_iter_for() {   
        let mut chain = Chain::for_strings();
        chain.feed_str("I like cats and I like dogs");
        assert_eq!(chain.str_iter_for(5).collect::<Vec<_>>().len(), 5);
    }


    #[test]
    fn save() {
        let mut chain = Chain::for_strings();
        chain.feed_str("I like cats and I like dogs");
        chain.save_utf8("save.json").unwrap();
    }

    #[test]
    fn load() {
        let mut chain = Chain::for_strings();
        chain.feed_str("I like cats and I like dogs");
        chain.save_utf8("load.json").unwrap();
        let other_chain: Chain<String> = Chain::load_utf8("load.json").unwrap();
        assert_eq!(other_chain, chain);
    }
}
