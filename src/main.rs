use roaring::RoaringBitmap;

fn main() {
    let mut rb = RoaringBitmap::new();
    rb.insert(1);
    let writer = std::fs::File::create("test.bin").unwrap();
    rb.serialize_into(writer).unwrap();
}
