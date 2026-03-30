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

**1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?**
> In the BambangShop implementation, a single Model struct is already sufficient because all subscribers share the same notifications behaviour via HTTP POST requests to a Rocket instance. While the Head First Design Pattern book uses interfaces to allow for diverse subscriber types (like different devices or logging systems), we don't really have that complexity yet. However, if we ever needed to support varied notification methods like Email or SMS, implementing Rust Trait would be necessary to keep the service decoupled from each delivery method.

**2. `id` in Program and `url` in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?**
> While a Vec is a straightforward way to store data, using a DashMap is necessary here to ensure uniqueness and maintain its efficiency. A Vec would require manual checking to prevent duplicate URLs. On the other hand, DashMap inherently prevents duplicate keys, which is vital for a system where subscribers might frequently join or leave. Furthermore, DashMap provides the thread-safe access required for global variables in a multi-threaded Rust environment.

**3. When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?**
> The use of DashMap is a necessity for thread safety that a standard Singleton pattern doesn't solve on its own. In Rust, even if we ensure only one instance of a list exists (a Singleton), the compiler will block any attempt to modify it from multiple threads to prevent data races. By using a static DashMap, we are essentially implementing a Singleton that also utilizes interior mutability. This allows multiple threads to safely and concurrently access and modify the subscriber list, which a traditional Singleton without synchronization primitives could not achieve.

#### Reflection Publisher-2
**1. In the Model-View Controller (MVC) compound pattern, there is no "Service" and "Repository". Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate "Service" and "Repository" from a Model?**
> The main reason to separate Service and Repository from a Model is based on the Single Responsibility Principle. Since in the MVC Model handles everything from data structures to database queries, this often leads to "fat models" that are difficult to maintain and test. By introducing a Service layer, we isolate the business logic from the underlying data structures. Similarly, the Repository layer acts as a mediator that handles the technical details of data persistence. This separation means that if the database changes from a simple DashMap to a SQL database, we only need to modify the Repository without touching the business logic in the Service or the structure of the Model.

**2. What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?**
> If we decide to put everything into the Model, we would likely end up with highly coupled code that is hard to navigate. Imagine the Subscriber model having to manage its own HTTP client to send notifications while also managing its own storage logic and validation. As the interactions between Program, Subscriber, and Notification grow, the logic for triggering an update would be scattered inside the data structs themselves. For instance, the Program model would need to know exactly how the Subscriber list is stored and how the Notification payload is formatted. This interdependency creates a "spaghetti" effect where changing a single field in one model could break the logic across all three, significantly increasing the cognitive load required to make even minor updates.

**3. Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.**
> Postman acts as a bridge between the backend logic and the end-user experience, allowing testing of API endpoints without the need for a finished frontend. In this project, it is particularly helpful for simulating the observer pattern by acting as a mock subscriber or by triggering the publisher's endpoints to verify that notifications are correctly dispatched. Other than basic GET and POST requests, the "Collections" feature is incredibly useful for grouping related endpoints, which will be a lifesaver for group projects where multiple people need to share the same testing environment.

#### Reflection Publisher-3
