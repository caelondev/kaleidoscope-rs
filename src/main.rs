use kaleidoscope_rs::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("def extern world");
    while !lexer.is_at_end() {
        print!("token: {:?}\n", lexer.get_tok());
    }
}
