#[allow(dead_code)]
pub enum Opcode {
    #[allow(non_camel_case_types)]
    Error = 0x00,
    #[allow(non_camel_case_types)]
    Nop = 0x01,
    #[allow(non_camel_case_types)]
    ProgramStop = 0x02,
    #[allow(non_camel_case_types)]
    ProgramStart = 0x03,
    #[allow(non_camel_case_types)]
    ObjectStop = 0x04,
    #[allow(non_camel_case_types)]
    ObjectStart = 0x05,
    #[allow(non_camel_case_types)]
    ObjectTrig = 0x06,
    #[allow(non_camel_case_types)]
    ObjectWait = 0x07,
    #[allow(non_camel_case_types)]
    Return = 0x08,
    #[allow(non_camel_case_types)]
    Call = 0x09,
    #[allow(non_camel_case_types)]
    ObjectEnd = 0x0A,
    #[allow(non_camel_case_types)]
    Sleep = 0x0B,
    #[allow(non_camel_case_types)]
    ProgramInfo = 0x0C,
    #[allow(non_camel_case_types)]
    Label = 0x0D,
    #[allow(non_camel_case_types)]
    Probe = 0x0E,
    #[allow(non_camel_case_types)]
    Do = 0x0F,
    #[allow(non_camel_case_types)]
    Add8 = 0x10,
    #[allow(non_camel_case_types)]
    Add16 = 0x11,
    #[allow(non_camel_case_types)]
    Add32 = 0x12,
    #[allow(non_camel_case_types)]
    AddF = 0x13,
    #[allow(non_camel_case_types)]
    Sub8 = 0x14,
    #[allow(non_camel_case_types)]
    Sub16 = 0x15,
    #[allow(non_camel_case_types)]
    Sub32 = 0x16,
    #[allow(non_camel_case_types)]
    SubF = 0x17,
    #[allow(non_camel_case_types)]
    Mul8 = 0x18,
    #[allow(non_camel_case_types)]
    Mul16 = 0x19,
    #[allow(non_camel_case_types)]
    Mul32 = 0x1A,
    #[allow(non_camel_case_types)]
    MulF = 0x1B,
    #[allow(non_camel_case_types)]
    Div8 = 0x1C,
    #[allow(non_camel_case_types)]
    Div16 = 0x1D,
    #[allow(non_camel_case_types)]
    Div32 = 0x1E,
    #[allow(non_camel_case_types)]
    DivF = 0x1F,
    #[allow(non_camel_case_types)]
    Or8 = 0x20,
    #[allow(non_camel_case_types)]
    Or16 = 0x21,
    #[allow(non_camel_case_types)]
    Or32 = 0x22,
    #[allow(non_camel_case_types)]
    And8 = 0x24,
    #[allow(non_camel_case_types)]
    And16 = 0x25,
    #[allow(non_camel_case_types)]
    And32 = 0x26,
    #[allow(non_camel_case_types)]
    Xor8 = 0x28,
    #[allow(non_camel_case_types)]
    Xor16 = 0x29,
    #[allow(non_camel_case_types)]
    Xor32 = 0x2A,
    #[allow(non_camel_case_types)]
    Rl8 = 0x2C,
    #[allow(non_camel_case_types)]
    Rl16 = 0x2D,
    #[allow(non_camel_case_types)]
    Rl32 = 0x2E,
    #[allow(non_camel_case_types)]
    InitBytes = 0x2F,
    #[allow(non_camel_case_types)]
    Move8_8 = 0x30,
    #[allow(non_camel_case_types)]
    Move8_16 = 0x31,
    #[allow(non_camel_case_types)]
    Move8_32 = 0x32,
    #[allow(non_camel_case_types)]
    Move8_F = 0x33,
    #[allow(non_camel_case_types)]
    Move16_8 = 0x34,
    #[allow(non_camel_case_types)]
    Move16_16 = 0x35,
    #[allow(non_camel_case_types)]
    Move16_32 = 0x36,
    #[allow(non_camel_case_types)]
    Move16_F = 0x37,
    #[allow(non_camel_case_types)]
    Move32_8 = 0x38,
    #[allow(non_camel_case_types)]
    Move32_16 = 0x39,
    #[allow(non_camel_case_types)]
    Move32_32 = 0x3A,
    #[allow(non_camel_case_types)]
    Move32_F = 0x3B,
    #[allow(non_camel_case_types)]
    MoveF_8 = 0x3C,
    #[allow(non_camel_case_types)]
    MoveF_16 = 0x3D,
    #[allow(non_camel_case_types)]
    MoveF_32 = 0x3E,
    #[allow(non_camel_case_types)]
    MoveF_F = 0x3F,
    #[allow(non_camel_case_types)]
    Jr = 0x40,
    #[allow(non_camel_case_types)]
    JrFalse = 0x41,
    #[allow(non_camel_case_types)]
    JrTrue = 0x42,
    #[allow(non_camel_case_types)]
    JrNan = 0x43,
    #[allow(non_camel_case_types)]
    CpLt8 = 0x44,
    #[allow(non_camel_case_types)]
    CpLt16 = 0x45,
    #[allow(non_camel_case_types)]
    CpLt32 = 0x46,
    #[allow(non_camel_case_types)]
    CpLtf = 0x47,
    #[allow(non_camel_case_types)]
    CpGt8 = 0x48,
    #[allow(non_camel_case_types)]
    CpGt16 = 0x49,
    #[allow(non_camel_case_types)]
    CpGt32 = 0x4A,
    #[allow(non_camel_case_types)]
    CpGtf = 0x4B,
    #[allow(non_camel_case_types)]
    CpEq8 = 0x4C,
    #[allow(non_camel_case_types)]
    CpEq16 = 0x4D,
    #[allow(non_camel_case_types)]
    CpEq32 = 0x4E,
    #[allow(non_camel_case_types)]
    CpEqf = 0x4F,
    #[allow(non_camel_case_types)]
    CpNeq8 = 0x50,
    #[allow(non_camel_case_types)]
    CpNeq16 = 0x51,
    #[allow(non_camel_case_types)]
    CpNeq32 = 0x52,
    #[allow(non_camel_case_types)]
    CpNeqf = 0x53,
    #[allow(non_camel_case_types)]
    CpLteq8 = 0x54,
    #[allow(non_camel_case_types)]
    CpLteq16 = 0x55,
    #[allow(non_camel_case_types)]
    CpLteq32 = 0x56,
    #[allow(non_camel_case_types)]
    CpLteqf = 0x57,
    #[allow(non_camel_case_types)]
    CpGteq8 = 0x58,
    #[allow(non_camel_case_types)]
    CpGteq16 = 0x59,
    #[allow(non_camel_case_types)]
    CpGteq32 = 0x5A,
    #[allow(non_camel_case_types)]
    CpGteqf = 0x5B,
    #[allow(non_camel_case_types)]
    Select8 = 0x5C,
    #[allow(non_camel_case_types)]
    Select16 = 0x5D,
    #[allow(non_camel_case_types)]
    Select32 = 0x5E,
    #[allow(non_camel_case_types)]
    Selectf = 0x5F,
    #[allow(non_camel_case_types)]
    System = 0x60,
    #[allow(non_camel_case_types)]
    PortCnvOutput = 0x61,
    #[allow(non_camel_case_types)]
    PortCnvInput = 0x62,
    #[allow(non_camel_case_types)]
    NoteToFreq = 0x63,
    #[allow(non_camel_case_types)]
    JrLt8 = 0x64,
    #[allow(non_camel_case_types)]
    JrLt16 = 0x65,
    #[allow(non_camel_case_types)]
    JrLt32 = 0x66,
    #[allow(non_camel_case_types)]
    JrLtf = 0x67,
    #[allow(non_camel_case_types)]
    JrGt8 = 0x68,
    #[allow(non_camel_case_types)]
    JrGt16 = 0x69,
    #[allow(non_camel_case_types)]
    JrGt32 = 0x6A,
    #[allow(non_camel_case_types)]
    JrGtf = 0x6B,
    #[allow(non_camel_case_types)]
    JrEq8 = 0x6C,
    #[allow(non_camel_case_types)]
    JrEq16 = 0x6D,
    #[allow(non_camel_case_types)]
    JrEq32 = 0x6E,
    #[allow(non_camel_case_types)]
    JrEqf = 0x6F,
    #[allow(non_camel_case_types)]
    JrNeq8 = 0x70,
    #[allow(non_camel_case_types)]
    JrNeq16 = 0x71,
    #[allow(non_camel_case_types)]
    JrNeq32 = 0x72,
    #[allow(non_camel_case_types)]
    JrNeqf = 0x73,
    #[allow(non_camel_case_types)]
    JrLteq8 = 0x74,
    #[allow(non_camel_case_types)]
    JrLteq16 = 0x75,
    #[allow(non_camel_case_types)]
    JrLteq32 = 0x76,
    #[allow(non_camel_case_types)]
    JrLteqf = 0x77,
    #[allow(non_camel_case_types)]
    JrGteq8 = 0x78,
    #[allow(non_camel_case_types)]
    JrGteq16 = 0x79,
    #[allow(non_camel_case_types)]
    JrGteq32 = 0x7A,
    #[allow(non_camel_case_types)]
    JrGteqf = 0x7B,
    #[allow(non_camel_case_types)]
    Info = 0x7C,
    #[allow(non_camel_case_types)]
    String = 0x7D,
    #[allow(non_camel_case_types)]
    MemoryWrite = 0x7E,
    #[allow(non_camel_case_types)]
    MemoryRead = 0x7F,
    #[allow(non_camel_case_types)]
    UiFlush = 0x80,
    #[allow(non_camel_case_types)]
    UiRead = 0x81,
    #[allow(non_camel_case_types)]
    UiWrite = 0x82,
    #[allow(non_camel_case_types)]
    UiButton = 0x83,
    #[allow(non_camel_case_types)]
    UiDraw = 0x84,
    #[allow(non_camel_case_types)]
    TimerWait = 0x85,
    #[allow(non_camel_case_types)]
    TimerReady = 0x86,
    #[allow(non_camel_case_types)]
    TimerRead = 0x87,
    #[allow(non_camel_case_types)]
    Bp0 = 0x88,
    #[allow(non_camel_case_types)]
    Bp1 = 0x89,
    #[allow(non_camel_case_types)]
    Bp2 = 0x8A,
    #[allow(non_camel_case_types)]
    Bp3 = 0x8B,
    #[allow(non_camel_case_types)]
    BpSet = 0x8C,
    #[allow(non_camel_case_types)]
    Math = 0x8D,
    #[allow(non_camel_case_types)]
    Random = 0x8E,
    #[allow(non_camel_case_types)]
    TimerReadUs = 0x8F,
    #[allow(non_camel_case_types)]
    KeepAlive = 0x90,
    #[allow(non_camel_case_types)]
    ComRead = 0x91,
    #[allow(non_camel_case_types)]
    ComWrite = 0x92,
    #[allow(non_camel_case_types)]
    Sound = 0x94,
    #[allow(non_camel_case_types)]
    SoundTest = 0x95,
    #[allow(non_camel_case_types)]
    SoundReady = 0x96,
    #[allow(non_camel_case_types)]
    InputSample = 0x97,
    #[allow(non_camel_case_types)]
    InputDeviceList = 0x98,
    #[allow(non_camel_case_types)]
    InputDevice = 0x99,
    #[allow(non_camel_case_types)]
    InputRead = 0x9A,
    #[allow(non_camel_case_types)]
    InputTest = 0x9B,
    #[allow(non_camel_case_types)]
    InputReady = 0x9C,
    #[allow(non_camel_case_types)]
    InputReadSi = 0x9D,
    #[allow(non_camel_case_types)]
    InputReadExt = 0x9E,
    #[allow(non_camel_case_types)]
    InputWrite = 0x9F,
    #[allow(non_camel_case_types)]
    OutputGetType = 0xA0,
    #[allow(non_camel_case_types)]
    OutputSetType = 0xA1,
    #[allow(non_camel_case_types)]
    OutputReset = 0xA2,
    #[allow(non_camel_case_types)]
    OutputStop = 0xA3,
    #[allow(non_camel_case_types)]
    OutputPower = 0xA4,
    #[allow(non_camel_case_types)]
    OutputSpeed = 0xA5,
    #[allow(non_camel_case_types)]
    OutputStart = 0xA6,
    #[allow(non_camel_case_types)]
    OutputPolarity = 0xA7,
    #[allow(non_camel_case_types)]
    OutputRead = 0xA8,
    #[allow(non_camel_case_types)]
    OutputTest = 0xA9,
    #[allow(non_camel_case_types)]
    OutputReady = 0xAA,
    #[allow(non_camel_case_types)]
    OutputPosition = 0xAB,
    #[allow(non_camel_case_types)]
    OutputStepPower = 0xAC,
    #[allow(non_camel_case_types)]
    OutputTimePower = 0xAD,
    #[allow(non_camel_case_types)]
    OutputStepSpeed = 0xAE,
    #[allow(non_camel_case_types)]
    OutputTimeSpeed = 0xAF,
    #[allow(non_camel_case_types)]
    OutputStepSync = 0xB0,
    #[allow(non_camel_case_types)]
    OutputTimeSync = 0xB1,
    #[allow(non_camel_case_types)]
    OutputClrCount = 0xB2,
    #[allow(non_camel_case_types)]
    OutputGetCount = 0xB3,
    #[allow(non_camel_case_types)]
    OutputPrgStop = 0xB4,
    #[allow(non_camel_case_types)]
    File = 0xC0,
    #[allow(non_camel_case_types)]
    Array = 0xC1,
    #[allow(non_camel_case_types)]
    ArrayWrite = 0xC2,
    #[allow(non_camel_case_types)]
    ArrayRead = 0xC3,
    #[allow(non_camel_case_types)]
    ArrayAppend = 0xC4,
    #[allow(non_camel_case_types)]
    MemoryUsage = 0xC5,
    #[allow(non_camel_case_types)]
    Filename = 0xC6,
    #[allow(non_camel_case_types)]
    Read8 = 0xC8,
    #[allow(non_camel_case_types)]
    Read16 = 0xC9,
    #[allow(non_camel_case_types)]
    Read32 = 0xCA,
    #[allow(non_camel_case_types)]
    Readf = 0xCB,
    #[allow(non_camel_case_types)]
    Write8 = 0xCC,
    #[allow(non_camel_case_types)]
    Write16 = 0xCD,
    #[allow(non_camel_case_types)]
    Write32 = 0xCE,
    #[allow(non_camel_case_types)]
    Writef = 0xCF,
    #[allow(non_camel_case_types)]
    ComReady = 0xD0,
    #[allow(non_camel_case_types)]
    ComReaddata = 0xD1,
    #[allow(non_camel_case_types)]
    ComWritedata = 0xD2,
    #[allow(non_camel_case_types)]
    ComGet = 0xD3,
    #[allow(non_camel_case_types)]
    ComSet = 0xD4,
    #[allow(non_camel_case_types)]
    ComTest = 0xD5,
    #[allow(non_camel_case_types)]
    ComRemove = 0xD6,
    #[allow(non_camel_case_types)]
    ComWritefile = 0xD7,
    #[allow(non_camel_case_types)]
    MailboxOpen = 0xD8,
    #[allow(non_camel_case_types)]
    MailboxWrite = 0xD9,
    #[allow(non_camel_case_types)]
    MailboxRead = 0xDA,
    #[allow(non_camel_case_types)]
    MailboxTest = 0xDB,
    #[allow(non_camel_case_types)]
    MailboxReady = 0xDC,
    #[allow(non_camel_case_types)]
    MailboxClose = 0xDD,
    #[allow(non_camel_case_types)]
    Tst = 0xFF,
}

