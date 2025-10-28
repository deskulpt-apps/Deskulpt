//! Mouse interaction management for canvas windows.

use std::collections::BTreeMap;

use crate::settings::WidgetSettings;

/// Manages mouse interaction state for the canvas window.
#[derive(Debug, Clone)]
pub struct MouseInteractionManager {
    /// Current state of cursor events (true = ignored/click-through, false =
    /// interactive)
    pub is_cursor_ignored: bool,
    /// Scale factor for coordinate conversion
    pub scale_factor: f64,
}

impl MouseInteractionManager {
    /// Create a new mouse interaction manager.
    pub fn new(scale_factor: f64) -> Self {
        Self {
            is_cursor_ignored: true, // Canvas starts click-through
            scale_factor,
        }
    }

    /// Process a mouse move event and determine if cursor state should change.
    /// Returns Some(new_state) if the state should be updated, None if no
    /// change needed.
    pub fn process_mouse_move(
        &mut self,
        x: f64,
        y: f64,
        widgets: &BTreeMap<String, WidgetSettings>,
    ) -> Option<bool> {
        let scaled_position = scale_coordinates(x, y, self.scale_factor);
        let mouse_over_widget = is_mouse_over_any_widget(scaled_position, widgets);
        let should_ignore_cursor = !mouse_over_widget;

        if should_ignore_cursor != self.is_cursor_ignored {
            self.is_cursor_ignored = should_ignore_cursor;
            Some(should_ignore_cursor)
        } else {
            None
        }
    }

    /// Get the current cursor ignore state.
    pub fn is_cursor_ignored(&self) -> bool {
        self.is_cursor_ignored
    }

    /// Set the cursor ignore state (for testing or manual control).
    pub fn set_cursor_ignored(&mut self, ignored: bool) {
        self.is_cursor_ignored = ignored;
    }
}

/// Represents a scaled mouse position.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaledPosition {
    pub x: i32,
    pub y: i32,
}

/// Scale mouse coordinates to match canvas coordinate system.
pub fn scale_coordinates(x: f64, y: f64, scale_factor: f64) -> ScaledPosition {
    ScaledPosition {
        x: (x / scale_factor) as i32,
        y: (y / scale_factor) as i32,
    }
}

/// Check if the mouse position is over any widget.
pub fn is_mouse_over_any_widget(
    position: ScaledPosition,
    widgets: &BTreeMap<String, WidgetSettings>,
) -> bool {
    widgets
        .values()
        .any(|widget| is_mouse_over_widget(position, widget))
}

