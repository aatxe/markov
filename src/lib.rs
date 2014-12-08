#![feature(slicing_syntax)]
use std::collections::HashMap;
use std::hash::Hash;
use std::io::{BufferedReader, File};
use std::rand::{Rng, task_rng};
use std::rc::Rc;

trait States<T: PartialEq> {
    fn add(&mut self, token: Rc<T>);
    fn next(&self) -> Rc<T>;
}

struct State<T: PartialEq> {
    occurrences: uint,
    token: Rc<T>
}

impl<T: PartialEq> PartialEq for State<T> {
    fn eq(&self, other: &State<T>) -> bool {
        self.token == other.token
    }
}

impl<T: PartialEq> State<T> {
    pub fn new(token: Rc<T>) -> State<T> {
        State { token: token, occurrences: 1u }
    }

    pub fn inc(&mut self) {
        self.occurrences += 1
    }

    pub fn val(&self) -> uint {
        self.occurrences
    }

    pub fn token(&self) -> Rc<T> {
        self.token.clone()
    }
}

impl<T: PartialEq> States<T> for Vec<State<T>> {
    fn add(&mut self, token: Rc<T>) {
        let state = State::new(token);
        match self.position_elem(&state) {
            Some(i) => self[i].inc(),
            None => self.push(state),
        }
    }
    
    fn next(&self) -> Rc<T> {
        let mut sum = 0;
        for state in self.iter() {
            sum += state.val();
        }
        let mut rng = task_rng();
        let cap = rng.gen_range(0, sum + 1);
        sum = 0;
        for state in self.iter() {
            sum += state.val();
            if sum >= cap {
                return state.token().clone()
            }
        }
        self.iter().next().unwrap().token().clone()
    }
}

pub struct Chain<T: Eq + Hash> {
    map: HashMap<Rc<T>, Vec<State<T>>>,
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
                map.insert(start.clone(), Vec::new());
                map
            }, 
            start: start, end: end 
        }
    }

    pub fn feed(&mut self, tokens: Vec<T>) -> &mut Chain<T> {
        if tokens.len() == 0 { return self }
        let mut past = None;
        for token in tokens.into_iter() {
            let rc = Rc::new(token);
            if let Some(ref past) = past {
                if !self.map.contains_key(past) {
                    self.map.insert(past.clone(), Vec::new());
                }
                self.map[*past].add(rc.clone())
            } else {
                self.map[self.start].add(rc.clone());
            }
            past = Some(rc)
        }
        let past = past.unwrap();
        if !self.map.contains_key(&past) {
            self.map.insert(past.clone(), Vec::new());
        }
        self.map[past].add(self.end.clone());
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
            let words: Vec<_> = line.split(|c: char| [' ', '\t', '\n', '\r'].contains(&c))
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
