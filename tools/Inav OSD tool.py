from PIL import Image
import os
import re
import argparse
import sys

# Run `python -m pip install argparse pillow` to install above used libraries

# This is used to convert INAV fonts to OSD files
# https://github.com/iNavFlight/inav-configurator/tree/master/resources/osd

def get_image_number(filename):
    match = re.findall(r"\d+", filename)
    if match:
        return list(map(int, match))
    return None


def stack_images(directory, output_path, image_width, image_height):
    images = []
    max_height = 0
    max_width = 0

    # Open and resize images
    for filename in os.listdir(directory):
        if filename.endswith(".png") or filename.endswith(".jpg"):
            img = Image.open(os.path.join(directory, filename))
            img = img.convert("RGBA")  # Convert image to RGBA mode

            image_numbers = get_image_number(filename)
            if image_numbers is None or len(image_numbers) == 1:
                img = img.resize((image_width, image_height))
                images.append([img])
                max_height += img.height
                max_width = max(max_width, img.width)
            else:
                start_num, end_num = image_numbers
                num_images = end_num - start_num + 1
                num_rows = (img.width - 1) // image_width + 1  # Calculate number of rows
                for i in range(num_images):
                    row_idx = i // num_rows  # Row index for positioning
                    col_idx = i % num_rows  # Column index for positioning
                    sub_img = img.crop(
                        (
                            image_width * col_idx,
                            image_height * row_idx,
                            image_width * (col_idx + 1),
                            image_height * (row_idx + 1),
                        )
                    )
                    sub_img = sub_img.resize((image_width, image_height))
                    images.append([sub_img])
                    max_height += sub_img.height
                    max_width = max(max_width, sub_img.width)

    stacked_image = Image.new("RGBA", (max_width, max_height), (0, 0, 0, 0))  # Transparent background

    # Stack images vertically
    y_offset = 0
    x_offset = 0
    for img_group in images:
        img = img_group[0]
        stacked_image.paste(img, (x_offset, y_offset), mask=img)
        x_offset += img.width
        if x_offset >= stacked_image.width:
            x_offset = 0
            y_offset += img.height

    stacked_image.save(output_path)
    print(f"Stacked image saved as {output_path}")


def main():
    # Parse command-line arguments
    parser = argparse.ArgumentParser(description="INAV OSD font generator")

    required_group = parser.add_argument_group("required arguments")
    required_group.add_argument("-d", "--directory", type=str, help="Path to the directory containing the images")
    required_group.add_argument("-o", "--output_path", type=str, help="Path to save the stacked image")
    required_group.add_argument("-w", "--image_width", type=int, help="Width of each image")

    required_group.add_argument("-he", "--height", type=int, help="Height of each image")

    args = parser.parse_args()

    if not all(vars(args).values()):
        parser.print_help()
        sys.exit(0)

    if args.output_path and not args.output_path.endswith(".png"):
        args.output_path = os.path.splitext(args.output_path)[0] + ".png"

    # Call stack_images with provided arguments
    stack_images(args.directory, args.output_path, args.image_width, args.height)


if __name__ == "__main__":
    main()
