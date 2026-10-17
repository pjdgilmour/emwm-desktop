# SPDX-License-Identifier: GPL-3.0-only
"""Wallpaper preparation runs as the user; the helper accepts only bounded PNG data."""
import io
import re
from PIL import Image, ImageOps

MODES = {'zoom': 'Preencher (recortar)', 'stretch': 'Esticar (expandir)',
         'fit': 'Ajustar (sem recortar)', 'center': 'Centralizar', 'tile': 'Mosaico (tile)'}
MAX_BYTES = 64 * 1024 * 1024
MAX_PIXELS = 40_000_000
Image.MAX_IMAGE_PIXELS = MAX_PIXELS


def dimensions(width, height):
    width, height = int(width), int(height)
    if not (1 <= width <= 16384 and 1 <= height <= 16384) or width * height > MAX_PIXELS:
        raise ValueError('Resolução inválida ou imagem maior que 40 megapixels.')
    return width, height


def load_image(stream, png_only=False):
    try:
        with Image.open(stream) as original:
            dimensions(*original.size)
            if png_only and original.format != 'PNG':
                raise ValueError('O auxiliar aceita somente imagens PNG preparadas pela ferramenta.')
            original.load()
            return ImageOps.exif_transpose(original).convert('RGBA')
    except (OSError, Image.DecompressionBombError) as error:
        raise ValueError('Não foi possível ler a imagem: '+str(error)) from error


def encode(image):
    stream = io.BytesIO()
    image.info.clear()
    image.save(stream, format='PNG')
    data = stream.getvalue()
    if len(data) > MAX_BYTES:
        raise ValueError('Imagem preparada maior que 64 MiB.')
    return data


def normalize_png(data):
    if not data or len(data) > MAX_BYTES:
        raise ValueError('Tamanho de imagem inválido (máximo 64 MiB).')
    # Decode and re-encode, removing metadata and ancillary/untrusted content.
    return encode(load_image(io.BytesIO(data), png_only=True).convert('RGB'))


def compose(image, mode, width, height, color='#000000'):
    if mode not in MODES: raise ValueError('Modo de fundo inválido.')
    width, height = dimensions(width, height)
    if not re.fullmatch(r'#[0-9a-fA-F]{6}', color): raise ValueError('Cor de fundo inválida.')
    canvas = Image.new('RGB', (width, height), color)
    if mode == 'zoom': image = ImageOps.fit(image, (width, height), Image.Resampling.LANCZOS)
    elif mode == 'stretch': image = image.resize((width, height), Image.Resampling.LANCZOS)
    elif mode == 'fit': image = ImageOps.contain(image, (width, height), Image.Resampling.LANCZOS)
    if mode == 'tile':
        # Doubling avoids millions of paste calls for tiny patterns.
        canvas.paste(image, (0, 0), image)
        filled_width, filled_height = min(image.width, width), min(image.height, height)
        while filled_width < width:
            amount = min(filled_width, width-filled_width)
            canvas.paste(canvas.crop((0, 0, amount, filled_height)), (filled_width, 0))
            filled_width += amount
        while filled_height < height:
            amount = min(filled_height, height-filled_height)
            canvas.paste(canvas.crop((0, 0, width, amount)), (0, filled_height))
            filled_height += amount
    else:
        canvas.paste(image, ((width-image.width)//2, (height-image.height)//2), image)
    return canvas


def prepare(path, mode, width, height, color='#000000'):
    if mode not in MODES: raise ValueError('Modo de fundo inválido.')
    dimensions(width, height)
    # Open as the desktop user, never pass a user-supplied path to root.
    with open(path, 'rb') as stream:
        image = load_image(stream)
    if mode in ('zoom', 'stretch'):
        # Native modes adapt independently to every monitor at the login screen.
        image = compose(image, 'center', *image.size, color)
    else:
        image = compose(image, mode, width, height, color)
    return encode(image)
