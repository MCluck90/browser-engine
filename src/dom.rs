use std::collections::HashMap;
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

// Pretty print DOM nodes
impl fmt::Debug for Node {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match &self.node_type {
			NodeType::Element(ref data) => {
				write!(f, "<{}", data.tag_name);

				for (key, val) in data.attributes.iter() {
					write!(f, " {}=\"{}\"", key, val);
				}

				if self.children.len() == 0 {
					write!(f, " />")
				} else {
					write!(f, ">");
					for node in &self.children {
						write!(f, "\n\t{:?}", node);
					}
					write!(f, "\n</{}>", data.tag_name)
				}
			}
			NodeType::Text(ref s) => write!(f, "{}", s),
		}
	}
}

#[cfg(test)]
mod dom_tests {
	use super::*;

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

	#[test]
	fn can_show_debug_output() {
		println!("{:?}", text("test".into()));
	}
}
