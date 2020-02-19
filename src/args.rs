use argparse::{ArgumentParser, Store, StoreFalse};

#[derive(Clone)]
pub struct Arguments {
    pub file: String,
    pub index_subtitles: bool,
    pub lang: String,
}

pub fn parse() -> Arguments {
    let mut args = Arguments {
        file: String::new(),
        index_subtitles: true,
        lang: String::from("EN"),
    };

    // Rust won't let you return args while there's a living mutable reference to it.
    // Putting the parser in its own block forces the references to be dropped before returning.
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("A dead-simple static site generator.");
        parser.refer(&mut args.index_subtitles).add_option(
            &["-i", "--no-index-subtitles"],
            StoreFalse,
            "don't prepend indexes of subtitles",
        );

        parser.refer(&mut args.lang).add_option(
            &["-l", "--lang"],
            Store,
            "set the <html> lang attribute (default: EN)",
        );

        parser
            .refer(&mut args.file)
            .add_argument("file", Store, "the Markdown file to parse")
            .required();
        parser.parse_args_or_exit();
    }

    if std::fs::metadata(&args.file).is_err() {
        println!("Provided file doesn't exist.");
        std::process::exit(2);
    }

    args
}
