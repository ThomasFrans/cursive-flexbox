use std::{cell::RefCell, rc::{Weak, Rc}};

use cursive_core::{view::IntoBoxedView, Rect, Vec2, View, XY};

// https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction
// https://w3c.github.io/csswg-drafts/css-flexbox/#flex-direction-property
/// Direction of a flex container's main axis.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
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
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum FlexWrap {
    /// Don't wrap items.
    NoWrap,
    /// Wrap items along the secondary axis.
    #[default]
    Wrap,
    /// Wrap items against the secondary axis.
    WrapReverse,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content
// https://w3c.github.io/csswg-drafts/css-flexbox/#propdef-justify-content
/// Alignment of items in a flexbox along the main axis.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
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
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AlignItems {
    /// Align items at the start of the secondary axis.
    #[default]
    FlexStart,
    /// Align items at the end of the secondery axis.
    FlexEnd,
    /// Align items at the center of the secondary axis.
    Center,
    /// Stretch items to fill all the space along the secondary axis.
    Stretch,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/align-content
// https://w3c.github.io/csswg-drafts/css-flexbox/#align-content-property
/// Alignment of the main axes in a flexbox.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AlignContent {
    /// Align content to the start of the container.
    #[default]
    FlexStart,
    /// Align content to the end of the container.
    FlexEnd,
    /// Align content to the center of the container.
    FlexCenter,
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

        for axis in &self.main_axes {
            for mut combo in axis.windows(self) {
                match self.options.direction {
                    FlexDirection::Row | FlexDirection::RowReverse => combo.1.offset(XY::from((0, cross_offset))),
                    FlexDirection::Column | FlexDirection::ColumnReverse => combo.1.offset(XY::from((cross_offset, 0))),
                }
                windows.push(combo);
            }
            cross_offset += axis.cross_axis_size(self);
        }

        windows
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
                let result = main_axis.add_item(content[added].clone(), &mut RefCell::borrow_mut(&layout));
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

    pub fn flexitem_main_axis_size(&self, item: &mut FlexItem) -> usize {
        match self.options.direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                item.view
                    .required_size(self.size)
                    .x
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                item.view
                    .required_size(self.size)
                    .y
            }
        }
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
                FlexDirection::Row | FlexDirection::RowReverse => RefCell::borrow(&layout_upgraded).size.x,
                FlexDirection::Column | FlexDirection::ColumnReverse => RefCell::borrow(&layout_upgraded).size.y,
        };
        MainAxis {
            items: Vec::new(),
            free_space
        }
    }

    /// Return the cross axis size. The size of the cross axis is the maximum size of its elements
    /// along the cross axis.
    pub fn cross_axis_size(&self, layout: &Layout) -> usize {
        let mut maximum_item_cross_axis_size = 0;
        match layout.options.direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    for item in &self.items {
                        maximum_item_cross_axis_size = maximum_item_cross_axis_size.max(RefCell::borrow_mut(&item.upgrade().unwrap()).view.required_size(layout.size).y);
                    }
                },
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    for item in &self.items {
                        maximum_item_cross_axis_size = maximum_item_cross_axis_size.max(RefCell::borrow_mut(&item.upgrade().unwrap()).view.required_size(layout.size).x);
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
        let mut remaining_free_space = self.free_space;

        match layout.options.justification {
            JustifyContent::FlexStart => {
                for item in &self.items {
                    let upgraded_item = item.upgrade().unwrap();

                    match layout.options.direction {
                        FlexDirection::Row | FlexDirection::RowReverse => {
                            windows.push((upgraded_item.clone(), Rect::from_size((offset, 0), RefCell::borrow_mut(&upgraded_item).view.required_size(layout.size))));
                        },
                        FlexDirection::Column | FlexDirection::ColumnReverse => {
                            windows.push((upgraded_item.clone(), Rect::from_size((0, offset), RefCell::borrow_mut(&upgraded_item).view.required_size(layout.size))));
                        },
                    }
                    offset += layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&upgraded_item)) + layout.options.main_axis_gap as usize;
                }

			},
            JustifyContent::FlexEnd => {
                offset += remaining_free_space;
                for item in &self.items {
                    let upgraded_item = item.upgrade().unwrap();

                    match layout.options.direction {
                        FlexDirection::Row | FlexDirection::RowReverse => {
                            windows.push((upgraded_item.clone(), Rect::from_size((offset, 0), RefCell::borrow_mut(&upgraded_item).view.required_size(layout.size))));
                        },
                        FlexDirection::Column | FlexDirection::ColumnReverse => {
                            windows.push((upgraded_item.clone(), Rect::from_size((0, offset), RefCell::borrow_mut(&upgraded_item).view.required_size(layout.size))));
                        },
                    }
                    offset += layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&upgraded_item)) + layout.options.main_axis_gap as usize;
                }
			},
            JustifyContent::Center => {
                offset += remaining_free_space / 2;
                for item in &self.items {
                    let upgraded_item = item.upgrade().unwrap();

                    match layout.options.direction {
                        FlexDirection::Row | FlexDirection::RowReverse => {
                            windows.push((upgraded_item.clone(), Rect::from_size((offset, 0), RefCell::borrow_mut(&upgraded_item).view.required_size(layout.size))));
                        },
                        FlexDirection::Column | FlexDirection::ColumnReverse => {
                            windows.push((upgraded_item.clone(), Rect::from_size((0, offset), RefCell::borrow_mut(&upgraded_item).view.required_size(layout.size))));
                        },
                    }
                    offset += layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&upgraded_item)) + layout.options.main_axis_gap as usize;
                }

			},
            JustifyContent::SpaceBetween => {
                let mut amount_of_spaces_to_fill = self.number_of_items().saturating_sub(1);
                for item in &self.items {
                    let upgraded_item = item.upgrade().unwrap();
                    match layout.options.direction {
                        FlexDirection::Row | FlexDirection::RowReverse => {
                            windows.push((upgraded_item.clone(), Rect::from_size((offset, 0), RefCell::borrow_mut(&upgraded_item).view.required_size(layout.size))));
                        },
                        FlexDirection::Column | FlexDirection::ColumnReverse => {
                            windows.push((upgraded_item.clone(), Rect::from_size((0, offset), RefCell::borrow_mut(&upgraded_item).view.required_size(layout.size))));
                        },
                    }

                    let assigned_free_space = remaining_free_space.checked_div(amount_of_spaces_to_fill).unwrap_or(1);
                    remaining_free_space = remaining_free_space.saturating_sub(assigned_free_space);
                    amount_of_spaces_to_fill = amount_of_spaces_to_fill.saturating_sub(1);
                    offset += layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&upgraded_item)) + layout.options.main_axis_gap as usize + assigned_free_space;
                }
			},
            JustifyContent::SpaceAround => {
                let mut amount_of_spaces_to_fill = self.number_of_items() * 2;
                for item in &self.items {
                    let upgraded_item = item.upgrade().unwrap();
                    let mut assigned_free_space = remaining_free_space / amount_of_spaces_to_fill;
                    remaining_free_space -= assigned_free_space;
                    amount_of_spaces_to_fill -= 1;
                    offset += assigned_free_space;
                
                    match layout.options.direction {
                        FlexDirection::Row | FlexDirection::RowReverse => {
                            windows.push((upgraded_item.clone(), Rect::from_size((offset, 0), RefCell::borrow_mut(&upgraded_item).view.required_size(layout.size))));
                        },
                        FlexDirection::Column | FlexDirection::ColumnReverse => {
                            windows.push((upgraded_item.clone(), Rect::from_size((0, offset), RefCell::borrow_mut(&upgraded_item).view.required_size(layout.size))));
                        },
                    }

                    assigned_free_space = remaining_free_space / amount_of_spaces_to_fill;
                    remaining_free_space -= assigned_free_space;
                    amount_of_spaces_to_fill -= 1;
                    offset += layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&upgraded_item)) + layout.options.main_axis_gap as usize + assigned_free_space;
                }
			},
            JustifyContent::SpaceEvenly => {
                let mut amount_of_spaces_to_fill = self.number_of_items() + 1;
                for item in &self.items {
                    let upgraded_item = item.upgrade().unwrap();

                    let assigned_free_space = remaining_free_space.checked_div(amount_of_spaces_to_fill).unwrap_or(1);
                    remaining_free_space = remaining_free_space.saturating_sub(assigned_free_space);
                    amount_of_spaces_to_fill = amount_of_spaces_to_fill.saturating_sub(1);
                    offset += assigned_free_space;

                    match layout.options.direction {
                        FlexDirection::Row | FlexDirection::RowReverse => {
                            windows.push((upgraded_item.clone(), Rect::from_size((offset, 0), RefCell::borrow_mut(&upgraded_item).view.required_size(layout.size))));
                        },
                        FlexDirection::Column | FlexDirection::ColumnReverse => {
                            windows.push((upgraded_item.clone(), Rect::from_size((0, offset), RefCell::borrow_mut(&upgraded_item).view.required_size(layout.size))));
                        },
                    }

                    offset += layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&upgraded_item)) + layout.options.main_axis_gap as usize;
                }
			},
        }

        windows
    }

    /// Try to add `item` to this main axis, fail if this axis can't accomodate the item.
    pub fn add_item(&mut self, item: Weak<RefCell<FlexItem>>, layout: &mut Layout) -> Result<(), FlexBoxError> {
        let upgraded_item = item.upgrade().unwrap();
        if self.can_accomodate(&mut RefCell::borrow_mut(&upgraded_item), layout) {
            self.free_space = self.free_space.saturating_sub(layout.flexitem_main_axis_size(&mut RefCell::borrow_mut(&upgraded_item)));

            // Only add gaps if there is already an item.
            if self.number_of_items() >= 1 {
                self.free_space = self.free_space.saturating_sub(layout.options.main_axis_gap as usize);
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

    /// The number of gaps between items.
    pub fn number_of_gaps(&self) -> usize {
        self.number_of_items().saturating_sub(1)
    }

    /// Total size taken up by gaps.
    pub fn total_gaps_space(&self, layout: &mut Layout) -> usize {
        self.number_of_gaps() * layout.options.main_axis_gap as usize
    }

    /// The total amount of cells on this axis. In practice, this will be the width for row
    /// layouts, and height for column layouts.
    pub fn total_space(&self, layout: &Layout) -> usize {
        let direction_option = layout.options.direction;
        match direction_option {
            FlexDirection::Row | FlexDirection::RowReverse => layout.size.x,
            FlexDirection::Column | FlexDirection::ColumnReverse => layout.size.y,
        }
    }

    /// The amount of cells left on this axis after subtracting the space taken up by the items on
    /// it plus the gaps.
    pub fn free_space(&self) -> usize {
        self.free_space
    }
}

impl Default for FlexBox {
    fn default() -> Self {
        Self {
            content: Vec::new(),
            active: None,
            options: Default::default(),
            layout: Default::default(),
        }
    }
}

impl<T: IntoIterator> From<T> for FlexBox
where
    <T as IntoIterator>::Item: IntoBoxedView,
{
    fn from(value: T) -> Self {
        let content: Vec<Rc<RefCell<FlexItem>>> = value
            .into_iter()
            .map(|item| Rc::new(RefCell::new(FlexItem {
                flex_grow: 0,
                view: item.into_boxed_view(),
            })))
            .collect();
        Self {
            content,
            ..Default::default()
        }
    }
}

/// A container that can be used to display a list of items in a flexible way.
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
    pub fn set_main_axis_gap(&mut self, gap: u16) {
        self.options.main_axis_gap = gap;
    }

    pub fn set_cross_axis_gap(&mut self, gap: u16) {
        self.options.cross_axis_gap = gap;
    }

    pub fn set_grow(&mut self, index: usize, flex_grow: u8) {
        Rc::as_ref(&self.content[index]).borrow_mut().flex_grow = flex_grow;
    }

    pub fn set_justify_content(&mut self, justify_content: JustifyContent) {
        self.options.justification = justify_content;
    }

    pub fn set_align_items(&mut self, item_alignment: AlignItems) {
        self.options.item_alignment = item_alignment;
    }

    pub fn set_direction(&mut self, direction: FlexDirection) {
        self.options.direction = direction;
    }

    pub fn set_wrap(&mut self, wrap: FlexWrap) {
        self.options.wrap = wrap;
    }
}

impl View for FlexBox {
    /// Draw this view using the printer.
    fn draw(&self, printer: &cursive_core::Printer) {
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
        let items: Vec<Weak<RefCell<FlexItem>>> = self.content.clone().iter()
            .map(|item| Rc::downgrade(item))
            .collect();
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
            && self.content.iter().any(|item| RefCell::borrow(&item).view.needs_relayout())
    }

    /// Given `constraint`, return the minimal required size the printer for this view should be.
    /// `constraint` is the maximum possible size for the printer.
    fn required_size(&mut self, constraint: cursive_core::Vec2) -> cursive_core::Vec2 {
        constraint
    }
}