impl Opcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Opcode, &'static str> {
        match i {
            0x00 => Ok(Opcode::Error),
            0x01 => Ok(Opcode::Nop),
            0x02 => Ok(Opcode::ProgramStop),
            0x03 => Ok(Opcode::ProgramStart),
            0x04 => Ok(Opcode::ObjectStop),
            0x05 => Ok(Opcode::ObjectStart),
            0x06 => Ok(Opcode::ObjectTrig),
            0x07 => Ok(Opcode::ObjectWait),
            0x08 => Ok(Opcode::Return),
            0x09 => Ok(Opcode::Call),
            0x0A => Ok(Opcode::ObjectEnd),
            0x0B => Ok(Opcode::Sleep),
            0x0C => Ok(Opcode::ProgramInfo),
            0x0D => Ok(Opcode::Label),
            0x0E => Ok(Opcode::Probe),
            0x0F => Ok(Opcode::Do),
            0x10 => Ok(Opcode::Add8),
            0x11 => Ok(Opcode::Add16),
            0x12 => Ok(Opcode::Add32),
            0x13 => Ok(Opcode::AddF),
            0x14 => Ok(Opcode::Sub8),
            0x15 => Ok(Opcode::Sub16),
            0x16 => Ok(Opcode::Sub32),
            0x17 => Ok(Opcode::SubF),
            0x18 => Ok(Opcode::Mul8),
            0x19 => Ok(Opcode::Mul16),
            0x1A => Ok(Opcode::Mul32),
            0x1B => Ok(Opcode::MulF),
            0x1C => Ok(Opcode::Div8),
            0x1D => Ok(Opcode::Div16),
            0x1E => Ok(Opcode::Div32),
            0x1F => Ok(Opcode::DivF),
            0x20 => Ok(Opcode::Or8),
            0x21 => Ok(Opcode::Or16),
            0x22 => Ok(Opcode::Or32),
            0x24 => Ok(Opcode::And8),
            0x25 => Ok(Opcode::And16),
            0x26 => Ok(Opcode::And32),
            0x28 => Ok(Opcode::Xor8),
            0x29 => Ok(Opcode::Xor16),
            0x2A => Ok(Opcode::Xor32),
            0x2C => Ok(Opcode::Rl8),
            0x2D => Ok(Opcode::Rl16),
            0x2E => Ok(Opcode::Rl32),
            0x2F => Ok(Opcode::InitBytes),
            0x30 => Ok(Opcode::Move8_8),
            0x31 => Ok(Opcode::Move8_16),
            0x32 => Ok(Opcode::Move8_32),
            0x33 => Ok(Opcode::Move8_F),
            0x34 => Ok(Opcode::Move16_8),
            0x35 => Ok(Opcode::Move16_16),
            0x36 => Ok(Opcode::Move16_32),
            0x37 => Ok(Opcode::Move16_F),
            0x38 => Ok(Opcode::Move32_8),
            0x39 => Ok(Opcode::Move32_16),
            0x3A => Ok(Opcode::Move32_32),
            0x3B => Ok(Opcode::Move32_F),
            0x3C => Ok(Opcode::MoveF_8),
            0x3D => Ok(Opcode::MoveF_16),
            0x3E => Ok(Opcode::MoveF_32),
            0x3F => Ok(Opcode::MoveF_F),
            0x40 => Ok(Opcode::Jr),
            0x41 => Ok(Opcode::JrFalse),
            0x42 => Ok(Opcode::JrTrue),
            0x43 => Ok(Opcode::JrNan),
            0x44 => Ok(Opcode::CpLt8),
            0x45 => Ok(Opcode::CpLt16),
            0x46 => Ok(Opcode::CpLt32),
            0x47 => Ok(Opcode::CpLtf),
            0x48 => Ok(Opcode::CpGt8),
            0x49 => Ok(Opcode::CpGt16),
            0x4A => Ok(Opcode::CpGt32),
            0x4B => Ok(Opcode::CpGtf),
            0x4C => Ok(Opcode::CpEq8),
            0x4D => Ok(Opcode::CpEq16),
            0x4E => Ok(Opcode::CpEq32),
            0x4F => Ok(Opcode::CpEqf),
            0x50 => Ok(Opcode::CpNeq8),
            0x51 => Ok(Opcode::CpNeq16),
            0x52 => Ok(Opcode::CpNeq32),
            0x53 => Ok(Opcode::CpNeqf),
            0x54 => Ok(Opcode::CpLteq8),
            0x55 => Ok(Opcode::CpLteq16),
            0x56 => Ok(Opcode::CpLteq32),
            0x57 => Ok(Opcode::CpLteqf),
            0x58 => Ok(Opcode::CpGteq8),
            0x59 => Ok(Opcode::CpGteq16),
            0x5A => Ok(Opcode::CpGteq32),
            0x5B => Ok(Opcode::CpGteqf),
            0x5C => Ok(Opcode::Select8),
            0x5D => Ok(Opcode::Select16),
            0x5E => Ok(Opcode::Select32),
            0x5F => Ok(Opcode::Selectf),
            0x60 => Ok(Opcode::System),
            0x61 => Ok(Opcode::PortCnvOutput),
            0x62 => Ok(Opcode::PortCnvInput),
            0x63 => Ok(Opcode::NoteToFreq),
            0x64 => Ok(Opcode::JrLt8),
            0x65 => Ok(Opcode::JrLt16),
            0x66 => Ok(Opcode::JrLt32),
            0x67 => Ok(Opcode::JrLtf),
            0x68 => Ok(Opcode::JrGt8),
            0x69 => Ok(Opcode::JrGt16),
            0x6A => Ok(Opcode::JrGt32),
            0x6B => Ok(Opcode::JrGtf),
            0x6C => Ok(Opcode::JrEq8),
            0x6D => Ok(Opcode::JrEq16),
            0x6E => Ok(Opcode::JrEq32),
            0x6F => Ok(Opcode::JrEqf),
            0x70 => Ok(Opcode::JrNeq8),
            0x71 => Ok(Opcode::JrNeq16),
            0x72 => Ok(Opcode::JrNeq32),
            0x73 => Ok(Opcode::JrNeqf),
            0x74 => Ok(Opcode::JrLteq8),
            0x75 => Ok(Opcode::JrLteq16),
            0x76 => Ok(Opcode::JrLteq32),
            0x77 => Ok(Opcode::JrLteqf),
            0x78 => Ok(Opcode::JrGteq8),
            0x79 => Ok(Opcode::JrGteq16),
            0x7A => Ok(Opcode::JrGteq32),
            0x7B => Ok(Opcode::JrGteqf),
            0x7C => Ok(Opcode::Info),
            0x7D => Ok(Opcode::String),
            0x7E => Ok(Opcode::MemoryWrite),
            0x7F => Ok(Opcode::MemoryRead),
            0x80 => Ok(Opcode::UiFlush),
            0x81 => Ok(Opcode::UiRead),
            0x82 => Ok(Opcode::UiWrite),
            0x83 => Ok(Opcode::UiButton),
            0x84 => Ok(Opcode::UiDraw),
            0x85 => Ok(Opcode::TimerWait),
            0x86 => Ok(Opcode::TimerReady),
            0x87 => Ok(Opcode::TimerRead),
            0x88 => Ok(Opcode::Bp0),
            0x89 => Ok(Opcode::Bp1),
            0x8A => Ok(Opcode::Bp2),
            0x8B => Ok(Opcode::Bp3),
            0x8C => Ok(Opcode::BpSet),
            0x8D => Ok(Opcode::Math),
            0x8E => Ok(Opcode::Random),
            0x8F => Ok(Opcode::TimerReadUs),
            0x90 => Ok(Opcode::KeepAlive),
            0x91 => Ok(Opcode::ComRead),
            0x92 => Ok(Opcode::ComWrite),
            0x94 => Ok(Opcode::Sound),
            0x95 => Ok(Opcode::SoundTest),
            0x96 => Ok(Opcode::SoundReady),
            0x97 => Ok(Opcode::InputSample),
            0x98 => Ok(Opcode::InputDeviceList),
            0x99 => Ok(Opcode::InputDevice),
            0x9A => Ok(Opcode::InputRead),
            0x9B => Ok(Opcode::InputTest),
            0x9C => Ok(Opcode::InputReady),
            0x9D => Ok(Opcode::InputReadSi),
            0x9E => Ok(Opcode::InputReadExt),
            0x9F => Ok(Opcode::InputWrite),
            0xA0 => Ok(Opcode::OutputGetType),
            0xA1 => Ok(Opcode::OutputSetType),
            0xA2 => Ok(Opcode::OutputReset),
            0xA3 => Ok(Opcode::OutputStop),
            0xA4 => Ok(Opcode::OutputPower),
            0xA5 => Ok(Opcode::OutputSpeed),
            0xA6 => Ok(Opcode::OutputStart),
            0xA7 => Ok(Opcode::OutputPolarity),
            0xA8 => Ok(Opcode::OutputRead),
            0xA9 => Ok(Opcode::OutputTest),
            0xAA => Ok(Opcode::OutputReady),
            0xAB => Ok(Opcode::OutputPosition),
            0xAC => Ok(Opcode::OutputStepPower),
            0xAD => Ok(Opcode::OutputTimePower),
            0xAE => Ok(Opcode::OutputStepSpeed),
            0xAF => Ok(Opcode::OutputTimeSpeed),
            0xB0 => Ok(Opcode::OutputStepSync),
            0xB1 => Ok(Opcode::OutputTimeSync),
            0xB2 => Ok(Opcode::OutputClrCount),
            0xB3 => Ok(Opcode::OutputGetCount),
            0xB4 => Ok(Opcode::OutputPrgStop),
            0xC0 => Ok(Opcode::File),
            0xC1 => Ok(Opcode::Array),
            0xC2 => Ok(Opcode::ArrayWrite),
            0xC3 => Ok(Opcode::ArrayRead),
            0xC4 => Ok(Opcode::ArrayAppend),
            0xC5 => Ok(Opcode::MemoryUsage),
            0xC6 => Ok(Opcode::Filename),
            0xC8 => Ok(Opcode::Read8),
            0xC9 => Ok(Opcode::Read16),
            0xCA => Ok(Opcode::Read32),
            0xCB => Ok(Opcode::Readf),
            0xCC => Ok(Opcode::Write8),
            0xCD => Ok(Opcode::Write16),
            0xCE => Ok(Opcode::Write32),
            0xCF => Ok(Opcode::Writef),
            0xD0 => Ok(Opcode::ComReady),
            0xD1 => Ok(Opcode::ComReaddata),
            0xD2 => Ok(Opcode::ComWritedata),
            0xD3 => Ok(Opcode::ComGet),
            0xD4 => Ok(Opcode::ComSet),
            0xD5 => Ok(Opcode::ComTest),
            0xD6 => Ok(Opcode::ComRemove),
            0xD7 => Ok(Opcode::ComWritefile),
            0xD8 => Ok(Opcode::MailboxOpen),
            0xD9 => Ok(Opcode::MailboxWrite),
            0xDA => Ok(Opcode::MailboxRead),
            0xDB => Ok(Opcode::MailboxTest),
            0xDC => Ok(Opcode::MailboxReady),
            0xDD => Ok(Opcode::MailboxClose),
            0xFF => Ok(Opcode::Tst),
            _ => Err("Invalid enum value for Opcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Opcode::Error => "Error",
            Opcode::Nop => "Nop",
            Opcode::ProgramStop => "ProgramStop",
            Opcode::ProgramStart => "ProgramStart",
            Opcode::ObjectStop => "ObjectStop",
            Opcode::ObjectStart => "ObjectStart",
            Opcode::ObjectTrig => "ObjectTrig",
            Opcode::ObjectWait => "ObjectWait",
            Opcode::Return => "Return",
            Opcode::Call => "Call",
            Opcode::ObjectEnd => "ObjectEnd",
            Opcode::Sleep => "Sleep",
            Opcode::ProgramInfo => "ProgramInfo",
            Opcode::Label => "Label",
            Opcode::Probe => "Probe",
            Opcode::Do => "Do",
            Opcode::Add8 => "Add8",
            Opcode::Add16 => "Add16",
            Opcode::Add32 => "Add32",
            Opcode::AddF => "AddF",
            Opcode::Sub8 => "Sub8",
            Opcode::Sub16 => "Sub16",
            Opcode::Sub32 => "Sub32",
            Opcode::SubF => "SubF",
            Opcode::Mul8 => "Mul8",
            Opcode::Mul16 => "Mul16",
            Opcode::Mul32 => "Mul32",
            Opcode::MulF => "MulF",
            Opcode::Div8 => "Div8",
            Opcode::Div16 => "Div16",
            Opcode::Div32 => "Div32",
            Opcode::DivF => "DivF",
            Opcode::Or8 => "Or8",
            Opcode::Or16 => "Or16",
            Opcode::Or32 => "Or32",
            Opcode::And8 => "And8",
            Opcode::And16 => "And16",
            Opcode::And32 => "And32",
            Opcode::Xor8 => "Xor8",
            Opcode::Xor16 => "Xor16",
            Opcode::Xor32 => "Xor32",
            Opcode::Rl8 => "Rl8",
            Opcode::Rl16 => "Rl16",
            Opcode::Rl32 => "Rl32",
            Opcode::InitBytes => "InitBytes",
            Opcode::Move8_8 => "Move8_8",
            Opcode::Move8_16 => "Move8_16",
            Opcode::Move8_32 => "Move8_32",
            Opcode::Move8_F => "Move8_F",
            Opcode::Move16_8 => "Move16_8",
            Opcode::Move16_16 => "Move16_16",
            Opcode::Move16_32 => "Move16_32",
            Opcode::Move16_F => "Move16_F",
            Opcode::Move32_8 => "Move32_8",
            Opcode::Move32_16 => "Move32_16",
            Opcode::Move32_32 => "Move32_32",
            Opcode::Move32_F => "Move32_F",
            Opcode::MoveF_8 => "MoveF_8",
            Opcode::MoveF_16 => "MoveF_16",
            Opcode::MoveF_32 => "MoveF_32",
            Opcode::MoveF_F => "MoveF_F",
            Opcode::Jr => "Jr",
            Opcode::JrFalse => "JrFalse",
            Opcode::JrTrue => "JrTrue",
            Opcode::JrNan => "JrNan",
            Opcode::CpLt8 => "CpLt8",
            Opcode::CpLt16 => "CpLt16",
            Opcode::CpLt32 => "CpLt32",
            Opcode::CpLtf => "CpLtf",
            Opcode::CpGt8 => "CpGt8",
            Opcode::CpGt16 => "CpGt16",
            Opcode::CpGt32 => "CpGt32",
            Opcode::CpGtf => "CpGtf",
            Opcode::CpEq8 => "CpEq8",
            Opcode::CpEq16 => "CpEq16",
            Opcode::CpEq32 => "CpEq32",
            Opcode::CpEqf => "CpEqf",
            Opcode::CpNeq8 => "CpNeq8",
            Opcode::CpNeq16 => "CpNeq16",
            Opcode::CpNeq32 => "CpNeq32",
            Opcode::CpNeqf => "CpNeqf",
            Opcode::CpLteq8 => "CpLteq8",
            Opcode::CpLteq16 => "CpLteq16",
            Opcode::CpLteq32 => "CpLteq32",
            Opcode::CpLteqf => "CpLteqf",
            Opcode::CpGteq8 => "CpGteq8",
            Opcode::CpGteq16 => "CpGteq16",
            Opcode::CpGteq32 => "CpGteq32",
            Opcode::CpGteqf => "CpGteqf",
            Opcode::Select8 => "Select8",
            Opcode::Select16 => "Select16",
            Opcode::Select32 => "Select32",
            Opcode::Selectf => "Selectf",
            Opcode::System => "System",
            Opcode::PortCnvOutput => "PortCnvOutput",
            Opcode::PortCnvInput => "PortCnvInput",
            Opcode::NoteToFreq => "NoteToFreq",
            Opcode::JrLt8 => "JrLt8",
            Opcode::JrLt16 => "JrLt16",
            Opcode::JrLt32 => "JrLt32",
            Opcode::JrLtf => "JrLtf",
            Opcode::JrGt8 => "JrGt8",
            Opcode::JrGt16 => "JrGt16",
            Opcode::JrGt32 => "JrGt32",
            Opcode::JrGtf => "JrGtf",
            Opcode::JrEq8 => "JrEq8",
            Opcode::JrEq16 => "JrEq16",
            Opcode::JrEq32 => "JrEq32",
            Opcode::JrEqf => "JrEqf",
            Opcode::JrNeq8 => "JrNeq8",
            Opcode::JrNeq16 => "JrNeq16",
            Opcode::JrNeq32 => "JrNeq32",
            Opcode::JrNeqf => "JrNeqf",
            Opcode::JrLteq8 => "JrLteq8",
            Opcode::JrLteq16 => "JrLteq16",
            Opcode::JrLteq32 => "JrLteq32",
            Opcode::JrLteqf => "JrLteqf",
            Opcode::JrGteq8 => "JrGteq8",
            Opcode::JrGteq16 => "JrGteq16",
            Opcode::JrGteq32 => "JrGteq32",
            Opcode::JrGteqf => "JrGteqf",
            Opcode::Info => "Info",
            Opcode::String => "String",
            Opcode::MemoryWrite => "MemoryWrite",
            Opcode::MemoryRead => "MemoryRead",
            Opcode::UiFlush => "UiFlush",
            Opcode::UiRead => "UiRead",
            Opcode::UiWrite => "UiWrite",
            Opcode::UiButton => "UiButton",
            Opcode::UiDraw => "UiDraw",
            Opcode::TimerWait => "TimerWait",
            Opcode::TimerReady => "TimerReady",
            Opcode::TimerRead => "TimerRead",
            Opcode::Bp0 => "Bp0",
            Opcode::Bp1 => "Bp1",
            Opcode::Bp2 => "Bp2",
            Opcode::Bp3 => "Bp3",
            Opcode::BpSet => "BpSet",
            Opcode::Math => "Math",
            Opcode::Random => "Random",
            Opcode::TimerReadUs => "TimerReadUs",
            Opcode::KeepAlive => "KeepAlive",
            Opcode::ComRead => "ComRead",
            Opcode::ComWrite => "ComWrite",
            Opcode::Sound => "Sound",
            Opcode::SoundTest => "SoundTest",
            Opcode::SoundReady => "SoundReady",
            Opcode::InputSample => "InputSample",
            Opcode::InputDeviceList => "InputDeviceList",
            Opcode::InputDevice => "InputDevice",
            Opcode::InputRead => "InputRead",
            Opcode::InputTest => "InputTest",
            Opcode::InputReady => "InputReady",
            Opcode::InputReadSi => "InputReadSi",
            Opcode::InputReadExt => "InputReadExt",
            Opcode::InputWrite => "InputWrite",
            Opcode::OutputGetType => "OutputGetType",
            Opcode::OutputSetType => "OutputSetType",
            Opcode::OutputReset => "OutputReset",
            Opcode::OutputStop => "OutputStop",
            Opcode::OutputPower => "OutputPower",
            Opcode::OutputSpeed => "OutputSpeed",
            Opcode::OutputStart => "OutputStart",
            Opcode::OutputPolarity => "OutputPolarity",
            Opcode::OutputRead => "OutputRead",
            Opcode::OutputTest => "OutputTest",
            Opcode::OutputReady => "OutputReady",
            Opcode::OutputPosition => "OutputPosition",
            Opcode::OutputStepPower => "OutputStepPower",
            Opcode::OutputTimePower => "OutputTimePower",
            Opcode::OutputStepSpeed => "OutputStepSpeed",
            Opcode::OutputTimeSpeed => "OutputTimeSpeed",
            Opcode::OutputStepSync => "OutputStepSync",
            Opcode::OutputTimeSync => "OutputTimeSync",
            Opcode::OutputClrCount => "OutputClrCount",
            Opcode::OutputGetCount => "OutputGetCount",
            Opcode::OutputPrgStop => "OutputPrgStop",
            Opcode::File => "File",
            Opcode::Array => "Array",
            Opcode::ArrayWrite => "ArrayWrite",
            Opcode::ArrayRead => "ArrayRead",
            Opcode::ArrayAppend => "ArrayAppend",
            Opcode::MemoryUsage => "MemoryUsage",
            Opcode::Filename => "Filename",
            Opcode::Read8 => "Read8",
            Opcode::Read16 => "Read16",
            Opcode::Read32 => "Read32",
            Opcode::Readf => "Readf",
            Opcode::Write8 => "Write8",
            Opcode::Write16 => "Write16",
            Opcode::Write32 => "Write32",
            Opcode::Writef => "Writef",
            Opcode::ComReady => "ComReady",
            Opcode::ComReaddata => "ComReaddata",
            Opcode::ComWritedata => "ComWritedata",
            Opcode::ComGet => "ComGet",
            Opcode::ComSet => "ComSet",
            Opcode::ComTest => "ComTest",
            Opcode::ComRemove => "ComRemove",
            Opcode::ComWritefile => "ComWritefile",
            Opcode::MailboxOpen => "MailboxOpen",
            Opcode::MailboxWrite => "MailboxWrite",
            Opcode::MailboxRead => "MailboxRead",
            Opcode::MailboxTest => "MailboxTest",
            Opcode::MailboxReady => "MailboxReady",
            Opcode::MailboxClose => "MailboxClose",
            Opcode::Tst => "Tst",
        }
    }
}
#[allow(dead_code)]
pub enum UiReadSubcode {
    #[allow(non_camel_case_types)]
    GET_VBATT = 1,
    #[allow(non_camel_case_types)]
    GET_IBATT = 2,
    #[allow(non_camel_case_types)]
    GET_OS_VERS = 3,
    #[allow(non_camel_case_types)]
    GET_EVENT = 4,
    #[allow(non_camel_case_types)]
    GET_TBATT = 5,
    #[allow(non_camel_case_types)]
    GET_IINT = 6,
    #[allow(non_camel_case_types)]
    GET_IMOTOR = 7,
    #[allow(non_camel_case_types)]
    GET_STRING = 8,
    #[allow(non_camel_case_types)]
    GET_HW_VERS = 9,
    #[allow(non_camel_case_types)]
    GET_FW_VERS = 10,
    #[allow(non_camel_case_types)]
    GET_FW_BUILD = 11,
    #[allow(non_camel_case_types)]
    GET_OS_BUILD = 12,
    #[allow(non_camel_case_types)]
    GET_ADDRESS = 13,
    #[allow(non_camel_case_types)]
    GET_CODE = 14,
    #[allow(non_camel_case_types)]
    KEY = 15,
    #[allow(non_camel_case_types)]
    GET_SHUTDOWN = 16,
    #[allow(non_camel_case_types)]
    GET_WARNING = 17,
    #[allow(non_camel_case_types)]
    GET_LBATT = 18,
    #[allow(non_camel_case_types)]
    TEXTBOX_READ = 21,
    #[allow(non_camel_case_types)]
    GET_VERSION = 26,
    #[allow(non_camel_case_types)]
    GET_IP = 27,
    #[allow(non_camel_case_types)]
    GET_POWER = 29,
    #[allow(non_camel_case_types)]
    GET_SDCARD = 30,
    #[allow(non_camel_case_types)]
    GET_USBSTICK = 31,
}

