use async_graphql::Enum;

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    ASC,
    DESC,
}

impl SortDirection {
    pub fn apply_sort<TQuery: sea_orm::QueryOrder, TCol: sea_orm::ColumnTrait>(
        &self,
        query: TQuery,
        col: TCol,
    ) -> TQuery {
        match self {
            Self::ASC => query.order_by_asc(col),
            Self::DESC => query.order_by_desc(col),
        }
    }
}
