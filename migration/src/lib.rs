pub use sea_orm_migration::prelude::*;

mod m20250508_153220_create_user;
mod m20250508_164621_create_permission;
mod m20250508_171534_create_update_at_function;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250508_153220_create_user::Migration),
            Box::new(m20250508_171534_create_update_at_function::Migration),
            Box::new(m20250508_164621_create_permission::Migration),
        ]
    }
}
