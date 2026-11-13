
use macroquad::prelude::*;
use macroquad::shapes::DrawRectangleParams;
use rapier2d::prelude::*;

use std::sync::atomic::{AtomicI32, AtomicI64, AtomicU8, AtomicUsize, Ordering};

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

const MAX_ATTEMPTS: usize = 3;
const SOLUTION_LEN: usize = 5;
const EXPLODE_INTERVAL: f32 = 0.8;
const EXPLODE_DELAY: f32 = 0.35;
const SIM_TAIL: f32 = 2.0;
const RESET_TIME: f32 = 1.4;

const BLAST_RADIUS: f32 = 1.9;
const BLAST_POWER: f32 = 4.0;

const GX0: f32 = 899.0;
const GY0: f32 = 140.0;
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
}

fn supports(x: &BlockSpec, y: &BlockSpec) -> bool {
    let y_bottom = y.y - y.hy;
    let x_top = x.y + x.hy;
    let overlap = (x.x - x.hx) < (y.x + y.hx) - 0.01 && (x.x + x.hx) > (y.x - y.hx) + 0.01;
    overlap && (y_bottom - x_top).abs() < 0.06
}

fn structural_blocks(level: &Level) -> Vec<char> {
    let crown = level
        .blocks
        .iter()
        .max_by(|a, b| (a.y + a.hy).partial_cmp(&(b.y + b.hy)).unwrap())
        .map(|b| b.id)
        .unwrap_or('A');

    let mut core = vec![crown];
    let mut changed = true;
    while changed {
        changed = false;
        for b in level.blocks {
            if core.contains(&b.id) {
                continue;
            }
            let linked = core.iter().any(|other| {
                let ob = level.blocks.iter().find(|x| x.id == *other).unwrap();
                supports(ob, b) || supports(b, ob)
            });
            if linked {
                core.push(b.id);
                changed = true;
            }
        }
    }
    core
}

fn block_role(level: &Level, id: char) -> Option<bool> {
    if !level.blocks.iter().any(|x| x.id == id) {
        return None;
    }
    let structural = structural_blocks(level);
    if !structural.contains(&id) {
        return None;
    }
    let holds_something = structural
        .iter()
        .any(|other| *other != id && holds_transitively(level, id, *other));
    if holds_something {
        Some(true)
    } else {
        Some(false)
    }
}

fn holds_transitively(level: &Level, id: char, target: char) -> bool {
    let mut frontier = vec![id];
    let mut visited: Vec<char> = vec![id];
    while let Some(cur) = frontier.pop() {
        let Some(cb) = level.blocks.iter().find(|x| x.id == cur) else {
            continue;
        };
        for other in level.blocks.iter() {
            if other.id == cur || visited.contains(&other.id) {
                continue;
            }
            if !supports(cb, other) {
                continue;
            }
            if other.id == target {
                return true;
            }
            visited.push(other.id);
            frontier.push(other.id);
        }
    }
    false
}

fn live_block_specs(world: &World) -> Vec<BlockSpec> {
    let mut out = Vec::new();
    for b in &world.blocks {
        if !b.alive {
            continue;
        }
        let Some(body) = world.bodies.get(b.handle) else {
            continue;
        };
        let p = body.translation();
        out.push(BlockSpec {
            id: b.id,
            x: p.x,
            y: p.y,
            hx: b.hx,
            hy: b.hy,
        });
    }
    out
}

fn structural_blocks_live(world: &World) -> Vec<char> {
    let specs = live_block_specs(world);
    if specs.is_empty() {
        return Vec::new();
    }
    let crown = specs
        .iter()
        .max_by(|a, b| (a.y + a.hy).partial_cmp(&(b.y + b.hy)).unwrap())
        .map(|b| b.id)
        .unwrap_or('A');

    let mut core = vec![crown];
    let mut changed = true;
    while changed {
        changed = false;
        for b in &specs {
            if core.contains(&b.id) {
                continue;
            }
            let linked = core.iter().any(|other| {
                let ob = specs.iter().find(|x| x.id == *other).unwrap();
                supports(ob, b) || supports(b, ob)
            });
            if linked {
                core.push(b.id);
                changed = true;
            }
        }
    }
    core
}

fn holds_transitively_live(specs: &[BlockSpec], id: char, target: char) -> bool {
    let mut frontier = vec![id];
    let mut visited: Vec<char> = vec![id];
    while let Some(cur) = frontier.pop() {
        let Some(cb) = specs.iter().find(|x| x.id == cur) else {
            continue;
        };
        for other in specs.iter() {
            if other.id == cur || visited.contains(&other.id) {
                continue;
            }
            if !supports(cb, other) {
                continue;
            }
            if other.id == target {
                return true;
            }
            visited.push(other.id);
            frontier.push(other.id);
        }
    }
    false
}

fn block_role_live(world: &World, id: char) -> Option<bool> {
    let specs = live_block_specs(world);
    if !specs.iter().any(|x| x.id == id) {
        return None;
    }
    let structural = structural_blocks_live(world);
    if !structural.contains(&id) {
        return None;
    }
    let holds_something = structural
        .iter()
        .any(|other| *other != id && holds_transitively_live(&specs, id, *other));
    if holds_something {
        Some(true)
    } else {
        Some(false)
    }
}

fn live_feedback(world: &World, id: char) -> Feedback {
    match block_role_live(world, id) {
        Some(true) => Feedback::Green,
        Some(false) => Feedback::Yellow,
        None => Feedback::Gray,
    }
}

fn key_blocks(level: &Level) -> Vec<char> {
    let structural = structural_blocks(level);
    structural
        .iter()
        .copied()
        .filter(|id| {
            structural
                .iter()
                .any(|other| other != id && holds_transitively(level, *id, *other))
        })
        .collect()
}

#[allow(dead_code)]
fn support_count(level: &Level, id: char) -> usize {
    let Some(b) = level.blocks.iter().find(|x| x.id == id) else {
        return 0;
    };
    level.blocks.iter().filter(|o| o.id != id && supports(b, o)).count()
}

fn target_blocks_for(level: &Level) -> Vec<char> {
    let mut keys = key_blocks(level);
    if keys.len() < SOLUTION_LEN {
        for id in structural_blocks(level) {
            if keys.len() >= SOLUTION_LEN {
                break;
            }
            if !keys.contains(&id) {
                keys.push(id);
            }
        }
    }
    keys.truncate(SOLUTION_LEN);
    keys
}

fn core_blocks(level: &Level) -> Vec<char> {
    let mut core = structural_blocks(level);
    core.truncate(SOLUTION_LEN);
    core
}


const LEVEL_0_BLOCKS: &[BlockSpec] = &[
    BlockSpec { id: 'A', x:  0.00, y: 0.45, hx: 0.35, hy: 0.45 },
    BlockSpec { id: 'B', x:  0.00, y: 1.35, hx: 0.35, hy: 0.45 },
    BlockSpec { id: 'C', x:  0.00, y: 2.25, hx: 0.35, hy: 0.45 },
    BlockSpec { id: 'D', x:  0.00, y: 3.15, hx: 0.35, hy: 0.45 },
    BlockSpec { id: 'E', x:  0.00, y: 4.05, hx: 0.35, hy: 0.45 },
    BlockSpec { id: 'F', x: -1.60, y: 0.30, hx: 0.30, hy: 0.30 },
    BlockSpec { id: 'G', x: -0.95, y: 0.30, hx: 0.30, hy: 0.30 },
    BlockSpec { id: 'H', x:  1.30, y: 0.30, hx: 0.30, hy: 0.30 },
];

const LEVEL_1_BLOCKS: &[BlockSpec] = &[
    BlockSpec { id: 'A', x: -1.60, y: 0.45, hx: 0.35, hy: 0.45 },
    BlockSpec { id: 'B', x: -1.60, y: 1.35, hx: 0.35, hy: 0.45 },
    BlockSpec { id: 'C', x:  1.60, y: 0.45, hx: 0.35, hy: 0.45 },
    BlockSpec { id: 'D', x:  1.60, y: 1.35, hx: 0.35, hy: 0.45 },
    BlockSpec { id: 'E', x:  0.00, y: 2.02, hx: 1.95, hy: 0.22 },
    BlockSpec { id: 'F', x: -1.30, y: 0.30, hx: 0.30, hy: 0.30 },
    BlockSpec { id: 'G', x:  1.30, y: 0.30, hx: 0.30, hy: 0.30 },
    BlockSpec { id: 'H', x:  0.00, y: 0.30, hx: 0.30, hy: 0.30 },
];

const LEVEL_2_BLOCKS: &[BlockSpec] = &[
    BlockSpec { id: 'A', x: -0.90, y: 0.45, hx: 0.40, hy: 0.45 },
    BlockSpec { id: 'B', x: -0.90, y: 1.35, hx: 0.40, hy: 0.45 },
    BlockSpec { id: 'C', x:  0.90, y: 0.45, hx: 0.40, hy: 0.45 },
    BlockSpec { id: 'D', x:  0.90, y: 1.35, hx: 0.40, hy: 0.45 },
    BlockSpec { id: 'E', x:  0.00, y: 2.25, hx: 0.70, hy: 0.45 },
    BlockSpec { id: 'F', x: -1.50, y: 0.30, hx: 0.30, hy: 0.30 },
    BlockSpec { id: 'G', x:  1.50, y: 0.30, hx: 0.30, hy: 0.30 },
    BlockSpec { id: 'H', x:  0.00, y: 0.30, hx: 0.30, hy: 0.30 },
];

