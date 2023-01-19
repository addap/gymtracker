use gt_core::entities::{prelude::*, *};
use sea_orm::*;

/// Populate the database with some information.
pub async fn populate(conn: &DatabaseConnection) -> Result<(), DbErr> {
    let names = vec![
        "Bench Press",
        "Deadlift",
        "Squat",
        "Leg Extension",
        "Cable Row",
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
                ..Default::default()
            };

            ExerciseName::insert(exercise_name).exec(conn).await?;
        }
    }

    Ok(())
}
