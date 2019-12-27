use std::process::Command;

pub fn extract_git_summary() -> String {
    let hash = git_command_output(&["rev-parse", "HEAD"], "hash")
        .trim()
        .to_owned();
    let files = git_command_output(&["status", "--porcelain"], "hash");
    if files.is_empty() {
        return format!("git+{}", hash);
    }
    let postfix = files
        .lines()
        .map(|ln| ln.chars().skip(3).collect::<String>().trim().to_owned())
        .collect::<Vec<_>>()
        .join("+");
    format!("git+{}+uncommitted_changed+{}", hash, postfix)
}

fn git_command_output(git_args: &[&str], git_description: &str) -> String {
    let output = match Command::new("git").args(git_args).output() {
        Ok(out) => {
            if !out.status.success() || !out.stderr.is_empty() {
                eprintln!("A problem occurred while getting git {}; is this a git repository that has commits?", git_description);
                match String::from_utf8(out.stderr) {
                    Ok(errmsg) => eprintln!("Code: {}, message: {}", out.status, errmsg),
                    Err(_) => eprintln!("Code: {} (stderr contained non-UTF8 data)", out.status),
                }
                panic!();
            }
            out.stdout
        }
        Err(err) => {
            eprintln!(
                "The command to get git {} failed; is this a git repository that has commits?",
                git_description
            );
            eprintln!("Error: {}", err);
            panic!();
        }
    };
    String::from_utf8(output).expect("git command output contained non-UTF8 data")
}
