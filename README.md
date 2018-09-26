# Toy Browser Engine

This is a toy browser engine being written by following the ["Let's build a browser engine"][1] series.

- [x] Part 1: Getting Started
  - [x] Start a new program in the language of your choice, and write code to represent a tree of DOM text nodes and elements.
  - [ ] Install the latest version of [Rust][rust], then download build [robinson][robinson]. Open up [dom.rs][robinson-dom-rs] and extend NodeType to include additional types like comment nodes.
  - [x] Write code to pretty-print a tree of DOM nodes.
- [x] Part 2: HTML
  - [x] Build a parser (either "by hand" or with a library or parser generator) that takes a subset of HTML as input and produces a tree of DOM nodes.
  - [x] Modify robinson's HTML parser to add some missing features, like comments. Or replace it with a better parser, perhaps built with a library or generator.
  - [ ] Create an invalid HTML file that causes your parser (or mine) to fail. Modify the parser to recover from the error and produce a DOM tree for your test file.
- [x] Part 3: CSS
  - [x] Implement your own simplified CSS parser and specificity calculation.
  - [ ] Extend robinson's CSS parser to support more values, or one or more selector combinators.
  - [ ] Extend the CSS parser to discard any declaration that contains a parse error, and follow the [error handling rules][css-error-handling-rules] to resume parsing after the end of the declaration.
  - [x] Make the HTML parser pass the contents of any `<style>` nodes to the CSS parser, and return a `Document` object that includes a list of `Stylesheet`s in additon to the DOM tree.
- [x] Part 4: Style
  - [ ] Cascading
  - [ ] Initial and/or computed values
  - [ ] Inheritance
  - [ ] The `style` attribute
- [x] Part 5: Boxes
- [ ] Part 6: Block Layout
  - [ ] Collapsing vertical margins.
  - [ ] [Relative positioning.][relative-positioning]
  - [ ] Parallelize the layout process, and measure the effect on performance.
- [ ] Part 7: Painting 101
  - [ ] Write an alternate painting function that takes a display list and produces vector output (for example, an SVG file) instead of a raster image.
  - [ ] Add support for opacity and alpha blending.
  - [ ] Write a function to optimize the display list by culling items that are completely outside of the canvas bounds.
  - [ ] If you're familiar with OpenGL, write a hardware-accelerated painting function that uses GL shaders to draw the rectangles.

[lets-build-a-browser-engine]: https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html

[rust]: https://www.rust-lang.org/
[robinson]: https://github.com/mbrubeck/robinson
[robinson-dom-rs]: https://github.com/mbrubeck/robinson/blob/master/src/dom.rs
[css-error-handling-rules]: http://www.w3.org/TR/CSS2/syndata.html#parsing-errors
[relative-positioning]: http://www.w3.org/TR/CSS2/visuren.html#relative-positioning