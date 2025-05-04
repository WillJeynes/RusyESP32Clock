pub struct AlwaysSame<T> {
    pub value: T,
}

pub struct AlwaysSameIter<T: Clone> {
    pub value: T,
}

impl<T: Clone> Iterator for AlwaysSameIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.value.clone()) // Always returns the same value
    }
}

impl<T: Clone> IntoIterator for AlwaysSame<T> {
    type Item = T;
    type IntoIter = AlwaysSameIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        AlwaysSameIter { value: self.value }
    }
}