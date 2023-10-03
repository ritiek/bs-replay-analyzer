use std::ffi::CString;
use std::env;
use std::process::ExitCode;

#[link(name = "decompress")]
extern "C" {
    fn decompress_replay_file(input_path: *const i8, output_path: *const i8) -> u16;
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_replay_file> <output_replay_file>", args[0]);
        return ExitCode::from(1);
    }
    let input_file = CString::new(args[1].as_str()).unwrap();
    let output_file = CString::new(args[2].as_str()).unwrap();
    unsafe {
        decompress_replay_file(input_file.as_ptr(), output_file.as_ptr());
    }
    ExitCode::from(0)
}
