
use std::env;

fn get_grid() -> [[u8; 9]; 9]
{
	let mut args = env::args();
	let mut grid: [[u8; 9]; 9] = [[0; 9]; 9];

	assert_eq!(args.len(), 9, "This program need 9 strings of 9 numbers between 0 and 9, got {}", args.len());
	let _ = args.next();
	for i in 0..8 {
		let mut line = args.next().unwrap();
		let mut values = line.split(' ');

		for j in 0..8 {
			let number: u8;

			match values.next() {
				Some(x) => number = x.parse().unwrap(),

				None => number = 0,
			}

			assert!(number <= 9, "Numbers should be unsigned integers lower than 9, got number {}", number);
			grid[i][j] = number;
		}
	}

	return grid;
}

fn in_square(grid: &[[u8; 9]; 9], index: usize, value: u8) -> bool {
	let square = [
		&grid[index / 9][index % 9:index % 9 + 3],
		&grid[index / 9 + 1][index % 9:index % 9 + 3],
		&grid[index / 9 + 2][index % 9:index % 9 + 3],
	];

	for i in 0..3 {
		for j in 0..3 {
			if square[i][j] == value {
				return true;
			}
		}
	}
	return false;
}

fn in_column(grid: &[[u8; 9]; 9], index: usize, value: u8) -> bool {
	let column_index = index % 9;

	for line_index in 1..9 {
		if grid[line_index][column_index] == value {
			return true;
		}
	}
	return false
}

fn in_line(grid: &[[u8; 9]; 9], index: usize, value: u8) -> bool {
	let line_index = index / 9;

	for column_index in 1..9 {
		if grid[line_index][column_index] == value {
			return true;
		}
	}
	return false
}

fn resolve(grid: &mut [[u8; 9]; 9], index: usize) -> bool {
	if index >= 9*9 {
		return true;
	}
	if grid[index / 9][index % 9] != 0 {
		return resolve(grid, index + 1);
	}
	for i in 1..9 {
		if !in_column(grid, index, i) && !in_line(grid, index, i) {
			grid[index / 9][index % 9] = i;

			if resolve(grid, index + 1) == true {
				return true;
			}
			grid[index / 9][index % 9] = 0;
		}
	}
	return false;
}

fn print_grid(grid: &[[u8; 9]; 9])
{
	for i in 0..9 {
		if i % 3 == 0 {
			println!("{}+", "+-------".repeat(3));
		}
		for j in 0..9 {
			if j % 3 == 0 {
				print!("| ");
			}
			print!("{} ", grid[i][j]);
		}
		println!("|");
	}
	println!("{}+", "+-------".repeat(3));
}

fn main()
{
	let mut grid: [[u8; 9]; 9] = get_grid();

	match resolve(& mut grid, 0) {
		true => print_grid(&grid),
		false => println!("This grid does not admi any response :("),
	}
}