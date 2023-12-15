use num_cpus;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct ThreadResult {
    id: usize,
    counter: i32,
    elapsed_time: Duration,
}
fn main() {
    let num_cpus: usize = num_cpus::get();
    let mut handles: Vec<thread::JoinHandle<ThreadResult>> = vec![];

    for i in 0..num_cpus {
        let handle: thread::JoinHandle<ThreadResult> = thread::spawn(move || {
            let mut x: i32 = 0;
            let start: Instant = Instant::now();
            for _ in 0..100000000 {
                x += 1;
            }
            let duration: Duration = start.elapsed();
            ThreadResult {
                id: i,
                counter: x,
                elapsed_time: duration,
            }
        });
        handles.push(handle);
    }

    let mut results_cpu: Vec<ThreadResult> = vec![];
    let mut results_speed: Vec<ThreadResult> = vec![];

    for handle in handles {
        let result: ThreadResult = handle.join().unwrap();
        let result_clone = result.clone();
        results_cpu.push(result_clone);
        results_speed.push(result);
    }
    // Sort results by CPU number
    println!("Sorting by cpu #:");
    // Sort results by CPU number
    results_cpu.sort_by(|a: &ThreadResult, b: &ThreadResult| a.id.cmp(&b.id));
    for result in results_cpu {
        println!("CPU {}: Counter = {}, Time elapsed = {:.8} seconds", result.id, result.counter, result.elapsed_time.as_secs_f64());
    }

    // Sort results by shortest time elapsed
    println!("Sorting by speed:");
    results_speed.sort_by(|a: &ThreadResult, b: &ThreadResult| a.elapsed_time.cmp(&b.elapsed_time));
    for result_speed in results_speed {
        println!("CPU {}: Counter = {}, Time elapsed = {:.8} seconds", result_speed.id, result_speed.counter, result_speed.elapsed_time.as_secs_f64());
    }
}