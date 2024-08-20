use crate::component::Component;
use crate::components::help_panel::{HelpPanel, KeyBinding};
use ratatui::prelude::*;

#[derive(Default)]
pub struct Dashboard;

impl Component for Dashboard {
  type Action = crate::actions::Action;

  fn render(&self, frame: &mut Frame, area: Rect) {
    let key_bindings = vec![
      KeyBinding {
        description: "Quit".into(),
        keys: vec!["Q".into()],
      },
      KeyBinding {
        description: "Help".into(),
        keys: vec!["H".into()],
      },
      KeyBinding {
        description: "Search".into(),
        keys: vec!["S".into()],
      },
    ];

    let help_panel = HelpPanel::new(key_bindings);

    frame.render_widget(help_panel, area);
  }
}
