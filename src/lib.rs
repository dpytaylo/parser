use std::cell::{Cell, Ref, RefMut, RefCell};
use std::str::Chars;

struct ParserTools<'a> {
    pos: usize,
    iter: Chars<'a>,
    current: char,
}

impl<'a> ParserTools<'a> {
    fn new(iter: Chars<'a>, current: char) -> Self {
        ParserTools {
            pos: 0,
            iter,
            current,
        }
    }
}

pub struct Parser<'a> {
    data: &'a str,

    vars: RefCell<ParserTools<'a>>,
    is_end: Cell<bool>,
}

impl<'a> Parser<'a> {
    pub fn from(data: &'a str) -> Self {
        let mut iter = data.chars();
        match iter.next() {
            Some(current) => {
                Parser {
                    data,
                    vars: RefCell::new(ParserTools::new(iter, current)),
                    is_end: Cell::new(false),
                }
            }
            None => {
                Parser {
                    data,
                    vars: RefCell::new(ParserTools::new(iter, ' ')),
                    is_end: Cell::new(true),
                }
            }
        }
    }

    fn vars(&self) -> Ref<ParserTools<'a>> {
        self.vars.borrow()
    }

    fn mvars(&self) -> RefMut<ParserTools<'a>> {
        self.vars.borrow_mut()
    }

    fn new_iter(&self, from_pos: usize) {
        self.mvars().pos = from_pos;

        self.mvars().iter = (&self.data[from_pos..]).chars();

        let curr = self.mvars().iter.next().unwrap_or_else(|| {
            self.is_end.set(true);
            ' '
        }).clone();
        self.mvars().current = curr;
    }

    pub fn get_char(&self) -> char {
        self.vars().current
    }

    pub fn get_position(&self) -> usize {
        self.vars().pos
    }

    pub fn next(&self) -> char {
        if self.is_end.get() {
            return self.vars().current;
        }

        self.mvars().pos = {
            let tmp = self.vars().pos + self.vars().current.len_utf8();
            tmp
        };

        let previous = self.vars().current;

        let curr = self.mvars().iter.next().unwrap_or_else(|| {
            self.is_end.set(true);
            ' '
        });
        self.mvars().current = curr;

        previous
    }

    pub fn next_count(&self, count: usize) {
        for _ in 0..count {
            self.next();
        }
    }

    pub fn finished(&self) -> bool {
        self.is_end.get()
    }

    pub fn is_space(&self) -> bool {
        let chr = self.get_char();
        chr == ' ' || chr == '\t' || chr == '\n' || chr == '\r'
    }

    pub fn skip(&self) {
        while !self.finished() && self.is_space() {
            self.next();
        }
    }

    pub fn skip_to_str(&self, break_str: &str) {
        let start = self.vars().pos;

        while !self.finished() {
            // Getting slice from start because we can't this: self.vars().pos - break_str.len() (may be invalid utf-8)
            if (&self.data[start..self.vars().pos]).ends_with(break_str) {
                break;
            }
            self.next();
        }

        let end = self.vars().pos - break_str.len();
        self.new_iter(end);
    }

    pub fn get_word(&self) -> &str {
        self.skip();

        let start = self.vars().pos;
        while !self.finished() && !self.is_space() {
            self.next();
        }

        &self.data[start..self.vars().pos]
    }

    // Returns string without break char
    pub fn get_to_char(&self, break_char: char) -> &str {
        let start = self.vars().pos;

        while !self.finished() && self.get_char() != break_char {
            self.next();
        }

        &self.data[start..self.vars().pos]
    }

    // Returns string without break chars
    pub fn get_to_chars(&self, break_chars: &[char]) -> &str {
        let start = self.vars().pos;

        'main: while !self.finished() {
            for break_char in break_chars {
                if self.get_char() == *break_char {
                    break 'main;
                }
            }

            self.next();
        }

        &self.data[start..self.vars().pos]
    }

    // Returns string without break string
    pub fn get_to_str(&self, break_str: &str) -> &str {
        let start = self.vars().pos;

        while !self.finished() {
            if (&self.data[start..self.vars().pos]).ends_with(break_str) {
                break;
            }
            self.next();
        }

        let end = self.vars().pos - break_str.len();
        self.new_iter(end);

        &self.data[start..end]
    }

    // Returns string without break strings
    pub fn get_to_strs(&self, break_strs: &[&str]) -> &str {
        let start = self.vars().pos;

        let mut len = 0;
        'main: while !self.finished() {
            for break_str in break_strs {
                if (&self.data[start..self.vars().pos]).ends_with(break_str) {
                    len = break_str.len();
                    break 'main;
                }
            }
            
            self.next();
        }

        let end = self.vars().pos - len;
        self.new_iter(end);

        &self.data[start..end]
    }

    pub fn get_to_end(&self) -> &str {
        let start = self.vars().pos;

        while !self.finished() {
            self.next();
        }

        &self.data[start..self.vars().pos]
    }
}


// use std::slice::Iter;

// pub struct Parser<'a> {
//     iter: Iter<'a, u8>,
//     current: char,
//     is_end: bool,
// }

// impl<'a> Parser<'a> {
//     pub fn from(code: &'a str) -> Self {
//         let mut iter = code.as_bytes().iter();
//         let value = match iter.next() {
//             Some(val) => *val as char,
//             None => ' ',
//         };

//         Parser {
//             iter,
//             current: value,
//             is_end: false,
//         }
//     }

//     pub fn get_char(&self) -> char {
//         self.current
//     }

//     pub fn next(&mut self) {
//         self.current = match self.iter.next() {
//             Some(val) => *val as char,
//             None => {
//                 self.is_end = true;
//                 ' '
//             },
//         };

//         self.iter.ptr
//     }

//     pub fn follow(&mut self) -> char {
//         let old = self.current;
//         self.next();
//         old
//     }

//     pub fn finished(&mut self) -> bool {
//         self.is_end
//     }

//     pub fn is_space(&mut self) -> bool {
//         self.get_char() == ' ' || self.get_char() == '\t' || self.get_char() == '\n' || self.get_char() == '\r'
//     }

//     pub fn skip_space(&mut self) {
//         while !self.is_end && self.is_space()  {
//             self.next();
//         }
//     }

//     pub fn get_word(&mut self) -> String {
//         let mut buffer = String::new();

//         self.skip_space();
//         while !self.is_end && !self.is_space() {
//             buffer.push(self.follow());
//         }

//         buffer
//     }

//     pub fn get_str_to_char(&mut self, chr: char) -> String {
//         let mut buffer = String::new();

//         while !self.is_end && self.get_char() != chr {
//             buffer.push(self.follow());
//         }

//         buffer
//     }

//     pub fn get_str_to_str(&mut self, break_str: &str) -> String {
//         let mut buffer = String::new();

//         while !self.is_end {
//             buffer.push(self.get_char());

//             if !buffer.ends_with(break_str) {
//                 continue;
//             }

//             buffer.drain(buffer.len() - break_str.len()..);
//             self.current

//             self.next();
//         }

//         buffer
//     }

//     pub fn get_str_to_strs(&mut self, break_strs: &[&str]) -> String {
//         let mut buffer = String::new();

//         'main:  while !self.is_end {
//             buffer.push(self.get_char());

//             for break_str in break_strs {
//                 if buffer.ends_with(break_str) {
//                     buffer.drain(buffer.len() - break_str.len()..);
//                     break 'main;
//                 }
//             }

//             self.next();
//         }

//         buffer
//     }
// }