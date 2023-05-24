// this module will ensure serialports connected are available for communication
use tokio::time::Duration;

use super::std_messages::{ACK, ENQ, EOT, NAK};

pub async fn list_ports() {
    let res = tokio_serial::available_ports().unwrap();
    for port in res {
        is_astm_compliant(port).await;
    }
}

enum BaudRates {
    ElevenK,
    NineK,
}

// get all ports
pub async fn is_astm_compliant(inp: tokio_serial::SerialPortInfo) -> bool {
    let handle = tokio_serial::new(inp.port_name, 115200)
        .timeout(Duration::from_secs(30))
        .data_bits(tokio_serial::DataBits::Eight)
        .flow_control(tokio_serial::FlowControl::Software)
        .parity(tokio_serial::Parity::None)
        .stop_bits(tokio_serial::StopBits::One)
        .open()
        .expect("failed to open port");
    // check_astm_implementation(handle).await;
    read_and_print_data(handle).await;
    return true;
}

async fn check_astm_implementation(mut inp: Box<dyn tokio_serial::SerialPort>) -> bool {
    let mut serial_buf = [0];
    let mut cloned_handle = inp.try_clone().expect("Failed to clone handle");
    let read_handle = tokio::task::spawn(async move {
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
    read_handle.await.unwrap();
    return true;
}

// create a channel that can communicate and inform if a new port is available
// if a new port is available we will safely pause current comms and then give
// an exclusive lock to the data to add the new serial port

// instead of this check if serial port is ready to send data
pub async fn read_and_print_data(mut handle: Box<dyn tokio_serial::SerialPort>) {
    let h = tokio::task::spawn(async move {
        let mut input_buf = [0; 4000];
        // let mut line: Vec<u8> = Vec::with_capacity(1000);
        // let mut interim_buffer = Vec::with_capacity(64000);
        tokio::time::sleep(Duration::from_millis(100)).await;
        loop {
            let some = handle
                .bytes_to_read()
                .expect("failed to get bytes number to be read");
            tokio::time::sleep(Duration::from_millis(50)).await;
            let some2 = handle
                .bytes_to_read()
                .expect("failed to get bytes number to be read");
            if some2 == 0 {
                tokio::time::sleep(Duration::from_secs(2)).await;
            } else if some2 == some {
                println!("Ready to read {some} bytes of data");
                let data = handle.read(&mut input_buf[..]);
                match data {
                    Err(k) => {
                        eprintln!("error in reading data {:#?}", k);
                        continue;
                    }
                    Ok(k) => {
                        println!(
                            "Read data is {:#?}",
                            String::from_utf8_lossy(&input_buf[0..k])
                        );
                    }
                }
                handle.write(&ACK).expect("failed to send ack");
            } else if some2 > some {
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        }
    });
    let data = h.await;
    match data {
        Err(k) => eprintln!("error in reading data {:#?}", k),
        Ok(k) => println!("returned data is {:#?}", k),
    };
}
