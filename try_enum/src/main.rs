enum LanguageKind {
    Python(String),
    Rust(String),
    Elixir(String),
}

impl LanguageKind {
    fn introduce(&self) {
        match self {
            LanguageKind::Python(s) => println!("Hello, {}", s),
            LanguageKind::Rust(s) => println!("Hello, {}", s),
            LanguageKind::Elixir(s) => println!("Hello, {}", s),
        }
    }
}

fn main() {
    println!("Enum lets you declare some value is of a set.");
    let python = LanguageKind::Python(String::from("Python"));
    let rust = LanguageKind::Rust(String::from("Rust"));
    let elixir = LanguageKind::Elixir(String::from("Elixir"));

    python.introduce();
    rust.introduce();
    elixir.introduce();
}
