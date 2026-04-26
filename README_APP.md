# 🏥 Sistema de Pacientes — Solana + Anchor + PDA

Programa descentralizado desarrollado en **Solana** usando el framework **Anchor**,
que implementa un sistema de registro de pacientes directamente en la blockchain.
Los datos se almacenan en cuentas derivadas del programa (PDA), sin servidores ni
bases de datos centralizadas.

---

## ✨ Características

- ✅ **CRUD completo** — Crear, Leer, Actualizar y Eliminar pacientes
- ✅ **PDA (Program Derived Address)** — Cada cuenta se deriva de la wallet del usuario
- ✅ **Validaciones** — Errores personalizados con mensajes descriptivos
- ✅ **Tests completos** — 7 tests que cubren todas las operaciones
- ✅ **Desplegado en Devnet** — Red pública de pruebas de Solana

---

## 🏗️ Tecnologías

| Tecnología | Versión | Uso |
|---|---|---|
| Solana | 1.18+ | Blockchain |
| Anchor | 0.29+ | Framework para programas Solana |
| Rust | 1.70+ | Lenguaje del programa |
| TypeScript | 4.3+ | Tests del cliente |

---

## 📁 Estructura del proyecto

```
pacientes_app/
├── src/
│   └── lib.rs                  ← Programa principal en Rust
├── tests/
│   └── pacientes_app.test.ts   ← Tests completos en TypeScript
└── README.md
```

---

## 🔑 ¿Qué es una PDA?

Una **Program Derived Address** es una dirección de cuenta generada
automáticamente a partir de semillas predefinidas, sin necesidad de un keypair.

```
seeds = ["paciente" + wallet_del_usuario]
         ─────────────────────────────────
                      ↓
              Dirección única y predecible
              para cada usuario del sistema
```

**Ventajas sobre un Keypair random:**
- No necesitas guardar ninguna clave privada
- La dirección siempre es recuperable si conoces la wallet
- Solo el dueño original puede modificar o eliminar su cuenta

---

## 📖 Instrucciones del programa

### `crear_paciente`
Crea un nuevo registro de paciente en la blockchain.

```typescript
await program.methods
  .crearPaciente("María García", 35, "Hipertensión leve")
  .accounts({
    paciente:      pdaPaciente,
    autoridad:     wallet.publicKey,
    systemProgram: web3.SystemProgram.programId,
  })
  .rpc();
```

### `actualizar_paciente`
Modifica los datos de un paciente existente. Solo el creador original puede hacerlo.

```typescript
await program.methods
  .actualizarPaciente("María García López", 36, "Hipertensión controlada")
  .accounts({
    paciente:  pdaPaciente,
    autoridad: wallet.publicKey,
  })
  .rpc();
```

### `eliminar_paciente`
Elimina la cuenta del paciente y devuelve los lamports de renta al dueño.

```typescript
await program.methods
  .eliminarPaciente()
  .accounts({
    paciente:  pdaPaciente,
    autoridad: wallet.publicKey,
  })
  .rpc();
```

### Leer datos (READ)
La lectura se hace desde el cliente, sin instrucción en el programa.

```typescript
// Leer un paciente específico
const datos = await program.account.paciente.fetch(pdaPaciente);
console.log(datos.nombre, datos.edad, datos.diagnostico);

// Leer todos los pacientes del programa
const todos = await program.account.paciente.all();
```

---

## 🧮 Estructura de datos

```rust
pub struct Paciente {
    pub nombre:      String,  // máx 50 caracteres
    pub edad:        u8,      // 1 - 255
    pub diagnostico: String,  // máx 200 caracteres
    pub autoridad:   Pubkey,  // wallet que creó el registro
    pub bump:        u8,      // bump de la PDA
}
```

**Espacio reservado en la blockchain: 300 bytes**

```
 8 bytes  → discriminador de Anchor
54 bytes  → nombre  (4 longitud + 50 contenido)
 1 byte   → edad
204 bytes → diagnostico (4 longitud + 200 contenido)
32 bytes  → autoridad (Pubkey)
 1 byte   → bump
```

---

## 🚀 Cómo usar en Solana Playground

### 1. Abrir Solana Playground
Ve a [https://beta.solpg.io](https://beta.solpg.io) y conecta tu wallet en **devnet**.

### 2. Pegar el código
- Copia el contenido de `src/lib.rs` en el editor principal
- Copia el contenido de `tests/pacientes_app.test.ts` en el archivo de tests

### 3. Build
Haz clic en el botón **Build** (ícono de martillo).
Espera el mensaje: `Build successful.`

### 4. Deploy
Haz clic en **Deploy** (ícono de cohete 🚀).
Espera: `Deployment successful.`

### 5. Correr los tests
En la terminal integrada escribe:
```bash
test
```

**Resultado esperado:**
```
pacientes_app — CRUD completo con PDA
  ✅ CREATE — Crea un paciente con PDA
  ✅ READ — Lee los datos del paciente desde la blockchain
  ✅ UPDATE — Actualiza los datos del paciente
  ✅ VALIDACIÓN — Rechaza nombre vacío
  ✅ VALIDACIÓN — Rechaza nombre mayor a 50 caracteres
  ✅ LIST — Lista todos los pacientes del programa
  ✅ DELETE — Elimina el paciente y recupera la renta

7 passing
```

---

## ❌ Errores comunes

| Error | Causa | Solución |
|---|---|---|
| `Build failed` | Typo en el código Rust | Revisa que copiaste todo el archivo |
| `insufficient funds` | No tienes SOL de prueba | Pide airdrop en la sección Wallet de Playground |
| `already in use` | La cuenta PDA ya existe | Elimina primero con `eliminarPaciente` |
| `has_one constraint` | Intentas modificar la cuenta de otro usuario | Solo el creador puede actualizar/eliminar |
| `Account does not exist` | La cuenta PDA no fue creada aún | Crea primero con `crearPaciente` |

---

## 📝 Validaciones implementadas

| Campo | Regla |
|---|---|
| `nombre` | No vacío, máximo 50 caracteres |
| `edad` | Mayor a 0 |
| `diagnostico` | No vacío, máximo 200 caracteres |
| `autoridad` | Solo el creador puede modificar (`has_one`) |

---

## 🌐 Deployment

- **Red:** Solana Devnet
- **Program ID:** `Ga51nvVsc7RCAWQLAfyDohJ5duLoPTPhfpfncThRoGCN`
- **Framework:** Anchor 0.29
- **IDE:** Solana Playground ([beta.solpg.io](https://beta.solpg.io))

---

## 📄 Licencia

MIT — libre para usar, modificar y distribuir.