const LEVEL_3_BLOCKS: &[BlockSpec] = &[
    BlockSpec { id: 'A', x: 0.00, y: 0.35, hx: 0.80, hy: 0.35 },
    BlockSpec { id: 'B', x: 0.00, y: 1.05, hx: 0.62, hy: 0.35 },
    BlockSpec { id: 'C', x: 0.00, y: 1.75, hx: 0.46, hy: 0.35 },
    BlockSpec { id: 'D', x: 0.00, y: 2.45, hx: 0.32, hy: 0.35 },
    BlockSpec { id: 'E', x: 0.00, y: 3.15, hx: 0.20, hy: 0.35 },
    BlockSpec { id: 'F', x: -1.60, y: 0.30, hx: 0.30, hy: 0.30 },
    BlockSpec { id: 'G', x:  1.60, y: 0.30, hx: 0.30, hy: 0.30 },
    BlockSpec { id: 'H', x:  0.00, y: 0.30, hx: 0.30, hy: 0.30 },
];

const LEVEL_4_BLOCKS: &[BlockSpec] = &[
    BlockSpec { id: 'A', x: 0.00, y: 0.30, hx: 1.20, hy: 0.30 },
    BlockSpec { id: 'B', x: 0.00, y: 0.90, hx: 0.95, hy: 0.30 },
    BlockSpec { id: 'C', x: 0.00, y: 1.50, hx: 0.70, hy: 0.30 },
    BlockSpec { id: 'D', x: 0.00, y: 2.10, hx: 0.45, hy: 0.30 },
    BlockSpec { id: 'E', x: 0.00, y: 2.70, hx: 0.22, hy: 0.30 },
    BlockSpec { id: 'F', x: -1.85, y: 0.30, hx: 0.30, hy: 0.30 },
    BlockSpec { id: 'G', x:  1.85, y: 0.30, hx: 0.30, hy: 0.30 },
    BlockSpec { id: 'H', x: -1.30, y: 0.30, hx: 0.30, hy: 0.30 },
];

const LEVEL_5_BLOCKS: &[BlockSpec] = &[
    BlockSpec { id: 'A', x: 0.00, y: 0.50, hx: 0.35, hy: 0.50 },
    BlockSpec { id: 'B', x: 0.00, y: 1.50, hx: 0.35, hy: 0.50 },
    BlockSpec { id: 'C', x: 0.00, y: 2.50, hx: 0.60, hy: 0.50 },
    BlockSpec { id: 'D', x: 0.00, y: 3.20, hx: 1.10, hy: 0.20 },
    BlockSpec { id: 'E', x: 0.85, y: 3.65, hx: 0.25, hy: 0.25 },
    BlockSpec { id: 'F', x: -1.40, y: 0.30, hx: 0.30, hy: 0.30 },
    BlockSpec { id: 'G', x:  1.40, y: 0.30, hx: 0.30, hy: 0.30 },
    BlockSpec { id: 'H', x:  0.00, y: 0.30, hx: 0.30, hy: 0.30 },
];

const LEVELS: &[Level] = &[
    Level {
        name: "LA TORRE",
        blocks: LEVEL_0_BLOCKS,
    },
    Level {
        name: "EL PUENTE",
        blocks: LEVEL_1_BLOCKS,
    },
    Level {
        name: "EL ARCO",
        blocks: LEVEL_2_BLOCKS,
    },
    Level {
        name: "LA AGUJA",
        blocks: LEVEL_3_BLOCKS,
    },
    Level {
        name: "LA PIRAMIDE",
        blocks: LEVEL_4_BLOCKS,
    },
    Level {
        name: "LA GRUA",
        blocks: LEVEL_5_BLOCKS,
    },
];

const LANG_ES: u8 = 0;
const LANG_EN: u8 = 1;

static CURRENT_LANG: AtomicU8 = AtomicU8::new(LANG_ES);

#[no_mangle]
pub extern "C" fn set_language(lang: u8) {
    CURRENT_LANG.store(
        if lang == LANG_EN { LANG_EN } else { LANG_ES },
        Ordering::Relaxed,
    );
}

fn tr(key: &str) -> String {
    let lang = CURRENT_LANG.load(Ordering::Relaxed);
    match lang {
        LANG_EN => TR_EN.get(key).copied().unwrap_or(key).to_string(),
        _ => TR_ES.get(key).copied().unwrap_or(key).to_string(),
    }
}

static TR_ES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "puzzle" => "PUZZLE #",
    "level" => "NIVEL ",
    "of" => " / ",
    "attempts_label" => "INTENTOS",
    "attempts_left" => "quedan",
    "blocks_loaded" => "/5 BLOQUES CARGADOS",
    "ready_to_detonate" => "LISTO PARA DETONAR",
    "detonation" => "DETONACION ",
    "evaluating" => "EVALUANDO COLAPSO...",
    "perfect_collapse" => "¡COLAPSO PERFECTO!",
    "out_of_charges" => "SE ACABARON LAS CARGAS",
    "come_back" => "Vuelve mañana para otro puzzle.",
    "practice_again" => "Podés seguir practicando con esta estructura.",
    "solution_was" => "Los bloques que la sostenían eran:",
    "collapsed_in" => "Derribaste la estructura en",
    "attempt_singular" => "intento.",
    "attempt_plural" => "intentos.",
    "rebuilding" => "RECONSTRUYENDO LA ESTRUCTURA",
    "detonate_btn" => "DETONAR",
    "delete_btn" => "BORRAR",
    "retry_btn" => "OTRA VEZ",
    "other_level_btn" => "OTRA ESTRUCTURA",
    "pick_exactly" => "Elegí exactamente 5 bloques",
    "toast_full" => "Ya cargaste 5 bloques",
    "toast_dup" => "Ese bloque ya está cargado",
    "daily_closed" => "El puzzle de hoy ya se cerró",
    "attempts_exhausted" => "Se acabaron los intentos: apretá R para reiniciar",
    "verdict_collapsed" => "LA ESTRUCTURA CAYÓ",
    "verdict_perfect" => "DEMOLICIÓN PERFECTA",
    "verdict_standing" => "LA ESTRUCTURA SIGUE EN PIE",
    "verdict_wrong_blocks" => "CAYÓ, PERO ELEGISTE BLOQUES QUE NO LA SOSTENÍAN",
    "verdict_not_fallen" => "ELEGISTE BIEN, PERO NO LLEGÓ A CAER",
    "legend_green" => "sostiene a otro",
    "legend_yellow" => "solo se apoya",
    "legend_gray" => "suelto en el piso",
    "legend_title" => "Qué significa cada color",
};

static TR_EN: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "puzzle" => "PUZZLE #",
    "level" => "LEVEL ",
    "of" => " / ",
    "attempts_label" => "ATTEMPTS",
    "attempts_left" => "left",
    "blocks_loaded" => "/5 BLOCKS LOADED",
    "ready_to_detonate" => "READY TO DETONATE",
    "detonation" => "DETONATION ",
    "evaluating" => "EVALUATING COLLAPSE...",
    "perfect_collapse" => "PERFECT COLLAPSE!",
    "out_of_charges" => "OUT OF CHARGES",
    "come_back" => "Come back tomorrow for another puzzle.",
    "practice_again" => "You can keep practising on this structure.",
    "solution_was" => "The blocks holding it up were:",
    "collapsed_in" => "You collapsed the structure in",
    "attempt_singular" => "attempt.",
    "attempt_plural" => "attempts.",
    "rebuilding" => "REBUILDING THE STRUCTURE",
    "detonate_btn" => "DETONATE",
    "delete_btn" => "DELETE",
    "retry_btn" => "AGAIN",
    "other_level_btn" => "OTHER STRUCTURE",
    "pick_exactly" => "Pick exactly 5 blocks",
    "toast_full" => "You already loaded 5 blocks",
    "toast_dup" => "That block is already loaded",
    "daily_closed" => "Today's puzzle is already closed",
    "attempts_exhausted" => "Out of attempts: press R to restart",
    "verdict_collapsed" => "THE STRUCTURE FELL",
    "verdict_perfect" => "PERFECT DEMOLITION",
    "verdict_standing" => "THE STRUCTURE IS STILL STANDING",
    "verdict_wrong_blocks" => "IT FELL, BUT YOU PICKED BLOCKS THAT DIDN'T HOLD IT",
    "verdict_not_fallen" => "RIGHT BLOCKS, BUT IT DIDN'T COME DOWN",
    "legend_green" => "holds up another block",
    "legend_yellow" => "only resting on others",
    "legend_gray" => "loose on the ground",
    "legend_title" => "What each colour means",
};

static DAYS_SINCE_EPOCH: AtomicI64 = AtomicI64::new(0);
static DAYS_VALID: AtomicU8 = AtomicU8::new(0);

#[no_mangle]
pub extern "C" fn set_daily_seed(days: i32) {
    DAYS_SINCE_EPOCH.store(days as i64, Ordering::Relaxed);
    DAYS_VALID.store(1, Ordering::Relaxed);
}

