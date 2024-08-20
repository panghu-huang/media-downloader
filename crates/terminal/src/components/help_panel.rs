use ratatui::layout::Constraint;
use ratatui::style::Stylize;
use ratatui::widgets::{Cell, Row, Table};

pub struct KeyBinding {
  pub description: String,
  pub keys: Vec<String>,
}

pub struct HelpPanel {
  key_bindings: Vec<KeyBinding>,
}

impl HelpPanel {
  pub fn new(key_bindings: Vec<KeyBinding>) -> Self {
    Self { key_bindings }
  }
}

impl ratatui::widgets::Widget for HelpPanel {
  fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer)
  where
    Self: Sized,
  {
    let max_key_len = self
      .key_bindings
      .iter()
      .map(|kb| kb.keys.join("-").len())
      .max()
      .unwrap_or(1);

    let mut rows = vec![];

    for key_binding in &self.key_bindings {
      let key = format!("<{}>", key_binding.keys.join("-"));

      rows.push(Row::new(vec![
        Cell::from(key).bold().green(),
        Cell::from(key_binding.description.clone()),
      ]));
    }

    let max_key_len = max_key_len.min(3) + 3;

    let widths = vec![Constraint::Length(max_key_len as u16), Constraint::Min(0)];

    let table = Table::new(rows, widths);

    table.render(area, buf);
  }
}
