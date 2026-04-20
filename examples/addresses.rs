fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for port in ports {
        println!("Found port: {}", port.port_name);
    }
}
