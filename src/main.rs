use std::{char, collections::HashMap};

fn main() {
    let input = include_str!("../input.txt");
    let alphabet: &'static str = "abcdefghijklmnopqrstuvwxyz";
    let char_count = input.to_lowercase().chars().filter(|c| alphabet.find(*c).is_some()).fold(
        HashMap::new(),
        |mut acc: HashMap<char, i32>, item| {
            match acc.get_mut(&item) {
                Some(count) => { *count += 1; },
                None => { acc.insert(item, 1); }
            };
            acc
        }
    );
    let max = char_count.iter().fold((&char::REPLACEMENT_CHARACTER, &0), |acc, char_count_pair| {
        let (c, count) = acc;
        let (fc, fcount) = char_count_pair;
        if count < fcount { (fc, fcount) }
        else { (c, count) }
    });
    println!("{:?}", char_count);
    println!("Max '{}' with {}", max.0, max.1);
    let max_index = alphabet.find(*max.0).unwrap() + 1;
    let e_index = alphabet.find('e').unwrap() + 1;
    println!("Max index: {}", max_index);
    println!("E index:   {}", e_index);
    let offset = max_index + e_index;
    println!("Offset:    {}", offset);

    let clear_text: String = input.to_lowercase().chars().map(|c| {
        match alphabet.find(c) {
            None => c,
            Some(index) => {
                println!("Index {}, offset {} => {}", index, offset, (index + offset) % alphabet.len());
                let new_c: String = alphabet.chars().skip((index + offset) % alphabet.len()).take(1).collect();
                println!("Changing '{}' to '{}'", c, new_c);
                new_c.chars().last().unwrap()
            }
        }
    }).collect();
    println!("Klartext: {}", clear_text);
}
