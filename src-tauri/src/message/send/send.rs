use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt};
use tokio::io::{self, AsyncBufReadExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Подключаемся к серверу (одно подключение на всю программу)
    let (mut ws, _) = connect_async("ws://127.0.0.1:8080").await?;
    println!("✅ Подключились к серверу");
    println!("💡 Вводи сообщения (пустая строка для выхода):");

    // НЕ разделяем поток - работаем с целым WebSocket
    let (mut write, mut read) = ws.split();

    // Задача для чтения сообщений от сервера
    let read_handle = tokio::spawn(async move {
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => println!("📥 Сервер: {}", msg),
                Err(e) => eprintln!("❌ Ошибка: {}", e),
            }
        }
    });

    // Чтение ввода пользователя и отправка
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    while let Some(line) = stdin.next_line().await? {
        if line.trim().is_empty() {
            break;
        }

        println!("📤 Отправляем: {}", line);
        write.send(line.into()).await?;
    }

    // Закрываем соединение
    drop(write); // Закрываем отправку
    read_handle.await?; // Ждем завершения чтения
    
    println!("🔌 Соединение закрыто");
    Ok(())
}