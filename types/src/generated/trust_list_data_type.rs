// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data_value::*;
#[allow(unused_imports)]
use attribute::*;
#[allow(unused_imports)]
use date_time::*;
#[allow(unused_imports)]
use node_id::*;
#[allow(unused_imports)]
use service_types::*;
#[allow(unused_imports)]
use variant::*;
#[allow(unused_imports)]
use generated::node_ids::*;
#[allow(unused_imports)]
use generated::status_codes::StatusCode;
#[allow(unused_imports)]
use generated::status_codes::StatusCode::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TrustListDataType {
    pub specified_lists: UInt32,
    pub trusted_certificates: Option<Vec<ByteString>>,
    pub trusted_crls: Option<Vec<ByteString>>,
    pub issuer_certificates: Option<Vec<ByteString>>,
    pub issuer_crls: Option<Vec<ByteString>>,
}

impl MessageInfo for TrustListDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::TrustListDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<TrustListDataType> for TrustListDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.specified_lists.byte_len();
        size += byte_len_array(&self.trusted_certificates);
        size += byte_len_array(&self.trusted_crls);
        size += byte_len_array(&self.issuer_certificates);
        size += byte_len_array(&self.issuer_crls);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.specified_lists.encode(stream)?;
        size += write_array(stream, &self.trusted_certificates)?;
        size += write_array(stream, &self.trusted_crls)?;
        size += write_array(stream, &self.issuer_certificates)?;
        size += write_array(stream, &self.issuer_crls)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let specified_lists = UInt32::decode(stream)?;
        let trusted_certificates: Option<Vec<ByteString>> = read_array(stream)?;
        let trusted_crls: Option<Vec<ByteString>> = read_array(stream)?;
        let issuer_certificates: Option<Vec<ByteString>> = read_array(stream)?;
        let issuer_crls: Option<Vec<ByteString>> = read_array(stream)?;
        Ok(TrustListDataType {
            specified_lists,
            trusted_certificates,
            trusted_crls,
            issuer_certificates,
            issuer_crls,
        })
    }
}
