use image::{GenericImage, RgbImage};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

// grid
// fn generate_frame_content(img: &mut RgbImage, frame_number: usize) {
//     let width = img.width();
//     let height = img.height();

//     for (x, y, pixel) in img.enumerate_pixels_mut() {
//         let shift = frame_number as u32;
//         let r = (((x + shift) % 256) as u8);
//         let g = (((y + shift) % 256) as u8);
//         let b = 128;
//         *pixel = image::Rgb([r, g, b]);
//     }
// }

// bouncing ball
// fn generate_frame_content(img: &mut RgbImage, frame_number: usize) {
//     use image::Pixel;

//     let width = img.width() as i32;
//     let height = img.height() as i32;

//     // Ball properties
//     let ball_radius = 100;
//     let speed = 15;

//     // Calculate ball position
//     let mut x = (frame_number as i32 * speed) % (2 * width);
//     let mut y = (frame_number as i32 * speed) % (2 * height);

//     if x >= width {
//         x = 2 * width - x;
//     }
//     if y >= height {
//         y = 2 * height - y;
//     }

//     // Clear the image (black background)
//     for pixel in img.pixels_mut() {
//         *pixel = image::Rgb([0, 0, 0]);
//     }

//     // Draw the ball (simple circle)
//     for (img_x, img_y, pixel) in img.enumerate_pixels_mut() {
//         let dx = img_x as i32 - x;
//         let dy = img_y as i32 - y;
//         if dx * dx + dy * dy <= ball_radius * ball_radius {
//             *pixel = image::Rgb([255, 0, 0]); // Red ball
//         }
//     }
// }

// rainbow
// fn generate_frame_content(img: &mut RgbImage, frame_number: usize) {
//     use palette::{FromColor, Hsv, Srgb};

//     let hue_shift = (frame_number as f32) % 360.0;

//     for (x, y, pixel) in img.enumerate_pixels_mut() {
//         let hue = ((x + y) as f32 + hue_shift) % 360.0;
//         let hsv = Hsv::new(hue, 1.0, 1.0);

//         // Convert Hsv to Srgb
//         let rgb: Srgb = Srgb::from_color(hsv);

//         // Convert to u8 format
//         let rgb_u8 = rgb.into_format::<u8>();

//         // Extract the RGB components
//         let (r, g, b) = rgb_u8.into_components();

//         // Set the pixel value
//         *pixel = image::Rgb([r, g, b]);
//     }
// }

// Mandelbrot
// fn generate_frame_content(img: &mut RgbImage, frame_number: usize) {
//     let width = img.width() as f64;
//     let height = img.height() as f64;

//     // Zoom parameters
//     let zoom = 0.01 + frame_number as f64 * 0.0005;
//     let move_x = -0.5;
//     let move_y = 0.0;
//     let max_iter = 256;

//     for (x, y, pixel) in img.enumerate_pixels_mut() {
//         let x0 = (x as f64 / width - 0.5) / zoom + move_x;
//         let y0 = (y as f64 / height - 0.5) / zoom + move_y;

//         let mut a = 0.0;
//         let mut b = 0.0;
//         let mut iter = 0;

//         while a * a + b * b <= 4.0 && iter < max_iter {
//             let temp = a * a - b * b + x0;
//             b = 2.0 * a * b + y0;
//             a = temp;
//             iter += 1;
//         }

//         let color = if iter < max_iter {
//             let c = (iter * 255 / max_iter) as u8;
//             [c, c, c]
//         } else {
//             [0, 0, 0]
//         };
//         *pixel = image::Rgb(color);
//     }
// }

// rotating square
// fn generate_frame_content(img: &mut RgbImage, frame_number: usize) {
//     use image::Pixel;
//     use std::f32::consts::PI;

//     let width = img.width() as i32;
//     let height = img.height() as i32;

//     // Square properties
//     let square_size = 200.0;
//     let angle = (frame_number as f32 * 0.05) % (2.0 * PI);

