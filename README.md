# Project of the Week: Rust
Learning Rust Fundamentals

Week 1:

Compiling and Running Rust (done in separate steps)
* Compile first:
    * rustc filename.rs
    * Shows executable file <filename> and source file filename.rs
* Then run:
    * ./filename
* Different from dynamic languages like Python or Javascript where programs are compiled and run together 

Package Management in Rust
*  Cargo
    * Tool used to manage Rust projects
    * Handles building code, downloading libraries code depends on, building said libraries
    * cargo new project_name
        * Creates new dir and project and files inside dir
    * Anatomy of cargo project
        * Cargo.toml
        * Src 
        * Initializes new git repo along with .gitignore
    * Cargo.toml? What is that?

      anatomy:

          [package]
      
            Name = “….”
      
            Version = “0.1.0”
      
            Edition = “2023"
   
           [dependencies]
      
        * package
            * Lines underneath configure package
        * dependecies
            * List any project dependencies 
    * Building and running cargo project
        * Cargo build
            * Creates executable file in target/debug/project_name
            * Cargo puts binary in debug dir by default 
        * Cargo run
            * Builds and runs executable in one step 
        * Cargo check
            * Check if code compiles without producing an executable
    * Building software for release
        * Cargo build —release
            * Creates executable in target/release instead of target/debug
            * Meant for when program is ready to be widely used/adopted to users
