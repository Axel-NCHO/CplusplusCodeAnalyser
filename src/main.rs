use std::fs::File;
use std::io::Write;
use serde::Serialize;
use tree_sitter::{Node, Parser};
extern crate tree_sitter_cpp;

#[derive(Serialize)]
struct CodeNode {
    kind: String,
    name: Option<String>,
    children: Vec<CodeNode>,
}


fn main() {
    // Some C++ code to parse
    let cpp_code = r#"
    #include <iostream>

    struct Number {
        int value;
    };
    
    void say_hello(const Number& x_times);

    int main() {
        Number x;
        x.value = 10;
        say_hello(x);
    }

    // Function to say hello
    void say_hello(const Number& x_times) {
        std::cout << "Hello " << x_times.value << " from function!" << std::endl;
    }
    "#;

    // Initializing the C++ parser
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_cpp::LANGUAGE.into())
        .expect("Error loading C++ grammar");

    // Parsing the C++ code
    let tree = parser.parse(cpp_code, None).unwrap();
    let root_node = tree.root_node();

    let code_structure = convert_node_to_json(&root_node, &cpp_code);
    
    // Writing the structure to a JSON file
    let json_output = serde_json::to_string_pretty(&code_structure).expect("Failed to serialize JSON");
    let mut file = File::create("code_structure.json").expect("Failed to create output file");
    file.write_all(json_output.as_bytes()).expect("Failed to write JSON to file");

    println!("Code structure saved to code_structure.json");
}

fn convert_node_to_json(node: &Node, source_code: &str) -> CodeNode {
    let kind = node.kind().to_string();

    let name = if kind.ends_with("identifier")  || kind.ends_with("type") {
        node.utf8_text(source_code.as_bytes()).ok().map(|name| name.to_string())
    } else { None };

    let mut children = Vec::new();
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() != "comment" {  // Ignore comments
                children.push(convert_node_to_json(&child, source_code));
            }
        }
    }

    CodeNode { kind, name, children }
}
