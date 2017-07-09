mod heme;

pub use self::heme::*;

#[get("/")]
pub fn index() -> &'static str {
    "heme_meat: An eVite api backend for the red meat MEATup"
}
