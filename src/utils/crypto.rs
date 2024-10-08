use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use argon2::{
            password_hash::{
                rand_core::OsRng,
                PasswordHash, PasswordHasher, SaltString    //, PasswordVerifier
            },
            Argon2
        };
        use leptos::server_fn::ServerFnError;

        pub fn get_parsed_hash(password: &str, salt_seed: &str) -> Result<String, ServerFnError> {
            // Argon2 with default params (Argon2id v19)
            let argon2_hash = Argon2::default();

            let b_password = password.to_string().into_bytes();
            // let salt = SaltString::generate(&mut OsRng);

            let salt = match SaltString::from_b64(salt_seed) {
                Ok(s) => s,
                Err(e) => return Err(ServerFnError::Args(e.to_string())),
            };

            // Raw Hash password - $argon2id$v=19$...
            let password_hash = match argon2_hash.hash_password(&b_password, &salt) {
                Ok(p) => p.to_string(),
                Err(e) => return Err(ServerFnError::Args(e.to_string())),
            };

            // Create PHC string.
            //
            // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
            // `Argon2` instance.
            let parsed_hash = match PasswordHash::new(&password_hash) {
                Ok(pass_h) => {
                    if let Some(p) = pass_h.hash {
                        p.to_string()
                    } else {
                        String::new()
                    }
                },
                Err(e) => return Err(ServerFnError::Args(e.to_string())),
            };

            Ok(parsed_hash)
        }


        pub fn get_salt() -> String {
            SaltString::generate(&mut OsRng).to_string()
        }

        // 没用的函数
        // Do not use
        // pub fn is_verified(password: String, parsed_hash: PasswordHash) -> bool {
        //     let b_password = password.clone().into_bytes();
        //     Argon2::default().verify_password(&b_password, &parsed_hash).is_ok()
        // }
    }
}
