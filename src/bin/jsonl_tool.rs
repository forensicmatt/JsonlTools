#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;
use std::io::BufRead;
use clap::{App, Arg};
use log::LevelFilter;
use jsonl::input::Input;
use jsonl::errors::CustomError;
use jsonl::jsonl::JsonlHandler;
use jsonl::text::TextHandler;

static VERSION: &'static str = "0.1.0";


fn make_app<'a, 'b>() -> App<'a, 'b> {
    let source_arg = Arg::with_name("source")
        .short("s")
        .long("source")
        .value_name("PATH")
        .help("The source of the jsonl file. (Omit this to use STDIN as the source)")
        .takes_value(true);

    let filter_arg = Arg::with_name("filter")
        .short("f")
        .long("filter")
        .value_name("JMES_FILTER")
        .help("The JMESPath filter to use.")
        .multiple(true)
        .takes_value(true);

    let text_arg = Arg::with_name("text")
        .short("t")
        .long("text")
        .value_name("TEXT_PATTERN")
        .help("The text pattern to use. (Must be a JMESPath query that results in an array)")
        .takes_value(true);

    let delimiter_arg = Arg::with_name("delimiter")
        .short("d")
        .long("delimiter")
        .value_name("DELIMITER")
        .help("The delimiter to use if result is an array.")
        .takes_value(true);

    let verbose = Arg::with_name("debug")
        .long("debug")
        .value_name("DEBUG")
        .takes_value(true)
        .possible_values(&["Off", "Error", "Warn", "Info", "Debug", "Trace"])
        .help("Debug level to use.");

    App::new("jsonl_tool")
        .version(VERSION)
        .author("Matthew Seyer <https://github.com/forensicmatt/JsonlTools>")
        .about("Tool to filter and format JSONL with JMESPath queries.")
        .arg(source_arg)
        .arg(delimiter_arg)
        .arg(filter_arg)
        .arg(text_arg)
        .arg(verbose)
}


/// Set the debug level based off of a string
pub fn set_debug_level(level: &str) -> Result<(), CustomError> {
    let level_filter = match level {
        "Off" => LevelFilter::Off,
        "Error" => LevelFilter::Error,
        "Warn" => LevelFilter::Warn,
        "Info" => LevelFilter::Info,
        "Debug" => LevelFilter::Debug,
        "Trace" => LevelFilter::Trace,
        unknown => {
            return Err(CustomError::cli_error(&format!(
                "Unknown debug level [{}]",
                unknown
            )));
        }
    };

    // Create logging with debug level that prints to stderr
    let result = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level_filter)
        .chain(std::io::stderr())
        .apply();

    // Ensure that logger was dispatched
    match result {
        Ok(_) => {
            trace!("Logging as been initialized!");
        }
        Err(error) => {
            return Err(CustomError::cli_error(&format!(
                "Error initializing fern logging: {}",
                error
            )));
        }
    }

    Ok(())
}


fn main() {
    let app = make_app();
    let options = app.get_matches();

    let str_filter_vec = match options.values_of("filter") {
        Some(values) => {
            values.map(|x| x.to_string()).collect()
        },
        None => Vec::new()
    };

    let handler = JsonlHandler::new(
        str_filter_vec
    ).expect("Error creating JsonlHandler");

    let stdin = std::io::stdin();

    let input = match options.value_of("source") {
        Some(p) => Input::from_file(p).expect("Error opening source"),
        None => Input::from_stdin(&stdin)
    };

    let delimiter = options.value_of("delimiter").unwrap_or("\t");

    let opt_text_handler = match options.value_of("text") {
        Some(p) => {
            println!("p: {}", p);
            Some(
                TextHandler::new(
                    delimiter.to_string(),
                    p.to_string()
                ).expect("Error creating TextHandler")
            )
        },
        None => None
    };

    for line_result in input.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };

        let line_value = serde_json::from_str(&line).expect("Error parsing json line");

        if handler.pass(&line_value) {
            let display = match opt_text_handler {
                Some(ref text_handler) => {
                    text_handler.format_value(&line_value).expect("Error formatting text")
                },
                None => {
                    line
                }
            };

            println!("{}", display);
        }
    }
}
