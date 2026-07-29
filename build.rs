fn main() {
    let version = match std::process::Command::new("git")
        .arg("describe")
        .arg("--long")
        .arg("--abbrev=7")
        .arg("--tags")
        .output()
    {
        Ok(output) if output.status.success() => {
            let raw = String::from_utf8_lossy(&output.stdout);
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                String::from("unknown")
            } else {
                let split = trimmed.split('-').collect::<Vec<_>>();
                if split.len() >= 3 {
                    let tag = if split[0].starts_with('v') || split[0].starts_with('V') {
                        &split[0][1..]
                    } else {
                        &split[0]
                    };
                    format!("{}-r{}-{}", tag, split[1], split[2])
                } else {
                    println!("cargo::warning=Unexpected git describe output: {trimmed}");
                    String::from("unknown")
                }
            }
        }
        Ok(_) => {
            println!("cargo::warning=Unable to get git version (no tags?)");
            String::from("unknown")
        }
        Err(err) => {
            println!("cargo::warning=Unable to get git version: {err:#}");
            String::from("unknown")
        }
    };

    let commit =
        match std::process::Command::new("git")
            .arg("log")
            .arg("--format=[%s]")
            .arg("-n")
            .arg("1")
            .output()
        {
            Ok(output) if output.status.success() => {
                let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if raw.is_empty() { String::from("unknown") } else { raw }
            }
            Ok(_) | Err(_) => {
                String::from("unknown")
            }
        };

    println!("cargo::rustc-env=GIT_VERSION={version} {commit}");
}
