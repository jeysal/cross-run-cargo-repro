use std::process::Command;

fn main() {
    print!(
        "{}",
        String::from_utf8_lossy(
            &Command::new(env!("CARGO"))
                .arg("version")
                .output()
                .unwrap()
                .stdout
        )
    )
}
