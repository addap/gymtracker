#[cfg(not(target_arch = "wasm32"))]
use sea_orm::{DeriveActiveEnum, EnumIter, FromQueryResult};

use derive_more::From;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[cfg_attr(not(target_arch = "wasm32"), derive(EnumIter, DeriveActiveEnum))]
#[cfg_attr(
    not(target_arch = "wasm32"),
    sea_orm(rs_type = "i32", db_type = "Integer")
)]
pub enum ExerciseKind {
    #[cfg_attr(not(target_arch = "wasm32"), sea_orm(num_value = 0))]
    Weighted = 0,
    #[cfg_attr(not(target_arch = "wasm32"), sea_orm(num_value = 1))]
    Bodyweight = 1,
}

#[derive(Deserialize, Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(FromQueryResult))]
pub struct ExerciseName {
    pub name: String,
    pub kind: ExerciseKind,
}

#[derive(Deserialize, Serialize)]
pub struct ExerciseSetWeighted {
    pub name: String,
    pub reps: i32,
    pub weight: f64,
}

#[derive(Deserialize, Serialize)]
pub struct ExerciseSetBodyweight {
    pub name: String,
    pub reps: i32,
}

#[derive(Deserialize, Serialize, From)]
pub enum ExerciseSet {
    Weighted(ExerciseSetWeighted),
    Bodyweight(ExerciseSetBodyweight),
}

#[derive(Deserialize, Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(FromQueryResult))]
pub struct ExerciseSetWeightedQuery {
    id: i32,
    user_id: i32,
    name_id: i32,
    name: String,
    reps: i32,
    weight: f64,
    created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(FromQueryResult))]
pub struct ExerciseSetBodyweightQuery {
    id: i32,
    user_id: i32,
    name_id: i32,
    name: String,
    reps: i32,
    created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Serialize, From)]
pub enum ExerciseSetQuery {
    Weighted(ExerciseSetWeightedQuery),
    Bodyweight(ExerciseSetBodyweightQuery),
}

impl ExerciseSet {
    pub fn name(&self) -> &str {
        match self {
            ExerciseSet::Weighted(exs) => &exs.name,
            ExerciseSet::Bodyweight(exs) => &exs.name,
        }
    }

    pub fn kind(&self) -> ExerciseKind {
        match self {
            ExerciseSet::Weighted(_) => ExerciseKind::Weighted,
            ExerciseSet::Bodyweight(_) => ExerciseKind::Bodyweight,
        }
    }
}
