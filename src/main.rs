
use macroquad::prelude::*;
use macroquad::shapes::DrawRectangleParams;
use rapier2d::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};



const SCREEN_W: f32 = 1280.0;
const SCREEN_H: f32 = 720.0;

const VIEW_W: f32 = 840.0;
const PANEL_X: f32 = 850.0;

const ORIGIN_X: f32 = 420.0;
const GROUND_SY: f32 = 600.0;
const PPM: f32 = 110.0;

const FIXED_DT: f32 = 1.0 / 60.0;
const GRAVITY: f32 = 12.0;

const MAX_ATTEMPTS: usize = 6;
const SOLUTION_LEN: usize = 5;
const EXPLODE_INTERVAL: f32 = 0.8;
const EXPLODE_DELAY: f32 = 0.35;
const SIM_TAIL: f32 = 2.0;
const RESET_TIME: f32 = 1.4;

const BLAST_RADIUS: f32 = 1.9;
const BLAST_POWER: f32 = 4.0;

const GX0: f32 = 899.0;
const GY0: f32 = 112.0;
const CELL: f32 = 60.0;
const CELL_GAP: f32 = 8.0;
const ROW_PITCH: f32 = 68.0;

const SEL_Y: f32 = 66.0;
const SEL_H: f32 = 30.0;
const SEL_BTN_W: f32 = 34.0;
const SEL_LEFT_X: f32 = 867.0;
const SEL_RIGHT_X: f32 = 1229.0;

const LETTER_Y: f32 = 544.0;
const LETTER_W: f32 = 44.0;
const LETTER_H: f32 = 40.0;
const LETTER_GAP: f32 = 6.0;
const LETTER_X0: f32 = 868.0;

const ACTION_Y: f32 = 600.0;
const ACTION_H: f32 = 54.0;
const BTN_BORRAR_X: f32 = 868.0;
const BTN_BORRAR_W: f32 = 130.0;
const BTN_DETONAR_X: f32 = 1008.0;
const BTN_DETONAR_W: f32 = 240.0;

const PANEL_CARD_X: f32 = 858.0;
const PANEL_CARD_Y: f32 = 14.0;
const PANEL_CARD_W: f32 = 414.0;
const PANEL_CARD_H: f32 = 672.0;
const PANEL_RADIUS: f32 = 16.0;

const RETRY_X: f32 = VIEW_W / 2.0 - 90.0;
const RETRY_Y: f32 = 470.0;
const RETRY_W: f32 = 180.0;
const RETRY_H: f32 = 46.0;


const C_BG: Color = Color { r: 0.05, g: 0.07, b: 0.10, a: 1.0 };
const C_PANEL: Color = Color { r: 0.063, g: 0.078, b: 0.106, a: 1.0 };
const C_PANEL_BORDER: Color = Color { r: 0.18, g: 0.22, b: 0.27, a: 1.0 };
const C_DIVIDER: Color = Color { r: 0.15, g: 0.18, b: 0.23, a: 1.0 };
const C_WHITE: Color = Color { r: 0.93, g: 0.94, b: 0.96, a: 1.0 };
const C_DIM: Color = Color { r: 0.55, g: 0.60, b: 0.68, a: 1.0 };
const C_FAINT: Color = Color { r: 0.36, g: 0.41, b: 0.48, a: 1.0 };
const C_ACCENT: Color = Color { r: 0.95, g: 0.55, b: 0.15, a: 1.0 };
const C_ACCENT_HI: Color = Color { r: 1.0, g: 0.67, b: 0.28, a: 1.0 };
const C_GREEN: Color = Color { r: 0.33, g: 0.55, b: 0.31, a: 1.0 };
const C_GREEN_BORDER: Color = Color { r: 0.45, g: 0.72, b: 0.42, a: 1.0 };
const C_YELLOW: Color = Color { r: 0.71, g: 0.62, b: 0.23, a: 1.0 };
const C_YELLOW_BORDER: Color = Color { r: 0.88, g: 0.78, b: 0.35, a: 1.0 };
const C_GRAY_CELL: Color = Color { r: 0.23, g: 0.23, b: 0.24, a: 1.0 };
const C_GRAY_BORDER: Color = Color { r: 0.35, g: 0.35, b: 0.37, a: 1.0 };
const C_CELL_EMPTY: Color = Color { r: 0.14, g: 0.17, b: 0.22, a: 1.0 };
const C_CELL_EMPTY_BORDER: Color = Color { r: 0.24, g: 0.29, b: 0.36, a: 1.0 };
const C_RED: Color = Color { r: 0.91, g: 0.34, b: 0.34, a: 1.0 };
const C_BTN_DARK: Color = Color { r: 0.14, g: 0.17, b: 0.21, a: 1.0 };
const C_BTN_BORDER: Color = Color { r: 0.26, g: 0.31, b: 0.38, a: 1.0 };
const C_INK: Color = Color { r: 0.14, g: 0.09, b: 0.05, a: 1.0 };


#[derive(Clone, Copy)]
struct BlockSpec {
    id: char,
    x: f32,
    y: f32,
    hx: f32,
    hy: f32,
}

struct Level {
    name: &'static str,
    blocks: &'static [BlockSpec],
    solution: [char; SOLUTION_LEN],
}

const LEVEL_0_BLOCKS: &[BlockSpec] = &[
    BlockSpec { id: 'A', x: -0.55, y: 0.50, hx: 0.28, hy: 0.50 },
    BlockSpec { id: 'B', x: -0.55, y: 1.50, hx: 0.28, hy: 0.50 },
    BlockSpec { id: 'C', x: -0.55, y: 2.50, hx: 0.28, hy: 0.50 },
    BlockSpec { id: 'D', x: 0.55, y: 0.50, hx: 0.28, hy: 0.50 },
    BlockSpec { id: 'E', x: 0.55, y: 1.50, hx: 0.28, hy: 0.50 },
    BlockSpec { id: 'F', x: 0.55, y: 2.50, hx: 0.28, hy: 0.50 },
    BlockSpec { id: 'G', x: 0.00, y: 3.22, hx: 1.05, hy: 0.22 },
    BlockSpec { id: 'H', x: 0.00, y: 3.74, hx: 0.40, hy: 0.30 },
];

const LEVEL_1_BLOCKS: &[BlockSpec] = &[
    BlockSpec { id: 'A', x: -2.00, y: 0.40, hx: 0.40, hy: 0.40 },
    BlockSpec { id: 'B', x: -2.00, y: 1.20, hx: 0.40, hy: 0.40 },
    BlockSpec { id: 'C', x: -2.00, y: 2.00, hx: 0.40, hy: 0.40 },
    BlockSpec { id: 'D', x: 2.00, y: 0.40, hx: 0.40, hy: 0.40 },
    BlockSpec { id: 'E', x: 2.00, y: 1.20, hx: 0.40, hy: 0.40 },
    BlockSpec { id: 'F', x: 2.00, y: 2.00, hx: 0.40, hy: 0.40 },
    BlockSpec { id: 'G', x: 0.00, y: 2.55, hx: 2.00, hy: 0.15 },
    BlockSpec { id: 'H', x: 0.00, y: 2.95, hx: 0.50, hy: 0.25 },
];

const LEVEL_2_BLOCKS: &[BlockSpec] = &[
    BlockSpec { id: 'A', x: 0.00, y: 0.35, hx: 0.75, hy: 0.35 },
    BlockSpec { id: 'B', x: 0.00, y: 1.07, hx: 0.55, hy: 0.37 },
    BlockSpec { id: 'C', x: 0.00, y: 1.81, hx: 0.42, hy: 0.37 },
    BlockSpec { id: 'D', x: 0.00, y: 2.48, hx: 0.32, hy: 0.30 },
    BlockSpec { id: 'E', x: 0.00, y: 3.08, hx: 0.28, hy: 0.30 },
    BlockSpec { id: 'F', x: 0.00, y: 3.68, hx: 0.24, hy: 0.30 },
    BlockSpec { id: 'G', x: -1.15, y: 0.30, hx: 0.35, hy: 0.30 },
    BlockSpec { id: 'H', x: 1.15, y: 0.30, hx: 0.35, hy: 0.30 },
];

const LEVEL_3_BLOCKS: &[BlockSpec] = &[
    BlockSpec { id: 'A', x: -1.30, y: 0.45, hx: 0.45, hy: 0.45 },
    BlockSpec { id: 'B', x: -1.30, y: 1.35, hx: 0.45, hy: 0.45 },
    BlockSpec { id: 'C', x: 1.30, y: 0.45, hx: 0.45, hy: 0.45 },
    BlockSpec { id: 'D', x: 1.30, y: 1.35, hx: 0.45, hy: 0.45 },
    BlockSpec { id: 'E', x: 0.00, y: 1.95, hx: 1.75, hy: 0.15 },
    BlockSpec { id: 'F', x: 0.00, y: 2.35, hx: 0.50, hy: 0.25 },
    BlockSpec { id: 'G', x: -0.75, y: 2.35, hx: 0.25, hy: 0.25 },
    BlockSpec { id: 'H', x: 0.75, y: 2.35, hx: 0.25, hy: 0.25 },
];

const LEVEL_4_BLOCKS: &[BlockSpec] = &[
    BlockSpec { id: 'A', x: 0.00, y: 0.30, hx: 1.60, hy: 0.30 },
    BlockSpec { id: 'B', x: 0.00, y: 0.95, hx: 1.20, hy: 0.35 },
    BlockSpec { id: 'C', x: 0.00, y: 1.65, hx: 0.85, hy: 0.35 },
    BlockSpec { id: 'D', x: 0.00, y: 2.35, hx: 0.55, hy: 0.35 },
    BlockSpec { id: 'E', x: 0.00, y: 2.92, hx: 0.30, hy: 0.22 },
    BlockSpec { id: 'H', x: 0.00, y: 3.44, hx: 0.20, hy: 0.30 },
    BlockSpec { id: 'F', x: -1.95, y: 0.25, hx: 0.30, hy: 0.25 },
    BlockSpec { id: 'G', x: 1.95, y: 0.25, hx: 0.30, hy: 0.25 },
];

