use entity::music;
use musicbrainz_rs::{entity::recording::Recording, Fetch};
use sea_orm::{prelude::*, Set, TryIntoModel};

// #[cfg(not(feature = "embed-ui"))]
pub mod cors;
pub mod jwt;
// pub mod lastfm;
pub mod room_id;

pub async fn get_music_or_store_music(
    db: &DatabaseConnection,
    mbid: Uuid,
) -> Result<music::Model, DbErr> {
    let music = music::Entity::find()
        .filter(music::Column::Mbid.eq(mbid))
        .one(db)
        .await?;

    match music {
        Some(music) => Ok(music),
        None => {
            let recording = Recording::fetch()
                .id(&mbid.to_string())
                .with_artists()
                .execute()
                .await
                .map_err(|e| DbErr::Custom(e.to_string()))?;

            dbg!(&recording);

            let artist = dbg!(recording.artist_credit)
                .as_deref()
                .and_then(<[_]>::first)
                .map(|credit| credit.name.clone());

            let music = music::ActiveModel {
                mbid: Set(mbid),
                title: Set(recording.title),
                artist: artist.map(Set).unwrap_or_default(),
                ..Default::default()
            };
            music
                .save(db)
                .await
                .and_then(music::ActiveModel::try_into_model)
        }
    }
}
