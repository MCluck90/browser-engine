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
}

fn is_valid_identifier_char(c: char) -> bool {
	char::is_alphanumeric(c) || c == '_' || c == '-'
}
