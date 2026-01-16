/*
 *  _       ___                 ____  __  _______     _________                      _ 
 * | |     / (_)___  __________/ __ \/ / / /  _/ \   / / ____/__  ____  ___  _____(_)
 * | | /| / / / __ \/ ___/ ___/ / / / / / // // _ \ / / / __/ _ \/ __ \/ _ \/ ___/ / 
 * | |/ |/ / / / / (__  ) /__/ /_/ / /_/ // // /_/ / / /_/ /  __/ / / /  __/ /  / /  
 * |__/|__/_/_/ /_/____/\___/_____/\____/___/____/____/\____/\___/_/ /_/\___/_/  /_/   
 * 
 * "Legacy Protocol Override Tool"
 */

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console in release

use eframe::egui;
use uuid::Uuid;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 350.0])
            .with_resizable(false)
            .with_maximize_button(false)
            .with_decorations(true), // Keep standard window frame for simplicity, or could go full custom
        ..Default::default()
    };

    eframe::run_native(
        "SYSTEM_ROOT_ACCESS // GUID_RESET_TOOL",
        options,
        Box::new(|cc| {
            configure_retro_style(&cc.egui_ctx);
            Ok(Box::new(MyApp::new()))
        }),
    )
}

struct MyApp {
    current_guid: String,
    status_terminal: Vec<String>, // Log lines
    is_admin: bool,
}

impl MyApp {
    fn new() -> Self {
        let is_admin = check_admin_access();
        let mut app = Self {
            current_guid: "PENDING_SCAN...".to_owned(),
            status_terminal: vec![
                "> SYSTEM_INIT... OK".to_owned(),
                "> CHECKING_PRIVILEGES...".to_owned(),
            ],
            is_admin,
        };

        if is_admin {
            app.status_terminal
                .push("> ACCESS_LEVEL: ADMINISTRATOR [GRANTED]".to_owned());
            app.fetch_guid();
        } else {
            app.status_terminal
                .push("> ACCESS_LEVEL: RESTRICTED [DENIED]".to_owned());
            app.status_terminal
                .push("> CRITICAL: ELEVATION REQUIRED".to_owned());
        }

        app
    }

    fn fetch_guid(&mut self) {
        match read_machine_guid() {
            Ok(guid) => {
                self.current_guid = guid;
                self.status_terminal
                    .push("> TARGET_ACQUIRED: HKLM\\...\\MachineGuid".to_owned());
            }
            Err(e) => {
                self.current_guid = "ERROR".to_owned();
                self.status_terminal.push(format!("> READ_ERR: {}", e));
            }
        }
    }

    fn reset_guid(&mut self) {
        if !self.is_admin {
            return;
        }

        self.status_terminal
            .push("> INITIATING_RESET_SEQUENCE...".to_owned());
        let new_uuid = Uuid::new_v4().to_string();

        match write_machine_guid(&new_uuid) {
            Ok(_) => {
                self.current_guid = new_uuid;
                self.status_terminal
                    .push(format!("> WRITE_SUCCESS: {}", self.current_guid));
                self.status_terminal
                    .push("> IDENTITY_REFRESH: COMPLETE".to_owned());
            }
            Err(e) => {
                self.status_terminal.push(format!("> WRITE_FAILURE: {}", e));
                self.status_terminal.push("> ABORTING OPERATION".to_owned());
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Enforce 1s repaint for blinking cursor effect if needed, but standard events are fine

        if !self.is_admin {
            self.render_access_denied(ctx);
            return;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("/// SYSTEM OVERRIDE TERMINAL ///")
                        .monospace()
                        .size(16.0)
                        .color(egui::Color32::from_rgb(0, 255, 0)),
                ); // Bright Green
                ui.add_space(5.0);
                ui.separator();
            });
            ui.add_space(15.0);

            // Main Info Block
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("TARGET_PARAMETER: MachineGuid")
                            .color(egui::Color32::from_rgb(0, 150, 0))
                            .monospace()
                            .size(10.0),
                    );
                    ui.add_space(2.0);

