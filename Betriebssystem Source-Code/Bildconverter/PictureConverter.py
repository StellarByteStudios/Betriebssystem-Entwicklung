import argparse
import os
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
    # Read the name of the input folder
    parser = argparse.ArgumentParser(description="Convert all images in a folder to Rust arrays")
    parser.add_argument('input_folder', help='The input folder containing the images')
    args = parser.parse_args()

    input_folder = args.input_folder

    # List all files in the folder
    for file_name in os.listdir(input_folder):
        # Full path to the image file
        image_path = os.path.join(input_folder, file_name)

        # Check if the file is an image (by extension)
        if file_name.lower().endswith(('.png', '.jpg', '.jpeg', '.bmp', '.gif')):
            # Define the output Rust file path
            output_file = os.path.join(input_folder, file_name.rsplit('.', 1)[0] + '.rs')
            # Convert the image to a Rust Bitmap
            image_to_rust_bitmap(image_path, output_file)



if __name__ == "__main__":
    main()
