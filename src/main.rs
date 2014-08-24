use std::io::BufferedReader;
use std::io::File;
use std::collections::HashSet;
use std::num::from_str_radix;

struct PrefixRDP {
    lookahead: Option<String>,
    tokens: Vec<String>
}

struct ZeroOneRDP {
    lookahead: Option<String>,
    tokens: Vec<String>
}

struct Lexer {
    tokens: HashSet<char>,
    keywords: HashSet<String>
}

struct Token  {
    line_number: int,
    token: String,
    tag: Tag
}

enum Tag {
    Number,
    ID,
    True,
    False,
    SEMICOLON,
    BACKSLASH,
    EQUALS,
    MINUS,
    PLUS,
    ASTERISK,
    FORWARDSLASH,
    QUESTIONMARK,
    LEFTBRACKET,
    RIGHTBRACKET,
    LEFTBRACE,
    RIGHTBRACE,
    LEFTPAREN,
    RIGHTPAREN,
    SINGLEQUOTE,
    DOUBLEQUOTE,
    COLON,
    LESSTHAN,
    GREATERTHAN,
    ANDSYMBOL,
    Nil
}

impl Tag  {
    fn string(&self) -> String {
        match *self {
            Number => { return "Number".to_string(); }
            ID => { return "ID".to_string(); }
            True => { return "True".to_string(); }
            False => { return "False".to_string(); }
            SEMICOLON => { return "SEMICOLON".to_string(); }
            BACKSLASH => { return "BACKSLASH".to_string(); }
            EQUALS => { return "EQUALS".to_string(); }
            MINUS => { return "MINUS".to_string(); }
            PLUS => { return "PLUS".to_string(); }
            ASTERISK => { return "ASTERISK".to_string(); }
            FORWARDSLASH => { return "FORWARDSLASH".to_string(); }
            QUESTIONMARK => { return "QUESTIONMARK".to_string(); }
            LEFTBRACKET => { return "LEFTBRACKET".to_string(); }
            RIGHTBRACKET => { return "RIGHTBRACKET".to_string(); }
            LEFTBRACE => { return "LEFTBRACE".to_string(); }
            RIGHTBRACE => { return "RIGHTBRACE".to_string(); }
            LEFTPAREN => { return "LEFTPAREN".to_string(); }
            RIGHTPAREN => { return "RIGHTPAREN".to_string(); }
            SINGLEQUOTE => { return "SINGLEQUOTE".to_string(); }
            DOUBLEQUOTE => { return "DOUBLEQUOTE".to_string(); }
            LESSTHAN => { return "LESSTHAN".to_string(); }
            GREATERTHAN => { return "GREATERTHAN".to_string(); }
            ANDSYMBOL => { return "ANDSYMBOL".to_string(); }
            COLON => { return "COLON".to_string(); }
            Nil => { return "Nil".to_string(); }
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Token {
        return Token {
            line_number: self.line_number.clone(),
            token: self.token.clone(),
            tag: self.tag
        }
    }
    fn clone_from(&mut self, source: &Token) {
        self.line_number = source.line_number.clone();
        self.token = source.token.clone();
        self.tag = source.tag;
    }
}

impl Lexer {
    fn new(tokens: &'static str, keywords: Vec<&'static str>) -> Lexer {
        let mut token_set = HashSet::new();
        let mut keyword_set = HashSet::new();
        for c in tokens.chars() {
            token_set.insert(c);
        }

        for k in keywords.iter() {
            keyword_set.insert(k.to_string());
        }

        Lexer {
            tokens: token_set,
            keywords: keyword_set
        }
    }

    fn tokenize_file(&self, path: Path) -> Vec<Token> {
        let mut file = BufferedReader::new(File::open(&path));
        let mut out_tokens : Vec<Token> = Vec::new();
        let mut s = 0;
        for line in file.lines() {
            s += 1;
            let mut tokens = self.tokenize(line.unwrap());
            for string_token in self.filter_line_comments(tokens).iter() {
                out_tokens.push(Token {
                    line_number: s,
                    token: string_token.clone(),
                    tag: self.determine_tag(string_token.clone())
                })
            }
        }
        return out_tokens;
    }

    fn filter_line_comments(&self, mut tokens: Vec<String>) -> Vec<String> {
        let mut s = 0;
        while s < tokens.len() {
            if tokens[s] == "/".to_string() {
                if s + 1 < tokens.len() && tokens[s] == "/".to_string() {
                    tokens.truncate(s);
                }
            }
            s += 1;
        }
        return tokens;
    }

    fn determine_tag(&self, token: String) -> Tag {
        // What type of element is this
        let s : Option<int>= from_str_radix(token.as_slice(), 10);
        if s.is_some() { return Number; }
        if token.len() == 1 {
            match token.as_slice().char_at(0) {
                '(' => { return LEFTPAREN; }
                ')' => { return RIGHTPAREN; }
                '{' => { return LEFTBRACE; }
                '}' => { return RIGHTBRACE; }
                '[' => { return LEFTBRACKET; }
                ']' => { return RIGHTBRACKET; }
                '?' => { return QUESTIONMARK; }
                '/' => { return FORWARDSLASH; }
                '<' => { return LESSTHAN; }
                '>' => { return GREATERTHAN; }
                '*' => { return ASTERISK; }
                '+' => { return PLUS; }
                '-' => { return MINUS; }
                '=' => { return EQUALS; }
                '\'' => { return SINGLEQUOTE; }
                '\"' => { return DOUBLEQUOTE; }
                '\\' => { return BACKSLASH; }
                ';' => { return SEMICOLON; }
                ':' => { return COLON; }
                '&' => { return ANDSYMBOL; }
                _ => { }
            }
        } else {
            if self.keywords.contains(&token) {
                return True;
            }
        }
        return ID;
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
        let lookahead = tokens.remove(0);
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
        if self.lookahead.clone().unwrap() == string {
            self.lookahead = self.tokens.remove(0);
        }
        else {
            fail!("syntax error");
        }
    }
}

impl ZeroOneRDP {
    fn new(mut tokens: Vec<String>) -> ZeroOneRDP {
        let lookahead = tokens.remove(0);
        ZeroOneRDP {
            lookahead: lookahead,
            tokens: tokens
        }
    }

    fn parse(&mut self){
        self.stmt();
        if self.lookahead.is_some() { println!("Syntax Error"); }
    }

    fn stmt(&mut self) {
        let zero = "0".to_string();
        let one = "1".to_string();

        let token = self.lookahead.clone();

        if token.is_none(){ return; }

        match token.unwrap().as_slice() {
            "0" => { self._match(zero); self.optexpr(); self._match(one); }
            _  => { return; },
        }
    }

    fn optexpr(&mut self) {
        let zero = "0".to_string();
        if self.lookahead.clone().unwrap() == zero {
            self.stmt();
        }
    }
    fn _match(&mut self, string: String) {
        if self.lookahead.clone().unwrap() == string {
            self.lookahead = self.tokens.remove(0);
        }
        else {
            return;
        }
    }
}


trait RDP {
    fn stmt(&self);
    fn optexpr(&self);
    fn _match(&self, string: String);
}

fn main() {
    let standard_tokens = "&(){}[]?/<>*+-='\"\'\\:;";
    let standard_keywords = vec![
        "int", "for", "while", "and", "bool", "if", "or", "return",
        "true", "false"
    ];
    let path = Path::new("/Users/BenP/projects/parser/program.txt");

    let lexer = Lexer::new(standard_tokens, standard_keywords);
    let tokens = lexer.tokenize_file(path);

    println!("Our tokens are");
    for token in tokens.iter() {
        println!("Line {} token {} tag {}", token.line_number, token.token, token.tag.string());
    }

    // let zero_one_tokenizer = Lexer::new("01");
    // let zero_one_tokens = zero_one_tokenizer.tokenize("0011".to_string());

    // // let mut parser = PrefixRDP::new(tokens);
    // // parser.stmt();

    // let mut parser = ZeroOneRDP::new(zero_one_tokens);
    // parser.parse();
}
