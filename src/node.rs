use crate::source;
pub use crate::source::Source;

pub trait LifecycleManager<ValueType> {
    fn init(&mut self);
    fn update(&mut self, new_value: &ValueType);
}

pub struct Node<'a, ValueType> {
    source: &'a mut dyn Source<ValueType>,
    value: Option<ValueType>,
    lifecycle_manager: &'a mut dyn LifecycleManager<ValueType>,
}

impl<'a, ValueType> Node<'a, ValueType> {
    pub fn new(
        source: &'a mut dyn Source<ValueType>,
        lifecycle_manager: &'a mut dyn LifecycleManager<ValueType>,
    ) -> Self {
        lifecycle_manager.init();

        Self {
            source,
            value: None,
            lifecycle_manager,
        }
    }

    pub fn query(&mut self) -> impl Source<&ValueType> {
        if let None = self.value {
            self.value = Some(self.source.read());
        };
        match self.value {
            None => unreachable!(),
            Some(ref value) => {
                self.lifecycle_manager.update(value);
                source::const_by_ref(value)
            }
        }
    }
}

impl<'a, ValueType> Node<'a, ValueType>
where
    ValueType: Copy,
{
    pub fn query_copying(&mut self) -> impl Source<ValueType> {
        match self.value {
            Some(value) => source::const_by_copy(value),
            None => {
                let value = self.source.read();
                self.value = Some(value);
                self.lifecycle_manager.update(&value);
                source::const_by_copy(value)
            }
        }
    }
}

impl<'a, ValueType> Node<'a, ValueType>
where
    ValueType: Clone,
{
    pub fn query_cloning(&mut self) -> impl Source<ValueType> {
        if let None = self.value {
            self.value = Some(self.source.read());
        };
        match self.value {
            None => unreachable!(),
            Some(ref value) => {
                self.lifecycle_manager.update(value);
                source::const_by_clone(value.clone())
            }
        }
    }
}
