// Represents Conway's cell
// Tracks life, 2d index, neighbors
struct Cell {
	state: char,
	isAlive: bool,
	nextLife: bool,
	index: (usize,usize),
	neighbors: Vec<usize>,
}

// Grid is a 1d vector of size cols * rows
// Every element is a Conway Cell
struct Grid {
	rows: i32,
	cols: i32,
	data: Vec<Cell>
}

// Defining Cell's functions
impl Cell {

	// Constructor for instantiating a new Cell
	fn new(c: char, pair: (usize,usize)) -> Cell {
		Cell {
			state: c,
			isAlive: false,
			nextLife: false,
			index: pair,
			neighbors: vec![],
		}
	}
}

// Defining Grid's Functions
impl Grid {

	// Constructor for instantiating a new Grid
	fn new(rows: i32, cols: i32) -> Grid {
		Grid {
			rows: rows,
			cols: cols,
			data: vec![],
		}
	}

	// Size of vector is rows * cols
	// Initializes the vector with default values
	// Using a for loop fill up the vector with Cells
	// Each Cell gets their 2d array index calculated
	// Neighbors for each cell are calculated
	fn initialize(&mut self) {
		let rows = self.rows as usize;
		let cols = self.cols as usize;
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

	// Finds a Cell's neighbors, marks them, and reveals them.
	fn showNeighbors(&mut self, twoDIndex: (usize, usize)) {
		let index: usize = self.getIndexConversion(twoDIndex);

		let neighbors = self.data[index].neighbors.to_vec();
		let length: usize = neighbors.len();

		for a in 0..length {
			let adjacentIndex: usize = neighbors[a];
			self.data[adjacentIndex].state = '=';
		}
		self.printAllStates();
	}

	// Returns all Cells back to empty state.
	fn clearGrid(&mut self) {
		for a in 0..self.data.len() {
			self.data[a].state = ' ';
		}
	}

	// Lists out a Cell's neighbors' 1d index
	fn showNeighborIndices(&mut self, twoDIndex: (usize, usize)) {
		let index: usize = self.getIndexConversion(twoDIndex);
		let neighbors = self.data[index].neighbors.to_vec();
		let length: usize = neighbors.len();

		for a in 0..length {
			let adjacentIndex: usize = neighbors[a];
			println!("{}",adjacentIndex);
		}
	}

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

	// Simulate life and death in the Grid for Cells
	// Rule 1: Any live cell with < 2 live neighbors dies
	// Rule 2: Any live cell with 2 or 3 live neighbors lives
	// Rule 3: Any live cell with > 3 live neighbors dies
	// Rule 4: Any dead cell with = 3 live neighbors becomes alive
	fn live(&mut self, index: usize) {
		let neighbors = self.data[index].neighbors.to_vec();
		let length: usize = neighbors.len();
		let mut neighborsAlive: i32 = 0;

		// Count all neighbors of the Cell that are alive
		for a in 0..length {
			let adjacentIndex: usize = neighbors[a];
			if self.data[adjacentIndex].isAlive == true {
				neighborsAlive += 1;
			}
		}

		// Apply Conway's Life Rules
		if self.data[index].isAlive == true {
			// Rule 1: Live becomes dead; under-population
			if neighborsAlive < 2 {
				self.data[index].isAlive = false;				
			} else if neighborsAlive < 4 {
			// Rule 2: Live stays alive; next generation
			} else {
			// Rule 3: Live becomes dead; over-population
				self.data[index].isAlive = false;
			}
		} else {
			// Rule 4: Dead becomes alive; reproduction
			if neighborsAlive == 3 {
				self.data[index].isAlive = true;
			}
		}
	}

	// Prints all current states of Cells in the vector
	// Lines separated on every column-th iteration to emulate grid
	fn printAllStates(&mut self) {
		let total = self.cols * self.rows;

		for a in 0..total {
			if a % self.cols == 0 {
				println!("");
			}
			print!("[{}]",self.data[a as usize].state);
		}
			println!("");
	}

	// Lists all 2d indices of the Cells in Grid
	// Corresponding to the 1d index of Cells
	fn printAllIndices(&mut self) {
		let total = self.cols * self.rows;

		for a in 0..total {
			println!("[{}] = {:?}",a,self.data[a as usize].index);
		}
	}

	// A simple test render of the Grid by filling
	// each Cell's state one at a time.
	// Thread delay is specified in wait time
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
//	Testing methods below
//	=====================
//	world.printAllIndices();
	world.testRender();
	world.clearGrid();
	world.printAllStates();
	world.showNeighbors((2,2));
//	world.showNeighborIndices((2,2));
}
