fn main() {
    bogo_sort(8, 100);
}

fn bogo_sort(length: i32, print_iterations_every: i32) {
    let mut iteration: i32 = 0;
    let mut list: Vec<i32> = generate_random_list(length);
    print_iteration(&list, iteration);
    while !is_sorted(&list) {
        iteration += 1;
        shuffle(&mut list);
        if iteration % print_iterations_every == 0 {
            print_iteration(&list, iteration);
        }
    }
    println!("Result:");
    print_iteration(&list, iteration);
}

fn shuffle(list: &mut [i32]) {
    for n in 0..list.len() {
        let r = rand::random::<usize>() % list.len();
        list.swap(n, r)
    }
}

fn is_sorted(list: &[i32]) -> bool {
    list.windows(2).all(|pair| pair[0] <= pair[1])
}

fn generate_random_list(length: i32) -> Vec<i32> {
    (0..length).map(|_| rand::random()).collect()
}

fn print_iteration(list: &[i32], iteration: i32) {
    println!("{:?}, {:?}", iteration, list);
}

#[cfg(test)]
mod test {
    use super::is_sorted;

    #[test]
    fn is_sorted_is_ok() {
        assert!(is_sorted(&[1]));
        assert!(is_sorted(&[1, 2, 2, 3]));
    }
}
