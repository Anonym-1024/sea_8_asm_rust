use std::time::Instant;

use crate::parser::result::ParserResult;







mod lexer;
mod parser;

#[derive(Debug)]
struct a {
    i: i32
}

fn main() {
    
    


    let start = Instant::now();
    
    let mut src = std::fs::read_to_string("resources/test.txt").expect("could not read");
    src.push('\n');
    let a = lexer::tokenise(src);
    
    
    match &a {
        Err(x) => println!("{}", x.desc()),
        Ok(x) => {
            
            for i in x {
                //println!("{i}");
            }

            let p = parser::parse(x);

            match &p {
                ParserResult::Err(err) => println!("{}", err.desc()),
                ParserResult::Some(s) => {println!("{:#?}", s)},
                ParserResult::None => {panic!("")}
            }
        }
    }


    
    let end = start.elapsed().as_millis();

    println!("Time: {}", end);
}


