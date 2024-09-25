use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use regex::Regex;

        #[derive(Debug, Clone)]
        pub struct InputValidationRegex {
            regex_username: Regex,
            regex_mobile_num: Regex,
            regex_mobile_code: Regex,
            regex_email: Regex,
        }

        impl InputValidationRegex {
            pub fn get_regex() -> InputValidationRegex {
                InputValidationRegex {
                    regex_username: Regex::new(r"^[a-zA-Z].*[a-zA-Z0-9]{4,20}$").unwrap(),
                    regex_mobile_num: Regex::new(r"^[0-9]{11,11}$").unwrap(),
                    regex_mobile_code: Regex::new(r"^[0-9]{6,6}$").unwrap(),
                    regex_email: Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap()
                }
            }

            pub fn get_username_regex(&self) -> Regex {
                self.regex_username.clone()
            }

            pub fn get_email_regex(&self) -> Regex {
                self.regex_email.clone()
            }

            pub fn get_mobile_num_regex(&self) -> Regex {
                self.regex_mobile_num.clone()
            }

            pub fn get_mobile_code_regex(&self) -> Regex {
                self.regex_mobile_code.clone()
            }
        }
    }
}
