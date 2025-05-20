//! A linked list implementation in rust

/// A linked list
#[derive(Debug, Clone)]
pub struct LinkedList<T>(Option<Element<T>>);

/// An element of a linked list
#[derive(Debug, Clone)]
struct Element<T> {
    data: T,
    next: Option<Box<Element<T>>>,
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(element) = self.0.take() {
            self.0 = element.next.map(|a| *a);
            return Some(element.data);
        };
        return None;
    }
}

impl<T> LinkedList<T> {
    /// Push an element onto the end of the linked list.
    /// O(1) complexity
    pub fn push(&mut self, element: T) {
        if let Some(old) = self.0.take() {
            self.0 = Some(Element {
                data: element,
                next: Some(Box::new(old)),
            })
        } else {
            self.0 = Some(Element {
                data: element,
                next: None,
            })
        }
    }

    /// Pop an element off the end of the linked list
    /// O(1) complexity
    pub fn pop(&mut self) -> Option<T> {
        return self.0.take().map(|e| {
            self.0 = e.next.map(|e2| *e2);
            e.data
        });
    }

    /// Read an element of the linked list
    /// O(1) complexity
    pub fn read(&self) -> Option<&T> {
        return self.0.as_ref().map(|a| &a.data);
    }

    /// Clear the linked list
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Create a new linked list
    pub fn new() -> Self {
        return Self(None);
    }
}
