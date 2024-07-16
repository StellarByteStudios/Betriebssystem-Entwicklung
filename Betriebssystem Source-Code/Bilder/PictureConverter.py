import re
import argparse
import subprocess

"""
def extract_info(c_code):
    # Regex to extract the struct declaration which contains width, height, and bytes_per_pixel
    struct_declaration = re.search(r'{\s*(\d+),\s*(\d+),\s*(\d+),', c_code, re.S)
    
    if not struct_declaration:
        raise ValueError("Could not find the struct declaration in the provided C code.")
    
    width, height, bytes_per_pixel = struct_declaration.groups()[:3]

    print(f"width: {width}\nheight: {height}\nbytes_per_pixel: {bytes_per_pixel}\n")

    # Improved regex to extract all pixel_data lines within the struct declaration
    pixel_data_matches = re.findall(r'"(.*?)"', c_code, re.S)

    #print(type(pixel_data_matches))
    
    if not pixel_data_matches:
        raise ValueError("Could not find the pixel data array in the provided C code.")
    
    # Concatenate all the matched pixel data strings
    pixel_data_string = ''.join(pixel_data_matches)

    #print(pixel_data_string)

    # Extract the pixel data from the string, handling escape sequences
    pixel_data = re.findall(r'\\([0-9A-Fa-f]{3})', pixel_data_string)
    #print(pixel_data)
    pixel_data = [int(byte, 8) for byte in pixel_data]

    #print(pixel_data)
    
    return width, height, bytes_per_pixel, pixel_data

def write_rust_file(output_file, width, height, bytes_per_pixel, pixel_data):
    with open(output_file, 'w') as f:
        f.write(f'pub const WIDTH:u32  = {width};\n')
        f.write(f'pub const HEIGHT:u32 = {height};\n')
        f.write(f'pub const BPP:u32    = {bytes_per_pixel};\n\n')
        f.write(f'pub const DATA: &[u8;{len(pixel_data)}] = b"')

        # ================= Hier muss noch zu richtigem Hex umgewandelt werden
        for byte in pixel_data:
            f.write(f'\\x{byte:02x}')
        f.write('";\n')

def main():
    parser = argparse.ArgumentParser(description="Convert a C-style bitmap to a Rust array")
    parser.add_argument('input_file', help='The input C file containing the bitmap')
    args = parser.parse_args()

    input_file = args.input_file
    output_file = input_file.rsplit('.', 1)[0] + '.rs'

    # Read the C input file
    with open(input_file, 'r') as f:
        c_code = f.read()

    # Extract information from the C code
    width, height, bytes_per_pixel, pixel_data = extract_info(c_code)

    # Write the extracted information to the Rust output file
    write_rust_file(output_file, width, height, bytes_per_pixel, pixel_data)
"""

def main():

    # Namen des Inputfiles lesen
    parser = argparse.ArgumentParser(description="Convert a C-style bitmap to a Rust array")
    parser.add_argument('input_file', help='The input C file containing the bitmap')
    args = parser.parse_args()

    input_file = args.input_file
    output_file = input_file.rsplit('.', 1)[0] + '.rs'

    # bashcomand zusammensetzen
    # Compilieren
    command = f"gcc -DINPUT_FILE='\"{input_file}\"' -o Converter bitmapConverter.c"
    #command = f"gcc -o Converter bitmapConverter.c"
    # Ausführen
    command = f"{command} && ./Converter"

    print(command)

    # bash command ausführen
    # Prozess erzeugen
    process = bash_command(command, ignoreStdout=False)
    # Prozess starten
    process.communicate()

    print(f"Bitmap is now in Rust style in the file {output_file}")







# Kapselung des Bashcommands, der das mit der Shell regelt
def bash_command(cmd, ignoreStdout = True):
    # stdout wird weggeworfen
    if ignoreStdout:    
        return subprocess.Popen(cmd, shell=True, executable='/bin/bash', stdout=subprocess.DEVNULL)
    # Stdout vom Skript in Pythonkonsole
    return subprocess.Popen(cmd, shell=True, executable='/bin/bash')





if __name__ == "__main__":
    main()
