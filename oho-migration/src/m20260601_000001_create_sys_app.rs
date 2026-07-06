use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysApp::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysApp::Id)
                            .big_integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(SysApp::AppCode).string_len(64).not_null())
                    .col(ColumnDef::new(SysApp::AppName).string_len(128).not_null())
                    .col(ColumnDef::new(SysApp::AppDesc).string_len(255))
                    .col(ColumnDef::new(SysApp::AppType).small_integer().not_null())
                    .col(ColumnDef::new(SysApp::Icon).string_len(100))
                    .col(ColumnDef::new(SysApp::Cover).string_len(500))
                    .col(ColumnDef::new(SysApp::HomePath).string_len(200))
                    .col(ColumnDef::new(SysApp::Config).text())
                    .col(ColumnDef::new(SysApp::OwnerId).big_integer())
                    .col(ColumnDef::new(SysApp::TeamIds).text())
                    .col(ColumnDef::new(SysApp::SystemMenuIds).text())
                    .col(ColumnDef::new(SysApp::Version).integer())
                    .col(
                        ColumnDef::new(SysApp::Status)
                            .small_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(SysApp::SortNum).integer())
                    .col(
                        ColumnDef::new(SysApp::DelFlag)
                            .small_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(SysApp::CreateBy).big_integer())
                    .col(ColumnDef::new(SysApp::CreateTime).timestamp())
                    .col(ColumnDef::new(SysApp::UpdateBy).big_integer())
                    .col(ColumnDef::new(SysApp::UpdateTime).timestamp())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sys_app_code")
                    .table(SysApp::Table)
                    .col(SysApp::AppCode)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysApp::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum SysApp {
    Table,
    Id,
    AppCode,
    AppName,
    AppDesc,
    AppType,
    Icon,
    Cover,
    HomePath,
    Config,
    OwnerId,
    TeamIds,
    SystemMenuIds,
    Version,
    Status,
    SortNum,
    DelFlag,
    CreateBy,
    CreateTime,
    UpdateBy,
    UpdateTime,
}
