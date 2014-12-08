#![feature(slicing_syntax)]
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::hash::Hash;
use std::io::{BufferedReader, File};
use std::rand::{Rng, task_rng};
use std::rc::Rc;

trait States<T: PartialEq> {
    fn add(&mut self, token: Rc<T>);
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
        unreachable!("RNG failed")
    }
}

pub struct Chain<T: Eq + Hash> {
    map: HashMap<Rc<T>, HashMap<Rc<T>, uint>>,
    start: Rc<T>,
    end: Rc<T>,
}

impl<T: Eq + Hash> Chain<T> {
    pub fn new(start: T, end: T) -> Chain<T> {
        let start = Rc::new(start);
        let end = Rc::new(end);
        Chain {
            map: {
                let mut map = HashMap::new();
                map.insert(start.clone(), HashMap::new());
                map.insert(end.clone(), HashMap::new());
                map
            },
            start: start, end: end
        }
    }
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
            self.map[p[0]].add(p[1].clone());
        }
        self
    }
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
}

impl Chain<String> {
    pub fn feed_str(&mut self, string: &str) -> &mut Chain<String> {
        self.feed(string.split_str(" ").map(|s| s.into_string()).collect())
    }

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

    pub fn generate_str(&self) -> String {
        let vec = self.generate();
        let mut ret = String::new();
        for s in vec.iter() {
            ret.push_str(s[]);
            ret.push_str(" ");
        }
        let len = ret.len();
        ret.truncate(len - 1);
        ret.push_str(".");
        ret
    }
}
