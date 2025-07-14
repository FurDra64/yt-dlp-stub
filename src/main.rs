use std::env;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let mut filtered_args = Vec::new();
    let mut skip_next = false;
    
    // 一部引数を除外
    for arg in &args[1..] {
        if skip_next {
            skip_next = false;
            continue;
        }
        
        if arg == "--exp-allow" {
            skip_next = true;
            continue;
        }

        if arg == "--wild-allow" {
            skip_next = true;
            continue;
        }
        filtered_args.push(arg.clone());
    }

    let exe_path = env::current_exe()?;
   
    if let Some(parent) = exe_path.parent() {
        let yt_dlp_path = parent.join("youtube-dl").join("yt-dlp.exe");

        let output = Command::new(yt_dlp_path)
            .arg("-U")
            .arg("-4")
            .args(&filtered_args)
            .output()?;
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    
    Ok(())
}
