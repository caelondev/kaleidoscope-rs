#[derive(Debug)]
pub enum Token {
    Eof,
    Def,
    Extern,
    Identifier,
    Number,

    Unknown(char),
}
