use sea_orm::*;

use super::user;
use crate::{AppState, Result};
use gt_core::entities::{prelude::*, *};
use gt_core::models;

pub struct PopulateData {
    pub superuser_name: String,
    pub superuser_password: String,
    pub superuser_email: String,
}

/// Populate the database with some information.
pub async fn populate(data: PopulateData, state: &AppState) -> Result<()> {
    add_exercise_names(&state.conn).await?;
    add_superuser(&data, state).await?;
    Ok(())
}

async fn add_exercise_names(conn: &DatabaseConnection) -> Result<()> {
    let names = vec![
        "Bench Press",
        "Deadlift",
        "Squat",
        "Leg Extension",
        "Cable Rows",
    ];
    let names_japanese = vec![
        "ベンチプレス",
        "デッドリフト",
        "スクワット",
        "レッグエクステンション",
        "シーテッドケーブルロー",
    ];

    for name in names.into_iter().chain(names_japanese.into_iter()) {
        let res = ExerciseName::find()
            .filter(exercise_name::Column::Name.eq(name))
            .one(conn)
            .await?;
        if res.is_none() {
            let exercise_name = exercise_name::ActiveModel {
                name: ActiveValue::Set(name.to_owned()),
                kind: ActiveValue::Set(models::ExerciseKind::Weighted.into()),
                ..Default::default()
            };

            ExerciseName::insert(exercise_name).exec(conn).await?;
        }
    }

    Ok(())
}

async fn add_superuser(data: &PopulateData, state: &AppState) -> Result<()> {
    let signup_data = models::UserSignup {
        display_name: "Administrator".to_string(),
        username: data.superuser_name.clone(),
        password: data.superuser_password.clone(),
        email: data.superuser_email.clone(),
    };
    let res = UserLogin::find()
        .filter(user_login::Column::Username.eq(data.superuser_name.clone()))
        .one(&state.conn)
        .await?;
    if res.is_none() {
        user::create_user(&signup_data, true, &state.conn).await?;
    }

    Ok(())
}
