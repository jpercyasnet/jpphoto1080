use std::path::Path;
use std::time::Instant;
use little_exif::metadata::Metadata;
use little_exif::exif_tag::ExifTag;
use image::ImageFormat::Jpeg;

pub fn rc_rotatepress (dir_value: String, mergescrol_value: String) -> (u32, String) {
     let mut errcode: u32 = 0;
     let mut errstring: String = "xx".to_string();
     let mut numrow = 0;
     let mut bolok = true;
     if Path::new(&dir_value).exists() {
         let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
         let mut lenmg1 = mergelistvec.len();
         if lenmg1 < 2 {
             errstring = "no images to rotate".to_string();
             errcode = 1;
         } else {
             lenmg1 = lenmg1 - 1;
             let start_time = Instant::now();
             for indl in 0..lenmg1 {
                let str_cur_dirfrom = dir_value.clone();
                let linestr = mergelistvec[indl];
                let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
                let filefromx = lineparse[0].to_string();
                let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx[1..];
                if !Path::new(&fullfrom).exists() {
                    errstring = format!("********* rotate problem: ERROR {} does not exist **********",fullfrom);
                    bolok = false;
                    errcode = 2;
                    break;
                } else {
                    let strval = lineparse[1].to_string();
                    let locind = strval.find("orientation");
                    if locind != None {
                        let start = locind.unwrap();
                        let start = start + 13;
                        let end = start + 1;
                        let getorient = strval.get(start..end);
                        let orient_int: i32 = getorient.unwrap().parse().unwrap_or(-99);
                        if orient_int > 0 {
                            if (orient_int == 3) | 
                               (orient_int == 6) |
                               (orient_int == 8) {
                                match image::open(&fullfrom) {   
                                    Ok(dyn_img) => {
 	                                    match Metadata::new_from_path(Path::new(&fullfrom)) {
                                            Ok(mut jpg_data) => {
                                                let mut dyn_img1 = dyn_img.clone(); 
                                                if orient_int == 3 {
                                                    dyn_img1 = dyn_img.rotate180();
                                                } else if orient_int == 6 {
                                                    dyn_img1 = dyn_img.rotate90();
                                                } else if orient_int == 8 {
                                                    dyn_img1 = dyn_img.rotate270();
                                                }
                                                let yy: Vec<u16> = vec![1];
                                                jpg_data.set_tag(ExifTag::Orientation(yy));
                                                match  dyn_img1.save_with_format(fullfrom.clone(), Jpeg) {
                                                    Ok(_okval)=> {
                                                        match jpg_data.write_to_file(Path::new(&fullfrom)) {
                                                            Ok(_okkval) => {
                                                                numrow = numrow + 1;
                                                            },
                                                            Err(errx) => {
                                                                errstring = format!("rotate ERROR save of {} metadata error of {}",fullfrom, errx);
                                                                bolok = false;
                                                                errcode = 3;
                                                                break;
                                                            }
                                                        }
                                                    },
                                                    Err(err) => {
                                                        errstring = format!("rotate ERROR {} save error of {}",fullfrom, err);
                                                        bolok = false;
                                                        errcode = 4;
                                                        break;
                                                    }
                                                }
                                            },
                                            Err(err) => {
                                                errstring = format!("rotate ERROR {} fail reading metadata of {}",fullfrom, err);
                                                bolok = false;
                                                errcode = 5;
                                                break;
                                            }
                                        } 
                                    },
                                    Err(err) => {
                                        errstring = format!("rotate ERROR {} fail reading image {}",fullfrom, err);
                                        bolok = false;
                                        errcode = 6;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
             }
             if bolok {
                 if numrow > 0 {
                     let diffx = start_time.elapsed();     
                     errstring = format!("rotated {} files in {} seconds", lenmg1, diffx.as_secs());
                     errcode = 0;
                 } else {
                     errstring = "no images to rotate".to_string();
                     errcode = 7;
                 }
             }
         }
     } else {
         errstring = "the directory does not exist".to_string();
         errcode = 8;
     }
     (errcode, errstring)
}

