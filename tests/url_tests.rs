#![allow(unused_imports, dead_code)]

pub mod common;
use std::str::FromStr;

use common::{TestContext, features::*, setup::*};
use sea_orm::{DatabaseConnection, IntoActiveModel, NotSet, Set, entity::prelude::*};

mod sample {
    use sea_orm::entity::prelude::*;

    #[sea_orm::model]
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "sample")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i64,
        pub location: TextUrl,
    }

    impl ActiveModelBehavior for ActiveModel {}
}

#[sea_orm_macros::test]
async fn text_url_test() -> Result<(), DbErr> {
    let ctx = TestContext::new("url_test").await;
    let db = &ctx.db;

    let location = Url::from_str("https://localhost:25565/")?;

    db.get_schema_builder()
        .register(sample::Entity)
        .apply(db)
        .await?;

    let entry = sample::ActiveModel {
        id: NotSet,
        location: Set(location.clone()),
    }
    .insert(db)
    .await?;

    assert_eq!(entry.location, location);

    Ok(())
}
