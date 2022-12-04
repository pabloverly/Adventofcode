use std::fs;

pub fn dia_tres(){
    let contents = fs::read_to_string("date3.txt")
        .expect("erro ler arquivo");
    
    let mut result: Vec<&str> = contents
    .split("\n\n")
    .map(|block| 
        block
        .split('\n')
        .filter(|item| !item.is_empty() )
        // .map(|item| item.parse::<u32>().unwrap())
        .sum::<u32>()
    )
    // .max()
    // .unwrap();
    .collect();
    

    println!("dia 3.: {:?}", result);

}