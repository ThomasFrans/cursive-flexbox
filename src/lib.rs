//! A [flexbox layout](https://developer.mozilla.org/en-US/docs/Learn/CSS/CSS_layout/Flexbox)
//! implementation for the [Rust Cursive TUI library](https://crates.io/crates/cursive) that tries
//! to adhere to the [CSS3 specification](https://w3c.github.io/csswg-drafts/css-flexbox/#intro)
//! as much as possible and where it makes sense for a TUI. Users who are already
//! familiar with it should feel right at home working with this library.

#![warn(
    missing_docs,
    future_incompatible,
    rust_2018_idioms,
    let_underscore,
    clippy::missing_docs_in_private_items
)]

mod layout;
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub mod prelude;

use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};

use cursive_core::{event::EventResult, view::IntoBoxedView, Rect, Vec2, View, XY};
use layout::{Layout, PlacedElement};

/// A container that can be used to display a list of items in a flexible way.
#[derive(Default)]
pub struct Flexbox {
    /// The content of the flexbox. Unlike some flexboxes, order is always dictated by the order of
    /// the items in `content`. There is no way to overwrite this.
    content: Vec<Rc<RefCell<FlexItem>>>,
    /// Options to alter the behavior.
    options: FlexBoxOptions,
    /// The currently active view.
    focused: Option<usize>,
    /// The actual layout of the items.
    layout: Option<Layout<Rc<RefCell<FlexItem>>>>,
}

/// A single item in a Flexbox.
pub struct FlexItem {
    /// The actual view represented by this flex item.
    view: Box<dyn View>,
    /// A relative amount of free space in the main axis this item is in that should be given to
    /// this item. The amount is relative as it's proportional to the total amount of free space
    /// requested by all items in the same main axis.
    flex_grow: u8,
}

/// Options that can alter the behavior of a flexbox.
#[derive(Default, Clone, Copy)]
struct FlexBoxOptions {
    /// The direction of the main axis.
    direction: FlexDirection,
    /// Algorithm that assigns extra space on the main axis. This does nothing if any of the items
    /// on a main axis request extra space with flex-grow.
    justification: JustifyContent,
    /// How to place items on the cross axis.
    item_alignment: AlignItems,
    /// How to place the main axes in the container.
    axes_alignment: AlignContent,
    /// Gap between items on the main axis. The gap doesn't get added to the sides.
    main_axis_gap: u32,
    /// Gap between the main axes.
    cross_axis_gap: u32,
    /// Wrapping behavior of the main axes.
    wrap: FlexWrap,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction
// https://w3c.github.io/csswg-drafts/css-flexbox/#flex-direction-property
/// Direction of a flex container's main axis.
#[non_exhaustive] // TODO: Implement RowReverse and ColumnReverse!
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlexDirection {
    /// Flex items are layed out in a row.
    #[default]
    Row,
    // /// Flex items are layed out in a row, in reverse order.
    // RowReverse,
    /// Flex items are layed out in a column.
    Column,
    // /// Flex items are layed out in a column, in reverse order.
    // ColumnReverse,
}

impl Display for FlexDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Row => "row",
                Self::Column => "column",
            }
        )
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/flex-wrap
// https://w3c.github.io/csswg-drafts/css-flexbox/#flex-wrap-property
/// Wrapping behavior and direction of a flexbox container's main axis.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlexWrap {
    /// Don't wrap the main axis.
    #[default]
    NoWrap,
    /// Wrap the main axis.
    Wrap,
    /// Wrap the main axis in the opposite direction.
    WrapReverse,
}

impl Display for FlexWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NoWrap => "nowrap",
                Self::Wrap => "wrap",
                Self::WrapReverse => "wrap-reverse",
            }
        )
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content
// https://w3c.github.io/csswg-drafts/css-flexbox/#propdef-justify-content
/// Alignment of items in a flexbox along the main axis.
#[non_exhaustive] // Specification lists more options. Might be added later.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JustifyContent {
    /// Flex items are packed against the start of the container.
    #[default] // Following w3c specification as there is no 'normal' option.
    FlexStart,
    /// Flex items are packed against the end of the container.
    FlexEnd,
    /// Flex items are packed in the center, with equal space to either side.
    Center,
    /// Flex items are packed with equal space between them.
    SpaceBetween,
    /// Flex items are packed with equal space around each item.
    SpaceAround,
    /// Flex items are packed with equal space between all items (including the sides).
    SpaceEvenly, // Included although not in w3c specification.
}

