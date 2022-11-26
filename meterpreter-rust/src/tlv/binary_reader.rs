use crate::packet::PacketType;

use super::TlvType;

pub struct BinaryReader;

impl BinaryReader {
    pub fn read_bool(storage: &Vec<u8>, position: &mut usize) -> bool {
        let mut val = false;
        if storage[*position] == 1 {
            val = true;
        }

        *position += 1;
        val
    }

    pub fn read_dword(storage: &Vec<u8>, position: &mut usize) -> u32 {
        let bytes = storage[*position..*position + 4].try_into().unwrap();
        *position += 4;
        u32::from_be_bytes(bytes)
    }

    pub fn read_qword(storage: &Vec<u8>, position: &mut usize) -> u64 {
        let bytes = storage[*position..*position + 8].try_into().unwrap();
        *position += 8;
        u64::from_be_bytes(bytes)
    }

    pub fn read_string(storage: &Vec<u8>, position: &mut usize, length: u32) -> String {
        let data = std::str::from_utf8(&storage[*position..*position + (length as usize - 1)])
            .unwrap()
            .to_string();
        *position += length as usize;
        data
    }

    pub fn read_bytes(storage: &Vec<u8>, position: &mut usize, length: u32) -> Vec<u8> {
        let bytes = storage[*position..*position + length as usize].to_vec();
        *position += length as usize;
        bytes
    }

    pub fn read_tlv_type(storage: &Vec<u8>, position: &mut usize) -> TlvType {
        let tlv_type = BinaryReader::read_dword(storage, position);
        TlvType::from(tlv_type)
    }

    pub fn read_packet_type(storage: &Vec<u8>, position: &mut usize) -> PacketType {
        let packet_type = BinaryReader::read_dword(storage, position);
        PacketType::from(packet_type)
    }
}

#[cfg(test)]
mod test {
    use super::BinaryReader;

    #[test]
    fn test_read_bool() {
        let storage: Vec<u8> = vec![1];
        let mut position = 0;
        let data = BinaryReader::read_bool(&storage, &mut position);
        assert_eq!(data, true);
    }

    #[test]
    fn test_read_dword() {
        let storage: Vec<u8> = vec![0, 0, 1, 2];
        let mut position = 0;
        let data = BinaryReader::read_dword(&storage, &mut position);
        assert_eq!(data, 258);
    }

    #[test]
    fn test_read_qword() {
        let storage: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 255, 255];
        let mut position = 0;
        let data = BinaryReader::read_qword(&storage, &mut position);
        assert_eq!(data, 65535);
    }

    #[test]
    fn test_read_string() {
        let storage: Vec<u8> = vec![104, 101, 108, 108, 111, 0];
        let mut position = 0;
        let data = BinaryReader::read_string(&storage, &mut position, 6);
        assert_eq!(data, "hello");
    }

    #[test]
    fn test_read_bytes() {
        let storage: Vec<u8> = vec![8, 9, 6, 5];
        let mut position = 0;
        let data = BinaryReader::read_bytes(&storage, &mut position, 4);
        assert!(data == [8, 9, 6, 5]);
    }
}
