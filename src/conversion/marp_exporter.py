from __future__ import annotations

import subprocess
from enum import Enum
from pathlib import Path


class OutputFormat(Enum):
    PDF = "pdf"
    HTML = "html"
    PNG = "png"
    PPTX = "pptx"


class MarpExporter:
    def __init__(
        self,
        slides_path: Path,
        output_dir: Path,
        theme_path: Path | None = None,
    ) -> None:
        self.slides_path = slides_path.resolve()
        self.output_dir = output_dir.resolve()
        self.theme_path = theme_path.resolve() if theme_path else None

    def export(self, output_format: OutputFormat, output_basename: str) -> Path:
        if not self.slides_path.exists():
            raise FileNotFoundError(
                f"Slides markdown was not found: {self.slides_path}"
            )
        if self.theme_path and not self.theme_path.exists():
            raise FileNotFoundError(f"Theme CSS was not found: {self.theme_path}")

        self.output_dir.mkdir(parents=True, exist_ok=True)
        output_path = self.output_dir / f"{output_basename}.{output_format.value}"

        command = [
            "marp",
            str(self.slides_path),
            "--allow-local-files",
            "-o",
            str(output_path),
        ]
        if self.theme_path:
            command.extend(["--theme", str(self.theme_path)])

        completed = subprocess.run(
            command,
            check=True,
            capture_output=True,
            text=True,
        )
        if completed.stdout:
            print(completed.stdout, end="")
        return output_path

    def export_all(self, output_basename: str) -> dict[OutputFormat, Path]:
        return {
            output_format: self.export(output_format, output_basename)
            for output_format in OutputFormat
        }
