use crate::Consumable;

pub struct ConsumeIter<'a, T>
where
    T: Consumable,
{
    phantom: std::marker::PhantomData<T>,
    unconsumed: &'a str,
}

impl<'a, T> Iterator for ConsumeIter<'a, T>
where
    T: Consumable,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match <T>::consume_from(self.unconsumed) {
            Ok((item, unconsumed)) => {
                self.unconsumed = unconsumed;

                Some(item)
            }
            Err(_) => None,
        }
    }
}
