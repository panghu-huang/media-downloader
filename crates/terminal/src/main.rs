use ::terminal::App;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let mut app = App::default();

  app.run().await?;

  Ok(())
}
