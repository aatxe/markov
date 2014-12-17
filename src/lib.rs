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
//! println!("{}", chain.generate());
//! ```
#![experimental]
#![feature(slicing_syntax)]
#![warn(missing_docs)]

extern crate serialize;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::hash::Hash;
use std::io::{BufferedReader, File, InvalidInput, IoError, IoResult};
use std::rand::{Rng, task_rng};
use std::rc::Rc;
use serialize::{Decodable, Encodable};
use serialize::json::{Decoder, DecoderError, Encoder, decode, encode};

/// A generic [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) for almost any type. This 
/// uses HashMaps internally, and so Eq and Hash are both required.
#[deriving(Encodable, Decodable, PartialEq, Show)]
pub struct Chain<T: Eq + Hash> {
    map: HashMap<Rc<T>, HashMap<Rc<T>, uint>>,
    start: Rc<T>,
    end: Rc<T>,
}

impl<T: Eq + Hash> Chain<T> {
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
            (&mut self.map[p[0]]).add(p[1].clone());
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
}

impl<T: Decodable<Decoder, DecoderError> + Eq + Hash> Chain<T> {
    /// Loads a chain from a JSON file at the specified path.
    pub fn load(path: &Path) -> IoResult<Chain<T>> {
        let mut file = try!(File::open(path));
        let data = try!(file.read_to_string());
        decode(data[]).map_err(|e| IoError {
            kind: InvalidInput,
            desc: "Decoder error",
            detail: Some(e.to_string()),
        })
    }

    /// Loads a chain from a JSON file using a string path.
    pub fn load_utf8(path: &str) -> IoResult<Chain<T>> {
        Chain::load(&Path::new(path))
    }
}

impl<'a, T: Encodable<Encoder<'a>, IoError> + Eq + Hash> Chain<T> {
    /// Saves a chain to a JSON file at the specified path.
    pub fn save(&self, path: &Path) -> IoResult<()> {
        let mut f = File::create(path);
        f.write_str(encode(self)[])
    }

    /// Saves a chain to a JSON file using a string path.
    pub fn save_utf8(&self, path: &str) -> IoResult<()> {
        self.save(&Path::new(path))
    }
}

impl Chain<String> {
    /// Creates a new Chain intended specifically for strings. This uses the Unicode start of text
    /// and end of text control characters as the starting and ending tokens for the chain.
    pub fn for_strings() -> Chain<String> {
        Chain::new("\u{0002}".into_string(), "\u{0003}".into_string())
    }

    /// Feeds a string of text into the chain.     
    pub fn feed_str(&mut self, string: &str) -> &mut Chain<String> {
        self.feed(string.split_str(" ").map(|s| s.into_string()).collect())
    }

    /// Feeds a properly formatted file into the chain. This file should be formatted such that
    /// each line is a new sentence. Punctuation may be included if it is desired.
    pub fn feed_file(&mut self, path: &Path) -> &mut Chain<String> {
        let mut reader = BufferedReader::new(File::open(path));
        for line in reader.lines() {
            let line = line.unwrap();
            let words: Vec<_> = line.split([' ', '\t', '\n', '\r'][])
                                    .filter(|word| !word.is_empty())
                                    .collect();
            self.feed(words.iter().map(|s| s.into_string()).collect());
        }
        self
    }

    /// Generates a random string of text.
    pub fn generate_str(&self) -> String { 
        let vec = self.generate();
        let mut ret = String::new();
        for s in vec.iter() {
            ret.push_str(s[]);
            ret.push_str(" ");
        }
        let len = ret.len();
        ret.truncate(len - 1);
        ret
    }

    /// Generates a random string of text starting with the desired token. This returns an empty
    /// string if the token is not found.
    pub fn generate_str_from_token(&self, string: &str) -> String {
        let vec = self.generate_from_token(string.into_string());
        let mut ret = String::new();
        for s in vec.iter() {
            ret.push_str(s[]);
            ret.push_str(" ");
        }
        let len = ret.len();
        if len > 0 { 
            ret.truncate(len - 1);
        }
        ret
    }
}

/// A collection of states for the Markov chain.
trait States<T: PartialEq> {
    /// Adds a state to this states collection.
    fn add(&mut self, token: Rc<T>);
    /// Gets the next state from this collection of states.
    fn next(&self) -> Rc<T>;
}

impl<T: Eq + Hash> States<T> for HashMap<Rc<T>, uint> {
    fn add(&mut self, token: Rc<T>) {
        match self.entry(token) {
            Occupied(mut e) => *e.get_mut() += 1,
            Vacant(e) => { e.set(1); },
        }
    }

    fn next(&self) -> Rc<T> {
        let mut sum = 0;
        for &value in self.values() {
            sum += value;
        }
        let mut rng = task_rng();
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
        Chain::new(0u, 100u);
        Chain::for_strings();
    }

    #[test]
    fn is_empty() {
        let mut chain = Chain::new(0u, 100u);
        assert!(chain.is_empty());
        chain.feed(vec![1u, 2u, 3u]);
        assert!(!chain.is_empty());
    }

    #[test]
    fn feed() {
        let mut chain = Chain::new(0u, 100u);
        chain.feed(vec![3u, 5u, 10u]).feed(vec![5u, 12u]);
    }

    #[test]
    fn generate() {
        let mut chain = Chain::new(0u, 100u);
        chain.feed(vec![3u, 5u, 10u]).feed(vec![5u, 12u]);
        let v = chain.generate().map_in_place(|v| *v);
        assert!([vec![3u, 5u, 10u], vec![3u, 5u, 12u], vec![5u, 10u], vec![5u, 12u]].contains(&v));
    }

    #[test]
    fn generate_from_token() {
        let mut chain = Chain::new(0u, 100u);
        chain.feed(vec![3u, 5u, 10u]).feed(vec![5u, 12u]);
        let v = chain.generate_from_token(5u).map_in_place(|v| *v);
        assert!([vec![5u, 10u], vec![5u, 12u]].contains(&v));
    }

    #[test]
    fn generate_from_unfound_token() {
        let mut chain = Chain::new(0u, 100u);
        chain.feed(vec![3u, 5u, 10u]).feed(vec![5u, 12u]);
        let v = chain.generate_from_token(9u).map_in_place(|v| *v);
        assert_eq!(v, vec![]);
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
        assert!(["I like cats", "I hate cats"].contains(&chain.generate_str()[]));
    }

    #[test]
    fn generate_str_from_token() {
        let mut chain = Chain::for_strings();
        chain.feed_str("I like cats").feed_str("cats are cute");
        assert!(["cats", "cats are cute"].contains(&chain.generate_str_from_token("cats")[]));
    }

    #[test]
    fn generate_str_from_unfound_token() {
        let mut chain = Chain::for_strings();
        chain.feed_str("I like cats").feed_str("cats are cute");
        assert_eq!(chain.generate_str_from_token("test"), "");
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
