use clap::{crate_version, crate_name, crate_description, crate_authors, App, AppSettings, Arg};
use log::info;
use std::env;
use lindera_wiki_tokenize::tokenizer::tokenize;


fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let app = App::new(crate_name!())
        .setting(AppSettings::DeriveDisplayOrder)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .help_message("Prints help information.")
        .version_message("Prints version information.")
        .version_short("v")
        .arg(
            Arg::with_name("INPUT_DIR")
                .help("The directory where JSON files made wiki-extractor-rs containing. Support only *.json files.")
                .value_name("INPUT_DIR")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("OUTPUT_DIR")
                .help("The directory that has the output tokens.")
                .value_name("OUTPUT_DIR")
                .required(true)
                .takes_value(true),
        );
    let matches = app.get_matches();
    let input_dir = matches.value_of("INPUT_DIR").unwrap();
    let output_dir = matches.value_of("OUTPUT_DIR").unwrap();

    match tokenize(input_dir, output_dir) {
        Ok(()) => {
            info!("{}", "done");
        }
        Err(msg) => info!("{}", msg),
    }
}