//     // Center of the image
//     let cx = width as f32 / 2.0;
//     let cy = height as f32 / 2.0;

//     // Clear the image (black background)
//     for pixel in img.pixels_mut() {
//         *pixel = image::Rgb([0, 0, 0]);
//     }

//     // Draw the rotating square
//     for (img_x, img_y, pixel) in img.enumerate_pixels_mut() {
//         let x = img_x as f32 - cx;
//         let y = img_y as f32 - cy;

//         // Rotate coordinates
//         let x_rot = x * angle.cos() - y * angle.sin();
//         let y_rot = x * angle.sin() + y * angle.cos();

//         if x_rot.abs() < square_size / 2.0 && y_rot.abs() < square_size / 2.0 {
//             *pixel = image::Rgb([0, 255, 0]); // Green square
//         }
//     }
// }

// Image slideshow
// fn generate_frame_content(img: &mut RgbImage, frame_number: usize) {
//     use image::{GenericImageView, Pixel};

//     // List of image paths
//     let images = [
//         "c:\\Users\\Brioche Elm\\Pictures\\2023-10-21_23-03.png",
//         "c:\\Users\\Brioche Elm\\Pictures\\blinding_lights.jpg",
//         "c:\\Users\\Brioche Elm\\Pictures\\crown.jpg",
//     ];
//     let frames_per_image = 60; // Display each image for 2 seconds at 30 fps

//     // Determine which image to display
//     let image_index = (frame_number / frames_per_image) % images.len();
//     let image_path = images[image_index];

//     // Load the image
//     let loaded_img = image::open(image_path).expect("Failed to load image");

//     // Resize the image to match the frame size
//     let resized = loaded_img.resize_exact(
//         img.width(),
//         img.height(),
//         image::imageops::FilterType::Lanczos3,
//     );

//     // Convert the resized image to an RgbImage
//     let rgb_image = resized.to_rgb8();

//     // Copy the pixels into the frame buffer
//     img.copy_from(&rgb_image, 0, 0)
//         .expect("Failed to copy image");
// }

use image::{ImageBuffer, Rgb};
use std::f32::consts::PI;
// Ball structure representing each ball's properties
struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
}

// Square structure representing each square's properties
struct Square {
    cx: f32,
    cy: f32,
    size: f32,
    angle: f32,
    angular_velocity: f32,
}

// Scene structure to hold all balls, squares, and scene properties
struct Scene {
    balls: Vec<Ball>,
    squares: Vec<Square>,
    gravity: f32,
    width: u32,
    height: u32,
}

impl Scene {
    fn new(width: u32, height: u32) -> Self {
        let ball_radius = 30.0;

        // Initialize balls with random positions and velocities
        let balls = (0..50)
            .map(|_| Ball {
                x: (rand::random::<f32>() * width as f32 * 0.8) + width as f32 * 0.1,
                y: (rand::random::<f32>() * height as f32 * 0.1),
                vx: rand::random::<f32>() * 10.0 - 5.0,
                vy: rand::random::<f32>() * 5.0,
                radius: ball_radius,
            })
            .collect();

        // Initialize squares
        let mut squares = Vec::new();

        // Add the square on the ground at center
        squares.push(Square {
            cx: width as f32 / 2.0,
            cy: height as f32 - (ball_radius * 2.0),
            size: height as f32 * 0.2,
            angle: 0.0,
            angular_velocity: 0.05,
        });

        // Add two more squares on the ground at left third and right third
        squares.push(Square {
            cx: width as f32 / 3.0,
            cy: height as f32 - (ball_radius * 2.0),
            size: height as f32 * 0.2,
            angle: 0.0,
            angular_velocity: -0.05, // opposite rotation
        });

        squares.push(Square {
            cx: 2.0 * width as f32 / 3.0,
            cy: height as f32 - (ball_radius * 2.0),
            size: height as f32 * 0.2,
            angle: 0.0,
            angular_velocity: 0.05,
        });

        // Add two floating squares in the lower half of the screen
        for _ in 0..2 {
            let square = Square {
                cx: (rand::random::<f32>() * width as f32 * 0.8) + width as f32 * 0.1,
                cy: (rand::random::<f32>() * height as f32 * 0.4) + height as f32 * 0.5, // lower half
                size: height as f32 * 0.15, // Slightly smaller size
                angle: rand::random::<f32>() * 2.0 * PI,
                angular_velocity: (rand::random::<f32>() * 0.1) - 0.05, // Random angular velocity
            };
            squares.push(square);
        }

        Scene {
            balls,
            squares,
            gravity: 0.5,
            width,
            height,
        }
    }

