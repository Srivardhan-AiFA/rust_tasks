use std::env;
use std::process;

use minigrep::Config;
use minigrep::search_content_case_insenstive;
use minigrep::search_content_case_senstive;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Unable to get file contents: {}", err);
        process::exit(1)
    });

    // println!("Searching for {}", config.search_string);
    // println!("In file {}", config.file_name);

    let file_contents = minigrep::get_file_contents(&config.file_name).unwrap_or_else(|err| {
        eprintln!("Unable to get file contents: {}", err);
        process::exit(1)
    });
    let result = if config.case_sensitive {
        search_content_case_senstive(&config.search_string, &file_contents)
    } else {
        search_content_case_insenstive(&config.search_string, &file_contents)
    };
    for i in result.iter() {
        println!("{}", i)
    }
}
