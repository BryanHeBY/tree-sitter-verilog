use tree_sitter::Parser;

fn main() {
    let mut parser = Parser::new();
    let language = tree_sitter_verilog::language();
    parser
        .set_language(language)
        .expect("Error loading Rust grammar");
    let mut tree = parser
        .parse(
            r#"
module A();
wire a, b;
endmodule

module B();
wire a, b;
assign a = b + 1;
endmodule

module C();
wire a, b;
assign a = b + 1;
endmodule
"#,
            None,
        )
        .unwrap();
    show(&tree.root_node(), 0);
    tree.edit(&tree_sitter::InputEdit {
        start_byte: 24,
        old_end_byte: 24,
        new_end_byte: 42,
        start_position: tree_sitter::Point { row: 0, column: 0 },
        old_end_position: tree_sitter::Point { row: 0, column: 0 },
        new_end_position: tree_sitter::Point { row: 0, column: 0 },
    });
    let tree = parser
        .parse(
            r#"
module A();
wire a, b;
assign a = 3 + 2;
endmodule

module B();
wire a, b;
assign a = b + 1;
endmodule

module C();
wire a, b;
assign a = b + 1;
endmodule
"#,
            Some(&tree),
        )
        .unwrap();
    show(&tree.root_node(), 0);
}

fn show(node: &tree_sitter::Node, depth: usize) {
    let kind = node.kind();
    let mut kind = if !node.is_named() {
        format!("\"{}\"", kind)
    } else {
        format!("{}", kind)
    };
    if node.is_missing() {
        kind = format!("{} (missing)", kind);
    }
    println!(
        "{}{} @ {}..{}",
        " ".repeat(depth * 2),
        kind,
        node.start_position(),
        node.end_position()
    );
    for children in node.children(&mut node.walk()) {
        let child = children;
        show(&child, depth + 1);
    }
}
