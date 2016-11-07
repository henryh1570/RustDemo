struct Cell {
}

struct Grid {
	cols: i32,
	rows: i32,
	data: Vec<i32>
}

impl Grid {
	fn new(cols: i32, rows: i32) -> Grid {
		Grid {
			cols: cols,
			rows: rows,
			data: vec![0; (cols * rows) as usize]
		}
	}

	fn printAll(&mut self) {
		let total = self.cols * self.rows;

		for a in 0..total {
			if (a % 10 == 0) {
				println!("");
			}
			self.data[a as usize] = a;
			print!("[{}]",self.data[a as usize]);
		}
	}
}


fn main() {
	let mut gr = Grid::new(10,10);
	gr.printAll();
	println!("hello!");
}
