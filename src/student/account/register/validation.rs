use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::utils::regex::InputValidationRegex;

        pub fn is_valid_username(validation_regex: &InputValidationRegex, input_username: &str) -> bool {
            //  between 5 - 30 charactors long without whitespace
            //  only allow alphabets and numbers
            //  can not start with number
            if input_username.chars().any(|c| c.is_whitespace())
                || !validation_regex.get_username_regex().is_match(input_username) {
                return false;
            }
            true
        }

        pub fn is_valid_password(input_password: &str) -> bool {
            //  not a valid password if it's empty,
            //  contains whitespace, less than 8 characters
            //  long and more than 100 characters
            if input_password.is_empty()
                || input_password.chars().any(|c| c.is_whitespace())
                || input_password.len() < 8
                || input_password.len() > 100 {
                    return false;
            }
            true
        }

        pub fn is_valid_fullname(input_fullname: &str) -> bool {
            //  whitespace and '.' are allowed in full name
            //  user input must be sanitized before inserting into database
            if input_fullname.is_empty()
                || input_fullname.len() > 60
                || input_fullname.chars().any(|c| !c.is_alphanumeric() && c != '.' && c != ' ') {
                return false;
            }
            true
        }

        pub fn is_valid_email(validation_regex: &InputValidationRegex, input_email: &str) -> bool {
            //  disallowed email address characters based on RFC 5322 standard
            //  domain names only allow big and small alphabets, numbers and '-'
            //  less than 256 characters long
            match input_email.chars().last() {
                Some(c) => if c == '.' || c == '-' { return false },
                None => return false,   // empty string
            }

            if !validation_regex.get_email_regex().is_match(input_email)
                || input_email.len() > 256 {
                return false;
            }

            let (mail_name, domain) = input_email.split_once('@').unwrap();

            if mail_name.contains('"')
                || validation_regex.get_email_forbidden_regex().is_match(mail_name)
                || domain.chars().any(|c| !c.is_ascii_alphanumeric() && c != '.' && c != '-') {
                return false;
            }

            true
        }

    }
}