impl Display for JustifyContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::FlexStart => "flex-start",
                Self::FlexEnd => "flex-end",
                Self::Center => "center",
                Self::SpaceBetween => "space-between",
                Self::SpaceAround => "space-around",
                Self::SpaceEvenly => "space-evenly",
            }
        )
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/align-items
// https://w3c.github.io/csswg-drafts/css-flexbox/#align-items-property
// Baseline isn't included as Cursive doesn't support it, and it makes little sense in a TUI.
/// Alignment of items in a flexbox along the cross axis.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlignItems {
    /// Align flex items at the start of the cross axis.
    FlexStart,
    /// Align flex items at the end of the cross axis.
    FlexEnd,
    /// Align flex items at the center of the cross axis.
    Center,
    /// Stretch flex items to fill all the space along the cross axis.
    #[default] // Following w3c specification as there is no 'normal' option.
    Stretch,
}

impl Display for AlignItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::FlexStart => "flex-start",
                Self::FlexEnd => "flex-end",
                Self::Center => "center",
                Self::Stretch => "stretch",
            }
        )
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/align-content
// https://w3c.github.io/csswg-drafts/css-flexbox/#align-content-property
/// Alignment of the main axes in a flexbox.
#[non_exhaustive] // Might add space-evenly, even though not in w3c specification.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlignContent {
    /// Align content to the start of the container.
    #[default]
    FlexStart,
    /// Align content to the end of the container.
    FlexEnd,
    /// Align content to the center of the container.
    Center,
    /// Stretch content along the cross axis.
    Stretch,
    /// Align main axis with an equal amount of space between them.
    SpaceBetween,
    /// Align main axis with an equal of margin per axis.
    SpaceAround,
}

impl Display for AlignContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::FlexStart => "flex-start",
                Self::FlexEnd => "flex-end",
                Self::Center => "center",
                Self::Stretch => "stretch",
                Self::SpaceBetween => "space-between",
                Self::SpaceAround => "space-around",
            }
        )
    }
}

/// An actual layout of a flexbox with real dimensions.
/// <https://developer.mozilla.org/en-US/docs/Learn/CSS/CSS_layout/Flexbox#the_flex_model>
#[derive(Default)]
struct FlexboxLayout {
    /// The dimensions of the container.
    size: XY<usize>,
    /// Options for this particular layout of the flexbox.
    options: FlexBoxOptions,
    /// Parts that together form the entire main axis of this flexbox.
    main_axes: Vec<MainAxis>,
}

/// Any error that can arise from operations on a flexbox.
#[derive(Debug)]
enum FlexboxError {
    /// Error when trying to add too many items to one axis.
    AxisFull,
}

