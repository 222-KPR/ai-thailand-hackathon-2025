// Copyright (c) 2025 AI4Thai Crop Guardian
// Licensed under the MIT License

//! Dopamine Color Palette System
//! 
//! This module defines the vibrant, energetic color palette designed to evoke
//! positive emotions and create an engaging user experience for Thai farmers.

/// Primary dopamine colors for main UI elements
pub struct PrimaryColors;

impl PrimaryColors {
    /// Electric blue - Primary action color
    pub const ELECTRIC_BLUE: &'static str = "#0066FF";
    
    /// Vibrant orange - Secondary action color
    pub const VIBRANT_ORANGE: &'static str = "#FF6B35";
    
    /// Energetic pink - Accent color for highlights
    pub const ENERGETIC_PINK: &'static str = "#FF1B8D";
}

/// Accent colors for variety and visual interest
pub struct AccentColors;

impl AccentColors {
    /// Lime green - Success states and positive feedback
    pub const LIME_GREEN: &'static str = "#32D74B";
    
    /// Purple - Premium features and special content
    pub const PURPLE: &'static str = "#AF52DE";
    
    /// Yellow - Warnings and attention-grabbing elements
    pub const YELLOW: &'static str = "#FFD60A";
    
    /// Cyan - Information and neutral actions
    pub const CYAN: &'static str = "#5AC8FA";
    
    /// Coral - Warm accent for agricultural themes
    pub const CORAL: &'static str = "#FF9F0A";
}

/// Background and surface colors for balance
pub struct SurfaceColors;

impl SurfaceColors {
    /// Light background - Primary light theme background
    pub const BG_LIGHT: &'static str = "#FAFAFA";
    
    /// Dark background - Primary dark theme background
    pub const BG_DARK: &'static str = "#1C1C1E";
    
    /// Light surface - Cards and elevated elements in light theme
    pub const SURFACE_LIGHT: &'static str = "#FFFFFF";
    
    /// Dark surface - Cards and elevated elements in dark theme
    pub const SURFACE_DARK: &'static str = "#2C2C2E";
    
    /// Light border - Subtle borders in light theme
    pub const BORDER_LIGHT: &'static str = "#E5E5E7";
    
    /// Dark border - Subtle borders in dark theme
    pub const BORDER_DARK: &'static str = "#38383A";
}

/// Text colors for optimal readability
pub struct TextColors;

impl TextColors {
    /// Primary text on light backgrounds
    pub const PRIMARY_LIGHT: &'static str = "#1D1D1F";
    
    /// Primary text on dark backgrounds
    pub const PRIMARY_DARK: &'static str = "#F2F2F7";
    
    /// Secondary text on light backgrounds
    pub const SECONDARY_LIGHT: &'static str = "#6D6D80";
    
    /// Secondary text on dark backgrounds
    pub const SECONDARY_DARK: &'static str = "#8E8E93";
    
    /// Tertiary text on light backgrounds
    pub const TERTIARY_LIGHT: &'static str = "#C7C7CC";
    
    /// Tertiary text on dark backgrounds
    pub const TERTIARY_DARK: &'static str = "#48484A";
}

/// Semantic colors for status and feedback
pub struct SemanticColors;

impl SemanticColors {
    /// Success color - Green for positive outcomes
    pub const SUCCESS: &'static str = "#34C759";
    
    /// Warning color - Orange for caution
    pub const WARNING: &'static str = "#FF9500";
    
    /// Error color - Red for errors and destructive actions
    pub const ERROR: &'static str = "#FF3B30";
    
    /// Info color - Blue for informational content
    pub const INFO: &'static str = "#007AFF";
}

/// Agricultural theme colors specific to farming context
pub struct AgricultureColors;

impl AgricultureColors {
    /// Healthy crop green
    pub const CROP_HEALTHY: &'static str = "#4CAF50";
    
    /// Diseased crop yellow/brown
    pub const CROP_DISEASED: &'static str = "#FF8F00";
    
    /// Soil brown
    pub const SOIL: &'static str = "#8D6E63";
    
    /// Water blue
    pub const WATER: &'static str = "#2196F3";
    
    /// Sun yellow
    pub const SUN: &'static str = "#FFC107";
}

/// Color palette utility functions
pub struct ColorPalette;

impl ColorPalette {
    /// Get primary color by index (0-2)
    pub fn primary(index: usize) -> &'static str {
        match index {
            0 => PrimaryColors::ELECTRIC_BLUE,
            1 => PrimaryColors::VIBRANT_ORANGE,
            2 => PrimaryColors::ENERGETIC_PINK,
            _ => PrimaryColors::ELECTRIC_BLUE,
        }
    }
    
    /// Get accent color by index (0-4)
    pub fn accent(index: usize) -> &'static str {
        match index {
            0 => AccentColors::LIME_GREEN,
            1 => AccentColors::PURPLE,
            2 => AccentColors::YELLOW,
            3 => AccentColors::CYAN,
            4 => AccentColors::CORAL,
            _ => AccentColors::LIME_GREEN,
        }
    }
    
    /// Get semantic color by type
    pub fn semantic(color_type: &str) -> &'static str {
        match color_type {
            "success" => SemanticColors::SUCCESS,
            "warning" => SemanticColors::WARNING,
            "error" => SemanticColors::ERROR,
            "info" => SemanticColors::INFO,
            _ => SemanticColors::INFO,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_colors() {
        assert_eq!(PrimaryColors::ELECTRIC_BLUE, "#0066FF");
        assert_eq!(PrimaryColors::VIBRANT_ORANGE, "#FF6B35");
        assert_eq!(PrimaryColors::ENERGETIC_PINK, "#FF1B8D");
    }

    #[test]
    fn test_color_palette_utility() {
        assert_eq!(ColorPalette::primary(0), "#0066FF");
        assert_eq!(ColorPalette::accent(0), "#32D74B");
        assert_eq!(ColorPalette::semantic("success"), "#34C759");
    }
}
