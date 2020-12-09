pub fn array_fib(n: usize, array: &mut [Option<u128>]) -> u128 {
    array[n].unwrap_or_else(|| {
        let result = {
            match n {
                0 => 0,
                1 => 1,
                _ => array_fib(n - 1, array) + array_fib(n - 2, array),
            }
        };
        array[n] = Some(result);
        result
    })
}

// pub fn vec_fib(n: usize, vec: &mut [Option<u128>]) -> u128 {
//     vec[n].unwrap_or_else(|| {
//         let result = {
//             match n {
//                 0 => 0,
//                 1 => 1,
//                 _ => vec_fib(n - 1, vec) + vec_fib(n - 2, vec),
//             }
//         };
//         vec[n] = Some(result);
//         result
//     })
// }
