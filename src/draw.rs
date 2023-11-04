use sdl2::rect::Rect;

pub fn draw_square(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32, w: u32, h: u32) {
    let mut square = Rect::new(x, y, w, h); // X, Y, Width, Height
    canvas.fill_rect(square).unwrap(); 
}

// mod draw {

// }

// mod cube {
//     pub struct Cube {
//         pub x: i32,
//         pub y: i32,
//         pub w: u32,
//         pub h: u32,
//     }
//     impl Cube {
//         pub fn new(x: i32, y: i32, w: u32, h: u32) -> Cube {
//             Cube {
//                 x: x,
//                 y: y,
//                 w: w,
//                 h: h,
//             }
//         }
//     }

//     /*
//     let mut square = Rect::new(375, 275, 50, 50); // X, Y, Width, Height
//       canvas.fill_rect(square).unwrap(); 
//     */
// }