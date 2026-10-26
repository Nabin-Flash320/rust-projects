
use std::env;
use std::path;
use std::io;
use std::io::BufRead;
use std::fs;

const SUPPORTED_COMMANDS: [&str; 1] = ["n"];

struct RSCommands {
    args: Vec<String>,          // Holds the arguments passed to the application
    flag_list: Vec<String>,     // This list holds all the flags one by one as a string
    filepath: String,
}

fn main() {
    handle_cat();
}

fn handle_cat() {
    println!("Starting cat");

    let mut rs_cat: RSCommands = RSCommands {
        args: env::args().collect(),
        flag_list: Vec::new(),
        filepath: String::new(),
    };
    
    // Collect arguments into a vector of Strings
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return;
    }

    rs_cat.flag_list = collect_options(&rs_cat.args[1..], rs_cat.flag_list);
    if rs_cat.flag_list.len() > 0 {
        // Now after all the options are collected, validate them if they are correct onese
        // if not exit immidiatley
        if !check_if_valid_options(&rs_cat.flag_list) {
            return;
        }

    }

    rs_cat.filepath = check_and_get_filepath(&rs_cat.args[1..], rs_cat.filepath);
    if rs_cat.filepath.len() > 0 {
        println!("filepath: {:?}", rs_cat.filepath);
    }


    // Start file reading process, starting from if file exist
    {
        let file_path: &path::Path = path::Path::new(&rs_cat.filepath);
        if !file_path.exists() {
            println!("Provided file path doesnot exist {:?}", file_path.to_str());
            return;
        }

        // Start reading file
        let file: fs::File = fs::File::open(file_path).expect("Failed to open file");
        let read_buffer: io::BufReader<fs::File> = io::BufReader::new(file);

        let mut line_count: u64 = 0;
        for line_result in read_buffer.lines() {
            let line_result = line_result.expect("Invalid");
            if SUPPORTED_COMMANDS.contains(&"n"){
                println!("{} {}", line_count, line_result);
                line_count = line_count + 1;
            }
        }

    }

}

fn collect_options(option_list: &[String], mut flag_list: Vec<String>) -> Vec<String> {

    // Extract all the individual options and popuilate the flag list
    for arg in option_list {
        if arg.starts_with('-') {

            let option_list = &arg[1..];
            if option_list.len() > 1 {
                for opt in option_list.chars() {
                    if !flag_list.contains(&opt.to_string()) {
                        flag_list.push(opt.to_string());
                    }
                }
            } else {
                if !flag_list.contains(&option_list.to_string()) {
                    flag_list.push(option_list.to_string());
                }
            }

        }
    }
    
    flag_list
}

fn check_and_get_filepath(option_list: &[String], mut filepath: String) -> String {

    for arg in option_list {
        if !arg.starts_with('-') {

            filepath = arg.clone();
            break;

        }
    }

    filepath

}

fn check_if_valid_options(flag_list: &Vec<String>) -> bool {

    let mut is_valid: bool = true;

    if flag_list.len() == 0 {
        is_valid = true;
    }
    else {
        for flag in flag_list {
    
            if !SUPPORTED_COMMANDS.contains(&flag.as_str()) {
                println!("Command {:?} not supported!!", flag);
                is_valid = false;
                break;
            }
    
        }
    }

    is_valid
}


