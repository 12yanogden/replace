use clap::{Arg, Command};
use regex::Regex;

/// Parses command line arguments using the `clap` crate.
///
/// # Returns
///
/// A `Command` instance containing the parsed arguments.
///
/// # Examples
///
/// ```
/// use replace::init_command;
/// use clap::CommandFactory;
///
/// let cmd = replace::init_command();
/// let matches = cmd.override_usage("replace --pattern <PATTERN> --replacement <REPLACEMENT> --haystack <HAYSTACK>")
///     .try_get_matches_from(vec!["replace", "--pattern", "foo", "--replacement", "bar", "--haystack", "test string"])
///     .unwrap();
/// assert_eq!(matches.get_one::<String>("pattern").unwrap(), "foo");
/// assert_eq!(matches.get_one::<String>("replacement").unwrap(), "bar");
/// assert_eq!(matches.get_one::<String>("haystack").unwrap(), "test string");
/// ```
pub fn init_command() -> Command {
    Command::new("replace")
        .version("1.0")
        .author("Ryan Ogden <12yanogden@gmail.com>")
        .about("Replaces matches to a regex in a string with a given string")
        .arg(
            Arg::new("pattern")
                .short('p')
                .long("pattern")
                .value_name("PATTERN")
                .help("The regex pattern to match")
                .action(clap::ArgAction::Set)
                .required(true),
        )
        .arg(
            Arg::new("replacement")
                .short('r')
                .long("replacement")
                .value_name("REPLACEMENT")
                .help("The string to replace matches with")
                .action(clap::ArgAction::Set)
                .required(true),
        )
        .arg(
            Arg::new("haystack")
                .long("haystack")
                .value_name("HAYSTACK")
                .help("The string to search within")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .help("Replace all matches of the pattern")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("every_nth")
                .long("every_nth")
                .value_name("EVERY_NTH")
                .help("Replace every nth match of the pattern")
                .value_parser(clap::value_parser!(u16).range(0..))
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("nth")
                .long("nth")
                .value_name("NTH")
                .help("Replace only the nth match of the pattern")
                .value_parser(clap::value_parser!(u16).range(0..))
                .action(clap::ArgAction::Set),
        )
}

/// Validates that the given pattern is a valid regular expression.
///
/// # Arguments
///
/// * `pattern` - A string slice that holds the regex pattern.
///
/// # Examples
///
/// ```
/// use replace::verify_is_valid_regex;
/// verify_is_valid_regex(r"\d+");
/// ```
pub fn verify_is_valid_regex(pattern: &str) {
    if Regex::new(pattern).is_err() {
        eprintln!(
            "Error: The pattern given is not a valid regular expression: {}",
            pattern
        );
        std::process::exit(1);
    }
}

/// Validates that at least one of the conflicting options in each pair is undefined.
///
/// # Arguments
///
/// * `option_pairs` - A list of tuples containing pairs of conflicting option values.
///
/// # Examples
///
/// ```
/// use replace::verify_has_no_conflicting_options;
/// verify_has_no_conflicting_options(vec![(Some("value1"), None), (None, Some("value2"))]);
/// ```
pub fn verify_has_no_conflicting_options(option_pairs: Vec<(Option<&str>, Option<&str>)>) {
    for (opt1, opt2) in option_pairs {
        if opt1.is_some() && opt2.is_some() {
            eprintln!(
                "error: conflicting options provided: {:?}, {:?}",
                opt1.unwrap(),
                opt2.unwrap()
            );
            std::process::exit(1);
        }
    }
}

/// Verifies that at least one option is provided.
///
/// # Arguments
///
/// * `options` - A vector of tuples where each tuple contains the name and value of two options.
///
/// # Panics
///
/// This function will panic if none of the options are provided.
///
/// # Examples
///
/// ```
/// use replace::verify_at_least_one_option_is_provided;
///
/// // This will not panic
/// verify_at_least_one_option_is_provided(vec![
///     (("haystack", Some("value")), ("stdin", None)),
///     (("option1", None), ("option2", None)),
/// ]);
///
/// // This will panic
/// // verify_at_least_one_option_is_provided(vec![
/// //     (("haystack", None), ("stdin", None)),
/// //     (("option1", None), ("option2", None)),
/// // ]);
/// ```
pub fn verify_at_least_one_option_is_provided(
    options: Vec<((&str, Option<&str>), (&str, Option<&str>))>,
) {
    let at_least_one_provided = options
        .iter()
        .any(|((_, opt1), (_, opt2))| opt1.is_some() || opt2.is_some());

    if !at_least_one_provided {
        let option_names: Vec<&str> = options
            .iter()
            .flat_map(|((name1, _), (name2, _))| vec![*name1, *name2])
            .collect();
        panic!(
            "error: at least one option must be provided: {}",
            option_names.join(", ")
        );
    }
}

/// Finds all matches of the given pattern in the content string.
///
/// # Arguments
///
/// * `pattern` - A regex pattern to match.
/// * `content` - The string to search within.
///
/// # Returns
///
/// A vector of tuples where each tuple contains the start and end indices of a match.
///
/// # Examples
///
/// ```
/// use replace::find_matches;
/// let matches = find_matches(r"\d+", "123 abc 456");
/// assert_eq!(matches, vec![(0, 3), (8, 11)]);
/// ```
pub fn find_matches(pattern: &str, content: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(pattern).expect(&format!("Invalid regex pattern: {}", pattern));
    re.find_iter(content)
        .map(|found_match| (found_match.start(), found_match.end()))
        .collect()
}
