pub use sea_orm_migration::prelude::*;

mod m20230113_140607_create_user_table;
mod m20230113_140612_create_exercise_table;
mod m20230113_190450_asd;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230113_140607_create_user_table::Migration),
            Box::new(m20230113_140612_create_exercise_table::Migration),
            // Box::new(m20230113_190450_asd::Migration),
        ]
    }
}
