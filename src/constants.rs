pub const HID_KEYBOARD: u16 = 0x03C1;
pub const KEYBOARD_ID: u8 = 0x01;
pub const MEDIA_KEYS_ID: u8 = 0x02;
pub const VENDOR_ID: u16 = 0x01;
pub const PRODUCT_ID: u16 = 0x01;
pub const VERSION: u16 = 0x01;

pub const HID_REPORT_DESCRIPTOR: &'static [u8] = &[
    0x01, // USAGE_PAGE (Generic Desktop Ctrls)
    0x06, // USAGE (Keyboard)
    0x01, // COLLECTION (Application)
    // -------------------------------- Keyboard
    KEYBOARD_ID, //   REPORT_ID (1)
    0x07,        //   USAGE_PAGE (Kbrd/Keypad)
    0xE0,        //   USAGE_MINIMUM (0xE0)
    0xE7,        //   USAGE_MAXIMUM (0xE7)
    0x00,        //   LOGICAL_MINIMUM (0)
    0x01,        //   Logical Maximum (1)
    0x01,        //   REPORT_SIZE (1)
    0x08,        //   REPORT_COUNT (8)
    0x02,        //   INPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x01,        //   REPORT_COUNT (1) ; 1 byte (Reserved)
    0x08,        //   REPORT_SIZE (8)
    0x01,        //   INPUT (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x05,        //   REPORT_COUNT (5) ; 5 bits (Num lock, Caps lock, Scroll lock, Compose, Kana)
    0x01,        //   REPORT_SIZE (1)
    0x08,        //   USAGE_PAGE (LEDs)
    0x01,        //   USAGE_MINIMUM (0x01) ; Num Lock
    0x05,        //   USAGE_MAXIMUM (0x05) ; Kana
    0x02, //   OUTPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x01, //   REPORT_COUNT (1) ; 3 bits (Padding)
    0x03, //   REPORT_SIZE (3)
    0x01, //   OUTPUT (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x06, //   REPORT_COUNT (6) ; 6 bytes (Keys)
    0x08, //   REPORT_SIZE(8)
    0x00, //   LOGICAL_MINIMUM(0)
    0x65, //   LOGICAL_MAXIMUM(0x65) ; 101 keys
    0x07, //   USAGE_PAGE (Kbrd/Keypad)
    0x00, //   USAGE_MINIMUM (0)
    0x65, //   USAGE_MAXIMUM (0x65)
    0x00, //   INPUT (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    // END_COLLECTION
    // -------------------------------- Media Keys
    0x0C,          // USAGE_PAGE (Consumer)
    0x01,          // USAGE (Consumer Control)
    0x01,          // COLLECTION (Application)
    MEDIA_KEYS_ID, //   REPORT_ID (3)
    0x0C,          //   USAGE_PAGE (Consumer)
    0x00,          //   LOGICAL_MINIMUM (0)
    0x01,          //   LOGICAL_MAXIMUM (1)
    0x01,          //   REPORT_SIZE (1)
    0x10,          //   REPORT_COUNT (16)
    0xB5,          //   USAGE (Scan Next Track)     ; bit 0: 1
    0xB6,          //   USAGE (Scan Previous Track) ; bit 1: 2
    0xB7,          //   USAGE (Stop)                ; bit 2: 4
    0xCD,          //   USAGE (Play/Pause)          ; bit 3: 8
    0xE2,          //   USAGE (Mute)                ; bit 4: 16
    0xE9,          //   USAGE (Volume Increment)    ; bit 5: 32
    0xEA,          //   USAGE (Volume Decrement)    ; bit 6: 64
    0x23,
    0x02, //   Usage (WWW Home)            ; bit 7: 128
    0x94,
    0x01, //   Usage (My Computer) ; bit 0: 1
    0x92,
    0x01, //   Usage (Calculator)  ; bit 1: 2
    0x2A,
    0x02, //   Usage (WWW fav)     ; bit 2: 4
    0x21,
    0x02, //   Usage (WWW search)  ; bit 3: 8
    0x26,
    0x02, //   Usage (WWW stop)    ; bit 4: 16
    0x24,
    0x02, //   Usage (WWW back)    ; bit 5: 32
    0x83,
    0x01, //   Usage (Media sel)   ; bit 6: 64
    0x8A,
    0x01, //   Usage (Mail)        ; bit 7: 128
    0x02, //   INPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
];

