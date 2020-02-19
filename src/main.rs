#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod args;
mod consts;

use pulldown_cmark::{html, Event, Options, Parser, Tag};

use std::fs;

fn build_title(title_count: u32, title: &'_ str) -> String {
    format!(
        "\t\t\t<h1 class=\"table-of-content-title\">{}. {}</h1>\n",
        title_count, title,
    )
}

fn build_subtitle(title_count: u32, subtitle_count: u32, title: &'_ str, index: bool) -> String {
    if index {
        format!(
            "\t\t\t<h2 class=\"table-of-content-subtitle\">{}.{}. {}</h2>\n",
            title_count,
            subtitle_count.to_string(),
            title,
        )
    } else {
        format!("\t\t\t<h2 class=\"table-of-content-subtitle\">{}</h1>\n", title)
    }
}

fn parse_name(path: &'_ str) -> (String, String) {
    let mut path_parts: Vec<String> = path.split('/').map(String::from).collect();
    let file = path_parts.last().expect("path is empty.").clone();
    path_parts.pop();

    let (file_name, with_ext) = {
        if file.contains('.') {
            let split = file.split('.').collect::<Vec<&str>>()[0];
            (split.to_string(), split.to_string() + ".html")
        } else {
            (file.to_string(), file + ".html")
        }
    };

    path_parts.push(with_ext);

    (file_name, path_parts.join("/"))
}

fn main() {
    let args = args::parse();

    let file_content = fs::read_to_string(&args.file).unwrap();
    let (name, output_name) = parse_name(&args.file);

    let mut options = Options::empty();
    options.insert(Options::all());
    // TODO: not make two iters here?
    let mut toc_parser = Parser::new_ext(&file_content, options).peekable();
    let file_parser = Parser::new_ext(&file_content, options);

    let mut doc = {
        let mut result = consts::html_start({if args.title.is_empty() { &name } else { &args.title } }, &args.lang);
        // Create table of contents
        result.push_str(&{
            let mut toc = String::new();

            let mut title_count = 0;
            let mut subtitle_count = 0;

            while let Some(event) = toc_parser.next() {
                if let Event::Start(Tag::Heading(level @ 1..=2)) = event {
                    let title = match toc_parser.peek().unwrap() {
                        Event::Text(cowstr) => cowstr.clone().into_string(),
                        _ => panic!("no text following heading"),
                    };

                    if level == 1 {
                        title_count += 1;
                        subtitle_count = 0;
                        toc.push_str(&build_title(title_count, &title));
                    } else {
                        subtitle_count += 1;
                        toc.push_str(&build_subtitle(
                            title_count,
                            subtitle_count,
                            &title,
                            args.index_subtitles,
                        ));
                    }
                }
            }
            toc
        });

        html::push_html(&mut result, file_parser);
        result
    };

    doc = format!(
        "{}\n{}",
        doc,
        consts::HTML_END,
    );

    match fs::write(&output_name, doc) {
        Ok(_) => println!("Sucessfully wrote to {}.", output_name),
        Err(err) => println!("! Failed to write to {} with: {}", output_name, err),
    }
}
