/// basically a direct copy of the bulge `vec_to_string` function,
/// but changed to be more "rusty"
pub fn vec_to_string(vec: Vec<String>) -> String {
    let len = vec.len();
    vec.into_iter().enumerate()
        .map(|(i, s)| if i == len - 1 { s } else { s + "," })
        .collect()
}

/// same as above, but for `string_to_vec`
pub fn string_to_vec(vec: String) -> Vec<String> {
    vec.split(',').map(|s| s.to_string()).collect()
}