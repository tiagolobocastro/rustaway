use std::thread;
use std::time::Duration;

/*
fn slow_calculation(intensity: u32) -> u32 {
    println!("Slow calculation...");
    thread::sleep(Duration::from_secs(intensity.into()));
    intensity
}
*/

fn do_some_work(intensity: u32) -> u32 {
    let calculation = |intensity: u32| {
        println!("Slow calculation...");
        thread::sleep(Duration::from_secs(intensity.into()));
        intensity
    };

    calculation(intensity)
}

fn main() {
    let intensity: u64 = 2;
    //let _result = slow_calculation(intensity);
    
    let calculation = |mut i| {
        println!("Slow calculation...");
        i+=1;
        thread::sleep(Duration::from_secs(i));
        i 
    };


    let _result = calculation(intensity);
    //let _result = do_some_work(intensity);
    println!("Got {}", _result);
    let intensity: u32 = 1;
    let _result = calculation(intensity.into());
    println!("Got {}", _result);
}

