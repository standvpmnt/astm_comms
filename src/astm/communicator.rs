// this module will ensure serialports connected are available for communication
use anyhow::Context;
use tokio::{task, time::Duration};
use tokio_serial::{available_ports, SerialPort, SerialPortInfo};

use std::collections::HashMap;

use crate::astm::records_parser::*;
use crate::astm::std_messages::{ACK, ENQ, EOT, ETX};

pub async fn all_machines(
    mut machines: HashMap<String, Box<dyn SerialPort>>,
) -> HashMap<String, Box<dyn SerialPort>> {
    let potential_ports = available_ports().unwrap();
    for port in potential_ports {
        match serial_handle(port) {
            Ok(handle) => {
                if is_astm_compliant(handle.try_clone().expect("Failed to clone device handle"))
                    .await
                {
                    machines.insert(handle.name().unwrap(), handle);
                }
            }
            Err(e) => {
                eprintln!("failed to get handle of machine {:#?}", e);
            }
        }
    }
    machines
}

// this assumes certain default values, these need to be configurable though
// TODO! implement these with configuration files or env variables instead
fn serial_handle(port_info: SerialPortInfo) -> Result<Box<dyn SerialPort>, anyhow::Error> {
    Ok(tokio_serial::new(port_info.port_name, 115200)
        .timeout(Duration::from_secs(30))
        .data_bits(tokio_serial::DataBits::Eight)
        .flow_control(tokio_serial::FlowControl::Software)
        .parity(tokio_serial::Parity::None)
        .stop_bits(tokio_serial::StopBits::One)
        .open()
        .context("failed to open port")?)
}

pub async fn is_astm_compliant(mut handle: Box<dyn SerialPort>) -> bool {
    let mut serial_buf = [0u8; 2];
    let mut cloned_handle = handle.try_clone().expect("Failed to clone handle");
    let read_handle = task::spawn(async move {
        loop {
            match handle.read(serial_buf.as_mut_slice()) {
                Ok(size) => {
                    if size == 1 {
                        if serial_buf[0] != b'\x06' {
                            eprintln!("{:#?} received for enq message", serial_buf[0]);
                            break false;
                        }
                        handle.write(&EOT).expect("Failed to send EOT to equipment");
                        break true;
                    };
                    if size != 1 {
                        eprintln!("{:#?} size response received for enq", size);
                        break false;
                    };
                }
                Err(k) => {
                    println!("{:#?}", k);
                    break false;
                }
            }
        }
    });
    cloned_handle.write(&ENQ).expect("Failed to send data");
    return read_handle.await.unwrap();
}

pub async fn process_incoming(mut handle: Box<dyn SerialPort>) {
    let h = task::spawn(async move {
        // let mut buf = bytes::BytesMut::with_capacity(400);
        let mut buf = [0u8; 600];

        tokio::time::sleep(Duration::from_millis(100)).await;
        let mut message: Vec<Record> = vec![];
        loop {
            let ready_bytes = handle
                .bytes_to_read()
                .expect("failed to get bytes number to be read");
            tokio::time::sleep(Duration::from_millis(10)).await;
            let ready_bytes_later = handle
                .bytes_to_read()
                .expect("failed to get bytes number to be read");

            if ready_bytes_later == 0 {
                tokio::time::sleep(Duration::from_secs(2)).await;
            } else if ready_bytes_later > ready_bytes {
                tokio::time::sleep(Duration::from_millis(50)).await;
            } else if ready_bytes_later == ready_bytes {
                if ready_bytes_later > buf.len() as u32 {
                    // if ready_bytes_later > buf.capacity() as u32 {
                    eprintln!(
                        "Unhandled scenario of buffer being undersized for \
                              data about to be transmitted, buffer length is {} \
                              data size is {}",
                        buf.len(),
                        ready_bytes_later
                    );
                    // raise some slack notifications here
                }
                println!(
                    "reading {} bytes from equipment to buffer",
                    ready_bytes_later
                );
                let data = handle.read(&mut buf[..]);
                match data {
                    Err(k) => {
                        eprintln!("error in reading data {:#?}", k);
                        continue;
                    }
                    Ok(k) => {
                        // ensure this is called only when the buffer contains values that
                        // are greater than 1
                        // actually check for various values that can possibly be in this
                        // eg. if single value of ENQ, etc., raise error in case the
                        // response is unknown case
                        if k > 1 {
                            // parse input into a record
                            // validate the record
                            // if validation fails send NAK and let loop re-run, raise alerts
                            // as well
                            // if validation passes, push the record into a vector
                            message.push(split_to_records(&buf[..k]));
                        }
                        if &buf[k - 5] == &ETX[0] {
                            handle_incoming_request(message);
                            // handle message
                            //
                            // reset message
                            // message.clear();
                            break;
                        }
                        println!("size of data received is {:?}", k);
                        // println!("received data in bytes is {:#?}", &buf[..k]);
                        println!("Read data is {:#?}", String::from_utf8_lossy(&buf[0..k]));
                        // check that the frame is an end-frame or mid-frame
                        handle.write(&ACK).expect("failed to send ack");
                    }
                }
            }
        }
    });
    h.await.unwrap();
}

// this is mut to ensure it is not changed underneath while processing
pub fn split_to_records(buf: &[u8]) -> Record {
    // split buffer on <CR> and include it in previous record
    // for slice in buf.split_inclusive(|item| item == &b'\x0D') {
    let rec = Record::parse_from_buf(&buf[..]).unwrap();
    let record = match rec {
        Record::Header(_) => Some(Header::new(rec)),
        _ => None,
    };
    if let Some(r) = record {
        dbg!(r.message_control_id());
    }
    // }

    Record::parse_from_buf(&buf[..]).unwrap()
}

