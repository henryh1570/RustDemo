// Represents Conway's cell
struct Cell {
	state: char,
	isAlive: bool,
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

	// Constructor for instantiating a new Cell
	fn new(c: char, pair: (usize,usize)) -> Cell {
		Cell {
			state: c,
			isAlive: false,
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

	// Constructor for instantiating a new Grid
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
	// Neighbors for each cell are calculated
	fn initialize(&mut self) {
		let cols = self.cols as usize;
		let rows = self.rows as usize;
		let mut x = 0 as usize;
		let mut y = 0 as usize;

		for a in 0..(cols * rows) {
			let mut index: (usize, usize) = (x, y);
			let mut adjacentCells: Vec<usize> = self.findNeighbors(index);
			self.data.push(Cell::new(' ', index));
			self.data[a as usize].neighbors = adjacentCells;

			y += 1;

			if y % rows == 0 {
				x += 1;
				y = 0;
			}
		}
	}

	// Providing the 2d index of a Cell to find and mark its neighbors.
	fn showNeighbors(&mut self, twoDIndex: (usize, usize)) {
		let index: usize = self.getIndexConversion(twoDIndex);

		let neighbors = self.data[index].neighbors.to_vec();
		let length: usize = neighbors.len();

		for a in 0..length {
			let adjacentIndex: usize = neighbors[a];
			self.data[adjacentIndex].state = '=';
		}
	}

	//TODO: Only works for n x n matrix right now.
	// Returns a vector containing the 1d indices of the neighbors
	// of a Cell using the Cell's the 2d index.
	fn findNeighbors(&mut self, twoDIndex: (usize, usize)) -> Vec<usize> {
		let mut adjacentCells: Vec<usize> = vec![];		
		let (x,y) = twoDIndex;
		let mut a: i32 = -1;

		// Iterates for neighbors (x +/- 1, y +/- 1)
		while a != 2 {
			let mut b: i32 = -1;
			while b != 2 {
				// Skips 1 case when (a,b) = (0,0)
				if (a != 0 || b != 0) {
					// Checks to make sure (x+a,y+b) cannot reach
					// IndexOutOfBounds (-1,-1) or (row, col)
					if ( ((x as i32 + a) >= 0) && ((y as i32 + b) >= 0) 
					&& (((x as i32) + a) < self.rows) 
					&& (((y as i32) + b) < self.cols)) {
						
						let num: (usize, usize) = 
						((x as i32 + a) as usize,(y as i32 + b) as usize);

						adjacentCells.push(self.getIndexConversion(num));
					}
				}
				b+= 1;
			}
			a+= 1;
		}
		adjacentCells
	}

	// Converts a Cell's 2d index into a 1d index number
	fn getIndexConversion(&mut self, twoDIndex: (usize, usize)) -> usize {
		let (x, y) = twoDIndex;
		let index: usize = x * (self.cols as usize) + y;
		index
	}

	// Prints all states of Cells in the vector
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
}

fn main() {
	let mut world = Grid::new(10,10);
	world.initialize();
//	world.printAllIndices();
	world.showNeighbors((5,5));
	world.printAllStates();
//	world.printAllStates();
//	world.testRender();
}
