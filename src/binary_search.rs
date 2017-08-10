pub fn binary_search(array: &[i32], target: i32) -> Option<usize> {

    let mut start = 0;
    let mut end = array.len() - 1;

    while start <= end {
        let mid = start + (end-start)/2;

        if array[mid] == target {
            return Some(mid);
        }
        else if array[mid] < target {
            start = mid + 1;
        }
        else {
            end = mid - 1;
        }
    }

    None
}