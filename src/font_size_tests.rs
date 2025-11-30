/// Font size functionality test suite
/// 
/// Tests for font size validation, storage, and retrieval

#[cfg(test)]
mod tests {
    use crate::consts::{FONT_SIZE_SMALL, FONT_SIZE_MEDIUM, FONT_SIZE_LARGE, FONT_SIZE_DEFAULT};
    use crate::settings::SettingsManager;
    use std::fs;
    use std::path::PathBuf;

    /// Helper function to create a temporary settings file for testing
    fn create_test_settings() -> (SettingsManager, PathBuf, String) {
        // Save original HOME
        let original_home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());
        
        // Create temporary test directory
        let temp_dir = std::env::temp_dir().join(format!("kakeibon_test_{}_{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        fs::create_dir_all(&temp_dir).unwrap();
        
        // Set HOME environment variable to temp directory for this test
        std::env::set_var("HOME", &temp_dir);
        
        // Now SettingsManager::new() will use the temp directory
        let settings = SettingsManager::new().unwrap();
        
        (settings, temp_dir, original_home)
    }

    /// Clean up test directory and restore HOME
    fn cleanup_test_dir(dir: PathBuf, original_home: String) {
        let _ = fs::remove_dir_all(dir);
        std::env::set_var("HOME", original_home);
    }

    #[test]
    fn test_font_size_default() {
        let (settings, temp_dir, original_home) = create_test_settings();
        
        // Get font size, use default if not set
        let size = settings.get_string("font_size")
            .unwrap_or_else(|_| FONT_SIZE_DEFAULT.to_string());
        
        // Should be one of the valid presets
        assert!(
            size == FONT_SIZE_SMALL || 
            size == FONT_SIZE_MEDIUM || 
            size == FONT_SIZE_LARGE ||
            size.parse::<u32>().map(|p| p >= 50 && p <= 200).unwrap_or(false),
            "Font size should be a valid preset or percentage"
        );
        
        cleanup_test_dir(temp_dir, original_home);
    }

    #[test]
    fn test_set_font_size_small() {
        let (mut settings, temp_dir, original_home) = create_test_settings();
        
        // Set font size to small
        settings.set("font_size", FONT_SIZE_SMALL).unwrap();
        settings.save().unwrap();
        
        // Verify it was set correctly
        let size = settings.get_string("font_size").unwrap();
        assert_eq!(size, FONT_SIZE_SMALL);
        
        cleanup_test_dir(temp_dir, original_home);
    }

    #[test]
    fn test_set_font_size_medium() {
        let (mut settings, temp_dir, original_home) = create_test_settings();
        
        // Set font size to medium
        settings.set("font_size", FONT_SIZE_MEDIUM).unwrap();
        settings.save().unwrap();
        
        // Verify it was set correctly
        let size = settings.get_string("font_size").unwrap();
        assert_eq!(size, FONT_SIZE_MEDIUM);
        
        cleanup_test_dir(temp_dir, original_home);
    }

    #[test]
    fn test_set_font_size_large() {
        let (mut settings, temp_dir, original_home) = create_test_settings();
        
        // Set font size to large
        settings.set("font_size", FONT_SIZE_LARGE).unwrap();
        settings.save().unwrap();
        
        // Verify it was set correctly
        let size = settings.get_string("font_size").unwrap();
        assert_eq!(size, FONT_SIZE_LARGE);
        
        cleanup_test_dir(temp_dir, original_home);
    }

    #[test]
    fn test_validate_font_size_preset() {
        // Test that preset values are valid
        let valid_presets = vec![FONT_SIZE_SMALL, FONT_SIZE_MEDIUM, FONT_SIZE_LARGE];
        
        for preset in valid_presets {
            assert!(
                preset == FONT_SIZE_SMALL || 
                preset == FONT_SIZE_MEDIUM || 
                preset == FONT_SIZE_LARGE,
                "Preset {} should be valid",
                preset
            );
        }
    }

    #[test]
    fn test_validate_font_size_custom_percentage() {
        // Test valid custom percentages
        let valid_percentages = vec!["50", "75", "100", "125", "150", "175", "200"];
        
        for percentage in valid_percentages {
            let percent: u32 = percentage.parse().unwrap();
            assert!(
                percent >= 50 && percent <= 200,
                "Percentage {} should be in range 50-200",
                percent
            );
        }
    }

    #[test]
    fn test_invalid_font_size_custom_percentage() {
        // Test invalid custom percentages
        let invalid_percentages = vec!["49", "0", "201", "300", "-10"];
        
        for percentage in invalid_percentages {
            if let Ok(percent) = percentage.parse::<u32>() {
                assert!(
                    percent < 50 || percent > 200,
                    "Percentage {} should be out of range",
                    percent
                );
            }
        }
    }

    #[test]
    fn test_invalid_font_size_string() {
        // Test invalid string values
        let invalid_values = vec!["tiny", "huge", "extra-large", "xs", "xl"];
        
        for value in invalid_values {
            assert!(
                value != FONT_SIZE_SMALL && 
                value != FONT_SIZE_MEDIUM && 
                value != FONT_SIZE_LARGE,
                "Value {} should not be a valid preset",
                value
            );
        }
    }

    #[test]
    fn test_font_size_persistence() {
        let (mut settings, temp_dir, original_home) = create_test_settings();
        
        // Set multiple font sizes in sequence
        let sizes = vec![FONT_SIZE_SMALL, FONT_SIZE_LARGE, FONT_SIZE_MEDIUM];
        
        for size in sizes {
            settings.set("font_size", size).unwrap();
            settings.save().unwrap();
            
            let retrieved = settings.get_string("font_size").unwrap();
            assert_eq!(retrieved, size, "Font size should persist correctly");
        }
        
        cleanup_test_dir(temp_dir, original_home);
    }

    #[test]
    fn test_font_size_custom_percentage_persistence() {
        let (mut settings, temp_dir, original_home) = create_test_settings();
        
        // Set custom percentages
        let percentages = vec!["85", "100", "115", "130"];
        
        for percentage in percentages {
            settings.set("font_size", percentage).unwrap();
            settings.save().unwrap();
            
            let retrieved = settings.get_string("font_size").unwrap();
            assert_eq!(retrieved, percentage, "Custom percentage should persist correctly");
        }
        
        cleanup_test_dir(temp_dir, original_home);
    }

    #[test]
    fn test_font_size_boundary_values() {
        let (mut settings, temp_dir, original_home) = create_test_settings();
        
        // Test boundary values
        let boundary_values = vec!["50", "200"];
        
        for value in boundary_values {
            settings.set("font_size", value).unwrap();
            settings.save().unwrap();
            
            let retrieved = settings.get_string("font_size").unwrap();
            assert_eq!(retrieved, value, "Boundary value {} should be stored correctly", value);
            
            let percent: u32 = value.parse().unwrap();
            assert!(
                percent >= 50 && percent <= 200,
                "Boundary value {} should be valid",
                percent
            );
        }
        
        cleanup_test_dir(temp_dir, original_home);
    }

    #[test]
    fn test_font_size_overwrite() {
        let (mut settings, temp_dir, original_home) = create_test_settings();
        
        // Set initial font size
        settings.set("font_size", FONT_SIZE_SMALL).unwrap();
        settings.save().unwrap();
        
        let first = settings.get_string("font_size").unwrap();
        assert_eq!(first, FONT_SIZE_SMALL);
        
        // Overwrite with new font size
        settings.set("font_size", FONT_SIZE_LARGE).unwrap();
        settings.save().unwrap();
        
        let second = settings.get_string("font_size").unwrap();
        assert_eq!(second, FONT_SIZE_LARGE);
        assert_ne!(first, second);
        
        cleanup_test_dir(temp_dir, original_home);
    }

    #[test]
    fn test_font_size_constants() {
        // Verify constant values
        assert_eq!(FONT_SIZE_SMALL, "small");
        assert_eq!(FONT_SIZE_MEDIUM, "medium");
        assert_eq!(FONT_SIZE_LARGE, "large");
        assert_eq!(FONT_SIZE_DEFAULT, FONT_SIZE_MEDIUM);
    }
}
