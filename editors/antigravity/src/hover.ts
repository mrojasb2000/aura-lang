import * as vscode from 'vscode';

export interface HoverDoc {
    title: string;
    category: 'Keyword' | 'Type' | 'Built-in Function' | 'Operator' | 'Module' | 'Decorator';
    signature: string;
    description: string;
    params?: { name: string; type: string; desc: string }[];
    returns?: { type: string; desc: string };
    example?: string;
    tips?: string;
}

export const HOVER_DATABASE: Record<string, HoverDoc> = {
    // ------------------------------------------------------------------------
    // Keywords
    // ------------------------------------------------------------------------
    'fn': {
        title: 'fn',
        category: 'Keyword',
        signature: 'fn <name>(<param1>: <Type1>, ...): <ReturnType> => <Body>',
        description: 'Declares a function in Aura. Functions are first-class citizens, support closures, and tail-recursive calls are automatically compiled into zero-overhead iterative loops via **Tail-Call Optimization (TCO)**.',
        params: [
            { name: 'name', type: 'Identifier', desc: 'The name of the function (use camelCase convention).' },
            { name: 'params', type: 'param: Type, ...', desc: 'Comma-separated typed parameter list.' },
            { name: 'ReturnType', type: 'Type', desc: 'Expected return type annotation (e.g., Int, String, Unit).' }
        ],
        returns: { type: 'ReturnType', desc: 'The result of evaluating the function body expression.' },
        example: 'fn add(a: Int, b: Int): Int => {\n    a + b\n}\n\n// Tail-recursive with automatic TCO\nfn factorial(n: Int, acc: Int = 1): Int => {\n    if n <= 1 => acc\n    else => factorial(n - 1, n * acc)\n}'
    },
    'let': {
        title: 'let',
        category: 'Keyword',
        signature: 'let <name>: <Type> = <expression>;',
        description: 'Binds an **immutable** variable. In Aura, immutability is the default. Once assigned, immutable bindings cannot be modified or rebound.',
        params: [
            { name: 'name', type: 'Identifier', desc: 'Name of the variable.' },
            { name: 'Type', type: 'Type (optional)', desc: 'Explicit type annotation. If omitted, inferred by Hindley-Milner engine.' },
            { name: 'expression', type: 'Expression', desc: 'Initial value to assign.' }
        ],
        returns: { type: 'Unit', desc: 'Statement; does not produce a value.' },
        example: 'let port: Int = 8080;\nlet greeting = "Hello, Aura!"; // Inferred as String'
    },
    'mut': {
        title: 'mut',
        category: 'Keyword',
        signature: 'let mut <name>: <Type> = <initial_value>;',
        description: 'Declares a **mutable** variable binding. Mutation must be explicitly declared with `let mut`, and reassignment is performed with `=`.',
        params: [
            { name: 'name', type: 'Identifier', desc: 'Name of the mutable variable.' },
            { name: 'Type', type: 'Type', desc: 'Type of the variable.' },
            { name: 'initial_value', type: 'Expression', desc: 'Initial value before mutations.' }
        ],
        example: 'let mut count = 0;\ncount = count + 1;\nprintln(`Count: ${count}`);'
    },
    'type': {
        title: 'type',
        category: 'Keyword',
        signature: 'type <Name> = | <Variant1>(<Type>) | <Variant2>;',
        description: 'Declares an **Algebraic Data Type (ADT / Sum Type)** or record/struct type. Variants can carry typed payloads and are checked exhaustively with `match`.',
        params: [
            { name: 'Name', type: 'Identifier (PascalCase)', desc: 'Name of the user-defined type.' },
            { name: 'Variants', type: 'ADT Arms', desc: 'List of union variants separated by `|`.' }
        ],
        example: '// Algebraic Sum Type (ADT)\ntype Shape =\n    | Circle(Float)\n    | Rectangle(Float, Float)\n    | Point;\n\n// Record Type\ntype User = {\n    id: Int,\n    username: String,\n    isActive: Bool,\n};'
    },
    'interface': {
        title: 'interface',
        category: 'Keyword',
        signature: 'interface <Name> {\n    fn <method>(<params>): <ReturnType>;\n}',
        description: 'Declares a **structural interface** (Go-style duck typing). Any type that implements all methods of the interface implicitly satisfies it without needing an explicit `implements` declaration.',
        params: [
            { name: 'Name', type: 'Identifier (PascalCase)', desc: 'Interface name.' },
            { name: 'methods', type: 'Function Signatures', desc: 'Required method signatures.' }
        ],
        example: 'interface Reader {\n    fn read(size: Int): String;\n}\n\nfn dump(r: Reader): Unit => {\n    println(r.read(1024));\n}'
    },
    'match': {
        title: 'match',
        category: 'Keyword',
        signature: 'match <expression> {\n    <Pattern1> => <Branch1>,\n    <Pattern2> when <condition> => <Branch2>,\n    _ => <DefaultBranch>\n}',
        description: 'Evaluates pattern matching over values, sum types, and records. Requires **exhaustive** coverage of all variants at compile time. Supports variable binding and `when` guards.',
        params: [
            { name: 'expression', type: 'Expression', desc: 'Target value being matched.' },
            { name: 'branches', type: 'Pattern => Expression', desc: 'Arms handling each pattern variant.' }
        ],
        returns: { type: 'T', desc: 'Evaluates to the result of the executed matching branch.' },
        example: 'match shape {\n    Circle(radius) when radius > 0.0 => 3.14159 * radius * radius,\n    Rectangle(w, h) => w * h,\n    Point => 0.0,\n    _ => 0.0,\n}'
    },
    'when': {
        title: 'when',
        category: 'Keyword',
        signature: '<Pattern> when <boolean_expression> => <Branch>',
        description: 'Adds a conditional guard expression to a `match` pattern arm. The arm is only entered if the pattern matches AND the `when` condition evaluates to `true`.',
        params: [
            { name: 'boolean_expression', type: 'Bool', desc: 'Condition evaluated before arm execution.' }
        ],
        example: 'match status {\n    Code(c) when c >= 200 && c < 300 => println("Success"),\n    Code(c) when c >= 400 => println("Client error"),\n    _ => println("Other"),\n}'
    },
    'defer': {
        title: 'defer',
        category: 'Keyword',
        signature: 'defer <statement_or_block>;',
        description: 'Defers execution of a cleanup statement until the enclosing function returns. Multiple `defer` statements are executed in deterministic **LIFO (Last-In, First-Out)** order. Ideal for closing file handles, database connections, and unlocking mutexes.',
        params: [
            { name: 'statement', type: 'Expression / Block', desc: 'Code to execute upon function exit.' }
        ],
        example: 'let db = pg::connect("postgres://localhost:5432/db")?;\ndefer db.close();\n\nlet tx = db.beginTransaction()?;\ndefer tx.rollback();\n// do work...\ntx.commit();'
    },
    'spawn': {
        title: 'spawn',
        category: 'Keyword',
        signature: 'spawn {\n    <work>\n};',
        description: 'Spawns a lightweight concurrent **fiber** (Go-style goroutine) executed concurrently in the event loop. Fibers communicate safely using typed channels.',
        params: [
            { name: 'block', type: 'Block', desc: 'Code to execute concurrently in the spawned fiber.' }
        ],
        returns: { type: 'Unit', desc: 'Asynchronous task launch.' },
        example: 'let ch = Channel<Int>::new(5);\n\nspawn {\n    for i in 1..10 {\n        ch <- i;\n    }\n};'
    },
    'select': {
        title: 'select',
        category: 'Keyword',
        signature: 'select {\n    case <msg> <- <ch> => { ... },\n    case <ch> <- <val> => { ... },\n    default => { ... }\n}',
        description: 'Multiplexes across multiple channel communications (sends and receives). Blocks until one channel is ready. If a `default` case is present, executes non-blocking.',
        example: 'select {\n    case msg <- ch1 => {\n        println(`Received: ${msg}`);\n    },\n    case ch2 <- "ping" => {\n        println("Sent ping");\n    },\n    default => {\n        println("No channel ready");\n    }\n}'
    'embed': {
        title: 'embed',
        category: 'Keyword',
        signature: 'embed("<relative_path>")',
        description: 'Embeds a static text asset directly into the compiled standalone binary at compile time as a UTF-8 `String`.',
        params: [
            { name: 'path', type: 'String Literal', desc: 'Relative path to text file on disk.' }
        ],
        returns: { type: 'String', desc: 'Inlined text content.' },
        example: 'let template = embed("./template.html");\nprintln(template);'
    },
    'embedBytes': {
        title: 'embedBytes',
        category: 'Keyword',
        signature: 'embedBytes("<relative_path>")',
        description: 'Embeds a static binary file directly into the compiled binary at compile time as a byte array (`Uint8Array` / `List<Byte>`).',
        params: [
            { name: 'path', type: 'String Literal', desc: 'Relative path to binary file.' }
        ],
        returns: { type: 'List<Byte>', desc: 'Inlined binary bytes.' },
        example: 'let iconData = embedBytes("./assets/logo.png");\nprintln(`Bytes loaded: ${len(iconData)}`);'
    },
    'async': {
        title: 'async',
        category: 'Keyword',
        signature: 'async fn <name>(<params>): Task<<ReturnType>, <ErrorType>> => ...',
        description: 'Marks a function or closure as asynchronous. In Aura, async functions return a `Task<T, E>`.',
        example: 'async fn fetchData(url: String): Task<String, Error> => {\n    let resp = await http::get(url)?;\n    resp.body()\n}'
    },
    'await': {
        title: 'await',
        category: 'Keyword',
        signature: 'await <task_expression>',
        description: 'Suspends execution until the awaited `Task<T, E>` or Promise completes, unwrapping its success value.',
        example: 'let content = await readFile("data.json")?;'
    },
    'panic': {
        title: 'panic',
        category: 'Built-in Function',
        signature: 'panic(message: String): Never',
        description: 'Stops normal execution and initiates stack unwinding with a fatal panic error message. Can be intercepted and handled by `recover()` within a `defer` block.',
        params: [
            { name: 'message', type: 'String', desc: 'Error explanation.' }
        ],
        returns: { type: 'Never', desc: 'Unwinds stack; does not return normally.' },
        example: 'if index >= len(items) {\n    panic("Index out of bounds!");\n}'
    },
    'recover': {
        title: 'recover',
        category: 'Built-in Function',
        signature: 'recover(): Option<Any>',
        description: 'Catches an active panic during stack unwinding. Returns `Some(error)` if the enclosing fiber is panicking, or `None` if execution is normal. Must be called inside a `defer` block.',
        returns: { type: 'Option<Any>', desc: 'Captured panic payload or None.' },
        example: 'defer fn() => {\n    match recover() {\n        Some(err) => println(`Recovered from panic: ${err}`),\n        None => (),\n    }\n}();'
    },
    'if': {
        title: 'if',
        category: 'Keyword',
        signature: 'if <condition> { <ThenBranch> } else { <ElseBranch> }',
        description: 'Conditional control flow expression. In Aura, `if/else` is an expression that produces a value; both branches must unify to the same type.',
        params: [
            { name: 'condition', type: 'Bool', desc: 'Boolean test expression.' }
        ],
        returns: { type: 'T', desc: 'Result of the executed branch.' },
        example: 'let status = if score >= 90 => "A" else => "B";'
    },
    'else': {
        title: 'else',
        category: 'Keyword',
        signature: 'if <cond> { ... } else { ... }',
        description: 'Alternative branch of a conditional expression.',
        example: 'if ok => doWork() else => logError()'
    },
    'while': {
        title: 'while',
        category: 'Keyword',
        signature: 'while <condition> { <Body> }',
        description: 'Iterates repeatedly as long as the condition evaluates to `true`. Supports labeled `break` and `continue`.',
        params: [
            { name: 'condition', type: 'Bool', desc: 'Loop invariant test.' }
        ],
        returns: { type: 'Unit', desc: 'Evaluates to Unit () upon loop exit.' },
        example: 'let mut i = 0;\nwhile i < 10 {\n    println(`i: ${i}`);\n    i = i + 1;\n}'
    },
    'for': {
        title: 'for',
        category: 'Keyword',
        signature: 'for <item> in <iterable_or_channel> { <Body> }',
        description: 'Iterates over a sequence (list, range, or CSP channel). When iterating over a channel, receives items until the channel is closed.',
        params: [
            { name: 'item', type: 'Pattern', desc: 'Variable or pattern binding per item.' },
            { name: 'iterable', type: 'List<T> | Range | Channel<T>', desc: 'Target collection or channel.' }
        ],
        returns: { type: 'Unit', desc: 'Evaluates to Unit ().' },
        example: '// Channel range iteration\nfor val in ch {\n    println(`Received: ${val}`);\n}\n\n// Numeric range\nfor i in 0..5 {\n    println(i);\n}'
    },
    'in': {
        title: 'in',
        category: 'Keyword',
        signature: 'for <item> in <collection>',
        description: 'Specifies the collection, range, or channel to iterate over in a `for` loop.',
        example: 'for x in [1, 2, 3] { ... }'
    },
    'break': {
        title: 'break',
        category: 'Keyword',
        signature: 'break; / break \'<label>;',
        description: 'Terminates execution of the innermost or labeled enclosing loop.',
        example: '\'outer: for i in 0..10 {\n    for j in 0..10 {\n        if i * j > 50 => break \'outer;\n    }\n}'
    },
    'continue': {
        title: 'continue',
        category: 'Keyword',
        signature: 'continue; / continue \'<label>;',
        description: 'Skips the remainder of the current iteration and proceeds to the next cycle.',
        example: 'for i in 0..10 {\n    if i % 2 == 0 => continue;\n    println(i);\n}'
    },
    'return': {
        title: 'return',
        category: 'Keyword',
        signature: 'return <expression>;',
        description: 'Exits early from the enclosing function, returning the specified value.',
        example: 'if invalid => return Err("Invalid input");'
    },
    'import': {
        title: 'import',
        category: 'Keyword',
        signature: 'import { <Symbol> } from "<path_or_module>";',
        description: 'Imports types, functions, and interfaces from another `.aura` file or npm module.',
        example: 'import { User, getUserById } from "./models/user.aura";\nimport { Router } from "net/http";'
    },
    'export': {
        title: 'export',
        category: 'Keyword',
        signature: 'export fn <name>... / export type <Name>...',
        description: 'Marks declarations as public to other modules and generates TypeScript declaration files (`.d.ts`).',
        example: 'export fn main(): Unit => {\n    println("Entry point");\n}'
    },

    // ------------------------------------------------------------------------
    // Core Types
    // ------------------------------------------------------------------------
    'Int': {
        title: 'Int',
        category: 'Type',
        signature: 'type Int',
        description: '64-bit signed integer primitive type (`i64`). Supports arithmetic operators (`+`, `-`, `*`, `/`, `%`) and bitwise operators (`&`, `|`, `^`, `<<`, `>>`).',
        example: 'let count: Int = 42;\nlet hexValue: Int = 0xFF;'
    },
    'Float': {
        title: 'Float',
        category: 'Type',
        signature: 'type Float',
        description: '64-bit IEEE 754 double-precision floating-point primitive type (`f64`).',
        example: 'let pi: Float = 3.14159265359;\nlet rate = 0.05;'
    },
    'String': {
        title: 'String',
        category: 'Type',
        signature: 'type String',
        description: 'Immutable UTF-8 string type. Supports double quotes and backtick template string interpolation (`${expr}`).',
        example: 'let greeting: String = "Hello";\nlet formatted = `Value: ${greeting}` ;'
    },
    'Bool': {
        title: 'Bool',
        category: 'Type',
        signature: 'type Bool = true | false',
        description: 'Boolean primitive type representing logical truth values (`true` or `false`).',
        example: 'let isReady: Bool = true;\nlet isValid = isReady && (count > 0);'
    },
    'Unit': {
        title: 'Unit',
        category: 'Type',
        signature: 'type Unit = ()',
        description: 'Unit type denoting the absence of a meaningful value (equivalent to `void` or empty tuple `()`). Used as return type for functions that produce side-effects.',
        example: 'fn log(msg: String): Unit => {\n    println(msg);\n}'
    },
    'Byte': {
        title: 'Byte',
        category: 'Type',
        signature: 'type Byte',
        description: '8-bit unsigned integer type (`0..255`). Used for raw binary buffers and network payloads.',
        example: 'let b: Byte = 255;'
    },
    'Option': {
        title: 'Option<T>',
        category: 'Type',
        signature: 'type Option<T> = | Some(T) | None',
        description: 'Represents an optional value: every `Option` is either `Some` containing a value of type `T`, or `None`. Eliminates null reference exceptions.',
        example: 'fn findUser(id: Int): Option<User> => {\n    if id == 1 => Some(user)\n    else => None\n}\n\nmatch findUser(42) {\n    Some(u) => println(u.username),\n    None => println("User not found"),\n}'
    },
    'Result': {
        title: 'Result<T, E>',
        category: 'Type',
        signature: 'type Result<T, E> = | Ok(T) | Err(E)',
        description: 'Represents either success (`Ok`) containing a value of type `T`, or failure (`Err`) containing an error of type `E`. Unwrapped with `?` operator or `match`.',
        example: 'fn divide(a: Float, b: Float): Result<Float, String> => {\n    if b == 0.0 => Err("Division by zero")\n    else => Ok(a / b)\n}\n\nlet res = divide(10.0, 2.0)?;'
    },
    'Channel': {
        title: 'Channel<T>',
        category: 'Type',
        signature: 'Channel<T>::new(buffer_size: Int = 0): Channel<T>',
        description: 'Go-style typed CSP concurrency channel. Transports messages of type `T` between fibers. Send with `ch <- val`, receive with `<-ch`. Buffered if `buffer_size > 0`.',
        params: [
            { name: 'buffer_size', type: 'Int', desc: 'Capacity of channel (0 = unbuffered rendezvous).' }
        ],
        returns: { type: 'Channel<T>', desc: 'New concurrent channel instance.' },
        example: 'let ch = Channel<String>::new(10);\n\n// Send\nch <- "Hello";\n\n// Receive\nlet msg = <-ch;'
    },
    'SendChannel': {
        title: 'SendChannel<T>',
        category: 'Type',
        signature: 'type SendChannel<T>',
        description: 'A write-only channel view. Only send operations (`ch <- val`) are permitted at compile time.',
        example: 'fn producer(out: SendChannel<Int>): Unit => {\n    out <- 1;\n}'
    },
    'RecvChannel': {
        title: 'RecvChannel<T>',
        category: 'Type',
        signature: 'type RecvChannel<T>',
        description: 'A read-only channel view. Only receive operations (`<-ch`) are permitted at compile time.',
        example: 'fn consumer(inCh: RecvChannel<Int>): Unit => {\n    let val = <-inCh;\n    println(val);\n}'
    },
    'Task': {
        title: 'Task<T, E>',
        category: 'Type',
        signature: 'type Task<T, E>',
        description: 'Asynchronous task representation. Can be awaited with `await` or chained with functional combinators.',
        example: 'let task: Task<String, Error> = http::fetch("https://api.example.com");\nlet body = await task?;'
    },
    'Context': {
        title: 'Context',
        category: 'Type',
        signature: 'Context::background() / Context::withTimeout(ctx, duration)',
        description: 'Structured concurrency context for propagating cancellation signals, deadlines, and request-scoped values across fibers.',
        example: 'let ctx = Context::withTimeout(Context::background(), 5000);\nlet res = await fetchWithContext(ctx, url)?;'
    },
    'WaitGroup': {
        title: 'WaitGroup',
        category: 'Type',
        signature: 'type WaitGroup',
        description: 'Synchronization primitive for waiting on a collection of concurrent fibers to finish. Methods: `add(n: Int)`, `done(): Unit`, `wait(): Unit`.',
        example: 'let wg = sync::WaitGroup::new();\nwg.add(1);\nspawn {\n    defer wg.done();\n    doWork();\n};\nwg.wait();'
    },
    'Mutex': {
        title: 'Mutex',
        category: 'Type',
        signature: 'type Mutex',
        description: 'Mutual exclusion lock primitive. Call `lock()` to acquire exclusive access and `unlock()` to release it.',
        example: 'let mu = sync::Mutex::new();\nmu.lock();\ndefer mu.unlock();\n// protected critical section'
    },

    // ------------------------------------------------------------------------
    // Built-in Functions
    // ------------------------------------------------------------------------
    'println': {
        title: 'println',
        category: 'Built-in Function',
        signature: 'println(value: Any): Unit',
        description: 'Prints string representation of the argument followed by a newline to standard output (`stdout`).',
        params: [
            { name: 'value', type: 'Any', desc: 'Value to display.' }
        ],
        returns: { type: 'Unit', desc: 'Produces no value.' },
        example: 'println("Hello, World!");\nprintln(`Number: ${42}`);'
    },
    'print': {
        title: 'print',
        category: 'Built-in Function',
        signature: 'print(value: Any): Unit',
        description: 'Prints string representation of the argument to standard output (`stdout`) without a trailing newline.',
        params: [
            { name: 'value', type: 'Any', desc: 'Value to display.' }
        ],
        returns: { type: 'Unit', desc: 'Produces no value.' },
        example: 'print("Loading: ");\nprint("[██████████] 100%\\n");'
    },
    'eprintln': {
        title: 'eprintln',
        category: 'Built-in Function',
        signature: 'eprintln(value: Any): Unit',
        description: 'Prints string representation followed by a newline to standard error (`stderr`).',
        params: [
            { name: 'value', type: 'Any', desc: 'Error message to display.' }
        ],
        returns: { type: 'Unit', desc: 'Produces no value.' },
        example: 'eprintln("ERROR: Database connection failed");'
    },
    'len': {
        title: 'len',
        category: 'Built-in Function',
        signature: 'len(container: List<T> | String | Map<K, V> | Channel<T>): Int',
        description: 'Returns the number of elements in a list, characters in a string, entries in a map, or pending buffered items in a channel.',
        params: [
            { name: 'container', type: 'Container', desc: 'List, String, Map, or Channel.' }
        ],
        returns: { type: 'Int', desc: 'Length or element count.' },
        example: 'let items = [1, 2, 3];\nprintln(`Count: ${len(items)}`); // 3'
    },

    // ------------------------------------------------------------------------
    // Native Database Drivers & Stdlib Modules
    // ------------------------------------------------------------------------
    'pg': {
        title: 'pg / postgres',
        category: 'Module',
        signature: 'pg::connect(connectionString: String): Result<PgClient, Error>',
        description: 'Built-in high-performance PostgreSQL client driver. Supports connection pooling, parameterized queries, transactions, and automatic JSON column mapping.',
        example: 'let db = pg::connect("postgres://postgres:pass@localhost:5432/main")?;\ndefer db.close();\n\nlet rows = db.query("SELECT id, name FROM users WHERE active = $1", [true])?;'
    },
    'redis': {
        title: 'redis',
        category: 'Module',
        signature: 'redis::connect(connectionString: String): Result<RedisClient, Error>',
        description: 'Built-in Redis client. Supports key-value caching, Pub/Sub channels, hashes, lists, and atomic transactions.',
        example: 'let client = redis::connect("redis://127.0.0.1:6379")?;\nclient.set("session:123", userJson, 3600)?;\nlet val = client.get("session:123")?;'
    },
    'mysql': {
        title: 'mysql',
        category: 'Module',
        signature: 'mysql::connect(connectionString: String): Result<MySqlClient, Error>',
        description: 'Built-in MySQL client driver with native prepared statement support and connection pooling.',
        example: 'let db = mysql::connect("mysql://user:pass@localhost:3306/db")?;'
    },
    'mongo': {
        title: 'mongo / mongodb',
        category: 'Module',
        signature: 'mongo::connect(connectionString: String): Result<MongoClient, Error>',
        description: 'Built-in MongoDB document database driver with type-safe BSON/JSON document querying.',
        example: 'let client = mongo::connect("mongodb://localhost:27017")?;\nlet collection = client.db("app").collection("users");'
    },
    'http': {
        title: 'http',
        category: 'Module',
        signature: 'http::Server::new() / http::get(url) / http::post(url, body)',
        description: 'Built-in HTTP server and client framework with fast routing, middleware chaining, and fiber-per-request concurrency.',
        example: 'let app = http::Server::new();\napp.get("/users/:id", fn(req, res) => {\n    res.json({ id: req.params.id });\n});\napp.listen(8080);'
    },

    // ------------------------------------------------------------------------
    // Operators
    // ------------------------------------------------------------------------
    '|>': {
        title: '|>',
        category: 'Operator',
        signature: '<value> |> <function>(<extra_args>)',
        description: 'The **Pipeline Operator**. Passes the left-hand expression as the first argument to the right-hand function call: `data |> f` is equivalent to `f(data)`. Allows clean linear data transformations.',
        example: 'let result = numbers\n    |> filter(fn(x) => x > 0)\n    |> map(fn(x) => x * 2)\n    |> sum();'
    },
    '<-': {
        title: '<-',
        category: 'Operator',
        signature: '<channel> <- <value> (send)  |  <-<channel> (receive)',
        description: 'Channel communication operator. Sends a message to a channel (`ch <- val`) or receives a message from a channel (`let val = <-ch`).',
        example: '// Send\nch <- "ping";\n\n// Receive\nlet msg = <-ch;'
    },
    '=>': {
        title: '=>',
        category: 'Operator',
        signature: 'fn(<params>): <Type> => <body>  |  <pattern> => <body>',
        description: 'Fat arrow operator. Defines lambda and function bodies, and separates match pattern arms from their evaluated branches.',
        example: 'let double = fn(x: Int): Int => x * 2;\n\nmatch val {\n    1 => "One",\n    _ => "Other"\n}'
    },
    '?': {
        title: '?',
        category: 'Operator',
        signature: '<result_or_option_expression>?',
        description: 'Error propagation operator. If the operand is `Ok(v)` or `Some(v)`, unwraps the inner value `v`. If `Err(e)` or `None`, early returns immediately from the enclosing function.',
        example: 'let file = openFile("config.json")?;\nlet data = parseJson(file)?;'
    },
    '@test': {
        title: '@test',
        category: 'Decorator',
        signature: '@test\nfn test_<name>(): Unit => { ... }',
        description: 'Test case decorator recognized by the `auratest` runner. Functions decorated with `@test` are executed automatically with isolation and coverage reporting.',
        example: '@test\nfn test_addition(): Unit => {\n    assert_eq(1 + 1, 2);\n}'
    }
};

export function createMarkdownHover(doc: HoverDoc): vscode.Hover {
    const md = new vscode.MarkdownString();
    md.isTrusted = true;
    md.supportHtml = true;

    // Header badge
    const badge = `**Aura ${doc.category}**`;
    md.appendMarkdown(`### ${doc.title} &nbsp; *(${badge})*\n\n`);

    // Signature
    md.appendCodeblock(doc.signature, 'aura');

    // Description
    md.appendMarkdown(`${doc.description}\n\n`);

    // Parameters table/list
    if (doc.params && doc.params.length > 0) {
        md.appendMarkdown(`---\n#### 📥 Parámetros / Entrada\n`);
        for (const p of doc.params) {
            md.appendMarkdown(`* **\`${p.name}\`** (\`${p.type}\`): ${p.desc}\n`);
        }
        md.appendMarkdown(`\n`);
    }

    // Return value
    if (doc.returns) {
        md.appendMarkdown(`---\n#### 📤 Retorno / Tipo Resultante\n`);
        md.appendMarkdown(`* **Tipo:** \`${doc.returns.type}\`\n`);
        md.appendMarkdown(`* **Detalle:** ${doc.returns.desc}\n\n`);
    }

    // Code Example
    if (doc.example) {
        md.appendMarkdown(`---\n#### 💡 Ejemplo de Código\n`);
        md.appendCodeblock(doc.example, 'aura');
    }

    return new vscode.Hover(md);
}

export class AuraHoverProvider implements vscode.HoverProvider {
    provideHover(
        document: vscode.TextDocument,
        position: vscode.Position,
        _token: vscode.CancellationToken
    ): vscode.ProviderResult<vscode.Hover> {
        // Regex to capture identifiers, keywords, decorator (@test), and operators (->, <-, |>, =>, ?)
        const range = document.getWordRangeAtPosition(
            position,
            /@[a-zA-Z_][a-zA-Z0-9_]*|\b[a-zA-Z_][a-zA-Z0-9_]*\b|\|>|<-|->|=>|\?/
        );

        if (!range) {
            return undefined;
        }

        const word = document.getText(range);
        const doc = HOVER_DATABASE[word];

        if (doc) {
            return createMarkdownHover(doc);
        }

        return undefined;
    }
}