const LEVEL_5_BLOCKS: &[BlockSpec] = &[
    BlockSpec { id: 'A', x: 0.00, y: 0.50, hx: 0.35, hy: 0.50 },
    BlockSpec { id: 'B', x: 0.00, y: 1.50, hx: 0.35, hy: 0.50 },
    BlockSpec { id: 'C', x: 0.00, y: 2.50, hx: 0.35, hy: 0.50 },
    BlockSpec { id: 'D', x: 0.00, y: 3.18, hx: 1.80, hy: 0.18 },
    BlockSpec { id: 'E', x: 1.20, y: 3.58, hx: 0.30, hy: 0.22 },
    BlockSpec { id: 'F', x: -1.20, y: 3.58, hx: 0.30, hy: 0.22 },
    BlockSpec { id: 'G', x: -0.90, y: 0.30, hx: 0.30, hy: 0.30 },
    BlockSpec { id: 'H', x: 0.90, y: 0.30, hx: 0.30, hy: 0.30 },
];

const LEVELS: &[Level] = &[
    Level {
        name: "LA TORRE",
        blocks: LEVEL_0_BLOCKS,
        solution: ['B', 'E', 'A', 'F', 'G'],
    },
    Level {
        name: "EL PUENTE",
        blocks: LEVEL_1_BLOCKS,
        solution: ['C', 'F', 'B', 'E', 'G'],
    },
    Level {
        name: "LA AGUJA",
        blocks: LEVEL_2_BLOCKS,
        solution: ['B', 'D', 'A', 'E', 'G'],
    },
    Level {
        name: "EL ARCO",
        blocks: LEVEL_3_BLOCKS,
        solution: ['B', 'D', 'A', 'C', 'E'],
    },
    Level {
        name: "LA PIRAMIDE",
        blocks: LEVEL_4_BLOCKS,
        solution: ['B', 'C', 'A', 'D', 'E'],
    },
    Level {
        name: "LA GRUA",
        blocks: LEVEL_5_BLOCKS,
        solution: ['A', 'C', 'B', 'D', 'E'],
    },
];

// --- Idiomas ---
#[derive(Clone, Copy, PartialEq, Eq)]
enum Lang {
    Es,
    En,
}

static mut CURRENT_LANG: Lang = Lang::Es;

#[no_mangle]
pub extern "C" fn set_language(lang: u8) {
    unsafe {
        CURRENT_LANG = match lang {
            0 => Lang::Es,
            1 => Lang::En,
            _ => Lang::Es,
        };
    }
}

fn tr(key: &str) -> String {
    unsafe {
        match CURRENT_LANG {
            Lang::Es => TR_ES.get(key).copied().unwrap_or(key).to_string(),
            Lang::En => TR_EN.get(key).copied().unwrap_or(key).to_string(),
        }
    }
}

static TR_ES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "title" => "Demoliword",
    "subtitle" => "Puzzle diario de demolicion",
    "how_to_play" => "Como jugar",
    "pick_blocks" => "Elegí 5 bloques A–H en el orden correcto para colapsar la estructura.",
    "attempts" => "Tenés 6 intentos.",
    "load_blocks" => "A–H / click cargá bloques",
    "detonate" => "Enter detona",
    "backspace" => "Backspace borra",
    "retry" => "R o Enter reintenta",
    "switch_level" => "← → o 1–6 cambia de estructura",
    "green" => "Verde · en su posición",
    "yellow" => "Amarillo · está, pero en otro lugar",
    "gray" => "Gris · no está en la solución",
    "puzzle" => "PUZZLE #",
    "structure" => "ESTRUCTURA: ",
    "level" => "NIVEL ",
    "of" => " / ",
    "attempts_label" => "INTENTOS",
    "blocks_loaded" => "/5 BLOQUES CARGADOS",
    "ready_to_detonate" => "LISTO PARA DETONAR",
    "detonation" => "DETONACION ",
    "evaluating" => "EVALUANDO COLAPSO...",
    "perfect_collapse" => "COLAPSO PERFECTO",
    "solution" => "SOLUCION: ",
    "out_of_charges" => "SE ACABARON LAS CARGAS",
    "try_again" => "ENTER / R: reintentar - FLECHAS o 1-6: otro nivel",
    "rebuilding" => "RECONSTRUYENDO LA TORRE",
    "detonate_btn" => "DETONAR",
    "delete_btn" => "BORRAR",
    "retry_btn" => "OTRA VEZ",
};

static TR_EN: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "title" => "Demoliword",
    "subtitle" => "Daily demolition puzzle",
    "how_to_play" => "How to Play",
    "pick_blocks" => "Pick 5 blocks A–H in the exact order to collapse the structure.",
    "attempts" => "You have 6 attempts.",
    "load_blocks" => "A–H / click to load blocks",
    "detonate" => "Enter detonates",
    "backspace" => "Backspace deletes",
    "retry" => "R or Enter retries",
    "switch_level" => "← → or 1–6 changes structure",
    "green" => "Green · correct position",
    "yellow" => "Yellow · exists, wrong place",
    "gray" => "Gray · not in solution",
    "puzzle" => "PUZZLE #",
    "structure" => "STRUCTURE: ",
    "level" => "LEVEL ",
    "of" => " / ",
    "attempts_label" => "ATTEMPTS",
    "blocks_loaded" => "/5 BLOCKS LOADED",
    "ready_to_detonate" => "READY TO DETONATE",
    "detonation" => "DETONATION ",
    "evaluating" => "EVALUATING COLLAPSE...",
    "perfect_collapse" => "PERFECT COLLAPSE",
    "solution" => "SOLUTION: ",
    "out_of_charges" => "OUT OF CHARGES",
    "try_again" => "ENTER / R: retry - ARROWS or 1-6: change level",
    "rebuilding" => "REBUILDING TOWER",
    "detonate_btn" => "DETONATE",
    "delete_btn" => "DELETE",
    "retry_btn" => "AGAIN",
};

static mut DAYS_SINCE_EPOCH: i64 = 0;

#[no_mangle]
pub extern "C" fn set_daily_seed(days: i64) {
    unsafe { DAYS_SINCE_EPOCH = days; }
}

fn days_since_epoch() -> i64 {
    let days = unsafe { DAYS_SINCE_EPOCH };
    if days != 0 {
        return days;
    }
    #[cfg(target_arch = "wasm32")]
    {
        return 0;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| (d.as_secs() / 86_400) as i64)
            .unwrap_or(0)
    }
}

fn get_daily_level_index() -> usize {
    (days_since_epoch().rem_euclid(LEVELS.len() as i64)) as usize
}


fn hash01(mut n: u32) -> f32 {
    n = n.wrapping_mul(0x9E37_79B1);
    n ^= n >> 15;
    n = n.wrapping_mul(0x85EB_CA77);
    n ^= n >> 13;
    (n & 0x00FF_FFFF) as f32 / 16_777_215.0
}

fn w2s(x: f32, y: f32) -> (f32, f32) {
    (ORIGIN_X + x * PPM, GROUND_SY - y * PPM)
}

fn text_center(text: impl AsRef<str>, cx: f32, baseline_y: f32, size: f32, color: Color) {
    let s = text.as_ref();
    let w = measure_text(s, None, size as u16, 1.0).width;
    draw_text(s, cx - w / 2.0, baseline_y, size, color);
}

fn text_right(text: impl AsRef<str>, rx: f32, baseline_y: f32, size: f32, color: Color) {
    let s = text.as_ref();
    let w = measure_text(s, None, size as u16, 1.0).width;
    draw_text(s, rx - w, baseline_y, size, color);
}

fn inside(x: f32, y: f32, w: f32, h: f32, mx: f32, my: f32) -> bool {
    mx >= x && mx <= x + w && my >= y && my <= y + h
}

fn draw_round_rect(x: f32, y: f32, w: f32, h: f32, r: f32, color: Color) {
    let r = r.min(w * 0.5).min(h * 0.5);
    draw_rectangle(x + r, y, w - r * 2.0, h, color);
    draw_rectangle(x, y + r, r, h - r * 2.0, color);
    draw_rectangle(x + w - r, y + r, r, h - r * 2.0, color);
    draw_circle(x + r, y + r, r, color);
    draw_circle(x + w - r, y + r, r, color);
    draw_circle(x + r, y + h - r, r, color);
    draw_circle(x + w - r, y + h - r, r, color);
}

fn draw_cell(cx: f32, cy: f32, size: f32, fill: Color, border: Color) {
    let half = size * 0.5;
    draw_rectangle(cx - half, cy - half, size, size, fill);
    draw_rectangle_lines(cx - half, cy - half, size, size, 2.0, border);
}

fn draw_cell_letter(cx: f32, cy: f32, size: f32, ch: char, color: Color) {
    let font = size * 0.52;
    text_center(&ch.to_string(), cx, cy + font * 0.35, font, color);
}

fn draw_cell_glow(cx: f32, cy: f32, size: f32, pad: f32, color: Color, thickness: f32) {
    let s = size + pad * 2.0;
    draw_rectangle_lines(cx - s * 0.5, cy - s * 0.5, s, s, thickness, color);
}

fn reveal_progress(t: f32, col: usize) -> f32 {
    ((t - col as f32 * 0.10) / 0.30).clamp(0.0, 1.0)
}

fn draw_cloud(x: f32, y: f32, w: f32, a: f32) {
    let c = Color { r: 0.72, g: 0.79, b: 0.90, a };
    draw_circle(x, y, w * 0.20, c);
    draw_circle(x + w * 0.22, y - w * 0.06, w * 0.25, c);
    draw_circle(x + w * 0.48, y + w * 0.01, w * 0.18, c);
    draw_circle(x + w * 0.28, y + w * 0.05, w * 0.20, c);
}

