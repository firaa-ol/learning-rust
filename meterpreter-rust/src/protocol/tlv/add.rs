use crate::protocol::tlv::{Tlv, TlvType};
pub trait Add {
    fn add_string(&mut self, tlv_type: TlvType, value: String);
    fn add_uint32(&mut self, tlv_type: TlvType, value: u32);
    fn add_uint64(&mut self, tlv_type: TlvType, value: u64);
    fn add_bool(&mut self, tlv_type: TlvType, value: bool);
    fn add_bytes(&mut self, tlv_type: TlvType, value: Vec<u8>);
    fn add_group(&mut self, tlv_type: TlvType);
    fn add_tlv(&mut self, tlv: Tlv);
}
