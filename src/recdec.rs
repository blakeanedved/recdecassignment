use std::{iter::Peekable, str::Chars};

pub struct RecursiveDescentParser<'a> {
    buff: Peekable<Chars<'a>>,
    assignments: i32
}

impl<'a> RecursiveDescentParser<'a> {
    pub fn new(buff: &'a String) -> RecursiveDescentParser {
        RecursiveDescentParser {
            buff: buff.chars().peekable(),
            assignments: 0
        }
    }

    pub fn get_assignments(&self) -> i32 {
        self.assignments
    }

    fn ignore_whitespace(&mut self) -> Option<char> {
        self.buff.find(|&x| match &x {
            '\n' | '\t' | ' ' | '\r' => false,
            _ => true,
        })
    }

    pub fn expect_program(&mut self) -> Result<(), &'static str> {
        let c = self.ignore_whitespace();

        if c == Some('p') {
            self.expect_block()?;

            let end = self.ignore_whitespace();
            if end == Some('.') {
                Ok(())
            } else {
                Err("end of program not found")
            }
        } else {
            Err("no program found")
        }
    }

    pub fn expect_block(&mut self) -> Result<(), &'static str> {
        let c = self.ignore_whitespace();
        if c == Some('b') {
            self.expect_stmtlist()?;

            let end = self.ignore_whitespace();
            if end == Some('n') {
                Ok(())
            } else {
                Err("end of block not found")
            }
        } else {
            Err("no block found")
        }
    }

    pub fn expect_stmtlist(&mut self) -> Result<(), &'static str> {
        self.expect_stmt()?;
        self.expect_morestmts()?;
        Ok(())
    }

    pub fn expect_morestmts(&mut self) -> Result<(), &'static str> {
        loop {
            match self.buff.peek() {
                Some('\n') | Some('\t') | Some(' ') | Some('\r') => { self.buff.next(); },
                Some(';') => {break},
                _ => { return Ok(()); }
            }
        }

        self.buff.next();

        self.expect_stmtlist()?;

        Ok(())
    }

    pub fn expect_stmt(&mut self) -> Result<(), &'static str> {
        self.expect_assign()
    }

    pub fn expect_assign(&mut self) -> Result<(), &'static str> {
        self.expect_variable()?;
        let c = self.ignore_whitespace();
        if c == Some('=') {
            self.expect_expr()?;
            self.assignments += 1;
            Ok(())
        } else {
            Err("no equals sign")
        }
    }

    pub fn expect_expr(&mut self) -> Result<(), &'static str> {
        loop {
            match self.buff.peek() {
                Some('\n') | Some('\t') | Some(' ') | Some('\r') => { self.buff.next(); },
                Some('a') | Some('b') | Some('c') => { return self.expect_variable() },
                Some('0') | Some('1') | Some('2') => { return self.expect_digit() },
                Some('+') => {break},
                _ => { return Err("no expr found") }
            }
        }

        self.buff.next();

        self.expect_expr()?;
        self.expect_expr()?;

        Ok(())

    }

    pub fn expect_variable(&mut self) -> Result<(), &'static str> {
        match self.ignore_whitespace() {
            Some('a') | Some('b') | Some('c') => Ok(()),
            _ => Err("no variable found")
        }
    }

    pub fn expect_digit(&mut self) -> Result<(), &'static str> {
        match self.ignore_whitespace() {
            Some('0') | Some('1') | Some('2') => Ok(()),
            _ => Err("no digit found")
        }
    }
}
