use super::generate_state;

pub fn google_login_uri() -> String {
  let state = generate_state();
  let access_type = "offline";
  let scope = url_escape::encode_www_form_urlencoded(
    "https://www.googleapis.com/auth/userinfo.profile https://www.googleapis.com/auth/userinfo.email openid",
  );
  let include_granted_scopes = "true";
  let response_type = "code";
  let client_id = "532126982705-htfeejm9s5e4tn2hnj5eiv2l174dm6dc.apps.googleusercontent.com";
  let redirect_uri = url_escape::encode_www_form_urlencoded("https://api.ribir.org/oauth/google");

  format!(
    "https://accounts.google.com/o/oauth2/v2/auth?access_type={access_type}&scope={scope}&include_granted_scopes={include_granted_scopes}&response_type={response_type}&client_id={client_id}&redirect_uri={redirect_uri}&prompt=select_account&state={state}",
  )
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn google() {
    let uri = google_login_uri();
    assert_eq!(
      uri,
      "https://accounts.google.com/o/oauth2/v2/auth?access_type=offline&scope=https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.profile%20https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.email%20openid&include_granted_scopes=true&response_type=code&client_id=532126982705-htfeejm9s5e4tn2hnj5eiv2l174dm6dc.apps.googleusercontent.com&redirect_uri=https%3A%2F%2Fapi.ribir.org%2Foauth%2Fgoogle&prompt=select_account&state="
    )
  }
}
