use std::thread;
use std::time::Duration;

fn main() {
    workout_test();
    cacher_test();
}

fn workout_test() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn cacher_test() {
    let plus_calc_closure = |num: u32| num + 5;
    let mut plus_cacher = Cacher::new(plus_calc_closure);

    plus_cacher.value(10);
    println! {"calc:{}", plus_cacher.value.unwrap()};
}

fn closure_test() {
    let mut x = 10;
    let fn_closure = || println!("fn x:{}", x);
    let fnmut_closure = || {
        x = 1;
        println!("fnmut x:{}", x);
    };
    let fnonce_closure = move || println!("fnonce x:{}", x);

    fn_closure();
    // fnmut_closure();

    println!("finaly x:{}", x);
}

fn simulation_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
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
