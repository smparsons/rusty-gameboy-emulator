use crate::emulator::initialize_emulator;

use super::*;

fn setup_emulator_with_test_memory() -> Emulator {
    let mut emulator = initialize_emulator();

    emulator.memory.bios[0] = 0xaf;
    emulator.memory.bios[1] = 0xF1;
    emulator.memory.bios[2] = 0x03;
    emulator.memory.bios[3] = 0x4D;

    emulator.memory.rom[0] = 0x1E;
    emulator.memory.rom[1] = 0xF2;
    emulator.memory.rom[2] = 0x01;
    emulator.memory.rom[3] = 0x09;

    emulator.memory.rom[0x20AF] = 0x11;
    emulator.memory.rom[0x20B0] = 0x17;
    emulator.memory.rom[0x20B1] = 0xEE;

    emulator.memory.rom[0x5ACC] = 0x13;
    emulator.memory.rom[0x5ACD] = 0x9C;
    emulator.memory.rom[0x5ACE] = 0x55;

    emulator.memory.video_ram[0] = 0xB1;
    emulator.memory.video_ram[1] = 0xD2;
    emulator.memory.video_ram[2] = 0xAA;

    emulator.memory.external_ram[0] = 0xC2;
    emulator.memory.external_ram[1] = 0x22;
    emulator.memory.external_ram[2] = 0x35;

    emulator.memory.working_ram[0] = 0xF1;
    emulator.memory.working_ram[1] = 0x22;
    emulator.memory.working_ram[2] = 0x2B;

    emulator.memory.working_ram[0x15F0] = 0x2B;
    emulator.memory.working_ram[0x15F1] = 0x7C;

    emulator.memory.object_attribute_memory[0x7A] = 0x44;
    emulator.memory.object_attribute_memory[0x7B] = 0x45;
    emulator.memory.object_attribute_memory[0x7C] = 0x9B;

    emulator.memory.zero_page_ram[0x20] = 0xBB;
    emulator.memory.zero_page_ram[0x21] = 0x44;
    emulator.memory.zero_page_ram[0x5B] = 0x5F;

    emulator.interrupts.enabled = 0x1F;
    emulator.interrupts.flags = 0xA;

    emulator.timers.divider = 0x3A;
    emulator.timers.counter = 0x04;
    emulator.timers.modulo = 0x02;
    emulator.timers.control = 0x07;

    emulator.gpu.registers.lcdc = 0x80;
    emulator.gpu.registers.scy = 0x55;
    emulator.gpu.registers.scx = 0xA1;
    emulator.gpu.registers.wy = 0xBB;
    emulator.gpu.registers.wx = 0xDD;
    emulator.gpu.registers.palette = 0xC1;
    emulator.gpu.registers.ly = 0x2B;
    emulator.gpu.registers.lyc = 0xAB;
    emulator.gpu.registers.stat = 0xD2;

    emulator.memory.in_bios = false;

    emulator
}

#[test]
fn reads_from_bios() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.in_bios = true;
    assert_eq!(read_byte(&emulator, 0x02), 0x03);
}

#[test]
fn reads_from_rom_in_bank_zero() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0x02), 0x01);
}

#[test]
fn reads_from_rom_in_bank_zero_scenario_two() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0x20B1), 0xEE);
}

#[test]
fn reads_from_rom_in_subsequent_bank() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0x5ACE), 0x55);
}

#[test]
fn reads_from_video_ram() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0x8002), 0xAA);
}

#[test]
fn reads_from_external_ram() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xA001), 0x22);
}

#[test]
fn reads_from_working_ram() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xC002), 0x2B);
}

#[test]
fn reads_from_working_ram_shadow() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xE002), 0x2B);
}

#[test]
fn reads_from_working_ram_shadow_scenario_two() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xF5F0), 0x2B);
}

#[test]
fn reads_from_object_attribute_memory() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFE7B), 0x45);
}

#[test]
fn reads_zero_values_outside_of_object_attribute_memory() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFEEE), 0x00);
}

#[test]
fn reads_from_zero_page_ram() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFFA0), 0xBB);
}

#[test]
fn reads_from_interrupts_enabled_register() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFFFF), 0x1F);
}

#[test]
fn reads_from_interrupt_flags_register() {
    let emulator= setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF0F), 0xA);
}

#[test]
fn reads_from_timer_divider_register() {
    let emulator= setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF04), 0x3A);
}

#[test]
fn reads_from_timer_counter_register() {
    let emulator= setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF05), 0x04);
}

#[test]
fn reads_from_timer_modulo_register() {
    let emulator= setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF06), 0x02);
}

#[test]
fn reads_from_timer_control_register() {
    let emulator= setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF07), 0x07);
}

#[test]
fn reads_lcdc_register_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF40), 0x80);
}

#[test]
fn reads_scroll_registers_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF42), 0x55);
    assert_eq!(read_byte(&emulator, 0xFF43), 0xA1);
}

#[test]
fn reads_window_position_registers_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF4A), 0xBB);
    assert_eq!(read_byte(&emulator, 0xFF4B), 0xDD);
}

#[test]
fn reads_palette_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF47), 0xC1);
}

#[test]
fn reads_ly_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF44), 0x2B);
}

#[test]
fn reads_lyc_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF45), 0xAB);
}

#[test]
fn reads_stat_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF41), 0xD2);
}

#[test]
fn reads_word_from_emulator() {
    let emulator= setup_emulator_with_test_memory();
    assert_eq!(read_word(&emulator, 0x20AF), 0x1711);
}

#[test]
fn loads_rom_buffer_into_emulator() {
    let mut emulator = setup_emulator_with_test_memory();

    let mut rom_buffer = vec![0; 0xA000];
    rom_buffer[0] = 0xA0;
    rom_buffer[1] = 0xCC;
    rom_buffer[2] = 0x3B;
    rom_buffer[3] = 0x4C;
    rom_buffer[0x7FFF] = 0xD4;
    rom_buffer[0x8000] = 0xBB;
    rom_buffer[0x8001] = 0xD1;

    emulator.memory = load_rom_buffer(emulator.memory, rom_buffer);

    assert_eq!(read_byte(&emulator, 0x0000), 0xA0);
    assert_eq!(read_byte(&emulator, 0x0001), 0xCC);
    assert_eq!(read_byte(&emulator, 0x0002), 0x3B);
    assert_eq!(read_byte(&emulator, 0x0003), 0x4C);
    assert_eq!(read_byte(&emulator, 0x7FFF), 0xD4);
    assert_eq!(read_byte(&emulator, 0x8000), 0xB1);
}

#[test]
fn writes_to_video_ram() {
    let mut emulator = setup_emulator_with_test_memory();
    write_byte(&mut emulator, 0x8002, 0xC1);
    assert_eq!(emulator.memory.video_ram[2], 0xC1);
}

#[test]
fn writes_word_to_video_ram() {
    let mut emulator = setup_emulator_with_test_memory();
    write_word(&mut emulator, 0x8002, 0xC1DD);
    assert_eq!(emulator.memory.video_ram[2], 0xDD);
    assert_eq!(emulator.memory.video_ram[3], 0xC1);
}
