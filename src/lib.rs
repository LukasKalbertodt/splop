/// Allows you to always do something, except the first time.
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
    pub fn skip_first(&mut self, f: impl FnOnce()) {
        if self.first {
            self.first = false;
        } else {
            f();
        }
    }
}
