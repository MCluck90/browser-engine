use parser::Parser;

#[derive(Debug)]
pub struct Stylesheet {
	pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
	pub selectors: Vec<Selector>,
	pub declarations: Vec<Declaration>,
}

pub type Specificity = (usize, usize, usize);

#[derive(Debug)]
pub enum Selector {
	Simple(SimpleSelector),
}

impl Selector {
	pub fn specificity(&self) -> Specificity {
		// http://www.w3.org/TR/selectors/#specificity
		let Selector::Simple(ref simple) = *self;
		let a = simple.id.iter().count();
		let b = simple.class.len();
		let c = simple.tag_name.iter().count();
		(a, b, c)
	}
}

#[derive(Debug)]
pub struct SimpleSelector {
	pub tag_name: Option<String>,
	pub id: Option<String>,
	pub class: Vec<String>,
	pub universal: bool,
}

#[derive(Debug)]
pub struct Declaration {
	pub name: String,
	pub value: Value,
}

#[derive(Debug)]
pub enum Value {
	Keyword(String),
	Length(f32, Unit),
	ColorValue(Color),
}

#[derive(Debug)]
pub enum Unit {
	Px,
}

#[derive(Debug)]
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

	// Tells us if it's possible to start parsing a rule
	fn can_start_rule(&self) -> bool {
		self.can_start_simple_selector()
	}

	// Tells us if the next char can be used to start a selector
	fn can_start_simple_selector(&self) -> bool {
		let next = self.inner.next_char();
		next == '#' || next == '.' || next == '*' || is_valid_identifier_char(next)
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
		if !self.inner.eof() && self.can_start_simple_selector() {
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
		let mut declarations = Vec::new();
		loop {
			if let Some(selector) = self.parse_selector() {
				selectors.push(selector);
				self.inner.consume_whitespace();
			} else {
				break;
			}
		}

		// Sort the selectors by specificity
		selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));

		self.inner.consume_whitespace();
		assert!(self.inner.consume_char() == '{');

		loop {
			self.inner.consume_whitespace();
			if self.inner.next_char() == '}' {
				self.inner.consume_char();
				break;
			}

			declarations.push(self.parse_declaration());
		}

		Rule {
			selectors,
			declarations,
		}
	}

	fn parse_stylesheet(&mut self) -> Result<Stylesheet, String> {
		let mut rules = Vec::new();
		loop {
			self.inner.consume_whitespace();
			if self.inner.eof() || !self.can_start_rule() {
				break;
			}
			rules.push(self.parse_rule());
		}

		if !self.inner.eof() {
			let pos = self.inner.pos();
			Err(format!(
				"Unknown character at {}:{}: {}",
				pos.line,
				pos.column,
				self.inner.next_char()
			))
		} else {
			Ok(Stylesheet { rules })
		}
	}
}

fn is_valid_identifier_char(c: char) -> bool {
	char::is_alphanumeric(c) || c == '_' || c == '-'
}

pub fn parse(source: String) -> Result<Stylesheet, String> {
	CssParser::new(source).parse_stylesheet()
}
