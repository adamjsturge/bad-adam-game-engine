mod object;

const PLAYER_SPEED: u32 = 5;

struct Player {
    object: object::Object,
}

pub impl Player {
    fn new(x_position: i32, y_position: i32, width: u32, height: u32) -> Self {
        Player {
            object: object::Object::new(x_position, y_position, width, height),
        }
    }

    fn go_
}