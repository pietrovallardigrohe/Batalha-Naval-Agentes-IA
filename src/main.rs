fn main() {
    let p1_board = [[0u8; 10]; 10];


    for row in p1_board {
        for column in row {
            print!("{column} ")
        }
        println!("");
    }
}
