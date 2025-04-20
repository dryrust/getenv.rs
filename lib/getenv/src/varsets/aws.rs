// This is free and unencumbered software released into the public domain.

use crate::var;

/// See: https://docs.aws.amazon.com/cli/v1/userguide/cli-configure-envvars.html
pub fn aws_access_key_id() -> Option<String> {
    var("AWS_ACCESS_KEY_ID")
}

/// See: https://docs.aws.amazon.com/cli/v1/userguide/cli-configure-envvars.html
pub fn aws_default_region() -> Option<String> {
    var("AWS_DEFAULT_REGION")
}

/// See: https://docs.aws.amazon.com/cli/v1/userguide/cli-configure-envvars.html
pub fn aws_secret_access_key() -> Option<String> {
    var("AWS_SECRET_ACCESS_KEY")
}
