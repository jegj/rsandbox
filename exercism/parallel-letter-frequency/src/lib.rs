use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{thread, usize};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunk_size = input.len() / worker_count;
    let letter_counter: Arc<Mutex<HashMap<char, i32>>> = Arc::new(Mutex::new(HashMap::new()));
    //let letter_counter = Arc::new(Mutex::new(0));
    let mut threads = vec![];
    for chunk in input.chunks(chunk_size) {
        let letter_counter_ref = Arc::clone(&letter_counter);
        let c = chunk.clone();
        println!("?===>{:?}", chunk.len());
        let handle = thread::spawn(move || {
            let mut letter_counter_ref_mut = letter_counter_ref.lock().unwrap();
            println!("===>{:?}", c)
        });
        threads.push(handle);
        /*
        let chunk_ref = &chunk[..]; // Create a reference to the chunk
        let handle = thread::spawn(move || {
            // This closure will be executed in a separate thread
            process_chunk(chunk_ref);
        });
        threads.push(handle);
        */
    }

    for handle in threads {
        handle.join().unwrap();
    }

    HashMap::new()
}

pub fn count_letters(chunk: &[&str], letter_count_ref: &Arc<Mutex<HashMap<char, i32>>>) {}
