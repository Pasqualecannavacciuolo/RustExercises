use std::collections::{HashMap};

fn main() {
    let a = [121, 144, 19, 161, 19, 144, 19, 11]; 
    let b = [121, 14641, 20736, 361, 25921, 361, 20736, 361];
    //let b = [25921, 361, 20736, 361];
    println!("{}",cmp(a, b));
}

fn cmp(a: [i32;8], b: [i32;8]) -> bool{
    let mut occurrencies = HashMap::new();
    for (_index,number) in a.iter().enumerate() {
        let quadratic_number = number*number;
        for (_index2,number2) in b.iter().enumerate() {
            if quadratic_number == *number2 {
                occurrencies.insert(number, quadratic_number);
            }
        }
        //println!("{:?}", occurrencies);
    }
    let mut cont = 0;
    for iterator in occurrencies.keys() {
        if a.contains(&iterator) {
            cont=cont+1;
        }
    }
    
    let mut a_dedup = Vec::new();
    for e in a {
        if !a_dedup.contains(&e) {
            a_dedup.push(e);
        }
    }
    a_dedup.sort();

    let mut keys = Vec::new();
    for e in occurrencies.keys().into_iter() {
        keys.push(**e as i32);
    }
    keys.sort();

    //println!("{:?}", a_dedup);
    //println!("{:?}", keys);

    if a_dedup == keys {
        return true;
    }

    return false;
}