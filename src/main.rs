mod pb; // because now pb is a normal module

use prost::Message;
use pb::websocket::PushDataV3ApiWrapper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example binary
    let encoded = b"\n3spot@public.aggre.bookTicker.v3.api.pb@10ms@SOLUSDT\x1a\x07SOLUSDT0\x97\xbf\xd7\xff\x923\xda\x13 \n\x06216.69\x12\x06953.77\x1a\x05216.7\"\x07113.135";
    let book_ticker = PushDataV3ApiWrapper::decode(encoded.as_slice())?;
    println!("Book Ticker: {:?}", book_ticker);
    Ok(())
}

