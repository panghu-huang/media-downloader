use protocol::channel::SearchMediaResponse;

pub enum SearchAction {
  Pending,
  StartEditing,
  KeywordChanged,
  Search(String),
  Completed(SearchMediaResponse),
  Cancelled,
  Error(String),
}
