use colored::Colorize;
use std::{env, error};
use std::time::{SystemTime, UNIX_EPOCH};

// How verbose is the output
enum Verbosity {
    Verbose,
    Quiet,
}

// What suffix should come after the value
enum Mode {
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
}

// What should the program do
enum Action {
    Help,
    Print,
    Version,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    // Get the args
    let args = env::args();
    // Default to quiet, seconds and printing
    let mut verbosity = Verbosity::Quiet;
    let mut mode = Mode::Seconds;
    let mut action = Action::Print;

    // Match the arguments
    for arg in args {
        match arg.as_str() {
            "-v" | "--verbose" => verbosity = Verbosity::Verbose,
            "-s" | "--seconds" => mode = Mode::Seconds,
            "-m" | "-ms" | "--milliseconds" => mode = Mode::Milliseconds,
            "-u" | "-us" | "--microseconds" => mode = Mode::Microseconds,
            "-n" | "-ns" | "--nanoseconds" => mode = Mode::Nanoseconds,
            "-h" | "--help" => action = Action::Help,
            "-V" | "--version" => action = Action::Version,
            &_ => (),
        }
    }

    match action {
        Action::Help => print_help(),
        Action::Print => print_time(verbosity, mode),
        Action::Version => print_version(verbosity),
    }
    Ok(())
}

// Printing the time in a variety of formats
fn print_time(verbosity: Verbosity, mode: Mode) {
    // Getting the time
    let epoch = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration,
        Err(_) => panic!("{} Unable to get system time!", "Error:".red().bold()),
    };

    let suffix: String;
    // Redefining epoch and suffix as seconds etc.
    let epoch: u128 = match mode {
        Mode::Seconds => {
            suffix = String::from(" seconds");
            epoch.as_secs() as u128
        }
        Mode::Milliseconds => {
            suffix = String::from(" milliseconds");
            epoch.as_millis()
        }
        Mode::Microseconds => {
            suffix = String::from(" microseconds");
            epoch.as_micros()
        }
        Mode::Nanoseconds => {
            suffix = String::from(" nanoseconds");
            epoch.as_nanos()
        }
    };

    match verbosity {
        Verbosity::Verbose => {
            println!("The Unix Epoch, 1970-01-01 00:00:00 UTC was {epoch}{suffix} ago!")
        }
        Verbosity::Quiet => println!("{epoch}"),
    };
}

// Printing the version number
fn print_version(verbosity: Verbosity) {
    match verbosity {
        Verbosity::Quiet => println!("{}", env!("CARGO_PKG_VERSION")),
        Verbosity::Verbose => println!("{}{}", "epoch-get ".green(), env!("CARGO_PKG_VERSION")),
    }
}

// Printing the help menu
fn print_help() {
    print_version(Verbosity::Verbose);
    println!();
    println!("{}", "USAGE:".yellow());
    println!("    epoch-get [OPTIONS]");
    println!();
    println!("{}", "OPTIONS:".yellow());
    println!("    {}", "-h, --help".green());
    println!("            Print this help menu.");
    println!();
    println!("    {}", "-V, --version".green());
    println!("            Print the program version.");
    println!();
    println!("    {}", "-v, --verbose".green());
    println!("            Be verbose when printing the time.");
    println!("            Can be combined with any of the following arguments:");
    println!();
    println!("    {} - default", "-s, --seconds".green());
    println!("            Print the value in seconds.");
    println!();
    println!("    {}", "-m, -ms, --milliseconds".green());
    println!("            Print the value in milliseconds.");
    println!();
    println!("    {}", "-u, -us, --microseconds".green());
    println!("            Print the value in microseconds.");
    println!();
    println!("    {}", "-n, -ns, --nanoseconds".green());
    println!("            Print the value in nanoseconds.");
}
