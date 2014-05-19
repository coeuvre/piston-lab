
use BackEnd;

pub trait Map<'a> {
    // the parameters of the closure `command` need more discussions.
    /// Returns a identifier for user to remove the mapping later.
    fn map<'a, B: BackEnd>(&self, back_end: &mut B, command: ||: 'a) -> uint;
}

