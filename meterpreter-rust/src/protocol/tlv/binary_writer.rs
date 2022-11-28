use crate::protocol::packet::PacketType;

use super::TlvType;

pub struct BinaryWriter;

impl BinaryWriter {
    pub fn write_bool(storage: &mut Vec<u8>, data: bool) {
        storage.push(data as u8);
    }

    pub fn write_dword(storage: &mut Vec<u8>, data: u32) {
        storage.extend(data.to_be_bytes());
    }

    pub fn write_qword(storage: &mut Vec<u8>, data: u64) {
        storage.extend(data.to_be_bytes());
    }

    pub fn write_string(storage: &mut Vec<u8>, data: String) {
        storage.extend(data.as_bytes());
        storage.push(0); // terminating null charachter
    }

    pub fn write_bytes(storage: &mut Vec<u8>, data: &[u8]) {
        storage.extend(data);
    }

    pub fn write_packet_type(storage: &mut Vec<u8>, packet_type: PacketType) {
        BinaryWriter::write_dword(storage, packet_type as u32);
    }

    pub fn write_tlv_type(storage: &mut Vec<u8>, tlv_type: TlvType) {
        BinaryWriter::write_dword(storage, tlv_type as u32);
    }
}

#[cfg(test)]
mod test {
    use super::BinaryWriter;

    #[test]
    fn test_write_bool() {
        let mut storage: Vec<u8> = vec![];
        BinaryWriter::write_bool(&mut storage, true);
        assert_eq!(storage.len(), 1);
        assert_eq!(storage[0], 1);
    }

    #[test]
    fn test_write_dword() {
        let mut storage: Vec<u8> = vec![];
        BinaryWriter::write_dword(&mut storage, 258);
        assert_eq!(storage.len(), 4);
        assert!(storage == [0, 0, 1, 2]);
    }

    #[test]
    fn test_write_qword() {
        let mut storage: Vec<u8> = vec![];
        BinaryWriter::write_qword(&mut storage, 65535);
        assert_eq!(storage.len(), 8);
        assert!(storage == [0, 0, 0, 0, 0, 0, 255, 255]);
    }

    #[test]
    fn test_write_string() {
        let mut storage: Vec<u8> = vec![];
        BinaryWriter::write_string(&mut storage, String::from("hello"));
        assert_eq!(storage.len(), 6);
        assert!(storage == [104, 101, 108, 108, 111, 0]);
    }

    #[test]
    fn test_write_bytes() {
        let mut storage: Vec<u8> = vec![];
        BinaryWriter::write_bytes(&mut storage, &[8, 9, 6, 5]);
        assert!(storage == [8, 9, 6, 5]);
    }
}
