fn main() {
    numbers()
}

fn numbers() {

    println!("Hello, world!");


    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let tt = v[2];
    println!("{}", tt);

    match v.get(9) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("v: {}", i);
    }
    
    v.push(3);
    
    match v.get(5) {
        Some(fith) => println!("The fith element is {}", fith),
        None => println!("There is no fith element."),
    }
}
