use std::thread;
use std::sync::Arc;

static NTHREADS: i32 = 10;

fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];
    // Arc is thread-safe reference counting pointer. It holds immutable data
    // and thread-safe reference counter of the immutable data.
    let shared_data: Arc<Vec<i32>> = Arc::new(vec![1, 2, 3]);

    for i in 0..NTHREADS {
        let shared_data_for_child: Arc<Vec<i32>> = shared_data.clone();

        // Spin up another thread.
        children.push(thread::spawn(move || {
            println!(
                "this is thread number {} with shared data: {:?}",
                i, &shared_data_for_child
            );
            i
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        println!("join result {:?}", child.join());
    }
}
