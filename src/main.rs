use std::collections::HashMap;

use raylib::prelude::*;

struct SpriteAnimation {
    texture: Texture2D,
    frame_width: f32,
    num_frames: u32,
    current_frame: u32,
    frames_counter: u32,
    anim_speed: u32, // REVERSED
}

impl SpriteAnimation {
    fn new(sprite: Texture2D, num_frames: u32, speed: u32) -> SpriteAnimation {
        let frame_width = sprite.width as f32 / num_frames as f32;
        SpriteAnimation { 
            texture: sprite, 
            frame_width,
            num_frames, 
            current_frame: 0, 
            frames_counter: 0, 
            anim_speed: speed, 
        }
    }

    fn animate(&mut self) {
        self.frames_counter += 1;
        if self.frames_counter >= (60 / self.anim_speed) {
            self.frames_counter = 0;
            self.current_frame += 1;

            if self.current_frame > self.num_frames - 1 {
                self.current_frame = 0;
            }
        }
    }

    fn draw(&self, pos: Vector2, d: &mut RaylibDrawHandle) {
        let source_rec = Rectangle::new(
            self.current_frame as f32 * self.frame_width, 
            0.0, 
            self.frame_width, 
            self.texture.height as f32
        );

        let dest_rec = Rectangle::new(pos.x, pos.y, self.frame_width, self.texture.height as f32);
        d.draw_texture_pro(
            &self.texture,
            source_rec,
            dest_rec,
            Vector2::new(0.0, 0.0), // Origin (for rotation/scaling)
            0.0,                    // Rotation
            Color::WHITE,
        );
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum AnimationType {
    Idle(Direction),
    Run(Direction),
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Player {
    collision: Rectangle,
    animations: HashMap<AnimationType, SpriteAnimation>,
    pos: Vector2,
    current_animation: AnimationType,
    is_moving: bool,
    speed: f32,
}

impl Player {
    fn new(x: f32, y: f32, width: f32, height: f32, speed: f32) -> Player {
        let animations = HashMap::new();
        Player { 
            collision: Rectangle::new(x, y, width, height), 
            animations, 
            pos: Vector2::zero(), 
            current_animation: AnimationType::Idle(Direction::DOWN),
            is_moving: false,
            speed,
        }
    }

    fn add_animation(&mut self, 
        rl: &mut RaylibHandle, thread: &RaylibThread, 
        animation_type: AnimationType, 
        file: &str,
        num_frames: u32, speed: u32)
    {
        let sprite = rl.load_texture(&thread, file).unwrap();
        let animation = SpriteAnimation::new(sprite, num_frames, speed);
        self.animations.insert(animation_type, animation);
    }

    fn change_animation(&mut self, animation_type: AnimationType) {
        self.current_animation = animation_type;
    }

    fn animate(&mut self) {
        let animation = self.animations.get_mut(&self.current_animation)
            .expect("Couldn't found animation {:?}, on player.");
        animation.animate();
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        let animation = self.animations.get(&self.current_animation)
            .expect("Couldn't found animation {:?}, on player.");
        animation.draw(self.pos, d);
    }

    fn move_player(&mut self, dir: Direction) {
        match dir {
            Direction::UP => {
                self.pos.y -= self.speed;
            },
            Direction::DOWN => {
                self.pos.y += self.speed;
            },
            Direction::RIGHT => {
                self.pos.x += self.speed;
            },
            Direction::LEFT => {
                self.pos.x -= self.speed;
            },
        }
        self.change_animation(AnimationType::Run(dir));
    }
}

fn main() {
    let w = 640;
    let h = 480;

    let (mut rl, thread) = raylib::init()
        .size(w, h)
        .title("Non-Hot Reloaded Game")
        .build();


    let mut player = Player::new(42.0, 58.0, 12.0, 28.0, 2.0);
    player.add_animation(
        &mut rl, &thread, 
        AnimationType::Idle(Direction::DOWN), "resources/Hero/Sprites/IDLE/idle_down.png",
        8, 20
    );

    player.add_animation(
        &mut rl, &thread, 
        AnimationType::Idle(Direction::UP), "resources/Hero/Sprites/IDLE/idle_up.png",
        8, 20
    );

    player.add_animation(
        &mut rl, &thread, 
        AnimationType::Idle(Direction::RIGHT), "resources/Hero/Sprites/IDLE/idle_right.png",
        8, 20
    );

    player.add_animation(
        &mut rl, &thread, 
        AnimationType::Idle(Direction::LEFT), "resources/Hero/Sprites/IDLE/idle_left.png",
        8, 20
    );

    player.add_animation(
        &mut rl, &thread, 
        AnimationType::Run(Direction::DOWN), "resources/Hero/Sprites/RUN/run_down.png",
        8, 20
    );

    player.add_animation(
        &mut rl, &thread, 
        AnimationType::Run(Direction::UP), "resources/Hero/Sprites/RUN/run_up.png",
        8, 20
    );

    player.add_animation(
        &mut rl, &thread, 
        AnimationType::Run(Direction::RIGHT), "resources/Hero/Sprites/RUN/run_right.png",
        8, 20
    );

    player.add_animation(
        &mut rl, &thread, 
        AnimationType::Run(Direction::LEFT), "resources/Hero/Sprites/RUN/run_left.png",
        8, 20
    );

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_A) {
            player.move_player(Direction::LEFT);
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            player.move_player(Direction::RIGHT);
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            player.move_player(Direction::DOWN); 
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            player.move_player(Direction::UP);
        }

        if rl.is_key_released(KeyboardKey::KEY_A) {
            player.change_animation(AnimationType::Idle(Direction::LEFT));
        }
        if rl.is_key_released(KeyboardKey::KEY_D) {
            player.change_animation(AnimationType::Idle(Direction::RIGHT));
        }
        if rl.is_key_released(KeyboardKey::KEY_S) {
            player.change_animation(AnimationType::Idle(Direction::DOWN));
        }
        if rl.is_key_released(KeyboardKey::KEY_W) {
            player.change_animation(AnimationType::Idle(Direction::UP));
        }

        player.animate();

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::get_color(0x181818FF));

        player.draw(&mut d);
    }
}
