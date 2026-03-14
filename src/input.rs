use std::io::{self, BufRead, BufReader, IsTerminal, Read};
use std::fs::File;

use std::sync::mpsc::{Sender};

use crate::data_parsing::DataParser;

use crate::cli::Cli;

pub fn read_data(parser: &DataParser, tx: &Sender<Vec<f64>>, cli: &Cli) {
    let input: Box<dyn Read> = match &cli.file {
        Some(path) => Box::new(File::open(path).expect("failed to open file!")),
        None => Box::new(io::stdin()),
    };

    let reader = BufReader::new(input);

    for line in reader.lines() {
        let line: String = line.expect("failed to read line");

        if !io::stdout().is_terminal() {
            println!("{}", &line);
        }

        if let Ok(data) = parser.parse_line(line, cli.x, &cli.y){
            match tx.send(data) {
                Ok(_) => (),
                Err(_) => return,
            }
        }
    }
}
