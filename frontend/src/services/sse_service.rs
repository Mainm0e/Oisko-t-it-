use crate::models::event::AppEvent;
use crate::services::application_service::API_BASE_URL;
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub type_: NotificationType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationType {
    Comment,
    StatusUpdate,
}

pub static NOTIFICATIONS: GlobalSignal<Vec<Notification>> = Signal::global(|| Vec::new());

pub fn use_sse() {
    use_effect(move || {
        let mut ev_source = document::eval(&format!(
            "
                const eventSource = new EventSource('{}/events');
                eventSource.onmessage = (event) => {{
                    dioxus.send(event.data);
                }};
                eventSource.onerror = (err) => {{
                    console.error('SSE connection failed');
                }};
            ",
            API_BASE_URL
        ));

        spawn(async move {
            while let Ok(data) = ev_source.recv::<String>().await {
                if let Ok(event) = serde_json::from_str::<AppEvent>(&data) {
                    handle_event(event);
                }
            }
        });
    });
}

fn handle_event(event: AppEvent) {
    let (title, message, type_) = match event {
        AppEvent::CommentCreated {
            visitor_name,
            company,
            ..
        } => (
            "NEW SIGNAL".to_string(),
            format!("{} commented on mission {}", visitor_name, company),
            NotificationType::Comment,
        ),
        AppEvent::ApplicationStatusUpdated {
            company, status, ..
        } => (
            "STATUS CHANGE".to_string(),
            format!("Mission {} updated to {}", company, status),
            NotificationType::StatusUpdate,
        ),
    };

    let id = Uuid::new_v4();
    let notification = Notification {
        id,
        title,
        message,
        type_,
    };

    NOTIFICATIONS.write().push(notification);

    // Auto-remove after 8 seconds
    spawn(async move {
        TimeoutFuture::new(8000).await;
        NOTIFICATIONS.write().retain(|n| n.id != id);
    });
}
