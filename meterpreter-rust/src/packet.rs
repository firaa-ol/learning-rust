use std::collections::HashMap;

use crate::tlv::{Add, Tlv, TlvType, TlvValue};

use uuid::Uuid;

#[repr(u32)]
pub enum PacketResult {
    Success = 0,
    InvalidFunction = 1,
    InvalidData = 13,
    CallNotImplemented = 120,
    BadArguments = 160,
    ErrorAlreadyExists = 183,
}

#[derive(PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PacketType {
    Request = 0,
    Response = 1,
    PlainRequest = 10,
    PlainResponse = 11,
}

pub struct Packet {
    packet_type: PacketType,
    tlvs: HashMap<TlvType, Vec<Tlv>>,
}

impl Packet {
    pub const HEADER_SIZE: u32 = 4 + 16 + 4 + 4 + 4;
    const ENC_LENGTH: u32 = 20;
    const OFFSET_LENGTH: u32 = 24;

    fn new(method: String) -> Packet {
        let mut instance = Self {
            packet_type: PacketType::Request,
            tlvs: HashMap::new(),
        };

        instance.set_method(method);
        instance.set_request_id(Uuid::new_v4().to_string().replace("-", ""));

        instance
    }

    fn from_raw(&self) {
        unimplemented!()
    }

    fn to_raw(&self) {
        unimplemented!()
    }

    pub fn get_request_id(&self) -> String {
        let tlv = self.get_tlv(TlvType::RequestId);
        tlv.value_as_string()
    }

    fn set_request_id(&mut self, request_id: String) {
        self.tlvs.remove(&TlvType::RequestId);
        self.add_string(TlvType::RequestId, request_id);
    }

    pub fn get_method(&self) -> String {
        let tlv = self.get_tlv(TlvType::Method);
        tlv.value_as_string()
    }

    fn set_method(&mut self, method: String) {
        self.tlvs.remove(&TlvType::Method);
        self.add_string(TlvType::Method, method);
    }

    pub fn get_result(&self) -> PacketResult {
        let tlv = self.get_tlv(TlvType::Result);
        let num_val = tlv.value_as_uint32();
        let packet_result: PacketResult = unsafe { ::std::mem::transmute(num_val) };
        packet_result
    }

    pub fn set_result(&mut self, packet_result: PacketResult) {
        self.tlvs.remove(&TlvType::Result);
        self.add_uint32(TlvType::Result, packet_result as u32);
    }

    fn get_tlv(&self, tlv_type: TlvType) -> &Tlv {
        let tlv = self.tlvs.get(&tlv_type).unwrap().first().unwrap();
        tlv
    }

    pub fn create_response(&self) -> Packet {
        let packet_type = if self.packet_type == PacketType::Request {
            PacketType::Response
        } else {
            PacketType::PlainResponse
        };
        let mut response = Self {
            packet_type: packet_type,
            tlvs: HashMap::new(),
        };

        response.set_request_id(self.get_request_id());
        response.set_method(self.get_method());

        response
    }
}

impl Add for Packet {
    fn add_string(&mut self, tlv_type: TlvType, value: String) {
        let tlv = Tlv::new(tlv_type, TlvValue::String(value));
        self.add_tlv(tlv);
    }

    fn add_uint32(&mut self, tlv_type: TlvType, value: u32) {
        let tlv = Tlv::new(tlv_type, TlvValue::UInt(value));
        self.add_tlv(tlv);
    }

    fn add_uint64(&mut self, tlv_type: TlvType, value: u64) {
        let tlv = Tlv::new(tlv_type, TlvValue::ULongInt(value));
        self.add_tlv(tlv);
    }

    fn add_bool(&mut self, tlv_type: TlvType, value: bool) {
        let tlv = Tlv::new(tlv_type, TlvValue::Bool(value));
        self.add_tlv(tlv);
    }

    fn add_bytes(&mut self, tlv_type: TlvType, value: Box<[u8]>) {
        let tlv = Tlv::new(tlv_type, TlvValue::Bytes(value));
        self.add_tlv(tlv);
    }

    fn add_group(&mut self, tlv_type: TlvType) {
        let tlv = Tlv {
            tlv_type,
            value: None,
            tlvs: HashMap::new(),
        };
        self.add_tlv(tlv);
    }

    fn add_tlv(&mut self, tlv: Tlv) {
        if let Some(tlv_list) = self.tlvs.get_mut(&tlv.tlv_type) {
            tlv_list.push(tlv);
        } else {
            self.tlvs.insert(tlv.tlv_type.clone(), vec![tlv]);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        packet::{Packet, PacketType},
        tlv::TlvType,
    };

    use super::Add;

    #[test]
    fn test_empty_packet() {
        let packet = Packet::new(String::from("core_channel_close"));
        assert!(packet.tlvs.contains_key(&TlvType::Method));
        assert!(packet.tlvs.contains_key(&TlvType::RequestId));
        assert_eq!(packet.get_method(), "core_channel_close");
    }

    #[test]
    fn test_add() {
        let mut packet = Packet::new(String::from("core_channel_open"));
        packet.add_string(TlvType::ChannelType, String::from("unidirectional"));
        packet.add_uint32(TlvType::ChannelId, 2);
        packet.add_bytes(TlvType::ChannelData, Box::new([3, 5, 8, 9]));

        assert_eq!(
            packet
                .tlvs
                .get(&TlvType::ChannelType)
                .unwrap()
                .first()
                .unwrap()
                .value_as_string(),
            "unidirectional"
        );
        assert_eq!(
            packet
                .tlvs
                .get(&TlvType::ChannelId)
                .unwrap()
                .first()
                .unwrap()
                .value_as_uint32(),
            2
        );
        assert_eq!(
            packet
                .tlvs
                .get(&TlvType::ChannelData)
                .unwrap()
                .first()
                .unwrap()
                .value_as_bytes()
                .as_ref(),
            [3, 5, 8, 9]
        );
    }

    #[test]
    fn test_create_response() {
        let request_packet = Packet::new(String::from("core_channel_open"));
        let response_packet = request_packet.create_response();

        assert_eq!(
            response_packet.get_request_id(),
            request_packet.get_request_id()
        );
        assert_eq!(response_packet.get_method(), request_packet.get_method());
        assert_eq!(response_packet.packet_type, PacketType::Response);
    }
}
