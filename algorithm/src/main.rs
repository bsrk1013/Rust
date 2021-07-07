fn main() {
    selection_sort();
    insertion_sort();
    bubble_sort();
    quick_sort();
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

fn bubble_sort() {
    println!("bubble_sort===================================");
    let mut array = vec![20, 1, 8, 9, 5, 7, 10, 19, 17];
    println!("before bubble sort\n{:?}", array);

    let mut max_index = array.len() as i32 - 1;
    while max_index > 0 {
        let mut cur_pos = 0;
        for i in 1..max_index + 1 {
            let a = array[cur_pos];
            let b = array[i as usize];
            if a > b {
                array.swap(cur_pos, i as usize);
            }
            cur_pos += 1;
        }
        max_index -= 1;
    }
    println!("after bubble sort\n{:?}", array);
    println!("==================================================");
}

fn quick_sort() {
    println!("quick_sort===================================");
    let mut array: Vec<i32> = vec![22, 4, 9, 23, 3, 21, 34, 31];
    println!("before quick sort\n{:?}", array);

    let right = array.len() - 1;
    internal_sort(&mut array, 0, right);

    println!("after quick sort\n{:?}", array);
    println!("==================================================");

    fn internal_sort(vec: &mut Vec<i32>, left: usize, right: usize) {
        let mut low = left;
        let mut high = right;
        let pivot = vec[left];

        while low < high {
            while vec[high] >= pivot && low < high {
                high -= 1;
            }
            if low != high {
                vec[low] = vec[high];
            }

            while vec[low] <= pivot && low < high {
                low += 1;
            }
            if low != high {
                vec[high] = vec[low];
                high -= 1;
            }
        }

        vec[low] = pivot;
        let pivot_index = low;
        if left < pivot_index {
            internal_sort(vec, left, pivot_index - 1);
        }
        if right > pivot_index {
            internal_sort(vec, pivot_index + 1, right);
        }
    }
}
