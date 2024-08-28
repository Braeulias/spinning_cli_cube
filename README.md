# Rotating ASCII Cube Renderer

This project is a Rust-based ASCII art program that renders and animates a rotating 3D cube in your terminal. The cube is displayed using various characters, creating a dynamic, mesmerizing visual experience.

## Features

- Real-time rotation of a 3D cube in a terminal.
- Customizable cube size, rotation speed, and character sets.
- Ability to add multiple cubes and adjust their positions relative to each other.

## Installation

### Arch Linux

1. **Install Rust:**

   If you don't have Rust installed, you can install it using `rustup`:

   ```bash
   sudo pacman -S rustup
   rustup default stable
   ```

2. **Clone the Repository:**

   ```bash
   git clone https://github.com/Braeulias/spinning_cli_cube.git
   cd spinning_cli_cube
   ```

3. **Build the Project:**

   ```bash
   cargo build --release
   ```

4. **Run the Program:**

   ```bash
   ./target/release/spinning_cli_cube
   ```

### Windows

1. **Install Rust:**

   Download and install Rust from [rustup.rs](https://rustup.rs/).

2. **Clone the Repository:**

   Open a command prompt and run:

   ```bash
   git clone https://github.com/Braeulias/spinning_cli_cube.git
   cd spinning_cli_cube
   ```

3. **Build the Project:**

   ```bash
   cargo build --release
   ```

4. **Run the Program:**

   ```bash
   .\target\release\spinning_cli_cube.exe
   ```

## Customization

### Cube Size and Rotation Speed

You can customize the cube size and rotation speed by modifying the following variables in the `main()` function:

- **Cube Size**: The `cube_width` variable controls the size of the cube.

  ```rust
  let cube_width = 15.0; // Change this value to resize the cube
  ```

- **Rotation Speed**: The `increment_speed` variable adjusts the speed at which the cube rotates.

  ```rust
  let increment_speed = 0.6; // Increase or decrease to change rotation speed
  ```

### Changing Characters

Each face of the cube is represented by different characters. You can customize these characters by modifying the `calculate_for_surface` calls in the loop:

```rust
calculate_for_surface(-cube_x, cube_y, cube_width, '#', &mut buffer, &mut z_buffer, width, height, a, b, c, distance_from_cam, k1, horizontel_offset);
```

Replace the `'#'` with any other character you prefer.

### Adding More Cubes

You can add more cubes by copying the block of code that renders a single cube and adjusting the parameters to avoid overlapping:

1. Copy the block of code that renders the existing cube.
2. Adjust `cube_width` and `horizontel_offset` to position the new cube:

```rust
let new_cube_width = 10.0; // New cube size
let new_horizontel_offset = 20.0; // Offset to position the new cube
```

3. Add the new cube render code below the existing one:

```rust
// Original cube
// ...
// New cube
let mut new_cube_x = -new_cube_width;
while new_cube_x < new_cube_width {
    let mut new_cube_y = -new_cube_width;
    while new_cube_y < new_cube_width {
        calculate_for_surface(-new_cube_x, new_cube_y, new_cube_width, '%', &mut buffer, &mut z_buffer, width, height, a, b, c, distance_from_cam, k1, new_horizontel_offset);
        // Add more surfaces for the new cube...
        new_cube_y += increment_speed;
    }
    new_cube_x += increment_speed;
}
```

This will render multiple cubes rotating simultaneously in your terminal.

## Contributing

Feel free to fork this project and submit pull requests if you have any improvements or new features. Contributions are always welcome!

## License

