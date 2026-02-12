use rustvncserver::VncServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (server, mut events) = VncServer::new(1920, 1080, "PLC".into(), None);

    tokio::spawn(async move {
        while let Some(event) = events.recv().await {
            match event {
                rustvncserver::server::ServerEvent::ClientConnected { client_id } => {
                    println!("Client {client_id} connected");
                }
                rustvncserver::server::ServerEvent::ClientDisconnected { client_id } => {
                    println!("Client {client_id} disconnected");
                }
                _ => {}
            }
        }
    });

    let framebuffer = server.framebuffer().clone();

    tokio::spawn(async move {
        if let Err(e) = server.listen(5900).await {
            eprintln!("Error listening: {}", e);
        }
    });

    let mut frame = 0u32;
    let mut pixels = vec![0u8; (1920 * 1080 * 4) as usize];

    loop {
        for y in 0..1080 {
            for x in 0..1920 {
                let offset = (y * 1920 + x) * 4;

                // Animated gradient
                let r = ((x as u32 + frame) % 256) as u8;
                let g = ((y as u32 + frame) % 256) as u8;
                let b = ((frame / 2) % 256) as u8;

                pixels[offset] = r;
                pixels[offset + 1] = g;
                pixels[offset + 2] = b;
                pixels[offset + 3] = 255; // Alpha
            }
        }

        framebuffer
            .update_cropped(&pixels, 0, 0, 1920, 1080)
            .await
            .expect("Failed to update framebuffer");

        frame = frame.wrapping_add(1);
        tokio::time::sleep(std::time::Duration::from_millis(16)).await;

        if frame.is_multiple_of(300) {
            println!("Frame: {}", frame);
        }
    }
}
