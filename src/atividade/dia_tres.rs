use std::fs;

pub fn dia_tres(){
    let contents = fs::read_to_string("date3.txt")
        .expect("erro ler arquivo");
    
    let mut result: Vec<&str> = contents
    .split("\n\n")
    .collect();
    

    println!("dia 3.: {:?}", result);

}