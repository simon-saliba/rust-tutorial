fn multiply_into_another_array() {
    println!("multiply into another array");

    let array: [u32; 5] = [0, 1, 2, 3, 4];
    let mut result: [u32; 5] = [0; 5];

    for i in 1..5 {
        result[i] = array[i] * 2;
    }

    println!("original array: {:?}", array);
    println!("result array: {:?}", result);
}


fn multiply_into_itself() {
    println!("multiply into itself");
    let mut array: [u32; 5] = [1, 2, 3, 4, 5];

    println!("original array: {:?}", array);

    for element in array.iter_mut() {
        *element = *element * 2;
    }

    println!("mutated array: {:?}", array);
}

fn multiply_using_map() {
    println!("multiply using map");
    let array: [u32; 5] = [1, 2, 3, 4, 5];
    let result: Vec<u32> = array.iter().map(|x| x * 2).collect();

    println!("original array: {:?}", array);
    println!("result array: {:?}", result);
}

fn main() {
    multiply_into_another_array();
    multiply_into_itself();
    multiply_using_map();
}