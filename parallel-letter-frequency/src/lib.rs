use std::collections::HashMap;

use std::sync::{Arc, Mutex};

use std::thread::{self, JoinHandle};
/* 
test bench_large_parallel   ... bench:   1,616,815 ns/iter (+/- 213,542)
test bench_large_sequential ... bench:   1,075,787 ns/iter (+/- 273,016)
*/
fn helper(input: &str, map: &mut Arc<Mutex<HashMap<char, usize>>>) {
    let mut map = map.lock().unwrap();

    for c in input.chars() {
        if c.is_alphabetic() {
            let char_input_lowercase = c.to_lowercase().next().unwrap();
            map.entry(char_input_lowercase)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

}
/// locks after computation is done.
/// we would lock for a lesser duration as we would loop through less data during lock 
/// or equal to helper at worst case scenario
fn helper_v2(input: &String, gmap: &mut Arc<Mutex<HashMap<char, usize>>>) {
    let mut local_map = HashMap::new();
    for c in input.chars() {
        if c.is_alphabetic() {
            // let char_input_lowercase = c.to_lowercase().next().unwrap();
            local_map.entry(c.to_ascii_lowercase())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }
    //too bad extend doesn't sum values
   //gmap.lock().unwrap().extend(local_map.iter());
    for (k, v) in local_map.into_iter() {
        gmap.lock().unwrap().entry(k)
        .and_modify(|counter| *counter += v)
        .or_insert(v);
    }

}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    let worker_count = if input.len() >= worker_count {
        worker_count
    } else {
        input.len()
    };

    let map: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let size_chunk = input.len() / worker_count;
    let mut hts: Vec<JoinHandle<()>> = Vec::with_capacity(worker_count);
    let chunks = input.chunks(size_chunk);

    for chunk in chunks {
        let mut map_clone = Arc::clone(&map);
        let chunk_clone = chunk.concat();
        let ht = thread::spawn(move || {
            //helper(&chunk_clone, &mut map_clone);
            helper_v2(&chunk_clone, &mut map_clone);
        });
        hts.push(ht);
    }

    for ht in hts {
        ht.join().unwrap();
    }

    Arc::try_unwrap(map).unwrap().into_inner().unwrap()
}