/// Check if the mouse position is over a specific widget.
pub fn is_mouse_over_widget(position: ScaledPosition, widget: &WidgetSettings) -> bool {
    position.x >= widget.x
        && position.x < widget.x + widget.width as i32
        && position.y >= widget.y
        && position.y < widget.y + widget.height as i32
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    fn create_test_widget(x: i32, y: i32, width: u32, height: u32) -> WidgetSettings {
        WidgetSettings {
            x,
            y,
            width,
            height,
            opacity: 100,
        }
    }

    #[test]
    fn test_scale_coordinates() {
        // Test normal scaling
        let pos = scale_coordinates(100.0, 200.0, 2.0);
        assert_eq!(pos, ScaledPosition { x: 50, y: 100 });

        // Test fractional coordinates
        let pos = scale_coordinates(150.5, 250.7, 1.5);
        assert_eq!(pos, ScaledPosition { x: 100, y: 167 });

        // Test scale factor of 1.0 (no scaling)
        let pos = scale_coordinates(75.0, 125.0, 1.0);
        assert_eq!(pos, ScaledPosition { x: 75, y: 125 });
    }

    #[test]
    fn test_is_mouse_over_widget() {
        let widget = create_test_widget(10, 20, 100, 50);

        // Inside widget
        assert!(is_mouse_over_widget(
            ScaledPosition { x: 50, y: 40 },
            &widget
        ));
        assert!(is_mouse_over_widget(
            ScaledPosition { x: 10, y: 20 },
            &widget
        )); // Top-left corner
        assert!(is_mouse_over_widget(
            ScaledPosition { x: 109, y: 69 },
            &widget
        )); // Bottom-right inside

        // Outside widget
        assert!(!is_mouse_over_widget(
            ScaledPosition { x: 9, y: 40 },
            &widget
        )); // Left edge
        assert!(!is_mouse_over_widget(
            ScaledPosition { x: 50, y: 19 },
            &widget
        )); // Top edge
        assert!(!is_mouse_over_widget(
            ScaledPosition { x: 110, y: 40 },
            &widget
        )); // Right edge
        assert!(!is_mouse_over_widget(
            ScaledPosition { x: 50, y: 70 },
            &widget
        )); // Bottom edge
        assert!(!is_mouse_over_widget(
            ScaledPosition { x: 0, y: 0 },
            &widget
        )); // Far outside
    }

    #[test]
    fn test_is_mouse_over_any_widget_empty() {
        let widgets: BTreeMap<String, WidgetSettings> = BTreeMap::new();
        let position = ScaledPosition { x: 50, y: 50 };

        assert!(!is_mouse_over_any_widget(position, &widgets));
    }

    #[test]
    fn test_is_mouse_over_any_widget_multiple() {
        let mut widgets = BTreeMap::new();
        widgets.insert("widget1".to_string(), create_test_widget(0, 0, 50, 50));
        widgets.insert("widget2".to_string(), create_test_widget(100, 100, 50, 50));
        widgets.insert("widget3".to_string(), create_test_widget(200, 200, 50, 50));

        // Over first widget
        assert!(is_mouse_over_any_widget(
            ScaledPosition { x: 25, y: 25 },
            &widgets
        ));

        // Over second widget
        assert!(is_mouse_over_any_widget(
            ScaledPosition { x: 125, y: 125 },
            &widgets
        ));

        // Over third widget
        assert!(is_mouse_over_any_widget(
            ScaledPosition { x: 225, y: 225 },
            &widgets
        ));

        // Not over any widget
        assert!(!is_mouse_over_any_widget(
            ScaledPosition { x: 75, y: 75 },
            &widgets
        ));
        assert!(!is_mouse_over_any_widget(
            ScaledPosition { x: 300, y: 300 },
            &widgets
        ));
    }

    #[test]
    fn test_is_mouse_over_any_widget_overlapping() {
        let mut widgets = BTreeMap::new();
        widgets.insert("widget1".to_string(), create_test_widget(0, 0, 100, 100));
        widgets.insert("widget2".to_string(), create_test_widget(50, 50, 100, 100));

        // Over both widgets (overlapping area)
        assert!(is_mouse_over_any_widget(
            ScaledPosition { x: 75, y: 75 },
            &widgets
        ));

        // Over only first widget
        assert!(is_mouse_over_any_widget(
            ScaledPosition { x: 25, y: 25 },
            &widgets
        ));

        // Over only second widget
        assert!(is_mouse_over_any_widget(
            ScaledPosition { x: 125, y: 125 },
            &widgets
        ));

        // Over neither widget
        assert!(!is_mouse_over_any_widget(
            ScaledPosition { x: 200, y: 200 },
            &widgets
        ));
    }

    #[test]
    fn test_mouse_interaction_manager_initial_state() {
        let manager = MouseInteractionManager::new(2.0);
        assert!(manager.is_cursor_ignored()); // Should start click-through
        assert_eq!(manager.scale_factor, 2.0);
    }

    #[test]
    fn test_mouse_interaction_manager_no_widgets() {
        let mut manager = MouseInteractionManager::new(1.0);
        let widgets: BTreeMap<String, WidgetSettings> = BTreeMap::new();

        // Mouse move with no widgets should not change state (already ignored)
        let result = manager.process_mouse_move(100.0, 100.0, &widgets);
        assert_eq!(result, None); // No state change
        assert!(manager.is_cursor_ignored());
    }

    #[test]
    fn test_mouse_interaction_manager_state_changes() {
        let mut manager = MouseInteractionManager::new(1.0);
        let mut widgets = BTreeMap::new();
        widgets.insert("widget1".to_string(), create_test_widget(50, 50, 100, 100));

        // Move mouse over widget - should change from ignored to interactive
        let result = manager.process_mouse_move(75.0, 75.0, &widgets);
        assert_eq!(result, Some(false)); // Should not ignore cursor
        assert!(!manager.is_cursor_ignored());

        // Move mouse over same widget - no state change
        let result = manager.process_mouse_move(80.0, 80.0, &widgets);
        assert_eq!(result, None); // No state change
        assert!(!manager.is_cursor_ignored());

        // Move mouse away from widget - should change back to ignored
        let result = manager.process_mouse_move(200.0, 200.0, &widgets);
        assert_eq!(result, Some(true)); // Should ignore cursor
        assert!(manager.is_cursor_ignored());

        // Move mouse away from widget again - no state change
        let result = manager.process_mouse_move(300.0, 300.0, &widgets);
        assert_eq!(result, None); // No state change
        assert!(manager.is_cursor_ignored());
    }

    #[test]
    fn test_mouse_interaction_manager_with_scaling() {
        let mut manager = MouseInteractionManager::new(2.0);
        let mut widgets = BTreeMap::new();
        widgets.insert("widget1".to_string(), create_test_widget(25, 25, 50, 50)); // Widget at scaled coords

        // Mouse at (100, 100) real coords = (50, 50) scaled coords, which is over
        // widget
        let result = manager.process_mouse_move(100.0, 100.0, &widgets);
        assert_eq!(result, Some(false)); // Should not ignore cursor
        assert!(!manager.is_cursor_ignored());

        // Mouse at (50, 50) real coords = (25, 25) scaled coords, still over widget
        let result = manager.process_mouse_move(50.0, 50.0, &widgets);
        assert_eq!(result, None); // No state change
        assert!(!manager.is_cursor_ignored());

        // Mouse at (40, 40) real coords = (20, 20) scaled coords, outside widget
        let result = manager.process_mouse_move(40.0, 40.0, &widgets);
        assert_eq!(result, Some(true)); // Should ignore cursor
        assert!(manager.is_cursor_ignored());
    }

    #[test]
    fn test_mouse_interaction_manager_set_state() {
        let mut manager = MouseInteractionManager::new(1.0);

        // Manually set state
        manager.set_cursor_ignored(false);
        assert!(!manager.is_cursor_ignored());

        manager.set_cursor_ignored(true);
        assert!(manager.is_cursor_ignored());
    }

    #[test]
    fn test_edge_cases_zero_size_widget() {
        let widget = create_test_widget(50, 50, 0, 0);

        // Mouse exactly at widget position
        assert!(!is_mouse_over_widget(
            ScaledPosition { x: 50, y: 50 },
            &widget
        ));

        // Mouse slightly offset
        assert!(!is_mouse_over_widget(
            ScaledPosition { x: 51, y: 51 },
            &widget
        ));
    }

    #[test]
    fn test_edge_cases_negative_coordinates() {
        let widget = create_test_widget(-50, -50, 100, 100);

        // Inside widget with negative coordinates
        assert!(is_mouse_over_widget(
            ScaledPosition { x: -25, y: -25 },
            &widget
        ));
        assert!(is_mouse_over_widget(
            ScaledPosition { x: -50, y: -50 },
            &widget
        )); // Top-left corner
        assert!(is_mouse_over_widget(
            ScaledPosition { x: 49, y: 49 },
            &widget
        )); // Bottom-right inside

        // Outside widget
        assert!(!is_mouse_over_widget(
            ScaledPosition { x: -51, y: -25 },
            &widget
        ));
        assert!(!is_mouse_over_widget(
            ScaledPosition { x: 50, y: -25 },
            &widget
        ));
    }
}
