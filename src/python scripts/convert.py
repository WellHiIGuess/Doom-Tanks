from PIL import Image
  
# Import an image from directory:
input_image = Image.open('StartScreen.png')
  
# Extracting pixel map:
pixel_map = input_image.load()

print('[', end='')

for x in range(input_image.width):
    print('[', end='')
    for y in range(input_image.height):
        if y != input_image.width - 1:
            print(f'Color::new({pixel_map[x, y][0]}, {pixel_map[x, y][1]}, {pixel_map[x, y][2]}, {pixel_map[x, y][3]}), ', end='')
        else:
            print(f'Color::new({pixel_map[x, y][0]}, {pixel_map[x, y][1]}, {pixel_map[x, y][2]}, {pixel_map[x, y][3]})', end='')

    print('], ', end='')
