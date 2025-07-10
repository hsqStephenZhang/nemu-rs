use std::fs;
use std::path::Path;
use std::process::Command;

const VIDEO_SRC: &str = "bad-apple.mp4";
const VIDEO_COL: u32 = 80;
const VIDEO_ROW: u32 = 25;

fn main() {
    // mkdir
    fs::create_dir_all("build").expect("Failed to create build directory");
    let video_out = Path::new("build").join("video.frame");

    // Run FFmpeg to generate the video.frame
    let status = Command::new("ffmpeg")
        .args(&[
            "-i",
            VIDEO_SRC,
            "-f",
            "image2pipe",
            "-s",
            &format!("{}x{}", VIDEO_COL, VIDEO_ROW),
            "-vcodec",
            "rawvideo",
            "-pix_fmt",
            "monow",
            video_out.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to execute ffmpeg");
    assert!(status.success(), "ffmpeg command failed");

    // Write config.rs with VIDEO_COL and VIDEO_ROW
    let config_code = format!(
        "pub const VIDEO_COL: usize = {};\npub const VIDEO_ROW: usize = {};\n",
        VIDEO_COL, VIDEO_ROW
    );

    fs::write("src/generated.rs", config_code).expect("Unable to write src/config.rs");

    // Tell Cargo to rerun build.rs if the source video changes
    println!("cargo:rerun-if-changed={}", VIDEO_SRC);
}