impl UiReadSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<UiReadSubcode, &'static str> {
        match i {
            1 => Ok(UiReadSubcode::GET_VBATT),
            2 => Ok(UiReadSubcode::GET_IBATT),
            3 => Ok(UiReadSubcode::GET_OS_VERS),
            4 => Ok(UiReadSubcode::GET_EVENT),
            5 => Ok(UiReadSubcode::GET_TBATT),
            6 => Ok(UiReadSubcode::GET_IINT),
            7 => Ok(UiReadSubcode::GET_IMOTOR),
            8 => Ok(UiReadSubcode::GET_STRING),
            9 => Ok(UiReadSubcode::GET_HW_VERS),
            10 => Ok(UiReadSubcode::GET_FW_VERS),
            11 => Ok(UiReadSubcode::GET_FW_BUILD),
            12 => Ok(UiReadSubcode::GET_OS_BUILD),
            13 => Ok(UiReadSubcode::GET_ADDRESS),
            14 => Ok(UiReadSubcode::GET_CODE),
            15 => Ok(UiReadSubcode::KEY),
            16 => Ok(UiReadSubcode::GET_SHUTDOWN),
            17 => Ok(UiReadSubcode::GET_WARNING),
            18 => Ok(UiReadSubcode::GET_LBATT),
            21 => Ok(UiReadSubcode::TEXTBOX_READ),
            26 => Ok(UiReadSubcode::GET_VERSION),
            27 => Ok(UiReadSubcode::GET_IP),
            29 => Ok(UiReadSubcode::GET_POWER),
            30 => Ok(UiReadSubcode::GET_SDCARD),
            31 => Ok(UiReadSubcode::GET_USBSTICK),
            _ => Err("Invalid enum value for UiReadSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            UiReadSubcode::GET_VBATT => "GET_VBATT",
            UiReadSubcode::GET_IBATT => "GET_IBATT",
            UiReadSubcode::GET_OS_VERS => "GET_OS_VERS",
            UiReadSubcode::GET_EVENT => "GET_EVENT",
            UiReadSubcode::GET_TBATT => "GET_TBATT",
            UiReadSubcode::GET_IINT => "GET_IINT",
            UiReadSubcode::GET_IMOTOR => "GET_IMOTOR",
            UiReadSubcode::GET_STRING => "GET_STRING",
            UiReadSubcode::GET_HW_VERS => "GET_HW_VERS",
            UiReadSubcode::GET_FW_VERS => "GET_FW_VERS",
            UiReadSubcode::GET_FW_BUILD => "GET_FW_BUILD",
            UiReadSubcode::GET_OS_BUILD => "GET_OS_BUILD",
            UiReadSubcode::GET_ADDRESS => "GET_ADDRESS",
            UiReadSubcode::GET_CODE => "GET_CODE",
            UiReadSubcode::KEY => "KEY",
            UiReadSubcode::GET_SHUTDOWN => "GET_SHUTDOWN",
            UiReadSubcode::GET_WARNING => "GET_WARNING",
            UiReadSubcode::GET_LBATT => "GET_LBATT",
            UiReadSubcode::TEXTBOX_READ => "TEXTBOX_READ",
            UiReadSubcode::GET_VERSION => "GET_VERSION",
            UiReadSubcode::GET_IP => "GET_IP",
            UiReadSubcode::GET_POWER => "GET_POWER",
            UiReadSubcode::GET_SDCARD => "GET_SDCARD",
            UiReadSubcode::GET_USBSTICK => "GET_USBSTICK",
        }
    }
}
#[allow(dead_code)]
pub enum UiWriteSubcode {
    #[allow(non_camel_case_types)]
    WRITE_FLUSH = 1,
    #[allow(non_camel_case_types)]
    FLOATVALUE = 2,
    #[allow(non_camel_case_types)]
    STAMP = 3,
    #[allow(non_camel_case_types)]
    PUT_STRING = 8,
    #[allow(non_camel_case_types)]
    VALUE8 = 9,
    #[allow(non_camel_case_types)]
    VALUE16 = 10,
    #[allow(non_camel_case_types)]
    VALUE32 = 11,
    #[allow(non_camel_case_types)]
    VALUEF = 12,
    #[allow(non_camel_case_types)]
    ADDRESS = 13,
    #[allow(non_camel_case_types)]
    CODE = 14,
    #[allow(non_camel_case_types)]
    DOWNLOAD_END = 15,
    #[allow(non_camel_case_types)]
    SCREEN_BLOCK = 16,
    #[allow(non_camel_case_types)]
    ALLOW_PULSE = 17,
    #[allow(non_camel_case_types)]
    SET_PULSE = 18,
    #[allow(non_camel_case_types)]
    TEXTBOX_APPEND = 21,
    #[allow(non_camel_case_types)]
    SET_BUSY = 22,
    #[allow(non_camel_case_types)]
    SET_TESTPIN = 24,
    #[allow(non_camel_case_types)]
    INIT_RUN = 25,
    #[allow(non_camel_case_types)]
    UPDATE_RUN = 26,
    #[allow(non_camel_case_types)]
    LED = 27,
    #[allow(non_camel_case_types)]
    POWER = 29,
    #[allow(non_camel_case_types)]
    GRAPH_SAMPLE = 30,
    #[allow(non_camel_case_types)]
    TERMINAL = 31,
}

impl UiWriteSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<UiWriteSubcode, &'static str> {
        match i {
            1 => Ok(UiWriteSubcode::WRITE_FLUSH),
            2 => Ok(UiWriteSubcode::FLOATVALUE),
            3 => Ok(UiWriteSubcode::STAMP),
            8 => Ok(UiWriteSubcode::PUT_STRING),
            9 => Ok(UiWriteSubcode::VALUE8),
            10 => Ok(UiWriteSubcode::VALUE16),
            11 => Ok(UiWriteSubcode::VALUE32),
            12 => Ok(UiWriteSubcode::VALUEF),
            13 => Ok(UiWriteSubcode::ADDRESS),
            14 => Ok(UiWriteSubcode::CODE),
            15 => Ok(UiWriteSubcode::DOWNLOAD_END),
            16 => Ok(UiWriteSubcode::SCREEN_BLOCK),
            17 => Ok(UiWriteSubcode::ALLOW_PULSE),
            18 => Ok(UiWriteSubcode::SET_PULSE),
            21 => Ok(UiWriteSubcode::TEXTBOX_APPEND),
            22 => Ok(UiWriteSubcode::SET_BUSY),
            24 => Ok(UiWriteSubcode::SET_TESTPIN),
            25 => Ok(UiWriteSubcode::INIT_RUN),
            26 => Ok(UiWriteSubcode::UPDATE_RUN),
            27 => Ok(UiWriteSubcode::LED),
            29 => Ok(UiWriteSubcode::POWER),
            30 => Ok(UiWriteSubcode::GRAPH_SAMPLE),
            31 => Ok(UiWriteSubcode::TERMINAL),
            _ => Err("Invalid enum value for UiWriteSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            UiWriteSubcode::WRITE_FLUSH => "WRITE_FLUSH",
            UiWriteSubcode::FLOATVALUE => "FLOATVALUE",
            UiWriteSubcode::STAMP => "STAMP",
            UiWriteSubcode::PUT_STRING => "PUT_STRING",
            UiWriteSubcode::VALUE8 => "VALUE8",
            UiWriteSubcode::VALUE16 => "VALUE16",
            UiWriteSubcode::VALUE32 => "VALUE32",
            UiWriteSubcode::VALUEF => "VALUEF",
            UiWriteSubcode::ADDRESS => "ADDRESS",
            UiWriteSubcode::CODE => "CODE",
            UiWriteSubcode::DOWNLOAD_END => "DOWNLOAD_END",
            UiWriteSubcode::SCREEN_BLOCK => "SCREEN_BLOCK",
            UiWriteSubcode::ALLOW_PULSE => "ALLOW_PULSE",
            UiWriteSubcode::SET_PULSE => "SET_PULSE",
            UiWriteSubcode::TEXTBOX_APPEND => "TEXTBOX_APPEND",
            UiWriteSubcode::SET_BUSY => "SET_BUSY",
            UiWriteSubcode::SET_TESTPIN => "SET_TESTPIN",
            UiWriteSubcode::INIT_RUN => "INIT_RUN",
            UiWriteSubcode::UPDATE_RUN => "UPDATE_RUN",
            UiWriteSubcode::LED => "LED",
            UiWriteSubcode::POWER => "POWER",
            UiWriteSubcode::GRAPH_SAMPLE => "GRAPH_SAMPLE",
            UiWriteSubcode::TERMINAL => "TERMINAL",
        }
    }
}
#[allow(dead_code)]
pub enum UiButtonSubcode {
    #[allow(non_camel_case_types)]
    SHORTPRESS = 1,
    #[allow(non_camel_case_types)]
    LONGPRESS = 2,
    #[allow(non_camel_case_types)]
    WAIT_FOR_PRESS = 3,
    #[allow(non_camel_case_types)]
    FLUSH = 4,
    #[allow(non_camel_case_types)]
    PRESS = 5,
    #[allow(non_camel_case_types)]
    RELEASE = 6,
    #[allow(non_camel_case_types)]
    GET_HORZ = 7,
    #[allow(non_camel_case_types)]
    GET_VERT = 8,
    #[allow(non_camel_case_types)]
    PRESSED = 9,
    #[allow(non_camel_case_types)]
    SET_BACK_BLOCK = 10,
    #[allow(non_camel_case_types)]
    GET_BACK_BLOCK = 11,
    #[allow(non_camel_case_types)]
    TESTSHORTPRESS = 12,
    #[allow(non_camel_case_types)]
    TESTLONGPRESS = 13,
    #[allow(non_camel_case_types)]
    GET_BUMBED = 14,
    #[allow(non_camel_case_types)]
    GET_CLICK = 15,
}

impl UiButtonSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<UiButtonSubcode, &'static str> {
        match i {
            1 => Ok(UiButtonSubcode::SHORTPRESS),
            2 => Ok(UiButtonSubcode::LONGPRESS),
            3 => Ok(UiButtonSubcode::WAIT_FOR_PRESS),
            4 => Ok(UiButtonSubcode::FLUSH),
            5 => Ok(UiButtonSubcode::PRESS),
            6 => Ok(UiButtonSubcode::RELEASE),
            7 => Ok(UiButtonSubcode::GET_HORZ),
            8 => Ok(UiButtonSubcode::GET_VERT),
            9 => Ok(UiButtonSubcode::PRESSED),
            10 => Ok(UiButtonSubcode::SET_BACK_BLOCK),
            11 => Ok(UiButtonSubcode::GET_BACK_BLOCK),
            12 => Ok(UiButtonSubcode::TESTSHORTPRESS),
            13 => Ok(UiButtonSubcode::TESTLONGPRESS),
            14 => Ok(UiButtonSubcode::GET_BUMBED),
            15 => Ok(UiButtonSubcode::GET_CLICK),
            _ => Err("Invalid enum value for UiButtonSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            UiButtonSubcode::SHORTPRESS => "SHORTPRESS",
            UiButtonSubcode::LONGPRESS => "LONGPRESS",
            UiButtonSubcode::WAIT_FOR_PRESS => "WAIT_FOR_PRESS",
            UiButtonSubcode::FLUSH => "FLUSH",
            UiButtonSubcode::PRESS => "PRESS",
            UiButtonSubcode::RELEASE => "RELEASE",
            UiButtonSubcode::GET_HORZ => "GET_HORZ",
            UiButtonSubcode::GET_VERT => "GET_VERT",
            UiButtonSubcode::PRESSED => "PRESSED",
            UiButtonSubcode::SET_BACK_BLOCK => "SET_BACK_BLOCK",
            UiButtonSubcode::GET_BACK_BLOCK => "GET_BACK_BLOCK",
            UiButtonSubcode::TESTSHORTPRESS => "TESTSHORTPRESS",
            UiButtonSubcode::TESTLONGPRESS => "TESTLONGPRESS",
            UiButtonSubcode::GET_BUMBED => "GET_BUMBED",
            UiButtonSubcode::GET_CLICK => "GET_CLICK",
        }
    }
}
#[allow(dead_code)]
pub enum ComReadSubcode {
    #[allow(non_camel_case_types)]
    COMMAND = 14,
}

impl ComReadSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<ComReadSubcode, &'static str> {
        match i {
            14 => Ok(ComReadSubcode::COMMAND),
            _ => Err("Invalid enum value for ComReadSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            ComReadSubcode::COMMAND => "COMMAND",
        }
    }
}
#[allow(dead_code)]
pub enum ComWriteSubcode {
    #[allow(non_camel_case_types)]
    REPLY = 14,
}

impl ComWriteSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<ComWriteSubcode, &'static str> {
        match i {
            14 => Ok(ComWriteSubcode::REPLY),
            _ => Err("Invalid enum value for ComWriteSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            ComWriteSubcode::REPLY => "REPLY",
        }
    }
}
#[allow(dead_code)]
pub enum ComGetSubcode {
    #[allow(non_camel_case_types)]
    GET_ON_OFF = 1,
    #[allow(non_camel_case_types)]
    GET_VISIBLE = 2,
    #[allow(non_camel_case_types)]
    GET_RESULT = 4,
    #[allow(non_camel_case_types)]
    GET_PIN = 5,
    #[allow(non_camel_case_types)]
    SEARCH_ITEMS = 8,
    #[allow(non_camel_case_types)]
    SEARCH_ITEM = 9,
    #[allow(non_camel_case_types)]
    FAVOUR_ITEMS = 10,
    #[allow(non_camel_case_types)]
    FAVOUR_ITEM = 11,
    #[allow(non_camel_case_types)]
    GET_ID = 12,
    #[allow(non_camel_case_types)]
    GET_BRICKNAME = 13,
    #[allow(non_camel_case_types)]
    GET_NETWORK = 14,
    #[allow(non_camel_case_types)]
    GET_PRESENT = 15,
    #[allow(non_camel_case_types)]
    GET_ENCRYPT = 16,
    #[allow(non_camel_case_types)]
    CONNEC_ITEMS = 17,
    #[allow(non_camel_case_types)]
    CONNEC_ITEM = 18,
    #[allow(non_camel_case_types)]
    GET_INCOMING = 19,
    #[allow(non_camel_case_types)]
    GET_MODE2 = 20,
}

impl ComGetSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<ComGetSubcode, &'static str> {
        match i {
            1 => Ok(ComGetSubcode::GET_ON_OFF),
            2 => Ok(ComGetSubcode::GET_VISIBLE),
            4 => Ok(ComGetSubcode::GET_RESULT),
            5 => Ok(ComGetSubcode::GET_PIN),
            8 => Ok(ComGetSubcode::SEARCH_ITEMS),
            9 => Ok(ComGetSubcode::SEARCH_ITEM),
            10 => Ok(ComGetSubcode::FAVOUR_ITEMS),
            11 => Ok(ComGetSubcode::FAVOUR_ITEM),
            12 => Ok(ComGetSubcode::GET_ID),
            13 => Ok(ComGetSubcode::GET_BRICKNAME),
            14 => Ok(ComGetSubcode::GET_NETWORK),
            15 => Ok(ComGetSubcode::GET_PRESENT),
            16 => Ok(ComGetSubcode::GET_ENCRYPT),
            17 => Ok(ComGetSubcode::CONNEC_ITEMS),
            18 => Ok(ComGetSubcode::CONNEC_ITEM),
            19 => Ok(ComGetSubcode::GET_INCOMING),
            20 => Ok(ComGetSubcode::GET_MODE2),
            _ => Err("Invalid enum value for ComGetSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            ComGetSubcode::GET_ON_OFF => "GET_ON_OFF",
            ComGetSubcode::GET_VISIBLE => "GET_VISIBLE",
            ComGetSubcode::GET_RESULT => "GET_RESULT",
            ComGetSubcode::GET_PIN => "GET_PIN",
            ComGetSubcode::SEARCH_ITEMS => "SEARCH_ITEMS",
            ComGetSubcode::SEARCH_ITEM => "SEARCH_ITEM",
            ComGetSubcode::FAVOUR_ITEMS => "FAVOUR_ITEMS",
            ComGetSubcode::FAVOUR_ITEM => "FAVOUR_ITEM",
            ComGetSubcode::GET_ID => "GET_ID",
            ComGetSubcode::GET_BRICKNAME => "GET_BRICKNAME",
            ComGetSubcode::GET_NETWORK => "GET_NETWORK",
            ComGetSubcode::GET_PRESENT => "GET_PRESENT",
            ComGetSubcode::GET_ENCRYPT => "GET_ENCRYPT",
            ComGetSubcode::CONNEC_ITEMS => "CONNEC_ITEMS",
            ComGetSubcode::CONNEC_ITEM => "CONNEC_ITEM",
            ComGetSubcode::GET_INCOMING => "GET_INCOMING",
            ComGetSubcode::GET_MODE2 => "GET_MODE2",
        }
    }
}
#[allow(dead_code)]
pub enum ComSetSubcode {
    #[allow(non_camel_case_types)]
    SET_ON_OFF = 1,
    #[allow(non_camel_case_types)]
    SET_VISIBLE = 2,
    #[allow(non_camel_case_types)]
    SET_SEARCH = 3,
    #[allow(non_camel_case_types)]
    SET_PIN = 5,
    #[allow(non_camel_case_types)]
    SET_PASSKEY = 6,
    #[allow(non_camel_case_types)]
    SET_CONNECTION = 7,
    #[allow(non_camel_case_types)]
    SET_BRICKNAME = 8,
    #[allow(non_camel_case_types)]
    SET_MOVEUP = 9,
    #[allow(non_camel_case_types)]
    SET_MOVEDOWN = 10,
    #[allow(non_camel_case_types)]
    SET_ENCRYPT = 11,
    #[allow(non_camel_case_types)]
    SET_SSID = 12,
    #[allow(non_camel_case_types)]
    SET_MODE2 = 13,
}

