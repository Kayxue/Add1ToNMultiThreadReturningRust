use std::thread;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let x: i128 = args[1].clone().parse().unwrap();
    let n: i128 = args[2].clone().parse().unwrap();
    let mut now = Instant::now();
    let mut handlers: Vec<thread::JoinHandle<i128>> = vec![];

    for i in 1..=x {
        handlers.push(thread::spawn(move || {
            println!("Thread {} started", i);
            let startFrom = (i - 1) * (n / x) + 1;
            let end = if i == x { n } else { i * (n / x) };
            let mut sum = 0i128;
            for k in startFrom..=end {
                sum += k;
            }
            return sum;
        }))
    }

    let mut sum = 0i128;
    for handler in handlers {
        sum += handler.join().unwrap();
    }
    let elapsed1= now.elapsed().as_secs_f64();
    println!("Sum from multi-thread: {}", sum);
    println!("Elapsed time: {:.8?}s", elapsed1);
    println!("-----------------------------------");
    now = Instant::now();
    sum = 0i128;
    for i in 1..=n {
        sum += i;
    }
    let elapsed2= now.elapsed().as_secs_f64();
    println!("Sum from single-thread: {}", sum);
    println!("Elapsed time: {:.8?}s", elapsed2);
    println!("-----------------------------------");
    println!("Speedup Factor: x{:.4}", elapsed2 / elapsed1);
}
