import re
import sys
from pathlib import Path

def parse_song_function(file_contents):
    notes = []
    play_pattern = re.compile(r"play\(([\d.]+),\s*(\d+)\)")
    delay_pattern = re.compile(r"delay\((\d+)\)")

    for line in file_contents.splitlines():
        line = line.strip()
        if match := play_pattern.match(line):
            freq = int(float(match.group(1)))
            dur = int(match.group(2))
            notes.append((freq, dur))
        elif match := delay_pattern.match(line):
            dur = int(match.group(1))
            notes.append((0, dur))
    return notes

def generate_rust_slice(notes, name):
    rust = f"pub const {name.upper()}: &[Note] = &[\n"
    for freq, dur in notes:
        rust += f"    Note {{ frequency: {freq}, duration: {dur} }},\n"
    rust += "];\n"
    return rust

def main():
    if len(sys.argv) < 2:
        print("⚠️  Bitte gib einen Songnamen als erstes Argument an, z.B.:")
        print("    python converter.py ImperialMarch")
        sys.exit(1)

    song_name = sys.argv[1]
    base_path = Path("./Functionconverter")
    input_path = base_path / f"{song_name}Function.txt"
    output_path = base_path / f"{song_name}Slice.txt"

    if not input_path.exists():
        print(f"❌ Datei nicht gefunden: {input_path}")
        sys.exit(1)

    with input_path.open("r", encoding="utf-8") as f:
        content = f.read()

    notes = parse_song_function(content)
    rust_output = generate_rust_slice(notes, song_name)

    with output_path.open("w", encoding="utf-8") as f:
        f.write(rust_output)

    print(f"✅ Konvertierung abgeschlossen: {output_path}")

if __name__ == "__main__":
    main()
