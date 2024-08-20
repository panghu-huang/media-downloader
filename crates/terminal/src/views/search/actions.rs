pub enum SearchAction {
  Pending,
  StartEditing,
  KeywordChanged,
  Search(String),
  Completed,
  Cancelled,
  Clear,
}
