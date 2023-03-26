use google_calendar::{Calendar, Event, InsertableEvent};
use google_authz::Authenticator;
use google_oauth2::{AccessToken, UserCredentials};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;
    let refresh_token = env::var("REFRESH_TOKEN")?;

    let auth = Authenticator::new(
        &client_id,
        &client_secret,
        UserCredentials::new(
            None,
            None,
            Some(refresh_token),
            None,
        ),
        None,
    );

    let access_token = auth.access_token()?;
    let calendar = Calendar::new(AccessToken::new(access_token.secret().to_string()));

    let event = InsertableEvent {
        summary: "Test Event".to_string(),
        description: Some("This is a test event".to_string()),
        start_date_time: "2023-02-01T09:00:00-07:00".to_string(),
        end_date_time: "2023-02-01T10:00:00-07:00".to_string(),
        ..InsertableEvent::default()
    };

    let inserted_event = calendar.insert_event("primary", &event)?;

    println!("Event created: {}", inserted_event.html_link);

    Ok(())
}
