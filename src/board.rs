#[derive(Debug, Clone)]
pub struct Board {
	pieces: u32,
	grid: [char; 42]
}

impl Board {

	pub fn drop_piece(&mut self, colour:char, position:i32) -> bool {
		let mut i=position+35;
		loop {
			if i<0 {
				break;
			}
			if self.grid[i as usize]=='0' {
				self.grid[i as usize]=colour;
				return true;
			}
			i-=7;
		}
		return false;
	}

	pub fn check_winner(&self) -> Option<char> {
		let lines: [[i32;7]; 25] = [
			//Horizontals
			[0,1,2,3,4,5,6],
			[7,8,9,10,11,12,13],
			[14,15,16,17,18,19,20],
			[21,22,23,24,25,26,27],
			[28,29,30,31,32,33,34],
			[35,36,37,38,39,40,41],
			//Verticals
			[0,7,14,21,28,35,-1],
			[1,8,15,22,29,36,-1],
			[2,9,16,23,30,37,-1],
			[3,10,17,24,31,38,-1],
			[4,11,18,25,32,39,-1],
			[5,12,19,26,33,40,-1],
			[6,13,20,27,34,41,-1],
			//Diagonals left to right
			[3,11,19,27,-1,-1,-1],
			[2,10,18,26,34,-1,-1],
			[1,9,17,25,33,41,-1],
			[0,8,16,24,32,40,-1],
			[7,15,23,31,39,-1,-1],
			[14,22,30,38,-1,-1,-1],
			//Diagonals right to left
			[3,9,15,21,-1,-1,-1],
			[4,10,16,22,28,-1,-1],
			[5,11,17,23,29,35,-1],
			[6,12,18,24,30,36,-1],
			[13,19,25,31,37,-1,-1],
			[20,26,32,36,-1,-1,-1],

		];
		for i in 0..lines.len() {
			let mut c=1;
			let mut p='0';
			for j in 0..lines[i].len() {
				let index=lines[i][j];
				if index>=0 {
					let v=self.grid[index as usize];
					if p==v {
						c+=1;
					} else {
						c=1;
						p=v;
					}
					if c==4 && p!='0' {
						return Some(p);
					}
				}
			}
		}
		return None;
	}

	pub fn is_full(&self) -> bool {
		self.pieces==6*7
	}

	pub fn is_col_full(&self, col: usize) -> bool {
		self.grid[col]!='0'
	}

	pub fn get_grid(&self) -> &[char;6*7] {
		&self.grid
	}

	pub fn new() -> Self {
		Board {
			pieces: 0,
			grid: ['0';6*7]
		}
	}

}
