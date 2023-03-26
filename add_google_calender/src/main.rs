use google_calendar3::api::{Event, Events, Error};
use yup_oauth2::{read_service_account_key, ServiceAccountAuthenticator};

fn main() -> Result<()> {
    let sa_key = read_service_account_key("path/to/key.json").unwrap();
    let auth = ServiceAccountAuthenticator::builder(sa_key)
        .build()
        .unwrap();

    // カレンダーAPIクライアントを初期化する
    let calendar = google_calendar3::Calendar::new(
        hyper::Client::builder().build_http(),
        auth,
        Default::default(),
    );

    let today = chrono::Local::today().format("%Y-%m-%d").to_string();

    let events: Events = calendar
        .events()
        .list("primary")
        .time_min(&today)
        .time_max(&today)
        .doit()
        .map_err(Error::from)?;

    // イベントを出力する
    for event in events.items.unwrap() {
        println!("{}", event.summary.unwrap());
    }

    Ok(())
}
