mod dom;

use dom::*;
use std::collections::HashMap;

fn main() {
	let root = elem(
		"h1".into(),
		HashMap::new(),
		vec![text("Hello world!".into())],
	);
	println!("{:?}", root);
}
