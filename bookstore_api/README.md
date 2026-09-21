# 📚 Servicio API REST en Aura Lang (Librería & Editorial)

Servicio API REST de alto rendimiento desarrollado en **Aura Lang**, ejecutado completamente en el lado del servidor con el motor HTTP nativo basado en estilo Golang (`http.newServeMux()`).

El servicio implementa la gestión completa (CRUD) para **Libros**, **Artículos**, **Autores**, **Editoriales** y un subsistema de **Venta de Libros** con control transaccional de stock, cálculo de impuestos, descuentos y emisión de recibos.

---

## 🌟 Características Principales

- **Ejecución Lado del Servidor (Server-Side)**: Desarrollado 100% en Aura Lang, compilado directamente a binario nativo autónomo de alto rendimiento con cero dependencias externas de runtime.
- **Arquitectura Net/HTTP (Estilo Go)**: Utiliza `ServeMux` con enrutamiento dinámico (`:id`), extracción de query params, middleware de registro (logger) y soporte para CORS pre-flight.
- **Tipado Fuerte e Inmutabilidad**: Modelos formales para `Author`, `Editorial`, `Book`, `Article` y `SaleRecord`.
- **Integridad Referencial**: Impide la eliminación de autores o editoriales que mantengan libros asociados en el catálogo.
- **Venta de Libros Transaccional**:
  - Descuento automático e inventario actualizado atómicamente.
  - Validación de stock disponible con errores descriptivos (`400 Bad Request`).
  - Cálculo de subtotal, descuentos porcentuales, IVA (19%) y total.
  - Generación de comprobante de venta (`receipt`).
  - Dashboard de métricas financieras y resumen de ventas.
- **Búsqueda y Filtros Avanzados**: Filtrado por género, autor, editorial, disponibilidad en stock y búsqueda por texto en títulos o ISBN.

---

## 📁 Estructura del Proyecto

```
bookstore_api/
├── server.aura       # Servidor HTTP y lógica de negocio completa en Aura Lang
├── test_api.sh       # Suite de pruebas automatizadas con curl (E2E)
└── README.md         # Documentación de la API, modelos y endpoints
```

También disponible en `examples/bookstore_service.aura` para ejecución directa en el directorio de ejemplos.

---

## 🚀 Inicio Rápido

### Requisitos
- Compilador de Aura Lang (`aurac`).
- Node.js (v18+).
- `curl` (para ejecutar el suite de pruebas o interactuar con la API).

### 1. Ejecutar el Servidor en Desarrollo
```bash
aurac run bookstore_api/server.aura
```
El servidor quedará a la escucha en: `http://localhost:8080`

### 2. Compilar a Binario Autónomo (Standalone Executable)
```bash
aurac build bookstore_api/server.aura -o bookstore-api --standalone
./bookstore-api
```

### 3. Ejecutar la Suite de Pruebas Automatizadas
Ejecuta todas las pruebas CRUD, ventas y validaciones:
```bash
chmod +x bookstore_api/test_api.sh
./bookstore_api/test_api.sh
```

---

## 📖 Catálogo Completo de Endpoints

### 1. General & Monitoreo
| Método | Endpoint | Descripción |
|---|---|---|
| `GET` | `/` | Documentación e índice de la API |
| `GET` | `/api/health` | Estado del servidor, tiempo activo y conteo de entidades |

### 2. Autores (`/api/authors`)
| Método | Endpoint | Descripción | Parámetros / Payload |
|---|---|---|---|
| `GET` | `/api/authors` | Lista todos los autores | Query: `?nationality=...`, `?search=...` |
| `GET` | `/api/authors/:id` | Detalle del autor + bibliografía de libros y artículos | Parámetro URL: `id` |
| `POST` | `/api/authors` | Crea un nuevo autor | JSON: `{ name, email, nationality, biography, birthYear }` |
| `PUT` | `/api/authors/:id` | Actualiza información del autor | JSON con campos a modificar |
| `DELETE` | `/api/authors/:id` | Elimina autor (si no posee libros asociados) | Parámetro URL: `id` |

### 3. Editoriales (`/api/editorials`)
| Método | Endpoint | Descripción | Parámetros / Payload |
|---|---|---|---|
| `GET` | `/api/editorials` | Lista todas las editoriales | Query: `?country=...`, `?search=...` |
| `GET` | `/api/editorials/:id` | Detalle de la editorial + catálogo de libros | Parámetro URL: `id` |
| `POST` | `/api/editorials` | Registra una nueva editorial | JSON: `{ name, country, foundedYear, website }` |
| `PUT` | `/api/editorials/:id` | Actualiza información de la editorial | JSON con campos a modificar |
| `DELETE` | `/api/editorials/:id` | Elimina editorial (si no posee libros asociados) | Parámetro URL: `id` |