impl FlexboxLayout {
    /// Return all the child items along with their absolute position. This makes drawing the
    /// flexbox very simple.
    pub fn windows(&mut self) -> Vec<(Rc<RefCell<FlexItem>>, Rect)> {
        let mut windows = Vec::new();
        let mut cross_offset = 0;
        let mut assignable_free_space = self.cross_axis_free_space();

        for (axis_index, axis) in self.main_axes.iter().enumerate() {
            match self.options.axes_alignment {
                AlignContent::FlexEnd => {
                    if assignable_free_space > 0 {
                        cross_offset += assignable_free_space;
                    }
                    assignable_free_space = 0;
                },
                AlignContent::Center => {
                    if assignable_free_space > 0 {
                        cross_offset += assignable_free_space / 2;
                    }
                    assignable_free_space = 0;
                },
                AlignContent::SpaceAround => {
                    let assigned_space =
                        assignable_free_space / (self.main_axes.len() * 2 - axis_index * 2);
                    if assignable_free_space > 0 {
                        cross_offset += assigned_space;
                    }
                    assignable_free_space -= assigned_space;
                },
                _ => {},
            }
            for mut combo in axis.windows(self) {
                match self.options.direction {
                    FlexDirection::Row => combo.1.offset(XY::from((0, cross_offset))),
                    FlexDirection::Column => combo.1.offset(XY::from((cross_offset, 0))),
                }
                windows.push(combo);
            }
            match self.options.axes_alignment {
                AlignContent::SpaceBetween => {
                    if assignable_free_space > 0 {
                        let assigned_space =
                            assignable_free_space / (self.main_axes.len() - axis_index - 1);
                        if assignable_free_space > 0 {
                            cross_offset += assigned_space;
                        }
                        assignable_free_space -= assigned_space;
                    }
                },
                AlignContent::Stretch => {
                    let assigned_space =
                        assignable_free_space / (self.main_axes.len() - axis_index);
                    if assignable_free_space > 0 {
                        cross_offset += assigned_space;
                    }
                    assignable_free_space -= assigned_space;
                },
                AlignContent::SpaceAround => {
                    let assigned_space =
                        assignable_free_space / (self.main_axes.len() * 2 - (axis_index * 2 + 1));
                    if assignable_free_space > 0 {
                        cross_offset += assigned_space;
                    }
                    assignable_free_space -= assigned_space;
                },
                _ => {},
            }
            cross_offset += axis.cross_axis_size(self) + self.options.cross_axis_gap as usize;
        }

        windows
    }

    /// Return the amount of left over space on the cross axis.
    pub fn cross_axis_free_space(&self) -> usize {
        let mut used_space = 0;

        for axis in &self.main_axes {
            used_space += axis.cross_axis_size(self);
        }

        used_space += (self.main_axis_count() - 1) * self.options.cross_axis_gap as usize;

        match self.options.direction {
            FlexDirection::Row => self.size.y.saturating_sub(used_space),
            FlexDirection::Column => self.size.x.saturating_sub(used_space),
        }
    }

    /// Generate the actual layout for the flexbox with `content` and given `width` and `height`.
    pub fn generate(
        content: &[Weak<RefCell<FlexItem>>],
        width: usize,
        height: usize,
        options: FlexBoxOptions,
    ) -> Rc<RefCell<Self>> {
        let layout = Rc::new(RefCell::new(FlexboxLayout {
            size: XY::from((width, height)),
            options,
            main_axes: Vec::new(),
        }));

        // TODO: This is a bit (very much) anti-idiomatic Rust...

        let mut added = 0;
        let length = content.len();

        while added < length {
            let mut main_axis = MainAxis::new(Rc::downgrade(&layout));

            loop {
                let result =
                    main_axis.add_item(content[added].clone(), &mut RefCell::borrow_mut(&layout));
                if result.is_err() {
                    // If the current main axis couldn't hold the item anymore.
                    break;
                } else if added + 1 == length {
                    // If this was the last element to add to the flexbox.
                    added += 1;
                    break;
                } else {
                    // If the current main axis could still hold the item.
                    added += 1;
                }
            }

            // PERF: Inserting elements at the front isn't ideal for performance.
            match options.wrap {
                FlexWrap::NoWrap | FlexWrap::Wrap => {
                    RefCell::borrow_mut(&layout).main_axes.push(main_axis)
                },
                FlexWrap::WrapReverse => {
                    RefCell::borrow_mut(&layout).main_axes.insert(0, main_axis)
                },
            }
        }

        layout
    }

    /// Return the size of a [FlexItem] along the main axis.
    pub fn flexitem_main_axis_size(&self, item: &mut FlexItem) -> usize {
        match self.options.direction {
            FlexDirection::Row => item.view.required_size(self.size).x,
            FlexDirection::Column => item.view.required_size(self.size).y,
        }
    }

    /// Return the amount of main axes in this layout.
    pub fn main_axis_count(&self) -> usize {
        self.main_axes.len()
    }
}

