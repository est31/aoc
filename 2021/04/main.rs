use std::fmt::Display;
use std::str::FromStr;
use std::collections::HashMap;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct StrErr(String);

impl<T :Display> From<T> for StrErr {
	fn from(v :T) -> Self {
		StrErr(format!("{}", v))
	}
}

type Result<T> = std::result::Result<T, StrErr>;

fn main() -> Result<()> {
	let (numbers, boards) = BingoBoards::parse(INPUT)?;
	let (sum_unmarked, winning_number) = boards.clone().play_numbers(&numbers).unwrap();
	println!("First winning score: {}", sum_unmarked * winning_number);
	let (sum_unmarked, winning_number) = boards.clone().play_numbers_last(&numbers).unwrap();
	println!("Last winning score: {}", sum_unmarked * winning_number);
	Ok(())
}

#[derive(Clone)]
struct Board {
	won :bool,
	lines_matched :[u16; 5],
	cols_matched :[u16; 5],
	sum_unmarked :u16,
}

#[derive(Clone)]
struct BingoBoards {
	indices :HashMap<u16, Vec<(usize, usize, usize)>>,
	boards :Vec<Board>,
}

impl BingoBoards {
	fn parse(input :&str) -> Result<(Vec<u16>, Self)> {
		let mut lines = input.lines();
		let numbers = lines.next().ok_or("No first line")?
			.split(',')
			.map(u16::from_str)
			.collect::<std::result::Result<Vec<_>, _>>()?;


		// Parse the boards
		let mut boards = Vec::new();
		while lines.next().is_some() {
			let board = [
				lines.next().unwrap(), lines.next().unwrap(),
				lines.next().unwrap(), lines.next().unwrap(),
				lines.next().unwrap()
			];
			let board = board.map(|l| l.split_whitespace()
					.map(u16::from_str)
					.collect::<std::result::Result<Vec<_>, _>>().unwrap());
			boards.push(board);
		}

		// Index table
		let mut indices = HashMap::<u16, Vec<_>>::new();
		for (bi, b) in boards.iter().enumerate() {
			for (li, l) in b.iter().enumerate() {
				for (vi, v) in l.iter().enumerate() {
					let j = indices.entry(*v).or_default();
					j.push((bi, li, vi));
				}
			}
		}

		// Boards
		let boards = boards.iter()
			.map(|b| {
				let sum_unmarked = b.iter()
					.map(|l| l.iter().sum::<u16>())
					.sum();
				Board {
					won : false,
					lines_matched : [0; 5],
					cols_matched : [0; 5],
					sum_unmarked,
				}
			})
			.collect();

		Ok((numbers, BingoBoards {
			indices,
			boards,
		}))
	}
	fn play_number(&mut self, n :u16) -> Vec<(u16, u16)> {
		let Some(indices) = self.indices.get(&n) else {
			return Vec::new();
		};
		let mut winning_boards = Vec::new();
		for (bi, li, vi) in indices {
			let b = &mut self.boards[*bi];
			if b.won {
				continue;
			}
			b.sum_unmarked -= n;
			b.lines_matched[*vi] += 1;
			b.cols_matched[*li] += 1;
			if b.lines_matched[*vi] >= 5 || b.cols_matched[*li] >= 5 {
				// BINGO!
				b.won = true;
				winning_boards.push((b.sum_unmarked, n));
			}
		}
		winning_boards
	}
	fn play_numbers(&mut self, numbers :&[u16]) -> Option<(u16, u16)> {
		for n in numbers {
			if let w @ Some(_) = self.play_number(*n).pop() {
				return w;
			}
		}
		// No board won
		None
	}
	fn play_numbers_last(&mut self, numbers :&[u16]) -> Option<(u16, u16)> {
		let mut board_won_last = None;
		for n in numbers {
			if let w @ Some(_) = self.play_number(*n).pop() {
				board_won_last = w;
			}
		}
		board_won_last
	}
}
