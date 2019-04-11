fn main() {
    let (adjective, name) = two_words();
    let name = format!("{} {}", adjective, name);
    print_out(name);
}

fn two_words() -> (String, String) {
    (format!("fellow"), format!("Rustaceans"))
}

fn remove_vowels(name: &String) -> String {
    // Goal #1: What is needed here to make this compile?
    // Answer: mut
    let mut output = String::new();
    for c in name.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
            }
            _ => {
                output.push(c);
            }
        }
    }
    output
}

fn print_out(name: String) {
    // we are moving name to the remove_vowels function and then we're trying to use it
    // afterwards in println aka giving ownership away. To solve this we can use shared borrow
    let devowelized_name = remove_vowels(&name);
    println!("Removing vowels yields {:?}", devowelized_name);

    // easiest solution without borrowing would be
    // let devowelized_name = remove_vowels(name.clone());
    // but keep in mind that copying is a heavy operation
    println!("Removing vowels from {:?} yields {:?}",
             name, devowelized_name);


}