fn make_light_texture(seed: u32) -> Texture2D {
    const SIZE: usize = 128;
    let mut img = Image::gen_image_color(
        SIZE as u16,
        SIZE as u16,
        Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0 },
    );
    let c = (SIZE as f32 - 1.0) * 0.5;

    let mut blobs = [(0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32); 6];
    for (i, b) in blobs.iter_mut().enumerate() {
        let k = i as u32;
        let angle = hash01(seed.wrapping_mul(131) + k * 37 + 1) * std::f32::consts::TAU;
        let dist = hash01(seed.wrapping_mul(977) + k * 53 + 2) * 0.34;
        *b = (
            0.42 + hash01(seed.wrapping_mul(31) + k * 17 + 3) * 0.50,
            angle.cos() * dist,
            angle.sin() * dist,
            (0.20 + hash01(seed.wrapping_mul(71) + k * 29 + 4) * 0.24)
                * (1.25 + hash01(seed.wrapping_mul(91) + k * 47 + 6) * 0.7),
            (0.13 + hash01(seed.wrapping_mul(111) + k * 59 + 7) * 0.16)
                * (0.75 + hash01(seed.wrapping_mul(143) + k * 67 + 8) * 0.5),
            hash01(seed.wrapping_mul(167) + k * 73) * std::f32::consts::TAU,
        );
    }

    for y in 0..SIZE {
        let py = (y as f32 - c) / c;
        for x in 0..SIZE {
            let px = (x as f32 - c) / c;
            let mut a = 0.0f32;
            for (w, bx, by, sx, sy, rot) in blobs {
                let dx = (f32::cos(rot)) * (px - bx) - (f32::sin(rot)) * (py - by);
                let dy = (f32::sin(rot)) * (px - bx) + (f32::cos(rot)) * (py - by);
                let dx = dx / sx.max(0.02);
                let dy = dy / sy.max(0.02);
                let d2 = dx * dx + dy * dy;
                a += w * (-0.52 * d2).exp();
            }
            let d_center = (px * px + py * py).sqrt();
            let core = 0.24 * (-d_center * d_center * 5.8).exp();
            let halo = 0.10 * (-d_center * d_center * 1.55).exp();
            a = a * 0.30 + core + halo * (0.62 + 0.38 * a.clamp(0.0, 1.0));
            let grain =
                hash01((y as u32).wrapping_mul(4096).wrapping_add(x as u32 + seed * 7919));
            a = (a * (0.75 + 0.25 * grain)).clamp(0.0, 1.0)
                * (1.0 - d_center * 0.32).clamp(0.0, 1.0);
            img.set_pixel(x as u32, y as u32, Color { r: 1.0, g: 1.0, b: 1.0, a });
        }
    }

    let tex = Texture2D::from_image(&img);
    tex.set_filter(FilterMode::Linear);
    tex
}

fn draw_light(tex: &Texture2D, x: f32, y: f32, r: f32, color: Color) {
    let phase = hash01((x.abs() + y.abs()) as u32) * std::f32::consts::TAU;
    let dx = (f32::sin(phase)) * r * 0.24;
    let dy = (f32::cos(phase * 1.37)) * r * 0.20;
    draw_texture_ex(
        tex,
        x - r + dx,
        y - r + dy,
        color,
        DrawTextureParams {
            dest_size: Some(vec2(r * 2.0, r * 2.0)),
            rotation: phase * 2.0,
            pivot: Some(vec2(x + dx, y + dy)),
            ..Default::default()
        },
    );
}


const MOON_X: f32 = 740.0;
const MOON_Y: f32 = 118.0;

fn draw_moon(t: f32, lights: &[Texture2D; 3]) {
    let breathe = 0.5 + 0.5 * (t * 0.55).sin();

    let halo = Color { r: 0.80, g: 0.88, b: 1.0, a: 1.0 };
    draw_light(&lights[0], MOON_X, MOON_Y, 205.0 + 22.0 * breathe, Color { a: 0.075, ..halo });
    draw_light(&lights[1], MOON_X, MOON_Y, 136.0 + 15.0 * breathe, Color { a: 0.115, ..halo });
    draw_light(&lights[2], MOON_X, MOON_Y, 84.0 + 9.0 * breathe, Color { a: 0.18, ..halo });
    draw_light(&lights[0], MOON_X, MOON_Y, 46.0 + 4.0 * breathe, Color { r: 0.97, g: 0.96, b: 0.90, a: 0.30 });

    let rays = 15u32;
    for i in 0..rays {
        let base = i as f32 / rays as f32 * std::f32::consts::TAU;
        let ang = base + t * 0.035 + hash01(i + 1101) * 0.5;
        let wob = 0.55 + 0.45 * (t * (0.5 + hash01(i + 1102) * 0.9) + i as f32 * 1.7).sin();
        let len = 110.0 + hash01(i + 1103) * 150.0 * wob;
        let spread = 0.010 + hash01(i + 1104) * 0.022;
        let a = (0.018 + hash01(i + 1105) * 0.030) * (0.35 + 0.65 * wob);
        let p0 = vec2(MOON_X + ang.cos() * 32.0, MOON_Y + ang.sin() * 32.0);
        let p1 = vec2(MOON_X + (ang - spread).cos() * len, MOON_Y + (ang - spread).sin() * len);
        let p2 = vec2(MOON_X + (ang + spread).cos() * len, MOON_Y + (ang + spread).sin() * len);
        draw_triangle(p0, p1, p2, Color { r: 0.80, g: 0.89, b: 1.0, a });
    }

    draw_triangle(
        vec2(MOON_X - 36.0, MOON_Y + 18.0),
        vec2(MOON_X + 36.0, MOON_Y + 18.0),
        vec2(MOON_X - 210.0, GROUND_SY),
        Color { r: 0.70, g: 0.82, b: 1.0, a: 0.022 },
    );
    draw_triangle(
        vec2(MOON_X - 24.0, MOON_Y + 24.0),
        vec2(MOON_X + 24.0, MOON_Y + 24.0),
        vec2(MOON_X + 235.0, GROUND_SY),
        Color { r: 0.70, g: 0.82, b: 1.0, a: 0.016 },
    );

    draw_circle(MOON_X, MOON_Y, 35.0, Color { r: 0.96, g: 0.94, b: 0.84, a: 0.97 });
    draw_circle(MOON_X - 11.0, MOON_Y - 8.0, 6.0, Color { r: 0.86, g: 0.83, b: 0.72, a: 0.5 });
    draw_circle(MOON_X + 10.0, MOON_Y + 7.0, 4.0, Color { r: 0.86, g: 0.83, b: 0.72, a: 0.45 });
    draw_circle(MOON_X + 2.0, MOON_Y - 14.0, 2.6, Color { r: 0.86, g: 0.83, b: 0.72, a: 0.4 });
}

fn draw_building(cx: f32, w: f32, h: f32, seed: u32, t: f32, near: bool) {
    let top = GROUND_SY - h;
    let left = cx - w * 0.5;
    let base = if near {
        Color { r: 0.060, g: 0.076, b: 0.102, a: 1.0 }
    } else {
        Color { r: 0.072, g: 0.088, b: 0.116, a: 1.0 }
    };

    draw_rectangle(left, top, w, h, base);

    let tier_w = w * (0.44 + hash01(seed + 7) * 0.26);
    let tier_h = 5.0 + hash01(seed + 8) * 10.0;
    draw_rectangle(cx - tier_w * 0.5, top - tier_h, tier_w, tier_h, base);
    draw_rectangle(
        cx - tier_w * 0.5 - 1.5,
        top - tier_h - 2.0,
        tier_w + 3.0,
        2.0,
        Color { r: base.r * 1.45, g: base.g * 1.45, b: base.b * 1.45, a: 1.0 },
    );

    let lit_a = if near { 0.15 } else { 0.08 };
    let lit_x = if cx < MOON_X { left + w - 3.0 } else { left };
    draw_rectangle(lit_x, top, 3.0, h, Color { r: 0.62, g: 0.72, b: 0.94, a: lit_a });

    let mut fy = GROUND_SY - 18.0;
    while fy > top + 10.0 {
        draw_rectangle(left, fy, w, 1.0, Color {
            r: 0.16,
            g: 0.19,
            b: 0.24,
            a: if near { 0.15 } else { 0.08 },
        });
        fy -= 22.0;
    }

    let cols = (w / 15.0).floor().max(2.0) as i32;
    let rows = (h / 23.0).floor().max(2.0) as i32;
    let sx = w / cols as f32;
    let sy = h / (rows as f32 + 1.2);
    for r in 0..rows {
        for c in 0..cols {
            let hsh = hash01(seed.wrapping_mul(977).wrapping_add((r * 31 + c * 7) as u32));
            if hsh <= 0.55 {
                continue;
            }
            let wx = left + sx * (c as f32 + 0.5) - 2.5;
            let wy = top + sy * (r as f32 + 0.75);
            let warm = hash01(seed.wrapping_add((r * 13 + c * 29) as u32)) > 0.22;
            let rate = 0.35 + hash01(seed.wrapping_add((r * 3 + c * 5 + 17) as u32)) * 1.1;
            let flick = 0.55 + 0.45 * (t * rate + seed as f32 * 0.7 + (r * 7 + c * 13) as f32).sin();
            let gain = if near { 1.0 } else { 0.55 };
            let col = if warm {
                Color { r: 0.97, g: 0.79, b: 0.43, a: (0.18 + 0.32 * flick) * gain }
            } else {
                Color { r: 0.66, g: 0.80, b: 1.0, a: (0.12 + 0.24 * flick) * gain }
            };
            draw_rectangle(wx, wy, 5.0, 7.0, col);
        }
    }

    match seed % 7 {
        0 => {
            let tw = 10.0 + hash01(seed + 21) * 7.0;
            let th = 9.0 + hash01(seed + 22) * 7.0;
            let tx = cx + (hash01(seed + 23) - 0.5) * w * 0.4;
            let ty = top - tier_h - th - 6.0;
            draw_rectangle(tx - tw * 0.5, ty, tw, th, Color { r: 0.105, g: 0.125, b: 0.16, a: 1.0 });
            draw_rectangle(tx - tw * 0.5, ty, tw, 2.0, Color { r: 0.24, g: 0.28, b: 0.34, a: 0.9 });
            draw_line(tx - tw * 0.3, ty + th, tx - tw * 0.3, top - tier_h, 1.0, Color { r: 0.18, g: 0.21, b: 0.27, a: 0.9 });
            draw_line(tx + tw * 0.3, ty + th, tx + tw * 0.3, top - tier_h, 1.0, Color { r: 0.18, g: 0.21, b: 0.27, a: 0.9 });
        }
        1 | 2 => {
            let ax = cx + (hash01(seed + 31) - 0.5) * w * 0.5;
            let ah = 12.0 + hash01(seed + 32) * 18.0;
            let ay = top - tier_h;
            draw_line(ax, ay, ax, ay - ah, 1.5, Color { r: 0.22, g: 0.25, b: 0.30, a: 1.0 });
            let blink = (0.5 + 0.5 * (t * 2.0 + seed as f32).sin()).powf(5.0);
            draw_circle(ax, ay - ah, 4.0 + 3.5 * blink, Color { r: 1.0, g: 0.25, b: 0.2, a: 0.12 * blink });
            draw_circle(ax, ay - ah, 1.6, Color { r: 1.0, g: 0.32, b: 0.26, a: 0.35 + 0.65 * blink });
        }
        _ => {
            let bw = 8.0 + hash01(seed + 42) * 9.0;
            let bh = 3.0 + hash01(seed + 43) * 5.0;
            let bx = left + w * (0.18 + hash01(seed + 44) * 0.5);
            draw_rectangle(bx, top - tier_h - bh, bw, bh, Color { r: 0.11, g: 0.13, b: 0.17, a: 1.0 });
        }
    }
}

