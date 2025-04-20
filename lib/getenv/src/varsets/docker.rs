// This is free and unencumbered software released into the public domain.

use crate::var;

/// See: https://docs.docker.com/reference/cli/docker/
pub fn docker_host() -> Option<String> {
    var("DOCKER_HOST")
}
