#[cfg(feature = "getopts")] extern crate getopts;
#[cfg(feature = "getopts")] extern crate markov;

#[cfg(all(feature = "getopts", not(test)))] use std::env::args;
#[cfg(feature = "getopts")] use std::path::Path;
#[cfg(feature = "getopts")] use getopts::Options;
#[cfg(feature = "getopts")] use markov::Chain;

#[cfg(all(feature = "getopts", not(test)))]
fn main() {
    markov_gen(args().collect()).iter().map(|s| println!("{}", s)).count();
}

#[cfg(all(not(feature = "getopts"), not(test)))]
fn main() {
    println!("markgen must be compiled with getopts enabled.")
}

/// Generates a number of strings using a markov chain on specified inputs. This is designed
/// primarily for command-line usage. The arguments are expected to be paths to files to be fed
/// into the chain. Additionally, the argument `-n #` is supported to specify the number of phrases
/// to be generated. This number must be a positive, non-zero integer. `-o #` is also supported to
/// specify the order of Markov chain to be used. Note `-o` must be specified before any file
/// names and must also be a positive, non-zero integer.
///
/// Some valid usages of this function:
/// `markov_gen(vec!["test".to_owned()])`
/// `markov_gen(vec!["test".to_owned(), "-n".to_owned(), "3".to_owned()])`
/// `markov_gen(vec!["-n".to_owned(), "3".to_owned(), "test".to_owned()])`
/// `markov_gen(vec!["-o".to_owned(), "2".to_owned(), "test".to_owned()])`
///
/// Some invalid usages of this function:
/// `markov_gen(vec!["-n".to_owned(), "3".to_owned()])`
/// `markov_gen(vec!["test".to_owned(), "-n".to_owned(), "0".to_owned()])`
/// `markov_gen(vec!["test".to_owned(), "-n".to_owned(), "test".to_owned()])`
/// `markov_gen(vec!["test".to_owned(), "-o".to_owned(), "3".to_owned()])`
/// `markov_gen(vec!["-o".to_owned(), "0".to_owned()], "test".to_owned())`
#[cfg(feature = "getopts")]
fn markov_gen(args: Vec<String>) -> Vec<String>{
    let mut opts = Options::new();
    opts.optopt("n", "count", "set the number of phrases to generate", "COUNT");
    opts.optopt("o", "order", "set the order of the Markov chain", "ORDER");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };
    if matches.opt_present("h") {
        let brief = format!("Usage: {} FILE1 [FILE2 FILE3 ...] [options]", args[0]);
        print!("{}", opts.usage(&brief));
        Vec::new()
    } else {
        let count = match matches.opt_str("n") {
            Some(arg) => match arg.parse() {
                Ok(n) if n > 0 => n,
                _ => panic!("Expected positive integer argument to -n, found {}.", &arg),
            },
            None => 1
        };
        let mut chain = match matches.opt_str("o").map(|arg| arg.parse()) {
            Some(Ok(n)) if n > 0 => Chain::new_with_order(n),
            None => Chain::new(),
            Some(_) => panic!(
                "Expected positive integer argument to -n, found {}.", matches.opt_str("o").unwrap()
            ),
        };
        for path in matches.free.iter() {
            chain.feed_file(Path::new(&path));
        }
        if chain.is_empty() { panic!("No files were fed into the chain.") }
        chain.str_iter_for(count).collect()
    }
}

#[cfg(all(feature = "getopts", test))]
mod test {
    use super::markov_gen;
    use std::borrow::ToOwned;

    #[test]
    fn gen_default() {
        assert_eq!(markov_gen(vec!["prog".to_owned(), "test".to_owned()]).len(), 1)
    }

    #[test]
    fn gen_number_after() {
        assert_eq!(markov_gen(
            vec!["markgen".to_owned(), "test".to_owned(), "-n".to_owned(), "3".to_owned()]
        ).len(), 3)
    }

    #[test]
    fn gen_before_number() {
        assert_eq!(markov_gen(
            vec!["markgen".to_owned(), "-n".to_owned(), "3".to_owned(), "test".to_owned()]
        ).len(), 3)
    }

    #[test]
    #[should_panic(expected = "No files were fed into the chain.")]
    fn gen_invalid_no_files() {
        markov_gen(vec!["prog".to_owned(), "-n".to_owned(), "3".to_owned()]);
    }

    #[test]
    #[should_panic(expected = "Expected positive integer argument to -n, found 0.")]
    fn gen_invalid_n_arg_zero() {
        markov_gen(vec!["prog".to_owned(), "test".to_owned(), "-n".to_owned(), "0".to_owned()]);
    }

    #[test]
    #[should_panic(expected = "Expected positive integer argument to -n, found test.")]
    fn gen_invalid_n_arg_string() {
        markov_gen(vec!["prog".to_owned(), "test".to_owned(), "-n".to_owned(), "test".to_owned()]);
    }
}
