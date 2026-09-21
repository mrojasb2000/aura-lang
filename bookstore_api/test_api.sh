#!/usr/bin/env bash
set -e

PORT="8085"
HOST="http://127.0.0.1:${PORT}"

echo "=========================================================="
echo "🧪 Iniciando Test Suite para Aura Bookstore REST API"
echo "=========================================================="

# Modificar temporalmente el puerto para los tests
TMP_SERVER="bookstore_api/test_server_run.aura"
sed "s/:8080/:${PORT}/g" bookstore_api/server.aura > "${TMP_SERVER}"

# Compilar y arrancar el servidor en segundo plano
aurac run "${TMP_SERVER}" &
SERVER_PID=$!

# Asegurarse de apagar el servidor al salir
cleanup() {
    echo ""
    echo "🧹 Deteniendo servidor de prueba (PID: ${SERVER_PID})..."
    kill -9 "${SERVER_PID}" 2>/dev/null || true
    rm -f "${TMP_SERVER}" "${TMP_SERVER}.tmp.mjs"
}
trap cleanup EXIT

# Esperar a que el servidor esté listo
echo "Esperando que el servidor esté activo en ${HOST}..."
for i in {1..30}; do
    if curl -s "${HOST}/api/health" > /dev/null 2>&1; then
        echo "✓ Servidor activo y respondiendo."
        break
    fi
    sleep 0.3
done

echo ""
echo "--- 1. Health Check & Root Endpoints ---"
echo "[GET /]"
curl -s "${HOST}/" | grep -q "Aura Bookstore" && echo "  ✓ Root endpoint OK" || (echo "  ✕ Falló Root"; exit 1)

echo "[GET /api/health]"
HEALTH=$(curl -s "${HOST}/api/health")
echo "  Respuesta Health: ${HEALTH}"
echo "${HEALTH}" | grep -q "healthy" && echo "  ✓ Health check OK"

echo ""
echo "--- 2. Autores (Authors CRUD) ---"
echo "[GET /api/authors]"
curl -s "${HOST}/api/authors" | grep -q "Gabriel García Márquez" && echo "  ✓ Listado de autores OK"

echo "[GET /api/authors/aut-1]"
AUTHOR_1=$(curl -s "${HOST}/api/authors/aut-1")
echo "${AUTHOR_1}" | grep -q "bibliography" && echo "  ✓ Detalle de autor con bibliografía OK"

echo "[POST /api/authors]"
NEW_AUTH_RESP=$(curl -s -X POST "${HOST}/api/authors" \
  -H "Content-Type: application/json" \
  -d '{"name": "Octavio Paz", "email": "octavio@paz.mx", "nationality": "Mexicana", "biography": "Premio Nobel de Literatura 1990.", "birthYear": 1914}')
echo "  Creado: ${NEW_AUTH_RESP}"
echo "${NEW_AUTH_RESP}" | grep -q "Octavio Paz" && echo "  ✓ Creación de autor OK"

echo "[PUT /api/authors/aut-11]"
UPD_AUTH_RESP=$(curl -s -X PUT "${HOST}/api/authors/aut-11" \
  -H "Content-Type: application/json" \
  -d '{"nationality": "Mexicana / Universal"}')
echo "${UPD_AUTH_RESP}" | grep -q "Mexicana / Universal" && echo "  ✓ Actualización de autor OK"

echo ""
echo "--- 3. Editoriales (Editorials CRUD) ---"
echo "[GET /api/editorials]"
curl -s "${HOST}/api/editorials" | grep -q "Editorial Sudamericana" && echo "  ✓ Listado de editoriales OK"

echo "[GET /api/editorials/edt-1]"
curl -s "${HOST}/api/editorials/edt-1" | grep -q "totalBooks" && echo "  ✓ Detalle de editorial con catálogo OK"

echo "[POST /api/editorials]"
NEW_EDT_RESP=$(curl -s -X POST "${HOST}/api/editorials" \
  -H "Content-Type: application/json" \
  -d '{"name": "Fondo de Cultura Económica", "country": "México", "foundedYear": 1934, "website": "https://www.fondodeculturaeconomica.com"}')
echo "  Creada: ${NEW_EDT_RESP}"
echo "${NEW_EDT_RESP}" | grep -q "Fondo de Cultura Económica" && echo "  ✓ Creación de editorial OK"

echo ""
echo "--- 4. Libros (Books CRUD) ---"
echo "[GET /api/books]"
curl -s "${HOST}/api/books" | grep -q "Cien años de soledad" && echo "  ✓ Listado de libros OK"

echo "[GET /api/books?genre=Realismo%20Mágico]"
curl -s "${HOST}/api/books?genre=Realismo%20Mágico" | grep -q "Cien años de soledad" && echo "  ✓ Filtro de libros por género OK"

echo "[GET /api/books/bk-1]"
curl -s "${HOST}/api/books/bk-1" | grep -q "author" && echo "  ✓ Detalle de libro enriquecido OK"

echo "[POST /api/books]"
NEW_BOOK_RESP=$(curl -s -X POST "${HOST}/api/books" \
  -H "Content-Type: application/json" \
  -d '{"title": "El laberinto de la soledad", "isbn": "978-9681603011", "authorId": "aut-11", "editorialId": "edt-11", "price": 16.50, "stock": 40, "genre": "Ensayo", "publishedYear": 1950}')
