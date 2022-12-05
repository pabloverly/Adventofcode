use std::fs;


pub fn dia_um() {
    let result = fs::read_to_string("date01_2015.txt")
    .expect("Erro ler arquivo");
    
    let andar: u32 = result
        .split("(")
        .map(|item| item.parse::<u32>().unwrap())
        .sum::<u32>();
        

    println!("{:?}", result);
}