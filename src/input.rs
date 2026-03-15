use std::fs::File;
use std::io::{self, BufRead, BufReader, IsTerminal, Read};

use std::sync::mpsc::Sender;

use crate::data_parsing::DataParser;

use crate::cli::Cli;

pub fn read_data(parser: &DataParser, tx: &Sender<Vec<f64>>, cli: &Cli) {
    let input: Box<dyn Read> = match &cli.file {
        Some(path) => Box::new(File::open(path).expect("failed to open file!")),
        None => Box::new(io::stdin()),
    };

    let mut reader = BufReader::new(input);

    let pipe_output = !io::stdout().is_terminal();

    let mut line = String::new();

    while reader.read_line(&mut line).expect("failed to read line") != 0 {
        if pipe_output {
            print!("{}", &line);
        }

        if line.ends_with('\n') {
            line.pop(); // remove '\n'
            if line.ends_with('\r') {
                line.pop(); // handle Windows "\r\n"
            }
        }

        if let Ok(data) = parser.parse_line(&line, cli.x, &cli.y) {
            match tx.send(data) {
                Ok(_) => (),
                Err(_) => return,
            }
        }

        line.clear();
    }
}