    fn update(&mut self) {
        // Update the angle of each rotating square
        for square in &mut self.squares {
            square.angle = (square.angle + square.angular_velocity) % (2.0 * PI);
        }

        // Update each ball's position and velocity
        for ball in &mut self.balls {
            // Apply gravity
            ball.vy += self.gravity;

            // Update position
            ball.x += ball.vx;
            ball.y += ball.vy;

            // Ground collision
            if ball.y + ball.radius > self.height as f32 {
                ball.y = self.height as f32 - ball.radius;
                ball.vy *= -0.7; // Dampen the velocity to simulate energy loss
            }

            // Wall collisions
            if ball.x - ball.radius < 0.0 {
                ball.x = ball.radius;
                ball.vx *= -1.0;
            }
            if ball.x + ball.radius > self.width as f32 {
                ball.x = self.width as f32 - ball.radius;
                ball.vx *= -1.0;
            }

            // Collision with each rotating square
            for square in &self.squares {
                if let Some((contact_point_x, contact_point_y, normal_x, normal_y)) =
                    circle_rotated_rect_collision(
                        ball.x,
                        ball.y,
                        ball.radius,
                        square.cx,
                        square.cy,
                        square.size,
                        square.angle,
                    )
                {
                    // Adjust the position to the point of contact
                    let penetration = ball.radius
                        - ((ball.x - contact_point_x).powi(2) + (ball.y - contact_point_y).powi(2))
                            .sqrt();
                    ball.x += normal_x * penetration;
                    ball.y += normal_y * penetration;

                    // Reflect the velocity about the normal vector
                    let dot_product = ball.vx * normal_x + ball.vy * normal_y;
                    ball.vx -= 2.0 * dot_product * normal_x;
                    ball.vy -= 2.0 * dot_product * normal_y;

                    // Apply friction or energy loss if desired
                    ball.vx *= 0.9;
                    ball.vy *= 0.9;

                    // Optional: Add rotational influence
                    let tangential_velocity = square.angular_velocity * square.size / 2.0;
                    ball.vx += -normal_y * tangential_velocity;
                    ball.vy += normal_x * tangential_velocity;
                }
            }
        }

        // Collision between balls (unchanged)
        for i in 0..self.balls.len() {
            for j in (i + 1)..self.balls.len() {
                let (ball1, ball2) = {
                    let (left, right) = self.balls.split_at_mut(j);
                    (&mut left[i], &mut right[0])
                };
                let dx = ball2.x - ball1.x;
                let dy = ball2.y - ball1.y;
                let distance_sq = dx * dx + dy * dy;
                let radii_sum = ball1.radius + ball2.radius;
                if distance_sq < radii_sum * radii_sum {
                    // Collision detected, resolve it
                    let distance = distance_sq.sqrt();
                    let overlap = 0.5 * (radii_sum - distance);

                    // Avoid division by zero
                    let normal_x;
                    let normal_y;
                    if distance == 0.0 {
                        // Balls are at the same position, choose a random normal
                        normal_x = 1.0;
                        normal_y = 0.0;
                    } else {
                        normal_x = dx / distance;
                        normal_y = dy / distance;
                    }

                    // Adjust positions
                    ball1.x -= overlap * normal_x;
                    ball1.y -= overlap * normal_y;
                    ball2.x += overlap * normal_x;
                    ball2.y += overlap * normal_y;

                    // Compute relative velocity
                    let dvx = ball2.vx - ball1.vx;
                    let dvy = ball2.vy - ball1.vy;
                    // Compute relative velocity in terms of the normal direction
                    let vn = dvx * normal_x + dvy * normal_y;
                    if vn > 0.0 {
                        // Balls are moving away from each other
                        continue;
                    }
                    // Impulse scalar
                    let restitution = 0.9; // Slightly inelastic collision
                    let impulse = -(1.0 + restitution) * vn / 2.0; // assuming equal mass
                                                                   // Apply impulse to the balls
                    ball1.vx -= impulse * normal_x;
                    ball1.vy -= impulse * normal_y;
                    ball2.vx += impulse * normal_x;
                    ball2.vy += impulse * normal_y;
                }
            }
        }
    }

