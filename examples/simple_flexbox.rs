use cursive::{
    theme::{BorderStyle, ColorStyle, Palette, Theme},
    views::{Layer, Panel, TextView},
    Cursive, CursiveExt,
};
use cursive_flexbox::{AlignContent, AlignItems, FlexBox, FlexDirection, FlexWrap, JustifyContent};

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
    let mut flexbox = FlexBox::from([
        Panel::new(Layer::with_color(
            TextView::new("This is one quick line.\nHi another time :)\nAnd yet another time..."),
            ColorStyle::back(cursive::theme::BaseColor::Green.dark()),
        )),
        Panel::new(Layer::with_color(
            TextView::new("And another line just for fun."),
            ColorStyle::back(cursive::theme::BaseColor::Green.dark()),
        )),
        Panel::new(Layer::with_color(
            TextView::new("Yes I think yet another is fine.\nJust for fun!"),
            ColorStyle::back(cursive::theme::BaseColor::Green.dark()),
        )),
        Panel::new(Layer::with_color(
            TextView::new("We're just testing tings here...\n1\n2\n3"),
            ColorStyle::back(cursive::theme::BaseColor::Green.dark()),
        )),
        Panel::new(Layer::with_color(
            TextView::new("That should be enough."),
            ColorStyle::back(cursive::theme::BaseColor::Green.dark()),
        )),
    ]);

    // Set a 1 cell gap between the items on the main axis.
    flexbox.set_main_axis_gap(0);

    // Wrap main axes when there is no more space.
    flexbox.set_wrap(FlexWrap::Wrap);

    // Place items on the main axis with even spacing between them.
    flexbox.set_justify_content(JustifyContent::SpaceEvenly);

    // Center items on the cross axis.
    flexbox.set_align_items(AlignItems::Center);

    // Put equal space arount the main axes.
    flexbox.set_align_content(AlignContent::SpaceAround);

    // Set the direction of the main axis.
    flexbox.set_direction(FlexDirection::Row);

    // Add the flexbox to the ui.
    cursive.add_fullscreen_layer(flexbox);

    // Start running the eventloop.
    cursive.run();
}
