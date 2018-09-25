use style::{Display, StyledNode};

// CSS box model. All sizes are in px.

pub struct Dimensions {
	// Position of the content area relative to the document origin.
	pub content: Rect,

	// Surrounding edges:
	pub border: EdgeSizes,
	pub margin: EdgeSizes,
	pub padding: EdgeSizes,
}

impl Dimensions {
	pub fn new() -> Dimensions {
		Dimensions {
			content: Rect::new(),
			border: EdgeSizes::new(),
			margin: EdgeSizes::new(),
			padding: EdgeSizes::new(),
		}
	}
}

pub struct Rect {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

impl Rect {
	pub fn new() -> Rect {
		Rect {
			x: 0.0,
			y: 0.0,
			width: 0.0,
			height: 0.0,
		}
	}
}

pub struct EdgeSizes {
	pub left: f32,
	pub right: f32,
	pub top: f32,
	pub bottom: f32,
}

impl EdgeSizes {
	pub fn new() -> EdgeSizes {
		EdgeSizes {
			left: 0.0,
			right: 0.0,
			top: 0.0,
			bottom: 0.0,
		}
	}
}

pub struct LayoutBox<'a> {
	pub dimensions: Dimensions,
	pub box_type: BoxType<'a>,
	pub children: Vec<LayoutBox<'a>>,
}

impl<'a> LayoutBox<'a> {
	pub fn new(box_type: BoxType<'a>) -> LayoutBox<'a> {
		LayoutBox {
			dimensions: Dimensions::new(),
			box_type,
			children: Vec::new(),
		}
	}

	// Where a new inline child should go
	pub fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
		match self.box_type {
			BoxType::InlineNode(_) | BoxType::AnonymousBlock => self,
			BoxType::BlockNode(_) => {
				// If we've just generated an anonymous block box, keep using it.
				// Otherwise, create a new one.
				match self.children.last() {
					Some(&LayoutBox {
						box_type: BoxType::AnonymousBlock,
						..
					}) => {}
					_ => self.children.push(LayoutBox::new(BoxType::AnonymousBlock)),
				}
				self.children.last_mut().unwrap()
			}
		}
	}
}

pub enum BoxType<'a> {
	BlockNode(&'a StyledNode<'a>),
	InlineNode(&'a StyledNode<'a>),
	AnonymousBlock,
}

// Build the tree of LayoutBoxes, but don't perform any layout calculations yet.
pub fn build_layout_tree<'a>(style_node: &'a StyledNode<'a>) -> LayoutBox<'a> {
	// Create the root box.
	let mut root = LayoutBox::new(match style_node.display() {
		Display::Block => BoxType::BlockNode(style_node),
		Display::Inline => BoxType::InlineNode(style_node),
		Display::None => panic!("Root node has display: none."),
	});

	// Create the descendant boxes.
	for child in &style_node.children {
		match child.display() {
			Display::Block => root.children.push(build_layout_tree(child)),
			Display::Inline => root
				.get_inline_container()
				.children
				.push(build_layout_tree(child)),
			Display::None => {} // Skip nodes with `display: none;`
		}
	}

	root
}