fn draw_skyline(t: f32, lights: &[Texture2D; 3]) {
    let mut x = -24.0;
    let mut i = 0u32;
    while x < VIEW_W + 24.0 {
        let w = 32.0 + hash01(i + 501) * 46.0;
        let h = 58.0 + hash01(i + 502) * 150.0;
        draw_building(x + w * 0.5, w, h, i + 600, t, false);
        x += w + 5.0 + hash01(i + 503) * 24.0;
        i += 1;
    }

    draw_city_haze(t, lights);

    let mut x = -30.0;
    let mut i = 0u32;
    while x < VIEW_W + 34.0 {
        let w = 42.0 + hash01(i + 701) * 58.0;
        let h = 72.0 + hash01(i + 702) * 190.0;
        draw_building(x + w * 0.5, w, h, i + 800, t, true);
        x += w + 14.0 + hash01(i + 703) * 42.0;
        i += 1;
    }
}

fn draw_street_lights(t: f32, lights: &[Texture2D; 3]) {
    for i in 0..7u32 {
        let x = 34.0 + i as f32 * 118.0 + hash01(i + 901) * 48.0;
        if x > VIEW_W + 10.0 {
            continue;
        }
        let base_y = GROUND_SY - 4.0;
        let phase = hash01(i + 902) * std::f32::consts::TAU;
        let speed = 0.7 + hash01(i + 903) * 1.5;
        let flick = 0.62 + 0.38 * (t * speed + phase).sin().max(-0.2);
        let height = 40.0 + hash01(i + 904) * 48.0;
        let top_y = base_y - height;

        draw_triangle(
            vec2(x - 11.0, base_y),
            vec2(x + 11.0, base_y),
            vec2(x + (hash01(i + 905) - 0.5) * 18.0, top_y),
            Color { r: 1.0, g: 0.74, b: 0.36, a: 0.020 * flick },
        );
        let pool = Color { r: 1.0, g: 0.70, b: 0.33, a: 1.0 };
        let tex = &lights[i as usize % lights.len()];
        draw_light(tex, x, base_y - 3.0, 34.0 + hash01(i + 906) * 22.0, Color { a: 0.055 * flick, ..pool });
        draw_light(
            &lights[(i as usize + 1) % lights.len()],
            x + 3.0,
            base_y - 5.0,
            18.0 + hash01(i + 907) * 14.0,
            Color { a: 0.12 * flick, ..pool },
        );
        draw_line(x, base_y, x, top_y, 2.0, Color { r: 0.13, g: 0.15, b: 0.19, a: 1.0 });
        draw_rectangle(x - 4.0, top_y - 2.0, 8.0, 3.0, Color { r: 0.16, g: 0.19, b: 0.23, a: 1.0 });
        draw_light(
            &lights[(i as usize + 2) % lights.len()],
            x,
            top_y,
            16.0 + 5.0 * flick,
            Color { r: 1.0, g: 0.82, b: 0.48, a: 0.20 * flick },
        );
        draw_circle(x, top_y, 2.4, Color { r: 1.0, g: 0.88, b: 0.62, a: 0.6 + 0.4 * flick });
    }
}

fn draw_city_haze(t: f32, lights: &[Texture2D; 3]) {
    for i in 0..6u32 {
        let x = 40.0 + i as f32 * 148.0 + (t * 0.045 + i as f32 * 1.7).sin() * 34.0;
        let pulse = 0.35 + 0.65 * (t * (0.18 + hash01(i + 1001) * 0.3) + i as f32 * 2.1).sin().max(0.0);
        let haze = Color { r: 0.95, g: 0.70, b: 0.42, a: 1.0 };
        let tex = &lights[i as usize % lights.len()];
        draw_light(
            tex,
            x,
            GROUND_SY - 38.0 - hash01(i + 1002) * 26.0,
            110.0 + hash01(i + 1003) * 100.0,
            Color { a: 0.035 * pulse, ..haze },
        );
        draw_light(
            &lights[(i as usize + 1) % lights.len()],
            x + 26.0,
            GROUND_SY - 52.0 - hash01(i + 1004) * 20.0,
            70.0 + hash01(i + 1005) * 60.0,
            Color { a: 0.028 * pulse, ..haze },
        );
    }
}

fn draw_button(x: f32, y: f32, w: f32, h: f32, label: &str, fill: Color, border: Color, txt: Color, size: f32) {
    draw_rectangle(x, y, w, h, fill);
    draw_rectangle_lines(x, y, w, h, 2.0, border);
    text_center(label, x + w / 2.0, y + h / 2.0 + size * 0.36, size, txt);
}

