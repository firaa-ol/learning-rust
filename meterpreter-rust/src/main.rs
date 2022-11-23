pub mod packet;
pub mod tlv;

fn main() {
    use tlv::MetaType;

    println!("{:?}", MetaType::String);
}
