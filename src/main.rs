use colored::Colorize;
use std::env;
use std::process;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

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

fn main() -> Result<(), std::io::Error> {
    // Default to quiet and seconds
    let mut verbosity: Verbosity = Verbosity::Quiet;
    let mut mode: Mode = Mode::Seconds;

    // Default to printing
    let mut action = Action::Print;

    // Default to printing if no args are given
    if env::args().len() == 1 {
        print_time(verbosity, mode);
        process::exit(0);
    }

    // Match the arguments
    for arg in env::args() {
        match arg.as_str() {
            "-h" | "--help" => action = Action::Help,
            "-V" | "--version" => action = Action::Version,
            "-s" | "--seconds" => mode = Mode::Seconds,
            "-m" | "--milliseconds" => mode = Mode::Milliseconds,
            "-u" | "--microseconds" => mode = Mode::Microseconds,
            "-n" | "--nanoseconds" => mode = Mode::Nanoseconds,
            "-v" | "--verbose" => verbosity = Verbosity::Verbose,
            &_ => (),
        }
    }

    match action {
        Action::Help => print_help(),
        Action::Print => print_time(verbosity, mode),
        Action::Version => print_version(),
    }
    Ok(())
}

// Printing the time in a variety of formats
fn print_time(verbosity: Verbosity, mode: Mode) {
    // Getting the time
    let epoch = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n,
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
fn print_version() {
    println!(
        "{} version {}",
        "epoch-get".green(),
        env!("CARGO_PKG_VERSION").bold()
    );
}

// Printing the help menu
fn print_help() {
    print_version();
    println!("\n{}", "USAGE:".yellow());
    println!("\tepoch-get {}", "[OPTIONS]".bold());
    println!("");
    println!("{}", "OPTIONS:".yellow());
    println!("\t{}", "-h, --help".green());
    println!("\t\tPrint this help menu.");
    println!("");
    println!("\t{}", "-V, --version".green());
    println!("\t\tPrint the program version.");
    println!("");
    println!("\t{}", "-v, --verbose".green());
    println!("\t\tBe verbose when printing the time.");
    println!("\t\tCan be combined with any of the following.");
    println!("");
    println!("\t{} - default", "-s, --seconds".green());
    println!("\t\tPrint the value in seconds.");
    println!("");
    println!("\t{}", "-m, --milliseconds".green());
    println!("\t\tPrint the value in milliseconds.");
    println!("");
    println!("\t{}", "-u, --microseconds".green());
    println!("\t\tPrint the value in microseconds.");
    println!("");
    println!("\t{}", "-n, --nanoseconds".green());
    println!("\t\tPrint the value in nanoseconds.");
    println!("");
}
