use std::ops::Deref;

fn main() {
    let b = Box::new(5);
    println!("b={}", b);

    test();
    test_1();
    test_2();
    test_3();
}

fn test() {
    let a = 5;
    let b = 2;
    let c = a ^ b;
    println!("result:{}", c);
}

fn test_1() {
    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    assert_eq!(*y, 5);
}

fn test_2() {
    let x = 5;
    let mut y = Box::new(x);
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    *y = 10;

    assert_eq!(x, 10);
    assert_eq!(*y, 10);
}

fn test_3() {
    struct CustomBox<T>(T);
    impl<T> CustomBox<T> {
        fn new(x: T) -> CustomBox<T> {
            CustomBox(x)
        }
    }

    impl<T> Deref for CustomBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    let x = 5;
    let y = CustomBox::new(x);
    assert_eq!(x, 5);
    assert_eq!(*y, 5);
}
