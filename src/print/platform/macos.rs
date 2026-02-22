use crate::print::{PrintError, PrintSettings, Printer, PrinterInfo, Result};
use std::path::PathBuf;

#[cfg(target_os = "macos")]
use objc::runtime::Object;
#[cfg(target_os = "macos")]
use objc::{class, msg_send, sel, sel_impl};

pub struct MacOSPrinter;

impl MacOSPrinter {
    #[allow(dead_code)]
    fn print_pdf_native(
        pdf_path: &PathBuf,
        settings: &PrintSettings,
        printer_name: Option<&str>,
    ) -> Result<()> {
        use std::process::Command;

        let printer = if let Some(name) = printer_name {
            name.to_string()
        } else {
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

    #[cfg(target_os = "macos")]
    fn show_native_print_dialog(pdf_path: &PathBuf) -> Result<()> {
        use std::ffi::CString;

        let path_str = pdf_path
            .to_str()
            .ok_or_else(|| PrintError::PrintError("Invalid PDF path".to_string()))?;

        unsafe {
            let path_cstr = CString::new(path_str)
                .map_err(|_| PrintError::PrintError("Invalid path encoding".to_string()))?;

            let ns_string: *mut Object =
                msg_send![class!(NSString), stringWithUTF8String: path_cstr.as_ptr()];
            if ns_string.is_null() {
                return Err(PrintError::PrintError(
                    "Failed to create NSString".to_string(),
                ));
            }

            let ns_url: *mut Object = msg_send![class!(NSURL), fileURLWithPath: ns_string];
            if ns_url.is_null() {
                return Err(PrintError::PrintError("Failed to create NSURL".to_string()));
            }

            let pdf_doc: *mut Object = msg_send![class!(PDFDocument), alloc];
            let pdf_doc: *mut Object = msg_send![pdf_doc, initWithURL: ns_url];

            if pdf_doc.is_null() {
                return Err(PrintError::PrintError(
                    "Failed to create PDFDocument".to_string(),
                ));
            }

            let print_info: *mut Object = msg_send![class!(NSPrintInfo), sharedPrintInfo];

            let pdf_view: *mut Object = msg_send![class!(PDFView), alloc];
            let pdf_view: *mut Object = msg_send![pdf_view, init];

            if pdf_view.is_null() {
                let _: () = msg_send![pdf_doc, release];
                return Err(PrintError::PrintError(
                    "Failed to create PDFView".to_string(),
                ));
            }

            let _: () = msg_send![pdf_view, setDocument: pdf_doc];

            let print_op: *mut Object = msg_send![class!(NSPrintOperation), printOperationWithView: pdf_view
                                                                            printInfo: print_info];

            if print_op.is_null() {
                let _: () = msg_send![pdf_view, release];
                let _: () = msg_send![pdf_doc, release];
                return Err(PrintError::PrintError(
                    "Failed to create NSPrintOperation".to_string(),
                ));
            }

            let _: () = msg_send![print_op, setShowsPrintPanel: true];
            let _: () = msg_send![print_op, setShowsProgressPanel: true];

            let _: () = msg_send![print_op, runOperation];

            let _: () = msg_send![pdf_view, release];
            let _: () = msg_send![pdf_doc, release];
        }

        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    fn show_native_print_dialog(_pdf_path: &PathBuf) -> Result<()> {
        Err(PrintError::PrintError(
            "Native print dialog not supported on this platform".to_string(),
        ))
    }
}

impl Printer for MacOSPrinter {
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
        Self::show_native_print_dialog(pdf_path)
    }

    fn show_print_preview(pdf_path: &PathBuf) -> Result<()> {
        Self::show_native_print_dialog(pdf_path)
    }
}

pub use MacOSPrinter as PlatformPrinter;