    fn draw(&self, img: &mut RgbImage) {
        // Clear the image (black background)
        for pixel in img.pixels_mut() {
            *pixel = Rgb([0, 0, 0]);
        }

        // Draw the rotating squares
        for square in &self.squares {
            for (img_x, img_y, pixel) in img.enumerate_pixels_mut() {
                let x = img_x as f32 - square.cx;
                let y = img_y as f32 - square.cy;

                // Rotate coordinates
                let x_rot = x * square.angle.cos() + y * square.angle.sin();
                let y_rot = -x * square.angle.sin() + y * square.angle.cos();

                if x_rot.abs() < square.size / 2.0 && y_rot.abs() < square.size / 2.0 {
                    *pixel = Rgb([0, 255, 0]); // Green square
                }
            }
        }

        // Draw the balls
        for ball in &self.balls {
            for (img_x, img_y, pixel) in img.enumerate_pixels_mut() {
                let dx = img_x as f32 - ball.x;
                let dy = img_y as f32 - ball.y;
                if dx * dx + dy * dy <= ball.radius * ball.radius {
                    *pixel = Rgb([255, 0, 0]); // Red ball
                }
            }
        }
    }
}

// Function to check collision between a circle and a rotated rectangle
fn circle_rotated_rect_collision(
    circle_x: f32,
    circle_y: f32,
    circle_radius: f32,
    rect_cx: f32,
    rect_cy: f32,
    rect_size: f32,
    rect_angle: f32,
) -> Option<(f32, f32, f32, f32)> {
    // Translate circle center to rectangle's coordinate frame
    let dx = circle_x - rect_cx;
    let dy = circle_y - rect_cy;

    // Rotate the point in the opposite direction (to align with rectangle)
    let cos_theta = rect_angle.cos();
    let sin_theta = rect_angle.sin();
    let x_rot = dx * cos_theta + dy * sin_theta;
    let y_rot = -dx * sin_theta + dy * cos_theta;

    // Axis-aligned rectangle bounds
    let half_size = rect_size / 2.0;
    let rx_min = -half_size;
    let ry_min = -half_size;
    let rx_max = half_size;
    let ry_max = half_size;

    // Find the closest point on the rectangle to the circle's center
    let closest_x = x_rot.clamp(rx_min, rx_max);
    let closest_y = y_rot.clamp(ry_min, ry_max);

    // Compute the distance between the circle's center and this closest point
    let dist_x = x_rot - closest_x;
    let dist_y = y_rot - closest_y;
    let distance_sq = dist_x * dist_x + dist_y * dist_y;

    if distance_sq < circle_radius * circle_radius {
        // Collision detected
        // Compute the collision normal in rectangle's coordinate frame
        let mut normal_x = dist_x;
        let mut normal_y = dist_y;

        // Handle the case when the circle's center is inside the rectangle
        if distance_sq == 0.0 {
            // Find the direction to push the circle out along the smallest axis
            let dx_min = x_rot - rx_min;
            let dx_max = rx_max - x_rot;
            let dy_min = y_rot - ry_min;
            let dy_max = ry_max - y_rot;
            let min_dx = dx_min.min(dx_max);
            let min_dy = dy_min.min(dy_max);

            if min_dx < min_dy {
                normal_x = if dx_min < dx_max { -1.0 } else { 1.0 };
                normal_y = 0.0;
            } else {
                normal_x = 0.0;
                normal_y = if dy_min < dy_max { -1.0 } else { 1.0 };
            }
        } else {
            let distance = distance_sq.sqrt();
            normal_x /= distance;
            normal_y /= distance;
        }

        // Rotate the normal back to the original coordinate frame
        let normal_x_world = normal_x * cos_theta - normal_y * sin_theta;
        let normal_y_world = normal_x * sin_theta + normal_y * cos_theta;

        // Rotate the contact point back to world coordinates
        let contact_point_x = closest_x * cos_theta - closest_y * sin_theta + rect_cx;
        let contact_point_y = closest_x * sin_theta + closest_y * cos_theta + rect_cy;

        Some((
            contact_point_x,
            contact_point_y,
            normal_x_world,
            normal_y_world,
        ))
    } else {
        None
    }
}

