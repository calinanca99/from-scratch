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

pub fn decompress(input: &[u8]) -> Vec<u8> {
    let len = input.len();

    let mut res = vec![];

    for i in (0..len).step_by(2) {
        let frequency = input[i];
        let element = input[i + 1];
        // TODO: Consider `push`ing to `res` without allocating
        // an extra `Vec`.
        let temp = vec![element; frequency.into()];
        res.extend(temp);
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::{compress, decompress};

    #[test]
    fn property_test() {
        let data = vec![25, 70, 70, 100, 100, 100, 2];
        assert_eq!(data, decompress(&compress(&data)));
    }
}
