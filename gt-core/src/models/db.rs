use sea_orm::*;

use crate::entities::{prelude::*, *};
use crate::models::exercise::ExerciseSet;

impl From<ExerciseSet> for exercise_set::ActiveModel {
    fn from(exs: ExerciseSet) -> Self {
        let created_at = exs.created_at();
        match exs {
            ExerciseSet::Weighted(exs) => exercise_set::ActiveModel {
                reps: ActiveValue::Set(Some(exs.reps)),
                weight: ActiveValue::Set(Some(exs.weight)),
                created_at: ActiveValue::Set(created_at),
                ..Default::default()
            },
            ExerciseSet::Bodyweight(exs) => exercise_set::ActiveModel {
                reps: ActiveValue::Set(Some(exs.reps)),
                created_at: ActiveValue::Set(created_at),
                ..Default::default()
            },
        }
    }
}
