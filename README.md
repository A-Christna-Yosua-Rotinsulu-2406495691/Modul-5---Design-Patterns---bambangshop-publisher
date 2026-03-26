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

<hr>


<h3>🔎 1. Simplicity over Abstraction: Apakah Trait Benar-Benar Dibutuhkan?</h3>
<hr>

Dalam konsep klasik *Observer Pattern*, `Subscriber` biasanya direpresentasikan sebagai sebuah **interface** (atau `trait` dalam Rust) untuk mendukung berbagai implementasi dengan perilaku berbeda (*polymorphism*). Pendekatan ini sangat berguna ketika terdapat variasi cara subscriber menerima notifikasi, seperti melalui email, SMS, atau push notification. 

Namun, pada kasus **BambangShop**, seluruh subscriber mempunyai perilaku yang seragam, yaitu menerima notifikasi melalui **HTTP request**. Oleh sebab itu, penggunaan satu `struct Subscriber` sudah cukup mampu untuk merepresentasikan kebutuhan sistem. 

Contoh implementasi `Model`:

```Rust
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub url: String,
    pub name: String,
}
```

Dalam konteks ini:

- Tidak ada **variasi perilaku** - tidak membutuhkan `polymorphism`
- Tidak ada kebutuhan **extensibility** kompleks
- `Trait` akan menambah kompleksitas tanpa manfaat nyata (*over-engineering*)

Dengan demikian, keputusan **tidak menggunakan trait** merupakan *trade-off* yang tepat antara **kesederhanaan dan kebutuhan sistem**.

<h3>🗺️ 2. Choosing the Right Tool: Vec vs DashMap dalam Dunia Nyata</h3>
<hr>

Pemilihan struktur data sangat berpengaruh terhadap performa dan efisiensi sistem. Secara sederhana, `Vec` dapat digunakan untuk menyimpan **daftar subscriber**, namun memiliki beberapa keterbatasan:

- Pencarian membutuhkan iterasi ➡️ kompleksitas **O(n)**
- Tidak menjamin keunikan data secara langsung
- Operasi delete juga membutuhkan pencarian terlebih dahulu

Sebaliknya, pada tutorial ini digunakan `DashMap`, yaitu struktur data berbasis *key-value* (*hash-map*) yang mendukung akses cepat dan *thread-safe*.

Contoh implementasi:

```Rust
lazy_static! {
    static ref SUBSCRIBERS: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}
```

Contoh fungsi `add`:

```Rust
pub fn add(product_type: &str, subscriber: Subscriber) -> Subscriber {
    let subscriber_value = subscriber.clone();

    if SUBSCRIBERS.get(product_type).is_none() {
        SUBSCRIBERS.insert(String::from(product_type), DashMap::new());
    }

    SUBSCRIBERS.get(product_type)
        .unwrap()
        .insert(subscriber_value.url.clone(), subscriber_value);

    subscriber
}
```

Keunggulan dari `DashMap`:

- Akses data mendekati **O(1)** atau konstan
- Key (`url`) menjaga keunikan data
- Operasi **insert, delete, dan lookup** menjadi lebih efisien.
- Mendukung *concurrent access*

Dengan demikian, dibandingkan menggunakan `Vec`, `DashMap` memberikan:

- **Performa lebih optimal**
- **Struktur data yang sesuai dengan kebutuhan**
- **Skalabilitas yang lebih baik**

### 🔑 3. Beyond Singleton: Mengapa DashMap Tetap Dibutuhkan?

Dalam desain sistem, *Singleton Pattern* digunakan untuk memastikan hanya ada satu **instance** dari suatu data. Namun, penting untuk dipahami bahwa:

> **Singleton tidak secara otomatis menjamin *thread safety***

Pada tutorial yang telah saya lakukan, digunakan pendekatan sebagai berikut:

```Rust
lazy_static! {
    static ref SUBSCRIBERS: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}
```

Di sini:

- `lazy_static` berperan sebagai **Singleton-like initialization**
- `DashMap` berperan sebagai **thread-safe data structure**

