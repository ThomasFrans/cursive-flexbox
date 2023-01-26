use cursive::{
    theme::{BorderStyle, ColorStyle, Palette, Theme},
    view::IntoBoxedView,
    views::{Layer, Panel, TextView},
    Cursive, CursiveExt,
};
use cursive_flexbox::{AlignContent, AlignItems, FlexDirection, FlexWrap, Flexbox, JustifyContent};

fn main() {
    // Create a cursive object. This is the basic object that handles the ui and event loop.
    let mut cursive = Cursive::new();

    cursive.add_global_callback('q', |cursive| cursive.quit());

    cursive.set_theme(Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: Palette::terminal_default(),
    });

    // Create the flexbox and put some items in it.
    let mut flexbox = Flexbox::from(vec![
        Panel::new(Layer::with_color(
            TextView::new("This is one quick line.\nAnother quick line.\nAnd yet another line."),
            ColorStyle::back(cursive::theme::BaseColor::Green.dark()),
        ))
        .into_boxed_view(),
        Panel::new(Layer::with_color(
            TextView::new("I doubt I will be wrapped..."),
            ColorStyle::back(cursive::theme::BaseColor::Green.dark()),
        ))
        .into_boxed_view(),
        Panel::new(Layer::with_color(
            TextView::new("Flexing a flexbox."),
            ColorStyle::back(cursive::theme::BaseColor::Green.dark()),
        ))
        .into_boxed_view(),
        Panel::new(Layer::with_color(
            TextView::new(
                "And a bigger container\nto test out the alignment\nof items in the main \
                          axis\na bit better.",
            ),
            ColorStyle::back(cursive::theme::BaseColor::Green.dark()),
        ))
        .into_boxed_view(),
        Panel::new(Layer::with_color(
            TextView::new("And a final item for good luck."),
            ColorStyle::back(cursive::theme::BaseColor::Green.dark()),
        ))
        .into_boxed_view(),
    ]);

    // Set a gap between the items on the main axis.
    flexbox.set_main_axis_gap(2);

    // Set a gap between the main axis (along the container cross axis).
    flexbox.set_cross_axis_gap(2);

    // Set item grow factors.
    flexbox.set_flex_grow(1, 1);
    flexbox.set_flex_grow(2, 2);

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
