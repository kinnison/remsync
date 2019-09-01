//! Types for the remsync API regarding JWTs in use

use serde::{Deserialize, Serialize};

/// A Device JWT's claims
///
/// This is basically the claim structure created/used by the reMarkable
/// authorisation web application via auth0.  It is necessary that we
/// can decode these as the `auth0-userid` is used in discover URLs
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceToken {
    #[serde(rename = "auth0-userid")]
    /// The user ID from auth0 used in discovery
    auth0_user_id: String,
    #[serde(rename = "device-desc")]
    /// The device descriptor used during registration for this token
    device_desc: String,
    #[serde(rename = "device-id")]
    /// The device ID used during registration for this token
    device_id: String,
    #[serde(rename = "iat")]
    /// The time when this was issued.
    issued_at: u64,
    #[serde(rename = "iss")]
    /// The issuer identity
    issuer: String,
    #[serde(rename = "jti")]
    /// The identity of this token
    token_identity: String,
    #[serde(rename = "nbf")]
    /// The time before which this token is not valid
    not_valid_before: u64,
    #[serde(rename = "sub")]
    /// The subject of this token, always `rM Device Token`
    subject: String,
}

impl DeviceToken {
    /// Create a new DeviceToken to be used in the claims of the JWT
    ///
    /// ```
    /// # use remsync_api_types::*;
    /// let claims = DeviceToken::new(
    ///     "some-user-id", "some-device-desc", "some-device-id",
    ///     123456, "some-issuer", "some-token-id", 4321, "some-subject"
    /// );
    /// ```
    pub fn new(
        auth0_user_id: &str,
        device_desc: &str,
        device_id: &str,
        issued_at: u64,
        issuer: &str,
        token_identity: &str,
        not_valid_before: u64,
        subject: &str,
    ) -> Self {
        Self {
            auth0_user_id: auth0_user_id.to_owned(),
            device_desc: device_desc.to_owned(),
            device_id: device_id.to_owned(),
            issued_at,
            issuer: issuer.to_owned(),
            token_identity: token_identity.to_owned(),
            not_valid_before,
            subject: subject.to_owned(),
        }
    }

    /// Retrieve the auth0 userid from the JWT claims DeviceToken
    ///
    /// ```
    /// # use remsync_api_types::*;
    /// # let claims = DeviceToken::new(
    /// #     "some-user-id", "some-device-desc", "some-device-id",
    /// #     123456, "some-issuer", "some-token-id", 4321, "some-subject"
    /// # );
    /// assert_eq!(claims.auth0_user_id(), "some-user-id");
    /// ```
    pub fn auth0_user_id(&self) -> &str {
        &self.auth0_user_id
    }

    /// Retrieve the device descriptor from the JWT claims DeviceToken
    ///
    /// ```
    /// # use remsync_api_types::*;
    /// # let claims = DeviceToken::new(
    /// #     "some-user-id", "some-device-desc", "some-device-id",
    /// #     123456, "some-issuer", "some-token-id", 4321, "some-subject"
    /// # );
    /// assert_eq!(claims.device_desc(), "some-device-desc");
    /// ```
    pub fn device_desc(&self) -> &str {
        &self.device_desc
    }
    /// Retrieve the device id from the JWT claims DeviceToken
    ///
    /// ```
    /// # use remsync_api_types::*;
    /// # let claims = DeviceToken::new(
    /// #     "some-user-id", "some-device-desc", "some-device-id",
    /// #     123456, "some-issuer", "some-token-id", 4321, "some-subject"
    /// # );
    /// assert_eq!(claims.device_id(), "some-device-id");
    /// ```
    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    /// Retrieve the time at which the JWT claims DeviceToken was issued
    ///
    /// ```
    /// # use remsync_api_types::*;
    /// # let claims = DeviceToken::new(
    /// #     "some-user-id", "some-device-desc", "some-device-id",
    /// #     123456, "some-issuer", "some-token-id", 4321, "some-subject"
    /// # );
    /// assert_eq!(claims.issued_at(), 123456);
    /// ```
    pub fn issued_at(&self) -> u64 {
        self.issued_at
    }

    /// Retrieve the issuer from the JWT claims DeviceToken
    ///
    /// ```
    /// # use remsync_api_types::*;
    /// # let claims = DeviceToken::new(
    /// #     "some-user-id", "some-device-desc", "some-device-id",
    /// #     123456, "some-issuer", "some-token-id", 4321, "some-subject"
    /// # );
    /// assert_eq!(claims.issuer(), "some-issuer");
    /// ```
    pub fn issuer(&self) -> &str {
        &self.issuer
    }

    /// Retrieve the token's ID from the JWT claims DeviceToken
    ///
    /// ```
    /// # use remsync_api_types::*;
    /// # let claims = DeviceToken::new(
    /// #     "some-user-id", "some-device-desc", "some-device-id",
    /// #     123456, "some-issuer", "some-token-id", 4321, "some-subject"
    /// # );
    /// assert_eq!(claims.token_identity(), "some-token-id");
    /// ```
    pub fn token_identity(&self) -> &str {
        &self.token_identity
    }

    /// Retrieve the time before which the JWT claims DeviceToken is not valid
    ///
    /// ```
    /// # use remsync_api_types::*;
    /// # let claims = DeviceToken::new(
    /// #     "some-user-id", "some-device-desc", "some-device-id",
    /// #     123456, "some-issuer", "some-token-id", 4321, "some-subject"
    /// # );
    /// assert_eq!(claims.not_valid_before(), 4321);
    /// ```
    pub fn not_valid_before(&self) -> u64 {
        self.not_valid_before
    }

