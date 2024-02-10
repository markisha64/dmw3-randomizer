
# Digimon World 3 Randomizer

The Digimon World 3 Randomizer is a tool developed in Rust that allows players to experience Digimon World 3 in a whole new way by randomizing various game elements.(currently encounters only) This repository contains the source code for the randomizer along with instructions on how to use it.

## Versions

Support is added for all 3 versions, if you notice any problems, fill out an issue.

## Features

- Randomized wild Digimon encounters
- Randomized bosses
- Randomized starting parties (currently not previewable)
- Randomized shops and item prices
- Randomized learned moves
- Randomized digivolutions
- Randomized starting stats and affinities
- Randomized map coloration
- Randomized map backgrounds
- Randomized item box items

## Binary Download

[Releases](https://github.com/markisha64/dmw3-randomizer/releases) for Windows/Ubuntu, built by github CI

## Requirements

- libwebkit2gtk-4.1-dev (Linux only)
- ~700MB of extra storage (for rom extracted files)

## Building from source

1. **Prerequisites**: Make sure you have Rust installed on your system. If not, you can download and install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).
   You also need to have [mkpsxiso](https://github.com/Lameguy64/mkpsxiso) installed or have the necessary tools to build it

2. **Clone the Repository**: Clone this repository to your local machine using the following command:
   ```shell
   git clone https://github.com/markisha64/dmw3-randomizer.git
   ```
3. **Build mkpsxiso(skip if installed)**: Navigate to the repository directory and built the randomizer using cmake:
   ```shell
   cmake --preset ci -S ./mkpsxiso -B ./mkpsxiso/build
   cmake --build ./mkpsxiso/build --config Release
   ```

4. **Build the Randomizer**: Navigate to the repository directory and build the randomizer using Cargo:
   ```shell
   cd dmw3-randomizer
   cargo build --release
   ```

5. **Usage**: Run the randomizer:
   ```shell
   cargo run --release
   ```
   Or if you want to use it as cli tool
   ```shell
   cargo run --release -- [options]
   ```
   Replace `[options]` with the specific options you want to use for randomization.


6. **Play the Randomized ROM**: Once the randomization process is complete, you'll find the randomized ROM file in the designated output folder. Load the ROM in your favorite Digimon World 3 emulator to start your randomized adventure!

## Contributing

We welcome contributions from the community to improve and expand the Digimon World 3 Randomizer. If you'd like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them.
4. Push your changes to your fork.
5. Open a pull request to this repository, detailing the changes you've made.

Please ensure that your code adheres to the project's coding standards and guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

## Acknowledgments

We would like to extend our gratitude to the developers of the original Digimon World 3 game for creating such an enjoyable experience, as well as to the Rust programming language community for providing the tools necessary to create this randomizer.

---

Have fun exploring the randomized world of Digimon World 3 with our randomizer tool! If you encounter any issues or have suggestions, please don't hesitate to open an issue on this repository. Happy gaming! 🎮🐾
