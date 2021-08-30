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

pub fn quick_sort(arr: &mut Vec<usize>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    let pivot = partition(arr, start, end);

    quick_sort(arr, start, (pivot - 1) as usize);
    quick_sort(arr, (pivot + 1) as usize, end);

}