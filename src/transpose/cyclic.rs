
/// Cycle following matrix transpose.
pub fn transpose_cyclic(matrix: &mut [u64], span: usize, width: usize, height: usize) {
    assert_eq!(matrix.len(), width * height * span);
    let stride = width * span;

    // Start of a row s from block i
    let row = |i| {
        debug_assert!(i < width * height);
        i * span..(i + 1) * span
    };

    // Permutation function for the transpose.
    let permute = |i| (i % height) * width + i / height;

    // Vector to keep track of which elements have been moved.
    let mut done = vec![false; width * height];

    // Buffer to hold elements while they are moved in cycles longer then 2.
    let mut buffer = vec![0; span];

    for i in 0..width * height {
        // If the element has already been moved, do nothing.
        if done[i] {
            continue;
        }

        // If the cycle length is 1, do nothing.
        let j = permute(i);
        if i == j {
            done[i] = true;
            continue;
        }

        // If the cycle length is 2, swap the elements.
        let k = permute(j);
        if i == k {
            let (a, b) = split_at_mut(matrix, row(i), row(j));
            a.swap_with_slice(b);
            done[i] = true;
            done[j] = true;
            continue;
        }

        // Follow the cycle using a buffer.
        let mut j = i;

        // Copy the first element of the cycle to the buffer.
        buffer.copy_from_slice(&matrix[row(i)]);

        // Move the rest of the cycle.
        loop {
            let k = permute(j);
            if k == i {
                break;
            }
            matrix.copy_within(row(k), row(j).start);
            done[j] = true;
            j = k;
        }

        // Copy the first element of the cycle from the buffer.
        matrix[row(j)].copy_from_slice(&buffer);
        done[j] = true;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_copy() {
        let mut matrix = [
            0, 1, 2, 
            3, 4, 5
        ];
        transpose_copy(&mut matrix, 3, 2);
        assert_eq!(matrix, [
            0, 3, 
            1, 4, 
            2, 5
        ]);
    }

    #[test]
    #[rustfmt::skip]
    fn test_cyclic_3x2() {
        let mut matrix = [
            0, 1, 2, 
            3, 4, 5
        ];
        transpose_cyclic(&mut matrix, 1, 3, 2);
        assert_eq!(matrix, [
            0, 3, 
            1, 4, 
            2, 5
        ]);
    }

    #[test]
    #[rustfmt::skip]
    fn test_cyclic_6x8() {
        let mut matrix = [
            00, 01,  10, 11,  20, 21,
            02, 03,  12, 13,  22, 23,
            30, 31,  40, 41,  50, 51,
            32, 33,  42, 43,  52, 53
        ];
        transpose_cyclic(&mut matrix, 2, 3, 4);
        assert_eq!(matrix, [
            00, 01,  30, 31,
            02, 03,  32, 33,
            10, 11,  40, 41, 
            12, 13,  42, 43, 
            20, 21,  50, 51,
            22, 23,  52, 53,
        ]);

        /*
        
        00, 01,   30, 31,   10, 11,
        02, 03,   32, 33,   12, 13,

        40, 41,   20, 21,   50, 51,
        42, 43,   22, 23,   52, 53
        
        */

        /*

        00, 01, 02, 03,
        30, 31, 32, 33,
        10, 11, 12, 13,
        40, 41, 42, 43,
        20, 21, 22, 23,
        50, 51, 52, 53

        */
    }
}