/// A single main axis of a flexbox. In a flexbox without wrap, this will be the only main axis and
/// contain all the items. In a flexbox with wrap, this axis will only hold as many items as it can
/// accomodate given the size of the main axis and the gapsize of the main axis.
struct MainAxis {
    /// The items in this main axis.
    items: Vec<Weak<RefCell<FlexItem>>>,
    /// Cache value for the remaining free space in this axis.
    free_space: usize,
}

impl MainAxis {
    /// Create a new main axis part for the given layout.
    pub fn new(layout: Weak<RefCell<FlexboxLayout>>) -> Self {
        let layout_upgraded = layout.upgrade().unwrap();
        let free_space = match RefCell::borrow(&layout_upgraded).options.direction {
            FlexDirection::Row => RefCell::borrow(&layout_upgraded).size.x,
            FlexDirection::Column => RefCell::borrow(&layout_upgraded).size.y,
        };
        MainAxis {
            items: Vec::new(),
            free_space,
        }
    }

    /// Return the cross axis size. The size of the cross axis is the maximum size of its elements
    /// along the cross axis.
    pub fn cross_axis_size(&self, layout: &FlexboxLayout) -> usize {
        let mut maximum_item_cross_axis_size = 0;
        match layout.options.direction {
            FlexDirection::Row => {
                for item in &self.items {
                    maximum_item_cross_axis_size = maximum_item_cross_axis_size.max(
                        RefCell::borrow_mut(&item.upgrade().unwrap())
                            .view
                            .required_size(layout.size)
                            .y,
                    );
                }
            },
            FlexDirection::Column => {
                for item in &self.items {
                    maximum_item_cross_axis_size = maximum_item_cross_axis_size.max(
                        RefCell::borrow_mut(&item.upgrade().unwrap())
                            .view
                            .required_size(layout.size)
                            .x,
                    );
                }
            },
        }

        maximum_item_cross_axis_size
    }

