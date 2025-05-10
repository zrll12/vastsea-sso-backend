use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{array, boolean, pk_auto, string, timestamp_with_time_zone};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(pk_auto(Permission::Id))
                    .col(string(Permission::OwnerSegment))
                    .col(array(Permission::Segments, ColumnType::Text))
                    .col(string(Permission::FullName))
                    .col(string(Permission::Description))
                    .col(boolean(Permission::Enabled))
                    .col(timestamp_with_time_zone(Permission::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Permission::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;
        manager.get_connection().execute_unprepared(r#"
            CREATE TRIGGER update_permission_updated_at
            BEFORE UPDATE ON permission
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at_column();"#).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-permission-owner-segment")
                    .table(Permission::Table)
                    .col(Permission::OwnerSegment)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-permission-segments-gin")
                    .table(Permission::Table)
                    .col(Permission::Segments)
                    .index_type(IndexType::Custom(DynIden::new("gin")))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permission::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Permission {
    Table,
    Id,
    OwnerSegment,
    Segments,
    FullName,
    Description,
    Enabled,
    CreatedAt,
    UpdatedAt,
}
