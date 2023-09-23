pub fn compress(input: &[u8]) -> Vec<u8> {
    let len = input.len();

    let mut counter = 1;
    let mut curr = input[0];

    let mut res = vec![];

    for i in 0..len - 1 {
        let next = input[i + 1];
        if curr == next {
            counter += 1;
        } else {
            // When reaching the end of a series of identical elements
            // push to the buffer the number of occurrences and the current
            // value. Besides that, reset the counter.
            res.push(counter);
            res.push(curr);

            counter = 1;
        }
        curr = next;
    }

    // When reaching the end of the stream push to the buffer
    // the number of occurrences and the current values of the
    // latest identical elements.
    res.push(counter);
    res.push(curr);

    res
}

#[cfg(test)]
mod tests {
    use crate::compress;

    #[test]
    fn it_compresses_array_of_zeroes() {
        // Arrange
        let input = [0; 50];

        // Act
        let res = compress(&input);

        // Assert
        assert_eq!(res, vec![50, 0])
    }

    #[test]
    fn it_compresses_array_of_ones() {
        // Arrange
        let input = [1; 50];

        // Act
        let res = compress(&input);

        // Assert
        assert_eq!(res, vec![50, 1])
    }

    #[test]
    fn it_compresses_mixed_array() {
        // Arrange
        let input = [25, 70, 70, 100, 100, 100, 2];

        // Act
        let res = compress(&input);

        // Assert
        assert_eq!(res, vec![1, 25, 2, 70, 3, 100, 1, 2])
    }
}
