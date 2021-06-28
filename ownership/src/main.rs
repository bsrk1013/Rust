fn main() {
    let test = String::from("Takes and Gives Ownership");
    let test = takes_and_gives_ownership(test);
    println!("main:{}", test);

    let test2 = String::from("Reference Ownership");
    reference_ownership(&test);
    println!("main:{}", test2);

    let test3 = String::from("Clone Ownership");
    clone_ownership(test3.clone());
    println!("main:{}", test3);

    let mut test4 = String::from("Mutable String");
    {
        let test4_1 = &mut test4;
        *test4_1 = String::from("Change Mutable String");
        println!("main:{}", test4_1);

        let test4_2 = &mut test4;
        println!("main:{}", test4_2);
        let test4_3 = &mut test4;
        println!("main:{}", test4_3);
        let test4_4 = &test4;
        println!("main:{}", test4_4);
        let test4_5 = &test4;
        println!("main:{}", test4_5);
    }
    println!("main:{}", test4);

    let mut test5 = String::from("Slice World");
    let world = first_word(&test5);
    println!("test5.len:{}", test5.len());
    test5.clear();
    println!("test5.len:{}", test5.len());
    println!("word:{}", world);
    test5 = String::from("World Slice");
    let slice = &test5[0..6];
    println!("slice:{}", slice);
}

fn takes_and_gives_ownership(test: String) -> String {
    println!("takes_and_gives_ownership:{}", test);
    test
}

fn reference_ownership(test: &String) {
    println!("reference_ownership:{}", test);
}

fn clone_ownership(test: String) {
    println!("clone_ownership:{}", test);
}

fn first_word(test: &String) -> usize {
    let bytes = test.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    test.len()
}
