/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: pcspk                                                           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Implementation for beep sound using the pc speaker. Works in    ║
   ║         qemu only if started with the correct audio settings.           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author:  Michael Schoettner, HHU, 22.9.2016                             ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
#![allow(dead_code)]

use crate::kernel::cpu;

use super::pit;

// Ports
const PORT_CTRL: u16 = 0x43;
const PORT_DATA0: u16 = 0x40;
const PORT_DATA2: u16 = 0x42;
const PORT_PPI: u16 = 0x61;

// Frequency of musical notes
// (Our OS does not really support floating point. The numbers will be converted to u32 in 'play')
const C0: f32 = 130.81;
const C0X: f32 = 138.59;
const D0: f32 = 146.83;
const D0X: f32 = 155.56;
const E0: f32 = 164.81;
const F0: f32 = 174.61;
const F0X: f32 = 185.00;
const G0: f32 = 196.00;
const G0X: f32 = 207.65;
const A0: f32 = 220.00;
const A0X: f32 = 233.08;
const B0: f32 = 246.94;

const C1: f32 = 261.63;
const C1X: f32 = 277.18;
const D1: f32 = 293.66;
const D1X: f32 = 311.13;
const E1: f32 = 329.63;
const F1: f32 = 349.23;
const F1X: f32 = 369.99;
const G1: f32 = 391.00;
const G1X: f32 = 415.30;
const A1: f32 = 440.00;
const A1X: f32 = 466.16;
const B1: f32 = 493.88;

const C2: f32 = 523.25;
const C2X: f32 = 554.37;
const D2: f32 = 587.33;
const D2X: f32 = 622.25;
const E2: f32 = 659.26;
const F2: f32 = 698.46;
const F2X: f32 = 739.99;
const G2: f32 = 783.99;
const G2X: f32 = 830.61;
const A2: f32 = 880.00;
const A2X: f32 = 923.33;
const B2: f32 = 987.77;
const C3: f32 = 1046.50;

/**
 Description: Play musical note with given frequency for given time. \
              Then the pc speaker ist turned off.

 Parameters: \
            `f` frequency of musical note \
            `d` duration in ms
*/
pub fn play(f: f32, d: u32) {

    //kprintln!("Playing Sound of {} Hz", f);

    // Conter starten
    programm_counter(f);

    speaker_on();

    delay(d);

    speaker_off();
    
}

/**
 Description: turns the speaker on
*/
pub fn speaker_on() {
    // Aktuellen Status holen
    let status = cpu::inb(PORT_PPI);
    // Lautsprecher einschalten
    cpu::outb(PORT_PPI, status | 3)
}

/**
 Description: turns the speaker off
*/
pub fn speaker_off() {

    // Aktuellen Status holen
    let status: u8 = cpu::inb(PORT_PPI);

    // Lautsprecher ausschalten
    let outbyte: u8 = status & !3;
    cpu::outb(PORT_PPI, outbyte);
}


/**
 Description: Helper function of `delay`. Returns the 16 bit counter value\
              of the counter 0 of the PIT.
*/
fn read_counter() -> u32 {
    /* Hier muss Code eingefuegt werden */
    return 0;
}

/**
 Description: Delay execution for given time in ms. \
              Minimum delay is 10ms. \
              For the implementation we use counter 0 of the PIT. \
              We might have to wait for several count downs to 0\
              because the counter has only 16 bit.

 Parameters: \
            `d` duration in ms
*/
fn delay(mut d: u32) {
    
    // Mindestens 10 ms
    if d < 10{
        d = 10;
    }

    // Syszeit speichern
    let old_systime: u64 = pit::get_systime();

    // So lange loopen bis Syszeit gleich old + d ist
    loop {
        if pit::get_systime() >= old_systime + (d/10) as u64 {
            break;
        }
    }
}




fn programm_counter(freq: f32){
    // Counter ausrechnen
    //let freq: f32 = 1.0/(d as f32 / 1000.0);
    let counter: u16 = (1_193_182_f32 / freq) as u16;

    // Command zusammenbauen
    // (10)Channel 2 | (11)Access-Mode: high/low Byte | (011)Timer_mode 3 = (square wave generator) | (0)Conter-Mode: Binary
    let pit_command: u8 = 0b10_11_011_0;

    cpu::outb(PORT_CTRL, pit_command);
    cpu::outb(PORT_DATA2,counter as u8);
    cpu::outb(PORT_DATA2,(counter >> 8) as u8);
}
 



