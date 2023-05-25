// this module will ensure serialports connected are available for communication
use anyhow::Context;
use tokio::{task, time::Duration};
use tokio_serial::{available_ports, SerialPort, SerialPortInfo};

use std::collections::HashMap;

use super::std_messages::{Record, ACK, ENQ, EOT};
use super::records_parser;

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
                    eprintln!("Unhandled scenario of buffer being undersized for \
                              data about to be transmitted, buffer length is {} \
                              data size is {}", buf.len(), ready_bytes_later
                              );
                    // raise some slack notifications here
                }
                println!("reading {} bytes from equipment to buffer", ready_bytes_later);
                let data = handle.read(&mut buf[..]);
                match data {
                    Err(k) => {
                        eprintln!("error in reading data {:#?}", k);
                        continue;
                    }
                    Ok(k) => {
                        println!("size of data received is {:?}", k);
                        // println!("received data in bytes is {:#?}", &buf[..k]);
                        println!("Read data is {:#?}", String::from_utf8_lossy(&buf[0..k]));
                        // check that the frame is an end-frame or mid-frame
                        handle.write(&ACK).expect("failed to send ack");
                        // ensure this is called only when the buffer contains values that
                        // are greater than 1
                        if k > 1 {
                            // push these records into a vector?
                            split_to_records(&buf[..k]);
                        }
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
            Record::Header(data) => Some(records_parser::Header::new(data)),
            _ => {None},
        };
        if let Some(r) = record {
            dbg!(r.message_control_id());
        }
    // }

    Record::Header(bytes::Bytes::copy_from_slice("hello".as_bytes()))
}
