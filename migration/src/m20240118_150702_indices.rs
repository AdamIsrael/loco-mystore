use std::borrow::BorrowMut;

use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Indices::Table)
                    .col(pk_auto(Indices::Id).borrow_mut())
                    .col(string(Indices::Name).borrow_mut())
                    .col(string_null(Indices::Description).borrow_mut())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Indices::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Indices {
    Table,
    Id,
    Name,
    Description,
    
}


