#[macro_use]
extern crate clap;
extern crate semver;

mod recognize;

use std::str::from_utf8;
use semver::{Identifier, Version};
use recognize::{Alt, Inclusive, OneOrMore, Recognize};
use clap::{App, Arg};

// "safe" main function for returning status code.
fn safe_main() -> i32 {
    let matches = App::new(crate_name!())
                          .version(crate_version!())
                          .about("Semantic version parsing and manipulation. See http://semver.org/ for more information.")
                          .arg(Arg::with_name("VERSION")
                               .help("version to modify")
                               .required(true)
                               .validator(is_not_empty))
                          .arg(Arg::with_name("major")
                                .short("M")
                                .long("major")
                                .multiple(false)
                                .conflicts_with("patch")
                                .conflicts_with("minor")
                                .help("increase major version (sets minor and patch versions to 0)"))
                          .arg(Arg::with_name("minor")
                                .short("m")
                                .long("minor")
                                .multiple(false)
                                .conflicts_with("patch")
                                .help("increase minor version (sets patch version to 0)"))
                          .arg(Arg::with_name("patch")
                                .short("p")
                                .long("patch")
                                .multiple(false) 
                                .help("increase patch version"))
                          .arg(Arg::with_name("pre")
                                .long("pre")
                                .takes_value(true)
                                .multiple(true) 
                                .help("set pre-release version (alpha, beta, ...)"))
                          .arg(Arg::with_name("meta")
                                .long("meta")
                                .takes_value(true)
                                .multiple(true)
                                .help("set metadata of version (ci, commit, ...)"))
                          .get_matches();

    // Since VERSION is required(true), unwrap is safe.
    let input_version = matches.value_of("VERSION").unwrap();

    // Try to parse input version (validate correctness), if error print it and return non-zero status code.
    let mut version = match Version::parse(input_version) {
        Ok(version) => version,
        Err(e) => {
            println!("{}", e);
            return 1;
        }
    };

    // Increase major version.
    if matches.is_present("major") {
        version.increment_major();
    }

    // Increase minor version.
    if matches.is_present("minor") {
        version.increment_minor();
    }

    // Increase patch version.
    if matches.is_present("patch") {
        version.increment_patch();
    }

    // Modify pre-release.
    if matches.is_present("pre") {
        for value in matches.values_of("pre").unwrap() {
            push_identifier(&mut version.pre, value);
        }
    }

    // Modify metadata.
    if matches.is_present("meta") {
        for value in matches.values_of("meta").unwrap() {
            push_identifier(&mut version.build, value);
        }
    }

    println!("{}", version.to_string());
    0
}

// Validation for empty strings.
fn is_not_empty(val: String) -> Result<(), String> {
    if !val.is_empty() {
        Ok(())
    } else {
        Err(String::from("VERSION must not be empty"))
    }
}

// Deduplicate and push identifier into vec of identifiers.
fn push_identifier(identifiers: &mut Vec<Identifier>, value: &str) {
    let identifier = if is_alpha_numeric(value) {
        Identifier::AlphaNumeric(value.to_string())
    } else {
        Identifier::Numeric(value.parse().unwrap())
    };
    if !identifiers.contains(&identifier) {
        identifiers.push(identifier);
    }
}

pub fn is_alpha_numeric(s: &str) -> bool {
    if let Some((_val, len)) = numeric_identifier(s.as_bytes()) {
        // Return true for number with leading zero
        // Note: doing it this way also handily makes overflow fail over.
        len != s.len()
    } else {
        true
    }
}

// Note: could plumb overflow error up to return value as Result
pub fn numeric_identifier(s: &[u8]) -> Option<(u64, usize)> {
    if let Some(len) = Alt(b'0', OneOrMore(Inclusive(b'0'..b'9'))).p(s) {
        from_utf8(&s[0..len])
            .unwrap()
            .parse()
            .ok()
            .map(|val| (val, len))
    } else {
        None
    }
}

fn main() {
    let exit_code = safe_main();
    std::process::exit(exit_code);
}