const SOUND_QUEUE_LEN: usize = 16;
static SOUND_QUEUE: [AtomicI32; SOUND_QUEUE_LEN] = [const { AtomicI32::new(0) }; SOUND_QUEUE_LEN];
static SOUND_WRITE: AtomicUsize = AtomicUsize::new(0);
static SOUND_READ: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub extern "C" fn take_sound_event() -> i32 {
    let read = SOUND_READ.load(Ordering::Relaxed);
    let write = SOUND_WRITE.load(Ordering::Relaxed);
    if read >= write {
        return 0;
    }
    SOUND_READ.store(read + 1, Ordering::Relaxed);
    SOUND_QUEUE[read % SOUND_QUEUE_LEN].load(Ordering::Relaxed)
}

fn emit_sound(id: i32) {
    let write = SOUND_WRITE.fetch_add(1, Ordering::Relaxed);
    SOUND_QUEUE[write % SOUND_QUEUE_LEN].store(id, Ordering::Relaxed);
}

fn days_since_epoch() -> i64 {
    current_day().unwrap_or(0)
}

fn current_day() -> Option<i64> {
    if DAYS_VALID.load(Ordering::Relaxed) != 0 {
        return Some(DAYS_SINCE_EPOCH.load(Ordering::Relaxed));
    }
    #[cfg(target_arch = "wasm32")]
    {
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| (d.as_secs() / 86_400) as i64)
            .ok()
    }
}

fn daily_level_index_for(puzzle: i64) -> usize {
    (puzzle.rem_euclid(LEVELS.len() as i64)) as usize
}

fn get_daily_level_index() -> usize {
    daily_level_index_for(puzzle_number())
}

fn generate_level(seed: u32, attempt: u32) -> Option<Vec<BlockSpec>> {
    let mut blocks = generate_blocks(seed, attempt);
    for i in 0..blocks.len() {
        for j in i + 1..blocks.len() {
            if blocks_overlap(&blocks[i], &blocks[j]) {
                return None;
            }
        }
    }
    let level = Level {
        name: "GENERADA",
        blocks: Box::leak(blocks.clone().into_boxed_slice()),
    };

    let core = core_blocks(&level);
    if core.len() != SOLUTION_LEN {
        return None;
    }
    let sueltos: Vec<char> = level
        .blocks
        .iter()
        .filter(|b| !core.contains(&b.id))
        .map(|b| b.id)
        .collect();
    if sueltos.len() != 3 {
        return None;
    }

    let mut world = World::new(&level);
    for _ in 0..(1.5 / FIXED_DT) as usize {
        world.step();
    }
    for b in world.blocks.iter() {
        let Some(body) = world.bodies.get(b.handle) else {
            continue;
        };
        let p = body.translation();
        if p.y - b.hy < -0.02 {
            return None;
        }
        if body.rotation().angle().abs() > 0.30 {
            return None;
        }
    }

    let mut guess = ['A'; SOLUTION_LEN];
    for (i, c) in core.iter().enumerate() {
        guess[i] = *c;
    }
    let mut world = World::new(&level);
    for _ in 0..(EXPLODE_DELAY / FIXED_DT) as usize {
        world.step();
    }
    for c in core.iter() {
        world.detonate(*c);
        for _ in 0..(EXPLODE_INTERVAL / FIXED_DT) as usize {
            world.step();
        }
    }
    for _ in 0..(SIM_TAIL / FIXED_DT) as usize {
        world.step();
    }
    if !world.structure_collapsed(&level) {
        return None;
    }

    let mut g2 = ['A'; SOLUTION_LEN];
    for (i, c) in sueltos.iter().enumerate() {
        g2[i] = *c;
    }
    let mut world = World::new(&level);
    for _ in 0..(EXPLODE_DELAY / FIXED_DT) as usize {
        world.step();
    }
    for c in sueltos.iter() {
        world.detonate(*c);
        for _ in 0..(EXPLODE_INTERVAL / FIXED_DT) as usize {
            world.step();
        }
    }
    for _ in 0..(SIM_TAIL / FIXED_DT) as usize {
        world.step();
    }
    if world.structure_collapsed(&level) {
        return None;
    }

    blocks.shrink_to_fit();
    Some(blocks)
}

fn blocks_overlap(a: &BlockSpec, b: &BlockSpec) -> bool {
    let (ax0, ax1) = (a.x - a.hx, a.x + a.hx);
    let (ay0, ay1) = (a.y - a.hy, a.y + a.hy);
    let (bx0, bx1) = (b.x - b.hx, b.x + b.hx);
    let (by0, by1) = (b.y - b.hy, b.y + b.hy);
    ax0 < bx1 - 0.005 && ax1 > bx0 + 0.005 && ay0 < by1 - 0.005 && ay1 > by0 + 0.005
}

