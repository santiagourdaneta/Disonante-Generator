use assert_cmd::Command;
use std::fs;

#[test]
fn test_random_mode_produces_file() -> Result<(), Box<dyn std::error::Error>> {
    let output_file = "test_random.mid";

    // 1. Simula la ejecución del programa en modo aleatorio
    Command::cargo_bin("disonante_generator")?
        .args(&["--mode", "random", "--output", output_file])
        .assert()
        .success()
        .stdout(predicates::str::contains("¡Generación completa!"));

    // 2. Verifica que el archivo de salida existe y no está vacío
    let metadata = fs::metadata(output_file)?;
    assert!(metadata.is_file());
    assert!(metadata.len() > 0);

    // 3. Limpia el archivo generado
    fs::remove_file(output_file)?;
    Ok(())
}

#[test]
fn test_markov_mode_produces_file() -> Result<(), Box<dyn std::error::Error>> {
    let output_file = "test_markov.mid";

    // 1. Simula la ejecución del programa en modo Markov
    Command::cargo_bin("disonante_generator")?
        .args(&["--mode", "markov", "--output", output_file])
        .assert()
        .success()
        .stdout(predicates::str::contains("¡Generación completa!"));

    // 2. Verifica que el archivo de salida existe y no está vacío
    let metadata = fs::metadata(output_file)?;
    assert!(metadata.is_file());
    assert!(metadata.len() > 0);
    
    // 3. Limpia el archivo generado
    fs::remove_file(output_file)?;
    Ok(())
}

#[test]
fn test_invalid_mode_shows_error() -> Result<(), Box<dyn std::error::Error>> {
    // Simula la ejecución con un modo inválido
    Command::cargo_bin("disonante_generator")?
        .args(&["--mode", "invalido"])
        .assert()
        .success() // Aún debe terminar exitosamente, pero con un mensaje de error
        .stderr(predicates::str::contains("Error: El modo debe ser 'random' o 'markov'"));

    Ok(())
}