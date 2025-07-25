use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use reqwest::blocking::get;

const YT_DLP_URL: &str = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe";

fn download_yt_dlp(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(YT_DLP_URL)?;
    let content = response.bytes()?;
    
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let mut file = fs::File::create(path)?;
    file.write_all(&content)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let mut filtered_args = Vec::new();
    let mut skip_next = false;
    
    // 一部引数を除外
    // 条件分岐は
    for arg in &args[1..] {
        match arg.as_str() {
            "--exp-allow" | "--wild-allow" => skip_next = true,
            _ if skip_next => skip_next = false,
            _ => filtered_args.push(arg.clone()),
        }
    }

    let exe_path = env::current_exe()?;

    if let Some(parent) = exe_path.parent() {
        let yt_dlp_path = parent.join("youtube-dl").join("yt-dlp.exe");
        let temp_dir = parent.join("youtube-dl").join("Temp");

        if !yt_dlp_path.exists() {
            download_yt_dlp(&yt_dlp_path)?;
        }

        if !temp_dir.exists() {
            fs::create_dir_all(&temp_dir)?;
        }

        let output = Command::new(yt_dlp_path)
            .env("TEMP", temp_dir.clone())
            .env("TMP", temp_dir.clone())
            .arg("-U")
            .arg("-4")
            .args(&filtered_args)
            .output()?;
        if !output.status.success() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            return Err("yt-dlp command failed".into());
        }    
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    Ok(())
}
