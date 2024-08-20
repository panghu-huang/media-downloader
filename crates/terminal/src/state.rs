#[derive(Default, Clone, Copy, PartialEq)]
pub enum CurrentlyView {
  #[default]
  Dashboard,
  Search,
}

#[derive(Clone)]
pub struct AppState {
  pub currently_view: CurrentlyView,
  pub editing: bool,
}

impl Default for AppState {
  fn default() -> Self {
    Self {
      currently_view: CurrentlyView::Dashboard,
      editing: false,
    }
  }
}
