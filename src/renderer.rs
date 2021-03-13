use crate::board::Board;

pub struct Renderer;

impl Renderer {

	pub fn new() -> Self {
		Renderer{}
	}

	pub fn cls(&self) {
		print!("\x1B[2J\x1B[1;1H");
	}

	pub fn show_board(&self, board: &Board) {
		let grid=board.get_grid();

		println!(" 1  2  3  4  5  6  7  ");
		for row in 0..6 {
			for col in 0..7 {
				let v=grid[(row*7)+col];
				print!("[");
				if v=='*' {
					print!("\x1b[1;31m"); //Red
				} else {
					print!("\x1b[1;33m"); //Yellow
				}
				print!("{}", if v=='0' { ' ' } else { v });
				print!("\x1b[1;0m");
				print!("]");
			}
			println!("");
		}
	}

}
