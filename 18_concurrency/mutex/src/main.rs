use std::{thread, time};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
struct SharedData {
    count: usize,  // Number of updates for this struct.
    ids: Vec<i32>, // Thread stores its id on updating this struct.
}

impl SharedData {
    fn new() -> Self {
        Self {
            count: 0,
            ids: vec![],
        }
    }
}

fn main() {
    // There is a number of threads increasing a counter and storing their id
    // when the update occurs.
    const NTHREADS: i32 = 8;
    const MAX_COUNT: usize = 100;
    const SLEEP_MILLIS: u64 = 10;

    let mut threads = vec![];
    let mutex = Arc::new(Mutex::new(SharedData::new()));

    for i in 0..NTHREADS {
        let thread_id = i;
        let thread_mutex = mutex.clone();
        let sleep_dur = time::Duration::from_millis(SLEEP_MILLIS);

        threads.push(thread::spawn(move || loop {
            {
                let mut thread_data = thread_mutex.lock().unwrap();
                if thread_data.count < MAX_COUNT {
                    thread_data.count += 1;
                    thread_data.ids.push(thread_id);
                } else {
                    break;
                }
                // Mutex released at the end of block.
            }
            // Let's give a chance to other threads to lock the mutex.
            thread::sleep(sleep_dur);
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }

    println!("resutl = {:?}", *mutex.lock().unwrap());
}