fn letter_rect(i: usize) -> (f32, f32, f32, f32) {
    let x = LETTER_X0 + i as f32 * (LETTER_W + LETTER_GAP);
    (x, LETTER_Y, LETTER_W, LETTER_H)
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Feedback {
    Green,
    Yellow,
    Gray,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Planning,
    Simulating,
    Revealing,
    Resetting,
    Won,
    Lost,
}

#[derive(Clone, Copy)]
struct Sim {
    next: usize,
    timer: f32,
    post: f32,
}

struct Block {
    id: char,
    hx: f32,
    hy: f32,
    handle: RigidBodyHandle,
    alive: bool,
    tint: f32,
}

struct World {
    pipeline: PhysicsPipeline,
    integration_parameters: IntegrationParameters,
    islands: IslandManager,
    broad_phase: BroadPhaseBvh,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    soft_bodies: SoftBodySet,
    ccd_solver: CCDSolver,
    blocks: Vec<Block>,
    _ground: RigidBodyHandle,
}

impl World {
    fn new(level: &Level) -> Self {
        let mut bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();

        let mut integration_parameters = IntegrationParameters::default();
        integration_parameters.dt = FIXED_DT;

        let ground_body = RigidBodyBuilder::fixed()
            .translation(Vector::new(0.0, -0.5))
            .build();
        let ground = bodies.insert(ground_body);
        colliders.insert_with_parent(
            ColliderBuilder::cuboid(12.0, 0.5).friction(0.9).build(),
            ground,
            &mut bodies,
        );

        let mut blocks = Vec::with_capacity(level.blocks.len());
        for (i, spec) in level.blocks.iter().enumerate() {
            let body = RigidBodyBuilder::dynamic()
                .translation(Vector::new(spec.x, spec.y))
                .linear_damping(0.05)
                .build();
            let handle = bodies.insert(body);
            let collider = ColliderBuilder::cuboid(spec.hx, spec.hy)
                .density(1.1)
                .friction(0.85)
                .restitution(0.0)
                .build();
            colliders.insert_with_parent(collider, handle, &mut bodies);

            blocks.push(Block {
                id: spec.id,
                hx: spec.hx,
                hy: spec.hy,
                handle,
                alive: true,
                tint: hash01(i as u32 * 7 + 3),
            });
        }

        World {
            pipeline: PhysicsPipeline::new(),
            integration_parameters,
            islands: IslandManager::new(),
            broad_phase: BroadPhaseBvh::new(),
            narrow_phase: NarrowPhase::new(),
            bodies,
            colliders,
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            soft_bodies: SoftBodySet::new(),
            ccd_solver: CCDSolver::new(),
            blocks,
            _ground: ground,
        }
    }

    fn step(&mut self) {
        self.pipeline.step(
            Vector::new(0.0, -GRAVITY),
            &self.integration_parameters,
            &mut self.islands,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.soft_bodies,
            &mut self.ccd_solver,
            &(),
            &(),
        );
    }

    fn detonate(&mut self, id: char) -> Option<(f32, f32)> {
        let idx = self
            .blocks
            .iter()
            .position(|b| b.id == id && b.alive)?;
        let center = self.bodies.get(self.blocks[idx].handle)?.translation();

        let mut hits: Vec<(RigidBodyHandle, f32, f32, f32)> = Vec::new();
        for (j, block) in self.blocks.iter().enumerate() {
            if j == idx || !block.alive {
                continue;
            }
            let Some(body) = self.bodies.get(block.handle) else {
                continue;
            };
            let p = body.translation();
            let dx = p.x - center.x;
            let dy = p.y - center.y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < BLAST_RADIUS && dist > 1e-4 {
                let falloff = 1.0 - dist / BLAST_RADIUS;
                let mag = BLAST_POWER * falloff * falloff.max(0.35);
                let ix = dx / dist * mag;
                let iy = dy / dist * mag + mag * 0.25;
                let torque = (dx * iy - dy * ix) * 0.4;
                hits.push((block.handle, ix, iy, torque));
            }
        }

        for (handle, ix, iy, torque) in hits {
            if let Some(body) = self.bodies.get_mut(handle) {
                body.apply_impulse(Vector::new(ix, iy), true);
                body.apply_torque_impulse(torque, true);
            }
        }

        let handle = self.blocks[idx].handle;
        self.blocks[idx].alive = false;
        self.bodies.remove(
            handle,
            &mut self.islands,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.soft_bodies,
            true,
        );
        Some((center.x, center.y))
    }
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    max_life: f32,
    size: f32,
    color: Color,
}

struct Flash {
    x: f32,
    y: f32,
    t: f32,
    max: f32,
}

struct Game {
    level_idx: usize,
    world: World,
    attempts_used: usize,
    history: Vec<([char; SOLUTION_LEN], [Feedback; SOLUTION_LEN])>,
    selection: Vec<char>,
    sim_guess: [char; SOLUTION_LEN],
    phase: Phase,
    next_phase: Phase,
    sim: Sim,
    reset_timer: f32,
    reveal_row: Option<usize>,
    reveal_t: f32,
    select_t: f32,
    last_letter: Option<char>,
    particles: Vec<Particle>,
    flashes: Vec<Flash>,
    particle_seed: u32,
    toast_msg: String,
    toast_timer: f32,
    shake: f32,
    acc: f32,
    light_tex: [Texture2D; 3],
}

impl Game {
    fn new(level_idx: usize) -> Self {
        let level_idx = level_idx % LEVELS.len();
        Game {
            level_idx,
            world: World::new(&LEVELS[level_idx]),
            attempts_used: 0,
            history: Vec::new(),
            selection: Vec::new(),
            sim_guess: ['A'; SOLUTION_LEN],
            phase: Phase::Planning,
            next_phase: Phase::Planning,
            sim: Sim { next: 0, timer: 0.0, post: 0.0 },
            reset_timer: 0.0,
            reveal_row: None,
            reveal_t: 0.0,
            select_t: 99.0,
            last_letter: None,
            particles: Vec::new(),
            flashes: Vec::new(),
            particle_seed: 0x5EED_1234,
            toast_msg: String::new(),
            toast_timer: 0.0,
            shake: 0.0,
            acc: 0.0,
            light_tex: [
                make_light_texture(11),
                make_light_texture(23),
                make_light_texture(37),
            ],
        }
    }

    fn level(&self) -> &'static Level {
        &LEVELS[self.level_idx]
    }


    fn toast(&mut self, msg: &str) {
        self.toast_msg = msg.to_string();
        self.toast_timer = 1.7;
        
    }

    fn push_letter(&mut self, ch: char) {
        if self.phase != Phase::Planning {
            return;
        }
        if self.selection.len() >= SOLUTION_LEN {
            self.toast("Ya cargaste 5 bloques");
            return;
        }
        if self.selection.contains(&ch) {
            self.toast("Ese bloque ya esta cargado");
            return;
        }
        self.selection.push(ch);
        self.select_t = 0.0;
        self.last_letter = Some(ch);
        
    }

    fn pop_letter(&mut self) {
        if self.phase == Phase::Planning {
            self.selection.pop();
        }
    }

    fn try_submit(&mut self) {
        if self.phase != Phase::Planning {
            return;
        }
        if self.selection.len() != SOLUTION_LEN {
            self.toast("Elige exactamente 5 bloques");
            return;
        }
        for (i, ch) in self.selection.iter().enumerate() {
            self.sim_guess[i] = *ch;
        }
        self.attempts_used += 1;
        self.phase = Phase::Simulating;
        self.sim = Sim { next: 0, timer: EXPLODE_DELAY, post: SIM_TAIL };
        self.acc = 0.0;
        
    }

    fn finish_attempt(&mut self) {
        let feedback = evaluate(&self.sim_guess, &self.level().solution);
        self.history.push((self.sim_guess, feedback));
        self.selection.clear();

        self.reveal_row = Some(self.history.len() - 1);
        self.reveal_t = 0.0;

        self.next_phase = if feedback.iter().all(|f| *f == Feedback::Green) {
            Phase::Won
        } else if self.attempts_used >= MAX_ATTEMPTS {
            Phase::Lost
        } else {
            Phase::Resetting
        };
        self.phase = Phase::Revealing;
    }

    fn rebuild(&mut self) {
        self.world = World::new(self.level());
        self.particles.clear();
        self.flashes.clear();
        self.acc = 0.0;
        self.shake = 0.0;
    }

    fn restart(&mut self) {
        self.attempts_used = 0;
        self.history.clear();
        self.selection.clear();
        self.phase = Phase::Planning;
        self.next_phase = Phase::Planning;
        self.reveal_row = None;
        self.reveal_t = 0.0;
        self.select_t = 99.0;
        self.last_letter = None;
        self.rebuild();
    }

    fn load_level(&mut self, idx: usize) {
        if idx >= LEVELS.len() {
            return;
        }
        self.level_idx = idx;
        self.restart();
    }

    fn spawn_explosion(&mut self, wx: f32, wy: f32) {
        let (sx, sy) = w2s(wx, wy);
        self.flashes.push(Flash { x: sx, y: sy, t: 0.0, max: 0.5 });
        self.shake = 9.0;

        for i in 0..26u32 {
            let h1 = hash01(self.particle_seed.wrapping_add(i * 13 + 1));
            let h2 = hash01(self.particle_seed.wrapping_add(i * 29 + 7));
            let h3 = hash01(self.particle_seed.wrapping_add(i * 53 + 11));
            let angle = h1 * std::f32::consts::TAU;
            let speed = 60.0 + h2 * 420.0;
            let (vy_bias, color, size, life) = match i % 3 {
                0 => (
                    -90.0,
                    C_ACCENT_HI,
                    2.0 + h3 * 2.5,
                    0.35 + h3 * 0.35,
                ),
                1 => (
                    -40.0,
                    Color { r: 1.0, g: 0.86, b: 0.46, a: 1.0 },
                    1.5 + h3 * 2.0,
                    0.5 + h3 * 0.4,
                ),
                _ => (
                    -30.0,
                    Color { r: 0.46, g: 0.43, b: 0.41, a: 1.0 },
                    3.0 + h3 * 4.0,
                    0.9 + h3 * 0.8,
                ),
            };
            self.particles.push(Particle {
                x: sx,
                y: sy,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed + vy_bias,
                life,
                max_life: life,
                size,
                color,
            });
        }
        self.particle_seed = self.particle_seed.wrapping_add(7919);
    }


    fn update(&mut self, dt: f32) {
        self.handle_input(dt);

        if self.toast_timer > 0.0 {
            self.toast_timer -= dt;
        }
        self.shake = (self.shake - dt * 45.0).max(0.0);
        self.select_t += dt;
        if self.reveal_row.is_some() {
            self.reveal_t += dt;
        }

        for p in &mut self.particles {
            p.vy += 1000.0 * dt;
            p.vx *= 1.0 - (1.4 * dt).min(0.5);
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);
        for f in &mut self.flashes {
            f.t += dt;
        }
        self.flashes.retain(|f| f.t < f.max);

        match self.phase {
            Phase::Simulating => {
                self.acc = (self.acc + dt).min(0.25);
                while self.acc >= FIXED_DT {
                    self.world.step();
                    self.acc -= FIXED_DT;
                }

                if self.sim.next < SOLUTION_LEN {
                    self.sim.timer -= dt;
                    if self.sim.timer <= 0.0 {
                        let ch = self.sim_guess[self.sim.next];
                        if let Some((wx, wy)) = self.world.detonate(ch) {
                            self.spawn_explosion(wx, wy);
                        }
                        self.sim.next += 1;
                        self.sim.timer = EXPLODE_INTERVAL;
                    }
                } else {
                    self.sim.post -= dt;
                    if self.sim.post <= 0.0 {
                        
                        self.finish_attempt();
                    }
                }
            }
            Phase::Revealing => {
                if self.reveal_t >= 0.85 {
                    self.phase = self.next_phase;
                    self.reveal_row = None;
                    match self.phase {
                        Phase::Won => {}
                        Phase::Lost => {}
                        Phase::Resetting => { self.reset_timer = RESET_TIME; }
                        _ => {}
                    }
                }
            }
            Phase::Resetting => {
                self.reset_timer -= dt;
                if self.reset_timer <= 0.0 {
                    self.rebuild();
                    self.phase = Phase::Planning;
                    self.selection.clear();
                }
            }
            _ => {}
        }
    }

    fn handle_input(&mut self, _dt: f32) {
        for (i, key) in [
            KeyCode::Key1,
            KeyCode::Key2,
            KeyCode::Key3,
            KeyCode::Key4,
            KeyCode::Key5,
            KeyCode::Key6,
        ]
        .iter()
        .enumerate()
        {
            if is_key_pressed(*key) && i < LEVELS.len() {
                self.load_level(i);
                return;
            }
        }
        if is_key_pressed(KeyCode::R) {
            self.restart();
            return;
        }

        if is_key_pressed(KeyCode::Right) {
            let n = (self.level_idx + 1) % LEVELS.len();
            self.load_level(n);
            return;
        }
        if is_key_pressed(KeyCode::Left) {
            let n = (self.level_idx + LEVELS.len() - 1) % LEVELS.len();
            self.load_level(n);
            return;
        }

        let (mx, my) = mouse_view();
        let click = is_mouse_button_pressed(MouseButton::Left);

        if click {
            if inside(SEL_LEFT_X, SEL_Y, SEL_BTN_W, SEL_H, mx, my) {
                let n = (self.level_idx + LEVELS.len() - 1) % LEVELS.len();
                self.load_level(n);
                return;
            }
            if inside(SEL_RIGHT_X, SEL_Y, SEL_BTN_W, SEL_H, mx, my) {
                let n = (self.level_idx + 1) % LEVELS.len();
                self.load_level(n);
                return;
            }
        }

        if click && inside(RETRY_X, RETRY_Y, RETRY_W, RETRY_H, mx, my) {
            if self.phase == Phase::Won || self.phase == Phase::Lost {
                self.restart();
                return;
            }
        }

        match self.phase {
            Phase::Planning => {
                const KEYS: [(KeyCode, char); 8] = [
                    (KeyCode::A, 'A'),
                    (KeyCode::B, 'B'),
                    (KeyCode::C, 'C'),
                    (KeyCode::D, 'D'),
                    (KeyCode::E, 'E'),
                    (KeyCode::F, 'F'),
                    (KeyCode::G, 'G'),
                    (KeyCode::H, 'H'),
                ];
                for (key, ch) in KEYS {
                    if is_key_pressed(key) {
                        self.push_letter(ch);
                    }
                }
                if is_key_pressed(KeyCode::Backspace) {
                    self.pop_letter();
                }
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
                    self.try_submit();
                }

                if click {
                    for i in 0..8usize {
                        let (bx, by, bw, bh) = letter_rect(i);
                        if inside(bx, by, bw, bh, mx, my) {
                            let ch = (b'A' + i as u8) as char;
                            self.push_letter(ch);
                        }
                    }
                    if inside(BTN_BORRAR_X, ACTION_Y, BTN_BORRAR_W, ACTION_H, mx, my) {
                        self.pop_letter();
                    }
                    if inside(BTN_DETONAR_X, ACTION_Y, BTN_DETONAR_W, ACTION_H, mx, my) {
                        self.try_submit();
                    }
                }
            }
            Phase::Won | Phase::Lost => {
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
                    self.restart();
                }
            }
            Phase::Simulating | Phase::Revealing | Phase::Resetting => {}
        }
    }


    fn draw(&self) {
        clear_background(C_BG);
        self.draw_sky();
        self.draw_ground();

        let shake = self.shake_offset();
        self.draw_blocks(shake);
        self.draw_particles(shake);
        self.draw_flashes(shake);

        self.draw_view_hud();
        self.draw_panel();
        self.draw_overlays();
    }

    fn shake_offset(&self) -> (f32, f32) {
        if self.shake <= 0.1 {
            return (0.0, 0.0);
        }
        let t = (get_time() * 1000.0) as u32;
        let ox = (hash01(t) * 2.0 - 1.0) * self.shake;
        let oy = (hash01(t.wrapping_add(999)) * 2.0 - 1.0) * self.shake;
        (ox, oy)
    }

    fn draw_sky(&self) {
        let t = get_time() as f32;

        let bands = 32;
        for i in 0..bands {
            let k = i as f32 / (bands - 1) as f32;
            let c = Color {
                r: 0.05 + 0.075 * k,
                g: 0.07 + 0.10 * k,
                b: 0.10 + 0.13 * k,
                a: 1.0,
            };
            let y = i as f32 * (GROUND_SY / bands as f32);
            draw_rectangle(0.0, y, SCREEN_W, GROUND_SY / bands as f32 + 1.0, c);
        }

        let palette = [
            (0.55, 0.75, 1.00),
            (0.48, 0.86, 0.86),
            (0.72, 0.62, 1.00),
            (0.92, 0.68, 0.46),
            (0.44, 0.70, 1.00),
        ];
        for i in 0..5u32 {
            let phase = hash01(i + 301) * std::f32::consts::TAU;
            let speed = 0.025 + hash01(i + 302) * 0.095;
            let amp = 170.0 + hash01(i + 303) * 300.0;
            let base_x = 70.0 + hash01(i + 304) * (VIEW_W - 140.0);
            let x = base_x
                + (t * speed + phase).sin() * amp
                + (t * speed * 2.4 + phase * 1.7).cos() * 42.0;
            let y = 80.0
                + hash01(i + 305) * 280.0
                + (t * (0.05 + hash01(i + 306) * 0.08) + phase).cos() * 62.0;
            let r = (88.0 + hash01(i + 307) * 205.0
                + 20.0 * (t * 0.35 + phase).sin())
            .max(48.0);
            let pulse = 0.45
                + 0.55
                    * (t * (0.18 + hash01(i + 308) * 0.55) + phase * 2.1)
                        .sin()
                        .max(0.0);
            let (cr, cg, cb) = palette[i as usize % palette.len()];
            let a = (0.022 + hash01(i + 309) * 0.030) * pulse;
            let color = Color { r: cr, g: cg, b: cb, a };
            let tex = &self.light_tex[i as usize % self.light_tex.len()];
            draw_light(tex, x, y, r, color);
            draw_light(
                &self.light_tex[(i as usize + 1) % self.light_tex.len()],
                x + r * 0.28,
                y - r * 0.18,
                r * 0.78,
                Color { a: a * 0.9, ..color },
            );
            draw_light(
                &self.light_tex[(i as usize + 2) % self.light_tex.len()],
                x - r * 0.30,
                y + r * 0.20,
                r * 0.62,
                Color { a: a * 0.8, ..color },
            );
        }

        for i in 0..80u32 {
            let base_x = hash01(i * 3 + 1) * VIEW_W;
            let speed = 1.2 + hash01(i * 17 + 4) * 5.0;
            let x = (base_x + t * speed) % VIEW_W;
            let y = 12.0 + hash01(i * 7 + 5) * 300.0;
            let phase = hash01(i * 13 + 2) * std::f32::consts::TAU;
            let twinkle = 0.55 + 0.45 * (t * 1.3 + phase).sin();
            let base_a = 0.22 + hash01(i * 11 + 9) * 0.6;
            let r = 0.7 + hash01(i * 19 + 6) * 1.1;
            draw_circle(x, y, r, Color {
                r: 0.9,
                g: 0.93,
                b: 1.0,
                a: base_a * twinkle,
            });
        }

        draw_moon(t, &self.light_tex);

        for i in 0..4u32 {
            let w = 130.0 + hash01(i + 31) * 130.0;
            let y = 58.0 + i as f32 * 42.0 + hash01(i + 61) * 26.0;
            let speed = 4.0 + i as f32 * 2.5;
            let span = VIEW_W + w * 2.0;
            let x = ((hash01(i + 91) * span + t * speed) % span) - w;
            draw_cloud(x, y, w, 0.05 + hash01(i + 121) * 0.035);
        }

        draw_skyline(t, &self.light_tex);
        draw_street_lights(t, &self.light_tex);
    }

    fn draw_ground(&self) {
        draw_rectangle(
            0.0,
            GROUND_SY,
            SCREEN_W,
            SCREEN_H - GROUND_SY,
            Color { r: 0.20, g: 0.15, b: 0.11, a: 1.0 },
        );
        draw_rectangle(
            0.0,
            GROUND_SY - 2.0,
            SCREEN_W,
            10.0,
            Color { r: 0.16, g: 0.17, b: 0.19, a: 1.0 },
        );
        let mut x = 0.0;
        while x < SCREEN_W {
            draw_rectangle(x, GROUND_SY + 10.0, 26.0, 6.0, Color {
                r: 0.85,
                g: 0.70,
                b: 0.20,
                a: 0.55,
            });
            x += 52.0;
        }
        for i in 0..24u32 {
            let px = hash01(i + 700) * VIEW_W;
            let py = GROUND_SY + 24.0 + hash01(i + 800) * 84.0;
            draw_circle(px, py, 1.0 + hash01(i + 900) * 2.0, Color {
                r: 0.28,
                g: 0.22,
                b: 0.16,
                a: 0.8,
            });
        }
    }

    fn draw_blocks(&self, shake: (f32, f32)) {
        for block in &self.world.blocks {
            if !block.alive {
                continue;
            }
            let Some(body) = self.world.bodies.get(block.handle) else {
                continue;
            };
            let p = body.translation();
            let angle = body.rotation().angle();
            let (sx, sy) = w2s(p.x, p.y);
            let (sx, sy) = (sx + shake.0, sy + shake.1);

            let w = block.hx * 2.0 * PPM;
            let h = block.hy * 2.0 * PPM;
            let k = 0.90 + block.tint * 0.16;
            let fill = Color { r: 0.72 * k, g: 0.55 * k, b: 0.33 * k, a: 1.0 };
            let border = Color { r: 0.36 * k, g: 0.23 * k, b: 0.12 * k, a: 0.95 };

            let params = DrawRectangleParams {
                offset: vec2(0.5, 0.5),
                rotation: angle,
                color: fill,
            };
            draw_rectangle_ex(sx, sy, w, h, params);
            draw_rectangle_lines_ex(sx, sy, w, h, 2.5, DrawRectangleParams {
                offset: vec2(0.5, 0.5),
                rotation: angle,
                color: border,
            });

            let letter = block.id.to_string();
            let size = (h * 0.45).clamp(18.0, 40.0);
            text_center(&letter, sx, sy + size * 0.36, size, C_INK);
        }
    }

    fn draw_particles(&self, shake: (f32, f32)) {
        for p in &self.particles {
            let a = (p.life / p.max_life).clamp(0.0, 1.0);
            let mut c = p.color;
            c.a = a;
            draw_circle(
                p.x + shake.0,
                p.y + shake.1,
                p.size * (0.35 + 0.65 * a),
                c,
            );
        }
    }

    fn draw_flashes(&self, shake: (f32, f32)) {
        let mut lit = 0.0;
        for f in &self.flashes {
            let t = (f.t / f.max).clamp(0.0, 1.0);
            let alpha = (1.0 - t).powf(1.6);
            lit += alpha * alpha;
            let radius = 16.0 + 95.0 * t;
            let tex = &self.light_tex[(f.x as u32 as usize + f.y as u32 as usize) % self.light_tex.len()];
            draw_light(
                tex,
                f.x + shake.0,
                f.y + shake.1,
                radius * 2.2,
                Color { r: 1.0, g: 0.80, b: 0.45, a: 0.42 * alpha },
            );
            draw_circle(f.x + shake.0, f.y + shake.1, radius, Color {
                r: 1.0,
                g: 0.88,
                b: 0.55,
                a: 0.28 * alpha,
            });
            draw_circle(f.x + shake.0, f.y + shake.1, radius * 0.55, Color {
                r: 1.0,
                g: 0.96,
                b: 0.82,
                a: 0.5 * alpha,
            });
            draw_circle_lines(
                f.x + shake.0,
                f.y + shake.1,
                radius,
                3.0,
                Color { r: 1.0, g: 0.85, b: 0.45, a: 0.85 * alpha },
            );
        }

        let lit = lit.min(1.0);
        if lit > 0.01 {
            draw_rectangle(0.0, 0.0, VIEW_W, SCREEN_H, Color {
                r: 1.0,
                g: 0.74,
                b: 0.40,
                a: 0.055 * lit,
            });
        }
    }

    fn draw_view_hud(&self) {
        let size = 44.0;
        let demi = "DEMOLI";
        let word = "WORD";
        let demi_w = measure_text(demi, None, size as u16, 1.0).width;
        let shadow = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.5 };
        draw_text(demi, 30.5, 56.5, size, shadow);
        draw_text(word, 30.5 + demi_w, 56.5, size, shadow);
        draw_text(demi, 28.0, 54.0, size, C_WHITE);
        draw_text(word, 28.0 + demi_w, 54.0, size, C_ACCENT);

        draw_text(self.level().name, 30.0, 86.0, 18.0, C_DIM);

        let puzzle = format!("PUZZLE #{}", days_since_epoch());
        text_right(&puzzle, VIEW_W - 30.0, 56.0, 18.0, C_DIM);
        let lvl = format!("{}{} {}{}", tr("level"), self.level_idx + 1, tr("of"), LEVELS.len());
        text_right(&lvl, VIEW_W - 30.0, 86.0, 18.0, C_FAINT);

        if self.phase == Phase::Planning {
            if self.selection.len() == SOLUTION_LEN {
                text_center(
                    "LISTO PARA DETONAR",
                    VIEW_W / 2.0,
                    560.0,
                    21.0,
                    C_ACCENT,
                );
            } else {
                let msg = format!("{}{}", self.selection.len(), tr("blocks_loaded"));
                draw_text(&msg, 30.0, 560.0, 19.0, C_DIM);
            }
        }

        if self.phase == Phase::Simulating {
            let label = if self.sim.next < SOLUTION_LEN {
                format!("{}{} / {}", tr("detonation"), self.sim.next + 1, SOLUTION_LEN)
            } else {
                tr("evaluating").to_string()
            };
            let w = measure_text(&label, None, 20, 1.0).width + 40.0;
            draw_rectangle(VIEW_W / 2.0 - w / 2.0, 112.0, w, 38.0, Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.62,
            });
            draw_rectangle_lines(
                VIEW_W / 2.0 - w / 2.0,
                112.0,
                w,
                38.0,
                2.0,
                Color { r: C_ACCENT.r, g: C_ACCENT.g, b: C_ACCENT.b, a: 0.7 },
            );
            text_center(&label, VIEW_W / 2.0, 138.0, 20.0, C_ACCENT_HI);
        }

        if self.toast_timer > 0.0 && !self.toast_msg.is_empty() {
            let alpha = (self.toast_timer / 0.5).min(1.0);
            let w = measure_text(&self.toast_msg, None, 18, 1.0).width + 36.0;
            draw_rectangle(VIEW_W / 2.0 - w / 2.0, 522.0, w, 34.0, Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.7 * alpha,
            });
            draw_rectangle_lines(
                VIEW_W / 2.0 - w / 2.0,
                522.0,
                w,
                34.0,
                1.5,
                Color { r: C_ACCENT.r, g: C_ACCENT.g, b: C_ACCENT.b, a: 0.5 * alpha },
            );
            text_center(
                &self.toast_msg,
                VIEW_W / 2.0,
                545.0,
                18.0,
                Color { r: C_WHITE.r, g: C_WHITE.g, b: C_WHITE.b, a: alpha },
            );
        }
    }

    fn draw_panel(&self) {
        draw_rectangle(
            PANEL_CARD_X + 7.0,
            PANEL_CARD_Y + 9.0,
            PANEL_CARD_W,
            PANEL_CARD_H,
            Color { r: 0.0, g: 0.0, b: 0.0, a: 0.28 },
        );
        draw_rectangle(
            PANEL_CARD_X + 3.0,
            PANEL_CARD_Y + 4.0,
            PANEL_CARD_W,
            PANEL_CARD_H,
            Color { r: 0.0, g: 0.0, b: 0.0, a: 0.20 },
        );
        draw_round_rect(
            PANEL_CARD_X - 2.0,
            PANEL_CARD_Y - 2.0,
            PANEL_CARD_W + 4.0,
            PANEL_CARD_H + 4.0,
            PANEL_RADIUS + 2.0,
            C_PANEL_BORDER,
        );
        draw_round_rect(
            PANEL_CARD_X,
            PANEL_CARD_Y,
            PANEL_CARD_W,
            PANEL_CARD_H,
            PANEL_RADIUS,
            C_PANEL,
        );
        draw_rectangle(
            PANEL_CARD_X + 20.0,
            PANEL_CARD_Y + 1.0,
            PANEL_CARD_W - 40.0,
            1.0,
            Color { r: 0.34, g: 0.40, b: 0.48, a: 0.35 },
        );

        let cx = PANEL_CARD_X + PANEL_CARD_W * 0.5;

        let t = get_time() as f32;
        let (mx, my) = mouse_view();

        draw_text(tr("attempts_label"), PANEL_X + 20.0, 52.0, 26.0, C_WHITE);
        let n = (self.attempts_used
            + if self.phase == Phase::Planning { 1 } else { 0 })
        .min(MAX_ATTEMPTS);
        text_right(
            &format!("{}/{}", n.max(1), MAX_ATTEMPTS),
            1262.0,
            52.0,
            20.0,
            C_DIM,
        );

        let hover_left = inside(SEL_LEFT_X, SEL_Y, SEL_BTN_W, SEL_H, mx, my);
        let hover_right = inside(SEL_RIGHT_X, SEL_Y, SEL_BTN_W, SEL_H, mx, my);
        draw_button(
            SEL_LEFT_X,
            SEL_Y,
            SEL_BTN_W,
            SEL_H,
            "<",
            if hover_left {
                Color { r: 0.22, g: 0.26, b: 0.32, a: 1.0 }
            } else {
                C_BTN_DARK
            },
            if hover_left { C_ACCENT } else { C_BTN_BORDER },
            C_WHITE,
            18.0,
        );
        draw_button(
            SEL_RIGHT_X,
            SEL_Y,
            SEL_BTN_W,
            SEL_H,
            ">",
            if hover_right {
                Color { r: 0.22, g: 0.26, b: 0.32, a: 1.0 }
            } else {
                C_BTN_DARK
            },
            if hover_right { C_ACCENT } else { C_BTN_BORDER },
            C_WHITE,
            18.0,
        );
        let sel = format!(
            "{}  -  {} / {}",
            self.level().name,
            self.level_idx + 1,
            LEVELS.len()
        );
        text_center(&sel, cx, SEL_Y + 20.0, 15.0, C_ACCENT_HI);

        let pulse = 0.5 + 0.5 * (t * 5.0).sin();
        for row in 0..MAX_ATTEMPTS {
            let cy = GY0 + row as f32 * ROW_PITCH + CELL * 0.5;
            for col in 0..SOLUTION_LEN {
                let cell_cx = GX0 + col as f32 * (CELL + CELL_GAP) + CELL * 0.5;

                if row < self.history.len() {
                    let (guess, feedback) = self.history[row];
                    let (fill, border) = feedback_style(feedback[col]);

                    let p = if self.reveal_row == Some(row) {
                        reveal_progress(self.reveal_t, col)
                    } else {
                        1.0
                    };

                    if p <= 0.0 {
                        draw_cell(cell_cx, cy, CELL, C_CELL_EMPTY, C_CELL_EMPTY_BORDER);
                        continue;
                    }

                    let pop = 1.0 + 0.26 * (1.0 - p).powf(2.0);
                    let size = CELL * pop;
                    draw_cell(cell_cx, cy, size, fill, border);

                    if p < 1.0 {
                        draw_cell_glow(
                            cell_cx,
                            cy,
                            size,
                            2.0 + 9.0 * (1.0 - p),
                            Color { r: border.r, g: border.g, b: border.b, a: 0.9 * (1.0 - p) },
                            2.5,
                        );
                        let half = size * 0.5;
                        draw_rectangle(
                            cell_cx - half,
                            cy - half,
                            size,
                            size,
                            Color { r: 1.0, g: 1.0, b: 1.0, a: 0.38 * (1.0 - p).powf(1.4) },
                        );
                    }

                    draw_cell_letter(cell_cx, cy, size, guess[col], C_WHITE);
                } else if row == self.history.len()
                    && (self.phase == Phase::Planning || self.phase == Phase::Simulating)
                {
                    let border = Color {
                        r: C_ACCENT.r,
                        g: C_ACCENT.g,
                        b: C_ACCENT.b,
                        a: 0.42 + 0.40 * pulse,
                    };
                    draw_cell(cell_cx, cy, CELL, C_CELL_EMPTY, border);

                    if col < self.selection.len() {
                        let is_last = col + 1 == self.selection.len();
                        let mut size = CELL;
                        if is_last && self.select_t < 0.22 {
                            let q = (self.select_t / 0.22).clamp(0.0, 1.0);
                            size = CELL * (1.0 + 0.16 * (1.0 - q));
                            draw_cell_glow(
                                cell_cx,
                                cy,
                                size,
                                2.0 + 7.0 * (1.0 - q),
                                Color { r: C_ACCENT.r, g: C_ACCENT.g, b: C_ACCENT.b, a: 0.9 * (1.0 - q) },
                                2.0,
                            );
                        }
                        draw_cell_letter(cell_cx, cy, size, self.selection[col], C_WHITE);
                    }
                } else {
                    draw_cell(cell_cx, cy, CELL, C_CELL_EMPTY, C_CELL_EMPTY_BORDER);
                }
            }
        }

        draw_rectangle(
            PANEL_CARD_X + 16.0,
            528.0,
            PANEL_CARD_W - 32.0,
            2.0,
            C_DIVIDER,
        );

        for i in 0..8usize {
            let (bx, by, bw, bh) = letter_rect(i);
            let ch = (b'A' + i as u8) as char;
            let used = self.selection.contains(&ch);
            let hover = inside(bx, by, bw, bh, mx, my) && !used && self.phase == Phase::Planning;

            let fill = if used {
                Color { r: 0.09, g: 0.11, b: 0.14, a: 1.0 }
            } else if hover {
                Color { r: 0.22, g: 0.26, b: 0.32, a: 1.0 }
            } else {
                C_BTN_DARK
            };
            let border = if hover { C_ACCENT } else { C_BTN_BORDER };
            let txt = if used { C_FAINT } else { C_WHITE };
            draw_button(bx, by, bw, bh, &ch.to_string(), fill, border, txt, 22.0);

            if used && self.last_letter == Some(ch) && self.select_t < 0.3 {
                let q = (self.select_t / 0.3).clamp(0.0, 1.0);
                let pad = 2.0 + 5.0 * (1.0 - q);
                draw_rectangle_lines(
                    bx - pad,
                    by - pad,
                    bw + pad * 2.0,
                    bh + pad * 2.0,
                    2.0,
                    Color { r: C_ACCENT.r, g: C_ACCENT.g, b: C_ACCENT.b, a: 0.9 * (1.0 - q) },
                );
            }
        }

        let can_detonate =
            self.phase == Phase::Planning && self.selection.len() == SOLUTION_LEN;
        let hover_borrar = inside(BTN_BORRAR_X, ACTION_Y, BTN_BORRAR_W, ACTION_H, mx, my);
        draw_button(
            BTN_BORRAR_X,
            ACTION_Y,
            BTN_BORRAR_W,
            ACTION_H,
            "BORRAR",
            if hover_borrar {
                Color { r: 0.21, g: 0.25, b: 0.31, a: 1.0 }
            } else {
                C_BTN_DARK
            },
            if hover_borrar { C_ACCENT } else { C_BTN_BORDER },
            C_WHITE,
            20.0,
        );

        let hover_det = inside(BTN_DETONAR_X, ACTION_Y, BTN_DETONAR_W, ACTION_H, mx, my);
        let (fill, txt, border) = if can_detonate {
            let fill = if hover_det { C_ACCENT_HI } else { C_ACCENT };
            (fill, Color { r: 0.10, g: 0.07, b: 0.03, a: 1.0 }, C_ACCENT_HI)
        } else {
            (C_BTN_DARK, C_FAINT, C_BTN_BORDER)
        };
        draw_button(
            BTN_DETONAR_X,
            ACTION_Y,
            BTN_DETONAR_W,
            ACTION_H,
            "DETONAR",
            fill,
            border,
            txt,
            24.0,
        );

        if can_detonate {
            let glow = 0.16 + 0.22 * (t * 6.0).sin().max(0.0);
            draw_rectangle_lines(
                BTN_DETONAR_X - 3.0,
                ACTION_Y - 3.0,
                BTN_DETONAR_W + 6.0,
                ACTION_H + 6.0,
                2.0,
                Color { r: C_ACCENT.r, g: C_ACCENT.g, b: C_ACCENT.b, a: glow },
            );
        }
    }

    fn draw_overlays(&self) {
        match self.phase {
            Phase::Resetting => {
                let t = 1.0 - (self.reset_timer / RESET_TIME).clamp(0.0, 1.0);
                draw_rectangle(0.0, 0.0, VIEW_W, SCREEN_H, Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.42,
                });
                text_center(
                    "RECONSTRUYENDO LA TORRE",
                    VIEW_W / 2.0,
                    336.0,
                    26.0,
                    C_WHITE,
                );
                draw_rectangle(VIEW_W / 2.0 - 140.0, 356.0, 280.0, 10.0, C_CELL_EMPTY);
                draw_rectangle(VIEW_W / 2.0 - 140.0, 356.0, 280.0 * t, 10.0, C_ACCENT);
            }
            Phase::Won | Phase::Lost => {
                draw_rectangle(0.0, 0.0, VIEW_W, SCREEN_H, Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.55,
                });

                if self.phase == Phase::Won {
                    text_center(tr("perfect_collapse"), VIEW_W / 2.0, 296.0, 52.0, C_ACCENT);
                    let sub = format!(
                        "Derribaste la estructura en {} intento{}.",
                        self.attempts_used,
                        if self.attempts_used == 1 { "" } else { "s" }
                    );
                    text_center(&sub, VIEW_W / 2.0, 336.0, 22.0, C_WHITE);
                } else {
                    text_center(tr("out_of_charges"), VIEW_W / 2.0, 296.0, 46.0, C_RED);
                    text_center(
                        "La secuencia correcta era:",
                        VIEW_W / 2.0,
                        338.0,
                        22.0,
                        C_WHITE,
                    );
                }

                let chip = 54.0;
                let gap = 10.0;
                let total = SOLUTION_LEN as f32 * chip + (SOLUTION_LEN - 1) as f32 * gap;
                let x0 = VIEW_W / 2.0 - total / 2.0;
                for (i, ch) in self.level().solution.iter().enumerate() {
                    let x = x0 + i as f32 * (chip + gap);
                    draw_rectangle(x, 372.0, chip, chip, C_GREEN);
                    draw_rectangle_lines(x, 372.0, chip, chip, 2.0, C_GREEN_BORDER);
                    text_center(
                        &ch.to_string(),
                        x + chip / 2.0,
                        372.0 + chip * 0.68,
                        28.0,
                        C_WHITE,
                    );
                }

                let hover_retry = {
                    let (mx, my) = mouse_view();
                    inside(RETRY_X, RETRY_Y, RETRY_W, RETRY_H, mx, my)
                };
                draw_button(
                    RETRY_X,
                    RETRY_Y,
                    RETRY_W,
                    RETRY_H,
                    "OTRA VEZ",
                    if hover_retry { C_ACCENT_HI } else { C_ACCENT },
                    C_ACCENT_HI,
                    Color { r: 0.10, g: 0.07, b: 0.03, a: 1.0 },
                    22.0,
                );
            }
            _ => {}
        }
    }
}


