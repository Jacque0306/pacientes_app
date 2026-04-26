use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111"); // Playground lo actualiza automático

// ═══════════════════════════════════════════════════════════════════
//  SISTEMA DE PACIENTES — Solana + Anchor + PDA
//
//  ¿Qué es una PDA (Program Derived Address)?
//  Es una dirección de cuenta que se genera automáticamente a partir
//  de "semillas" (seeds). No necesita un keypair separado.
//  La dirección se deriva de: [b"paciente", wallet_del_usuario]
//  Esto significa que cada wallet solo puede tener UN paciente,
//  y la dirección siempre es predecible y verificable.
//
//  Instrucciones disponibles:
//  1. crear_paciente   → Create
//  2. actualizar_paciente → Update
//  3. eliminar_paciente  → Delete
//  (Read se hace desde el cliente con .fetch())
// ═══════════════════════════════════════════════════════════════════

#[program]
pub mod pacientes_app {
    use super::*;

    // ─────────────────────────────────────────────
    //  CREATE — Crear un nuevo paciente
    // ─────────────────────────────────────────────
    pub fn crear_paciente(
        ctx: Context<CrearPaciente>,
        nombre: String,
        edad: u8,
        diagnostico: String,
    ) -> Result<()> {
        // Validaciones
        require!(!nombre.is_empty(), ErrorCode::NombreVacio);
        require!(nombre.len() <= 50, ErrorCode::NombreDemasiadoLargo);
        require!(!diagnostico.is_empty(), ErrorCode::DiagnosticoVacio);
        require!(
            diagnostico.len() <= 200,
            ErrorCode::DiagnosticoDemasiadoLargo
        );
        require!(edad > 0, ErrorCode::EdadInvalida);

        let paciente = &mut ctx.accounts.paciente;
        paciente.nombre = nombre;
        paciente.edad = edad;
        paciente.diagnostico = diagnostico;
        paciente.autoridad = ctx.accounts.autoridad.key();
        paciente.bump = ctx.bumps.paciente; // guardamos el bump de la PDA

        msg!("✅ Paciente creado: {}", paciente.nombre);
        Ok(())
    }

    // ─────────────────────────────────────────────
    //  UPDATE — Actualizar datos del paciente
    // ─────────────────────────────────────────────
    pub fn actualizar_paciente(
        ctx: Context<ActualizarPaciente>,
        nombre: String,
        edad: u8,
        diagnostico: String,
    ) -> Result<()> {
        // Validaciones
        require!(!nombre.is_empty(), ErrorCode::NombreVacio);
        require!(nombre.len() <= 50, ErrorCode::NombreDemasiadoLargo);
        require!(!diagnostico.is_empty(), ErrorCode::DiagnosticoVacio);
        require!(
            diagnostico.len() <= 200,
            ErrorCode::DiagnosticoDemasiadoLargo
        );
        require!(edad > 0, ErrorCode::EdadInvalida);

        let paciente = &mut ctx.accounts.paciente;
        paciente.nombre = nombre;
        paciente.edad = edad;
        paciente.diagnostico = diagnostico;

        msg!("✏️ Paciente actualizado: {}", paciente.nombre);
        Ok(())
    }

    // ─────────────────────────────────────────────
    //  DELETE — Eliminar paciente y recuperar renta
    // ─────────────────────────────────────────────
    pub fn eliminar_paciente(ctx: Context<EliminarPaciente>) -> Result<()> {
        msg!(
            "🗑️ Paciente eliminado: {}",
            ctx.accounts.paciente.nombre
        );
        Ok(())
        // Anchor cierra la cuenta automáticamente con `close = autoridad`
        // y devuelve los lamports (renta) a la wallet del usuario
    }
}

// ═══════════════════════════════════════════════════════════════════
//  CONTEXTOS DE CUENTAS
//  Cada instrucción declara qué cuentas necesita y cómo las usa
// ═══════════════════════════════════════════════════════════════════

#[derive(Accounts)]
pub struct CrearPaciente<'info> {
    #[account(
        init,                          // crea la cuenta nueva
        payer = autoridad,             // quien paga la renta en SOL
        space = Paciente::TAMANIO,     // bytes reservados (calculado abajo)
        seeds = [b"paciente", autoridad.key().as_ref()], // ← PDA: semillas
        bump                           // Anchor calcula el bump automáticamente
    )]
    pub paciente: Account<'info, Paciente>,

    #[account(mut)]
    pub autoridad: Signer<'info>, // tu wallet (firma la transacción)

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarPaciente<'info> {
    #[account(
        mut,                           // la cuenta se va a modificar
        seeds = [b"paciente", autoridad.key().as_ref()], // mismas semillas
        bump = paciente.bump,          // usamos el bump guardado al crear
        has_one = autoridad            // verifica que sea el dueño original
    )]
    pub paciente: Account<'info, Paciente>,

    pub autoridad: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarPaciente<'info> {
    #[account(
        mut,
        seeds = [b"paciente", autoridad.key().as_ref()],
        bump = paciente.bump,
        has_one = autoridad,
        close = autoridad              // cierra la cuenta y devuelve lamports
    )]
    pub paciente: Account<'info, Paciente>,

    #[account(mut)]
    pub autoridad: Signer<'info>,
}

// ═══════════════════════════════════════════════════════════════════
//  ESTRUCTURA DE DATOS
//  Define qué información se guarda en cada cuenta de paciente
// ═══════════════════════════════════════════════════════════════════

#[account]
pub struct Paciente {
    pub nombre: String,      // máx 50 caracteres
    pub edad: u8,            // 1 - 255
    pub diagnostico: String, // máx 200 caracteres
    pub autoridad: Pubkey,   // wallet que creó el registro
    pub bump: u8,            // bump de la PDA (necesario para verificar)
}

impl Paciente {
    // Cálculo exacto del espacio en bytes que ocupa la cuenta:
    //
    //   8  → discriminador interno de Anchor (siempre fijo)
    //   4  → prefijo de longitud del String "nombre"
    //  50  → contenido máximo de "nombre"
    //   1  → "edad" es u8 (1 byte)
    //   4  → prefijo de longitud del String "diagnostico"
    // 200  → contenido máximo de "diagnostico"
    //  32  → "autoridad" es un Pubkey (siempre 32 bytes)
    //   1  → "bump" es u8 (1 byte)
    // ────
    // 300  bytes en total
    pub const TAMANIO: usize = 8 + (4 + 50) + 1 + (4 + 200) + 32 + 1;
}

// ═══════════════════════════════════════════════════════════════════
//  ERRORES PERSONALIZADOS
// ═══════════════════════════════════════════════════════════════════

#[error_code]
pub enum ErrorCode {
    #[msg("El nombre no puede estar vacío")]
    NombreVacio,
    #[msg("El nombre no puede tener más de 50 caracteres")]
    NombreDemasiadoLargo,
    #[msg("El diagnóstico no puede estar vacío")]
    DiagnosticoVacio,
    #[msg("El diagnóstico no puede tener más de 200 caracteres")]
    DiagnosticoDemasiadoLargo,
    #[msg("La edad debe ser mayor a 0")]
    EdadInvalida,
}
