pub fn bubblesort_once(lst: &[u32]) -> Vec<u32> {
    let len = lst.len();
    let mut _lst = lst.to_vec();
    for indx in 0..len - 1 {
        // if indx + 1 == len {
        //     break;
        // }

        println!("{:} and {:}", _lst[indx], _lst[indx + 1]);
        if _lst[indx] > _lst[indx + 1] {
            _lst.swap(indx, indx + 1);
            println!("{_lst:?}");
        }
    }
    _lst
}


#[cfg(test)]
mod tests {
    use super::bubblesort_once;
        
    fn dotest(a: &[u32], expected: &[u32]) {
        let actual = bubblesort_once(a);
        assert!(actual == expected, 
            "With a = {a:?}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn example_test() {
        dotest(&[9, 7, 5, 3, 1, 2, 4, 6, 8], &[7, 5, 3, 1, 2, 4, 6, 8, 9]);
    }
}
