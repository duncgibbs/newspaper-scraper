from pathlib import Path
from pdf2image import convert_from_path
import os
import tempfile
import pytesseract

#pdfs = list(Path("./random_newspapers").rglob("*.[pP][dD][fF]"))
pdfs = list(Path("./random_newspapers").rglob("DOK_1919_12_18.[pP][dD][fF]"))
for newspaper in pdfs:
    with tempfile.TemporaryDirectory() as path:
        page_images = convert_from_path(newspaper, output_folder=path)
        for page in page_images:
            year = newspaper.parts[1]
            month = newspaper.parts[2]
            day = newspaper.parts[3].split('_')[-1].split('.')[0]

            ocr_filename = f'./ocr/{year}/{month}/{day}.json'

            os.makedirs

            print(page_images.index(page))
            #print(pytesseract.image_to_string(page))