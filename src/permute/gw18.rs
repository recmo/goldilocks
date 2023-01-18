
pub fn transpose<T: Copy>(values: &mut [T], (rows, cols): (usize, usize)) {
    assert_eq!(values.len(), rows * cols);
    if rows >= cols {
        row_transpose(src, cols, rows);
    } else {
        column_transpose(src, cols, rows);
    }
}
