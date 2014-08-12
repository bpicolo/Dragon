use std::io::File;
use std::collections::HashSet;


struct PrefixRDP {
    lookahead: Option<String>,
    tokens: Vec<String>
    }

struct Tokenizer {
    tokens: HashSet<char>
}

impl Tokenizer {
    fn new(tokens: &'static str) -> Tokenizer {
        let mut token_set = HashSet::new();
        for c in tokens.chars() {
            token_set.insert(c);
        }

        Tokenizer {
            tokens: token_set
        }
    }

    fn tokenize(&self, string: String) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut s = string.as_slice();
        let mut start = 0;
        let mut end = 0;
        while s.len() > 0 {
            let idx = s.find(|c: char| !c.is_whitespace());

            if idx.is_none() {
                return tokens;
            } else {
                start = idx.unwrap();
                end = start;
            }

            while end < s.len() && !s.char_at(end).is_whitespace(){
                if self.tokens.contains(&s.char_at(end)) {
                    if start == end {
                        end+=1;
                    }
                    break;
                }
                end+=1;
            }

            tokens.push(s.slice(start, end).to_string());

            s = s.slice_from(end);
        }

        return tokens;
    }

}

impl PrefixRDP {
    fn new(mut tokens: Vec<String>) -> PrefixRDP {
        let lookahead = tokens.shift();
        PrefixRDP {
            lookahead: lookahead,
            tokens: tokens
        }
    }

    fn stmt(&mut self) {
        let plus = "+".to_string();
        let minus = "-".to_string();
        let a = "a".to_string();

        let token = self.lookahead.clone();

        if token.is_none(){ return; }

        match token.unwrap().as_slice() {
            "+" => { self._match(plus); self.stmt(); self.stmt(); }
            "-" => { self._match(minus); self.stmt(); self.stmt(); }
            "a" => { self._match(a); }
            _  => { fail!("syntax error"); },
        }
    }

    fn optexpr(&self) {}
    fn _match(&mut self, string: String) {
        if (self.lookahead.clone().unwrap() == string) {
            self.lookahead = self.tokens.shift();
        }
        else {
            fail!("syntax error");
        }
    }
}


trait RDP {
    fn stmt(&self);
    fn optexpr(&self);
    fn _match(&self, string: String);
}

fn main() {
    let standard_tokens = "(){}[]?/*+-='\";";
    let path = Path::new("/Users/BenP/projects/parser/program.txt");

    let tokenizer = Tokenizer::new(standard_tokens);

    let tokens = tokenizer.tokenize("+ a -aa a".to_string());

    let mut parser = PrefixRDP::new(tokens);

    parser.stmt();
}
