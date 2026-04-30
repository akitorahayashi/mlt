import os

import pytest

from src.marp_service import MarpService


@pytest.fixture
def marp_service(tmp_path):
    """
    Create a MarpService instance with a temporary output directory and
    a minimal test slides.md file.
    """
    output_dir = tmp_path / "output"
    output_dir.mkdir()

    slides_path = tmp_path / "slides.md"
    slides_path.write_text(
        "---\n"
        "marp: true\n"
        "theme: custom-theme\n"
        "paginate: true\n"
        "header: 'Test Deck'\n"
        "footer: '© 2025 Test'\n"
        "---\n\n"
        "# Test Slide\n\n"
        "- This is a minimal slide used only for tests.\n",
        encoding="utf-8",
    )

    service = MarpService(slides_path=str(slides_path), output_dir=str(output_dir))
    return service


@pytest.mark.parametrize(
    "output_type, generator_method_name, output_filename",
    [
        (MarpService.OutputType.PDF, "generate_pdf", "test.pdf"),
        (MarpService.OutputType.HTML, "generate_html", "test.html"),
        (MarpService.OutputType.PNG, "generate_png", "test.png"),
        (MarpService.OutputType.PPTX, "generate_pptx", "test.pptx"),
    ],
)
def test_marp_service_generation(
    marp_service, output_type, generator_method_name, output_filename
):
    """
    Tests that the MarpService can generate all supported output file types.
    """
    generator_method = getattr(marp_service, generator_method_name)

    output_path = generator_method(output_filename)

    assert os.path.exists(output_path)
    assert os.path.basename(output_path) == output_filename
    assert marp_service.output_dir in output_path
