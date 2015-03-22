extern crate markov;

use std::env::args;
use std::path::Path;
use markov::Chain;

#[cfg(not(test))]
fn main() {
    markov_gen(args().skip(1).collect()).iter().map(|s| println!("{}", s)).count();
}

/// Generates a number of strings using a markov chain on specified inputs. This is designed
/// primarily for command-line usage. The arguments are expected to be paths to files to be fed
/// into the chain. Additionally, the argument `-n #` is supported to specify the number of phrases
/// to be generated. This number must be a positive, non-zero integer. 
///
/// Some valid usages of this function:
/// `markov_gen(vec!["test".to_owned()])`
/// `markov_gen(vec!["test".to_owned(), "-n".to_owned(), "3".to_owned()])`
/// `markov_gen(vec!["-n".to_owned(), "3".to_owned(), "test".to_owned()])`
///
/// Some invalid usages of this function:
/// `markov_gen(vec!["-n".to_owned(), "3".to_owned()])`
/// `markov_gen(vec!["test".to_owned(), "-n".to_owned(), "0".to_owned()])`
/// `markov_gen(vec!["test".to_owned(), "-n".to_owned(), "test".to_owned()])`
fn markov_gen(args: Vec<String>) -> Vec<String> {
    let mut chain = Chain::for_strings();
    let mut expecting_num = false;
    let mut count = 1usize;
    for arg in args.iter() {
        if expecting_num {
            match arg.parse() {
                Ok(n) if n > 0 => count = n,
                _ => panic!("Expected positive integer argument to -n, found {}.", &arg)
            }
            expecting_num = false;
        } else if &arg[..] == "-n" {
            expecting_num = true;
        } else {
            chain.feed_file(Path::new(&arg));
        }
    }
    if chain.is_empty() { panic!("No files were fed into the chain.") }
    chain.str_iter_for(count).collect()
}

#[cfg(test)]
mod test {
    use super::markov_gen;
    use std::borrow::ToOwned;

    #[test]
    fn gen_default() {
        assert_eq!(markov_gen(vec!["test".to_owned()]).len(), 1)
    }

    #[test]
    fn gen_number_after() {
        assert_eq!(markov_gen(
            vec!["test".to_owned(), "-n".to_owned(), "3".to_owned()]
        ).len(), 3)
    }

    #[test]
    fn gen_before_number() {
        assert_eq!(markov_gen(
            vec!["-n".to_owned(), "3".to_owned(), "test".to_owned()]
        ).len(), 3)
    }

    #[test]
    #[should_fail(message = "No files were fed into the chain.")]
    fn gen_invalid_no_files() {
        markov_gen(vec!["-n".to_owned(), "3".to_owned()]);
    }

    #[test]
    #[should_fail(message = "Expected positive integer argument to -n, found 0.")]
    fn gen_invalid_n_arg_zero() {
        markov_gen(vec!["test".to_owned(), "-n".to_owned(), "0".to_owned()]);
    }

    #[test]
    #[should_fail(message = "Expected positive integer argument to -n, found test.")]
    fn gen_invalid_n_arg_string() {
        markov_gen(vec!["test".to_owned(), "-n".to_owned(), "test".to_owned()]);
    }
}
