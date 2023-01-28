//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_login")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub pw_hash: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::exercise_set::Entity")]
    ExerciseSet,
    #[sea_orm(has_one = "super::user_info::Entity")]
    UserInfo,
    #[sea_orm(has_many = "super::user_info_ts::Entity")]
    UserInfoTs,
}

impl Related<super::exercise_set::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExerciseSet.def()
    }
}

impl Related<super::user_info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserInfo.def()
    }
}

impl Related<super::user_info_ts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserInfoTs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}