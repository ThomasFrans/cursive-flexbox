use cursive::{
    theme::{BorderStyle, Palette, Theme},
    view::IntoBoxedView,
    views::{Button, TextContent, TextView},
    Cursive, CursiveExt,
};

use cursive_flexbox::prelude::*;

fn main() {
    // Create a cursive object. This is the basic object that handles the ui and event loop.
    let mut cursive = Cursive::new();

    cursive.add_global_callback('q', |cursive| cursive.quit());

    cursive.set_theme(Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: Palette::terminal_default(),
    });

    let content = TextContent::new("unset");
    let textview = TextView::new_with_content(content.clone());

    let content_clone = content.clone();
    let button1 = Button::new_raw("Set text 'hello'.", move |_| {
        content_clone.set_content("hello");
    });

    let button2 = Button::new_raw("Set text 'world'.", move |_| {
        content.set_content("world");
    });

    // Create the flexbox and put some items in it.
    let mut flexbox = Flexbox::from(vec![
        button1.into_boxed_view(),
        button2.into_boxed_view(),
        textview.into_boxed_view(),
    ]);

    // Set a gap between the items on the main axis.
    flexbox.set_main_axis_gap(2);

    // Set a gap between the main axis (along the container cross axis).
    flexbox.set_cross_axis_gap(2);

    // Set item grow factors.
    // flexbox.set_flex_grow(1, 1);
    // flexbox.set_flex_grow(2, 2);

    // Set the wrapping behavior of the main axes.
    flexbox.set_flex_wrap(FlexWrap::Wrap);

    // Set the algorithm to assign free space along the main axis.
    flexbox.set_justify_content(JustifyContent::SpaceEvenly);

    // Set the algorithm to assign free space along the cross axis for flex items.
    flexbox.set_align_items(AlignItems::Center);

    // Set the algorithm to assign free space along the cross axis in the container.
    flexbox.set_align_content(AlignContent::FlexStart);

    // Set the direction of the main axis.
    flexbox.set_flex_direction(FlexDirection::Row);

    // Add the flexbox to the ui.
    cursive.add_fullscreen_layer(flexbox);

    // Start running the eventloop.
    cursive.run();
}