    /// Returns the flexitems and their corresponding windows in the local coordinates (relative to
    /// the topleft of the bounding box of this axis.
    pub fn windows(&self, layout: &FlexboxLayout) -> Vec<(Rc<RefCell<FlexItem>>, Rect)> {
        let mut windows = Vec::new();
        let mut offset = 0;
        let mut assignable_free_space = self.free_space;
        let combined_grow_factor = self.combined_grow_factor();
        let mut remaining_grow_factor = combined_grow_factor;
        let cross_axis_size = self.cross_axis_size(layout);

        for (item_index, item) in self
            .items
            .iter()
            .map(|item| item.upgrade().unwrap())
            .enumerate()
        {
            let mut start_x = 0;
            let mut start_y = 0;
            let mut width = 1;
            let mut height = 1;

            if combined_grow_factor > 0 {
                // Axis contains elements that want the free space. Give it to them, don't use
                // justify-content.

                let mut current_item_assigned_space = 0;
                let item_main_axis_size =
                    layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&item));
                if remaining_grow_factor > 0 {
                    current_item_assigned_space =
                        ((RefCell::borrow(&item).flex_grow as f64 / remaining_grow_factor as f64)
                            * assignable_free_space as f64) as usize;
                }

                match layout.options.direction {
                    FlexDirection::Row => {
                        start_x = offset;
                        width = item_main_axis_size + current_item_assigned_space;
                    },
                    FlexDirection::Column => {
                        start_y = offset;
                        height = item_main_axis_size + current_item_assigned_space;
                    },
                }
                offset += item_main_axis_size
                    + layout.options.main_axis_gap as usize
                    + current_item_assigned_space;
                assignable_free_space -= current_item_assigned_space;
                remaining_grow_factor -= RefCell::borrow(&item).flex_grow as usize;
            } else {
                // Axis doesn't contain elements that want free space. Use justify-content property
                // to decide positioning.

                match layout.options.direction {
                    FlexDirection::Row => {
                        width = RefCell::borrow_mut(&item).view.required_size(layout.size).x;
                    },
                    FlexDirection::Column => {
                        height = RefCell::borrow_mut(&item).view.required_size(layout.size).y;
                    },
                }

                // Decides `start_x`, `width` is item's preferred width.
                match layout.options.justification {
                    JustifyContent::FlexStart => {
                        match layout.options.direction {
                            FlexDirection::Row => {
                                start_x = offset;
                            },
                            FlexDirection::Column => {
                                start_y = offset;
                            },
                        }

                        offset += layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&item))
                            + layout.options.main_axis_gap as usize;
                    },
                    JustifyContent::FlexEnd => {
                        if assignable_free_space > 0 {
                            offset = assignable_free_space;
                            assignable_free_space = 0;
                        }
                        match layout.options.direction {
                            FlexDirection::Row => {
                                start_x = offset;
                            },
                            FlexDirection::Column => {
                                start_y = offset;
                            },
                        }

                        offset += layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&item))
                            + layout.options.main_axis_gap as usize;
                    },
                    JustifyContent::Center => {
                        if assignable_free_space > 0 {
                            offset = assignable_free_space / 2;
                            assignable_free_space = 0;
                        }

                        match layout.options.direction {
                            FlexDirection::Row => {
                                start_x = offset;
                            },
                            FlexDirection::Column => {
                                start_y = offset;
                            },
                        }

                        offset += layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&item))
                            + layout.options.main_axis_gap as usize;
                    },
                    JustifyContent::SpaceBetween => {
                        match layout.options.direction {
                            FlexDirection::Row => {
                                start_x = offset;
                            },
                            FlexDirection::Column => {
                                start_y = offset;
                            },
                        }

                        if assignable_free_space > 0 && item_index + 1 < self.number_of_items() {
                            let extra_free_space = assignable_free_space
                                / (self.number_of_items().saturating_sub(1 + item_index));
                            assignable_free_space -= extra_free_space;
                            offset += extra_free_space;
                        }
                        offset += layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&item))
                            + layout.options.main_axis_gap as usize;
                    },
                    JustifyContent::SpaceAround => {
                        let mut extra_free_space =
                            assignable_free_space / (self.number_of_items() * 2 - item_index * 2);
                        if assignable_free_space > 0 {
                            offset += extra_free_space;
                        }
                        assignable_free_space -= extra_free_space;

                        match layout.options.direction {
                            FlexDirection::Row => {
                                start_x = offset;
                            },
                            FlexDirection::Column => {
                                start_y = offset;
                            },
                        }

                        extra_free_space = assignable_free_space
                            / (self.number_of_items() * 2 - (item_index * 2 + 1));
                        if assignable_free_space > 0 {
                            offset += extra_free_space;
                        }
                        assignable_free_space -= extra_free_space;

                        offset += layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&item))
                            + layout.options.main_axis_gap as usize;
                    },
                    JustifyContent::SpaceEvenly => {
                        let extra_free_space =
                            assignable_free_space / (self.number_of_items() + 1 - item_index);
                        if assignable_free_space > 0 {
                            offset += extra_free_space;
                        }
                        assignable_free_space -= extra_free_space;

                        match layout.options.direction {
                            FlexDirection::Row => {
                                start_x = offset;
                            },
                            FlexDirection::Column => {
                                start_y = offset;
                            },
                        }

                        offset += layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&item))
                            + layout.options.main_axis_gap as usize;
                    },
                }
            }

            // Decides `start_y` and `height`. Item's `layout()` called with this calculated height
            // later.
            match layout.options.item_alignment {
                AlignItems::FlexStart => match layout.options.direction {
                    FlexDirection::Row => {
                        start_y = 0;
                        height = RefCell::borrow_mut(&item).view.required_size(layout.size).y;
                    },
                    FlexDirection::Column => {
                        start_x = 0;
                        width = RefCell::borrow_mut(&item).view.required_size(layout.size).x;
                    },
                },
                AlignItems::FlexEnd => match layout.options.direction {
                    FlexDirection::Row => {
                        height = RefCell::borrow_mut(&item).view.required_size(layout.size).y;
                        start_y = cross_axis_size - height;
                    },
                    FlexDirection::Column => {
                        width = RefCell::borrow_mut(&item).view.required_size(layout.size).x;
                        start_x = cross_axis_size - width;
                    },
                },
                AlignItems::Center => match layout.options.direction {
                    FlexDirection::Row => {
                        height = RefCell::borrow_mut(&item).view.required_size(layout.size).y;
                        start_y = (cross_axis_size - height) / 2;
                    },
                    FlexDirection::Column => {
                        width = RefCell::borrow_mut(&item).view.required_size(layout.size).x;
                        start_x = (cross_axis_size - width) / 2;
                    },
                },
                AlignItems::Stretch => match layout.options.direction {
                    FlexDirection::Row => {
                        height = cross_axis_size;
                        start_y = 0;
                    },
                    FlexDirection::Column => {
                        width = cross_axis_size;
                        start_x = 0;
                    },
                },
            }

            RefCell::borrow_mut(&item)
                .view
                .layout((width, height).into());
            windows.push((item, Rect::from_size((start_x, start_y), (width, height))));
        }

        windows
    }

    /// Try to add `item` to this main axis, fail if this axis can't accomodate the item.
    pub fn add_item(
        &mut self,
        item: Weak<RefCell<FlexItem>>,
        layout: &mut FlexboxLayout,
    ) -> Result<(), FlexboxError> {
        let upgraded_item = item.upgrade().unwrap();
        if self.can_accomodate(&mut RefCell::borrow_mut(&upgraded_item), layout) {
            self.free_space = self.free_space.saturating_sub(
                layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&upgraded_item)),
            );

            // Only add gaps if there is already an item.
            if self.number_of_items() >= 1 {
                self.free_space = self
                    .free_space
                    .saturating_sub(layout.options.main_axis_gap as usize);
            }

            self.items.push(item);

            Ok(())
        } else {
            Err(FlexboxError::AxisFull)
        }
    }

    /// Return whether this axis can accomodate `item` with the amount of free space it has left. A
    /// main axis can accomodate an item if either it is the first axis in a non-wrapped flexbox,
    /// or it has enough space for the item and possible gap that would be added.
    pub fn can_accomodate(&self, item: &mut FlexItem, layout: &mut FlexboxLayout) -> bool {
        if let FlexWrap::NoWrap = layout.options.wrap {
            // There can only be one main axis in a non-wrapping layout.
            layout.main_axes.len() == 1
        } else if self.items.is_empty() {
            // Each main axis must be able to hold at least one item!
            true
        } else {
            let extra_used_space = if self.number_of_items() >= 1 {
                layout.flexitem_main_axis_size(item) + layout.options.main_axis_gap as usize
            } else {
                layout.flexitem_main_axis_size(item)
            };
            extra_used_space <= self.free_space
        }
    }

    /// Return the number of items on this axis.
    pub fn number_of_items(&self) -> usize {
        self.items.len()
    }

    /// Sum of the flex-grow of all the [FlexItem]s in this axis.
    pub fn combined_grow_factor(&self) -> usize {
        let mut total_grow_factor = 0usize;
        self.items.iter().for_each(|item| {
            total_grow_factor += RefCell::borrow(&item.upgrade().unwrap()).flex_grow as usize;
        });
        total_grow_factor
    }
}

