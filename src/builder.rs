use crate::{aictable::Aictable, factory::Factory};

/// A builder to build a new [`Factory`](crate::Factory).
pub struct FactoryBuilder<T: Aictable> {
    initial_value: T,
    looping: bool,
    rewind: bool,
}

impl<T: Aictable> Default for FactoryBuilder<T> {
    fn default() -> Self {
        Self {
            initial_value: T::INITIAL,
            looping: false,
            rewind: false,
        }
    }
}

impl<T: Aictable> FactoryBuilder<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the initial value for the IDs in the [`Factory`](crate::Factory).
    ///
    /// - Default: The [`Aictable::INITIAL`] value of the type.
    /// Usually is the minimum value of the type.
    pub fn initial_value(mut self, initial_value: T) -> Self {
        self.initial_value = initial_value;
        self
    }

    /// If true, the [`Factory`](crate::Factory) will loop back to the initial value
    /// after reaching the maximum value.
    ///
    /// - Default: false
    pub fn looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }

    /// If true, the [`Factory`](crate::Factory) will rewind to the position of
    /// the latest removed ID when generating the next ID.
    ///
    /// - Default: false
    pub fn rewind(mut self, rewind: bool) -> Self {
        self.rewind = rewind;
        self
    }

    pub fn build(self) -> Factory<T> {
        Factory::new(self.initial_value, self.looping, self.rewind)
    }
}
