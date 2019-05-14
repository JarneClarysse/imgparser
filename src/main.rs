extern crate sdl2;
#[macro_use] extern crate simple_error;

use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Cursor};
use std::io::{self, BufReader};
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt;
use std::io::prelude::*;
use std::io::{Seek, SeekFrom};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use shuteye::sleep;
use std::time::Duration;
use std::str;
use std::str::FromStr;

#[derive(Clone)]
struct Pixel
{
    R: u32,
    G: u32,
    B: u32
}

struct Image
{
    width: u32,
    height: u32,
    pixels: Vec<Vec<Pixel>>
}

fn show_image(image: &Image)
{
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let display_mode = video_subsystem.current_display_mode(0).unwrap();

    let w = match display_mode.w as u32 > image.width {
        true => image.width,
        false => display_mode.w as u32
    };
    let h = match display_mode.h as u32 > image.height {
        true => image.height,
        false => display_mode.h as u32
    };
    
    let window = video_subsystem
        .window("Image", w, h)
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();
    let black = sdl2::pixels::Color::RGB(0, 0, 0);

    let mut event_pump = sdl.event_pump().unwrap();
    // render image
        canvas.set_draw_color(black);
        canvas.clear();

        for r in 0..image.height {
            for c in 0..image.width {
                let pixel = &image.pixels[image.height as usize - r as usize - 1][c as usize];
                canvas.set_draw_color(Color::RGB(pixel.R as u8, pixel.G as u8, pixel.B as u8));
                canvas.fill_rect(Rect::new(c as i32, r as i32, 1, 1)).unwrap();
            }
        }
        
        canvas.present();

    'main: loop 
    {        
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        sleep(Duration::new(0, 250000000));
    }
    
}

fn read_pixel(cursor: &mut Cursor<Vec<u8>>) -> Result<Pixel, Box<std::error::Error>>{

    let mut r = cursor.read_u8()?;
    let mut g = cursor.read_u8()?;
    let mut b = cursor.read_u8()?;

    let mut pixel = Pixel {
        R: r as u32,
        G: g as u32,
        B: b as u32
    };


    Ok(pixel)

}

fn read_num(cursor: &mut Cursor<Vec<u8>>) -> Result<u32, Box<std::error::Error>> {
    let mut v: Vec<u8> = vec![];
    let mut c:[u8; 1] = [0];




    //consume whitespace
    loop{
        cursor.read(&mut c)?;
        match &c {
           b" " | b"\t" | b"\n" => {},
            _ => { cursor.seek(std::io::SeekFrom::Current(-1)); break; }
        };
    };


    //read number
    loop{
        cursor.read(&mut c)?;
        match c[0] {
            b'0' ... b'9' => { v.push(c[0]);},
            b' '| b'\t' | b'\n' => { cursor.seek(std::io::SeekFrom::Current(-1)); break;}
            _ => {bail!("Parse error");}
        };
    };



    let num_str = std::str::from_utf8(&v)?;
    let num = num_str.parse::<u32>()?;

 

    Ok(num)

}
fn decode_ppm_image(cursor: &mut Cursor<Vec<u8>>) -> Result<Image, Box<std::error::Error>> {
    let mut image = Image {
        width: 0,
        height: 0,
        pixels: vec![]
    };
    //let mut buf2 = vec![];

    let mut header: [u8;2] = [0,2];
    cursor.read(&mut header);



    match &header {
        b"P6" => {println!("Header match"); },
        _ => {bail!("header mismatch"); }
    }

    println!("test");

    let mut c:[u8; 1] = [0];
    loop{
        cursor.read(&mut c)?;
        match &c {
            b"#" => {loop{
                cursor.read(&mut c)?;
                match &c {
                    b"\n" => {break;},
                    _ => {}
                };
            };},
            b" " | b"\t" | b"\n" => {},
            _ => {cursor.seek(std::io::SeekFrom::Current(-1)); break; }
        };
    };

    let w = read_num(cursor)?;


    loop{
        cursor.read(&mut c)?;
        match &c {
            b"#" => {loop{
                cursor.read(&mut c)?;
                match &c {
                    b"\n" => {break;},
                    _ => {}
                };
            };},
            b" " | b"\t" | b"\n" => {},
            _ => {cursor.seek(std::io::SeekFrom::Current(-1)); break; }
        };
    };


    let h = read_num(cursor)?;

    loop{
        cursor.read(&mut c)?;
        match &c {
            b"#" => {loop{
                cursor.read(&mut c)?;
                match &c {
                    b"\n" => {break;},
                    _ => {}
                };
            };},
            b" " | b"\t" | b"\n" => {},
            _ => {cursor.seek(std::io::SeekFrom::Current(-1)); break; }
        };
    };

    let max = read_num(cursor)?;

    loop{
        cursor.read(&mut c)?;
        match &c {
            b"#" => {loop{
                cursor.read(&mut c)?;
                match &c {
                    b"\n" => {break;},
                    _ => {}
                };
            };},
            b" " | b"\t" | b"\n" => {},
            _ => {cursor.seek(std::io::SeekFrom::Current(-1)); break; }
        };
    };

    println!("{}", h);
    println!("{}", w);
    println!("{}",max);

    let mut allePix: Vec<Vec<Pixel>> = vec![];



    loop{
        cursor.read(&mut c)?;
        match &c {
            b" " | b"\t" | b"\n" => {},
            _ => { cursor.seek(std::io::SeekFrom::Current(-1)); break; }
        };
    };

    for x in 0..h {
        let mut hoogte_pix: Vec<Pixel> = vec![];

        for x in 0..w {
            let pixel = read_pixel(cursor)?;
            hoogte_pix.push(pixel);
        }
        allePix.insert(0, hoogte_pix)
    }

	// TODO: Parse the image here
    image.width=w;
    image.height=h;
    image.pixels=allePix;


    //println!("{}",split_iter.next());



    Ok(image)
}

fn main() 
{
    let args: Vec<String> = std::env::args().collect();

    //println!("test");

    if args.len() < 2 {
        eprintln!("Syntax: {} <filename>", args[0]);
        return;
    };

    //println!("test");

    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path)    {
        Err(why) => panic!("Could not open file: {} (Reason: {})",
            display, why.description()),
        Ok(file) => file
    };

    // read the full file into memory. panic on failure
    let mut raw_file = Vec::new();
    file.read_to_end(&mut raw_file).unwrap();

    // construct a cursor so we can seek in the raw buffer
    let mut cursor = Cursor::new(raw_file);
    let mut image = match decode_ppm_image(&mut cursor) {
        Ok(img) => img,
        Err(why) => panic!("Could not parse PPM file - Desc: {}", why.description()),
    };

    show_image(&image);
}