use printpdf::*;
use std::fs;

enum TextElement {
    Text(String),
    Glyph { content: String },
}

// let elements = vec![
//     TextElement::Text("Hello ".to_string()),
//     TextElement::Svg {
//         path: "pona.svg".to_string(),
//         width: 12.0,
//         height: 12.0,
//     },
//     TextElement::Text(" World".to_string()),
// ];

// let svg = Svg::parse(&svg, &mut warnings).unwrap();
// let xobject_id = doc.add_xobject(&svg);
//
// let operations = vec![
//     Op::UseXobject {
//         id: xobject_id,
//         transform: XObjectTransform {
//             translate_x: Some(Mm(20.0).into()), <- This stuff is just x and y
//             translate_y: Some(Mm(200.0).into()),
//             scale_x: Some(1.0), <- This scales it
//             scale_y: Some(1.0),
//             ..Default::default()
//         },
//     },
// ];

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
