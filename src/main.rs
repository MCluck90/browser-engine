mod dom;
mod parser;

use dom::*;
use parser::parse;
use std::collections::HashMap;

fn main() {
	println!("{}", "Text Element:");
	let just_text = text("Testing text element".into());
	println!("{:?}\n", just_text);

	println!("{}", "Empty Element:");
	let empty_el = elem("empty".into(), HashMap::new(), vec![]);
	println!("{:?}\n", empty_el);

	let mut attrs = HashMap::new();
	attrs.insert("id".to_string(), "test".to_string());
	let root = elem("h1".into(), attrs, vec![text("Hello world!".into())]);

	println!("{}", "Element with One Child:");
	println!("{:?}\n", root);

	let input = "
	<html>
		<body>
			<h1>Hello, world!</h1>
		</body>
	</html>";
	let output = parse(input.into());
	println!("{}", "Complex DOM:");
	println!("{: >2?}", output);
}