impl ComSetSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<ComSetSubcode, &'static str> {
        match i {
            1 => Ok(ComSetSubcode::SET_ON_OFF),
            2 => Ok(ComSetSubcode::SET_VISIBLE),
            3 => Ok(ComSetSubcode::SET_SEARCH),
            5 => Ok(ComSetSubcode::SET_PIN),
            6 => Ok(ComSetSubcode::SET_PASSKEY),
            7 => Ok(ComSetSubcode::SET_CONNECTION),
            8 => Ok(ComSetSubcode::SET_BRICKNAME),
            9 => Ok(ComSetSubcode::SET_MOVEUP),
            10 => Ok(ComSetSubcode::SET_MOVEDOWN),
            11 => Ok(ComSetSubcode::SET_ENCRYPT),
            12 => Ok(ComSetSubcode::SET_SSID),
            13 => Ok(ComSetSubcode::SET_MODE2),
            _ => Err("Invalid enum value for ComSetSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            ComSetSubcode::SET_ON_OFF => "SET_ON_OFF",
            ComSetSubcode::SET_VISIBLE => "SET_VISIBLE",
            ComSetSubcode::SET_SEARCH => "SET_SEARCH",
            ComSetSubcode::SET_PIN => "SET_PIN",
            ComSetSubcode::SET_PASSKEY => "SET_PASSKEY",
            ComSetSubcode::SET_CONNECTION => "SET_CONNECTION",
            ComSetSubcode::SET_BRICKNAME => "SET_BRICKNAME",
            ComSetSubcode::SET_MOVEUP => "SET_MOVEUP",
            ComSetSubcode::SET_MOVEDOWN => "SET_MOVEDOWN",
            ComSetSubcode::SET_ENCRYPT => "SET_ENCRYPT",
            ComSetSubcode::SET_SSID => "SET_SSID",
            ComSetSubcode::SET_MODE2 => "SET_MODE2",
        }
    }
}
#[allow(dead_code)]
pub enum InputDeviceSubcode {
    #[allow(non_camel_case_types)]
    INSERT_TYPE = 1,
    #[allow(non_camel_case_types)]
    GET_FORMAT = 2,
    #[allow(non_camel_case_types)]
    CAL_MINMAX = 3,
    #[allow(non_camel_case_types)]
    CAL_DEFAULT = 4,
    #[allow(non_camel_case_types)]
    GET_TYPEMODE = 5,
    #[allow(non_camel_case_types)]
    GET_SYMBOL = 6,
    #[allow(non_camel_case_types)]
    CAL_MIN = 7,
    #[allow(non_camel_case_types)]
    CAL_MAX = 8,
    #[allow(non_camel_case_types)]
    SETUP = 9,
    #[allow(non_camel_case_types)]
    CLR_ALL = 10,
    #[allow(non_camel_case_types)]
    GET_RAW = 11,
    #[allow(non_camel_case_types)]
    GET_CONNECTION = 12,
    #[allow(non_camel_case_types)]
    STOP_ALL = 13,
    #[allow(non_camel_case_types)]
    SET_TYPEMODE = 14,
    #[allow(non_camel_case_types)]
    READY_IIC = 15,
    #[allow(non_camel_case_types)]
    GET_NAME = 21,
    #[allow(non_camel_case_types)]
    GET_MODENAME = 22,
    #[allow(non_camel_case_types)]
    SET_RAW = 23,
    #[allow(non_camel_case_types)]
    GET_FIGURES = 24,
    #[allow(non_camel_case_types)]
    GET_CHANGES = 25,
    #[allow(non_camel_case_types)]
    CLR_CHANGES = 26,
    #[allow(non_camel_case_types)]
    READY_PCT = 27,
    #[allow(non_camel_case_types)]
    READY_RAW = 28,
    #[allow(non_camel_case_types)]
    READY_SI = 29,
    #[allow(non_camel_case_types)]
    GET_MINMAX = 30,
    #[allow(non_camel_case_types)]
    GET_BUMPS = 31,
}

impl InputDeviceSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<InputDeviceSubcode, &'static str> {
        match i {
            1 => Ok(InputDeviceSubcode::INSERT_TYPE),
            2 => Ok(InputDeviceSubcode::GET_FORMAT),
            3 => Ok(InputDeviceSubcode::CAL_MINMAX),
            4 => Ok(InputDeviceSubcode::CAL_DEFAULT),
            5 => Ok(InputDeviceSubcode::GET_TYPEMODE),
            6 => Ok(InputDeviceSubcode::GET_SYMBOL),
            7 => Ok(InputDeviceSubcode::CAL_MIN),
            8 => Ok(InputDeviceSubcode::CAL_MAX),
            9 => Ok(InputDeviceSubcode::SETUP),
            10 => Ok(InputDeviceSubcode::CLR_ALL),
            11 => Ok(InputDeviceSubcode::GET_RAW),
            12 => Ok(InputDeviceSubcode::GET_CONNECTION),
            13 => Ok(InputDeviceSubcode::STOP_ALL),
            14 => Ok(InputDeviceSubcode::SET_TYPEMODE),
            15 => Ok(InputDeviceSubcode::READY_IIC),
            21 => Ok(InputDeviceSubcode::GET_NAME),
            22 => Ok(InputDeviceSubcode::GET_MODENAME),
            23 => Ok(InputDeviceSubcode::SET_RAW),
            24 => Ok(InputDeviceSubcode::GET_FIGURES),
            25 => Ok(InputDeviceSubcode::GET_CHANGES),
            26 => Ok(InputDeviceSubcode::CLR_CHANGES),
            27 => Ok(InputDeviceSubcode::READY_PCT),
            28 => Ok(InputDeviceSubcode::READY_RAW),
            29 => Ok(InputDeviceSubcode::READY_SI),
            30 => Ok(InputDeviceSubcode::GET_MINMAX),
            31 => Ok(InputDeviceSubcode::GET_BUMPS),
            _ => Err("Invalid enum value for InputDeviceSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            InputDeviceSubcode::INSERT_TYPE => "INSERT_TYPE",
            InputDeviceSubcode::GET_FORMAT => "GET_FORMAT",
            InputDeviceSubcode::CAL_MINMAX => "CAL_MINMAX",
            InputDeviceSubcode::CAL_DEFAULT => "CAL_DEFAULT",
            InputDeviceSubcode::GET_TYPEMODE => "GET_TYPEMODE",
            InputDeviceSubcode::GET_SYMBOL => "GET_SYMBOL",
            InputDeviceSubcode::CAL_MIN => "CAL_MIN",
            InputDeviceSubcode::CAL_MAX => "CAL_MAX",
            InputDeviceSubcode::SETUP => "SETUP",
            InputDeviceSubcode::CLR_ALL => "CLR_ALL",
            InputDeviceSubcode::GET_RAW => "GET_RAW",
            InputDeviceSubcode::GET_CONNECTION => "GET_CONNECTION",
            InputDeviceSubcode::STOP_ALL => "STOP_ALL",
            InputDeviceSubcode::SET_TYPEMODE => "SET_TYPEMODE",
            InputDeviceSubcode::READY_IIC => "READY_IIC",
            InputDeviceSubcode::GET_NAME => "GET_NAME",
            InputDeviceSubcode::GET_MODENAME => "GET_MODENAME",
            InputDeviceSubcode::SET_RAW => "SET_RAW",
            InputDeviceSubcode::GET_FIGURES => "GET_FIGURES",
            InputDeviceSubcode::GET_CHANGES => "GET_CHANGES",
            InputDeviceSubcode::CLR_CHANGES => "CLR_CHANGES",
            InputDeviceSubcode::READY_PCT => "READY_PCT",
            InputDeviceSubcode::READY_RAW => "READY_RAW",
            InputDeviceSubcode::READY_SI => "READY_SI",
            InputDeviceSubcode::GET_MINMAX => "GET_MINMAX",
            InputDeviceSubcode::GET_BUMPS => "GET_BUMPS",
        }
    }
}
#[allow(dead_code)]
pub enum ProgramInfoSubcode {
    #[allow(non_camel_case_types)]
    OBJ_STOP = 0,
    #[allow(non_camel_case_types)]
    OBJ_START = 4,
    #[allow(non_camel_case_types)]
    GET_STATUS = 22,
    #[allow(non_camel_case_types)]
    GET_SPEED = 23,
    #[allow(non_camel_case_types)]
    GET_PRGRESULT = 24,
    #[allow(non_camel_case_types)]
    SET_INSTR = 25,
}

impl ProgramInfoSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<ProgramInfoSubcode, &'static str> {
        match i {
            0 => Ok(ProgramInfoSubcode::OBJ_STOP),
            4 => Ok(ProgramInfoSubcode::OBJ_START),
            22 => Ok(ProgramInfoSubcode::GET_STATUS),
            23 => Ok(ProgramInfoSubcode::GET_SPEED),
            24 => Ok(ProgramInfoSubcode::GET_PRGRESULT),
            25 => Ok(ProgramInfoSubcode::SET_INSTR),
            _ => Err("Invalid enum value for ProgramInfoSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            ProgramInfoSubcode::OBJ_STOP => "OBJ_STOP",
            ProgramInfoSubcode::OBJ_START => "OBJ_START",
            ProgramInfoSubcode::GET_STATUS => "GET_STATUS",
            ProgramInfoSubcode::GET_SPEED => "GET_SPEED",
            ProgramInfoSubcode::GET_PRGRESULT => "GET_PRGRESULT",
            ProgramInfoSubcode::SET_INSTR => "SET_INSTR",
        }
    }
}
#[allow(dead_code)]
pub enum UiDrawSubcode {
    #[allow(non_camel_case_types)]
    UPDATE = 0,
    #[allow(non_camel_case_types)]
    CLEAN = 1,
    #[allow(non_camel_case_types)]
    PIXEL = 2,
    #[allow(non_camel_case_types)]
    LINE = 3,
    #[allow(non_camel_case_types)]
    CIRCLE = 4,
    #[allow(non_camel_case_types)]
    TEXT = 5,
    #[allow(non_camel_case_types)]
    ICON = 6,
    #[allow(non_camel_case_types)]
    PICTURE = 7,
    #[allow(non_camel_case_types)]
    VALUE = 8,
    #[allow(non_camel_case_types)]
    FILLRECT = 9,
    #[allow(non_camel_case_types)]
    RECT = 10,
    #[allow(non_camel_case_types)]
    NOTIFICATION = 11,
    #[allow(non_camel_case_types)]
    QUESTION = 12,
    #[allow(non_camel_case_types)]
    KEYBOARD = 13,
    #[allow(non_camel_case_types)]
    BROWSE = 14,
    #[allow(non_camel_case_types)]
    VERTBAR = 15,
    #[allow(non_camel_case_types)]
    INVERSERECT = 16,
    #[allow(non_camel_case_types)]
    SELECT_FONT = 17,
    #[allow(non_camel_case_types)]
    TOPLINE = 18,
    #[allow(non_camel_case_types)]
    FILLWINDOW = 19,
    #[allow(non_camel_case_types)]
    SCROLL = 20,
    #[allow(non_camel_case_types)]
    DOTLINE = 21,
    #[allow(non_camel_case_types)]
    VIEW_VALUE = 22,
    #[allow(non_camel_case_types)]
    VIEW_UNIT = 23,
    #[allow(non_camel_case_types)]
    FILLCIRCLE = 24,
    #[allow(non_camel_case_types)]
    STORE = 25,
    #[allow(non_camel_case_types)]
    RESTORE = 26,
    #[allow(non_camel_case_types)]
    ICON_QUESTION = 27,
    #[allow(non_camel_case_types)]
    BMPFILE = 28,
    #[allow(non_camel_case_types)]
    POPUP = 29,
    #[allow(non_camel_case_types)]
    GRAPH_SETUP = 30,
    #[allow(non_camel_case_types)]
    GRAPH_DRAW = 31,
    #[allow(non_camel_case_types)]
    TEXTBOX = 32,
}

impl UiDrawSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<UiDrawSubcode, &'static str> {
        match i {
            0 => Ok(UiDrawSubcode::UPDATE),
            1 => Ok(UiDrawSubcode::CLEAN),
            2 => Ok(UiDrawSubcode::PIXEL),
            3 => Ok(UiDrawSubcode::LINE),
            4 => Ok(UiDrawSubcode::CIRCLE),
            5 => Ok(UiDrawSubcode::TEXT),
            6 => Ok(UiDrawSubcode::ICON),
            7 => Ok(UiDrawSubcode::PICTURE),
            8 => Ok(UiDrawSubcode::VALUE),
            9 => Ok(UiDrawSubcode::FILLRECT),
            10 => Ok(UiDrawSubcode::RECT),
            11 => Ok(UiDrawSubcode::NOTIFICATION),
            12 => Ok(UiDrawSubcode::QUESTION),
            13 => Ok(UiDrawSubcode::KEYBOARD),
            14 => Ok(UiDrawSubcode::BROWSE),
            15 => Ok(UiDrawSubcode::VERTBAR),
            16 => Ok(UiDrawSubcode::INVERSERECT),
            17 => Ok(UiDrawSubcode::SELECT_FONT),
            18 => Ok(UiDrawSubcode::TOPLINE),
            19 => Ok(UiDrawSubcode::FILLWINDOW),
            20 => Ok(UiDrawSubcode::SCROLL),
            21 => Ok(UiDrawSubcode::DOTLINE),
            22 => Ok(UiDrawSubcode::VIEW_VALUE),
            23 => Ok(UiDrawSubcode::VIEW_UNIT),
            24 => Ok(UiDrawSubcode::FILLCIRCLE),
            25 => Ok(UiDrawSubcode::STORE),
            26 => Ok(UiDrawSubcode::RESTORE),
            27 => Ok(UiDrawSubcode::ICON_QUESTION),
            28 => Ok(UiDrawSubcode::BMPFILE),
            29 => Ok(UiDrawSubcode::POPUP),
            30 => Ok(UiDrawSubcode::GRAPH_SETUP),
            31 => Ok(UiDrawSubcode::GRAPH_DRAW),
            32 => Ok(UiDrawSubcode::TEXTBOX),
            _ => Err("Invalid enum value for UiDrawSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            UiDrawSubcode::UPDATE => "UPDATE",
            UiDrawSubcode::CLEAN => "CLEAN",
            UiDrawSubcode::PIXEL => "PIXEL",
            UiDrawSubcode::LINE => "LINE",
            UiDrawSubcode::CIRCLE => "CIRCLE",
            UiDrawSubcode::TEXT => "TEXT",
            UiDrawSubcode::ICON => "ICON",
            UiDrawSubcode::PICTURE => "PICTURE",
            UiDrawSubcode::VALUE => "VALUE",
            UiDrawSubcode::FILLRECT => "FILLRECT",
            UiDrawSubcode::RECT => "RECT",
            UiDrawSubcode::NOTIFICATION => "NOTIFICATION",
            UiDrawSubcode::QUESTION => "QUESTION",
            UiDrawSubcode::KEYBOARD => "KEYBOARD",
            UiDrawSubcode::BROWSE => "BROWSE",
            UiDrawSubcode::VERTBAR => "VERTBAR",
            UiDrawSubcode::INVERSERECT => "INVERSERECT",
            UiDrawSubcode::SELECT_FONT => "SELECT_FONT",
            UiDrawSubcode::TOPLINE => "TOPLINE",
            UiDrawSubcode::FILLWINDOW => "FILLWINDOW",
            UiDrawSubcode::SCROLL => "SCROLL",
            UiDrawSubcode::DOTLINE => "DOTLINE",
            UiDrawSubcode::VIEW_VALUE => "VIEW_VALUE",
            UiDrawSubcode::VIEW_UNIT => "VIEW_UNIT",
            UiDrawSubcode::FILLCIRCLE => "FILLCIRCLE",
            UiDrawSubcode::STORE => "STORE",
            UiDrawSubcode::RESTORE => "RESTORE",
            UiDrawSubcode::ICON_QUESTION => "ICON_QUESTION",
            UiDrawSubcode::BMPFILE => "BMPFILE",
            UiDrawSubcode::POPUP => "POPUP",
            UiDrawSubcode::GRAPH_SETUP => "GRAPH_SETUP",
            UiDrawSubcode::GRAPH_DRAW => "GRAPH_DRAW",
            UiDrawSubcode::TEXTBOX => "TEXTBOX",
        }
    }
}
#[allow(dead_code)]
pub enum FileSubcode {
    #[allow(non_camel_case_types)]
    OPEN_APPEND = 0,
    #[allow(non_camel_case_types)]
    OPEN_READ = 1,
    #[allow(non_camel_case_types)]
    OPEN_WRITE = 2,
    #[allow(non_camel_case_types)]
    READ_VALUE = 3,
    #[allow(non_camel_case_types)]
    WRITE_VALUE = 4,
    #[allow(non_camel_case_types)]
    READ_TEXT = 5,
    #[allow(non_camel_case_types)]
    WRITE_TEXT = 6,
    #[allow(non_camel_case_types)]
    CLOSE = 7,
    #[allow(non_camel_case_types)]
    LOAD_IMAGE = 8,
    #[allow(non_camel_case_types)]
    GET_HANDLE = 9,
    #[allow(non_camel_case_types)]
    MAKE_FOLDER = 10,
    #[allow(non_camel_case_types)]
    GET_POOL = 11,
    #[allow(non_camel_case_types)]
    SET_LOG_SYNC_TIME = 12,
    #[allow(non_camel_case_types)]
    GET_FOLDERS = 13,
    #[allow(non_camel_case_types)]
    GET_LOG_SYNC_TIME = 14,
    #[allow(non_camel_case_types)]
    GET_SUBFOLDER_NAME = 15,
    #[allow(non_camel_case_types)]
    WRITE_LOG = 16,
    #[allow(non_camel_case_types)]
    CLOSE_LOG = 17,
    #[allow(non_camel_case_types)]
    GET_IMAGE = 18,
    #[allow(non_camel_case_types)]
    GET_ITEM = 19,
    #[allow(non_camel_case_types)]
    GET_CACHE_FILES = 20,
    #[allow(non_camel_case_types)]
    PUT_CACHE_FILE = 21,
    #[allow(non_camel_case_types)]
    GET_CACHE_FILE = 22,
    #[allow(non_camel_case_types)]
    DEL_CACHE_FILE = 23,
    #[allow(non_camel_case_types)]
    DEL_SUBFOLDER = 24,
    #[allow(non_camel_case_types)]
    GET_LOG_NAME = 25,
    #[allow(non_camel_case_types)]
    OPEN_LOG = 27,
    #[allow(non_camel_case_types)]
    READ_BYTES = 28,
    #[allow(non_camel_case_types)]
    WRITE_BYTES = 29,
    #[allow(non_camel_case_types)]
    REMOVE = 30,
    #[allow(non_camel_case_types)]
    MOVE = 31,
}

