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
