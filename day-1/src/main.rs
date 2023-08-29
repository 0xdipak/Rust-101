Why Rust ?
- High level lamguage features without performance penalties
- Program behaviours can be enforced at compile time
    - Enhanced program reliability
- Built-in dependency management, similar to npm
- Quickly growing ecosystem of libraries
- Friendly & welcoming developer community

Technical Rust Goodies :
- First-class multithreading
    - Compiler error to improperly access shared data
- Type system
    - Can uncover bugs at compile time
    - Makes refactoring simple
    - Reduces the number of tests needed
- Module system makes code separation simple
- Adding a dependency is 1 line in a config file 
- Tooling 
    - Generate docs, lint code, auto format



=======================================================  DAY-1 ==========================================================
Data Types :
- Memory only stores binary data
    - Anthing can be represented in binary
- Program determines what the binary represents
- Basic types that are universally useful are provided by the language.

Basic Data Types :
- Boolen 
    - true, false
- Integer
    - 1,2,50,99,-2
- Double / Float
    - 1.1, 5.5, 200.001
- Character 
    - 'A', 'B', 'c', '6' , '$'
- String'
    - "Hello", "my name is max", "it's 42"



fn main() {
    println!("Hello, world!");
}
