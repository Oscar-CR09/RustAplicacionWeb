use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // Escuchar en el puerto 8081
    let address = "127.0.0.1:8081";
    let listener = TcpListener::bind(&address).unwrap();
    println!("Servidor iniciado en: http://{}", address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Convertimos el buffer a String para facilitar la comparación
    let request = String::from_utf8_lossy(&buffer[..]);

    // Imprimimos la petición para depurar
    // println!("Request: {}", request);

    // Verificamos si piden la raíz "/" o "/index.html"
    let is_root = request.starts_with("GET / HTTP/1.1");
    let is_index = request.starts_with("GET /index.html HTTP/1.1");

    if is_root || is_index {
        send_file(stream, "index.html", "HTTP/1.1 200 OK");
    } else {
        send_file(stream, "404.html", "HTTP/1.1 404 NOT FOUND");
    }
}

// Unifiqué tus funciones send_index y send_not_found en una sola más flexible
fn send_file(mut stream: TcpStream, filename: &str, status_line: &str) {
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            // Si el archivo (ej. 404.html) tampoco existe, devolvemos un HTML de emergencia
            String::from("<!DOCTYPE html><html><body><h1>Error de Servidor</h1><p>Archivo no encontrado.</p></body></html>")
        }
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}