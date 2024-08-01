use open::that;


pub fn open(url: &str) -> std::io::Result<()> {
   that(url)?;
   Ok(())
}