#![allow(unused)]

struct Solidity {
    version: String,
}

struct Vyper {
    version: String
}

// accept both solidity and vyper struct 
trait Compiler {
    // self is to compile who implemets this
    fn compile(&self, file_path: &str) -> String;
}
trait Test {
    fn test(&self, file_path: &str) -> String {
        format!("test {}", file_path)
    }
}
impl Test for Solidity {
    fn test(&self, file_path: &str) -> String {
        format!("forge {}", file_path)
    }
}
impl Test for Vyper {
}
impl  Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc {file_path}")
    }
}
impl  Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("vyper {file_path}")
    }
}
// reference to anyone who implements the compiler trait 
fn compile(lang: &impl Compiler, file_path: &str) -> String {
    return lang.compile(file_path)
}
fn main() {
    let sol = Solidity {
        version: "0.8".to_string(),
    };
    let vy = Vyper {
        version: "0.4".to_string(),
    };
    println!("compile sol {}", compile(&sol, "hello.soL"));
    println!("compile vyper {}", compile(&vy, "hello.vy"));
    println!("test sol {}", sol.test("helo sol")); // different implementation 
    println!("test sol {}", vy.test("hello vyper")); // default implementation

}