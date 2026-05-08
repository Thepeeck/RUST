use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Incidente {
    id: u32,
    prioridad: u8,
    descripcion: String,
}

struct Nodo {
    incidente: Incidente,
    izq: Option<Box<Nodo>>,
    der: Option<Box<Nodo>>,
}

pub struct CentroEmergencias {
    directorio: HashMap<u32, Incidente>,
    raiz: Option<Box<Nodo>>,
}

impl CentroEmergencias {
    pub fn nuevo() -> Self {
        Self {
            directorio: HashMap::new(),
            raiz: None,
        }
    }

    // 🔹 Rotación derecha
    fn rotar_derecha(mut x: Box<Nodo>) -> Box<Nodo> {
        let mut y = x.izq.take().unwrap();
        x.izq = y.der.take();
        y.der = Some(x);
        y
    }

    // 🔹 Rotación izquierda
    fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
        let mut y = x.der.take().unwrap();
        x.der = y.izq.take();
        y.izq = Some(x);
        y
    }

    // 🔹 SPLAY
    fn splay(root: Option<Box<Nodo>>, prioridad: u8) -> Option<Box<Nodo>> {
        let mut root = match root {
            None => return None,
            Some(r) => r,
        };

        if prioridad < root.incidente.prioridad {
            if root.izq.is_none() {
                return Some(root);
            }

            let mut left = root.izq.take().unwrap();

            if prioridad < left.incidente.prioridad {
                left.izq = Self::splay(left.izq.take(), prioridad);
                root.izq = Some(left);
                root = Self::rotar_derecha(root);
            } else if prioridad > left.incidente.prioridad {
                left.der = Self::splay(left.der.take(), prioridad);
                if left.der.is_some() {
                    left = Self::rotar_izquierda(left);
                }
                root.izq = Some(left);
            }

            return if root.izq.is_none() {
                Some(root)
            } else {
                Some(Self::rotar_derecha(root))
            };
        } else if prioridad > root.incidente.prioridad {
            if root.der.is_none() {
                return Some(root);
            }

            let mut right = root.der.take().unwrap();

            if prioridad > right.incidente.prioridad {
                right.der = Self::splay(right.der.take(), prioridad);
                root.der = Some(right);
                root = Self::rotar_izquierda(root);
            } else if prioridad < right.incidente.prioridad {
                right.izq = Self::splay(right.izq.take(), prioridad);
                if right.izq.is_some() {
                    right = Self::rotar_derecha(right);
                }
                root.der = Some(right);
            }

            return if root.der.is_none() {
                Some(root)
            } else {
                Some(Self::rotar_izquierda(root))
            };
        }

        Some(root)
    }

    // 🔹 Insertar
    pub fn registrar_incidente(&mut self, id: u32, prioridad: u8, desc: &str) {
        let incidente = Incidente {
            id,
            prioridad,
            descripcion: desc.to_string(),
        };

        self.directorio.insert(id, incidente.clone());

        let raiz = Self::splay(self.raiz.take(), prioridad);

        let mut nuevo = Box::new(Nodo {
            incidente,
            izq: None,
            der: None,
        });

        match raiz {
            None => self.raiz = Some(nuevo),
            Some(mut r) => {
                if prioridad < r.incidente.prioridad {
                    nuevo.der = Some(r);
                    self.raiz = Some(nuevo);
                } else {
                    nuevo.izq = Some(r);
                    self.raiz = Some(nuevo);
                }
            }
        }
    }

    // 🔹 Buscar (también hace splay)
    pub fn buscar_incidente(&mut self, prioridad: u8) {
        self.raiz = Self::splay(self.raiz.take(), prioridad);

        if let Some(ref r) = self.raiz {
            println!(
                "Raíz tras acceso → {} (Prioridad {})",
                r.incidente.descripcion, r.incidente.prioridad
            );
        }
    }
}

fn main() {
    let mut sistema = CentroEmergencias::nuevo();

    println!("--- Sistema con Splay Tree ---");

    sistema.registrar_incidente(1, 10, "Ataque cardíaco");
    sistema.registrar_incidente(2, 2, "Gato atrapado");
    sistema.registrar_incidente(3, 7, "Accidente de tránsito");

    sistema.buscar_incidente(7);
    sistema.buscar_incidente(2);
}