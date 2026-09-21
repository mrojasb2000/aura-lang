# 🌟 Aura Language (`aurac`)

[![Rust CI](https://img.shields.io/badge/Rust_CI-177_tests_passing-brightgreen.svg)](tests)
[![Compiler Speed](https://img.shields.io/badge/Speed-850k%2B_LOC%2Fs-blue.svg)#-benchmarks-y-rendimiento-empírico)
[![Native Backend](<https://img.shields.io/badge/Backend-Cranelift_Native_(Mach--O_/_ELF)-orange.svg>)#-arquitectura-del-compilador-y-runtime)
[![Go Backend](https://img.shields.io/badge/Backend-Golang_Toolchain_Transpiler-blue.svg)#-backend-nativo-y-modelo-golang)
[![Concurrency](https://img.shields.io/badge/Concurrency-Go--style_CSP_Fibers_%26_Channels-purple.svg)#5-concurrencia-csp-modelo-estilo-go)
[![Self-Hosted](https://img.shields.io/badge/Self--Hosted-Stage_1_Bootstrapped-success.svg)#-compilador-self-hosted-y-bootstrapping)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**Aura** es un lenguaje de programación de sistemas y backend de alto rendimiento, fuertemente tipado e inmutable por defecto. Combina la sintaxis ergonómica y moderna de **TypeScript y Go** con la seguridad formal, tipos algebraicos y coincidencia exhaustiva de patrones de **ML y Rust**.

Aura se compila directamente a **binarios nativos ejecutables autónomos** (Mach-O en macOS, ELF en Linux) con cero dependencias externas de runtime mediante su backend nativo [**Cranelift**](https://cranelift.dev/) y su runtime M:N ultraligero (`aura-runtime`), o alternativamente mediante transpilación directa a la cadena de herramientas de **Golang**. Su motor de compilación escrito en Rust procesa más de **850,000 líneas de código por segundo** con un tiempo de arranque en frío (_cold-start_) inferior a **2 ms**.

---

## 📑 Tabla de Contenidos

1. [🚀 Inicio Rápido](#-inicio-rápido)
2. [✨ Características Principales](#-características-principales)
3. [⚡ Benchmarks y Rendimiento Empírico](#-benchmarks-y-rendimiento-empírico)
4. [📁 Arquitectura del Compilador y Runtime](#-arquitectura-del-compilador-y-runtime)
5. [📘 Guía Completa del Lenguaje](#-guía-completa-del-lenguaje)
   - [1. Variables, Inmutabilidad y Tipos de Datos](#1-variables-inmutabilidad-y-tipos-de-datos)
   - [2. Control de Flujo, Expresiones y Bucles Etiquetados](#2-control-de-flujo-expresiones-y-bucles-etiquetados)
   - [3. Tipos Suma (ADTs) y Pattern Matching Exhaustivo](#3-tipos-suma-adts-y-pattern-matching-exhaustivo)
   - [4. Pipelines (`|>`) y Optimización de Llamadas por la Cola (TCO)](#4-pipelines--y-optimización-de-llamadas-por-la-cola-tco)
   - [5. Concurrencia CSP (Fibers, Canales, Select y Sync)](#5-concurrencia-csp-modelo-estilo-go)
   - [6. Context, Defer, Panic / Recover y Punteros](#6-context-defer-panic--recover-y-punteros)
   - [7. Interfaces Implícitas y Structural Duck Typing](#7-interfaces-implícitas-y-structural-duck-typing)
   - [8. Genéricos y Polimorfismo Paramétrico](#8-genéricos-y-polimorfismo-paramétrico)
   - [9. Struct Tags y Build Tags (Modelo Go)](#9-struct-tags-y-build-tags-modelo-go)
   - [10. Ingestión Directa de Definiciones TypeScript (`.d.ts`)](#10-ingestión-directa-de-definiciones-typescript-dts)
   - [11. Inclusión Estática de Recursos en Compilación (`embed`)](#11-inclusión-estática-de-recursos-en-compilación-embed)
   - [12. Programación de Sistemas y Rendimiento Inspirados en Zig](#12-programación-de-sistemas-y-rendimiento-inspirados-en-zig)
6. [🗄️ Librería Estándar, Servidor HTTP y Bases de Datos](#️-librería-estándar-servidor-http-y-bases-de-datos)
   - [Servidor Web HTTP y Mux de Enrutamiento (`net/http`)](#-servidor-web-http-y-mux-de-enrutamiento-nethttp)
   - [Controlador Nativo de PostgreSQL (`pg` / `postgres`)](#-controlador-nativo-de-postgresql-pg--postgres)
   - [Controlador Nativo de MySQL (`mysql`)](#-controlador-nativo-de-mysql-mysql)
   - [Controlador Nativo de MongoDB (`mongodb` / `mongo`)](#-controlador-nativo-de-mongodb-mongodb--mongo)
   - [Cliente Nativo de Redis (`redis`)](#-cliente-nativo-de-redis-redis)
   - [Ejemplo de Producción: Microservicio REST `bookstore_api`](#-ejemplo-de-producción-microservicio-rest-bookstore_api)
7. [🐛 Depuración Interactiva y Source Maps V3](#-depuración-interactiva-y-source-maps-v3)
   - [Depurador Paso a Paso en Terminal (`aurac step`)](#depurador-paso-a-paso-en-terminal-aurac-step)
   - [Servidor de Depuración V8 / DAP (`aurac debug`)](#servidor-de-depuración-v8--dap-aurac-debug)
8. [🛠️ Referencia de Herramientas y CLI](#️-referencia-de-herramientas-y-cli)
   - [Compilador y Sistema de Construcción (`aurac`)](#1-compilador-y-sistema-de-construcción-aurac)
   - [Runner de Pruebas, Benchmarks y Cobertura (`auratest`)](#2-runner-de-pruebas-benchmarks-y-cobertura-auratest)
   - [Formateador de Código de Estilo Opinado (`aurafmt`)](#3-formateador-de-código-aurafmt-estilo-gofmt)
   - [Protocolo de Servidor de Lenguaje (`auralsp`)](#4-servidor-de-lenguaje-auralsp--aurac-lsp)
   - [Playground Web Interactivo (`aurac playground`)](#5-playground-web-interactivo)
9. [🧩 Configuración en Editores (VS Code, Antigravity IDE, Neovim, Zed)](#-configuración-en-editores)
10. [🔄 Compilador Self-Hosted y Bootstrapping](#-compilador-self-hosted-y-bootstrapping)
11. [🗺️ Estado Actual, Roadmap e Hitos](#️-estado-actual-roadmap-e-hitos)
12. [📄 Licencia](#-licencia)

---

## 🚀 Inicio Rápido

### 1. Construir la Cadena de Herramientas de Aura

Asegúrate de tener instalada la herramienta Rust (edición 2024):

```bash
git clone https://github.com/mrojasb2000/aura-lang.git
cd aura-lang
cargo build --release
```

Los ejecutables resultantes estarán disponibles en `target/release/`:

- `aurac` — Compilador central, constructor nativo, ejecutor, watch, depurador, LSP y playground.
- `auratest` — Suite completa de pruebas unitarias, benchmarks (`ns/op`) y cobertura HTML.
- `aurafmt` — Formateador de código rápido, idempotente y opinado (estilo `gofmt`).
- `auralsp` — Demonio del Language Server Protocol (LSP) compatible con cualquier editor.

### 2. Tu Primer Programa en Aura

Crea el archivo `hello.aura`:

```aura
export fn main(): Unit => {
    println("¡Hola desde Aura Lang!");
}
```

Si prefieres la ergonomía de escribir aura sin la c, puedes crear un alias en tu shell (~/.zshrc):

```aura
alias aura="aurac"
```

Una vez aplicado (source ~/.zshrc), el comando:

```bash
aura run hello.aura
¡Hola desde Aura Lang!
```

#### Ejecución directa en desarrollo:

```bash
cargo run --release -- run hello.aura
# Salida: ¡Hola desde Aura Lang!
```

#### Compilación a binario nativo independiente con Cranelift:

```bash
cargo run --release -- build hello.aura -o hello --target native
./hello
# Salida: ¡Hola desde Aura Lang!
```

#### Compilación mediante transpilación a la cadena de herramientas de Go:

```bash
cargo run --release -- build hello.aura -o hello --target go
./hello
# O transpilar a código fuente Go puro inspeccionable:
cargo run --release -- emit-go hello.aura -o hello.go
```

---

## ✨ Características Principales

- **🏎️ Backend Nativo Cranelift**: Genera código máquina nativo (Mach-O / ELF) enlazado estáticamente con el runtime ligero en Rust (`libaura_runtime.a`), sin intérpretes ni dependencias externas.
- **🔒 Inferencia de Tipos Hindley-Milner Bidireccional**: Resolución de tipos robusta, genéricos paramétricos completos (`<T>`), y eliminación total de errores de `null` o `undefined` mediante los tipos suma estándar `Option<T>` (`Some(v)` / `None`) y `Result<T, E>` (`Ok(v)` / `Err(e)`).
- **🔪 Slices y Arreglos Dinámicos (Modelo Go)**: Sintaxis de tipo `[]T` nativa (sinónimo de `List<T>`), expresiones de segmentación `s[low:high]`, `s[:high]`, `s[low:]`, `s[:]` y `s[low:high:max]`, built-ins idiomáticos `len`, `cap`, `append`, `make`, soporte de string slicing y compatibilidad completa en todos los backends (Cranelift nativo, Go y JS).
- **🔄 Concurrencia CSP (Estilo Go)**: Fibers livianos con stack switching (`spawn`), canales fuertemente tipados con buffers configurables (`Channel<T>`), tipos direccionales (`SendChannel<T>`, `RecvChannel<T>`), iteración de canales (`for val in ch`), multiplexación no bloqueante con `select` (send, recv, timeout, default), `WaitGroup` y primitivas de sincronización (`Mutex`, `RWMutex`, `Once`, `Pool`).
- **🛡️ Semántica Moderna y Control de Flujo**: Expresiones en bloques `{ ... }`, sentencias `defer` con garantía LIFO para liberación de recursos, gestión de contexto (`Context`), recuperación ante pánicos (`panic` / `recover`), bucles etiquetados con `break label` y punteros de memoria explícitos (`&` y `*`).
- **🧩 Interfaces Implícitas (Duck Typing Estructural)**: Satisfacción automática de interfaces sin requerir palabras clave como `implements`.
- **⚡ Optimización Tail-Call (TCO)**: Las funciones recursivas en posición de cola se transforman en bucles iterativos `while (true)` con consumo de stack $O(1)$.
- **🗄️ Librería Estándar "Batteries-Included"**: Controladores nativos tipados para **PostgreSQL**, **MySQL**, **MongoDB** y **Redis**, junto con un servidor web **HTTP ServeMux** de alta velocidad.
- **📦 Ingestión Directa de Archivos `.d.ts` de NPM**: Chequeo de tipos estático frente a definiciones externas de TypeScript sin envoltorios manuales.
- **🦀 & 🐹 Arquitectura Unificada Rust + Go**: Tipado estático formal y seguro con Hindley-Milner, tipos algebraicos (ADTs), coincidencia de patrones exhaustiva, operador `?` (`Result`/`Option`), inmutabilidad por defecto, concurrencia CSP nativa (fibers, canales, `select`), slices Go (`[]T`, `s[low:high:max]`), duck typing estructural e interfaces implícitas.
- **🐛 Depurador Interactivo de Terminal y Source Maps V3**: Depurador nativo con comandos de paso a paso (`aurac step`), e integración con DAP / V8 Inspector (`aurac debug`) para editores modernos.
- **🔄 Compilador Self-Hosted**: Implementación completa del compilador escrita en el propio lenguaje Aura (`src/aura_compiler/`).

---

## ⚡ Benchmarks y Rendimiento Empírico

Aura incluye una suite de pruebas de rendimiento automatizada y reproducible ubicada en [`benchmarks/`](benchmarks), evaluada rigurosamente en hardware Apple Silicon (Darwin arm64) comparando **Aura Lang (`aurac`)** frente a **Golang (`go 1.27+`)**.

### 1. Tiempos de Ejecución y Consumo de Memoria (Peak RSS)

| Escenario Evaluado                         | Aura Lang (`aurac`) | Golang (`go`) |     Ratio (Aura vs Go)      | Memoria Aura (RSS) | Memoria Go (RSS) |    Ventaja de Aura     |
| :----------------------------------------- | :-----------------: | :-----------: | :-------------------------: | :----------------: | :--------------: | :--------------------: |
| **Fibonacci Recursivo ($N=38$)**           |    **41.43 ms**     |   88.04 ms    | **0.47x (2.1x más rápido)** |    **2.45 MB**     |     4.09 MB      |  **-40.1% menos RAM**  |
| **Criba de Eratóstenes (2M primos)**       |    **14.62 ms**     |    5.50 ms    |            2.66x            |    **3.05 MB**     |     5.94 MB      |  **-48.6% menos RAM**  |
| **Pipeline de Datos Funcional (1M items)** |    **14.80 ms**     |    4.21 ms    |            3.52x            |    **2.86 MB**     |     20.02 MB     |  **-85.7% menos RAM**  |
| **Canales CSP Ping-Pong (200k msgs)**      |    **25.03 ms**     |   21.15 ms    |            1.18x            |      11.05 MB      |     4.02 MB      |    Paridad virtual     |
| **Spawn Concurrente (50k tareas)**         |    **12.27 ms**     |   10.62 ms    |            1.16x            |    **11.84 MB**    |     13.14 MB     |  **-9.9% menos RAM**   |
| **Arranque en Frío / Cold-Start (CLI)**    |     **3.59 ms**     |    2.54 ms    |            1.41x            |    **2.25 MB**     |     3.92 MB      |  **-42.6% menos RAM**  |
| **Sincronización Mutex (50k ops)**         |    **16.65 ms**     |   17.34 ms    | **0.96x (Aura más rápido)** |      13.47 MB      |     8.17 MB      | Paridad de rendimiento |
| **Cache KV Concurrente (50k ops)**         |    **16.70 ms**     |   19.23 ms    | **0.87x (13% más rápido)**  |      15.55 MB      |     15.20 MB     |   Paridad de memoria   |

### 2. Tamaño de Binarios Autónomos (Cero Dependencias)

| Escenario                              | Binario Go (MB) | Binario Go Stripped (`-s -w`) | Binario Standalone Aura |                  Ratio Aura vs Go                  |
| :------------------------------------- | :-------------: | :---------------------------: | :---------------------: | :------------------------------------------------: |
| **Cálculo Numérico (Cranelift)**       |     2.32 MB     |            1.51 MB            |   **2.97 – 2.99 MB**    |                       1.28x                        |
| **Pipeline de Datos (Cranelift)**      |     2.32 MB     |            1.51 MB            |       **2.99 MB**       |                       1.29x                        |
| **Servidor HTTP REST / Microservicio** |     8.84 MB     |            5.96 MB            |       **1.78 MB**       | **0.20x (Aura 80% más liviano / 5x más compacto)** |
| **Concurrencia CSP / Canales**         |     2.33 MB     |            1.53 MB            |       **5.06 MB**       |                       2.17x                        |

### 3. Rendimiento de Compilación frente a TypeScript (`tsc`)

| Métrica Clave                    |      Aura Lang (`aurac`)       |   TypeScript (`tsc`)   |          Ventaja de Aura           |
| :------------------------------- | :----------------------------: | :--------------------: | :--------------------------------: |
| **Arranque Frío de Compilador**  |     **~1.50 ms – 1.78 ms**     |    ~130 ms – 150 ms    |    🚀 **~80x – 100x más veloz**    |
| **Latencia Interna del Motor**   |    **0.016 ms – 0.089 ms**     |     ~15 ms – 45 ms     | ⚡ **~200x – 500x menor latencia** |
| **Throughput de Compilación**    | **~850,000 – 1,680,000 LOC/s** | ~25,000 – 60,000 LOC/s | 📈 **~20x – 40x mayor throughput** |
| **Consumo de Memoria RAM (RSS)** |          **2.27 MB**           |  103.9 MB – 143.0 MB   |      📉 **45x menor consumo**      |

---

## 📁 Arquitectura del Compilador y Runtime

```mermaid
flowchart TD
    Src["Código Fuente Aura (*.aura)"] --> Lex["Lexer (lexer.rs)"]
    Lex --> Parse["Parser Pratt (parser.rs)"]
    Parse --> AST["Árbol de Sintaxis Abstracta (ast.rs)"]

    DTS["Definiciones NPM (*.d.ts)"] --> DTSP["DTS Parser (dts_parser.rs)"]
    DTSP --> TC

    AST --> TC["TypeChecker Hindley-Milner (typechecker.rs)<br/>Inferencia, Unificación y Duck Typing"]

    TC --> BackendMux{"Selector de Backend<br/>(aurac build / run)"}

    BackendMux -->|"--target native (por defecto)"| CL["Cranelift Native Backend (codegen_cranelift.rs)"]
    CL --> Runtime["Runtime Nativo (crates/aura-runtime)<br/>• M:N Fiber Scheduler<br/>• Recolector de Basura (GC)<br/>• Canales CSP y Mutex<br/>• Servidor HTTP y JSON"]
    Runtime --> BinNative["Binario Nativo Autónomo<br/>(Mach-O / ELF)"]

    BackendMux -->|"--target go / emit-go"| GoCG["Transpilador Go (codegen_go.rs)"]
    GoCG --> GoTool["Cadena de Herramientas Go (go build)"]
    GoTool --> BinGo["Binario Nativo Ejecutable Go"]

    BackendMux -->|"aurac run / debug / watch"| NodeCG["Generador ES6 + Source Maps V3 (codegen.rs)"]
    NodeCG --> NodeRuntime["Ejecución Instantánea / Depurador V8 DAP"]
```

### Organización del Código Fuente:

- [`src/ast.rs`](file:///Users/mavro/Projects/aura-lang/src/ast.rs) — Definición formal del AST (expresiones, sentencias, tipos algebraicos, concurrencia CSP e interfaces).
- [`src/lexer.rs`](file:///Users/mavro/Projects/aura-lang/src/lexer.rs) — Lexer determinista con soporte para operadores funcionales (`|>`, `=>`), canales (`<-`) y palabras clave de sistemas.
- [`src/parser.rs`](file:///Users/mavro/Projects/aura-lang/src/parser.rs) — Parser descendente recursivo y de precedencia de operadores (Pratt Parser).
- [`src/typechecker.rs`](file:///Users/mavro/Projects/aura-lang/src/typechecker.rs) — Inferencia bidireccional Hindley-Milner, exhaustividad de patrones y satisfacción estructural de interfaces.
- [`src/codegen_cranelift.rs`](file:///Users/mavro/Projects/aura-lang/src/codegen_cranelift.rs) — Backend nativo que emite código máquina mediante Cranelift y enlaza con `libaura_runtime.a`.
- [`crates/aura-runtime/`](file:///Users/mavro/Projects/aura-lang/crates/aura-runtime) — Runtime nativo autónomo en Rust: scheduler de fibers M:N, recolector de basura (GC), canales CSP, sincronización de mutexes, servidor HTTP y serializador JSON.
- [`src/codegen_go.rs`](file:///Users/mavro/Projects/aura-lang/src/codegen_go.rs) — Generador de código idiomatico para Golang (`net/http`, goroutines, canales, select).
- [`src/backend.rs`](file:///Users/mavro/Projects/aura-lang/src/backend.rs) — Orquestador de validación estricta de backend y compilación de binarios.
- [`src/codegen.rs`](file:///Users/mavro/Projects/aura-lang/src/codegen.rs) — Generador de código para desarrollo ágil, TCO y empaquetado de runtime.
- [`src/sourcemap.rs`](file:///Users/mavro/Projects/aura-lang/src/sourcemap.rs) — Generador de Source Maps V3 con codificador VLQ en Base64 para depuración de alta precisión.
- [`src/lsp.rs`](file:///Users/mavro/Projects/aura-lang/src/lsp.rs) — Servidor LSP JSON-RPC 2.0 (diagnósticos en tiempo real, hover, navegación a definición y formateo).
- [`src/testing.rs`](file:///Users/mavro/Projects/aura-lang/src/testing.rs) — Motor de pruebas unitarias y benchmarks estilo `go test`, con generador de reportes de cobertura HTML.
- [`src/formatter.rs`](file:///Users/mavro/Projects/aura-lang/src/formatter.rs) — Formateador de código fuente opinado e idempotente estilo `gofmt`.
- [`src/aura_compiler/`](file:///Users/mavro/Projects/aura-lang/src/aura_compiler) — Compilador _self-hosted_ escrito enteramente en Aura.

---

## 📘 Guía Completa del Lenguaje

### 1. Variables, Inmutabilidad y Tipos de Datos

En Aura, las vinculaciones con `let` son **inmutables por defecto**. Para declarar estado mutable, se requiere explícitamente `let mut`.

```aura
// Inmutables (por defecto)
let pi: Float = 3.14159;
let appName = "Aura Engine"; // Tipo inferido como String

// Mutabilidad explícita
let mut counter: Int = 0;
counter = counter + 1;

// Tipos numéricos de tamaño fijo (paridad con Rust y Go)
let a: Int8 = 127;
let b: Int32 = 100000;
let c: Uint64 = 9999999;
let r: Rune = 65; // Rune UTF-32 (carácter 'A')

// Colecciones estándar y Slices (Modelo Go)
let numbers: []Int = [1, 2, 3, 4, 5]; // []T es equivalente nativo a List<T>
let userMap = Map.new();
userMap.set("role", "admin");

// Tuplas y desestructuración de registros
let coords = (10, 20);
let user = { id: 101, name: "Lucas", email: "lucas@aura.dev" };
let { id, name } = user;
```

### 2. Slices y Segmentación de Colecciones (Modelo Golang)

Aura adopta fielmente el modelo conceptual de los **Slices de Golang**, ofreciendo una abstracción liviana, eficiente y expresiva sobre secuencias de memoria contigua en todos los backends (Cranelift nativo, Go y JavaScript).

#### A. Sintaxis de Tipos y Declaración
Los slices se declaran utilizando la notación concisa `[]T` (o genérica `List<T>` indistintamente):

```aura
let mut numeros: []Int = [10, 20, 30, 40, 50];
let mut frameworks: []String = ["Aura", "Go", "Rust", "TypeScript"];
```

#### B. Expresiones de Slicing (`s[low:high]`)
Aura permite segmentar cualquier slice o arreglo mediante las expresiones de slicing estándar de Go (intervalo semiabierto `[low, high)`):

```aura
let s = [10, 20, 30, 40, 50];

let sub1 = s[1:4];     // [20, 30, 40] (índices 1 a 3)
let sub2 = s[:3];      // [10, 20, 30] (desde el inicio hasta índice 2)
let sub3 = s[2:];      // [30, 40, 50] (desde índice 2 hasta el final)
let sub4 = s[:];       // [10, 20, 30, 40, 50] (vista completa)
let sub5 = s[1:3:5];   // [20, 30] con límite de capacidad máxima 5 (Go 3-index slice)
let sub6 = s[1..4];    // [20, 30, 40] (notación de rango alternativa)
```

#### C. Funciones Built-in Estilo Go (`len`, `cap`, `append`, `make`)
El preludio de Aura expone directamente las funciones de manipulación de slices familiares para desarrolladores de Golang:

```aura
// Longitud y capacidad
println(`Longitud: ${len(numeros)}`);    // 5
println(`Capacidad: ${cap(numeros)}`);   // 5

// Agregar elementos con append (modelo idiomático Go)
numeros = append(numeros, 60);

// Asignación pre-dimensionada con make
let buffer = make([]Int, 5, 10);
```

#### D. Acceso por Propiedades y Métodos Fluídos
Para mantener la ergonomía moderna de TypeScript y el paradigma orientado a objetos, Aura también proporciona acceso directo a propiedades y métodos:

```aura
println(`Longitud: ${numeros.len} o ${numeros.length}`);
println(`Capacidad: ${numeros.cap} o ${numeros.capacity}`);

let sub = numeros.slice(1, 4); // [20, 30, 40]
numeros.push(70);              // Inserción in-place
```

#### E. Segmentación de Cadenas (String Slicing)
Las expresiones de slicing operan de forma transparente y segura sobre cadenas `String`:

```aura
let texto: String = "¡Hola, Mundo!";
let saludo = texto[1:5]; // "Hola"
```

> **Ejemplo Completo Ejecutable:** Puedes explorar el ejemplo de referencia integral en [`examples/slices_golang_model.aura`](examples/slices_golang_model.aura):
> ```bash
> # Ejecución interactiva
> cargo run --release -- run examples/slices_golang_model.aura
> 
> # Compilación a binario nativo standalone mediante backend Go
> cargo run --release -- build examples/slices_golang_model.aura -o slices_demo --target go
> ./slices_demo
> ```

### 3. Control de Flujo, Expresiones y Bucles Etiquetados

En Aura, los bloques de código `{ ... }` son expresiones cuyo valor resultante es el de la última expresión sin punto y coma:

```aura
let total = {
    let base = 100.0;
    let tax = 0.19;
    base * (1.0 + tax) // Retorna 119.0
};

// Expresión If-Else
let status = if score >= 70 { "Aprobado" } else { "Reprobado" };

// Bucles While y For-In
let mut i = 0;
while i < 10 {
    i = i + 1;
};

// Bucles con etiquetas (Labeled Loops) y break / continue dirigido
outerLoop: for (r, row) in matrix {
    for (c, val) in row {
        if val == target {
            println(`Encontrado en fila ${r}, columna ${c}`);
            break outerLoop; // Rompe directamente el bucle exterior
        }
    }
};
```

### 4. Tipos Suma (ADTs) y Pattern Matching Exhaustivo

Los tipos suma permiten modelar estados de dominio con verificación estricta de exhaustividad en tiempo de compilación:

```aura
export type OrderStatus =
    | Pending
    | Shipped { trackingCode: String, carrier: String }
    | Delivered
    | Cancelled(String)

export fn renderStatus(status: OrderStatus): String =>
    match status {
        Pending => "El pedido está en preparación.",
        Shipped { trackingCode, carrier } => `En tránsito vía ${carrier}: ${trackingCode}`,
        Delivered => "Pedido entregado con éxito.",
        Cancelled(reason) => `Pedido cancelado: ${reason}`,
    }
```

El compilador comprueba que todos los casos estén cubiertos o de lo contrario genera un error de exhaustividad antes de generar código máquina.

### 5. Pipelines (`|>`) y Optimización de Llamadas por la Cola (TCO)

El operador pipeline (`|>`) permite encadenar transformaciones legibles de izquierda a derecha. Además, las funciones recursivas en posición de cola son optimizadas automáticamente a bucles iterativos `while (true)` con cero consumo adicional de stack:

```aura
// Transformación fluida de colecciones
let activeUserNames = users
    |> filter(u => u.isActive)
    |> map(u => u.name)
    |> join(", ");

// Función recursiva optimizada con TCO (Tail-Call Optimization)
export fn sumList(items: List<Int>, acc: Int = 0): Int =>
    match items {
        [] => acc,
        [head, ...tail] => sumList(tail, acc + head) // Se compila a un bucle while sin overhead de pila
    }
```

### 6. Concurrencia CSP (Modelo Estilo Go)

Aura implementa el modelo de concurrencia CSP (_Communicating Sequential Processes_) con fibers ultralivianos y canales tipados:

```aura
// 1. Productor concurrente con canal direccional de solo envío
fn producer(outCh: SendChannel<String>) {
    outCh <- "Tarea Alpha";
    outCh <- "Tarea Beta";
    Channel.close(outCh);
}

// 2. Consumidor que procesa el canal mediante bucle for-in
async fn consumer(inCh: RecvChannel<String>): Task<Unit, String> {
    for task in inCh {
        println(`Procesando tarea: ${task}`);
    }
}

// 3. Multiplexación con select (con timeouts y fallback no bloqueante)
let ch1 = Channel<Int>::new(5);
let ch2 = Channel<String>::new(5);

select {
    case val = <-ch1 => println(`Recibido de ch1: ${val}`),
    case ch2 <- "listo" => println("Mensaje enviado a ch2"),
    case timeout(200) => println("Tiempo límite agotado (200 ms)"),
    default => println("Ningún canal disponible de inmediato")
}

// 4. Sincronización con Mutex y WaitGroup
let wg = WaitGroup.new();
let mu = Mutex.new();
wg.add(2);

spawn {
    mu.lock();
    // Sección crítica protegida
    mu.unlock();
    wg.done();
};
```

### 7. Context, Defer, Panic / Recover y Punteros

Aura provee utilidades de grado de producción para la administración de recursos del sistema:

```aura
// 1. Limpieza garantizada LIFO con defer
fn processFile(path: String) {
    let file = openFile(path)?;
    defer file.close(); // Se ejecutará al salir de la función, incluso ante errores
    file.write("Datos procesados");
}

// 2. Cancelación y tiempos límite con Context
let (ctx, cancel) = Context.withTimeout(Context.background(), 500);
defer cancel();

// 3. Captura y recuperación ante pánicos
fn safeExecution() {
    defer {
        let err = recover();
        if err != None {
            println(`Recuperado con éxito del pánico: ${err}`);
        }
    };
    panic("Condición irrecuperable en worker");
}

// 4. Operaciones de punteros explícitas (& y *)
fn increment(ptr: *Int) {
    *ptr = *ptr + 1;
}

let mut val = 41;
increment(&val);
println(`${val}`); // 42
```

### 8. Interfaces Implícitas y Structural Duck Typing (Modelo Golang)

Siguiendo el modelo exacto de **Golang**, los `struct` en Aura definen únicamente campos de datos. Los métodos se declaran desacoplados de la estructura mediante funciones con receptor por valor `fn (s: Struct) ...` o puntero `fn (s: *Struct) ...`. Cualquier tipo cuyo conjunto de métodos (*method set*) implemente las firmas requeridas por una `interface` la satisface automáticamente sin `implements`:

```aura
// 1. Definición de la interfaz
interface Greeter {
    greet(target: String): String;
}

// 2. Struct con datos puros
struct User {
    name: String,
};

// 3. Método con receptor al estilo Golang
fn (u: User) greet(target: String): String => {
    `¡Hola ${target}, soy ${u.name}!`
}

// 4. Consumo polimórfico mediante la interfaz
fn sendWelcome(g: Greeter): String => {
    g.greet("Mundo")
}

export fn main(): Unit => {
    let user: User = { name: "Alice" };
    // User satisface Greeter implícitamente a través de su método receiver:
    println(sendWelcome(user));
}
```

### 9. Genéricos y Polimorfismo Paramétrico

```aura
type Box<T> = BoxVal(T);

fn wrap<T>(value: T): Box<T> {
    BoxVal(value)
}

fn unwrapOr<T>(opt: Option<T>, defaultValue: T): T =>
    match opt {
        Some(v) => v,
        None => defaultValue
    }
```

### 10. Struct Tags y Build Tags (Modelo Go)

Aura soporta etiquetas de metadatos en estructuras para serialización JSON automática y etiquetas de compilación condicional:

```aura
// +build premium,darwin
// Tags de compilación condicional: aurac build --tags "premium,darwin"

type UserAccount = {
    id: Int,
    fullName: String,
    secretHash: String,
} `json:"user_account" db:"users"`;
```

### 11. Ingestión Directa de Definiciones TypeScript (`.d.ts`)

Aura puede cargar tipos externos de TypeScript para validar llamadas a librerías del ecosistema sin escribir bindings manuales:

```bash
aurac build app.aura -o app --dts ./node_modules/@types/lodash/index.d.ts
```

### 12. Inclusión Estática de Recursos en Compilación (`embed`)

Incrusta archivos de configuración o imágenes binarias directamente dentro del binario compilado sin requerir lecturas en disco en tiempo de ejecución:

```aura
let configText: String = embed("config/settings.yaml");
let iconBytes = embedBytes("assets/logo.png");
```

### 12. Programación de Sistemas y Rendimiento Inspirados en Zig

Inspirado en la filosofía de precisión en la gestión de recursos, empaquetado de memoria a nivel de bit y compilación cruzada transparente de **Zig Lang**, Aura incorpora primitivas clave de bajo nivel que conviven armónicamente con su sistema de tipos estático Hindley-Milner, su modelo de concurrencia CSP (Go-style) y sus backends nativos Cranelift y Go:

#### A. Limpieza Condicional en Errores (`errdefer`)

Mientras que `defer` garantiza la ejecución incondicional de una expresión al salir del ámbito de la función (estilo Go), **`errdefer`** ejecuta la limpieza **exclusivamente si la función retorna un error (`Err(...)`), si falla la propagación mediante el operador `?` o si ocurre un `panic`**.

Esto implementa una semántica transaccional nativa de *rollback* sin necesidad de anidar bloques `try/catch` ni mantener banderas booleanas manuales (`let mut failed = true;`):

```aura
fn transferFunds(fromId: Int, toId: Int, amount: Float): Result<String, String> => {
    // defer se ejecuta SIEMPRE al salir del ámbito (cierre ordenado de conexiones)
    defer releaseConnection();

    let tx = beginTransaction();
    // errdefer se ejecuta SOLAMENTE si la función sale con error o panic
    errdefer tx.rollback();

    debitAccount(fromId, amount)?;
    creditAccount(toId, amount)?;

    tx.commit();
    Ok("transfer_completed")
}
```

- **Orden LIFO garantizado**: Las sentencias `defer` y `errdefer` se intercalan determinísticamente en orden inverso de declaración (*Last-In, First-Out*).
- **Cero sobrecoste en el camino feliz**: Si la función tiene éxito y retorna `Ok(...)`, las expresiones de `errdefer` se omiten sin realizar ninguna llamada ni alocación adicional.
- **Soporte completo en todos los backends**: Totalmente implementado en el backend nativo Cranelift (inspección de variantes y bloques de salto condicional IR), en el backend Go (`defer func() { if recover() != nil || !__aura_ret.IsOk { ... } }()`) y en JavaScript/TypeScript runtime.

#### B. Estructuras de Memoria Compactas sin Relleno (`packed struct`)

En la programación de redes, emuladores, controladores de dispositivos e interoperabilidad binaria, las estructuras convencionales añaden relleno de alineación (*padding bytes*) según la palabra del procesador. Mediante **`packed struct`**, Aura garantiza una disposición física continua de memoria sin ningún padding:

```aura
// Cabecera TCP binaria empaquetada (cero bytes de padding)
export packed struct TcpHeader {
    src_port: Uint16,
    dst_port: Uint16,
    seq_no: Uint32,
    ack_no: Uint32,
};

// También disponible con la sintaxis de alias de tipo:
export type ArpPacket = packed struct {
    hardware_type: Uint16,
    protocol_type: Uint16,
    hardware_size: Uint8,
    protocol_size: Uint8,
    opcode: Uint16,
};
```

- **Exportación TypeScript (.d.ts)**: Se generan tipos con anotaciones `@packed` y tipado fuerte de enteros de tamaño fijo (`Uint8`, `Uint16`, `Uint32`, `Uint64`).
- **Compatibilidad con Struct Tags**: Las estructuras `packed struct` pueden combinarse con tags de serialización (`json`, `db`).

#### C. Interoperabilidad Nativa C (`extern "C"`)

Permite declarar prototipos de funciones foráneas de librerías en C directamente dentro del código fuente Aura, enlazándolas automáticamente en el backend nativo Cranelift o generando adaptadores seguros en el backend Go:

```aura
extern "C" {
    fn getpid(): Int;
    fn abs(n: Int): Int;
}

export fn currentProcessId(): Int => {
    getpid()
}
```

#### D. Compilación Cruzada Hermética (`--target <triple>`)

Inspirado en la capacidad de Zig de compilar a cualquier sistema operativo y arquitectura sin necesidad de instalar cadenas cruzadas externas engorrosas, el comando `aurac build` acepta el flag `--target`:

```bash
# Compilar para Linux Musl x86_64 desde macOS (Apple Silicon) o Windows
aurac build app.aura -o app_linux --target x86_64-unknown-linux-musl

# Alias cortos ergonómicos (formato SO/arquitectura)
aurac build app.aura -o app_linux --target linux/amd64
aurac build app.aura -o app_arm   --target linux/arm64
aurac build app.aura -o app_mac   --target darwin/arm64
aurac build app.aura -o app.exe   --target windows/amd64
```

> 🧪 **Ejemplo Ejecutable:** Revise y ejecute [`examples/zig_inspired_features.aura`](examples/zig_inspired_features.aura) con `aurac run examples/zig_inspired_features.aura` para comprobar en vivo el comportamiento de `errdefer`, `packed struct` y `extern "C"`.

---

## 🗄️ Librería Estándar, Servidor HTTP y Bases de Datos

### 🌐 Servidor Web HTTP y Mux de Enrutamiento (`net/http`)

Aura incorpora un servidor web completo y enrutador multiplexor inspirado en `net/http` de Go, con soporte para parámetros dinámicos en URL (`:id`), verbos HTTP, middlewares y codificación nativa JSON:

```aura
import { http, os } from "net/http";

// 1. Instanciar ServeMux
let mux = http.newServeMux();

// 2. Middleware de Registro (Logging y Headers)
mux.use(fn(req: Any, res: Any, next: Any) => {
    println(`[HTTP] ${req.method} ${req.url}`);
    res.setHeader("X-Powered-By", "Aura-Native-Engine");
    next();
});

// 3. Ruta GET con Parámetro Dinámico (:id) y Query Params
mux.get("/api/users/:id", fn(req: Any, res: Any) => {
    let userId = http.pathValue(req, "id");
    let format = http.query(req, "format");

    if userId == "0" {
        http.error(res, "Usuario no encontrado", http.StatusNotFound);
        return ();
    }

    http.json(res, http.StatusOK, {
        id: userId,
        name: `Usuario_${userId}`,
        format: format
    });
});

// 4. Ruta POST con Parsing Asíncrono de JSON
mux.post("/api/users", fn(req: Any, res: Any) => async {
    let bodyResult = await http.parseJson(req);
    match bodyResult {
        Ok(data) => http.json(res, http.StatusCreated, { success: true, user: data }),
        Err(err) => http.error(res, `Payload inválido: ${err}`, http.StatusBadRequest)
    }
});

// 5. Iniciar escucha
let port = os.env("PORT") != "" ? os.env("PORT") : "8080";
println(`🚀 Servidor escuchando en :${port}...`);
mux.listenAndServe(`:${port}`);
```

### 🐘 Controlador Nativo de PostgreSQL (`pg` / `postgres`)

```aura
let db: PgPool = postgres.createPool({
    host: "localhost",
    port: 5432,
    user: "postgres",
    password: "secret_password",
    database: "production_db"
});

async fn fetchUsers(): Task<(), String> {
    let result = await db.query("SELECT id, name, email FROM users WHERE active = $1", [true]);
    match result {
        Ok(rows) => for u in rows { println(`- ${u.name} <${u.email}>`); },
        Err(err) => println(`Error en consulta: ${err}`)
    }
}
```

### 🐬 Controlador Nativo de MySQL (`mysql`)

```aura
let pool: MysqlPool = mysql.open("mysql://root:secret@localhost:3306/production_db");

async fn registerUser(name: String, email: String): Task<Int, String> {
    let res = await pool.execute("INSERT INTO users (name, email) VALUES (?, ?)", [name, email]);
    match res {
        Ok(info) => info.insertId,
        Err(err) => -1
    }
}
```

### 🍃 Controlador Nativo de MongoDB (`mongodb` / `mongo`)

```aura
let client: MongoClient = mongodb.open("mongodb://localhost:27017/my_app");
let orders: MongoCollection = client.db().collection("orders");

async fn findOrder(orderId: String): Task<(), String> {
    let doc = await orders.findOne({ orderId: orderId });
    match doc {
        Ok(Some(order)) => println(`Pedido encontrado: Total $${order.total}`),
        Ok(None) => println("Pedido inexistente"),
        Err(err) => println(`Error de base de datos: ${err}`)
    }
}
```

### ⚡ Cliente Nativo de Redis (`redis`)

```aura
let client: RedisClient = redis.open("redis://127.0.0.1:6379");

async fn manageSession(userId: String, token: String): Task<Bool, String> {
    await client.set(`session:${userId}`, token, 3600); // 1 hora de TTL
    let active = await client.get(`session:${userId}`);
    true
}
```

### 📚 Ejemplo de Producción: Microservicio REST `bookstore_api`

El proyecto incluye un servicio REST completo de producción en [`bookstore_api/`](bookstore_api) (también en [`examples/bookstore_service.aura`](examples/bookstore_service.aura)) con suite de pruebas de integración E2E ([`bookstore_api/test_api.sh`](bookstore_api/test_api.sh)):

- CRUD completo para **Libros**, **Autores**, **Editoriales** y **Artículos**.
- Subsistema transaccional de **Venta de Libros** con deducción atómica de inventario, validación de stock, cálculo de IVA (19%) y emisión de recibos.
- Búsqueda textual y filtros dinámicos (`?genre=...`, `?inStock=true`, `?search=...`).
- Compilable a binario autónomo de **~2.99 MB** con cold start en sub-2ms.

---

## 🐛 Depuración Interactiva y Source Maps V3

Aura cuenta con soporte integral de depuración tanto desde la línea de comandos como en editores gráficos mediante **Source Maps V3** de alta precisión.

### Depurador Paso a Paso en Terminal (`aurac step`)

Aura incorpora un depurador interactivo que corre directamente en cualquier terminal:

```bash
aurac step examples/defer_demo.aura
```

```
⚡ Aura Interactive Step Debugger
Target file: defer_demo.aura

📍 [defer_demo.aura:28] in main()
      27 | export fn main(): Unit => {
  ➜   28 |     println("--- Demostración de Defer ---");
      29 |     let res = executeQuery();

(aura-dbg) next
(aura-dbg) into
(aura-dbg) vars
(aura-dbg) break 35
(aura-dbg) continue
(aura-dbg) backtrace
```

| Comando         | Atajos         | Descripción                                                                        |
| :-------------- | :------------- | :--------------------------------------------------------------------------------- |
| `next`          | `n`, `<ENTER>` | **Paso Siguiente (Step Over)**: Avanza a la siguiente línea en la función actual.  |
| `into`          | `s`, `into`    | **Entrar (Step Into)**: Entra en la función que se invoca.                         |
| `out`           | `o`, `finish`  | **Salir (Step Out)**: Ejecuta hasta retornar a la función llamadora.               |
| `continue`      | `c`            | **Continuar**: Reanuda la ejecución hasta el siguiente breakpoint.                 |
| `break <linea>` | `b <linea>`    | **Punto de Interrupción**: Establece o remueve un breakpoint en esa línea.         |
| `vars`          | `locals`       | **Inspeccionar Variables**: Lista las variables del entorno local con sus valores. |
| `backtrace`     | `bt`, `stack`  | **Pila de Llamadas**: Muestra la traza de llamadas mapeada a los archivos `.aura`. |
| `print <expr>`  | `p <expr>`     | **Evaluar Expresión**: Evalúa una expresión en el contexto del frame activo.       |
| `quit`          | `q`, `exit`    | **Salir**: Termina la sesión de depuración.                                        |

### Servidor de Depuración V8 / DAP (`aurac debug`)

Inicia un servidor de depuración para conectar IDEs gráficos o Google Chrome:

```bash
# Iniciar y detenerse en el punto de entrada (puerto 9229)
aurac debug src/main.aura

# Conectar en un puerto específico sin detenerse en la primera línea
aurac debug src/main.aura --port 9300 --no-brk
```

Conectable directamente desde:

- **Google Antigravity IDE / VS Code**: Presionando `F5` con la configuración _"Attach to Aura Process"_.
- **Zed / Neovim**: Vía clientes DAP estándar en `127.0.0.1:9229`.
- **Google Chrome**: Abriendo `chrome://inspect` en el navegador.

---

## 🛠️ Referencia de Herramientas y CLI

### 1. Compilador y Sistema de Construcción (`aurac`)

```bash
# Compilar a binario nativo independiente vía Cranelift (por defecto)
aurac build main.aura -o dist/mi-app

# Compilar a binario nativo mediante la cadena de herramientas de Go
aurac build main.aura -o dist/mi-app --target go

# Compilación cruzada hermética para cualquier plataforma (estilo Zig)
aurac build main.aura -o dist/mi-app-linux --target linux/amd64
aurac build main.aura -o dist/mi-app-arm64 --target linux/arm64
aurac build main.aura -o dist/mi-app.exe   --target windows/amd64
aurac build main.aura -o dist/mi-app-musl  --target x86_64-unknown-linux-musl

# Transpilar directamente a código fuente Go puro (.go)
aurac emit-go main.aura -o dist/main.go

# Ejecución inmediata en desarrollo
aurac run main.aura

# Verificación estática y chequeo de tipos ultra rápido (sin emitir archivos)
aurac check main.aura

# Modo Watch en vivo (recompila y ejecuta automáticamente ante cualquier cambio)
aurac watch main.aura --run

# Depuración paso a paso en consola
aurac step main.aura

# Iniciar servidor de depuración gráfica
aurac debug main.aura --brk

# Compilación condicional mediante build tags
aurac build main.aura --tags "premium,darwin"
```

### 2. Runner de Pruebas, Benchmarks y Cobertura (`auratest`)

```bash
# Ejecutar todas las pruebas del proyecto
auratest ./...
# o mediante aurac:
aurac test ./...

# Modo detallado (verbose) con duración individual por test
auratest -v examples/tests/...

# Filtrar suites de pruebas mediante expresión regular
auratest -run TestUserAuthentication

# Ejecutar suite de benchmarks con métricas de operaciones y tiempo (ns/op)
auratest -bench . ./...

# Generar reporte de cobertura con perfil y exportación a HTML interactivo
auratest --coverage --coverprofile=coverage.out --coverage-html=coverage.html ./...

# Modo de transmisión de eventos JSON para pipelines de CI/CD
auratest -json ./...

# Modo Watch para ejecutar pruebas automáticamente al guardar cambios
auratest -w ./...
```

### 3. Formateador de Código (`aurafmt`, Estilo `gofmt`)

```bash
# Formatear todos los archivos (.aura) en su lugar recursivamente
aurafmt -w .
# o mediante aurac:
aurac fmt -w .

# Mostrar diferencias unificadas (diff) sin alterar los archivos
aurafmt -d src/main.aura

# Listar los archivos que requieren formateo
aurafmt -l .

# Modo verificación para CI/CD (retorna código de salida 1 si hay archivos desalineados)
aurafmt -c .
```

### 4. Servidor de Lenguaje (`auralsp` / `aurac lsp`)

Inicia el servidor LSP compatible con JSON-RPC 2.0:

```bash
aurac lsp
# o
auralsp
```

**Capacidades:** Diagnósticos y errores de tipo en tiempo real, Hover informativo con firmas y documentación, Navegación hacia la definición (_Go to Definition_), Autocompletado inteligente de palabras clave, métodos y constructores, y formateo automático al guardar.

### 5. Playground Web Interactivo

Aura incluye un entorno de desarrollo web completo con editor de código, visualizador de AST, consola de salida y visor de tipos:

```bash
aurac playground --port 3000
```

Abre en tu navegador: [http://localhost:3000](http://localhost:3000).

### 6. Gestor de Paquetes y Módulos Descentralizado (`auramod` / `aurac mod`)

Aura incluye un gestor de módulos descentralizado inspirado en el modelo de **Golang**:

```bash
# Inicializar un nuevo módulo con archivo aura.mod
aurac mod init github.com/miusuario/mi-app

# Descargar e instalar una dependencia remota (Git URL)
aurac mod get github.com/aura-lang/math@v1.0.0

# Sincronizar dependencias automáticamente desde el código (.aura)
aurac mod tidy

# Empaquetar dependencias localmente en ./vendor para builds 100% offline (air-gapped)
aurac mod vendor

# Verificar integridad criptográfica (SHA-256) contra aura.lock
aurac mod verify

# Listar todas las dependencias instaladas y su estado
aurac mod list
```

---

## 🧩 Configuración en Editores

### Google Antigravity IDE

El soporte de primera clase con inteligencia artificial se encuentra en [`editors/antigravity/`](editors/antigravity):

- **Inteligencia LSP Integrada**: Inferencia de tipos en tiempo real, autocompletado y formateo automático con `aurafmt`.
- **Programación en Pareja con IA**: Integración nativa con autocompletado en línea (`⌘+I`), sugerencias contextuales e integración con reglas de agente (`.agents/plugins/aura-lang`).
- **Auto-Reparación de Diagnósticos**: Capacidad de activar agentes de IA para corregir errores de tipado directamente desde el panel de Problemas.
- **Instalación rápida**:
  ```bash
  antigravity --install-extension editors/antigravity/aura-antigravity-0.1.0.vsix
  ```

### Visual Studio Code

Extensión empaquetada disponible en [`editors/vscode/`](editors/vscode):

```bash
cd editors/vscode
npm install && npm run package
# Instalar el archivo aura-vscode-0.2.0.vsix generado
```

### Neovim (`nvim-lspconfig`)

Agrega la configuración incluida en [`editors/nvim/aura.lua`](editors/nvim/aura.lua) a tu `init.lua`:

```lua
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

if not configs.aura then
  configs.aura = {
    default_config = {
      cmd = { 'aurac', 'lsp' },
      filetypes = { 'aura' },
      root_dir = lspconfig.util.root_pattern('.git', 'Cargo.toml'),
      settings = {},
    },
  }
end
lspconfig.aura.setup{}
```

### Editor Zed

Utiliza la definición de extensión empaquetada en [`editors/zed/`](editors/zed).

---

## 🔄 Compilador Self-Hosted y Bootstrapping

Aura cuenta con un **compilador completamente auto-alojado** (_self-hosted_) escrito en el propio lenguaje Aura, ubicado en [`src/aura_compiler/`](src/aura_compiler):

- [`ast.aura`](src/aura_compiler/ast.aura) — Definición formal del AST en Aura.
- [`lexer.aura`](src/aura_compiler/lexer.aura) — Analizador léxico completo.
- [`parser.aura`](src/aura_compiler/parser.aura) — Analizador sintáctico descendente recursivo.
- [`codegen.aura`](src/aura_compiler/codegen.aura) — Generador de código.
- [`main.aura`](src/aura_compiler/main.aura) — Punto de entrada del compilador escrito en Aura.

### Compilación de la Etapa 1 mediante el compilador en Rust:

```bash
cargo run --bin aurac -- compile src/aura_compiler/ast.aura -o dist/ast.mjs
cargo run --bin aurac -- compile src/aura_compiler/lexer.aura -o dist/lexer.mjs
cargo run --bin aurac -- compile src/aura_compiler/parser.aura -o dist/parser.mjs
cargo run --bin aurac -- compile src/aura_compiler/codegen.aura -o dist/codegen.mjs
cargo run --bin aurac -- compile src/aura_compiler/main.aura -o dist/aurac.mjs
```

### Compilar programas Aura utilizando el compilador Self-Hosted:

```bash
node dist/aurac.mjs examples/bootstrap_demo/hello.aura -o dist/hello.js
node dist/hello.js
```

### Verificación Automatizada del Bootstrap:

```bash
cargo test --test bootstrap_tests
```

---

## 🗺️ Estado Actual, Roadmap e Hitos

- [x] **Núcleo del Lenguaje**: Inferencia de tipos Hindley-Milner, tipos algebraicos (ADTs), coincidencia de patrones exhaustiva, optimización TCO, inmutabilidad por defecto y arquitectura 100% backend (modelo Golang).
- [x] **Concurrencia CSP Completa**: Fibers (`spawn`), canales tipados con buffers, canales direccionales, iteración de canales, multiplexación `select`, sincronización (`Mutex`, `WaitGroup`, `Once`, `Pool`), gestión de `Context` y sentencias `defer`.
- [x] **Backend Nativo Cranelift**: Emisión directa de código máquina nativo para macOS (Mach-O) y Linux (ELF) con enlazado estático a `libaura_runtime.a` (scheduler M:N + GC).
- [x] **Backend y Transpilación Golang**: Generación de código idiomatico Go (`aurac emit-go`) y compilación con `go build`.
- [x] **Librería Estándar y Bases de Datos**: Servidor HTTP ServeMux nativo con middlewares y enrutamiento dinámico, controladores para PostgreSQL, MySQL, MongoDB y Redis.
- [x] **Herramientas de Grado de Producción**: Servidor LSP oficial, extensiones para VS Code, Google Antigravity, Neovim y Zed, Playground Web, depurador interactivo de consola (`aurac step`), suite de pruebas con benchmarks y cobertura (`auratest`), y formateador opinado (`aurafmt`).
- [x] **Arquitectura Simplificada y Robusta Rust + Go**: Eliminación de sobreingeniería no tipada; integración unificada de tipos algebraicos (ADTs), exhaustividad estática, slices Go (`[low:high:max]`), concurrencia CSP nativa (`spawn`, `Channel<T>`, `select`), interfaces implícitas, `defer`/`errdefer` y backend dual nativo (Cranelift y Go).
- [x] **Suite de Calidad y Tests**: 177 pruebas automatizadas pasando exitosamente en el repositorio.
- [ ] **Backend WebAssembly (WASM)**: Emisión de módulos WASM/WASI para microservicios serverless en el edge.
- [x] **Gestor de Paquetes Descentralizado**: Sistema de gestión de dependencias y módulos (`aurac mod` / `aurac pkg` / `auramod`) con manifiesto `aura.mod`, lockfile criptográfico `aura.lock`, vendoring offline y verificación de integridad.
- [ ] **Optimizaciones Avanzadas en LLVM**: Pipeline opcional de optimización LTO y vectorización SIMD para cargas de cómputo intensivo.

---

## 📄 Licencia

Aura Language es software libre y de código abierto distribuido bajo la [Licencia MIT](LICENSE).

### ⚖️ Marcas Registradas (Trademarks)

> Go is a trademark of Google LLC. Rust is a trademark of the Rust Foundation. TypeScript is a trademark of Microsoft Corp. Aura Lang is an independent project and is not affiliated with or endorsed by these entities.

