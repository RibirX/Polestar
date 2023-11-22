use super::generate_state;

pub fn apple_login_uri() -> String {
  let state = generate_state();
  let client_id = "org.ribir.client";
  let redirect_uri = url_escape::encode_www_form_urlencoded("https://api.ribir.org/oauth/apple");
  let response_type = url_escape::encode_www_form_urlencoded("code id_token");
  let scope = url_escape::encode_www_form_urlencoded("name email");
  let response_mode = "form_post";
  format!(
    "https://appleid.apple.com/auth/authorize?client_id={client_id}&redirect_uri={redirect_uri}&response_type={response_type}&scope={scope}&response_mode={response_mode}&state={state}",
  )
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn apple() {
    let uri = apple_login_uri();
    assert_eq!(
      uri,
      "https://appleid.apple.com/auth/authorize?client_id=org.ribir.client&redirect_uri=https%3A%2F%2Fapi.ribir.org%2Foauth%2Fapple&response_type=code%20id_token&scope=name%20email&response_mode=form_post&state="
    )
  }
}
