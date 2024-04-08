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
-   [v] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [v] Commit: `Create Subscriber model struct.`
    -   [v] Commit: `Create Notification model struct.`
    -   [v] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [v] Commit: `Implement add function in Subscriber repository.`
    -   [v] Commit: `Implement list_all function in Subscriber repository.`
    -   [v] Commit: `Implement delete function in Subscriber repository.`
    -   [v] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [v] Commit: `Create Notification service struct skeleton.`
    -   [v] Commit: `Implement subscribe function in Notification service.`
    -   [v] Commit: `Implement subscribe function in Notification controller.`
    -   [v] Commit: `Implement unsubscribe function in Notification service.`
    -   [v] Commit: `Implement unsubscribe function in Notification controller.`
    -   [v] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [v] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [v] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [v] Commit: `Implement publish function in Program service and Program controller.`
    -   [v] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [v] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. Tergantung dari penggunaan design pattern observernya. Jika kita memiliki obeserver yang memiliki banyak jenis sehingga terdiri dari berbagai macam class, maka akan diperlukan pembuatan observer menggunakan trait. Namun pada BambangShop observernya, yaitu Subscriber hanya berupa satu class, sehingga tidak diperlukannya dibuat suatu trait, kecuali kedepannya akan ada penambahan observer baru.

2. Kita perlu menggunakan DashMap karena dengan DashMap kita akan memiliki pemetaan tiap jenis produk ke tiap subscriber yang menginginkannya. Jika kita ganti menjadi vector, maka perlu dibuat dua vector untuk masing masing produk dimana 1 vector menyimpan url dan satu lagi subscribernya, di mana hal tersebut akan mempersulit perubahan data.

3. Menurut pemahaman saya, kita memerlukan Dashmap dibanding dengan Hashmap karena dashmap adalah built in data structure yang merupakan hashmap yang cocok untuk multithreading. Dalam kasus kita, kita memerlukannya karena memang aplikasi BambangShop akan menggunakan multi threading dan Map SUBSCRIBER tersebut akan diakses oleh banyak thread. Untuk Singleton, singleton befungsi untuk memastikan selama program berjalan hanya akan ada 1 instance dari objek tersebut. Hal ini berguna agar kita bisa selalu memastikan kalau daftar subscriber terhadap produk kita berada hanya pada 1 dash map dan tidak berceceran di berbagai struktur data.

#### Reflection Publisher-2

1. Service perlu dipisahkan dari Repository untuk memnuhi single responsibility principle. Pemisahan Service berfungsi untuk modul-modul yang mempunya fungsi mendapatkan dan mengolah data yang didapatkan dari repository sedangkan Layer Repository berfungsi sebagai layer yang berguna untuk mengakses data base, mengubah dan mendelete isi data base. Pemisahan dua layer ini membantu dalam pengembangan dan dan maintainability kode.

2. Jika kita hanya menggunakan layer model tanpa layer yang lain, maka akan tercipta program yang memiliki coupling tinggi sehingga jika terdapat suatu perubahan maka akan perlu banyak perubahan yang dilakukan ke kode.

3. Menurut ku, Postman sangat berguna untuk menguji aplikasi yang sudah kita buat dan melihat apakah aplikasinya akan mengembalikkan response yang sesuai dengan harapan kita berdasarkan request yang kita buat. Saya juga bisa menyesuaikan method yang diinginkan seperti CRUD-nya sehingga saya bisa melihat apakah data yang di-retrieve benar atau salah melalui Postman.

#### Reflection Publisher-3

1. Pada kasus tutorial ini yang digunakan adalah model push yang dapat dilihat pada bagian algoritma dimana ketika terjadi sesuatu pada modul objek misal create, delete atau update, notification service akan memanggil method yang akan mengiterasi semua subscribernya untuk mendapatkan update terbarunya.

2. Jika kita menggunakan metode pull, maka tiap tiap subscriber harus menentukan sendiri apakah perubahan data yang terjadi relevan untuk mereka. Keuntungannya adalah observer menjadi bebas untuk menentukan data apa yang akan dia ambil dan kapan Ia ambil. Kerugiannya adalah observer menjadi perlu mengetahui struktur dari data sourcenya agar bisa melakukan hal-hal yang disebutkan tadi

3. Yang akan terjadi adalah pada kode notificationservice saat notificationservice perlu me-notify tiap-tiap subscribernya maka akan tercipta antrian panjang jika terdapat banyak sekali subscriber yang membuat pengiriman notifikasi ke tiap subscriber menjadi terhambat akibat bottle neck kemampuan komputasi.