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

    index
}

pub fn quick_sort<T: PartialOrd + Copy>(arr: &mut Vec<T>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    let pivot = partition(arr, start, end);

    if pivot > 0 {
        quick_sort(arr, start, pivot - 1);
    }

    if pivot + 1 < arr.len() {
        quick_sort(arr, pivot + 1, end);
    }

}