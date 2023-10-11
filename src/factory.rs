use std::collections::HashSet;

use crate::{aictable::Aictable, builder::FactoryBuilder, error::Error};

/// A factory for generating unique IDs of a specific type.
pub struct Factory<T: Aictable> {
    initial_value: T,
    looping: bool,
    rewind: bool,

    cursor: T,
    set: HashSet<T>,
}

impl<T: Aictable> Factory<T> {
    /// Creates a new `Factory` with the specified initial value, looping behavior, and rewind behavior.
    ///
    /// It's recommended to use a [`FactoryBuilder`](crate::FactoryBuilder) to build a `Factory`.
    /// See the document of `FactoryBuilder` for details.
    pub fn new(initial_value: T, looping: bool, rewind: bool) -> Self {
        let cursor = initial_value.clone();

        Self {
            initial_value,
            looping,
            rewind,

            cursor,
            set: HashSet::new(),
        }
    }

    /// Returns a new [`FactoryBuilder`](crate::FactoryBuilder).
    pub fn builder() -> FactoryBuilder<T> {
        FactoryBuilder::default()
    }

    /// Generates and returns the next unique ID.
    ///
    /// # Errors
    ///
    /// - If no more IDs can be generated, it returns an [`Error::OutOfSpace`].
    /// - If `Factory::looping` is false and the current ID is already
    /// the maximum value of the type, it returns an [`Error::MaxReached`].
    pub fn next(&mut self) -> Result<T, Error<T>> {
        let start = self.cursor.clone();

        while self.set.contains(&self.cursor) {
            if self.looping {
                self.cursor = self.cursor.wrapping_next();

                if self.cursor == start {
                    return Err(Error::OutOfSpace);
                }
            } else {
                self.cursor = self.cursor.saturating_next();

                if self.cursor.is_max_reached() {
                    return Err(Error::MaxReached);
                }
            }
        }

        self.set.insert(self.cursor.clone());

        let next = self.cursor.clone();

        if self.looping {
            self.cursor = self.cursor.wrapping_next();
        } else {
            self.cursor = self.cursor.saturating_next();
        }

        Ok(next)
    }

    /// Removes the specified ID from the set of generated IDs,
    /// so that it can be reused.
    ///
    /// Returns whether the ID was present in the set.
    pub fn remove(&mut self, id: T) -> bool {
        if self.set.remove(&id) {
            if self.rewind {
                self.cursor = id;
            }

            return true;
        }

        false
    }

    /// Resets the factory to its initial state.
    pub fn reset(&mut self) {
        self.cursor = self.initial_value.clone();

        self.set.clear();
    }

    /// Manually marks the specified ID as used.
    ///
    /// # Errors
    ///
    /// - If the ID is already taken, it returns an [`Error::AlreadyExist<T>`].
    pub fn take_up(&mut self, id: T) -> Result<(), Error<T>> {
        if self.set.contains(&id) {
            return Err(Error::AlreadyExist(id));
        }

        self.set.insert(id);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory() {
        let mut factory = Factory::<u32>::builder().build();

        assert_eq!(factory.next().unwrap(), 0);
        assert_eq!(factory.next().unwrap(), 1);
        factory.remove(1);
        assert_eq!(factory.next().unwrap(), 2);
        assert!(factory.take_up(2).is_err());
        assert!(factory.take_up(3).is_ok());
        assert!(factory.take_up(3).is_err());
        factory.remove(3);
        assert!(factory.take_up(3).is_ok());
        assert_eq!(factory.next().unwrap(), 4);
        factory.reset();
        assert_eq!(factory.next().unwrap(), 0);

        factory = Factory::<u32>::builder().initial_value(1).build();

        assert_eq!(factory.next().unwrap(), 1);
        factory.reset();
        assert_eq!(factory.next().unwrap(), 1);

        factory = Factory::<u32>::builder().initial_value(u32::MAX).build();

        assert_eq!(factory.next().unwrap(), u32::MAX);
        assert!(factory.next().is_err());

        factory = Factory::<u32>::builder()
            .initial_value(u32::MAX)
            .looping(true)
            .build();

        assert_eq!(factory.next().unwrap(), u32::MAX);
        assert_eq!(factory.next().unwrap(), u32::MIN);

        factory = Factory::<u32>::builder().rewind(true).build();

        assert_eq!(factory.next().unwrap(), 0);
        assert_eq!(factory.next().unwrap(), 1);
        factory.remove(1);
        assert_eq!(factory.next().unwrap(), 1);
        factory.remove(0);
        assert_eq!(factory.next().unwrap(), 0);
        assert_eq!(factory.next().unwrap(), 2);
        assert_eq!(factory.next().unwrap(), 3);
        factory.remove(0);
        factory.remove(1);
        assert_eq!(factory.next().unwrap(), 1);
    }
}
