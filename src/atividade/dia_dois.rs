use std::io;
// cargo run < date2.txt
#[allow(unused, dead_code)]
pub  fn dia_dois(){
    let mut result =  0;
    for line in io::stdin().lines(){
        let line_result = match line.unwrap().as_ref(){
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => 0,
        };
        result +=  line_result;
    }
    println!("Dia 2.: {}", result);
}


/* 1 part
A ou x = Pedra   BONUS = 1
B ou y = Papel   BONUS = 2 
C ou z = Tesoura BONUS = 3
+ SCORE
PERDER  = 0
EMPATE  = 3
GANHA   = 6

MATRIZ DE PONTUACAO

A VS X = EMPATE (1 + 3) = 4
A VS Y = GANHEI (2 + 6) = 8
A VS Z = PERDI (3 + 0) = 3
B VS X = PERDI (1 + 0) = 1
B VS Y = ENPATE (2 + 3) = 5
B VS Z = GANHEI (3 + 6) = 9
C VS X = GANHEI (1 + 6) = 7
C VS Y = PERDI (2 + 0) = 2
C VS Z = EMPATE (3 + 3) = 6 
*/

/* 2 part
A = Pedra   BONUS = 1
B = Papel   BONUS = 2 
C = Tesoura BONUS = 3

perdi = 0
empate = 3
ganhar = 6

*/