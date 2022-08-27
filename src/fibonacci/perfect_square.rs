pub fn test_is_perfect_square(num: u32) -> bool {
    // iterative way to find square root instead of using sqrt method
    let start: f64 = num.into();
    let mut test_num: f64 = start - 1.0;
    let mut previous_test_num: f64 = 0.0;
    
    let calculated_square_root = loop {
        test_num = (test_num + start / test_num )/ 2.0;
        if test_num == previous_test_num {
            break test_num;
        }
        previous_test_num = test_num;
    };

    if start % calculated_square_root > 0.0 {
        return false;
    }

    true
}
