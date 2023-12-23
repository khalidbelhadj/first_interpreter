use lexer::Lexer;

mod token;
mod lexer;
mod parser;
mod ast;
mod vm;
mod code_generator;

fn main() {
    let file_path = "test.txt";
    let content = std::fs::read_to_string(&file_path).expect(format!("Unable to read file {file_path}").as_str());

    let mut lexer = Lexer::new(content.clone());
    lexer.lex();
    let mut parser = parser::Parser::new(lexer.tokens);
    parser.parse();
    println!("{:#?}", parser.root);
}
