//  local codes = require("image/backends/kitty/codes")
// local utils = require("image/utils")
//
// local uv = vim.uv
//  -- Allow for loop to be used on older versions
// if not uv then uv = vim.loop end
//
// local stdout = vim.loop.new_tty(1, false)
// if not stdout then error("failed to open stdout") end
//
// local is_SSH = (vim.env.SSH_CLIENT ~= nil) or (vim.env.SSH_TTY ~= nil)
//
// -- https://github.com/edluffy/hologram.nvim/blob/main/lua/hologram/terminal.lua#L77
// local get_chunked = function(str)
//   local chunks = {}
//   for i = 1, #str, 4096 do
//     local chunk = str:sub(i, i + 4096 - 1):gsub("%s", "")
//     if #chunk > 0 then table.insert(chunks, chunk) end
//   end
//   return chunks
// end
//
// ---@param data string
// ---@param tty? string
// ---@param escape? boolean
// local write = function(data, tty, escape)
//   if data == "" then return end
//
//   local payload = data
//   if escape and utils.tmux.is_tmux then payload = utils.tmux.escape(data) end
//   -- utils.debug("write:", vim.inspect(payload), tty)
//   if tty then
//     local handle = io.open(tty, "w")
//     if not handle then error("failed to open tty") end
//     handle:write(payload)
//     handle:close()
//   else
//     -- vim.fn.chansend(vim.v.stderr, payload)
//     stdout:write(payload)
//   end
// end
//
// local move_cursor = function(x, y, save)
//   if is_SSH and utils.tmux.is_tmux then
//     -- When tmux is running over ssh, set-cursor sometimes doesn't actually get sent
//     -- I don't know why this fixes the issue...
//     utils.tmux.get_cursor_x()
//     utils.tmux.get_cursor_y()
//   end
//   if save then write("\x1b[s") end
//   write("\x1b[" .. y .. ";" .. x .. "H")
//   uv.sleep(1)
// end
//
// local restore_cursor = function()
//   write("\x1b[u")
// end
//
// local update_sync_start = function()
//   write("\x1b[?2026h")
// end
//
// local update_sync_end = function()
//   write("\x1b[?2026l")
// end
//
// ---@param config KittyControlConfig
// ---@param data? string
// -- https://github.com/edluffy/hologram.nvim/blob/main/lua/hologram/terminal.lua#L52
// local write_graphics = function(config, data)
//   local control_payload = ""
//
//   -- utils.debug("kitty.write_graphics()", config, data)
//
//   for k, v in pairs(config) do
//     if v ~= nil then
//       local key = codes.control.keys[k]
//       if key then control_payload = control_payload .. key .. "=" .. v .. "," end
//     end
//   end
//   control_payload = control_payload:sub(0, -2)
//
//   if data then
//     if config.transmit_medium == codes.control.transmit_medium.direct then
//       local file = io.open(data, "rb")
//       data = file:read("*all")
//     end
//     data = utils.base64.encode(data):gsub("%-", "/")
//     local chunks = get_chunked(data)
//     local m = #chunks > 1 and 1 or 0
//     control_payload = control_payload .. ",m=" .. m
//     for i = 1, #chunks do
//       write("\x1b_G" .. control_payload .. ";" .. chunks[i] .. "\x1b\\", config.tty, true)
//       if i == #chunks - 1 then
//         control_payload = "m=0"
//       else
//         control_payload = "m=1"
//       end
//       uv.sleep(1)
//     end
//   else
//     -- utils.debug("kitty control payload:", control_payload)
//     write("\x1b_G" .. control_payload .. "\x1b\\", config.tty, true)
//   end
// end
//
// local write_placeholder = function(image_id, x, y, width, height)
//   local foreground = "\x1b[38;5;" .. image_id .. "m"
//   local restore = "\x1b[39m"
//
//   write(foreground)
//   for i = 0, height - 1 do
//     move_cursor(x, y + i + 1)
//     for j = 0, width - 1 do
//       write(codes.placeholder .. codes.diacritics[i + 1] .. codes.diacritics[j + 1])
//     end
//   end
//   write(restore)
// end
//
// return {
//   move_cursor = move_cursor,
//   restore_cursor = restore_cursor,
//   write = write,
//   write_graphics = write_graphics,
//   write_placeholder = write_placeholder,
//   update_sync_start = update_sync_start,
//   update_sync_end = update_sync_end,
// }