Dalam konteks Rust:

- Thread safety dikontrol melalui `trait` seperti `Send` dan `Sync`
- `DashMap` sudah mengelola *concurrency* secara internal
- Tidak perlu menggunakan `Mutex` atau `RwLock` secara manual

Contoh fungsi `delete`:

```Rust
pub fn delete(product_type: &str, url: &str) -> Option<Subscriber> {
    if SUBSCRIBERS.get(product_type).is_none() {
        SUBSCRIBERS.insert(String::from(product_type), DashMap::new());
    }

    let result = SUBSCRIBERS.get(product_type).unwrap().remove(url);

    if !result.is_none() {
        return Some(result.unwrap().1);
    }

    None
}
```

Kesimpulan:

- **Singleton menjawab jumlah instance**
- **DashMap menjawab keamanan akses data**

Oleh sebab itu, dalam kasus ini:
> Penggunaan **DashMap tetap krusial**, bahkan jika sudah menggunakan pola **Singleton**

### 🔖Final Insight: Belajar Menyeimbangkan Teori dan Praktik
<hr>

Melalui implementasi dari tutorial ini, saya mendapatkan pemahaman baru bahwa desain sistem tidak hanya bergantung pada teori saja, tetapi juga pada kebutuhan nyata dari aplikasi. Tidak semua pattern atau abstraksi harus diterapkan jika tidak memberikan nilai tambah yang signifikan. Selain itu, saya juga menyadari bahwa:

- Pemilihan struktur data sangat memengaruhi performa
- **Over-engineering** dapat memperumit sistem tanpa manfaat nyata
- **Thread safety** merupakan aspek penting dalam pengembangan menggunakan Rust

Dengan demikian, keseimbangan antara **kesederhanaan, efisiensi, dan skalabilitas** menjadi kunci dalam merancang sebuah sistem yang baik.

#### Reflection Publisher-2
<hr>

<h3>1. 🔎 Breaking the Monolith: Mengapa Service dan Repository Perlu Dipisahkan?</h3>

<hr>

  

Dalam pola <b>Model-View-Controller</b> tradisional, `Model` sering kali memegang dua peran sekaligus, yaitu sebagai <b>penyimpanan data</b> dan <b>pengelola logika bisnis</b>. Namun, pendekatan ini bertentangan dengan prinsip **Single Responsibility Principle (SRP)** dan **Separation of Concerns (SoC)** yang menekankan bahwa setiap komponen sebaiknya mempunyai satu fokus tanggung jawab utama. 

Dengan memisahkan **Service** dan **Repository**, arsitektur sistem akan menjadi lebih modular:

- **Repository** bertugas untuk mengelola akses data `(CRUD)`
- **Service** menangani logika bisnis seperti proses *subscribe* dan *unsubscribe*
- **Model** hanya merepresentasikan struktur data saja

Contoh implementasi pada tutorial:

```Rust
pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(product_type: &str, subscriber: Subscriber) -> Result<Subscriber> {
        let product_type_upper: String = product_type.to_uppercase();
        let product_type_str: &str = product_type_upper.as_str();
        let subscriber_result: Subscriber =
            SubscriberRepository::add(product_type_str, subscriber);

        Ok(subscriber_result)
    }
}
```

Dari kode tersebut terlihat bahwa:

- `NotificationService` hanya menangani **logika bisnis** (normalisasi data & proses *subscribe*),
- Penyimpanan data didelegasikan ke `SubscriberRepository`

Contoh Repository:

``` Rust
pub struct SubscriberRepository;

impl SubscriberRepository {
    pub fn add(product_type: &str, subscriber: Subscriber) -> Subscriber {
        // logic penyimpanan data
        subscriber
    }
}
```

Keuntungan dari pendekatan ini:

- Mudah untuk diuji (**unit testing** pada `Service`)
- Mudah di-*maintain*
- Lebih *scalable* sebab tanggung jawab terpisah


<h3>⚠️ When Models Do Too Much: Risiko Jika Semua Ditangani Oleh Model</h3>
<hr>

