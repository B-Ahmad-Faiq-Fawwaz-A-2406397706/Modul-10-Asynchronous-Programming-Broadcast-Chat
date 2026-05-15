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

Server mendengarkan koneksi WebSocket pada alamat yang sudah ditentukan di kode. Setiap client yang berhasil terhubung dapat mengirim pesan melalui koneksi tersebut. Ketika salah satu client mengirim pesan, server menerima pesan itu lalu meneruskannya ke client lain yang sedang aktif. Proses broadcast dilakukan menggunakan `tokio::sync::broadcast::channel`, sehingga satu pesan dapat dikirim ke banyak penerima. Server juga mencetak informasi pesan yang diterima beserta alamat client pengirim, sehingga aktivitas komunikasi dapat dipantau dari terminal server. Client lain akan menerima pesan tersebut dan menampilkannya di terminal masing-masing. Dengan demikian, percobaan ini menunjukkan bagaimana async Rust dan WebSocket dapat digunakan untuk membuat aplikasi chat sederhana yang berjalan secara concurrent.

Saat pesan diketik pada salah satu client, server akan membaca pesan tersebut dari stream WebSocket yang sedang aktif. Setelah pesan diterima, server mencetak informasi pengirim dan isi pesan ke terminal agar proses komunikasi terlihat jelas. Pesan kemudian dikirim ke channel broadcast supaya dapat diterima oleh koneksi client lain. Client pengirim tidak perlu menerima kembali pesan yang sama, karena pesan tersebut sudah berasal dari dirinya sendiri. Client lain yang sedang terhubung akan memperoleh pesan dari server dan menampilkannya pada terminal mereka. Perilaku ini membuktikan bahwa server berperan sebagai penghubung utama antara beberapa client. Hasil percobaan tersebut sudah sesuai dengan konsep broadcast chat, karena satu pesan dari satu client dapat disebarkan ke beberapa client lain.

## Experiment 2.2 - Modifying the WebSocket Port

**Hanya mengubah port di client**
![](images/Screenshot%202026-05-15%20204236.png)

Pada percobaan ini, port WebSocket perlu diperhatikan pada sisi server dan sisi client. Server menggunakan `TcpListener::bind(...)` untuk menentukan alamat dan port tempat aplikasi menerima koneksi masuk. Client menggunakan `ClientBuilder::from_uri(...)` untuk menentukan alamat WebSocket yang akan dituju ketika ingin terhubung ke server. Jika hanya port di client yang diubah, client akan mencoba terhubung ke alamat yang berbeda dari alamat yang sedang didengarkan oleh server. Akibatnya, koneksi akan gagal karena tidak ada server yang menerima koneksi pada port tersebut. Oleh karena itu, perubahan port seharusnya dilakukan secara konsisten pada kedua sisi agar server dan client tetap menggunakan alamat yang sama. Keduanya tetap memakai protokol `ws://`, yaitu WebSocket tanpa TLS, sehingga format URL dan konfigurasi listener harus saling cocok.
