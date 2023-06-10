use chrono::Utc;
use email_address::EmailAddress;
use http::StatusCode;
use migration::{Alias, Expr, PostgresQueryBuilder, Query, SimpleExpr, SubQueryStatement};
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};
use sea_orm::*;

use crate::{AppError, Result};
use gt_core::entities::{prelude::*, *};
use gt_core::models;

pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let pw_hash = Pbkdf2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::StatusCode(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .to_string();
    Ok(pw_hash)
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
    let pw_hash = hash_password(&data.password)?;

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
