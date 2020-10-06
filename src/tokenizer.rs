use crate::document::Document;
use glob::glob;
use lindera::tokenizer::Tokenizer;
use log::info;
use rayon::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

struct TokenMaker {
    output_prefix: String,
    file: File,
    counter: u16,
    page_limit: u16,
    file_counter: u16,
}

impl TokenMaker {
    fn new(output_dir: &str, filename: &str) -> Self {
        let output_prefix = format!("{}/{}", output_dir, filename).replace(".json", "");
        let file_path = format!("{}_0.txt", &output_prefix);
        Self {
            output_prefix,
            file: OpenOptions::new()
                .write(true)
                .create(true)
                .open(&file_path)
                .expect(format!("can't open file[{}] with write option", &file_path).as_str()),
            counter: 0,
            page_limit: 10000,
            file_counter: 0,
        }
    }

    pub fn output(&mut self, tokens: &Vec<&str>) {
        writeln!(self.file, "{:?}", tokens).expect("write error.");
        self.counter += 1;
        if self.counter == self.page_limit {
            self.file.flush().expect("Flush error.");
            self.file_counter += 1;
            let path = format!("{}_{}.txt", self.output_prefix, self.file_counter);
            self.file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(path.as_str())
                .expect(format!("can't open file[{}] with write option", path.as_str()).as_str());
            self.counter = 0;
        }
    }

    pub fn flush(&mut self) {
        self.file.flush().expect("Flush error.");
    }
}

fn make_tokens(filepath: &str, token_maker: &mut TokenMaker) -> Result<String, String> {
    info!("Start tokenizing {}...", filepath);
    let mut tokenizer = Tokenizer::new("normal", "");
    for line in BufReader::new(File::open(filepath).unwrap()).lines() {
        let line = line.unwrap();
        let doc = Document::new(line.as_str());
        let target = format!(
            "{} {} {}",
            doc.title,
            doc.headings.join(" "),
            doc.contents.join(" ")
        );
        let tokens = tokenizer.tokenize_str(target.as_str());
        token_maker.output(&tokens);
    }
    token_maker.flush();
    Ok(format!("Finish {}", filepath))
}

pub fn tokenize(input_dir: &str, output_dir: &str) -> Result<(), String> {
    let input_files = Path::new(input_dir).join(Path::new("**/*.json"));
    let files: Vec<_> = glob(input_files.to_str().unwrap())
        .unwrap()
        .filter_map(|x| x.ok())
        .collect();
    files
        .par_iter()
        .map(|filepath| {
            let filename = filepath.file_name().unwrap().to_str().unwrap();
            let mut token_maker = TokenMaker::new(output_dir, filename);
            make_tokens(filepath.to_str().unwrap(), &mut token_maker)
        })
        .filter_map(|x| x.ok())
        .collect::<String>();
    Ok(())
}
