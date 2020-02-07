fn main() {
    
    let phrase = String::from("welcome to our rusty code!");

    let _ix = find_word( &phrase);

    // how to "build" a string until ix?
    // looks like we use a slice!
    
    let slice = &phrase[0.._ix];
    println!("The first word is: {}", slice);

    let slice = &phrase[..];
    println!("The whole phrase is: {}", slice);

    let _slice = find_word2( &phrase);
    let slice = find_word3( &phrase);
    println!("The first word is: {}", slice);
}

fn find_word(string: &String) -> usize {

    for (i, ch) in string.as_bytes().iter().enumerate() {
        let ch = *ch as char;
        if ch == ' ' {
            return i;
        }
    }
    
    string.len()
}

fn find_word2(string: &String) -> &str {

    for (i, &ch) in string.as_bytes().iter().enumerate() {
        if ch == b' ' {
            return &string[0..i];
        }
    }
    
    &string[..]
}

fn find_word3(string: &String) -> String {

    for (i, &ch) in string.as_bytes().iter().enumerate() {
        if ch == b' ' {
            return String::from(&string[0..i]);
        }
    }
    
    String::from(&string[..])
}
