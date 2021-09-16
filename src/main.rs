extern crate raster;
extern crate walkdir;

use std::{fs::{self}};
use walkdir::WalkDir;
use raster::{Color, Image, error::RasterError};

fn main(){

    stitch(
        "tests/in/", 
        "tests/out/", 
        "out.png", 
        0, 
        0, 
        32, 
        32, 
        1024, 
        1024).unwrap();

}

fn stitch(path_in: &str, path_out: &str, name_out: &str, offset_x: i32, offset_y: i32, in_w: i32, in_h: i32, out_w: i32, out_h: i32) -> Result<(), RasterError>{

    let mut image_out = Image::blank(out_w, out_h);
    let mut _image_opened = Image::blank(in_w, in_h);
    raster::editor::fill(&mut image_out, Color::rgba(255, 0, 255, 255))?;

    let mut position = (0 + offset_x, 0 + offset_y);
    let parts_count = WalkDir::new(path_in).into_iter().count();
    let parts_max = (out_w * out_h) / (in_w * in_h); 

    for entry in fs::read_dir(path_in)? {

        let entry = entry?;
        let path = entry.path();
        let path_str = path.as_os_str().to_string_lossy();
        let words: Vec<&str> = path_str.split('/').collect();
        let file_name= words.last().unwrap();

        println!("proccessing : {}", &file_name);
        
        _image_opened = raster::open(&path_str).unwrap();
        if _image_opened.height == in_h && _image_opened.width == in_w {

            image_out = match raster::editor::blend(
                &image_out, 
                &_image_opened, 
                raster::BlendMode::Normal, 
                1.0, 
                raster::PositionMode::TopLeft, 
                position.0, 
                position.1){
                    Ok(i) => { i },
                    Err(_err) => { 
                        println!("Failed to process file : {}", &file_name);
                        continue;
                    }
                };
    
            if position.0 + in_w*2 + offset_x > image_out.width {
                if position.1 + in_h*2 + offset_y > image_out.height {
                    // reach end of image
                    break;
                } else {
                    position.1 += in_h + offset_y;
                }

                position.0 = 0 + offset_x;
            } else {

                position.0 += in_w + offset_x;
            }



        } else {

            println!("incorrect dimensions for image : {}", &file_name);
        }
        
        
    }

    println!("PARTS : {:?}, MAX : {:?}", parts_count, parts_max);
    raster::save(&image_out, &format!("{}{}", path_out, name_out))
}
