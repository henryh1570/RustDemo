struct Cell {
	state: char,
}

// Grid is a 1d vector of size cols * rows
struct Grid {
	cols: i32,
	rows: i32,
	data: Vec<Cell>
}

impl Cell {
	fn new(c: char) -> Cell {
		Cell {
			state: c,
		}
	}
}

impl Grid {
	fn new(cols: i32, rows: i32) -> Grid {
		Grid {
			cols: cols,
			rows: rows,
			data: vec![],
		}
	}

	fn initialize(&mut self) {
		let total = self.cols * self.rows;

		for a in 0..total {
			self.data.push(Cell::new('-'));
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
}

fn main() {
	let mut gr = Grid::new(10,10);
	gr.initialize();
	gr.printAll();
}