impl<T: Into<FlexItem>> From<Vec<T>> for Flexbox {
    fn from(value: Vec<T>) -> Self {
        let content: Vec<Rc<RefCell<FlexItem>>> = value
            .into_iter()
            .map(|item| Rc::new(RefCell::new(item.into())))
            .collect();
        Self {
            content,
            ..Default::default()
        }
    }
}

impl Flexbox {
    /// Create a new Flexbox with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a view to the end.
    pub fn push(&mut self, item: impl Into<FlexItem>) {
        self.content.push(Rc::new(RefCell::new(item.into())));
    }

    /// Remove all items.
    pub fn clear(&mut self) {
        self.content.clear();
    }

    /// Insert a view at `index`.
    ///
    /// # Panics
    /// Panics if `index > self.len()`.
    pub fn insert(&mut self, index: usize, item: impl Into<FlexItem>) {
        self.content
            .insert(index, Rc::new(RefCell::new(item.into())));
    }

    /// Set the grow factor of an item.
    ///
    /// # Panics
    /// Panics if `index >= self.len()`.
    pub fn set_flex_grow(&mut self, index: usize, flex_grow: u8) {
        Rc::as_ref(&self.content[index]).borrow_mut().flex_grow = flex_grow;
    }

    /// Returns the number of items in the flexbox.
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Returns whether the flexbox is empty.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Remove an item from the flexbox.
    ///
    /// # Panics
    /// Panics if `index >= self.len()`.
    pub fn remove(&mut self, index: usize) {
        self.content.remove(index);
    }

