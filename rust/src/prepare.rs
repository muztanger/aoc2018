use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin prepare <day>");
        eprintln!("Example: cargo run --bin prepare 1");
        std::process::exit(1);
    }

    let day: u8 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Invalid day number: {}", args[1]);
        std::process::exit(1);
    });

    if !(1..=25).contains(&day) {
        eprintln!("Day must be between 1 and 25");
        std::process::exit(1);
    }

    // Get session cookie from environment variable
    let session = env::var("AOC_SESSION").unwrap_or_else(|_| {
        eprintln!("Error: AOC_SESSION environment variable not set");
        eprintln!();
        eprintln!("To set your session cookie:");
        eprintln!("1. Log in to https://adventofcode.com");
        eprintln!("2. Open browser DevTools (F12)");
        eprintln!("3. Go to Application/Storage > Cookies");
        eprintln!("4. Copy the value of the 'session' cookie");
        eprintln!("5. Set it as an environment variable:");
        eprintln!("   PowerShell: $env:AOC_SESSION=\"your_session_cookie\"");
        eprintln!("   Or create a .env file with: AOC_SESSION=your_session_cookie");
        std::process::exit(1);
    });

    let url = format!("https://adventofcode.com/2018/day/{}/input", day);
    let output_file = format!("inputs/day{:02}.txt", day);

    println!("Downloading input for day {}...", day);

    // Create inputs directory if it doesn't exist
    fs::create_dir_all("inputs").expect("Failed to create inputs directory");

    // Check if file already exists
    if Path::new(&output_file).exists() {
        print!("File {} already exists. Overwrite? (y/N): ", output_file);
        std::io::stdout().flush().unwrap();
        
        let mut response = String::new();
        std::io::stdin().read_line(&mut response).unwrap();
        
        if !response.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return;
        }
    }

    // Download the input
    let response = ureq::get(&url)
        .set("Cookie", &format!("session={}", session))
        .set("User-Agent", "github.com/aoc2018 by muzta")
        .call();

    match response {
        Ok(resp) => {
            let content = resp.into_string().unwrap_or_else(|e| {
                eprintln!("Failed to read response: {}", e);
                std::process::exit(1);
            });

            fs::write(&output_file, content).unwrap_or_else(|e| {
                eprintln!("Failed to write to file: {}", e);
                std::process::exit(1);
            });

            println!("✓ Successfully saved input to {}", output_file);
        }
        Err(ureq::Error::Status(code, _)) => {
            eprintln!("Failed to download input: HTTP {}", code);
            eprintln!("Make sure your AOC_SESSION cookie is valid and day {} is unlocked", day);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            std::process::exit(1);
        }
    }
}
