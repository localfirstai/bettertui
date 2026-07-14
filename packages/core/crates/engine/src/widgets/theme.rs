use std::collections::HashMap;

use crate::tree::{Color, NamedColor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeToken {
    Background,
    Surface,
    SurfaceHigh,
    SurfaceLow,
    Primary,
    PrimaryForeground,
    Secondary,
    SecondaryForeground,
    Text,
    TextMuted,
    TextDim,
    Border,
    BorderFocused,
    Accent,
    AccentForeground,
    Error,
    Warning,
    Success,
    Info,
    Scrollbar,
    ScrollbarThumb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpacingToken {
    None,
    Xxs,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: Box<str>,
    pub colors: HashMap<ThemeToken, Color>,
    pub spacing: HashMap<SpacingToken, u16>,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    pub fn dark() -> Self {
        let mut colors = HashMap::new();
        colors.insert(ThemeToken::Background, Color::Default);
        colors.insert(ThemeToken::Surface, Color::rgb(30, 30, 40));
        colors.insert(ThemeToken::SurfaceHigh, Color::rgb(40, 40, 55));
        colors.insert(ThemeToken::SurfaceLow, Color::rgb(20, 20, 28));
        colors.insert(ThemeToken::Primary, Color::rgb(100, 140, 220));
        colors.insert(
            ThemeToken::PrimaryForeground,
            Color::Named(NamedColor::White),
        );
        colors.insert(ThemeToken::Secondary, Color::rgb(140, 100, 200));
        colors.insert(
            ThemeToken::SecondaryForeground,
            Color::Named(NamedColor::White),
        );
        colors.insert(ThemeToken::Text, Color::rgb(220, 220, 230));
        colors.insert(ThemeToken::TextMuted, Color::rgb(140, 140, 160));
        colors.insert(ThemeToken::TextDim, Color::rgb(90, 90, 105));
        colors.insert(ThemeToken::Border, Color::rgb(60, 60, 80));
        colors.insert(ThemeToken::BorderFocused, Color::rgb(100, 140, 220));
        colors.insert(ThemeToken::Accent, Color::rgb(80, 200, 160));
        colors.insert(
            ThemeToken::AccentForeground,
            Color::Named(NamedColor::White),
        );
        colors.insert(ThemeToken::Error, Color::rgb(220, 80, 80));
        colors.insert(ThemeToken::Warning, Color::rgb(220, 180, 60));
        colors.insert(ThemeToken::Success, Color::rgb(80, 200, 120));
        colors.insert(ThemeToken::Info, Color::rgb(80, 160, 220));
        colors.insert(ThemeToken::Scrollbar, Color::rgb(50, 50, 65));
        colors.insert(ThemeToken::ScrollbarThumb, Color::rgb(100, 100, 130));

        let mut spacing = HashMap::new();
        spacing.insert(SpacingToken::None, 0);
        spacing.insert(SpacingToken::Xxs, 1);
        spacing.insert(SpacingToken::Xs, 2);
        spacing.insert(SpacingToken::Sm, 4);
        spacing.insert(SpacingToken::Md, 8);
        spacing.insert(SpacingToken::Lg, 12);
        spacing.insert(SpacingToken::Xl, 16);
        spacing.insert(SpacingToken::Xxl, 24);

        Self {
            name: "dark".into(),
            colors,
            spacing,
        }
    }

    pub fn light() -> Self {
        let mut colors = HashMap::new();
        colors.insert(ThemeToken::Background, Color::Named(NamedColor::White));
        colors.insert(ThemeToken::Surface, Color::rgb(245, 245, 250));
        colors.insert(ThemeToken::SurfaceHigh, Color::rgb(255, 255, 255));
        colors.insert(ThemeToken::SurfaceLow, Color::rgb(235, 235, 242));
        colors.insert(ThemeToken::Primary, Color::rgb(60, 100, 180));
        colors.insert(
            ThemeToken::PrimaryForeground,
            Color::Named(NamedColor::White),
        );
        colors.insert(ThemeToken::Secondary, Color::rgb(100, 70, 160));
        colors.insert(
            ThemeToken::SecondaryForeground,
            Color::Named(NamedColor::White),
        );
        colors.insert(ThemeToken::Text, Color::rgb(30, 30, 40));
        colors.insert(ThemeToken::TextMuted, Color::rgb(100, 100, 120));
        colors.insert(ThemeToken::TextDim, Color::rgb(160, 160, 175));
        colors.insert(ThemeToken::Border, Color::rgb(200, 200, 215));
        colors.insert(ThemeToken::BorderFocused, Color::rgb(60, 100, 180));
        colors.insert(ThemeToken::Accent, Color::rgb(40, 160, 120));
        colors.insert(
            ThemeToken::AccentForeground,
            Color::Named(NamedColor::White),
        );
        colors.insert(ThemeToken::Error, Color::rgb(200, 50, 50));
        colors.insert(ThemeToken::Warning, Color::rgb(200, 150, 30));
        colors.insert(ThemeToken::Success, Color::rgb(40, 160, 80));
        colors.insert(ThemeToken::Info, Color::rgb(40, 120, 200));
        colors.insert(ThemeToken::Scrollbar, Color::rgb(220, 220, 230));
        colors.insert(ThemeToken::ScrollbarThumb, Color::rgb(160, 160, 180));

        let mut spacing = HashMap::new();
        spacing.insert(SpacingToken::None, 0);
        spacing.insert(SpacingToken::Xxs, 1);
        spacing.insert(SpacingToken::Xs, 2);
        spacing.insert(SpacingToken::Sm, 4);
        spacing.insert(SpacingToken::Md, 8);
        spacing.insert(SpacingToken::Lg, 12);
        spacing.insert(SpacingToken::Xl, 16);
        spacing.insert(SpacingToken::Xxl, 24);

        Self {
            name: "light".into(),
            colors,
            spacing,
        }
    }

    pub fn color(&self, token: ThemeToken) -> Color {
        self.colors.get(&token).copied().unwrap_or(Color::Default)
    }

    pub fn spacing(&self, token: SpacingToken) -> u16 {
        self.spacing.get(&token).copied().unwrap_or(0)
    }

    pub fn spacing_f32(&self, token: SpacingToken) -> f32 {
        self.spacing(token) as f32
    }

    pub fn set_color(&mut self, token: ThemeToken, color: Color) {
        self.colors.insert(token, color);
    }

    pub fn set_spacing(&mut self, token: SpacingToken, value: u16) {
        self.spacing.insert(token, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_dark_default() {
        let theme = Theme::dark();
        assert_eq!(theme.name.as_ref(), "dark");
        assert!(theme.colors.contains_key(&ThemeToken::Background));
        assert!(theme.colors.contains_key(&ThemeToken::Primary));
        assert!(theme.colors.contains_key(&ThemeToken::Text));
    }

    #[test]
    fn theme_light() {
        let theme = Theme::light();
        assert_eq!(theme.name.as_ref(), "light");
        assert!(theme.colors.contains_key(&ThemeToken::Background));
    }

    #[test]
    fn theme_default_is_dark() {
        let theme = Theme::default();
        assert_eq!(theme.name.as_ref(), "dark");
    }

    #[test]
    fn theme_color_lookup() {
        let theme = Theme::dark();
        let bg = theme.color(ThemeToken::Background);
        assert_eq!(bg, Color::Default);

        let primary = theme.color(ThemeToken::Primary);
        assert_ne!(primary, Color::Default);
    }

    #[test]
    fn theme_spacing_lookup() {
        let theme = Theme::dark();
        assert_eq!(theme.spacing(SpacingToken::None), 0);
        assert_eq!(theme.spacing(SpacingToken::Xxs), 1);
        assert_eq!(theme.spacing(SpacingToken::Sm), 4);
        assert_eq!(theme.spacing(SpacingToken::Md), 8);
        assert_eq!(theme.spacing(SpacingToken::Lg), 12);
        assert_eq!(theme.spacing(SpacingToken::Xl), 16);
        assert_eq!(theme.spacing(SpacingToken::Xxl), 24);
    }

    #[test]
    fn theme_spacing_f32() {
        let theme = Theme::dark();
        assert_eq!(theme.spacing_f32(SpacingToken::Md), 8.0);
    }

    #[test]
    fn theme_set_color() {
        let mut theme = Theme::dark();
        theme.set_color(ThemeToken::Primary, Color::rgb(255, 0, 0));
        assert_eq!(theme.color(ThemeToken::Primary), Color::rgb(255, 0, 0));
    }

    #[test]
    fn theme_set_spacing() {
        let mut theme = Theme::dark();
        theme.set_spacing(SpacingToken::Md, 10);
        assert_eq!(theme.spacing(SpacingToken::Md), 10);
    }

    #[test]
    fn theme_token_equality() {
        assert_eq!(ThemeToken::Primary, ThemeToken::Primary);
        assert_ne!(ThemeToken::Primary, ThemeToken::Secondary);
    }

    #[test]
    fn theme_token_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        ThemeToken::Primary.hash(&mut h1);
        ThemeToken::Primary.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn theme_light_colors_differ_from_dark() {
        let dark = Theme::dark();
        let light = Theme::light();
        assert_ne!(
            dark.color(ThemeToken::Background),
            light.color(ThemeToken::Background)
        );
    }
}
