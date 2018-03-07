extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("helm-lintr")
                          .version("0.1.0")
                          .author("Matthew Fisher <matt.fisher@microsoft.com>")
                          .about("Runs a series of tests to verify that a Helm chart is well-formed.")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Specify a configuration file")
                               .takes_value(true))
                          .arg(Arg::with_name("PATH")
                               .help("The path to the chart")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // Gets a value for config if supplied by user, or defaults to "lintr.conf"
    let config = matches.value_of("config").unwrap_or("lintr.conf");
    println!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "PATH" is required
    println!("Using chart file: {}", matches.value_of("PATH").unwrap());
}