impl FileSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<FileSubcode, &'static str> {
        match i {
            0 => Ok(FileSubcode::OPEN_APPEND),
            1 => Ok(FileSubcode::OPEN_READ),
            2 => Ok(FileSubcode::OPEN_WRITE),
            3 => Ok(FileSubcode::READ_VALUE),
            4 => Ok(FileSubcode::WRITE_VALUE),
            5 => Ok(FileSubcode::READ_TEXT),
            6 => Ok(FileSubcode::WRITE_TEXT),
            7 => Ok(FileSubcode::CLOSE),
            8 => Ok(FileSubcode::LOAD_IMAGE),
            9 => Ok(FileSubcode::GET_HANDLE),
            10 => Ok(FileSubcode::MAKE_FOLDER),
            11 => Ok(FileSubcode::GET_POOL),
            12 => Ok(FileSubcode::SET_LOG_SYNC_TIME),
            13 => Ok(FileSubcode::GET_FOLDERS),
            14 => Ok(FileSubcode::GET_LOG_SYNC_TIME),
            15 => Ok(FileSubcode::GET_SUBFOLDER_NAME),
            16 => Ok(FileSubcode::WRITE_LOG),
            17 => Ok(FileSubcode::CLOSE_LOG),
            18 => Ok(FileSubcode::GET_IMAGE),
            19 => Ok(FileSubcode::GET_ITEM),
            20 => Ok(FileSubcode::GET_CACHE_FILES),
            21 => Ok(FileSubcode::PUT_CACHE_FILE),
            22 => Ok(FileSubcode::GET_CACHE_FILE),
            23 => Ok(FileSubcode::DEL_CACHE_FILE),
            24 => Ok(FileSubcode::DEL_SUBFOLDER),
            25 => Ok(FileSubcode::GET_LOG_NAME),
            27 => Ok(FileSubcode::OPEN_LOG),
            28 => Ok(FileSubcode::READ_BYTES),
            29 => Ok(FileSubcode::WRITE_BYTES),
            30 => Ok(FileSubcode::REMOVE),
            31 => Ok(FileSubcode::MOVE),
            _ => Err("Invalid enum value for FileSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            FileSubcode::OPEN_APPEND => "OPEN_APPEND",
            FileSubcode::OPEN_READ => "OPEN_READ",
            FileSubcode::OPEN_WRITE => "OPEN_WRITE",
            FileSubcode::READ_VALUE => "READ_VALUE",
            FileSubcode::WRITE_VALUE => "WRITE_VALUE",
            FileSubcode::READ_TEXT => "READ_TEXT",
            FileSubcode::WRITE_TEXT => "WRITE_TEXT",
            FileSubcode::CLOSE => "CLOSE",
            FileSubcode::LOAD_IMAGE => "LOAD_IMAGE",
            FileSubcode::GET_HANDLE => "GET_HANDLE",
            FileSubcode::MAKE_FOLDER => "MAKE_FOLDER",
            FileSubcode::GET_POOL => "GET_POOL",
            FileSubcode::SET_LOG_SYNC_TIME => "SET_LOG_SYNC_TIME",
            FileSubcode::GET_FOLDERS => "GET_FOLDERS",
            FileSubcode::GET_LOG_SYNC_TIME => "GET_LOG_SYNC_TIME",
            FileSubcode::GET_SUBFOLDER_NAME => "GET_SUBFOLDER_NAME",
            FileSubcode::WRITE_LOG => "WRITE_LOG",
            FileSubcode::CLOSE_LOG => "CLOSE_LOG",
            FileSubcode::GET_IMAGE => "GET_IMAGE",
            FileSubcode::GET_ITEM => "GET_ITEM",
            FileSubcode::GET_CACHE_FILES => "GET_CACHE_FILES",
            FileSubcode::PUT_CACHE_FILE => "PUT_CACHE_FILE",
            FileSubcode::GET_CACHE_FILE => "GET_CACHE_FILE",
            FileSubcode::DEL_CACHE_FILE => "DEL_CACHE_FILE",
            FileSubcode::DEL_SUBFOLDER => "DEL_SUBFOLDER",
            FileSubcode::GET_LOG_NAME => "GET_LOG_NAME",
            FileSubcode::OPEN_LOG => "OPEN_LOG",
            FileSubcode::READ_BYTES => "READ_BYTES",
            FileSubcode::WRITE_BYTES => "WRITE_BYTES",
            FileSubcode::REMOVE => "REMOVE",
            FileSubcode::MOVE => "MOVE",
        }
    }
}
#[allow(dead_code)]
pub enum ArraySubcode {
    #[allow(non_camel_case_types)]
    DELETE = 0,
    #[allow(non_camel_case_types)]
    CREATE8 = 1,
    #[allow(non_camel_case_types)]
    CREATE16 = 2,
    #[allow(non_camel_case_types)]
    CREATE32 = 3,
    #[allow(non_camel_case_types)]
    CREATEF = 4,
    #[allow(non_camel_case_types)]
    RESIZE = 5,
    #[allow(non_camel_case_types)]
    FILL = 6,
    #[allow(non_camel_case_types)]
    COPY = 7,
    #[allow(non_camel_case_types)]
    INIT8 = 8,
    #[allow(non_camel_case_types)]
    INIT16 = 9,
    #[allow(non_camel_case_types)]
    INIT32 = 10,
    #[allow(non_camel_case_types)]
    INITF = 11,
    #[allow(non_camel_case_types)]
    SIZE = 12,
    #[allow(non_camel_case_types)]
    READ_CONTENT = 13,
    #[allow(non_camel_case_types)]
    WRITE_CONTENT = 14,
    #[allow(non_camel_case_types)]
    READ_SIZE = 15,
}

impl ArraySubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<ArraySubcode, &'static str> {
        match i {
            0 => Ok(ArraySubcode::DELETE),
            1 => Ok(ArraySubcode::CREATE8),
            2 => Ok(ArraySubcode::CREATE16),
            3 => Ok(ArraySubcode::CREATE32),
            4 => Ok(ArraySubcode::CREATEF),
            5 => Ok(ArraySubcode::RESIZE),
            6 => Ok(ArraySubcode::FILL),
            7 => Ok(ArraySubcode::COPY),
            8 => Ok(ArraySubcode::INIT8),
            9 => Ok(ArraySubcode::INIT16),
            10 => Ok(ArraySubcode::INIT32),
            11 => Ok(ArraySubcode::INITF),
            12 => Ok(ArraySubcode::SIZE),
            13 => Ok(ArraySubcode::READ_CONTENT),
            14 => Ok(ArraySubcode::WRITE_CONTENT),
            15 => Ok(ArraySubcode::READ_SIZE),
            _ => Err("Invalid enum value for ArraySubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            ArraySubcode::DELETE => "DELETE",
            ArraySubcode::CREATE8 => "CREATE8",
            ArraySubcode::CREATE16 => "CREATE16",
            ArraySubcode::CREATE32 => "CREATE32",
            ArraySubcode::CREATEF => "CREATEF",
            ArraySubcode::RESIZE => "RESIZE",
            ArraySubcode::FILL => "FILL",
            ArraySubcode::COPY => "COPY",
            ArraySubcode::INIT8 => "INIT8",
            ArraySubcode::INIT16 => "INIT16",
            ArraySubcode::INIT32 => "INIT32",
            ArraySubcode::INITF => "INITF",
            ArraySubcode::SIZE => "SIZE",
            ArraySubcode::READ_CONTENT => "READ_CONTENT",
            ArraySubcode::WRITE_CONTENT => "WRITE_CONTENT",
            ArraySubcode::READ_SIZE => "READ_SIZE",
        }
    }
}
#[allow(dead_code)]
pub enum FilenameSubcode {
    #[allow(non_camel_case_types)]
    EXIST = 16,
    #[allow(non_camel_case_types)]
    TOTALSIZE = 17,
    #[allow(non_camel_case_types)]
    SPLIT = 18,
    #[allow(non_camel_case_types)]
    MERGE = 19,
    #[allow(non_camel_case_types)]
    CHECK = 20,
    #[allow(non_camel_case_types)]
    PACK = 21,
    #[allow(non_camel_case_types)]
    UNPACK = 22,
    #[allow(non_camel_case_types)]
    GET_FOLDERNAME = 23,
}

impl FilenameSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<FilenameSubcode, &'static str> {
        match i {
            16 => Ok(FilenameSubcode::EXIST),
            17 => Ok(FilenameSubcode::TOTALSIZE),
            18 => Ok(FilenameSubcode::SPLIT),
            19 => Ok(FilenameSubcode::MERGE),
            20 => Ok(FilenameSubcode::CHECK),
            21 => Ok(FilenameSubcode::PACK),
            22 => Ok(FilenameSubcode::UNPACK),
            23 => Ok(FilenameSubcode::GET_FOLDERNAME),
            _ => Err("Invalid enum value for FilenameSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            FilenameSubcode::EXIST => "EXIST",
            FilenameSubcode::TOTALSIZE => "TOTALSIZE",
            FilenameSubcode::SPLIT => "SPLIT",
            FilenameSubcode::MERGE => "MERGE",
            FilenameSubcode::CHECK => "CHECK",
            FilenameSubcode::PACK => "PACK",
            FilenameSubcode::UNPACK => "UNPACK",
            FilenameSubcode::GET_FOLDERNAME => "GET_FOLDERNAME",
        }
    }
}
#[allow(dead_code)]
pub enum InfoSubcode {
    #[allow(non_camel_case_types)]
    SET_ERROR = 1,
    #[allow(non_camel_case_types)]
    GET_ERROR = 2,
    #[allow(non_camel_case_types)]
    ERRORTEXT = 3,
    #[allow(non_camel_case_types)]
    GET_VOLUME = 4,
    #[allow(non_camel_case_types)]
    SET_VOLUME = 5,
    #[allow(non_camel_case_types)]
    GET_MINUTES = 6,
    #[allow(non_camel_case_types)]
    SET_MINUTES = 7,
}

impl InfoSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<InfoSubcode, &'static str> {
        match i {
            1 => Ok(InfoSubcode::SET_ERROR),
            2 => Ok(InfoSubcode::GET_ERROR),
            3 => Ok(InfoSubcode::ERRORTEXT),
            4 => Ok(InfoSubcode::GET_VOLUME),
            5 => Ok(InfoSubcode::SET_VOLUME),
            6 => Ok(InfoSubcode::GET_MINUTES),
            7 => Ok(InfoSubcode::SET_MINUTES),
            _ => Err("Invalid enum value for InfoSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            InfoSubcode::SET_ERROR => "SET_ERROR",
            InfoSubcode::GET_ERROR => "GET_ERROR",
            InfoSubcode::ERRORTEXT => "ERRORTEXT",
            InfoSubcode::GET_VOLUME => "GET_VOLUME",
            InfoSubcode::SET_VOLUME => "SET_VOLUME",
            InfoSubcode::GET_MINUTES => "GET_MINUTES",
            InfoSubcode::SET_MINUTES => "SET_MINUTES",
        }
    }
}
#[allow(dead_code)]
pub enum SoundSubcode {
    #[allow(non_camel_case_types)]
    BREAK = 0,
    #[allow(non_camel_case_types)]
    TONE = 1,
    #[allow(non_camel_case_types)]
    PLAY = 2,
    #[allow(non_camel_case_types)]
    REPEAT = 3,
    #[allow(non_camel_case_types)]
    SERVICE = 4,
}

impl SoundSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<SoundSubcode, &'static str> {
        match i {
            0 => Ok(SoundSubcode::BREAK),
            1 => Ok(SoundSubcode::TONE),
            2 => Ok(SoundSubcode::PLAY),
            3 => Ok(SoundSubcode::REPEAT),
            4 => Ok(SoundSubcode::SERVICE),
            _ => Err("Invalid enum value for SoundSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            SoundSubcode::BREAK => "BREAK",
            SoundSubcode::TONE => "TONE",
            SoundSubcode::PLAY => "PLAY",
            SoundSubcode::REPEAT => "REPEAT",
            SoundSubcode::SERVICE => "SERVICE",
        }
    }
}
#[allow(dead_code)]
pub enum StringSubcode {
    #[allow(non_camel_case_types)]
    GET_SIZE = 1,
    #[allow(non_camel_case_types)]
    ADD = 2,
    #[allow(non_camel_case_types)]
    COMPARE = 3,
    #[allow(non_camel_case_types)]
    DUPLICATE = 5,
    #[allow(non_camel_case_types)]
    VALUE_TO_STRING = 6,
    #[allow(non_camel_case_types)]
    STRING_TO_VALUE = 7,
    #[allow(non_camel_case_types)]
    STRIP = 8,
    #[allow(non_camel_case_types)]
    NUMBER_TO_STRING = 9,
    #[allow(non_camel_case_types)]
    SUB = 10,
    #[allow(non_camel_case_types)]
    VALUE_FORMATTED = 11,
    #[allow(non_camel_case_types)]
    NUMBER_FORMATTED = 12,
}

impl StringSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<StringSubcode, &'static str> {
        match i {
            1 => Ok(StringSubcode::GET_SIZE),
            2 => Ok(StringSubcode::ADD),
            3 => Ok(StringSubcode::COMPARE),
            5 => Ok(StringSubcode::DUPLICATE),
            6 => Ok(StringSubcode::VALUE_TO_STRING),
            7 => Ok(StringSubcode::STRING_TO_VALUE),
            8 => Ok(StringSubcode::STRIP),
            9 => Ok(StringSubcode::NUMBER_TO_STRING),
            10 => Ok(StringSubcode::SUB),
            11 => Ok(StringSubcode::VALUE_FORMATTED),
            12 => Ok(StringSubcode::NUMBER_FORMATTED),
            _ => Err("Invalid enum value for StringSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            StringSubcode::GET_SIZE => "GET_SIZE",
            StringSubcode::ADD => "ADD",
            StringSubcode::COMPARE => "COMPARE",
            StringSubcode::DUPLICATE => "DUPLICATE",
            StringSubcode::VALUE_TO_STRING => "VALUE_TO_STRING",
            StringSubcode::STRING_TO_VALUE => "STRING_TO_VALUE",
            StringSubcode::STRIP => "STRIP",
            StringSubcode::NUMBER_TO_STRING => "NUMBER_TO_STRING",
            StringSubcode::SUB => "SUB",
            StringSubcode::VALUE_FORMATTED => "VALUE_FORMATTED",
            StringSubcode::NUMBER_FORMATTED => "NUMBER_FORMATTED",
        }
    }
}
#[allow(dead_code)]
pub enum Type {
    #[allow(non_camel_case_types)]
    TYPE_KEEP = 0,
    #[allow(non_camel_case_types)]
    TYPE_NXT_TOUCH = 1,
    #[allow(non_camel_case_types)]
    TYPE_NXT_LIGHT = 2,
    #[allow(non_camel_case_types)]
    TYPE_NXT_SOUND = 3,
    #[allow(non_camel_case_types)]
    TYPE_NXT_COLOR = 4,
    #[allow(non_camel_case_types)]
    TYPE_NXT_ULTRASONIC = 5,
    #[allow(non_camel_case_types)]
    TYPE_NXT_TEMPERATURE = 6,
    #[allow(non_camel_case_types)]
    TYPE_TACHO = 7,
    #[allow(non_camel_case_types)]
    TYPE_MINITACHO = 8,
    #[allow(non_camel_case_types)]
    TYPE_NEWTACHO = 9,
    #[allow(non_camel_case_types)]
    TYPE_TOUCH = 16,
    #[allow(non_camel_case_types)]
    TYPE_COLOR = 29,
    #[allow(non_camel_case_types)]
    TYPE_ULTRASONIC = 30,
    #[allow(non_camel_case_types)]
    TYPE_GYRO = 32,
    #[allow(non_camel_case_types)]
    TYPE_IR = 33,
    #[allow(non_camel_case_types)]
    TYPE_THIRD_PARTY_START = 50,
    #[allow(non_camel_case_types)]
    TYPE_THIRD_PARTY_END = 98,
    #[allow(non_camel_case_types)]
    TYPE_ENERGYMETER = 99,
    #[allow(non_camel_case_types)]
    TYPE_IIC_UNKNOWN = 100,
    #[allow(non_camel_case_types)]
    TYPE_NXT_TEST = 101,
    #[allow(non_camel_case_types)]
    TYPE_NXT_IIC = 123,
    #[allow(non_camel_case_types)]
    TYPE_TERMINAL = 124,
    #[allow(non_camel_case_types)]
    TYPE_UNKNOWN = 125,
    #[allow(non_camel_case_types)]
    TYPE_NONE = 126,
    #[allow(non_camel_case_types)]
    TYPE_ERROR = 127,
}

