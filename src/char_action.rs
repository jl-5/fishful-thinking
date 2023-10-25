use rand::Rng;
use crate::Animation;
pub struct Char_action {
    pub screen_region: [f32; 4],
    pub sheet_region: [f32; 4],
    pub animations: Vec<Animation>,
    pub current_animation_index: usize,
    pub speed: f32,
    pub facing_left: bool,
    pub sprites_index: usize,

}

impl Char_action {

    pub fn new(screen_re: [f32; 4],
        sheet_re: [f32; 4],
        anims: Vec<Animation>,
        cur_anim_index: usize,
        spe: f32,
        facing_lef: bool,
        sprites_ind: usize,) -> Char_action {
            Self { screen_region: (screen_re), 
                sheet_region: (sheet_re),
                animations: (anims), 
                current_animation_index: (cur_anim_index),
                speed: (spe), 
                facing_left: (facing_lef), 
                sprites_index: (sprites_ind) }
    }

    pub fn walk(&mut self){
        if self.facing_left {
            self.screen_region[0] -= self.speed;
        }
        // if facing right
        else {
            self.screen_region[0] += self.speed;
        }
    }
    pub fn face_left(&mut self) {
        self.facing_left = true;
        self.animations[self.current_animation_index].apply_face_left();
        
    }

    pub fn face_right(&mut self) {
        self.facing_left = false;
        self.animations[self.current_animation_index].apply_face_right();
    }
    pub fn move_down(&mut self) {
        self.screen_region[1] -= self.speed;

        if self.screen_region[1] <= 0.0 {
            self.screen_region[1] = 768.0;
            self.screen_region[0] = rand::thread_rng().gen_range(0..1025) as f32;
        }
    }

    pub fn travel_down(&mut self){
        self.screen_region[1] -= self.speed;
    }

    pub fn travel_up(&mut self){
        self.screen_region[1] += self.speed;
    }

    pub fn move_right(&mut self) {
        self.screen_region[0] += self.speed;

        if self.screen_region[0] >= 1024.0 {
            self.screen_region[0] = 0.0;
            self.screen_region[1] = rand::thread_rng().gen_range(0..500) as f32;
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

    pub fn get_current_animation_state(&mut self)  -> [f32; 4]{
        if (self.facing_left) {
            self.animations[self.current_animation_index].apply_face_left();
        }
        else {
            self.animations[self.current_animation_index].apply_face_right();
        }
        return self.animations[self.current_animation_index].get_current_state();
        
    }
}