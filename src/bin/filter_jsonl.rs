#[macro_use]
extern crate log;
extern crate clap;
extern crate jmespath;
use std::io;
use std::rc::Rc;
use std::fs::File;
use std::path::Path;
use std::process::exit;
use jmespath::Expression;
use jmespath::Variable;
use clap::{App, Arg, ArgMatches};
use std::io::{BufRead, BufReader};

static VERSION: &'static str = "0.1.1";


fn make_app<'a, 'b>() -> App<'a, 'b> {
    let source_arg = Arg::with_name("source")
        .short("s")
        .long("source")
        .value_name("PATH")
        .help("The source of the jsonl file.")
        .takes_value(true);

    let filter_arg = Arg::with_name("filter")
        .short("f")
        .long("filter")
        .value_name("JMES_FILTER")
        .help("The JMESPath filter to use.")
        .takes_value(true);

    let delimiter_arg = Arg::with_name("delimiter")
        .short("d")
        .long("delimiter")
        .value_name("DELIMITER")
        .help("The delimiter to use if result is an array.")
        .takes_value(true);

    let bool_arg = Arg::with_name("bool_expr")
        .short("b")
        .long("bool_expr")
        .help("JMES Query as bool only. (Prints whole record if true.)");

    let verbose = Arg::with_name("debug")
        .short("-d")
        .long("debug")
        .value_name("DEBUG")
        .takes_value(true)
        .possible_values(&["Off", "Error", "Warn", "Info", "Debug", "Trace"])
        .help("Debug level to use.");

    App::new("filter_jsonl")
        .version(VERSION)
        .author("Matthew Seyer <https://github.com/forensicmatt/JsonlTools>")
        .about("Tool to filter JSONL with JMESPath queries.")
        .arg(source_arg)
        .arg(delimiter_arg)
        .arg(filter_arg)
        .arg(bool_arg)
        .arg(verbose)
}


fn get_expression<'a>(options: &ArgMatches) -> Option<Expression<'a>> {
    match options.is_present("filter") {
        true => {
            match options.value_of("filter") {
                Some(expr_str) => {
                    match jmespath::compile(expr_str) {
                        Ok(expr) => Some(expr),
                        Err(error) => {
                            eprintln!("Error compiling JMESPath expression: {}", error);
                            exit(-1);
                        }
                    }
                },
                None => {
                    None
                }
            }
        },
        false => None
    }
}


fn array_to_text(result: Rc<jmespath::Variable>, delimiter: &str) -> String {
    match &*result {
        Variable::Array(array) => {
            let mut string_list: Vec<String> = Vec::with_capacity(
                array.len()
            );

            for item in array {
                let text = match item.as_string() {
                    Some(s) => s.to_owned(),
                    None => format!("{}", item)
                };

                string_list.push(
                    text
                );
            }

            return string_list.join(delimiter);
        },
        _ => {
            return "".to_string()
        }
    };
}


fn process_line(jmes_expr: &Option<Expression>, line_str: &str, options: &ArgMatches) {
    match jmes_expr {
        Some(ref expr) => {
            let data = jmespath::Variable::from_json(&line_str).unwrap();
            let result = match expr.search(data) {
                Ok(r) => r,
                Err(error) => {
                    eprintln!("Search error: {}", error);
                    exit(-1);
                }
            };

            match options.is_present("bool_expr") {
                true => {
                    match result.as_boolean() {
                        Some(bool_value) => {
                            match bool_value {
                                true => println!("{}",line_str),
                                false => {}
                            }
                        },
                        None => {
                            panic!("Query expression is not a bool expression!");
                        }
                    }
                },
                false => {
                    if result.is_array() {
                        let output = array_to_text(
                            result, 
                            options.value_of("delimiter").unwrap_or("\t")
                        );
                        println!("{}", &output);
                    } else {
                        println!("{}", result);
                    }
                }
            }
        },
        None => {
            println!("{}", line_str);
        }
    }
}


fn process_file(file_location: &str, options: &ArgMatches) {
    info!("processing {}", file_location);

    let file = match File::open(file_location) { 
        Ok(fh) => fh,
        Err(error) => {
            eprintln!("{}", error);
            exit(-1);
        } 
    };

    let jmes_expr = get_expression(&options);

    for line in BufReader::new(file).lines() {
        match line {
            Ok(line_str) => {
                process_line(&jmes_expr, &line_str, &options);
            },
            Err(error) => {
                eprintln!("{}", error);
                exit(-1);
            }
        }
    }
}


fn process_stdin(options: &ArgMatches) {
    let jmes_expr = get_expression(&options);

    for line in io::stdin().lock().lines() {
        match line {
            Ok(line_str) => {
                process_line(&jmes_expr, &line_str, &options);
            },
            Err(error) => {
                eprintln!("{}", error);
                exit(-1);
            }
        }
    }
}


fn main() {
    let app = make_app();
    let options = app.get_matches();

    match options.is_present("source") {
        true => {
            match options.value_of("source") {
                Some(path_location) => {
                    // Verify that the supplied path exists
                    if !Path::new(path_location).exists() {
                        eprintln!("{} does not exist.", path_location);
                        exit(-1);
                    }

                    process_file(path_location, &options);
                },
                None => {
                    eprintln!("filter_jsonl requires a source to parse.");
                    exit(-1);
                }
            }
        },
        false => {
            process_stdin(&options);
        }
    }
}
