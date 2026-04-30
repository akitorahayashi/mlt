import os
import sys

from marp_service import MarpService


def main() -> None:
    """
    Minimal CLI around MarpService for ad-hoc export.

    This script is optional: the primary interface for exports is the Makefile
    (`make pdf`, `make pptx`, `make html`) using `output/slides.md`.
    """
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    slides_path = os.path.join(project_root, "output", "slides.md")
    output_dir = os.path.join(project_root, "output")
    theme_path = os.path.join("src", "theme.css")

    # Default output type is HTML, can be overridden by the first CLI argument.
    output_type_name = sys.argv[1] if len(sys.argv) > 1 else "html"

    try:
        output_type = MarpService.OutputType(output_type_name.lower())
    except ValueError:
        valid = ", ".join(t.value for t in MarpService.OutputType)
        print(f"Unknown output type: {output_type_name}")
        print("Usage: python src/main.py [pdf|html|png|pptx]")
        print(f"Valid types: {valid}")
        sys.exit(1)

    output_filename = f"slides.{output_type.value}"

    marp_service = MarpService(slides_path, output_dir)
    generation_methods = {
        MarpService.OutputType.PDF: marp_service.generate_pdf,
        MarpService.OutputType.HTML: marp_service.generate_html,
        MarpService.OutputType.PNG: marp_service.generate_png,
        MarpService.OutputType.PPTX: marp_service.generate_pptx,
    }

    method_to_call = generation_methods.get(output_type)
    if not method_to_call:
        print(f"Error: Unsupported output type '{output_type.value}'.")
        sys.exit(1)

    print(f"Generating {output_type.value} file from {slides_path}...")
    method_to_call(output_filename, theme=theme_path)


if __name__ == "__main__":
    main()
