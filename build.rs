use lyre::check_hashes;

fn main() -> Result<(), String> {
    match check_hashes() {
        Ok(hashes_matched) => {
            if hashes_matched {
                Ok(())
            } else {
                Err("Hashes did not match, please re-run lyre".into())
            }
        }
        Err(e) => Err(format!(
            "You likely need to re-run lyre:\n{}",
            e.to_string()
        )),
    }
}