/**
 Description: Tetris Sound, Kévin Rapaille, August 2013\
              https://gist.github.com/XeeX/6220067
*/
pub fn tetris() {
    play(658.0, 125);
    play(1320.0, 500);
    play(990.0, 250);
    play(1056.0, 250);
    play(1188.0, 250);
    play(1320.0, 125);
    play(1188.0, 125);
    play(1056.0, 250);
    play(990.0, 250);
    play(880.0, 500);
    play(880.0, 250);
    play(1056.0, 250);
    play(1320.0, 500);
    play(1188.0, 250);
    play(1056.0, 250);
    play(990.0, 750);
    play(1056.0, 250);
    play(1188.0, 500);
    play(1320.0, 500);
    play(1056.0, 500);
    play(880.0, 500);
    play(880.0, 500);
    delay(250);
    play(1188.0, 500);
    play(1408.0, 250);
    play(1760.0, 500);
    play(1584.0, 250);
    play(1408.0, 250);
    play(1320.0, 750);
    play(1056.0, 250);
    play(1320.0, 500);
    play(1188.0, 250);
    play(1056.0, 250);
    play(990.0, 500);
    play(990.0, 250);
    play(1056.0, 250);
    play(1188.0, 500);
    play(1320.0, 500);
    play(1056.0, 500);
    play(880.0, 500);
    play(880.0, 500);
    delay(500);
    play(1320.0, 500);
    play(990.0, 250);
    play(1056.0, 250);
    play(1188.0, 250);
    play(1320.0, 125);
    play(1188.0, 125);
    play(1056.0, 250);
    play(990.0, 250);
    play(880.0, 500);
    play(880.0, 250);
    play(1056.0, 250);
    play(1320.0, 500);
    play(1188.0, 250);
    play(1056.0, 250);
    play(990.0, 750);
    play(1056.0, 250);
    play(1188.0, 500);
    play(1320.0, 500);
    play(1056.0, 500);
    play(880.0, 500);
    play(880.0, 500);
    delay(250);
    play(1188.0, 500);
    play(1408.0, 250);
    play(1760.0, 500);
    play(1584.0, 250);
    play(1408.0, 250);
    play(1320.0, 750);
    play(1056.0, 250);
    play(1320.0, 500);
    play(1188.0, 250);
    play(1056.0, 250);
    play(990.0, 500);
    play(990.0, 250);
    play(1056.0, 250);
    play(1188.0, 500);
    play(1320.0, 500);
    play(1056.0, 500);
    play(880.0, 500);
    play(880.0, 500);
    delay(500);
    play(660.0, 1000);
    play(528.0, 1000);
    play(594.0, 1000);
    play(495.0, 1000);
    play(528.0, 1000);
    play(440.0, 1000);
    play(419.0, 1000);
    play(495.0, 1000);
    play(660.0, 1000);
    play(528.0, 1000);
    play(594.0, 1000);
    play(495.0, 1000);
    play(528.0, 500);
    play(660.0, 500);
    play(880.0, 1000);
    play(838.0, 2000);
    play(660.0, 1000);
    play(528.0, 1000);
    play(594.0, 1000);
    play(495.0, 1000);
    play(528.0, 1000);
    play(440.0, 1000);
    play(419.0, 1000);
    play(495.0, 1000);
    play(660.0, 1000);
    play(528.0, 1000);
    play(594.0, 1000);
    play(495.0, 1000);
    play(528.0, 500);
    play(660.0, 500);
    play(880.0, 1000);
    play(838.0, 2000);
    speaker_off();
}

/**
 Description: Clint, Part of Daft Punk’s Aerodynamic\
               https://www.kirrus.co.uk/2010/09/linux-beep-music
*/
pub fn aerodynamic() {
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    speaker_off();
}





























































































































































pub fn alle_meine_entchen() {
    play(261.63, 500);
    play(293.66, 500);
    play(329.63, 500);
    play(349.23, 500);
    play(392.00, 1000);
    play(392.00, 1000);
    play(440.00, 500);
    play(440.00, 500);
    play(440.00, 500);
    play(440.00, 500);
    play(392.00, 2000);
    play(440.00, 500);
    play(440.00, 500);
    play(440.00, 500);
    play(440.00, 500);
    play(392.00, 2000);
    play(349.23, 500);
    play(349.23, 500);
    play(349.23, 500);
    play(349.23, 500);
    play(329.63, 1000);
    play(329.63, 1000);
    play(293.66, 500);
    play(293.66, 500);
    play(293.66, 500);
    play(293.66, 500);
    play(261.63, 2000);
}





