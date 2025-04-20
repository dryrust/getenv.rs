// This is free and unencumbered software released into the public domain.

use crate::var;

/// See: https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#env-vars
pub fn cuda_cache_disable() -> Option<bool> {
    var("CUDA_CACHE_DISABLE").map(|s| s == "1")
}
