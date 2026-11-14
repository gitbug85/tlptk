use printpdf::*;
use std::fs;

fn main() {
    let mut doc = PdfDocument::new("My first PDF");
    let font_bytes = include_bytes!("../ArchitectsDaughter-Regular.ttf");
    let mut font_warnings = Vec::new();
    let parsed_font = ParsedFont::from_bytes(font_bytes, 0, &mut font_warnings).unwrap();
    let font_id = doc.add_font(&parsed_font);
    let page1_contents = vec![
        Op::StartTextSection,
        Op::SetTextCursor {
            pos: Point::new(Mm(20.0), Mm(270.0)),
        },
        Op::SetFont {
            font: PdfFontHandle::External(font_id),
            size: Pt(24.0),
        },
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
