/// anything that can be cleared
pub trait Clear {
    fn clear(&mut self);
}

impl<T> Clear for Option<T> {
    fn clear(&mut self) {
        self.take();
    }
}

impl Clear for String {
    fn clear(&mut self) {
        self.clear();
    }
}

impl<T> Clear for Vec<T> {
    fn clear(&mut self) {
        self.clear();
    }
}
