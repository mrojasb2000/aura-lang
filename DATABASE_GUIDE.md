# 🗄️ Guía de Acceso a Bases de Datos (SQL & NoSQL) en Aura Lang

Aura Lang incluye soporte nativo y tipado estático para bases de datos relacionales (**PostgreSQL** y **MySQL**) y bases de datos NoSQL (**MongoDB** y **Redis**), siguiendo la filosofía *"Batteries Included"* (similar al ecosistema estándar en Go).

---

## 🐘 1. Soporte Nativo para PostgreSQL

El objeto global `postgres` (y su alias `pg`) junto con los tipos (`PgConfig`, `PgPool`, `PgQueryResult`, `PgTransaction`) están disponibles directamente en el preludio estándar de Aura.

### Configuración y Creación de Connection Pool

```aura
let config: PgConfig = {
    host: "localhost",
    port: 5432,
    user: "postgres",
    password: "secret_password",
    database: "aura_db",
    uri: "",
    connectionLimit: 10
}

let db: PgPool = postgres.createPool(config)

// O mediante Connection URI String:
// let db = postgres.open("postgresql://postgres:secret_password@localhost:5432/aura_db")
```

### Ejecución DDL y Modificaciones con `RETURNING`

El método `db.execute(sql, params)` ejecuta sentencias parametrizadas y retorna un `Task<Result<PgQueryResult, String>, Any>`.

```aura
async fn setupSchema(): Task<(), String> {
    let ddl = "CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        name VARCHAR(100) NOT NULL,
        email VARCHAR(100) UNIQUE NOT NULL,
        balance INT DEFAULT 0
    )"

    let res = await db.execute(ddl, [])
    match res {
        Ok(info) => println(`Tabla PostgreSQL creada. Command: ${info.command}`),
        Err(err) => println(`Error DDL: ${err}`)
    }
}

// Inserción con placeholders nativos ($1, $2) y cláusula RETURNING
async fn createUser(name: String, email: String): Task<Int, String> {
    let res = await db.execute(
        "INSERT INTO users (name, email) VALUES ($1, $2) ON CONFLICT (email) DO NOTHING RETURNING id",
        [name, email]
    )

    match res {
        Ok(info) => info.insertId,
        Err(err) => {
            println(`Error al insertar: ${err}`)
            -1
        }
    }
}
```

### Consultas SELECT (`db.query` y `db.queryRow`)

```aura
async fn listUsers(): Task<(), String> {
    let result = await db.query("SELECT id, name, email, balance FROM users WHERE balance >= $1", [100])

    match result {
        Ok(rows) => {
            println(`Encontrados ${rows.length} usuarios:`)
            for u in rows {
                println(`- ${u.name} <${u.email}> ($${u.balance})`)
            }
        },
        Err(err) => println(`Error en consulta: ${err}`)
    }
}

// Consulta de fila única con Option<T>
async fn findUserByEmail(email: String): Task<(), String> {
    let res = await db.queryRow("SELECT * FROM users WHERE email = $1", [email])

    match res {
        Ok(opt) => {
            match opt {
                Some(user) => println(`Usuario: ${user.name} (Balance: $${user.balance})`),
                None => println("Usuario no encontrado")
            }
        },
        Err(err) => println(`Error: ${err}`)
    }
}
```

### Transacciones Atómicas en PostgreSQL (`db.transaction`)

```aura
async fn transferMoney(fromEmail: String, toEmail: String, amount: Int): Task<Bool, String> {
    let txResult = await db.transaction(async fn(tx: PgTransaction): Task<(), String> {
        let deduct = await tx.execute("UPDATE users SET balance = balance - $1 WHERE email = $2", [amount, fromEmail])
        let credit = await tx.execute("UPDATE users SET balance = balance + $1 WHERE email = $2", [amount, toEmail])
    })

    match txResult {
        Ok(_) => {
            println("✓ Transferencia PostgreSQL completada con éxito.")
            true
        },
        Err(err) => {
            println(`✕ Transferencia cancelada (rollback): ${err}`)
            false
        }
    }
}
```

### Sanitización y Utilidades (`pg.escapeIdentifier`, `pg.escapeLiteral`, `pg.format`)

```aura
let tableSafe = pg.escapeIdentifier("user_data")
let literalSafe = pg.escapeLiteral("Robert'); DROP TABLE Students;--")
let formatted = pg.format("SELECT * FROM users WHERE id = $1 AND role = $2", [42, "admin"])
```

### Runtime & Dependencias de PostgreSQL

Aura Lang utiliza importación dinámica (`await import('pg')`), por lo que no añade sobrecarga si el proyecto no utiliza PostgreSQL. En el entorno de ejecución Node.js:

```bash
npm install pg
```

---

## 🐬 2. Soporte Nativo para MySQL

El objeto global `mysql` y los tipos estándar (`MysqlConfig`, `MysqlPool`, `MysqlQueryResult`, `MysqlTransaction`) están disponibles automáticamente.

### Configuración y Creación de Connection Pool

```aura
let config: MysqlConfig = {
    host: "localhost",
    port: 3306,
    user: "root",
    password: "secret_password",
    database: "app_db",
    uri: "",
    connectionLimit: 10
}

let db: MysqlPool = mysql.createPool(config)

// O mediante Connection URI String:
// let db = mysql.open("mysql://root:secret_password@localhost:3306/app_db")
```

