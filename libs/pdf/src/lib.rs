use printpdf::*;
use std::{fs, path::PathBuf, process::Command};

// This function will save PDFs by interpreting lexemes and saving at the designated path
pub async fn save_pdf(path: PathBuf, width: f32, height: f32) {
    let current_exe = std::env::current_exe().expect("Failed to find pdf executable.");

    let parent = current_exe
        .parent()
        .ok_or("Could not get executable parent directory")
        .expect("Failed to find parent of pdf executable.");

    let lex_path: PathBuf = parent.join("lex");

    let mut cmd = Command::new(lex_path);
    cmd.arg(path.to_string_lossy().to_string());

    let output = cmd
        .output()
        .expect("Could not get output from pdf executable.");

    let stdout = String::from_utf8(output.stdout).expect("Could not convert pdf output to String.");

    let result: Vec<String> =
        serde_json::from_str(&stdout).expect("Could not make Vec<String> from stdout");

    let mut doc = PdfDocument::new("tlptk generated pdf");
    let sitelen_wan_dir = parent
        .parent()
        .expect("Could not get parent of bin directory.")
        .join("sitelen_wan");

    let mut x_pos = 0.0;
    let mut y_pos = (height * 2.83464567) - 100.0;
    let mut ops = vec![];

    for lexeme in &result {
        if lexeme == "\n" {
            x_pos = 0.0;
            y_pos -= 100.0;
            continue;
        }

        let svg_path = sitelen_wan_dir.join(format!("{}.svg", lexeme));
        let svg_data = fs::read_to_string(svg_path).unwrap();
        let svg = Svg::parse(&svg_data, &mut Vec::new()).unwrap();
        let svg_id = doc.add_xobject(&svg);
        let op = Op::UseXobject {
            id: svg_id,
            transform: XObjectTransform {
                translate_x: Some(Pt(x_pos)),
                translate_y: Some(Pt(y_pos)),
                scale_x: Some(0.5),
                scale_y: Some(0.5),
                ..Default::default()
            },
        };
        ops.push(op);
        x_pos += 60.0;
    }

    let page = PdfPage::new(Mm(width), Mm(height), ops);

    let mut warnings = Vec::new();
    let new_path = path.with_extension("pdf");
    let pdf_bytes = doc
        .with_pages(vec![page])
        .save(&PdfSaveOptions::default(), &mut warnings);
    fs::write(new_path, pdf_bytes).unwrap();
}
