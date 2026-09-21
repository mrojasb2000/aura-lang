# 🚀 Aura Backend Lang & Standalone Native Executable Guide (Golang Model)

Aura Lang en esta rama (`feat/aura-backend-golang-executable`) se especializa **exclusivamente como un lenguaje para desarrollo de Backend y Sistemas**, adoptando íntegramente la filosofía, modelo de concurrencia y arquitectura de distribución de **Golang**.

---

## 🎯 Pilares del Modelo Golang en Aura

| Concepto | Golang | Aura Backend |
|---|---|---|
| **Punto de Entrada** | `package main` con `func main()` | `export fn main(): Unit => { ... }` |
| **Generación de Binario** | `go build -o app` | `aurac build app.aura -o app` |
| **Ejecución Directa** | `go run app.go` | `aurac run app.aura` |
| **Chequeo de Tipos / Linter** | `go vet` | `aurac check app.aura` |
| **Formateo Automático** | `gofmt -w .` | `aurafmt -w .` |
| **Pruebas Automatizadas** | `go test ./...` | `auratest` |
| **Concurrencia Liviana** | Goroutines (`go func()`) | Fibers (`spawn { ... }`) |
| **Paso de Mensajes** | Canales tipados (`chan T`) | `Channel<T>` con `ch <- v` y `<-ch` |
| **Multiplexación** | `select { case ... default: }` | `select { case ... default => }` |
| **Limpieza Determinista LIFO** | `defer res.Close()` | `defer res.close()` |
| **Cancelación y Tiempos Límite**| `context.Context` | `Context::background()`, `withTimeout`, `withCancel` |
| **Servidor HTTP Nativo** | `net/http` (`http.ServeMux`) | `http.newServeMux()`, verbos HTTP y middleware |
| **Sincronización** | `sync.Mutex`, `sync.WaitGroup` | `Mutex.new()`, `WaitGroup.new()` |
| **Acceso al Sistema Operativo** | Paquete `os` (`os.Args`, `os.Getenv`) | Espacio `os` (`os.args`, `os.env`, `os.writeFile`) |
| **Interfaces y Receptores** | Métodos con receiver (`func (s S) M()`) | Métodos con receiver (`fn (s: S) m()`) |
| **Transpilación Directa a Go** | N/A | `aurac emit-go app.aura` / `aurac build --target go` |

---

## 📦 Generación de Ejecutables Autónomos

En Golang, la gran ventaja competitiva para DevOps y despliegues en producción es la creación de un **único archivo binario ejecutable independiente**, sin requerir que la máquina destino tenga instalado Python, Node.js ni ningún intérprete externo.

Aura replica este comportamiento mediante:

```bash
aurac build app.aura -o app
```

### ¿Cómo funciona el motor de compilación binaria?

1. **Validación Estricta de Backend**: El compilador verifica que el código sea 100% backend (las etiquetas JSX o componentes de interfaz de usuario de React son rechazadas con diagnóstico explicativo).
2. **Generación de Bundle Autónomo**: Empaqueta el AST tipado con el runtime de Aura (scheduler de fibers CSP, HTTP Server net/http, sync primitives y drivers de base de datos).
3. **Compilación a Código Máquina Nativo**:
   - Utiliza el compilador de binarios nativos para producir un ejecutable **Mach-O (macOS)** o **ELF (Linux)** de 64 bits.
   - Aplica permisos de ejecución (`chmod +x` / `0o755`).
4. **Despliegue Cero Dependencias**: El binario resultante se puede copiar directamente a servidores o imágenes mínimas de Docker (`scratch` / `alpine`) y ejecutar con `./app`.

### Compilación usando la Cadena de Herramientas de Go (`--target go`)

Si tienes instalado el compilador de Go (`go`), Aura también puede transpilar tu código directamente a Go y compilarlo con `go build`:

```bash
# Compilar a binario nativo vía Go
aurac build app.aura -o app --target go

# O inspeccionar el código fuente Go generado
aurac emit-go app.aura -o app.go
```

---

## 💻 Ejemplo: Microservicio Backend Completo

Ver implementación completa en [`examples/backend_service.aura`](file:///Users/mavro/Projects/aura-lang/examples/backend_service.aura).

```aura
// 1. Instanciar ServeMux estilo net/http de Go
let mux = http.newServeMux();

// 2. Middleware de Registro (Logging)
mux.use(fn(req: Any, res: Any, next: Any) => {
    println(`[HTTP] ${req.method} ${req.url}`);
    res.setHeader("X-Server", "Aura-Backend-Engine");
    next();
});

// 3. Mutex para almacenamiento en memoria thread-safe
let storageMutex = Mutex.new();
let mut data = [
    { id: "1", name: "Servidor Aura", active: true }
];

// 4. Ruta GET con Parámetro Dinámico (:id)
mux.get("/items/:id", fn(req: Any, res: Any) => {
    let itemId = http.pathValue(req, "id");
    
    storageMutex.lock();
    defer storageMutex.unlock();

    let matched = data.filter(fn(item: Any): Bool => item.id == itemId);
    if (matched.length == 0) {
        http.error(res, "Item no encontrado", http.StatusNotFound);
        return ();
    }

    http.json(res, http.StatusOK, matched[0]);
});

// 5. Ruta POST con Parsing Asíncrono de JSON
mux.post("/items", fn(req: Any, res: Any) => async {
    let bodyResult = await http.parseJson(req);
    match bodyResult {
        Ok(payload) => {
            storageMutex.lock();
            defer storageMutex.unlock();

            data.push(payload);
            http.json(res, http.StatusCreated, { ok: true, item: payload });
        },
        Err(err) => {
            http.error(res, `Payload inválido: ${err}`, http.StatusBadRequest);
        }
    }
});

// 6. Concurrencia CSP: Fibers (Goroutines) y Canales
let queue = Channel.new(50);
spawn(async {
    println("Worker fiber de background iniciado.");
});

// 7. Arrancar servidor en puerto configurable
let port = os.env("PORT") != "" ? os.env("PORT") : "8080";
println(`🚀 Servidor Aura Backend escuchando en :${port}...`);
mux.listenAndServe(`:${port}`);
```

---

## 🛠️ Comandos CLI de Aura Backend

```bash
# 1. Compilar a binario ejecutable autónomo (Modelo Golang)
aurac build main.aura -o binario_app

# 2. Compilar a binario usando el compilador de Go
aurac build main.aura -o binario_app --target go

# 3. Transpilar a código Go legible (.go)
aurac emit-go main.aura -o main.go

# 4. Ejecutar directamente en desarrollo (estilo 'go run')
aurac run main.aura

# 5. Verificación estática y chequeo de tipos (estilo 'go vet')
aurac check main.aura

# 6. Ejecución de pruebas unitarias y benchmarks (estilo 'go test')
auratest

# 7. Formateo de código fuente (estilo 'gofmt')
aurafmt -w main.aura
```
