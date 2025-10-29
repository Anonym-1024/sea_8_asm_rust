use std::time::Instant;







mod lexer;
mod parser;




fn main() {
    
    let start = Instant::now();
    
    let src = std::fs::read_to_string("resources/text.txt").expect("could not read");
    let a = lexer::tokenise(src);
    let end = start.elapsed().as_millis();
    match a {
        Err(x) => println!("{}", x.desc()),
        Ok(x) => {
            for i in &x {
                println!("{i}");
            }
        }
    }

    println!("Time: {}", end);
}


