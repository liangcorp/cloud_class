use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::fmt;
        #[derive(Clone, Debug)]
        pub enum RegistrationError {
            InvalidUsername,
            InvalidPassword,
            PasswordNotMatch,
            InvalidFullName,
            InvalidEmailAddress,
            InvalidMobileNumber,
            InvalidMobileVerifyCode,
            // MobileVerifyFailed,
            ExistingUsername,
            ExistingMobileNumber,
            ExistingEmailAddress,
            ErrorDuringUserCreation,
            UnknownError,
        }

        impl fmt::Display for RegistrationError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    RegistrationError::InvalidUsername => write!(f, "用户名无效"),
                    RegistrationError::InvalidPassword => write!(f, "密码无效"),
                    RegistrationError::PasswordNotMatch => write!(f, "确认密码不符"),
                    RegistrationError::InvalidFullName => write!(f, "姓名无效"),
                    RegistrationError::InvalidEmailAddress => write!(f, "电子邮件无效"),
                    RegistrationError::InvalidMobileNumber => write!(f, "手机号无效"),
                    RegistrationError::InvalidMobileVerifyCode => write!(f, "验证码无效"),
                    // RegistrationError::MobileVerifyFailed => write!(f, "验证码错误"),
                    RegistrationError::ExistingUsername => write!(f, "用户名以注册"),
                    RegistrationError::ExistingMobileNumber => write!(f, "手机号以注册"),
                    RegistrationError::ExistingEmailAddress => write!(f, "油箱以注册"),
                    RegistrationError::ErrorDuringUserCreation => write!(f, "用户注册失败"),
                    RegistrationError::UnknownError => write!(f, "系统问题请稍后再试"),
                }
            }
        }
    }
}