### Operaciones Básicas en MySQL

```aura
async fn createUser(name: String, email: String): Task<Int, String> {
    let res = await db.execute(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        [name, email]
    )

    match res {
        Ok(info) => info.insertId,
        Err(err) => {
            println(`Error al insertar: ${err}`)
            -1
        }
    }
}
```

### Runtime & Dependencias de MySQL

```bash
npm install mysql2
```

---

## 🍃 3. Soporte Nativo para MongoDB (NoSQL Document Store)

El objeto global `mongodb` (y su alias `mongo`) junto con los tipos (`MongoClient`, `MongoDatabase`, `MongoCollection`, `MongoInsertResult`, `MongoUpdateResult`, `MongoDeleteResult`) están disponibles directamente en el preludio estándar de Aura.

### Conexión y Acceso a Colecciones

```aura
let client: MongoClient = mongodb.open("mongodb://localhost:27017/my_app_db")
let db: MongoDatabase = client.db()
let users: MongoCollection = db.collection("users")
```

### Inserción y Búsqueda de Documentos

```aura
async fn handleUsers(): Task<(), String> {
    // Inserción de un documento
    let insertRes = await users.insertOne({
        name: "Carlos",
        email: "carlos@example.com",
        role: "admin",
        score: 95
    })

    match insertRes {
        Ok(res) => println(`Documento insertado con ID: ${res.insertedId}`),
        Err(err) => println(`Error de inserción: ${err}`)
    }

    // Búsqueda de documento único con Option<T>
    let found = await users.findOne({ email: "carlos@example.com" })
    match found {
        Ok(opt) => {
            match opt {
                Some(doc) => println(`Usuario encontrado: ${doc.name} (Puntaje: ${doc.score})`),
                None => println("Usuario no encontrado")
            }
        },
        Err(err) => println(`Error de consulta: ${err}`)
    }
}
```

### Actualizaciones, Agregaciones y Conteo

```aura
async fn updateAndAggregate(): Task<(), String> {
    // Actualización de documentos
    let updateRes = await users.updateOne({ email: "carlos@example.com" }, { "$set": { score: 100 } })
    
    // Conteo de documentos coincidentes
    let totalAdmins = await users.countDocuments({ role: "admin" })

    // Pipelines de agregación
    let aggRes = await users.aggregate([
        { "$match": { role: "admin" } },
        { "$group": { _id: "$role", avgScore: { "$avg": "$score" } } }
    ])
}
```

### Runtime & Dependencias de MongoDB

```bash
npm install mongodb
```

---

## ⚡ 4. Soporte Nativo para Redis (NoSQL Key-Value & Cache)

El objeto global `redis` y los tipos estándar (`RedisConfig`, `RedisClient`) permiten almacenamiento en memoria de alta velocidad, manejo de sesiones y operaciones con expiración TTL.

### Conexión y Operaciones Clave-Valor con TTL

```aura
let cache: RedisClient = redis.open("redis://127.0.0.1:6379")

async fn handleSession(userId: String, token: String): Task<Bool, String> {
    // Guardar clave con expiración (TTL en segundos)
    let setRes = await cache.set(`session:${userId}`, token, 3600)

    // Obtener valor con Option<String>
    let sessionToken = await cache.get(`session:${userId}`)
    match sessionToken {
        Ok(opt) => {
            match opt {
                Some(tok) => println(`Token de sesión activo: ${tok}`),
                None => println("Sesión expirada o no encontrada")
            }
        },
        Err(err) => println(`Error de lectura de caché: ${err}`)
    }

    // Contadores atómicos
    let views = await cache.incr("page:views:total")

    // Operaciones con Hashes (hset / hget / hgetall)
    let hsetRes = await cache.hset(`user:profile:${userId}`, "theme", "dark")
    let themeRes = await cache.hget(`user:profile:${userId}`, "theme")

    true
}
```

### Runtime & Dependencias de Redis

```bash
npm install ioredis
# o alternativamente: npm install redis
```

---

## 📊 Comparativa de Características de Bases de Datos en Aura Lang

| Característica | PostgreSQL (`postgres` / `pg`) | MySQL (`mysql`) | MongoDB (`mongodb` / `mongo`) | Redis (`redis`) |
| :--- | :--- | :--- | :--- | :--- |
| **Paradigma** | Relacional SQL (ACID) | Relacional SQL (ACID) | NoSQL Documentos JSON/BSON | NoSQL Clave-Valor / In-Memory / Caché |
| **Driver Dinámico en Runtime** | `pg` (`node-postgres`) | `mysql2/promise` | `mongodb` | `ioredis` / `redis` |
| **Estructuras de Retorno** | `PgQueryResult` | `MysqlQueryResult` | `MongoInsertResult`, `MongoUpdateResult`, etc. | `Option<String>`, `Int`, `Bool`, etc. |
| **Consultas Individuales** | `db.queryRow(...) -> Option<T>` | `db.queryRow(...) -> Option<T>` | `col.findOne(...) -> Option<T>` | `cache.get(...) -> Option<String>` |
| **Manejo de Errores** | `Task<Result<T, String>, Any>` | `Task<Result<T, String>, Any>` | `Task<Result<T, String>, Any>` | `Task<Result<T, String>, Any>` |
| **Definiciones TypeScript (`.d.ts`)** | Automáticas y tipadas | Automáticas y tipadas | Automáticas y tipadas | Automáticas y tipadas |

