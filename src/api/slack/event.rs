use axum::{routing::post, Json, Router};

pub async fn get_event_router() -> Router {
    Router::new().route("/event", post(control))
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Request {
    EventCallback {
        event_id: String,
        event_time: u64,
        event: Event,
    },
    UrlVerification {
        challenge: String,
    },
}

#[derive(Debug, serde::Deserialize)]
struct Event {
    user: String,
    r#type: EventType,
    text: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum EventType {
    Message,
}

async fn control(Json(request): Json<Request>) -> Json<serde_json::Value> {
    match request {
        Request::UrlVerification { challenge } => {
            Json(serde_json::json!({ "challenge": challenge }))
        }
        Request::EventCallback {
            event_id,
            event_time,
            event,
        } => {
            // TODO: Implement the logic to handle a message event
            println!(
                "Received event: id={:?}, time={:?}, user={:?}, type={:?}, text={:?}",
                event_id, event_time, event.user, event.r#type, event.text
            );
            Json(serde_json::json!({}))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_request_is_deserialised_correctly() {
        let request =
            serde_json::from_str(r#"{"type": "url_verification", "challenge": "haha"}"#).unwrap();
        match request {
            Request::UrlVerification { challenge } => {
                assert_eq!(challenge, "haha");
            }
            _ => panic!("Unexpected request: {:?}", request),
        }
    }

    #[test]
    fn test_event_callback_request_is_deserialised_correctly() {
        let request = serde_json::from_str(
            r#"{"type": "event_callback", "event_id": "Ev079KRHNHCY", "event_time": 1718843771, "event": {"user": "U0412QH4ZPE", "type": "message", "text": "hi, there?"}}"#
        ).unwrap();
        match request {
            Request::EventCallback {
                event_id,
                event_time,
                event,
            } => {
                assert_eq!(event_id, "Ev079KRHNHCY");
                assert_eq!(event_time, 1718843771);
                assert_eq!(event.user, "U0412QH4ZPE");
            }
            _ => panic!("Unexpected request: {:?}", request),
        }
    }

    #[test]
    fn test_other_type_request_is_not_deserialised() {
        let _ = serde_json::from_str::<Request>(
            r#"{ "type": "other", "event_id": "Ev079KRHNHCY", "event_time": 1718843771, "event": {"user": "U0412QH}"#,
        ).unwrap_err();
    }
}
