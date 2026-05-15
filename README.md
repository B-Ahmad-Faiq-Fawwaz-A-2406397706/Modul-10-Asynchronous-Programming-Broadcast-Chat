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

## Experiment 2.3 - Small changes. Add some information to client

**Server**
![](images/Screenshot%202026-05-15%20210924.png)

**Client 1**
![](images/Screenshot%202026-05-15%20210930.png)

**Client 2**
![](images/Screenshot%202026-05-15%20210936.png)

Pada percobaan ini, saya melakukan perubahan kecil pada format pesan agar client dapat melihat informasi pengirim dengan lebih jelas. Server sudah mengetahui alamat setiap client dari `SocketAddr` ketika koneksi diterima, sehingga informasi IP dan port dapat digunakan sebagai identitas sementara. Karena aplikasi ini belum memiliki fitur nama pengguna, IP dan port menjadi cara paling sederhana untuk membedakan satu client dengan client lain. Server mengirim pesan ke client lain dalam format `From 127.0.0.1:PORT -> pesan`, sehingga penerima dapat langsung mengetahui dari koneksi mana pesan tersebut berasal. Client juga diubah agar menampilkan pesan masuk dengan teks `Message received from another client`, sehingga output terminal lebih mudah dipahami. Perubahan ini membantu menjelaskan bahwa pesan tidak dikirim langsung dari satu client ke client lain, tetapi lewat server sebagai perantara. Dengan melihat IP dan port pada output, alur pengiriman pesan menjadi lebih sound karena setiap pesan dapat ditelusuri dari pengirim sampai penerimanya.

## Bonus - Change the WebSocket Server

![](images/Screenshot%202026-05-15%20230444.png)

![](images/Screenshot%202026-05-15%20230458.png)

Pada bagian bonus ini, server Rust dari Tutorial 2 diubah agar bisa melayani webclient YewChat dari Tutorial 3. Perubahan utama yang dilakukan adalah mengganti komunikasi plain text menjadi komunikasi berbasis JSON, tetapi JSON tersebut tetap dikirim sebagai text frame WebSocket. Server sekarang dapat menerima pesan dengan `messageType` bernilai `register` untuk menyimpan username, lalu mengirim pesan `users` berisi daftar username yang sedang terhubung. Server juga dapat menerima pesan dengan `messageType` bernilai `message`, lalu membungkus isi chat ke dalam JSON baru yang berisi `from`, `message`, dan `time`. Format ini dibuat agar cocok dengan webclient YewChat yang melakukan serialize dan deserialize menggunakan `serde_json` di sisi Rust frontend. Di dalam server, `SocketAddr` tetap dipakai sebagai identitas koneksi internal, sedangkan username disimpan di `HashMap` yang dilindungi `tokio::sync::Mutex` agar aman dipakai oleh beberapa task async. Perubahan ini berhasil karena server Rust sudah diuji dengan dua client WebSocket yang melakukan register sebagai `faiq` dan `yamal`, lalu server mengirim event `users` dan `message` dalam format JSON yang sama seperti server JavaScript tutorial. Menurut saya, versi JavaScript lebih cepat untuk dibuat dan lebih sederhana untuk tutorial awal, tetapi versi Rust lebih saya sukai untuk pengembangan yang lebih serius karena type system, error handling, dan model async-nya membuat struktur data komunikasi lebih eksplisit dan lebih aman.