impl Type {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Type, &'static str> {
        match i {
            0 => Ok(Type::TYPE_KEEP),
            1 => Ok(Type::TYPE_NXT_TOUCH),
            2 => Ok(Type::TYPE_NXT_LIGHT),
            3 => Ok(Type::TYPE_NXT_SOUND),
            4 => Ok(Type::TYPE_NXT_COLOR),
            5 => Ok(Type::TYPE_NXT_ULTRASONIC),
            6 => Ok(Type::TYPE_NXT_TEMPERATURE),
            7 => Ok(Type::TYPE_TACHO),
            8 => Ok(Type::TYPE_MINITACHO),
            9 => Ok(Type::TYPE_NEWTACHO),
            16 => Ok(Type::TYPE_TOUCH),
            29 => Ok(Type::TYPE_COLOR),
            30 => Ok(Type::TYPE_ULTRASONIC),
            32 => Ok(Type::TYPE_GYRO),
            33 => Ok(Type::TYPE_IR),
            50 => Ok(Type::TYPE_THIRD_PARTY_START),
            98 => Ok(Type::TYPE_THIRD_PARTY_END),
            99 => Ok(Type::TYPE_ENERGYMETER),
            100 => Ok(Type::TYPE_IIC_UNKNOWN),
            101 => Ok(Type::TYPE_NXT_TEST),
            123 => Ok(Type::TYPE_NXT_IIC),
            124 => Ok(Type::TYPE_TERMINAL),
            125 => Ok(Type::TYPE_UNKNOWN),
            126 => Ok(Type::TYPE_NONE),
            127 => Ok(Type::TYPE_ERROR),
            _ => Err("Invalid enum value for Type")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Type::TYPE_KEEP => "TYPE_KEEP",
            Type::TYPE_NXT_TOUCH => "TYPE_NXT_TOUCH",
            Type::TYPE_NXT_LIGHT => "TYPE_NXT_LIGHT",
            Type::TYPE_NXT_SOUND => "TYPE_NXT_SOUND",
            Type::TYPE_NXT_COLOR => "TYPE_NXT_COLOR",
            Type::TYPE_NXT_ULTRASONIC => "TYPE_NXT_ULTRASONIC",
            Type::TYPE_NXT_TEMPERATURE => "TYPE_NXT_TEMPERATURE",
            Type::TYPE_TACHO => "TYPE_TACHO",
            Type::TYPE_MINITACHO => "TYPE_MINITACHO",
            Type::TYPE_NEWTACHO => "TYPE_NEWTACHO",
            Type::TYPE_TOUCH => "TYPE_TOUCH",
            Type::TYPE_COLOR => "TYPE_COLOR",
            Type::TYPE_ULTRASONIC => "TYPE_ULTRASONIC",
            Type::TYPE_GYRO => "TYPE_GYRO",
            Type::TYPE_IR => "TYPE_IR",
            Type::TYPE_THIRD_PARTY_START => "TYPE_THIRD_PARTY_START",
            Type::TYPE_THIRD_PARTY_END => "TYPE_THIRD_PARTY_END",
            Type::TYPE_ENERGYMETER => "TYPE_ENERGYMETER",
            Type::TYPE_IIC_UNKNOWN => "TYPE_IIC_UNKNOWN",
            Type::TYPE_NXT_TEST => "TYPE_NXT_TEST",
            Type::TYPE_NXT_IIC => "TYPE_NXT_IIC",
            Type::TYPE_TERMINAL => "TYPE_TERMINAL",
            Type::TYPE_UNKNOWN => "TYPE_UNKNOWN",
            Type::TYPE_NONE => "TYPE_NONE",
            Type::TYPE_ERROR => "TYPE_ERROR",
        }
    }
}
#[allow(dead_code)]
pub enum Slot {
    #[allow(non_camel_case_types)]
    GUI_SLOT = 0,
    #[allow(non_camel_case_types)]
    USER_SLOT = 1,
    #[allow(non_camel_case_types)]
    CMD_SLOT = 2,
    #[allow(non_camel_case_types)]
    TERM_SLOT = 3,
    #[allow(non_camel_case_types)]
    DEBUG_SLOT = 4,
}

impl Slot {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Slot, &'static str> {
        match i {
            0 => Ok(Slot::GUI_SLOT),
            1 => Ok(Slot::USER_SLOT),
            2 => Ok(Slot::CMD_SLOT),
            3 => Ok(Slot::TERM_SLOT),
            4 => Ok(Slot::DEBUG_SLOT),
            _ => Err("Invalid enum value for Slot")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Slot::GUI_SLOT => "GUI_SLOT",
            Slot::USER_SLOT => "USER_SLOT",
            Slot::CMD_SLOT => "CMD_SLOT",
            Slot::TERM_SLOT => "TERM_SLOT",
            Slot::DEBUG_SLOT => "DEBUG_SLOT",
        }
    }
}
#[allow(dead_code)]
pub enum Buttontype {
    #[allow(non_camel_case_types)]
    NO_BUTTON = 0,
    #[allow(non_camel_case_types)]
    UP_BUTTON = 1,
    #[allow(non_camel_case_types)]
    ENTER_BUTTON = 2,
    #[allow(non_camel_case_types)]
    DOWN_BUTTON = 3,
    #[allow(non_camel_case_types)]
    RIGHT_BUTTON = 4,
    #[allow(non_camel_case_types)]
    LEFT_BUTTON = 5,
    #[allow(non_camel_case_types)]
    BACK_BUTTON = 6,
    #[allow(non_camel_case_types)]
    ANY_BUTTON = 7,
    #[allow(non_camel_case_types)]
    BUTTONTYPES = 8,
}

impl Buttontype {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Buttontype, &'static str> {
        match i {
            0 => Ok(Buttontype::NO_BUTTON),
            1 => Ok(Buttontype::UP_BUTTON),
            2 => Ok(Buttontype::ENTER_BUTTON),
            3 => Ok(Buttontype::DOWN_BUTTON),
            4 => Ok(Buttontype::RIGHT_BUTTON),
            5 => Ok(Buttontype::LEFT_BUTTON),
            6 => Ok(Buttontype::BACK_BUTTON),
            7 => Ok(Buttontype::ANY_BUTTON),
            8 => Ok(Buttontype::BUTTONTYPES),
            _ => Err("Invalid enum value for Buttontype")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Buttontype::NO_BUTTON => "NO_BUTTON",
            Buttontype::UP_BUTTON => "UP_BUTTON",
            Buttontype::ENTER_BUTTON => "ENTER_BUTTON",
            Buttontype::DOWN_BUTTON => "DOWN_BUTTON",
            Buttontype::RIGHT_BUTTON => "RIGHT_BUTTON",
            Buttontype::LEFT_BUTTON => "LEFT_BUTTON",
            Buttontype::BACK_BUTTON => "BACK_BUTTON",
            Buttontype::ANY_BUTTON => "ANY_BUTTON",
            Buttontype::BUTTONTYPES => "BUTTONTYPES",
        }
    }
}
#[allow(dead_code)]
pub enum MathSubcode {
    #[allow(non_camel_case_types)]
    EXP = 1,
    #[allow(non_camel_case_types)]
    MOD = 2,
    #[allow(non_camel_case_types)]
    FLOOR = 3,
    #[allow(non_camel_case_types)]
    CEIL = 4,
    #[allow(non_camel_case_types)]
    ROUND = 5,
    #[allow(non_camel_case_types)]
    ABS = 6,
    #[allow(non_camel_case_types)]
    NEGATE = 7,
    #[allow(non_camel_case_types)]
    SQRT = 8,
    #[allow(non_camel_case_types)]
    LOG = 9,
    #[allow(non_camel_case_types)]
    LN = 10,
    #[allow(non_camel_case_types)]
    SIN = 11,
    #[allow(non_camel_case_types)]
    COS = 12,
    #[allow(non_camel_case_types)]
    TAN = 13,
    #[allow(non_camel_case_types)]
    ASIN = 14,
    #[allow(non_camel_case_types)]
    ACOS = 15,
    #[allow(non_camel_case_types)]
    ATAN = 16,
    #[allow(non_camel_case_types)]
    MOD8 = 17,
    #[allow(non_camel_case_types)]
    MOD16 = 18,
    #[allow(non_camel_case_types)]
    MOD32 = 19,
    #[allow(non_camel_case_types)]
    POW = 20,
    #[allow(non_camel_case_types)]
    TRUNC = 21,
}

impl MathSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<MathSubcode, &'static str> {
        match i {
            1 => Ok(MathSubcode::EXP),
            2 => Ok(MathSubcode::MOD),
            3 => Ok(MathSubcode::FLOOR),
            4 => Ok(MathSubcode::CEIL),
            5 => Ok(MathSubcode::ROUND),
            6 => Ok(MathSubcode::ABS),
            7 => Ok(MathSubcode::NEGATE),
            8 => Ok(MathSubcode::SQRT),
            9 => Ok(MathSubcode::LOG),
            10 => Ok(MathSubcode::LN),
            11 => Ok(MathSubcode::SIN),
            12 => Ok(MathSubcode::COS),
            13 => Ok(MathSubcode::TAN),
            14 => Ok(MathSubcode::ASIN),
            15 => Ok(MathSubcode::ACOS),
            16 => Ok(MathSubcode::ATAN),
            17 => Ok(MathSubcode::MOD8),
            18 => Ok(MathSubcode::MOD16),
            19 => Ok(MathSubcode::MOD32),
            20 => Ok(MathSubcode::POW),
            21 => Ok(MathSubcode::TRUNC),
            _ => Err("Invalid enum value for MathSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            MathSubcode::EXP => "EXP",
            MathSubcode::MOD => "MOD",
            MathSubcode::FLOOR => "FLOOR",
            MathSubcode::CEIL => "CEIL",
            MathSubcode::ROUND => "ROUND",
            MathSubcode::ABS => "ABS",
            MathSubcode::NEGATE => "NEGATE",
            MathSubcode::SQRT => "SQRT",
            MathSubcode::LOG => "LOG",
            MathSubcode::LN => "LN",
            MathSubcode::SIN => "SIN",
            MathSubcode::COS => "COS",
            MathSubcode::TAN => "TAN",
            MathSubcode::ASIN => "ASIN",
            MathSubcode::ACOS => "ACOS",
            MathSubcode::ATAN => "ATAN",
            MathSubcode::MOD8 => "MOD8",
            MathSubcode::MOD16 => "MOD16",
            MathSubcode::MOD32 => "MOD32",
            MathSubcode::POW => "POW",
            MathSubcode::TRUNC => "TRUNC",
        }
    }
}
#[allow(dead_code)]
pub enum TstSubcode {
    #[allow(non_camel_case_types)]
    OPEN = 10,
    #[allow(non_camel_case_types)]
    CLOSE = 11,
    #[allow(non_camel_case_types)]
    READ_PINS = 12,
    #[allow(non_camel_case_types)]
    WRITE_PINS = 13,
    #[allow(non_camel_case_types)]
    READ_ADC = 14,
    #[allow(non_camel_case_types)]
    WRITE_UART = 15,
    #[allow(non_camel_case_types)]
    READ_UART = 16,
    #[allow(non_camel_case_types)]
    ENABLE_UART = 17,
    #[allow(non_camel_case_types)]
    DISABLE_UART = 18,
    #[allow(non_camel_case_types)]
    ACCU_SWITCH = 19,
    #[allow(non_camel_case_types)]
    BOOT_MODE2 = 20,
    #[allow(non_camel_case_types)]
    POLL_MODE2 = 21,
    #[allow(non_camel_case_types)]
    CLOSE_MODE2 = 22,
    #[allow(non_camel_case_types)]
    RAM_CHECK = 23,
}

impl TstSubcode {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<TstSubcode, &'static str> {
        match i {
            10 => Ok(TstSubcode::OPEN),
            11 => Ok(TstSubcode::CLOSE),
            12 => Ok(TstSubcode::READ_PINS),
            13 => Ok(TstSubcode::WRITE_PINS),
            14 => Ok(TstSubcode::READ_ADC),
            15 => Ok(TstSubcode::WRITE_UART),
            16 => Ok(TstSubcode::READ_UART),
            17 => Ok(TstSubcode::ENABLE_UART),
            18 => Ok(TstSubcode::DISABLE_UART),
            19 => Ok(TstSubcode::ACCU_SWITCH),
            20 => Ok(TstSubcode::BOOT_MODE2),
            21 => Ok(TstSubcode::POLL_MODE2),
            22 => Ok(TstSubcode::CLOSE_MODE2),
            23 => Ok(TstSubcode::RAM_CHECK),
            _ => Err("Invalid enum value for TstSubcode")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            TstSubcode::OPEN => "OPEN",
            TstSubcode::CLOSE => "CLOSE",
            TstSubcode::READ_PINS => "READ_PINS",
            TstSubcode::WRITE_PINS => "WRITE_PINS",
            TstSubcode::READ_ADC => "READ_ADC",
            TstSubcode::WRITE_UART => "WRITE_UART",
            TstSubcode::READ_UART => "READ_UART",
            TstSubcode::ENABLE_UART => "ENABLE_UART",
            TstSubcode::DISABLE_UART => "DISABLE_UART",
            TstSubcode::ACCU_SWITCH => "ACCU_SWITCH",
            TstSubcode::BOOT_MODE2 => "BOOT_MODE2",
            TstSubcode::POLL_MODE2 => "POLL_MODE2",
            TstSubcode::CLOSE_MODE2 => "CLOSE_MODE2",
            TstSubcode::RAM_CHECK => "RAM_CHECK",
        }
    }
}
#[allow(dead_code)]
pub enum Browsertype {
    #[allow(non_camel_case_types)]
    BROWSE_FOLDERS = 0,
    #[allow(non_camel_case_types)]
    BROWSE_FOLDS_FILES = 1,
    #[allow(non_camel_case_types)]
    BROWSE_CACHE = 2,
    #[allow(non_camel_case_types)]
    BROWSE_FILES = 3,
}

impl Browsertype {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Browsertype, &'static str> {
        match i {
            0 => Ok(Browsertype::BROWSE_FOLDERS),
            1 => Ok(Browsertype::BROWSE_FOLDS_FILES),
            2 => Ok(Browsertype::BROWSE_CACHE),
            3 => Ok(Browsertype::BROWSE_FILES),
            _ => Err("Invalid enum value for Browsertype")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Browsertype::BROWSE_FOLDERS => "BROWSE_FOLDERS",
            Browsertype::BROWSE_FOLDS_FILES => "BROWSE_FOLDS_FILES",
            Browsertype::BROWSE_CACHE => "BROWSE_CACHE",
            Browsertype::BROWSE_FILES => "BROWSE_FILES",
        }
    }
}
#[allow(dead_code)]
pub enum Fonttype {
    #[allow(non_camel_case_types)]
    NORMAL_FONT = 0,
    #[allow(non_camel_case_types)]
    SMALL_FONT = 1,
    #[allow(non_camel_case_types)]
    LARGE_FONT = 2,
    #[allow(non_camel_case_types)]
    TINY_FONT = 3,
}

impl Fonttype {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Fonttype, &'static str> {
        match i {
            0 => Ok(Fonttype::NORMAL_FONT),
            1 => Ok(Fonttype::SMALL_FONT),
            2 => Ok(Fonttype::LARGE_FONT),
            3 => Ok(Fonttype::TINY_FONT),
            _ => Err("Invalid enum value for Fonttype")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Fonttype::NORMAL_FONT => "NORMAL_FONT",
            Fonttype::SMALL_FONT => "SMALL_FONT",
            Fonttype::LARGE_FONT => "LARGE_FONT",
            Fonttype::TINY_FONT => "TINY_FONT",
        }
    }
}
#[allow(dead_code)]
pub enum Icontype {
    #[allow(non_camel_case_types)]
    NORMAL_ICON = 0,
    #[allow(non_camel_case_types)]
    SMALL_ICON = 1,
    #[allow(non_camel_case_types)]
    LARGE_ICON = 2,
    #[allow(non_camel_case_types)]
    MENU_ICON = 3,
    #[allow(non_camel_case_types)]
    ARROW_ICON = 4,
}

impl Icontype {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Icontype, &'static str> {
        match i {
            0 => Ok(Icontype::NORMAL_ICON),
            1 => Ok(Icontype::SMALL_ICON),
            2 => Ok(Icontype::LARGE_ICON),
            3 => Ok(Icontype::MENU_ICON),
            4 => Ok(Icontype::ARROW_ICON),
            _ => Err("Invalid enum value for Icontype")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Icontype::NORMAL_ICON => "NORMAL_ICON",
            Icontype::SMALL_ICON => "SMALL_ICON",
            Icontype::LARGE_ICON => "LARGE_ICON",
            Icontype::MENU_ICON => "MENU_ICON",
            Icontype::ARROW_ICON => "ARROW_ICON",
        }
    }
}
#[allow(dead_code)]
pub enum SIconNo {
    #[allow(non_camel_case_types)]
    SICON_CHARGING = 0,
    #[allow(non_camel_case_types)]
    SICON_BATT_4 = 1,
    #[allow(non_camel_case_types)]
    SICON_BATT_3 = 2,
    #[allow(non_camel_case_types)]
    SICON_BATT_2 = 3,
    #[allow(non_camel_case_types)]
    SICON_BATT_1 = 4,
    #[allow(non_camel_case_types)]
    SICON_BATT_0 = 5,
    #[allow(non_camel_case_types)]
    SICON_WAIT1 = 6,
    #[allow(non_camel_case_types)]
    SICON_WAIT2 = 7,
    #[allow(non_camel_case_types)]
    SICON_BT_ON = 8,
    #[allow(non_camel_case_types)]
    SICON_BT_VISIBLE = 9,
    #[allow(non_camel_case_types)]
    SICON_BT_CONNECTED = 10,
    #[allow(non_camel_case_types)]
    SICON_BT_CONNVISIB = 11,
    #[allow(non_camel_case_types)]
    SICON_WIFI_3 = 12,
    #[allow(non_camel_case_types)]
    SICON_WIFI_2 = 13,
    #[allow(non_camel_case_types)]
    SICON_WIFI_1 = 14,
    #[allow(non_camel_case_types)]
    SICON_WIFI_CONNECTED = 15,
    #[allow(non_camel_case_types)]
    SICON_USB = 21,
}

