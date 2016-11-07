// Represents Conway's cell
struct Cell {
	state: char,
}

// Grid is a 1d vector of size cols * rows
struct Grid {
	cols: i32,
	rows: i32,
	data: Vec<Cell>
}

// Defining Cell's functions
impl Cell {
	fn new(c: char) -> Cell {
		Cell {
			state: c,
		}
	}

	fn changeState(&mut self, c: char) {
		self.state = c
	}
}

// Defining Grid's Functions
impl Grid {
	fn new(cols: i32, rows: i32) -> Grid {
		Grid {
			cols: cols,
			rows: rows,
			data: vec![],
		}
	}

	// Size of vector is rows * cols
	// Initializes the vector with default values
	fn initialize(&mut self) {
		let total = self.cols * self.rows;

		for a in 0..total {
			self.data.push(Cell::new(' '));
		}
	}

	// Method to print all states of Cells in the vector
	// Lines separated on every 10th iteration to emulate grid
	fn printAll(&mut self) {
		let total = self.cols * self.rows;

		for a in 0..total {
			if a % self.cols == 0 {
				println!("");
			}
			print!("[{}]",self.data[a as usize].state);
		}
			println!("");
	}

	// Testing render of Grid each state
	// Delay is specified in wait time
	fn testRender(&mut self) {
		let total = self.cols * self.rows;

		for a in 0..total {
			self.data[a as usize].state = 'X';
			self.printAll();

			use std::{thread,time};
			let wait = time::Duration::from_millis(25);
			thread::sleep(wait);
		}
	}
}

fn main() {
	let mut world = Grid::new(10,10);
	world.initialize();
	world.printAll();
	world.testRender();
}
