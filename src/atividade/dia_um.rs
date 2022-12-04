use std::fs;

pub fn dia_um() {
    let contents = fs::read_to_string("date1.txt")
        .expect("Cannot open the file.");

    //Vec<u32> para trazer no console caso culculo so u32
    let mut result: Vec<u32> = contents
        .split("\n\n")
        .map(|block| 
            block
            .split('\n')
            .filter(|item| !item.is_empty() )
            .map(|item| item.parse::<u32>().unwrap())
            .sum::<u32>()
        )
        // .max()
        // .unwrap();
        .collect();
        
        //1 forma - inverter o sort
        // result.sort_by(|a, b| b.cmp(a));
        result.sort();

        let sum: u32= result
            .iter()
            .rev()
            .take(3)
            .sum();

    println!("Dia 1.:  {:?}", sum);

}