impl SIconNo {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<SIconNo, &'static str> {
        match i {
            0 => Ok(SIconNo::SICON_CHARGING),
            1 => Ok(SIconNo::SICON_BATT_4),
            2 => Ok(SIconNo::SICON_BATT_3),
            3 => Ok(SIconNo::SICON_BATT_2),
            4 => Ok(SIconNo::SICON_BATT_1),
            5 => Ok(SIconNo::SICON_BATT_0),
            6 => Ok(SIconNo::SICON_WAIT1),
            7 => Ok(SIconNo::SICON_WAIT2),
            8 => Ok(SIconNo::SICON_BT_ON),
            9 => Ok(SIconNo::SICON_BT_VISIBLE),
            10 => Ok(SIconNo::SICON_BT_CONNECTED),
            11 => Ok(SIconNo::SICON_BT_CONNVISIB),
            12 => Ok(SIconNo::SICON_WIFI_3),
            13 => Ok(SIconNo::SICON_WIFI_2),
            14 => Ok(SIconNo::SICON_WIFI_1),
            15 => Ok(SIconNo::SICON_WIFI_CONNECTED),
            21 => Ok(SIconNo::SICON_USB),
            _ => Err("Invalid enum value for SIconNo")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            SIconNo::SICON_CHARGING => "SICON_CHARGING",
            SIconNo::SICON_BATT_4 => "SICON_BATT_4",
            SIconNo::SICON_BATT_3 => "SICON_BATT_3",
            SIconNo::SICON_BATT_2 => "SICON_BATT_2",
            SIconNo::SICON_BATT_1 => "SICON_BATT_1",
            SIconNo::SICON_BATT_0 => "SICON_BATT_0",
            SIconNo::SICON_WAIT1 => "SICON_WAIT1",
            SIconNo::SICON_WAIT2 => "SICON_WAIT2",
            SIconNo::SICON_BT_ON => "SICON_BT_ON",
            SIconNo::SICON_BT_VISIBLE => "SICON_BT_VISIBLE",
            SIconNo::SICON_BT_CONNECTED => "SICON_BT_CONNECTED",
            SIconNo::SICON_BT_CONNVISIB => "SICON_BT_CONNVISIB",
            SIconNo::SICON_WIFI_3 => "SICON_WIFI_3",
            SIconNo::SICON_WIFI_2 => "SICON_WIFI_2",
            SIconNo::SICON_WIFI_1 => "SICON_WIFI_1",
            SIconNo::SICON_WIFI_CONNECTED => "SICON_WIFI_CONNECTED",
            SIconNo::SICON_USB => "SICON_USB",
        }
    }
}
#[allow(dead_code)]
pub enum NIconNo {
    #[allow(non_camel_case_types)]
    ICON_RUN = 0,
    #[allow(non_camel_case_types)]
    ICON_FOLDER = 1,
    #[allow(non_camel_case_types)]
    ICON_FOLDER2 = 2,
    #[allow(non_camel_case_types)]
    ICON_USB = 3,
    #[allow(non_camel_case_types)]
    ICON_SD = 4,
    #[allow(non_camel_case_types)]
    ICON_SOUND = 5,
    #[allow(non_camel_case_types)]
    ICON_IMAGE = 6,
    #[allow(non_camel_case_types)]
    ICON_SETTINGS = 7,
    #[allow(non_camel_case_types)]
    ICON_ONOFF = 8,
    #[allow(non_camel_case_types)]
    ICON_SEARCH = 9,
    #[allow(non_camel_case_types)]
    ICON_WIFI = 10,
    #[allow(non_camel_case_types)]
    ICON_CONNECTIONS = 11,
    #[allow(non_camel_case_types)]
    ICON_ADD_HIDDEN = 12,
    #[allow(non_camel_case_types)]
    ICON_TRASHBIN = 13,
    #[allow(non_camel_case_types)]
    ICON_VISIBILITY = 14,
    #[allow(non_camel_case_types)]
    ICON_KEY = 15,
    #[allow(non_camel_case_types)]
    ICON_CONNECT = 16,
    #[allow(non_camel_case_types)]
    ICON_DISCONNECT = 17,
    #[allow(non_camel_case_types)]
    ICON_UP = 18,
    #[allow(non_camel_case_types)]
    ICON_DOWN = 19,
    #[allow(non_camel_case_types)]
    ICON_WAIT1 = 20,
    #[allow(non_camel_case_types)]
    ICON_WAIT2 = 21,
    #[allow(non_camel_case_types)]
    ICON_BLUETOOTH = 22,
    #[allow(non_camel_case_types)]
    ICON_INFO = 23,
    #[allow(non_camel_case_types)]
    ICON_TEXT = 24,
    #[allow(non_camel_case_types)]
    ICON_QUESTIONMARK = 27,
    #[allow(non_camel_case_types)]
    ICON_INFO_FILE = 28,
    #[allow(non_camel_case_types)]
    ICON_DISC = 29,
    #[allow(non_camel_case_types)]
    ICON_CONNECTED = 30,
    #[allow(non_camel_case_types)]
    ICON_OBP = 31,
    #[allow(non_camel_case_types)]
    ICON_OBD = 32,
    #[allow(non_camel_case_types)]
    ICON_OPENFOLDER = 33,
    #[allow(non_camel_case_types)]
    ICON_BRICK1 = 34,
}

impl NIconNo {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<NIconNo, &'static str> {
        match i {
            0 => Ok(NIconNo::ICON_RUN),
            1 => Ok(NIconNo::ICON_FOLDER),
            2 => Ok(NIconNo::ICON_FOLDER2),
            3 => Ok(NIconNo::ICON_USB),
            4 => Ok(NIconNo::ICON_SD),
            5 => Ok(NIconNo::ICON_SOUND),
            6 => Ok(NIconNo::ICON_IMAGE),
            7 => Ok(NIconNo::ICON_SETTINGS),
            8 => Ok(NIconNo::ICON_ONOFF),
            9 => Ok(NIconNo::ICON_SEARCH),
            10 => Ok(NIconNo::ICON_WIFI),
            11 => Ok(NIconNo::ICON_CONNECTIONS),
            12 => Ok(NIconNo::ICON_ADD_HIDDEN),
            13 => Ok(NIconNo::ICON_TRASHBIN),
            14 => Ok(NIconNo::ICON_VISIBILITY),
            15 => Ok(NIconNo::ICON_KEY),
            16 => Ok(NIconNo::ICON_CONNECT),
            17 => Ok(NIconNo::ICON_DISCONNECT),
            18 => Ok(NIconNo::ICON_UP),
            19 => Ok(NIconNo::ICON_DOWN),
            20 => Ok(NIconNo::ICON_WAIT1),
            21 => Ok(NIconNo::ICON_WAIT2),
            22 => Ok(NIconNo::ICON_BLUETOOTH),
            23 => Ok(NIconNo::ICON_INFO),
            24 => Ok(NIconNo::ICON_TEXT),
            27 => Ok(NIconNo::ICON_QUESTIONMARK),
            28 => Ok(NIconNo::ICON_INFO_FILE),
            29 => Ok(NIconNo::ICON_DISC),
            30 => Ok(NIconNo::ICON_CONNECTED),
            31 => Ok(NIconNo::ICON_OBP),
            32 => Ok(NIconNo::ICON_OBD),
            33 => Ok(NIconNo::ICON_OPENFOLDER),
            34 => Ok(NIconNo::ICON_BRICK1),
            _ => Err("Invalid enum value for NIconNo")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            NIconNo::ICON_RUN => "ICON_RUN",
            NIconNo::ICON_FOLDER => "ICON_FOLDER",
            NIconNo::ICON_FOLDER2 => "ICON_FOLDER2",
            NIconNo::ICON_USB => "ICON_USB",
            NIconNo::ICON_SD => "ICON_SD",
            NIconNo::ICON_SOUND => "ICON_SOUND",
            NIconNo::ICON_IMAGE => "ICON_IMAGE",
            NIconNo::ICON_SETTINGS => "ICON_SETTINGS",
            NIconNo::ICON_ONOFF => "ICON_ONOFF",
            NIconNo::ICON_SEARCH => "ICON_SEARCH",
            NIconNo::ICON_WIFI => "ICON_WIFI",
            NIconNo::ICON_CONNECTIONS => "ICON_CONNECTIONS",
            NIconNo::ICON_ADD_HIDDEN => "ICON_ADD_HIDDEN",
            NIconNo::ICON_TRASHBIN => "ICON_TRASHBIN",
            NIconNo::ICON_VISIBILITY => "ICON_VISIBILITY",
            NIconNo::ICON_KEY => "ICON_KEY",
            NIconNo::ICON_CONNECT => "ICON_CONNECT",
            NIconNo::ICON_DISCONNECT => "ICON_DISCONNECT",
            NIconNo::ICON_UP => "ICON_UP",
            NIconNo::ICON_DOWN => "ICON_DOWN",
            NIconNo::ICON_WAIT1 => "ICON_WAIT1",
            NIconNo::ICON_WAIT2 => "ICON_WAIT2",
            NIconNo::ICON_BLUETOOTH => "ICON_BLUETOOTH",
            NIconNo::ICON_INFO => "ICON_INFO",
            NIconNo::ICON_TEXT => "ICON_TEXT",
            NIconNo::ICON_QUESTIONMARK => "ICON_QUESTIONMARK",
            NIconNo::ICON_INFO_FILE => "ICON_INFO_FILE",
            NIconNo::ICON_DISC => "ICON_DISC",
            NIconNo::ICON_CONNECTED => "ICON_CONNECTED",
            NIconNo::ICON_OBP => "ICON_OBP",
            NIconNo::ICON_OBD => "ICON_OBD",
            NIconNo::ICON_OPENFOLDER => "ICON_OPENFOLDER",
            NIconNo::ICON_BRICK1 => "ICON_BRICK1",
        }
    }
}
#[allow(dead_code)]
pub enum LIconNo {
    #[allow(non_camel_case_types)]
    YES_NOTSEL = 0,
    #[allow(non_camel_case_types)]
    YES_SEL = 1,
    #[allow(non_camel_case_types)]
    NO_NOTSEL = 2,
    #[allow(non_camel_case_types)]
    NO_SEL = 3,
    #[allow(non_camel_case_types)]
    OFF = 4,
    #[allow(non_camel_case_types)]
    WAIT_VERT = 5,
    #[allow(non_camel_case_types)]
    WAIT_HORZ = 6,
    #[allow(non_camel_case_types)]
    TO_MANUAL = 7,
    #[allow(non_camel_case_types)]
    WARNSIGN = 8,
    #[allow(non_camel_case_types)]
    WARN_BATT = 9,
    #[allow(non_camel_case_types)]
    WARN_POWER = 10,
    #[allow(non_camel_case_types)]
    WARN_TEMP = 11,
    #[allow(non_camel_case_types)]
    NO_USBSTICK = 12,
    #[allow(non_camel_case_types)]
    TO_EXECUTE = 13,
    #[allow(non_camel_case_types)]
    TO_BRICK = 14,
    #[allow(non_camel_case_types)]
    TO_SDCARD = 15,
    #[allow(non_camel_case_types)]
    TO_USBSTICK = 16,
    #[allow(non_camel_case_types)]
    TO_BLUETOOTH = 17,
    #[allow(non_camel_case_types)]
    TO_WIFI = 18,
    #[allow(non_camel_case_types)]
    TO_TRASH = 19,
    #[allow(non_camel_case_types)]
    TO_COPY = 20,
    #[allow(non_camel_case_types)]
    TO_FILE = 21,
    #[allow(non_camel_case_types)]
    CHAR_ERROR = 22,
    #[allow(non_camel_case_types)]
    COPY_ERROR = 23,
    #[allow(non_camel_case_types)]
    PROGRAM_ERROR = 24,
    #[allow(non_camel_case_types)]
    WARN_MEMORY = 27,
}

impl LIconNo {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<LIconNo, &'static str> {
        match i {
            0 => Ok(LIconNo::YES_NOTSEL),
            1 => Ok(LIconNo::YES_SEL),
            2 => Ok(LIconNo::NO_NOTSEL),
            3 => Ok(LIconNo::NO_SEL),
            4 => Ok(LIconNo::OFF),
            5 => Ok(LIconNo::WAIT_VERT),
            6 => Ok(LIconNo::WAIT_HORZ),
            7 => Ok(LIconNo::TO_MANUAL),
            8 => Ok(LIconNo::WARNSIGN),
            9 => Ok(LIconNo::WARN_BATT),
            10 => Ok(LIconNo::WARN_POWER),
            11 => Ok(LIconNo::WARN_TEMP),
            12 => Ok(LIconNo::NO_USBSTICK),
            13 => Ok(LIconNo::TO_EXECUTE),
            14 => Ok(LIconNo::TO_BRICK),
            15 => Ok(LIconNo::TO_SDCARD),
            16 => Ok(LIconNo::TO_USBSTICK),
            17 => Ok(LIconNo::TO_BLUETOOTH),
            18 => Ok(LIconNo::TO_WIFI),
            19 => Ok(LIconNo::TO_TRASH),
            20 => Ok(LIconNo::TO_COPY),
            21 => Ok(LIconNo::TO_FILE),
            22 => Ok(LIconNo::CHAR_ERROR),
            23 => Ok(LIconNo::COPY_ERROR),
            24 => Ok(LIconNo::PROGRAM_ERROR),
            27 => Ok(LIconNo::WARN_MEMORY),
            _ => Err("Invalid enum value for LIconNo")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            LIconNo::YES_NOTSEL => "YES_NOTSEL",
            LIconNo::YES_SEL => "YES_SEL",
            LIconNo::NO_NOTSEL => "NO_NOTSEL",
            LIconNo::NO_SEL => "NO_SEL",
            LIconNo::OFF => "OFF",
            LIconNo::WAIT_VERT => "WAIT_VERT",
            LIconNo::WAIT_HORZ => "WAIT_HORZ",
            LIconNo::TO_MANUAL => "TO_MANUAL",
            LIconNo::WARNSIGN => "WARNSIGN",
            LIconNo::WARN_BATT => "WARN_BATT",
            LIconNo::WARN_POWER => "WARN_POWER",
            LIconNo::WARN_TEMP => "WARN_TEMP",
            LIconNo::NO_USBSTICK => "NO_USBSTICK",
            LIconNo::TO_EXECUTE => "TO_EXECUTE",
            LIconNo::TO_BRICK => "TO_BRICK",
            LIconNo::TO_SDCARD => "TO_SDCARD",
            LIconNo::TO_USBSTICK => "TO_USBSTICK",
            LIconNo::TO_BLUETOOTH => "TO_BLUETOOTH",
            LIconNo::TO_WIFI => "TO_WIFI",
            LIconNo::TO_TRASH => "TO_TRASH",
            LIconNo::TO_COPY => "TO_COPY",
            LIconNo::TO_FILE => "TO_FILE",
            LIconNo::CHAR_ERROR => "CHAR_ERROR",
            LIconNo::COPY_ERROR => "COPY_ERROR",
            LIconNo::PROGRAM_ERROR => "PROGRAM_ERROR",
            LIconNo::WARN_MEMORY => "WARN_MEMORY",
        }
    }
}
#[allow(dead_code)]
pub enum MIconNo {
    #[allow(non_camel_case_types)]
    ICON_STAR = 0,
    #[allow(non_camel_case_types)]
    ICON_LOCKSTAR = 1,
    #[allow(non_camel_case_types)]
    ICON_LOCK = 2,
    #[allow(non_camel_case_types)]
    ICON_PC = 3,
    #[allow(non_camel_case_types)]
    ICON_PHONE = 4,
    #[allow(non_camel_case_types)]
    ICON_BRICK = 5,
    #[allow(non_camel_case_types)]
    ICON_UNKNOWN = 6,
    #[allow(non_camel_case_types)]
    ICON_FROM_FOLDER = 7,
    #[allow(non_camel_case_types)]
    ICON_CHECKBOX = 8,
    #[allow(non_camel_case_types)]
    ICON_CHECKED = 9,
    #[allow(non_camel_case_types)]
    ICON_XED = 10,
}

impl MIconNo {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<MIconNo, &'static str> {
        match i {
            0 => Ok(MIconNo::ICON_STAR),
            1 => Ok(MIconNo::ICON_LOCKSTAR),
            2 => Ok(MIconNo::ICON_LOCK),
            3 => Ok(MIconNo::ICON_PC),
            4 => Ok(MIconNo::ICON_PHONE),
            5 => Ok(MIconNo::ICON_BRICK),
            6 => Ok(MIconNo::ICON_UNKNOWN),
            7 => Ok(MIconNo::ICON_FROM_FOLDER),
            8 => Ok(MIconNo::ICON_CHECKBOX),
            9 => Ok(MIconNo::ICON_CHECKED),
            10 => Ok(MIconNo::ICON_XED),
            _ => Err("Invalid enum value for MIconNo")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            MIconNo::ICON_STAR => "ICON_STAR",
            MIconNo::ICON_LOCKSTAR => "ICON_LOCKSTAR",
            MIconNo::ICON_LOCK => "ICON_LOCK",
            MIconNo::ICON_PC => "ICON_PC",
            MIconNo::ICON_PHONE => "ICON_PHONE",
            MIconNo::ICON_BRICK => "ICON_BRICK",
            MIconNo::ICON_UNKNOWN => "ICON_UNKNOWN",
            MIconNo::ICON_FROM_FOLDER => "ICON_FROM_FOLDER",
            MIconNo::ICON_CHECKBOX => "ICON_CHECKBOX",
            MIconNo::ICON_CHECKED => "ICON_CHECKED",
            MIconNo::ICON_XED => "ICON_XED",
        }
    }
}
#[allow(dead_code)]
pub enum AIconNo {
    #[allow(non_camel_case_types)]
    ICON_LEFT = 1,
    #[allow(non_camel_case_types)]
    ICON_RIGHT = 2,
}

impl AIconNo {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<AIconNo, &'static str> {
        match i {
            1 => Ok(AIconNo::ICON_LEFT),
            2 => Ok(AIconNo::ICON_RIGHT),
            _ => Err("Invalid enum value for AIconNo")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            AIconNo::ICON_LEFT => "ICON_LEFT",
            AIconNo::ICON_RIGHT => "ICON_RIGHT",
        }
    }
}
#[allow(dead_code)]
pub enum Bttype {
    #[allow(non_camel_case_types)]
    BTTYPE_PC = 3,
    #[allow(non_camel_case_types)]
    BTTYPE_PHONE = 4,
    #[allow(non_camel_case_types)]
    BTTYPE_BRICK = 5,
    #[allow(non_camel_case_types)]
    BTTYPE_UNKNOWN = 6,
}

