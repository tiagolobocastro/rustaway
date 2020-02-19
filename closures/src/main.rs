use std::thread;
use std::time::Duration;

/*
fn slow_calculation(intensity: u32) -> u32 {
    println!("Slow calculation...");
    thread::sleep(Duration::from_secs(intensity.into()));
    intensity
}

fn do_some_work(intensity: u32) -> u32 {
    let calculation = |intensity: u32| {
        println!("Slow calculation...");
        thread::sleep(Duration::from_secs(intensity.into()));
        intensity
    };

    calculation(intensity)
}
*/

struct Cacher<T> 
{
    closure: T,
    value: Option<u32>,
}

impl<T> Cacher<T> 
{
    fn new(closure: T) -> Cacher<T> { 
        Cacher {
            closure,
            value: None,
        }
    }
}
impl<T> Cacher<T> 
    where T: Fn(u32) -> u32
{
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let value = (self.closure)(arg);
                self.value = Some(value);
                value
            }
        }
    }
}

struct Mt {
    p: u32
}

impl Copy for Mt {
}

impl Clone for Mt {
    fn clone(&self) -> Self {
        Mt { p: 7 }
    }
    fn clone_from(&mut self, source: &Self) {
        self.p = source.p + 99;
    }
}

fn mm(_m: Mt) {

}

fn te() -> impl FnMut(u32) -> bool {

    let mut x = Mt { p:1 };

    let mut clj = | y | {
        x.p += 2;
        y + x.p
    };
    
    let _ = clj(1);
    println!("{}", x.p);


    let a = Mt { p: 1 };

    mm(a);

    let mut b = a.clone();
    b.p += 1;
    println!("{}", b.p);
    println!("{}", a.p);

    let mut x = Mt {
        p: 2,
    };
    let eq_to_x = move | y | {
        x.p += 1;
        x.p == y
    };
    eq_to_x
}

fn env_closures() {

    let mut b = te();
    println!("b {}", b(1));
    println!("b {}", b(4));
    let a = Mt { p: 1 };
    let b = a;
    println!("b {}", b.p);

    let x = 4;
    let eq_to_x = | y | y == x;
    assert!( eq_to_x(x));

    let x = 4;
    let eq_to_x = | y | y == x;
    assert!( eq_to_x(x));
    
    let mut x = "test";
    let mut eq_to_x = | z | -> bool {
        x = "asd";
        z == x
    };

    assert!( eq_to_x("asd"));
    assert!( x == "asd");
    
    let /*mut*/ x = 4;
    
    // if a closure uses memory to store the value of x, why does it then borrow it??
    let eq_to_x = | y | y == x;
    
    println!("{:?}", x);
    //x = 4;

    //assert!( eq_to_x(x));
    assert!( eq_to_x(4));
   
    // we can modify it, since it's after the borrow completes 
    // x = 4;

    let x = 4;
    let eq_to_x = move | z | z == x;
    assert!( eq_to_x(x));
    
    let mut x = 4;
    let eq_to_x = move | z | z == x;
    println!("{}", x);
    x = 3;
    println!("{:?}", x);
    assert!( eq_to_x(4));


    let x = vec![1];
    let eq_to_x = | z | z == x;
    println!("{:?}", x);
    let y = vec![1];
    assert!( eq_to_x(y));
}

#[test]
fn test_closure_cache() {

    let closure_sum = | z | { z + 1 };

    let first = closure_sum(1);
    let second = closure_sum(2);

    assert_ne!(first, second, "Testing that the closure doesn't cache the values!");

    let mut cacher = Cacher::new(closure_sum);

    let first = cacher.value(1);
    let second = cacher.value(2);

    assert_eq!(first, second, "Testing that the cacher closure caches the values!");
}

fn single() {

    let same = |same| { same };

    //println!("{}", same("Hello!"));
    println!("{}", same(1));
    println!("{}", same(2));


    let mut clj = Cacher::new(|same| { same + 1});
    println!("{}", clj.value(3));
    println!("{}", clj.value(4));
}

fn main() {

    env_closures();
    single();



    let intensity: u64 = 2;
    
    let calculation = |mut i| {
        println!("Slow calculation...");
        thread::sleep(Duration::from_secs(0));
        i += 1;
        i
    };


    let _result = calculation(intensity);
    println!("Got {}", _result);
}

