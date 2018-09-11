pub struct Parser {
	pos: usize,
	input: String,
}

impl Parser {
	pub fn new(input: String) -> Parser {
		Parser { pos: 0, input }
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
}
