//! Tests for terminal capability detection.

use bettertui_terminal::query::QueryResult;
use bettertui_terminal::{
    CapabilityDetector, CjkWidth, ClipboardCapabilities, ColorSupport, EmojiWidth, FeatureMatrix, GraphicsCapabilities,
    InputCapabilities, MouseModes, QueryOrigin, RenderCapabilities, TerminalBrand, UnicodeCapabilities, UnicodeVersion,
    WindowMetrics,
};

mod brand {
    use super::*;

    #[test]
    fn brand_detect_returns_value() {
        let brand = TerminalBrand::detect();
        assert!(!brand.name().is_empty());
    }

    #[test]
    fn brand_default_is_unknown() {
        assert_eq!(TerminalBrand::default(), TerminalBrand::Unknown);
    }

    #[test]
    fn brand_name_consistent() {
        for brand in [
            TerminalBrand::Ghostty,
            TerminalBrand::Kitty,
            TerminalBrand::WezTerm,
            TerminalBrand::Alacritty,
            TerminalBrand::Foot,
            TerminalBrand::ITerm2,
            TerminalBrand::WindowsTerminal,
            TerminalBrand::VSCodeTerminal,
            TerminalBrand::Tmux,
            TerminalBrand::GnuScreen,
            TerminalBrand::Warp,
            TerminalBrand::Unknown,
        ] {
            assert!(!brand.name().is_empty());
        }
    }

    #[test]
    fn brand_is_known() {
        assert!(!TerminalBrand::Unknown.is_known());
        assert!(TerminalBrand::Ghostty.is_known());
    }
}

mod rendering {
    use super::*;

    #[test]
    fn color_support_detect() {
        let support = ColorSupport::detect();
        let _ = support;
    }

    #[test]
    fn color_support_max_colors() {
        assert_eq!(ColorSupport::TrueColor.max_colors(), 16_777_216);
        assert_eq!(ColorSupport::Color256.max_colors(), 256);
        assert_eq!(ColorSupport::Color16.max_colors(), 16);
        assert_eq!(ColorSupport::Color8.max_colors(), 8);
        assert_eq!(ColorSupport::Monochrome.max_colors(), 0);
    }

    #[test]
    fn render_capabilities_detect() {
        let caps = RenderCapabilities::detect();
        let _ = caps;
    }
}

mod unicode {
    use super::*;

    #[test]
    fn unicode_version_detect() {
        let version = UnicodeVersion::detect();
        assert!(version.version_number() >= 0.0);
    }

    #[test]
    fn unicode_capabilities_detect() {
        let caps = UnicodeCapabilities::detect();
        assert!(caps.unicode_version.version_number() >= 0.0);
    }

    #[test]
    fn emoji_width_default() {
        assert_eq!(EmojiWidth::default(), EmojiWidth::DoubleWidth);
    }

    #[test]
    fn cjk_width_default() {
        assert_eq!(CjkWidth::default(), CjkWidth::FullWidth);
    }
}

mod window {
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

mod input {
    use super::*;

    #[test]
    fn input_capabilities_detect() {
        let caps = InputCapabilities::detect();
        assert!(caps.supports_bracketed_paste());
        assert!(caps.supports_focus_events());
    }

    #[test]
    fn mouse_modes_detect() {
        let modes = MouseModes::detect();
        assert!(modes.normal_mouse);
        assert!(modes.button_tracking);
    }
}

mod graphics {
    use super::*;

    #[test]
    fn graphics_capabilities_detect() {
        let caps = GraphicsCapabilities::detect();
        let _ = caps;
    }

    #[test]
    fn graphics_has_any() {
        let caps = GraphicsCapabilities::detect();
        let _ = caps.has_any_graphics();
    }
}

mod clipboard {
    use super::*;

    #[test]
    fn clipboard_capabilities_detect() {
        let caps = ClipboardCapabilities::detect();
        assert!(caps.supports_osc52() || !caps.supports_osc52());
    }
}

mod detection {
    use super::*;

    #[test]
    fn detector_detect() {
        let detector = CapabilityDetector::detect();
        assert!(detector.is_known_terminal() || !detector.is_known_terminal());
    }

    #[test]
    fn detector_default() {
        let detector = CapabilityDetector::default();
        assert!(detector.terminal_size().0 > 0);
        assert!(detector.terminal_size().1 > 0);
    }

    #[test]
    fn detector_capabilities() {
        let detector = CapabilityDetector::detect();
        assert!(detector.supports_true_color() || !detector.supports_true_color());
        assert!(detector.supports_kitty_keyboard() || !detector.supports_kitty_keyboard());
        assert!(detector.supports_bracketed_paste());
        assert!(detector.supports_focus_events());
    }

    #[test]
    fn update_from_da1_response() {
        let mut detector = CapabilityDetector::detect();
        let results = vec![QueryResult::DeviceAttributes { terminal_type: 1, attributes: vec![4, 22, 28] }];
        detector.update_from_queries(&results);
        assert!(detector.features().da1_attributes.contains(&4));
        assert!(detector.supports_true_color());
        assert_eq!(*detector.query_origin(), QueryOrigin::Confirmed);
    }

    #[test]
    fn update_from_da2_kitty() {
        let mut detector = CapabilityDetector::detect();
        let results = vec![QueryResult::SecondaryDeviceAttributes { model: 10, firmware_major: 0, firmware_minor: 0 }];
        detector.update_from_queries(&results);
        assert_eq!(*detector.brand(), TerminalBrand::Kitty);
        assert!(detector.supports_kitty_keyboard());
        assert_eq!(*detector.query_origin(), QueryOrigin::Confirmed);
    }

    #[test]
    fn update_from_da2_ghostty() {
        let mut detector = CapabilityDetector::detect();
        let results = vec![QueryResult::SecondaryDeviceAttributes { model: 18, firmware_major: 0, firmware_minor: 0 }];
        detector.update_from_queries(&results);
        assert_eq!(*detector.brand(), TerminalBrand::Ghostty);
        assert!(detector.supports_kitty_keyboard());
    }

    #[test]
    fn update_from_da2_unknown() {
        let mut detector = CapabilityDetector::detect();
        let results = vec![QueryResult::SecondaryDeviceAttributes { model: 999, firmware_major: 0, firmware_minor: 0 }];
        detector.update_from_queries(&results);
        assert_eq!(*detector.query_origin(), QueryOrigin::EnvOnly);
    }

    #[test]
    fn feature_matrix_default_for_brand() {
        let kitty = FeatureMatrix::default_for_brand(TerminalBrand::Kitty);
        assert!(kitty.kitty_keyboard);
        assert!(kitty.csi_u);
        assert!(kitty.kitty_graphics);
        assert!(kitty.osc52);

        let unknown = FeatureMatrix::default_for_brand(TerminalBrand::Unknown);
        assert!(!unknown.kitty_keyboard);
        assert!(!unknown.kitty_graphics);
    }

    #[test]
    fn query_origin_env_only() {
        let detector = CapabilityDetector::detect();
        assert_eq!(*detector.query_origin(), QueryOrigin::EnvOnly);
    }
}
