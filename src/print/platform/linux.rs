use crate::print::{PrintError, PrintSettings, Printer, PrinterInfo, Result};
use std::path::PathBuf;

pub struct LinuxPrinter;

impl LinuxPrinter {
    fn print_pdf_native(
        pdf_path: &PathBuf,
        settings: &PrintSettings,
        printer_name: Option<&str>,
    ) -> Result<()> {
        use std::process::Command;

        // Get printer name
        let printer = if let Some(name) = printer_name {
            name.to_string()
        } else {
            // Get default printer using lpstat
            let output = Command::new("lpstat")
                .args(&["-d"])
                .output()
                .ok()
                .and_then(|o| {
                    let s = String::from_utf8_lossy(&o.stdout);
                    s.find("destination: ")
                        .map(|pos| s[pos + 12..].trim().to_string())
                })
                .ok_or(PrintError::NoPrinter)?;
            output
        };

        // Build lp command with native options
        let mut cmd = Command::new("lp");
        cmd.arg("-d").arg(&printer);

        if settings.copies > 1 {
            cmd.arg("-n").arg(settings.copies.to_string());
        }

        if let Some(ref range) = settings.page_range {
            let range_str = format!("{}-{}", range.start + 1, range.end + 1);
            cmd.arg("-P").arg(range_str);
        }

        if settings.duplex {
            cmd.arg("-o").arg("sides=two-sided-long-edge");
        } else {
            cmd.arg("-o").arg("sides=one-sided");
        }

        if settings.color {
            cmd.arg("-o").arg("print-color-mode=color");
        } else {
            cmd.arg("-o").arg("print-color-mode=monochrome");
        }

        match settings.orientation {
            crate::print::Orientation::Portrait => cmd.arg("-o").arg("orientation-requested=3"),
            crate::print::Orientation::Landscape => cmd.arg("-o").arg("orientation-requested=4"),
        };

        let media = match settings.paper_size {
            crate::print::PaperSize::A4 => "A4",
            crate::print::PaperSize::A3 => "A3",
            crate::print::PaperSize::A5 => "A5",
            crate::print::PaperSize::Letter => "Letter",
            crate::print::PaperSize::Legal => "Legal",
            crate::print::PaperSize::Tabloid => "Tabloid",
        };
        cmd.arg("-o").arg(format!("media={}", media));

        cmd.arg(pdf_path);

        let output = cmd
            .output()
            .map_err(|e| PrintError::PrintError(format!("Failed to execute lp command: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PrintError::PrintError(format!(
                "lp command failed: {}",
                stderr
            )));
        }

        Ok(())
    }
}

impl Printer for LinuxPrinter {
    fn get_printers() -> Result<Vec<PrinterInfo>> {
        use std::process::Command;

        let output = Command::new("lpstat")
            .args(&["-a"])
            .output()
            .map_err(|e| PrintError::InitError(format!("Failed to list printers: {}", e)))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let default_printer = Command::new("lpstat")
            .args(&["-d"])
            .output()
            .ok()
            .and_then(|o| {
                let s = String::from_utf8_lossy(&o.stdout);
                s.find("destination: ")
                    .map(|pos| s[pos + 12..].trim().to_string())
            });

        let mut printers = Vec::new();

        for line in output_str.lines() {
            if let Some(pos) = line.find(' ') {
                let name = &line[..pos];
                if !name.is_empty() {
                    let is_default = default_printer.as_ref().map(|d| d == name).unwrap_or(false);
                    printers.push(PrinterInfo {
                        name: name.to_string(),
                        is_default,
                        supports_color: true,
                        supports_duplex: true,
                    });
                }
            }
        }

        if printers.is_empty() {
            return Err(PrintError::NoPrinter);
        }

        Ok(printers)
    }

    fn print_pdf(
        pdf_path: &PathBuf,
        settings: &PrintSettings,
        printer_name: Option<&str>,
    ) -> Result<()> {
        Self::print_pdf_native(pdf_path, settings, printer_name)
    }

    fn show_print_dialog(pdf_path: &PathBuf) -> Result<()> {
        use std::process::Command;

        // Try xdg-open first to open with default PDF viewer
        let result = Command::new("xdg-open").arg(pdf_path).spawn();

        if result.is_ok() {
            return Ok(());
        }

        // Fallback to lp with default settings
        let settings = PrintSettings::default();
        Self::print_pdf(pdf_path, &settings, None)
    }

    fn show_print_preview(pdf_path: &PathBuf) -> Result<()> {
        use std::process::Command;

        // Open PDF in default viewer which has print preview capability
        let result = Command::new("xdg-open").arg(pdf_path).spawn();

        if result.is_err() {
            // Fallback: try evince
            let _ = Command::new("evince").arg(pdf_path).spawn();
        }

        Ok(())
    }
}

pub use LinuxPrinter as PlatformPrinter;
