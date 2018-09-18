extern crate num;

mod css;
mod dom;
mod html;
mod parser;

use dom::*;
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
			<div id=\"div-id\">
				<h2 class=\"a b c\">Inner stuff</h2>
			</div>
		</body>
	</html>";
	let output = html::parse(input.into());
	println!("Complex DOM:");
	println!("{: >2?}\n", output);

	let input = "
	p {
		color: rgba(127, 23, 64, 255);
	}";
	let output = css::parse(input.into());
	println!("Basic CSS:");
	println!("{:?}", output);
}
