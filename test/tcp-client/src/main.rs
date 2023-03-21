fn main() {
    let d = quick_protocol::QpInstruction::base();
    let ret = serde_json::to_string(&d);
    if ret.is_err() {
        panic!("json_to_string error.{}", ret.unwrap_err());
    }
    println!("{:?}", ret.unwrap());
}
