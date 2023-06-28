use axum::http::StatusCode;
use axum_test_helper::TestClient;
use chrono::{Duration, Utc};
use migration::MigratorTrait;
use sea_orm::Database;
use serde_json::json;

use crate::utils::jwt;

use super::{router, state::ApiState};

#[tokio::test]
async fn test_admin_login() {
    jwt::set_jwt_secret("secret");
    let db = Database::connect("sqlite::memory:").await.unwrap();
    let username = "admin";
    // password: admin
    let password = "$argon2id$v=19$m=19456,t=2,p=1$bjFCSXBGR3pJclBraDFOSA$Aiqx8jvWC8UT8Xj9K37DqA";
    let state = ApiState::new(db, username.to_string(), password.to_string());
    let api = router(state);

    let client = TestClient::new(api);

    // Successful login

    let body = json!( {
        "username": "admin",
        "password": "admin"
    });

    let res = client.post("/admin/login").json(&body).send().await;
    assert_eq!(res.status(), StatusCode::OK, "Successful login");

    // Wrong password

    let body = json!( {
        "username": "admin",
        "password": "wrong"
    });
    let res = client.post("/admin/login").json(&body).send().await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED, "Wrong password");

    // Wrong username

    let body = json!( {
        "username": "wrong",
        "password": "admin"
    });
    let res = client.post("/admin/login").json(&body).send().await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED, "Wrong username");

    // Missing username

    let body = json!( {
        "password": "admin"
    });
    let res = client.post("/admin/login").json(&body).send().await;
    assert_ne!(res.status(), StatusCode::OK, "Missing username");
}

#[tokio::test]
async fn test_room_create_and_join() {
    let admin_token = jwt::admin_token();

    let db = Database::connect("sqlite::memory:").await.unwrap();
    migration::Migrator::up(&db, None).await.unwrap();
    let username = "admin";
    // password: admin
    let password = "$argon2id$v=19$m=19456,t=2,p=1$bjFCSXBGR3pJclBraDFOSA$Aiqx8jvWC8UT8Xj9K37DqA";
    let state = ApiState::new(db, username.to_string(), password.to_string());
    let client = TestClient::new(router(state));

    // Create room

    let res = client
        .post("/room")
        .json(&json!({
            "id": "AAAAAA",
            "expiration": Utc::now() + Duration::hours(5)
        }))
        .header("Authorization", admin_token)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK, "Create rooom");

    // Join the room

    let res = client.get("/room/AAAAAA/join").send().await;
    let status = res.status();
    eprintln!("{}", res.text().await);
    assert_eq!(status, StatusCode::OK, "Join room");

    // Join error

    let res = client.get("/room/ZZZZZZ/join").send().await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "Room does not exist");

    // Room id error

    let res = client.get("/room/ZZZZ/join").send().await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST, "Id to short");

    let res = client.get("/room/ZZZZZZZZZ/join").send().await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST, "Id to short");
}
