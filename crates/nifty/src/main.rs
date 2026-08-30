//! nifty, a NIF inspector.
//!
//! Usage:
//!   cargo run -p nifty -- [file.nif ...] [texture-dir ...] [options]
//!
//! Arguments are sorted by what they are, so files and directories can be given in any order.
//! Drop a .nif onto the window to open it, or a folder to add it as a texture directory.
//!
//! Options:
//!   --capture[=DIR]   turn each file through a full circle, save the preview at every step
//!                     and exit. DIR defaults to `captures`.
//!   --yaw-step=DEG    degrees between one capture and the next, default 45.
//!   --pitch=DEG       height above the horizon to capture from, default the viewer's own.
//!   --time=SECONDS    where on the timeline to capture, default the resting pose.
//!   --size=WxH        the window to open, which decides how large the captured images are.

mod app;
mod capture;
mod details;

use std::path::PathBuf;

use eframe::egui;

use crate::app::Nifty;

struct Args {
    roots: Vec<PathBuf>,
    files: Vec<PathBuf>,
    capture: Option<capture::Request>,
}

fn parse(raw: impl Iterator<Item = String>) -> Result<Args, String> {
    let (mut roots, mut files) = (Vec::new(), Vec::new());
    let mut request = capture::Request::default();
    let mut capturing = false;

    for arg in raw {
        let (flag, value) = match arg.split_once('=') {
            Some((flag, value)) => (flag, Some(value)),
            None => (arg.as_str(), None),
        };
        let degrees = |what: &str| match value {
            Some(value) => value
                .parse::<f32>()
                .map_err(|_| format!("{what} wants a number of degrees, not `{value}`")),
            None => Err(format!("{what} wants a value, as {what}=45")),
        };
        match flag {
            "--capture" => {
                capturing = true;
                if let Some(dir) = value {
                    request.dir = PathBuf::from(dir);
                }
            }
            "--yaw-step" => request.step = degrees("--yaw-step")?,
            "--pitch" => request.pitch = degrees("--pitch")?,
            "--time" => {
                let value = value.ok_or("--time wants a value, as --time=1.5")?;
                request.time = Some(
                    value
                        .parse::<f32>()
                        .map_err(|_| format!("--time wants seconds, not `{value}`"))?,
                );
            }
            "--size" => {
                let value = value.ok_or("--size wants a value, as --size=1280x900")?;
                let (width, height) = value
                    .split_once(['x', 'X'])
                    .ok_or_else(|| format!("--size wants WxH, not `{value}`"))?;
                let side = |side: &str| {
                    side.parse::<f32>()
                        .map_err(|_| format!("--size wants numbers, not `{side}`"))
                };
                request.size = [side(width)?, side(height)?];
            }
            _ if flag.starts_with("--") => return Err(format!("unknown option `{arg}`")),
            _ => {
                let path = PathBuf::from(&arg);
                match path.is_dir() {
                    true => roots.push(path),
                    false => files.push(path),
                }
            }
        }
    }

    if capturing && files.is_empty() {
        return Err("--capture needs a file to capture".into());
    }
    Ok(Args {
        roots,
        files,
        capture: capturing.then_some(request),
    })
}

fn main() -> eframe::Result {
    let args = match parse(std::env::args().skip(1)) {
        Ok(args) => args,
        Err(problem) => {
            eprintln!("nifty: {problem}");
            eprintln!("{}", USAGE);
            std::process::exit(2);
        }
    };

    let mut viewport = egui::ViewportBuilder::default();
    // a capture is only reproducible if the window it is taken from is the same size every run
    if let Some(request) = &args.capture {
        viewport = viewport.with_inner_size(request.size);
    }
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        depth_buffer: 32,
        multisampling: nif_wgpu::scene::MSAA_SAMPLES as u16,
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "nifty",
        options,
        Box::new(move |cc| {
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);

            let mut app = Nifty::new(cc);
            // roots first, so the scenes resolve textures and shaders as they are built
            for root in args.roots {
                app.add_root(root);
            }
            // every file named on the command line goes into one document, drawn together
            app.open_together(args.files);
            app.focus_first();
            if let Some(request) = args.capture {
                app.capture(request);
            }
            Ok(Box::new(app))
        }),
    )
}

const USAGE: &str = "usage: nifty [file.nif ...] [texture-dir ...] \
                     [--capture[=DIR]] [--yaw-step=DEG] [--pitch=DEG] [--size=WxH] \
                     [--time=SECONDS]";

#[cfg(test)]
mod tests {
    use super::*;

    fn parsed(args: &[&str]) -> Result<Args, String> {
        parse(args.iter().map(|a| a.to_string()))
    }

    #[test]
    fn a_bare_capture_takes_eight_shots() {
        let args = parsed(&["a.nif", "--capture"]).unwrap();
        let request = args.capture.unwrap();
        assert_eq!(request.yaws().len(), 8);
        assert_eq!(request.dir, PathBuf::from("captures"));
    }

    #[test]
    fn a_step_that_does_not_divide_the_circle_stops_short_of_it() {
        let args = parsed(&["a.nif", "--capture", "--yaw-step=100"]).unwrap();
        let yaws = args.capture.unwrap().yaws();
        assert_eq!(yaws, vec![0.0, 100.0, 200.0, 300.0]);
    }

    #[test]
    fn capture_wants_something_to_capture() {
        assert!(parsed(&["--capture"]).is_err());
    }

    #[test]
    fn an_unknown_option_is_not_a_file_name() {
        assert!(parsed(&["--yaw=45"]).is_err());
    }

    #[test]
    fn a_time_is_seconds_on_the_timeline() {
        let args = parsed(&["a.nif", "--capture", "--time=2.5"]).unwrap();
        assert_eq!(args.capture.unwrap().time, Some(2.5));
        assert!(parsed(&["a.nif", "--capture", "--time"]).is_err());
    }

    #[test]
    fn a_size_is_two_numbers() {
        let args = parsed(&["a.nif", "--capture", "--size=800x600"]).unwrap();
        assert_eq!(args.capture.unwrap().size, [800.0, 600.0]);
        assert!(parsed(&["a.nif", "--capture", "--size=800"]).is_err());
    }
}
