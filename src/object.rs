const MAX_X_SPEED: i32 = 10;
const MAX_Y_SPEED: i32 = 30;
const GRAVITY: i32 = 1;
const FRICTION: i32 = 1;
const WINDOW_HEIGHT: u32 = 600;

pub struct Object {
    x_position: i32,
    y_position: i32,
    width: u32,
    height: u32,
    x_speed: i32,
    y_speed: i32,
}

impl Object {
    // A constructor-like method
    pub fn new(x_position: i32, y_position: i32, width: u32, height: u32) -> Self {
        Object {
            x_position: x_position,
            y_position: y_position,
            width: width,
            height: height,
            x_speed: 0,
            y_speed: 0,
        }
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    fn set_x_speed(&mut self, speed: i32) {
        self.x_speed = speed;
    }

    fn get_x_speed(&self) -> i32 {
        self.x_speed
    }

    fn set_y_speed(&mut self, speed: i32) {
        self.y_speed = speed;
    }

    fn get_y_speed(&self) -> i32 {
        self.y_speed
    }

    pub fn change_x_speed(&mut self, speed: i32) {
        // Implement a max speed check
        let new_speed = self.get_x_speed() + speed;
        if new_speed > MAX_X_SPEED as i32 {
            self.set_x_speed(MAX_X_SPEED as i32);
        } else if new_speed < -MAX_X_SPEED as i32 {
            self.set_x_speed(-MAX_X_SPEED as i32);
        } else {
            self.set_x_speed(new_speed);
        }
    }

    pub fn change_y_speed(&mut self, speed: i32) {
        let new_speed = self.get_y_speed() + speed;

        if new_speed > MAX_Y_SPEED as i32 {
            self.set_y_speed(MAX_Y_SPEED as i32);
        } else {
            self.set_y_speed(new_speed);
        }
        
    }

    fn set_x_position(&mut self, x_position: i32) {
        self.x_position = x_position;
    }

    pub fn get_x_position(&self) -> i32 {
        self.x_position
    }

    fn set_y_position(&mut self, y_position: i32) {
        //Stop them from going below the bottom of the screen
        if y_position >= WINDOW_HEIGHT as i32 - self.height as i32 {
            self.y_position = WINDOW_HEIGHT as i32 - self.height as i32;
        } else {
            self.y_position = y_position;
        }
    }

    pub fn get_y_position(&self) -> i32 {
        self.y_position
    }

    pub fn calculate_new_position(&mut self) {
        // self.set_y_speed(self.get_y_speed() + GRAVITY as i32);
        if self.is_at_bottom() && self.get_y_speed() >= 0 {
            self.set_y_speed(0);
        } else {
            self.gravity();
        }
        
        self.friction();

        self.set_x_position(self.get_x_position() + self.get_x_speed());
        self.set_y_position(self.get_y_position() + self.get_y_speed());

        //Debug by printing the current speed
        println!("X Speed: {}, Y Speed: {}", self.get_x_speed(), self.get_y_speed());
    }

    fn gravity(&mut self) {
        self.change_y_speed(GRAVITY as i32);
    }

    pub fn anti_gravity(&mut self) {
        self.set_y_speed(-GRAVITY as i32);
        // self.change_y_speed(-GRAVITY as i32);
    }

    fn friction(&mut self) {
        if self.get_x_speed() == 0 {
            return;
        } else {
            if self.get_x_speed() > 0 {
                self.change_x_speed(-FRICTION as i32);
            } else if self.get_x_speed() < 0 {
                self.change_x_speed(FRICTION as i32);
            }
        }
    }

    fn is_at_bottom(&self) -> bool {
        // y_position > WINDOW_HEIGHT as i32 - self.height as i32 
        self.get_y_position() >= WINDOW_HEIGHT as i32 - self.get_height() as i32
    }



    // // A regular method
    // fn describe(&self) {
    //     println!("field1 is {}, field2 is {}", self.field1, self.field2);
    // }
}
