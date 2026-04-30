from __future__ import annotations

import shutil
import uuid
from pathlib import Path

import pytest

from src.conversion.marp_exporter import MarpExporter, OutputFormat


@pytest.fixture
def marp_workspace() -> Path:
    workspace = Path(".tmp/tests") / f"marp-exporter-{uuid.uuid4().hex}"
    workspace.mkdir(parents=True, exist_ok=False)

    slides_path = workspace / "slides.md"
    slides_path.write_text(
        "---\n"
        "marp: true\n"
        "theme: marp-pj-default\n"
        "paginate: true\n"
        "---\n\n"
        "# Test Slide\n\n"
        "- This is a minimal slide used only for tests.\n",
        encoding="utf-8",
    )

    theme_path = workspace / "test-theme.css"
    theme_path.write_text(
        "/* @theme marp-pj-default */\n@import 'default';\n",
        encoding="utf-8",
    )

    try:
        yield workspace
    finally:
        shutil.rmtree(workspace, ignore_errors=True)


@pytest.mark.parametrize("output_format", list(OutputFormat))
def test_marp_exporter_generates_output_files(
    marp_workspace: Path, output_format: OutputFormat
):
    exporter = MarpExporter(
        slides_path=marp_workspace / "slides.md",
        output_dir=marp_workspace / "output",
        theme_path=marp_workspace / "test-theme.css",
    )

    output_path = exporter.export(output_format, "slides")

    assert output_path.exists()
    assert output_path.suffix == f".{output_format.value}"
