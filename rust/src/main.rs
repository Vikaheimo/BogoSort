

fn main() {
    bogo_sort(8, 100);
}

fn bogo_sort(length: i32, print_iterations_every: i32) {
    let mut iteration: i32 = 0;
    let mut list: Vec<i32> = generate_random_list(length);
    print_iteration(&list, iteration);
    while !is_sorted(&list, length) {
        iteration += 1;
        shuffle(&mut list, length);
        if iteration % print_iterations_every == 0 {
            print_iteration(&list, iteration);
        }
    }
    println!("Result:");
    print_iteration(&list, iteration);
}

fn shuffle(list: &mut [i32], length: i32) {
    for n in 0..length {
        let r: i32 = (rand::random::<i32>() % length).abs();
        list.swap(n as usize, r as usize)
    }
}

fn is_sorted(list: &[i32], length: i32) -> bool {
    for n in 1..length {
        if list[(n - 1) as usize] > list[n as usize] {
            return false;
        }
    }
    true
}

fn generate_random_list(length: i32) -> Vec<i32> {
    (0..length).map(|_| rand::random()).collect()
}

fn print_iteration(list: &[i32], iteration: i32) {
    println!("{:?}, {:?}", iteration, list);
}