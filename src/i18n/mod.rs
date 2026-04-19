pub mod no;

pub fn get_translation(key: &'static str) -> &'static str {
    no::get().get(key).copied().unwrap_or(key)
}
