
type Token<'a> = &'a str;

pub struct TokenResult<'a> {
    pub token: Token<'a>,
    pub remainder: &'a str,
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    pub remainder: &'a str,
    pub separators: &'static [char],
    pub separator_index: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(seps: &'static [char], line: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            remainder: line,
            separators: seps,
            separator_index: 0,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.separator_index >= self.separators.len() {
            return Some(self.remainder);
        }
        let separator = self.separators[self.separator_index];
        self.separator_index += 1;
        let token = next_token(self.remainder, separator);
        self.remainder = token.remainder;

        Some(token.token)
    }
}

pub fn next_token<'a>(data: &'a str, sep: char) -> TokenResult<'a> {
    let v: Vec<&str> = data.splitn(2, sep).collect();

    TokenResult {
        token: v[0],
        remainder: v[1],
    }
}
