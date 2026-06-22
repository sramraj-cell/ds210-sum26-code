use std::sync::Arc;

fn random_vector(size: usize) -> Vec<u64> {
    let mut v: Vec<u64> = Vec::with_capacity(size);
    for i in 0..size {
        v.push(rand::random_range(0..10));
    }
    return v;
}

fn sum(v: &Vec<u64>, start: usize, end: usize) -> u64 {
    let mut sum = 0;
    for i in start..end {
        sum += v[i];
    }
    return sum;
}

fn one_thread(v: Vec<u64>) {
    // Start timer.
    let timer = std::time::Instant::now();

    // Sum using one thread.
    let f = move || {
        return sum(&v, 0, v.len());
    };
    let thread = std::thread::spawn(f);

    // Wait for the thread and get the sum.
    let sum = thread.join().unwrap();
    let time = timer.elapsed();
    println!("one_thread: Sum of vector is {}", sum);
    println!("one_thread: Took {:?}", time);
}

fn pass_using_clones(v: Vec<u64>) {
    // Start timer.
    let timer = std::time::Instant::now();

    // We need to make a copy of the vector and pass each copy to a different thread.
    let v2 = v.clone();
    let first_half = move || {
        return sum(&v, 0, v.len() / 2);
    };
    let second_half = move || {
        return sum(&v2, v2.len() / 2, v2.len());
    };

    // Sum each half in parallel.
    let thread1 = std::thread::spawn(first_half);
    let thread2 = std::thread::spawn(second_half);

    // Add the sums of each half
    let sum1 = thread1.join().unwrap();
    let sum2 = thread2.join().unwrap();
    let sum = sum1 + sum2;
    let time = timer.elapsed();

    println!("Clone (2 threads): Sum of vector is {}", sum);
    println!("Clone (2 threads): Took {:?}", time);
}

fn pass_using_arc(v: Vec<u64>) {
    // Start timer.
    let timer = std::time::Instant::now();

    // We do not copy the vector anymore, we instead copy the Arc (the underlying vector is shared).
    let v: Arc<Vec<u64>> = Arc::new(v);
    let v2: Arc<Vec<u64>> = Arc::clone(&v);

    // Arc is like a Python refcount! Notice how the two vectors have the same address.
    // I.e., both of v and v2 point to the same vector!
    //println!("Arc: Address of vector inside 1st arc {:p}", &*v);
    //println!("Arc: Address of vector inside 2nd arc {:p}", &*v2);

    // We can move the Arcs into the threads.
    // The vector does not actually move anywhere (it stays on the heap),
    // but the Arcs (i.e., pointers) move.
    let first_half = move || {
        return sum(&v, 0, v.len() / 2);
    };
    let second_half = move || {
        return sum(&v2, v2.len() / 2, v2.len());
    };

    // Sum each half in parallel.
    let thread1 = std::thread::spawn(first_half);
    let thread2 = std::thread::spawn(second_half);

    // Add the sums of each half
    let sum1 = thread1.join().unwrap();
    let sum2 = thread2.join().unwrap();
    let sum = sum1 + sum2;
    let time = timer.elapsed();

    println!("Arc (2 threads): Sum of vector is {}", sum);
    println!("Arc (2 threads): Took {:?}", time);
}


fn pass_using_many_threads(v: Vec<u64>, thread_count: usize) {
    // Start timer.
    let timer = std::time::Instant::now();

    // We do not copy the vector anymore, we instead copy the Arc (the underlying vector is shared).
    let v: Arc<Vec<u64>> = Arc::new(v);

    // We can move the Arcs into the threads.
    // The vector does not actually move anywhere (it stays on the heap),
    // but the Arcs (i.e., pointers) move.
    let mut threads = Vec::with_capacity(4);
    for i in 0..thread_count {
        let v2 = v.clone();
        let f = move || {
            let slice = v2.len() / thread_count;
            let start = i * slice;
            let end = (i + 1) * slice;
            return sum(&v2, start, end);
        };
        let thread = std::thread::spawn(f);
        threads.push(thread);
    }

    // Add the sums of part.
    let mut sum = 0;
    for thread in threads {
        sum += thread.join().unwrap();
    }
    let time = timer.elapsed();

    println!("Arc ({} threads): Sum of vector is {}", thread_count, sum);
    println!("Arc ({} threads): Took {:?}", thread_count, time);
}


fn main() {
    // Create a random vector.
    let v = random_vector(200_000_000);
    one_thread(v.clone());
    pass_using_clones(v.clone());
    pass_using_arc(v.clone());
    pass_using_many_threads(v.clone(), 4);
}
