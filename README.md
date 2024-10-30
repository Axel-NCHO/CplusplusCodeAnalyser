# C++ code analysis

This program uses the library Tree-sitter to get the lexical structure of a C++  
code file as a tree. It then outputs it in a JSON file with simplified nodes that only contains   
the kind of node, its name and its children with the same structure. The nodes' simplification  
is done recursively on the tree.

### How to run it
Ensure you have the rust compiler and cargo installed.  

After that, go to the project's root directory "CPlusPlusParser" and run the program with cargo.  
Here are the arguments :  

| Name  | Long name | Description                               | Default value  |
|-------|-----------|-------------------------------------------|----------------|
| c     | cpp       | Path to a C++ code file                   |                |
| o     | output    | Path to the output file without extension | code_structure |

Command examples

```shell 
cargo run
```
```shell
cargo run -- -c <cpp_file_name> -o <output_file_name>
```
```shell
cargo run -- --cpp <cpp_file_name>
```