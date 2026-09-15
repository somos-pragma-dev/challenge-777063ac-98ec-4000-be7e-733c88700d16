# Prompt para Mejorar el Codigo Base

Copia y pega el contenido del bloque de abajo en un asistente de IA (Claude, ChatGPT)
para obtener un ZIP con el proyecto completo y arrancable.

Si preferis trabajar en tu editor con un agente local (Claude Code, Cursor, Copilot), usa `AGENTS.md` en vez de este archivo: dice lo mismo pero para que escriba los archivos en disco.

## Las dos reglas que no se negocian

1. **Completa el boilerplate.** Todo lo que el proyecto necesita para compilar y arrancar: manifiesto de dependencias, punto de entrada, configuracion, capa de interfaz, y las capas del patron arquitectonico declarado. Eso es andamiaje y es tu trabajo.
2. **NO resuelvas el reto.** Los entregables de las fases son el trabajo de la persona. El hueco pedagogico se deja como esta: el proyecto arranca, pero lo que el reto pide implementar NO esta implementado.

Dicho de otra forma: si algo impide compilar, arreglalo. Si algo es logica de negocio incompleta, validaciones ausentes, un secreto hardcodeado o un patron mejorable, dejalo exactamente como esta — es lo que la persona tiene que encontrar.

## Lo que le falta a este proyecto

Esto NO lo tenes que adivinar: salio de comparar el proyecto contra la arquitectura declarada del reto y de un analisis estatico del codigo. Completalo TODO.

### Boilerplate del stack que falta

Sin esto no compila ni arranca. Es andamiaje, no toca nada de lo pedagogico:

- **Punto de entrada del stack elegido** — Sin un punto de entrada reconocible, el runtime no tiene por donde arrancar la aplicacion.
- **Capa de interfaz (controller/handler)** — Sin una capa de interfaz explicita, no hay forma de invocar la logica de negocio desde afuera del proceso.

## Como saber que terminaste

```bash
el comando de build o arranque canonico del stack elegido
```

Ese comando corriendo sin errores es la definicion de "listo".

---

