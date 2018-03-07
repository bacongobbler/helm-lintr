extern crate clap;
#[macro_use]
extern crate log;
extern crate simple_logger;

use clap::{Arg, App};
use log::Level;

fn main() {
    let log_level;

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
    // (i.e. 'helm-lintr -v -v -v' or 'helm-lintr -vvv'
    match matches.occurrences_of("v") {
        0 => log_level = Level::Error,
        1 => log_level = Level::Warn,
        2 => log_level = Level::Info,
        3 => log_level = Level::Debug,
        4 | _ => log_level = Level::Trace,
    }

    simple_logger::init_with_level(log_level).unwrap();

    // Gets a value for config if supplied by user, or defaults to "lintr.conf"
    let config = matches.value_of("config").unwrap_or("lintr.conf");
    debug!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "PATH" is required
    debug!("Using chart file: {}", matches.value_of("PATH").unwrap());
}
