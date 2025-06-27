use rayon::prelude::*;
use std::io::Write;
use terminal_size::{Width, terminal_size};

const N: usize = 100000; //Size of Simulation

fn main() {
    let no_render = std::env::args().any(|arg| arg == "--bench");
    let multi_threaded = std::env::args().any(|arg| arg == "--m");
    print!("\x1B[2J\x1B[H");
    let dz: f64 = 0.5; //Spatial step-size
    let fps: u64 = 60; //Animation FPS, lower if laggy

    let pi: f64 = std::f64::consts::PI;
    let u0: f64 = 4.0 * pi * 10f64.powf(-7.0);
    let e0: f64 = 8.854187817 * 10f64.powf(-12.0);
    let c0 = 1.0 / ((e0 * u0).sqrt());
    let z0: f64 = N as f64 / 2.0;
    let mut hy: Vec<f64> = vec![0f64; N - 1];
    let dt: f64 = dz / (2.0 * c0);

    let ce = dt / (e0 * dz);
    let ch = dt / (u0 * dz);

    let mut ex: Vec<f64> = (0..N)
        .map(|i| {
            let z = i as f64;
            (-((z - z0).powi(2)) / (2.0 * (N as f64 / 10.0).powi(2))).exp()
        })
        .collect(); //Initial magnetic field condition, pulse.

    let mut t = 0;
    let width = terminal_size()
        .map(|(Width(w), _)| (w as usize).saturating_sub(12))
        .unwrap_or(100);
    let height = terminal_size()
        .map(|(_, terminal_size::Height(h))| (h as usize).saturating_sub(6) / 2)
        .unwrap_or(20);

    print!("\x1B[?25l");
    std::io::stdout().flush().unwrap();
    let mut draw = std::time::Instant::now();
    let start_time = std::time::Instant::now();
    if multi_threaded {
        loop {
            t += 1;
            if t > N * 8 {
                break;
            }

            update_ex(&mut hy, &ex, ce);
            update_hy(&mut ex, &hy, ch);
            if no_render && t % 100 == 0 {
                println!("{}/{}", t, N * 8);
            }

            if !no_render && draw.elapsed() >= std::time::Duration::from_millis(1000 / fps) {
                print!("\x1B[H");
                render_waveform(&ex, width, height, "Ex", "\x1b[34m");
                render_waveform(&hy, width, height, "Hy", "\x1b[31m");
                draw = std::time::Instant::now()
            }
        }
    } else {
        loop {
            t += 1;
            if t > N * 8 {
                break;
            }

            for i in 1..N - 1 {
                ex[i] = ex[i] + ce * (hy[i] - hy[i - 1]);
            }

            for i in 2..N - 1 {
                hy[i] = hy[i] + ch * (ex[i + 1] - ex[i]);
            }
            if no_render && t % 100 == 0 {
                println!("{}/{}", t, N * 8);
            }

            if !no_render && draw.elapsed() >= std::time::Duration::from_millis(1000 / fps) {
                print!("\x1B[H");
                render_waveform(&ex, width, height, "Ex", "\x1b[34m");
                render_waveform(&hy, width, height, "Hy", "\x1b[31m");
                draw = std::time::Instant::now()
            }
        }
    }
    let elapsed = start_time.elapsed().as_secs_f64();
    println!("\nSimulation time: {:.4} seconds", elapsed);
    print!("\x1B[?25h");
    std::io::stdout().flush().unwrap();
}

fn update_hy(hy: &mut [f64], ex: &[f64], ch: f64) {
    hy.par_iter_mut().enumerate().for_each(|(i, hy_i)| {
        if i + 1 < ex.len() {
            *hy_i += ch * (ex[i + 1] - ex[i]);
        }
    });
}

fn update_ex(ex: &mut [f64], hy: &[f64], ce: f64) {
    ex.par_iter_mut().enumerate().for_each(|(i, ex_i)| {
        if i > 0 && i < hy.len() {
            *ex_i += ce * (hy[i] - hy[i - 1]);
        }
    });
}
//
//
//
//
//
//
//
//
//
//
//

fn render_waveform(data: &[f64], width: usize, height: usize, label: &str, color_code: &str) {
    use std::io::{self, Write};

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let len = data.len();
    let full_height = height * 2;

    let min_val = data.iter().copied().fold(f64::INFINITY, f64::min);
    let max_val = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    let zero_row = if (max_val - min_val).abs() < 1e-12 {
        full_height / 2
    } else {
        ((0.0 - min_val) / (max_val - min_val) * (full_height - 1) as f64)
            .round()
            .clamp(0.0, (full_height - 1) as f64) as usize
    };

    let sampled: Vec<f64> = (0..width)
        .map(|col| {
            let idx_f = col as f64 * (len - 1) as f64 / (width - 1).max(1) as f64;
            let idx = idx_f.round() as usize;
            data[idx.min(len - 1)]
        })
        .collect();

    let mut buffer = String::new();

    buffer.push_str(&format!(
        "  {:>6.2} +{}\n",
        max_val,
        "─".repeat(width.saturating_sub(9))
    ));

    for row in (0..height).rev() {
        let upper = row * 2 + 1;
        let lower = row * 2;

        let prefix = if upper == zero_row || lower == zero_row {
            format!("{:<6} │ ", label)
        } else {
            "       │ ".to_string()
        };
        buffer.push_str(&prefix);

        for &val in &sampled {
            let scaled = ((val - min_val) / (max_val - min_val + 1e-12) * (full_height - 1) as f64)
                .round()
                .clamp(0.0, (full_height - 1) as f64) as usize;

            let ch = match (scaled == upper, scaled == lower) {
                (true, true) => "█",
                (true, false) => "▀",
                (false, true) => "▄",
                _ => {
                    if upper == zero_row || lower == zero_row {
                        "\x1b[37m─\x1b[0m"
                    } else {
                        " "
                    }
                }
            };
            buffer.push_str(&format!("{}{}\x1b[0m", color_code, ch));
        }

        buffer.push_str("\x1b[K\n");
    }

    buffer.push_str(&format!(
        "  {:>6.2} +{}\n",
        min_val,
        "─".repeat(width.saturating_sub(9))
    ));

    write!(out, "\x1b[?25l{}\x1b[?25h", buffer).unwrap();
}
