#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use crossbeam_channel::{Receiver, Sender};
use eframe::{egui};
use serialport::{SerialPort, SerialPortType};
use std::io::{BufRead, BufReader, Write};

use std::thread;
use std::time::Duration;


mod digimic;
mod egui_log;

#[derive(PartialEq, Debug)]
pub enum Commands {
    SendCommand(String),
    Response(String),
    CurrentReading(f64),
    ContinuousToggle(bool),
    SerialNumber(i64),
    MicrometerType(digimic::MicrometerSize),
}

fn main() -> Result<(), eframe::Error> {
    let _ = egui_log::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 500.0)),
        resizable: false,
        ..Default::default()
    };

    let ch340 = serialport::available_ports()
        .unwrap()
        .into_iter()
        .find(|port| match &port.port_type {
            SerialPortType::UsbPort(info) => {
                info.product.as_ref().map_or(false, |p| p.contains("CH340"))
            }
            _ => false,
        });

    let (gui_sender, serial_recv) = crossbeam_channel::unbounded::<Commands>();
    let (serial_sender, gui_recv) = crossbeam_channel::unbounded::<Commands>();

    let serial_port = serialport::new(ch340.unwrap().port_name, 4800)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Seven)
        .parity(serialport::Parity::Even)
        .timeout(Duration::from_millis(5))
        .open()
        .unwrap();

    let mut serial_thread = SerialThread::new(serial_port, serial_sender, serial_recv);

    thread::spawn(move || serial_thread.start());

    eframe::run_native(
        "Digimic-rs Console/Configurator",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::new(digimic::DigimicApp::new(gui_sender, gui_recv))
        }),
    )
}

struct SerialThread {
    serial_port: Box<dyn SerialPort>,
    serial_sender: Sender<Commands>,
    serial_recv: Receiver<Commands>,
    continuous_mode: bool,
}

impl SerialThread {
    pub fn new(
        serial_port: Box<dyn SerialPort>,
        serial_sender: Sender<Commands>,
        serial_recv: Receiver<Commands>,
    ) -> Self {
        Self {
            serial_port,
            serial_recv,
            serial_sender,
            continuous_mode: false,
        }
    }

    pub fn start(&mut self) {
        let cloned_port = self.serial_port.try_clone().unwrap();
        let sender = self.serial_sender.clone();
        thread::spawn(|| serial_port_to_logger(cloned_port, sender));

        loop {
            //command handling loop i guess
            match self.serial_recv.try_recv() {
                Ok(Commands::SendCommand(command)) => {
                    let command = command + "\r";
                    self.serial_port.write(command.as_bytes());
                }
                _ => (),
            }
        }
    }
}

fn serial_port_to_logger(serial_port: Box<dyn SerialPort>, serial_sender: Sender<Commands>) {
    let mut reader = BufReader::new(serial_port);
    loop {
        reader.get_mut().write("?\r".as_bytes());
        reader.get_mut().flush();

        let mut buffer = Vec::new();
        reader.read_until(b'd', &mut buffer);
        buffer.pop();

        if let Ok(string) = std::str::from_utf8(&*buffer) {
            if string.contains("HCT") {
                if let Some(reading) = string.get(6..9) {
                    let mic_type = match reading {
                        "025" => Commands::MicrometerType(digimic::MicrometerSize::Range0to25),
                        "050" => Commands::MicrometerType(digimic::MicrometerSize::Range25to50),
                        "075" => Commands::MicrometerType(digimic::MicrometerSize::Range50to75),
                        "100" => Commands::MicrometerType(digimic::MicrometerSize::Range75to100),
                        _ => Commands::MicrometerType(digimic::MicrometerSize::Range0to25),
                    };
                    
                    serial_sender.send(mic_type);
                }
            }

            if string.contains("SN") {
                if let Some(reading) = string.get(2..string.len()).and_then(|sn| sn.parse::<i64>().ok()) {
                    serial_sender.send(Commands::SerialNumber(reading));
                }
            }
            if let Some(reading) = string
                .get(1..8)
                .and_then(|floaty| floaty.parse::<f64>().ok())
            {
                serial_sender.send(Commands::CurrentReading(reading));
            } else {
                if !string.is_empty() {
                    log::info!("{string}");
                }
            }
            
        }

        std::thread::sleep(Duration::from_millis(50));
    }
}
/*



use serialport;
use serialport::SerialPortType;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Duration;

fn main() {
    let ch340 = serialport::available_ports()
        .unwrap()
        .into_iter()
        .find(|port| match &port.port_type {
            SerialPortType::UsbPort(info) => {
                info.product.as_ref().map_or(false, |p| p.contains("CH340"))
            }
            _ => false,
        });

    let mut serial_port = serialport::new(ch340.unwrap().port_name, 4800)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Seven)
        .parity(serialport::Parity::Even)
        .timeout(Duration::from_millis(5))
        .open()
        .unwrap();

    let mut reader = BufReader::new(serial_port);

    loop {
        reader.get_mut().write("?\r".as_bytes());
        reader.get_mut().flush();

        let mut buffer = Vec::new();
        reader.read_until(b'd', &mut buffer);
        buffer.pop();
        println!("{:x?}", buffer);
        println!("{:?}", std::str::from_utf8(&buffer));
        std::thread::sleep(Duration::from_millis(100));
        buffer.clear();

        /*
        serial_port.get_mut().write("?\r".as_bytes());
        serial_port.read(&mut buffer);
        println!("{:x?}", buffer);
        println!("{:?}", std::str::from_utf8(&buffer));
        std::thread::sleep(Duration::from_millis(100));
        */
    }
}
*/
