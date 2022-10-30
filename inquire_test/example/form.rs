use inquire::Password;
use inquire_derive::InquireForm;

#[derive(Debug, Default, InquireForm)]
pub struct FormLogin {
    #[inquire(text(
        prompt_message = "What's your username?",
        placeholder_value = "Your username",
        help_message = "my helper message for path",
    ))]
    pub username: String,
    #[inquire(password(
        prompt_message = "What's your password?",
        help_message = "use your custom password",
        formatter = "&|_| String::from(\"xoxox\")",
        validators = "Password::DEFAULT_VALIDATORS",
        display_mode = "masked",
        enable_display_toggle = "true"
    ))]
    pub password: String,
}

#[derive(Debug, Default, InquireForm)]
pub struct FormData {
    #[inquire(text(
        prompt_message = "How old are you?",
        placeholder_value = "Your username",
        help_message = "my helper message for path",
    ))]
    pub age: String,
}

#[derive(Debug, Default, InquireForm)]
pub struct FormTest {
    #[inquire(nested())]
    pub login: FormLogin,
    #[inquire(nested())]
    pub data: FormData,
}

fn main() {
    let mut ex = FormTest::default();
    ex.inquire_mut().unwrap();
    println!("{:?}", ex);
}
