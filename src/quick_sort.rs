pub fn sort(array: &mut [i32]) {
	let length = array.len()-1;
	_sort(array, 0, length);
}

fn _sort(array: &mut [i32], start: usize, end: usize) {

	if start < end {

		let p =  partition(array, start, end);
		_sort(array, start, p-1);
		_sort(array, p+1, end);
	}
}

fn partition(array: &mut [i32], start: usize, end: usize) -> usize {

	let pivot = array[end];
	let mut p = end;
	for i in (start..end).rev() {

		if array[i] >= pivot {
			p = p-1;
			swap(array, i, p);			
		}
	}

	swap(array, end, p);			

	p
}

fn swap(array: &mut [i32], source: usize, dest: usize) {
	let temp = array[dest];
	array[dest] = array[source];
	array[source] = temp;
}