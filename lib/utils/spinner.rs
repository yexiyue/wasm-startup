use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub struct Spinner {
    pub spinner: ProgressBar,
    pub text: String,
}

impl Spinner {
    pub fn new(text: String) -> Self {
        let res = Self {
            spinner: ProgressBar::new_spinner(),
            text,
        };
        res.spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["/", "|", "\\", "-", "/"])
                .template("{spinner:.green} {msg:.blue} {elapsed:.yellow}")
                .unwrap(),
        );
        res
    }

    pub fn start(&self) {
        self.spinner.enable_steady_tick(Duration::from_millis(100));
        self.spinner.set_message((&self.text).clone());
    }

    pub fn stop(&self, msg: Option<&'static str>) {
        if msg.is_some() {
            self.spinner.finish_with_message(msg.unwrap());
        } else {
            self.spinner.finish_and_clear()
        }
    }
}
