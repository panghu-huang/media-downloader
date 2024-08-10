use scraper::ElementRef;

pub trait EasySelector {
  fn inner_text(&self) -> String;
}

impl<'a> EasySelector for ElementRef<'a> {
  fn inner_text(&self) -> String {
    self.text().fold(String::new(), |inner_text, text| {
      format!("{}{}", inner_text, text)
    })
  }
}