impl Bttype {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Bttype, &'static str> {
        match i {
            3 => Ok(Bttype::BTTYPE_PC),
            4 => Ok(Bttype::BTTYPE_PHONE),
            5 => Ok(Bttype::BTTYPE_BRICK),
            6 => Ok(Bttype::BTTYPE_UNKNOWN),
            _ => Err("Invalid enum value for Bttype")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Bttype::BTTYPE_PC => "BTTYPE_PC",
            Bttype::BTTYPE_PHONE => "BTTYPE_PHONE",
            Bttype::BTTYPE_BRICK => "BTTYPE_BRICK",
            Bttype::BTTYPE_UNKNOWN => "BTTYPE_UNKNOWN",
        }
    }
}
#[allow(dead_code)]
pub enum Ledpattern {
    #[allow(non_camel_case_types)]
    LED_BLACK = 0,
    #[allow(non_camel_case_types)]
    LED_GREEN = 1,
    #[allow(non_camel_case_types)]
    LED_RED = 2,
    #[allow(non_camel_case_types)]
    LED_ORANGE = 3,
    #[allow(non_camel_case_types)]
    LED_GREEN_FLASH = 4,
    #[allow(non_camel_case_types)]
    LED_RED_FLASH = 5,
    #[allow(non_camel_case_types)]
    LED_ORANGE_FLASH = 6,
    #[allow(non_camel_case_types)]
    LED_GREEN_PULSE = 7,
    #[allow(non_camel_case_types)]
    LED_RED_PULSE = 8,
    #[allow(non_camel_case_types)]
    LED_ORANGE_PULSE = 9,
}

impl Ledpattern {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Ledpattern, &'static str> {
        match i {
            0 => Ok(Ledpattern::LED_BLACK),
            1 => Ok(Ledpattern::LED_GREEN),
            2 => Ok(Ledpattern::LED_RED),
            3 => Ok(Ledpattern::LED_ORANGE),
            4 => Ok(Ledpattern::LED_GREEN_FLASH),
            5 => Ok(Ledpattern::LED_RED_FLASH),
            6 => Ok(Ledpattern::LED_ORANGE_FLASH),
            7 => Ok(Ledpattern::LED_GREEN_PULSE),
            8 => Ok(Ledpattern::LED_RED_PULSE),
            9 => Ok(Ledpattern::LED_ORANGE_PULSE),
            _ => Err("Invalid enum value for Ledpattern")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Ledpattern::LED_BLACK => "LED_BLACK",
            Ledpattern::LED_GREEN => "LED_GREEN",
            Ledpattern::LED_RED => "LED_RED",
            Ledpattern::LED_ORANGE => "LED_ORANGE",
            Ledpattern::LED_GREEN_FLASH => "LED_GREEN_FLASH",
            Ledpattern::LED_RED_FLASH => "LED_RED_FLASH",
            Ledpattern::LED_ORANGE_FLASH => "LED_ORANGE_FLASH",
            Ledpattern::LED_GREEN_PULSE => "LED_GREEN_PULSE",
            Ledpattern::LED_RED_PULSE => "LED_RED_PULSE",
            Ledpattern::LED_ORANGE_PULSE => "LED_ORANGE_PULSE",
        }
    }
}
#[allow(dead_code)]
pub enum Ledtype {
    #[allow(non_camel_case_types)]
    LED_ALL = 0,
    #[allow(non_camel_case_types)]
    LED_RR = 1,
    #[allow(non_camel_case_types)]
    LED_RG = 2,
    #[allow(non_camel_case_types)]
    LED_LR = 3,
    #[allow(non_camel_case_types)]
    LED_LG = 4,
}

impl Ledtype {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Ledtype, &'static str> {
        match i {
            0 => Ok(Ledtype::LED_ALL),
            1 => Ok(Ledtype::LED_RR),
            2 => Ok(Ledtype::LED_RG),
            3 => Ok(Ledtype::LED_LR),
            4 => Ok(Ledtype::LED_LG),
            _ => Err("Invalid enum value for Ledtype")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Ledtype::LED_ALL => "LED_ALL",
            Ledtype::LED_RR => "LED_RR",
            Ledtype::LED_RG => "LED_RG",
            Ledtype::LED_LR => "LED_LR",
            Ledtype::LED_LG => "LED_LG",
        }
    }
}
#[allow(dead_code)]
pub enum Filetype {
    #[allow(non_camel_case_types)]
    FILETYPE_UNKNOWN = 0x00,
    #[allow(non_camel_case_types)]
    TYPE_FOLDER = 0x01,
    #[allow(non_camel_case_types)]
    TYPE_SOUND = 0x02,
    #[allow(non_camel_case_types)]
    TYPE_BYTECODE = 0x03,
    #[allow(non_camel_case_types)]
    TYPE_GRAPHICS = 0x04,
    #[allow(non_camel_case_types)]
    TYPE_DATALOG = 0x05,
    #[allow(non_camel_case_types)]
    TYPE_PROGRAM = 0x06,
    #[allow(non_camel_case_types)]
    TYPE_TEXT = 0x07,
    #[allow(non_camel_case_types)]
    TYPE_SDCARD = 0x10,
    #[allow(non_camel_case_types)]
    TYPE_USBSTICK = 0x20,
}

impl Filetype {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Filetype, &'static str> {
        match i {
            0x00 => Ok(Filetype::FILETYPE_UNKNOWN),
            0x01 => Ok(Filetype::TYPE_FOLDER),
            0x02 => Ok(Filetype::TYPE_SOUND),
            0x03 => Ok(Filetype::TYPE_BYTECODE),
            0x04 => Ok(Filetype::TYPE_GRAPHICS),
            0x05 => Ok(Filetype::TYPE_DATALOG),
            0x06 => Ok(Filetype::TYPE_PROGRAM),
            0x07 => Ok(Filetype::TYPE_TEXT),
            0x10 => Ok(Filetype::TYPE_SDCARD),
            0x20 => Ok(Filetype::TYPE_USBSTICK),
            _ => Err("Invalid enum value for Filetype")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Filetype::FILETYPE_UNKNOWN => "FILETYPE_UNKNOWN",
            Filetype::TYPE_FOLDER => "TYPE_FOLDER",
            Filetype::TYPE_SOUND => "TYPE_SOUND",
            Filetype::TYPE_BYTECODE => "TYPE_BYTECODE",
            Filetype::TYPE_GRAPHICS => "TYPE_GRAPHICS",
            Filetype::TYPE_DATALOG => "TYPE_DATALOG",
            Filetype::TYPE_PROGRAM => "TYPE_PROGRAM",
            Filetype::TYPE_TEXT => "TYPE_TEXT",
            Filetype::TYPE_SDCARD => "TYPE_SDCARD",
            Filetype::TYPE_USBSTICK => "TYPE_USBSTICK",
        }
    }
}
#[allow(dead_code)]
pub enum Result {
    #[allow(non_camel_case_types)]
    OK = 0,
    #[allow(non_camel_case_types)]
    BUSY = 1,
    #[allow(non_camel_case_types)]
    FAIL = 2,
    #[allow(non_camel_case_types)]
    STOP = 4,
    #[allow(non_camel_case_types)]
    START = 8,
}

impl Result {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Result, &'static str> {
        match i {
            0 => Ok(Result::OK),
            1 => Ok(Result::BUSY),
            2 => Ok(Result::FAIL),
            4 => Ok(Result::STOP),
            8 => Ok(Result::START),
            _ => Err("Invalid enum value for Result")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Result::OK => "OK",
            Result::BUSY => "BUSY",
            Result::FAIL => "FAIL",
            Result::STOP => "STOP",
            Result::START => "START",
        }
    }
}
#[allow(dead_code)]
pub enum DataFormat {
    #[allow(non_camel_case_types)]
    DATA_8 = 0x00,
    #[allow(non_camel_case_types)]
    DATA_16 = 0x01,
    #[allow(non_camel_case_types)]
    DATA_32 = 0x02,
    #[allow(non_camel_case_types)]
    DATA_F = 0x03,
    #[allow(non_camel_case_types)]
    DATA_S = 0x04,
    #[allow(non_camel_case_types)]
    DATA_A = 0x05,
    #[allow(non_camel_case_types)]
    DATA_V = 0x07,
    #[allow(non_camel_case_types)]
    DATA_PCT = 0x10,
    #[allow(non_camel_case_types)]
    DATA_RAW = 0x12,
    #[allow(non_camel_case_types)]
    DATA_SI = 0x13,
}

impl DataFormat {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<DataFormat, &'static str> {
        match i {
            0x00 => Ok(DataFormat::DATA_8),
            0x01 => Ok(DataFormat::DATA_16),
            0x02 => Ok(DataFormat::DATA_32),
            0x03 => Ok(DataFormat::DATA_F),
            0x04 => Ok(DataFormat::DATA_S),
            0x05 => Ok(DataFormat::DATA_A),
            0x07 => Ok(DataFormat::DATA_V),
            0x10 => Ok(DataFormat::DATA_PCT),
            0x12 => Ok(DataFormat::DATA_RAW),
            0x13 => Ok(DataFormat::DATA_SI),
            _ => Err("Invalid enum value for DataFormat")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            DataFormat::DATA_8 => "DATA_8",
            DataFormat::DATA_16 => "DATA_16",
            DataFormat::DATA_32 => "DATA_32",
            DataFormat::DATA_F => "DATA_F",
            DataFormat::DATA_S => "DATA_S",
            DataFormat::DATA_A => "DATA_A",
            DataFormat::DATA_V => "DATA_V",
            DataFormat::DATA_PCT => "DATA_PCT",
            DataFormat::DATA_RAW => "DATA_RAW",
            DataFormat::DATA_SI => "DATA_SI",
        }
    }
}
#[allow(dead_code)]
pub enum Del {
    #[allow(non_camel_case_types)]
    DEL_NONE = 0,
    #[allow(non_camel_case_types)]
    DEL_TAB = 1,
    #[allow(non_camel_case_types)]
    DEL_SPACE = 2,
    #[allow(non_camel_case_types)]
    DEL_RETURN = 3,
    #[allow(non_camel_case_types)]
    DEL_COLON = 4,
    #[allow(non_camel_case_types)]
    DEL_COMMA = 5,
    #[allow(non_camel_case_types)]
    DEL_LINEFEED = 6,
    #[allow(non_camel_case_types)]
    DEL_CRLF = 7,
}

impl Del {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Del, &'static str> {
        match i {
            0 => Ok(Del::DEL_NONE),
            1 => Ok(Del::DEL_TAB),
            2 => Ok(Del::DEL_SPACE),
            3 => Ok(Del::DEL_RETURN),
            4 => Ok(Del::DEL_COLON),
            5 => Ok(Del::DEL_COMMA),
            6 => Ok(Del::DEL_LINEFEED),
            7 => Ok(Del::DEL_CRLF),
            _ => Err("Invalid enum value for Del")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Del::DEL_NONE => "DEL_NONE",
            Del::DEL_TAB => "DEL_TAB",
            Del::DEL_SPACE => "DEL_SPACE",
            Del::DEL_RETURN => "DEL_RETURN",
            Del::DEL_COLON => "DEL_COLON",
            Del::DEL_COMMA => "DEL_COMMA",
            Del::DEL_LINEFEED => "DEL_LINEFEED",
            Del::DEL_CRLF => "DEL_CRLF",
        }
    }
}
#[allow(dead_code)]
pub enum HWType {
    #[allow(non_camel_case_types)]
    HW_USB = 1,
    #[allow(non_camel_case_types)]
    HW_BT = 2,
    #[allow(non_camel_case_types)]
    HW_WIFI = 3,
}

impl HWType {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<HWType, &'static str> {
        match i {
            1 => Ok(HWType::HW_USB),
            2 => Ok(HWType::HW_BT),
            3 => Ok(HWType::HW_WIFI),
            _ => Err("Invalid enum value for HWType")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            HWType::HW_USB => "HW_USB",
            HWType::HW_BT => "HW_BT",
            HWType::HW_WIFI => "HW_WIFI",
        }
    }
}
#[allow(dead_code)]
pub enum Encrypt {
    #[allow(non_camel_case_types)]
    ENCRYPT_NONE = 0,
    #[allow(non_camel_case_types)]
    ENCRYPT_WPA2 = 1,
}

impl Encrypt {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Encrypt, &'static str> {
        match i {
            0 => Ok(Encrypt::ENCRYPT_NONE),
            1 => Ok(Encrypt::ENCRYPT_WPA2),
            _ => Err("Invalid enum value for Encrypt")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Encrypt::ENCRYPT_NONE => "ENCRYPT_NONE",
            Encrypt::ENCRYPT_WPA2 => "ENCRYPT_WPA2",
        }
    }
}
#[allow(dead_code)]
pub enum Color {
    #[allow(non_camel_case_types)]
    RED = 0,
    #[allow(non_camel_case_types)]
    GREEN = 1,
    #[allow(non_camel_case_types)]
    BLUE = 2,
    #[allow(non_camel_case_types)]
    BLANK = 3,
}

impl Color {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Color, &'static str> {
        match i {
            0 => Ok(Color::RED),
            1 => Ok(Color::GREEN),
            2 => Ok(Color::BLUE),
            3 => Ok(Color::BLANK),
            _ => Err("Invalid enum value for Color")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Color::RED => "RED",
            Color::GREEN => "GREEN",
            Color::BLUE => "BLUE",
            Color::BLANK => "BLANK",
        }
    }
}
#[allow(dead_code)]
pub enum NXTColor {
    #[allow(non_camel_case_types)]
    BLACK = 1,
    #[allow(non_camel_case_types)]
    BLUE = 2,
    #[allow(non_camel_case_types)]
    GREEN = 3,
    #[allow(non_camel_case_types)]
    YELLOW = 4,
    #[allow(non_camel_case_types)]
    RED = 5,
    #[allow(non_camel_case_types)]
    WHITE = 6,
}

impl NXTColor {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<NXTColor, &'static str> {
        match i {
            1 => Ok(NXTColor::BLACK),
            2 => Ok(NXTColor::BLUE),
            3 => Ok(NXTColor::GREEN),
            4 => Ok(NXTColor::YELLOW),
            5 => Ok(NXTColor::RED),
            6 => Ok(NXTColor::WHITE),
            _ => Err("Invalid enum value for NXTColor")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            NXTColor::BLACK => "BLACK",
            NXTColor::BLUE => "BLUE",
            NXTColor::GREEN => "GREEN",
            NXTColor::YELLOW => "YELLOW",
            NXTColor::RED => "RED",
            NXTColor::WHITE => "WHITE",
        }
    }
}
#[allow(dead_code)]
pub enum Warning {
    #[allow(non_camel_case_types)]
    WARNING_TEMP = 0x01,
    #[allow(non_camel_case_types)]
    WARNING_CURRENT = 0x02,
    #[allow(non_camel_case_types)]
    WARNING_VOLTAGE = 0x04,
    #[allow(non_camel_case_types)]
    WARNING_MEMORY = 0x08,
    #[allow(non_camel_case_types)]
    WARNING_DSPSTAT = 0x10,
    #[allow(non_camel_case_types)]
    WARNING_RAM = 0x20,
    #[allow(non_camel_case_types)]
    WARNING_BATTLOW = 0x40,
    #[allow(non_camel_case_types)]
    WARNING_BUSY = 0x80,
    #[allow(non_camel_case_types)]
    WARNINGS = 0x3F,
}

impl Warning {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Warning, &'static str> {
        match i {
            0x01 => Ok(Warning::WARNING_TEMP),
            0x02 => Ok(Warning::WARNING_CURRENT),
            0x04 => Ok(Warning::WARNING_VOLTAGE),
            0x08 => Ok(Warning::WARNING_MEMORY),
            0x10 => Ok(Warning::WARNING_DSPSTAT),
            0x20 => Ok(Warning::WARNING_RAM),
            0x40 => Ok(Warning::WARNING_BATTLOW),
            0x80 => Ok(Warning::WARNING_BUSY),
            0x3F => Ok(Warning::WARNINGS),
            _ => Err("Invalid enum value for Warning")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Warning::WARNING_TEMP => "WARNING_TEMP",
            Warning::WARNING_CURRENT => "WARNING_CURRENT",
            Warning::WARNING_VOLTAGE => "WARNING_VOLTAGE",
            Warning::WARNING_MEMORY => "WARNING_MEMORY",
            Warning::WARNING_DSPSTAT => "WARNING_DSPSTAT",
            Warning::WARNING_RAM => "WARNING_RAM",
            Warning::WARNING_BATTLOW => "WARNING_BATTLOW",
            Warning::WARNING_BUSY => "WARNING_BUSY",
            Warning::WARNINGS => "WARNINGS",
        }
    }
}
#[allow(dead_code)]
pub enum Objstat {
    #[allow(non_camel_case_types)]
    RUNNING = 0x0010,
    #[allow(non_camel_case_types)]
    WAITING = 0x0020,
    #[allow(non_camel_case_types)]
    STOPPED = 0x0040,
    #[allow(non_camel_case_types)]
    HALTED = 0x0080,
}

impl Objstat {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Objstat, &'static str> {
        match i {
            0x0010 => Ok(Objstat::RUNNING),
            0x0020 => Ok(Objstat::WAITING),
            0x0040 => Ok(Objstat::STOPPED),
            0x0080 => Ok(Objstat::HALTED),
            _ => Err("Invalid enum value for Objstat")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Objstat::RUNNING => "RUNNING",
            Objstat::WAITING => "WAITING",
            Objstat::STOPPED => "STOPPED",
            Objstat::HALTED => "HALTED",
        }
    }
}
#[allow(dead_code)]
pub enum Devcmd {
    #[allow(non_camel_case_types)]
    DEVCMD_RESET = 0x10,
    #[allow(non_camel_case_types)]
    DEVCMD_FIRE = 0x11,
    #[allow(non_camel_case_types)]
    DEVCMD_CHANNEL = 0x12,
}

impl Devcmd {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<Devcmd, &'static str> {
        match i {
            0x10 => Ok(Devcmd::DEVCMD_RESET),
            0x11 => Ok(Devcmd::DEVCMD_FIRE),
            0x12 => Ok(Devcmd::DEVCMD_CHANNEL),
            _ => Err("Invalid enum value for Devcmd")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Devcmd::DEVCMD_RESET => "DEVCMD_RESET",
            Devcmd::DEVCMD_FIRE => "DEVCMD_FIRE",
            Devcmd::DEVCMD_CHANNEL => "DEVCMD_CHANNEL",
        }
    }
}

