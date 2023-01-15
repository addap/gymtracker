use sea_orm_migration::prelude::*;

use crate::m20230113_140607_create_user_table::LoginUser;

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
                    .col(ColumnDef::new(ExerciseSet::Name).string().not_null())
                    .col(ColumnDef::new(ExerciseSet::Reps).integer().not_null())
                    .col(ColumnDef::new(ExerciseSet::Weight).float().not_null())
                    .col(
                        ColumnDef::new(ExerciseSet::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-exerciseset-user_id")
                            .from(ExerciseSet::Table, ExerciseSet::UserId)
                            .to(LoginUser::Table, LoginUser::Id)
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
    Name,
    Reps,
    Weight,
    CreatedAt,
}
