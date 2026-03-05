#[test]
fn test_manhatten_ways() {
    const X:usize = 17;
    const Y:usize = 13;
    let mut field:[[usize;X];Y] = [[0;X];Y];
    for i in 0..X {
        field[Y-1][i] = 1;
    }
    for j in 0..Y {
        field[j][X-1] = 1;
    }
    

    for j in (0..Y-1).rev() {
        for i in (0..X-1).rev() {
            field[j][i] = field[j+1][i] + field[j][i+1];
        }
    }

    for j in 0..Y {
        for i in 0..X {
            print!("{:6}", field[j][i]);
        }
        println!("");
    }

    println!("final result: {}", field[0][0]);
}