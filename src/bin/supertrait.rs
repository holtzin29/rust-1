#![allow(unused)]

trait Language {
    fn name(&self) -> String;
}
trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}
// have to need lang and compiler trait in then
trait CompiledLanguage: Language + Compiler {
fn exec(&self, file_path: &str) {
    let name = &self.name();
    println!("name is {name}");
    let compiler = &self.compile(file_path);
    println!("compiler is {compiler}"); // sort of inheritance
}
}
struct Rust;

impl Language for Rust {
    fn name(&self) -> String {
        "rust".to_string()
    }
}
impl Compiler for Rust {
    fn compile(&self, file_path: &str) -> String {
        format!("cargo build {file_path}")
    }
}
impl CompiledLanguage for Rust {
    // fn execute() if want not default
} // will execute default
fn main() {
  let rust = Rust;
  rust.exec("hello.rs");
}