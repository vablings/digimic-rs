#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::prelude::*;
use crossbeam_channel::{Sender, Receiver};
use eframe::{egui, glow::FALSE};
use serialport::{SerialPort, SerialPortInfo, SerialPortType, UsbPortInfo, COMPort};
use std::time::Duration;
use std::{
    thread,
};
use std::io::{Write, BufReader, BufRead};
use winput::{Vk, Button};
use std::str;

mod digimic;
mod egui_log;

#[derive(PartialEq, Debug)]
pub enum Commands {
    SendCommand(String),
    Response(String),
    CurrentReading(f64),
    ContinuousToggle(bool),
    SerialNumber(i64),
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
            SerialPortType::UsbPort(info) => info.product.as_ref().map_or(false, |p| p.contains("CH340")),
            _ => false,
        });

    


    let (gui_sender, serial_recv) = crossbeam_channel::unbounded::<Commands>();
    let (serial_sender, gui_recv) = crossbeam_channel::unbounded::<Commands>();

    let mut serial_port = serialport::new(ch340.unwrap().port_name, 4800)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Seven)
        .parity(serialport::Parity::Even).timeout(Duration::from_millis(5))
        .open().unwrap();

    let mut serial_thread = SerialThread::new(serial_port, serial_sender, serial_recv);
        
    thread::spawn(move || serial_thread.start() );


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
    pub fn new(serial_port: Box<dyn SerialPort>, serial_sender: Sender<Commands>, serial_recv: Receiver<Commands>) -> Self {
        Self {
            serial_port,
            serial_recv,
            serial_sender,
            continuous_mode: false,
        }
    }
    pub fn start(&mut self) {
        loop {
            match self.serial_recv.try_recv() {
                Ok(Commands::SendCommand(command)) => {
                    let command = command + "\r";
                    self.serial_port.write(command.as_bytes());
                },
                Ok(Commands::ContinuousToggle(T)) => {
                    self.continuous_mode = T;
                }
                _ => {
                },
            }

            let mut buffer = [0; 100];

            if self.continuous_mode {
                self.serial_port.write("\r".as_bytes());
            }

            match self.serial_port.read(&mut buffer[..]) {
                Ok(_) => match str::from_utf8(&buffer) {
                    Ok(string) => {

                        //self.serial_sender.send(Commands::SerialNumber(serial_number));
                        
                        if let Ok(size) = string[1..8].parse::<f64>() {
                            self.serial_sender.send(Commands::CurrentReading(size));
                            if !self.continuous_mode {
                                size.to_string().chars().into_iter().for_each(|char| {
                                    winput::send(char);
                                });
                                winput::send(Vk::Enter);
                            }
                        } else {
                            if !string.contains("ERR") {
                                log::info!("{}", string);
                            } 
                        }
                    },
                    Err(_) => (),
                }
                Err(_) => (),
            } 
        }

    }
}

