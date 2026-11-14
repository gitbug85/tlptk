use printpdf::*;
use std::fs;

fn main() {
    let mut doc = PdfDocument::new("My first PDF");
    let font = BuiltinFont::Helvetica;
    let page1_contents = vec![
        Op::StartTextSection,
        Op::SetTextCursor {
            pos: Point::new(Mm(20.0), Mm(270.0)),
        },
        Op::SetFont {
            font: PdfFontHandle::Builtin(BuiltinFont::Helvetica),
            size: Pt(24.0),
        },
        // Write the text
        Op::ShowText {
            items: vec![TextItem::Text("Hello, world!".to_string())],
        },
        Op::EndTextSection,
    ];
    let page1 = PdfPage::new(Mm(210.0), Mm(297.0), page1_contents);
    let mut warnings = Vec::new();
    let pdf_bytes: Vec<u8> = doc
        .with_pages(vec![page1])
        .save(&PdfSaveOptions::default(), &mut warnings);
    fs::write("my_first_pdf.pdf", pdf_bytes).expect("Failed to write PDF file");
}
