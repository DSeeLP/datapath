use std::{borrow::Cow, marker::PhantomData, ops::Deref};

pub trait StaticPath<T, RT> {
    fn static_get(data: &T) -> &RT;
}

#[macro_export]
macro_rules! static_path {
    ($name:ident<$t:ty,$rt:ty>: $($idt:tt).+) => {
        pub struct $name;
        impl StaticPath<$t, $rt> for $name {
            fn static_get(data: &$t) -> &$rt {
                &data.$($idt).+
            }
        }
    };

    ($( $(#[$outer:meta])* $mod:vis $name:ident {$(
         <$ty:ty, $rt:ty>: $($idt:tt).+ ;
    )+} )+) => {
        $(
            $(#[$outer])*
            $mod struct $name;
            $(

            impl StaticPath<$ty, $rt> for $name {
                fn static_get(data: &$ty) -> &$rt {
                    &data.$($idt).+
                }
            }
            )+
        )*
    }
}

#[macro_export]
macro_rules! path_alias {
    ($mod:vis $name:ident<$path:ty, $rt:ty> ) => {
        $mod type $name<'a, T> = $crate::StaticPathRef<'a, T, $path, $rt>;
    };
}

pub struct StaticPathRef<'a, T, P: StaticPath<T, RT>, RT: Clone> {
    value: Cow<'a, RT>,
    _t: PhantomData<T>,
    _path: PhantomData<P>,
}

impl<'a, T, P: StaticPath<T, RT>, RT: Clone> Deref for StaticPathRef<'a, T, P, RT> {
    type Target = Cow<'a, RT>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<'a, T, P: StaticPath<T, RT>, RT: Clone> AsRef<RT> for StaticPathRef<'a, T, P, RT> {
    fn as_ref(&self) -> &RT {
        &self.value
    }
}

impl<'a, T, P: StaticPath<T, RT>, RT: Clone> StaticPathRef<'a, T, P, RT> {
    pub fn new(value: RT) -> Self {
        Self {
            value: Cow::Owned(value),
            _t: PhantomData,
            _path: PhantomData,
        }
    }

    pub fn new_ref(value: &'a RT) -> Self {
        Self {
            value: Cow::Borrowed(value),
            _t: PhantomData,
            _path: PhantomData,
        }
    }

    pub fn from_obj(obj: &'a T) -> Self {
        Self::new_ref(P::static_get(obj))
    }
}
