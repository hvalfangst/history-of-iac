pub mod en;

pub fn get_translation(key: &'static str) -> &'static str {
    en::get().get(key).copied().unwrap_or(key)
}
