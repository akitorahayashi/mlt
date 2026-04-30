from __future__ import annotations

import argparse
import sys
from pathlib import Path

from src.conversion.marp_exporter import MarpExporter, OutputFormat
from src.deck_management.deck_locator import resolve_deck_paths, resolve_repo_root


def main() -> None:
    parser = _build_parser()
    args = parser.parse_args()

    try:
        if args.command == "convert":
            _handle_convert(args)
        elif args.command == "export":
            _handle_export(args)
        else:
            parser.error("A command is required")
    except Exception as error:  # noqa: BLE001
        print(f"Error: {error}", file=sys.stderr)
        raise SystemExit(1) from error


def _build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Manage deck exports and direct Marp conversions."
    )
    subparsers = parser.add_subparsers(dest="command")

    convert_parser = subparsers.add_parser(
        "convert",
        help="Convert a completed slides.md file without deck management",
    )
    convert_parser.add_argument("slides_path", help="Path to the completed slides.md")
    convert_parser.add_argument(
        "--to",
        required=True,
        choices=[*sorted(output.value for output in OutputFormat), "all"],
        help="Output format to generate",
    )
    convert_parser.add_argument(
        "--output-dir",
        required=True,
        help="Directory for generated files",
    )
    convert_parser.add_argument(
        "--output-basename",
        default="slides",
        help="Base filename used for generated files",
    )
    convert_parser.add_argument(
        "--theme-file",
        help="Optional CSS file passed to Marp as the export theme",
    )

    export_parser = subparsers.add_parser(
        "export",
        help="Resolve a deck directory and export its slides.md",
    )
    export_parser.add_argument(
        "deck_reference",
        help="Deck directory path or lower-kebab-case deck id",
    )
    export_parser.add_argument(
        "--to",
        required=True,
        choices=[*sorted(output.value for output in OutputFormat), "all"],
        help="Output format to generate",
    )

    return parser


def _handle_convert(args: argparse.Namespace) -> None:
    theme_path = Path(args.theme_file).resolve() if args.theme_file else None
    exporter = MarpExporter(
        slides_path=Path(args.slides_path),
        output_dir=Path(args.output_dir),
        theme_path=theme_path,
    )
    _run_export(exporter, args.to, args.output_basename)


def _handle_export(args: argparse.Namespace) -> None:
    deck_paths = resolve_deck_paths(
        args.deck_reference,
        repo_root=resolve_repo_root(),
    )
    exporter = MarpExporter(
        slides_path=deck_paths.slides_path,
        output_dir=deck_paths.output_dir,
        theme_path=deck_paths.theme_path,
    )
    _run_export(exporter, args.to, deck_paths.definition.output_basename)


def _run_export(
    exporter: MarpExporter,
    output_name: str,
    output_basename: str,
) -> None:
    if output_name == "all":
        exported_paths = exporter.export_all(output_basename)
        for output_format, output_path in exported_paths.items():
            print(f"{output_format.value}: {output_path}")
        return

    output_path = exporter.export(OutputFormat(output_name), output_basename)
    print(output_path)


if __name__ == "__main__":
    main()
