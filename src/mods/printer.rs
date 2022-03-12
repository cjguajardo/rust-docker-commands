use crossterm::terminal::size;

//pub fn pl(text: &colored::ColoredString) {
//print!("{}", text);
//}
pub fn pln(text: &colored::ColoredString) {
    println!("{}", text);
}
pub fn plnc(text: &colored::ColoredString) {
    let (width, _) = size().unwrap();
    let text = text.to_string();
    let text_len = text.len() as u16;
    let padding = (width - text_len) / 2;
    let padding_str = " ".repeat(padding as usize);
    println!("{}{}", padding_str, text);
}
// print a line filled with a given character
pub fn plf(fill: &colored::ColoredString) {
    let (width, _) = size().unwrap();
    let padding_str = fill.to_string().repeat(width as usize);
    println!("{}", padding_str);
}
// print a menu item
pub fn plm(text: &colored::ColoredString) {
    let text = text.to_string();
    let padding_left_str = " ".repeat(4);
    println!("{}{}", padding_left_str, text);
}
