extern crate tree_sitter_cpp;
use clap::{arg, Parser};
use serde::Serialize;
use std::fs::File;
use std::io::{Read, Write};
use tree_sitter::Node;


#[derive(Serialize)]
/// A simplified version of tre_sitter's node for serialization.
/// Only retains the kind of node, its name and its children.
struct CodeNode {
    kind: String,
    name: Option<String>,
    children: Vec<CodeNode>,
}


#[derive(clap::Parser, Debug)]
#[command(about, long_about = None)]
/// Command-line arguments
struct Args {
    /// Path to the C++ code to parse - Optional
    #[arg(short, long, default_value_t = String::new())]
    cpp: String,

    /// Path to output file without extension - Optional
    #[arg(short, long, default_value_t = String::from("code_structure"))]
    output: String,
}

fn get_default_cpp_code() -> String {
    String::from(r#"
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
    "#)
}

/// Retrieve the cpp file's content if any. Else, return the default cpp code.
fn get_cpp_code_as_string(path: String) -> String {
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return get_default_cpp_code()
    };

    let mut content = String::new();

    if file.read_to_string(&mut content).is_err() {
        return get_default_cpp_code()
    }

    content
}

fn to_json(node: &CodeNode, path: &String) -> () {
    let json_output = serde_json::to_string_pretty(node).expect("Failed to serialize JSON");
    let mut file = File::create(&path).expect("Failed to create output file");
    file.write_all(json_output.as_bytes()).expect("Failed to write JSON to file");
}


fn main() {
    let args = Args::parse();
    let cpp_code = get_cpp_code_as_string(args.cpp);

    // Initializing the C++ parser
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_cpp::LANGUAGE.into())
        .expect("Error loading C++ grammar");

    // Parsing the C++ code
    let tree = parser.parse(&cpp_code, None).unwrap();
    let root_node = tree.root_node();

    let code_structure = to_code_node(&root_node, &cpp_code);

    let out = args.output + ".json";
    to_json(&code_structure, &out);

    println!("Code structure saved to {}", out);
}

/// Simplifies tree_sitter's node for serialization.
fn to_code_node(node: &Node, source_code: &String) -> CodeNode {
    let kind = node.kind().to_string();

    let name = if kind.ends_with("identifier")  || kind.ends_with("type") {
        node.utf8_text(source_code.as_bytes()).ok().map(|name| name.to_string())
    } else { None };

    let mut children = Vec::new();
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() != "comment" {  // Ignore comments
                children.push(to_code_node(&child, source_code));
            }
        }
    }

    CodeNode { kind, name, children }
}
