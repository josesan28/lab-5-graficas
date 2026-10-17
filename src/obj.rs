use std::error::Error;
use std::fs;
use std::io::{Error as IoError, ErrorKind};

use raylib::prelude::Vector3;

pub struct Obj {
    pub vertices: Vec<Vector3>,
    pub indices: Vec<usize>,
}

pub fn load_obj(path: &str) -> Result<Obj, Box<dyn Error>> {
    let contenido = fs::read_to_string(path)?;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for (numero_linea, linea) in contenido.lines().enumerate() {
        let mut partes = linea.split_whitespace();

        match partes.next() {
            Some("v") => {
                let x = leer_componente(partes.next(), numero_linea, "x")?;
                let y = leer_componente(partes.next(), numero_linea, "y")?;
                let z = leer_componente(partes.next(), numero_linea, "z")?;

                vertices.push(Vector3::new(x, y, z));
            }
            Some("f") => {
                for _ in 0..3 {
                    let grupo = partes.next().ok_or_else(|| {
                        error_de_linea(numero_linea, "la cara no contiene tres índices")
                    })?;
                    let indice_obj = grupo
                        .split('/')
                        .next()
                        .ok_or_else(|| error_de_linea(numero_linea, "índice vacío"))?
                        .parse::<usize>()?;
                    let indice_rust = indice_obj.checked_sub(1).ok_or_else(|| {
                        error_de_linea(numero_linea, "los índices OBJ comienzan en 1")
                    })?;

                    indices.push(indice_rust);
                }

                if partes.next().is_some() {
                    return Err(error_de_linea(
                        numero_linea,
                        "se encontró una cara que no está triangulada",
                    )
                    .into());
                }
            }
            _ => {}
        }
    }

    if let Some(indice) = indices.iter().find(|&&indice| indice >= vertices.len()) {
        return Err(IoError::new(
            ErrorKind::InvalidData,
            format!("el índice {} no corresponde a ningún vértice", indice + 1),
        )
        .into());
    }

    Ok(Obj { vertices, indices })
}

fn leer_componente(
    valor: Option<&str>,
    numero_linea: usize,
    componente: &str,
) -> Result<f32, Box<dyn Error>> {
    valor
        .ok_or_else(|| {
            error_de_linea(
                numero_linea,
                &format!("falta la componente {componente} del vértice"),
            )
        })?
        .parse::<f32>()
        .map_err(Into::into)
}

fn error_de_linea(numero_linea: usize, mensaje: &str) -> IoError {
    IoError::new(
        ErrorKind::InvalidData,
        format!("línea {}: {mensaje}", numero_linea + 1),
    )
}
