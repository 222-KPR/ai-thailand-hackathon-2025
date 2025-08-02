// Pages module for AI4Thai Crop Guardian - 2025 Design System
// Exports all page components

pub mod home;

pub use home::{HomePage, HomePageProps, FarmStats, generate_homepage_css};

// Re-export commonly used types
pub use home::{FarmStats as Stats};
