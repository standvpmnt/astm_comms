use anyhow::Context;
use bytes::Bytes;
use super::records_parser;

pub const SOH: [u8; 1] = [1];
pub const STX: [u8; 1] = [2];
pub const ETX: [u8; 1] = [3];
pub const EOT: [u8; 1] = [4];
pub const ENQ: [u8; 1] = [5];
pub const ACK: [u8; 1] = [6];
pub const DLE: [u8; 1] = [10];
// dc1 through dc4 from 11 through 14
pub const NAK: [u8; 1] = [b'\x15'];
pub const SYN: [u8; 1] = [16];
pub const ETB: [u8; 1] = [17];
pub const CR: [u8; 1] = [b'\x0D'];
pub const LF: [u8; 1] = [b'\x0A'];

#[derive(Debug, PartialEq)]
pub enum Record {
    // indicators are case insensitive
    Header(Bytes),                  //H
    Patient(Bytes),                 //P
    TestOrder(Bytes),               //O
    Result(Bytes),                  //R
    Comment(Bytes),                 //C
    RequestInformation(Bytes),      //Q
    Scientific(Bytes),              //S
    MessageTerminator(Bytes),       // L
    ManufacturerInformation(Bytes), //M
}

#[derive(thiserror::Error, Debug)]
pub enum RecordError {
    #[error("{0}")]
    InvalidInput(String),
    #[error("{0}")]
    MalformedRecord(String),
}

impl Record {
    pub fn parse_from_buf(buf_slice: &[u8]) -> Result<Record, RecordError> {
        println!("parsing buf slice: {:?}", buf_slice);
        match buf_slice[2] {
            b'h' | b'H' => Ok(Record::Header(Bytes::copy_from_slice(buf_slice))),
            b'p' | b'P' => Ok(Record::Patient(Bytes::copy_from_slice(buf_slice))),
            b'o' | b'O' => Ok(Record::TestOrder(Bytes::copy_from_slice(buf_slice))),
            b'r' | b'R' => Ok(Record::Result(Bytes::copy_from_slice(buf_slice))),
            b'c' | b'C' => Ok(Record::Comment(Bytes::copy_from_slice(buf_slice))),
            b'q' | b'Q' => Ok(Record::RequestInformation(Bytes::copy_from_slice(buf_slice))),
            b's' | b'S' => Ok(Record::Scientific(Bytes::copy_from_slice(buf_slice))),
            b'l' | b'L' => Ok(Record::MessageTerminator(Bytes::copy_from_slice(buf_slice))),
            b'm' | b'M' => Ok(Record::ManufacturerInformation(Bytes::copy_from_slice(buf_slice))),
            _ => Err(RecordError::MalformedRecord(format!(
                "record has some other item,{}", buf_slice[2]
            ))),
        }
    }

    // pub fn header_record() -> super::records_parser::Header {

    // }

    // fn frame_number(&self) -> u32 {
    //     self.inner()
    // }
}

#[cfg(test)]
mod tests {
    use super::Record;
    use claims::*;

    #[test]
    fn can_parse_head_record() {
        let input = "1H|\\^&|||c111^Roche^c111^4.2.2.1730^1^13085|||||host|PCUPL^BATCH|P|1|20230515160340\r";
        let output = Record::parse(input.to_owned()).expect("failed to parse string");
        match output {
            Record::Header(k) => assert!(k.len() > 0),
            _ => {
                println!("Failed to parse input \n {input}");
                assert_err!(Ok(5));
            }
        }
    }

    #[test]
    fn invalid_inputs_are_adequately_handles() {
        let input = "14";
        let input1 = "";
        let input2 = "ajdf";

        assert_err!(Record::parse(input.to_owned()));
        assert_err!(Record::parse(input1.to_owned()));
        assert_err!(Record::parse(input2.to_owned()));
    }

    #[test]
    fn can_get_frame_number_of_a_record() {
        let input = "1H|\\^&|||c111^Roche^c111^4.2.2.1730^1^13085|||||host|PCUPL^BATCH|P|1|20230515160340\r";
        let input1 = "2M|1|CR^BM^c111^1|712^BILT3|57884601|umol/L|BS^BILT3|712^SR^12547\\712^R1^1209|N^R|2|20230428183346|A^$SYS$||1.349997E-03^2.383310E-04|SD^^^59514300|70.7^0.0182^0.0186^0.0178^0^0\\0^0.00135^0.0016^0.0011^0^0\r";
        let input2 = "3L|1|N\r";

        assert_eq!(Record::parse(input.to_owned()).unwrap().frame_number(), 1);
        assert_eq!(Record::parse(input1.to_owned()).unwrap().frame_number(), 2);
        assert_eq!(Record::parse(input2.to_owned()).unwrap().frame_number(), 3);
    }
}
