use std::process::Command;

pub fn open(url: &str) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(&["/C", "start", url])
        .spawn()?;

    #[cfg(target_os = "macos")]
    Command::new("open")
        .arg(url)
        .spawn()?;

    #[cfg(target_os = "linux")]
    Command::new("xdg-open")
        .arg(url)
        .spawn()?;

    Ok(())
}