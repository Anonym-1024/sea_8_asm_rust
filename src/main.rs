





mod lexer;




fn main() {
    

    
    let src = std::fs::read_to_string("src/test.txt").expect("could not read");
    let a = lexer::tokenise(src);

    match a {
        Err(x) => println!("{}", x.desc()),
        Ok(x) => {
            for i in &x {
                println!("{i}");
            }
        }
    }
}


