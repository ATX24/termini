use std::process::{exit, Command};

use clap::{Parser, Subcommand};

/// Tile your Terminal windows into a grid; maximize back.
#[derive(Parser)]
#[command(name = "termini", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Zoom OUT: tile every Terminal window on screen in a grid sized by window count.
    Out,
    /// Zoom IN: maximize the current Terminal window.
    In,
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Out => run(TILE_OUT),
        Commands::In => run(MAXIMIZE_IN),
    };
    match result {
        Ok(s) => {
            let s = s.trim();
            if !s.is_empty() {
                println!("termini: {s}");
            }
        }
        Err(e) => {
            eprintln!("termini: {e}");
            if e.to_lowercase().contains("assistive") || e.contains("-1719") {
                eprintln!(
                    "\ngrant Accessibility: System Settings -> Privacy & Security -> Accessibility\n\
                     then enable your terminal app."
                );
            }
            exit(1);
        }
    }
}

fn run(script: &str) -> Result<String, String> {
    let out = Command::new("osascript")
        .args(["-e", script])
        .output()
        .map_err(|e| format!("osascript failed to launch: {e}"))?;
    if !out.status.success() {
        let msg = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(if msg.is_empty() {
            format!("osascript exited with {}", out.status)
        } else {
            msg
        });
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

// ---- AppleScript ------------------------------------------------------------
//
// We count / position OS-level windows via `tell application "System Events"`.
// AppleScript's `tell application "Terminal" to windows` counts TABS, not
// containers, which is why every previous approach went sideways. System
// Events operates on real macOS windows, one per visible container.

const TILE_OUT: &str = r#"
use framework "AppKit"
use scripting additions

tell application "Terminal" to activate
delay 0.1

-- Usable screen area (excludes menu bar + Dock) via NSScreen.visibleFrame.
-- Cocoa y-origin is the BOTTOM of the screen, so we flip to UI coords (y-top).
set mainScreen to current application's NSScreen's mainScreen()
set vf to mainScreen's visibleFrame()
set sf to mainScreen's frame()
set screenH to (second item of second item of sf) as integer
set xMin to (first item of first item of vf) as integer
set wTotal to (first item of second item of vf) as integer
set hTotal to (second item of second item of vf) as integer
set yMin to screenH - ((second item of first item of vf) as integer) - hTotal

tell application "System Events"
    tell process "Terminal"
        set nWins to count of windows
        if nWins < 1 then return "0 Terminal windows"
        set nCols to 1
        repeat while (nCols * nCols) < nWins
            set nCols to nCols + 1
        end repeat
        set nRows to (nWins + nCols - 1) div nCols
        set wCell to wTotal div nCols
        set hCell to hTotal div nRows

        repeat with i from 1 to nWins
            set cIdx to (i - 1) mod nCols
            set rIdx to (i - 1) div nCols
            set xA to xMin + cIdx * wCell
            set yA to yMin + rIdx * hCell
            try
                set position of window i to {xA, yA}
                set size of window i to {wCell, hCell}
            end try
        end repeat
        return "tiled " & (nWins as string) & " window(s) into " & (nCols as string) & "x" & (nRows as string)
    end tell
end tell
"#;

const MAXIMIZE_IN: &str = r#"
use framework "AppKit"
use scripting additions

tell application "Terminal" to activate
delay 0.1

set mainScreen to current application's NSScreen's mainScreen()
set vf to mainScreen's visibleFrame()
set sf to mainScreen's frame()
set screenH to (second item of second item of sf) as integer
set xMin to (first item of first item of vf) as integer
set wTotal to (first item of second item of vf) as integer
set hTotal to (second item of second item of vf) as integer
set yMin to screenH - ((second item of first item of vf) as integer) - hTotal

tell application "System Events"
    tell process "Terminal"
        if (count of windows) < 1 then return "no Terminal windows"
        try
            set position of window 1 to {xMin, yMin}
            set size of window 1 to {wTotal, hTotal}
        end try
        return "maximized"
    end tell
end tell
"#;