pub fn starwars_imperial() {
    play(196.00, 130);
    delay(740);
    play(196.00, 74);
    delay(365);
    play(196.00, 74);
    delay(46);
    play(196.00, 74);
    delay(55);
    play(196.00, 74);
    delay(65);
    play(196.00, 74);
    delay(356);
    play(196.00, 74);
    delay(55);
    play(196.00, 74);
    delay(36);
    play(196.00, 83);
    delay(55);
    play(155.56, 83);
    delay(46);
    play(155.56, 83);
    delay(36);
    play(155.56, 74);
    delay(65);
    play(155.56, 83);
    delay(356);
    play(196.00, 74);
    delay(816);
    play(196.00, 74);
    delay(356);
    play(196.00, 74);
    delay(55);
    play(196.00, 74);
    delay(55);
    play(196.00, 74);
    delay(46);
    play(196.00, 74);
    delay(365);
    play(196.00, 74);
    delay(55);
    play(196.00, 74);
    delay(46);
    play(196.00, 83);
    delay(46);
    play(155.56, 83);
    delay(36);
    play(155.56, 93);
    delay(27);
    play(155.56, 74);
    delay(65);
    play(155.56, 83);
    delay(365);
    play(196.00, 74);
    delay(797);
    play(196.00, 74);
    delay(346);
    play(196.00, 74);
    delay(46);
    play(196.00, 74);
    delay(55);
    play(196.00, 74);
    delay(46);
    play(196.00, 74);
    delay(374);
    play(196.00, 74);
    delay(55);
    play(196.00, 83);
    delay(46);
    play(196.00, 74);
    delay(46);
    play(155.56, 83);
    delay(36);
    play(155.56, 93);
    delay(36);
    play(155.56, 74);
    delay(65);
    play(155.56, 83);
    delay(365);
    play(196.00, 74);
    delay(797);
    play(196.00, 74);
    delay(374);
    play(196.00, 74);
    delay(46);
    play(196.00, 74);
    delay(55);
    play(196.00, 74);
    delay(36);
    play(196.00, 83);
    delay(365);
    play(196.00, 74);
    delay(55);
    play(196.00, 83);
    delay(46);
    play(196.00, 74);
    delay(46);
    play(155.56, 83);
    delay(46);
    play(155.56, 83);
    delay(36);
    play(155.56, 74);
    delay(65);
    play(155.56, 74);
    delay(421);
    play(196.00, 496);
    delay(356);
    play(196.00, 543);
    delay(346);
    play(196.00, 553);
    delay(299);
    play(155.56, 431);
    delay(234);
    play(233.08, 102);
    delay(112);
    play(196.00, 628);
    delay(224);
    play(155.56, 431);
    delay(243);
    play(233.08, 121);
    delay(74);
    play(196.00, 1135);
    delay(637);
    play(293.66, 562);
    delay(327);
    play(293.66, 553);
    delay(309);
    play(293.66, 581);
    delay(337);
    play(311.13, 384);
    delay(243);
    play(233.08, 130);
    delay(102);
    play(185.00, 628);
    delay(224);
    play(155.56, 374);
    delay(262);
    play(233.08, 121);
    delay(112);
    play(196.00, 1088);
    delay(675);
    play(392.00, 684);
    delay(205);
    play(196.00, 440);
    delay(243);
    play(196.00, 168);
    delay(36);
    play(392.00, 562);
    delay(243);
    play(369.99, 478);
    delay(234);
    play(349.23, 140);
    delay(36);
    play(329.63, 205);
    delay(65);
    play(311.13, 149);
    play(329.63, 196);
    delay(712);
    play(207.65, 158);
    delay(271);
    play(277.18, 590);
    delay(290);
    play(261.63, 487);
    delay(168);
    play(246.94, 158);
    delay(55);
    play(233.08, 168);
    delay(18);
    play(220.00, 130);
    delay(83);
    play(233.08, 187);
    delay(703);
    play(155.56, 140);
    delay(290);
    play(185.00, 534);
    delay(365);
    play(155.56, 506);
    delay(149);
    play(185.00, 102);
    delay(121);
    play(233.08, 600);
    delay(243);
    play(196.00, 496);
    delay(158);
    play(233.08, 112);
    delay(112);
    play(293.66, 1125);
    delay(600);
    play(392.00, 684);
    delay(205);
    play(196.00, 440);
    delay(243);
    play(196.00, 168);
    delay(36);
    play(392.00, 562);
    delay(243);
    play(369.99, 478);
    delay(234);
    play(349.23, 140);
    delay(36);
    play(329.63, 205);
    delay(65);
    play(311.13, 149);
    play(329.63, 196);
    delay(712);
    play(207.65, 158);
    delay(271);
    play(277.18, 590);
    delay(290);
    play(261.63, 487);
    delay(168);
    play(246.94, 121);
    delay(74);
    play(233.08, 168);
    delay(27);
    play(220.00, 130);
    delay(83);
    play(233.08, 187);
    delay(712);
    play(155.56, 140);
    delay(290);
    play(185.00, 534);
    delay(337);
    play(155.56, 506);
    delay(168);
    play(233.08, 102);
    delay(83);
    play(196.00, 600);
    delay(262);
    play(155.56, 496);
    delay(205);
    play(233.08, 112);
    delay(65);
    play(196.00, 1125);
}




