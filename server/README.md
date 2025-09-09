# AIBOM Generator Server

An HTTP server based on actix_web that provides the same AIBOM generation functionality as the CLI tool.

## Starting the Server

```bash
cargo run -p server
```

The server will start at `http://localhost:8080`.

## API Endpoints

### Health Check

```bash
GET /health
```

Response:
```json
{
  "status": "healthy",
  "service": "aibom-generator-server"
}
```

### Generate AIBOM

```bash
POST /generate
```

Request Body:
```json
{
  "model_id": "microsoft/DialoGPT-medium",
  "verbose": false
}
```

Response:
```json
{
  "success": true,
  "aibom": { ... },
  "error": null
}
```

## Usage Examples

### Using curl

```bash
# Health check
curl http://localhost:8080/health

# Generate AIBOM
curl -X POST http://localhost:8080/generate \
  -H "Content-Type: application/json" \
  -d '{"model_id": "microsoft/DialoGPT-medium", "verbose": true}'
```



## Feature Comparison

| Feature | CLI | Server |
|---------|-----|--------|
| Generate AIBOM | ✅ | ✅ |
| Specify model_id | ✅ | ✅ |
| Verbose Output | ✅ | ✅ |
| Save to File | ✅ | ❌ (Returns JSON) |
| HTTP API | ❌ | ✅ |
| Health Check | ❌ | ✅ |

The server version returns AIBOM JSON data through HTTP API instead of saving directly to files, making it more suitable for integration with other systems.