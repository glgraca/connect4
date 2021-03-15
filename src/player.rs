use crate::board::Board;

use rand::Rng;

pub trait Player {

	fn play(&mut self, board: &mut Board) {
		let mut rng = rand::thread_rng();
		loop {
			if board.drop_piece(self.get_symbol(),rng.gen_range(0..7)) {
				break;
			}
		}
	}

	fn get_symbol(&self) -> char;

}