Apabila hanya menggunakan `Model` tanpa memisahkan `Service` dan `Repository`, maka setiap model seperti **Program, Subscriber, dan Notification** akan menjadi terlalu kompleks (*God Object*). 

Sebagai ilustrasi singkat yang dapat saya berikan mengenai pendekatan yang kurang baik:

``` Rust
impl Subscriber {
    pub fn subscribe(&self, product_type: &str) {
        let product_type_upper = product_type.to_uppercase();

        // logika bisnis + penyimpanan digabung
        // (tidak direkomendasikan)
        println!("Saving subscriber to database...");
    }
}
```

Masalah dari pendekatan ini : 

- **Tight coupling** antar model
- **Kode sulit dipahami dan dipelihara**
- **Perubahan kecil berdampak luas**
- **Sulit untuk reuse dan testing** sebab tidak terisolasi
- **Model** menangani terlalu banyak hal (violasi **SRP**)

Sebaliknya, dengan arsitektur berlapis seperti yang sudah saya lakukan:

```
Controller ➡️ Service ➡️ Repository ➡️ Model
```

Contoh `Controller` yang memanggil `Service` pada tutorial ini:

```Rust
#[post("/subscribe/<product_type>", data = "<subscriber>")]
pub fn subscribe(product_type: &str, subscriber: Json<Subscriber>)
    -> Result<Created<Json<Subscriber>>> {

    match NotificationService::subscribe(product_type, subscriber.into_inner()) {
        Ok(f) => Ok(Created::new("/").body(Json::from(f))),
        Err(e) => Err(e),
    }
}
```

Terlihat bahwa dengan pendekatan ini:

- `Controller` hanya menangani request/response
- `Service` menangani logika bisnis
- `Repository` menangani data
- `Model` tetap sederhana

<h3>🚀 Testing Made Easy: Peran Penting Postman dalam Pengembangan API</h3>
<hr>

Dalam pengembangan API, **Postman** sangat membantu *developer* untuk melakukan pengujian endpoint tanpa perlu membuat *frontend*.

Contoh request yang digunakan dalam tutorial ini:

**Endpoint**:
``` 
POST /notification/subscribe/APPLIANCES
```

**Request Body (JSON)**:

```JSON
{
  "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
  "name": "Rick Astley"
}
```

Harapan *response* jika *request* berhasil dikirim akan berupa:

```JSON
{
  "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
  "name": "Rick Astley"
}
```
Untuk fitur **unsubsribe**:

```Rust
pub fn unsubscribe(product_type: &str, url: &str) -> Result<Subscriber> {
    let product_type_upper: String = product_type.to_uppercase();
    let product_type_str: &str = product_type_upper.as_str();

    let result: Option<Subscriber> =
        SubscriberRepository::delete(product_type_str, url);

    if result.is_none() {
        return Err(compose_error_response(
            Status::NotFound,
            String::from("Subscriber not found."),
        ));
    }

    Ok(result.unwrap())
}
```

Dan `Controller`-nya:

```Rust
#[post("/unsubscribe/<product_type>?<url>")]
pub fn unsubscribe(product_type: &str, url: &str)
    -> Result<Json<Subscriber>> {

    match NotificationService::unsubscribe(product_type, url) {
        Ok(f) => Ok(Json::from(f)),
        Err(e) => Err(e),
    }
}
```

Manfaat yang saya peroleh dengan menggunakan **Postman**:

- Mudah mengirim **request HTTP**
- Mendukung berbagai metode (**GET, POST, dll**)
- Bisa langsung melihat *response*
- Membantu *debugging* API

Fitur penting pada **Postman**: 

- **Collections ➡️** mengelompokkan endpoint
- **Environment variables ➡️** fleksibilitas konfigurasi
- **Test scripts ➡️** validasi otomatis
- **History ➡️** tracking request

Dengan **Postman**, saya dapat memastikan bahwa:

- Endpoint berjalan dengan benar
- Data dikirim dan diterima sesuai format
- Error handling bekerja dengan baik


#### Reflection Publisher-3
<hr>

