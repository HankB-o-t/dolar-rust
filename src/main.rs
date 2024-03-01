use serde::{Serialize, Deserialize};
use console_engine::screen::Screen;
use console_engine::pixel;

#[derive(Debug, Serialize, Deserialize)]
struct Dolar {
    compra: f32,
    venta: f32,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
/*
 ____ ____ ____ ____ 
||D |||l |||. |||B ||
||__|||__|||__|||__||
|/__\|/__\|/__\|/__\|

 */

    println!(" ____ ____ ____ ____ ");
    println!("||D |||l |||. |||B ||");
    println!("||__|||__|||__|||__||");
    println!("|/__\\|/__\\|/__\\|/__\\|");
    
    let mut scr = Screen::new(20,6);

    // -----------------API----------------------------
    let info_dolar: Vec<Dolar> = reqwest::Client::new()
        .get("https://dolarapi.com/v1/dolares")
        .send()
        .await?
        .json()
        .await?;

    let precioco = info_dolar[1].compra;
    let preciove = info_dolar[1].venta;
    let precioc = precioco.to_string();
    let preciov = preciove.to_string();
    // -------------------------------------------------

    scr.rect(1, 1, 16, 4, pixel::pxl('#'));
    scr.print(2, 2, "Compra: $");
    scr.print(2, 3, "Venta: $");
    scr.print(11, 2, precioc.as_str());
    scr.print(10, 3, preciov.as_str());
    
    scr.draw();
    Ok(())
}