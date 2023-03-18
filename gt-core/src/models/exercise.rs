use anyhow::anyhow;
use chrono::{NaiveDateTime, Utc};
use derive_more::From;
use log::info;
use num_enum::{IntoPrimitive, TryFromPrimitive};
#[cfg(not(target_arch = "wasm32"))]
use sea_orm::{DeriveActiveEnum, EnumIter, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, TryFromPrimitive, IntoPrimitive, PartialEq)]
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(FromQueryResult))]
pub struct ExerciseName {
    pub name: String,
    pub kind: ExerciseKind,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(FromQueryResult))]
pub struct ExerciseNameQuery {
    pub name: String,
    pub kind: ExerciseKind,
    pub last_weight: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ExerciseSetWeighted {
    pub name: String,
    pub reps: i32,
    pub weight: f64,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ExerciseSetBodyweight {
    pub name: String,
    pub reps: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, From, PartialEq)]
pub enum ExerciseSet {
    Weighted(ExerciseSetWeighted),
    Bodyweight(ExerciseSetBodyweight),
}

#[derive(Debug, Clone, Deserialize, Serialize, From, PartialEq)]
pub struct ExerciseSetDelete {
    pub id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(FromQueryResult))]
pub struct ExerciseSetJoinQuery {
    pub id: i32,
    pub user_id: i32,
    pub name_id: i32,
    pub name: String,
    pub kind: ExerciseKind,
    pub reps: Option<i32>,
    pub weight: Option<f64>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(FromQueryResult))]
pub struct ExerciseSetWeightedQuery {
    pub id: i32,
    pub user_id: i32,
    pub name_id: i32,
    pub name: String,
    pub reps: i32,
    pub weight: f64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(FromQueryResult))]
pub struct ExerciseSetBodyweightQuery {
    pub id: i32,
    pub user_id: i32,
    pub name_id: i32,
    pub name: String,
    pub reps: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize, From, PartialEq)]
pub enum ExerciseSetQuery {
    Weighted(ExerciseSetWeightedQuery),
    Bodyweight(ExerciseSetBodyweightQuery),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(FromQueryResult))]
pub struct ExerciseGraphJoinQuery {
    pub name: String,
    pub reps: i32,
    pub weight: f64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize, From, PartialEq)]
pub struct ExerciseGraphQueryPerDate {
    pub date: chrono::NaiveDate,
    pub weights: Vec<(f64, i32)>,
}

#[derive(Debug, Clone, Deserialize, Serialize, From, PartialEq)]
pub struct ExerciseGraphQuery {
    pub name: String,
    pub per_date: Vec<ExerciseGraphQueryPerDate>,
}

#[derive(Debug, Clone, Deserialize, Serialize, From, PartialEq)]
pub struct PRWeightedQuery {
    pub name: String,
    pub pr: Vec<(f64, i32)>,
}

#[derive(Debug, Clone, Deserialize, Serialize, From, PartialEq)]
pub struct PRBodyweightQuery {
    pub name: String,
    pub pr: Vec<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, From, PartialEq)]
pub struct PRQuery {
    pub weighted: Vec<PRWeightedQuery>,
    pub bodyweight: Vec<PRBodyweightQuery>,
}

#[derive(Debug, Clone, Deserialize, Serialize, From, PartialEq)]
pub struct MergeNames {
    pub to_delete: String,
    pub to_expand: String,
}

impl ExerciseSetQuery {
    pub fn name(&self) -> &str {
        match self {
            ExerciseSetQuery::Weighted(exs) => &exs.name,
            ExerciseSetQuery::Bodyweight(exs) => &exs.name,
        }
    }

    pub fn id(&self) -> i32 {
        match self {
            ExerciseSetQuery::Weighted(exs) => exs.id,
            ExerciseSetQuery::Bodyweight(exs) => exs.id,
        }
    }
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

    pub fn created_at(&self) -> NaiveDateTime {
        let created_at = match self {
            ExerciseSet::Weighted(exs) => &exs.created_at,
            ExerciseSet::Bodyweight(exs) => &exs.created_at,
        };
        let x = NaiveDateTime::parse_from_str(created_at.as_str(), "%Y-%m-%dT%H:%M");
        info!("{:?}", x);
        x.unwrap_or(Utc::now().naive_utc())
    }
}

impl TryFrom<ExerciseSetJoinQuery> for ExerciseSetBodyweightQuery {
    type Error = anyhow::Error;

    fn try_from(value: ExerciseSetJoinQuery) -> Result<Self, Self::Error> {
        let reps = value
            .reps
            .ok_or(anyhow!("Malformed input. Field `reps` not present."))?;

        Ok(Self {
            id: value.id,
            user_id: value.user_id,
            name_id: value.name_id,
            name: value.name,
            reps,
            created_at: value.created_at,
        })
    }
}

impl TryFrom<ExerciseSetJoinQuery> for ExerciseSetWeightedQuery {
    type Error = anyhow::Error;

    fn try_from(value: ExerciseSetJoinQuery) -> Result<Self, Self::Error> {
        let reps = value
            .reps
            .ok_or(anyhow!("Malformed input. Field `reps` not present."))?;
        let weight = value
            .weight
            .ok_or(anyhow!("Malformed input. Field `weight` not present."))?;

        Ok(Self {
            id: value.id,
            user_id: value.user_id,
            name_id: value.name_id,
            name: value.name,
            reps,
            weight,
            created_at: value.created_at,
        })
    }
}
