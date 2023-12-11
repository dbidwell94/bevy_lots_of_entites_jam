use bevy::{prelude::*, text::BreakLineOn};

/// Display: Flex centered on screen
pub fn root_full_screen(
    justify: Option<JustifyContent>,
    align: Option<AlignItems>,
) -> impl Fn(&mut NodeBundle) {
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

pub fn text_style(
    font_size: Option<f32>,
) -> impl Fn(&bevy::prelude::AssetServer, &mut bevy::prelude::TextStyle) {
    return move |assets: &AssetServer, ts: &mut TextStyle| {
        ts.font = assets.load("pixel.ttf");
        ts.font_size = font_size.unwrap_or(40.0);
    };
}

pub fn top_right_anchor(node: &mut NodeBundle) {
    node.style = Style {
        display: Display::Flex,
        position_type: PositionType::Absolute,
        top: Val::Percent(0.),
        right: Val::Percent(0.),
        width: Val::Auto,
        height: Val::Auto,
        ..default()
    };
}

pub fn bottom_center_anchor(node: &mut NodeBundle) {
    top_right_anchor(node);
    node.style.position_type = PositionType::Absolute;
    node.style.top = Val::Auto;
    node.style.bottom = Val::Percent(0.);
    node.style.right = Val::Auto;
    node.style.left = Val::Auto;
    node.style.margin = UiRect::all(Val::Px(5.0));

    node.style.justify_self = JustifySelf::End;
}

pub fn spawn_menu_button(
    image_location: Option<&'static str>,
) -> impl Fn(&AssetServer, &mut ButtonBundle) {
    return move |assets: &AssetServer, b: &mut ButtonBundle| {
        b.image = UiImage {
            texture: assets.load(image_location.unwrap_or_default()),
            ..default()
        };
        b.style = Style {
            width: Val::Px(50.0),
            height: Val::Px(50.0),
            display: Display::Flex,
            padding: UiRect::all(Val::Px(10.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        b.border_color = BorderColor(Color::WHITE);
    };
}