struct VideoRenderer {
    width: u32,
    height: u32,
    framerate: u32,
    frame_count: u32,
    ffmpeg: std::process::Child,
    // stdin: std::process::ChildStdin,
}

impl VideoRenderer {
    fn new(width: u32, height: u32, framerate: u32, output_file_path: &Path) -> Self {
        // Start the FFmpeg process
        let ffmpeg = Command::new("ffmpeg")
            .args(&[
                "-y", // Overwrite output files without asking
                "-f",
                "rawvideo",
                "-pixel_format",
                "rgb24",
                "-video_size",
                &format!("{}x{}", width, height),
                "-framerate",
                &framerate.to_string(),
                "-i",
                "-",                                // Read input from stdin
                output_file_path.to_str().unwrap(), // Output file
            ])
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to start FFmpeg process");

        Self {
            width,
            height,
            framerate,
            frame_count: 0,
            ffmpeg,
        }
    }

    fn append_frame(&mut self, img: RgbImage) -> std::io::Result<u32> {
        let stdin = self
            .ffmpeg
            .stdin
            .as_mut()
            .expect("Failed to open FFmpeg stdin");

        // Convert the image to raw RGB data
        let raw_data = img.into_raw();

        // Write the raw data to FFmpeg's stdin
        stdin.write_all(&raw_data)?;

        self.frame_count += 1;

        Ok(self.frame_count)
    }

    fn finish(mut self) -> std::io::Result<()> {
        // Close the stdin to signal FFmpeg that we're done sending data
        drop(self.ffmpeg.stdin.take());

        // Wait for FFmpeg to finish processing
        self.ffmpeg.wait()?;

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    // // Video parameters
    let width = 1920;
    let height = 1080;
    let framerate = 60;
    let total_frames = 900; // For a 10-second video at 30 fps
                            // Initialize the VideoRenderer

    // path from str
    let out_video_path = Path::new("output.mp4");
    let mut renderer = VideoRenderer::new(width, height, framerate, out_video_path);

    // // Append frames
    // for frame_number in 0..total_frames {
    //     let mut img = RgbImage::new(width, height);
    //     generate_frame_content(&mut img, frame_number.try_into().unwrap());
    //     let _frames_encoded = renderer.append_frame(img)?;
    // }

    // Ok(())

    let mut scene = Scene::new(width, height);
    // let width = 800;
    // let height = 600;

    for frame_number in 0..total_frames {
        let mut img = ImageBuffer::new(width, height);
        scene.update();
        scene.draw(&mut img);

        let _frames_encoded = renderer.append_frame(img)?;

        // // Save the frame as an image (you can use a different method to create a video)
        // let filename = format!("frame_{:03}.png", frame_number);
        // img.save(&filename).unwrap();
    }

    // Finalize the video rendering
    renderer.finish()?;
    Ok(())
}
