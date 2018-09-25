use css::{Rule, Selector, SimpleSelector, Specificity, Stylesheet, Value};
use dom::{ElementData, Node, NodeType};
use std::collections::HashMap;

// Map from CSS property names to values.
type PropertyMap = HashMap<String, Value>;

// A node with associated style data.
pub struct StyledNode<'a> {
	pub node: &'a Node,
	pub specified_values: PropertyMap,
	pub children: Vec<StyledNode<'a>>,
}

fn matches(elem: &ElementData, selector: &Selector) -> bool {
	match *selector {
		Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector),
	}
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
	// Check type selector
	if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
		return false;
	}

	// Check ID selector
	if selector.id.iter().any(|id| elem.id() != Some(id)) {
		return false;
	}

	// Check class selectors
	let elem_classes = elem.classes();
	if selector
		.class
		.iter()
		.any(|class| !elem_classes.contains(&**class))
	{
		return false;
	}

	// We didn't find any non-matching selector components
	return true;
}

type MatchedRule<'a> = (Specificity, &'a Rule);

// If `rule` matches `elem`, return a `MatchedRule`. Otherwise return `None`.
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
	// Find the first (highest-specificity) matching selector
	rule
		.selectors
		.iter()
		.find(|selector| matches(elem, *selector))
		.map(|selector| (selector.specificity(), rule))
}

// Find all CSS rules that match the given element.
fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
	stylesheet
		.rules
		.iter()
		.filter_map(|rule| match_rule(elem, rule))
		.collect()
}

// Apply styles to a single element, returning the specified values.
fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
	let mut values = HashMap::new();
	let mut rules = matching_rules(elem, stylesheet);

	// Go through the rules from lowest to highest priority
	rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
	for (_, rule) in rules {
		for declaration in &rule.declarations {
			values.insert(declaration.name.clone(), declaration.value.clone());
		}
	}

	return values;
}

pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
	StyledNode {
		node: root,
		specified_values: match root.node_type {
			NodeType::Element(ref elem) => specified_values(elem, stylesheet),
			NodeType::Text(_) => HashMap::new(),
		},
		children: root
			.children
			.iter()
			.map(|child| style_tree(child, stylesheet))
			.collect(),
	}
}
