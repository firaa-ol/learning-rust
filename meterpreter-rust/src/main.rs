pub mod protocol;

fn main() {
    use protocol::tlv::MetaType;

    println!("{:?}", MetaType::String);
}
