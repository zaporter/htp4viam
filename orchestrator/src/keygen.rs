use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::Command;

//
// sed 's/PLACEHOLDER_PUBLIC_KEY/'"$(cat public_key.txt | tr -d '\n')"'/' install_public_key.sh > new_install_public_key.sh
//
pub fn gen_ssh_key(path: &PathBuf) -> Result<(), std::io::Error> {
    let key_path = path
        .to_str()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Invalid path"))?;
    let output = Command::new("ssh-keygen")
        .arg("-t")
        .arg("rsa")
        .arg("-b")
        .arg("2048")
        .arg("-N")
        .arg("")
        .arg("-f")
        .arg(key_path)
        .output()
        .expect("Failed to execute ssh-keygen command");

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(Error::new(
            ErrorKind::Other,
            format!("ssh-keygen error: {}", stderr),
        ))
    }
}
