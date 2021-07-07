fn main() {
    selection_sort();
    insertion_sort();
    bubble_sort();
    quick_sort();
    sieve_of_eratosthenes();
    binary_search();
    dfs();
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

fn sieve_of_eratosthenes() {
    println!("sieve_of_eratosthenes===================================");
    let max = 10000;
    let mut is_prime = vec![true; max];
    is_prime[0] = false;
    is_prime[1] = false;

    let sqrtn: i32 = ((max as f64).sqrt()) as i32;
    for i in 2..sqrtn {
        match is_prime[i as usize] {
            true => {
                let mut num = i as usize;
                while true {
                    num += i as usize;
                    if num >= max {
                        break;
                    }
                    is_prime[num] = false;
                }
            }
            false => {}
        };
    }

    println!("2 is prime:{}", is_prime[2]);
    println!("3 is prime:{}", is_prime[3]);
    println!("5 is prime:{}", is_prime[5]);
    println!("7 is prime:{}", is_prime[7]);
    println!("8 is prime:{}", is_prime[8]);
    println!("9 is prime:{}", is_prime[9]);
    println!("10 is prime:{}", is_prime[10]);
    println!("11 is prime:{}", is_prime[11]);
    println!("12 is prime:{}", is_prime[12]);
    println!("13 is prime:{}", is_prime[13]);
    println!("17 is prime:{}", is_prime[17]);
    println!("19 is prime:{}", is_prime[19]);
    println!("23 is prime:{}", is_prime[23]);
    println!("29 is prime:{}", is_prime[29]);
    println!("==================================================");
}

fn binary_search() {
    let mut num_vec = vec![-3, 0, 1, 4, 7, 9, 11, 16];
    let k = 1;

    let right = num_vec.len();
    match internal_search(&mut num_vec, 0, right, k) {
        -1 => println!("not found key, k:{}", k),
        index => println!(
            "found key, index:{}, key:{}",
            index, num_vec[index as usize]
        ),
    };

    fn internal_search(vec: &mut Vec<i32>, left: usize, right: usize, key: i32) -> i32 {
        let pivot = (left + right) / 2;
        let pivot_value = vec[pivot];

        if pivot_value == key {
            pivot as i32
        } else if pivot_value > key {
            internal_search(vec, left, pivot - 1, key)
        } else {
            internal_search(vec, pivot + 1, right, key)
        }
    }
    println!("==================================================");
}

fn dfs() {
    println!("dfs===================================");
    let edge = 7;
    let line = 11;

    let info_vec = vec![
        (1, 2),
        (1, 3),
        (1, 7),
        (2, 3),
        (2, 4),
        (3, 5),
        (4, 3),
        (5, 7),
        (6, 5),
        (7, 6),
        (6, 4),
    ];

    let mut edges: Vec<Vec<i32>> = Vec::new();
    let mut visited = vec![false; edge + 1];

    edges.push(vec![]);
    for info in info_vec.iter() {
        let index = info.0;
        let dest = info.1;

        match edges.get_mut(index) {
            Some(value) => {
                value.push(dest);
            }
            None => {
                edges.insert(index, vec![dest]);
            }
        };
    }

    println!("{:?}", edges);

    internal_dfs(1, &edges, &mut visited);
    println!("");

    fn internal_dfs(cur_pos: i32, edges: &Vec<Vec<i32>>, visited: &mut Vec<bool>) {
        visited[cur_pos as usize] = true;
        print!("{} ", cur_pos);
        for i in 0..edges[cur_pos as usize].len() {
            let there = edges[cur_pos as usize][i];
            if visited[there as usize] {
                continue;
            }
            internal_dfs(there, edges, visited);
        }
    }

    println!("==================================================");
}