pub const KEY_LEFT_CTRL: u8 = 0x80;
pub const KEY_LEFT_SHIFT: u8 = 0x81;
pub const KEY_LEFT_ALT: u8 = 0x82;
pub const KEY_LEFT_GUI: u8 = 0x83;
pub const KEY_RIGHT_CTRL: u8 = 0x84;
pub const KEY_RIGHT_SHIFT: u8 = 0x85;
pub const KEY_RIGHT_ALT: u8 = 0x86;
pub const KEY_RIGHT_GUI: u8 = 0x87;
pub const KEY_UP_ARROW: u8 = 0xDA;
pub const KEY_DOWN_ARROW: u8 = 0xD9;
pub const KEY_LEFT_ARROW: u8 = 0xD8;
pub const KEY_RIGHT_ARROW: u8 = 0xD7;
pub const KEY_BACKSPACE: u8 = 0xB2;
pub const KEY_TAB: u8 = 0xB3;
pub const KEY_RETURN: u8 = 0xB0;
pub const KEY_ESC: u8 = 0xB1;
pub const KEY_INSERT: u8 = 0xD1;
pub const KEY_PRTSC: u8 = 0xCE;
pub const KEY_DELETE: u8 = 0xD4;
pub const KEY_PAGE_UP: u8 = 0xD3;
pub const KEY_PAGE_DOWN: u8 = 0xD6;
pub const KEY_HOME: u8 = 0xD2;
pub const KEY_END: u8 = 0xD5;
pub const KEY_CAPS_LOCK: u8 = 0xC1;
pub const KEY_F1: u8 = 0xC2;
pub const KEY_F2: u8 = 0xC3;
pub const KEY_F3: u8 = 0xC4;
pub const KEY_F4: u8 = 0xC5;
pub const KEY_F5: u8 = 0xC6;
pub const KEY_F6: u8 = 0xC7;
pub const KEY_F7: u8 = 0xC8;
pub const KEY_F8: u8 = 0xC9;
pub const KEY_F9: u8 = 0xCA;
pub const KEY_F10: u8 = 0xCB;
pub const KEY_F11: u8 = 0xCC;
pub const KEY_F12: u8 = 0xCD;
pub const KEY_F13: u8 = 0xF0;
pub const KEY_F14: u8 = 0xF1;
pub const KEY_F15: u8 = 0xF2;
pub const KEY_F16: u8 = 0xF3;
pub const KEY_F17: u8 = 0xF4;
pub const KEY_F18: u8 = 0xF5;
pub const KEY_F19: u8 = 0xF6;
pub const KEY_F20: u8 = 0xF7;
pub const KEY_F21: u8 = 0xF8;
pub const KEY_F22: u8 = 0xF9;
pub const KEY_F23: u8 = 0xFA;
pub const KEY_F24: u8 = 0xFB;
pub const KEY_NUM_0: u8 = 0xEA;
pub const KEY_NUM_1: u8 = 0xE1;
pub const KEY_NUM_2: u8 = 0xE2;
pub const KEY_NUM_3: u8 = 0xE3;
pub const KEY_NUM_4: u8 = 0xE4;
pub const KEY_NUM_5: u8 = 0xE5;
pub const KEY_NUM_6: u8 = 0xE6;
pub const KEY_NUM_7: u8 = 0xE7;
pub const KEY_NUM_8: u8 = 0xE8;
pub const KEY_NUM_9: u8 = 0xE9;
pub const KEY_NUM_SLASH: u8 = 0xDC;
pub const KEY_NUM_ASTERISK: u8 = 0xDD;
pub const KEY_NUM_MINUS: u8 = 0xDE;
pub const KEY_NUM_PLUS: u8 = 0xDF;
pub const KEY_NUM_ENTER: u8 = 0xE0;
pub const KEY_NUM_PERIOD: u8 = 0xEB;

#[repr(C)]
pub struct MediaKey([u8; 2]);
pub const KEY_MEDIA_NEXT_TRACK: MediaKey = MediaKey { 0: [1, 0] };
pub const KEY_MEDIA_PREVIOUS_TRACK: MediaKey = MediaKey { 0: [2, 0] };
pub const KEY_MEDIA_STOP: MediaKey = MediaKey { 0: [4, 0] };
pub const KEY_MEDIA_PLAY_PAUSE: MediaKey = MediaKey { 0: [8, 0] };
pub const KEY_MEDIA_MUTE: MediaKey = MediaKey { 0: [16, 0] };
pub const KEY_MEDIA_VOLUME_UP: MediaKey = MediaKey { 0: [32, 0] };
pub const KEY_MEDIA_VOLUME_DOWN: MediaKey = MediaKey { 0: [64, 0] };
pub const KEY_MEDIA_WWW_HOME: MediaKey = MediaKey { 0: [128, 0] };
pub const KEY_MEDIA_LOCAL_MACHINE_BROWSER: MediaKey = MediaKey { 0: [0, 1] }; // Opens "My Computer" on Windows
pub const KEY_MEDIA_CALCULATOR: MediaKey = MediaKey { 0: [0, 2] };
pub const KEY_MEDIA_WWW_BOOKMARKS: MediaKey = MediaKey { 0: [0, 4] };
pub const KEY_MEDIA_WWW_SEARCH: MediaKey = MediaKey { 0: [0, 8] };
pub const KEY_MEDIA_WWW_STOP: MediaKey = MediaKey { 0: [0, 16] };
pub const KEY_MEDIA_WWW_BACK: MediaKey = MediaKey { 0: [0, 32] };
pub const KEY_MEDIA_CONSUMER_CONTROL_CONFIGURATION: MediaKey = MediaKey { 0: [0, 64] }; // Media Selection
pub const KEY_MEDIA_EMAIL_READER: MediaKey = MediaKey { 0: [0, 128] };

