#[derive(
    Debug,
    Clone,
    Copy,
    strum_macros::EnumString,
    strum_macros::Display,
    strum_macros::IntoStaticStr,
    strum_macros::EnumIter
)]
pub enum LongTermFields {
    #[strum(serialize = "aws_access_key_id")]
    AwsAccessKeyId,

    #[strum(serialize = "aws_secret_access_key")]
    AwsSecretAccessKey,

    #[strum(serialize = "aws_mfa_device")]
    AwsMfaDevice,
}

#[derive(
    Debug,
    Clone,
    Copy,
    strum_macros::EnumString,
    strum_macros::Display,
    strum_macros::IntoStaticStr,
    strum_macros::EnumIter
)]
pub enum MainProfileFields {
    #[strum(serialize = "assumed_role")]
    AssumedRole,

    #[strum(serialize = "aws_access_key_id")]
    AwsAccessKeyId,

    #[strum(serialize = "aws_secret_access_key")]
    AwsSecretAccessKey,

    #[strum(serialize = "aws_mfa_device")]
    AwsMfaDevice,

    #[strum(serialize = "aws_session_token")]
    AwsSessionToken,

    #[strum(serialize = "aws_security_token")]
    AwsSecurityToken,

    #[strum(serialize = "expiration")]
    Expiration,
}
