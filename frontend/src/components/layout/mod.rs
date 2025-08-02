// Layout components module for AI4Thai Crop Guardian - 2025 Design System
// Exports all layout-related components

pub mod bento_grid;

pub use bento_grid::{
    BentoGrid, BentoGridProps,
    BentoCard, BentoCardProps,
    BentoSection, BentoSectionProps,
    GridBreakpoint, GridConfig,
    use_responsive_grid,
    generate_bento_css
};

// Re-export commonly used components
pub use bento_grid::{BentoGrid as Grid, BentoCard as Card, BentoSection as Section};
