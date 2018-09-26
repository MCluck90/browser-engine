use dom;
use parser::Parser;
use std::collections::HashMap;

pub struct HtmlParser {
	inner: Parser,
}

impl HtmlParser {
	fn new(input: String) -> HtmlParser {
		HtmlParser {
			inner: Parser::new(input),
		}
	}

	// Parse a tag or attribute name
	fn parse_tag_name(&mut self) -> String {
		self.inner.consume_while(|c| match c {
			'a'...'z' | 'A'...'Z' | '0'...'9' => true,
			_ => false,
		})
	}

	// Parse a single node
	fn parse_node(&mut self) -> dom::Node {
		if self.inner.starts_with("<!--") {
			return self.parse_comment();
		}

		match self.inner.next_char() {
			'<' => self.parse_element(),
			_ => self.parse_text(),
		}
	}

	// Parse a text node
	fn parse_text(&mut self) -> dom::Node {
		dom::text(self.inner.consume_while(|c| c != '<'))
	}

	// Parse a single element, including its open tag, contents, and closing tag
	fn parse_element(&mut self) -> dom::Node {
		// Opening tag
		assert!(self.inner.consume_char() == '<');
		let tag_name = self.parse_tag_name();
		let attrs = self.parse_attributes();
		assert!(self.inner.consume_char() == '>');

		// Contents
		let children = self.parse_nodes();

		// Closing tag
		assert!(self.inner.consume_char() == '<');
		assert!(self.inner.consume_char() == '/');
		assert!(self.parse_tag_name() == tag_name);
		assert!(self.inner.consume_char() == '>');

		dom::elem(tag_name, attrs, children)
	}

	// Parse out a comment
	fn parse_comment(&mut self) -> dom::Node {
		// Opening tag
		self.inner.consume_string("<!--");

		// Contents
		let mut contents = String::new();
		loop {
			if self.inner.starts_with("-->") {
				break;
			}

			contents.push(self.inner.consume_char());
		}

		// Closing tag
		self.inner.consume_string("-->");

		dom::comment(contents)
	}

	// Parse a single name="value" pair
	fn parse_attr(&mut self) -> (String, String) {
		let name = self.parse_tag_name();
		assert!(self.inner.consume_char() == '=');
		let value = self.parse_attr_value();
		(name, value)
	}

	// Parse a quoted value
	fn parse_attr_value(&mut self) -> String {
		let open_quote = self.inner.consume_char();
		assert!(open_quote == '"' || open_quote == '\'');
		let value = self.inner.consume_while(|c| c != open_quote);
		assert!(self.inner.consume_char() == open_quote);
		value
	}

	// Parse a list of name="value" pairs, separated by whitespace
	fn parse_attributes(&mut self) -> dom::AttrMap {
		let mut attributes = HashMap::new();
		loop {
			self.inner.consume_whitespace();
			if self.inner.next_char() == '>' {
				break;
			}
			let (name, value) = self.parse_attr();
			attributes.insert(name, value);
		}
		attributes
	}

	// Parse a sequence of sibling nodes
	fn parse_nodes(&mut self) -> Vec<dom::Node> {
		let mut nodes = Vec::new();
		loop {
			self.inner.consume_whitespace();
			if self.inner.eof() || self.inner.starts_with("</") {
				break;
			}
			nodes.push(self.parse_node());
		}
		nodes
	}
}

pub fn parse(source: String) -> dom::Node {
	let mut nodes = HtmlParser::new(source).parse_nodes();

	// If the document contains a root element, just return it. Otherwise, create one
	if nodes.len() == 1 {
		nodes.swap_remove(0)
	} else {
		dom::elem("html".to_string(), HashMap::new(), nodes)
	}
}

#[cfg(test)]
mod html_tests {
	use super::*;
	use dom;
	use std::collections::HashMap;

	#[test]
	fn can_parse_basic_html() {
		let input = "<html></html>".into();
		let expected = dom::elem("html".into(), HashMap::new(), Vec::new());
		let actual = parse(input);
		assert_eq!(expected, actual);
	}

	#[test]
	fn can_parse_more_complex_dom() {
		let input = "
			<html>
				<body>
					<h1 id=\"test\">Test</h1>
				</body>
			</html>
			".into();
		let mut h1_attrs = HashMap::new();
		h1_attrs.insert("id".into(), "test".into());
		let expected = dom::elem(
			"html".into(),
			HashMap::new(),
			vec![dom::elem(
				"body".into(),
				HashMap::new(),
				vec![dom::elem(
					"h1".into(),
					h1_attrs,
					vec![dom::text("Test".into())],
				)],
			)],
		);
		let actual = parse(input);
		assert_eq!(expected, actual);
	}

	#[test]
	fn can_parse_basic_comment() {
		let input = "<!-- some comment -->".into();
		let expected = dom::comment(" some comment ".into());
		let actual = parse(input);
		assert_eq!(expected, actual);
	}
}
