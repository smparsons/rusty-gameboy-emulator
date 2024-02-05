use crate::emulator::initialize_emulator;
use super::*;

#[test]
fn should_move_from_oam_to_vram_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 2;
    emulator.gpu.line = 0;
    emulator.gpu.mode_clock = 76;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 3);
    assert_eq!(emulator.gpu.mode_clock, 0);
}

#[test]
fn should_move_from_vram_to_hblank_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 3;
    emulator.gpu.line = 0;
    emulator.gpu.mode_clock = 168;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 0);
    assert_eq!(emulator.gpu.mode_clock, 0);
}

#[test]
fn should_not_move_from_oam_to_vram_mode_too_early() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 2;
    emulator.gpu.line = 0;
    emulator.gpu.mode_clock = 40;
    emulator.cpu.clock.instruction_clock_cycles = 4; 
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 2);
    assert_eq!(emulator.gpu.mode_clock, 44);
}

#[test]
fn should_move_back_to_oam_mode_from_hblank_if_not_at_last_line() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 0;
    emulator.gpu.line = 100;
    emulator.gpu.mode_clock = 200;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 2);
    assert_eq!(emulator.gpu.mode_clock, 0);
    assert_eq!(emulator.gpu.line, 101);
}

#[test]
fn should_move_to_vblank_mode_from_hblank_if_at_last_line() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 0;
    emulator.gpu.line = 142;
    emulator.gpu.mode_clock = 200;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 1);
    assert_eq!(emulator.gpu.mode_clock, 0);
    assert_eq!(emulator.gpu.line, 143);
}

#[test]
fn should_move_back_to_oam_mode_from_vblank_at_correct_time() {
    let mut emulator = initialize_emulator();
    emulator.gpu.mode = 1;
    emulator.gpu.line = 152;
    emulator.gpu.mode_clock = 452;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.gpu.mode, 2);
    assert_eq!(emulator.gpu.mode_clock, 0);
    assert_eq!(emulator.gpu.line, 0);
}

#[test]
fn should_return_bg_and_window_enabled_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.lcdc = 0x01;
    let result = get_bg_and_window_enabled_mode(emulator.gpu.registers.lcdc);
    assert_eq!(result, true);
}

#[test]
fn should_return_obj_enabled_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.lcdc = 0x02;
    let result = get_obj_enabled_mode(emulator.gpu.registers.lcdc);
    assert_eq!(result, true);
}

#[test]
fn should_return_obj_size_mode() {
   let mut emulator = initialize_emulator();
   emulator.gpu.registers.lcdc = 0x04;
   let result = get_obj_size_mode(emulator.gpu.registers.lcdc);
   assert_eq!(result, true); 
}

#[test]
fn should_return_bg_tile_map_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.lcdc = 0x08;
    let result = get_bg_tile_map_mode(emulator.gpu.registers.lcdc);
    assert_eq!(result, true);
}

#[test]
fn should_return_tile_data_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.lcdc = 0x10;
    let result = get_tile_data_addressing_mode(emulator.gpu.registers.lcdc);
    assert_eq!(result, true);
}

#[test]
fn should_return_window_enabled_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.lcdc = 0x20;
    let result = get_window_enabled_mode(emulator.gpu.registers.lcdc);
    assert_eq!(result, true);
}

#[test]
fn should_return_window_tile_map_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.lcdc = 0x40;
    let result = get_window_tile_map_mode(emulator.gpu.registers.lcdc);
    assert_eq!(result, true);
}

#[test]
fn should_return_lcdc_enabled_mode() {
    let mut emulator = initialize_emulator();
    emulator.gpu.registers.lcdc = 0x80;
    let result = get_lcd_enabled_mode(emulator.gpu.registers.lcdc);
    assert_eq!(result, true);
}