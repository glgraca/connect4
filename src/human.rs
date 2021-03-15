use std::io::{stdin, stdout, Write};

use crate::board::Board;
use crate::player::Player;

pub struct Human {
	symbol: char
}

impl Human {

	pub fn new(symbol: char) -> Self {
		Human{
			symbol
		}
	}

}

impl Player for Human {

	fn get_symbol(&self) -> char {
		self.symbol
	}

	fn play(&mut self, board: &mut Board) {
		let mut s=String::new();
		let mut col:i32=0;
		loop {
			print!("Please enter a column number (1-7): ");
			let _=stdout().flush();
			s="".to_string();
			stdin().read_line(&mut s).expect("Did not enter a correct value");

			match s.chars().next().unwrap().to_digit(10) {
				Some(c) => {
					if c>0 && c<8 {
						col=c as i32;
						break;
					}
				},
				None => ()
			}
		}
		board.drop_piece(self.get_symbol(),col-1);
	}

}
