//! **Not ready for use: Unfinished implementation and unstable API.**
//!
//! A library that allows the user to create a flexbox for the Rust Cursive TUI library. This
//! library tries to adhere to the CSS3 specification of the flexbox as much as possible. Users who
//! are already familiar with it should feel right at home working with this implementation.
//!
//! ### Examples
//! - [Cargo examples](https://github.com/ThomasFrans/cursive-flexbox/tree/main/examples)

#![warn(missing_docs, future_incompatible, rust_2018_idioms, let_underscore)]

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use cursive_core::{event::EventResult, view::IntoBoxedView, Rect, Vec2, View, XY};

// https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction
// https://w3c.github.io/csswg-drafts/css-flexbox/#flex-direction-property
/// Direction of a flex container's main axis.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    /// Lay out the items in a row.
    #[default]
    Row,
    /// Lay out the items in a row (reverse order).
    RowReverse,
    /// Lay out the items in a column.
    Column,
    /// Lay out the items in a column (reverse order).
    ColumnReverse,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/flex-wrap
// https://w3c.github.io/csswg-drafts/css-flexbox/#flex-wrap-property
/// Wrapping behavior and direction of the cross axis of a flexbox.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FlexWrap {
    /// Don't wrap items.
    #[default]
    NoWrap,
    /// Wrap items along the secondary axis.
    Wrap,
    /// Wrap items against the secondary axis.
    WrapReverse,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content
// https://w3c.github.io/csswg-drafts/css-flexbox/#propdef-justify-content
/// Alignment of items in a flexbox along the main axis.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum JustifyContent {
    /// Justify items at the start of the container.
    #[default]
    FlexStart,
    /// Justify items at the end of the container.
    FlexEnd,
    /// Justify items at the center of the container.
    Center,
    /// Justify items with an equal amount of space between them.
    SpaceBetween,
    /// Justify items with an equal amount of margin per item.
    SpaceAround,
    /// Justify items with an equal amount of space between them (including sides).
    SpaceEvenly,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/align-items
// https://w3c.github.io/csswg-drafts/css-flexbox/#align-items-property
/// Alignment of items in a flexbox along the cross axis.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AlignItems {
    /// Align items at the start of the secondary axis.
    FlexStart,
    /// Align items at the end of the secondery axis.
    FlexEnd,
    /// Align items at the center of the secondary axis.
    Center,
    /// Stretch items to fill all the space along the secondary axis.
    #[default]
    Stretch,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/align-content
// https://w3c.github.io/csswg-drafts/css-flexbox/#align-content-property
/// Alignment of the main axes in a flexbox.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AlignContent {
    /// Align content to the start of the container.
    #[default]
    FlexStart,
    /// Align content to the end of the container.
    FlexEnd,
    /// Align content to the center of the container.
    Center,
    /// Stretch content along the secondary axis.
    Stretch,
    /// Align main axis with an equal amount of space between them.
    SpaceBetween,
    /// Align main axis with an equal of margin per axis.
    SpaceAround,
}

struct FlexItem {
    /// The proportion of extra space in the container along the main axis that should be given to
    /// this item.
    /// https://css-tricks.com/snippets/css/a-guide-to-flexbox/#aa-flex-grow
    flex_grow: u8,
    view: Box<dyn View>,
}

/// Options that can alter the behavior of a flexbox.
#[derive(Default, Clone, Copy)]
struct FlexBoxOptions {
    /// The direction of the main axis.
    direction: FlexDirection,
    /// Algorithm that assigns extra space on the main axis. This does nothing if any of the items
    /// on a main axis request to grow.
    justification: JustifyContent,
    /// How to place items on the cross axis.
    item_alignment: AlignItems,
    /// How to place the main axes in the container.
    axes_alignment: AlignContent,
    /// Gap between items on the main axis. The gap doesn't get added to the sides.
    main_axis_gap: u16,
    /// Gap between the main axes.
    cross_axis_gap: u16,
    /// Wrapping behavior of the main axes.
    wrap: FlexWrap,
}

