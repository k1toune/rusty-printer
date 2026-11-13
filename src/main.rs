
// Rusty Printer
// Made by K1toune
// ──────────────────
//     K1T0UNE <3

use eframe::egui;
use printers::{get_default_printer, get_printers};

struct ClxApp {
    clx_printers: Vec<String>,
    clx_selected: usize,
    clx_file: Option<std::path::PathBuf>,
    clx_status: String,
}

impl Default for ClxApp {
    fn default() -> Self {
        let clx_printers = get_printers()
            .iter()
            .map(|clx_prn| format!("{:?}", clx_prn))
            .collect();

        Self {
            clx_printers,
            clx_selected: 0,
            clx_file: None,
            clx_status: "Ready to print.".to_string(),
        }
    }
}

impl eframe::App for ClxApp {
    fn update(
        &mut self,
        clx_ctx: &egui::Context,
        _clx_frame: &mut eframe::Frame,
    ) {
        egui::CentralPanel::default().show(clx_ctx, |clx_ui| {
            clx_ui.heading("Rusty Printer");
            clx_ui.label("A simple printing utility by K1toune.");

            clx_ui.add_space(20.0);

            clx_ui.heading("Printer");

            if self.clx_printers.is_empty() {
                clx_ui.label("No printers found.");
            } else {
                egui::ComboBox::from_id_salt("clx_printer")
                    .selected_text(
                        self.clx_printers
                            .get(self.clx_selected)
                            .map(String::as_str)
                            .unwrap_or("Select printer"),
                    )
                    .show_ui(clx_ui, |clx_ui| {
                        for (clx_idx, clx_prn) in
                            self.clx_printers.iter().enumerate()
                        {
                            clx_ui.selectable_value(
                                &mut self.clx_selected,
                                clx_idx,
                                clx_prn,
                            );
                        }
                    });
            }

            clx_ui.add_space(20.0);

            clx_ui.heading("Document");

            if let Some(clx_path) = &self.clx_file {
                clx_ui.label(clx_path.display().to_string());
            } else {
                clx_ui.label("No document selected.");
            }

            if clx_ui.button("Browse files").clicked() {
                if let Some(clx_path) = rfd::FileDialog::new().pick_file() {
                    self.clx_file = Some(clx_path);
                }
            }

            clx_ui.add_space(20.0);

            let clx_can_print =
                self.clx_file.is_some() && !self.clx_printers.is_empty();

            if clx_ui
                .add_enabled(clx_can_print, egui::Button::new("Print document"))
                .clicked()
            {
                if let Some(clx_path) = &self.clx_file {
                    if let Some(clx_prn) = get_default_printer() {
                        match clx_prn.print_file(
                            clx_path,
                            Default::default(),
                        ) {
                            Ok(clx_job) => {
                                self.clx_status =
                                    format!("Print job submitted: {:?}", clx_job);
                            }
                            Err(clx_err) => {
                                self.clx_status =
                                    format!("Printing failed: {:?}", clx_err);
                            }
                        }
                    } else {
                        self.clx_status =
                            "No default printer configured.".to_string();
                    }
                }
            }

            clx_ui.add_space(15.0);
            clx_ui.separator();

            clx_ui.label(&self.clx_status);

            clx_ui.with_layout(
                egui::Layout::bottom_up(egui::Align::LEFT),
                |clx_ui| {
                    clx_ui.label("Made by K1toune");
                },
            );
        });
    }
}

fn main() -> eframe::Result<()> {
    let clx_args: Vec<String> = std::env::args().collect();

    match clx_args.get(1).map(String::as_str) {
        Some("list") => {
            for clx_prn in get_printers() {
                println!("{:?}", clx_prn);
            }
            Ok(())
        }

        Some("print") => {
            let Some(clx_file) = clx_args.get(2) else {
                eprintln!("Usage: rusty-printer print <file>");
                return Ok(());
            };

            let Some(clx_prn) = get_default_printer() else {
                eprintln!("No default printer configured.");
                return Ok(());
            };

            match clx_prn.print_file(clx_file, Default::default()) {
                Ok(clx_job) => println!("Print job submitted: {:?}", clx_job),
                Err(clx_err) => eprintln!("Printing failed: {:?}", clx_err),
            }

            Ok(())
        }

        _ => {
            let clx_options = eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default()
                    .with_inner_size([480.0, 520.0])
                    .with_min_inner_size([400.0, 450.0])
                    .with_title("Rusty Printer"),
                ..Default::default()
            };

            eframe::run_native(
                "Rusty Printer",
                clx_options,
                Box::new(|_clx_cc| Ok(Box::new(ClxApp::default()))),
            )
        }
    }
}
