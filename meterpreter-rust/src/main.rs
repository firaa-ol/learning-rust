pub mod tlv;
pub mod packet;

fn main() {
    use tlv::MetaType;

    println!("{:?}", MetaType::String);
}
