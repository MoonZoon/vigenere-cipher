use moon::*;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Vigenère Cipher")
        // @TODO add `.icon`?
        // @TODO add `public_url` (copy-paste from Zoon (?))
        .append_to_head(r#"<link rel="icon" href="/_api/public/rustacean-flat-noshadow.svg">"#)
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
