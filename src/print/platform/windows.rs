use crate::pdf::PdfDocument;
use crate::print::{PrintError, PrintSettings, Printer, PrinterInfo, Result};
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use windows::{
    core::PCWSTR,
    Win32::Foundation::HWND,
    Win32::Graphics::Gdi::{
        CreateDCW, DeleteDC, GetDeviceCaps, StretchDIBits, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
        DIB_RGB_COLORS, HDC, LOGPIXELSX, LOGPIXELSY, SRCCOPY,
    },
    Win32::Graphics::Printing::{EnumPrintersW, PRINTER_ENUM_LOCAL, PRINTER_INFO_2W},
    Win32::UI::Controls::Dialogs::{
        PrintDlgW, PD_ALLPAGES, PD_NOPAGENUMS, PD_NOSELECTION, PD_RETURNDC, PRINTDLGW,
    },
};

// Import GDI printing functions from winspool
extern "system" {
    fn StartDocW(hdc: HDC, lpdi: *const DOCINFOW) -> i32;
    fn EndDoc(hdc: HDC) -> i32;
    fn StartPage(hdc: HDC) -> i32;
    fn EndPage(hdc: HDC) -> i32;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct DOCINFOW {
    cbSize: i32,
    lpszDocName: PCWSTR,
    lpszOutput: PCWSTR,
    lpszDatatype: PCWSTR,
    fwType: u32,
}

pub struct WindowsPrinter;

impl WindowsPrinter {
    fn get_printer_dc(printer_name: &str) -> Result<HDC> {
        let printer_name_wide: Vec<u16> = OsString::from(printer_name)
            .encode_wide()
            .chain(Some(0))
            .collect();

        unsafe {
            let hdc = CreateDCW(
                PCWSTR::null(),
                PCWSTR(printer_name_wide.as_ptr()),
                PCWSTR::null(),
                None,
            );

            if hdc.is_invalid() {
                Err(PrintError::InitError(
                    "Failed to create printer DC".to_string(),
                ))
            } else {
                Ok(hdc)
            }
        }
    }

    fn print_pdf_to_dc(hdc: HDC, pdf_path: &PathBuf, settings: &PrintSettings) -> Result<()> {
        unsafe {
            let dpi_x = GetDeviceCaps(hdc, LOGPIXELSX);
            let dpi_y = GetDeviceCaps(hdc, LOGPIXELSY);

            let (page_width_mm, page_height_mm) = settings.paper_size.dimensions_mm();
            let page_width_px = (page_width_mm / 25.4 * dpi_x as f32) as i32;
            let page_height_px = (page_height_mm / 25.4 * dpi_y as f32) as i32;

            let pdf_doc = PdfDocument::open(pdf_path)
                .map_err(|e| PrintError::PrintError(format!("Failed to open PDF: {}", e)))?;

            let page_count = pdf_doc.page_count();
            let page_range = settings
                .page_range
                .clone()
                .unwrap_or_else(|| crate::print::PageRange::all(page_count));

            let doc_name: Vec<u16> = OsString::from("PDF Document")
                .encode_wide()
                .chain(Some(0))
                .collect();

            let doc_info = DOCINFOW {
                cbSize: std::mem::size_of::<DOCINFOW>() as i32,
                lpszDocName: PCWSTR(doc_name.as_ptr()),
                lpszOutput: PCWSTR::null(),
                lpszDatatype: PCWSTR::null(),
                fwType: 0,
            };

            if StartDocW(hdc, &doc_info) <= 0 {
                return Err(PrintError::PrintError(
                    "Failed to start document".to_string(),
                ));
            }

            for page_num in page_range.start..=page_range.end {
                if page_num >= page_count {
                    break;
                }

                if StartPage(hdc) <= 0 {
                    EndDoc(hdc);
                    return Err(PrintError::PrintError("Failed to start page".to_string()));
                }

                if let Err(e) = Self::render_pdf_page_to_dc(
                    hdc,
                    &pdf_doc,
                    page_num,
                    page_width_px,
                    page_height_px,
                    dpi_x,
                    dpi_y,
                    settings,
                ) {
                    EndPage(hdc);
                    EndDoc(hdc);
                    return Err(e);
                }

                if EndPage(hdc) <= 0 {
                    EndDoc(hdc);
                    return Err(PrintError::PrintError("Failed to end page".to_string()));
                }
            }

            if EndDoc(hdc) <= 0 {
                return Err(PrintError::PrintError("Failed to end document".to_string()));
            }

            Ok(())
        }
    }

    fn render_pdf_page_to_dc(
        hdc: HDC,
        pdf_doc: &PdfDocument,
        page_num: usize,
        page_width: i32,
        page_height: i32,
        dpi_x: i32,
        _dpi_y: i32,
        settings: &PrintSettings,
    ) -> Result<()> {
        let zoom = (dpi_x as f32 / 72.0) * 2.0;
        let (bitmap_data, width, height) = pdf_doc
            .render_page(page_num, zoom)
            .map_err(|e| PrintError::PrintError(format!("Failed to render page: {}", e)))?;

        let (dest_width, dest_height) = if settings.scale_to_fit {
            let scale_x = page_width as f32 / width as f32;
            let scale_y = page_height as f32 / height as f32;
            let scale = scale_x.min(scale_y);
            (
                (width as f32 * scale) as i32,
                (height as f32 * scale) as i32,
            )
        } else {
            (width as i32, height as i32)
        };

        let x_offset = (page_width - dest_width) / 2;
        let y_offset = (page_height - dest_height) / 2;

        unsafe {
            let mut bmi: BITMAPINFO = std::mem::zeroed();
            bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
            bmi.bmiHeader.biWidth = width as i32;
            bmi.bmiHeader.biHeight = -(height as i32);
            bmi.bmiHeader.biPlanes = 1;
            bmi.bmiHeader.biBitCount = 32;
            bmi.bmiHeader.biCompression = BI_RGB.0 as u32;

            let result = StretchDIBits(
                hdc,
                x_offset,
                y_offset,
                dest_width,
                dest_height,
                0,
                0,
                width as i32,
                height as i32,
                Some(bitmap_data.as_ptr() as *const core::ffi::c_void),
                &bmi,
                DIB_RGB_COLORS,
                SRCCOPY,
            );

            if result == 0 || result == -1 {
                return Err(PrintError::PrintError(
                    "Failed to draw bitmap to printer".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn show_print_dialog_with_settings(_pdf_path: &PathBuf) -> Result<(PrintSettings, String)> {
        unsafe {
            // Try to get printers, but don't fail if none are available
            let printer_name = match Self::get_printers() {
                Ok(printers) if !printers.is_empty() => printers
                    .iter()
                    .find(|p| p.is_default)
                    .map(|p| p.name.clone())
                    .or_else(|| printers.first().map(|p| p.name.clone()))
                    .unwrap_or_default(),
                _ => String::new(),
            };

            // Setup PRINTDLG structure
            let mut pd: PRINTDLGW = std::mem::zeroed();
            pd.lStructSize = std::mem::size_of::<PRINTDLGW>() as u32;
            pd.hwndOwner = HWND(0);
            pd.Flags = PD_ALLPAGES | PD_RETURNDC | PD_NOSELECTION | PD_NOPAGENUMS;
            pd.nFromPage = 1;
            pd.nToPage = 1;
            pd.nMinPage = 1;
            pd.nMaxPage = 0xFFFF;
            pd.nCopies = 1;

            // Show print dialog - this will show even if no printers are available
            if PrintDlgW(&mut pd) == false {
                return Err(PrintError::PrintError("Print dialog cancelled".to_string()));
            }

            // Extract settings from dialog
            let settings = PrintSettings {
                paper_size: crate::print::PaperSize::A4,
                orientation: crate::print::Orientation::Portrait,
                page_range: None,
                copies: pd.nCopies as u32,
                duplex: false,
                color: true,
                scale_to_fit: true,
                margins: crate::print::Margins::default(),
            };

            // Clean up DC from dialog
            if !pd.hDC.is_invalid() {
                let _ = DeleteDC(pd.hDC);
            }

            Ok((settings, printer_name))
        }
    }

    fn show_print_preview_dialog(_pdf_path: &PathBuf) -> Result<()> {
        // Print preview is shown in the application's own UI
        // The main application window already displays the PDF content
        // which serves as the print preview
        // For now, return an error indicating preview is not implemented
        Err(PrintError::PlatformError(
            "Print preview should use the application's built-in PDF viewer".to_string(),
        ))
    }
}

impl Printer for WindowsPrinter {
    fn get_printers() -> Result<Vec<PrinterInfo>> {
        unsafe {
            let mut needed: u32 = 0;
            let mut returned: u32 = 0;

            let _ = EnumPrintersW(
                PRINTER_ENUM_LOCAL,
                PCWSTR::null(),
                2,
                None,
                &mut needed,
                &mut returned,
            );

            if needed == 0 {
                return Err(PrintError::NoPrinter);
            }

            let mut buffer: Vec<u8> = vec![0; needed as usize];

            let result = EnumPrintersW(
                PRINTER_ENUM_LOCAL,
                PCWSTR::null(),
                2,
                Some(&mut buffer),
                &mut needed,
                &mut returned,
            );

            if result.is_err() {
                return Err(PrintError::InitError(
                    "Failed to enumerate printers".to_string(),
                ));
            }

            let mut printers = Vec::new();
            let printer_info = buffer.as_ptr() as *const PRINTER_INFO_2W;

            for i in 0..returned {
                let info = &*printer_info.add(i as usize);

                let name = if !info.pPrinterName.0.is_null() {
                    let len = (0..)
                        .take_while(|&i| *info.pPrinterName.0.add(i) != 0)
                        .count();
                    let slice = std::slice::from_raw_parts(info.pPrinterName.0, len);
                    String::from_utf16_lossy(slice)
                } else {
                    continue;
                };

                let is_default = info.Attributes & 0x00000004 != 0;

                printers.push(PrinterInfo {
                    name,
                    is_default,
                    supports_color: true,
                    supports_duplex: true,
                });
            }

            if printers.is_empty() {
                return Err(PrintError::NoPrinter);
            }

            Ok(printers)
        }
    }

    fn print_pdf(
        pdf_path: &PathBuf,
        settings: &PrintSettings,
        printer_name: Option<&str>,
    ) -> Result<()> {
        let printer = if let Some(name) = printer_name {
            name.to_string()
        } else {
            let printers = Self::get_printers()?;
            printers
                .into_iter()
                .find(|p| p.is_default)
                .map(|p| p.name)
                .ok_or(PrintError::NoPrinter)?
        };

        let hdc = Self::get_printer_dc(&printer)?;
        let result = Self::print_pdf_to_dc(hdc, pdf_path, settings);

        unsafe {
            let _ = DeleteDC(hdc);
        }

        result
    }

    fn show_print_dialog(pdf_path: &PathBuf) -> Result<()> {
        let (settings, printer_name) = Self::show_print_dialog_with_settings(pdf_path)?;
        Self::print_pdf(pdf_path, &settings, Some(&printer_name))
    }

    fn show_print_preview(pdf_path: &PathBuf) -> Result<()> {
        // Use the application's built-in PDF viewer for preview
        // The main window already displays the PDF content
        Self::show_print_preview_dialog(pdf_path)
    }
}

pub use WindowsPrinter as PlatformPrinter;
