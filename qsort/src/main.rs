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

    qsort(&mut array, 0, ARRAY_LENGTH - 1);

    print!("Sorted array: ");
    for element in &array {
        print!("{} ", element);
    }
}

fn qsort(arr: &mut [u8; ARRAY_LENGTH], bottom_index: usize, top_index: usize) {
    if bottom_index >= top_index {
        return
    }
    // とりあえず現在の範囲の左端をピボットにする
    let pivot = arr[bottom_index];

    // bottom/topで指定したindexから、それぞれ右側/左側にindexをずらしつつピボットの値と比較していく
    let mut lower_search_index = bottom_index;
    let mut upper_search_index = top_index;
    while lower_search_index < upper_search_index {
        // 左側の要素がピボットより大きい場合そこで止める
        while lower_search_index <= upper_search_index && arr[lower_search_index] <= pivot {
            lower_search_index += 1;
        }
        // 右側の要素がピボットと同じか小さい場合はそこで止める
        while lower_search_index <= upper_search_index && arr[upper_search_index] > pivot {
            upper_search_index -= 1;
        }
        // 止めた箇所の要素を置換（indexが同じなら意味がないのでなにもしない）
        if lower_search_index < upper_search_index {
            let temporary_value = arr[lower_search_index];
            arr[lower_search_index] = arr[upper_search_index];
            arr[upper_search_index] = temporary_value;
        }
    }

    // 左端（ピボットの値）を真ん中に持ってくる
    let temporary_value = arr[bottom_index];
    arr[bottom_index] = arr[upper_search_index];
    arr[upper_search_index] = temporary_value;

    // 左側を分割（arrayの先頭からピボットの左隣まで）
    if upper_search_index > 0 {
        qsort(arr, bottom_index, upper_search_index - 1);
    }
    // 右側を分割（ピボットの右隣からarrayの最後尾まで）
    if upper_search_index + 1 < ARRAY_LENGTH {
        qsort(arr, upper_search_index + 1, top_index);
    }
}
