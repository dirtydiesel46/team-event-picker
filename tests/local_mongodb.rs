use std::{collections::HashSet, sync::Arc};

use team_event_picker::{
    domain::events::{create_event, delete_event, pick_participant},
    repository::{
        errors::FindError,
        event::{MongoDbRepository, Repository},
    },
};

/// Runs only when explicitly requested, against a unique disposable database.
#[tokio::test]
#[ignore = "requires the local MongoDB service; see README"]
async fn event_lifecycle() {
    let uri = std::env::var("TEST_MONGODB_URL").expect("set TEST_MONGODB_URL");
    let name = format!("picker_test_{}", bson::oid::ObjectId::new().to_hex());
    let client = mongodb::Client::with_uri_str(&uri).await.unwrap();
    let repo = Arc::new(MongoDbRepository::new(&uri, &name, 5).await.unwrap());
    let channel = "C_LOCAL_TEST".to_string();
    let participants = vec!["U_ONE".to_string(), "U_TWO".to_string()];
    let created = create_event::execute(
        repo.clone(),
        create_event::Request {
            name: "Local smoke test".into(),
            timestamp: 1893456000,
            timezone: "UTC".into(),
            repeat: "none".into(),
            participants: participants.clone(),
            channel: channel.clone(),
            team_id: "T_LOCAL_TEST".into(),
            max_events: 100,
        },
    )
    .await
    .unwrap();
    assert_eq!(repo.count_events(channel.clone()).await.unwrap(), 1);
    assert_eq!(
        repo.find_event(created.id, "OTHER_CHANNEL".into())
            .await
            .err(),
        Some(FindError::NotFound)
    );

    let mut picked = HashSet::new();
    for _ in 0..2 {
        let pick = pick_participant::execute(
            repo.clone(),
            pick_participant::Request {
                event: created.id,
                channel: channel.clone(),
            },
        )
        .await
        .unwrap();
        assert!(
            picked.insert(pick.id),
            "participants must not repeat before a full round"
        );
    }
    assert_eq!(picked, participants.into_iter().collect());
    let stored = repo.find_event(created.id, channel.clone()).await.unwrap();
    assert!(stored.participants.iter().all(|p| p.picked));
    let next = pick_participant::execute(
        repo.clone(),
        pick_participant::Request {
            event: created.id,
            channel: channel.clone(),
        },
    )
    .await
    .unwrap();
    assert!(picked.contains(&next.id));
    let stored = repo.find_event(created.id, channel.clone()).await.unwrap();
    assert_eq!(stored.participants.iter().filter(|p| p.picked).count(), 1);

    delete_event::execute(
        repo.clone(),
        delete_event::Request {
            id: created.id,
            channel: channel.clone(),
        },
    )
    .await
    .unwrap();
    assert_eq!(repo.count_events(channel.clone()).await.unwrap(), 0);
    assert_eq!(
        repo.find_event(created.id, channel).await.err(),
        Some(FindError::NotFound)
    );
    client.database(&name).drop(None).await.unwrap();
}
