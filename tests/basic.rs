use crossbeam::channel::{Receiver, Sender};
use cursive::backends::puppet::observed::ObservedScreen;
use cursive::backends::puppet::Backend;
use cursive::event::Event;
use cursive::views::{Panel, TextView};
use cursive::Vec2;
use cursive_flexbox::{AlignContent, AlignItems, FlexWrap, Flexbox, JustifyContent};
use insta::assert_display_snapshot;

// The TestCursive code below was copied and altered from deinstabpel/cursive-tabs.
// https://github.com/deinstapel/cursive-tabs
//
// BSD 3-Clause License
//
// Copyright (c) 2019, deinstapel
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its
//    contributors may be used to endorse or promote products derived from
//    this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
fn setup_test_environment<F>(cb: F) -> (Receiver<ObservedScreen>, Sender<Option<Event>>)
where
    F: FnOnce(&mut cursive::Cursive),
{
    let backend = Backend::init(Some(Vec2::new(20, 10)));
    let frames = backend.stream();
    let input = backend.input();
    let mut siv = cursive::Cursive::new().into_runner(backend);
    cb(&mut siv);
    input
        .send(Some(Event::Refresh))
        .expect("Refresh not accepted, backend not valid");
    siv.step();
    (frames, input)
}

struct TestCursive {
    siv: cursive::CursiveRunner<cursive::Cursive>,
    frames: Receiver<ObservedScreen>,
    input: Sender<Option<Event>>,
}

impl TestCursive {
    fn new<F>(cb: F) -> Self
    where
        F: FnOnce(&mut cursive::Cursive),
    {
        let backend = Backend::init(Some(Vec2::new(20, 10)));
        let frames = backend.stream();
        let input = backend.input();
        let mut siv = cursive::Cursive::new().into_runner(backend);
        cb(&mut siv);
        input
            .send(Some(Event::Refresh))
            .expect("Refresh not accepted, backend not valid");
        siv.step();
        Self {
            siv,
            frames,
            input,
        }
    }
    fn _call_on<F>(&mut self, cb: F)
    where
        F: FnOnce(&mut cursive::Cursive),
    {
        cb(&mut self.siv);
    }

    #[allow(dead_code)] // Unused for now, leaving it here as it could be handy later.
    fn input(&mut self, event: Event) {
        self.input
            .send(Some(event))
            .expect("Refresh not accepted, backend could not react");
        self.step();
    }

    #[allow(dead_code)] // Unused for now, leaving it here as it could be handy later.
    fn step(&mut self) {
        self.input
            .send(Some(Event::Refresh))
            .expect("Refresh not accepted, backend could not react");
        self.siv.step();
    }

    fn last_screen(&mut self) -> ObservedScreen {
        self.frames.try_iter().last().unwrap()
    }
}
// ============= End of code copied and altered from cursive-tabs. =======================

#[test]
fn test_justify_content_flexstart_single_item() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![TextView::new("Hello world!")]);
        flexbox.set_justify_content(JustifyContent::FlexStart);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_justify_content_flexend_single_item() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![TextView::new("Hello world!")]);
        flexbox.set_justify_content(JustifyContent::FlexEnd);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_justify_content_center_single_item() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![TextView::new("Hello world!")]);
        flexbox.set_justify_content(JustifyContent::Center);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_justify_content_space_between_single_item() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![TextView::new("Hello world!")]);
        flexbox.set_justify_content(JustifyContent::SpaceBetween);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_justify_content_space_around_single_item() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![TextView::new("Hello world!")]);
        flexbox.set_justify_content(JustifyContent::SpaceAround);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_justify_content_space_evenly_single_item() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![TextView::new("Hello world!")]);
        flexbox.set_justify_content(JustifyContent::SpaceEvenly);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_justify_content_flexstart_multiple_items() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![
            TextView::new("Ape"),
            TextView::new("Bat"),
            TextView::new("Cat"),
        ]);
        flexbox.set_justify_content(JustifyContent::FlexStart);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_justify_content_flexend_multiple_items() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![
            TextView::new("Ape"),
            TextView::new("Bat"),
            TextView::new("Cat"),
        ]);
        flexbox.set_justify_content(JustifyContent::FlexEnd);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_justify_content_center_multiple_items() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![
            TextView::new("Ape"),
            TextView::new("Bat"),
            TextView::new("Cat"),
        ]);
        flexbox.set_justify_content(JustifyContent::Center);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_justify_content_space_between_multiple_items() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![
            TextView::new("Ape"),
            TextView::new("Bat"),
            TextView::new("Cat"),
        ]);
        flexbox.set_justify_content(JustifyContent::SpaceBetween);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_justify_content_space_around_multiple_items() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![
            TextView::new("Ape"),
            TextView::new("Bat"),
            TextView::new("Cat"),
        ]);
        flexbox.set_justify_content(JustifyContent::SpaceAround);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_justify_content_space_evenly_multiple_items() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![
            TextView::new("Ape"),
            TextView::new("Bat"),
            TextView::new("Cat"),
        ]);
        flexbox.set_justify_content(JustifyContent::SpaceEvenly);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_align_items_flexstart() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![
            Panel::new(TextView::new("Ape")),
            Panel::new(TextView::new("Bat\nCat")),
            Panel::new(TextView::new("Dog\nEwe\nFrog")),
        ]);
        flexbox.set_align_items(AlignItems::FlexStart);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_align_items_flexend() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![
            Panel::new(TextView::new("Ape")),
            Panel::new(TextView::new("Bat\nCat")),
            Panel::new(TextView::new("Dog\nEwe\nFrog")),
        ]);
        flexbox.set_align_items(AlignItems::FlexEnd);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_align_items_center() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![
            Panel::new(TextView::new("Ape")),
            Panel::new(TextView::new("Bat\nCat")),
            Panel::new(TextView::new("Dog\nEwe\nFrog")),
        ]);
        flexbox.set_align_items(AlignItems::Center);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_align_items_stretch() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![
            Panel::new(TextView::new("Ape")),
            Panel::new(TextView::new("Bat\nCat")),
            Panel::new(TextView::new("Dog\nEwe\nFrog")),
        ]);
        flexbox.set_align_items(AlignItems::Stretch);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap())
}

#[test]
fn test_align_content_flexstart() {
    let mut tsiv = TestCursive::new(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![
            TextView::new("|Ape|"),
            TextView::new("|Bat|"),
            TextView::new("|Cat|"),
            TextView::new("|Dog|"),
            TextView::new("|Elk|"),
            TextView::new("|Fly|"),
            TextView::new("|Gnu|"),
        ]);
        flexbox.set_flex_wrap(FlexWrap::Wrap);
        flexbox.set_align_content(AlignContent::FlexStart);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(tsiv.last_screen());
}

#[test]
fn test_align_content_flexend() {
    let mut tsiv = TestCursive::new(|siv: &mut cursive::Cursive| {
        let mut flexbox = Flexbox::from(vec![
            TextView::new("|Ape|"),
            TextView::new("|Bat|"),
            TextView::new("|Cat|"),
            TextView::new("|Dog|"),
            TextView::new("|Elk|"),
            TextView::new("|Fly|"),
            TextView::new("|Gnu|"),
        ]);
        flexbox.set_flex_wrap(FlexWrap::Wrap);
        flexbox.set_align_content(AlignContent::FlexEnd);
        siv.add_fullscreen_layer(flexbox);
    });
    assert_display_snapshot!(tsiv.last_screen());
}
