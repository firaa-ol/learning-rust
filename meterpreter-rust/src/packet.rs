use rand::Rng;
use std::collections::HashMap;

use crate::tlv::{Add, BinaryReader, BinaryWriter, Tlv, TlvType, TlvValue};

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

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u32)]
pub enum PacketType {
    Request = 0,
    Response = 1,
    PlainRequest = 10,
    PlainResponse = 11,
}

impl From<u32> for PacketType {
    fn from(val: u32) -> Self {
        let packet_type: PacketType = unsafe { ::std::mem::transmute(val) };
        packet_type
    }
}

#[derive(Debug)]
pub struct Packet {
    packet_type: PacketType,
    tlvs: HashMap<TlvType, Vec<Tlv>>,
}

impl Packet {
    pub const HEADER_SIZE: u32 = 4 + 16 + 4 + 4 + 4; // XOR Key + SESSION GUID + ENCRYPTION FLAG + Packet Body Length + Packet Type
    const ENC_LENGTH: u32 = 20;

    fn new(method: String) -> Packet {
        let mut instance = Self {
            packet_type: PacketType::Request,
            tlvs: HashMap::new(),
        };

        instance.set_method(method);
        instance.set_request_id(Uuid::new_v4().to_string().replace("-", ""));

        instance
    }

    pub fn from_raw(storage: &Vec<u8>, position: &mut usize) -> Self {
        let mut header = BinaryReader::read_bytes(storage, position, Packet::HEADER_SIZE);
        let mut xor_key = [0; 4];
        header
            .iter()
            .take(4)
            .enumerate()
            .for_each(|(index, elt)| xor_key[index] = *elt);

        Packet::xor(&mut header, xor_key);

        // Move to encryption flags
        let mut header_position = Packet::ENC_LENGTH as usize;
        let encryption_flag = BinaryReader::read_dword(&header, &mut header_position);
        let tlv_bytes_length = BinaryReader::read_dword(&header, &mut header_position) - 8; // tlv bytes length + packe type + packet length
        let packet_type = BinaryReader::read_packet_type(&header, &mut header_position);

        *position = Packet::HEADER_SIZE as usize;
        let mut packet_body = BinaryReader::read_bytes(storage, position, tlv_bytes_length);
        let encrypted = encryption_flag == 1; //TODO: turn this to an enum when implementing encryption

        Packet::xor(&mut packet_body, xor_key);

        if encrypted {
            panic!("Encryption is not implemented");
        }

        let mut packet = Packet {
            packet_type,
            tlvs: HashMap::new(),
        };
        let mut body_position = 0;
        while body_position < packet_body.len() {
            packet.add_tlv(Tlv::from_raw(&packet_body, &mut body_position))
        }

        packet
    }

    pub fn to_raw(&self, session_guid: &[u8]) -> Vec<u8> {
        let mut tlv_data: Vec<u8> = vec![];
        for tlv in self.tlvs.values().flatten() {
            tlv.to_raw(&mut tlv_data);
        }
        //TODO: encrypt tlv_data
        let mut packet_data: Vec<u8> = vec![];
        BinaryWriter::write_dword(&mut packet_data, 0); //XOR key, will be filled later

        BinaryWriter::write_bytes(&mut packet_data, session_guid);
        //TODO: replace encryption flag by an enum
        BinaryWriter::write_dword(&mut packet_data, 0); // Encryption flag - 0 -> None
        BinaryWriter::write_dword(&mut packet_data, tlv_data.len() as u32 + 8); // tlv Length + packetType + packetLength field
        BinaryWriter::write_packet_type(&mut packet_data, self.packet_type);
        BinaryWriter::write_bytes(&mut packet_data, &tlv_data);

        let xor_key = Packet::generate_xor_key();
        Packet::xor(&mut packet_data, xor_key);

        packet_data
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

    fn generate_xor_key() -> [u8; 4] {
        let random_bytes = rand::thread_rng().gen::<[u8; 4]>();
        return random_bytes;
    }

    fn xor(target: &mut Vec<u8>, xor_key: [u8; 4]) {
        for i in 0..target.len() {
            target[i] = target[i] ^ xor_key[i % xor_key.len()];
        }
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

    fn add_bytes(&mut self, tlv_type: TlvType, value: Vec<u8>) {
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
        tlv::{TlvType, TlvValue},
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
        packet.add_bytes(TlvType::ChannelData, vec![3, 5, 8, 9]);

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

    #[test]
    fn test_packet_to_raw() {
        let request_packet = Packet::new(String::from("core_channel_open"));
        let mut response_packet = request_packet.create_response();
        response_packet.add_bool(TlvType::StdapiProxyCfgAutodetect, true);
        response_packet.add_uint32(TlvType::ChannelId, 2);
        response_packet.add_uint64(TlvType::StdapiMountSpaceFree, 65535);
        response_packet.add_string(TlvType::ChannelType, "duplex".to_owned());

        let session_guid = [0; 16];
        let mut raw_data = response_packet.to_raw(&session_guid);

        println!("before decryption: {:?}", raw_data);
        //xor decrypt
        let xor_key = raw_data[0..4].try_into().unwrap();
        Packet::xor(&mut raw_data, xor_key);

        println!("after decryption: {:?}", raw_data);

        assert_eq!(raw_data[0..4], [0; 4]); // xor_key xored with itself
        assert_eq!(raw_data[4..20], [0; 16]); // session guid
        assert_eq!(raw_data[20..24], [0; 4]); // encryption flag
        assert_eq!(raw_data[28..32], [0, 0, 0, 1]) // packet type - response = 1
    }

    #[test]
    fn test_from_raw_to_packet() {
        let request_packet = Packet::new(String::from("core_channel_open"));
        let mut response_packet = request_packet.create_response();
        response_packet.add_bool(TlvType::StdapiProxyCfgAutodetect, true);
        response_packet.add_uint32(TlvType::ChannelId, 2);
        response_packet.add_uint64(TlvType::StdapiMountSpaceFree, 65535);
        response_packet.add_string(TlvType::ChannelType, "duplex".to_owned());

        let session_guid = [0; 16];
        let raw_data = response_packet.to_raw(&session_guid);

        let mut position = 0;
        let packet = Packet::from_raw(&raw_data, &mut position);

        assert_eq!(packet.packet_type, PacketType::Response);
        assert_eq!(
            packet
                .tlvs
                .get(&TlvType::StdapiProxyCfgAutodetect)
                .unwrap()
                .first()
                .unwrap()
                .value
                .as_ref()
                .unwrap(),
            &TlvValue::Bool(true)
        );

        assert_eq!(
            packet
                .tlvs
                .get(&TlvType::ChannelId)
                .unwrap()
                .first()
                .unwrap()
                .value
                .as_ref()
                .unwrap(),
            &TlvValue::UInt(2)
        );

        assert_eq!(
            packet
                .tlvs
                .get(&TlvType::StdapiMountSpaceFree)
                .unwrap()
                .first()
                .unwrap()
                .value
                .as_ref()
                .unwrap(),
            &TlvValue::ULongInt(65535)
        );

        assert_eq!(
            packet
                .tlvs
                .get(&TlvType::ChannelType)
                .unwrap()
                .first()
                .unwrap()
                .value
                .as_ref()
                .unwrap(),
            &TlvValue::String("duplex".to_string())
        );
    }
}
