use rand::{distributions::Uniform, Rng};
use std::time::{Instant};

fn swap(arr: &mut Vec<usize>, first_pos: usize, second_pos: usize) {
    let temp = arr[first_pos];
    arr[first_pos] = arr[second_pos];
    arr[second_pos] = temp;
}

fn partition(arr: &mut Vec<usize>, start: usize, end: usize) -> i32 {
    let pivot = arr[end];

    let mut index = start;

    let mut i = start;

    while i < end {
        if arr[i] < pivot {
            swap(arr, i, index);
            index += 1;
        }

        i+=1;
    }

    swap(arr, index, end);

    return index as i32;
}

fn quick_sort(arr: &mut Vec<usize>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    let pivot = partition(arr, start, end);

    quick_sort(arr, start, (pivot - 1) as usize);
    quick_sort(arr, (pivot + 1) as usize, end);

}

fn bubble_sort(arr: &mut Vec<u32>) {
    for _i in 0..arr.len() {
        for j in 0..arr.len() - 1 {
            if arr[j] > arr[j + 1] {
                //swap(arr, j, j+ 1)
            }
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 1000000);
    
    let mut vec: Vec<usize> = (0..200).map(|_| rng.sample(&range)).collect();

    println!("{:?}", vec);

    let now = Instant::now();
    //bubble_sort(&mut vec);
    
    let size  = vec.len() - 1;

    println!("{}", size);

    quick_sort(&mut vec, 0, size);

    let end = now.elapsed().as_secs();

    println!("{:?}", vec);

    println!("Czas:  {}", end);
}
