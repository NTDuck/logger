use struson::reader::{JsonReader, JsonStreamReader};

pub fn parse(bytes: &[u8]) {
    let mut reader = JsonStreamReader::new(bytes);
    let vt = reader.peek().unwrap();
    if let struson::reader::ValueType::String = vt {
        let _s = reader.next_string().unwrap();
    } else if let struson::reader::ValueType::Number = vt {
        let _n = reader.next_number_as_string().unwrap();
    } else if let struson::reader::ValueType::Boolean = vt {
        let _b = reader.next_bool().unwrap();
    } else if let struson::reader::ValueType::Null = vt {
        reader.next_null().unwrap();
    }
}
fn main() {}
