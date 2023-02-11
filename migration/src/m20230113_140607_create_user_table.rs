use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserLogin::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserLogin::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(UserLogin::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(UserLogin::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(UserLogin::PwHash).string().not_null())
                    .col(
                        ColumnDef::new(UserLogin::IsSuperuser)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(UserLogin::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserInfo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserInfo::UserId)
                            .integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(UserInfo::DisplayName).string().not_null())
                    .col(ColumnDef::new(UserInfo::Photo).blob(BlobSize::Medium))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-userinfo-user_id")
                            .from(UserInfo::Table, UserInfo::UserId)
                            .to(UserLogin::Table, UserLogin::Id), // .on_delete(ForeignKeyAction::Cascade)
                                                                  // .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserInfoTs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserInfoTs::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserInfoTs::UserId).integer().not_null())
                    .col(ColumnDef::new(UserInfoTs::Height).double())
                    .col(ColumnDef::new(UserInfoTs::Weight).double())
                    .col(ColumnDef::new(UserInfoTs::MuscleMass).double())
                    .col(ColumnDef::new(UserInfoTs::BodyFat).double())
                    .col(ColumnDef::new(UserInfoTs::CreatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-userinfots-user_id")
                            .from(UserInfoTs::Table, UserInfoTs::UserId)
                            .to(UserLogin::Table, UserLogin::Id)
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
            .drop_table(Table::drop().table(UserLogin::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserInfo::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserInfoTs::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum UserLogin {
    Table,
    Id,
    Username,
    Email,
    PwHash,
    IsSuperuser,
    CreatedAt,
}

#[derive(Iden)]
enum UserInfo {
    Table,
    Id,
    UserId,
    DisplayName,
    Photo,
}

#[derive(Iden)]
enum UserInfoTs {
    Table,
    Id,
    UserId,
    Weight,
    Height,
    BodyFat,
    MuscleMass,
    CreatedAt,
}
