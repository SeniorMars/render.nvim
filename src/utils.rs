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
