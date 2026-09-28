use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_trim::{string_trim, vec_string_trim};

use crate::domain::entities::{Event, RepeatPeriod};
use crate::domain::timezone::Timezone;
use crate::repository::errors::{FindError, UpdateError};
use crate::repository::event::Repository;

#[derive(Deserialize, Clone)]
pub struct Request {
    pub id: u32,
    #[serde(deserialize_with = "string_trim")]
    pub name: String,
    pub timestamp: i64,
    pub timezone: String,
    pub repeat: String,
    #[serde(deserialize_with = "vec_string_trim")]
    pub participants: Vec<String>,
    #[serde(skip_deserializing)]
    pub channel: String,
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub id: u32,
    pub timestamp: i64,
    pub timezone: Timezone,
    pub repeat: RepeatPeriod,
}

#[derive(PartialEq, Debug)]
pub enum Error {
    BadRequest,
    Conflict,
    NotFound,
    Unknown,
}

pub async fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
    let existing_event = match repo.clone().find_event(req.id.clone(), req.channel).await {
        Ok(event) => event,
        Err(error) => {
            return Err(match error {
                FindError::NotFound => Error::NotFound,
                FindError::Unknown => Error::Unknown,
            })
        }
    };

    let event = Event {
        id: existing_event.id,
        name: req.name.clone(),
        timestamp: req.timestamp,
        timezone: Timezone::from(req.timezone.clone()),
        repeat: RepeatPeriod::try_from(req.repeat.clone()).map_err(|_| Error::BadRequest)?,
        participants: req
            .participants
            .into_iter()
            .map(|user| {
                existing_event
                    .participants
                    .iter()
                    .find(|participant| participant.user == user)
                    .cloned()
                    .unwrap_or_else(|| user.into())
            })
            .collect(),
        channel: existing_event.channel,
        team_id: existing_event.team_id,
        deleted: false,
    };

    match repo.update_event(event.clone()).await {
        Ok(..) => Ok(Response {
            id: event.id,
            timestamp: event.timestamp,
            timezone: event.timezone,
            repeat: event.repeat,
        }),
        Err(err) => Err(match err {
            UpdateError::Conflict => Error::Conflict,
            UpdateError::NotFound => Error::NotFound,
            UpdateError::Unknown => Error::Unknown,
        }),
    }
}
