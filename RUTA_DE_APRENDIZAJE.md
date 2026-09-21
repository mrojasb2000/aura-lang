# 🗺️ Ruta de Aprendizaje de Aura Lang: Desarrollo de Servicios y Aplicaciones de Sistemas de Alto Rendimiento

> **Objetivo:** Esta guía proporciona un itinerario paso a paso, exhaustivo y estructurado para dominar **Aura Lang** (`aurac`). Aprenderás desde la sintaxis básica y el sistema de tipos formal, pasando por el modelo de concurrencia CSP (estilo Go) y las herramientas de sistemas (`defer`, `Context`, punteros), hasta la arquitectura de microservicios, bases de datos nativas y patrones de alto rendimiento con latencias sub-milisegundo.

---

## 📑 Tabla de Contenidos

1. [Fase 1: Fundamentos del Lenguaje y Sistema de Tipos](#-fase-1-fundamentos-del-lenguaje-y-sistema-de-tipos)
   - 1.1. [Herramientas y CLI (`aurac`, `auratest`, `aurafmt`, `auralsp`)](#11-herramientas-y-cli)
   - 1.2. [Variables, Inmutabilidad y Tipado Estático Hindley-Milner](#12-variables-inmutabilidad-y-tipado-estático)
   - 1.3. [Slices y Colecciones Dinámicas (Modelo Golang)](#13-slices-y-colecciones-dinámicas-modelo-golang)
   - 1.4. [Expresiones de Bloque, Control de Flujo y Bucles Etiquetados](#14-expresiones-de-bloque-control-de-flujo-y-bucles-etiquetados)
   - 1.5. [Ausencia Total de Null: `Option<T>` y `Result<T, E>`](#15-ausencia-total-de-null-optiont-y-resultt-e)
   - 1.6. [Tipos Suma (ADTs) y Pattern Matching Exhaustivo](#16-tipos-suma-adts-y-pattern-matching-exhaustivo)
2. [Fase 2: Programación Funcional Ergonómica de Alto Rendimiento](#-fase-2-programación-funcional-ergonómica-de-alto-rendimiento)
   - 2.1. [Funciones de Primera Clase y Sintaxis Flecha](#21-funciones-de-primera-clase-y-sintaxis-flecha)
   - 2.2. [Operador Pipeline (`|>`)](#22-operador-pipeline-)
   - 2.3. [Optimización de Llamadas por la Cola (TCO - Tail-Call Optimization)](#23-optimización-de-llamadas-por-la-cola-tco)
   - 2.4. [Átomos Internados (`:keyword`) con Comparación $O(1)$](#24-átomos-internados-keyword-con-comparación-o1)
   - 2.5. [Transductores sin Alocaciones Intermedias (`compose`, `into`)](#25-transductores-sin-alocaciones-intermedias)
3. [Fase 3: Concurrencia CSP y Programación de Sistemas (Modelo Golang)](#-fase-3-concurrencia-csp-y-programación-de-sistemas-modelo-golang)
   - 3.1. [Fibers Ultralivianos (`spawn`) y Planificación M:N](#31-fibers-ultralivianos-spawn-y-planificación-mn)
   - 3.2. [Canales Tipados (`Channel<T>`), Buffers y Tipos Direccionales](#32-canales-tipados-channelt-buffers-y-tipos-direccionales)
   - 3.3. [Iteración sobre Canales (`for val in ch`)](#33-iteración-sobre-canales-for-val-in-ch)
   - 3.4. [Multiplexación con `select` (Send, Recv, Timeout, Default)](#34-multiplexación-con-select)
   - 3.5. [Sincronización con `Mutex` y `WaitGroup`](#35-sincronización-con-mutex-y-waitgroup)
   - 3.6. [Limpieza Garantizada con `defer` (Garantía LIFO)](#36-limpieza-garantizada-con-defer)
   - 3.7. [Cancelación Estructurada y Tiempos Límite con `Context`](#37-cancelación-estructurada-y-tiempos-límite-con-context)
   - 3.8. [Pánicos y Recuperación (`panic` / `recover`) y Punteros Explícitos (`&`, `*`)](#38-pánicos-recuperación-y-punteros-explícitos)
   - 3.9. [Variables Dinámicas (`DynamicVar<T>`) para Eliminar el Context Drilling](#39-variables-dinámicas-dynamicvart)
   - 3.10. [El Sistema de Condiciones y Reinicios (*Conditions & Restarts*)](#310-el-sistema-de-condiciones-y-reinicios)
4. [Fase 4: Tipado Estructural, Interfaces y Metadatos](#-fase-4-tipado-estructural-interfaces-y-metadatos)
   - 4.1. [Structs Puros y Métodos con Receptor (Estilo Go)](#41-structs-puros-y-métodos-con-receptor)
   - 4.2. [Interfaces Implícitas (Duck Typing Estructural)](#42-interfaces-implícitas-duck-typing-estructural)
   - 4.3. [Genéricos y Polimorfismo Paramétrico](#43-genéricos-y-polimorfismo-paramétrico)
   - 4.4. [Struct Tags (`json:"..." db:"..."`) y Build Tags (`// +build`)](#44-struct-tags-y-build-tags)
   - 4.5. [Incrustación de Recursos Binarios en Tiempo de Compilación (`embed`)](#45-incrustación-de-recursos-binarios-con-embed)
5. [Fase 5: Desarrollo de Servicios Backend y Microservicios](#-fase-5-desarrollo-de-servicios-backend-y-microservicios)
   - 5.1. [Servidor Web Nativo `net/http` (`http.newServeMux`)](#51-servidor-web-nativo-nethttp)
   - 5.2. [Middlewares, Enrutamiento Dinámico y Parámetros](#52-middlewares-enrutamiento-dinámico-y-parámetros)
   - 5.3. [Documentación Interactiva Swagger UI y OpenAPI 3.0](#53-documentación-interactiva-swagger-ui-y-openapi-30)
   - 5.4. [Bases de Datos Relacionales: PostgreSQL y MySQL](#54-bases-de-datos-relacionales-postgresql-y-mysql)
   - 5.5. [Bases de Datos NoSQL y Caché: MongoDB y Redis](#55-bases-de-datos-nosql-y-caché-mongodb-y-redis)
   - 5.6. [Arquitectura Hexagonal (Ports & Adapters) y Pure Dependency Injection](#56-arquitectura-hexagonal-y-pure-dependency-injection)
6. [Fase 6: Patrones y Arquitecturas de Alto Rendimiento](#-fase-6-patrones-y-arquitecturas-de-alto-rendimiento)
   - 6.1. [Worker Pool Concurrente con Backpressure Acotado](#61-worker-pool-concurrente-con-backpressure-acotado)
   - 6.2. [Canales de Transmisión de Flujos sin Asignaciones (`channelTransduce`)](#62-canales-de-transmisión-de-flujos-sin-asignaciones)
   - 6.3. [Optimización de Memoria: Pre-alocación con `make`, Zero-Allocations y Átomos](#63-optimización-de-memoria-pre-alocación-con-make)
   - 6.4. [Estrategias de Compilación a Binario Standalone: Cranelift vs Go Toolchain](#64-estrategias-de-compilación-a-binario-standalone)
   - 6.5. [Depuración Interactiva y Source Maps V3 (`aurac step` / `aurac debug`)](#65-depuración-interactiva-y-source-maps-v3)

---

## 🚀 Fase 1: Fundamentos del Lenguaje y Sistema de Tipos

### 1.1. Herramientas y CLI

Aura provee un conjunto integral de utilidades de desarrollo en línea de comandos escritas en Rust para máxima velocidad de ejecución:

| Comando | Descripción |
|---|---|
| `aurac check <archivo.aura>` | Verificación estática rápida e inferencia de tipos sin emitir código. |
| `aurac run <archivo.aura>` | Compila y ejecuta el programa directamente (estilo `go run`). |
| `aurac build <archivo.aura> -o <binario>` | Genera un binario nativo autónomo de 64 bits (sin dependencias externas). |
| `aurac build <archivo.aura> --target go` | Compila utilizando la cadena de herramientas de Golang. |
| `aurac watch <archivo.aura> --run` | Modo watch reactivo que recompila y ejecuta ante cada guardado. |
| `auratest [rutas...]` | Ejecuta la suite de pruebas unitarias y benchmarks con reporte de cobertura. |
| `aurafmt -w <archivo.aura>` | Formatea el código de manera opinada e idempotente (estilo `gofmt`). |
| `aurac step <archivo.aura>` | Depurador paso a paso interactivo en la terminal. |

```bash
# Ejemplo: Verificar y ejecutar tu primer archivo
aurac check main.aura
aurac run main.aura
```

---

### 1.2. Variables, Inmutabilidad y Tipado Estático

En Aura, las vinculaciones con `let` son **inmutables por defecto**. Si necesitas mutar una variable, debes usar explícitamente `let mut`. El tipo es inferido automáticamente por el motor Hindley-Milner, aunque puede anotarse explícitamente.

```aura
// Inmutables (garantía de seguridad en entornos concurrentes)
let appName: String = "Aura Core Engine";
let maxConnections = 1000; // Inferido como Int
let pi: Float = 3.14159265;

// Mutables (usar únicamente cuando sea necesario)
let mut requestsCount: Int = 0;
requestsCount = requestsCount + 1;

// Tipos numéricos de tamaño fijo (adecuados para programación de sistemas)
let port: Int32 = 8080;
let byteVal: Int8 = 127;
let memoryOffset: Uint64 = 1048576;
let asciiRune: Rune = 65; // 'A' (representación UTF-32)

// Tuplas inmutables
let metric: (String, Int, Float) = ("cpu_usage", 4, 82.5);
let (metricName, cores, usage) = metric;
```

---

### 1.3. Slices y Colecciones Dinámicas (Modelo Golang)

Aura implementa nativamente el modelo de **Slices de Golang**. La sintaxis `[]T` es sinónimo de `List<T>`. Ofrece soporte de segmentación `[low:high]`, capacidad máxima `[low:high:max]` y las funciones built-in de Go: `len`, `cap`, `append` y `make`.

```aura
// 1. Declaración de Slice
let mut numbers: []Int = [10, 20, 30, 40, 50];

// 2. Funciones built-in estilo Go
println(`Longitud: ${len(numbers)}`);  // 5
println(`Capacidad: ${cap(numbers)}`); // 5

// 3. Segmentación (Intervalo semiabierto [low, high))
let sub1 = numbers[1:4];     // [20, 30, 40]
let sub2 = numbers[:3];      // [10, 20, 30]
let sub3 = numbers[2:];      // [30, 40, 50]
let full = numbers[:];       // [10, 20, 30, 40, 50]
let withMax = numbers[1:3:5];// [20, 30] con límite de capacidad 5

// 4. Agregar elementos con append (modelo idiomático)
numbers = append(numbers, 60);

// 5. Prealocación eficiente con make (tamaño 5, capacidad inicial 10)
let buffer = make([]Int, 5, 10);

// 6. String slicing nativo
let banner: String = "Aura Language";
let subBanner = banner[0:4]; // "Aura"
```

> **Ver ejemplo en el repositorio:** [`examples/slices_golang_model.aura`](file:///Users/mavro/Projects/aura-lang/examples/slices_golang_model.aura)

---

### 1.4. Expresiones de Bloque, Control de Flujo y Bucles Etiquetados

En Aura, los bloques de código `{ ... }` y las estructuras condicionales `if/else` son **expresiones** que retornan el valor de la última sentencia sin punto y coma.

```aura
// Bloque como expresión
let responsePayload = {
    let base = 500.0;
    let surcharge = 25.0;
    base + surcharge // Retorna 525.0
};

// If-Else como expresión (reemplaza al operador ternario)
let httpStatus = if isSuccess { 200 } else { 500 };

// Bucle while
let mut counter = 0;
while counter < 5 {
    counter = counter + 1;
};

// Bucles con etiquetas (Labeled Loops) y break / continue dirigido
let matrix = [
    [1, 2, 3],
    [4, 99, 6],
    [7, 8, 9]
];

searchLoop: for (rowIdx, row) in matrix {
    for (colIdx, val) in row {
        if val == 99 {
            println(`Valor 99 encontrado en fila ${rowIdx}, col ${colIdx}`);
            break searchLoop; // Termina ambos bucles instantáneamente
        }
    }
};
```

---

### 1.5. Ausencia Total de Null: `Option<T>` y `Result<T, E>`

En Aura **no existen `null` ni `undefined`**. Las ausencias y los errores se representan mediante tipos suma formales:

```aura
// 1. Manejo de valores opcionales con Option<T>
fn findConfigValue(key: String): Option<String> => {
    if key == "PORT" {
        Some("8080")
    } else {
        None
    }
}

// 2. Manejo de resultados y fallos con Result<T, E>
fn parsePort(portStr: String): Result<Int, String> => {
    if portStr == "8080" {
        Ok(8080)
    } else {
        Err(`Puerto no admitido: ${portStr}`)
    }
}

export fn main(): Unit => {
    match findConfigValue("PORT") {
        Some(val) => match parsePort(val) {
            Ok(port) => println(`Servidor configurado en el puerto ${port}`),
            Err(err) => println(`Error de configuración: ${err}`)
        },
        None => println("Variable no encontrada")
    }
}
```

---

### 1.6. Tipos Suma (ADTs) y Pattern Matching Exhaustivo

Los tipos algebraicos de datos (ADTs) permiten modelar la máquina de estados de un sistema. El compilador comprueba la **exhaustividad**: si olvidas manejar un caso, el código no compila.

```aura
export type PaymentStatus =
    | Pending
    | Processing { gateway: String, attempt: Int }
    | Completed { transactionId: String, amount: Float }
    | Failed(String);

export fn describePayment(status: PaymentStatus): String =>
    match status {
        Pending => "Pago en cola de procesamiento.",
        Processing { gateway, attempt } => `Procesando en ${gateway}, intento #${attempt}...`,
        Completed { transactionId, amount } => `Pago confirmado: $${amount} (ID: ${transactionId})`,
        Failed(reason) => `Pago rechazado: ${reason}`
    };
```

---

## ⚡ Fase 2: Programación Funcional Ergonómica de Alto Rendimiento

### 2.1. Funciones de Primera Clase y Sintaxis Flecha

Las funciones en Aura pueden ser pasadas como argumentos, retornadas y compuestas:

```aura
// Función flecha concisa
let square = fn(x: Int): Int => x * x;

// Función con cuerpo de bloque
fn computeMetric(factor: Float, compute: fn(Float): Float): Float => {
    let result = compute(factor);
    result * 1.05
}
```

---

### 2.2. Operador Pipeline (`|>`)

El operador pipeline pasa el resultado de la expresión izquierda como el primer parámetro de la función derecha, eliminando la anidación excesiva de llamadas:

```aura
let users = [
    { name: "Carlos", age: 30, active: true },
    { name: "Ada", age: 25, active: true },
    { name: "Bob", age: 17, active: false }
];

let activeNames = users
    |> filter(fn(u) => u.active && u.age >= 18)
    |> map(fn(u) => u.name)
    |> join(", ");

println(`Usuarios activos: ${activeNames}`); // "Carlos, Ada"
```

---

### 2.3. Optimización de Llamadas por la Cola (TCO)

Las funciones recursivas cuya última operación es llamarse a sí mismas son reconocidas por el analizador semántico de Aura y transformadas automáticamente en un bucle iterativo `while (true)` con **consumo de pila $O(1)$**, evitando desbordamientos de pila (*Stack Overflow*).

```aura
// Función recursiva en posición de cola (TCO automática)
export fn sumSlice(items: []Int, acc: Int = 0): Int =>
    match items {
        [] => acc,
        [head, ...tail] => sumSlice(tail, acc + head) // Se compila a bucle sin sobrecarga
    };

export fn main(): Unit => {
    let bigList = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let total = sumSlice(bigList, 0);
    println(`Suma calculada: ${total}`); // 55
}
```

---

### 2.4. Átomos Internados (`:keyword`) con Comparación $O(1)$

Inspirados en LISP y Clojure, los **Átomos** (`Atom` o `:nombre`) son identificadores inmutables internados en una tabla hash global durante la compilación. A diferencia de las cadenas (`String`), comparar dos átomos es una comparación numérica directa de punteros o enteros de 64 bits en tiempo constante **$O(1)$**, sin alocaciones en el heap:

```aura
type WorkerState = Atom;

fn inspectState(state: WorkerState): String => {
    match state {
        :idle     => "Worker disponible en el pool",
        :busy     => "Worker ejecutando tarea crítica",
        :draining => "Worker drenando canales antes de salir",
        _         => "Estado desconocido"
    }
}

export fn main(): Unit => {
    let current = :busy;
    // Comparación O(1) ultra veloz
    if current == :busy {
        println(inspectState(current));
    }
}
```

---

### 2.5. Transductores sin Alocaciones Intermedias

Los **Transductores** desacoplan las transformaciones lógicas (`filtering`, `mapping`, `taking`) de la estructura de datos. Al aplicar transformaciones sobre 1 millón de elementos, no se crean arreglos temporales intermedios en memoria, reduciendo el consumo de memoria hasta en un 85%:

```aura
// Componer la transformación algorítmica una sola vez:
let processingPipeline = compose(
    filtering(fn(x: Int): Bool => x % 2 == 0), // Filtrar solo pares
    mapping(fn(x: Int): Int => x * 10),        // Multiplicar por 10
    taking(3)                                  // Tomar los primeros 3 resultados
);

// Aplicar en una única pasada en memoria:
let sourceData = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let finalResult = into([], processingPipeline, sourceData);

println(`Resultado: ${finalResult}`); // [20, 40, 60]
```

> **Ver especificación teórica completa:** [`LISP_KNOWLEDGE_BASE.md`](file:///Users/mavro/Projects/aura-lang/LISP_KNOWLEDGE_BASE.md)

---

## 🔄 Fase 3: Concurrencia CSP y Programación de Sistemas (Modelo Golang)

### 3.1. Fibers Ultralivianos (`spawn`) y Planificación M:N

Aura utiliza fibers concurrentes multiplexados sobre hilos del sistema operativo mediante un scheduler M:N. Lanzar un fiber cuesta solo un par de kilobytes de memoria:

```aura
export fn main(): Unit => {
    // Generar 1000 fibers en paralelo
    let mut i = 0;
    while i < 1000 {
        let fiberId = i;
        spawn {
            // Tarea liviana ejecutada en segundo plano
            println(`Fiber ${fiberId} en ejecución`);
        };
        i = i + 1;
    };
}
```

---

### 3.2. Canales Tipados (`Channel<T>`), Buffers y Tipos Direccionales

Los canales permiten comunicar fibers concurrentes sin compartir memoria. Soporta canales sin buffer (sincronizados) y con buffer (asíncronos acotados):

```aura
// 1. Canal con buffer de capacidad 10
let ch = Channel.make<String>(10);

// 2. Operación de envío (<- como operador de flecha)
ch <- "Mensaje de telemetría";

// 3. Operación de recepción
let msg = <-ch; // Retorna Option<String> (None si el canal fue cerrado)

// 4. Tipos direccionales para firmas seguras de APIs:
// SendChannel<T> solo permite envío; RecvChannel<T> solo permite recepción
fn producer(outCh: SendChannel<Int>) {
    outCh <- 42;
    Channel.close(outCh);
}

fn consumer(inCh: RecvChannel<Int>) {
    let value = <-inCh;
    println(`Valor recibido: ${value}`);
}
```

---

### 3.3. Iteración sobre Canales (`for val in ch`)

Puedes iterar directamente sobre un canal utilizando un bucle `for-in`. El bucle continuará leyendo mensajes hasta que el canal sea explícitamente cerrado con `Channel.close(ch)`:

```aura
fn dataProducer(ch: SendChannel<Int>) {
    let mut i = 1;
    while i <= 5 {
        ch <- i * 100;
        i = i + 1;
    };
    Channel.close(ch);
}

export fn main(): Unit => {
    let stream = Channel.make<Int>(5);
    spawn { dataProducer(stream); };

    // Itera automáticamente hasta que el canal se cierra
    for val in stream {
        println(`Dato procesado: ${val}`);
    };
    println("Flujo completado.");
}
```

> **Ver ejemplo en el repositorio:** [`examples/channel_range_demo.aura`](file:///Users/mavro/Projects/aura-lang/examples/channel_range_demo.aura)

---

### 3.4. Multiplexación con `select`

La sentencia `select` multiplexa operaciones de canales de manera no bloqueante, soportando lecturas, envíos, temporizadores (`timeout`) y fallback (`default`):

```aura
let dataCh = Channel.make<String>(1);
let alertCh = Channel.make<String>(1);

select {
    case msg = <-dataCh => match msg {
        Some(val) => println(`Dato normal recibido: ${val}`),
        None => println("Canal de datos cerrado.")
    },
    case alert = <-alertCh => match alert {
        Some(warn) => println(`¡ALERTA CRÍTICA!: ${warn}`),
        None => ()
    },
    case timeout(250) => {
        println("Timeout agotado: ningún mensaje llegó en 250 milisegundos.");
    },
    default => {
        println("Ningún canal tenía datos listos de inmediato.");
    }
}
```

---

### 3.5. Sincronización con `Mutex` y `WaitGroup`

Para estado mutable compartido o sincronización de múltiples fibers, Aura provee primitivas estándar equivalentes al paquete `sync` de Go:

```aura
let wg = WaitGroup.new();
let mu = Mutex.new();
let mut sharedCounter = 0;

wg.add(3);

let mut worker = 0;
while worker < 3 {
    spawn {
        defer wg.done();

        // Sección crítica protegida por Mutex
        mu.lock();
        sharedCounter = sharedCounter + 1;
        mu.unlock();
    };
    worker = worker + 1;
};

// Espera determinista a que todos los fibers terminen
await wg.wait();
println(`Valor final protegido: ${sharedCounter}`); // 3
```

---

### 3.6. Limpieza Garantizada con `defer`

`defer` programa la ejecución de una instrucción o bloque para justo antes de que la función actual retorne. Sigue un orden estricto **LIFO** (*Last-In, First-Out*), ideal para liberar sockets, cerrar archivos y desbloquear mutexes:

```aura
fn executeTransaction(): String => {
    println("1. Abriendo conexión a la base de datos...");
    defer println("4. Conexión cerrada (LIFO último en entrar, primero en salir)");

    println("2. Iniciando transacción...");
    defer println("3. Transacción confirmada (LIFO primero)");

    "Transacción OK"
};

export fn main(): Unit => {
    let result = executeTransaction();
    println(result);
}
```

> **Ver ejemplo en el repositorio:** [`examples/defer_demo.aura`](file:///Users/mavro/Projects/aura-lang/examples/defer_demo.aura)

---

### 3.7. Cancelación Estructurada y Tiempos Límite con `Context`

El módulo `Context` permite propagar señales de cancelación y tiempos límite a través de cadenas complejas de llamadas y fibers:

```aura
fn backgroundTask(ctx: Context): Task<Unit, String> => {
    let ch = Channel.make<String>(1);
    spawn {
        await sleep(100);
        ch <- "Tarea exitosa";
    };

    select {
        case <-ctx.done() => {
            println(`Operación cancelada. Causa: ${ctx.err()}`);
        },
        case res = <-ch => {
            println(`Resultado recibido: ${res}`);
        }
    };
};

export fn main(): Unit => {
    let root = Context.background();
    // Contexto con timeout automático de 50ms
    let (ctx, cancel) = Context.withTimeout(root, 50);
    defer cancel(); // Siempre limpiar la función cancel

    await backgroundTask(ctx);
}
```

> **Ver ejemplo en el repositorio:** [`examples/context_demo.aura`](file:///Users/mavro/Projects/aura-lang/examples/context_demo.aura)

---

### 3.8. Pánicos, Recuperación y Punteros Explícitos

Aura provee mecanismos de bajo nivel para situaciones imprevistas y acceso directo a direcciones de memoria:

```aura
// 1. Manejo seguro de pánicos
fn safeWorker() {
    defer {
        let err = recover();
        if err != None {
            println(`Recuperado con éxito del pánico: ${err}`);
        }
    };
    panic("Fallo crítico en subsistema de red");
}

// 2. Operaciones de punteros explícitos (& y *)
fn incrementRef(ptr: *Int) {
    *ptr = *ptr + 1;
}

export fn main(): Unit => {
    safeWorker();

    let mut counter = 100;
    incrementRef(&counter);
    println(`Valor incrementado por referencia: ${counter}`); // 101
}
```

---

### 3.9. Variables Dinámicas (`DynamicVar<T>`)

Inspiradas en Common Lisp y Clojure, las variables dinámicas permiten propagar identificadores de rastreo (`requestId`), autenticación (`userId`) o métricas de observabilidad en fibers locales sin necesidad de ensuciar las firmas de las funciones con parámetros innecesarios (*anti-patrón Context Drilling*):

```aura
let REQUEST_ID = DynamicVar.new("default-trace-id");

fn logWithTrace(msg: String): Unit => {
    // Obtiene el valor dinámico activo en el ámbito actual de ejecución
    println(`[Trace: ${REQUEST_ID.get()}] ${msg}`);
}

fn deeplyNestedService(): Unit => {
    logWithTrace("Ejecutando consulta a tabla de órdenes");
}

export fn main(): Unit => {
    // Enlace temporal acotado a la petición actual:
    REQUEST_ID.bind("req-txn-9841", fn() => {
        logWithTrace("Inicio de procesamiento HTTP");
        deeplyNestedService();
    });

    // Al salir, vuelve automáticamente a su valor por defecto:
    logWithTrace("Fuera del ámbito temporal"); // Muestra 'default-trace-id'
}
```

---

### 3.10. El Sistema de Condiciones y Reinicios

A diferencia de las excepciones tradicionales que destruyen la pila (*stack unwinding*), el sistema de condiciones permite que las capas bajas emitan una anomalía (`signalCondition`) y declaren posibles estrategias de recuperación (`withRestarts`). Una capa superior decide qué reinicio invocar **sin destruir la pila ni las variables locales**:

```aura
fn queryDatabaseWithFailover(service: String): String => {
    withRestarts({
        "UseCache": fn(cachedVal: String): String => {
            println("[RESTART] Retornando valor del caché local...");
            cachedVal
        },
        "Retry": fn(): String => {
            println("[RESTART] Reintentando conexión con pool réplica...");
            "replica_data_ok"
        }
    }, fn(): String => {
        if service == "orders_replica" {
            // Emite la condición preservando el stack de ejecución:
            return signalCondition("DatabaseTimeout", { "target": service });
        }
        return "primary_data_ok";
    })
}

// El middleware superior establece la política de recuperación:
let output = handleCondition("DatabaseTimeout", fn(cond: Any, restarts: Any): Any => {
    println(`Condición detectada: '${cond.name}'. Activando 'UseCache'...`);
    return restarts.invoke("UseCache", "datos_en_cache_v1");
}, fn(): String => {
    return queryDatabaseWithFailover("orders_replica");
});

println(`Resultado recuperado: ${output}`); // "datos_en_cache_v1"
```

---

## 🧩 Fase 4: Tipado Estructural, Interfaces y Metadatos

### 4.1. Structs Puros y Métodos con Receptor

Siguiendo el modelo exacto de **Golang**, los `struct` en Aura definen únicamente campos de datos. Los métodos se declaran desacoplados mediante funciones con receptor por valor `fn (s: S) ...` o por puntero `fn (s: *S) ...`:

```aura
struct MetricsCollector {
    serviceName: String,
    totalErrors: Int,
};

// Método receptor por valor:
fn (m: MetricsCollector) report(): String => {
    `[${m.serviceName}] Errores acumulados: ${m.totalErrors}`
}

// Método receptor por puntero (para mutaciones in-place):
fn (m: *MetricsCollector) recordError(): Unit => {
    m.totalErrors = m.totalErrors + 1;
}

export fn main(): Unit => {
    let mut collector: MetricsCollector = {
        serviceName: "PaymentGateway",
        totalErrors: 0
    };

    (&collector).recordError();
    println(collector.report()); // "[PaymentGateway] Errores acumulados: 1"
}
```

---

### 4.2. Interfaces Implícitas (Duck Typing Estructural)

Cualquier tipo que posea los métodos requeridos por una `interface` satisface automáticamente el contrato en tiempo de compilación sin necesidad de palabras clave como `implements`:

```aura
// 1. Contrato de interfaz
interface Reader {
    read(bytes: Int): []Int8;
}

// 2. Struct con datos
struct NetworkSocket {
    address: String,
};

// 3. Receptor que satisface la interfaz implícitamente
fn (s: NetworkSocket) read(bytes: Int): []Int8 => {
    println(`Leyendo ${bytes} bytes de ${s.address}`);
    [10, 20, 30]
}

// 4. Consumidor polimórfico
fn parseStream(r: Reader): []Int8 => {
    r.read(128)
}

export fn main(): Unit => {
    let sock: NetworkSocket = { address: "127.0.0.1:9000" };
    // NetworkSocket satisface Reader automáticamente:
    let data = parseStream(sock);
}
```

---

### 4.3. Genéricos y Polimorfismo Paramétrico

Aura soporta parámetros de tipos genéricos completos `<T>`, tanto en funciones como en tipos algebraicos y contenedores:

```aura
type Envelope<T> = {
    id: String,
    timestamp: Int,
    payload: T
};

fn wrapPayload<T>(id: String, data: T): Envelope<T> => {
    {
        id: id,
        timestamp: 1720000000,
        payload: data
    }
}

export fn main(): Unit => {
    let intEnv = wrapPayload<Int>("msg-1", 42);
    let strEnv = wrapPayload<String>("msg-2", "Aura Rocks");
}
```

---

### 4.4. Struct Tags y Build Tags

Los metadatos en structs permiten configurar la serialización JSON de manera declarativa, mientras que los build tags permiten la compilación condicional de código para diferentes arquitecturas o entornos:

```aura
// +build linux,amd64

// Struct con struct tags para serialización
type DatabaseUser = {
    userId: Int,
    emailAddress: String,
    passwordHash: String
} `json:"user_data" db:"users"`;
```

---

### 4.5. Incrustación de Recursos Binarios con `embed`

Permite empacar archivos estáticos, plantillas HTML, scripts de migración o imágenes directamente dentro del binario nativo compilado, evitando lecturas del sistema de archivos en tiempo de ejecución:

```aura
// Incrusta el contenido de un archivo de texto en tiempo de compilación
let serverConfig: String = embed("config/server.yaml");

// Incrusta un binario como arreglo de bytes
let brandLogo: []Int8 = embedBytes("assets/logo.png");
```

---

## 🌐 Fase 5: Desarrollo de Servicios Backend y Microservicios

### 5.1. Servidor Web Nativo `net/http`

Aura incluye un servidor HTTP nativo de alto rendimiento y un multiplexor de rutas (`ServeMux`) inspirado directamente en `net/http` de Go:

```aura
import { http, os } from "net/http";

export fn main(): Unit => {
    let mux = http.newServeMux();

    // Endpoint GET simple
    mux.get("/health", fn(req: Any, res: Any) => {
        http.json(res, http.StatusOK, {
            status: "UP",
            engine: "Aura-Native-Mach-O",
            uptimeSeconds: 120
        });
    });

    let port = os.env("PORT") != "" ? os.env("PORT") : "8080";
    println(`🚀 Servidor HTTP escuchando en :${port}...`);
    mux.listenAndServe(`:${port}`);
}
```

---

### 5.2. Middlewares, Enrutamiento Dinámico y Parámetros

El enrutador soporta middlewares en cadena (`mux.use`), parámetros dinámicos de ruta (`:id`) y extracción de cadenas de consulta (*query params*):

```aura
let mux = http.newServeMux();

// 1. Middleware de registro de peticiones y cabeceras
mux.use(fn(req: Any, res: Any, next: Any) => {
    println(`[HTTP] ${req.method} ${req.url}`);
    res.setHeader("X-Powered-By", "Aura-Lang");
    next();
});

// 2. Ruta con parámetro de URL y lectura asíncrona de JSON
mux.get("/api/users/:id", fn(req: Any, res: Any) => {
    let userId = http.pathValue(req, "id");
    let detail = http.query(req, "detail");

    http.json(res, http.StatusOK, {
        id: userId,
        name: `Usuario #${userId}`,
        hasDetail: detail == "true"
    });
});

// 3. Ruta POST con validación de cuerpo
mux.post("/api/users", fn(req: Any, res: Any) => async {
    let parseResult = await http.parseJson(req);
    match parseResult {
        Ok(body) => http.json(res, http.StatusCreated, { success: true, created: body }),
        Err(err) => http.error(res, `Cuerpo inválido: ${err}`, http.StatusBadRequest)
    }
});
```

---

### 5.3. Documentación Interactiva Swagger UI y OpenAPI 3.0

Aura cuenta con soporte de primera clase para **Swagger UI** y generación automática de especificaciones **OpenAPI 3.0.3** en su enrutador nativo `net/http`:

```aura
let mux = http.newServeMux();

// Registrar rutas de la API
mux.get("/api/users", handleListUsers);
mux.get("/api/users/:id", handleGetUser);
mux.post("/api/users", handleCreateUser);

// Habilitar Swagger UI con un único comando
mux.enableSwagger("/swagger");
// O con metadatos personalizados:
// mux.swagger({ title: "Users API", version: "1.0.0", path: "/swagger" });
```

Al invocar `mux.enableSwagger("/swagger")`, Aura automáticamente:
1. Inspecciona todas las rutas registradas y genera la especificación OpenAPI 3.0 (`/swagger/doc.json` y `/swagger/openapi.json`).
2. Mapea parámetros dinámicos de ruta (`:id` ➜ `{id}`) y esquemas de petición/respuesta.
3. Configura esquemas de seguridad estándar (JWT Bearer Token y API Key).
4. Sirve la interfaz interactiva de **Swagger UI** en `/swagger` (y `/swagger/`) con soporte para pruebas en vivo ("Try it out") y autorización Bearer.

---

### 5.4. Bases de Datos Relacionales: PostgreSQL y MySQL

Aura incorpora controladores nativos en su preludio con soporte para transacciones ACID seguras y consultas parametrizadas:

#### PostgreSQL (`postgres` / `pg`):
```aura
let db: PgPool = postgres.createPool({
    host: "localhost",
    port: 5432,
    user: "postgres",
    password: "secret_password",
    database: "production_db",
    uri: "",
    connectionLimit: 10
});

async fn getActiveUsers(): Task<Unit, String> {
    // Consulta con placeholders nativos ($1)
    let result = await db.query("SELECT id, name, email FROM users WHERE active = $1", [true]);
    match result {
        Ok(rows) => for u in rows { println(`Usuario: ${u.name} <${u.email}>`); },
        Err(err) => println(`Error SQL: ${err}`)
    };
}

// Transacción ACID atómica
async fn updateBalance(accountId: Int, amount: Int): Task<Bool, String> {
    let txResult = await db.transaction(async fn(tx: PgTransaction): Task<Unit, String> {
        await tx.execute("UPDATE accounts SET balance = balance + $1 WHERE id = $2", [amount, accountId]);
    });
    match txResult {
        Ok(_) => true,
        Err(_) => false
    }
}
```

> **Ver guía de bases de datos completa:** [`DATABASE_GUIDE.md`](file:///Users/mavro/Projects/aura-lang/DATABASE_GUIDE.md)

---

### 5.5. Bases de Datos NoSQL y Caché: MongoDB y Redis

#### MongoDB (`mongodb` / `mongo`):
```aura
let client: MongoClient = mongodb.open("mongodb://localhost:27017/analytics_db");
let events: MongoCollection = client.db().collection("events");

async fn recordEvent(eventType: String): Task<Unit, String> {
    let res = await events.insertOne({
        type: eventType,
        timestamp: "2026-09-17T16:00:00Z"
    });
    println(`Evento registrado con ID: ${res.insertedId}`);
}
```

#### Redis (`redis`):
```aura
let cache: RedisClient = redis.open("redis://127.0.0.1:6379");

async fn checkSession(token: String): Task<Option<String>, String> {
    // Caché con lectura en O(1)
    let sessionUser = await cache.get(`session:${token}`);
    sessionUser
}
```

---

### 5.6. Arquitectura Hexagonal y Pure Dependency Injection

Para mantener los servicios escalables y fáciles de probar, desacoplamos el dominio de la infraestructura utilizando **Puertos** (interfaces) y **Adaptadores** (implementaciones concretas).

```
+-----------------------------------------------------------+
|              ADAPTADORES PRIMARIOS / INBOUND              |
|        (Controlador HTTP net/http, Comandos CLI)          |
+-----------------------------+-----------------------------+
                              |
                              v [Driving Port: OrderUseCase]
+-----------------------------------------------------------+
|                   CAPA DE APLICACIÓN                      |
|                      OrderService                         |
|     (Orquesta reglas de negocio sin saber de HTTP o SQL)   |
+-----------------------------+-----------------------------+
                              |
                              v [Driven Ports: Interfaces]
+-----------------------------------------------------------+
|              ADAPTADORES SECUNDARIOS / OUTBOUND           |
|  (PostgresOrderRepo, RedisCacheAdapter, EventPublisher)   |
+-----------------------------------------------------------+
```

```aura
// 1. Dominio Puro (Domain Layer)
export type Order = { id: String, amount: Float, status: String };

// 2. Puerto Secundario (Driven Port)
export interface OrderRepository {
    save(order: Order): Result<Order, String>;
    findById(id: String): Option<Order>;
}

// 3. Caso de Uso con Inyección por Constructor (Application Layer)
export fn newOrderService(repo: OrderRepository) => {
    {
        createOrder: (id: String, amount: Float): Result<Order, String> => {
            if amount <= 0.0 {
                return Err("El monto de la orden debe ser positivo.");
            }
            let ord: Order = { id: id, amount: amount, status: "CREATED" };
            repo.save(ord)
        }
    }
};

// 4. Composition Root (main.aura)
export fn main(): Unit => {
    // Se inyecta la implementación concreta en el servicio
    let repo = createInMemoryOrderRepository(); // o createPostgresOrderRepo()
    let service = newOrderService(repo);
    let result = service.createOrder("ORD-001", 99.5);
    println(`Orden creada con éxito.`);
}
```

> **Ver microservicio hexagonal completo:** [`examples/hexagonal_microservice/`](file:///Users/mavro/Projects/aura-lang/examples/hexagonal_microservice)

---

## 🏎️ Fase 6: Patrones y Arquitecturas de Alto Rendimiento

### 6.1. Worker Pool Concurrente con Backpressure Acotado

Un error común en aplicaciones concurrentes es generar fibras o goroutines sin límite ante una avalancha de peticiones HTTP, agotando la memoria. El patrón **Worker Pool con Bounded Channel** acota el número de trabajadores activos y bloquea a los productores cuando la cola está saturada, garantizando estabilidad:

```aura
export type Job = {
    id: Int,
    payload: String
};

export type JobResult = {
    jobId: Int,
    workerId: Int,
    output: String
};

// Worker consumer recursivo
fn worker(id: Int, jobs: RecvChannel<Job>, results: SendChannel<JobResult>, wg: WaitGroup): Task<Unit, String> => {
    for job in jobs {
        // Simular procesamiento intensivo
        let processed: JobResult = {
            jobId: job.id,
            workerId: id,
            output: `Trabajo procesado por worker ${id}`
        };
        results <- processed;
    };
    wg.done();
};

export fn runHighThroughputPool(): Task<Unit, String> => {
    let poolSize = 8; // Número de fibras concurrentes fijas
    let bufferCap = 100; // Capacidad máxima antes de aplicar backpressure

    let jobsCh = Channel.make<Job>(bufferCap);
    let resultsCh = Channel.make<JobResult>(bufferCap);
    let wg = WaitGroup.new();

    // Inicializar el pool fijo de workers
    wg.add(poolSize);
    let mut w = 1;
    while w <= poolSize {
        let workerId = w;
        spawn { worker(workerId, jobsCh, resultsCh, wg); };
        w = w + 1;
    };

    // Productor: Enviar 1,000 tareas
    spawn {
        let mut j = 1;
        while j <= 1000 {
            jobsCh <- { id: j, payload: `Payload data #${j}` };
            j = j + 1;
        };
        Channel.close(jobsCh); // Notifica a los workers que no hay más tareas
    };

    // Supervisor: Cierra el canal de resultados cuando todos los workers terminan
    spawn {
        await wg.wait();
        Channel.close(resultsCh);
    };

    // Consumidor de métricas y resultados
    let mut totalProcessed = 0;
    for res in resultsCh {
        totalProcessed = totalProcessed + 1;
    };
    println(`Procesamiento finalizado con éxito: ${totalProcessed} tareas.`);
};
```

> **Ver implementación de referencia:** [`examples/concurrency_csp_pipeline.aura`](file:///Users/mavro/Projects/aura-lang/examples/concurrency_csp_pipeline.aura)

---

### 6.2. Canales de Transmisión de Flujos sin Asignaciones (`channelTransduce`)

Para procesar flujos de datos en tiempo real (por ejemplo, streams de WebSockets o eventos Kafka) con latencia ultra baja, los transductores de Aura pueden conectarse directamente a un canal CSP mediante `channelTransduce`, transformando y filtrando los paquetes al vuelo:

```aura
let inStream = Channel.make<Int>(100);
let outStream = Channel.make<Int>(100);

let streamFilter = compose(
    filtering(fn(packetId: Int): Bool => packetId % 2 == 0),
    mapping(fn(packetId: Int): Int => packetId * 2)
);

// Conecta inStream con outStream aplicando la transformación con cero overhead
spawn {
    channelTransduce(inStream, streamFilter, outStream);
};
```

---

### 6.3. Optimización de Memoria: Pre-alocación con `make`

El colector de basura o el gestor de memoria sufre cuando un arreglo dinámico se redimensiona repetidamente. Si conoces de antemano el tamaño de un lote de datos, preasigna la capacidad con `make([]T, len, cap)`:

```aura
// ❌ INEFICIENTE: Provoca múltiples re-alocaciones y copias en memoria
let mut dynamicList: []Int = [];
let mut i = 0;
while i < 100000 {
    dynamicList = append(dynamicList, i);
    i = i + 1;
};

//  ALTO RENDIMIENTO: Una sola asignación continua en el heap
let mut optimalList = make([]Int, 0, 100000);
let mut k = 0;
while k < 100000 {
    optimalList = append(optimalList, k);
    k = k + 1;
};
```

---

### 6.4. Estrategias de Compilación a Binario Standalone

Para desplegar en contenedores de producción (Docker `scratch` o Kubernetes), Aura ofrece dos backends de compilación nativa:

#### 1. Backend Nativo Cranelift (`--target native`)
Genera código máquina nativo para la arquitectura de tu procesador (ARM64 o x86_64) enlazado con `libaura_runtime.a`:
```bash
# Compilar a binario autónomo ultrarrápido
aurac build server.aura -o server --target native

# Ejecución instantánea con cold-start < 2ms
./server
```

#### 2. Backend Golang Toolchain (`--target go`)
Transpila el código Aura a Go idiomático y genera un ejecutable mediante `go build`:
```bash
# Compilar usando la cadena de herramientas de Go
aurac build server.aura -o server --target go

# O inspeccionar el código Go puro resultante:
aurac emit-go server.aura -o server.go
```

#### Comparativa de Tamaño y Rendimiento en Producción:
- **Microservicio REST Standalone en Aura:** ~1.78 MB (un 80% más compacto que los 8.84 MB de un binario equivalente en Go estándar).
- **Cold Start:** 1.5 ms – 3.5 ms.
- **Consumo de Memoria Base (Peak RSS):** ~2.8 MB en reposo.

---

### 6.5. Depuración Interactiva y Source Maps V3

Aura cuenta con herramientas integradas de diagnóstico para inspeccionar problemas de concurrencia y memoria en desarrollo:

```bash
# Iniciar depurador interactivo paso a paso en consola:
aurac step examples/defer_demo.aura
```

Comandos en la consola interactiva `(aura-dbg)`:
- `next`: Avanza a la siguiente línea de ejecución.
- `into`: Entra dentro de la función actual.
- `vars`: Muestra todas las variables en el ámbito actual y sus valores.
- `break <línea>`: Coloca un punto de interrupción (*breakpoint*).
- `continue`: Continúa la ejecución hasta el siguiente punto de interrupción.
- `backtrace`: Muestra la pila activa de llamadas del fiber actual.

---

## 🏆 Resumen del Itinerario de Maestría

| Nivel | Hito Principal | Proyectos de Práctica Recomendados |
|---|---|---|
| **Nivel 1: Básico** | Sintaxis, Inmutabilidad, `Option`/`Result`, Slices | [`examples/slices_golang_model.aura`](file:///Users/mavro/Projects/aura-lang/examples/slices_golang_model.aura) |
| **Nivel 2: Intermedio** | Pattern Matching, Pipelines `\|>`, TCO, Átomos `:keyword` | [`examples/lisp_paradigms_demo.aura`](file:///Users/mavro/Projects/aura-lang/examples/lisp_paradigms_demo.aura) |
| **Nivel 3: Avanzado** | Concurrencia CSP (`spawn`, `Channel`, `select`), `defer`, `Context` | [`examples/concurrency_csp_pipeline.aura`](file:///Users/mavro/Projects/aura-lang/examples/concurrency_csp_pipeline.aura) |
| **Nivel 4: Experto** | Servidor HTTP `net/http`, Bases de datos, Arquitectura Hexagonal | [`examples/hexagonal_microservice/`](file:///Users/mavro/Projects/aura-lang/examples/hexagonal_microservice), [`bookstore_api/`](file:///Users/mavro/Projects/aura-lang/bookstore_api) |
| **Nivel 5: Maestro** | Worker Pools, Transductores CSP, Binarios nativos standalone | Compilación con Cranelift (`--target native`) para despliegues en producción |
