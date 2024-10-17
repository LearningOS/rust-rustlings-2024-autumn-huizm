/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]) {
	//TODO    
    let mut sink = |array: &mut [T], mut i: usize, heap_size: usize| {
        loop {
            let left_child = i * 2 + 1;
            let right_child = i * 2 + 2;
            
            let largest_child: usize;
            if i * 2 + 1 >= heap_size {
                break;
            } else if i * 2 + 2 >= heap_size || array[i * 2 + 1] >= array[i * 2 + 2] {
                largest_child = left_child;
            } else {
                largest_child = right_child;
            };
            
            if array[largest_child] <= array[i] {
                break;
            } else {
                array.swap(i, largest_child);
                i = largest_child;
            };
        };
    };
    
    // build max heap
    let array_len = array.len();
    for i in (0..array_len).rev() {
        sink(array, i, array_len);
    };

    // sort the array in place
    for i in (1..array_len).rev() {
        array.swap(0, i);
        sink(array, 0, i);
    };
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}