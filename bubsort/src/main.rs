use rand::{thread_rng, Rng};

const ARRAY_LENGTH: usize = 32;

fn main() {
    let mut array = [0u8; ARRAY_LENGTH];
    thread_rng().fill(&mut array[..]);

    print!("Unsorted array: ");
    for element in &array {
        print!("{} ", element);
    }
    print!("\n");

    bubsort(&mut array, 0);

    print!("Sorted array: ");
    for element in &array {
        print!("{} ", element);
    }
}

fn bubsort(arr: &mut[u8; ARRAY_LENGTH], bottom_index: usize) {
    if bottom_index == ARRAY_LENGTH - 1 {
        return;
    }
    let mut lower_search_index = 0;
    while lower_search_index < ARRAY_LENGTH - 1 {
        if arr[lower_search_index] > arr[lower_search_index + 1] {
            let temporary_value = arr[lower_search_index];
            arr[lower_search_index] = arr[lower_search_index + 1];
            arr[lower_search_index + 1] = temporary_value;
        }
        lower_search_index += 1;
    }
    bubsort(arr, bottom_index + 1)
}