fn evaluate(guess: &[char; SOLUTION_LEN], secret: &[char; SOLUTION_LEN]) -> [Feedback; SOLUTION_LEN] {
    let mut out = [Feedback::Gray; SOLUTION_LEN];
    for i in 0..SOLUTION_LEN {
        if guess[i] == secret[i] {
            out[i] = Feedback::Green;
        }
    }
    for i in 0..SOLUTION_LEN {
        if out[i] == Feedback::Green {
            continue;
        }
        if secret.contains(&guess[i]) {
            out[i] = Feedback::Yellow;
        }
    }
    out
}

fn feedback_style(fb: Feedback) -> (Color, Color) {
    match fb {
        Feedback::Green => (C_GREEN, C_GREEN_BORDER),
        Feedback::Yellow => (C_YELLOW, C_YELLOW_BORDER),
        Feedback::Gray => (C_GRAY_CELL, C_GRAY_BORDER),
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Demoliword".to_owned(),
        window_width: 1600,
        window_height: 900,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4,
        ..Default::default()
    }
}

fn view_scale() -> f32 {
    (screen_width() / SCREEN_W).min(screen_height() / SCREEN_H)
}

fn screen_to_view(mx: f32, my: f32) -> (f32, f32) {
    let s = view_scale();
    let ox = (screen_width() - SCREEN_W * s) * 0.5;
    let oy = (screen_height() - SCREEN_H * s) * 0.5;
    ((mx - ox) / s, (my - oy) / s)
}

