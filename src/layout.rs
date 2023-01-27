//! Functionality for laying out items in the 2D plane.

use std::{
    slice::{Iter, IterMut},
    vec::IntoIter,
};

use cursive_core::{Rect, XY};

/// A generic element that has a place (position + size) on the 2D plane.
pub struct PlacedElement<T> {
    /// The actual element that is represented at `position`.
    pub element: T,
    /// The location of the element on the 2D plane.
    pub position: Rect,
}

/// A concrete layout of elements.
pub struct Layout<T> {
    /// The elements that make up this layout.
    pub elements: Vec<PlacedElement<T>>,
}

impl<T> Layout<T> {
    /// Return the item at `position`, or None if there is no item.
    pub fn element_at(&self, position: XY<usize>) -> Option<&PlacedElement<T>> {
        self.iter()
            .find(|&element| element.position.contains(position))
    }

    /// Return an iterator over the items of this layout.
    pub fn iter(&self) -> Iter<'_, PlacedElement<T>> {
        self.into_iter()
    }

    /// Return the size of this layout.
    pub fn size(&self) -> XY<usize> {
        let mut max_size = XY::from((0, 0));

        self.iter().for_each(|item| {
            if item.position.right() > max_size.x {
                max_size.x = item.position.right() + 1;
            }
            if item.position.bottom() > max_size.y {
                max_size.y = item.position.bottom() + 1;
            }
        });

        max_size
    }
}

impl<T> IntoIterator for Layout<T> {
    type Item = PlacedElement<T>;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Layout<T> {
    type Item = &'a PlacedElement<T>;

    type IntoIter = Iter<'a, PlacedElement<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Layout<T> {
    type Item = &'a mut PlacedElement<T>;

    type IntoIter = IterMut<'a, PlacedElement<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}
