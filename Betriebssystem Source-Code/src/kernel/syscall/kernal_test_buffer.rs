use spin::Mutex;

// Maximallänge des Buffers
pub const BUFFERLENGTH: usize = 2048;
// Buffer zum speichern
pub static BUFFER: Mutex<[u8; BUFFERLENGTH]> = Mutex::new([0; BUFFERLENGTH]);
// gibt die aktuelle Länge des Inhaltes an
pub static CURRENTLENGHT: Mutex<u64> = Mutex::new(0);
// Main lock, welches zuerst geholt werden muss um nicht in dead-locks zu kommen
pub static MAINBUFFERLOCK: Mutex<bool> = Mutex::new(false);
