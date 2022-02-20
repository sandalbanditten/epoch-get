use colored::Colorize;
use std::env;
use std::process;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

enum Verbosity {
    Verbose,
    Quiet,
}

enum Mode {
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
}

fn main() -> Result<(), std::io::Error> {
    // Default to quiet and seconds
    let mut verbosity: Verbosity = Verbosity::Quiet;
    let mut mode: Mode = Mode::Seconds;

    // Default to printing if no args are given
    if env::args().len() == 1 {
        print_time(verbosity, mode);
        process::exit(0);
    }

    let mut help: bool = false;

    for arg in env::args() {
        match arg.as_str() {
            "-h" => help = true,
            "-s" => mode = Mode::Seconds,
            "-m" => mode = Mode::Milliseconds,
            "-u" => mode = Mode::Microseconds,
            "-n" => mode = Mode::Nanoseconds,
            "-v" => verbosity = Verbosity::Verbose,
            &_ => (),
        }
    }

    // If -h is *any* flag, print the help menu
    if help {
        print_help();
        Ok(())
    } else {
        print_time(verbosity, mode);
        Ok(())
    }
}

fn print_time(verbosity: Verbosity, mode: Mode) {
    // Getting the time
    let epoch = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n,
        Err(_) => panic!("{} Unable to get system time!", "Error:".red().bold()),
    };

    // rustc complains - no idea why
    #[allow(unused_assignments)]
    let mut suffix = String::from("");
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

fn print_help() {
    println!("Help goes here! make it colorful, boi");
}
