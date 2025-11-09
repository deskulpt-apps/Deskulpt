use crate::{ShortcutKey, Theme};

type OnThemeChange = Box<dyn Fn(&Theme, &Theme) + Send + Sync>;
type OnShortcutChange = Box<dyn Fn(&ShortcutKey, Option<&String>, Option<&String>) + Send + Sync>;

/// The collection of hooks on settings changes.
#[derive(Default)]
pub struct SettingsHooks {
    /// The collection of hooks on theme changes.
    ///
    /// See [`SettingsStateExt::on_theme_change`] for more details.
    pub on_theme_change: Vec<OnThemeChange>,
    /// The collection of hooks on shortcut changes.
    ///
    /// See [`SettingsStateExt::on_shortcut_change`] for more details.
    pub on_shortcut_change: Vec<OnShortcutChange>,
}
