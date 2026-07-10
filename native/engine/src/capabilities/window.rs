use std::env;

#[derive(Debug, Clone)]
pub struct WindowMetrics {
    pub terminal_width: u16,
    pub terminal_height: u16,
    pub pixel_width: Option<u32>,
    pub pixel_height: Option<u32>,
    pub cell_width: Option<u32>,
    pub cell_height: Option<u32>,
    pub dpi: Option<f64>,
}

impl WindowMetrics {
    pub fn detect() -> Self {
        let (terminal_width, terminal_height) = Self::detect_terminal_size();
        let (pixel_width, pixel_height) = Self::detect_pixel_size();
        let (cell_width, cell_height) = Self::detect_cell_size();
        let dpi = Self::detect_dpi();

        Self {
            terminal_width,
            terminal_height,
            pixel_width,
            pixel_height,
            cell_width,
            cell_height,
            dpi,
        }
    }

    fn detect_terminal_size() -> (u16, u16) {
        if let Ok((w, h)) = crossterm::terminal::size() {
            (w, h)
        } else {
            (80, 24)
        }
    }

    fn detect_pixel_size() -> (Option<u32>, Option<u32>) {
        if let Ok(val) = env::var("WINDOW像素宽度")
            && let Ok(w) = val.parse()
            && let Ok(val) = env::var("WINDOW像素高度")
            && let Ok(h) = val.parse()
        {
            return (Some(w), Some(h));
        }

        if let Ok(val) = env::var("GHOSTTY窗口宽度")
            && let Ok(w) = val.parse()
            && let Ok(val) = env::var("GHOSTTY窗口高度")
            && let Ok(h) = val.parse()
        {
            return (Some(w), Some(h));
        }

        (None, None)
    }

    fn detect_cell_size() -> (Option<u32>, Option<u32>) {
        if let Ok(val) = env::var("GHOSTTY单元宽度")
            && let Ok(w) = val.parse()
            && let Ok(val) = env::var("GHOSTTY单元高度")
            && let Ok(h) = val.parse()
        {
            return (Some(w), Some(h));
        }

        (None, None)
    }

    fn detect_dpi() -> Option<f64> {
        if let Ok(val) = env::var("GHOSTTY DPI")
            && let Ok(dpi) = val.parse()
        {
            return Some(dpi);
        }

        None
    }

    pub fn cell_aspect_ratio(&self) -> Option<f64> {
        if let (Some(w), Some(h)) = (self.cell_width, self.cell_height)
            && h > 0
        {
            return Some(w as f64 / h as f64);
        }
        None
    }

    pub fn pixels_per_cell(&self) -> Option<(u32, u32)> {
        if let (Some(pw), Some(ph)) = (self.pixel_width, self.pixel_height)
            && self.terminal_width > 0
            && self.terminal_height > 0
        {
            let cell_w = pw / self.terminal_width as u32;
            let cell_h = ph / self.terminal_height as u32;
            return Some((cell_w, cell_h));
        }
        None
    }
}

impl Default for WindowMetrics {
    fn default() -> Self {
        Self::detect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_metrics_detect() {
        let metrics = WindowMetrics::detect();
        assert!(metrics.terminal_width > 0);
        assert!(metrics.terminal_height > 0);
    }

    #[test]
    fn window_metrics_default() {
        let metrics = WindowMetrics::default();
        assert!(metrics.terminal_width > 0);
        assert!(metrics.terminal_height > 0);
    }
}
