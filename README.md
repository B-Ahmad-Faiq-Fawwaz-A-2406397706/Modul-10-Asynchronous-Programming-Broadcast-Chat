## Experiment 2.1 - Original Code

### Cara Menjalankan

Buka **4 terminal terpisah**:

**Terminal 1 - Server:**
```bash
cd tutorial2-broadcast-chat
cargo run --bin server
```

**Terminal 2, 3, 4 - Client (masing-masing terminal):**
```bash
cargo run --bin client
```

Ketik pesan di salah satu client, semua client lain akan menerimanya.

**Server**
![](images/Screenshot%202026-05-15%20201302.png)

**Client 1**
![](images/Screenshot%202026-05-15%20201309.png)

**Client 2**
![](images/Screenshot%202026-05-15%20201314.png)

**Client 3**
![](images/Screenshot%202026-05-15%20201319.png)

Server mendengarkan koneksi WebSocket. Setiap client yang terhubung bisa mengirim pesan; server akan mem-broadcast pesan tersebut ke semua client yang sedang terhubung menggunakan `tokio::sync::broadcast::channel`.

Saat kamu mengetik di satu client:
- Server mencetak `Message received: 127.0.0.1:PORT: <pesan>`
- Semua client lain menerima pesan tersebut