pub fn super_mario() {
    play(659.26, 10);
    play(659.26, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(659.26, 10);
    play(783.99, 10);
    play(523.25, 10);
    play(392.00, 10);
    play(329.63, 10);
    play(440.00, 10);
    play(493.88, 10);
    play(466.16, 10);
    play(440.00, 10);
    play(392.00, 10);
    play(659.26, 10);
    play(783.99, 10);
    play(880.00, 10);
    play(698.46, 10);
    play(783.99, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(587.33, 10);
    play(493.88, 10);
    play(523.25, 10);
    play(392.00, 10);
    play(329.63, 10);
    play(440.00, 10);
    play(493.88, 10);
    play(466.16, 10);
    play(440.00, 10);
    play(392.00, 10);
    play(659.26, 10);
    play(783.99, 10);
    play(880.00, 10);
    play(698.46, 10);
    play(783.99, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(587.33, 10);
    play(493.88, 10);
    play(783.99, 10);
    play(739.99, 10);
    play(698.46, 10);
    play(622.25, 10);
    play(659.26, 10);
    play(415.30, 10);
    play(440.00, 10);
    play(523.25, 10);
    play(440.00, 10);
    play(523.25, 10);
    play(587.33, 10);
    play(783.99, 10);
    play(739.99, 10);
    play(698.46, 10);
    play(622.25, 10);
    play(659.26, 10);
    play(1046.50, 10);
    play(1046.50, 10);
    play(1046.50, 10);
    play(783.99, 10);
    play(739.99, 10);
    play(698.46, 10);
    play(622.25, 10);
    play(659.26, 10);
    play(415.30, 10);
    play(440.00, 10);
    play(523.25, 10);
    play(440.00, 10);
    play(523.25, 10);
    play(587.33, 10);
    play(622.25, 10);
    play(587.33, 10);
    play(523.25, 10);
    play(783.99, 10);
    play(739.99, 10);
    play(698.46, 10);
    play(622.25, 10);
    play(659.26, 10);
    play(415.30, 10);
    play(440.00, 10);
    play(523.25, 10);
    play(440.00, 10);
    play(523.25, 10);
    play(587.33, 10);
    play(783.99, 10);
    play(739.99, 10);
    play(698.46, 10);
    play(622.25, 10);
    play(659.26, 10);
    play(1046.50, 10);
    play(1046.50, 10);
    play(1046.50, 10);
    play(783.99, 10);
    play(739.99, 10);
    play(698.46, 10);
    play(622.25, 10);
    play(659.26, 10);
    play(415.30, 10);
    play(440.00, 10);
    play(523.25, 10);
    play(440.00, 10);
    play(523.25, 10);
    play(587.33, 10);
    play(622.25, 10);
    play(587.33, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(587.33, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(440.00, 10);
    play(392.00, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(587.33, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(587.33, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(440.00, 10);
    play(392.00, 10);
    play(659.26, 10);
    play(659.26, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(659.26, 10);
    play(783.99, 10);
    play(523.25, 10);
    play(392.00, 10);
    play(329.63, 10);
    play(440.00, 10);
    play(493.88, 10);
    play(466.16, 10);
    play(440.00, 10);
    play(392.00, 10);
    play(659.26, 10);
    play(783.99, 10);
    play(880.00, 10);
    play(698.46, 10);
    play(783.99, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(587.33, 10);
    play(493.88, 10);
    play(523.25, 10);
    play(392.00, 10);
    play(329.63, 10);
    play(440.00, 10);
    play(493.88, 10);
    play(466.16, 10);
    play(440.00, 10);
    play(392.00, 10);
    play(659.26, 10);
    play(783.99, 10);
    play(880.00, 10);
    play(698.46, 10);
    play(783.99, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(587.33, 10);
    play(493.88, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(392.00, 10);
    play(415.30, 10);
    play(440.00, 10);
    play(698.46, 10);
    play(698.46, 10);
    play(440.00, 10);
    play(493.88, 10);
    play(880.00, 10);
    play(880.00, 10);
    play(880.00, 10);
    play(783.99, 10);
    play(698.46, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(440.00, 10);
    play(392.00, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(392.00, 10);
    play(415.30, 10);
    play(440.00, 10);
    play(698.46, 10);
    play(698.46, 10);
    play(440.00, 10);
    play(493.88, 10);
    play(698.46, 10);
    play(698.46, 10);
    play(698.46, 10);
    play(659.26, 10);
    play(587.33, 10);
    play(523.25, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(392.00, 10);
    play(415.30, 10);
    play(440.00, 10);
    play(698.46, 10);
    play(698.46, 10);
    play(440.00, 10);
    play(493.88, 10);
    play(880.00, 10);
    play(880.00, 10);
    play(880.00, 10);
    play(783.99, 10);
    play(698.46, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(440.00, 10);
    play(392.00, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(392.00, 10);
    play(415.30, 10);
    play(440.00, 10);
    play(698.46, 10);
    play(698.46, 10);
    play(440.00, 10);
    play(493.88, 10);
    play(698.46, 10);
    play(698.46, 10);
    play(698.46, 10);
    play(659.26, 10);
    play(587.33, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(587.33, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(440.00, 10);
    play(392.00, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(587.33, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(523.25, 10);
    play(587.33, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(440.00, 10);
    play(392.00, 10);
    play(659.26, 10);
    play(659.26, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(659.26, 10);
    play(783.99, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(392.00, 10);
    play(415.30, 10);
    play(440.00, 10);
    play(698.46, 10);
    play(698.46, 10);
    play(440.00, 10);
    play(493.88, 10);
    play(880.00, 10);
    play(880.00, 10);
    play(880.00, 10);
    play(783.99, 10);
    play(698.46, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(440.00, 10);
    play(392.00, 10);
    play(659.26, 10);
    play(523.25, 10);
    play(392.00, 10);
    play(415.30, 10);
    play(440.00, 10);
    play(698.46, 10);
    play(698.46, 10);
    play(440.00, 10);
    play(493.88, 10);
    play(698.46, 10);
    play(698.46, 10);
    play(698.46, 10);
    play(659.26, 10);
    play(587.33, 10);
    play(523.25, 10);
}



pub fn doom() {
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 270);
    play(164.81, 232);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 257);
    play(146.83, 206);
    delay(15);
    play(82.41, 66);
    delay(155);
    play(82.41, 232);
    play(130.81, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 257);
    play(116.54, 232);
    play(82.41, 92);
    delay(130);
    delay(385);
    play(123.47, 219);
    delay(2);
    play(82.41, 653);
    play(130.81, 206);
    delay(15);
    play(82.41, 41);
    delay(181);
    delay(423);
    play(82.41, 283);
    play(164.81, 206);
    delay(15);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 270);
    play(146.83, 232);
    play(82.41, 66);
    delay(155);
    play(82.41, 232);
    play(130.81, 194);
    delay(41);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 1418);
    delay(15);
    play(116.54, 1239);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 347);
    play(164.81, 232);
    play(82.41, 79);
    delay(155);
    delay(398);
    play(82.41, 270);
    play(146.83, 232);
    play(82.41, 92);
    delay(130);
    delay(372);
    play(82.41, 245);
    play(130.81, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 245);
    delay(219);
    play(116.54, 270);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(123.47, 232);
    delay(2);
    play(82.41, 615);
    play(130.81, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 410);
    delay(53);
    play(164.81, 257);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 296);
    play(146.83, 232);
    play(82.41, 66);
    delay(155);
    play(82.41, 232);
    delay(232);
    play(82.41, 79);
    play(130.81, 359);
    delay(117);
    delay(359);
    play(82.41, 1418);
    play(116.54, 1201);
    play(82.41, 53);
    delay(168);
    delay(410);
    play(82.41, 270);
    play(164.81, 232);
    play(82.41, 66);
    delay(155);
    delay(410);
    play(82.41, 270);
    play(146.83, 206);
    delay(15);
    play(82.41, 66);
    delay(155);
    play(82.41, 232);
    play(130.81, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 257);
    play(116.54, 219);
    delay(2);
    play(82.41, 92);
    delay(130);
    delay(372);
    play(123.47, 232);
    play(82.41, 640);
    play(130.81, 206);
    delay(28);
    play(82.41, 28);
    delay(194);
    delay(436);
    play(82.41, 283);
    play(164.81, 206);
    delay(15);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 270);
    play(146.83, 232);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 245);
    play(130.81, 194);
    delay(28);
    play(82.41, 79);
    delay(143);
    delay(398);
    play(82.41, 1431);
    delay(15);
    play(116.54, 1240);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 347);
    play(164.81, 232);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 270);
    play(146.83, 245);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 245);
    play(130.81, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 245);
    delay(219);
    play(116.54, 270);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(123.47, 232);
    play(82.41, 615);
    play(130.81, 232);
    play(82.41, 79);
    delay(155);
    delay(398);
    play(82.41, 410);
    delay(53);
    play(164.81, 257);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 296);
    play(146.83, 232);
    play(82.41, 66);
    delay(155);
    play(82.41, 130);
    delay(92);
    play(369.99, 117);
    play(329.63, 104);
    play(311.13, 117);
    play(369.99, 104);
    play(440.00, 117);
    play(392.00, 104);
    play(369.99, 117);
    play(311.13, 117);
    play(369.99, 104);
    play(392.00, 117);
    play(440.00, 104);
    play(493.88, 117);
    play(440.00, 104);
    play(392.00, 117);
    play(369.99, 104);
    play(311.13, 117);
    play(82.41, 53);
    delay(168);
    delay(410);
    play(82.41, 270);
    play(164.81, 232);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 270);
    play(146.83, 206);
    delay(15);
    play(82.41, 79);
    delay(155);
    play(82.41, 232);
    play(130.81, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 245);
    play(116.54, 219);
    delay(2);
    play(82.41, 92);
    delay(130);
    delay(372);
    play(123.47, 232);
    play(82.41, 640);
    play(130.81, 206);
    delay(15);
    play(82.41, 41);
    delay(181);
    delay(436);
    play(82.41, 296);
    play(164.81, 206);
    delay(15);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 257);
    play(146.83, 232);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 245);
    play(130.81, 194);
    delay(28);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 1418);
    delay(28);
    play(116.54, 1252);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 334);
    play(164.81, 232);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 270);
    delay(194);
    play(146.83, 245);
    play(82.41, 92);
    delay(130);
    delay(385);
    play(82.41, 257);
    play(130.81, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 245);
    delay(219);
    play(116.54, 257);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(123.47, 232);
    play(82.41, 615);
    play(130.81, 232);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 423);
    delay(53);
    play(164.81, 270);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 296);
    play(146.83, 232);
    play(82.41, 53);
    delay(168);
    play(82.41, 130);
    delay(92);
    play(493.88, 117);
    play(392.00, 104);
    play(329.63, 117);
    play(392.00, 104);
    play(493.88, 117);
    play(392.00, 104);
    play(493.88, 117);
    play(659.26, 104);
    play(493.88, 117);
    play(392.00, 104);
    play(493.88, 117);
    play(392.00, 117);
    play(493.88, 104);
    play(659.26, 117);
    play(783.99, 104);
    play(987.77, 117);
    play(110.00, 53);
    delay(168);
    delay(410);
    play(110.00, 270);
    play(220.00, 219);
    delay(2);
    play(110.00, 66);
    delay(155);
    delay(398);
    play(110.00, 270);
    play(196.00, 206);
    delay(15);
    play(110.00, 79);
    delay(143);
    delay(385);
    play(110.00, 245);
    play(174.61, 232);
    delay(2);
    play(110.00, 66);
    delay(155);
    delay(398);
    play(110.00, 245);
    play(155.56, 219);
    delay(2);
    play(110.00, 92);
    delay(130);
    delay(372);
    play(164.81, 219);
    delay(2);
    play(110.00, 640);
    play(174.61, 206);
    delay(15);
    play(110.00, 41);
    delay(181);
    delay(423);
    play(110.00, 283);
    play(220.00, 219);
    delay(2);
    play(110.00, 66);
    delay(168);
    delay(410);
    play(110.00, 257);
    play(196.00, 219);
    delay(2);
    play(110.00, 66);
    delay(155);
    delay(398);
    play(110.00, 245);
    play(174.61, 194);
    delay(28);
    play(110.00, 79);
    delay(143);
    delay(385);
    play(110.00, 1418);
    delay(28);
    play(155.56, 1252);
    play(110.00, 66);
    delay(155);
    delay(398);
    play(110.00, 334);
    play(220.00, 219);
    delay(2);
    play(110.00, 66);
    delay(155);
    delay(398);
    play(110.00, 270);
    play(196.00, 232);
    play(110.00, 92);
    delay(130);
    delay(372);
    play(110.00, 257);
    play(174.61, 232);
    play(110.00, 79);
    delay(155);
    delay(398);
    play(110.00, 245);
    delay(219);
    play(155.56, 257);
    play(110.00, 66);
    delay(155);
    delay(398);
    play(164.81, 232);
    play(110.00, 615);
    play(174.61, 232);
    play(110.00, 79);
    delay(143);
    delay(385);
    play(110.00, 410);
    delay(53);
    play(220.00, 270);
    play(110.00, 79);
    delay(143);
    delay(398);
    play(110.00, 308);
    play(196.00, 232);
    play(110.00, 53);
    delay(168);
    play(110.00, 130);
    delay(92);
    play(329.63, 104);
    play(293.66, 117);
    play(261.63, 104);
    play(329.63, 117);
    play(261.63, 117);
    play(220.00, 104);
    play(261.63, 117);
    play(329.63, 104);
    play(440.00, 117);
    play(329.63, 104);
    play(261.63, 117);
    play(329.63, 104);
    play(261.63, 117);
    play(329.63, 104);
    play(261.63, 117);
    play(220.00, 117);
    play(82.41, 53);
    delay(168);
    delay(410);
    play(82.41, 257);
    play(164.81, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 257);
    play(146.83, 206);
    delay(15);
    play(82.41, 66);
    delay(155);
    play(82.41, 232);
    play(130.81, 232);
    play(82.41, 79);
    delay(143);
    delay(398);
    play(82.41, 257);
    play(116.54, 219);
    delay(2);
    play(82.41, 92);
    delay(130);
    delay(372);
    play(123.47, 219);
    delay(2);
    play(82.41, 640);
    play(130.81, 206);
    delay(15);
    play(82.41, 41);
    delay(181);
    delay(423);
    play(82.41, 283);
    play(164.81, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 270);
    play(146.83, 232);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 245);
    play(130.81, 194);
    delay(28);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 1418);
    delay(15);
    play(116.54, 1240);
    play(82.41, 79);
    delay(155);
    delay(398);
    play(82.41, 334);
    play(164.81, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 270);
    play(146.83, 232);
    play(82.41, 92);
    delay(130);
    delay(372);
    play(82.41, 257);
    play(130.81, 232);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 245);
    delay(232);
    play(116.54, 270);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(123.47, 232);
    play(82.41, 615);
    play(130.81, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 410);
    delay(53);
    play(164.81, 270);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 296);
    play(146.83, 232);
    play(82.41, 66);
    delay(168);
    play(82.41, 232);
    delay(232);
    play(82.41, 79);
    play(130.81, 347);
    delay(117);
    delay(359);
    play(82.41, 1418);
    play(116.54, 1201);
    play(138.59, 66);
    delay(155);
    delay(410);
    play(138.59, 270);
    play(277.18, 219);
    delay(2);
    play(138.59, 66);
    delay(155);
    delay(398);
    play(138.59, 257);
    play(246.94, 206);
    delay(15);
    play(138.59, 66);
    delay(155);
    play(138.59, 232);
    play(220.00, 219);
    delay(2);
    play(138.59, 66);
    delay(155);
    delay(398);
    play(138.59, 257);
    play(196.00, 232);
    play(138.59, 92);
    delay(143);
    delay(385);
    play(207.65, 219);
    delay(2);
    play(138.59, 640);
    play(220.00, 206);
    delay(15);
    play(123.47, 41);
    delay(181);
    delay(423);
    play(123.47, 283);
    play(246.94, 206);
    delay(15);
    play(123.47, 66);
    delay(155);
    delay(398);
    play(123.47, 270);
    play(220.00, 232);
    play(123.47, 79);
    delay(143);
    delay(398);
    play(123.47, 257);
    play(196.00, 181);
    delay(41);
    play(116.54, 79);
    delay(143);
    delay(385);
    play(110.00, 1418);
    delay(15);
    play(174.61, 1240);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 347);
    play(164.81, 232);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 270);
    play(146.83, 232);
    play(82.41, 92);
    delay(130);
    delay(372);
    play(82.41, 245);
    play(130.81, 232);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 245);
    delay(219);
    play(116.54, 270);
    play(82.41, 79);
    delay(143);
    delay(398);
    play(123.47, 219);
    delay(2);
    play(82.41, 615);
    play(130.81, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 410);
    delay(53);
    play(164.81, 257);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 296);
    play(146.83, 232);
    play(82.41, 66);
    delay(155);
    play(82.41, 232);
    play(329.63, 117);
    play(392.00, 117);
    play(246.94, 104);
    play(196.00, 117);
    play(329.63, 104);
    play(246.94, 117);
    play(392.00, 104);
    play(329.63, 117);
    play(392.00, 104);
    play(329.63, 117);
    play(246.94, 104);
    play(196.00, 117);
    play(329.63, 117);
    play(392.00, 104);
    play(493.88, 117);
    play(659.26, 104);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 270);
    play(164.81, 232);
    play(82.41, 66);
    delay(168);
    delay(410);
    play(82.41, 257);
    play(146.83, 206);
    delay(15);
    play(82.41, 66);
    delay(155);
    play(82.41, 232);
    play(130.81, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 257);
    play(116.54, 232);
    play(82.41, 92);
    delay(130);
    delay(372);
    play(123.47, 232);
    delay(2);
    play(82.41, 640);
    play(130.81, 194);
    delay(28);
    play(82.41, 28);
    delay(194);
    delay(436);
    play(82.41, 283);
    play(164.81, 206);
    delay(15);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 270);
    play(146.83, 232);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 245);
    play(130.81, 194);
    delay(28);
    play(82.41, 79);
    delay(155);
    delay(398);
    play(82.41, 1418);
    delay(15);
    play(116.54, 1240);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 347);
    play(164.81, 232);
    play(82.41, 79);
    delay(143);
    delay(398);
    play(82.41, 283);
    play(146.83, 232);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 245);
    play(130.81, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 245);
    delay(219);
    play(116.54, 270);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(123.47, 232);
    play(82.41, 615);
    play(130.81, 232);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 410);
    delay(53);
    play(164.81, 257);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 296);
    play(146.83, 232);
    play(82.41, 66);
    delay(155);
    play(82.41, 143);
    delay(79);
    play(311.13, 117);
    play(261.63, 104);
    play(246.94, 117);
    play(185.00, 104);
    play(369.99, 117);
    play(311.13, 117);
    play(246.94, 104);
    play(220.00, 117);
    play(440.00, 104);
    play(369.99, 117);
    play(311.13, 104);
    play(246.94, 117);
    play(493.88, 104);
    play(440.00, 117);
    play(369.99, 104);
    play(311.13, 117);
    play(82.41, 53);
    delay(168);
    delay(410);
    play(82.41, 270);
    play(196.00, 232);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 270);
    play(185.00, 219);
    delay(15);
    play(82.41, 66);
    delay(155);
    play(82.41, 232);
    play(155.56, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 245);
    play(146.83, 219);
    delay(2);
    play(82.41, 92);
    delay(130);
    delay(372);
    play(146.83, 232);
    play(82.41, 640);
    play(164.81, 206);
    delay(15);
    play(82.41, 41);
    delay(194);
    delay(436);
    play(82.41, 283);
    play(196.00, 206);
    delay(15);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 270);
    play(185.00, 232);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 245);
    play(164.81, 194);
    delay(28);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 1431);
    play(138.59, 1214);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 334);
    play(196.00, 232);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 270);
    delay(194);
    play(185.00, 245);
    play(82.41, 92);
    delay(143);
    delay(385);
    play(82.41, 245);
    play(155.56, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 245);
    delay(219);
    play(146.83, 257);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(146.83, 232);
    play(82.41, 614);
    play(164.81, 232);
    play(82.41, 79);
    delay(143);
    delay(398);
    play(82.41, 423);
    delay(53);
    play(196.00, 257);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 296);
    play(185.00, 232);
    play(82.41, 66);
    delay(155);
    play(82.41, 130);
    delay(92);
    play(659.26, 117);
    play(493.88, 104);
    play(392.00, 117);
    play(329.63, 104);
    play(493.88, 117);
    play(659.26, 104);
    play(493.88, 117);
    play(392.00, 104);
    play(329.63, 117);
    play(392.00, 117);
    play(493.88, 104);
    play(392.00, 117);
    play(659.26, 104);
    play(493.88, 117);
    play(392.00, 104);
    play(329.63, 117);
    play(110.00, 53);
    delay(168);
    delay(410);
    play(110.00, 270);
    play(261.63, 232);
    play(110.00, 66);
    delay(155);
    delay(398);
    play(110.00, 270);
    play(246.94, 206);
    delay(15);
    play(110.00, 79);
    delay(143);
    play(110.00, 245);
    play(207.65, 219);
    delay(2);
    play(110.00, 66);
    delay(155);
    delay(398);
    play(110.00, 245);
    play(196.00, 219);
    delay(2);
    play(110.00, 92);
    delay(130);
    delay(372);
    play(196.00, 219);
    delay(2);
    play(110.00, 640);
    play(220.00, 206);
    delay(15);
    play(110.00, 41);
    delay(181);
    delay(423);
    play(110.00, 283);
    play(261.63, 219);
    delay(15);
    play(110.00, 66);
    delay(155);
    delay(398);
    play(110.00, 257);
    play(246.94, 219);
    delay(2);
    play(110.00, 66);
    delay(155);
    delay(398);
    play(110.00, 245);
    play(220.00, 194);
    delay(28);
    play(110.00, 79);
    delay(143);
    delay(385);
    play(110.00, 1418);
    delay(28);
    play(196.00, 1252);
    play(110.00, 66);
    delay(155);
    delay(398);
    play(110.00, 334);
    play(261.63, 219);
    delay(2);
    play(110.00, 66);
    delay(155);
    delay(398);
    play(110.00, 270);
    play(246.94, 232);
    play(110.00, 92);
    delay(130);
    delay(372);
    play(110.00, 257);
    play(207.65, 232);
    delay(2);
    play(110.00, 66);
    delay(155);
    delay(398);
    play(110.00, 245);
    delay(219);
    play(196.00, 257);
    play(110.00, 66);
    delay(155);
    delay(398);
    play(196.00, 232);
    play(110.00, 614);
    play(220.00, 232);
    play(110.00, 79);
    delay(143);
    delay(385);
    play(110.00, 423);
    delay(41);
    play(261.63, 270);
    play(110.00, 79);
    delay(155);
    delay(398);
    play(110.00, 296);
    play(246.94, 232);
    play(110.00, 53);
    delay(168);
    play(110.00, 130);
    delay(92);
    play(329.63, 104);
    play(293.66, 117);
    play(261.63, 117);
    play(329.63, 104);
    play(261.63, 117);
    play(220.00, 104);
    play(261.63, 117);
    play(329.63, 104);
    play(440.00, 117);
    play(329.63, 104);
    play(261.63, 117);
    play(329.63, 104);
    play(261.63, 117);
    play(329.63, 117);
    play(261.63, 104);
    play(220.00, 117);
    play(82.41, 53);
    delay(168);
    delay(410);
    play(82.41, 257);
    play(196.00, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 270);
    play(185.00, 206);
    delay(15);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 245);
    play(155.56, 232);
    play(82.41, 79);
    delay(155);
    delay(398);
    play(82.41, 245);
    play(146.83, 219);
    delay(2);
    play(82.41, 92);
    delay(130);
    delay(372);
    play(146.83, 219);
    delay(2);
    play(82.41, 640);
    play(164.81, 206);
    delay(15);
    play(82.41, 41);
    delay(181);
    delay(423);
    play(82.41, 283);
    play(196.00, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(410);
    play(82.41, 270);
    play(185.00, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 245);
    play(164.81, 194);
    delay(28);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 1418);
    delay(28);
    play(138.59, 1252);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 334);
    play(196.00, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 270);
    play(185.00, 232);
    play(82.41, 92);
    delay(130);
    delay(372);
    play(82.41, 257);
    play(155.56, 232);
    play(82.41, 79);
    delay(143);
    delay(398);
    play(82.41, 257);
    delay(219);
    play(146.83, 257);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(146.83, 232);
    play(82.41, 614);
    play(164.81, 219);
    delay(2);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 410);
    delay(53);
    play(196.00, 270);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 296);
    play(185.00, 245);
    play(82.41, 53);
    delay(168);
    play(82.41, 232);
    delay(232);
    play(82.41, 79);
    play(164.81, 347);
    delay(117);
    delay(359);
    play(82.41, 1418);
    play(138.59, 1201);
    play(138.59, 66);
    delay(168);
    delay(410);
    play(138.59, 257);
    play(392.00, 219);
    delay(2);
    play(138.59, 66);
    delay(155);
    delay(398);
    play(138.59, 257);
    play(369.99, 206);
    delay(15);
    play(138.59, 66);
    delay(155);
    play(138.59, 232);
    play(329.63, 232);
    play(138.59, 66);
    delay(155);
    delay(398);
    play(138.59, 257);
    play(277.18, 232);
    delay(2);
    play(138.59, 92);
    delay(130);
    delay(372);
    play(277.18, 219);
    delay(2);
    play(138.59, 640);
    play(329.63, 206);
    delay(15);
    play(123.47, 41);
    delay(181);
    delay(423);
    play(123.47, 283);
    play(369.99, 206);
    delay(15);
    play(123.47, 66);
    delay(155);
    delay(398);
    play(123.47, 270);
    play(329.63, 232);
    play(123.47, 79);
    delay(155);
    delay(398);
    play(123.47, 245);
    play(311.13, 194);
    delay(28);
    play(116.54, 79);
    delay(143);
    delay(385);
    play(110.00, 1418);
    delay(15);
    play(246.94, 1239);
    play(82.41, 79);
    delay(143);
    delay(398);
    play(82.41, 347);
    play(196.00, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 270);
    play(185.00, 232);
    play(82.41, 92);
    delay(130);
    delay(372);
    play(82.41, 257);
    play(155.56, 232);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 245);
    delay(219);
    play(146.83, 270);
    play(82.41, 79);
    delay(155);
    delay(398);
    play(146.83, 232);
    play(82.41, 602);
    play(164.81, 219);
    delay(2);
    play(82.41, 66);
    delay(155);
    delay(398);
    play(82.41, 410);
    delay(53);
    play(196.00, 270);
    play(82.41, 79);
    delay(143);
    delay(385);
    play(82.41, 296);
    play(185.00, 232);
    play(82.41, 66);
    delay(155);
    play(82.41, 245);
    play(440.00, 104);
    play(369.99, 117);
    play(311.13, 104);
    play(246.94, 117);
    play(220.00, 104);
    play(185.00, 117);
    play(155.56, 104);
    play(123.47, 117);
    play(493.88, 104);
    play(440.00, 117);
    play(369.99, 117);
    play(311.13, 104);
    play(246.94, 117);
    play(220.00, 104);
    play(185.00, 117);
    play(155.56, 104);
}
