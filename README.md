# trill
Small, compact music player based on rodio.

## Usage
Run the program with --help to see the available options.

To just play a sound file:

```trill -p <path>```

## Manipulating settings

Settings are defined through environment variables.

Supported settings listed below:

- volume, controlled through the VOLUME environment variable. MUST BE FLOATING POINT BETWEEN 0.0 - 1.0
  - 0.0 = 0% volume, 1.0 = 0% volume.

## Currently supported formats.
- MP3
- WAV
- FLAC
- OGG
