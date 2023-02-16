use chrono::Utc;
use email_address::EmailAddress;
use http::StatusCode;
use migration::{Alias, Expr, PostgresQueryBuilder, Query, SimpleExpr, SubQueryStatement};
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
        create_user(&signup_data, true, &state.conn).await?;
    }

    Ok(())
}

pub async fn create_user(
    data: &models::UserSignup,
    is_superuser: bool,
    conn: &DatabaseConnection,
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
    let new_user = UserLogin::insert(new_user_login).exec(conn).await?;

    let new_user_info = user_info::ActiveModel {
        display_name: ActiveValue::Set(data.display_name.clone()),
        user_id: ActiveValue::Set(new_user.last_insert_id),
        ..Default::default()
    };
    UserInfo::insert(new_user_info).exec(conn).await?;

    Ok(new_user.last_insert_id)
}

pub async fn get_exercise_sets(
    user_id: i32,
    limit_opt: Option<u64>,
    conn: &DatabaseConnection,
) -> Result<Vec<models::ExerciseSetQuery>> {
    let mut q = ExerciseSet::find()
        .filter(exercise_set::Column::UserId.eq(user_id))
        .column_as(exercise_name::Column::Name, "name")
        .column_as(exercise_name::Column::Kind, "kind")
        .order_by(exercise_set::Column::CreatedAt, Order::Desc)
        .join(
            JoinType::InnerJoin,
            exercise_set::Relation::ExerciseName.def(),
        );

    if let Some(limit) = limit_opt {
        q = q.limit(limit)
    }

    log::info!("{}", q.build(DbBackend::Sqlite).to_string());

    let res = q
        .into_model::<models::ExerciseSetJoinQuery>()
        .all(conn)
        .await?;

    let res = res
        .into_iter()
        .map(|exsj| match exsj.kind {
            models::ExerciseKind::Weighted => {
                let exs: models::ExerciseSetWeightedQuery = exsj.try_into()?;
                Ok(models::ExerciseSetQuery::Weighted(exs))
            }
            models::ExerciseKind::Bodyweight => {
                let exs: models::ExerciseSetBodyweightQuery = exsj.try_into()?;
                Ok(models::ExerciseSetQuery::Bodyweight(exs))
            }
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(res)
}

pub async fn get_user_info(
    user: user_login::Model,
    conn: &DatabaseConnection,
) -> Result<models::UserInfoQuery> {
    let user_info = user
        .find_related(UserInfo)
        .one(conn)
        .await?
        .ok_or(AppError::ResourceNotFound)?;

    // Pieced together from documentation and implementation of sea-query. I did not see an abstraction for a simple subquery that was
    // not part of an `IN` or other expressions so I use the constructor directly.
    // It implements a statement like this, in order to get the latest value for each of muscle_mass, body_fat, ...
    // SELECT
    //     (SELECT height FROM user_info_ts WHERE height IS NOT NULL ORDER BY created_at DESC LIMIT 1) AS height,
    //     (SELECT muscle_mass FROM user_info_ts WHERE muscle_mass IS NOT NULL ORDER BY created_at DESC LIMIT 1) AS muscle_mass
    //     ...
    let q = Query::select()
        .expr_as(
            SimpleExpr::SubQuery(
                None,
                Box::new(SubQueryStatement::SelectStatement(
                    Query::select()
                        .expr(Expr::value(user_info.display_name))
                        .to_owned(),
                )),
            ),
            Alias::new("display_name"),
        )
        .expr_as(
            SimpleExpr::SubQuery(
                None,
                Box::new(SubQueryStatement::SelectStatement(
                    Query::select()
                        .column((user_info_ts::Entity, user_info_ts::Column::Height))
                        .from(user_info_ts::Entity)
                        .and_where(user_info_ts::Column::UserId.eq(user.id))
                        .and_where(user_info_ts::Column::Height.is_not_null())
                        .order_by(user_info_ts::Column::CreatedAt, Order::Desc)
                        .limit(1)
                        .to_owned(),
                )),
            ),
            Alias::new("height"),
        )
        .expr_as(
            SimpleExpr::SubQuery(
                None,
                Box::new(SubQueryStatement::SelectStatement(
                    Query::select()
                        .column((user_info_ts::Entity, user_info_ts::Column::Weight))
                        .from(user_info_ts::Entity)
                        .and_where(user_info_ts::Column::UserId.eq(user.id))
                        .and_where(user_info_ts::Column::Weight.is_not_null())
                        .order_by(user_info_ts::Column::CreatedAt, Order::Desc)
                        .limit(1)
                        .to_owned(),
                )),
            ),
            Alias::new("weight"),
        )
        .expr_as(
            SimpleExpr::SubQuery(
                None,
                Box::new(SubQueryStatement::SelectStatement(
                    Query::select()
                        .column((user_info_ts::Entity, user_info_ts::Column::MuscleMass))
                        .from(user_info_ts::Entity)
                        .and_where(user_info_ts::Column::UserId.eq(user.id))
                        .and_where(user_info_ts::Column::MuscleMass.is_not_null())
                        .order_by(user_info_ts::Column::CreatedAt, Order::Desc)
                        .limit(1)
                        .to_owned(),
                )),
            ),
            Alias::new("muscle_mass"),
        )
        .expr_as(
            SimpleExpr::SubQuery(
                None,
                Box::new(SubQueryStatement::SelectStatement(
                    Query::select()
                        .column((user_info_ts::Entity, user_info_ts::Column::BodyFat))
                        .from(user_info_ts::Entity)
                        .and_where(user_info_ts::Column::UserId.eq(user.id))
                        .and_where(user_info_ts::Column::BodyFat.is_not_null())
                        .order_by(user_info_ts::Column::CreatedAt, Order::Desc)
                        .limit(1)
                        .to_owned(),
                )),
            ),
            Alias::new("body_fat"),
        )
        .to_owned();

    log::info!("{}", q.to_string(PostgresQueryBuilder));

    let res =
        models::UserInfoQuery::find_by_statement(StatementBuilder::build(&q, &DbBackend::Postgres))
            .one(conn)
            .await?
            .unwrap();

    log::info!("{:#?}", res);

    Ok(res)
}
