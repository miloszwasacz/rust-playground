fn main() {
    let strings = ["first", "apple"];
    println!("{:?}", strings);

    let mut res = vec![];
    for str in strings {
        if is_vowel(str.chars().next().unwrap()) {
            res.push(format!("{}-hay", str));
        } else {
            let new_str = String::from(&str[1..]);
            let first = &str.chars().next().unwrap();
            res.push(format!("{}-{}ay", new_str, first));
        }
    }
    println!("{:?}", res);
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}