/// An actual layout of a flexbox with real dimensions.
/// https://developer.mozilla.org/en-US/docs/Learn/CSS/CSS_layout/Flexbox#the_flex_model
#[derive(Default)]
struct Layout {
    /// The dimensions of the container.
    size: XY<usize>,
    /// The currently active item.
    active: Option<usize>,
    options: FlexBoxOptions,
    main_axes: Vec<MainAxis>,
}

/// Any error that can arise from operations on a flexbox.
#[derive(Debug)]
pub enum FlexBoxError {
    /// Error when trying to add too many items to one axis.
    AxisFull,
}

impl Layout {
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
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        combo.1.offset(XY::from((0, cross_offset)))
                    },
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
                        combo.1.offset(XY::from((cross_offset, 0)))
                    },
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
            FlexDirection::Row | FlexDirection::RowReverse => {
                self.size.y.saturating_sub(used_space)
            },
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                self.size.x.saturating_sub(used_space)
            },
        }
    }

    /// Generate the actual layout for the flexbox with `content` and given `width` and `height`.
    pub fn generate(
        content: &[Weak<RefCell<FlexItem>>],
        width: usize,
        height: usize,
        active: Option<usize>,
        options: FlexBoxOptions,
    ) -> Rc<RefCell<Self>> {
        let layout = Rc::new(RefCell::new(Layout {
            size: XY::from((width, height)),
            active,
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

            RefCell::borrow_mut(&layout).main_axes.push(main_axis);
        }

        layout
    }

    /// Return the size of a [FlexItem] along the main axis.
    pub fn flexitem_main_axis_size(&self, item: &mut FlexItem) -> usize {
        match self.options.direction {
            FlexDirection::Row | FlexDirection::RowReverse => item.view.required_size(self.size).x,
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                item.view.required_size(self.size).y
            },
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
    // Cache value for the remaining free space in this axis.
    free_space: usize,
}

impl MainAxis {
    pub fn new(layout: Weak<RefCell<Layout>>) -> Self {
        let layout_upgraded = layout.upgrade().unwrap();
        let free_space = match RefCell::borrow(&layout_upgraded).options.direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                RefCell::borrow(&layout_upgraded).size.x
            },
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                RefCell::borrow(&layout_upgraded).size.y
            },
        };
        MainAxis {
            items: Vec::new(),
            free_space,
        }
    }

    /// Return the cross axis size. The size of the cross axis is the maximum size of its elements
    /// along the cross axis.
    pub fn cross_axis_size(&self, layout: &Layout) -> usize {
        let mut maximum_item_cross_axis_size = 0;
        match layout.options.direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                for item in &self.items {
                    maximum_item_cross_axis_size = maximum_item_cross_axis_size.max(
                        RefCell::borrow_mut(&item.upgrade().unwrap())
                            .view
                            .required_size(layout.size)
                            .y,
                    );
                }
            },
            FlexDirection::Column | FlexDirection::ColumnReverse => {
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
    pub fn windows(&self, layout: &Layout) -> Vec<(Rc<RefCell<FlexItem>>, Rect)> {
        let mut windows = Vec::new();
        let mut offset = 0;
        let mut assignable_free_space = self.free_space;
        let combined_grow_factor = self.combined_grow_factor();
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

                // BUG: This doesn't guarantee to assign all free space! Implement a better
                // algorithm!
                let added_space = ((RefCell::borrow(&item).flex_grow as f64
                    / combined_grow_factor as f64)
                    * assignable_free_space as f64) as usize;
                let item_main_axis_size =
                    layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&item));

                // Decides `start_x` and `width`.
                match layout.options.direction {
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        start_x = offset;
                        width = item_main_axis_size + added_space;
                    },
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
                        start_y = offset;
                        height = item_main_axis_size + added_space;
                    },
                }
                offset += item_main_axis_size + layout.options.main_axis_gap as usize + added_space;
            } else {
                // Axis doesn't contain elements that want free space. Use justify-content property
                // to decide positioning.

                match layout.options.direction {
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        width = RefCell::borrow_mut(&item).view.required_size(layout.size).x;
                    },
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
                        height = RefCell::borrow_mut(&item).view.required_size(layout.size).y;
                    },
                }

                // Decides `start_x`, `width` is item's preferred width.
                match layout.options.justification {
                    JustifyContent::FlexStart => {
                        match layout.options.direction {
                            FlexDirection::Row | FlexDirection::RowReverse => {
                                start_x = offset;
                            },
                            FlexDirection::Column | FlexDirection::ColumnReverse => {
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
                            FlexDirection::Row | FlexDirection::RowReverse => {
                                start_x = offset;
                            },
                            FlexDirection::Column | FlexDirection::ColumnReverse => {
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
                            FlexDirection::Row | FlexDirection::RowReverse => {
                                start_x = offset;
                            },
                            FlexDirection::Column | FlexDirection::ColumnReverse => {
                                start_y = offset;
                            },
                        }

                        offset += layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&item))
                            + layout.options.main_axis_gap as usize;
                    },
                    JustifyContent::SpaceBetween => {
                        match layout.options.direction {
                            FlexDirection::Row | FlexDirection::RowReverse => {
                                start_x = offset;
                            },
                            FlexDirection::Column | FlexDirection::ColumnReverse => {
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
                            FlexDirection::Row | FlexDirection::RowReverse => {
                                start_x = offset;
                            },
                            FlexDirection::Column | FlexDirection::ColumnReverse => {
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
                            FlexDirection::Row | FlexDirection::RowReverse => {
                                start_x = offset;
                            },
                            FlexDirection::Column | FlexDirection::ColumnReverse => {
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
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        start_y = 0;
                        height = RefCell::borrow_mut(&item).view.required_size(layout.size).y;
                    },
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
                        start_x = 0;
                        width = RefCell::borrow_mut(&item).view.required_size(layout.size).x;
                    },
                },
                AlignItems::FlexEnd => match layout.options.direction {
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        height = RefCell::borrow_mut(&item).view.required_size(layout.size).y;
                        start_y = cross_axis_size - height;
                    },
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
                        width = RefCell::borrow_mut(&item).view.required_size(layout.size).x;
                        start_x = cross_axis_size - width;
                    },
                },
                AlignItems::Center => match layout.options.direction {
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        height = RefCell::borrow_mut(&item).view.required_size(layout.size).y;
                        start_y = (cross_axis_size - height) / 2;
                    },
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
                        width = RefCell::borrow_mut(&item).view.required_size(layout.size).x;
                        start_x = (cross_axis_size - width) / 2;
                    },
                },
                AlignItems::Stretch => match layout.options.direction {
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        height = cross_axis_size;
                        start_y = 0;
                    },
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
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
        layout: &mut Layout,
    ) -> Result<(), FlexBoxError> {
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
            Err(FlexBoxError::AxisFull)
        }
    }

    /// Return whether this axis can accomodate `item` with the amount of free space it has left. A
    /// main axis can accomodate an item if either it is the first axis in a non-wrapped flexbox,
    /// or it has enough space for the item and possible gap that would be added.
    pub fn can_accomodate(&self, item: &mut FlexItem, layout: &mut Layout) -> bool {
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

impl From<Vec<Box<dyn View>>> for FlexBox {
    fn from(value: Vec<Box<dyn View>>) -> Self {
        let content: Vec<Rc<RefCell<FlexItem>>> = value
            .into_iter()
            .map(|item| {
                Rc::new(RefCell::new(FlexItem {
                    flex_grow: 0,
                    view: item.into_boxed_view(),
                }))
            })
            .collect();
        Self {
            content,
            ..Default::default()
        }
    }
}

/// A container that can be used to display a list of items in a flexible way.
#[derive(Default)]
pub struct FlexBox {
    /// The content of the flexbox. Unlike some flexboxes, order is always dictated by the order of
    /// the items in `content`. There is no way to overwrite this.
    content: Vec<Rc<RefCell<FlexItem>>>,
    /// The currently active view.
    active: Option<usize>,
    /// Options to alter the behavior.
    options: FlexBoxOptions,
    /// The actual layout of the items.
    layout: Rc<RefCell<Layout>>,
}

impl FlexBox {
    /// Set the fixed gap between elements on the main axis.
    pub fn set_main_axis_gap(&mut self, gap: u16) {
        self.options.main_axis_gap = gap;
    }

    /// Set the fixed gap between the main axes.
    pub fn set_cross_axis_gap(&mut self, gap: u16) {
        self.options.cross_axis_gap = gap;
    }

    /// Set the grow factor of an item.
    ///
    /// # Panics
    /// Panics if the index is too big.
    pub fn set_flex_grow(&mut self, index: usize, flex_grow: u8) {
        Rc::as_ref(&self.content[index]).borrow_mut().flex_grow = flex_grow;
    }

    /// Set the justify-content option.
    pub fn set_justify_content(&mut self, justify_content: JustifyContent) {
        self.options.justification = justify_content;
    }

    /// Set the align-items option.
    pub fn set_align_items(&mut self, item_alignment: AlignItems) {
        self.options.item_alignment = item_alignment;
    }

    /// Set the align-content option.
    pub fn set_align_content(&mut self, axes_alignment: AlignContent) {
        self.options.axes_alignment = axes_alignment;
    }

    /// Set the direction of the main axis.
    pub fn set_direction(&mut self, direction: FlexDirection) {
        self.options.direction = direction;
    }

    /// Set the wrapping behavior.
    pub fn set_wrap(&mut self, wrap: FlexWrap) {
        self.options.wrap = wrap;
    }
}

impl View for FlexBox {
    /// Draw this view using the printer.
    fn draw(&self, printer: &cursive_core::Printer<'_, '_>) {
        // TODO: Move all the calculations from draw to layout phase. Now `windows()` has to
        // calculate the windows, which should be cached from the layout phase.
        for (child, window) in RefCell::borrow_mut(&self.layout).windows() {
            RefCell::borrow(&child).view.draw(&printer.windowed(window));
        }
    }

    /// Called when the final size has been determined. `printer_size` will be the actual size of
    /// the printer given to `draw()`. This should call layout on all child items with their
    /// respective sizes.
    fn layout(&mut self, printer_size: Vec2) {
        let items: Vec<Weak<RefCell<FlexItem>>> =
            self.content.clone().iter().map(Rc::downgrade).collect();
        self.layout = Layout::generate(
            &items,
            printer_size.x,
            printer_size.y,
            self.active,
            self.options,
        );
    }

    /// Return true if this view needs a relayout before the next call to `draw()`. If the view's
    /// layout is somehow cached, returning true here will cause `layout()` to be called so the new
    /// layout can be computed.
    fn needs_relayout(&self) -> bool {
        self.options.direction == RefCell::borrow(&self.layout).options.direction
            && self.active == RefCell::borrow(&self.layout).active
            && self.options.cross_axis_gap == RefCell::borrow(&self.layout).options.cross_axis_gap
            && self.options.main_axis_gap == RefCell::borrow(&self.layout).options.main_axis_gap
            && self
                .content
                .iter()
                .any(|item| RefCell::borrow(item).view.needs_relayout())
    }

    /// Given `constraint`, return the minimal required size the printer for this view should be.
    /// `constraint` is the maximum possible size for the printer.
    fn required_size(&mut self, constraint: cursive_core::Vec2) -> cursive_core::Vec2 {
        constraint
    }

    // TODO: Find correct child for mouse events.
    fn on_event(&mut self, event: cursive_core::event::Event) -> cursive_core::event::EventResult {
        if let Some(active_child) = self.active {
            RefCell::borrow_mut(&self.content[active_child])
                .view
                .on_event(event)
        } else {
            EventResult::Ignored
        }
    }
}

#[cfg(test)]
mod test {
    use cursive::views::TextView;

    use super::*;

    #[test]
    fn justify_content_single_item() {
        let mut flexbox = FlexBox::from(vec![TextView::new("Hello").into_boxed_view()]);

        // JustifyContent::FlexStart
        flexbox.set_justify_content(JustifyContent::FlexStart);
        let layout = Layout::generate(
            &flexbox
                .content
                .iter()
                .map(|item| Rc::downgrade(&Rc::clone(item)))
                .collect::<Vec<_>>(),
            9,
            1,
            Some(0),
            flexbox.options,
        );
        let mut layout_mut = RefCell::borrow_mut(&layout);
        let windows = layout_mut.windows();
        assert_eq!(windows[0].1.left(), 0);
        assert_eq!(windows[0].1.width(), 5);

        // JustifyContent::FlexEnd
        flexbox.set_justify_content(JustifyContent::FlexEnd);
        let layout = Layout::generate(
            &flexbox
                .content
                .iter()
                .map(|item| Rc::downgrade(&Rc::clone(item)))
                .collect::<Vec<_>>(),
            9,
            1,
            Some(0),
            flexbox.options,
        );
        let mut layout_mut = RefCell::borrow_mut(&layout);
        let windows = layout_mut.windows();
        assert_eq!(windows[0].1.left(), 4);
        assert_eq!(windows[0].1.width(), 5);

        // JustifyContent::Center
        flexbox.set_justify_content(JustifyContent::Center);
        let layout = Layout::generate(
            &flexbox
                .content
                .iter()
                .map(|item| Rc::downgrade(&Rc::clone(item)))
                .collect::<Vec<_>>(),
            9,
            1,
            Some(0),
            flexbox.options,
        );
        let mut layout_mut = RefCell::borrow_mut(&layout);
        let windows = layout_mut.windows();
        assert_eq!(windows[0].1.left(), 2);
        assert_eq!(windows[0].1.width(), 5);

        // JustifyContent::SpaceEvenly
        flexbox.set_justify_content(JustifyContent::SpaceBetween);
        let layout = Layout::generate(
            &flexbox
                .content
                .iter()
                .map(|item| Rc::downgrade(&Rc::clone(item)))
                .collect::<Vec<_>>(),
            9,
            1,
            Some(0),
            flexbox.options,
        );
        let mut layout_mut = RefCell::borrow_mut(&layout);
        let windows = layout_mut.windows();
        assert_eq!(windows[0].1.left(), 0);
        assert_eq!(windows[0].1.width(), 5);

        // JustifyContent::SpaceAround
        flexbox.set_justify_content(JustifyContent::SpaceAround);
        let layout = Layout::generate(
            &flexbox
                .content
                .iter()
                .map(|item| Rc::downgrade(&Rc::clone(item)))
                .collect::<Vec<_>>(),
            9,
            1,
            Some(0),
            flexbox.options,
        );
        let mut layout_mut = RefCell::borrow_mut(&layout);
        let windows = layout_mut.windows();
        assert_eq!(windows[0].1.left(), 2);
        assert_eq!(windows[0].1.width(), 5);

        // JustifyContent::SpaceEvenly
        flexbox.set_justify_content(JustifyContent::SpaceEvenly);
        let layout = Layout::generate(
            &flexbox
                .content
                .iter()
                .map(|item| Rc::downgrade(&Rc::clone(item)))
                .collect::<Vec<_>>(),
            9,
            1,
            Some(0),
            flexbox.options,
        );
        let mut layout_mut = RefCell::borrow_mut(&layout);
        let windows = layout_mut.windows();
        assert_eq!(windows[0].1.left(), 2);
        assert_eq!(windows[0].1.width(), 5);
    }

    #[test]
    fn justify_content_multiple_items() {
        let mut flexbox = FlexBox::from(vec![
            TextView::new("Hello").into_boxed_view(),
            TextView::new("flexbox").into_boxed_view(),
        ]);

        flexbox.set_justify_content(JustifyContent::FlexStart);

        let layout = Layout::generate(
            &flexbox
                .content
                .iter()
                .map(|item| Rc::downgrade(&Rc::clone(item)))
                .collect::<Vec<_>>(),
            18,
            1,
            Some(0),
            flexbox.options,
        );

        let mut layout_mut = RefCell::borrow_mut(&layout);
        let windows = layout_mut.windows();

        assert_eq!(windows[0].1.left(), 0);
        assert_eq!(windows[0].1.width(), 5);

        assert_eq!(windows[1].1.left(), 5);
        assert_eq!(windows[1].1.width(), 7);

        flexbox.set_justify_content(JustifyContent::FlexEnd);

        let layout = Layout::generate(
            &flexbox
                .content
                .iter()
                .map(|item| Rc::downgrade(&Rc::clone(item)))
                .collect::<Vec<_>>(),
            18,
            1,
            Some(0),
            flexbox.options,
        );

        let mut layout_mut = RefCell::borrow_mut(&layout);
        let windows = layout_mut.windows();

        assert_eq!(windows[0].1.left(), 6);
        assert_eq!(windows[0].1.width(), 5);

        assert_eq!(windows[1].1.left(), 11);
        assert_eq!(windows[1].1.width(), 7);

        flexbox.set_justify_content(JustifyContent::Center);

        let layout = Layout::generate(
            &flexbox
                .content
                .iter()
                .map(|item| Rc::downgrade(&Rc::clone(item)))
                .collect::<Vec<_>>(),
            18,
            1,
            Some(0),
            flexbox.options,
        );

        let mut layout_mut = RefCell::borrow_mut(&layout);
        let windows = layout_mut.windows();

        assert_eq!(windows[0].1.left(), 3);
        assert_eq!(windows[0].1.width(), 5);

        assert_eq!(windows[1].1.left(), 8);
        assert_eq!(windows[1].1.width(), 7);

        flexbox.set_justify_content(JustifyContent::SpaceBetween);

        let layout = Layout::generate(
            &flexbox
                .content
                .iter()
                .map(|item| Rc::downgrade(&Rc::clone(item)))
                .collect::<Vec<_>>(),
            18,
            1,
            Some(0),
            flexbox.options,
        );

        let mut layout_mut = RefCell::borrow_mut(&layout);
        let windows = layout_mut.windows();

        assert_eq!(windows[0].1.left(), 0);
        assert_eq!(windows[0].1.width(), 5);

        assert_eq!(windows[1].1.left(), 11);
        assert_eq!(windows[1].1.width(), 7);

        flexbox.set_justify_content(JustifyContent::SpaceAround);

        let layout = Layout::generate(
            &flexbox
                .content
                .iter()
                .map(|item| Rc::downgrade(&Rc::clone(item)))
                .collect::<Vec<_>>(),
            16,
            1,
            Some(0),
            flexbox.options,
        );

        let mut layout_mut = RefCell::borrow_mut(&layout);
        let windows = layout_mut.windows();

        assert_eq!(windows[0].1.left(), 1);
        assert_eq!(windows[0].1.width(), 5);

        assert_eq!(windows[1].1.left(), 8);
        assert_eq!(windows[1].1.width(), 7);

        flexbox.set_justify_content(JustifyContent::SpaceEvenly);

        let layout = Layout::generate(
            &flexbox
                .content
                .iter()
                .map(|item| Rc::downgrade(&Rc::clone(item)))
                .collect::<Vec<_>>(),
            18,
            1,
            Some(0),
            flexbox.options,
        );

        let mut layout_mut = RefCell::borrow_mut(&layout);
        let windows = layout_mut.windows();

        assert_eq!(windows[0].1.left(), 2);
        assert_eq!(windows[0].1.width(), 5);

        assert_eq!(windows[1].1.left(), 9);
        assert_eq!(windows[1].1.width(), 7);
    }
}
