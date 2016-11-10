// Represents Conway's cell
// Tracks life, 2d index, neighbors
#[allow(non_snake_case)]
#[allow(dead_code)]
struct Cell {
	state: char,
	isAlive: bool,
	nextLife: bool,
	index: (usize,usize),
	neighbors: Vec<usize>,
}

// Grid is a 1d vector of size cols * rows
// Every element is a Conway Cell
#[allow(non_snake_case)]
struct Grid {
	rows: i32,
	cols: i32,
	waitTime: u64,
	data: Vec<Cell>,
	liveState: char,
	deadState: char,
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
#[allow(non_snake_case)]
impl Grid {

	// Constructor for instantiating a new Grid
	fn new(rows: i32, cols: i32, time: u64, live: char, dead: char) -> Grid {
		Grid {
			rows: rows,
			cols: cols,
			waitTime: time,
			data: vec![],
			liveState: live,
			deadState: dead,
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
			let index: (usize, usize) = (x, y);
			let adjacentCells: Vec<usize> = self.findNeighbors(index);
			self.data.push(Cell::new(self.deadState, index));
			self.data[a as usize].neighbors = adjacentCells;

			y += 1;

			if y % rows == 0 {
				x += 1;
				y = 0;
			}
		}
	}

	// Returns a vector containing the 1d indices of the neighbors
	// of a Cell using the Cell's the 2d index.
	#[allow(unused_parens)]
	fn findNeighbors(&mut self, twoDIndex: (usize, usize)) -> Vec<usize> {
		let mut adjacentCells: Vec<usize> = vec![];		
		let (x,y) = twoDIndex;
		let mut a: i32 = -1;
		// Iterates for neighbors (x +/- 1, y +/- 1)
		while a != 2 {
			let mut b: i32 = -1;
			while b != 2 {
				// Skips 1 case when (a,b) = (0,0)
				if a != 0 || b != 0 {
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
	// Updates the nextLife bool in a Cell
	// Rule 1: Any live cell with < 2 live neighbors dies
	// Rule 2: Any live cell with 2 or 3 live neighbors lives
	// Rule 3: Any live cell with > 3 live neighbors dies
	// Rule 4: Any dead cell with = 3 live neighbors becomes alive
	fn conwaysLife(&mut self, index: usize) {
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
				self.data[index].nextLife = false;				
			} else if neighborsAlive < 4 {
			// Rule 2: Live stays alive; next generation
				self.data[index].nextLife = true;
			} else {
			// Rule 3: Live becomes dead; over-population
				self.data[index].nextLife = false;
			}
		} else {
			// Rule 4: Dead becomes alive; reproduction
			if neighborsAlive == 3 {
				self.data[index].nextLife = true;
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
			print!("{}|",self.data[a as usize].state);
		}
			println!("");
	}

	// All Cells in Grid become aware of their fate in the next generation
	// If a Cell's nextLife is false, they will be represented as deadState
	// Otherwise they will be represented as liveState
	fn updateCells(&mut self) {
		let total = self.cols * self.rows;

		for a in 0..total {
			if self.data[a as usize].nextLife == true {
				self.data[a as usize].state = self.liveState;
				self.data[a as usize].isAlive = true;
			} else {
				self.data[a as usize].state = self.deadState;
				self.data[a as usize].isAlive = false;
			}
		}
	}

	// An entire "tick" of the Grid
	fn renderAll(&mut self) {
		use std::{thread,time};
		let wait = time::Duration::from_millis(self.waitTime);

		// Simultaneously impose all Cells to Conway's Life Rules
		for a in 0..(self.cols * self.rows) {
			self.conwaysLife(a as usize);
		}
		// Now update state and isAlive for all Cells
		self.updateCells();
		// And finally show the Grid's look
		self.printAllStates();
		// Slow down the printing
		thread::sleep(wait);
	}

	// Takes in an array of 1d indices to set true in the Grid
	fn seedGrid(&mut self, indices: &[i32]) {
		let length = indices.len();
		for a in 0..length {
			self.data[indices[a] as usize].nextLife = true;
		}
		self.updateCells();
	}
}

	// These seeds are alligned for 10 x 10
	// Oscillator - Blinker
	#[allow(dead_code)]
	static SEED1: [i32; 9] = [3, 13, 23, 28, 38,48,74,75,73];

	// Oscillator - Toad
	#[allow(dead_code)]
	static SEED2: [i32; 6] = [45,46,47,54,55,56];

	// Oscillator - Beacon
	#[allow(dead_code)]
	static SEED3: [i32; 6] = [45,46,55,68,77,78];
	
	// Spaceship - Glider
	#[allow(dead_code)]
	static SEED4: [i32; 5] = [2,13,23,22,21];

	// Weird Thing:
	#[allow(dead_code)]
	static SEED5: [i32; 9] = [3,13,23,51,61,71,74,75,73];

	// For a 20 x 20:	
	// Oscillator - Pulsar
	#[allow(dead_code)]
	static SEED6: [i32; 48] = [46,47,48,52,53,54,
							146,147,148,152,153,154,
							186,187,188,192,193,194,
							286,287,288,292,293,294,
							//Vertical bars
							84,104,124, 89,109,129,
							91,111,131,96,116,136,
							204,224,244,209,229,249,
							211,231,251,216,236,256];

// World contains a 1d vector treated as a 2d vector containing Cells.
// The Initialize function will set default values and populate World.
// Fill the vector will Cells that will calculate their neighbor's indices.
// Then the world will constantly run a Render function in loop forever.
// In the loop, the World will "tick".
// First every Cell will simultaneous check its living neighbors.
// Conway's 4 Rules will apply to the Cells to kill or resurrect them.
// Then the living cells will move in some way via printAllStates. Repeat.
#[allow(unused_variables)]
fn main() {
	// Grid params (row, col, delay in ms, live state, dead state)
	// Some UTF-8 chars:'◈' '◎' 'փ' '۩' '⁂' '☮' '⚡' '♬' '✿' '✮' '⧖' '■'
	let mut world = Grid::new(20,20,350,'✿',' ');
	world.initialize();
	world.seedGrid(&SEED6);

	loop {
		world.renderAll();
	}
}
