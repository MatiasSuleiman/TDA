// Ej 2
// b
use std::vec;
fn main(){
    println!("{}",magi_cuadrados_de_orden_n(3))
}
fn magi_cuadrados_de_orden_n(n:usize) -> i32 {
    let magi_cuadrado: Vec<Vec<i32>> = vec![vec![0;n];n];
    let mut i: usize  = 0;
    let mut rango:Vec<i32> = vec![];
    while i<(n*n){
        rango.push(i as i32 + 1);
        i += 1;
    }
    return soluciones_validas_de_la_rama(magi_cuadrado, rango, 0, 0, n);
    }
fn soluciones_validas_de_la_rama(cuadrado:Vec<Vec<i32>>,numeros_disponibles:Vec<i32>, fila_actual:usize, columna_actual:usize, n:usize) -> i32{  
    if fila_actual == (n-1) && columna_actual == (n-1){
        let mut solucion = cuadrado.clone();
        solucion[n-1] [n-1] = numeros_disponibles [0];
        return solucion_valida(&mut solucion) as i32;
    }
    else {
        if columna_actual == (n-1){
            let mut res: i32 = 0;
            let mut i = 0;
            while i< numeros_disponibles.len(){
              let mut solucion_parcial = cuadrado.clone();
              solucion_parcial[fila_actual][columna_actual] = numeros_disponibles[i];
              if cambio_valido(&mut solucion_parcial, fila_actual, columna_actual){
                let mut nuevos_disponibles = numeros_disponibles.clone();
                nuevos_disponibles.remove(i);
                res += soluciones_validas_de_la_rama(solucion_parcial, nuevos_disponibles, fila_actual+1, 0, n);
              }
              i += 1;
            }
            return res;
        }
        else{
            let mut res: i32 = 0;
            let mut i = 0;
            while i< numeros_disponibles.len(){
              let mut solucion_parcial = cuadrado.clone();
              solucion_parcial[fila_actual][columna_actual] = numeros_disponibles[i];
              if cambio_valido(&mut solucion_parcial, fila_actual, columna_actual){
                let mut nuevos_disponibles = numeros_disponibles.clone();
                nuevos_disponibles.remove(i);
                res += soluciones_validas_de_la_rama(solucion_parcial, nuevos_disponibles, fila_actual, columna_actual + 1, n);
              }
              
              i += 1;
            }
            return res;       
        }
        
    }
    }
fn cambio_valido (matriz:&mut Vec<Vec<i32>>, fila_cambiada:usize, columna_cambiada:usize) -> bool{
    let n = matriz.len();
    let mut i:usize = 0;
    let mut sumatoria:i32 = 0;
    let num_magico = ((n*n*n + n) as i32)/2;
    while i < n{
        sumatoria += matriz [fila_cambiada] [i];
        if sumatoria > num_magico {
        return false;
        }
        i += 1;
    }
    sumatoria = 0;
    i = 0;
    while i < n{
        sumatoria += matriz [i] [columna_cambiada];
        if sumatoria > num_magico {
            return false;
        }
        i += 1
    }
    if columna_cambiada == fila_cambiada{
        i = 0;
        sumatoria = 0;
        while i < n{
            sumatoria += matriz [i] [i];
            if sumatoria > num_magico {
                return false
            }
            i +=1
        }
    }
    if columna_cambiada + fila_cambiada == n -1{
        i = 0;
        sumatoria = 0;
        while i < n{
            sumatoria += matriz [n-1-i] [i];
            if sumatoria > num_magico{
                return false
            }
            i +=1
        }
    }
    return true;
}
fn solucion_valida(matriz:&mut Vec<Vec<i32>>) -> bool{
    let n:usize = matriz.len();
    let mut i:usize = 1;
    let mut j:usize = 0;
    let mut candidato :i32 = 0;
    while j < n{
        candidato += matriz [0] [j];
        j += 1;
    }
    while i < n{
        j = 0;
        let mut contador:i32 = 0;
        while j < n{
            contador += matriz [i] [j];
            j += 1;
        }
        if contador != candidato{
            return false;
        }
        i += 1;
    }
    j = 0;
    while j < n{
        let mut contador:i32 = 0;
        i = 0;
        while i < n{
            contador += matriz [i] [j];
            i += 1;
        }
        if contador != candidato{
            return false;
        }
        j += 1;
    }
    i = 0;
    j = 0;
    let mut contador = 0;
    while i < n{
        contador += matriz [i] [j];
        i += 1;
        j += 1;
    }
    if contador != candidato{
        return false;
    }
    i = n-1;
    j = 0;
    contador = 0;
    while j < n{
        contador += matriz [i] [j];
        if i != 0 {
        i -= 1;
    }
        j += 1;
    }
    if contador != candidato{
        return false
    }
    return true;
}
