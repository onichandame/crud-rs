use async_graphql::InputObject;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;

macro_rules! create_filter {
    ($filter:ident, $data:ident) => {
        #[derive(InputObject, Clone, Default, Serialize, Debug)]
        pub struct $filter {
            pub eq: Option<$data>,
            pub null: Option<bool>,
            pub lt: Option<$data>,
            pub lte: Option<$data>,
            pub gt: Option<$data>,
            pub gte: Option<$data>,
            pub like: Option<String>,
            pub and: Option<Vec<$filter>>,
            pub or: Option<Vec<$filter>>,
            pub not: Option<bool>,
        }

        impl $filter {
            /// build filter into a SeaORM filter condition
            pub fn build<TCol: sea_orm::ColumnTrait>(&self, col: TCol) -> sea_orm::Condition {
                let mut filter = sea_orm::Condition::all();
                if let Some(eq) = &self.eq {
                    filter = filter.add(col.eq(eq.clone()));
                }
                if let Some(null) = &self.null {
                    if *null {
                        filter = filter.add(col.is_null());
                    } else {
                        filter = filter.add(col.is_not_null());
                    }
                }
                if matches!(self.null, Some(true)) {}
                if let Some(lt) = &self.lt {
                    filter = filter.add(col.lt(lt.clone()));
                }
                if let Some(lte) = &self.lte {
                    filter = filter.add(col.lte(lte.clone()));
                }
                if let Some(gt) = &self.gt {
                    filter = filter.add(col.gt(gt.clone()));
                }
                if let Some(gte) = &self.gte {
                    filter = filter.add(col.gte(gte.clone()));
                }
                if let Some(like) = &self.like {
                    filter = filter.add(col.like(&like));
                }
                if let Some(and) = &self.and {
                    for child_filter in and {
                        filter = filter.add(child_filter.build(col.clone()));
                    }
                }
                if let Some(or) = &self.or {
                    let mut cond = sea_orm::Condition::any();
                    for child_filter in or {
                        cond = cond.add(child_filter.build(col.clone()));
                    }
                    filter = filter.add(cond);
                }
                if matches!(&self.not, Some(true)) {
                    filter = filter.not();
                }
                filter
            }
        }
    };
}

create_filter!(StringFilter, String);
create_filter!(IntFilter, i64);
create_filter!(BooleanFilter, bool);
create_filter!(DateTimeFilter, NaiveDateTime);
type UtcDateTime = DateTime<Utc>;
create_filter!(UtcDateTimeFilter, UtcDateTime);
