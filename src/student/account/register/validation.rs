use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::use_context;
        use super::errors::RegistrationError;
        use super::InputRegistrationInfo;
        use crate::{state::AppState, utils::regex::InputValidationRegex};

        fn is_valid_username(validation_regex: &InputValidationRegex, input_username: &str) -> bool {
            //  between 5 - 30 charactors long without whitespace
            //  only allow alphabets and numbers
            //  can not start with number
            if input_username.chars().any(|c| c.is_whitespace())
                || !validation_regex.get_username_regex().is_match(input_username) {
                return false;
            }
            true
        }

        fn is_valid_password(input_password: &str) -> bool {
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

        fn is_valid_fullname(input_fullname: &str) -> bool {
            //  whitespace and '.' are allowed in full name
            //  user input must be sanitized before inserting into database
            if input_fullname.is_empty()
                || input_fullname.len() > 60
                || input_fullname.chars().any(|c| !c.is_alphanumeric() && c != '.' && c != ' ') {
                return false;
            }
            true
        }

        fn is_valid_email(validation_regex: &InputValidationRegex, input_email: &str) -> bool {
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

        pub fn verify_input_content(input_reg: &InputRegistrationInfo) -> Option<String> {
            //  取得软件状态
            let state = match use_context::<AppState>() {
                Some(s) => s,
                None => return Some(RegistrationError::UnknownError.to_string()),
            };

            let validation_regex = state.validation_regex;

            if !is_valid_username(&validation_regex, &input_reg.username) {
                return Some(RegistrationError::InvalidUsername.to_string());
            } else if !is_valid_password(&input_reg.password) {
                return Some(RegistrationError::InvalidPassword.to_string());
            } else if input_reg.password != input_reg.confirm_password {
                return Some(RegistrationError::PasswordNotMatch.to_string());
            } else if !is_valid_fullname(&input_reg.fullname) {
                return Some(RegistrationError::InvalidFullName.to_string());
            } else if !is_valid_email(&validation_regex, &input_reg.email)  {
                return Some(RegistrationError::InvalidEmailAddress.to_string());
            } else if !validation_regex.get_mobile_num_regex().is_match(&input_reg.mobile_num) {
                return Some(RegistrationError::InvalidMobileNumber.to_string());
            } else if !validation_regex.get_mobile_code_regex().is_match(&input_reg.mobile_verify_code) {
                return Some(RegistrationError::InvalidMobileVerifyCode.to_string());
            }

            None
        }
    }
}
