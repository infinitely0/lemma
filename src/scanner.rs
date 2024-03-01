pub struct Scanner {
    input: Vec<char>,
    line: usize,
    // current position
    index: usize,
    // lexeme start
    start: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let input: Vec<char> = source.chars().collect();
        Self {
            input,
            line: 1,
            index: 0,
            start: 0,
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        let c = self.input.get(self.index).cloned();
        if c == Some('\n') {
            self.line += 1;
        }
        self.index += 1;
        c
    }

    pub fn peek(&self) -> Option<char> {
        self.input.get(self.index).cloned()
    }

    pub fn take_lexeme(&mut self) -> String {
        self.input[self.start..self.index].iter().collect()
    }

    pub fn reset_start(&mut self) {
        self.start = self.index;
    }

    pub fn line(&self) -> usize {
        self.line
    }
}