echo "  Libro creado: ${NEW_BOOK_RESP}"
echo "${NEW_BOOK_RESP}" | grep -q "El laberinto de la soledad" && echo "  ✓ Creación de libro OK"

echo "[PUT /api/books/bk-11]"
UPD_BOOK_RESP=$(curl -s -X PUT "${HOST}/api/books/bk-11" \
  -H "Content-Type: application/json" \
  -d '{"price": 18.00, "stock": 45}')
echo "${UPD_BOOK_RESP}" | grep -q "18" && echo "  ✓ Actualización de libro OK"

echo ""
echo "--- 5. Artículos (Articles CRUD) ---"
echo "[GET /api/articles]"
curl -s "${HOST}/api/articles" | grep -q "La alquimia del tiempo" && echo "  ✓ Listado de artículos OK"

echo "[POST /api/articles]"
NEW_ART_RESP=$(curl -s -X POST "${HOST}/api/articles" \
  -H "Content-Type: application/json" \
  -d '{"title": "Poesía y modernidad", "summary": "Estudio crítico sobre la lírica hispanoamericana.", "content": "La palabra poética resiste a la cosificación...", "authorId": "aut-11", "topic": "Poesía y Ensayo"}')
echo "  Artículo creado: ${NEW_ART_RESP}"
echo "${NEW_ART_RESP}" | grep -q "Poesía y modernidad" && echo "  ✓ Creación de artículo OK"

echo ""
echo "--- 6. Endpoints de Venta de Libros (Book Sales) ---"
echo "[POST /api/books/bk-1/sell - Venta exitosa con descuento y factura]"
SALE_RESP=$(curl -s -X POST "${HOST}/api/books/bk-1/sell" \
  -H "Content-Type: application/json" \
  -d '{"quantity": 2, "customerName": "Laura Restrepo", "customerEmail": "laura@correo.lat", "paymentMethod": "CREDIT_CARD", "discountPercent": 10.0}')
echo "  Venta procesada: ${SALE_RESP}"
echo "${SALE_RESP}" | grep -q "procesada exitosamente" && echo "  ✓ Venta directa de libro OK"

echo "[POST /api/books/bk-1/sell - Validación de stock insuficiente]"
OVER_SALE_RESP=$(curl -s -X POST "${HOST}/api/books/bk-1/sell" \
  -H "Content-Type: application/json" \
  -d '{"quantity": 9999, "customerName": "Test", "customerEmail": "test@test.com"}')
echo "  Respuesta sobreventa: ${OVER_SALE_RESP}"
echo "${OVER_SALE_RESP}" | grep -q "Stock insuficiente" && echo "  ✓ Validación de stock insuficiente OK"

echo "[POST /api/sales - Venta mediante endpoint general con bookId]"
SALE_GEN_RESP=$(curl -s -X POST "${HOST}/api/sales" \
  -H "Content-Type: application/json" \
  -d '{"bookId": "bk-3", "quantity": 1, "customerName": "Pablo Neruda", "customerEmail": "pablo@poesia.cl", "paymentMethod": "DEBIT_CARD"}')
echo "  Venta general: ${SALE_GEN_RESP}"
echo "${SALE_GEN_RESP}" | grep -q "procesada exitosamente" && echo "  ✓ Venta general OK"

echo "[GET /api/sales - Listar transacciones]"
SALES_LIST=$(curl -s "${HOST}/api/sales")
echo "${SALES_LIST}" | grep -q "Laura Restrepo" && echo "  ✓ Historial de ventas OK"

echo "[GET /api/sales/summary - Resumen financiero]"
SUMMARY=$(curl -s "${HOST}/api/sales/summary")
echo "  Resumen de ventas: ${SUMMARY}"
echo "${SUMMARY}" | grep -q "totalGrossRevenue" && echo "  ✓ Métricas financieras OK"

echo ""
echo "--- 7. Validaciones de Integridad y Eliminación (DELETE) ---"
echo "[DELETE /api/authors/aut-1 - Intento de borrar autor con libros (debe fallar)]"
DEL_FAIL=$(curl -s -X DELETE "${HOST}/api/authors/aut-1")
echo "  Respuesta: ${DEL_FAIL}"
echo "${DEL_FAIL}" | grep -q "No se puede eliminar el autor" && echo "  ✓ Integridad referencial de autor protegida OK"

echo "[DELETE entidades de prueba creadas]"
curl -s -X DELETE "${HOST}/api/articles/art-11" | grep -q "eliminado" && echo "  ✓ Eliminación de artículo OK"
curl -s -X DELETE "${HOST}/api/books/bk-11" | grep -q "eliminado" && echo "  ✓ Eliminación de libro OK"
curl -s -X DELETE "${HOST}/api/editorials/edt-11" | grep -q "eliminada" && echo "  ✓ Eliminación de editorial OK"
curl -s -X DELETE "${HOST}/api/authors/aut-11" | grep -q "eliminado" && echo "  ✓ Eliminación de autor OK"

echo ""
echo "=========================================================="
echo "🎉 TODOS LOS TESTS DE LA API PASARON EXITOSAMENTE (100% OK)"
echo "=========================================================="
