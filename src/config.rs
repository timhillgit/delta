use std::str::FromStr;

use syntect::highlighting::{Color, Style, StyleModifier, Theme, ThemeSet};
use syntect::parsing::SyntaxSet;

use crate::cli;
use crate::style;

pub struct Config<'a> {
    pub theme: Option<&'a Theme>,
    pub theme_name: &'a str,
    pub minus_style_modifier: StyleModifier,
    pub minus_emph_style_modifier: StyleModifier,
    pub plus_style_modifier: StyleModifier,
    pub plus_emph_style_modifier: StyleModifier,
    pub syntax_set: &'a SyntaxSet,
    pub terminal_width: usize,
    pub width: Option<usize>,
    pub tab_width: usize,
    pub opt: &'a cli::Opt,
    pub no_style: Style,
    pub max_buffered_lines: usize,
}

pub fn get_config<'a>(
    opt: &'a cli::Opt,
    syntax_set: &'a SyntaxSet,
    theme_set: &'a ThemeSet,
    terminal_width: usize,
    width: Option<usize>,
) -> Config<'a> {
    let theme_name = match opt.theme {
        Some(ref theme) => theme,
        None => {
            if opt.light {
                style::DEFAULT_LIGHT_THEME
            } else {
                style::DEFAULT_DARK_THEME
            }
        }
    };
    let theme = if theme_name.to_lowercase() == "none" {
        None
    } else {
        Some(&theme_set.themes[theme_name])
    };
    let is_light_theme = if theme.is_none() {
        !opt.dark
    } else {
        style::LIGHT_THEMES.contains(&theme_name)
    };

    let minus_style_modifier = StyleModifier {
        background: Some(color_from_arg(
            opt.minus_color.as_ref(),
            is_light_theme,
            style::LIGHT_THEME_MINUS_COLOR,
            style::DARK_THEME_MINUS_COLOR,
        )),
        foreground: if opt.highlight_removed {
            None
        } else {
            Some(style::NO_COLOR)
        },
        font_style: None,
    };

    let minus_emph_style_modifier = StyleModifier {
        background: Some(color_from_arg(
            opt.minus_emph_color.as_ref(),
            is_light_theme,
            style::LIGHT_THEME_MINUS_EMPH_COLOR,
            style::DARK_THEME_MINUS_EMPH_COLOR,
        )),
        foreground: if opt.highlight_removed {
            None
        } else {
            Some(style::NO_COLOR)
        },
        font_style: None,
    };

    let plus_style_modifier = StyleModifier {
        background: Some(color_from_arg(
            opt.plus_color.as_ref(),
            is_light_theme,
            style::LIGHT_THEME_PLUS_COLOR,
            style::DARK_THEME_PLUS_COLOR,
        )),
        foreground: None,
        font_style: None,
    };

    let plus_emph_style_modifier = StyleModifier {
        background: Some(color_from_arg(
            opt.plus_emph_color.as_ref(),
            is_light_theme,
            style::LIGHT_THEME_PLUS_EMPH_COLOR,
            style::DARK_THEME_PLUS_EMPH_COLOR,
        )),
        foreground: None,
        font_style: None,
    };

    Config {
        theme,
        theme_name,
        minus_style_modifier,
        minus_emph_style_modifier,
        plus_style_modifier,
        plus_emph_style_modifier,
        terminal_width,
        width,
        tab_width: opt.tab_width,
        syntax_set,
        opt,
        no_style: style::get_no_style(),
        max_buffered_lines: 32,
    }
}

fn color_from_arg(
    arg: Option<&String>,
    is_light_theme: bool,
    light_theme_default: Color,
    dark_theme_default: Color,
) -> Color {
    arg.and_then(|s| Color::from_str(s).ok())
        .unwrap_or_else(|| {
            if is_light_theme {
                light_theme_default
            } else {
                dark_theme_default
            }
        })
}
