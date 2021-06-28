use std::collections::HashMap;

fn main() {
    hash_map_collection();
    string_collection();
}

fn hash_map_collection() {
    let mut unit_map: HashMap<i32, String> = HashMap::new();

    unit_map.entry(1).or_insert(String::from("마린"));
    println!("unit_map:{:#?}", unit_map);

    let key_vec: Vec<i32> = vec![1, 2, 3];
    let value_vec = vec![
        String::from("히드라"),
        String::from("뮤탈"),
        String::from("질럿"),
    ];

    let unit_map2: HashMap<_, _> = key_vec.iter().zip(value_vec.iter()).collect();
    println! {"unit_map2:{:#?}", unit_map2};
}

fn string_collection() {
    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    let hello = format!("{}, {}", hello, "안녕");
    println!("{}", hello);

    let text = "नमस्ते";
    let mut array: Vec<u8> = Vec::new();

    for b in text.as_bytes() {
        array.push(*b);
    }

    let text2 = std::str::from_utf8(&array).unwrap();
    println!("{}", text2);
}
