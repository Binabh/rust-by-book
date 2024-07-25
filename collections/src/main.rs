use std::collections::HashMap;
fn main() {
    let (median, mode) = median_and_mode(&mut vec![1, 3, 7, 4, 5, 9, 77, 1, 8]);
    println!("Median is {median} and mode is {mode}");
    let sentence = String::from("Eating my first apple here and there");
    let pig_sentence: String = sentence
        .split(' ')
        .map(|word| word_to_pig_latin(word))
        .collect();
    println!("{}", pig_sentence);
}

fn median_and_mode(list: &mut Vec<i32>) -> (i32, i32) {
    list.sort();
    let mut median: i32 = 0;
    if let Some(&value) = list.get(list.len() / 2) {
        median = value
    };
    let mut count_map = HashMap::new();
    for n in list {
        let count = count_map.entry(n).or_insert(0);
        *count += 1;
    }
    let mut mode: i32 = 0;
    if let Some(val) = count_map.iter().max_by(|a, b| a.1.cmp(&b.1)) {
        mode = **val.0;
    };
    (median, mode)
}
fn word_to_pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut result = String::new();
    let first_char = &word.chars().next().unwrap();
    if !vowels.contains(first_char) {
        result.push_str(&word[1..]);
        result.push('-');
        result.push(*first_char);
        result.push_str("ay ");
    } else {
        result.push_str(word);
        result.push_str("-hay ");
    }
    result
}
