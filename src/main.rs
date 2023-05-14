use roaring::RoaringBitmap;

fn main() {
    let mut rb = RoaringBitmap::new();
    rb.insert_range(0..1_000_000);
    let writer = std::fs::File::create("data/test_2.bin").unwrap();
    rb.serialize_into(writer).unwrap();
}
