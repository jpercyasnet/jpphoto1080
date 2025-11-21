mod get_dirlistc8;
mod get_dirlistr8;
// mod c8_diroutpress;
mod c8_copypress;
mod rc_rotatepress;
mod get_dirlistc;
mod fdox;
//mod mergepressx;
//mod copypressx;
//mod create_mergelist;
//mod parse_moddate;
//mod get_strvector;
mod dump_file;
mod get_dirlist;
//mod copyit;

//use mergepressx::mergepressx;
//use copypressx::copypressx;
//use create_mergelist::create_mergelist;
use get_dirlistc::get_dirlistc;
use get_dirlistc8::get_dirlistc8;
use get_dirlistr8::get_dirlistr8;
// use c8_diroutpress::c8_diroutpress;
use c8_copypress::c8_copypress;
use rc_rotatepress::rc_rotatepress;
use get_dirlist::get_dirlist;
//use copyit::copyit;

use crate::fdox::Fdfilter;
use crate::fdox::FdFileMessage;
use crate::fdox::FdFile;
use crate::fdox::fdview_controls;
use crate::fdox::fdempty_message;

use iced::widget::{Column, text, column, button, Row, row, Scrollable, Text,
                   text_input, Radio, horizontal_space, container, scrollable, checkbox};
use iced::{Element, Task, Length, Alignment, Color, Theme};

use std::path::Path;
use std::process::Command as stdCommand;

