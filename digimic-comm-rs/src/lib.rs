use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use data::CommandKind;
use serialport::SerialPort;
use serialport::SerialPortType;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::thread;
use std::{error::Error, time::Duration};

mod data;


struct Digimic {
    serial_port: Box<dyn SerialPort>,
    sender: Sender<CommandKind>,
    reciver: Receiver<CommandKind>
}

impl Digimic {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let ch340 = serialport::available_ports()?
            .into_iter()
            .find(|port| match &port.port_type {
                SerialPortType::UsbPort(info) => {
                    info.product.as_ref().map_or(false, |p| p.contains("CH340"))
                }
                _ => false,
            });

        let (s, r) = crossbeam_channel::unbounded::<CommandKind>();


        let serial_port = serialport::new(ch340.unwrap().port_name, 4800)
            .stop_bits(serialport::StopBits::One)
            .data_bits(serialport::DataBits::Seven)
            .parity(serialport::Parity::Even)
            .timeout(Duration::from_millis(5))
            .open()?;
        Ok(Digimic {
            serial_port: serial_port,
            sender: s,
            reciver: r,
        })
    }

    pub fn start(&mut self) {


        std::thread::spawn(move || { reciver(self.serial_port.try_clone().unwrap(), self.reciver.clone()) } );
        //std::thread::spawn(self.reader());
    }
    
}


fn reciver(serial_port: Box<dyn SerialPort>, reciver:Receiver<CommandKind>) { 
    loop {
        let recv = reciver.recv();
        if let Ok(recv) = recv {
            log::debug!("{:?}",recv);
        }
    }
}

fn reader(serial_port: Box<dyn SerialPort>, sender: Sender<CommandKind>) {
    let mut reader = BufReader::new(self.serial_port.try_clone().unwrap());
    let mut buffer = Vec::new();
    loop {
        reader.read_until(b'd', &mut buffer).unwrap();
        buffer.pop();
        let parsed_command = data::CommandKind::from_str(std::str::from_utf8(& *buffer).unwrap()).unwrap();
        log::info!("{}", parsed_command);
        self.sender.send(parsed_command);
    }
}