                    // The GUID Display
                    ui.label(
                        egui::RichText::new(&self.current_guid)
                            .color(egui::Color32::from_rgb(0, 255, 0))
                            .monospace()
                            .size(18.0),
                    );
                });
            });

            ui.add_space(20.0);

            // Terminal Output
            ui.label(
                egui::RichText::new("LOG_OUTPUT:")
                    .size(10.0)
                    .color(egui::Color32::GRAY),
            );
            egui::ScrollArea::vertical()
                .max_height(100.0)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    for line in &self.status_terminal {
                        ui.label(
                            egui::RichText::new(line)
                                .monospace()
                                .size(12.0)
                                .color(egui::Color32::from_rgb(100, 255, 100)),
                        );
                    }
                });

            ui.add_space(20.0);

            // Action Button
            let btn = egui::Button::new(
                egui::RichText::new("[ CLICK TO RESET ]")
                    .monospace()
                    .size(16.0)
                    .strong(),
            )
            .min_size(egui::vec2(ui.available_width(), 40.0));

            if ui.add(btn).clicked() {
                self.reset_guid();
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("SECURE_CONNECTION: ENCRYPTED")
                        .size(9.0)
                        .color(egui::Color32::DARK_GRAY),
                );
            });
        });
    }
}

impl MyApp {
    fn render_access_denied(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let time = ctx.input(|i| i.time);
            let blink = (time * 2.0).sin() > 0.0;

            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.label(
                    egui::RichText::new("WARNING")
                        .size(40.0)
                        .monospace()
                        .strong()
                        .color(egui::Color32::RED),
                );

                ui.add_space(20.0);

                if blink {
                    ui.label(
                        egui::RichText::new("ACCESS DENIED")
                            .size(32.0)
                            .monospace()
                            .strong()
                            .background_color(egui::Color32::RED)
                            .color(egui::Color32::BLACK),
                    );
                } else {
                    ui.label(
                        egui::RichText::new("ACCESS DENIED")
                            .size(32.0)
                            .monospace()
                            .strong()
                            .color(egui::Color32::RED),
                    );
                }

                ui.add_space(40.0);

                ui.label(
                    egui::RichText::new("ADMINISTRATOR PRIVILEGES REQUIRED")
                        .monospace()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("EXECUTE AS ADMIN TO PROCEED")
                        .monospace()
                        .color(egui::Color32::WHITE),
                );

                ui.add_space(40.0);

                if ui.button("TERMINATE SESSION").clicked() {
                    std::process::exit(0);
                }
            });

            // Force continuous repaint for blink
            ctx.request_repaint();
        });
    }
}

fn configure_retro_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // Theme: Hacker Green
    let black = egui::Color32::BLACK;
    let green = egui::Color32::from_rgb(0, 255, 0);
    // let dark_green = egui::Color32::from_rgb(0, 50, 0);

    style.visuals.widgets.noninteractive.bg_fill = black;
    style.visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, green);

    style.visuals.widgets.active.bg_fill = green;
    style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, black);
    style.visuals.widgets.active.weak_bg_fill = green;

    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(0, 50, 0);
    style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, green);

    style.visuals.widgets.inactive.bg_fill = black;
    style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, green);

    style.visuals.window_fill = black;
    style.visuals.window_stroke = egui::Stroke::new(2.0, green); // Green border
    style.visuals.window_rounding = egui::Rounding::ZERO; // Sharp corners
    style.visuals.selection.bg_fill = green;
    style.visuals.selection.stroke = egui::Stroke::new(1.0, black);

    ctx.set_style(style);
}

// Logic Helpers
fn check_admin_access() -> bool {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    // Try to open with write access as the litmus test
    match hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Cryptography", KEY_ALL_ACCESS) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn read_machine_guid() -> anyhow::Result<String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let crypto = hklm.open_subkey("SOFTWARE\\Microsoft\\Cryptography")?;
    let val: String = crypto.get_value("MachineGuid")?;
    Ok(val)
}

fn write_machine_guid(new_guid: &str) -> anyhow::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    // Needs write access
    let crypto =
        hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Cryptography", KEY_ALL_ACCESS)?;
    crypto.set_value("MachineGuid", &new_guid)?;
    Ok(())
}
