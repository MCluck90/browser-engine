use parser::Parser;

pub struct Stylesheet {
	pub rules: Vec<Rule>,
}

pub struct Rule {
	pub selectors: Vec<Selector>,
	pub declarations: Vec<Declaration>,
}

pub enum Selector {
	Simple(SimpleSelector),
}

pub struct SimpleSelector {
	pub tag_name: Option<String>,
	pub id: Option<String>,
	pub class: Vec<String>,
	pub universal: bool,
}

pub struct Declaration {
	pub name: String,
	pub value: Value,
}

pub enum Value {
	Keyword(String),
	Length(f32, Unit),
	ColorValue(Color),
}

pub enum Unit {
	Px,
}

pub struct Color {
	r: u8,
	g: u8,
	b: u8,
	a: u8,
}

struct CssParser {
	inner: Parser,
}

impl CssParser {
	fn new(input: String) -> CssParser {
		CssParser {
			inner: Parser::new(input),
		}
	}

	// Parse an identifier
	fn parse_identifier(&mut self) -> String {
		self.inner.consume_while(is_valid_identifier_char)
	}

	// Parse a simple selector such as type#id.class1.class2
	fn parse_simple_selector(&mut self) -> SimpleSelector {
		let mut selector = SimpleSelector {
			tag_name: None,
			id: None,
			class: Vec::new(),
			universal: false,
		};

		while !self.inner.eof() {
			match self.inner.next_char() {
				'#' => {
					self.inner.consume_char();
					selector.id = Some(self.parse_identifier());
				}
				'.' => {
					self.inner.consume_char();
					selector.class.push(self.parse_identifier());
				}
				'*' => {
					// Universal selector
					self.inner.consume_char();
					selector.universal = true;
				}
				c if is_valid_identifier_char(c) => {
					selector.tag_name = Some(self.parse_identifier());
				}
				_ => break,
			}
		}

		selector
	}

	fn parse_selector(&mut self) -> Option<Selector> {
		// Take a simple selector
		if !self.inner.eof() {
			return Some(Selector::Simple(self.parse_simple_selector()));
		}
		None
	}

	fn parse_declaration(&mut self) -> Declaration {
		let name = self.parse_identifier();
		self.inner.consume_whitespace();
		assert!(self.inner.consume_char() == ':');
		self.inner.consume_whitespace();
		let value = self.parse_value();
		self.inner.consume_whitespace();
		assert!(self.inner.consume_char() == ';');
		Declaration { name, value }
	}

	fn parse_value(&mut self) -> Value {
		// Attempt to parse a color value
		if self.inner.starts_with("rgba(") {
			self.inner.consume_string("rgba(");
			self.inner.consume_whitespace();
			let r = self.inner.consume_number().unwrap();
			self.inner.consume_whitespace();
			assert!(self.inner.consume_char() == ',');
			self.inner.consume_whitespace();
			let g = self.inner.consume_number().unwrap();
			self.inner.consume_whitespace();
			assert!(self.inner.consume_char() == ',');
			self.inner.consume_whitespace();
			let b = self.inner.consume_number().unwrap();
			self.inner.consume_whitespace();
			assert!(self.inner.consume_char() == ',');
			self.inner.consume_whitespace();
			let a = self.inner.consume_number().unwrap();
			self.inner.consume_whitespace();
			assert!(self.inner.consume_char() == ')');

			return Value::ColorValue(Color { r, g, b, a });
		}

		// Attempt to parse a length value
		if let Ok(num) = self.inner.consume_number() {
			assert!(self.inner.consume_char() == 'p');
			assert!(self.inner.consume_char() == 'x');
			return Value::Length(num, Unit::Px);
		}

		Value::Keyword(self.inner.consume_while(is_valid_identifier_char))
	}

	fn parse_rule(&mut self) -> Rule {
		let mut selectors = Vec::new();
		let declarations = Vec::new();
		loop {
			if let Some(selector) = self.parse_selector() {
				selectors.push(selector);
			} else {
				break;
			}
		}

		Rule {
			selectors,
			declarations,
		}
	}

	fn parse_stylesheet(&mut self) -> Stylesheet {
		let mut rules = Vec::new();
		loop {
			self.inner.consume_whitespace();
			if self.inner.eof() {
				break;
			}
			rules.push(self.parse_rule());
		}

		Stylesheet { rules }
	}
}

fn is_valid_identifier_char(c: char) -> bool {
	char::is_alphanumeric(c) || c == '_' || c == '-'
}

pub fn parse(source: String) -> Stylesheet {
	CssParser::new(source).parse_stylesheet()
}
