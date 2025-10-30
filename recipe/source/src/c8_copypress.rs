use std::path::Path;
use std::fs;
use image::GenericImageView;
use image::ImageFormat::Jpeg;
use image::imageops::Lanczos3;
use image::{ImageBuffer, RgbaImage};
use image::imageops;
use std::time::Instant;

pub fn c8_copypress (dir_value: String, outdir_value: String, mergescrol_value: String) -> (u32, String) {
     let mut errcode: u32 = 0;
     let mut errstring: String = "no message".to_string();
     let mut numrow = 0;
     if Path::new(&dir_value).exists() {
         if Path::new(&outdir_value).exists() {
             let mut bolok = true;
             for entry1 in fs::read_dir(&outdir_value).unwrap() {
                  let entry = entry1.unwrap();
                  if let Ok(metadata) = entry.metadata() {
                      if let Ok(_file_name) = entry.file_name().into_string() {
                          if metadata.is_file() {
                              bolok = false;
                              break;
                          }
                      }
                  }
             }
             if bolok {
                 let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
                 let mut lenmg1 = mergelistvec.len();
                 if lenmg1 < 2 {
                     errstring = "no values in directory list".to_string();
                     errcode = 1;
                 } else {
                     let start_time = Instant::now();
                     let width = 1920;
                     let height = 1080;
                     let mut img: RgbaImage = ImageBuffer::new(width, height);
                     let red = 0 as u8;
                     let green = 0;
                     let blue = 0;
                     let alp = 0;
                     for x in 0..width {
                          for y in 0..height {
                               *img.get_pixel_mut(x, y) = image::Rgba([red, green, blue, alp]);
                          }
                     }
                     for x in 0..width {
                          *img.get_pixel_mut(x, 0) = image::Rgba([red, green, blue, alp]);
                          *img.get_pixel_mut(x, height-1) = image::Rgba([red, green, blue, alp]);
                     }
                     for y in 0..height {
                          *img.get_pixel_mut(0, y) = image::Rgba([red, green, blue, alp]);
                          *img.get_pixel_mut(width - 1, y) = image::Rgba([red, green, blue, alp]);
                     }
                     lenmg1 = lenmg1 - 1;
                     for indl in 0..lenmg1 {
                          let str_cur_dirfrom = dir_value.clone();
                          let linestr = mergelistvec[indl];
                          let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
                          let filefromx = lineparse[0].to_string();
                          let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx[1..];
                          if !Path::new(&fullfrom).exists() {
                              errstring = format!("convert error : ERROR {} does not exist",fullfrom);
                              errcode = 2;
                              bolok = false;
                              break;
                          }
                          let str_cur_dirout = outdir_value.clone();
                          let fullto = str_cur_dirout.clone() + "/" + &filefromx;
                          if Path::new(&fullto).exists() {
                              errstring = format!("convert Copy: ERROR {} already exists ", fullto);
                              errcode = 3;
                              bolok = false;
                              break;
                          }
                          match image::open(&fullfrom) {   
                              Ok(dyn_img) => {
                                  let (w,h) = dyn_img.dimensions();
                                  let mut newwidth: u32;
                                  let mut newheight: u32;
                                  if w > h {
                                      newwidth = 1920;
                                      newheight = 1920 * h / w;
                                      if newheight > 1080 {
                                          newheight = 1080;
                                          newwidth = 1080 * w / h;
                                      }
                                  } else {
                                      newheight = 1080;
                                      newwidth = 1080 * w / h;
                                  }
                                  let xloc = (1920 - newwidth) / 2;
                                  let yloc = (1080 - newheight) / 2;
                                  let dyn_img1 = image::imageops::resize(&dyn_img, newwidth, newheight, Lanczos3); 
                                  let mut img1 = img.clone();
                                  imageops::overlay(&mut img1, &dyn_img1, xloc.into(), yloc.into());
                                  let img_byte_vec = image::DynamicImage::ImageRgba8(img1).into_rgb8();
                                  match img_byte_vec.save_with_format(fullto.clone(), Jpeg) {
                                      Ok(_okval)=> {
                                          numrow = numrow + 1;
                                      },
                                      Err(err) => {
                                          errstring = format!("convert Copy: ERROR  saving {} of {}", fullto, err);
                                          errcode = 4;
                                          bolok = false;
                                          break;
                                      }
                                  }
                              },
                              Err(err) => {
                                  errstring = format!("convert Copy: ERROR opening {} of {}", fullfrom, err);
                                  errcode = 5;
                                  bolok = false;
                                  break;
                              }
                          }
                     }
                     if bolok {
                         let diffx = start_time.elapsed();     
                         errstring = format!("converted copied {} files in {} seconds", numrow, diffx.as_secs());
                         errcode = 0;
                     }
                 }
             } else {
                 errstring = "the output directory has files in it".to_string();
                 errcode = 6;
             }
         } else {
             errstring = "the output directory does not exist".to_string();
             errcode = 7;
         }
     } else {
         errstring = "the directory does not exist".to_string();
         errcode = 8;
     }
     (errcode, errstring)
}

