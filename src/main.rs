mod dom;

use dom::*;
use std::collections::HashMap;

fn main() {
	let just_text = text("Testing text element".into());
	println!("{:?}", just_text);

	let empty_el = elem("empty".into(), HashMap::new(), vec![]);
	println!("{:?}", empty_el);

	let mut attrs = HashMap::new();
	attrs.insert("id".to_string(), "test".to_string());
	let root = elem("h1".into(), attrs, vec![text("Hello world!".into())]);
	println!("{:?}", root);
}
