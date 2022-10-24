#[cfg(test)]
mod tests {
    use inquire_derive::InquireInvoke;

    #[test]
    fn basic_inquire() {
        #[derive(Debug, InquireInvoke)]
        pub struct TestStruct {
            #[inquire(text(prompt_message = "What's your main_text?"))]
            pub main_text: String,
            #[inquire(text())]
            pub text: String,
        }

        impl Default for TestStruct {
            fn default() -> Self {
                Self {
                    main_text: String::from("test"),
                    text: String::from("test"),
                }
            }
        }

        let mut ex = TestStruct {
            main_text: String::from("difference1"),
            text: String::from("difference2"),
        };
        ex.inquire_mut().unwrap();
        println!("{:?}", ex);
        let df = TestStruct::from_inquire();
        println!("{:?}", df)
    }
}
