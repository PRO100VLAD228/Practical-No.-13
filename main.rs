fn count_permutation(shipments: &Vec<u32>) -> Result<usize, i32> {
    let total_weight: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total_weight % n != 0 {
        return Err(-1);
    }

    let target_weight = total_weight / n;
    let mut moves = 0;
    let mut balance = 0;

    for &weight in shipments {
        balance += weight as i32 - target_weight as i32;
        moves += balance.abs() as usize;
    }

    Ok(moves)
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut shipments = vec![0; n];
    
    for i in 0..(n as u32) {
        shipments[(i as usize) % n] += 1;
    }

    shipments
}

fn main() {
    let example1 = vec![8, 2, 2, 4, 4];
    let example2 = vec![9, 3, 7, 2, 9];

    match count_permutation(&example1) {
        Ok(moves) => println!("Moves needed for example1: {}", moves),
        Err(_) => println!("Equal distribution not possible for example1"),
    }

    match count_permutation(&example2) {
        Ok(moves) => println!("Moves needed for example2: {}", moves),
        Err(_) => println!("Equal distribution not possible for example2"),
    }

    let generated_shipments = gen_shipments(5);
    println!("Generated shipments: {:?}", generated_shipments);
}
