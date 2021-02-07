
use serialport::{available_ports, SerialPortType};
use std::collections::HashMap;
use clap::{App, AppSettings, Arg};
use std::io::BufRead;
use std::time::Duration;


const  MIN_BAUD_RATE:u32 = 115200;


fn main() {
  eprintln!("fanalog");

  let mut serial_buf: Vec<u8> = vec![0; 1024];

  let mut active_ports = HashMap::new();
  loop {
    // TODO check for newly-available ports
    if let Ok(avail_ports) = available_ports() {
        let mut cur_avail_ports = HashMap::new();
        // detect any newly-inserted devices and add to our list of active ports
        for port in avail_ports {
          let new_port_name = port.port_name.clone();
          cur_avail_ports.insert(new_port_name, 0);
          if !active_ports.contains_key(&port.port_name) {
            eprintln!("inserted: {}",port.port_name );

            let attach_port = serialport::new(port.port_name, MIN_BAUD_RATE)
              .timeout(Duration::from_millis(1))
              .open();
            if let Ok(opened_port) = attach_port {
              active_ports.insert(&port.port_name, &opened_port);
            }
          }
        }
        // remove any ports where devices have disconnected
        for (port_name, _port_info) in &active_ports  {
            if !cur_avail_ports.contains_key(port_name) {
              //TODO close the port first?
              eprintln!("removed: {}", port_name);
              active_ports.remove_entry(&port_name);
            }
        }

        for (port_name, mut port_info) in &active_ports {
          if let Ok(log_line) =  port_info.read(serial_buf.as_mut_slice()) {
            println!("{} {}", port_name, log_line);
          }
        }
    }

  }

/*
    match available_ports() {
        Ok(ports) => {
            match ports.len() {
                0 => println!("No ports found."),
                1 => println!("Found 1 port:"),
                n => println!("Found {} ports:", n),
            };
            for p in ports {
                println!("  {}", p.port_name);
                match p.port_type {
                    SerialPortType::UsbPort(info) => {
                        println!("    Type: USB");
                        println!("    VID:{:04x} PID:{:04x}", info.vid, info.pid);
                        println!(
                            "     Serial Number: {}",
                            info.serial_number.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "      Manufacturer: {}",
                            info.manufacturer.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "           Product: {}",
                            info.product.as_ref().map_or("", String::as_str)
                        );
                    }
                    SerialPortType::BluetoothPort => {
                        println!("    Type: Bluetooth");
                    }
                    SerialPortType::PciPort => {
                        println!("    Type: PCI");
                    }
                    SerialPortType::Unknown => {
                        println!("    Type: Unknown");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
            eprintln!("Error listing serial ports");
        }
    }

  */

}
