// Typography system for AI4Thai Crop Guardian - 2025 Design
// Supports Thai language with optimized font rendering

use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TypographyScale {
    // Font families
    pub font_heading: &'static str,
    pub font_body: &'static str,
    pub font_mono: &'static str,
    pub font_thai: &'static str,
    
    // Font sizes (rem units)
    pub text_xs: &'static str,
    pub text_sm: &'static str,
    pub text_base: &'static str,
    pub text_lg: &'static str,
    pub text_xl: &'static str,
    pub text_2xl: &'static str,
    pub text_3xl: &'static str,
    pub text_4xl: &'static str,
    pub text_5xl: &'static str,
    
    // Font weights
    pub weight_light: u16,
    pub weight_normal: u16,
    pub weight_medium: u16,
    pub weight_semibold: u16,
    pub weight_bold: u16,
    pub weight_extrabold: u16,
    
    // Line heights
    pub leading_tight: &'static str,
    pub leading_normal: &'static str,
    pub leading_relaxed: &'static str,
    pub leading_loose: &'static str,
    
    // Letter spacing
    pub tracking_tight: &'static str,
    pub tracking_normal: &'static str,
    pub tracking_wide: &'static str,
}

impl Default for TypographyScale {
    fn default() -> Self {
        Self {
            // Font families optimized for Thai language
            font_heading: "'Poppins', 'Sarabun', -apple-system, BlinkMacSystemFont, sans-serif",
            font_body: "'Inter', 'Sarabun', -apple-system, BlinkMacSystemFont, sans-serif",
            font_mono: "'JetBrains Mono', 'Courier New', monospace",
            font_thai: "'Sarabun', 'Prompt', 'Kanit', sans-serif",
            
            // Font sizes
            text_xs: "0.75rem",    // 12px
            text_sm: "0.875rem",   // 14px
            text_base: "1rem",     // 16px
            text_lg: "1.125rem",   // 18px
            text_xl: "1.25rem",    // 20px
            text_2xl: "1.5rem",    // 24px
            text_3xl: "1.875rem",  // 30px
            text_4xl: "2.25rem",   // 36px
            text_5xl: "3rem",      // 48px
            
            // Font weights
            weight_light: 300,
            weight_normal: 400,
            weight_medium: 500,
            weight_semibold: 600,
            weight_bold: 700,
            weight_extrabold: 800,
            
            // Line heights optimized for Thai text
            leading_tight: "1.25",
            leading_normal: "1.5",
            leading_relaxed: "1.625",
            leading_loose: "2",
            
            // Letter spacing
            tracking_tight: "-0.025em",
            tracking_normal: "0",
            tracking_wide: "0.025em",
        }
    }
}