fn mouse_view() -> (f32, f32) {
    let (mx, my) = mouse_position();
    screen_to_view(mx, my)
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new(get_daily_level_index());
    let mut fullscreen = false;
    loop {
        if is_key_pressed(KeyCode::F11) {
            fullscreen = !fullscreen;
            set_fullscreen(fullscreen);
        }

        let dt = get_frame_time().min(0.05);
        game.update(dt);

        clear_background(C_BG);
        let s = view_scale();
        let camera = Camera2D {
            target: vec2(SCREEN_W * 0.5, SCREEN_H * 0.5),
            zoom: vec2(
                2.0 * s / screen_width().max(1.0),
                2.0 * s / screen_height().max(1.0),
            ),
            ..Default::default()
        };
        set_camera(&camera);
        game.draw();
        set_default_camera();

        next_frame().await;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn settle(world: &mut World, seconds: f32) {
        let steps = (seconds / FIXED_DT) as usize;
        for _ in 0..steps {
            world.step();
        }
    }

    #[test]
    fn evaluacion_wordle() {
        let secret = ['B', 'E', 'A', 'H', 'C'];
        assert_eq!(evaluate(&secret, &secret), [Feedback::Green; SOLUTION_LEN]);

        let guess = ['E', 'B', 'A', 'X', 'C'];
        let fb = evaluate(&guess, &secret);
        assert_eq!(fb[0], Feedback::Yellow);
        assert_eq!(fb[1], Feedback::Yellow);
        assert_eq!(fb[2], Feedback::Green);
        assert_eq!(fb[3], Feedback::Gray);
        assert_eq!(fb[4], Feedback::Green);
    }

    #[test]
    fn la_torre_se_mantiene_en_pie() {
        for level in LEVELS {
            let mut world = World::new(level);
            settle(&mut world, 2.0);
            for block in &world.blocks {
                let body = world.bodies.get(block.handle).expect("el bloque sigue vivo");
                let p = body.translation();
                assert!(
                    p.y > 0.2,
                    "{}: el bloque {} se hundio (y = {})",
                    level.name,
                    block.id,
                    p.y
                );
                assert!(
                    body.rotation().angle().abs() < 0.35,
                    "{}: el bloque {} giro demasiado",
                    level.name,
                    block.id
                );
            }
        }
    }

    #[test]
    fn detonar_la_solucion_elimina_los_bloques() {
        for level in LEVELS {
            let mut world = World::new(level);
            settle(&mut world, 1.0);
            for ch in level.solution {
                assert!(world.detonate(ch).is_some(), "no exploto el bloque {}", ch);
                settle(&mut world, EXPLODE_INTERVAL);
            }
            settle(&mut world, SIM_TAIL);

            let alive = world.blocks.iter().filter(|b| b.alive).count();
            assert_eq!(alive, level.blocks.len() - SOLUTION_LEN);
            for block in world.blocks.iter().filter(|b| b.alive) {
                if let Some(body) = world.bodies.get(block.handle) {
                    assert!(
                        body.translation().y.is_finite(),
                        "{}: NaN en el bloque {}",
                        level.name,
                        block.id
                    );
                }
            }
        }
    }
}
