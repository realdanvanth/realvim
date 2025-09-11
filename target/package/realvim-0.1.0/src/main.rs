#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
extern crate crossterm;
use crossterm::{execute,ExecutableCommand,cursor,terminal};
use crossterm::terminal::{enable_raw_mode,disable_raw_mode};
use crossterm::event::{poll,read,KeyCode,Event};
use crossterm::style::{Print,SetBackgroundColor,SetForegroundColor,Color};
use std::io;
use std::io::{BufReader,BufWriter,stdout,Write};
use ropey::Rope;
use std::{time::Duration};
use std::fs::{File};
use std::path::Path;
#[macro_export]
macro_rules! eterm{
    (clear) => {stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap()}; 
    (clearline($x:expr,$y:expr))=>{
        eterm!(move($x,0));
        execute!(stdout(),Print(" ".repeat(($y).into()))).unwrap();
    };
    (flush) => {stdout().flush().unwrap()};
    (bold)  => {stdout().execute(SetAttribute(Attribute::Bold))};
    (size)  => {terminal::size().unwrap()};
    (raw)   => {enable_raw_mode().unwrap()};
    (unraw) => {disable_raw_mode().unwrap()};
    (blink) => {stdout().execute(cursor::EnableBlinking)};
    (dblink) => {stdout().execute(cursor::DisableBlinking)};
    (hide) => {stdout().execute(cursor::Hide).unwrap()};
    (show) => {stdout().execute(cursor::Show).unwrap()};
    (underline) => {stdout().execute(SetAttribute(Attribute::underline))};
    (move($x:expr,$y:expr))=> {stdout().execute(cursor::MoveTo($y,$x)).unwrap()};
    (color(fg,$color:ident)) =>{stdout().execute(SetForegroundColor(Color::$color)).unwrap()};
    (color(bg,$color:ident)) =>{stdout().execute(SetBackgroundColor(Color::$color)).unwrap()};
    (print($string:literal)) => {stdout().execute(Print($string.to_string())).unwrap()};
    (poll($time:expr)) => {poll(Duration::from_millis($time)).unwrap()};
}
macro_rules! input{
    ($t:ty) => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<$t>().unwrap()
    }};
}
macro_rules! whichtype{
    ($i:ident) => {
        std::any::type_name_of_val(&$i)
    };
}
#[derive(Debug)]
enum Mode{
    Logo,
    Insert,
    Visual,
    Command
}
#[derive(Debug)]
struct Term{
    trows:u16,
    tcols:u16,
    mode:Mode,
    path:String,
    cx:u16,
    cy:u16,
}
fn main(){
    //let mode:Mode = Mode::Logo;
    let (mut cols,mut rows) = eterm!(size);
    let mut terminal = Term{
        trows: rows,
        tcols: cols,
        mode :Mode::Logo,
        path :String::from("No File"),
        cx:0,
        cy:0,
    };
    logo(&mut terminal);
    //println!("{:?}",terminal);
    //visual(&mut rows,&mut cols,"sample.txt");
    //command(&mut rows,&mut cols);

}
fn clearup(){
    eterm!(clear);
    eterm!(show);
    eterm!(unraw);
    eterm!(move(0,0));
    std::process::exit(0);
}
fn displaybar(terminal:& Term){
    eterm!(move(terminal.trows-2,0));
    eterm!(color(bg,DarkGreen));
    eterm!(color(fg,Black)); 
    match terminal.mode{
        Mode::Command => {
             eterm!(print("COMMAND"));
        },
        Mode::Visual => {
             eterm!(print("VISUAL "));
        },
        Mode::Insert => {
            eterm!(print("INSERT  "));
        },
        _=> {
            eterm!(print("INVALID "));
        }

    };
    let offset = terminal.tcols - 8 - terminal.path.len() as u16;
    execute!(stdout(),Print(" ".repeat(offset.into()))).unwrap();
    eterm!(move(terminal.trows-2,terminal.tcols-terminal.path.len()as u16));
    execute!(stdout(),Print(terminal.path.clone())).unwrap();
}
fn visual(terminal:&mut Term,path:&str){ 
    let mut text = Rope::from_reader(BufReader::new(match File::open(path){
        Ok(file) => {
            terminal.path = path.to_string();
            file
        },
        _ => {
            eterm!(move(terminal.trows-1,0));
            eterm!(print("enter a valid file"));
            return;
        }
    })).unwrap();
    eterm!(clear);
    displaybar(&terminal);
    eterm!(color(fg,White));
    eterm!(color(bg,Black));
    eterm!(move(0,0));
    println!("{}",text.line(1));
}
fn command(terminal:&mut Term){
    //eterm!(clearline(terminal.trows-1,terminal.tcols));
    terminal.mode = Mode::Command;
    displaybar(&terminal);
    eterm!(move(terminal.trows-1,0)); 
    eterm!(color(bg,Black));
    eterm!(color(fg,White));
    eterm!(print(":"));
    let mut input = String::new();
    loop{
        if eterm!(poll(500))
        {
            match read().unwrap(){
            Event::Key(event)=>{
                match event.code {
                    KeyCode::Char(c)=>{
                        input.push(c);
                        eterm!(move(terminal.trows-1,(input.len()).try_into().unwrap()));
                        execute!(stdout(),Print(c));
                    },
                    KeyCode::Backspace=>{
                        eterm!(move(terminal.trows-1,(input.len()-1).try_into().unwrap()));
                        eterm!(print(" "));
                    }
                    KeyCode::Enter=>{
                        break;
                    },
                    _=>{
                        continue;
                    }
                }
            }
            Event::Resize(width,height) => {
                    if height<=16 || width<= 50{ 
                        clearup(); 
                    }
                    terminal.tcols=width;
                    terminal.trows=height;
                    logo(terminal);
                }
                _ => {continue;},
            }
        }
    }
    let commands:Vec<&str>=input.trim().split(' ').collect(); 
    match commands.as_slice(){
        ["q"]|["q",_] => {clearup()},
        ["o",path]=> {
            visual(terminal,path);
        },
        _=>{
            eterm!(clearline(terminal.trows-1,terminal.tcols));
            eterm!(clearline(terminal.trows-2,terminal.tcols));
            eterm!(clearline(terminal.trows-3,terminal.tcols));
            command(terminal);
        }
    };
}
fn logo(terminal:&mut Term){
    eterm!(hide);
    eterm!(clear); 
    let rows:u16 = terminal.trows- 20;
    let cols:u16 = terminal.tcols- 50;  
    eterm!(move(rows/2,cols/2));
    eterm!(raw);
    eterm!(color(fg,DarkBlue)); 
    print!("██████╗ ███████╗ █████╗ ██╗    ██╗   ██╗██╗███╗   ███╗");
    eterm!(move(rows/2+1,cols/2));
    print!("██╔══██╗██╔════╝██╔══██╗██║    ██║   ██║██║████╗ ████║");
    eterm!(move(rows/2+2,cols/2));
    print!("██████╔╝█████╗  ███████║██║    ██║   ██║██║██╔████╔██║");
    eterm!(move(rows/2+3,cols/2));
    print!("██╔══██╗██╔══╝  ██╔══██║██║    ╚██╗ ██╔╝██║██║╚██╔╝██║");
    eterm!(move(rows/2+4,cols/2));
    print!("██║  ██║███████╗██║  ██║███████╗╚████╔╝ ██║██║ ╚═╝ ██║");
    eterm!(move(rows/2+5,cols/2));
    println!("╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚══════╝ ╚═══╝  ╚═╝╚═╝     ╚═╝");
    eterm!(move(rows/2+10,cols/2+10));
    eterm!(color(fg,Blue));
    eterm!(print(" open a file with :open FileName  "));
    eterm!(move(rows/2+11,cols/2+10));
    eterm!(print("󰈆 quit the editor with :q      ")); 
    loop{
        if eterm!(poll(50)) {
            match read().unwrap(){
                Event::Key(event)=>{
                    if event.code==KeyCode::Char('q'){
                        clearup();
                    }
                    else if event.code==KeyCode::Char(':'){
                       command(terminal);
                    }
                }
                Event::Resize(width,height) => {
                    if height<=16 || width<= 50{ 
                        clearup(); 
                    }
                    terminal.tcols=width;
                    terminal.trows=height;
                    logo(terminal);
                }
                _ => {continue;},
            }
        }
    }
}