use crate::utils::{is_ssh, tmux};
use std::os::fd::RawFd;

use super::control::Control;
const CHUNK_SIZE: usize = 4096;

fn get_chunked(s: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    for chunk in s.chars().collect::<Vec<char>>().chunks(CHUNK_SIZE) {
        let chunk_str: String = chunk.iter().collect();
        let chunk_str = chunk_str.replace(' ', "");
        if !chunk_str.is_empty() {
            chunks.push(chunk_str);
        }
    }
    chunks
}

fn write(data: &str, tty: Option<RawFd>, escape: bool) {
    if data.is_empty() {
        return;
    }
    let mut payload = data.to_string();
    if escape && tmux::is_tmux() {
        payload = tmux::escape(&payload);
    }

    // if let Some(tty) = tty {
    //     let mut handle = unsafe { File::from_raw_fd(tty) };
    //     handle.write_all(payload.as_bytes()).unwrap();
    // } else {
    //     io::stdout().write_all(payload.as_bytes()).unwrap();
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chunked() {
        let s = "Hello, world! This is a test string.";
        let chunks = get_chunked(s);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], "Hello,world!Thisisateststring.");
    }

    // #[test]
    // fn test_write() {
    //     // Test writing to stdout
    //     write("Hello, world!", None, false);
    //     // You can capture stdout and verify the output if needed
    //
    //     // Test writing to a TTY (you may need to set up a dummy TTY for testing)
    //     // let tty = ...; // Set up a dummy TTY
    //     // write("Hello, TTY!", Some(tty), false);
    //     // Verify the output written to the TTY
    // }

    // #[test]
    // fn test_move_cursor() {
    //     // Test moving the cursor without saving position
    //     move_cursor(10, 20, false);
    //     // Verify the cursor position
    //
    //     // Test moving the cursor with saving position
    //     move_cursor(5, 15, true);
    //     // Verify the cursor position and saved position
    // }
    //
    // #[test]
    // fn test_restore_cursor() {
    //     // Save the cursor position
    //     write("\x1b[s", None, false);
    //
    //     // Move the cursor to a different position
    //     move_cursor(10, 20, false);
    //
    //     // Restore the cursor position
    //     restore_cursor();
    //
    //     // Verify the cursor position is restored to the saved position
    // }
    //
    // #[test]
    // fn test_update_sync_start_and_end() {
    //     // Test update_sync_start
    //     update_sync_start();
    //     // Verify the written control sequence
    //
    //     // Test update_sync_end
    //     update_sync_end();
    //     // Verify the written control sequence
    // }
    //
    // #[test]
    // fn test_write_graphics() {
    //     let config = KittyControlConfig {
    //         // Set up the necessary configuration
    //         // ...
    //     };
    //
    //     // Test writing graphics without data
    //     write_graphics(&config, None);
    //     // Verify the written control sequence
    //
    //     // Test writing graphics with data
    //     let data = "graphic_data";
    //     write_graphics(&config, Some(data));
    //     // Verify the written control sequence and data
    // }
    //
    // #[test]
    // fn test_write_placeholder() {
    //     let image_id = 1;
    //     let x = 10;
    //     let y = 20;
    //     let width = 5;
    //     let height = 3;
    //
    //     write_placeholder(image_id, x, y, width, height);
    //     // Verify the written placeholder characters and control sequences
    // }
}
