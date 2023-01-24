use sea_orm_migration::prelude::*;

use crate::m20230113_140607_create_user_table::UserLogin;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExerciseName::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExerciseName::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ExerciseName::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ExerciseSet::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExerciseSet::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ExerciseSet::UserId).integer().not_null())
                    .col(ColumnDef::new(ExerciseSet::NameId).integer().not_null())
                    .col(ColumnDef::new(ExerciseSet::Reps).integer().not_null())
                    .col(ColumnDef::new(ExerciseSet::Weight).double().not_null())
                    .col(
                        ColumnDef::new(ExerciseSet::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-exerciseset-user_id")
                            .from(ExerciseSet::Table, ExerciseSet::UserId)
                            .to(UserLogin::Table, UserLogin::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-exerciseset-name")
                            .from(ExerciseSet::Table, ExerciseSet::NameId)
                            .to(ExerciseName::Table, ExerciseName::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExerciseName::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ExerciseSet::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum ExerciseName {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum ExerciseSet {
    Table,
    Id,
    UserId,
    NameId,
    Reps,
    Weight,
    CreatedAt,
}
