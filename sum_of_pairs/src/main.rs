fn main() {
    let numbers = [1,3,7,9,8,9]; // Dimensione pari
    //let numbers = [1,3,7,9,3]; // Dimensione dispari
    let sum = 10;
    sum_of_pairs(numbers, sum);
}


fn sum_of_pairs(array:[i32;6], sum: i32) {
    let mut results = Vec::new();
    for(_idx1,e1) in array.iter().enumerate() {
        for(_idx2,e2) in array.iter().enumerate() {
            // Controllo che i 2 valori selezionati in questo mometno equivalgano alla somma
            if e1+e2 == sum {
                // Crea la tupla con il valore più piccolo a sinistra ed il più grande a destra
                // Questa operazione mi servirà per eliminare i duplicati
                if e1 < e2 {
                    let tmp_tuple = (e1,e2);
                    results.push(tmp_tuple);
                }

                if e1 > e2 {
                    let tmp_tuple = (e2,e1);
                    results.push(tmp_tuple);
                }
                
            }
        }
    }
    println!("BEFORE: {:?}", results);
    let no_duplicate_vector = remove_duplicate_tuples(results);
    println!("AFTER: {:?}", no_duplicate_vector);
    if no_duplicate_vector.len() > 1 {
        let final_result = select_smallest(no_duplicate_vector);
        println!("FINAL RESULT: {:?}", final_result);
    } else {
        println!("FINAL RESULT: {:?}", no_duplicate_vector);
    }
    


}


fn remove_duplicate_tuples<'v>(vector_of_tuples: Vec<(&'v i32,&'v i32)>)-> Vec<(&'v i32,&'v i32)>{
    let mut _unique_data = vector_of_tuples.clone();
    let mut output:Vec<(&i32,&i32)> = Vec::new();

    for (_pos1, e1) in _unique_data.to_vec().iter().enumerate() {
        for (_pos2, e2) in _unique_data.to_vec().iter().enumerate() {
            if !output.contains(e1) {
                output.push(*e1);
            }
        }
    }

    return output;
}


fn select_smallest(vector_of_tuples: Vec<(&i32,&i32)>) -> (i32,i32) {
    let mut smallest_tuple = (0,0);
    for(_pos1, e1) in vector_of_tuples.to_vec().iter().enumerate() {
        for(_pos2, e2) in vector_of_tuples.to_vec().iter().enumerate() {
            if e1.1 < e2.1 {
                smallest_tuple = (*e1.0,*e1.1);
            }
        }   
    }
    return smallest_tuple;
}
