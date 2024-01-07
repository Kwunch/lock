/*
    Linux / Mac command to lock the computer for the current user:
*/
fn main() {
    // 1. Get the current user
    let user = whoami::username();

    // 2. Get Session Number of current user from loginctl
    let output = std::process::Command::new("loginctl")
        .output()
        .expect("failed to execute process");

    // Convert output to string then split by new line
    let output = String::from_utf8_lossy(&output.stdout).to_string();
    let output = output.split("\n").collect::<Vec<&str>>();

    // Find the line that contains the current user
    for line in output {
        if line.contains(&user) {
            // Get the session number
            let session_number = line.trim().split(" ").collect::<Vec<&str>>()[0];

            // 3. Lock the computer for the current user
            std::process::Command::new("loginctl")
                .arg("lock-session")
                .arg(session_number)
                .output()
                .expect("failed to execute process");
        }
    }
}

