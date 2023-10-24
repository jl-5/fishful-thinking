use rand::Rng;
use crate::Animation;
pub struct Char_action {
    pub screen_region: [f32; 4],
    pub animations: Vec<Animation>,
    pub current_animation_index: usize,
    pub speed: f32,
    pub facing_right: bool,
    pub sprites_index: usize,

}

impl Char_action {

    pub fn new(screen_re: [f32; 4],
        anims: Vec<Animation>,
        cur_anim_index: usize,
        spe: f32,
        facing_rig: bool,
        sprites_ind: usize,) -> Char_action {
            Self { screen_region: (screen_re), 
                animations: (anims), 
                current_animation_index: (cur_anim_index),
                speed: (spe), 
                facing_right: (facing_rig), 
                sprites_index: (sprites_ind) }
    }

    pub fn walk(&mut self){
        if self.facing_right {
            self.screen_region[0] += self.speed;
        }
        // if facing left
        else {
            self.screen_region[0] -= self.speed;
        }
    }
    pub fn face_left(&mut self) {
        self.facing_right = false;
        if self.screen_region[2] < 0.0 {
            self.screen_region[2] *= -1.0;
            self.screen_region[0] -= 60.0;
        }
        
    }
    pub fn face_right(&mut self) {
        self.facing_right = true;
        if self.screen_region[2] > 0.0 {
            self.screen_region[2] *= -1.0;
            self.screen_region[0] += 60.0;
        }
    }
    pub fn move_down(&mut self) {
        self.screen_region[1] -= self.speed;

        if self.screen_region[1] <= 0.0 {
            self.screen_region[1] = 768.0;
            self.screen_region[0] = rand::thread_rng().gen_range(0..1025) as f32;
        }
    }
    pub fn move_right(&mut self) {
        self.screen_region[0] -= self.speed;

        if self.screen_region[0] <= 0.0 {
            self.screen_region[0] = 1024.0;
            self.screen_region[1] = rand::thread_rng().gen_range(0..769) as f32;
        }
    }
    pub fn reset_x(&mut self){
        self.screen_region[0] = 1024.0;
        self.screen_region[1] = rand::thread_rng().gen_range(0..769) as f32;
    }
    pub fn reset_y(&mut self){
        self.screen_region[1] = 768.0;
        self.screen_region[0] = rand::thread_rng().gen_range(0..1025) as f32;
    }

    pub fn set_animation_index(&mut self, index: usize) {
        self.current_animation_index = index;
    }
}