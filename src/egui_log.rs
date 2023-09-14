use std::sync::Mutex;

use chrono::Local;
use egui::Color32;
use log::SetLoggerError;

use regex::Regex;

const LEVELS: [log::Level; log::Level::Trace as usize] = [
    log::Level::Error,
    log::Level::Warn,
    log::Level::Info,
    log::Level::Debug,
    log::Level::Trace,
];

struct EguiLogger;

impl log::Log for EguiLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::STATIC_MAX_LEVEL
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            try_mut_log(|logs| {
                logs.push((
                    Local::now().format("%H:%M:%S").to_string(),
                    record.level(),
                    record.args().to_string(),
                ))
            });
        }
    }

    fn flush(&self) {}
}

/// Initilizes the global logger.
/// Should be called very early in the program
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&EguiLogger).map(|()| log::set_max_level(log::LevelFilter::Info))
}

type GlobalLog = Vec<(String, log::Level, String)>;

static LOG: Mutex<GlobalLog> = Mutex::new(Vec::new());

static LOGGER_UI: once_cell::sync::Lazy<Mutex<LoggerUi>> =
    once_cell::sync::Lazy::new(Default::default);

pub fn try_mut_log<F, T>(f: F) -> Option<T>
where
    F: FnOnce(&mut GlobalLog) -> T,
{
    match LOG.lock() {
        Ok(ref mut global_log) => Some((f)(global_log)),
        Err(_) => None,
    }
}

fn try_get_log<F, T>(f: F) -> Option<T>
where
    F: FnOnce(&GlobalLog) -> T,
{
    match LOG.lock() {
        Ok(ref global_log) => Some((f)(global_log)),
        Err(_) => None,
    }
}

struct LoggerUi {
    loglevels: [bool; log::Level::Trace as usize],
    search_term: String,
    regex: Option<Regex>,
    search_case_sensitive: bool,
    search_use_regex: bool,
    max_log_length: usize,
}

impl Default for LoggerUi {
    fn default() -> Self {
        Self {
            loglevels: [true, true, true, false, false],
            search_term: String::new(),
            search_case_sensitive: false,
            regex: None,
            search_use_regex: false,
            max_log_length: 1000,
        }
    }
}

impl LoggerUi {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        try_mut_log(|logs| {
            let dropped_entries = logs.len().saturating_sub(self.max_log_length);
            drop(logs.drain(..dropped_entries));
        });

        let mut logs_displayed: usize = 0;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .enable_scrolling(false)
            .show(ui, |ui| {
                try_get_log(|logs| {
                    logs.iter().rev().for_each(|(timestamp, level, string)| {
                        if (!self.search_term.is_empty() && !self.match_string(string))
                            || !(self.loglevels[*level as usize - 1])
                        {
                            return;
                        }

                        ui.horizontal(|ui| {
                            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
                            ui.colored_label(Color32::GRAY, format!("{}", timestamp));
                            match level {
                                log::Level::Info => ui
                                    .colored_label(Color32::LIGHT_GREEN, format!(" [{}]: ", level)),
                                log::Level::Warn => ui.colored_label(
                                    Color32::LIGHT_YELLOW,
                                    format!(" [{}]: ", level),
                                ),
                                log::Level::Error => {
                                    ui.colored_label(Color32::LIGHT_RED, format!(" [{}]: ", level))
                                }
                                _ => ui.label(format!("{}", level)),
                            };
                            ui.colored_label(Color32::WHITE, format!(" {}", string));
                        });

                        logs_displayed += 1;
                    });
                    //ui.debug_paint_cursor();
                });
            });
    }

    fn match_string(&self, string: &str) -> bool {
        if self.search_use_regex {
            if let Some(matcher) = &self.regex {
                matcher.is_match(string)
            } else {
                false
            }
        } else if self.search_case_sensitive {
            string.contains(&self.search_term)
        } else {
            string
                .to_lowercase()
                .contains(&self.search_term.to_lowercase())
        }
    }
}

/// Draws the logger ui
/// has to be called after [`init()`](init());
pub fn logger_ui(ui: &mut egui::Ui) {
    if let Ok(ref mut logger_ui) = LOGGER_UI.lock() {
        logger_ui.ui(ui);
    } else {
        ui.colored_label(Color32::RED, "Something went wrong loading the log");
    }
}
