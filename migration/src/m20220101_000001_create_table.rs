use sea_orm_migration::prelude::*;

#[derive(Iden)]
enum Room {
    Table,
    Id,
    PublicId,
    // UserId,
    CreationDate,
    ExpirationDate,
    UserCount,
}

// #[derive(Iden)]
// enum User {
//     Table,
//     Id,
//     CreationDate,
// }

#[derive(Iden)]
enum Music {
    Table,
    Id,
    /// MusicBrainz ID
    Mbid,
    Title,
    Artist,
}

#[derive(Iden)]
enum Vote {
    Table,
    Id,
    UserToken,
    RoomId,
    MusicId,
    VoteDate,
    Like,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // manager
        //     .create_table(
        //         Table::create()
        //             .table(User::Table)
        //             .if_not_exists()
        //             .col(
        //                 ColumnDef::new(User::Id)
        //                     .uuid()
        //                     .not_null()
        //                     .default(Value::Uuid(None))
        //                     .primary_key(),
        //             )
        //             .col(
        //                 ColumnDef::new(User::CreationDate)
        //                     .date_time()
        //                     .not_null()
        //                     .default(Value::ChronoDateTime(None)),
        //             )
        //             .to_owned(),
        //     )
        //     .await?;

        manager
            .create_table(
                Table::create()
                    .table(Room::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Room::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Room::PublicId)
                            .unsigned()
                            .not_null()
                            // If you remove unique key be sure that to check the expiration in the joining api
                            .unique_key(),
                    )
                    // .col(
                    //     ColumnDef::new(Room::UserId)
                    //         .uuid()
                    //         .not_null(),
                    // )
                    // .foreign_key(
                    //     ForeignKey::create()
                    //         .from_tbl(User::Table)
                    //         .from_col(User::Id)
                    //         .to_tbl(Room::Table)
                    //         .to_col(Room::UserId),
                    // )
                    .col(
                        ColumnDef::new(Room::CreationDate)
                            .date_time()
                            .not_null()
                            .default(Keyword::CurrentTimestamp),
                    )
                    .col(ColumnDef::new(Room::ExpirationDate).date_time().not_null())
                    .col(
                        ColumnDef::new(Room::UserCount)
                            .unsigned()
                            .not_null()
                            .default(Value::Int(Some(0))),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Music::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Music::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Music::Mbid).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Music::Title).text().not_null())
                    .col(ColumnDef::new(Music::Artist).text().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Vote::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Vote::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Vote::RoomId).unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Vote::Table)
                            .from_col(Vote::RoomId)
                            .to_tbl(Room::Table)
                            .to_col(Room::PublicId),
                    )
                    .col(ColumnDef::new(Vote::UserToken).uuid().not_null())
                    .col(ColumnDef::new(Vote::MusicId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Vote::Table)
                            .from_col(Vote::MusicId)
                            .to_tbl(Music::Table)
                            .to_col(Music::Mbid),
                    )
                    .col(
                        ColumnDef::new(Vote::VoteDate)
                            .date_time()
                            .not_null()
                            .default(Keyword::CurrentTimestamp),
                    )
                    .col(
                        ColumnDef::new(Vote::Like)
                            .boolean()
                            .not_null()
                            .default(Value::Bool(Some(true))),
                    )
                    .to_owned(),
            )
            .await?;

        return Ok(());
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Room::Table).to_owned())
            .await?;

        // manager
        //     .drop_table(Table::drop().table(User::Table).to_owned())
        //     .await?;

        manager
            .drop_table(Table::drop().table(Vote::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Music::Table).to_owned())
            .await?;

        return Ok(());
    }
}
