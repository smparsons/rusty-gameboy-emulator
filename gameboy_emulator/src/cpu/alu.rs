use crate::cpu::{Register, CpuState};
use crate::cpu::microops;

pub fn add_value_to_register(cpu_state: &mut CpuState, register: Register, value: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    let sum = byte.wrapping_add(value);

    microops::store_in_register(cpu_state, register, sum);

    microops::set_flag_z(cpu_state, sum == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, (value & 0xF) + (byte & 0xF) > 0xF);
    microops::set_flag_c(cpu_state, (value as u16 + byte as u16) > 0xFF);
}

pub fn add_value_and_carry_to_register(cpu_state: &mut CpuState, register: Register, value: u8) {
    let carry_value = if microops::is_c_flag_set(cpu_state) { 1 as u8 } else { 0 as u8 };
    add_value_to_register(cpu_state, register, value.wrapping_add(carry_value));
}

pub fn subtract_value_from_register(cpu_state: &mut CpuState, register: Register, value: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    let difference = byte.wrapping_sub(value);

    microops::store_in_register(cpu_state, register, difference);

    microops::set_flag_z(cpu_state, difference == 0);
    microops::set_flag_n(cpu_state, true);
    microops::set_flag_h(cpu_state, (byte & 0xF) < (value & 0xF));
    microops::set_flag_c(cpu_state, byte < value);
}

pub fn subtract_value_and_carry_from_register(cpu_state: &mut CpuState, register: Register, value: u8) {
    let carry_value = if microops::is_c_flag_set(cpu_state) { 1 as u8 } else { 0 as u8 };
    subtract_value_from_register(cpu_state, register, value.wrapping_add(carry_value));
}

pub fn logical_and_with_register(cpu_state: &mut CpuState, register: Register, value: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    let result = byte & value;

    microops::store_in_register(cpu_state, register, result);

    microops::set_flag_z(cpu_state, result == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, true);
    microops::set_flag_c(cpu_state, false);
}

pub fn logical_or_with_register(cpu_state: &mut CpuState, register: Register, value: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    let result = byte | value;

    microops::store_in_register(cpu_state, register, result);

    microops::set_flag_z(cpu_state, result == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, false);
}

pub fn logical_xor_with_register(cpu_state: &mut CpuState, register: Register, value: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    let result = byte ^ value;

    microops::store_in_register(cpu_state, register, result);

    microops::set_flag_z(cpu_state, result == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, false);
}