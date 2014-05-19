
pub trait AddLasting<'a, T> {
    /// time in second
    fn lasting(&'a self, time: f64) -> T;
}

