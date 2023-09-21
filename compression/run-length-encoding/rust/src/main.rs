fn compress(input: &[u8]) -> Vec<u8> {
    let mut i = 0;
    let len = input.len();

    let mut counter = 1;
    let mut curr = input[0];

    let mut res = vec![];

    loop {
        if i == (len - 1) {
            // When reaching the end of the stream push to the buffer
            // the number of occurrences and the current values of the
            // latest identical elements.
            res.push(counter);
            res.push(curr);
            break;
        }

        let next = input[i + 1];
        if curr == next {
            curr = next;
            counter += 1;

            i += 1;
        } else {
            // When reaching the end of a series of identical elements
            // push to the buffer the number of occurrences and the current
            // value. Besides that, update the current value and reset the
            // counter.
            res.push(counter);
            res.push(curr);

            curr = next;
            counter = 1;

            i += 1;
        }
    }

    res
}

fn main() {
    let data = [0, 0, 0, 0, 1, 1, 1, 0, 0];

    println!("Before compression: {:?}", data);
    let compressed_data = compress(&data);
    println!("After compression: {:?}", compressed_data);
}
