#[cfg(test)]
mod tests {
    use systemstat_warp::{Cipher256, Cipher512, CipherParser, UserConfig, UserConfigParser};

    #[tokio::test]
    async fn cipher_works() -> Result<(), Box<dyn std::error::Error>> {
        let source = "meteorCat123";

        let cipher256 = Cipher256::create(source.as_bytes());
        println!("sha256({}) = {:?}", source, cipher256.cipher());

        let cipher512 = Cipher512::create(source.as_bytes());
        println!("sha512({}) = {:?}", source, cipher512.cipher());


        Ok(())
    }


    #[tokio::test]
    async fn users_works() -> Result<(), Box<dyn std::error::Error>> {
        // first user
        let username_first = "MeteorCat";
        let password_first = "MeteorCatPassword";
        let cipher_first = Cipher256::create(password_first.as_bytes());
        let hash_first = cipher_first.cipher().unwrap_or_default();

        // last user
        let username_last = "MeteorCatLast";
        let password_last = "MeteorCatPasswordLast";
        let cipher_last = Cipher256::create(password_last.as_bytes());
        let hash_last = cipher_last.cipher().unwrap_or_default();

        // merge user
        let line = format!("{}:{},{}:{}", username_first, hash_first, username_last, hash_last);
        println!("Users = {}", line);

        // parse users
        let users: UserConfig = UserConfigParser::user_config(line.as_str());
        println!("User Struct = {:?}", users);


        Ok(())
    }
}