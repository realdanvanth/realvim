#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
extern crate crossterm;
use crossterm::cursor::SetCursorStyle::SteadyBar;
use crossterm::event::{poll, read, EnableMouseCapture,DisableMouseCapture, Event, KeyCode, MouseButton, MouseEvent, MouseEventKind};
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{ExecutableCommand, QueueableCommand, cursor, execute, queue, terminal};
use proctitle::set_title;
use ropey::Rope;
use std::fs::File;
use std::{io, panic};
use std::io::{BufReader, BufWriter, Write, stdout};
use std::path::Path;
use std::time::Duration;
use syntect::easy::HighlightLines;
use syntect::highlighting;
use syntect::highlighting::Theme;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxReference;
use syntect::parsing::SyntaxSet;
use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};
use syntect_assets::assets::HighlightingAssets;
use std::env;
#[macro_export]
macro_rules! eterm{
    (clear) => {stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap()};
    (clearline($x:expr,$y:expr))=>{
        eterm!(move($x,0));
        execute!(stdout(),Print(" ".repeat(($y).into()))).unwrap();
    };
    (moveleft)=>{stdout().execute(cursor::MoveLeft(1)).unwrap()};
    (steadybar)=>{stdout().execute(cursor::SetCursorStyle::SteadyBar).unwrap()};
    (steadyblock)=>{stdout().execute(cursor::SetCursorStyle::SteadyBlock).unwrap()};
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
macro_rules! qterm{
    (clear) => {stdout().queue(terminal::Clear(terminal::ClearType::All)).unwrap()};
    (clearline($x:expr,$y:expr))=>{
        eterm!(move($x,0));
        queue!(stdout(),Print(" ".repeat(($y).into()))).unwrap();
    };
    (moveleft)=>{stdout().queue(cursor::MoveLeft(1)).unwrap()};
    (steadybar)=>{stdout().queue(cursor::SetCursorStyle::SteadyBar).unwrap()};
    (steadyblock)=>{stdout().queue(cursor::SetCursorStyle::SteadyBlock).unwrap()};
    (bold)  => {stdout().queue(SetAttribute(Attribute::Bold))};
    (blink) => {stdout().queue(cursor::EnableBlinking)};
    (dblink) => {stdout().queue(cursor::DisableBlinking)};
    (hide) => {stdout().queue(cursor::Hide).unwrap()};
    (show) => {stdout().queue(cursor::Show).unwrap()};
    (underline) => {stdout().queue(SetAttribute(Attribute::underline))};
    (move($x:expr,$y:expr))=> {stdout().queue(cursor::MoveTo($y,$x)).unwrap()};
    (color(fg,$color:ident)) =>{stdout().queue(SetForegroundColor(Color::$color)).unwrap()};
    (color(bg,$color:ident)) =>{stdout().queue(SetBackgroundColor(Color::$color)).unwrap()};
    (print($string:literal)) => {stdout().queue(Print($string.to_string())).unwrap()};
    (poll($time:expr)) => {poll(Duration::from_millis($time)).unwrap()};
}
macro_rules! input {
    ($t:ty) => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<$t>().unwrap()
    }};
}
macro_rules! whichtype {
    ($i:ident) => {
        std::any::type_name_of_val(&$i)
    };
}
#[derive(Debug)]
enum Mode {
    Logo,
    Insert,
    Visual,
    Command,
}
#[derive(Debug)]
enum Scroll{
    Up,
    Down,
}
#[derive(Debug)]
struct Term {
    trows: u16,
    tcols: u16,
    mode: Mode,
    path: String,
    text: Rope,
    line: usize,
    htext:Vec<String>,
    cx: u16, 
    cy: u16,
    theme: Theme,
    ps: SyntaxSet,
    syntax: SyntaxReference,
}
fn main() {
    //let mode:Mode = Mode::Logo;
    set_title("realvim");
    let assets = HighlightingAssets::from_binary();
    let ps = SyntaxSet::load_defaults_newlines();
    let (cols,rows) = eterm!(size);
    let mut terminal = Term {
        trows: rows,
        tcols: cols,
        mode: Mode::Logo,
        path: String::from("No File"),
        text: Rope::new(),
        line: 0,
        cx: 0,
        cy: 0,
        theme: assets.get_theme("Visual Studio Dark+").clone(),
        syntax: ps.find_syntax_by_extension("rs").unwrap().clone(),
        ps: SyntaxSet::load_defaults_newlines(),
        htext:Vec::new(),
    };
    logo(&mut terminal);
    //println!("{:?}",terminal);
    //visual(&mut rows,&mut cols,"sample.txt");
    //command(&mut rows,&mut cols);
}
fn clearup() {
    eterm!(clear);
    eterm!(show);
    eterm!(unraw);
    eterm!(move(0,0));
    execute!(std::io::stdout(),EnableMouseCapture).unwrap();
    std::process::exit(0);
}
fn displaybar(terminal: &Term) {
    eterm!(move(terminal.trows-2,0));
    eterm!(color(bg, DarkGreen));
    eterm!(color(fg, Black));
    execute!(stdout(), Print(" ".repeat(terminal.tcols.into()))).unwrap();
    eterm!(move(terminal.trows-2,0));
    match terminal.mode {
        Mode::Command => {
            eterm!(print("COMMAND "));
        }
        Mode::Visual => {
            eterm!(print("VISUAL "));
        }
        Mode::Insert => {
            eterm!(print("INSERT "));
        }
        _ => {
            eterm!(print("LOGO   "));
        }
    };
    eterm!(move(terminal.trows-2,terminal.tcols-terminal.path.len()as u16));
    execute!(stdout(), Print(&terminal.path)).unwrap();
    eterm!(color(bg, Black));
    eterm!(color(fg, White));
}
fn visual(terminal: &mut Term) {
    eterm!(steadyblock);
    eterm!(show);
    eterm!(clear);
    terminal.mode = Mode::Visual;
    /*while line<terminal.text.len_lines()&&c<terminal.trows-2 {
        eterm!(move(c,0));
        eterm!(color(fg,DarkGreen));
        execute!(stdout(),Print(format!(" {:>width$} ",line,width=padding))).unwrap();
        eterm!(color(fg,White));
        execute!(stdout(),Print(format!("{}",terminal.text.line(line)))).unwrap();
        line+=1;
        c+=1;
    }*/
    inittext(terminal);
    displaytext(terminal);
    displaybar(&terminal);
    let padding = (terminal.line as usize + terminal.trows as usize - 2)
        .to_string()
        .len()
        + 2;
    let mut cx: i32 = terminal.cx as i32;
    let mut cy: i32 = terminal.cy as i32;
    if terminal.cy < padding as u16 {
        terminal.cy = padding as u16
    }
    eterm!(move(terminal.cx,terminal.cy));
    execute!(std::io::stdout(),EnableMouseCapture).unwrap();
    loop {
        if eterm!(poll(50)) {
            match read().unwrap() {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('a') {
                        cy -= 1;
                    } else if event.code == KeyCode::Char('s') {
                        cx += 1;
                    } else if event.code == KeyCode::Char('w') {
                        cx -= 1;
                    } else if event.code == KeyCode::Char('d') {
                        cy += 1
                    } else if event.code == KeyCode::Char('i') {
                        todo!();
                    } else if event.code == KeyCode::Char(':') {
                        command(terminal);
                    }
                    if cy < padding as i32 {
                        cy = padding as i32;
                        cx -= 1;
                    }
                    if cy > terminal.tcols.into() {
                        cx += 1;
                        cy = 0;
                    }
                    if cx < 0 {
                        if terminal.line > 0 {
                            terminal.line -= 1;
                            //mod syntax display text

                            modsyntax(terminal,Scroll::Up);
                            displaytext(terminal);
                            displaybar(&terminal);
                        }
                        cx = 0;
                    }
                    if cx > (terminal.trows - 3).into() {
                        if usize::from(terminal.line as u16 + (terminal.trows - 2))
                            < terminal.text.len_lines()
                        {
                            terminal.line += 1;
                            terminal.cx = terminal.trows - 3;
                            //modsyntax disp text
                            modsyntax(terminal,Scroll::Down);
                            displaytext(terminal);
                            displaybar(&terminal);
                        }
                        cx = terminal.trows as i32 - 3;
                    }
                    terminal.cx = cx as u16;
                    terminal.cy = cy as u16;
                    eterm!(move(terminal.cx,terminal.cy));
                }
                Event::Mouse(event)=>{
                    if event.kind == MouseEventKind::ScrollDown{
                        cx += 1; 
                    }
                    else if event.kind == MouseEventKind::ScrollUp{
                        cx -= 1;
                    }
                    if cx < 0 {
                        if terminal.line > 0 {
                            terminal.line -= 1;
                            //mod syntax display text

                            modsyntax(terminal,Scroll::Up);
                            displaytext(terminal);
                            displaybar(&terminal); 
                        }
                        cx = 0;
                    }
                    if cx > (terminal.trows - 3).into() {
                        if usize::from(terminal.line as u16 + (terminal.trows - 2))
                            < terminal.text.len_lines()
                        {
                            terminal.line += 1;
                            terminal.cx = terminal.trows - 3;
                            //modsyntax disp text
                            modsyntax(terminal,Scroll::Down);
                            displaytext(terminal);
                            displaybar(&terminal);
                        }
                        cx = terminal.trows as i32 - 3;
                    }
                    terminal.cx = cx as u16;
                    terminal.cy = cy as u16;
                    eterm!(move(terminal.cx,terminal.cy+padding as u16));
                    //eterm!(clear);
                    //execute!(stdout(),Print(format!("{:?}",event.kind)));
                },
                Event::Resize(width, height) => {
                    if height <= 16 || width <= 50 {
                        clearup();
                    }
                    terminal.tcols = width;
                    terminal.trows = height;
                    visual(terminal);
                }
                _ => {
                    continue;
                }
            }
        }
    }
}
fn modsyntax(terminal:&mut Term,dir:Scroll){
    let mut h = HighlightLines::new(&terminal.syntax,&terminal.theme);
    match dir{
        Scroll::Up => {
            terminal.htext.pop();
            let code = terminal.text.line(terminal.line).to_string();
            let ranges: Vec<(Style, &str)> = h.highlight_line(&code, &terminal.ps).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            terminal.htext.insert(0,escaped);
        },
        Scroll::Down =>{
            terminal.htext.remove(0);
            let code = terminal.text.line(terminal.line+terminal.cx as usize).to_string();
            let ranges: Vec<(Style, &str)> = h.highlight_line(&code, &terminal.ps).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            terminal.htext.push(escaped);
        },
    };
}
fn inittext(terminal: &mut Term) {
    let mut h = HighlightLines::new(&terminal.syntax, &terminal.theme);
    let mut c: u16 = 0;
    let mut line = terminal.line;
    qterm!(hide);
    while line < terminal.text.len_lines() && c < terminal.trows - 2 {                                                   
        let code = terminal.text.line(line).to_string();
        let ranges: Vec<(Style, &str)> = h.highlight_line(&code, &terminal.ps).unwrap();
        terminal.htext.push(as_24_bit_terminal_escaped(&ranges[..], false)); 
        line += 1;
        c += 1;
    }
}
fn displaytext(terminal:&Term){ 
    qterm!(clear);
    let mut c: usize = 0;
    let padding = (terminal.line as usize + terminal.trows as usize - 2)
        .to_string()
        .len();
    let mut line = terminal.line;
    qterm!(hide);
    while c<terminal.htext.len(){
        qterm!(move(c.try_into().unwrap(),0));
        qterm!(color(fg, DarkGreen));
        queue!(
            stdout(),
            Print(format!(" {:>width$} ", line, width = padding))
        )
        .unwrap();
        qterm!(color(fg, White));
        queue!(stdout(), Print(format!("{}", terminal.htext[c]))).unwrap();
        line += 1;
        c += 1;
    }
    qterm!(show);
    eterm!(flush);
}
fn command(terminal: &mut Term) {
    eterm!(steadybar);
    terminal.mode = Mode::Command;
    displaybar(&terminal);
    eterm!(show);
    eterm!(color(bg, Black));
    eterm!(color(fg, White));
    eterm!(clearline(terminal.trows - 1, terminal.tcols));
    eterm!(move(terminal.trows-1,0));
    eterm!(print(":"));
    let mut input = String::new();
    let mut cx: u16 = 0;
    loop {
        if eterm!(poll(50)) {
            match read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                        cx += 1;
                        eterm!(move(terminal.trows-1,cx));
                        execute!(stdout(), Print(c)).unwrap();
                    }
                    KeyCode::Backspace => {
                        if input.len() != 0 {
                            eterm!(move(terminal.trows-1,cx));
                            input.pop();
                            cx -= 1;
                            eterm!(print(" "));
                            eterm!(moveleft);
                        }
                    }
                    KeyCode::Esc => {
                        return;
                    }
                    KeyCode::Enter => {
                        eterm!(clearline(terminal.trows - 1, terminal.tcols));
                        break;
                    }
                    _ => {
                        continue;
                    }
                },
                Event::Resize(width, height) => {
                    if height <= 16 || width <= 50 {
                        clearup();
                    }
                    terminal.tcols = width;
                    terminal.trows = height;
                    command(terminal);
                }
                _ => {
                    continue;
                }
            }
        }
    }
    let commands: Vec<&str> = input.trim().split(' ').collect();
    match commands.as_slice() {
        ["q"] | ["q", _] => clearup(),
        ["o", path] => {
            terminal.text = Rope::from_reader(BufReader::new(match File::open(path) {
                Ok(file) => {
                    terminal.path = String::from(*path);
                    file
                }
                _ => {
                    eterm!(steadybar);
                    eterm!(move(terminal.trows-1,0));
                    execute!(
                        stdout(),
                        Print(format!("{}: is not a file", path.to_string()))
                    )
                    .unwrap();
                    return;
                }
            }))
            .unwrap();
            visual(terminal);
        },
        _ => {
            command(terminal);
        }
    };
}
fn logo(terminal: &mut Term) {
    eterm!(clear);
    terminal.mode = Mode::Logo;
    eterm!(hide);
    let rows: u16 = terminal.trows - 20;
    let cols: u16 = terminal.tcols - 50;
    eterm!(move(rows/2,cols/2));
    eterm!(raw);
    eterm!(color(fg, DarkBlue));
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
    eterm!(color(fg, Blue));
    eterm!(print(" open a file with :open FileName  "));
    eterm!(move(rows/2+11,cols/2+10));
    eterm!(print("󰈆 quit the editor with :q           "));
    //displaybar(&terminal);
    loop {
        if eterm!(poll(50)) {
            match read().unwrap() {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        clearup();
                    } else if event.code == KeyCode::Char(':') {
                        command(terminal);
                    }
                }
                Event::Resize(width, height) => {
                    if height <= 16 || width <= 50 {
                        clearup();
                    }
                    terminal.tcols = width;
                    terminal.trows = height;
                    logo(terminal);
                }
                _ => {
                    continue;
                }
            }
        }
    }
}
