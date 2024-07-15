import re
import argparse

def extract_info(c_code):
    print(c_code[0:500])
    width = re.search(r'width\s*=\s*(\d+)', c_code).group(1)
    height = re.search(r'height\s*=\s*(\d+)', c_code).group(1)
    bytes_per_pixel = re.search(r'bytes_per_pixel\s*=\s*(\d+)', c_code).group(1)
    pixel_data = re.search(r'pixel_data\s*=\s*{(.*?)};', c_code, re.S).group(1).split(',')
    pixel_data = [int(x.strip(), 0) for x in pixel_data]  # Convert hex to int
    return width, height, bytes_per_pixel, pixel_data

def write_rust_file(output_file, width, height, bytes_per_pixel, pixel_data):
    with open(output_file, 'w') as f:
        f.write(f'pub const WIDTH:u32  = {width};\n')
        f.write(f'pub const HEIGHT:u32 = {height};\n')
        f.write(f'pub const BPP:u32    = {bytes_per_pixel};\n\n')
        f.write(f'pub const DATA: &[u8;{len(pixel_data)}] = b"')
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

if __name__ == "__main__":
    main()