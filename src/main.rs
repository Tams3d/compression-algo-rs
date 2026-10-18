use compression_algo_rs::algorithms::rle::{compress, decompress};

fn main() {
    let text = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW";

    let compressed = compress(text);
    let restored = decompress(&compressed);

    println!("original:   {} bytes ({text})", &text.len());
    println!("compressed: {} bytes ({compressed})", compressed.len());
    println!("restored:   {} bytes ({restored})", restored.len());
}