fn handle_incoming_request(message: Vec<Record>) {
    // let h = tokio::spawn(async move{
    let mut messages = message.into_iter();
    for record in messages {
        println!("{:?}", record.inner());
    }
    // let header = Header::new(messages.next().unwrap());
    // println!("{:?}", header.sent_at());
    // println!("{:#?}", header.special_instructions());
    // println!("{:#?}", header.receiver_id());
    // println!(
    //     "{:#?}",
    //     std::str::from_utf8(header.sender_characteristics().unwrap())
    // );
    // println!("{:#?}", header.version_number());
    // println!("{:#?}", header.processing_id());
    // for record in message.into_iter() {
    //     let header = Header::new(record);
    //     dbg!(header);
    // }
    // });
    // h.await.unwrap();
}

fn checksum(input: &[u8]) -> (u8, u8) {
    let len = input.len();
    // using indexing here instead of filtering, since filtering leaves out
    // inner data values
    let checksum: u32 = input[1..=len-5].iter()
        .map(|&x| x as u32)
        .sum();
    let checksum = format!("{:x}", (checksum % 256) as u8);
    // let checksum = checksum.as_str();
    println!("{}", checksum);
    if checksum.len() != 2 {
        panic!("Issue with checksum logic implementation");
    }
    // TODO! use a buffer instead
    let alpha = (checksum.chars().nth(0).unwrap().to_ascii_uppercase().to_string().as_bytes()[0],
                 checksum.chars().nth(1).unwrap().to_ascii_uppercase().to_string().as_bytes()[0]);
    alpha
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::*;

    #[test]
    fn sending_multiple_records_for_splitting() {
        let head_record: &[u8] = &[
            2, 49, 72, 124, 92, 94, 38, 124, 124, 124, 99, 49, 49, 49, 94, 82, 111, 99, 104, 101,
            94, 99, 49, 49, 49, 94, 52, 46, 50, 46, 50, 46, 49, 55, 51, 48, 94, 49, 94, 49, 51, 48,
            56, 53, 124, 124, 124, 124, 124, 104, 111, 115, 116, 124, 82, 83, 85, 80, 76, 94, 66,
            65, 84, 67, 72, 124, 80, 124, 49, 124, 50, 48, 50, 51, 48, 53, 50, 53, 49, 54, 52, 57,
            51, 51, 13, 23, 70, 68, 13, 10,
        ];
        let patient_record: &[u8] = &[2, 50, 80, 124, 49, 124, 124, 13, 23, 52, 66, 13, 10];
        let order_record: &[u8] = &[
            2, 51, 79, 124, 49, 124, 80, 67, 67, 67, 49, 94, 53, 50, 53, 48, 50, 55, 48, 48, 94,
            50, 48, 50, 51, 49, 50, 51, 49, 124, 49, 51, 57, 49, 124, 94, 94, 94, 55, 49, 50, 124,
            124, 124, 124, 124, 124, 124, 81, 124, 124, 124, 124, 124, 124, 124, 124, 124, 124,
            124, 50, 48, 50, 51, 48, 53, 50, 53, 49, 54, 52, 57, 51, 51, 124, 124, 124, 70, 13, 23,
            48, 50, 13, 10,
        ];
        let result_record: &[u8] = &[
            2, 52, 82, 124, 49, 124, 94, 94, 94, 55, 49, 50, 124, 48, 46, 57, 124, 109, 103, 47,
            100, 76, 124, 49, 46, 48, 92, 48, 46, 57, 92, 49, 46, 48, 124, 78, 124, 124, 82, 124,
            124, 36, 83, 89, 83, 36, 124, 124, 50, 48, 50, 51, 48, 52, 50, 56, 49, 56, 52, 49, 49,
            54, 13, 23, 67, 55, 13, 10,
        ];
        let comment_record: &[u8] = &[
            2, 53, 67, 124, 49, 124, 73, 124, 124, 73, 13, 23, 52, 70, 13, 10,
        ];
        let termination_record: &[u8] = &[2, 54, 76, 124, 49, 124, 78, 13, 3, 48, 57, 13, 10];

        let message = vec![
            Record::parse_from_buf(head_record).expect("failed to parse buffer"),
            Record::parse_from_buf(patient_record).expect("failed to parse buffer"),
            Record::parse_from_buf(order_record).expect("failed to parse buffer"),
            Record::parse_from_buf(result_record).expect("failed to parse buffer"),
            Record::parse_from_buf(comment_record).expect("failed to parse buffer"),
            Record::parse_from_buf(termination_record).expect("failed to parse buffer"),
        ];

        handle_incoming_request(message);
    }

    #[test]
    fn checksum_calculates_correct_value() {
        let input = [02, b'\x31', b'\x54', b'\x65', b'\x73', b'\x74', 03, 52, 70, 13, 10];
        assert_eq!(checksum(&input), (b'D', b'4'))
    }

    #[test]
    fn checksum_calculates_correct_value_for_record_types() {
        let input: &[u8] = &[
            2, 49, 72, 124, 92, 94, 38, 124, 124, 124, 99, 49, 49, 49, 94, 82, 111, 99, 104, 101,
            94, 99, 49, 49, 49, 94, 52, 46, 50, 46, 50, 46, 49, 55, 51, 48, 94, 49, 94, 49, 51, 48,
            56, 53, 124, 124, 124, 124, 124, 104, 111, 115, 116, 124, 82, 83, 85, 80, 76, 94, 66,
            65, 84, 67, 72, 124, 80, 124, 49, 124, 50, 48, 50, 51, 48, 53, 50, 53, 49, 54, 52, 57,
            51, 51, 13, 23, 70, 68, 13, 10,
        ];
        assert_eq!(checksum(&input), (b'F', b'D'))
    }
}
