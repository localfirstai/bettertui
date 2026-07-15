use bettertui_engine::tree::{BorderStyle, Color, NamedColor};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemeColors {
    pub background: Color,
    pub surface: Color,
    pub surface_high: Color,
    pub surface_low: Color,
    pub primary: Color,
    pub primary_foreground: Color,
    pub secondary: Color,
    pub secondary_foreground: Color,
    pub text: Color,
    pub text_muted: Color,
    pub text_dim: Color,
    pub border: Color,
    pub border_focused: Color,
    pub accent: Color,
    pub accent_foreground: Color,
    pub error: Color,
    pub warning: Color,
    pub success: Color,
    pub info: Color,
    pub scrollbar: Color,
    pub scrollbar_thumb: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemeSpacing {
    pub none: u16,
    pub xxs: u16,
    pub xs: u16,
    pub sm: u16,
    pub md: u16,
    pub lg: u16,
    pub xl: u16,
    pub xxl: u16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemeBorders {
    pub style: BorderStyle,
    pub fg: Color,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub name: Box<str>,
    pub colors: ThemeColors,
    pub spacing: ThemeSpacing,
    pub borders: ThemeBorders,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            name: "dark".into(),
            colors: ThemeColors {
                background: Color::Default,
                surface: Color::rgb(30, 30, 40),
                surface_high: Color::rgb(40, 40, 55),
                surface_low: Color::rgb(20, 20, 28),
                primary: Color::rgb(100, 140, 220),
                primary_foreground: Color::Named(NamedColor::White),
                secondary: Color::rgb(140, 100, 200),
                secondary_foreground: Color::Named(NamedColor::White),
                text: Color::rgb(220, 220, 230),
                text_muted: Color::rgb(140, 140, 160),
                text_dim: Color::rgb(90, 90, 105),
                border: Color::rgb(60, 60, 80),
                border_focused: Color::rgb(100, 140, 220),
                accent: Color::rgb(80, 200, 160),
                accent_foreground: Color::Named(NamedColor::White),
                error: Color::rgb(220, 80, 80),
                warning: Color::rgb(220, 180, 60),
                success: Color::rgb(80, 200, 120),
                info: Color::rgb(80, 160, 220),
                scrollbar: Color::rgb(50, 50, 65),
                scrollbar_thumb: Color::rgb(100, 100, 130),
            },
            spacing: ThemeSpacing { none: 0, xxs: 1, xs: 2, sm: 4, md: 8, lg: 12, xl: 16, xxl: 24 },
            borders: ThemeBorders { style: BorderStyle::Solid, fg: Color::rgb(60, 60, 80) },
        }
    }

    pub fn light() -> Self {
        Self {
            name: "light".into(),
            colors: ThemeColors {
                background: Color::Named(NamedColor::White),
                surface: Color::rgb(245, 245, 250),
                surface_high: Color::rgb(255, 255, 255),
                surface_low: Color::rgb(235, 235, 242),
                primary: Color::rgb(60, 100, 180),
                primary_foreground: Color::Named(NamedColor::White),
                secondary: Color::rgb(100, 70, 160),
                secondary_foreground: Color::Named(NamedColor::White),
                text: Color::rgb(30, 30, 40),
                text_muted: Color::rgb(100, 100, 120),
                text_dim: Color::rgb(160, 160, 175),
                border: Color::rgb(200, 200, 215),
                border_focused: Color::rgb(60, 100, 180),
                accent: Color::rgb(40, 160, 120),
                accent_foreground: Color::Named(NamedColor::White),
                error: Color::rgb(200, 50, 50),
                warning: Color::rgb(200, 150, 30),
                success: Color::rgb(40, 160, 80),
                info: Color::rgb(40, 120, 200),
                scrollbar: Color::rgb(220, 220, 230),
                scrollbar_thumb: Color::rgb(160, 160, 180),
            },
            spacing: ThemeSpacing { none: 0, xxs: 1, xs: 2, sm: 4, md: 8, lg: 12, xl: 16, xxl: 24 },
            borders: ThemeBorders { style: BorderStyle::Solid, fg: Color::rgb(200, 200, 215) },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_dark_default() {
        let theme = Theme::dark();
        assert_eq!(theme.name.as_ref(), "dark");
        assert_eq!(theme.colors.background, Color::Default);
        assert_ne!(theme.colors.primary, Color::Default);
        assert_ne!(theme.colors.text, Color::Default);
    }

    #[test]
    fn theme_light() {
        let theme = Theme::light();
        assert_eq!(theme.name.as_ref(), "light");
        assert_eq!(theme.colors.background, Color::Named(NamedColor::White));
    }

    #[test]
    fn theme_default_is_dark() {
        let theme = Theme::default();
        assert_eq!(theme.name.as_ref(), "dark");
    }

    #[test]
    fn theme_color_values() {
        let theme = Theme::dark();
        assert_eq!(theme.colors.primary, Color::rgb(100, 140, 220));
        assert_eq!(theme.colors.accent, Color::rgb(80, 200, 160));
    }

    #[test]
    fn theme_spacing_values() {
        let theme = Theme::dark();
        assert_eq!(theme.spacing.none, 0);
        assert_eq!(theme.spacing.xxs, 1);
        assert_eq!(theme.spacing.sm, 4);
        assert_eq!(theme.spacing.md, 8);
        assert_eq!(theme.spacing.lg, 12);
        assert_eq!(theme.spacing.xl, 16);
        assert_eq!(theme.spacing.xxl, 24);
    }

    #[test]
    fn theme_borders() {
        let theme = Theme::dark();
        assert_eq!(theme.borders.style, BorderStyle::Solid);
        assert_eq!(theme.borders.fg, Color::rgb(60, 60, 80));
    }

    #[test]
    fn theme_light_colors_differ_from_dark() {
        let dark = Theme::dark();
        let light = Theme::light();
        assert_ne!(dark.colors.background, light.colors.background);
    }

    #[test]
    fn theme_colors_copy() {
        let theme = Theme::dark();
        let colors = theme.colors;
        assert_eq!(colors.primary, Color::rgb(100, 140, 220));
    }

    #[test]
    fn theme_spacing_copy() {
        let theme = Theme::dark();
        let spacing = theme.spacing;
        assert_eq!(spacing.md, 8);
    }
}
