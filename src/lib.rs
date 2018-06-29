//! Functions and types to do something special when repeating for the first or
//! last time (or in between!). This crate offers two distinct features:
//!
//! - [`IterStatusExt::with_status`]: a new method for **iterators**, that
//!   creates a new iterator which yields the item paired with information to
//!   tell you if this is the first/last item.
//! - [`SkipFirst`]: a simple struct to help you always do something, except on
//!   the first repetition. Works without iterators, too!

use std::{
    iter::{FusedIterator, Peekable},
};

/// Allows you to always do something, except the first time.
///
/// Internally, this is simply a `bool`. It stores whether
/// [`skip_first`][SkipFirst::skip_first] has already been called. This struct
/// is really just a wrapper for a dead simple logic you could easily write
/// yourself. However, if you need to write it multiple times, it's better to
/// use this type to avoid duplicate code.
///
/// # Example
///
/// In this example, it's also possible to use [`IterStatusExt::with_status`].
///
/// ```
/// use splop::SkipFirst;
///
/// let mut comma = SkipFirst::new();
/// for name in &["peter", "ingrid", "barbara"] {
///     comma.skip_first(|| print!(", "));
///     print!("{}", name);
/// }
/// println!();
///
/// // Printed "peter, ingrid, barbara"
/// ```
pub struct SkipFirst {
    first: bool,
}

impl SkipFirst {
    /// Creates a new instance of `SkipFirst`.
    pub fn new() -> Self {
        Self {
            first: true,
        }
    }

    /// Executes the given function, except the first time this method is
    /// called on this instance.
    ///
    /// # Example
    ///
    /// ```
    /// use splop::SkipFirst;
    ///
    /// let mut v = Vec::new();
    /// let mut skipper = SkipFirst::new();
    /// skipper.skip_first(|| v.push(1));  // won't be executed
    /// skipper.skip_first(|| v.push(2));  // will be executed
    /// skipper.skip_first(|| v.push(3));  // will be executed
    ///
    /// assert_eq!(v, [2, 3]);
    /// ```
    ///
    /// Note that the state "has been called already" is stored in the
    /// [`SkipFirst`] instance and not globally:
    ///
    /// ```
    /// use splop::SkipFirst;
    ///
    /// let mut v = Vec::new();
    /// let mut skipper_a = SkipFirst::new();
    /// let mut skipper_b = SkipFirst::new();
    /// skipper_a.skip_first(|| v.push("a"));  // won't be executed
    /// skipper_b.skip_first(|| v.push("b"));  // won't be executed
    /// skipper_b.skip_first(|| v.push("b2"));  // will be executed
    /// skipper_a.skip_first(|| v.push("a2"));  // will be executed
    ///
    /// assert_eq!(v, ["b2", "a2"]);
    /// ```
    pub fn skip_first(&mut self, f: impl FnOnce()) {
        if self.first {
            self.first = false;
        } else {
            f();
        }
    }
}

/// Iterator wrapper which keeps track of the status. See
/// [`IterStatusExt::with_status`] for more information.
pub struct WithStatus<I: Iterator> {
    iter: Peekable<I>,
    first: bool,
}

impl<I: Iterator> WithStatus<I> {
    fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
            first: true,
        }
    }
}

impl<I: Iterator> Iterator for WithStatus<I> {
    type Item = (I::Item, Status);

    fn next(&mut self) -> Option<Self::Item> {
        // Get the next item from the iterator.
        let item = self.iter.next();

        let status = Status {
            first: self.first,
            // Since we already got the real item above, we can now peek if
            // there is still another item.
            last: self.iter.peek().is_none(),
        };

        if self.first {
            self.first = false;
        }

        item.map(|elem| (elem, status))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // We pass through the `size_hint` method, as the underlying iterator
        // might have size information.
        self.iter.size_hint()
    }
}

// Implement traits when the underlying iterator implements them.
impl<I: FusedIterator> FusedIterator for WithStatus<I> {}
impl<I: ExactSizeIterator> ExactSizeIterator for WithStatus<I> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

/// Adds the `with_status` method to all iterators.
pub trait IterStatusExt: Iterator + Sized {
    /// Creates an iterator that yields the original items paired with a
    /// status, which tells you if the item is the first and/or last one.
    ///
    /// The new iterator's item has the type `(Self::Item, Status)`. See
    /// [`Status`] for detailed information. The new iterator uses `peekable()`
    /// internally, so if the `next()` call of the underlying iterator has
    /// side effects, those will be visible earlier than expected.
    ///
    /// # Example
    ///
    /// ```
    /// use splop::IterStatusExt;
    ///
    ///
    /// let mut s = String::new();
    /// let names = ["anna", "peter", "bob"];
    ///
    /// for (name, status) in names.iter().with_status() {
    ///     if !status.is_first() {
    ///         s += ", ";
    ///     }
    ///
    ///     s += name;
    /// }
    ///
    /// assert_eq!(s, "anna, peter, bob");
    /// ```
    fn with_status(self) -> WithStatus<Self>;
}

