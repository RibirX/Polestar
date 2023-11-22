use super::generate_state;

// https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id=0954f528-3a5b-4f51-af9e-1c1a38ca9576&response_type=code&redirect_uri=https%3A%2F%2Fapi.ribir.org%2Foauth%2Fmicrosoft&scope=openid%20offline_access%20user.read
pub fn microsoft_login_uri() -> String {
  let state = generate_state();
  let client_id = "0954f528-3a5b-4f51-af9e-1c1a38ca9576";
  let redirect_uri =
    url_escape::encode_www_form_urlencoded("https://api.ribir.org/oauth/microsoft");
  let scope = url_escape::encode_www_form_urlencoded("openid offline_access user.read");
  let response_type = "code";
  format!(
    "https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id={client_id}&response_type={response_type}&redirect_uri={redirect_uri}&scope={scope}&prompt=select_account&state={state}",
  )
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn microsoft() {
    let uri = microsoft_login_uri();
    assert_eq!(
      uri,
      "https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id=0954f528-3a5b-4f51-af9e-1c1a38ca9576&response_type=code&redirect_uri=https%3A%2F%2Fapi.ribir.org%2Foauth%2Fmicrosoft&scope=openid%20offline_access%20user.read&prompt=select_account&state="
    )
  }
}
