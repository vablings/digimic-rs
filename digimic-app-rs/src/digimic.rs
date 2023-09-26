use crossbeam_channel::{Receiver, Sender};



use crate::{egui_log, Commands};

#[derive(PartialEq, Debug)]
pub enum MicrometerSize {
    Range0to25,
    Range25to50,
    Range50to75,
    Range75to100,
}
impl MicrometerSize {
    fn max_travel(self) -> i32 {
        match self {
            MicrometerSize::Range0to25 => 26,
            MicrometerSize::Range25to50 => 0,
            MicrometerSize::Range50to75 => 0,
            MicrometerSize::Range75to100 => 0,
        }
    }
}
pub struct DigimicApp {
    gui_sender: Sender<Commands>,
    gui_recv: Receiver<Commands>,
    console_input: String,
    serial_number: String,
    date: Option<chrono::NaiveDate>,
    lin_correction: i32,
    micrometer_size: MicrometerSize,
    last_reading: f64,
    stream_mode: bool,
}
impl DigimicApp {
    pub fn new(gui_sender: Sender<Commands>, gui_recv: Receiver<Commands>) -> DigimicApp {
        DigimicApp {
            gui_sender,
            gui_recv,
            console_input: "".to_string(),
            serial_number: "".to_string(),
            date: None,
            lin_correction: 0,
            micrometer_size: MicrometerSize::Range0to25,
            last_reading: 0.0,
            stream_mode: false,
        }
    }

    fn side_panel_left(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("Console")
            .resizable(false)
            .exact_width(300.0)
            .show(ctx, |ui| {
                if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.send_command(self.console_input.clone());
                    self.console_input.clear();
                }
                ui.with_layout(egui::Layout::bottom_up(egui::Align::TOP), |ui| {
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.console_input)
                                .hint_text("Input a command"),
                        );

                        if ui.button("Submit").clicked() {
                            self.send_command(self.console_input.clone());
                            self.console_input.clear();
                        }
                        if ui.button("Clear").clicked() {
                            self.console_input.clear();
                            egui_log::try_mut_log(|logs| logs.clear());
                        }
                    });
                    egui_log::logger_ui(ui);
                });
            });
    }

    fn central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Settings");
            });
            ui.separator();

            ui.horizontal(|ui| { 
                ui.heading(format!("{:0>7.4}",self.last_reading));
            });

            let response = ui.add(
                egui::Slider::new(&mut self.lin_correction, -200..=200).text("Lin Correction Value"),
            );
            if response.drag_released() || response.lost_focus() {
                self.send_command(format!("LIN {}", self.lin_correction))
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.serial_number)
                        .hint_text("Enter the S/N").desired_width(100.0),
                );


                if ui.button("Submit").clicked() {
                    self.send_command(format!("FACCFGSN {}", self.serial_number));
                }
                if ui.button("Verify").clicked() {
                    self.send_command(format!("SN?"));
                }
            });
            ui.horizontal(|ui| {
                let date = self.date.get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
                ui.add(egui_extras::DatePickerButton::new(date));
                if ui.button("Submit").clicked() {
                    let fmt = self.date.unwrap().format("%d.%m.%Y").to_string();
                    self.send_command(format!("LCAL {}", fmt));
                }
                if ui.button("Verify").clicked() {
                    self.send_command(format!("LCAL?"));
                }
            });
            ui.separator();

            ui.horizontal(|ui| {
                if ui.radio_value(
                    &mut self.micrometer_size,
                    MicrometerSize::Range0to25,
                    "0-25mm",
                ).clicked() {
                    self.send_command(format!("BTDN HCT-MM025"));
                };
                if ui.button("Verify").clicked() {
                    self.send_command(format!("BTDN?"));
                }
            });
            if ui.radio_value(
                &mut self.micrometer_size,
                MicrometerSize::Range25to50,
                "25-50mm",
            ).clicked() {
                self.send_command(format!("BTDN HCT-MM050"));
            };
            if ui.radio_value(
                &mut self.micrometer_size,
                MicrometerSize::Range50to75,
                "50-75mm",
            ).clicked() {
                self.send_command(format!("BTDN HCT-MM075"));
            };
            if ui.radio_value(
                &mut self.micrometer_size,
                MicrometerSize::Range75to100,
                "75-100mm",
            ).clicked() {
                self.send_command(format!("BTDN HCT-MM100"));
            };

            ui.separator();
            ui.heading("Misc buttons");
            ui.horizontal(|ui| { 
                if ui.button("Reference").clicked() {
                    self.send_command(format!("REF"));
                }
                if ui.button("Set").clicked() {
                    self.send_command(format!("SET"));
                }
            });
            ui.horizontal(|ui| { 
                if ui.button("Calibration On").clicked() {
                    self.send_command(format!("CAL ON"));
                }
                if ui.button("Calibration Off").clicked() {
                    self.send_command(format!("CAL OFF"));
                }
            });

            if ui.button("Reset").clicked() {
                self.send_command(format!("RST"));
            }


        });
    }

    fn send_command(&mut self, command: String) {
        if !command.is_empty() {
            log::info!("Sending {}", command);
            self.gui_sender.send(Commands::SendCommand(command));
        }
    }
}

impl eframe::App for DigimicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.gui_sender
            .send(Commands::ContinuousToggle(self.stream_mode));
        if let Ok(serial_send_me_a_job) = self.gui_recv.try_recv() {
            match serial_send_me_a_job {
                Commands::CurrentReading(reading) => self.last_reading = reading,
                Commands::SerialNumber(serial_number) => {
                    self.serial_number = serial_number.to_string()
                }
                _ => (),
            }
        }




        ctx.request_repaint();
        self.side_panel_left(ctx);
        self.central_panel(ctx);
    }
}
