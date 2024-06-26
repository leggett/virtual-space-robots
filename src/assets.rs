use raylib::{
    texture::{Image, Texture2D},
    RaylibHandle, RaylibThread,
};

pub struct Assets {
    pub ship: Texture2D,
    pub ship_dead: Texture2D,
    pub bullet: Texture2D,
    pub rock: Texture2D,
}

fn load_texture(path: &str, rl: &mut RaylibHandle, thread: &RaylibThread) -> Texture2D {
    rl.load_texture_from_image(&thread, &Image::load_image(path).expect("Image not found."))
        .expect("Failed to load texture")
}

pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Assets {
    Assets {
        ship: load_texture("assets/ship.png", rl, thread),
        ship_dead: load_texture("assets/ship_dead.png", rl, thread),
        bullet: load_texture("assets/bullet.png", rl, thread),
        rock: load_texture("assets/rock.png", rl, thread),
    }
}