    /// Retrieve the subject from the JWT claims DeviceToken
    ///
    /// ```
    /// # use remsync_api_types::*;
    /// # let claims = DeviceToken::new(
    /// #     "some-user-id", "some-device-desc", "some-device-id",
    /// #     123456, "some-issuer", "some-token-id", 4321, "some-subject"
    /// # );
    /// assert_eq!(claims.subject(), "some-subject");
    /// ```
    pub fn subject(&self) -> &str {
        &self.subject
    }
}

/// User profile used as part of a UserToken JWT claim (see below)
///
/// In theory, this can be incomplete, but we include as much as we know
/// about because that allows for dealing with things expecting values
/// later.
#[derive(Debug, Serialize, Deserialize)]
pub struct Auth0Profile {
    #[serde(rename = "ClientID")]
    /// Client ID?  Always ""
    client_id: String,
    #[serde(rename = "Connection")]
    /// Connection string?  Always ""
    connection: String,
    #[serde(rename = "CreatedAt")]
    /// When this profile was created
    created_at: String,
    #[serde(rename = "Email")]
    /// The email address for this user
    email: String,
    #[serde(rename = "EmailVerified")]
    /// Whether or not the email address was verified
    email_verified: bool,
    #[serde(rename = "FamilyName")]
    /// The user's family name (Can be blank)
    family_name: String,
    #[serde(rename = "GivenName")]
    /// The user's given name (Can be blank)
    given_name: String,
    #[serde(rename = "IsSocial")]
    /// If this account is via social media of some kind
    is_social: bool,
    #[serde(rename = "Locale")]
    /// The user's locale (Can be blank)
    locale: String,
    #[serde(rename = "Name")]
    /// The user's name (reMarkable uses the email address again)
    name: String,
    #[serde(rename = "Nickname")]
    /// A nickname for the user (reMarkable uses the email address local part)
    nickname: String,
    #[serde(rename = "Picture")]
    /// A URL to a picture, usually gravatar
    picture: String,
    #[serde(rename = "UpdatedAt")]
    /// When this profile was last updated
    updated_at: String,
    #[serde(rename = "UserID")]
    /// The userid for this profile (MOST IMPORTANT)
    user_id: String,
}

impl Auth0Profile {
    // TODO: Finish this API

    /// Retrieve the user id from the given profile
    ///
    /// This is usually of the form "auth0|BUNCHOFHEXDIGITS"
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}

/// User token used as claims in JWT in reMarkable API
///
/// These are issued by the reMarkable authorisation web application and
/// use auth0 as the backing provider.  This structure forms the claims for
/// the token.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    #[serde(rename = "auth0-profile")]
    /// The profile of the user
    auth0_profile: Auth0Profile,
    #[serde(rename = "device-desc")]
    /// The device descriptor
    device_desc: String,
    #[serde(rename = "device-id")]
    /// The ID of the device
    device_id: String,
    #[serde(rename = "exp")]
    /// The time when this will expire / did expire.
    expires_at: u64,
    #[serde(rename = "iat")]
    /// The time when this was issued.
    issued_at: u64,
    #[serde(rename = "iss")]
    /// The issuer identity
    issuer: String,
    #[serde(rename = "jti")]
    /// The identity of this token
    token_identity: String,
    #[serde(rename = "nbf")]
    /// The time before which this token is not valid
    not_valid_before: u64,
    #[serde(rename = "sub")]
    /// The subject of this token, always `rM User Token`
    subject: String,
}

impl UserToken {
    // TODO: Complete this API
    /// Retrieve the auth0 information from this user token
    pub fn auth0_profile(&self) -> &Auth0Profile {
        &self.auth0_profile
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use serde_json::{from_str, to_string_pretty};

    fn round_trip<'de, T>(content: &'de str)
    where
        T: serde::Serialize + serde::Deserialize<'de>,
    {
        let obj: T = from_str(content.trim()).expect("Unable to parse");
        let s = to_string_pretty(&obj).expect("Unable to reserialize");
        assert_eq!(content.trim(), s.trim());
    }

    #[test]
    fn device_token() {
        round_trip::<DeviceToken>(
            r#"
{
  "auth0-userid": "auth0|5d6b9d86fe3c560e1e4da801",
  "device-desc": "desktop-linux",
  "device-id": "c81d2351-4723-4cfc-93ac-b032d3a053e7",
  "iat": 1567333901,
  "iss": "rM WebApp",
  "jti": "ck0tAmNaRSs=",
  "nbf": 1567333901,
  "sub": "rM Device Token"
}"#,
        );
    }

    #[test]
    fn user_token() {
        round_trip::<UserToken>(
            r#"
{
  "auth0-profile": {
    "ClientID": "",
    "Connection": "",
    "CreatedAt": "2019-09-01T10:29:26.440Z",
    "Email": "dsilvers+devremarkable@digital-scurf.org",
    "EmailVerified": true,
    "FamilyName": "",
    "GivenName": "",
    "IsSocial": false,
    "Locale": "",
    "Name": "dsilvers+devremarkable@digital-scurf.org",
    "Nickname": "dsilvers+devremarkable",
    "Picture": "https://s.gravatar.com/avatar/2ccb7db4162bf759176024ac4ef66e08?s=480&r=pg&d=https%3A%2F%2Fcdn.auth0.com%2Favatars%2Fds.png",
    "UpdatedAt": "2019-09-01T10:31:21.678Z",
    "UserID": "auth0|5d6b9d86fe3c560e1e4da801"
  },
  "device-desc": "desktop-linux",
  "device-id": "c81d2351-4723-4cfc-93ac-b032d3a053e7",
  "exp": 1567431013,
  "iat": 1567344613,
  "iss": "rM WebApp",
  "jti": "ck0tCMsf9jQ=",
  "nbf": 1567344613,
  "sub": "rM User Token"
}
"#
        );
    }
}
