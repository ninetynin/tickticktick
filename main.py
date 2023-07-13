from PIL import Image, ImageDraw, ImageFont
from datetime import datetime
from PyWallpaper import change_wallpaper
import ctypes
import os
import time

path = r"./temp/output.jpg"
####!todo make the path dynamic

def main():
    width = 1920
    height = 1080
    background_color = (0, 0, 0) 
    font_path = "./fonts/static/Montserrat-Bold.ttf"
    font_size = 150
    font_color = (255, 255, 255)  
    SPI_SETDESKWALLPAPER = 20
    while True:
        image = Image.new("RGB", (width, height), background_color)
        font = ImageFont.truetype(font_path, font_size)
        current_time = datetime.now().strftime("%H:%M:%S")
        text_width, text_height = font.getsize(current_time)

        x = (width - text_width) // 2
        y = (height - text_height) // 2

        draw = ImageDraw.Draw(image)
        draw.text((x, y), current_time, font=font, fill=font_color)
        image.save(path)
        ctypes.windll.user32.SystemParametersInfoA(SPI_SETDESKWALLPAPER, 0, os.path.abspath(path).encode(), 3)
        # time.sleep(1/10)


if __name__ == "__main__":
    main()
