fn sum_two_arrays() {
    println!("Sum two arrays");
    let first_array: [u32; 5] = [1, 2, 3, 4, 5];
    let second_array: [u32; 5] = [2, 3, 4, 5, 6];

    let mut result: [u32; 5] = [0; 5];

    for i in 0..5 {
        result[i] = first_array[i] + second_array[i];
    }

    println!("first array: {:?}", first_array);
    println!("second array: {:?}", second_array);
    println!("result array: {:?}", result);
}

fn subtract_one_from_the_other() {
    println!("Subtract one from the other");
    let first_array: [u32; 5] = [1, 2, 3, 4, 5];
    let mut second_array: [u32; 5] = [2, 3, 4, 5, 6];

    println!("first array: {:?}", first_array);
    println!("second array: {:?}", second_array);

    for i in 0..5 {
        second_array[i] = second_array[i] - first_array[i];
    }

    println!("second array: {:?}", second_array);
}

fn main() {

    sum_two_arrays();
    subtract_one_from_the_other();

}