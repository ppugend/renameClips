use std::{process, fs};
use std::path::PathBuf;
use std::fs::File;
use std::{io};
use std::io::prelude::*;
use std::fmt;
use native_dialog::{FileDialog, MessageDialog, MessageType};
use serde_json::{Value};
use std::path::Path;
//use std::ffi::OsStr;
use regex::Regex;
use rand::Rng;

struct ClipInfo {
    title: String,
    category: String,
    created_time: String,
    creator_name: String,
    file_path_name: String,
}
impl Default for ClipInfo {
    fn default() -> ClipInfo {
        ClipInfo {
            title: "".to_string(),
            category: "".to_string(),
            created_time: "".to_string(),
            creator_name: "".to_string(),
            file_path_name: "".to_string(),
        }
    }
}
impl fmt::Display for ClipInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}, Creator-{}, Created on {}", 
            self.title.replace("\"", ""),
            self.category.replace("\"", ""),                            
            self.creator_name.replace("\"", ""),
            self.created_time.replace("\"", ""),
    )}
}


fn exit_program(msg:String,need_pause:bool){
    if need_pause {
        pause(msg);
    }
    let _machine_kind =  if cfg!(windows){
        process::exit(0);
      } else if cfg!(unix) || cfg!(osx) || cfg!(linux) {
        process::exit(0x0100);
      } else {
        process::exit(0x0100);
      };
    
}

fn pause(msg:String) {
    let mut stdin = io:: stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "{}\nPress any key...",msg).unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn join_file_full_path(json_path:&String, clip_info:&ClipInfo, needs_incremental_suffix:bool) -> (PathBuf, PathBuf) {
    let unallow_chars_regex = Regex::new(r#"["/?<*:|\\\[\]<>]"#).unwrap();            

    let _path = Path::new(json_path).parent().unwrap();
    let _file_stem = Path::new(json_path).file_stem().unwrap();
    let _file_ext = Path::new(clip_info.file_path_name.as_str()).extension().unwrap();
    
    //let mut old_path = Path::new(_path).join(clip_info.file_path_name.to_string().replace("\"", ""));
    let mut old_path = Path::new(_path).join(clip_info.file_path_name.to_string());
    old_path.set_extension(_file_ext);

    let mut new_path;
    if needs_incremental_suffix {
        let mut rng = rand::thread_rng();
        let random_value = rng.gen::<u32>();
        new_path = Path::new(_path).join(unallow_chars_regex.replace_all(&clip_info.to_string(), "_").to_string()+"_" + &random_value.to_string());
        //println!("{}",new_path.to_string_lossy());
    }else{
        new_path = Path::new(_path).join(unallow_chars_regex.replace_all(&clip_info.to_string(), "_").to_string());
    }
    new_path.set_extension(_file_ext);
    //println!("{}",new_path.to_string_lossy());

    //remove double quotation
    let _o = old_path.to_string_lossy().replace("\"", "");
     old_path = Path::new(&_o).to_path_buf();
     let _n = new_path.to_string_lossy().replace("\"", "");
     new_path = Path::new(&_n).to_path_buf();

    return (old_path,new_path);
}

fn rename_process(json_fullpath:PathBuf){
    //println!("{}",&format!("{:#?}", jsonPath));
    
    let json_fullpath_string = &format!("{:#?}", json_fullpath);
    
    let mut f = File::open(json_fullpath).expect("file open error");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");
    
    
    let v: Value  = serde_json::from_str(&contents).expect("parse error");
    //println!("{}", v[0]["id"]);
    //println!("{}", v.as_array().unwrap().len());
    let array_from_json = v.as_array().unwrap();
    let array_length = array_from_json.len();
    if array_length > 0 {  

    
        for n in 0..array_length {            
            let mut clip_info = ClipInfo::default();
            clip_info.title = array_from_json[n]["title"].to_string();
            clip_info.category = array_from_json[n]["gamename"].to_string();
            clip_info.created_time = array_from_json[n]["created_at"].to_string();
            clip_info.creator_name = array_from_json[n]["creator_name"].to_string();
            clip_info.file_path_name = array_from_json[n]["filename"].to_string();
            clip_info.file_path_name = clip_info.file_path_name.replace("%7C", "_");
            clip_info.file_path_name = clip_info.file_path_name.replace("%7c", "_");
            
            //println!("{}",clip_info.to_string());

            let mut paths = join_file_full_path(&json_fullpath_string, &clip_info, false);
            
            //change new filename if the filename is exist.
            //println!("{}", Path::new(&paths.1.to_string_lossy().replace("\"", "")).exists());
            if Path::new(&paths.1.to_string_lossy().replace("\"", "")).exists() {
                paths = join_file_full_path(&json_fullpath_string, &clip_info, true);
            }

            if Path::new(&paths.0).exists() {
                fs::rename(paths.0, paths.1);
            }else {
                println!("src file is not exist {}",(paths.0).to_string_lossy());
            }
            

        }   
       
    }

}

fn main(){
    println!("Rename twitch clips automatically!");

    let path = FileDialog::new()
    .set_location("~/Desktop")
    .add_filter("Exported JSON Data File", &["json"])
    .show_open_single_file()
    .unwrap();

    let path = match path {
        Some(path) => path,
        None => return,
    };
    
    let yes = MessageDialog::new()
    .set_type(MessageType::Info)
    .set_title("Do you want to rename files?")
    .set_text(&format!("{:#?}", path))
    .show_confirm()
    .unwrap();

    if yes {
        rename_process(path);

        let end_message = "process completed";
        MessageDialog::new()
            .set_type(MessageType::Info)
            .set_text(end_message)
            .show_alert()
            .unwrap();
        
        exit_program(end_message.to_owned(),false);
    }else {
        //
    }
}
