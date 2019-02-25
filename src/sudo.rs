
use std::env;
use std::process::exit;

fn get_grid() -> [[u8; 9]; 9]
{
	let mut grid: [[u8; 9]; 9] = Default::default();
	let mut args: Vec<String> = env::args().collect();

	if args.len() != 10 {
		eprintln!("This program need 9 strings of 9 numbers between 0 and 9");
		exit(1);
	}

	args.remove(0);

	for (i, arg) in args.iter().enumerate() {
		for (j, value) in arg.split(' ').enumerate() {
			match value.parse() {
				Ok(x) => {
					grid[i][j] = x;
				},
				Err(e) => {
					eprintln!("Value {} is not a valid integer [{}]", value, e);
					exit(1);
				},
			}
		}
	}

	return grid;
}

fn in_square(grid: &[[u8; 9]; 9], line_index: usize, column_index: usize, value: u8) -> bool {
	let line = ((line_index) / 3) * 3;
	let column = ((column_index) / 3) * 3;

	let square: [&[u8]; 3] = [
	 	&grid[line + 0][column..column + 3],
	 	&grid[line + 1][column..column + 3],
	 	&grid[line + 2][column..column + 3],
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

fn in_column(grid: &[[u8; 9]; 9], column_index: usize, value: u8) -> bool {
	for line_index in 0..9 {
		if grid[line_index][column_index] == value {
			return true;
		}
	}
	return false
}

fn in_line(grid: &[[u8; 9]; 9], line_index: usize, value: u8) -> bool {
	for column_index in 0..9 {
		if grid[line_index][column_index] == value {
			return true;
		}
	}
	return false
}

fn resolve(grid: &mut [[u8; 9]; 9], index: usize) -> bool {
	let line_index = index / 9;
	let column_index = index % 9;

	if index >= 9*9 - 1 {
		return true;
	}
	if grid[line_index][column_index] != 0 {
		return resolve(grid, index + 1);
	}
	for i in 1..10 {
		if !in_column(grid, column_index, i)
			&& !in_line(grid, line_index, i)
			&& !in_square(grid, line_index, column_index, i)
		{
			grid[line_index][column_index] = i;

			if resolve(grid, index + 1) == true {
				return true;
			}
			grid[line_index][column_index] = 0;
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
		false => println!("This grid does not admit any response :("),
	}
}