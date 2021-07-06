fn main() {
    selection_sort();
    insertion_sort();
}

/// 정렬되지않은 모든 원소를 돌며 정렬
/// O(N^2)
fn selection_sort() {
    println!("selection_sort===================================");
    let mut array = vec![1, 8, 9, 5, 7, 10, 19, 17];
    println!("before selection sort\n{:?}", array);

    for i in 0..array.len() {
        let mut min_index = i;
        for j in i + 1..array.len() {
            if array[min_index] > array[j] {
                min_index = j;
            }
        }

        array.swap(i, min_index);
    }

    println!("after selection sort\n{:?}", array);
    println!("==================================================");
}

fn insertion_sort() {
    println!("insertion_sort===================================");
    let mut array = vec![1, 8, 9, 5, 7, 10, 19, 17];
    println!("before insertion sort\n{:?}", array);

    for i in 0..array.len() {
        let temp = array[i];
        let mut j: i32 = i as i32 - 1;
        while j >= 0 {
            if array[j as usize] < temp {
                break;
            }
            array[(j + 1) as usize] = array[j as usize];
            j -= 1;
        }
        array[(j + 1) as usize] = temp;
    }
    println!("after insertion sort\n{:?}", array);
    println!("==================================================");
}
