// Represents Conway's cell
struct Cell {
	state: char,
	index: (usize,usize),
	neighbors: Vec<usize>,
}

// Grid is a 1d vector of size cols * rows
struct Grid {
	cols: i32,
	rows: i32,
	data: Vec<Cell>
}

// Defining Cell's functions
impl Cell {
	fn new(c: char, pair: (usize,usize)) -> Cell {
		Cell {
			state: c,
			index: pair,
			neighbors: vec![],
		}
	}

	//TODO: May be unnecessary at the momment.
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
	// Using a for loop fill up the vector with Cells
	// Each Cell gets their 2d array index calculated
	fn initialize(&mut self) {
		let cols = self.cols as usize;
		let rows = self.rows as usize;
		let mut x = 0 as usize;
		let mut y = 0 as usize;

		for a in 0..(cols * rows) {
			let mut index: (usize, usize) = (x, y);
			let mut adjacentCells: Vec<usize> = vec![];
			self.data.push(Cell::new(' ', index));
			self.data[a as usize].neighbors = adjacentCells;

			y += 1;

			if y % rows == 0 {
				x += 1;
				y = 0;
			}
		}
	}

	// Converts a Cell's 2d index into a 1d index number
	// Formula: i = x * COL_SIZE + y
	fn getIndexConversion(&mut self, twoDIndex: (usize, usize)) -> usize {
		let (x, y) = twoDIndex;
		let index: usize = x * (self.cols as usize) + y;
		index
	}

	// Method to print all states of Cells in the vector
	// Lines separated on every 10th iteration to emulate grid
	fn printAllStates(&mut self) {
		let total = self.cols * self.rows;

		for a in 0..total {
			if a % self.rows == 0 {
				println!("");
			}
			print!("[{}]",self.data[a as usize].state);
		}
			println!("");
	}

	// Method to print all 2d indices of the Cells in Grid
	// Corresponding to the 1d index of Cells
	fn printAllIndices(&mut self) {
		let total = self.cols * self.rows;

		for a in 0..total {
			println!("[{}] = {:?}",a,self.data[a as usize].index);
		}
	}

	// Testing render of Grid each state
	// Delay is specified in wait time
	fn testRender(&mut self) {
		let total = self.cols * self.rows;

		for a in 0..total {
			self.data[a as usize].state = '=';
			self.printAllStates();

			use std::{thread,time};
			let wait = time::Duration::from_millis(25);
			thread::sleep(wait);
		}
	}

	//TODO: Retrieves a single Cell's adjacent neighbors
	fn getCellNeighbors(&mut self, index: usize) -> Vec<usize>{
		let a: Vec<usize> = vec![];
		a
	}
}

fn main() {
	let mut world = Grid::new(3,7);
	world.initialize();
	world.printAllIndices();
//	world.printAllStates();
//	world.testRender();
}
