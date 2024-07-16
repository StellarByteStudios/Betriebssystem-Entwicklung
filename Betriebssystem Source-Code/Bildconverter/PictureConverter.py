import argparse
from PIL import Image

def image_to_rust_bitmap(image_path, output_path):
    # Open the image
    with Image.open(image_path) as img:
        # Ensure the image is in RGBA format
        img = img.convert('RGBA')
        width, height = img.size
        bpp = 4  # Bytes per pixel in RGBA

        # Extract pixel data
        pixel_data = img.tobytes()

        # Convert pixel data to the required format
        pixel_data_str = ''.join(f'\\x{b:02x}' for b in pixel_data)

        # Write the Rust file
        with open(output_path, 'w') as f:
            f.write(f'pub const WIDTH:u32  = {width};\n')
            f.write(f'pub const HEIGHT:u32 = {height};\n')
            f.write(f'pub const BPP:u32    = {bpp};\n\n')
            f.write(f'pub const DATA: &[u8;{len(pixel_data)}] = b"{pixel_data_str}";\n')


def main():
    # Namen des Inputfiles lesen
    parser = argparse.ArgumentParser(description="Convert a C-style bitmap to a Rust array")
    parser.add_argument('input_file', help='The input C file containing the bitmap')
    args = parser.parse_args()

    input_file = args.input_file
    output_file = input_file.rsplit('.', 1)[0] + '.rs'

    image_to_rust_bitmap(input_file, output_file)



if __name__ == "__main__":
    main()
