# Ray Tracer in Rust

## Opis (PL)

Repozytorium zawiera projekt semestralny zrealizowany w ramach kursu **Programowanie w języku Rust** na kierunku **Informatyka** na **Akademii Górniczo-Hutniczej w Krakowie**.

Projekt przedstawia prosty silnik ray tracingu napisany w języku Rust. Implementacja została oparta na książce _"Ray Tracing in One Weekend"_ autorstwa Petera Shirley'a. Główne elementy projektu to:
- Obsługa podstawowych materiałów (Lambertian, Metal, Dielectric)
- Antyaliasing przez wielokrotne próbkowanie
- Rekurencyjne śledzenie promieni odbijających się
- Kamera z możliwością konfigurowania FOV i rozmycia głębi ostrości

---

## Description (EN)

This repository contains a semestral project developed for the **Rust Programming** course as part of the **Computer Science** program at the **AGH University of Science and Technology in Kraków**.

The project is a basic ray tracing engine written in Rust. The implementation is based on the book _"Ray Tracing in One Weekend"_ by Peter Shirley. Key features include:
- Support for basic materials (Lambertian, Metal, Dielectric)
- Anti-aliasing via multiple samples per pixel
- Recursive ray tracing
- Configurable camera with depth of field and field of view settings

---

## Compilation

This project uses [Cargo](https://doc.rust-lang.org/cargo/) as the build system. To compile and run:

```bash
cargo run --release
```
## Źródła / Sources

- Peter Shirley – [_Ray Tracing in One Weekend_](https://raytracing.github.io/)
- [Rust Programming Course – rust-course.vercel.app](https://rust-course.vercel.app/)
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)


