mod consts;

use pulldown_cmark::{Parser, Options, html};
use std::{process, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Bad args.");
        process::exit(1);
    }
    let file_content = fs::read_to_string(&args[1]).unwrap();

    let output_name = {
        let mut path_parts: Vec<&str> = args[1].split('/').collect();
        let file = path_parts.last().expect("path is empty.").clone();
        path_parts.pop();

        let file_name = {
            if file.contains(".") {
                file.split(".").collect::<Vec<&str>>()[0]
            } else {
                file 
            }
        }.to_owned() + ".html";
        path_parts.push(&file_name);

        path_parts.join("/").clone()
    };

    let mut options = Options::empty();
    options.insert(Options::all());
    let parser = Parser::new_ext(&file_content, options);
    let mut result = String::new();
    html::push_html(&mut result, parser);

    result.push_str(consts::CSS);

    fs::write(output_name, result);
    println!("Done.");
}