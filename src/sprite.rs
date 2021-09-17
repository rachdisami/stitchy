use std::fs;
use std::{collections::HashMap, hash::Hasher};
use std::hash::Hash;

use raster::{Color, Image};
use exitfailure::ExitFailure;
use raster::error::RasterError;

#[derive(Clone)]
pub struct Sprite {
    id: u32,
    image: Image,
    width: i32,
    height: i32,
    path: String,
    position: (i32, i32)
}
impl Sprite{

    pub fn new(path_str: &str) -> Result<Sprite, ExitFailure> {

        let path = format!("{}", path_str);
        let image = raster::open(&path).unwrap();
        let width = image.width;
        let height = image.height;

        Ok(Sprite{
            id: 0,
            image,
            width,
            height,
            path,
            position: (0, 0)
        })
    }
}
impl Hash for Sprite {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl PartialEq for Sprite {
    fn eq(&self, other: &Sprite) -> bool {
        match raster::compare::equal(&self.image, &other.image){
            Ok(b) => {b},
            Err(_err) => {false}
        }
    }
}
impl Eq for Sprite {}

pub struct Spritesheet{
    sprites: HashMap<Sprite, bool>,
    image: Image,
    width: i32,
    height: i32,
    sprite_max_w: i32,
    sprite_max_h: i32,
    offset_x: i32,
    offset_y: i32,
    _bgc: Color,
    _name: String,
    path: String
}
impl Spritesheet{
    pub fn new(path_str: &str, name: &str, width: i32, height: i32, sprite_max_w: i32, sprite_max_h: i32, offset_x: i32, offset_y: i32,  rgba: raster::Color) -> Result<Spritesheet, ExitFailure> {

        let name = name;
        let _bgc = rgba.clone();
        let path = format!("{}{}.png", path_str, name);
        let width = width;
        let height = height;
        let mut image = Image::blank(width, height);
        let sprite_max_w = sprite_max_w;
        let sprite_max_h = sprite_max_h;
        let offset_x = offset_x;
        let offset_y = offset_y;

        let sprites = HashMap::new();

        raster::editor::fill(&mut image, rgba).unwrap();

        Ok(Spritesheet{
            sprites,
            image,
            width,
            height,
            sprite_max_w,
            sprite_max_h,
            offset_x,
            offset_y,
            _bgc,
            _name: name.to_string(),
            path
        })
    }

    pub fn insert(&mut self, sprite: Sprite){
        if sprite.width == self.sprite_max_w && sprite.height == self.sprite_max_h {
            self.sprites.insert(sprite, true);
        } else {
            self.sprites.insert(sprite, false);
        }
    }

    pub fn populate(&mut self, path_in: &str) -> Result<(), ExitFailure>{

        for entry in fs::read_dir(path_in)? {

            let entry = entry?;
            let path = entry.path();
            let path_str = path.as_os_str().to_string_lossy();
            let words: Vec<&str> = path_str.split('/').collect();
            let file_name= words.last().unwrap();

            self.insert(Sprite::new(&format!("{}{}", path_in, file_name))?);
        }

        Ok(())
    }

    pub fn _clear(&mut self){
        self.sprites = HashMap::new();
        raster::editor::fill(&mut self.image, self._bgc.clone()).unwrap();
    }

    pub fn stitch(&mut self) -> Result<(), RasterError>{

        let mut position = (0 + self.offset_x, 0 + self.offset_y);
        let mut updated_sprites = HashMap::<Sprite, bool>::new();

        for (sprite, proccess) in &self.sprites{

            if *proccess {

                self.image = match raster::editor::blend(
                    &self.image, 
                    &sprite.image, 
                    raster::BlendMode::Normal, 
                    1.0, 
                    raster::PositionMode::TopLeft, 
                    position.0, 
                    position.1){
                        Ok(i) => { i },
                        Err(_err) => { 
                            println!("Failed to process file : {}", &sprite.path);
                            continue;
                        }
                    };
                
                let mut new_sprite = sprite.clone();
                new_sprite.position = position;
                updated_sprites.insert(new_sprite, *proccess);
                
                if position.0 + sprite.width*2 + self.offset_x > self.width {
                    if position.1 + sprite.height*2 + self.offset_y > self.height {
                        // reach end of image
                        break;
                    } else {
                        position.1 += sprite.height + self.offset_y;
                    }
    
                    position.0 = 0 + self.offset_x;
                } else {
    
                    position.0 += sprite.width + self.offset_x;
                }
                
            }

            
        }

        self.sprites = updated_sprites;
        raster::save(&self.image, &self.path)

    }

    pub fn _get_sprites(&self) -> HashMap<String, (i32, i32)>{

        let mut sprites = HashMap::<String, (i32, i32)>::new();
        for (sprite, _) in &self.sprites {
            sprites.insert(sprite.path.clone(), sprite.position);
        }

        sprites
    }
}