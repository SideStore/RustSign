// Jackson Coxson

pub mod anisette;
pub mod request;

#[derive(Debug)]
pub enum Error {
    HttpRequest,
    Parse,
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn fetch_anisette() {
        println!("gsa auth test");
        let password = std::env::var("apple_password").unwrap();
        let email = std::env::var("apple_email").unwrap();
        let ad = anisette::AnisetteData::from_url(anisette::SIDELOADLY_ANISETTE).unwrap();
        print!("{:?}", ad);
        let _ = request::GsaClient::new(email, password, ad);
    }
}
