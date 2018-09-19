use std::collections::{HashMap, HashSet};
use std::fmt;

pub type AttrMap = HashMap<String, String>;

#[derive(PartialEq)]
pub struct Node {
	// Data common to all nodes
	children: Vec<Node>,

	// Data specific to each node type
	node_type: NodeType,
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
	Text(String),
	Element(ElementData),
}

#[derive(Debug, PartialEq)]
pub struct ElementData {
	tag_name: String,
	attributes: AttrMap,
}

pub fn text(data: String) -> Node {
	Node {
		children: Vec::new(),
		node_type: NodeType::Text(data),
	}
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
	Node {
		children,
		node_type: NodeType::Element(ElementData {
			tag_name: name,
			attributes: attrs,
		}),
	}
}

// Node methods
impl Node {
	pub fn append(&mut self, child: Node) {
		self.children.push(child);
	}
}

// Element methods
impl ElementData {
	pub fn id(&self) -> Option<&String> {
		self.attributes.get("id")
	}

	pub fn classes(&self) -> HashSet<&str> {
		match self.attributes.get("class") {
			Some(classlist) => classlist.split(' ').collect(),
			None => HashSet::new(),
		}
	}
}

// Pretty print DOM nodes
impl fmt::Debug for Node {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.pretty_print_helper(f, 0)
	}
}

impl Node {
	fn pretty_print_helper(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
		let c = f.fill();
		let width = f.width().unwrap_or(0) * depth;
		let indent = if depth > 0 {
			format!("{:width$}", c, width = width)
		} else {
			"".to_string()
		};

		match &self.node_type {
			NodeType::Element(ref data) => {
				write!(f, "{}<{}", indent, data.tag_name);

				for (key, val) in data.attributes.iter() {
					write!(f, " {}=\"{}\"", key, val);
				}

				if self.children.len() == 0 {
					write!(f, " />")
				} else {
					write!(f, ">");
					for node in &self.children {
						write!(f, "\n");
						let _ = node.pretty_print_helper(f, depth + 1);
					}
					write!(f, "\n{}</{}>", indent, data.tag_name)
				}
			}
			NodeType::Text(ref s) => write!(f, "{}{}", indent, s),
		}
	}
}

#[cfg(test)]
mod dom_tests {
	use super::*;

	macro_rules! test_with_output {
		($test_name:ident, $body:expr) => {
			#[test]
			fn $test_name() {
				println!("\n\n--Begin dom::dom_tests::{}--", stringify!($test_name));
				$body
				println!("--End dom::dom_tests::{}--\n", stringify!($test_name));
			}
		};
	}

	#[test]
	fn can_generate_a_text_node() {
		let input = "test".to_string();
		let expected = Node {
			children: vec![],
			node_type: NodeType::Text(input.clone()),
		};
		let actual = text(input.clone());
		assert_eq!(expected, actual);
	}

	test_with_output!(can_show_debug_output, {
		println!("{:?}", text("test".into()));
	});

	#[test]
	fn can_append_children() {
		let expected = Node {
			node_type: NodeType::Text("parent".into()),
			children: vec![Node {
				children: vec![],
				node_type: NodeType::Text("child".into()),
			}],
		};
		let mut actual = text("parent".into());
		actual.append(text("child".into()));
		assert_eq!(expected, actual);
	}

	#[test]
	fn can_extract_the_id_from_a_node() {
		let mut attrs = HashMap::new();
		attrs.insert("id".into(), "test".into());
		let node = elem("test".into(), attrs, vec![]);
		if let NodeType::Element(ref data) = node.node_type {
			assert_eq!(data.id(), Some(&"test".to_string()));
		} else {
			unreachable!();
		}
	}

	#[test]
	fn can_get_classes() {
		let mut attrs = HashMap::new();
		attrs.insert("class".into(), "a b".into());
		let node = elem("test".into(), attrs, vec![]);
		if let NodeType::Element(ref data) = node.node_type {
			let mut expected = HashSet::new();
			expected.insert("a");
			expected.insert("b");
			assert_eq!(data.classes(), expected);
		} else {
			unreachable!();
		}
	}
}
