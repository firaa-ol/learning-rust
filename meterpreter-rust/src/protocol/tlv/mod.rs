use std::collections::HashMap;
use std::hash::Hash;

mod add;
mod binary_reader;
mod binary_writer;

pub use add::Add;

pub use self::binary_reader::BinaryReader;
pub use self::binary_writer::BinaryWriter;

#[derive(Debug, PartialEq)]
#[repr(u32)]
pub enum MetaType {
    None = 0,
    String = (1 << 16),
    Uint = (1 << 17),
    Raw = (1 << 18),
    Bool = (1 << 19),
    Qword = (1 << 20),
    Compressed = (1 << 29),
    Group = (1 << 30),
    Complex = (1 << 31),
    All = MetaType::None as u32
        | MetaType::String as u32
        | MetaType::Uint as u32
        | MetaType::Raw as u32
        | MetaType::Bool as u32
        | MetaType::Qword as u32
        | MetaType::Compressed as u32
        | MetaType::Group as u32
        | MetaType::Complex as u32,
}

pub const STDAPI_PLUGIN: u32 = 0;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[repr(u32)]
pub enum TlvType {
    // General/Base Type Tlvs
    Any = MetaType::None as u32,
    Method = MetaType::String as u32 | 1,
    RequestId = MetaType::String as u32 | 2,
    Exception = MetaType::Group as u32 | 3,
    Result = MetaType::Uint as u32 | 4,
    String = MetaType::String as u32 | 10,
    Uint = MetaType::Uint as u32 | 11,
    Bool = MetaType::Bool as u32 | 12,
    Length = MetaType::Uint as u32 | 25,
    Data = MetaType::Raw as u32 | 26,
    Flags = MetaType::Uint as u32 | 27,
    // Channel TLVs
    ChannelId = MetaType::Uint as u32 | 50,
    ChannelType = MetaType::String as u32 | 51,
    ChannelData = MetaType::Raw as u32 | 52,
    ChannelDataGroup = MetaType::Group as u32 | 53,
    ChannelClass = MetaType::Uint as u32 | 54,
    ChannelParentId = MetaType::Uint as u32 | 55,
    // File seeking TLVs
    SeekWhence = MetaType::Uint as u32 | 70,
    SeekOffset = MetaType::Uint as u32 | 71,
    SeekPos = MetaType::Uint as u32 | 72,
    // Exception/error TLVs
    ExceptionCode = MetaType::Uint as u32 | 300,
    ExceptionString = MetaType::String as u32 | 301,
    // Migration TLVs
    LibraryPath = MetaType::String as u32 | 400,
    TargetPath = MetaType::String as u32 | 401,
    MigratePid = MetaType::Uint as u32 | 402,
    MigratePayloadLen = MetaType::Uint as u32 | 403,
    MigratePayload = MetaType::String as u32 | 404,
    MigrateArch = MetaType::Uint as u32 | 405,
    MigrateBaseAddr = MetaType::Uint as u32 | 407,
    MigrateEntryPoint = MetaType::Uint as u32 | 408,
    MigrateSocketPath = MetaType::Uint as u32 | 409,
    MigrateStubLen = MetaType::Uint as u32 | 410,
    MigrateStub = MetaType::Uint as u32 | 411,
    // Transport TLVs
    TransType = MetaType::Uint as u32 | 430,
    TransUrl = MetaType::String as u32 | 431,
    TransUa = MetaType::String as u32 | 432,
    TransCommTimeout = MetaType::Uint as u32 | 433,
    TransSessExp = MetaType::Uint as u32 | 434,
    TransCertHash = MetaType::Raw as u32 | 435,
    TransProxyHost = MetaType::String as u32 | 436,
    TransProxyUser = MetaType::String as u32 | 437,
    TransProxyPass = MetaType::String as u32 | 438,
    TransRetryTotal = MetaType::Uint as u32 | 439,
    TransRetryWait = MetaType::Uint as u32 | 440,
    TransHeaders = MetaType::String as u32 | 441,
    TransGroup = MetaType::Group as u32 | 442,
    // Identification/session TLVs
    MachineId = MetaType::String as u32 | 460,
    UUID = MetaType::Raw as u32 | 461,
    SessionGuid = MetaType::Raw as u32 | 462,
    // Packet encryption TLVs
    RsaPubKey = MetaType::String as u32 | 550,
    SymKeyType = MetaType::Uint as u32 | 551,
    SymKey = MetaType::Raw as u32 | 552,
    EncSymKey = MetaType::Raw as u32 | 553,
    // Pivot TLVs
    PivotId = MetaType::Raw as u32 | 650,
    PivotStageData = MetaType::Raw as u32 | 651,
    PivotStageDataLen = MetaType::Uint as u32 | 652,
    PivotNamedPipeName = MetaType::String as u32 | 653,
    // STDAPI stuff
    StdapiComputerName = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1040),
    StdapiOperatingSystemName = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1041),
    StdapiUserName = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1042),
    StdapiArchitecture = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1043),
    StdapiLangSystem = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1044),
    StdapiSid = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1045),
    StdapiDomain = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1046),
    StdapiLoggedOnUserCount = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 1047),
    StdapiLocalDateTime = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1048),
    StdapiEnvVariable = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1100),
    StdapiEnvValue = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1101),
    StdapiEnvGroup = MetaType::Group as u32 | (STDAPI_PLUGIN as u32 + 1102),
    StdapiDirectoryPath = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1200),
    StdapiFileName = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1201),
    StdapiFilePath = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1202),
    StdapiFileMode = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1203),
    StdapiFileSize = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 1204),
    StdapiFileShortName = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1205),
    StdapiFileHash = MetaType::Raw as u32 | (STDAPI_PLUGIN as u32 + 1206),
    StdapiMount = MetaType::Group as u32 | (STDAPI_PLUGIN as u32 + 1207),
    StdapiMountName = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1208),
    StdapiMountType = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 1209),
    StdapiMountSpaceUser = MetaType::Qword as u32 | (STDAPI_PLUGIN as u32 + 1210),
    StdapiMountSpaceTotal = MetaType::Qword as u32 | (STDAPI_PLUGIN as u32 + 1211),
    StdapiMountSpaceFree = MetaType::Qword as u32 | (STDAPI_PLUGIN as u32 + 1212),
    StdapiMountUncPath = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1213),
    StdapiStatBuf32 = MetaType::Complex as u32 | (STDAPI_PLUGIN as u32 + 1220),
    StdapiStatBuf = MetaType::Complex as u32 | (STDAPI_PLUGIN as u32 + 1221),
    StdapiInterfaceMtu = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 1402),
    StdapiInterfaceFlags = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1403),
    StdapiInterfaceIndex = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 1404),
    StdapiSubnet = MetaType::Raw as u32 | (STDAPI_PLUGIN as u32 + 1420),
    StdapiNetmask = MetaType::Raw as u32 | (STDAPI_PLUGIN as u32 + 1421),
    StdapiGateway = MetaType::Raw as u32 | (STDAPI_PLUGIN as u32 + 1422),
    StdapiNetworkRoute = MetaType::Group as u32 | (STDAPI_PLUGIN as u32 + 1423),
    StdapiIpPrefix = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 1424),
    StdapiArpEntry = MetaType::Group as u32 | (STDAPI_PLUGIN as u32 + 1425),
    StdapiIp = MetaType::Raw as u32 | (STDAPI_PLUGIN as u32 + 1430),
    StdapiMacAddr = MetaType::Raw as u32 | (STDAPI_PLUGIN as u32 + 1431),
    StdapiMacName = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1432),
    StdapiNetworkInterface = MetaType::Group as u32 | (STDAPI_PLUGIN as u32 + 1433),
    StdapiIp6Scope = MetaType::Raw as u32 | (STDAPI_PLUGIN as u32 + 1434),
    StdapiSubnetString = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1440),
    StdapiNetmaskString = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1441),
    StdapiGatewayString = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1442),
    StdapiRouteMetric = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 1443),
    StdapiAddrType = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 1444),
    StdapiProxyCfgAutodetect = MetaType::Bool as u32 | (STDAPI_PLUGIN as u32 + 1445),
    StdapiProxyCfgAutoConfigUrL = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1446),
    StdapiProxyCfgProxy = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1447),
    StdapiProxyCfgProxyBypass = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1448),
    StdapiPeerHost = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1500),
    StdapiPeerPort = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 1501),
    StdapiLocalHost = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 1502),
    StdapiLocalPort = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 1503),
    StdapiConnectRetries = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 1504),
    StdapiNetstatEntry = MetaType::Group as u32 | (STDAPI_PLUGIN as u32 + 1505),
    StdapiPeerHostRaw = MetaType::Raw as u32 | (STDAPI_PLUGIN as u32 + 1506),
    StdapiLocalHostRaw = MetaType::Raw as u32 | (STDAPI_PLUGIN as u32 + 1507),
    StdapiShutdownHow = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 1530),
    StdapiProcessId = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 2300),
    StdapiProcessName = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 2301),
    StdapiProcessPath = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 2302),
    StdapiProcessGroup = MetaType::Group as u32 | (STDAPI_PLUGIN as u32 + 2303),
    StdapiProcessFlags = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 2304),
    StdapiProcessArguments = MetaType::String as u32 | (STDAPI_PLUGIN as u32 + 2305),
    StdapiProcessArch = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 2306),
    StdapiProcessParentProcessId = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 2307),
    StdapiProcessSession = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 2308),
    StdapiPowerFlags = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 4100),
    StdapiPowerReason = MetaType::Uint as u32 | (STDAPI_PLUGIN as u32 + 4101),
}