    /// Gap between items on the main axis.
    pub fn main_axis_gap(&self) -> u32 {
        self.options.main_axis_gap
    }

    /// Set the fixed gap between elements on the main axis.
    pub fn set_main_axis_gap(&mut self, gap: u32) {
        self.options.main_axis_gap = gap;
    }

    /// Gap between the main axes.
    pub fn cross_axis_gap(&self) -> u32 {
        self.options.cross_axis_gap
    }

    /// Set the fixed gap between the main axes.
    pub fn set_cross_axis_gap(&mut self, gap: u32) {
        self.options.cross_axis_gap = gap;
    }

    /// Get the justify-content option.
    pub fn justify_content(&self) -> JustifyContent {
        self.options.justification
    }

    /// Set the justify-content option.
    pub fn set_justify_content(&mut self, justify_content: JustifyContent) {
        self.options.justification = justify_content;
    }

    /// Get the align-items option.
    pub fn align_items(&self) -> AlignItems {
        self.options.item_alignment
    }

    /// Set the align-items option.
    pub fn set_align_items(&mut self, item_alignment: AlignItems) {
        self.options.item_alignment = item_alignment;
    }

    /// Get the align-content option.
    pub fn align_content(&self) -> AlignContent {
        self.options.axes_alignment
    }

    /// Set the align-content option.
    pub fn set_align_content(&mut self, axes_alignment: AlignContent) {
        self.options.axes_alignment = axes_alignment;
    }

    /// Get the flex-direction option.
    pub fn flex_direction(&self) -> FlexDirection {
        self.options.direction
    }

    /// Set the direction of the main axis.
    pub fn set_flex_direction(&mut self, direction: FlexDirection) {
        self.options.direction = direction;
    }

    /// Get the flex-wrap option.
    pub fn flex_wrap(&self) -> FlexWrap {
        self.options.wrap
    }

    /// Set the wrapping behavior.
    pub fn set_flex_wrap(&mut self, wrap: FlexWrap) {
        self.options.wrap = wrap;
    }

    /// Generate the concrete layout of this flexbox with the given constraints.
    fn generate_layout(&self, constraints: XY<usize>) -> Layout<Rc<RefCell<FlexItem>>> {
        let layout = FlexboxLayout::generate(
            &self.content.iter().map(Rc::downgrade).collect::<Vec<_>>(),
            constraints.x,
            constraints.y,
            self.options,
        );
        let mut result = Layout {
            elements: Vec::new(),
        };
        RefCell::borrow_mut(&layout)
            .windows()
            .into_iter()
            .for_each(|item| {
                result.elements.push(PlacedElement {
                    element: item.0,
                    position: item.1,
                })
            });
        result
    }
}

impl View for Flexbox {
    /// Draw this view using the printer.
    fn draw(&self, printer: &cursive_core::Printer<'_, '_>) {
        if let Some(ref layout) = self.layout {
            for placed_element in layout {
                RefCell::borrow(&placed_element.element)
                    .view
                    .draw(&printer.windowed(placed_element.position));
            }
        }
    }

    /// Called when the final size has been determined. `printer_size` will be the actual size of
    /// the printer given to `draw()`. This should call layout on all child items with their
    /// respective sizes.
    fn layout(&mut self, printer_size: Vec2) {
        // Generate the concrete layout for this flexbox.
        self.layout = Some(self.generate_layout(printer_size));

        // Use the layout to lay out the child views.
        for placed_element in self.layout.as_ref().unwrap() {
            RefCell::borrow_mut(&placed_element.element)
                .view
                .layout(placed_element.position.size());
        }
    }

