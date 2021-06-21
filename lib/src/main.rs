//===----------------- main.rs - Main file, initial file --------------------===//
//
// This source file is part of the englang open source project
//
// Copyright (c) 2021 KittyX29 and the englang project authors
// Licensed under Apache License v2.0 with Runtime Library Exception
//
// See https://github.com/KittyX29/englang/blob/main/LICENSE for license information
// See https://github.com/KittyX29/englang/blob/main/CONTRIBUTORS.md for the list of englang project authors
//
//===----------------------------------------------------------------------===//

use logos::Logos;
use std::fs::File;
use std::io::prelude::*;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("tell")]
    EngKeywords,

    #[token(".")]
    EngEol,

    
    #[regex("[a-zA-Z]+")]
    EngString,

    #[regex("[1-9]+")]
    EngInt,
    
    #[error]
   
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Spaces,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("myfile.eng")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(for token in Token::lexer(&contents) {
        dbg!(token);
    })
}