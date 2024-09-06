# license-generator [![Test](https://github.com/azu/license-generator/actions/workflows/test.yml/badge.svg)](https://github.com/azu/license-generator/actions/workflows/test.yml)

A Command line tool that generates `LICENSE` file.

## Installation

Install with [Cargo](https://crates.io/):

    cargo install license-generator

## Usage

    $ license-generator --author <name> [LICENSE_TYPE]

    [LICENSE_TYPE]:
    - AGPL
    - Apache
    - CC-BY
    - CC-BY-NC
    - CC-BY-NC-SA
    - CC-BY-SA
    - CC0
    - GPL
    - LGPL
    - MIT
    - MPL
    - Unlicense

    Options:
      -l, --list lists the available licenses
      --author input author name. Default: `GitName <GitEmail>`
      --project input project name that is required by some license
      --year input license year
      --output path to the output. Default: ./LICENSE

## Supported Licenses

This CLI supports the following licenses:

- [AGPL-3.0](http://www.gnu.org/licenses/agpl-3.0)
- [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [CC-BY-4.0](https://creativecommons.org/licenses/by/4.0/)
- [CC-BY-NC-4.0](https://creativecommons.org/licenses/by-nc/4.0/)
- [CC-BY-NC-SA-4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/)
- [CC-BY-SA-4.0](https://creativecommons.org/licenses/by-sa/4.0/)
- [CC0-1.0](http://creativecommons.org/publicdomain/zero/1.0/)
- [GPL-3.0](http://www.gnu.org/licenses/gpl-3.0)
- [LGPL-3.0](http://www.gnu.org/licenses/lgpl-3.0)
- [MIT](https://opensource.org/licenses/MIT)
- [MPL-2.0](https://www.mozilla.org/en-US/MPL/2.0/)
- [Unlicense](http://unlicense.org/)

## Examples

### Generate MIT LICENSE

    license-generator MIT

### Output to stdout

    license-generator MIT --output /dev/stdout

### Multi LICENSE

    license-generator MIT Apache
    # generates `LICENSE-MIT` and `LICENSE-APACHE`

### Specify year and author

    license-generator MIT --author azu --year 2024

## Tests

    cargo test

## Releases

Use [cargo-release](https://github.com/sunng87/cargo-release).

    cargo release --execute --publish {patch,minor,major}

## Contributing

1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin my-new-feature`
5. Submit a pull request :D

## License

MIT © azu