```
## Briefing del reto (autoridad)
Este bloque manda sobre los archivos adjuntos. El stack y el rol salen de AQUÍ, no de un topic genérico ni de markdown placeholder.

### Contexto técnico original
Build a REST API with Rust, Actix Web and Diesel ORM

### Reto
- Tema: rust-actix-web
- Seniority: junior-l1
- Tipo: practical
- Título: Desarrollo de API REST con Rust y Actix Web
- Tiempo estimado: 8 horas

### Fases (trabajo del HUMANO — PROHIBIDO completarlas)
No implementes estos entregables. Dejalos como hueco pedagógico. El asistente solo materializa el proyecto arrancable para que el participante pueda trabajar.
- Fase 1: Definición de endpoints y modelos de datos — objetivo: Definir los endpoints necesarios para la API y los modelos de datos correspondientes. — entregable (NO resolver): Definición de endpoints y modelos de datos para la API REST.
- Fase 2: Implementación de endpoints y lógica de negocio — objetivo: Implementar los endpoints definidos y la lógica de negocio correspondiente. — entregable (NO resolver): Implementación de endpoints y lógica de negocio para la API REST.
- Fase 3: Pruebas y optimización — objetivo: Realizar pruebas unitarias y de integración para asegurar la funcionalidad correcta de la API y optimizar su rendimiento. — entregable (NO resolver): Pruebas unitarias y de integración, y optimización de rendimiento para la API REST.

Eres un asistente experto en análisis, corrección y generación de archivos de cualquier tipo:
código fuente, documentación, hojas de cálculo, documentos Word, configuraciones, entre otros.
Voy a enviarte una cadena de texto que contiene uno o más archivos. Cada archivo está delimitado por un marcador con el siguiente formato:
// === ARCHIVO: ruta/del/archivo.extension ===
o también puede aparecer como:
## === ARCHIVO: ruta/del/archivo.extension ===
Lo que sigue al marcador puede ser:

El contenido real del archivo (código, texto, YAML, etc.)
Una descripción en lenguaje natural de lo que debe contener el archivo


TU TAREA
PASO 0 — ¿Esto es un proyecto o una carcasa?
Antes de extraer archivos, leé el Briefing (si está) y diagnosticá el adjunto.

Es CARCASA si ocurre CUALQUIERA de estas:
- No hay manifiesto de dependencias del stack del briefing (manifest.json de VTEX IO / package.json / pom.xml / build.gradle / requirements.txt / go.mod / *.tf / *.csproj, según corresponda)
- Hay un "binario" que en realidad es un comentario ("no puede ser mostrado como texto plano", placeholder .fig/.docx vacío)
- Los markdowns ya completan entregables de fases posteriores ("se implementó fade-in", lista de áreas ya resuelta)

Si es CARCASA:
- MATERIALIZÁ un proyecto que arranca en el stack del briefing (VTEX IO Store Framework, Angular, Terraform, pytest, Nest, etc.). Incluí manifiesto, punto de entrada y capa de interfaz reales.
- NO copies los markdowns de "solución" como si fueran el producto. Son ruido de generación.
- NO resuelvas las fases del briefing (están marcadas PROHIBIDO). Dejá el hueco pedagógico: el flujo existe, las microinteracciones/calidad/infra que el reto pide NO están hechas.
- Después seguí al PASO 5 (ZIP).

Si es un proyecto REAL (manifiesto + código que compila o arranca):
- Seguí PASO 1 en adelante. 🔴 compilación sí. 🟡 pedagógico no.

PASO 1 — Detección y extracción
Identifica todos los archivos presentes en la cadena. Para cada archivo extrae:

Su ruta completa (ej: src/main/java/com/pragma/Service.java)
Su contenido o descripción

PASO 2 — Clasificación por tipo
Clasifica cada archivo en una de estas categorías:
A) Código fuente (Java, Python, TypeScript, JavaScript, Kotlin, etc.)
B) Configuración / documentación (YAML, properties, Markdown, JSON, txt, etc.)
C) Excel (.xlsx, .xls, .csv)
D) Word (.docx, .doc)
E) Otro tipo de archivo binario o especial
PASO 3 — Clasificación de errores en código fuente

Objetivo prioritario: que el proyecto compile. No corrijas flujo de negocio ni lógica funcional.

Antes de modificar cualquier archivo de código fuente, clasifica cada problema encontrado en una de estas dos categorías:
🔴 ERROR DE COMPILACIÓN — corregir siempre
Son errores que impiden que el proyecto arranque, sin valor pedagógico:

Import faltante o incorrecto
Clase, método o variable referenciada que no existe en ningún archivo del proyecto
Error de sintaxis
Anotación con atributos inválidos
Dependencia ausente en pom.xml, package.json, etc.
Archivo referenciado que no existe y debe ser creado con implementación mínima

→ CORREGIR estos errores.
🟡 PROBLEMA FUNCIONAL O DE CALIDAD — preservar siempre
Son problemas que no impiden compilar. Pueden ser intencionales para el aprendizaje:

Clave secreta hardcodeada ("secret", "password123")
API deprecada que funciona pero tiene reemplazo moderno
Lógica de negocio incorrecta o incompleta
Código redundante o de baja legibilidad
Falta de validaciones en flujo de negocio
Patrones de diseño incorrectos pero funcionales
Concurrencia no segura
Configuración funcional pero no óptima

→ PRESERVAR tal cual. No corregir, no mejorar, no comentar.
PASO 4 — Procesamiento según tipo de archivo
Tipo A — Código fuente
Aplica únicamente las correcciones clasificadas como 🔴 ERROR DE COMPILACIÓN.
No alteres ningún elemento clasificado como 🟡 PROBLEMA FUNCIONAL O DE CALIDAD.
Si falta un archivo referenciado, créalo con la implementación mínima necesaria para compilar.
Tipo B — Configuración / documentación
Extrae el contenido tal cual, sin modificaciones salvo errores evidentes de sintaxis
(ej: YAML mal indentado).
Tipo C — Excel (.xlsx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un archivo Excel funcional con:

Fila de encabezados en negrita con color de fondo distintivo
Columnas con ancho ajustado al contenido
Tipos de dato correctos por columna
Validaciones si la descripción lo indica
Hojas nombradas descriptivamente si hay más de una
Filas de ejemplo si no hay datos reales

Tipo D — Word (.docx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un documento Word funcional con:

Estilos de título (Título 1, Título 2) para jerarquía de secciones
Fuente legible (Calibri o equivalente), tamaño 11-12pt para cuerpo
Márgenes estándar
Tabla de contenido si tiene múltiples secciones
Tablas con encabezados en negrita si aplica

Tipo E — Otro
Genera el archivo con el contenido o estructura más apropiada según la descripción.
PASO 5 — Exportación en ZIP
Empaqueta todos los archivos en un único archivo ZIP descargable respetando exactamente
la estructura de rutas indicada por los marcadores.
El ZIP debe incluir:

Archivos de código con únicamente los errores de compilación corregidos
Archivos de configuración y documentación sin cambios
Archivos nuevos creados para resolver dependencias de compilación faltantes
Archivos Excel y Word generados desde descripción

IMPORTANTE: El ZIP debe estar listo para descargar al finalizar. No preguntes si el usuario
quiere generarlo. Simplemente genera el archivo y proporciona el enlace de descarga; No debes desplegar en el chat el resumen de lo que arreglaste al Zip, solo entregalo.

REGLAS IMPORTANTES

No omitas ningún archivo aunque no tenga errores ni modificaciones
Respeta los nombres y rutas exactas indicadas por los marcadores
Si un archivo no tiene marcador claro, infiere el nombre desde su contenido
Si la cadena contiene solo documentación, placeholders o binarios fake, NO la reproduzcas:
aplicá PASO 0 (materializar el proyecto del briefing). Reproducir la carcasa es un fallo.
No agregues texto después del enlace de descarga del ZIP
No preguntes si el usuario quiere el ZIP: simplemente generalo siempre
Si detectas que falta un archivo de configuración necesario para compilar
(pom.xml, package.json, requirements.txt, build.gradle, etc.), créalo e inclúyelo
inferiendo su contenido desde los imports y frameworks detectados en el código
Nunca corrijas problemas 🟡 aunque parezcan obvios o fáciles de mejorar.
El participante que recibirá este proyecto los debe encontrar y resolver él mismo.


INPUT
Aquí está la cadena con los archivos:

// === ARCHIVO: package.json ===
{
  "name": "rust-actix-web-loans-api",
  "version": "1.0.0",
  "description": "REST API for loan management system built with Rust and Actix Web",
  "main": "src/main.rs",
  "scripts": {
    "dev": "cargo run",
    "build": "cargo build --release",
    "test": "cargo test"
  },
  "keywords": ["rust", "actix-web", "api", "rest", "loans"],
  "author": "",
  "license": "MIT",
  "devDependencies": {},
  "dependencies": {}
}

// === ARCHIVO: src/models/loan.rs ===
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Utc};
use std::fmt;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Loan {
    pub id: i64,
    pub amount: f64,
    pub interest_rate: f64,
    pub due_date: NaiveDate,
    pub status: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl Loan {
    pub fn new(id: i64, amount: f64, interest_rate: f64, due_date: NaiveDate) -> Self {
        let now = Utc::now();
        Self {
            id,
            amount,
            interest_rate,
            due_date,
            status: LoanStatus::Pending.to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_status(mut self, status: LoanStatus) -> Self {
        self.status = status.to_string();
        self.updated_at = Utc::now();
        self
    }

    pub fn is_active(&self) -> bool {
        self.status == LoanStatus::Active.to_string()
    }

    pub fn is_overdue(&self) -> bool {
        if self.status != LoanStatus::Active.to_string() {
            return false;
        }
        let today = Utc::now().date_naive();
        self.due_date < today
    }

    pub fn calculate_total_interest(&self) -> f64 {
        let days = self.days_until_due();
        let daily_rate = self.interest_rate / 365.0;
        self.amount * daily_rate * days as f64
    }

    pub fn calculate_total_amount(&self) -> f64 {
        self.amount + self.calculate_total_interest()
    }

    fn days_until_due(&self) -> i64 {
        let today = Utc::now().date_naive();
        if self.due_date > today {
            (self.due_date - today).num_days()
        } else {
            0
        }
    }

    pub fn update_amount(&mut self, new_amount: f64) -> Result<(), String> {
        if new_amount <= 0.0 {
            return Err("Loan amount must be positive".to_string());
        }
        self.amount = new_amount;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_interest_rate(&mut self, new_rate: f64) -> Result<(), String> {
        if new_rate < 0.0 || new_rate > 1.0 {
            return Err("Interest rate must be between 0.0 and 1.0".to_string());
        }
        self.interest_rate = new_rate;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_due_date(&mut self, new_due_date: NaiveDate) -> Result<(), String> {
        let today = Utc::now().date_naive();
        if new_due_date <= today {
            return Err("Due date must be in the future".to_string());
        }
        self.due_date = new_due_date;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn activate(&mut self) {
        self.status = LoanStatus::Active.to_string();
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.status = LoanStatus::Inactive.to_string();
        self.updated_at = Utc::now();
    }

    pub fn mark_as_paid(&mut self) {
        self.status = LoanStatus::Paid.to_string();
        self.updated_at = Utc::now();
    }

    pub fn mark_as_defaulted(&mut self) {
        self.status = LoanStatus::Defaulted.to_string();
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoanStatus {
    Pending,
    Active,
    Inactive,
    Paid,
    Defaulted,
}

impl LoanStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoanStatus::Pending => "pending",
            LoanStatus::Active => "active",
            LoanStatus::Inactive => "inactive",
            LoanStatus::Paid => "paid",
            LoanStatus::Defaulted => "defaulted",
        }
    }
}

impl fmt::Display for LoanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for LoanStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(LoanStatus::Pending),
            "active" => Ok(LoanStatus::Active),
            "inactive" => Ok(LoanStatus::Inactive),
            "paid" => Ok(LoanStatus::Paid),
            "defaulted" => Ok(LoanStatus::Defaulted),
            _ => Err(format!("Unknown loan status: {}", s)),
        }
    }
}

impl Default for LoanStatus {
    fn default() -> Self {
        LoanStatus::Pending
    }
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::loans)]
pub struct NewLoan {
    pub amount: f64,
    pub interest_rate: f64,
    pub due_date: NaiveDate,
}

impl NewLoan {
    pub fn validate(&self) -> Result<(), String> {
        if self.amount <= 0.0 {
            return Err("Amount must be greater than zero".to_string());
        }
        if self.interest_rate < 0.0 || self.interest_rate > 1.0 {
            return Err("Interest rate must be between 0 and 1 (0% to 100%)".to_string());
        }
        let today = Utc::now().date_naive();
        if self.due_date <= today {
            return Err("Due date must be in the future".to_string());
        }
        Ok(())
    }
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::loans)]
pub struct UpdateLoan {
    pub amount: Option<f64>,
    pub interest_rate: Option<f64>,
    pub due_date: Option<NaiveDate>,
    pub status: Option<String>,
}

impl UpdateLoan {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(amount) = self.amount {
            if amount <= 0.0 {
                return Err("Amount must be greater than zero".to_string());
            }
        }
        if let Some(rate) = self.interest_rate {
            if rate < 0.0 || rate > 1.0 {
                return Err("Interest rate must be between 0 and 1".to_string());
            }
        }
        if let Some(due_date) = self.due_date {
            let today = Utc::now().date_naive();
            if due_date <= today {
                return Err("Due date must be in the future".to_string());
            }
        }
        Ok(())
    }
}

// === ARCHIVO: src/dto/loan_dto.rs ===
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct CreateLoanRequest {
    #[validate(range(min = 0.01, message = "Amount must be greater than 0"))]
    pub amount: f64,
    
    #[validate(range(min = 0.0, max = 1.0, message = "Interest rate must be between 0 and 1"))]
    pub interest_rate: f64,
    
    #[validate(custom = "validate_future_date")]
    pub due_date: String,
}

fn validate_future_date(date: &str) -> Result<(), validator::ValidationError> {
    let parsed_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|_| validator::ValidationError::new("invalid_date_format"));
    
    match parsed_date {
        Ok(d) => {
            let today = chrono::Utc::now().date_naive();
            if d <= today {
                Err(validator::ValidationError::new("due_date_must_be_future"))
            } else {
                Ok(())
            }
        }
        Err(e) => Err(e),
    }
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct UpdateLoanRequest {
    #[validate(range(min = 0.01, message = "Amount must be greater than 0"))]
    pub amount: Option<f64>,
    
    #[validate(range(min = 0.0, max = 1.0, message = "Interest rate must be between 0 and 1"))]
    pub interest_rate: Option<f64>,
    
    #[validate(custom = "validate_future_date")]
    pub due_date: Option<String>,
    
    #[validate(length(min = 1, max = 20))]
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoanResponse {
    pub id: i64,
    pub amount: f64,
    pub interest_rate: f64,
    pub due_date: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub total_interest: Option<f64>,
    pub total_amount: Option<f64>,
}

impl LoanResponse {
    pub fn from_loan(loan: &crate::models::loan::Loan) -> Self {
        Self {
            id: loan.id,
            amount: loan.amount,
            interest_rate: loan.interest_rate,
            due_date: loan.due_date.format("%Y-%m-%d").to_string(),
            status: loan.status.clone(),
            created_at: loan.created_at.to_rfc3339(),
            updated_at: loan.updated_at.to_rfc3339(),
            total_interest: Some(loan.calculate_total_interest()),
            total_amount: Some(loan.calculate_total_amount()),
        }
    }

    pub fn from_loan_without_calculations(loan: &crate::models::loan::Loan) -> Self {
        Self {
            id: loan.id,
            amount: loan.amount,
            interest_rate: loan.interest_rate,
            due_date: loan.due_date.format("%Y-%m-%d").to_string(),
            status: loan.status.clone(),
            created_at: loan.created_at.to_rfc3339(),
            updated_at: loan.updated_at.to_rfc3339(),
            total_interest: None,
            total_amount: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoanListResponse {
    pub loans: Vec<LoanResponse>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}

impl LoanListResponse {
    pub fn new(loans: Vec<LoanResponse>, page: usize, per_page: usize) -> Self {
        Self {
            total: loans.len(),
            loans,
            page,
            per_page,
        }
    }

    pub fn empty(page: usize, per_page: usize) -> Self {
        Self {
            loans: Vec::new(),
            total: 0,
            page,
            per_page,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: String,
}

impl ErrorResponse {
    pub fn new(code: &str, error: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            error: error.to_string(),
            message: message.to_string(),
        }
    }

    pub fn validation_error(message: &str) -> Self {
        Self::new("VALIDATION_ERROR", "Validation Failed", message)
    }

    pub fn not_found(id: i64) -> Self {
        Self::new("NOT_FOUND", "Loan Not Found", &format!("Loan with id {} not found", id))
    }

    pub fn internal_error(message: &str) -> Self {
        Self::new("INTERNAL_ERROR", "Internal Server Error", message)
    }

    pub fn bad_request(message: &str) -> Self {
        Self::new("BAD_REQUEST", "Bad Request", message)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
    pub message: Option<String>,
}

impl<T> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data,
            message: None,
        }
    }

    pub fn with_message(data: T, message: &str) -> Self {
        Self {
            success: true,
            data,
            message: Some(message.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteResponse {
    pub success: bool,
    pub deleted_id: i64,
    pub message: String,
}

impl DeleteResponse {
    pub fn deleted(id: i64) -> Self {
        Self {
            success: true,
            deleted_id: id,
            message: format!("Loan with id {} was successfully deleted", id),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PaginationQuery {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(10),
        }
    }
}

impl PaginationQuery {
    pub fn get_page(&self) -> usize {
        self.page.unwrap_or(1)
    }

    pub fn get_per_page(&self) -> usize {
        let per_page = self.per_page.unwrap_or(10);
        if per_page > 100 {
            100
        } else {
            per_page
        }
    }

    pub fn get_offset(&self) -> usize {
        let page = self.get_page();
        let per_page = self.get_per_page();
        (page.saturating_sub(1)) * per_page
    }
}


// === ARCHIVO: Cargo.toml ===
[package]
name = "rust-actix-web-loans-api"
version = "1.0.0"
edition = "2021"
description = "REST API for loan management system built with Rust and Actix Web"
authors = [""]
license = "MIT"

[lib]
name = "rust_actix_web_loans_api"
path = "src/lib.rs"

[[bin]]
name = "rust-actix-web-loans-api"
path = "src/main.rs"

[dependencies]
actix-web = "4.9.0"
actix-web-validator = "5.0.1"
anyhow = "1.0.79"
diesel = { version = "2.1.0", features = ["postgres", "r2d2", "chrono", "serde_json"] }
diesel_migrations = "2.1.0"
dotenv = "0.15.0"
serde = { version = "1.0.196", features = ["derive"] }
serde_json = "1.0.113"
thiserror = "1.0.56"
tokio = { version = "1.36.0", features = ["full"] }
validator = { version = "2.1.0", features = ["derive"] }

[dev-dependencies]
actix-rt = "4.0.0"
actix-service = "2.0.0"
actix-testing = "1.0.0"
http = "1.0.0"

[features]
default = ["diesel/postgres"]

[profile.release]
opt-level = 3
lto = true
codegen-units = 1

[profile.dev]
opt-level = 0
debug = true

[workspace]
members = []
resolver = "2"

// === ARCHIVO: .env ===
# Configuration file for environment variables
# Copy this file to .env and update with your actual values

# Database Configuration
DATABASE_URL=postgres://postgres:postgres@localhost:5432/loans_db
DATABASE_POOL_SIZE=10
DATABASE_TIMEOUT_SECS=30

# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
SERVER_WORKERS=4

# Application Configuration
APP_ENV=development
APP_LOG_LEVEL=info
APP_SECRET_KEY=your-secret-key-here-change-in-production

# CORS Configuration
CORS_ALLOWED_ORIGINS=http://localhost:3000,http://localhost:8080
CORS_MAX_AGE_SECS=3600

# Rate Limiting Configuration
RATE_LIMIT_ENABLED=true
RATE_LIMIT_MAX_REQUESTS=100
RATE_LIMIT_WINDOW_SECS=60

# Loan Configuration
LOAN_DEFAULT_INTEREST_RATE=0.05
LOAN_MIN_AMOUNT=100.0
LOAN_MAX_AMOUNT=1000000.0
LOAN_DEFAULT_DUE_DAYS=30

# Pagination Configuration
DEFAULT_PAGE_SIZE=20
MAX_PAGE_SIZE=100

# Logging Configuration
LOG_FORMAT=json
LOG_FILE_PATH=/var/log/loans-api.log
LOG_MAX_FILE_SIZE_MB=100
LOG_MAX_FILES=5

# Security Configuration
BCRYPT_ROUNDS=12
JWT_EXPIRATION_HOURS=24
SESSION_TIMEOUT_MINUTES=60

# Feature Flags
ENABLE_SWAGGER_DOCS=false
ENABLE_METRICS=true
ENABLE_TRACING=false


// === ARCHIVO: src/main.rs ===
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use std::env;

mod schema;
mod models;
mod dto;
mod services;
mod repositories;
mod controllers;
mod error;

use controllers::loan_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("SERVER_PORT must be a valid number");
    
    println!("Starting server at http://{}:{}", host, port);
    
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(
                services::loan_service::LoanService::new(
                    repositories::loan_repository::PgLoanRepository::new()
                )
            ))
            .route("/health", web::get().to(health_check))
            .configure(loan_controller::configure)
    })
    .bind((host, port))?
    .run()
    .await
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "loan-api",
        "version": "1.0.0"
    })))
}

// === ARCHIVO: src/schema.rs ===
diesel::table! {
    loans (id) {
        id -> Int8,
        amount -> Float8,
        interest_rate -> Float8,
        due_date -> Timestamp,
        status -> VarChar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

#[derive(Insertable, Queryable, Identifiable, Associations, Debug, Clone)]
#[diesel(table_name = loans)]
#[diesel(belongs_to(LoanStatus))]
pub struct Loan {
    pub id: i64,
    pub amount: f64,
    pub interest_rate: f64,
    pub due_date: chrono::NaiveDateTime,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(QueryableByName, Debug, Clone)]
#[diesel(table_name = loans)]
pub struct LoanRow {
    pub id: i64,
    pub amount: f64,
    pub interest_rate: f64,
    pub due_date: chrono::NaiveDateTime,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Loan {
    pub fn from_row(row: LoanRow) -> Self {
        Loan {
            id: row.id,
            amount: row.amount,
            interest_rate: row.interest_rate,
            due_date: row.due_date,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

// === ARCHIVO: src/controllers/loan_controller.rs ===
use actix_web::{web, HttpResponse, Result, ResponseError};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::dto::loan_dto::{LoanRequest, LoanResponse, ApiResponse, PaginatedResponse};
use crate::error::loan_error::LoanError;
use crate::services::loan_service::LoanService;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateLoanRequest {
    #[validate(range(min = 0.01, message = "Amount must be positive"))]
    pub amount: f64,
    
    #[validate(range(min = 0.0, max = 100.0, message = "Interest rate must be between 0 and 100"))]
    pub interest_rate: f64,
    
    #[validate(regex(path = "*", message = "Due date must be in YYYY-MM-DD format"))]
    pub due_date: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateLoanRequest {
    #[validate(range(min = 0.01, message = "Amount must be positive"))]
    pub amount: Option<f64>,
    
    #[validate(range(min = 0.0, max = 100.0, message = "Interest rate must be between 0 and 100"))]
    pub interest_rate: Option<f64>,
    
    #[validate(regex(path = "*", message = "Due date must be in YYYY-MM-DD format"))]
    pub due_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoanPathParams {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: usize,
    
    #[serde(default = "default_per_page")]
    pub per_page: usize,
}

fn default_page() -> usize { 1 }
fn default_per_page() -> usize { 10 }

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/loans")
            .route("", web::get().to(list_loans))
            .route("", web::post().to(create_loan))
            .route("/{id}", web::get().to(get_loan))
            .route("/{id}", web::put().to(update_loan))
            .route("/{id}", web::delete().to(delete_loan))
            .route("/{id}/activate", web::post().to(activate_loan))
            .route("/{id}/deactivate", web::post().to(deactivate_loan))
            .route("/{id}/mark-paid", web::post().to(mark_as_paid))
            .route("/{id}/mark-defaulted", web::post().to(mark_as_defaulted))
    );
}

async fn list_loans(
    service: web::Data<LoanService>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, LoanError> {
    let page = query.page;
    let per_page = query.per_page;
    
    if per_page > 100 {
        return Err(LoanError::BadRequest("Per page cannot exceed 100".to_string()));
    }
    
    let result = service.list_loans(page, per_page).await?;
    Ok(HttpResponse::Ok().json(result))
}

async fn get_loan(
    service: web::Data<LoanService>,
    path: web::Path<LoanPathParams>,
) -> Result<HttpResponse, LoanError> {
    let loan = service.get_loan(path.id).await?;
    let response = LoanResponse::from_loan(&loan);
    Ok(HttpResponse::Ok().json(ApiResponse::new(response)))
}

async fn create_loan(
    service: web::Data<LoanService>,
    body: web::Json<CreateLoanRequest>,
) -> Result<HttpResponse, LoanError> {
    body.validate().map_err(|e| {
        LoanError::ValidationError(e.to_string())
    })?;
    
    let loan = service.create_loan(
        body.amount,
        body.interest_rate,
        &body.due_date,
    ).await?;
    
    let response = LoanResponse::from_loan(&loan);
    Ok(HttpResponse::Created().json(ApiResponse::new(response)))
}

async fn update_loan(
    service: web::Data<LoanService>,
    path: web::Path<LoanPathParams>,
    body: web::Json<UpdateLoanRequest>,
) -> Result<HttpResponse, LoanError> {
    body.validate().map_err(|e| {
        LoanError::ValidationError(e.to_string())
    })?;
    
    let loan = service.update_loan(
        path.id,
        body.amount,
        body.interest_rate,
        body.due_date.as_deref(),
    ).await?;
    
    let response = LoanResponse::from_loan(&loan);
    Ok(HttpResponse::Ok().json(ApiResponse::new(response)))
}

async fn delete_loan(
    service: web::Data<LoanService>,
    path: web::Path<LoanPathParams>,
) -> Result<HttpResponse, LoanError> {
    service.delete_loan(path.id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::deleted(path.id)))
}

async fn activate_loan(
    service: web::Data<LoanService>,
    path: web::Path<LoanPathParams>,
) -> Result<HttpResponse, LoanError> {
    let loan = service.activate_loan(path.id).await?;
    let response = LoanResponse::from_loan(&loan);
    Ok(HttpResponse::Ok().json(ApiResponse::with_message(response, "Loan activated successfully")))
}

async fn deactivate_loan(
    service: web::Data<LoanService>,
    path: web::Path<LoanPathParams>,
) -> Result<HttpResponse, LoanError> {
    let loan = service.deactivate_loan(path.id).await?;
    let response = LoanResponse::from_loan(&loan);
    Ok(HttpResponse::Ok().json(ApiResponse::with_message(response, "Loan deactivated successfully")))
}

async fn mark_as_paid(
    service: web::Data<LoanService>,
    path: web::Path<LoanPathParams>,
) -> Result<HttpResponse, LoanError> {
    let loan = service.mark_as_paid(path.id).await?;
    let response = LoanResponse::from_loan(&loan);
    Ok(HttpResponse::Ok().json(ApiResponse::with_message(response, "Loan marked as paid")))
}

async fn mark_as_defaulted(
    service: web::Data<LoanService>,
    path: web::Path<LoanPathParams>,
) -> Result<HttpResponse, LoanError> {
    let loan = service.mark_as_defaulted(path.id).await?;
    let response = LoanResponse::from_loan(&loan);
    Ok(HttpResponse::Ok().json(ApiResponse::with_message(response, "Loan marked as defaulted")))
}


// === ARCHIVO: src/error/loan_error.rs ===
use actix_web::{ResponseError, http::StatusCode};
use diesel::result::Error as DieselError;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoanError {
    #[error("El préstamo con ID {0} no fue encontrado")]
    NotFound(i64),
    
    #[error("Error de validación: {0}")]
    Validation(String),
    
    #[error("El monto debe ser mayor a cero")]
    InvalidAmount,
    
    #[error("La tasa de interés debe estar entre 0 y 100")]
    InvalidInterestRate,
    
    #[error("La fecha de vencimiento debe ser futura")]
    InvalidDueDate,
    
    #[error("Error de base de datos: {0}")]
    Database(String),
    
    #[error("Error interno del servidor: {0}")]
    Internal(String),
    
    #[error("El préstamo no puede ser modificado en su estado actual")]
    InvalidStateTransition,
    
    #[error("Conflicto de datos: {0}")]
    Conflict(String),
}

impl LoanError {
    pub fn not_found(id: i64) -> Self {
        LoanError::NotFound(id)
    }
    
    pub fn validation(message: impl Into<String>) -> Self {
        LoanError::Validation(message.into())
    }
    
    pub fn invalid_amount() -> Self {
        LoanError::InvalidAmount
    }
    
    pub fn invalid_interest_rate() -> Self {
        LoanError::InvalidInterestRate
    }
    
    pub fn invalid_due_date() -> Self {
        LoanError::InvalidDueDate
    }
    
    pub fn database(error: impl Into<String>) -> Self {
        LoanError::Database(error.into())
    }
    
    pub fn internal(error: impl Into<String>) -> Self {
        LoanError::Internal(error.into())
    }
    
    pub fn invalid_state_transition() -> Self {
        LoanError::InvalidStateTransition
    }
    
    pub fn conflict(message: impl Into<String>) -> Self {
        LoanError::Conflict(message.into())
    }
}

impl fmt::Display for LoanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl ResponseError for LoanError {
    fn status_code(&self) -> StatusCode {
        match self {
            LoanError::NotFound(_) => StatusCode::NOT_FOUND,
            LoanError::Validation(_) => StatusCode::BAD_REQUEST,
            LoanError::InvalidAmount => StatusCode::BAD_REQUEST,
            LoanError::InvalidInterestRate => StatusCode::BAD_REQUEST,
            LoanError::InvalidDueDate => StatusCode::BAD_REQUEST,
            LoanError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LoanError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LoanError::InvalidStateTransition => StatusCode::CONFLICT,
            LoanError::Conflict(_) => StatusCode::CONFLICT,
        }
    }
}

impl From<DieselError> for LoanError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => LoanError::NotFound(0),
            DieselError::DatabaseError(_, info) => {
                LoanError::Database(info.message().to_string())
            }
            _ => LoanError::Database(err.to_string()),
        }
    }
}

impl From<LoanError> for crate::dto::loan_dto::ErrorResponse {
    fn from(err: LoanError) -> Self {
        let message = err.to_string();
        let (code, error) = match &err {
            LoanError::NotFound(_) => ("NOT_FOUND", "Recurso no encontrado"),
            LoanError::Validation(_) => ("VALIDATION_ERROR", "Error de validación"),
            LoanError::InvalidAmount => ("INVALID_AMOUNT", "Monto inválido"),
            LoanError::InvalidInterestRate => ("INVALID_INTEREST_RATE", "Tasa de interés inválida"),
            LoanError::InvalidDueDate => ("INVALID_DUE_DATE", "Fecha de vencimiento inválida"),
            LoanError::Database(_) => ("DATABASE_ERROR", "Error de base de datos"),
            LoanError::Internal(_) => ("INTERNAL_ERROR", "Error interno"),
            LoanError::InvalidStateTransition => ("INVALID_STATE", "Transición de estado inválida"),
            LoanError::Conflict(_) => ("CONFLICT", "Conflicto de datos"),
        };
        crate::dto::loan_dto::ErrorResponse::new(code, error, &message)
    }
}

// === ARCHIVO: src/repositories/loan_repository.rs ===
use crate::models::loan::{Loan, LoanStatus};
use crate::error::loan_error::LoanError;
use diesel::{prelude::*, PgConnection, QueryResult};
use std::sync::Arc;
use tokio::sync::RwLock;

pub trait LoanRepository: Send + Sync {
    fn create(&self, loan: Loan) -> Result<Loan, LoanError>;
    fn find_by_id(&self, id: i64) -> Result<Loan, LoanError>;
    fn find_all(&self, page: usize, per_page: usize) -> Result<Vec<Loan>, LoanError>;
    fn update(&self, loan: Loan) -> Result<Loan, LoanError>;
    fn delete(&self, id: i64) -> Result<(), LoanError>;
    fn count(&self) -> Result<i64, LoanError>;
    fn find_by_status(&self, status: LoanStatus) -> Result<Vec<Loan>, LoanError>;
}

pub struct DieselLoanRepository {
    connection: Arc<RwLock<PgConnection>>,
}

impl DieselLoanRepository {
    pub fn new(connection: Arc<RwLock<PgConnection>>) -> Self {
        Self { connection }
    }
}

impl LoanRepository for DieselLoanRepository {
    fn create(&self, loan: Loan) -> Result<Loan, LoanError> {
        let conn = futures::executor::block_on(self.connection.try_write())
            .map_err(|e| LoanError::database(e.to_string()))?;
        
        use crate::schema::loans;
        
        let result = diesel::insert_into(loans::table)
            .values((
                loans::amount.eq(loan.amount),
                loans::interest_rate.eq(loan.interest_rate),
                loans::due_date.eq(loan.due_date),
                loans::status.eq(loan.as_str()),
            ))
            .get_result::<Loan>(&*conn)
            .map_err(LoanError::from)?;
        
        Ok(result)
    }
    
    fn find_by_id(&self, id: i64) -> Result<Loan, LoanError> {
        let conn = futures::executor::block_on(self.connection.try_write())
            .map_err(|e| LoanError::database(e.to_string()))?;
        
        use crate::schema::loans;
        
        let result = loans::table
            .filter(loans::id.eq(id))
            .first::<Loan>(&*conn)
            .map_err(LoanError::from)?;
        
        Ok(result)
    }
    
    fn find_all(&self, page: usize, per_page: usize) -> Result<Vec<Loan>, LoanError> {
        let conn = futures::executor::block_on(self.connection.try_write())
            .map_err(|e| LoanError::database(e.to_string()))?;
        
        use crate::schema::loans;
        
        let offset = (page.saturating_sub(1)) * per_page;
        
        let results = loans::table
            .order(loans::id.desc())
            .limit(per_page as i64)
            .offset(offset as i64)
            .load::<Loan>(&*conn)
            .map_err(LoanError::from)?;
        
        Ok(results)
    }
    
    fn update(&self, loan: Loan) -> Result<Loan, LoanError> {
        let conn = futures::executor::block_on(self.connection.try_write())
            .map_err(|e| LoanError::database(e.to_string()))?;
        
        use crate::schema::loans;
        
        let result = diesel::update(loans::table.find(loan.id))
            .set((
                loans::amount.eq(loan.amount),
                loans::interest_rate.eq(loan.interest_rate),
                loans::due_date.eq(loan.due_date),
                loans::status.eq(loan.as_str()),
            ))
            .get_result::<Loan>(&*conn)
            .map_err(LoanError::from)?;
        
        Ok(result)
    }
    
    fn delete(&self, id: i64) -> Result<(), LoanError> {
        let conn = futures::executor::block_on(self.connection.try_write())
            .map_err(|e| LoanError::database(e.to_string()))?;
        
        use crate::schema::loans;
        
        diesel::delete(loans::table.find(id))
            .execute(&*conn)
            .map_err(LoanError::from)?;
        
        Ok(())
    }
    
    fn count(&self) -> Result<i64, LoanError> {
        let conn = futures::executor::block_on(self.connection.try_write())
            .map_err(|e| LoanError::database(e.to_string()))?;
        
        use crate::schema::loans;
        
        let count: i64 = loans::table
            .count()
            .get_result(&*conn)
            .map_err(LoanError::from)?;
        
        Ok(count)
    }
    
    fn find_by_status(&self, status: LoanStatus) -> Result<Vec<Loan>, LoanError> {
        let conn = futures::executor::block_on(self.connection.try_write())
            .map_err(|e| LoanError::database(e.to_string()))?;
        
        use crate::schema::loans;
        
        let results = loans::table
            .filter(loans::status.eq(status.as_str()))
            .load::<Loan>(&*conn)
            .map_err(LoanError::from)?;
        
        Ok(results)
    }
}

// === ARCHIVO: src/services/loan_service.rs ===
use crate::models::loan::{Loan, LoanStatus};
use crate::dto::loan_dto::{LoanResponse, PaginatedLoansResponse, ApiResponse, ErrorResponse};
use crate::repositories::loan_repository::LoanRepository;
use crate::error::loan_error::LoanError;
use chrono::NaiveDate;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct LoanService<R: LoanRepository> {
    repository: Arc<R>,
}

impl<R: LoanRepository> LoanService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
    
    pub fn create_loan(
        &self,
        amount: f64,
        interest_rate: f64,
        due_date: NaiveDate,
    ) -> Result<ApiResponse<LoanResponse>, LoanError> {
        self.validate_loan_data(amount, interest_rate, due_date)?;
        
        let loan = Loan::new(0, amount, interest_rate, due_date);
        loan.validate().map_err(LoanError::validation)?;
        
        let created = self.repository.create(loan)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let response = LoanResponse::from_loan(&created);
        Ok(ApiResponse::new(response))
    }
    
    pub fn get_loan(&self, id: i64) -> Result<ApiResponse<LoanResponse>, LoanError> {
        let loan = self.repository.find_by_id(id)
            .map_err(|e| match e {
                LoanError::NotFound(_) => LoanError::not_found(id),
                _ => LoanError::internal(e.to_string()),
            })?;
        
        let response = LoanResponse::from_loan(&loan);
        Ok(ApiResponse::new(response))
    }
    
    pub fn list_loans(&self, page: usize, per_page: usize) -> Result<PaginatedLoansResponse, LoanError> {
        if page == 0 {
            return Err(LoanError::validation("El número de página debe ser mayor a 0"));
        }
        if per_page == 0 {
            return Err(LoanError::validation("Los elementos por página deben ser mayor a 0"));
        }
        
        let loans = self.repository.find_all(page, per_page)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let total = self.repository.count()
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let responses: Vec<LoanResponse> = loans.iter()
            .map(LoanResponse::from_loan)
            .collect();
        
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as usize;
        
        if responses.is_empty() {
            return Ok(PaginatedLoansResponse::empty(page, per_page));
        }
        
        Ok(PaginatedLoansResponse::new(responses, page, per_page))
    }
    
    pub fn update_loan(
        &self,
        id: i64,
        amount: Option<f64>,
        interest_rate: Option<f64>,
        due_date: Option<NaiveDate>,
    ) -> Result<ApiResponse<LoanResponse>, LoanError> {
        let mut loan = self.repository.find_by_id(id)
            .map_err(|e| match e {
                LoanError::NotFound(_) => LoanError::not_found(id),
                _ => LoanError::internal(e.to_string()),
            })?;
        
        if !loan.is_active() {
            return Err(LoanError::invalid_state_transition());
        }
        
        if let Some(new_amount) = amount {
            if new_amount <= 0.0 {
                return Err(LoanError::invalid_amount());
            }
            loan.update_amount(new_amount)
                .map_err(LoanError::validation)?;
        }
        
        if let Some(new_rate) = interest_rate {
            if new_rate < 0.0 || new_rate > 100.0 {
                return Err(LoanError::invalid_interest_rate());
            }
            loan.update_interest_rate(new_rate)
                .map_err(LoanError::validation)?;
        }
        
        if let Some(new_due_date) = due_date {
            let today = chrono::Local::now().date_naive();
            if new_due_date <= today {
                return Err(LoanError::invalid_due_date());
            }
            loan.update_due_date(new_due_date)
                .map_err(LoanError::validation)?;
        }
        
        loan.validate().map_err(LoanError::validation)?;
        
        let updated = self.repository.update(loan)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let response = LoanResponse::from_loan(&updated);
        Ok(ApiResponse::new(response))
    }
    
    pub fn delete_loan(&self, id: i64) -> Result<ApiResponse<()>, LoanError> {
        let loan = self.repository.find_by_id(id)
            .map_err(|e| match e {
                LoanError::NotFound(_) => LoanError::not_found(id),
                _ => LoanError::internal(e.to_string()),
            })?;
        
        if loan.is_active() {
            return Err(LoanError::conflict("No se puede eliminar un préstamo activo"));
        }
        
        self.repository.delete(id)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        Ok(ApiResponse::new(()))
    }
    
    pub fn activate_loan(&self, id: i64) -> Result<ApiResponse<LoanResponse>, LoanError> {
        let mut loan = self.repository.find_by_id(id)
            .map_err(|e| match e {
                LoanError::NotFound(_) => LoanError::not_found(id),
                _ => LoanError::internal(e.to_string()),
            })?;
        
        loan.activate();
        
        let updated = self.repository.update(loan)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let response = LoanResponse::from_loan(&updated);
        Ok(ApiResponse::new(response))
    }
    
    pub fn deactivate_loan(&self, id: i64) -> Result<ApiResponse<LoanResponse>, LoanError> {
        let mut loan = self.repository.find_by_id(id)
            .map_err(|e| match e {
                LoanError::NotFound(_) => LoanError::not_found(id),
                _ => LoanError::internal(e.to_string()),
            })?;
        
        loan.deactivate();
        
        let updated = self.repository.update(loan)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let response = LoanResponse::from_loan(&updated);
        Ok(ApiResponse::new(response))
    }
    
    pub fn mark_as_paid(&self, id: i64) -> Result<ApiResponse<LoanResponse>, LoanError> {
        let mut loan = self.repository.find_by_id(id)
            .map_err(|e| match e {
                LoanError::NotFound(_) => LoanError::not_found(id),
                _ => LoanError::internal(e.to_string()),
            })?;
        
        if !loan.is_active() {
            return Err(LoanError::invalid_state_transition());
        }
        
        loan.mark_as_paid();
        
        let updated = self.repository.update(loan)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let response = LoanResponse::from_loan(&updated);
        Ok(ApiResponse::new(response))
    }
    
    pub fn mark_as_defaulted(&self, id: i64) -> Result<ApiResponse<LoanResponse>, LoanError> {
        let mut loan = self.repository.find_by_id(id)
            .map_err(|e| match e {
                LoanError::NotFound(_) => LoanError::not_found(id),
                _ => LoanError::internal(e.to_string()),
            })?;
        
        if !loan.is_active() {
            return Err(LoanError::invalid_state_transition());
        }
        
        loan.mark_as_defaulted();
        
        let updated = self.repository.update(loan)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let response = LoanResponse::from_loan(&updated);
        Ok(ApiResponse::new(response))
    }
    
    fn validate_loan_data(
        &self,
        amount: f64,
        interest_rate: f64,
        due_date: NaiveDate,
    ) -> Result<(), LoanError> {
        if amount <= 0.0 {
            return Err(LoanError::invalid_amount());
        }
        
        if interest_rate < 0.0 || interest_rate > 100.0 {
            return Err(LoanError::invalid_interest_rate());
        }
        
        let today = chrono::Local::now().date_naive();
        if due_date <= today {
            return Err(LoanError::invalid_due_date());
        }
        
        Ok(())
    }
}


// === ARCHIVO: tests/loan_controller_tests.rs ===
use actix_web::{test, web, App, http::{StatusCode, Method}};
use serde_json::json;

mod integration_tests {
    use super::*;

    #[actix_rt::test]
    async fn test_create_loan_success() {
        let app = test::init_service(
            App::new()
                .route("/api/loans", web::post().to(crate::controllers::loan_controller::create_loan))
        ).await;

        let payload = json!({
            "amount": 10000.0,
            "interest_rate": 5.5,
            "due_date": "2024-12-31"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_rt::test]
    async fn test_create_loan_invalid_amount() {
        let app = test::init_service(
            App::new()
                .route("/api/loans", web::post().to(crate::controllers::loan_controller::create_loan))
        ).await;

        let payload = json!({
            "amount": -1000.0,
            "interest_rate": 5.5,
            "due_date": "2024-12-31"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_create_loan_invalid_interest_rate() {
        let app = test::init_service(
            App::new()
                .route("/api/loans", web::post().to(crate::controllers::loan_controller::create_loan))
        ).await;

        let payload = json!({
            "amount": 10000.0,
            "interest_rate": -5.0,
            "due_date": "2024-12-31"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_create_loan_invalid_date_format() {
        let app = test::init_service(
            App::new()
                .route("/api/loans", web::post().to(crate::controllers::loan_controller::create_loan))
        ).await;

        let payload = json!({
            "amount": 10000.0,
            "interest_rate": 5.5,
            "due_date": "invalid-date"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_get_loan_by_id_success() {
        let app = test::init_service(
            App::new()
                .route("/api/loans/{id}", web::get().to(crate::controllers::loan_controller::get_loan))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/loans/1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_loan_by_id_not_found() {
        let app = test::init_service(
            App::new()
                .route("/api/loans/{id}", web::get().to(crate::controllers::loan_controller::get_loan))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/loans/99999")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_list_loans_pagination() {
        let app = test::init_service(
            App::new()
                .route("/api/loans", web::get().to(crate::controllers::loan_controller::list_loans))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/loans?page=1&per_page=10")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_list_loans_default_pagination() {
        let app = test::init_service(
            App::new()
                .route("/api/loans", web::get().to(crate::controllers::loan_controller::list_loans))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/loans")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_update_loan_success() {
        let app = test::init_service(
            App::new()
                .route("/api/loans/{id}", web::put().to(crate::controllers::loan_controller::update_loan))
        ).await;

        let payload = json!({
            "amount": 15000.0,
            "interest_rate": 6.0,
            "due_date": "2025-06-30"
        });

        let req = test::TestRequest::put()
            .uri("/api/loans/1")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_update_loan_not_found() {
        let app = test::init_service(
            App::new()
                .route("/api/loans/{id}", web::put().to(crate::controllers::loan_controller::update_loan))
        ).await;

        let payload = json!({
            "amount": 15000.0,
            "interest_rate": 6.0,
            "due_date": "2025-06-30"
        });

        let req = test::TestRequest::put()
            .uri("/api/loans/99999")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_delete_loan_success() {
        let app = test::init_service(
            App::new()
                .route("/api/loans/{id}", web::delete().to(crate::controllers::loan_controller::delete_loan))
        ).await;

        let req = test::TestRequest::delete()
            .uri("/api/loans/1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }

    #[actix_rt::test]
    async fn test_delete_loan_not_found() {
        let app = test::init_service(
            App::new()
                .route("/api/loans/{id}", web::delete().to(crate::controllers::loan_controller::delete_loan))
        ).await;

        let req = test::TestRequest::delete()
            .uri("/api/loans/99999")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}

mod unit_tests {
    use super::*;
    use crate::models::loan::{Loan, LoanStatus};
    use crate::dto::loan_dto::{LoanRequest, LoanResponse};
    use chrono::NaiveDate;

    #[test]
    fn test_loan_creation() {
        let loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        assert_eq!(loan.is_active(), true);
    }

    #[test]
    fn test_loan_calculation_total_interest() {
        let loan = Loan::new(1, 10000.0, 10.0, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let interest = loan.calculate_total_interest();
        assert!(interest > 0.0);
    }

    #[test]
    fn test_loan_calculation_total_amount() {
        let loan = Loan::new(1, 10000.0, 10.0, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let total = loan.calculate_total_amount();
        assert!(total > 10000.0);
    }

    #[test]
    fn test_loan_update_amount() {
        let mut loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let result = loan.update_amount(15000.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_loan_update_invalid_amount() {
        let mut loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let result = loan.update_amount(-1000.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_loan_update_interest_rate() {
        let mut loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let result = loan.update_interest_rate(7.5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_loan_update_invalid_interest_rate() {
        let mut loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let result = loan.update_interest_rate(-2.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_loan_mark_as_paid() {
        let mut loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        loan.mark_as_paid();
        assert!(!loan.is_active());
    }

    #[test]
    fn test_loan_mark_as_defaulted() {
        let mut loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        loan.mark_as_defaulted();
        assert!(!loan.is_active());
    }

    #[test]
    fn test_loan_validate_valid() {
        let loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let result = loan.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_loan_validate_invalid_amount() {
        let loan = Loan::new(1, -1000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let result = loan.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_loan_validate_invalid_interest_rate() {
        let loan = Loan::new(1, 10000.0, -5.0, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let result = loan.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_loan_dto_from_loan() {
        let loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let dto = LoanResponse::from_loan(&loan);
        assert_eq!(dto.id, 1);
        assert_eq!(dto.amount, 10000.0);
    }

    #[test]
    fn test_loan_dto_from_loan_without_calculations() {
        let loan = Loan::new(1, 10000.0, 5.5, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        let dto = LoanResponse::from_loan_without_calculations(&loan);
        assert_eq!(dto.id, 1);
        assert_eq!(dto.amount, 10000.0);
    }
}

mod error_handling_tests {
    use super::*;
    use crate::error::loan_error::LoanError;

    #[test]
    fn test_loan_error_not_found() {
        let error = LoanError::NotFound(1);
        assert!(matches!(error, LoanError::NotFound(_)));
    }

    #[test]
    fn test_loan_error_validation() {
        let error = LoanError::Validation("Invalid amount".to_string());
        assert!(matches!(error, LoanError::Validation(_)));
    }

    #[test]
    fn test_loan_error_internal() {
        let error = LoanError::Internal("Database error".to_string());
        assert!(matches!(error, LoanError::Internal(_)));
    }
}

mod pagination_tests {
    use super::*;
    use crate::dto::loan_dto::PaginationMetadata;

    #[test]
    fn test_pagination_metadata_first_page() {
        let metadata = PaginationMetadata::new(100, 1, 10);
        assert_eq!(metadata.get_page(), 1);
        assert_eq!(metadata.get_per_page(), 10);
        assert_eq!(metadata.get_offset(), 0);
    }

    #[test]
    fn test_pagination_metadata_second_page() {
        let metadata = PaginationMetadata::new(100, 2, 10);
        assert_eq!(metadata.get_page(), 2);
        assert_eq!(metadata.get_offset(), 10);
    }

    #[test]
    fn test_pagination_metadata_empty() {
        let metadata = PaginationMetadata::empty(1, 10);
        assert_eq!(metadata.get_page(), 1);
        assert_eq!(metadata.get_per_page(), 10);
    }
}

mod validation_tests {
    use super::*;
    use validator::Validate;
    use crate::dto::loan_dto::LoanRequest;
    use chrono::NaiveDate;

    #[test]
    fn test_loan_request_validate_positive_amount() {
        let req = LoanRequest {
            amount: 1000.0,
            interest_rate: 5.0,
            due_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_loan_request_validate_negative_amount() {
        let req = LoanRequest {
            amount: -1000.0,
            interest_rate: 5.0,
            due_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_loan_request_validate_negative_interest_rate() {
        let req = LoanRequest {
            amount: 1000.0,
            interest_rate: -5.0,
            due_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_loan_request_validate_zero_interest_rate() {
        let req = LoanRequest {
            amount: 1000.0,
            interest_rate: 0.0,
            due_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        };
        assert!(req.validate().is_ok());
    }
}

```
