use chrono::Utc;
use email_address::EmailAddress;
use http::StatusCode;
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};
use sea_orm::*;

use crate::{AppError, AppState, Result};
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
        create_user(&signup_data, true, state).await?;
    }

    Ok(())
}

pub async fn create_user(
    data: &models::UserSignup,
    is_superuser: bool,
    state: &AppState,
) -> Result<i32> {
    if !EmailAddress::is_valid(&data.email)
        || data.display_name.is_empty()
        || data.username.is_empty()
        || data.password.is_empty()
    {
        return Err(AppError::ValidationError);
    }

    // Hash password to PHC string ($pbkdf2-sha256$...)
    let salt = SaltString::generate(&mut OsRng);
    let pw_hash = Pbkdf2
        .hash_password(data.password.as_bytes(), &salt)
        .map_err(|e| AppError::StatusCode(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .to_string();

    let new_user_login = user_login::ActiveModel {
        username: ActiveValue::Set(data.username.clone()),
        email: ActiveValue::Set(data.email.clone()),
        pw_hash: ActiveValue::Set(pw_hash),
        created_at: ActiveValue::Set(Utc::now().naive_utc()),
        is_superuser: ActiveValue::Set(is_superuser),
        ..Default::default()
    };
    let new_user = UserLogin::insert(new_user_login).exec(&state.conn).await?;

    let new_user_info = user_info::ActiveModel {
        display_name: ActiveValue::Set(data.display_name.clone()),
        user_id: ActiveValue::Set(new_user.last_insert_id),
        ..Default::default()
    };
    UserInfo::insert(new_user_info).exec(&state.conn).await?;

    Ok(new_user.last_insert_id)
}
