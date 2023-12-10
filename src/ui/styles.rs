use bevy::{prelude::*, text::BreakLineOn};

/// Display: Flex centered on screen
pub fn root_full_screen(justify: Option<JustifyContent>, align: Option<AlignItems>) -> impl Fn(&mut NodeBundle) {
    move |node: &mut NodeBundle| {
        node.style = Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: justify.unwrap_or(JustifyContent::Center),
            align_items: align.unwrap_or(AlignItems::Center),
            ..default()
        };
    }
}

pub fn c_pixel_text(_: &AssetServer, tb: &mut TextBundle) {
    tb.style = Style {
        border: UiRect::all(Val::Px(1.0)),
        padding: UiRect::all(Val::Px(5.0)),
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    tb.text.alignment = TextAlignment::Center;
    tb.text.linebreak_behavior = BreakLineOn::WordBoundary;
    tb.background_color = BackgroundColor(Color::rgba(0., 0., 0., 0.85));
}

pub fn text_style(assets: &AssetServer, ts: &mut TextStyle) {
    ts.font = assets.load("pixel.ttf");
    ts.font_size = 40.0;
}
