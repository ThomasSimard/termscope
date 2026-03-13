use std::io::{self, BufRead, IsTerminal};
use std::sync::mpsc::{Sender};

use crate::DataPoint;
use crate::data_parsing::DataParser;

use crate::cli::Cli;

pub fn read_data(parser: &DataParser, tx: &Sender<DataPoint>, cli: &Cli) {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line: String = line.expect("failed to read line");

        if !io::stdout().is_terminal() {
            println!("{}", &line);
        }

        if let Ok(Some(data)) = parser.parse_line(line, cli.x, cli.y){
            match tx.send(data) {
                Ok(_) => (),
                Err(_) => return,
            }
        }
    }
}