// Typography component variants
#[derive(Debug, Clone, PartialEq)]
pub enum TypographyVariant {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Body1,
    Body2,
    Caption,
    Overline,
    Button,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypographyColor {
    Primary,
    Secondary,
    Disabled,
    Inverse,
    Success,
    Warning,
    Error,
    Info,
}

#[derive(Properties, PartialEq)]
pub struct TypographyProps {
    pub children: Children,
    pub variant: Option<TypographyVariant>,
    pub color: Option<TypographyColor>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub align: Option<String>,
    pub weight: Option<u16>,
    pub size: Option<String>,
    pub thai_optimized: Option<bool>,
}

#[function_component(Typography)]
pub fn typography(props: &TypographyProps) -> Html {
    let scale = TypographyScale::default();
    let variant = props.variant.as_ref().unwrap_or(&TypographyVariant::Body1);
    let color = props.color.as_ref().unwrap_or(&TypographyColor::Primary);
    let thai_optimized = props.thai_optimized.unwrap_or(false);
    
    let (tag, base_class, font_size, font_weight, line_height) = match variant {
        TypographyVariant::H1 => ("h1", "typography-h1", scale.text_5xl, scale.weight_extrabold, scale.leading_tight),
        TypographyVariant::H2 => ("h2", "typography-h2", scale.text_4xl, scale.weight_bold, scale.leading_tight),
        TypographyVariant::H3 => ("h3", "typography-h3", scale.text_3xl, scale.weight_bold, scale.leading_normal),
        TypographyVariant::H4 => ("h4", "typography-h4", scale.text_2xl, scale.weight_semibold, scale.leading_normal),
        TypographyVariant::H5 => ("h5", "typography-h5", scale.text_xl, scale.weight_semibold, scale.leading_normal),
        TypographyVariant::H6 => ("h6", "typography-h6", scale.text_lg, scale.weight_medium, scale.leading_normal),
        TypographyVariant::Body1 => ("p", "typography-body1", scale.text_base, scale.weight_normal, scale.leading_relaxed),
        TypographyVariant::Body2 => ("p", "typography-body2", scale.text_sm, scale.weight_normal, scale.leading_relaxed),
        TypographyVariant::Caption => ("span", "typography-caption", scale.text_xs, scale.weight_normal, scale.leading_normal),
        TypographyVariant::Overline => ("span", "typography-overline", scale.text_xs, scale.weight_medium, scale.leading_normal),
        TypographyVariant::Button => ("span", "typography-button", scale.text_sm, scale.weight_semibold, scale.leading_normal),
    };
    
    let color_class = match color {
        TypographyColor::Primary => "text-primary",
        TypographyColor::Secondary => "text-secondary",
        TypographyColor::Disabled => "text-disabled",
        TypographyColor::Inverse => "text-inverse",
        TypographyColor::Success => "text-success",
        TypographyColor::Warning => "text-warning",
        TypographyColor::Error => "text-error",
        TypographyColor::Info => "text-info",
    };
    
    let font_family = if thai_optimized {
        scale.font_thai
    } else {
        match variant {
            TypographyVariant::H1 | TypographyVariant::H2 | TypographyVariant::H3 | 
            TypographyVariant::H4 | TypographyVariant::H5 | TypographyVariant::H6 => scale.font_heading,
            _ => scale.font_body,
        }
    };
    
    let style = format!(
        "font-family: {}; font-size: {}; font-weight: {}; line-height: {}; {}",
        font_family,
        props.size.as_deref().unwrap_or(font_size),
        props.weight.unwrap_or(font_weight),
        line_height,
        props.style.as_deref().unwrap_or("")
    );
    
    let align_class = props.align.as_ref().map(|a| format!("text-{}", a)).unwrap_or_default();
    
    let classes = classes!(
        base_class,
        color_class,
        align_class,
        if thai_optimized { "thai-optimized" } else { "" },
        props.class.clone()
    );
    
    match tag {
        "h1" => html! { <h1 class={classes} {style}>{ for props.children.iter() }</h1> },
        "h2" => html! { <h2 class={classes} {style}>{ for props.children.iter() }</h2> },
        "h3" => html! { <h3 class={classes} {style}>{ for props.children.iter() }</h3> },
        "h4" => html! { <h4 class={classes} {style}>{ for props.children.iter() }</h4> },
        "h5" => html! { <h5 class={classes} {style}>{ for props.children.iter() }</h5> },
        "h6" => html! { <h6 class={classes} {style}>{ for props.children.iter() }</h6> },
        "p" => html! { <p class={classes} {style}>{ for props.children.iter() }</p> },
        _ => html! { <span class={classes} {style}>{ for props.children.iter() }</span> },
    }
}

// CSS generator for typography system
pub fn generate_typography_css(scale: &TypographyScale) -> String {
    format!(
        r#"/* Typography System - 2025 Design */

/* Font imports */
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&family=Poppins:wght@600;700;800&display=swap');
@import url('https://fonts.googleapis.com/css2?family=Sarabun:wght@300;400;500;600;700;800&family=Prompt:wght@300;400;500;600;700;800&family=Kanit:wght@300;400;500;600;700;800&display=swap');

:root {{
  /* Font families */
  --font-heading: {};
  --font-body: {};
  --font-mono: {};
  --font-thai: {};
  
  /* Font sizes */
  --text-xs: {};
  --text-sm: {};
  --text-base: {};
  --text-lg: {};
  --text-xl: {};
  --text-2xl: {};
  --text-3xl: {};
  --text-4xl: {};
  --text-5xl: {};
  
  /* Font weights */
  --weight-light: {};
  --weight-normal: {};
  --weight-medium: {};
  --weight-semibold: {};
  --weight-bold: {};
  --weight-extrabold: {};
  
  /* Line heights */
  --leading-tight: {};
  --leading-normal: {};
  --leading-relaxed: {};
  --leading-loose: {};
  
  /* Letter spacing */
  --tracking-tight: {};
  --tracking-normal: {};
  --tracking-wide: {};
}}

/* Base typography styles */
body {{
  font-family: var(--font-body);
  font-size: var(--text-base);
  font-weight: var(--weight-normal);
  line-height: var(--leading-relaxed);
  color: var(--color-text-primary);
}}

/* Typography component classes */
.typography-h1 {{
  font-family: var(--font-heading);
  font-size: var(--text-5xl);
  font-weight: var(--weight-extrabold);
  line-height: var(--leading-tight);
  margin-bottom: 1.5rem;
}}

.typography-h2 {{
  font-family: var(--font-heading);
  font-size: var(--text-4xl);
  font-weight: var(--weight-bold);
  line-height: var(--leading-tight);
  margin-bottom: 1.25rem;
}}

.typography-h3 {{
  font-family: var(--font-heading);
  font-size: var(--text-3xl);
  font-weight: var(--weight-bold);
  line-height: var(--leading-normal);
  margin-bottom: 1rem;
}}

.typography-h4 {{
  font-family: var(--font-heading);
  font-size: var(--text-2xl);
  font-weight: var(--weight-semibold);
  line-height: var(--leading-normal);
  margin-bottom: 0.875rem;
}}

.typography-h5 {{
  font-family: var(--font-heading);
  font-size: var(--text-xl);
  font-weight: var(--weight-semibold);
  line-height: var(--leading-normal);
  margin-bottom: 0.75rem;
}}

.typography-h6 {{
  font-family: var(--font-heading);
  font-size: var(--text-lg);
  font-weight: var(--weight-medium);
  line-height: var(--leading-normal);
  margin-bottom: 0.625rem;
}}

.typography-body1 {{
  font-family: var(--font-body);
  font-size: var(--text-base);
  font-weight: var(--weight-normal);
  line-height: var(--leading-relaxed);
  margin-bottom: 1rem;
}}

.typography-body2 {{
  font-family: var(--font-body);
  font-size: var(--text-sm);
  font-weight: var(--weight-normal);
  line-height: var(--leading-relaxed);
  margin-bottom: 0.875rem;
}}

.typography-caption {{
  font-family: var(--font-body);
  font-size: var(--text-xs);
  font-weight: var(--weight-normal);
  line-height: var(--leading-normal);
  color: var(--color-text-secondary);
}}

.typography-overline {{
  font-family: var(--font-body);
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  line-height: var(--leading-normal);
  text-transform: uppercase;
  letter-spacing: var(--tracking-wide);
  color: var(--color-text-secondary);
}}

.typography-button {{
  font-family: var(--font-body);
  font-size: var(--text-sm);
  font-weight: var(--weight-semibold);
  line-height: var(--leading-normal);
  letter-spacing: var(--tracking-normal);
}}

/* Thai language optimizations */
.thai-optimized {{
  font-family: var(--font-thai) !important;
  line-height: var(--leading-loose);
  word-break: break-word;
  overflow-wrap: break-word;
}}

/* Text color utilities */
.text-primary {{ color: var(--color-text-primary); }}
.text-secondary {{ color: var(--color-text-secondary); }}
.text-disabled {{ color: var(--color-text-disabled); }}
.text-inverse {{ color: var(--color-text-inverse); }}
.text-success {{ color: var(--color-success); }}
.text-warning {{ color: var(--color-warning); }}
.text-error {{ color: var(--color-error); }}
.text-info {{ color: var(--color-info); }}

/* Text alignment utilities */
.text-left {{ text-align: left; }}
.text-center {{ text-align: center; }}
.text-right {{ text-align: right; }}
.text-justify {{ text-align: justify; }}

/* Responsive typography */
@media (max-width: 768px) {{
  .typography-h1 {{ font-size: var(--text-4xl); }}
  .typography-h2 {{ font-size: var(--text-3xl); }}
  .typography-h3 {{ font-size: var(--text-2xl); }}
  .typography-h4 {{ font-size: var(--text-xl); }}
  .typography-h5 {{ font-size: var(--text-lg); }}
  .typography-h6 {{ font-size: var(--text-base); }}
}}

@media (max-width: 480px) {{
  .typography-h1 {{ font-size: var(--text-3xl); }}
  .typography-h2 {{ font-size: var(--text-2xl); }}
  .typography-h3 {{ font-size: var(--text-xl); }}
  .typography-h4 {{ font-size: var(--text-lg); }}
}}"#,
        scale.font_heading,
        scale.font_body,
        scale.font_mono,
        scale.font_thai,
        scale.text_xs,
        scale.text_sm,
        scale.text_base,
        scale.text_lg,
        scale.text_xl,
        scale.text_2xl,
        scale.text_3xl,
        scale.text_4xl,
        scale.text_5xl,
        scale.weight_light,
        scale.weight_normal,
        scale.weight_medium,
        scale.weight_semibold,
        scale.weight_bold,
        scale.weight_extrabold,
        scale.leading_tight,
        scale.leading_normal,
        scale.leading_relaxed,
        scale.leading_loose,
        scale.tracking_tight,
        scale.tracking_normal,
        scale.tracking_wide,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typography_scale_default() {
        let scale = TypographyScale::default();
        assert!(scale.font_heading.contains("Poppins"));
        assert!(scale.font_body.contains("Inter"));
        assert!(scale.font_thai.contains("Sarabun"));
    }

    #[test]
    fn test_css_generation() {
        let scale = TypographyScale::default();
        let css = generate_typography_css(&scale);
        assert!(css.contains("--font-heading"));
        assert!(css.contains("typography-h1"));
        assert!(css.contains("thai-optimized"));
    }
}
