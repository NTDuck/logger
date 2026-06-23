use struson::reader::{JsonStreamReader, JsonReader};
fn main() {
    let mut reader = JsonStreamReader::new("{}".as_bytes());
    // Try to see what next_event returns
    let _e = reader.peek();
}
