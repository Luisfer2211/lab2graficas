use minifb::{Key, Window, WindowOptions};
use std::thread;
use std::time::{Duration, Instant};

// Resolución lógica y escala para la ventana
const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const SCALE: usize = 6;
const WINDOW_WIDTH: usize = WIDTH * SCALE;
const WINDOW_HEIGHT: usize = HEIGHT * SCALE;

// Colores personalizados
const ALIVE_COLOR: u32 = 0x39FF14; // verde fluorescente
const DEAD_COLOR:  u32 = 0x0C0015; // morado muy oscuro

fn main() {
    // Buffers lógicos de células
    let mut grid = vec![false; WIDTH * HEIGHT];
    let mut next = vec![false; WIDTH * HEIGHT];

    // Crear ventana
    let mut window = Window::new(
        "Conway's Game of Life (Rust)",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    ).unwrap();
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    // Patrón inicial: múltiples organismos
    spawn_glider(&mut grid, 10, 10);
    spawn_blinker(&mut grid, 30, 20);
    spawn_beacon(&mut grid, 50, 50);
    spawn_toad(&mut grid, 20, 60);
    spawn_pulsar(&mut grid, 60, 10);
    spawn_pentadecathlon(&mut grid, 40, 30);
    spawn_lwss(&mut grid, 80, 20);
    spawn_mwss(&mut grid, 80, 50);
    spawn_hwss(&mut grid, 80, 80);

    // Bucle principal
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let t0 = Instant::now();

        // Calcular siguiente generación
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = index(x, y);
                let alive = grid[idx];
                let n = count_neighbors(&grid, x, y);
                next[idx] = match (alive, n) {
                    (true, 2) | (true, 3) => true,
                    (false, 3)          => true,
                    _                   => false,
                };
            }
        }
        std::mem::swap(&mut grid, &mut next);

        // Render: dibujar cada célula escalada
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                // Usamos nuestros colores personalizados
                let color = if grid[index(x,y)] { ALIVE_COLOR } else { DEAD_COLOR };
                point_scaled(&mut buffer, x, y, color);
            }
        }
        window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

        // Mantener ~10 FPS
        let elapsed = t0.elapsed();
        if elapsed < Duration::from_millis(100) {
            thread::sleep(Duration::from_millis(100) - elapsed);
        }
    }
}

// Índice lineal de celda
fn index(x: usize, y: usize) -> usize {
    y * WIDTH + x
}

// Contar vecinos con frontera toroidal
fn count_neighbors(grid: &[bool], x: usize, y: usize) -> usize {
    let mut c = 0;
    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            if dx == 0 && dy == 0 { continue; }
            let nx = ((x as isize + dx + WIDTH as isize) as usize) % WIDTH;
            let ny = ((y as isize + dy + HEIGHT as isize) as usize) % HEIGHT;
            if grid[index(nx, ny)] { c += 1; }
        }
    }
    c
}

// Dibuja un bloque SCALE×SCALE en buffer
fn point_scaled(buffer: &mut [u32], x: usize, y: usize, color: u32) {
    let base_x = x * SCALE;
    let base_y = y * SCALE;
    for dy in 0..SCALE {
        for dx in 0..SCALE {
            buffer[(base_y + dy) * WINDOW_WIDTH + (base_x + dx)] = color;
        }
    }
}

// Funciones spawn para distintos organismos:

fn spawn_glider(grid: &mut [bool], oy: usize, ox: usize) {
    let pts = [(0,1),(1,2),(2,0),(2,1),(2,2)];
    for (dy,dx) in &pts { grid[index(ox+*dx, oy+*dy)] = true; }
}

fn spawn_blinker(grid: &mut [bool], oy: usize, ox: usize) {
    let pts = [(1,0),(1,1),(1,2)];
    for (dy,dx) in &pts { grid[index(ox+*dx, oy+*dy)] = true; }
}

fn spawn_beacon(grid: &mut [bool], oy: usize, ox: usize) {
    let pts = [(0,0),(0,1),(1,0),(1,1),(2,2),(2,3),(3,2),(3,3)];
    for (dy,dx) in &pts { grid[index(ox+*dx, oy+*dy)] = true; }
}

fn spawn_toad(grid: &mut [bool], oy: usize, ox: usize) {
    let pts = [(1,1),(1,2),(1,3),(2,0),(2,1),(2,2)];
    for (dy,dx) in &pts { grid[index(ox+*dx, oy+*dy)] = true; }
}

fn spawn_pulsar(grid: &mut [bool], oy: usize, ox: usize) {
    let offs = [2,7,9,14];
    for &i in &offs {
        let a = i - 2;
        let b = i + 2;
        for j in (4..7) {
            grid[index(ox + a, oy + j)] = true;
            grid[index(ox + b, oy + j)] = true;
            grid[index(ox + j, oy + a)] = true;
            grid[index(ox + j, oy + b)] = true;
        }
        grid[index(ox + i, oy + 0)] = true;
        grid[index(ox + i, oy + 5)] = true;
        grid[index(ox + i, oy + 7)] = true;
        grid[index(ox + i, oy + 12)] = true;
        grid[index(ox + 0, oy + i)] = true;
        grid[index(ox + 5, oy + i)] = true;
        grid[index(ox + 7, oy + i)] = true;
        grid[index(ox + 12,oy + i)] = true;
    }
}

fn spawn_pentadecathlon(grid: &mut [bool], oy: usize, ox: usize) {
    let pts = [(1,3),(2,3),(3,3),(4,3),(5,3),(6,3),(7,3),(8,3),(9,3),(10,3),
               (0,4),(11,4),(1,5),(2,5),(3,5),(4,5),(5,5),(6,5),(7,5),(8,5),(9,5),(10,5)];
    for (dy,dx) in &pts { grid[index(ox+dx, oy+dy)] = true; }
}

fn spawn_lwss(grid: &mut [bool], oy: usize, ox: usize) {
    let pts = [(0,1),(0,4),(1,0),(2,0),(3,0),(4,0),(4,1),(4,2),(3,3)];
    for (dy,dx) in &pts { grid[index(ox+dx, oy+*dy)] = true; }
}

fn spawn_mwss(grid: &mut [bool], oy: usize, ox: usize) {
    let pts = [(0,1),(0,2),(0,5),(1,0),(2,0),(3,0),(4,0),(5,0),(6,1),(6,2),(6,3),(5,4),(4,5)];
    for (dy,dx) in &pts { grid[index(ox+dx, oy+*dy)] = true; }
}

fn spawn_hwss(grid: &mut [bool], oy: usize, ox: usize) {
    let pts = [(0,2),(0,5),(1,0),(2,0),(3,0),(4,0),(5,0),(6,0),(7,1),(7,2),(7,3),(7,4),(6,5),(5,6)];
    for (dy,dx) in &pts { grid[index(ox+dx, oy+*dy)] = true; }
}
