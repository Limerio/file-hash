use clap::{Arg, ArgAction, Command};
use inquire::{Select, Text};
use ring::digest;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let cmd = Command::new("file-hash")
        .bin_name("file-hash")
        .version("0.1.0")
        .arg(
            Arg::new("path")
                .short('p')
                .required(false)
                .action(ArgAction::Set)
                .help("Absolute path to a file"),
        )
        .arg(
            Arg::new("algorithm")
                .short('a')
                .required(false)
                .help("Choice an algorithm in the list")
                .action(ArgAction::Set)
                .value_parser(["SHA256", "SHA384", "SHA512"]),
        );

    let matches = cmd.get_matches();
    let path = matches.get_one::<String>("path");

    let filename = if !path.is_none() {
        path.unwrap().to_string()
    } else {
        Text::new("What is the path?").prompt().unwrap()
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let algorithm_arg = matches.get_one::<String>("algorithm");

    let algorithm = if !algorithm_arg.is_none() {
        let algorithm_result = if algorithm_arg.unwrap() == "SHA384" {
            &digest::SHA384
        } else if algorithm_arg.unwrap() == "SHA512" {
            &digest::SHA512
        } else {
            &digest::SHA256
        };

        algorithm_result
    } else {
        let choice = Select::new("Choice an algorithm", vec!["SHA256", "SHA384", "SHA512"])
            .prompt()
            .unwrap();

        if choice == "SHA384" {
            &digest::SHA384
        } else if choice == "SHA512" {
            &digest::SHA512
        } else {
            &digest::SHA256
        }
    };

    let hash = digest::digest(algorithm, &contents);
    println!("Result: {:?}", hash)
}
