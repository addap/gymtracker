use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserLogin::Table)
                    .add_column(
                        ColumnDef::new(UserLogin::IsSuperuser)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserLogin::Table)
                    .drop_column(Alias::new("is_superuser"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum UserLogin {
    Table,
    IsSuperuser,
}
