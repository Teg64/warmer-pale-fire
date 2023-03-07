# Warmer Pale Fire

A modified version of matklad's [Pale Fire](https://github.com/matklad/pale-fire) Vs Code theme, originally based on [emacs-zenburn](https://github.com/bbatsov/zenburn-emacs).

Regular:

![Default](images/default_sample.png)

High Contrast:

![High Contrast](images/high_contrast_sample.png)

Darker:

![Darker](images/darker_sample.png)

Stealth:

![Stealth](images/stealth_sample.png)

## How to install

(Currently unavailable in the Vs Code extension marketplace)

Prerequisites:

- Have [Rust](https://www.rust-lang.org/) installed
- Have [nodejs and npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) installed

Installation guide:

1. Clone this repository
2. In the project root directory \[`~/.../warmer-pale-fire`\], run `cargo run` in the terminal
   - This should generate a `themes` folder in the project root directory, containing json files specifying the Vs Code themes
3. Run `npm run package` in the terminal
   - This should generate a .vsix file, named `warmer-pale-fire-X.X.X.vsix` (where X.X.X is the project's version)
4. Run `code --install-extension warmer-pale-fire-X.X.X.vsix` to add the extension to Vs Code
5. The themes should now be added to Vs Code, and can be selected under color themes

## License

Licensed under GPL3, as with [Pale Fire](https://github.com/matklad/pale-fire) and [emacs-zenburn](https://github.com/bbatsov/zenburn-emacs).
