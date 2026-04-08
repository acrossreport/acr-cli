// ACR CLI — Across Report Renderer
// Copyright © 2002-2026 Across Systems Corporation
// Licensed under the Apache License 2.0
//
// Patent pending: ARC-001 (Japan, 2026)
// The intermediate drawing model and device-independent rendering architecture
// described in this software are subject to patent protection.

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // activate — license registration via GitHub account
    if args.len() == 2 && args[1] == "activate" {
        // Opens https://licensvr.acrossreport.com/
        todo!("license activation")
    }

    // --version / -v
    if args.len() == 2 && (args[1] == "--version" || args[1] == "-v") {
        println!("acr-cli {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // usage: acr_cli <template.json> <data.json> [--D]
    if args.len() < 3 {
        eprintln!("usage: acr_cli <template.json> <data.json> [--D]");
        std::process::exit(1);
    }

    let template_path = &args[1];
    let data_path     = &args[2];
    let debug_mode    = args.iter().any(|a| a == "--D");

    // License check
    // todo!("check_license")

    // Monthly quota check (free plan: up to 3 reports/month)
    // todo!("check_and_increment_quota")

    // Load template and data (UTF-8 / Shift_JIS auto-detection)
    let _template_text = read_text_auto(template_path)?;
    let _data_text     = read_text_auto(data_path)?;

    // Build intermediate render commands via acr_core
    // todo!("acr_core::build_report")

    // Render to PDF via acr_renderer_skia
    // todo!("acr_renderer_skia::render_pdf_pages")

    // Render to PNG (ZIP package) via acr_renderer_skia
    // todo!("acr_renderer_skia::render_png_page_to_memory")

    if debug_mode {
        eprintln!("debug mode on");
    }

    Ok(())
}

/// Read text file with automatic encoding detection (UTF-8 / Shift_JIS)
fn read_text_auto(path: &str) -> anyhow::Result<String> {
    let bytes = std::fs::read(path)?;

    // UTF-8 BOM
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Ok(String::from_utf8(bytes[3..].to_vec())?);
    }

    // UTF-8
    if std::str::from_utf8(&bytes).is_ok() {
        return Ok(String::from_utf8(bytes)?);
    }

    // Shift_JIS fallback
    let (text, _, _) = encoding_rs::SHIFT_JIS.decode(&bytes);
    Ok(text.into_owned())
}