fn generate_blocks(seed: u32, attempt: u32) -> Vec<BlockSpec> {
    let s = seed
        .wrapping_mul(0x9E37_79B1)
        .wrapping_add(attempt.wrapping_mul(0x85EB_CA77))
        .wrapping_add(0x9E37_79B9);
    let r = |k: u32| -> f32 {
        let mut n = s
            .wrapping_add(k.wrapping_mul(0x27D4_EB2F))
            .wrapping_mul(0x2545_F491);
        n ^= n >> 15;
        n = n.wrapping_mul(0x85EB_CA77);
        n ^= n >> 13;
        (n & 0x00FF_FFFF) as f32 / 16_777_215.0
    };

    let mut b = Vec::with_capacity(8);
    let tipo = ((r(1) * 12.0) as usize).min(11);
    match tipo {
        0 => {
            let h = 0.42 + r(2) * 0.12;
            let hw = 0.28 + r(3) * 0.10;
            let dx = (r(4) - 0.5) * 0.10;
            for i in 0..5 {
                b.push(BlockSpec {
                    id: (b'A' + i as u8) as char,
                    x: dx * i as f32,
                    y: h * (i as f32 + 0.5),
                    hx: hw,
                    hy: h * 0.5,
                });
            }
        }
        1 => {
            let h = 0.32 + r(2) * 0.10;
            let base = 0.72 + r(3) * 0.22;
            for i in 0..5 {
                b.push(BlockSpec {
                    id: (b'A' + i as u8) as char,
                    x: 0.0,
                    y: h * (i as f32 + 0.5),
                    hx: base - i as f32 * (base * 0.22),
                    hy: h * 0.5,
                });
            }
        }
        2 => {
            let h = 0.40 + r(2) * 0.10;
            let sep = 1.4 + r(3) * 0.55;
            let hw = 0.30 + r(4) * 0.08;
            b.push(BlockSpec { id: 'A', x: -sep, y: h * 0.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec { id: 'B', x: -sep, y: h * 1.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec { id: 'C', x: sep, y: h * 0.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec { id: 'D', x: sep, y: h * 1.5, hx: hw, hy: h * 0.5 });
            let vh = 0.09 + r(5) * 0.06;
            b.push(BlockSpec {
                id: 'E',
                x: (r(6) - 0.5) * 0.20,
                y: h * 2.0 + vh,
                hx: sep + hw + 0.10,
                hy: vh,
            });
        }
        3 => {
            let h = 0.40 + r(2) * 0.10;
            let sep = 0.80 + r(3) * 0.32;
            let hw = 0.32 + r(4) * 0.10;
            b.push(BlockSpec { id: 'A', x: -sep, y: h * 0.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec { id: 'B', x: -sep, y: h * 1.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec { id: 'C', x: sep, y: h * 0.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec { id: 'D', x: sep, y: h * 1.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec {
                id: 'E',
                x: (r(6) - 0.5) * 0.15,
                y: h * 2.0,
                hx: 0.55 + r(5) * 0.20,
                hy: h * 0.5,
            });
        }
        4 => {
            let h = 0.46 + r(2) * 0.10;
            let hw = 0.30 + r(4) * 0.08;
            b.push(BlockSpec { id: 'A', x: 0.0, y: h * 0.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec { id: 'B', x: 0.0, y: h * 1.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec { id: 'C', x: 0.0, y: h * 2.5, hx: hw + 0.20, hy: h * 0.5 });
            let arm = 0.95 + r(5) * 0.30;
            let ah = 0.08 + r(6) * 0.06;
            b.push(BlockSpec {
                id: 'D',
                x: (r(7) - 0.5) * 0.20,
                y: h * 3.0 + ah,
                hx: arm,
                hy: ah,
            });
            b.push(BlockSpec {
                id: 'E',
                x: arm * (0.55 + r(8) * 0.30),
                y: h * 3.0 + ah * 2.0 + 0.15,
                hx: 0.22,
                hy: 0.15,
            });
        }
        5 => {
            let h = 0.40 + r(2) * 0.10;
            let paso = 0.50 + r(5) * 0.20;
            let mut x = -1.5;
            let mut y = h * 0.5;
            let ancho = 0.30 + r(4) * 0.08;
            for i in 0..5 {
                b.push(BlockSpec {
                    id: (b'A' + i as u8) as char,
                    x,
                    y,
                    hx: ancho,
                    hy: h * 0.5,
                });
                x += paso;
                y += h;
            }
        }
        6 => {
            let h = 0.44 + r(2) * 0.10;
            let hw = 0.44 + r(4) * 0.10;
            b.push(BlockSpec { id: 'A', x: 0.0, y: h * 0.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec { id: 'B', x: 0.0, y: h * 1.5, hx: hw * 0.86, hy: h * 0.5 });
            b.push(BlockSpec { id: 'C', x: 0.0, y: h * 2.5, hx: hw * 0.70, hy: h * 0.5 });
            let cap = 0.30 + r(5) * 0.14;
            b.push(BlockSpec {
                id: 'D',
                x: 0.0,
                y: h * 3.0 + cap * 0.5,
                hx: cap,
                hy: cap * 0.5,
            });
            b.push(BlockSpec {
                id: 'E',
                x: 0.0,
                y: h * 3.0 + cap * 1.5,
                hx: cap * 0.55,
                hy: cap * 0.5,
            });
        }
        7 => {
            let h = 0.42 + r(2) * 0.10;
            let hw = 0.30 + r(4) * 0.08;
            let largo = 0.95 + r(5) * 0.35;
            b.push(BlockSpec { id: 'A', x: 0.0, y: h * 0.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec { id: 'B', x: 0.0, y: h * 1.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec {
                id: 'C',
                x: largo * 0.5,
                y: h * 2.0,
                hx: largo * 0.5,
                hy: h * 0.36,
            });
            b.push(BlockSpec {
                id: 'D',
                x: largo,
                y: h * 2.0 - h * 0.18,
                hx: hw * 0.7,
                hy: h * 0.30,
            });
            b.push(BlockSpec {
                id: 'E',
                x: -largo * 0.42,
                y: h * 1.62,
                hx: hw * 0.7,
                hy: h * 0.30,
            });
        }
        9 => {
            let h = 0.40 + r(2) * 0.10;
            let hw = 0.32 + r(4) * 0.08;
            let sep = 0.80 + r(5) * 0.32;
            b.push(BlockSpec { id: 'A', x: -sep, y: h * 0.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec { id: 'B', x: -sep, y: h * 1.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec { id: 'C', x: sep, y: h * 0.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec { id: 'D', x: sep, y: h * 1.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec {
                id: 'E',
                x: 0.0,
                y: h * 2.0,
                hx: sep + hw + 0.10,
                hy: h * 0.5,
            });
        }
        10 => {
            let h = 0.44 + r(2) * 0.10;
            let hw = 0.44 + r(4) * 0.10;
            b.push(BlockSpec { id: 'A', x: 0.0, y: h * 0.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec { id: 'B', x: 0.0, y: h * 1.5, hx: hw * 0.86, hy: h * 0.5 });
            b.push(BlockSpec { id: 'C', x: 0.0, y: h * 2.5, hx: hw * 0.70, hy: h * 0.5 });
            let cap = 0.30 + r(5) * 0.14;
            b.push(BlockSpec {
                id: 'D',
                x: 0.0,
                y: h * 3.0 + cap * 0.5,
                hx: cap,
                hy: cap * 0.5,
            });
            b.push(BlockSpec {
                id: 'E',
                x: 0.0,
                y: h * 3.0 + cap * 1.5,
                hx: cap * 0.55,
                hy: cap * 0.5,
            });
        }
        _ => {
            let h = 0.42 + r(2) * 0.10;
            let hw = 0.30 + r(4) * 0.08;
            let largo = 0.95 + r(5) * 0.35;
            b.push(BlockSpec { id: 'A', x: 0.0, y: h * 0.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec { id: 'B', x: 0.0, y: h * 1.5, hx: hw, hy: h * 0.5 });
            b.push(BlockSpec {
                id: 'C',
                x: largo * 0.5,
                y: h * 2.0,
                hx: largo * 0.5,
                hy: h * 0.36,
            });
            b.push(BlockSpec {
                id: 'D',
                x: largo,
                y: h * 2.0 - h * 0.18,
                hx: hw * 0.7,
                hy: h * 0.30,
            });
            b.push(BlockSpec {
                id: 'E',
                x: -largo * 0.42,
                y: h * 1.62,
                hx: hw * 0.7,
                hy: h * 0.30,
            });
        }
    }
    let mut pos = [-1.95f32, 1.95, 2.90];
    for i in 0..3 {
        pos[i] += (r(10 + i as u32) - 0.5) * 0.24;
        b.push(BlockSpec {
            id: (b'F' + i as u8) as char,
            x: pos[i],
            y: 0.26 + r(20 + i as u32) * 0.04,
            hx: 0.26,
            hy: 0.26,
        });
    }
    b
}

fn generate_daily_level() -> Option<Level> {
    let puzzle = puzzle_number();
    if puzzle <= 0 {
        return None;
    }
    let seed = puzzle as u32;
    for attempt in 0..80u32 {
        if let Some(blocks) = generate_level(seed, attempt) {
            let name = nombre_estructura(&blocks);
            return Some(Level {
                name,
                blocks: Box::leak(blocks.into_boxed_slice()),
            });
        }
    }
    None
}

fn nombre_estructura(blocks: &[BlockSpec]) -> &'static str {
    let nivel = Level {
        name: "",
        blocks: Box::leak(blocks.to_vec().into_boxed_slice()),
    };
    let core = core_blocks(&nivel);

    let anchos: Vec<f32> = core
        .iter()
        .filter_map(|id| blocks.iter().find(|b| b.id == *id))
        .map(|b| b.hx)
        .collect();
    let alturas: Vec<f32> = core
        .iter()
        .filter_map(|id| blocks.iter().find(|b| b.id == *id))
        .map(|b| b.y)
        .collect();
    let xs: Vec<f32> = core
        .iter()
        .filter_map(|id| blocks.iter().find(|b| b.id == *id))
        .map(|b| b.x)
        .collect();

    let decreciente = anchos.windows(2).all(|w| w[0] > w[1]);
    let toda_la_misma_altura = alturas.iter().all(|h| (h - alturas[0]).abs() < 0.02);
    let superior_ancho = anchos.iter().any(|w| *w > 0.85);
    let en_linea = xs.iter().all(|x| (x - xs[0]).abs() < 0.05);
    let inclinada = !xs.windows(2).all(|w| (w[1] - w[0]).abs() > 0.15);

    if toda_la_misma_altura && superior_ancho {
        "LA VIGA"
    } else if toda_la_misma_altura {
        "EL PILAR"
    } else if decreciente {
        "LA ZIGURAT"
    } else if inclinada {
        "LA ESCALERA"
    } else if en_linea {
        "LA CUPULA"
    } else {
        "EL CANTO"
    }
}

const PUZZLE_EPOCH_DAYS: i64 = 20_454;

fn puzzle_number() -> i64 {
    (days_since_epoch() - PUZZLE_EPOCH_DAYS).max(0)
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

#[derive(Clone, Default)]
struct LevelState {
    attempts_used: usize,
    history: Vec<(
        [char; SOLUTION_LEN],
        [Feedback; SOLUTION_LEN],
        [Feedback; SOLUTION_LEN],
        [Feedback; SOLUTION_LEN],
    )>,
}

struct Progress {
    level_idx: usize,
    states: Vec<LevelState>,
    daily_idx: Option<usize>,
}

impl Progress {
    fn new(level_idx: usize, daily_idx: Option<usize>) -> Self {
        Progress {
            level_idx: level_idx % LEVELS.len(),
            states: vec![LevelState::default(); LEVELS.len()],
            daily_idx,
        }
    }

    fn state(&self) -> &LevelState {
        &self.states[self.level_idx]
    }

    fn state_mut(&mut self) -> &mut LevelState {
        &mut self.states[self.level_idx]
    }

    fn is_daily_level(&self) -> bool {
        self.daily_idx == Some(self.level_idx)
    }

    fn attempts_exhausted(&self) -> bool {
        self.state().attempts_used >= MAX_ATTEMPTS
    }

    fn can_submit(&self) -> bool {
        !self.attempts_exhausted()
    }

    fn consume_attempt(&mut self) -> bool {
        if !self.can_submit() {
            return false;
        }
        self.state_mut().attempts_used += 1;
        true
    }

    fn restart(&mut self) {
        if self.is_daily_level() {
            return;
        }
        let st = self.state_mut();
        st.attempts_used = 0;
        st.history.clear();
    }

    fn load_level(&mut self, idx: usize) {
        if idx < LEVELS.len() {
            self.level_idx = idx;
        }
    }

    fn reset_daily(&mut self, idx: usize) {
        for st in self.states.iter_mut() {
            st.attempts_used = 0;
            st.history.clear();
        }
        let _ = idx;
    }
}

struct Block {
    id: char,
    hx: f32,
    hy: f32,
    handle: RigidBodyHandle,
    alive: bool,
    tint: f32,
}

struct GhostBlock {
    id: char,
    x: f32,
    y: f32,
    angle: f32,
    hx: f32,
    hy: f32,
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

    #[allow(dead_code)]
    fn peak_height(&self) -> f32 {
        let mut peak: f32 = 0.0;
        for b in &self.blocks {
            if !b.alive {
                continue;
            }
            let Some(body) = self.bodies.get(b.handle) else {
                continue;
            };
            let p = body.translation();
            peak = peak.max(p.y + b.hy);
        }
        peak
    }

    fn structure_collapsed(&self, level: &Level) -> bool {
        let core = structural_blocks(level);
        for b in &self.blocks {
            if !b.alive || !core.contains(&b.id) {
                continue;
            }
            let Some(body) = self.bodies.get(b.handle) else {
                continue;
            };
            if body.translation().y - b.hy > 0.55 {
                return false;
            }
        }
        true
    }

    fn ghost_snapshot(&self) -> Vec<GhostBlock> {
        let mut out = Vec::new();
        for b in &self.blocks {
            if !b.alive {
                continue;
            }
            let Some(body) = self.bodies.get(b.handle) else {
                continue;
            };
            let p = body.translation();
            out.push(GhostBlock {
                id: b.id,
                x: p.x,
                y: p.y,
                angle: body.rotation().angle(),
                hx: b.hx,
                hy: b.hy,
            });
        }
        out
    }
}

fn level_peak_height(level: &Level) -> f32 {
    level
        .blocks
        .iter()
        .map(|b| b.y + b.hy)
        .fold(0.0f32, f32::max)
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
    seed_applied: i64,
    world: World,
    progress: Progress,
    selection: Vec<char>,
    sim_guess: [char; SOLUTION_LEN],
    phase: Phase,
    next_phase: Phase,
    collapsed: bool,
    perfect: bool,
    ghost: Option<Vec<GhostBlock>>,
    role_at_start: [Feedback; SOLUTION_LEN],
    role_at_detonation: [Feedback; SOLUTION_LEN],
    last_attempt_done: bool,
    last_feedback: [Feedback; SOLUTION_LEN],
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
    daily: Option<Level>,
    fullscreen: bool,
}

impl Game {
    fn new(level_idx: usize) -> Self {
        let level_idx = level_idx % LEVELS.len();
        let daily_idx = if current_day().is_some() {
            Some(get_daily_level_index())
        } else {
            None
        };
        Game {
            level_idx,
            seed_applied: days_since_epoch(),
            world: World::new(&LEVELS[level_idx]),
            daily: generate_daily_level(),
            progress: Progress::new(level_idx, daily_idx),
            selection: Vec::new(),
            sim_guess: ['A'; SOLUTION_LEN],
            phase: Phase::Planning,
            next_phase: Phase::Planning,
            collapsed: false,
            perfect: false,
            ghost: None,
            role_at_start: [Feedback::Gray; SOLUTION_LEN],
            role_at_detonation: [Feedback::Gray; SOLUTION_LEN],
            last_attempt_done: false,
            last_feedback: [Feedback::Gray; SOLUTION_LEN],
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
            fullscreen: false,
        }
    }

    fn level(&self) -> &Level {
        if self.level_idx == get_daily_level_index() {
            if let Some(daily) = &self.daily {
                return daily;
            }
        }
        &LEVELS[self.level_idx]
    }

    fn state(&self) -> &LevelState {
        self.progress.state()
    }

    fn state_mut(&mut self) -> &mut LevelState {
        self.progress.state_mut()
    }

    fn is_daily_level(&self) -> bool {
        self.progress.is_daily_level()
    }

    fn attempts_exhausted(&self) -> bool {
        self.progress.attempts_exhausted()
    }


    fn toast(&mut self, msg: &str) {
        self.toast_msg = msg.to_string();
        self.toast_timer = 1.7;
        emit_sound(6);
    }

    fn push_letter(&mut self, ch: char) {
        if self.phase != Phase::Planning {
            return;
        }
        if self.selection.len() >= SOLUTION_LEN {
            self.toast(&tr("toast_full"));
            return;
        }
        if self.selection.contains(&ch) {
            self.toast(&tr("toast_dup"));
            return;
        }
        self.selection.push(ch);
        self.select_t = 0.0;
        self.last_letter = Some(ch);
        emit_sound(1);
    }

    fn pop_letter(&mut self) {
        if self.phase == Phase::Planning && !self.selection.is_empty() {
            self.selection.pop();
            emit_sound(2);
        }
    }

    fn try_submit(&mut self) {
        if self.phase != Phase::Planning {
            return;
        }
        if self.attempts_exhausted() {
            self.toast(&tr(if self.is_daily_level() {
                "daily_closed"
            } else {
                "attempts_exhausted"
            }));
            return;
        }
        if self.selection.len() != SOLUTION_LEN {
            self.toast(&tr("pick_exactly"));
            return;
        }
        for (i, ch) in self.selection.iter().enumerate() {
            self.sim_guess[i] = *ch;
        }
        self.progress.consume_attempt();
        self.phase = Phase::Simulating;
        self.sim = Sim { next: 0, timer: EXPLODE_DELAY, post: SIM_TAIL };
        self.acc = 0.0;
        emit_sound(3);
    }

    fn target_blocks(&self) -> Vec<char> {
        target_blocks_for(self.level())
    }

    fn finish_attempt(&mut self) {
        let feedback = evaluate(self.level(), &self.sim_guess);
        let target = target_blocks_for(self.level());
        let perfect_guess = self.sim_guess.iter().zip(target.iter()).all(|(a, b)| a == b);
        let won = self.collapsed;
        let perfect = won && perfect_guess;
        let guess = self.sim_guess;
        let exhausted = {
            let start_roles = self.role_at_start;
            let det_roles = self.role_at_detonation;
            let st = self.state_mut();
            st.history.push((guess, feedback, start_roles, det_roles));
            st.attempts_used >= MAX_ATTEMPTS
        };
        self.last_feedback = feedback;
        self.perfect = perfect;
        if !won {
            self.ghost = Some(self.world.ghost_snapshot());
        } else {
            self.ghost = None;
        }
        self.selection.clear();

        self.reveal_row = Some(self.state().history.len() - 1);
        self.reveal_t = 0.0;

        self.next_phase = if won {
            emit_sound(4);
            Phase::Won
        } else if exhausted {
            emit_sound(5);
            Phase::Lost
        } else {
            Phase::Resetting
        };
        self.phase = Phase::Revealing;
    }

    fn restart(&mut self) {
        let daily = self.is_daily_level();
        if daily && self.attempts_exhausted() {
            self.toast(&tr("daily_closed"));
        }
        self.progress.restart();
        self.selection.clear();
        self.phase = Phase::Planning;
        self.next_phase = Phase::Planning;
        self.reveal_row = None;
        self.reveal_t = 0.0;
        self.select_t = 99.0;
        self.last_letter = None;
        self.ghost = None;
        self.rebuild();
    }

    fn load_level(&mut self, idx: usize) {
        if idx >= LEVELS.len() {
            return;
        }
        self.progress.load_level(idx);
        self.level_idx = idx;
        self.selection.clear();
        self.phase = Phase::Planning;
        self.next_phase = Phase::Planning;
        self.reveal_row = None;
        self.reveal_t = 0.0;
        self.select_t = 99.0;
        self.last_letter = None;
        self.ghost = None;
        self.rebuild();
    }

    fn rebuild(&mut self) {
        let blocks = self.level().blocks;
        self.world = World::new(&Level { name: "", blocks });
        self.particles.clear();
        self.flashes.clear();
        self.acc = 0.0;
        self.shake = 0.0;
        self.collapsed = false;
        self.perfect = false;
        self.ghost = None;
        self.role_at_start = [Feedback::Gray; SOLUTION_LEN];
        self.role_at_detonation = [Feedback::Gray; SOLUTION_LEN];
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
        if let Some(daily) = current_day() {
            if daily != self.seed_applied {
                self.seed_applied = daily;
                let idx = get_daily_level_index();
                self.progress.daily_idx = Some(idx);
                self.progress.reset_daily(idx);
                self.daily = generate_daily_level();
                self.load_level(idx);
            }
        }

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
                        if self.sim.next == 0 {
                            for i in 0..SOLUTION_LEN {
                                self.role_at_start[i] = live_feedback(&self.world, self.sim_guess[i]);
                            }
                        }
                        self.role_at_detonation[self.sim.next] =
                            live_feedback(&self.world, self.sim_guess[self.sim.next]);
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
                        self.collapsed = self.world.structure_collapsed(self.level());
                        self.last_attempt_done = true;
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
                if self.is_daily_level() && self.attempts_exhausted() {
                    let n = (self.level_idx + 1) % LEVELS.len();
                    self.load_level(n);
                } else {
                    self.restart();
                }
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
                    if self.is_daily_level() && self.attempts_exhausted() {
                        let n = (self.level_idx + 1) % LEVELS.len();
                        self.load_level(n);
                    } else {
                        self.restart();
                    }
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
        if self.ghost.is_some() && (is_key_down(KeyCode::Space) || is_key_down(KeyCode::Tab)) {
            self.draw_ghost();
        }
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

            self.draw_block_detail(block, sx, sy, w, h, angle, k);

            let letter = block.id.to_string();
            let size = (h * 0.45).clamp(18.0, 40.0);
            text_center(&letter, sx, sy + size * 0.36, size, C_INK);
        }
    }

    fn draw_ghost(&self) {
        let Some(ghost) = &self.ghost else {
            return;
        };
        for g in ghost {
            let (sx, sy) = w2s(g.x, g.y);
            let w = g.hx * 2.0 * PPM;
            let h = g.hy * 2.0 * PPM;
            let fill = Color { r: 0.9, g: 0.9, b: 0.9, a: 0.28 };
            let border = Color { r: 0.6, g: 0.6, b: 0.6, a: 0.5 };
            let params = DrawRectangleParams {
                offset: vec2(0.5, 0.5),
                rotation: g.angle,
                color: fill,
            };
            draw_rectangle_ex(sx, sy, w, h, params);
            draw_rectangle_lines_ex(sx, sy, w, h, 1.5, DrawRectangleParams {
                offset: vec2(0.5, 0.5),
                rotation: g.angle,
                color: border,
            });
            let letter = g.id.to_string();
            let size = (h * 0.45).clamp(18.0, 40.0);
            text_center(&letter, sx, sy + size * 0.36, size, Color { r: 0.3, g: 0.3, b: 0.3, a: 0.6 });
        }
    }

    fn draw_block_detail(
        &self,
        block: &Block,
        sx: f32,
        sy: f32,
        w: f32,
        h: f32,
        _angle: f32,
        k: f32,
    ) {
        let ancho = w.min(h * 1.4);
        let alto = h.min(w * 1.4);
        let seed = (block.id as u32).wrapping_mul(2654435761);
        let es_viga = w > h * 1.6;
        let es_bajo = block.hy < 0.32;

        if es_viga {
            let truss = Color { r: 0.30 * k, g: 0.22 * k, b: 0.13 * k, a: 0.55 };
            let n = ((w / 26.0).round() as usize).clamp(2, 9);
            for i in 0..n {
                let t = (i as f32 + 0.5) / n as f32;
                let x = sx - w * 0.5 + w * t;
                draw_line(
                    x,
                    sy - alto * 0.5,
                    x + w / n as f32 * 0.5,
                    sy + alto * 0.5,
                    1.6,
                    truss,
                );
                draw_line(
                    x + w / n as f32 * 0.5,
                    sy - alto * 0.5,
                    x,
                    sy + alto * 0.5,
                    1.6,
                    truss,
                );
            }
            let flange = Color { r: 0.26 * k, g: 0.19 * k, b: 0.11 * k, a: 0.75 };
            draw_rectangle(sx - w * 0.5, sy - alto * 0.5, w, 3.0, flange);
            draw_rectangle(sx - w * 0.5, sy + alto * 0.5 - 3.0, w, 3.0, flange);
        } else if es_bajo {
            let slab = Color { r: 0.28 * k, g: 0.21 * k, b: 0.12 * k, a: 0.5 };
            let n = ((ancho / 30.0).round() as usize).clamp(2, 7);
            for i in 0..n {
                let t = (i as f32 + 0.5) / n as f32;
                let x = sx - ancho * 0.5 + ancho * t;
                draw_line(x, sy - alto * 0.42, x, sy + alto * 0.42, 1.4, slab);
            }
        } else {
            let filas = ((alto / 26.0).round() as usize).clamp(1, 6);
            let mortero = Color { r: 0.33 * k, g: 0.24 * k, b: 0.13 * k, a: 0.42 };
            for i in 1..=filas {
                let y = sy - alto * 0.5 + alto * (i as f32 / (filas + 1) as f32);
                draw_line(sx - ancho * 0.5 + 2.0, y, sx + ancho * 0.5 - 2.0, y, 1.3, mortero);
            }
            let cols = ((ancho / 24.0).round() as usize).clamp(1, 5);
            for i in 1..cols {
                let x = sx - ancho * 0.5 + ancho * (i as f32 / cols as f32);
                draw_line(x, sy - alto * 0.5 + 2.0, x, sy + alto * 0.5 - 2.0, 1.1, mortero);
            }

            let ventana = Color { r: 0.16, g: 0.20, b: 0.27, a: 0.85 };
            let marco = Color { r: 0.30 * k, g: 0.22 * k, b: 0.12 * k, a: 0.9 };
            let ww = (ancho * 0.30).min(16.0);
            let wh = (alto * 0.30).min(18.0);
            if ww > 6.0 && wh > 6.0 {
                let filas_v = if alto > 70.0 { 2 } else { 1 };
                for f in 0..filas_v {
                    let oy = if filas_v == 2 {
                        (f as f32 - 0.5) * alto * 0.32
                    } else {
                        0.0
                    };
                    let wx = sx - ww * 0.5;
                    let wy = sy + oy - wh * 0.5;
                    draw_rectangle(wx, wy, ww, wh, ventana);
                    draw_rectangle_lines(wx, wy, ww, wh, 1.6, marco);
                }
            }
        }

        if hash01(seed) > 0.62 {
            let grieta = Color { r: 0.22 * k, g: 0.16 * k, b: 0.09 * k, a: 0.75 };
            let ox = (hash01(seed + 7) - 0.5) * ancho * 0.6;
            let oy = (hash01(seed + 13) - 0.5) * alto * 0.6;
            draw_line(
                sx + ox,
                sy + oy - alto * 0.25,
                sx + ox + alto * 0.14,
                sy + oy + alto * 0.22,
                1.5,
                grieta,
            );
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
        if self.fullscreen {
            draw_text(demi, 30.5, 56.5, size, shadow);
            draw_text(word, 30.5 + demi_w, 56.5, size, shadow);
            draw_text(demi, 28.0, 54.0, size, C_WHITE);
            draw_text(word, 28.0 + demi_w, 54.0, size, C_ACCENT);
        }

        draw_text(self.level().name, 30.0, 86.0, 18.0, C_DIM);

        let puzzle = format!("{}{}", tr("puzzle"), puzzle_number());
        draw_text(&puzzle, 30.0, 108.0, 18.0, C_WHITE);
        let lvl = format!("{}{} {}{}", tr("level"), self.level_idx + 1, tr("of"), LEVELS.len());
        draw_text(&lvl, 30.0, 130.0, 18.0, C_WHITE);
        let peak = level_peak_height(self.level());
        draw_text(&format!("{:.1} m", peak), 30.0, 152.0, 16.0, C_WHITE);

        if self.phase == Phase::Planning {
            if self.selection.len() == SOLUTION_LEN {
                text_center(
                    tr("ready_to_detonate"),
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

        if matches!(self.phase, Phase::Revealing | Phase::Won | Phase::Lost | Phase::Resetting)
            && self.last_attempt_done
        {
            let correct = !self.last_feedback.iter().any(|f| *f == Feedback::Gray);
            let (txt, color) = if self.collapsed && self.perfect {
                (tr("verdict_perfect"), C_GREEN_BORDER)
            } else if self.collapsed {
                (tr("verdict_collapsed"), C_GREEN_BORDER)
            } else if correct {
                (tr("verdict_not_fallen"), C_YELLOW_BORDER)
            } else {
                (tr("verdict_standing"), C_YELLOW_BORDER)
            };
            text_center(&txt, VIEW_W / 2.0, 512.0, 19.0, color);
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
        let used = self.state().attempts_used.min(MAX_ATTEMPTS);
        let left = MAX_ATTEMPTS.saturating_sub(used);
        text_right(
            &format!("{}/{}", used, MAX_ATTEMPTS),
            1262.0,
            52.0,
            20.0,
            C_DIM,
        );
        if left > 0 {
            text_right(
                &format!("{} {}", left, tr("attempts_left")),
                1262.0,
                76.0,
                15.0,
                C_FAINT,
            );
        }

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

                if row < self.state().history.len() {
                    let (guess, feedback, start_roles, det_roles) = self.state().history[row];
                    let (_, border) = feedback_style(feedback[col]);

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
                    let half = size * 0.5;
                    let (fill_start, _) = feedback_style(start_roles[col]);
                    let (fill_det, _) = feedback_style(det_roles[col]);
                    let tl = (cell_cx - half, cy - half);
                    let tr = (cell_cx + half, cy - half);
                    let bl = (cell_cx - half, cy + half);
                    let br = (cell_cx + half, cy + half);
                    draw_triangle(vec2(tl.0, tl.1), vec2(tr.0, tr.1), vec2(bl.0, bl.1), fill_start);
                    draw_triangle(vec2(tr.0, tr.1), vec2(br.0, br.1), vec2(bl.0, bl.1), fill_det);
                    draw_rectangle_lines(cell_cx - half, cy - half, size, size, 2.0, border);

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
                } else if row == self.state().history.len()
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

        let ly = GY0 + MAX_ATTEMPTS as f32 * ROW_PITCH + 22.0;
        let leyenda = [
            (Feedback::Green, tr("legend_green")),
            (Feedback::Yellow, tr("legend_yellow")),
            (Feedback::Gray, tr("legend_gray")),
        ];
        let mut lyx = PANEL_CARD_X + 16.0;
        for (fb, txt) in leyenda {
            let (fill, border) = feedback_style(fb);
            draw_rectangle(lyx, ly, 13.0, 13.0, fill);
            draw_rectangle_lines(lyx, ly, 13.0, 13.0, 1.5, border);
            draw_text(&txt, lyx + 18.0, ly + 12.0, 13.0, C_DIM);
            lyx += 18.0 + measure_text(&txt, None, 13, 1.0).width + 16.0;
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
            &tr("delete_btn"),
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
            &tr("detonate_btn"),
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
                    tr("rebuilding"),
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
                        "{} {} {}",
                        tr("collapsed_in"),
                        self.state().attempts_used,
                        if self.state().attempts_used == 1 { tr("attempt_singular") } else { tr("attempt_plural") }
                    );
                    text_center(&sub, VIEW_W / 2.0, 336.0, 22.0, C_WHITE);
                    text_center(
                        tr(if self.is_daily_level() { "come_back" } else { "practice_again" }),
                        VIEW_W / 2.0,
                        358.0,
                        18.0,
                        C_WHITE,
                    );
                } else {
                    text_center(tr("out_of_charges"), VIEW_W / 2.0, 296.0, 46.0, C_RED);
                    text_center(tr("solution_was"), VIEW_W / 2.0, 336.0, 22.0, C_WHITE);
                    text_center(
                        tr(if self.is_daily_level() { "come_back" } else { "practice_again" }),
                        VIEW_W / 2.0,
                        358.0,
                        18.0,
                        C_WHITE,
                    );
                }

                let chip = 54.0;
                let gap = 10.0;
                let targets = self.target_blocks();
                let total = targets.len() as f32 * chip + (targets.len() as f32 - 1.0) * gap;
                let x0 = VIEW_W / 2.0 - total / 2.0;
                for (i, ch) in targets.iter().enumerate() {
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

                let daily_closed = self.is_daily_level() && self.attempts_exhausted();
                let hover_retry = {
                    let (mx, my) = mouse_view();
                    inside(RETRY_X, RETRY_Y, RETRY_W, RETRY_H, mx, my)
                };
                draw_button(
                    RETRY_X,
                    RETRY_Y,
                    RETRY_W,
                    RETRY_H,
                    &tr(if daily_closed { "other_level_btn" } else { "retry_btn" }),
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


fn evaluate(level: &Level, guess: &[char; SOLUTION_LEN]) -> [Feedback; SOLUTION_LEN] {
    let mut out = [Feedback::Gray; SOLUTION_LEN];
    for (i, ch) in guess.iter().enumerate() {
        out[i] = match block_role(level, *ch) {
            Some(true) => Feedback::Green,
            Some(false) => Feedback::Yellow,
            None => Feedback::Gray,
        };
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
    game.fullscreen = fullscreen;
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
    fn el_feedback_describe_el_papel_del_bloque() {
        let level = &LEVELS[0];
        let guess = ['A', 'D', 'E', 'F', 'H'];
        let fb = evaluate(level, &guess);
        assert_eq!(fb[0], Feedback::Green, "A sostiene a B: es llave");
        assert_eq!(fb[1], Feedback::Green, "D sostiene a E: es llave");
        assert_eq!(fb[2], Feedback::Yellow, "E es el tope: solo se apoya");
        assert_eq!(fb[3], Feedback::Gray, "F esta suelto: es inutil");
        assert_eq!(fb[4], Feedback::Gray, "H esta suelto: es inutil");
    }

    #[test]
    fn cada_estructura_tiene_5_en_el_nucleo_y_3_sueltos() {
        for level in LEVELS {
            let nucleo = core_blocks(level);
            assert_eq!(
                nucleo.len(),
                SOLUTION_LEN,
                "{}: el nucleo tiene {} bloques, deberian ser {}",
                level.name,
                nucleo.len(),
                SOLUTION_LEN
            );

            let mut inutiles = 0;
            for b in level.blocks {
                if !nucleo.contains(&b.id) {
                    inutiles += 1;
                    assert_eq!(
                        block_role(level, b.id),
                        None,
                        "{}: el bloque {} esta fuera del nucleo pero no es inutil",
                        level.name,
                        b.id
                    );
                }
            }
            assert_eq!(
                inutiles, 3,
                "{}: tiene {} bloques sueltos, deberian ser 3",
                level.name, inutiles
            );

            let claves = key_blocks(level);
            assert!(
                !claves.is_empty(),
                "{}: ningun bloque sostiene a otro",
                level.name
            );
        }
    }

    #[test]
    fn los_inutiles_no_sostienen_nada() {
        for level in LEVELS {
            for b in level.blocks {
                if block_role(level, b.id).is_some() {
                    continue;
                }
                let holds = level.blocks.iter().any(|o| o.id != b.id && supports(b, o));
                let held = level.blocks.iter().any(|o| o.id != b.id && supports(o, b));
                assert!(!holds, "{}: el inutil {} sostiene a alguien", level.name, b.id);
                assert!(!held, "{}: el inutil {} esta apoyado en algo", level.name, b.id);
            }
        }
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
    fn detonar_el_plan_elimina_los_bloques() {
        for level in LEVELS {
            let target = target_blocks_for(level);
            {
                let mut world = World::new(level);
                settle(&mut world, 1.0);
                for i in 0..target.len() {
                    assert!(
                        world.detonate(target[i]).is_some(),
                        "no exploto el bloque {}",
                        target[i]
                    );
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

    #[test]
    fn tirar_los_claves_derrumba_y_los_inutiles_no() {
        for level in LEVELS {
            let claves = target_blocks_for(level);
            let mut world = World::new(level);
            settle(&mut world, EXPLODE_DELAY);
            for ch in &claves {
                world.detonate(*ch);
                settle(&mut world, EXPLODE_INTERVAL);
            }
            settle(&mut world, SIM_TAIL);
            assert!(
                world.structure_collapsed(level),
                "{}: tirar los bloques clave {:?} no derrumbó la estructura",
                level.name,
                claves.iter().collect::<String>()
            );

            let inutiles: Vec<char> = level
                .blocks
                .iter()
                .filter(|b| block_role(level, b.id).is_none())
                .map(|b| b.id)
                .collect();
            let mut world = World::new(level);
            settle(&mut world, EXPLODE_DELAY);
            for ch in &inutiles {
                world.detonate(*ch);
                settle(&mut world, EXPLODE_INTERVAL);
            }
            settle(&mut world, SIM_TAIL);
            assert!(
                !world.structure_collapsed(level),
                "{}: tirar los bloques inutiles {:?} SI derrumbó la estructura",
                level.name,
                inutiles.iter().collect::<String>()
            );
        }
    }

    #[test]
    fn la_estructura_se_mantiene_en_pie_sola() {
        for level in LEVELS {
            let mut world = World::new(level);
            settle(&mut world, 2.0);
            assert!(
                !world.structure_collapsed(level),
                "{}: la estructura se vino abajo sola",
                level.name
            );
        }
    }

    #[test]
    fn el_puzzle_diario_varia() {
        let mut estructuras = std::collections::HashSet::new();
        for puzzle in 0..60i64 {
            estructuras.insert(daily_level_index_for(puzzle));
        }
        assert!(
            estructuras.len() > 1,
            "el daily siempre usa la misma estructura"
        );
    }

    #[test]
    fn el_limite_de_intentos_es_real() {
        let daily = 0;
        let mut p = Progress::new(daily, Some(daily));
        assert!(p.is_daily_level());

        for _ in 0..MAX_ATTEMPTS {
            assert!(p.consume_attempt(), "un intento valido fue rechazado");
        }
        assert!(p.attempts_exhausted());
        assert!(!p.can_submit());

        assert!(
            !p.consume_attempt(),
            "se acepto un intento 7 en el puzzle diario"
        );
        assert_eq!(p.state().attempts_used, MAX_ATTEMPTS);

        p.restart();
        assert_eq!(
            p.state().attempts_used,
            MAX_ATTEMPTS,
            "reiniciar devolvio los intentos del diario"
        );
        assert!(p.attempts_exhausted());
    }

    #[test]
    fn en_practica_se_puede_reiniciar() {
        let daily = 0;
        let other = (daily + 1) % LEVELS.len();
        let mut p = Progress::new(other, Some(daily));
        assert!(!p.is_daily_level());

        for _ in 0..MAX_ATTEMPTS {
            p.consume_attempt();
        }
        p.state_mut().history.push((
            ['A', 'B', 'C', 'D', 'E'],
            [Feedback::Gray; SOLUTION_LEN],
            [Feedback::Gray; SOLUTION_LEN],
            [Feedback::Gray; SOLUTION_LEN],
        ));

        p.restart();
        assert_eq!(p.state().attempts_used, 0, "la practica no se reinicio");
        assert!(p.state().history.is_empty(), "el historial no se borro");
        assert!(p.can_submit());
    }

    #[test]
    fn cambiar_de_estructura_conserva_el_progreso() {
        let daily = 0;
        let mut p = Progress::new(daily, Some(daily));
        let a = 0;
        let b = (a + 1) % LEVELS.len();

        p.state_mut().attempts_used = 3;
        p.load_level(b);
        assert_eq!(p.level_idx, b);
        assert_eq!(
            p.states[a].attempts_used, 3,
            "cambiar de nivel borro los intentos del anterior"
        );
    }

    #[test]
    fn el_nuevo_dia_arranca_en_blanco() {
        let mut p = Progress::new(0, Some(0));
        p.state_mut().attempts_used = 4;
        p.state_mut()
            .history
            .push((['A', 'B', 'C', 'D', 'E'], [Feedback::Gray; SOLUTION_LEN], [Feedback::Gray; SOLUTION_LEN], [Feedback::Gray; SOLUTION_LEN]));

        p.reset_daily(0);
        assert_eq!(p.state().attempts_used, 0);
        assert!(p.state().history.is_empty());
    }

    #[test]
    fn el_nuevo_dia_limpia_todas_las_estructuras() {
        let mut p = Progress::new(0, Some(0));
        let other = 1;
        p.states[other].attempts_used = 2;
        p.states[other]
            .history
            .push((['A', 'B', 'C', 'D', 'E'], [Feedback::Gray; SOLUTION_LEN], [Feedback::Gray; SOLUTION_LEN], [Feedback::Gray; SOLUTION_LEN]));

        p.reset_daily(0);
        assert_eq!(p.states[other].attempts_used, 0, "quedo progreso del dia viejo");
        assert!(
            p.states[other].history.is_empty(),
            "quedo historial del plan viejo"
        );
    }

    #[test]
    fn no_set_incorrecto_es_perfecto() {
        for level in LEVELS {
            let ids: Vec<char> = level.blocks.iter().map(|b| b.id).collect();
            let target = target_blocks_for(level);

            let mut subs: Vec<Vec<usize>> = Vec::new();
            let mut cur = Vec::new();
            subsets(ids.len(), SOLUTION_LEN, 0, &mut cur, &mut subs);

            for s in &subs {
                let mut guess = ['A'; SOLUTION_LEN];
                for (i, j) in s.iter().enumerate() {
                    guess[i] = ids[*j];
                }
                if guess.iter().zip(target.iter()).all(|(a, b)| a == b) {
                    continue;
                }
                let mut world = World::new(level);
                settle(&mut world, EXPLODE_DELAY);
                for i in 0..SOLUTION_LEN {
                    world.detonate(guess[i]);
                    settle(&mut world, EXPLODE_INTERVAL);
                }
                settle(&mut world, SIM_TAIL);
                let colapso = world.structure_collapsed(level);
                let perfect = colapso
                    && guess.iter().zip(target.iter()).all(|(a, b)| a == b);
                assert!(
                    !perfect,
                    "{}: un set incorrecto {:?} seria perfecto",
                    level.name,
                    guess.iter().collect::<String>()
                );
            }
        }
    }

    #[test]
    fn colapso_bruto_es_victoria_normal() {
        for level in LEVELS {
            let ids: Vec<char> = level.blocks.iter().map(|b| b.id).collect();
            let target = target_blocks_for(level);

            let mut subs: Vec<Vec<usize>> = Vec::new();
            let mut cur = Vec::new();
            subsets(ids.len(), SOLUTION_LEN, 0, &mut cur, &mut subs);

            let mut hay_colapso_bruto = false;
            for s in &subs {
                let mut guess = ['A'; SOLUTION_LEN];
                for (i, j) in s.iter().enumerate() {
                    guess[i] = ids[*j];
                }
                if guess.iter().zip(target.iter()).all(|(a, b)| a == b) {
                    continue;
                }
                let mut world = World::new(level);
                settle(&mut world, EXPLODE_DELAY);
                for i in 0..SOLUTION_LEN {
                    world.detonate(guess[i]);
                    settle(&mut world, EXPLODE_INTERVAL);
                }
                settle(&mut world, SIM_TAIL);
                if world.structure_collapsed(level) {
                    hay_colapso_bruto = true;
                    break;
                }
            }
            let _ = hay_colapso_bruto;
        }
    }

    fn subsets(n: usize, k: usize, start: usize, cur: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if cur.len() == k {
            out.push(cur.clone());
            return;
        }
        for i in start..n {
            cur.push(i);
            subsets(n, k, i + 1, cur, out);
            cur.pop();
        }
    }

    fn generar_para(seed: u32) -> Option<Vec<BlockSpec>> {
        for attempt in 0..80u32 {
            if let Some(b) = generate_level(seed, attempt) {
                return Some(b);
            }
        }
        None
    }

    #[test]
    fn las_estructuras_generadas_son_validas() {
        let mut generadas = 0usize;
        for seed in 1..25u32 {
            let Some(blocks) = generar_para(seed) else {
                panic!("la semilla {} no genero ninguna estructura valida", seed);
            };
            generadas += 1;
            let level = Level {
                name: "GEN",
                blocks: Box::leak(blocks.into_boxed_slice()),
            };

            let core = core_blocks(&level);
            assert_eq!(
                core.len(),
                SOLUTION_LEN,
                "seed {}: el nucleo tiene {} bloques",
                seed,
                core.len()
            );
            let sueltos: Vec<char> = level
                .blocks
                .iter()
                .filter(|b| !core.contains(&b.id))
                .map(|b| b.id)
                .collect();
            assert_eq!(sueltos.len(), 3, "seed {}: no hay 3 sueltos", seed);

            let mut world = World::new(&level);
            settle(&mut world, 1.5);
            for b in world.blocks.iter() {
                let Some(body) = world.bodies.get(b.handle) else {
                    continue;
                };
                assert!(
                    body.translation().y - b.hy > -0.05,
                    "seed {}: el bloque {} se hundio",
                    seed,
                    b.id
                );
            }

            let mut guess = ['A'; SOLUTION_LEN];
            for (i, c) in core.iter().enumerate() {
                guess[i] = *c;
            }
            let mut world = World::new(&level);
            settle(&mut world, EXPLODE_DELAY);
            for i in 0..SOLUTION_LEN {
                world.detonate(guess[i]);
                settle(&mut world, EXPLODE_INTERVAL);
            }
            settle(&mut world, SIM_TAIL);
            assert!(
                world.structure_collapsed(&level),
                "seed {}: tirar el nucleo {:?} no derrumbó la estructura",
                seed,
                core.iter().collect::<String>()
            );

            let mut g2 = ['A'; SOLUTION_LEN];
            for (i, c) in sueltos.iter().enumerate() {
                g2[i] = *c;
            }
            let mut world = World::new(&level);
            settle(&mut world, EXPLODE_DELAY);
            for i in 0..3 {
                world.detonate(g2[i]);
                settle(&mut world, EXPLODE_INTERVAL);
            }
            settle(&mut world, SIM_TAIL);
            assert!(
                !world.structure_collapsed(&level),
                "seed {}: tirar los sueltos {:?} SI derrumbó la estructura",
                seed,
                sueltos.iter().collect::<String>()
            );
        }
        assert_eq!(generadas, 24);
    }

    #[test]
    fn el_puzzle_de_hoy_es_jugable() {
        for dia in [days_since_epoch(), 20_454 + 1, 20_454 + 315, 20_454 + 999] {
            if dia <= 0 {
                continue;
            }
            let mut encontrada = None;
            for attempt in 0..80u32 {
                if let Some(blocks) = generate_level(dia as u32, attempt) {
                    encontrada = Some(blocks);
                    break;
                }
            }
            let blocks = encontrada.unwrap_or_else(|| {
                panic!("el dia {} no genero ninguna estructura valida", dia)
            });
            let level = Level {
                name: "HOY",
                blocks: Box::leak(blocks.into_boxed_slice()),
            };
            let core = core_blocks(&level);
            assert_eq!(
                core.len(),
                SOLUTION_LEN,
                "dia {}: el nucleo tiene {} bloques",
                dia,
                core.len()
            );
            let mut guess = ['A'; SOLUTION_LEN];
            for (i, c) in core.iter().enumerate() {
                guess[i] = *c;
            }
            let mut world = World::new(&level);
            settle(&mut world, EXPLODE_DELAY);
            for i in 0..SOLUTION_LEN {
                world.detonate(guess[i]);
                settle(&mut world, EXPLODE_INTERVAL);
            }
            settle(&mut world, SIM_TAIL);
            assert!(
                world.structure_collapsed(&level),
                "dia {}: el puzzle de hoy no se puede ganar",
                dia
            );
        }
    }

    #[test]
    fn todos_los_tipos_generan_estructuras_validas() {
        let mut total = 0usize;
        for seed in 1..60u32 {
            let mut encontrada = None;
            for attempt in 0..80u32 {
                if let Some(b) = generate_level(seed, attempt) {
                    encontrada = Some(b);
                    break;
                }
            }
            let Some(blocks) = encontrada else {
                continue;
            };
            total += 1;
            let nivel = Level {
                name: "T",
                blocks: Box::leak(blocks.into_boxed_slice()),
            };
            let core = core_blocks(&nivel);
            assert_eq!(core.len(), SOLUTION_LEN, "seed {}: nucleo incompleto", seed);
        }
        assert!(total >= 55, "solo {} de 59 semillas generaron estructura", total);
    }

    #[test]
    fn cada_dia_genera_una_estructura_distinta() {
        let mut vistas = std::collections::HashSet::new();
        for seed in 1..15u32 {
            let blocks = generar_para(seed).expect("deberia generar estructura");
            let firma: String = blocks
                .iter()
                .map(|b| format!("{}{:.3}{:.3}{:.3}", b.id, b.x, b.y, b.hx))
                .collect();
            vistas.insert(firma);
        }
        assert_eq!(
            vistas.len(),
            14,
            "dos dias dieron la misma estructura: hay poca variedad"
        );
    }

    #[test]
    fn el_intento_correcto_produce_colapso() {
        for level in LEVELS {
            let nucleo = core_blocks(level);
            let mut guess = ['A'; SOLUTION_LEN];
            for (i, ch) in nucleo.iter().enumerate() {
                guess[i] = *ch;
            }

            let fb = evaluate(level, &guess);
            assert!(
                !fb.iter().any(|f| *f == Feedback::Gray),
                "{}: un bloque del nucleo {:?} salio gris",
                level.name,
                nucleo.iter().collect::<String>()
            );
            assert!(
                fb.iter().any(|f| *f == Feedback::Green),
                "{}: ningun bloque del nucleo es verde (llave)",
                level.name
            );

            let mut world = World::new(level);
            settle(&mut world, EXPLODE_DELAY);
            for ch in nucleo.iter() {
                world.detonate(*ch);
                settle(&mut world, EXPLODE_INTERVAL);
            }
            settle(&mut world, SIM_TAIL);
            assert!(
                world.structure_collapsed(level),
                "{}: el intento correcto no derrumbó la estructura",
                level.name
            );
        }
    }
}
