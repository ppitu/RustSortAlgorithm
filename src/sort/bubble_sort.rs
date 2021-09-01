pub fn bubble_sort<T: PartialOrd + Copy>(arr: &mut Vec<T>) {
    let mut size = arr.len();

    while size > 1 {
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
            }
        }
        size -= 1;
    }
}