impl TlvType {
    fn to_meta_type(&self) -> MetaType {
        let val = MetaType::All as u32 & *self as u32;
        //TODO: find a better way without unsafe
        let meta_type: MetaType = unsafe { ::std::mem::transmute(val) };
        meta_type
    }
}

impl From<u32> for TlvType {
    fn from(val: u32) -> Self {
        let tlv_type: TlvType = unsafe { ::std::mem::transmute(val) };
        tlv_type
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum TlvValue {
    Bool(bool),
    UInt(u32),
    ULongInt(u64),
    String(String),
    Bytes(Vec<u8>),
}

#[derive(Debug)]
pub struct Tlv {
    pub value: Option<TlvValue>,
    pub tlv_type: TlvType,
    pub tlvs: HashMap<TlvType, Vec<Tlv>>,
}

impl Tlv {
    pub fn new(tlv_type: TlvType, value: TlvValue) -> Tlv {
        Self {
            tlv_type,
            value: Some(value),
            tlvs: HashMap::new(),
        }
    }

    pub fn from_raw(storage: &Vec<u8>, position: &mut usize) -> Self {
        let length = BinaryReader::read_dword(storage, position) - 8;
        let tlv_type = BinaryReader::read_tlv_type(storage, position);
        let meta_type = tlv_type.to_meta_type();
        let mut tlv = Self {
            tlv_type,
            value: None,
            tlvs: HashMap::new(),
        };

        if meta_type == MetaType::Group {
            while *position < storage.len() {
                tlv.add_tlv(Tlv::from_raw(storage, position))
            }
        } else {
            match meta_type {
                MetaType::Bool => {
                    tlv.value = Some(TlvValue::Bool(BinaryReader::read_bool(storage, position)));
                }
                MetaType::Uint => {
                    tlv.value = Some(TlvValue::UInt(BinaryReader::read_dword(storage, position)));
                }
                MetaType::Qword => {
                    tlv.value = Some(TlvValue::ULongInt(BinaryReader::read_qword(
                        storage, position,
                    )));
                }
                MetaType::String => {
                    tlv.value = Some(TlvValue::String(BinaryReader::read_string(
                        storage, position, length,
                    )));
                }
                MetaType::Raw | MetaType::Complex => {
                    tlv.value = Some(TlvValue::Bytes(BinaryReader::read_bytes(
                        storage, position, length,
                    )))
                }
                MetaType::None | MetaType::Compressed => {
                    panic!("Compressed or None MetaType Not Supported")
                }
                _ => panic!("Unexpected MetaType {:?}", meta_type),
            }
        }

        tlv
    }

    pub fn to_raw(&self, storage: &mut Vec<u8>) {
        let meta_type = self.tlv_type.to_meta_type();
        if meta_type == MetaType::Group {
            let mut tlv_group_data: Vec<u8> = vec![];
            for tlv in self.tlvs.values().flatten() {
                tlv.to_raw(&mut tlv_group_data);
            }

            BinaryWriter::write_dword(storage, tlv_group_data.len() as u32 + 8);
            BinaryWriter::write_tlv_type(storage, self.tlv_type);
            BinaryWriter::write_bytes(storage, &tlv_group_data);
        } else {
            match meta_type {
                MetaType::Bool => {
                    BinaryWriter::write_dword(storage, 1 + 8); // Length
                    BinaryWriter::write_tlv_type(storage, self.tlv_type); // Type
                    BinaryWriter::write_bool(storage, self.value_as_bool()); //Value
                }
                MetaType::Uint => {
                    BinaryWriter::write_dword(storage, 4 + 8);
                    BinaryWriter::write_tlv_type(storage, self.tlv_type);
                    BinaryWriter::write_dword(storage, self.value_as_uint32());
                }
                MetaType::Qword => {
                    BinaryWriter::write_dword(storage, 8 + 8);
                    BinaryWriter::write_tlv_type(storage, self.tlv_type);
                    BinaryWriter::write_qword(storage, self.value_as_uint64());
                }
                MetaType::String => {
                    let value = self.value_as_string();
                    BinaryWriter::write_dword(storage, value.len() as u32 + 1 + 8);
                    BinaryWriter::write_tlv_type(storage, self.tlv_type);
                    BinaryWriter::write_string(storage, value);
                }
                MetaType::Raw | MetaType::Complex => {
                    let value = self.value_as_bytes();
                    BinaryWriter::write_dword(storage, value.len() as u32 + 8);
                    BinaryWriter::write_tlv_type(storage, self.tlv_type);
                    BinaryWriter::write_bytes(storage, value);
                }
                MetaType::None | MetaType::Compressed => {
                    panic!("Compressed or None MetaType Not Supported")
                }
                _ => panic!("Unexpected MetaType {:?}", meta_type),
            }
        }
    }

    fn validate_meta_type(&self, expected_types: Vec<MetaType>) {
        if !expected_types.contains(&self.tlv_type.to_meta_type()) {
            panic!(
                "Expecting MetaType {:?} but provided type {:?}",
                expected_types, &self.tlv_type
            );
        }
    }

    pub fn value_as_string(&self) -> String {
        match self
            .value
            .as_ref()
            .expect("Unable to extract value from a TLV")
        {
            TlvValue::String(val) => val.to_string(),
            _ => panic!("Didn't find expected type"),
        }
    }

    pub fn value_as_bool(&self) -> bool {
        match self
            .value
            .as_ref()
            .expect("Unable to extract value from a TLV")
        {
            TlvValue::Bool(val) => val.to_owned(),
            _ => panic!("Didn't find expected type"),
        }
    }

    pub fn value_as_uint32(&self) -> u32 {
        match self
            .value
            .as_ref()
            .expect("Unable to extract value from a TLV")
        {
            TlvValue::UInt(val) => val.to_owned(),
            _ => panic!("Didn't find expected type"),
        }
    }

    pub fn value_as_uint64(&self) -> u64 {
        match self
            .value
            .as_ref()
            .expect("Unable to extract value from a TLV")
        {
            TlvValue::ULongInt(val) => val.to_owned(),
            _ => panic!("Didn't find expected type"),
        }
    }

    pub fn value_as_bytes(&self) -> &Vec<u8> {
        match self
            .value
            .as_ref()
            .expect("Unable to extract value from a TLV")
        {
            TlvValue::Bytes(val) => val,
            _ => panic!("Didn't find expected type"),
        }
    }
}
//TODO: pass byte array by reference or boxed
//TODO: metatype validation
impl Add for Tlv {
    fn add_string(&mut self, tlv_type: TlvType, value: String) {
        self.validate_meta_type(vec![MetaType::Group]);
        let tlv = Tlv::new(tlv_type, TlvValue::String(value));
        self.add_tlv(tlv);
    }

    fn add_uint32(&mut self, tlv_type: TlvType, value: u32) {
        self.validate_meta_type(vec![MetaType::Group]);
        let tlv = Tlv::new(tlv_type, TlvValue::UInt(value));
        self.add_tlv(tlv);
    }

    fn add_uint64(&mut self, tlv_type: TlvType, value: u64) {
        self.validate_meta_type(vec![MetaType::Group]);
        let tlv = Tlv::new(tlv_type, TlvValue::ULongInt(value));
        self.add_tlv(tlv);
    }

    fn add_bool(&mut self, tlv_type: TlvType, value: bool) {
        self.validate_meta_type(vec![MetaType::Group]);
        let tlv = Tlv::new(tlv_type, TlvValue::Bool(value));
        self.add_tlv(tlv);
    }

    fn add_bytes(&mut self, tlv_type: TlvType, value: Vec<u8>) {
        self.validate_meta_type(vec![MetaType::Group]);
        let tlv = Tlv::new(tlv_type, TlvValue::Bytes(value));
        self.add_tlv(tlv);
    }

    fn add_group(&mut self, tlv_type: TlvType) {
        self.validate_meta_type(vec![MetaType::Group]);
        let tlv = Self {
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

//TODO: better grouping of tests
#[cfg(test)]
mod test {
    use crate::protocol::tlv::{MetaType, Tlv, TlvType, TlvValue};
    use std::collections::HashMap;

    use super::Add;

    #[test]
    fn test_tlvtype_to_metatype() {
        assert_eq!(TlvType::Any.to_meta_type(), MetaType::None);
        assert_eq!(TlvType::Method.to_meta_type(), MetaType::String);
        assert_eq!(
            TlvType::StdapiMountSpaceTotal.to_meta_type(),
            MetaType::Qword
        );
    }

    #[test]
    fn test_value_as_string() {
        let tlv = Tlv::new(
            TlvType::ChannelType,
            TlvValue::String(String::from("OneWay")),
        );
        assert_eq!(tlv.value_as_string(), "OneWay");
    }

    #[test]
    fn test_value_as_uint32() {
        let tlv = Tlv::new(TlvType::ChannelId, TlvValue::UInt(2));
        assert_eq!(tlv.value_as_uint32(), 2);
    }

    #[test]
    fn test_value_as_uint64() {
        let tlv = Tlv::new(
            TlvType::StdapiMountSpaceFree,
            TlvValue::ULongInt(624636823236762),
        );
        assert_eq!(tlv.value_as_uint64(), 624636823236762);
    }

    #[test]
    fn test_value_as_bool() {
        let tlv = Tlv::new(TlvType::StdapiProxyCfgAutodetect, TlvValue::Bool(true));
        assert_eq!(tlv.value_as_bool(), true);
    }

    #[test]
    fn test_value_as_bytes() {
        let tlv = Tlv::new(TlvType::ChannelData, TlvValue::Bytes(vec![35, 67, 0, 255]));
        let byte_val = tlv.value_as_bytes();
        assert_eq!(byte_val[0], 35);
        assert_eq!(byte_val[1], 67);
        assert_eq!(byte_val[2], 0);
        assert_eq!(byte_val[3], 255);
    }

    #[test]
    fn test_add() {
        let mut tlv = Tlv {
            tlv_type: TlvType::StdapiMount,
            value: None,
            tlvs: HashMap::new(),
        };
        tlv.add_string(TlvType::StdapiMountName, String::from("/sdf"));
        tlv.add_uint32(TlvType::StdapiMountType, 2);
        tlv.add_uint64(TlvType::StdapiMountSpaceFree, 2614672732);

        assert_eq!(
            tlv.tlvs
                .get(&TlvType::StdapiMountName)
                .unwrap()
                .first()
                .unwrap()
                .value_as_string(),
            "/sdf"
        );
        assert_eq!(
            tlv.tlvs
                .get(&TlvType::StdapiMountType)
                .unwrap()
                .first()
                .unwrap()
                .value_as_uint32(),
            2
        );
        assert_eq!(
            tlv.tlvs
                .get(&TlvType::StdapiMountSpaceFree)
                .unwrap()
                .first()
                .unwrap()
                .value_as_uint64(),
            2614672732
        );
    }

    #[test]
    fn test_bool_tlv_to_raw() {
        let tlv = Tlv::new(TlvType::StdapiProxyCfgAutodetect, TlvValue::Bool(true));
        let mut storage: Vec<u8> = vec![];
        tlv.to_raw(&mut storage);
        assert_eq!(storage.len(), 9);
        assert!(storage == [0, 0, 0, 9, /**/ 0, 8, 5, 165, /**/ 1]);
    }

    #[test]
    fn test_uint_tlv_to_raw() {
        let tlv = Tlv::new(TlvType::ChannelId, TlvValue::UInt(2));
        let mut storage: Vec<u8> = vec![];
        tlv.to_raw(&mut storage);
        assert_eq!(storage.len(), 12);
        assert!(storage == [0, 0, 0, 12, /**/ 0, 2, 0, 50, /**/ 0, 0, 0, 2]);
    }

    #[test]
    fn test_qword_tlv_to_raw() {
        let tlv = Tlv::new(TlvType::StdapiMountSpaceFree, TlvValue::ULongInt(65535));
        let mut storage: Vec<u8> = vec![];
        tlv.to_raw(&mut storage);
        assert_eq!(storage.len(), 16);
        assert!(storage == [0, 0, 0, 16, /**/ 0, 16, 4, 188, /**/ 0, 0, 0, 0, 0, 0, 255, 255]);
    }

    #[test]
    fn test_string_tlv_to_raw() {
        let tlv = Tlv::new(TlvType::ChannelType, TlvValue::String("duplex".to_owned()));
        let mut storage: Vec<u8> = vec![];
        tlv.to_raw(&mut storage);
        assert_eq!(storage.len(), 15);
        assert!(storage == [0, 0, 0, 15, /**/ 0, 1, 0, 51, /**/ 100, 117, 112, 108, 101, 120, 0]);
    }

    #[test]
    fn test_bytes_tlv_to_raw() {
        let tlv = Tlv::new(
            TlvType::TransCertHash,
            TlvValue::Bytes(vec![89, 77, 22, 23, 45]),
        );
        let mut storage: Vec<u8> = vec![];
        tlv.to_raw(&mut storage);
        assert_eq!(storage.len(), 13);
        assert!(storage == [0, 0, 0, 13, /**/ 0, 4, 1, 179, /**/ 89, 77, 22, 23, 45]);
    }

    #[test]
    #[should_panic]
    fn test_any_tlv_to_raw() {
        let tlv = Tlv::new(TlvType::Any, TlvValue::Bool(false));
        let mut storage: Vec<u8> = vec![];
        tlv.to_raw(&mut storage);
    }

    #[test]
    fn test_from_raw_to_bool_tlv() {
        let tlv = Tlv::new(TlvType::StdapiProxyCfgAutodetect, TlvValue::Bool(true));
        let mut storage: Vec<u8> = vec![];
        tlv.to_raw(&mut storage);

        let mut position = 0;
        let tlv = Tlv::from_raw(&storage, &mut position);

        assert_eq!(tlv.tlv_type, TlvType::StdapiProxyCfgAutodetect);
        assert_eq!(tlv.value.unwrap(), TlvValue::Bool(true));
    }

    #[test]
    fn test_from_raw_to_uint_tlv() {
        let tlv = Tlv::new(TlvType::ChannelId, TlvValue::UInt(2));
        let mut storage: Vec<u8> = vec![];
        tlv.to_raw(&mut storage);

        let mut position = 0;
        let tlv = Tlv::from_raw(&storage, &mut position);

        assert_eq!(tlv.tlv_type, TlvType::ChannelId);
        assert_eq!(tlv.value.unwrap(), TlvValue::UInt(2));
    }

    #[test]
    fn test_from_raw_to_qword_tlv() {
        let tlv = Tlv::new(TlvType::StdapiMountSpaceFree, TlvValue::ULongInt(65535));
        let mut storage: Vec<u8> = vec![];
        tlv.to_raw(&mut storage);

        let mut position = 0;
        let tlv = Tlv::from_raw(&storage, &mut position);

        assert_eq!(tlv.tlv_type, TlvType::StdapiMountSpaceFree);
        assert_eq!(tlv.value.unwrap(), TlvValue::ULongInt(65535));
    }

    #[test]
    fn test_from_raw_to_string_tlv() {
        let tlv = Tlv::new(TlvType::ChannelType, TlvValue::String("duplex".to_owned()));
        let mut storage: Vec<u8> = vec![];
        tlv.to_raw(&mut storage);

        let mut position = 0;
        let tlv = Tlv::from_raw(&storage, &mut position);

        assert_eq!(tlv.tlv_type, TlvType::ChannelType);
        assert_eq!(tlv.value.unwrap(), TlvValue::String("duplex".to_owned()));
    }

    #[test]
    fn test_from_raw_to_bytes_tlv() {
        let tlv = Tlv::new(
            TlvType::TransCertHash,
            TlvValue::Bytes(vec![89, 77, 22, 23, 45]),
        );
        let mut storage: Vec<u8> = vec![];
        tlv.to_raw(&mut storage);

        let mut position = 0;
        let tlv = Tlv::from_raw(&storage, &mut position);

        assert_eq!(tlv.tlv_type, TlvType::TransCertHash);
        assert_eq!(
            tlv.value.unwrap(),
            TlvValue::Bytes(vec![89, 77, 22, 23, 45])
        );
    }

    #[test]
    fn test_from_raw_to_group_tlv() {
        let mut tlv = Tlv {
            tlv_type: TlvType::TransGroup,
            value: None,
            tlvs: HashMap::new(),
        };
        tlv.add_uint32(TlvType::TransType, 3);
        tlv.add_string(TlvType::TransUrl, "https://ch.rs".to_string());
        tlv.add_bytes(TlvType::UUID, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        tlv.add_uint64(TlvType::StdapiMountSpaceFree, 65548);

        let mut storage: Vec<u8> = vec![];
        tlv.to_raw(&mut storage);

        let mut position = 0;
        let tlv = Tlv::from_raw(&storage, &mut position);

        assert_eq!(tlv.tlv_type, TlvType::TransGroup);
        assert_eq!(
            tlv.tlvs
                .get(&TlvType::TransType)
                .unwrap()
                .first()
                .unwrap()
                .value
                .as_ref()
                .unwrap(),
            &TlvValue::UInt(3)
        );
        assert_eq!(
            tlv.tlvs
                .get(&TlvType::TransUrl)
                .unwrap()
                .first()
                .unwrap()
                .value
                .as_ref()
                .unwrap(),
            &TlvValue::String("https://ch.rs".to_string())
        );
        assert_eq!(
            tlv.tlvs
                .get(&TlvType::UUID)
                .unwrap()
                .first()
                .unwrap()
                .value
                .as_ref()
                .unwrap(),
            &TlvValue::Bytes(vec![1, 2, 3, 4, 5, 6, 7, 8])
        );
        assert_eq!(
            tlv.tlvs
                .get(&TlvType::StdapiMountSpaceFree)
                .unwrap()
                .first()
                .unwrap()
                .value
                .as_ref()
                .unwrap(),
            &TlvValue::ULongInt(65548)
        );
    }
}
