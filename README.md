
# DDS-to-img

Rust CLI to recursively convert a directory of DDS files to other image formats.

## Usage
1. [ffmpeg](https://ffmpeg.org) must be installed and in your [`path`](https://learn.microsoft.com/en-us/previous-versions/office/developer/sharepoint-2010/ee537574(v=office.14)#to-add-a-path-to-the-path-environment-variable).
2. [texconv](https://github.com/Microsoft/DirectXTex/wiki/Texconv) must be installed and in your [`path`](https://learn.microsoft.com/en-us/previous-versions/office/developer/sharepoint-2010/ee537574(v=office.14)#to-add-a-path-to-the-path-environment-variable). 
3. Run [`dds-to-img.exe`](https://github.com/ronvoluted/dds-to-img/releases/latest) on a directory of `*.dds` files
```
dds-to-img <directory> [{png|webp}]
```
**Example:**
```
dds-to-image textures/ui webp
```
The directory structure will be maintained and output to a new sibling folder named `<directory>_<image format>`

### Arguments
- `directory` (required): Folder to look for `*.dds` files in.
- `{png|webp}` (optional): Image format to convert to. Defaults to `png`.
