use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
};
use serde::{Deserialize, Serialize};

use crate::resources::GlobalResources;

pub trait ItemBase {
    fn get_cost(&self) -> u32;
    fn get_level(&self) -> u8;
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_texture(
        &self,
        draw_handle: &mut RaylibDrawHandle,
        resources: &GlobalResources,
        dest: Rectangle,
    );
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StunGun {
    pub range: f32,
    pub duration: f64,
    pub level: u8,
    cost: u32,
}

impl StunGun {
    pub fn lvl1() -> Self {
        Self {
            range: 30.0,
            duration: 2.0,
            level: 1,
            cost: 15,
        }
    }
    pub fn lvl2() -> Self {
        Self {
            range: 60.0,
            duration: 2.5,
            level: 2,
            cost: 25,
        }
    }
    pub fn lvl3() -> Self {
        Self {
            range: 80.0,
            duration: 3.0,
            level: 3,
            cost: 40,
        }
    }
}

impl ItemBase for StunGun {
    fn get_cost(&self) -> u32 {
        self.cost
    }

    fn get_name(&self) -> String {
        return "Stun Bomb".to_string();
    }

    fn get_description(&self) -> String {
        return "Right click to stun enemies\n one use per dive".to_string();
    }

    fn get_texture(
        &self,
        draw_handle: &mut RaylibDrawHandle,
        resources: &GlobalResources,
        dest: Rectangle,
    ) {
        let texture = match self.get_level() {
            1 => (&resources.stun_gun_one),
            2 => (&resources.stun_gun_two),
            3 | _ => (&resources.stun_gun_three),
        };

        draw_handle.draw_texture_pro(
            texture,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: texture.width as f32,
                height: texture.height as f32,
            },
            dest,
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,
        );
    }
    fn get_level(&self) -> u8 {
        self.level
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AirBag {
    pub extra_oxygen: f32,
    pub level: u8,
    cost: u32,
}

impl AirBag {
    pub fn lvl1() -> Self {
        Self {
            extra_oxygen: 0.15,
            level: 1,
            cost: 25,
        }
    }
    pub fn lvl2() -> Self {
        Self {
            extra_oxygen: 0.30,
            level: 2,
            cost: 35,
        }
    }
    pub fn lvl3() -> Self {
        Self {
            extra_oxygen: 0.45,
            level: 3,
            cost: 50,
        }
    }
}

impl ItemBase for AirBag {
    fn get_cost(&self) -> u32 {
        self.cost
    }

    fn get_name(&self) -> String {
        return "Bag of Air".to_string();
    }

    fn get_description(&self) -> String {
        return "Its.. a bag.\nFilled with air. Duh".to_string();
    }

    fn get_texture(
        &self,
        draw_handle: &mut RaylibDrawHandle,
        resources: &GlobalResources,
        dest: Rectangle,
    ) {
        let texture = match self.get_level() {
            1 => (&resources.air_one),
            2 => (&resources.air_two),
            3 | _ => (&resources.air_three),
        };

        draw_handle.draw_texture_pro(
            texture,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: texture.width as f32,
                height: texture.height as f32,
            },
            dest,
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,
        );
    }
    fn get_level(&self) -> u8 {
        self.level
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Flashlight {
    pub radius: f32,
    pub level: u8,
    cost: u32,
}

impl Flashlight {
    pub fn lvl1() -> Self {
        Self {
            radius: 0.25,
            level: 1,
            cost: 20,
        }
    }
    pub fn lvl2() -> Self {
        Self {
            radius: 0.5,
            level: 2,
            cost: 30,
        }
    }
    pub fn lvl3() -> Self {
        Self {
            radius: 1.0,
            level: 3,
            cost: 50,
        }
    }
}

impl ItemBase for Flashlight {
    fn get_cost(&self) -> u32 {
        self.cost
    }

    fn get_name(&self) -> String {
        return "Flashlight".to_string();
    }

    fn get_description(&self) -> String {
        return "See better for longer".to_string();
    }

    fn get_texture(
        &self,
        draw_handle: &mut RaylibDrawHandle,
        resources: &GlobalResources,
        dest: Rectangle,
    ) {
        let texture = match self.get_level() {
            1 => (&resources.flashlight_one),
            2 => (&resources.flashlight_two),
            3 | _ => (&resources.flashlight_three),
        };

        draw_handle.draw_texture_pro(
            texture,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: texture.width as f32,
                height: texture.height as f32,
            },
            dest,
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,
        );
    }
    fn get_level(&self) -> u8 {
        self.level
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Flippers {
    pub speed_increase: f32,
    pub level: u8,
    cost: u32,
}

impl Flippers {
    pub fn lvl1() -> Self {
        Self {
            speed_increase: 1.1,
            level: 1,
            cost: 30,
        }
    }
    pub fn lvl2() -> Self {
        Self {
            speed_increase: 1.2,
            level: 2,
            cost: 40,
        }
    }
    pub fn lvl3() -> Self {
        Self {
            speed_increase: 1.3,
            level: 3,
            cost: 50,
        }
    }
}

impl ItemBase for Flippers {
    fn get_cost(&self) -> u32 {
        self.cost
    }

    fn get_name(&self) -> String {
        return "Flippers".to_string();
    }

    fn get_description(&self) -> String {
        return "Swim faster, and look stupid\nat the same time!".to_string();
    }

    fn get_texture(
        &self,
        draw_handle: &mut RaylibDrawHandle,
        resources: &GlobalResources,
        dest: Rectangle,
    ) {
        let texture = match self.get_level() {
            1 => (&resources.flippers_one),
            2 => (&resources.flippers_two),
            3 | _ => (&resources.flippers_three),
        };

        draw_handle.draw_texture_pro(
            texture,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: texture.width as f32,
                height: texture.height as f32,
            },
            dest,
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,
        );
    }
    fn get_level(&self) -> u8 {
        self.level
    }
}
