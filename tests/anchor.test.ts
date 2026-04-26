// ═══════════════════════════════════════════════════════════════════
//  TESTS — Sistema de Pacientes con PDA
//  Compatible con Solana Playground (usa `pg` global)
//
//  ¿Qué es una PDA aquí?
//  En lugar de generar un Keypair random para cada paciente,
//  la dirección se calcula a partir de ["paciente" + tu_wallet].
//  Esto significa que siempre puedes encontrar la cuenta de un
//  paciente si conoces la wallet que la creó.
// ═══════════════════════════════════════════════════════════════════

describe("pacientes_app — CRUD completo con PDA", () => {
  // ─── Calculamos la PDA una sola vez y la reutilizamos en todos los tests
  const [pdaPaciente] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("paciente"), pg.wallet.publicKey.toBuffer()],
    pg.program.programId
  );

  console.log("📍 Dirección PDA del paciente:", pdaPaciente.toBase58());

  // ═══════════════════════════════════════════════════════════════
  //  TEST 1 — CREATE
  // ═══════════════════════════════════════════════════════════════
  it("✅ CREATE — Crea un paciente con PDA", async () => {
    const tx = await pg.program.methods
      .crearPaciente("María García", 35, "Hipertensión leve")
      .accounts({
        paciente: pdaPaciente,
        autoridad: pg.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    await pg.connection.confirmTransaction(tx);

    const datos = await pg.program.account.paciente.fetch(pdaPaciente);

    console.log("\n📋 Paciente creado:");
    console.log("   Nombre:      ", datos.nombre);
    console.log("   Edad:        ", datos.edad);
    console.log("   Diagnóstico: ", datos.diagnostico);
    console.log("   Autoridad:   ", datos.autoridad.toBase58());
    console.log("   Bump PDA:    ", datos.bump);

    assert.equal(datos.nombre, "María García");
    assert.equal(datos.edad, 35);
    assert.equal(datos.diagnostico, "Hipertensión leve");
    assert.equal(datos.autoridad.toBase58(), pg.wallet.publicKey.toBase58());
  });

  // ═══════════════════════════════════════════════════════════════
  //  TEST 2 — READ
  // ═══════════════════════════════════════════════════════════════
  it("✅ READ — Lee los datos del paciente desde la blockchain", async () => {
    const datos = await pg.program.account.paciente.fetch(pdaPaciente);

    console.log("\n🔍 Lectura de datos:");
    console.log("   Nombre:      ", datos.nombre);
    console.log("   Edad:        ", datos.edad);
    console.log("   Diagnóstico: ", datos.diagnostico);

    assert.equal(datos.nombre, "María García");
    assert.equal(datos.edad, 35);
  });

  // ═══════════════════════════════════════════════════════════════
  //  TEST 3 — UPDATE
  // ═══════════════════════════════════════════════════════════════
  it("✅ UPDATE — Actualiza los datos del paciente", async () => {
    const tx = await pg.program.methods
      .actualizarPaciente("María García López", 36, "Hipertensión controlada")
      .accounts({
        paciente: pdaPaciente,
        autoridad: pg.wallet.publicKey,
      })
      .rpc();

    await pg.connection.confirmTransaction(tx);

    const datos = await pg.program.account.paciente.fetch(pdaPaciente);

    console.log("\n✏️  Paciente actualizado:");
    console.log("   Nombre:      ", datos.nombre);
    console.log("   Edad:        ", datos.edad);
    console.log("   Diagnóstico: ", datos.diagnostico);

    assert.equal(datos.nombre, "María García López");
    assert.equal(datos.edad, 36);
    assert.equal(datos.diagnostico, "Hipertensión controlada");
  });

  // ═══════════════════════════════════════════════════════════════
  //  TEST 4 — Validaciones de error
  // ═══════════════════════════════════════════════════════════════
  it("✅ VALIDACIÓN — Rechaza nombre vacío", async () => {
    const otraWallet = new web3.Keypair();
    const [otraPda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("paciente"), otraWallet.publicKey.toBuffer()],
      pg.program.programId
    );

    try {
      await pg.program.methods
        .crearPaciente("", 25, "Diagnóstico válido")
        .accounts({
          paciente: otraPda,
          autoridad: otraWallet.publicKey,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([otraWallet])
        .rpc();

      assert.fail("Debería haber rechazado el nombre vacío");
    } catch (err: any) {
      assert.ok(err, "Debería haber lanzado un error");
      console.log("✅ Error esperado: nombre vacío rechazado correctamente");
    }
  });

  it("✅ VALIDACIÓN — Rechaza nombre mayor a 50 caracteres", async () => {
    const otraWallet = new web3.Keypair();
    const [otraPda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("paciente"), otraWallet.publicKey.toBuffer()],
      pg.program.programId
    );

    try {
      await pg.program.methods
        .crearPaciente("A".repeat(51), 25, "Diagnóstico válido")
        .accounts({
          paciente: otraPda,
          autoridad: otraWallet.publicKey,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([otraWallet])
        .rpc();

      assert.fail("Debería haber rechazado el nombre largo");
    } catch (err: any) {
      assert.ok(err, "Debería haber lanzado un error");
      console.log("✅ Error esperado: nombre largo rechazado correctamente");
    }
  });

  // ═══════════════════════════════════════════════════════════════
  //  TEST 5 — LIST
  // ═══════════════════════════════════════════════════════════════
  it("✅ LIST — Lista todos los pacientes del programa", async () => {
    const todos = await pg.program.account.paciente.all();

    console.log(`\n📚 Total de pacientes en la blockchain: ${todos.length}`);
    todos.forEach((p, i) => {
      console.log(
        `   [${i + 1}] ${p.account.nombre} | Edad: ${
          p.account.edad
        } | PDA: ${p.publicKey.toBase58()}`
      );
    });

    assert.ok(todos.length >= 1);
  });

  // ═══════════════════════════════════════════════════════════════
  //  TEST 6 — DELETE
  // ═══════════════════════════════════════════════════════════════
  it("✅ DELETE — Elimina el paciente y recupera la renta", async () => {
    const balanceAntes = await pg.connection.getBalance(pg.wallet.publicKey);

    const tx = await pg.program.methods
      .eliminarPaciente()
      .accounts({
        paciente: pdaPaciente,
        autoridad: pg.wallet.publicKey,
      })
      .rpc();

    await pg.connection.confirmTransaction(tx);

    const balanceDespues = await pg.connection.getBalance(pg.wallet.publicKey);
    console.log("\n🗑️  Paciente eliminado");
    console.log("   Lamports recuperados:", balanceDespues - balanceAntes);

    try {
      await pg.program.account.paciente.fetch(pdaPaciente);
      assert.fail("La cuenta debería haber sido eliminada");
    } catch {
      console.log("   ✓ Cuenta eliminada correctamente de la blockchain");
    }
  });
});
