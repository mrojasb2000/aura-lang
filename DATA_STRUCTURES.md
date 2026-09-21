# Estructuras de Datos en Aura Lang

Aura Lang proporciona un conjunto de estructuras de datos fundamentales para el desarrollo de aplicaciones. A continuación, se detallan las estructuras de datos disponibles, sus características y ejemplos de uso.

## 1. Listas y Slices (Lists & Slices - Modelo Golang)

Aura Lang adopta el modelo de **Slices de Golang** integrado nativamente con las listas dinámicas. Los tipos `[]T` y `List<T>` son equivalentes en el sistema de tipos estático.

### Características:
- **Sintaxis de Tipos Golang:** Declara slices usando `[]T` (por ejemplo, `[]Int`, `[]String`) o `List<T>`.
- **Expresiones de Slicing Nativas:** Segmenta colecciones con `s[low:high]`, `s[:high]`, `s[low:]`, `s[:]` y con límite de capacidad `s[low:high:max]` (o `s[low..high]`).
- **Funciones Built-in Estilo Go:** Disponibilidad directa de `len(s)`, `cap(s)`, `append(s, elemento)` y `make([]T, len, cap)`.
- **Propiedades y Métodos Fluídos:** Compatible tanto con acceso por propiedades (`s.len`, `s.cap`, `s.length`, `s.capacity`) como con métodos (`s.slice(start, end)`, `s.push(item)`).
- **Segmentación de Cadenas (String Slicing):** Las mismas expresiones de slicing aplican sobre strings (`texto[0:4]`).
- **Indexación Basada en Cero:** Acceso directo por índice `s[i]`.

### Ejemplos de uso:
```aura
// Declaración con sintaxis de slice Go
let mut numeros: []Int = [10, 20, 30, 40, 50];

// Funciones built-in de Go: len y cap
println(`Longitud: ${len(numeros)}`);    // 5
println(`Capacidad: ${cap(numeros)}`);   // 5

// Expresiones de Slicing (sub-slices)
let medio = numeros[1:4];     // [20, 30, 40]
let inicio = numeros[:3];     // [10, 20, 30]
let cola = numeros[3:];       // [40, 50]
let copia = numeros[:];       // [10, 20, 30, 40, 50]
let conMax = numeros[1:3:5];  // [20, 30] con límite de capacidad

// Operación append (Go style) y push (method style)
numeros = append(numeros, 60);
numeros.push(70);

// Slicing de Strings
let saludo = "Hola, Mundo!";
let sub = saludo[0:4];        // "Hola"
```

## 2. Diccionarios (Dictionaries / Maps)

Los diccionarios son colecciones de pares clave-valor. Son útiles cuando necesitas asociar valores con claves únicas para búsquedas rápidas.

### Características:
- **No ordenados:** No garantizan un orden específico de los elementos.
- **Claves únicas:** Cada clave en el diccionario debe ser única.
- **Mutables:** Puedes agregar, eliminar o actualizar los pares clave-valor.

### Ejemplos de uso:
```aura
// Declaración de un diccionario
let usuario = {
    "nombre": "Ana",
    "edad": 28,
    "activo": true
}

// Acceder a un valor
let nombreUsuario = usuario["nombre"] // "Ana"

// Modificar un valor
usuario["edad"] = 29

// Agregar un nuevo par clave-valor
usuario["email"] = "ana@example.com"
```

## 3. Tuplas (Tuples)

Las tuplas son colecciones ordenadas pero **inmutables**. Una vez creadas, no se pueden modificar (no se pueden agregar, eliminar ni cambiar los elementos).

### Características:
- **Indexadas:** Al igual que las listas, se accede a los elementos mediante un índice.
- **Inmutables:** Su contenido no puede cambiar después de la inicialización.
- **Tipos fijos:** A menudo se utilizan para agrupar diferentes tipos de datos en una sola estructura.

### Ejemplos de uso:
```aura
// Declaración de una tupla
let coordenadas = (10, 20)
let persona = ("Carlos", 35, true)

// Acceder a un elemento
let x = coordenadas[0] // 10

// Intento de modificación (Error en tiempo de compilación o ejecución)
// coordenadas[0] = 15 // ¡Esto generará un error!
```

## 4. Conjuntos (Sets)

Los conjuntos son colecciones desordenadas de elementos únicos. Son muy útiles para operaciones matemáticas de conjuntos y para eliminar duplicados.

### Características:
- **Elementos únicos:** No se permiten valores duplicados.
- **No ordenados:** Los elementos no tienen un orden o índice específico.
- **Eficiencia:** Ideales para verificar rápidamente si un elemento existe en la colección.

### Ejemplos de uso:
```aura
// Declaración de un conjunto (la sintaxis exacta dependerá de Aura)
let colores = Set(["rojo", "verde", "azul", "rojo"])

// El conjunto resultante será solo: ["rojo", "verde", "azul"]

// Verificar si un elemento existe
let tieneRojo = colores.contains("rojo") // true
```

---

## Consideraciones Adicionales

- **Tipado Fuerte vs Dinámico:** Asegúrate de verificar si Aura Lang requiere declarar los tipos de los elementos de las estructuras de datos (si es fuertemente tipado) o si los infiere automáticamente.
- **Iteración:** Todas estas estructuras de datos pueden ser iteradas utilizando bucles como `for` o `foreach`, dependiendo de las construcciones de control de flujo de Aura Lang.
```aura
for num in numeros {
    // operaciones con num
}
```

> **Nota:** La sintaxis específica mostrada arriba es ilustrativa y basada en convenciones comunes de lenguajes modernos. Consulta la guía de referencia del lenguaje Aura para obtener detalles precisos sobre los métodos disponibles para cada estructura de datos.