fn main() -> iced::Result {
     let widthxx: f32 = 1050.0;
     let heightxx: f32 = 650.0;
     iced::application(MainX::title, MainX::update, MainX::view)
        .window_size((widthxx, heightxx))
        .theme(|_| Theme::SolarizedDark)
        .run_with(MainX::new)

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageChoice {
    MNR,
    MNC,
    MNI,
//--------file dialog choice
    GD,
    OD,
 }

impl Default for PageChoice {
    fn default() -> Self {
        PageChoice::MNR
    }
}

struct MainX {
    mess_color: Color,
    msg_value: String,
    pagechoice_value: PageChoice,
    dir_value: String,
    outdir_value: String,
    scrol_value: String,
    dirset: u32,
    c8scrol_value: String,
    rcscrol_value: String,
// --- file dialog variables
    fdoutdir_value: String,
    fdfilter: Fdfilter,
    fdfiles: Vec<FdFile>,
    fdgetdiritems: bool,

}
#[derive(Clone, Debug)]
enum Message {
    PageRadioSelected(PageChoice),
// main program
    INStartButton,
    RCListPressed,
    RCRotallPressed,
    C8ListPressed,
    C8CopyPressed,
// --- file dialog messages
    FdSetDirPressed,
    FdListPressed,
    FdChgDirPressed,
    FdFilterChanged(Fdfilter),
    FdFileMessage(usize, FdFileMessage),
    FdGetDirItemsChk(bool),
    FdNewoutdir(String),
    FdChgDir2Pressed,
}

impl MainX {
    fn new() -> (Self, iced::Task<Message>) {
        (  MainX {
                mess_color: Color::from([0.5, 0.5, 1.0]),
                msg_value: "no message".to_string(),
                pagechoice_value: PageChoice::MNR,
                dir_value: "no directory".to_string(),
                outdir_value: "no directory".to_string(),
                dirset: 0,
                c8scrol_value: " nothing to process ".to_string(),
                rcscrol_value: " nothing to process ".to_string(),
                scrol_value: " No directory selected \n \
                            ".to_string(),
// --- file dialog variables
                fdoutdir_value: String::new(),
                fdfilter:Fdfilter::All,
                fdfiles:Vec::<FdFile>::new(),
                fdgetdiritems: false,
           },
            Task::none(),
        )

    }

    fn title(&self) -> String {
        String::from("Photo Rotate Convert 1080 - no deps")
    }

    fn update(&mut self, message: Message) -> Task<Message>  {
        match message {

            Message::PageRadioSelected(xchoice) => {
                let mut strx: String;  
                match xchoice {
                    PageChoice::MNR => {
                             strx = "Rotate Correction selected".to_string();
                             self.dirset = 0;
                    },
                    PageChoice::MNC => {
                             strx = "Convert to 1080 selected".to_string();
                             self.dirset = 0;
                    },
                    PageChoice::MNI => {
                             strx = "Individual Rotations selected".to_string();
                             self.dirset = 0;
                    },
                    PageChoice::GD => {
                             strx = "get directory selected".to_string();
                             self.dirset = 1;
                             let (errcd, errstr, newdir, listitems) = get_dirlistc(self.dir_value.clone(), self.fdgetdiritems.clone());
                             if errcd == 0 {
                                 self.fdfiles.clear();                         
                                 self.fdoutdir_value = newdir.to_string();
                                 let listitemlen = listitems.len();
                                 let newtoi = listitemlen as i32 ;
                                 for indexi in 0..newtoi {
                                      self.fdfiles.push(FdFile::new(listitems[indexi as usize].clone()));
                                 } 
                                 self.mess_color = Color::from([0.0, 1.0, 0.0]);
                             } else {
                                 self.mess_color = Color::from([1.0, 0.0, 0.0]);
                                 strx = errstr;
                             }
                    },
                    PageChoice::OD => {
                             strx = "out directory selected".to_string();
                             self.dirset = 2;
                             let mut pathlook = self.outdir_value.to_string();
                             if !Path::new(&self.outdir_value).exists() {
                                 if Path::new(&self.dir_value).exists() {
                                     pathlook = self.dir_value.to_string();
                                 }
                             }
                             let (errcd, errstr, newdir, listitems) = get_dirlistc(pathlook, self.fdgetdiritems.clone());
                             if errcd == 0 {
                                 self.fdfiles.clear();                         
                                 self.fdoutdir_value = newdir.to_string();
                                 let listitemlen = listitems.len();
                                 let newtoi = listitemlen as i32 ;
                                 for indexi in 0..newtoi {
                                      self.fdfiles.push(FdFile::new(listitems[indexi as usize].clone()));
                                 } 
                                 self.mess_color = Color::from([0.0, 1.0, 0.0]);
                             } else {
                                 self.mess_color = Color::from([1.0, 0.0, 0.0]);
                                 strx = errstr;
                             }
                    },
                };
                self.pagechoice_value = xchoice;
                self.mess_color = Color::from([0.0, 1.0, 0.0]);
                self.msg_value = strx.to_string();
                Task::none()
            }
// main program message
            Message::INStartButton => {
                if Path::new(&self.dir_value).exists() {
                    stdCommand::new("jpindrot")
                             .arg(&self.dir_value)
                             .spawn()
                             .expect("failed to execute process");
                    self.msg_value = "started jpindrot program".to_string();
                    self.mess_color = Color::from([0.0, 1.0, 0.0]);
                } else {
                    self.msg_value = "The directory does not exist".to_string();
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                }   
                Task::none()
            }
            Message::C8ListPressed => {
                self.c8scrol_value = " nothing to process ".to_string();
                if !Path::new(&self.dir_value).exists() {
                    self.msg_value = format!("directory does not exist: {}", self.dir_value);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                } else {
                    let dir_path = Path::new(&self.dir_value);
                    let (errcd, errstr, newliststr) = get_dirlistc8(dir_path.to_path_buf());
                    if errcd == 0 {
                        self.c8scrol_value  = newliststr.to_string();
                        self.msg_value = format!("directory entries for: {}", self.dir_value);
                        self.mess_color = Color::from([0.0, 1.0, 0.0]);
                    } else {
                        self.msg_value = errstr.to_string();
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    }
                }
                Task::none()
            }
            Message::C8CopyPressed => {
                let (errcode, errstr) = c8_copypress(self.dir_value.clone(), self.outdir_value.clone(), self.c8scrol_value.clone());
                self.msg_value = errstr.to_string();
                if errcode == 0 {
                    self.mess_color = Color::from([0.0, 1.0, 0.0]);
                } else {
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                }
                Task::none()
            }
            Message::RCListPressed => {
                self.rcscrol_value = " nothing to process ".to_string();
                if !Path::new(&self.dir_value).exists() {
                    self.msg_value = format!("directory does not exist: {}", self.dir_value);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                } else {
                    let dir_path = Path::new(&self.dir_value);
                    let (errcd, errstr, newliststr) = get_dirlistr8(dir_path.to_path_buf());
                    if errcd == 0 {
                        self.rcscrol_value  = newliststr.to_string();
                        self.msg_value = format!("directory entries for: {}", self.dir_value);
                        self.mess_color = Color::from([0.0, 1.0, 0.0]);
                    } else {
                        self.msg_value = errstr.to_string();
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    }
                }
                Task::none()
            }
            Message::RCRotallPressed => {
                if !Path::new(&self.dir_value).exists() {
                    self.msg_value = format!("directory does not exist: {}", self.dir_value);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    Task::none()
                } else {
                    let dir_path = Path::new(&self.dir_value);
                    let (errcd, errstr, newliststr) = get_dirlistr8(dir_path.to_path_buf());
                    if errcd == 0 {
                        let (errrc, errstrrc) = rc_rotatepress(self.dir_value.clone(), newliststr.clone());
                        self.msg_value = errstrrc.to_string();
                        if errrc == 0 {
                            self.mess_color = Color::from([0.0, 1.0, 0.0]);
//                            Task::perform(Rotatex::rotateit(self.dir_value.clone(), newliststr.clone(), self.tx_send.clone()), Message::RotatexFound)
                        } else {
                            self.mess_color = Color::from([1.0, 0.0, 0.0]);
                        }
                        Task::none()
                    } else {
                        self.msg_value = errstr.to_string();
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                        Task::none()
                    }
                }
            }
// -------------------------
// --- file dialog messages
            Message::FdFilterChanged(fdfilter) => {
                self.fdfilter = fdfilter;
                Task::none()
            }
            Message::FdFileMessage(i, fdfile_message) => {
                if let Some(fdfile) = self.fdfiles.get_mut(i) {
                    fdfile.update(fdfile_message);
                    Task::none()
                } else {
                    Task::none()
                }
            }
            Message::FdListPressed => {
                let (errcd, errstr, newdir, listitems) = get_dirlistc(self.fdoutdir_value.clone(), self.fdgetdiritems.clone());
                self.msg_value = errstr.to_string();
                if errcd == 0 {
                    self.fdfiles.clear();                         
                    self.fdoutdir_value = newdir.to_string();
                    let listitemlen = listitems.len();
                    let newtoi = listitemlen as i32 ;
                    for indexi in 0..newtoi {
                         self.fdfiles.push(FdFile::new(listitems[indexi as usize].clone()));
                    } 
                    self.mess_color = Color::from([0.0, 1.0, 0.0]);
                } else {
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                }
                Task::none()
            }
            Message::FdSetDirPressed => {
                let a_dir: String = self.fdoutdir_value.clone();
                if Path::new(&a_dir).exists() {
                    self.msg_value = format!("directory has been set with {}", a_dir);
                    if self.dirset == 1 {
                        self.dir_value = a_dir.clone();
                        let dir_path = Path::new(&a_dir);
                        let (errcd, errstr, newliststr) = get_dirlist(dir_path.to_path_buf());
                        if errcd == 0 {
                            self.scrol_value  = newliststr.to_string();
                            self.dir_value = a_dir.to_string();
                            self.mess_color = Color::from([0.0, 1.0, 0.0]);
                            self.pagechoice_value = PageChoice::MNR;
                        } else {
                            self.mess_color = Color::from([1.0, 0.0, 0.0]);
                            self.msg_value = errstr;
                        }
                    } else {
                        self.outdir_value = a_dir.clone();
                        self.mess_color = Color::from([0.0, 1.0, 0.0]);
                        self.pagechoice_value = PageChoice::MNC;
                    }
                } else {
                    self.msg_value = format!("directory {} does not exist", a_dir);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                }
                Task::none()
            }
            Message::FdGetDirItemsChk(chked) => {
                self.fdgetdiritems = chked;
                Task::none()
            } 
            Message::FdNewoutdir(value) => { self.fdoutdir_value = value; Task::none() }
            Message::FdChgDirPressed => {
                let files_selected = self.fdfiles.iter().filter(|fileitem| fileitem.fdcompleted).count();
                if files_selected < 1 {
                    self.msg_value = "no item selected".to_string();
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                } else if files_selected > 1 {
                    self.msg_value = "more than 1 item selected".to_string();
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                } else {
                    let mut itemstr: String = " ".to_string();
                    for filesy in self.fdfiles.iter() {
                         if filesy.fdcompleted {
                             itemstr = filesy.fddescription.clone();
                         }
                    }
                    let lineparse: Vec<&str> = itemstr[0..].split(" | ").collect();
                    if lineparse[0] != "DIR" {
                        self.msg_value = format!("{} is not a directory", itemstr);
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    } else {
                        let newdirx: String;
                        if lineparse[2] == "..parent" {
                            newdirx = lineparse[1].to_string();
                        } else {
                            newdirx = format!("{}/{}", self.fdoutdir_value, lineparse[1]);
                        }
                        let (errcd, errstr, newdir, listitems) = get_dirlistc(newdirx.clone(), self.fdgetdiritems.clone());
                        self.msg_value = errstr.to_string();
                        if errcd == 0 {
                            self.fdfiles.clear();                         
                            self.fdoutdir_value = newdir.to_string();
                            let listitemlen = listitems.len();
                            let newtoi = listitemlen as i32 ;
                            for indexi in 0..newtoi {
                                 self.fdfiles.push(FdFile::new(listitems[indexi as usize].clone()));
                            } 
                            self.mess_color = Color::from([0.0, 1.0, 0.0]);
                         } else {
                            self.mess_color = Color::from([1.0, 0.0, 0.0]);
                         }
                    }
                }
                Task::none()
            }
            Message::FdChgDir2Pressed => {
                if !Path::new(&self.fdoutdir_value).exists() {
                    self.msg_value = format!("out directory {} does not exist", self.fdoutdir_value);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                } else {
                    let (errcd, errstr, newdir, listitems) = get_dirlistc(self.fdoutdir_value.clone(), self.fdgetdiritems.clone());
                    self.msg_value = errstr.to_string();
                    if errcd == 0 {
                        self.fdfiles.clear();                         
                        self.fdoutdir_value = newdir.to_string();
                        let listitemlen = listitems.len();
                        let newtoi = listitemlen as i32 ;
                        for indexi in 0..newtoi {
                             self.fdfiles.push(FdFile::new(listitems[indexi as usize].clone()));
                        } 
                        self.mess_color = Color::from([0.0, 1.0, 0.0]);
                    } else {
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    }
                }
                Task::none()
            }
       }
    }

    fn view(&self) -> Element<Message> {
            let selected_pagechoice = Some(self.pagechoice_value);
            let mr = Radio::new(
                     "Rotate Correction",
                     PageChoice::MNR,
                     selected_pagechoice,
                     Message::PageRadioSelected,
            ).size(15);
            let mc = Radio::new(
                     "Convert to 1080",
                     PageChoice::MNC,
                     selected_pagechoice,
                     Message::PageRadioSelected,
            ).size(15);
            let mi = Radio::new(
                     "Individual Rotations",
                     PageChoice::MNI,
                     selected_pagechoice,
                     Message::PageRadioSelected,
            ).size(15);
// --- file dialog button
            let ub = Radio::new(
                     "Get Directory: ",
                     PageChoice::GD,
                     selected_pagechoice,
                     Message::PageRadioSelected,
            ).size(15);
            let uc = Radio::new(
                     "Get Output Directory: ",
                     PageChoice::OD,
                     selected_pagechoice,
                     Message::PageRadioSelected,
            ).size(15);

            let mut topshow = Column::new().spacing(5);
            topshow = topshow.push(container(row![text("Message:").size(20),
                                              text(&self.msg_value).size(20).color(*&self.mess_color),
                                              ].align_y(Alignment::Center).spacing(5).padding(5),
            ));
            topshow = topshow.push(container(row![
                                              mr, horizontal_space(), mc, horizontal_space(), mi
                                              ].align_y(Alignment::Center).spacing(5).padding(5),
            ));
            topshow = topshow.push(container(row![
                                              ub, text(&self.dir_value).size(15)
                                              ].align_y(Alignment::Center).spacing(5).padding(5),
            ));

            let mut subshow = Column::new().spacing(5);
//            let mut subshow = Column::new().spacing(5).align_x(Alignment::Center);

            if self.dirset > 0 {
                let controlsf = fdview_controls(&self.fdfiles, *&self.fdfilter);
                let filtered_files =
                    self.fdfiles.iter().filter(|file| self.fdfilter.matches(file));

                let mut filescol1 = Column::new().spacing(5);
                let mut n = 0;
                if filtered_files.clone().count() == 0 {
                    filescol1 = filescol1.push(container(row![fdempty_message(match self.fdfilter {
                        Fdfilter::All => "No directory selected or no files in directory",
                        Fdfilter::Active => "All files have been selected",
                        Fdfilter::Completed => "No files have been selected" 
                    })]));
                } else {
                    for filesy in self.fdfiles.iter() {
                         if filesy.fdcompleted {
                             if (self.fdfilter == Fdfilter::All) || (self.fdfilter == Fdfilter::Completed) {
                                 filescol1 = filescol1.push(container(row![filesy.view(n).map(move |message| {
                                    Message::FdFileMessage(n, message)
                                   })]));
                             }
                         } else {
                             if (self.fdfilter == Fdfilter::All) || (self.fdfilter == Fdfilter::Active) {
                                 filescol1 = filescol1.push(container(row![filesy.view(n).map(move |message| {
                                    Message::FdFileMessage(n, message)
                                   })]));
                             }
                         }
                         n = n + 1;
                    }
                }
                let mut filesrow = Row::new().spacing(5);
                filesrow = filesrow.push(container(filescol1).padding(5).width(Length::Fill));
//                filesrow = filesrow.push(container(filescol1).padding(5).width(Length::Fixed(400.0)));
//                filesrow = filesrow.push(container(filescol1).padding(10));

                let scrollable_contentf: Element<Message> =
                  Element::from(scrollable(
                    filesrow
                )
                .height(Length::Fill)
//                .width(Length::Fixed(500.0))
               .direction({
                    let scrollbar = scrollable::Scrollbar::new()
                        .width(10)
                        .margin(10)
                        .scroller_width(10);

                    scrollable::Direction::Both {
                        horizontal: scrollbar,
                        vertical: scrollbar,
                    }
                 })
                ); 
                   subshow = subshow.push(container(row![horizontal_space(),
                                                          checkbox("Get Directory Items", self.fdgetdiritems).on_toggle(Message::FdGetDirItemsChk),
                                                          horizontal_space(),
                                                          button("List Directory Button").on_press(Message::FdListPressed),
                                                          horizontal_space(),
                                                          button("Change Directory Button").on_press(Message::FdChgDirPressed), 
                                                          horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(5).padding(5),
                         ));
                    subshow = subshow.push(container(controlsf
                        ),
                           );
                    subshow = subshow.push(container(scrollable_contentf
                        ),
                           );
                    subshow = subshow.push(container(row![
                                                                         button("Set Directory Button").on_press(Message::FdSetDirPressed),
                                                                         text_input("No directory ...", &self.fdoutdir_value).on_input(Message::FdNewoutdir).padding(5).size(15),
                                                                         button("Change Directory").on_press(Message::FdChgDir2Pressed),
                                                                         
                                              ].align_y(Alignment::Center).spacing(20).padding(5),
                        ));
            }

//            let mut secshow = Column::new().spacing(5).align_x(Alignment::Center);
//            let mut thrshow = Column::new().spacing(5).align_x(Alignment::Center);
            let mut secshow = Column::new().spacing(5);
            let mut thrshow = Column::new().spacing(5);

            match self.pagechoice_value  {
// main program
                PageChoice::MNR => {
                    secshow = secshow.push(container(row![horizontal_space(),
                                                                         button("List Orientation Button").on_press(Message::RCListPressed),
                                                                         horizontal_space(),
                                                                         button("Rotate All Button").on_press(Message::RCRotallPressed), 
                                                                         horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
                         ));
                    secshow = secshow.push(container(Scrollable::new(
                        Column::new()
                           .width(Length::Fill)
                           .align_x(Alignment::Center)
                           .push(
                              Text::new(format!("{}",&self.rcscrol_value)),
                           )
                           ).height(Length::Fill),
                        ),
                           );
                },
                PageChoice::MNC => {
                    secshow = secshow.push(container(row![horizontal_space(),
//                                                                         button("Out Directory Button").on_press(Message::C8OutDirPressed),
                                                                         uc,
                                                                         text(&self.outdir_value).size(20), 
                                                                         horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
                        ));
                    secshow = secshow.push(container(row![horizontal_space(),
                                                                         button("List Directory Button").on_press(Message::C8ListPressed),
                                                                         horizontal_space(),
                                                                         button("Copy Button").on_press(Message::C8CopyPressed), 
                                                                         horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
                         ));
                    secshow = secshow.push(container(Scrollable::new(
                        Column::new()
                           .width(Length::Fill)
                           .align_x(Alignment::Center)
                           .push(
                              Text::new(format!("{}",&self.c8scrol_value)),
                           )
                           ).height(Length::Fill),
                        ),
                           );
                },
                PageChoice::MNI => {
                    secshow = secshow.push(container(row![horizontal_space(),
                                                                         button("Individual rotate start button").on_press(Message::INStartButton),
                                                                         horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
                        ));
                },
// --- file dialog setup
                PageChoice::GD => {
                    secshow = subshow;
                },
// --- file dialog setup
                PageChoice::OD => {
                    secshow = secshow.push(container(row![horizontal_space(),
//                                                                         button("Out Directory Button").on_press(Message::C8OutDirPressed),
                                                                         uc,
                                                                         text(&self.outdir_value).size(20), 
                                                                         horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
                        ));

                    thrshow = subshow;
                },
// --- end of file dialog
           }
        column![
         topshow,
         secshow,
         thrshow
         ]
         .padding(1)
        .into()
    }
}