    /// Return true if this view needs a relayout before the next call to `draw()`. If the view's
    /// layout is somehow cached, returning true here will cause `layout()` to be called so the new
    /// layout can be computed.
    fn needs_relayout(&self) -> bool {
        // TODO: Reimplement proper detection of relayout requirements. Returning true always works
        // but isn't efficient!
        true
    }

    /// Given `constraint`, return the minimal required size the printer for this view should be.
    /// `constraint` is the maximum possible size for the printer.
    fn required_size(&mut self, constraint: cursive_core::Vec2) -> cursive_core::Vec2 {
        // PERF: Cache the values that the previous layout was generated with and regenerate if
        // cached version is outdated.
        constraint
    }

    fn on_event(
        &mut self,
        mut event: cursive_core::event::Event,
    ) -> cursive_core::event::EventResult {
        if let cursive_core::event::Event::Mouse {
            ref mut offset,
            ref mut position,
            ..
        } = event
        {
            if let Some(ref layout) = self.layout {
                if let Some(placed_element) =
                    layout.element_at(global_to_view_coordinates(*position, *offset))
                {
                    *offset = *offset + placed_element.position.top_left();
                    RefCell::borrow_mut(&placed_element.element)
                        .view
                        .on_event(event)
                } else {
                    EventResult::Ignored
                }
            } else {
                EventResult::Ignored
            }
        } else if let Some(active_child) = self.focused {
            RefCell::borrow_mut(&self.content[active_child])
                .view
                .on_event(event)
        } else {
            EventResult::Ignored
        }
    }

    fn focus_view(
        &mut self,
        selector: &cursive_core::view::Selector<'_>,
    ) -> Result<EventResult, cursive_core::view::ViewNotFound> {
        for (index, view) in self.content.iter_mut().enumerate() {
            if let Ok(event_result) = RefCell::borrow_mut(view).view.focus_view(selector) {
                self.focused = Some(index);
                return Ok(event_result);
            }
        }
        Err(cursive_core::view::ViewNotFound)
    }

    fn call_on_any(
        &mut self,
        selector: &cursive_core::view::Selector<'_>,
        callback: cursive_core::event::AnyCb<'_>,
    ) {
        for view in self.content.iter_mut() {
            RefCell::borrow_mut(view)
                .view
                .call_on_any(selector, callback);
        }
    }

    fn take_focus(
        &mut self,
        _source: cursive_core::direction::Direction,
    ) -> Result<EventResult, cursive_core::view::CannotFocus> {
        Ok(EventResult::Consumed(None))
    }

    fn important_area(&self, _view_size: Vec2) -> Rect {
        if let Some(ref layout) = self.layout {
            if let Some(focused) = self.focused {
                layout.elements[focused].position
            } else {
                Rect::from_size((0, 0), (1, 1))
            }
        } else {
            Rect::from_size((0, 0), (1, 1))
        }
    }
}

impl FlexItem {
    /// Create a flex item with the given grow factor.
    pub fn with_flex_grow(view: impl IntoBoxedView, flex_grow: u8) -> Self {
        Self {
            view: view.into_boxed_view(),
            flex_grow,
        }
    }

    /// Set the flex-grow.
    pub fn set_flex_grow(&mut self, flex_grow: u8) {
        self.flex_grow = flex_grow;
    }

    /// Returns the flex-grow.
    pub fn flex_grow(&self) -> u8 {
        self.flex_grow
    }
}

impl<T: IntoBoxedView> From<T> for FlexItem {
    fn from(value: T) -> Self {
        Self {
            view: value.into_boxed_view(),
            flex_grow: 0,
        }
    }
}

/// Convert `global_coordinates` to coordinates within a View, using `view_offset` as the top-left
/// point of the view to convert to.
fn global_to_view_coordinates(global_coordinates: XY<usize>, view_offset: XY<usize>) -> XY<usize> {
    global_coordinates - view_offset
}
