use std::str;

// this module will ensure serialports connected are available for communication
use tokio::time::Duration;

use super::std_messages::{ACK, ENQ, EOT, NAK};

pub async fn list_ports() {
    let res = serialport::available_ports().unwrap();
    for port in res {
        is_astm_compliant(port).await;
    }
}

enum BaudRates {
    elevenK,
    nineK,
}

// get all ports
pub async fn is_astm_compliant(inp: serialport::SerialPortInfo) -> bool {
    let handle = serialport::new(inp.port_name, 9600)
        .timeout(Duration::from_secs(30))
        .data_bits(serialport::DataBits::Eight)
        .flow_control(serialport::FlowControl::Software)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .open()
        .expect("failed to open port");
    // check_astm_implementation(handle).await;
    read_and_print_data(handle).await;
    return true;
}

async fn check_astm_implementation(mut inp: Box<dyn serialport::SerialPort>) -> bool {
    let mut serial_buf = [0];
    let mut cloned_handle = inp.try_clone().expect("Failed to clone handle");
    let read_hd = tokio::task::spawn(async move {
        loop {
            match inp.read(serial_buf.as_mut_slice()) {
                Ok(size) => {
                    if size > 0 {
                        println!("data received is {} long", size);
                        println!("data received is {:#?}", serial_buf);
                        let data_received = String::from_utf8_lossy(&serial_buf);
                        println!("Data received is {}", data_received);
                        break;
                    };
                }
                Err(k) => {
                    println!("{:#?}", k);
                    break;
                }
            }
        }
    });

    let temp = cloned_handle.write(&ENQ).expect("Failed to send data");
    println!("Sent {} bytes succesfully", temp);
    read_hd.await.unwrap();
    return true;
}

// create a channel that can communicate and inform if a new port is available
// if a new port is available we will safely pause current comms and then give
// an exclusive lock to the data to add the new serial port

// instead of this check if serial port is ready to send data
pub async fn read_and_print_data(mut handle: Box<dyn serialport::SerialPort>) {
    let h = tokio::task::spawn(async move {
        let mut input_buf = [0; 1000];
        // let mut line: Vec<u8> = Vec::with_capacity(1000);
        let mut interim_buffer = Vec::with_capacity(1000);
        tokio::time::sleep(Duration::from_millis(50)).await;
        loop {
                match handle.read(& mut input_buf[..]) {
                    Ok(size) => {if size > 0 {
                        let data_received = &input_buf[0..size];
                        if let Some(x) = data_received.last() {
                            match x {
                                &5 => {handle.write(&ACK).expect("Failed to send ack for enquiry");},
                                &10 => {
                                    for item in data_received.clone() {
                                        interim_buffer.push(item.clone());
                                    }
                                    println!("line created so far is {:?}", String::from_utf8_lossy(&interim_buffer[..]));
                                    handle.write(&ACK).expect("Failed to send ack after receiving data");
                                },
                                _ => {
                                    // println!("data received is:");
                                    println!("last character received is {:#?}", x);
                                    // println!("{:#?}", data_received);
                                    let data = data_received.clone();
                                    for item in data {
                                        interim_buffer.push(item.clone());
                                    }
                                    // handle.write(&NAK).expect("Failed to receive complete data requesting data");
                                }
                            }
                            // println!("Data received is {:?}, adding only 0th: {}", data_received, data_received[0]);
                            // line.push(data_received[0].clone());
                        }
                    }},
                    Err(e) => {
                        eprintln!("{}",e);
                    }
                }
            tokio::time::sleep(Duration::from_millis(50)).await;
            }
    });
    h.await;
}