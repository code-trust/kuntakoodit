//! Look up Finnish municipality codes (kuntakoodi) by municipality name.
//!
//! Data comes from the Statistics Finland municipality classification and is
//! embedded in the crate at compile time.
//!
//! ```
//! let helsinki = kuntakoodit::by_name("helsingfors").unwrap();
//! assert_eq!(helsinki.code, "091");
//! ```

mod data;
mod municipality;
mod normalize;

pub use data::{
    all,
    by_code,
    by_name,
};
pub use municipality::Municipality;
