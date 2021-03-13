mod board;
mod renderer;
mod robot;
mod human;
mod player;

use board::Board;
use renderer::Renderer;
use player::Player;
use human::Human;
use robot::Robot;

fn main() {
	let r = Renderer::new();
	r.cls();
	println!("Starting Connect4");

    let mut b = Board::new();
    let human:&dyn Player = &mut Human::new('*');
    let robot:&dyn Player = &mut Robot::new('R', '*');
    let mut turn=true;
    let mut winner=' ';
    loop {
		r.cls();
		r.show_board(&b);
		if turn {
    		human.play(&mut b);
    	} else {
    		robot.play(&mut b);
    	}
    	turn=!turn;
    	match b.check_winner() {
    		Some(w) => {
    			winner=w;
    			break
    		},
    		None => ()
    	}
    }
	r.cls();
	r.show_board(&b);

	if winner!=' ' {
    	println!("Winner is {}", winner);
	} else {
		println!("It's a draw");
	}
}
