use async_graphql::MaybeUndefined;
use sea_orm::{sea_query::Nullable, ActiveValue, NotSet, Set, Value};

pub trait IntoActiveValue<T>
where
    T: Into<Value>,
{
    fn into_active_value(&self) -> ActiveValue<T>;
}

impl<T> IntoActiveValue<T> for T
where
    T: Into<Value> + ToOwned<Owned = T>,
{
    fn into_active_value(&self) -> ActiveValue<T> {
        Set(self.to_owned())
    }
}

impl<T> IntoActiveValue<T> for Option<T>
where
    T: Into<Value> + ToOwned<Owned = T>,
{
    fn into_active_value(&self) -> ActiveValue<T> {
        match self {
            None => NotSet,
            Some(val) => Set(val.to_owned()),
        }
    }
}

impl<T> IntoActiveValue<Option<T>> for MaybeUndefined<T>
where
    T: Into<Value> + ToOwned<Owned = T> + Nullable,
{
    fn into_active_value(&self) -> ActiveValue<Option<T>> {
        match self {
            MaybeUndefined::Null => Set(None),
            MaybeUndefined::Undefined => NotSet,
            MaybeUndefined::Value(v) => Set(Some(v.to_owned())),
        }
    }
}
