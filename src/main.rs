mod sort;

use rand::{distributions::Uniform, Rng};
use std::time::{Instant};

fn main() {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 100);
    
    let mut vec: Vec<usize> = (0..200).map(|_| rng.sample(&range)).collect();

    println!("{:?}", vec);

    let now = Instant::now();
    //bubble_sort(&mut vec);
    
    let size  = vec.len() - 1;

    println!("{}", size);

    //sort::quick_sort::quick_sort(&mut vec, 0, size);
    //sort::bubble_sort::bubble_sort(&mut vec);
    sort::insertion_sort::insertion_sort(&mut vec);

    let end = now.elapsed().as_secs();

    println!("{:?}", vec);

    println!("Czas:  {}", end);
}
