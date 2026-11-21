#include <iostream>
#include <memory>
#include <poppler-document.h>
#include <poppler-page.h>

int main()
{
    // Open a PDF document
    std::unique_ptr<poppler::document> doc(
        poppler::document::load_from_file("example.pdf")
    );

    // Check whether the PDF opened successfully
    if (!doc)
    {
        std::cerr << "Could not open PDF!" << std::endl;
        return 1;
    }

    // Get the number of pages
    int pages = doc->pages();

    std::cout << "Number of pages: " << pages << std::endl;

    // Extract text from the first page
    if (pages > 0)
    {
        std::unique_ptr<poppler::page> page(doc->create_page(0));

        if (page)
        {
            poppler::byte_array text = page->text().to_utf8();

            std::cout << "PDF text:\n";

            std::cout.write(
                reinterpret_cast<const char*>(text.data()),
                text.size()
            );
        }
    }

    return 0;
}
