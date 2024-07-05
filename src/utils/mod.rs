pub mod tmux;
pub mod term;
// Create a file in temporary dir and write the byte slice to it.
// fn store_in_tmp_file(buf: &[u8]) -> std::result::Result<std::path::PathBuf, Error> {
//     let (mut tmpfile, path) = tempfile::Builder::new()
//         .prefix(TEMP_FILE_PREFIX)
//         .rand_bytes(1)
//         .tempfile()?
//         // Since the file is persisted, the user is responsible for deleting it afterwards. However,
//         // Kitty does this automatically after printing from a temp file.
//         .keep()?;
//
//     tmpfile.write_all(buf)?;
//     tmpfile.flush()?;
//     Ok(path)
// }
pub(crate) fn is_ssh() -> bool {
    let ssh_client = std::env::var("SSH_CLIENT").is_ok();
    let ssh_tty = std::env::var("SSH_TTY").is_ok();
    ssh_client || ssh_tty
}
