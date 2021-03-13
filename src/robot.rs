use crate::board::Board;
use crate::player::Player;

use std::thread;
use std::io::stdin;

pub struct Robot {
	symbol: char,
	opponent_symbol: char
}

impl Robot {

	pub fn new(symbol: char, opponent_symbol: char) -> Self {
		Robot{
			symbol,
			opponent_symbol
		}
	}

	fn search(&self, board: &Board, lookahead: i32, my_turn: bool) -> [i32; 7] {
		let mut weights: [i32; 7]=[0; 7];
		if lookahead<9 {
			for i in 0..weights.len() {
				let mut b=board.clone();
				let mut valid_play=true;
				if my_turn {
					valid_play=b.drop_piece(self.symbol, i as i32);
				} else if !my_turn {
					valid_play=b.drop_piece(self.opponent_symbol, i as i32);
				}
				if !valid_play {
					weights[i]=0;
				} else {
					match b.check_winner() {
						Some(w) => {
							if w==self.symbol {
								weights[i]=1;
							} else {
								weights[i]=-2;
							};
							break;
						},
						None => {
							let w=self.search(&b, lookahead+1, !my_turn);
							for j in 0..w.len() {
								weights[i]+=w[j];
							}
						}
					}
				}
			}
		}
		return weights;
	}
}

impl Player for Robot {

	fn play(&self, board: &mut Board) {
		let weights=self.search(&board, 0, true);
		//println!("{:?}", weights);
		//let mut s=String::new();
		//stdin().read_line(&mut s);
		let mut index: i32=0;
		let mut score: i32=i32::MIN;
		for i in 0..weights.len() {
			if weights[i]>score && !board.is_col_full(i) {
				score=weights[i];
				index=i as i32;
			}
		}
		board.drop_piece(self.get_symbol(), index);
	}

	fn get_symbol(&self) -> char {
		self.symbol
	}

}
