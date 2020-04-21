use desktop_ntfy::send_notification;

#[test]
fn it_works() {
    send_notification("Integration Test", "If you see this, it works!")
        .expect("It doesn't work :(");
}
