#[derive(Debug, Clone, Copy)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let ie = &mut ImportantExcerpt { part: "Default" };
    let mut ss = "asdasd".to_string();
    get_part2(ie, &mut ss);
}

/*
fn get_part2(ie: &mut ImportantExcerpt) {
    let novel: String = "Call me Ishmael. Some years ago...".to_string();
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    ie.part = first_sentence.clone();
    println!("part: {}", ie.part);
}
*/
fn get_part2<'a>(ie: &'a mut ImportantExcerpt<'a>, ss: &'a mut String) {
    let novel: String = "Call me Ishmael. Some years ago...".to_string();
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    *ss = String::from(first_sentence);
    ie.part = ss;
    println!("part: {}", ie.part);
}
