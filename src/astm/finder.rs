// this module will ensure serialports connected are available for communication

pub fn list_ports() {
    let res = serialport::available_ports().unwrap();
    println!("{:#?}", res);
}