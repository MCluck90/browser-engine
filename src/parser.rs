use num::Num;
use std::str::FromStr;

pub struct LinePos {
	pub pos: usize,
	pub line: usize,
	pub column: usize,
}

pub struct Parser {
	pos: usize,
	input: String,
}

impl Parser {
	pub fn new(input: String) -> Parser {
		Parser { pos: 0, input }
	}

	// Gets the current position in the input
	pub fn pos(&self) -> LinePos {
		let line = self
			.input
			.chars()
			.take(self.pos)
			.fold(0, |acc, c| if c == '\n' { acc + 1 } else { acc });
		let column = self
			.input
			.chars()
			.rev()
			.position(|c| c == '\n')
			.unwrap_or(0);
		LinePos {
			pos: self.pos,
			line,
			column,
		}
	}

	// Read the next character without consuming it
	pub fn next_char(&self) -> char {
		self.input[self.pos..].chars().next().unwrap()
	}

	// Do the next characters start with a given string?
	pub fn starts_with(&self, s: &str) -> bool {
		self.input[self.pos..].starts_with(s)
	}

	// Return true if all input is consumed
	pub fn eof(&self) -> bool {
		self.pos >= self.input.len()
	}

	// Return the current character, and advance self.pos to the next character
	pub fn consume_char(&mut self) -> char {
		let mut iter = self.input[self.pos..].char_indices();
		let (_, cur_char) = iter.next().unwrap();
		let (next_pos, _) = iter.next().unwrap_or((1, ' '));
		self.pos += next_pos;
		cur_char
	}

	// Return the next set of characters which form a number
	pub fn consume_number<T>(&mut self) -> Result<T, <T as FromStr>::Err>
	where
		T: Num + FromStr,
	{
		let val = self.consume_while(|c| char::is_numeric(c));
		val.parse()
	}

	// Consume characters until `test` returns false
	pub fn consume_while<F>(&mut self, test: F) -> String
	where
		F: Fn(char) -> bool,
	{
		let mut result = String::new();
		while !self.eof() && test(self.next_char()) {
			result.push(self.consume_char());
		}
		result
	}

	// Consume and discard zero or more whitespace characters
	pub fn consume_whitespace(&mut self) {
		self.consume_while(char::is_whitespace);
	}

	// Consume and discard a specific string
	pub fn consume_string(&mut self, val: &str) {
		for c in val.chars() {
			let next = self.consume_char();
			assert!(c == next);
		}
	}
}
