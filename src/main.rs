use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn calculate_x(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32 {
    //uses 3D Rotation Matrix
    j * a.sin() * b.sin() * c.cos() - k * a.cos() * b.sin() * c.cos()
        + j * a.cos() * c.sin()
        + k * a.sin() * c.sin()
        + i * b.cos() * c.cos()
}

fn calculate_y(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32 {
    //uses 3D Rotation Matrix
    j * a.cos() * c.cos() + k * a.sin() * c.cos() - j * a.sin() * b.sin() * c.sin()
        + k * a.cos() * b.sin() * c.sin()
        - i * b.cos() * c.sin()
}

fn calculate_z(i: f32, j: f32, k: f32, a: f32, b: f32) -> f32 {
    //uses 3D Rotation Matrix
    k * a.cos() * b.cos() - j * a.sin() * b.cos() + i * b.sin()
}

fn calculate_for_surface(
    cube_x: f32,
    cube_y: f32,
    cube_z: f32,
    ch: char,
    buffer: &mut [char],
    z_buffer: &mut [f32],
    width: usize,
    height: usize,
    a: f32,
    b: f32,
    c: f32,
    distance_from_cam: f32,
    k1: f32,
    horizontal_offset: f32,
) {
    let x = calculate_x(cube_x, cube_y, cube_z, a, b, c);
    let y = calculate_y(cube_x, cube_y, cube_z, a, b, c);
    let z = calculate_z(cube_x, cube_y, cube_z, a, b) + distance_from_cam;

    if z <= 0.0 {
        return;
    }

    let ooz = 1.0 / z; //reverse

    let mut xp = (width as f32 / 2.0 + horizontal_offset + k1 * ooz * x * 2.0).round() as isize; //calculates the point in the center + the offset + k1 to see how large it should be + ooz to check if its visible
    let mut yp = (height as f32 / 2.0 + k1 * ooz * y).round() as isize;

    xp = xp.clamp(0, width as isize - 1);
    yp = yp.clamp(0, height as isize - 1);

    let idx = (xp + yp * width as isize) as usize; //what idx to place the char in the Buffer

    if ooz > z_buffer[idx] {
        z_buffer[idx] = ooz;
        buffer[idx] = ch;
    }
}

fn main() {
    let width = 160; //of the Terminal
    let height = 44; //again of the Terminal :)
    let mut a = 0.0; //rotation x
    let mut b = 0.0; //rotation y
    let mut c = 0.0; //rotation z

    let distance_from_cam = 100.0; //used to change size of all cubes if you have multiple
    let k1 = 40.0;
    let increment_speed = 0.4; //speed of cube

    let mut buffer = vec![' '; width * height]; //cube array
    let mut z_buffer = vec![0.0; width * height]; //visible? array

    print!("\x1b[2J");
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    loop {
        buffer.fill(' ');
        z_buffer.fill(0.0);

        //change it you want more cubes
        let cube_width = 15.0; //Change cube size
        let horizontel_offset = -2.0 * cube_width; //only important if you want multiple cubes

        //cube
        let mut cube_x = -cube_width;
        while cube_x < cube_width {
            let mut cube_y = -cube_width;
            while cube_y < cube_width {
                calculate_for_surface(
                    -cube_x,
                    cube_y,
                    cube_width,
                    '#',
                    &mut buffer,
                    &mut z_buffer,
                    width,
                    height,
                    a,
                    b,
                    c,
                    distance_from_cam,
                    k1,
                    horizontel_offset,
                );
                calculate_for_surface(
                    cube_x,
                    cube_y,
                    -cube_width,
                    '@',
                    &mut buffer,
                    &mut z_buffer,
                    width,
                    height,
                    a,
                    b,
                    c,
                    distance_from_cam,
                    k1,
                    horizontel_offset,
                );
                calculate_for_surface(
                    cube_width,
                    cube_y,
                    cube_x,
                    '$',
                    &mut buffer,
                    &mut z_buffer,
                    width,
                    height,
                    a,
                    b,
                    c,
                    distance_from_cam,
                    k1,
                    horizontel_offset,
                );
                calculate_for_surface(
                    -cube_width,
                    cube_y,
                    -cube_x,
                    '~',
                    &mut buffer,
                    &mut z_buffer,
                    width,
                    height,
                    a,
                    b,
                    c,
                    distance_from_cam,
                    k1,
                    horizontel_offset,
                );
                calculate_for_surface(
                    cube_x,
                    cube_width,
                    cube_y,
                    '+',
                    &mut buffer,
                    &mut z_buffer,
                    width,
                    height,
                    a,
                    b,
                    c,
                    distance_from_cam,
                    k1,
                    horizontel_offset,
                );
                calculate_for_surface(
                    cube_x,
                    -cube_width,
                    -cube_y,
                    ';',
                    &mut buffer,
                    &mut z_buffer,
                    width,
                    height,
                    a,
                    b,
                    c,
                    distance_from_cam,
                    k1,
                    horizontel_offset,
                );

                cube_y += increment_speed;
            }
            cube_x += increment_speed;
        }

        // if you want to add cubes just copy the code below cube and change the cube with of the new one plus the offset so that it doesnt overlap

        print!("\x1b[H");
        for k in 0..width * height {
            if k % width == 0 && k != 0 {
                writeln!(handle).unwrap();
            }
            write!(handle, "{}", buffer[k]).unwrap();
        }

        handle.flush().unwrap();

        a += 0.04;
        b += 0.04;
        c += 0.008;
        sleep(Duration::from_millis(16));
    }
}
