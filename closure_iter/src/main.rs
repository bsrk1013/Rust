extern crate closure_iter;

use std::thread;
use std::time::Duration;

fn main() {
    workout_test();
    closure_test();
    iter_test();
    counter_test();
    modified_counter_test();
}

fn workout_test() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    fn generate_workout(intensity: u32, random_number: u32) {
        let expensive_closure = |num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_millis(500));
            num
        };
        let mut workout_cacher = Cacher::new(expensive_closure);
        let intensity = workout_cacher.value(intensity);
        if intensity < 25 {
            println!("Today, do {} pushups!", intensity);
            println!("Next, do {} situps!", intensity);
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {} minutes!", intensity);
            }
        }
    }
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }
    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => {
                    if v != arg {
                        self.value = Some(arg);
                    }
                    arg
                }
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
}

fn closure_test() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
    //println!("finaly x:{}", x);
}

fn iter_test() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4])
}

fn counter_test() {
    let counter = closure_iter::Counter::new();
    for num in counter {
        println!("{}", num);
    }
}

fn modified_counter_test() {
    let mut counter = closure_iter::ModifiedCounter::new();
    let iter = counter.iter();
    for num in iter {
        println!("iter:{}", num);
    }
    println!("counter.count:{}", counter.count);

    let array = vec![1, 2, 3, 4];
    for num in array {
        println!("array:{}", num);
    }
}
