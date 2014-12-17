#![feature(slicing_syntax)]
extern crate markov;

use std::os::args;
use markov::Chain;

#[cfg(not(test))]
fn main() {
    markov_gen(args().into_iter().skip(1).collect()).iter().map(|s| println!("{}", s)).count();
}

/// Generates a number of strings using a markov chain on specified inputs. This is designed
/// primarily for command-line usage. The arguments are expected to be paths to files to be fed
/// into the chain. Additionally, the argument `-n #` is supported to specify the number of phrases
/// to be generated. This number must be a positive, non-zero integer. 
///
/// Some valid usages of this function:
/// `markov_gen(vec!["test".into_string()])`
/// `markov_gen(vec!["test".into_string(), "-n".into_string(), "3".into_string()])`
/// `markov_gen(vec!["-n".into_string(), "3".into_string(), "test".into_string()])`
///
/// Some invalid usages of this function:
/// `markov_gen(vec!["-n".into_string(), "3".into_string()])`
/// `markov_gen(vec!["test".into_string(), "-n".into_string(), "0".into_string()])`
/// `markov_gen(vec!["test".into_string(), "-n".into_string(), "test".into_string()])`
fn markov_gen(args: Vec<String>) -> Vec<String> {
    let mut chain = Chain::for_strings();
    let mut expecting_num = false;
    let mut count = 1u;
    for arg in args.iter() {
        if expecting_num {
            match from_str(arg[]) {
                Some(n) if n > 0u => count = n,
                _ => panic!("Expected positive integer argument to -n, found {}.", arg[])
            }
            expecting_num = false;
        } else if arg[] == "-n" {
            expecting_num = true;
        } else {
            chain.feed_file(&Path::new(arg[]));
        }
    }
    if chain.is_empty() { panic!("No files were fed into the chain.") }
    chain.str_iter_for(count).collect()
}

#[cfg(test)]
mod test {
    use super::markov_gen;
    
    #[test]
    fn gen_default() {
        assert_eq!(markov_gen(vec!["test".into_string()]).len(), 1)
    }

    #[test]
    fn gen_number_after() {
        assert_eq!(markov_gen(
            vec!["test".into_string(), "-n".into_string(), "3".into_string()]
        ).len(), 3)
    }

    #[test]
    fn gen_before_number() {
        assert_eq!(markov_gen(
            vec!["-n".into_string(), "3".into_string(), "test".into_string()]
        ).len(), 3)
    }

    #[test]
    #[should_fail(message = "No files were fed into the chain.")]
    fn gen_invalid_no_files() {
        markov_gen(vec!["-n".into_string(), "3".into_string()]);
    }

    #[test]
    #[should_fail(message = "Expected positive integer argument to -n, found 0.")]
    fn gen_invalid_n_arg_zero() {
        markov_gen(vec!["test".into_string(), "-n".into_string(), "0".into_string()]);
    }

    #[test]
    #[should_fail(message = "Expected positive integer argument to -n, found test.")]
    fn gen_invalid_n_arg_string() {
        markov_gen(vec!["test".into_string(), "-n".into_string(), "test".into_string()]);
    }
}
