use std::io::{self, BufRead, IsTerminal};
use std::sync::mpsc::{Sender};

use crate::DataPoint;
use crate::data_parsing::Parser;

pub fn read_data(parser: &Parser, tx: &Sender<DataPoint>) {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line: String = line.expect("failed to read line");

        if !io::stdout().is_terminal() {
            println!("{}", &line);
        }

        if let Ok(Some(data)) = parser.parse_line(line){
            match tx.send(data) {
                Ok(_) => (),
                Err(_) => return,
            }
        }
    }
}
