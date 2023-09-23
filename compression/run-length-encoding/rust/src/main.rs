use run_length_encoding::compress;

fn main() {
    let data = [0, 0, 0, 0, 1, 1, 1, 0, 0];

    println!("Before compression: {:?}", data);
    let compressed_data = compress(&data);
    println!("After compression: {:?}", compressed_data);
}
