#include <iostream>
#include <memory>
#include <string>
#include <poppler/cpp/poppler-document.h>
#include <poppler/cpp/poppler-page.h>
#include <poppler/cpp/poppler-image.h>
#include <poppler/cpp/poppler-page-renderer.h>
#include <cstdio>
#include <jpeglib.h>
#include <cstdlib>

void print_jpeg(poppler::image& img)
{
    struct jpeg_compress_struct cinfo;
    struct jpeg_error_mgr jerr;

    cinfo.err = jpeg_std_error(&jerr);
    jpeg_create_compress(&cinfo);

    unsigned char* output = nullptr;
    unsigned long output_size = 0;

    jpeg_mem_dest(&cinfo, &output, &output_size);

    cinfo.image_width = img.width();
    cinfo.image_height = img.height();
    cinfo.input_components = 3;
    cinfo.in_color_space = JCS_RGB;

    jpeg_set_defaults(&cinfo);
    jpeg_set_quality(&cinfo, 95, TRUE);

    jpeg_start_compress(&cinfo, TRUE);

    while (cinfo.next_scanline < cinfo.image_height) {
        JSAMPROW row = const_cast<JSAMPROW>(
            reinterpret_cast<const JSAMPLE*>(
                img.data() + cinfo.next_scanline * img.bytes_per_row()
            )
        );

        jpeg_write_scanlines(&cinfo, &row, 1);
    }

    jpeg_finish_compress(&cinfo);

    fwrite(output, 1, output_size, stdout);
    fflush(stdout);

    free(output);
    jpeg_destroy_compress(&cinfo);
}

int main(int argc, char* argv[])
{
    if (argc < 3) {
        std::cerr << "Missing number of required arguments.\n";
        return 1;
    }

    const std::string pdf_path = argv[1];
    const std::string pdf_size_type = argv[2];

    int WIDTH = 600;
    int HEIGHT = 900;

    // Load the PDF.
    std::unique_ptr<poppler::document> doc(
        poppler::document::load_from_file(pdf_path)
    );

    if (!doc) {
        std::cerr << "Failed to load PDF.\n";
        return 1;
    }

    if (doc->pages() < 1) {
        std::cerr << "PDF contains no pages.\n";
        return 1;
    }

    // Load the first page.
    std::unique_ptr<poppler::page> page(
        doc->create_page(0)
    );

    if (!page) {
        std::cerr << "Failed to load page.\n";
        return 1;
    }

    // Get the page dimensions in points (1/72 inch).
    const poppler::rectf page_rect = page->page_rect();

    const double page_width = page_rect.width();
    const double page_height = page_rect.height();

    // Calculate the DPI required for the desired dimensions.
    const double dpi_x = WIDTH * 72.0 / page_width;
    const double dpi_y = HEIGHT * 72.0 / page_height;

    // Create renderer.
    poppler::page_renderer renderer;

    if (!poppler::page_renderer::can_render()) {
        std::cerr << "Poppler rendering backend unavailable.\n";
        return 1;
    }

    renderer.set_image_format(poppler::image::format_rgb24);

    renderer.set_render_hint(poppler::page_renderer::antialiasing);
    renderer.set_render_hint(poppler::page_renderer::text_antialiasing);

    // Render the page.
    poppler::image img = renderer.render_page(
        page.get(),
        dpi_x,
        dpi_y
    );

    if (!img.is_valid()) {
        std::cerr << "Failed to render PDF page.\n";
        return 1;
    }

    print_jpeg(img);

    return 0;
}