### 4. Libros (`/api/books`)
| Método | Endpoint | Descripción | Parámetros / Payload |
|---|---|---|---|
| `GET` | `/api/books` | Lista libros (con autor y editorial resueltos) | Query: `?genre=...`, `?authorId=...`, `?editorialId=...`, `?inStock=true`, `?search=...` |
| `GET` | `/api/books/:id` | Detalle del libro con autor y editorial completos | Parámetro URL: `id` |
| `POST` | `/api/books` | Registra un nuevo libro | JSON: `{ title, isbn, authorId, editorialId, price, stock, genre, publishedYear }` |
| `PUT` | `/api/books/:id` | Actualiza un libro | JSON con campos a modificar |
| `DELETE` | `/api/books/:id` | Elimina un libro del catálogo | Parámetro URL: `id` |

### 5. Artículos (`/api/articles`)
| Método | Endpoint | Descripción | Parámetros / Payload |
|---|---|---|---|
| `GET` | `/api/articles` | Lista artículos | Query: `?topic=...`, `?authorId=...`, `?search=...` |
| `GET` | `/api/articles/:id` | Detalle del artículo con datos del autor | Parámetro URL: `id` |
| `POST` | `/api/articles` | Publica un nuevo artículo | JSON: `{ title, summary, content, authorId, topic, publishedAt }` |
| `PUT` | `/api/articles/:id` | Actualiza un artículo | JSON con campos a modificar |
| `DELETE` | `/api/articles/:id` | Elimina un artículo | Parámetro URL: `id` |

### 6. Venta de Libros (`/api/books/:id/sell` & `/api/sales`)
| Método | Endpoint | Descripción | Parámetros / Payload |
|---|---|---|---|
| `POST` | `/api/books/:id/sell` | Venta directa de un libro por su ID en la ruta | JSON: `{ quantity, customerName, customerEmail, paymentMethod, discountPercent }` |
| `POST` | `/api/sales` | Venta general especificando `bookId` en el body | JSON: `{ bookId, quantity, customerName, customerEmail, paymentMethod, discountPercent }` |
| `GET` | `/api/sales` | Historial de transacciones de ventas | Query: `?bookId=...`, `?customerEmail=...` |
| `GET` | `/api/sales/:id` | Consulta un comprobante de venta por ID | Parámetro URL: `id` |
| `GET` | `/api/sales/summary` | Dashboard de ingresos brutos, unidades vendidas, impuestos y ticket promedio | N/A |

---

## 💻 Ejemplos Prácticos con `curl`

### 1. Venta Directa de Libros
```bash
curl -X POST http://localhost:8080/api/books/bk-1/sell \
  -H "Content-Type: application/json" \
  -d '{
    "quantity": 2,
    "customerName": "Laura Restrepo",
    "customerEmail": "laura@correo.lat",
    "paymentMethod": "CREDIT_CARD",
    "discountPercent": 10.0
  }'
```

**Respuesta Exitosa (201 Created):**
```json
{
  "success": true,
  "message": "Venta #sale-1002 procesada exitosamente",
  "receipt": {
    "id": "sale-1002",
    "bookId": "bk-1",
    "bookTitle": "Cien años de soledad",
    "authorName": "Gabriel García Márquez",
    "quantity": 2,
    "unitPrice": 24.99,
    "subtotal": 49.98,
    "discountPercent": 10,
    "discountAmount": 4.998,
    "taxPercent": 19,
    "taxAmount": 8.54658,
    "total": 53.52858,
    "customerName": "Laura Restrepo",
    "customerEmail": "laura@correo.lat",
    "paymentMethod": "CREDIT_CARD",
    "timestamp": "2026-09-04T23:20:00Z",
    "status": "COMPLETED"
  },
  "inventory": {
    "bookId": "bk-1",
    "bookTitle": "Cien años de soledad",
    "remainingStock": 28
  }
}
```

### 2. Validación de Stock Insuficiente
```bash
curl -X POST http://localhost:8080/api/books/bk-1/sell \
  -H "Content-Type: application/json" \
  -d '{"quantity": 9999}'
```

**Respuesta (400 Bad Request):**
```json
{
  "success": false,
  "error": "Stock insuficiente para 'Cien años de soledad'. Stock disponible: 28, cantidad solicitada: 9999"
}
```

### 3. Crear un Nuevo Libro
```bash
curl -X POST http://localhost:8080/api/books \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Rayuela",
    "isbn": "978-8466331906",
    "authorId": "aut-1",
    "editorialId": "edt-1",
    "price": 22.00,
    "stock": 35,
    "genre": "Contranovela",
    "publishedYear": 1963
  }'
```

### 4. Consultar Resumen Financiero de Ventas
```bash
curl http://localhost:8080/api/sales/summary
```

**Respuesta:**
```json
{
  "success": true,
  "summary": {
    "totalTransactions": 3,
    "totalBooksSoldUnits": 4,
    "totalGrossRevenue": 104.68,
    "totalTaxCollected": 16.71,
    "totalDiscountsGiven": 4.99,
    "averageTicketValue": 34.89,
    "currency": "USD"
  }
}
```

### 5. Integridad Referencial al Eliminar
Si intentas eliminar un autor que tiene libros publicados en el catálogo:
```bash
curl -X DELETE http://localhost:8080/api/authors/aut-1
```
**Respuesta:**
```json
{
  "success": false,
  "error": "No se puede eliminar el autor: tiene 2 libro(s) asociado(s) en el catálogo."
}
```
