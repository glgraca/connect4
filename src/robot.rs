use crate::board::Board;
use crate::player::Player;

use std::thread;
use std::thread::JoinHandle;
use std::io::stdin;

pub struct Robot {
	symbol: char,
	opponent_symbol: char,
	max_lookahead: i32,
	first_move: bool
}

impl Robot {

	pub fn new(symbol: char, opponent_symbol: char) -> Self {
		Robot{
			symbol,
			opponent_symbol,
			max_lookahead: 4,
			first_move: true
		}
	}

}

fn search(symbol:char, opponent_symbol: char, board: &Board, lookahead: i32, max_lookahead: i32, my_turn: bool) -> [i32; 7] {
	let mut weights: [i32; 7]=[0; 7];
	if lookahead<max_lookahead {
		let mut handles=Vec::with_capacity(7);
		for i in 0..weights.len() {
			let mut b=board.clone();
			let mut valid_play=true;
			if my_turn {
				valid_play=b.drop_piece(symbol, i as i32);
			} else if !my_turn {
				valid_play=b.drop_piece(opponent_symbol, i as i32);
			}
			if !valid_play {
				weights[i]=0;
				if lookahead==0 {
					handles.push(None);
				}
			} else {
				match b.check_winner() {
					Some(w) => {
						if w==symbol {
							weights[i]=1;
						} else {
							weights[i]=-1;
						}
						if lookahead==0 {
							handles.push(None);
						}
					},
					None => {
						if lookahead==0 {
							handles.push(Some(thread::spawn(move || {
								search(symbol, opponent_symbol, &b, lookahead+1, max_lookahead, !my_turn)
							})));
						} else {
							let w=search(symbol, opponent_symbol, &b, lookahead+1, max_lookahead, !my_turn);
							for j in 0..w.len() {
								weights[i]+=w[j];
							}
						}
					}
				}
			}
		}
		if lookahead==0 {
			for i in 0..7 {
				match &handles[i] {
					Some(h) => {
						let w=handles[i].take().unwrap().join().unwrap();
						for j in 0..w.len() {
							weights[i]+=w[j];
						}
					},
					None => ()
				}
			}
		}
	}
	return weights;
}

impl Player for Robot {

	fn play(&mut self, board: &mut Board) {
		if self.first_move {
			board.drop_piece(self.get_symbol(), 3);
			self.first_move=false;
		} else {
			let weights=search(self.symbol, self.opponent_symbol, &board, 0, self.max_lookahead, true);
			if self.max_lookahead<9 {
				self.max_lookahead+=1;
			}
			//Print weights
			println!("{:?}", weights);
			let mut s=String::new();
			stdin().read_line(&mut s);
			//
			let mut index: i32=0;
			let mut score: i32=i32::MIN;
			for i in 0..weights.len() {
				//If Robot wins next play, weight is 1
				//If Human wins next play, weight is 0
				if weights[i].abs()<2 {
					index=i as i32;
					break;
				} else if weights[i]>score && !board.is_col_full(i) {
					score=weights[i];
					index=i as i32;
				}
			}
			board.drop_piece(self.get_symbol(), index);
		}
	}

	fn get_symbol(&self) -> char {
		self.symbol
	}

}
