use qrcode_lib::fancy::{ModuleShape, FinderShape, FancyOptions};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum QrStyle {
    Standard,
    MinimalLogo,
    GradientLogo,
    Premium,
    BrandedFinders,
    MinimalFinders,
    GradientFinders,
    GradientMinimal,
}

impl QrStyle {
    pub fn name(&self) -> &'static str {
        match self {
            QrStyle::Standard => "Standard with Logo",
            QrStyle::MinimalLogo => "Minimal with Logo",
            QrStyle::GradientLogo => "Gradient with Logo",
            QrStyle::Premium => "Ultra Premium",
            QrStyle::BrandedFinders => "Branded Finders",
            QrStyle::MinimalFinders => "Minimal Finders",
            QrStyle::GradientFinders => "Gradient Finders",
            QrStyle::GradientMinimal => "Gradient Minimal",
        }
    }
}

pub fn get_style_options(style: QrStyle, logo_base64: &str) -> FancyOptions {
    let mut options = FancyOptions::default();
    
    match style {
        QrStyle::Standard => {
            options.color_background = "#FFFFFF".to_string();
            options.color_data = "#4d3695".to_string();
            options.color_finder = "#4d3695".to_string();
            options.shape_module = ModuleShape::RoundedSquare(0.3);
            options.shape_finder = FinderShape::Rounded(1.5);
            if !logo_base64.is_empty() {
                options.center_image_url = Some(logo_base64.to_string());
                options.overlay_scale = 0.3;
            }
        },
        QrStyle::MinimalLogo => {
            options.color_background = "#FFFFFF".to_string();
            options.color_data = "#000000".to_string();
            options.color_finder = "#4d3695".to_string();
            options.shape_module = ModuleShape::Square;
            options.shape_finder = FinderShape::Rounded(1.0);
            if !logo_base64.is_empty() {
                options.center_image_url = Some(logo_base64.to_string());
                options.overlay_scale = 0.25;
            }
        },
        QrStyle::GradientLogo => {
            options.color_background = "#F5F3FF".to_string();
            options.color_data = "#4d3695".to_string();
            options.color_finder = "#5B34A8".to_string();
            options.shape_module = ModuleShape::Circle;
            options.shape_finder = FinderShape::Rounded(2.0);
            if !logo_base64.is_empty() {
                options.center_image_url = Some(logo_base64.to_string());
                options.overlay_scale = 0.28;
            }
        },
        QrStyle::Premium => {
            options.color_background = "#FFFFFF".to_string();
            options.color_data = "#4d3695".to_string();
            options.color_finder = "#4d3695".to_string();
            options.shape_module = ModuleShape::RoundedSquare(0.35);
            options.shape_finder = FinderShape::Rounded(1.8);
            if !logo_base64.is_empty() {
                options.center_image_url = Some(logo_base64.to_string());
                options.overlay_scale = 0.26;
            }
        },
        QrStyle::BrandedFinders => {
            options.color_background = "#FFFFFF".to_string();
            options.color_data = "#1a1a1a".to_string();
            options.color_finder = "#4d3695".to_string();
            options.shape_module = ModuleShape::RoundedSquare(0.25);
            options.shape_finder = FinderShape::Rounded(2.2);
            options.overlay_scale = 0.0;
        },
        QrStyle::MinimalFinders => {
            options.color_background = "#FFFFFF".to_string();
            options.color_data = "#000000".to_string();
            options.color_finder = "#4d3695".to_string();
            options.shape_module = ModuleShape::Square;
            options.shape_finder = FinderShape::Rounded(1.5);
            options.overlay_scale = 0.0;
        },
        QrStyle::GradientFinders => {
            options.color_background = "#FAF5FF".to_string();
            options.color_data = "#6B4B8A".to_string();
            options.color_finder = "#4d3695".to_string();
            options.shape_module = ModuleShape::Circle;
            options.shape_finder = FinderShape::Rounded(2.5);
            options.overlay_scale = 0.0;
        },
        QrStyle::GradientMinimal => {
            options.color_background = "#FAF5FF".to_string();
            options.color_data = "#6B4B8A".to_string();
            options.color_finder = "#4d3695".to_string();
            options.shape_module = ModuleShape::Square;
            options.shape_finder = FinderShape::Rounded(1.5);
            if !logo_base64.is_empty() {
                options.center_image_url = Some(logo_base64.to_string());
                options.overlay_scale = 0.25;
            }
        },
    }
    options
}

pub fn get_custom_style_options(
    style: QrStyle, 
    logo_base64: &str,
    background_color: &str,
    data_color: &str,
    finder_color: &str
) -> FancyOptions {
    let mut options = get_style_options(style, logo_base64);
    
    // Override with custom colors if they're not empty
    if !background_color.is_empty() {
        options.color_background = background_color.to_string();
    }
    if !data_color.is_empty() {
        options.color_data = data_color.to_string();
    }
    if !finder_color.is_empty() {
        options.color_finder = finder_color.to_string();
    }
    
    options
}

