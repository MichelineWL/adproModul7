# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. Penggunaan Trait vs. Single Model
Apabila kita hanya memiliki satu tipe observer, menggunakan satu struct saja sudah memadai. Namun, pada dasarnya Observer Pattern lebih fleksibel jika kita mendefinisikan trait, sehingga di kemudian hari kita bisa menambah atau memodifikasi observer lain tanpa perlu banyak perubahan pada kode utama.

2. Efisiensi DashMap daripada Vec
DashMap memungkinkan pencarian data dengan kompleksitas O(1) dan juga mendukung akses thread-safe tanpa kita harus mengatur lock manual. Sementara itu, Vec membutuhkan pencarian linear (O(n)), sehingga kurang efisien jika kita sering melakukan lookup.

3. Thread-safety vs. Singleton
Pola Singleton hanya memastikan jumlah instance terbatas pada satu, tapi tidak serta-merta menjadikannya aman di lingkungan multithreaded. Kita tetap perlu mekanisme sinkronisasi, seperti DashMap atau lock lain. Jadi, walau Singleton berguna, itu belum cukup untuk menangani persoalan concurrency.

#### Reflection Publisher-2
1. Pemisahan Service dan Repository
Memecah kode menjadi Service (untuk logika bisnis) dan Repository (untuk akses data) membantu menjaga tanggung jawab masing-masing bagian agar tidak saling bercampur. Dengan begitu, perubahan di lapisan data tidak akan berdampak besar pada logika bisnis, dan sebaliknya.

2. Dampak Jika Hanya Menggunakan Model
Menyatukan semua logika dan operasi database dalam satu Model akan menimbulkan coupling yang tinggi. Jika Program, Subscriber, dan Notification saling berinteraksi dalam satu tempat, perubahan kecil pada satu bagian bisa mengganggu keseluruhan sistem. Dengan memisahkannya, kita dapat mengurangi kompleksitas dan meningkatkan kemudahan pemeliharaan.

3. Manfaat Postman
Postman memudahkan pengujian API dengan cepat. Kita bisa melakukan request HTTP, menambahkan autentikasi, dan melihat respon secara langsung. Fitur seperti environment, collection runner, dan test script mempermudah pengujian berulang serta memvalidasi konsistensi API di setiap tahap pengembangan.

#### Reflection Publisher-3
1. Penggunaan Push Model
Pada tutorial ini, publisher secara proaktif mengirim pembaruan ke semua subscriber. Begitu terjadi perubahan (misal, create, update, atau delete), publisher akan mengeksekusi mekanisme pengiriman notifikasi ke masing-masing subscriber.

2. Keunggulan dan Kekurangan Pull Model
Keunggulan: Subscriber hanya mengambil data yang benar-benar diperlukan, sehingga dapat mengurangi beban pengiriman data.
Kekurangan: Subscriber harus memiliki pengetahuan tentang cara dan waktu untuk mengambil data, sehingga menambah kerumitan di sisi subscriber dan berpotensi menimbulkan delay.

3. Konsekuensi Tanpa Multi-threading
Jika kita tidak menggunakan multi-threading, proses pengiriman notifikasi akan berjalan secara berurutan. Hal ini berpotensi menciptakan bottleneck saat jumlah subscriber banyak atau jika proses pengiriman ke salah satu subscriber memakan waktu lama. Akibatnya, kinerja dan responsivitas sistem menurun.
