/// Convenience struct for building lists of items
pub struct ListBuilder<T>(Vec<T>);

impl<T> ListBuilder<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Adds item to the list
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    /// Adds items to the list
    pub fn list<I>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        for it in items.into_iter() {
            self.push(it);
        }
        self
    }

    /// Optionally adds item to the list if value is provided, item calculated
    /// from passed function
    pub fn optional<V, F>(mut self, value: Option<V>, get_item: F) -> Self
    where
        F: Fn(V) -> T,
    {
        if let Some(v) = value {
            self.push(get_item(v));
        }
        self
    }

    pub fn to_vec(self) -> Vec<T> {
        self.0
    }
}

impl<T> Default for ListBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<ListBuilder<T>> for Vec<T> {
    fn from(value: ListBuilder<T>) -> Self {
        value.0
    }
}

pub fn list<I, T>(items: I) -> ListBuilder<T>
where
    I: IntoIterator<Item = T>,
{
    let mut list_builder = ListBuilder::new();
    for it in items.into_iter() {
        list_builder.push(it);
    }
    list_builder
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list_builder() {
        let b = list([1, 2])
            .list([1])
            .optional(None, |v| v)
            .optional(Some(10), |v| v);
        assert_eq!(b.to_vec(), vec![1, 2, 1, 10]);
    }
}
