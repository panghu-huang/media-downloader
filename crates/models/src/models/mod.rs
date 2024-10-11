pub mod download_records;

use std::fmt::{self, Display, Formatter};
use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(
  Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct Id(pub i32);

pub type DateTime = chrono::DateTime<chrono::Utc>;

impl Deref for Id {
  type Target = i32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl From<i32> for Id {
  fn from(value: i32) -> Self {
    Id(value)
  }
}

impl Display for Id {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}
