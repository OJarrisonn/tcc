use std::{ops::ShrAssign, str::FromStr};

pub struct Action<T, F>(F)
where
    F: FnOnce() -> T;

impl<T, F> Action<T, F>
where 
    F: FnOnce() -> T 
{
    /// Creates a new action using the given function
    /// The idea is that the action execution will be delayed to the [`Action::unwrap`] call
    #[inline]
    pub const fn new(f: F) -> Self {
        Self(f)
    }

    /// Will consume and execute an action. The idea is that 
    /// `unwrap` is only called in impure scopes
    #[inline]
    pub fn unwrap(self) -> T {
        (self.0)()
    }

    /// Permits to extend a given Action by doing something with its result and producing a 
    /// new Action. The difference between [`Action::then`] and [`Action::unwrap`] is that the
    /// former will keep the current scope pure, while the other one wont
    #[inline]
    pub fn then<G, U>(self, g: G) -> Action<U, impl FnOnce() -> U> 
    where 
        G: FnOnce(T) -> U
    {
        let f = move || g((self.0)());
        Action(f)
    } 

    #[inline]
    pub fn chain<G, H, U>(self, g: G) -> Action<U, impl FnOnce() -> U>
    where
        H: FnOnce() -> U,
        G: FnOnce(T) -> Action<U, H> 
    {
        let f = move || g((self.0)()).unwrap();
        Action(f)
    }

    #[inline]
    pub fn into<U>(self) -> Action<U, impl FnOnce() -> U>
    where 
        U: From<T>
    {
        let f = move || (self.0)().into();
        Action(f)
    }

    #[inline]
    pub fn try_into<U>(self) -> Action<Result<U, <U as TryFrom<T>>::Error>, impl FnOnce() -> Result<U, <U as TryFrom<T>>::Error>>
    where 
        U: TryFrom<T>
    {
        let f = move || (self.0)().try_into();
        Action(f)
    }
}

#[macro_export]
macro_rules! bind {
    ($(mut)? $pat:ident <- $expr:expr) => {
        let $(mut)? $pat = $expr.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::Action;

    #[test]
    fn test_simple_print() {
        let a = Action::new(|| println!("From action"));
        let a = a.then(|_| println!("Second print"));
        println!("From regular scope");
        a.unwrap();
    }
}