impl<I: Iterator> IterStatusExt for I {
    fn with_status(self) -> WithStatus<Self> {
        WithStatus::new(self)
    }
}

/// The status of an item from an iterator (e.g. "is this the first item?").
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Status {
    first: bool,
    last: bool,
}

impl Status {
    /// Returns `true` if this is the first item of the iterator.
    ///
    /// Note that an item might simultaniously be the first and last item (if
    /// the iterator only contains one item). To check if the item is the first
    /// and and not the last, use [`Status::is_first_only`].
    ///
    /// # Example
    ///
    /// ```
    /// use splop::IterStatusExt;
    ///
    /// let v: Vec<_> = (0..4)
    ///     .with_status()
    ///     .map(|(i, status)| (i, status.is_first()))
    ///     .collect();
    ///
    /// assert_eq!(v, [
    ///     (0, true),
    ///     (1, false),
    ///     (2, false),
    ///     (3, false),
    /// ]);
    /// ```
    ///
    /// If there is only one element, this function returns `true`, as does
    /// `is_last`:
    ///
    /// ```
    /// use splop::IterStatusExt;
    ///
    /// let (_, status) = [27].iter()
    ///     .with_status()
    ///     .next()
    ///     .unwrap();
    ///
    /// assert!(status.is_first());
    /// assert!(status.is_last());
    /// ```
    pub fn is_first(&self) -> bool {
        self.first
    }

    /// Returns `true` if this is the first item and it's not the only item in
    /// the iterator.
    ///
    /// # Example
    ///
    /// ```
    /// use splop::IterStatusExt;
    ///
    /// let v: Vec<_> = (0..4)
    ///     .with_status()
    ///     .map(|(i, status)| (i, status.is_first_only()))
    ///     .collect();
    ///
    /// assert_eq!(v, [
    ///     (0, true),
    ///     (1, false),
    ///     (2, false),
    ///     (3, false),
    /// ]);
    /// ```
    ///
    /// If there is only one element, this function returns `false`:
    ///
    /// ```
    /// use splop::IterStatusExt;
    ///
    /// let (_, status) = [27].iter()
    ///     .with_status()
    ///     .next()
    ///     .unwrap();
    ///
    /// assert!(!status.is_first_only());
    /// ```
    pub fn is_first_only(&self) -> bool {
        self.first && !self.last
    }

    /// Returns `true` if this is the last item of the iterator.
    ///
    /// Note that an item might simultaniously be the last and first item (if
    /// the iterator only contains one item). To check if the item is the last
    /// and and not the first, use [`Status::is_last_only`].
    ///
    /// # Example
    ///
    /// ```
    /// use splop::IterStatusExt;
    ///
    /// let v: Vec<_> = (0..4)
    ///     .with_status()
    ///     .map(|(i, status)| (i, status.is_last()))
    ///     .collect();
    ///
    /// assert_eq!(v, [
    ///     (0, false),
    ///     (1, false),
    ///     (2, false),
    ///     (3, true),
    /// ]);
    /// ```
    ///
    /// If there is only one element, this function returns `true`, as does
    /// `is_first`:
    ///
    /// ```
    /// use splop::IterStatusExt;
    ///
    /// let (_, status) = [27].iter()
    ///     .with_status()
    ///     .next()
    ///     .unwrap();
    ///
    /// assert!(status.is_first());
    /// assert!(status.is_last());
    /// ```
    pub fn is_last(&self) -> bool {
        self.last
    }

    /// Returns `true` if this is the last item and it's not the only item in
    /// the iterator.
    ///
    /// # Example
    ///
    /// ```
    /// use splop::IterStatusExt;
    ///
    /// let v: Vec<_> = (0..4)
    ///     .with_status()
    ///     .map(|(i, status)| (i, status.is_last_only()))
    ///     .collect();
    ///
    /// assert_eq!(v, [
    ///     (0, false),
    ///     (1, false),
    ///     (2, false),
    ///     (3, true),
    /// ]);
    /// ```
    ///
    /// If there is only one element, this function returns `false`:
    ///
    /// ```
    /// use splop::IterStatusExt;
    ///
    /// let (_, status) = [27].iter()
    ///     .with_status()
    ///     .next()
    ///     .unwrap();
    ///
    /// assert!(!status.is_last_only());
    /// ```
    pub fn is_last_only(&self) -> bool {
        self.last && !self.first
    }

    /// Returns `true` if this is neither the first nor the last item.
    ///
    /// # Example
    ///
    /// ```
    /// use splop::IterStatusExt;
    ///
    /// let v: Vec<_> = (0..4)
    ///     .with_status()
    ///     .map(|(i, status)| (i, status.is_in_between()))
    ///     .collect();
    ///
    /// assert_eq!(v, [
    ///     (0, false),
    ///     (1, true),
    ///     (2, true),
    ///     (3, false),
    /// ]);
    /// ```
    pub fn is_in_between(&self) -> bool {
        !self.first && !self.last
    }
}
