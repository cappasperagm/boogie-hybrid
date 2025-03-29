use rfd::MessageDialog;

pub struct UI;

impl UI {
    pub fn show_message(title: &str, message: &str) {
        MessageDialog::new()
            .set_title(title)
            .set_description(message)
            .show();
    }
}