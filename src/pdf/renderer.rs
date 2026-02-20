use super::{PdfDocument, Result, PdfError};
use image::{DynamicImage, RgbaImage};

pub struct PageRenderer;

impl PageRenderer {
    /// 渲染指定页面为 RGBA 图像数据
    pub fn render_page(doc: &PdfDocument, page_num: usize, zoom: f32) -> Result<DynamicImage> {
        if page_num >= doc.page_count() {
            return Err(PdfError::InvalidPage(page_num));
        }

        let (rgba_data, width, height) = doc.render_page(page_num, zoom)?;

        // 创建图像
        let image = RgbaImage::from_raw(width, height, rgba_data)
            .ok_or_else(|| PdfError::RenderError("Failed to create image from raw data".to_string()))?;

        Ok(DynamicImage::ImageRgba8(image))
    }

    /// 计算适应宽度的缩放比例
    pub fn calculate_zoom_to_fit_width(doc: &PdfDocument, page_num: usize, viewport_width: f32) -> Result<f32> {
        let (page_width, _) = doc.get_page_size(page_num)?;
        Ok(viewport_width / page_width)
    }

    /// 计算适应高度的缩放比例
    pub fn calculate_zoom_to_fit_height(doc: &PdfDocument, page_num: usize, viewport_height: f32) -> Result<f32> {
        let (_, page_height) = doc.get_page_size(page_num)?;
        Ok(viewport_height / page_height)
    }

    /// 计算适应页面的缩放比例
    pub fn calculate_zoom_to_fit_page(
        doc: &PdfDocument,
        page_num: usize,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Result<f32> {
        let (page_width, page_height) = doc.get_page_size(page_num)?;
        let zoom_x = viewport_width / page_width;
        let zoom_y = viewport_height / page_height;
        Ok(zoom_x.min(zoom_y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 这些测试需要实际的 PDF 文件
    // #[test]
    // fn test_render_page() {
    //     // 测试页面渲染
    // }
}
