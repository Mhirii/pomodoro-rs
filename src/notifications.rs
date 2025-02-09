use notify_rust::Notification;

pub fn notify(title: &str, body: &str) {
    let mut notification = Notification::new();
    notification.summary(title).body(body).show().unwrap();
}
