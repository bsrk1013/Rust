use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::io;
use std::str::FromStr;

fn main() {
    // test();
    network();
    // generator_test();
    // a_number_test();
}

fn test() {
    let mut args = env::args();

    let str_num = match args.next() {
        Some(value) => value,
        None => {
            panic!("failed")
        }
    };
    // println!("{}", str_num);

    let split_vec: Vec<_> = str_num.split(" ").collect();
    let split_vec: Vec<i32> = split_vec
        .into_iter()
        .map(|x| FromStr::from_str(x).unwrap())
        .collect();

    let a = split_vec[0];
    let b = split_vec[1];

    if a > b {
        println!(">");
    } else if a < b {
        println!("<");
    } else {
        println!("==")
    }
}

fn hash_test_1() {
    let participant = vec!["mislav", "stanko", "mislav", "ana"];
    let completion = vec!["stanko", "ana", "mislav"];

    let mut tables = HashMap::<&str, i32>::new();

    for member in participant {
        match tables.get_mut(member) {
            Some(value) => *value += 1,
            None => {
                tables.insert(member, 1);
            }
        };
    }

    for member in completion {
        match tables.get_mut(member) {
            Some(value) => {
                *value -= 1;
                match value {
                    0 => {
                        tables.remove(member);
                    }
                    _ => {}
                };
            }
            _ => {}
        };
    }

    let result: Vec<_> = tables.iter().collect();
    let result = result[0];
    println!("result:{}", result.0);
}

fn a_number_test() {
    let max_pow_count = 10;
    let mut ten_pow_vec = Vec::<u64>::new();

    let mut ten_pow = 1;
    for i in 1..max_pow_count + 1 {
        ten_pow_vec.push(ten_pow);
        ten_pow *= 10;
    }

    let mut n = String::new();
    println!("Enter th number");
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let number = n.trim().parse::<u64>().unwrap();

    let mut array = Vec::<u64>::new();
    for i in 1..number + 1 {
        if fjudge(i, &ten_pow_vec) == true {
            array.push(i);
        }
    }

    let mut largest_number = 0;
    for num in array {
        if num > largest_number {
            largest_number = num;
        }
    }

    println!("largest_number:{}", largest_number);

    fn fjudge(num: u64, ten_pow_vec: &Vec<u64>) -> bool {
        let multiply = num * (num - 1);
        let length = get_length(num);

        let ten_pow: u64 = ten_pow_vec[length];
        if multiply % ten_pow == 0 {
            return true;
        }
        false
    }

    fn get_length(num: u64) -> usize {
        let mut num = num;
        let mut length = 0;
        while num > 0 {
            num /= 10;
            length += 1;
        }
        return length;
    }

    // let mut largest_number = 0;
    // for i in 1..(number + 1) {
    //     let squared = i * i;
    //     let str_squared = squared.to_string();

    //     let str_num = i.to_string();
    //     if str_squared.len() >= str_num.len() {
    //         let max = str_squared.len();
    //         let last_num = &str_squared[max - str_num.len()..max]
    //             .parse::<u64>()
    //             .unwrap();
    //         // println!("last_num:{}", last_num);

    //         if last_num == &i {
    //             largest_number = i;
    //         }
    //     }
    // }

    // println!("largest_number:{}", largest_number);
}

fn generator_test() {
    let mut generator = HashSet::<u32>::new();

    for i in 1..5001 {
        let str_num = i.to_string();

        let mut chiper: u32 = 0;
        for c in str_num.chars() {
            chiper += (c.to_string()).parse::<u32>().unwrap();
        }
        chiper += i;
        generator.insert(chiper);
    }

    let mut result: u32 = 0;
    for i in 1..5001 {
        if generator.contains(&i) == false {
            result += i;
        }
    }

    println!("result:{}", result);
}

fn network() {
    let computers = vec![vec![1, 1, 0], vec![1, 1, 1], vec![0, 1, 1]];
    let n = computers.len();
    let mut visited = vec![false; n];

    let mut result = 0;
    for i in 0..n {
        if visited[i] == true {
            continue;
        }
        dfs(&computers, &mut visited, i);
        result += 1;
    }

    println!("network:{}", result);

    fn dfs(computers: &Vec<Vec<i32>>, visited: &mut Vec<bool>, index: usize) {
        visited[index] = true;
        let size = computers.len();

        for i in 0..size {
            if computers[index][i] == 1 && visited[i] == false {
                dfs(computers, visited, i);
            }
        }
    }
}