pub const SHIFT: u8 = 0x80;
pub const ASCIIMAP: [u8; 128] = [
    0x00,         // NUL
    0x00,         // SOH
    0x00,         // STX
    0x00,         // ETX
    0x00,         // EOT
    0x00,         // ENQ
    0x00,         // ACK
    0x00,         // BEL
    0x2a,         // BS	Backspace
    0x2b,         // TAB	Tab
    0x28,         // LF	Enter
    0x00,         // VT
    0x00,         // FF
    0x00,         // CR
    0x00,         // SO
    0x00,         // SI
    0x00,         // DEL
    0x00,         // DC1
    0x00,         // DC2
    0x00,         // DC3
    0x00,         // DC4
    0x00,         // NAK
    0x00,         // SYN
    0x00,         // ETB
    0x00,         // CAN
    0x00,         // EM
    0x00,         // SUB
    0x00,         // ESC
    0x00,         // FS
    0x00,         // GS
    0x00,         // RS
    0x00,         // US
    0x2c,         //  ' '
    0x1e | SHIFT, // !
    0x34 | SHIFT, // "
    0x20 | SHIFT, // #
    0x21 | SHIFT, // $
    0x22 | SHIFT, // %
    0x24 | SHIFT, // &
    0x34,         // '
    0x26 | SHIFT, // (
    0x27 | SHIFT, // )
    0x25 | SHIFT, // *
    0x2e | SHIFT, // +
    0x36,         // ,
    0x2d,         // -
    0x37,         // .
    0x38,         // /
    0x27,         // 0
    0x1e,         // 1
    0x1f,         // 2
    0x20,         // 3
    0x21,         // 4
    0x22,         // 5
    0x23,         // 6
    0x24,         // 7
    0x25,         // 8
    0x26,         // 9
    0x33 | SHIFT, // :
    0x33,         // ;
    0x36 | SHIFT, // <
    0x2e,         // =
    0x37 | SHIFT, // >
    0x38 | SHIFT, // ?
    0x1f | SHIFT, // @
    0x04 | SHIFT, // A
    0x05 | SHIFT, // B
    0x06 | SHIFT, // C
    0x07 | SHIFT, // D
    0x08 | SHIFT, // E
    0x09 | SHIFT, // F
    0x0a | SHIFT, // G
    0x0b | SHIFT, // H
    0x0c | SHIFT, // I
    0x0d | SHIFT, // J
    0x0e | SHIFT, // K
    0x0f | SHIFT, // L
    0x10 | SHIFT, // M
    0x11 | SHIFT, // N
    0x12 | SHIFT, // O
    0x13 | SHIFT, // P
    0x14 | SHIFT, // Q
    0x15 | SHIFT, // R
    0x16 | SHIFT, // S
    0x17 | SHIFT, // T
    0x18 | SHIFT, // U
    0x19 | SHIFT, // V
    0x1a | SHIFT, // W
    0x1b | SHIFT, // X
    0x1c | SHIFT, // Y
    0x1d | SHIFT, // Z
    0x2f,         // [
    0x31,         // bslash
    0x30,         // ]
    0x23 | SHIFT, // ^
    0x2d | SHIFT, // _
    0x35,         // `
    0x04,         // a
    0x05,         // b
    0x06,         // c
    0x07,         // d
    0x08,         // e
    0x09,         // f
    0x0a,         // g
    0x0b,         // h
    0x0c,         // i
    0x0d,         // j
    0x0e,         // k
    0x0f,         // l
    0x10,         // m
    0x11,         // n
    0x12,         // o
    0x13,         // p
    0x14,         // q
    0x15,         // r
    0x16,         // s
    0x17,         // t
    0x18,         // u
    0x19,         // v
    0x1a,         // w
    0x1b,         // x
    0x1c,         // y
    0x1d,         // z
    0x2f | SHIFT, // {
    0x31 | SHIFT, // |
    0x30 | SHIFT, // }
    0x35 | SHIFT, // ~
    0,            // DEL
];

pub const MAX_KEYPRESSES: usize = 6;

// LED Usage ID for Num Lock
pub const LED_NUM_LOCK: u8 = 0x01;

// LED Usage ID for Caps Lock
pub const LED_CAPS_LOCK: u8 = 0x02;

// LED Usage ID for Scroll Lock
pub const LED_SCROLL_LOCK: u8 = 0x04;

// LED Usage ID for Compose
pub const LED_COMPOSE: u8 = 0x08;
