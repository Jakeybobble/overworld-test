pub mod chunkdata;
pub mod constants;

pub fn yalify(string: &str) -> String {
    format!("🦕 Yal says: \"{}\"", string)
}