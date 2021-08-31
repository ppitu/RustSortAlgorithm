
fn partition<T: PartialOrd + Copy>(arr: &mut Vec<T>, start: usize, end: usize) -> usize {
    let pivot = arr[end];

    let mut index = start;

    let mut i = start;

    while i < end {
        if arr[i] < pivot {
            arr.swap(i, index);
            index += 1;
        }

        i+=1;
    }
    arr.swap(index, end);

    return index;
}

pub fn quick_sort<T: PartialOrd + Copy>(arr: &mut Vec<T>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    let pivot = partition(arr, start, end);

    if pivot as isize - 1 < 0 || pivot + 1 > arr.len(){
        return;
    }

    quick_sort(arr, start, pivot - 1);
    quick_sort(arr, pivot + 1, end);

}