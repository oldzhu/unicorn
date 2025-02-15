#![allow(non_camel_case_types)]

// ARM64 registers
#[repr(C)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RegisterARM64 {
    INVALID = 0,
    X29 = 1,
    X30 = 2,
    NZCV = 3,
    SP = 4,
    WSP = 5,
    WZR = 6,
    XZR = 7,
    B0 = 8,
    B1 = 9,
    B2 = 10,
    B3 = 11,
    B4 = 12,
    B5 = 13,
    B6 = 14,
    B7 = 15,
    B8 = 16,
    B9 = 17,
    B10 = 18,
    B11 = 19,
    B12 = 20,
    B13 = 21,
    B14 = 22,
    B15 = 23,
    B16 = 24,
    B17 = 25,
    B18 = 26,
    B19 = 27,
    B20 = 28,
    B21 = 29,
    B22 = 30,
    B23 = 31,
    B24 = 32,
    B25 = 33,
    B26 = 34,
    B27 = 35,
    B28 = 36,
    B29 = 37,
    B30 = 38,
    B31 = 39,
    D0 = 40,
    D1 = 41,
    D2 = 42,
    D3 = 43,
    D4 = 44,
    D5 = 45,
    D6 = 46,
    D7 = 47,
    D8 = 48,
    D9 = 49,
    D10 = 50,
    D11 = 51,
    D12 = 52,
    D13 = 53,
    D14 = 54,
    D15 = 55,
    D16 = 56,
    D17 = 57,
    D18 = 58,
    D19 = 59,
    D20 = 60,
    D21 = 61,
    D22 = 62,
    D23 = 63,
    D24 = 64,
    D25 = 65,
    D26 = 66,
    D27 = 67,
    D28 = 68,
    D29 = 69,
    D30 = 70,
    D31 = 71,
    H0 = 72,
    H1 = 73,
    H2 = 74,
    H3 = 75,
    H4 = 76,
    H5 = 77,
    H6 = 78,
    H7 = 79,
    H8 = 80,
    H9 = 81,
    H10 = 82,
    H11 = 83,
    H12 = 84,
    H13 = 85,
    H14 = 86,
    H15 = 87,
    H16 = 88,
    H17 = 89,
    H18 = 90,
    H19 = 91,
    H20 = 92,
    H21 = 93,
    H22 = 94,
    H23 = 95,
    H24 = 96,
    H25 = 97,
    H26 = 98,
    H27 = 99,
    H28 = 100,
    H29 = 101,
    H30 = 102,
    H31 = 103,
    Q0 = 104,
    Q1 = 105,
    Q2 = 106,
    Q3 = 107,
    Q4 = 108,
    Q5 = 109,
    Q6 = 110,
    Q7 = 111,
    Q8 = 112,
    Q9 = 113,
    Q10 = 114,
    Q11 = 115,
    Q12 = 116,
    Q13 = 117,
    Q14 = 118,
    Q15 = 119,
    Q16 = 120,
    Q17 = 121,
    Q18 = 122,
    Q19 = 123,
    Q20 = 124,
    Q21 = 125,
    Q22 = 126,
    Q23 = 127,
    Q24 = 128,
    Q25 = 129,
    Q26 = 130,
    Q27 = 131,
    Q28 = 132,
    Q29 = 133,
    Q30 = 134,
    Q31 = 135,
    S0 = 136,
    S1 = 137,
    S2 = 138,
    S3 = 139,
    S4 = 140,
    S5 = 141,
    S6 = 142,
    S7 = 143,
    S8 = 144,
    S9 = 145,
    S10 = 146,
    S11 = 147,
    S12 = 148,
    S13 = 149,
    S14 = 150,
    S15 = 151,
    S16 = 152,
    S17 = 153,
    S18 = 154,
    S19 = 155,
    S20 = 156,
    S21 = 157,
    S22 = 158,
    S23 = 159,
    S24 = 160,
    S25 = 161,
    S26 = 162,
    S27 = 163,
    S28 = 164,
    S29 = 165,
    S30 = 166,
    S31 = 167,
    W0 = 168,
    W1 = 169,
    W2 = 170,
    W3 = 171,
    W4 = 172,
    W5 = 173,
    W6 = 174,
    W7 = 175,
    W8 = 176,
    W9 = 177,
    W10 = 178,
    W11 = 179,
    W12 = 180,
    W13 = 181,
    W14 = 182,
    W15 = 183,
    W16 = 184,
    W17 = 185,
    W18 = 186,
    W19 = 187,
    W20 = 188,
    W21 = 189,
    W22 = 190,
    W23 = 191,
    W24 = 192,
    W25 = 193,
    W26 = 194,
    W27 = 195,
    W28 = 196,
    W29 = 197,
    W30 = 198,
    X0 = 199,
    X1 = 200,
    X2 = 201,
    X3 = 202,
    X4 = 203,
    X5 = 204,
    X6 = 205,
    X7 = 206,
    X8 = 207,
    X9 = 208,
    X10 = 209,
    X11 = 210,
    X12 = 211,
    X13 = 212,
    X14 = 213,
    X15 = 214,
    X16 = 215,
    X17 = 216,
    X18 = 217,
    X19 = 218,
    X20 = 219,
    X21 = 220,
    X22 = 221,
    X23 = 222,
    X24 = 223,
    X25 = 224,
    X26 = 225,
    X27 = 226,
    X28 = 227,
    V0 = 228,
    V1 = 229,
    V2 = 230,
    V3 = 231,
    V4 = 232,
    V5 = 233,
    V6 = 234,
    V7 = 235,
    V8 = 236,
    V9 = 237,
    V10 = 238,
    V11 = 239,
    V12 = 240,
    V13 = 241,
    V14 = 242,
    V15 = 243,
    V16 = 244,
    V17 = 245,
    V18 = 246,
    V19 = 247,
    V20 = 248,
    V21 = 249,
    V22 = 250,
    V23 = 251,
    V24 = 252,
    V25 = 253,
    V26 = 254,
    V27 = 255,
    V28 = 256,
    V29 = 257,
    V30 = 258,
    V31 = 259,

    // pseudo registers
    PC = 260,
    CPACR_EL1 = 261,

    // thread registers, depreciated, use CP_REG instead
    TPIDR_EL0 = 262,
    TPIDRRO_EL0 = 263,
    TPIDR_EL1 = 264,
    PSTATE = 265,

    // exception link registers, depreciated, use CP_REG instead
    ELR_EL0 = 266,
    ELR_EL1 = 267,
    ELR_EL2 = 268,
    ELR_EL3 = 269,

    // stack pointers registers, depreciated, use CP_REG instead
    SP_EL0 = 270,
    SP_EL1 = 271,
    SP_EL2 = 272,
    SP_EL3 = 273,

    // other CP15 registers, depreciated, use CP_REG instead
    TTBR0_EL1 = 274,
    TTBR1_EL1 = 275,
    ESR_EL0 = 276,
    ESR_EL1 = 277,
    ESR_EL2 = 278,
    ESR_EL3 = 279,
    FAR_EL0 = 280,
    FAR_EL1 = 281,
    FAR_EL2 = 282,
    FAR_EL3 = 283,
    PAR_EL1 = 284,
    MAIR_EL1 = 285,
    VBAR_EL0 = 286,
    VBAR_EL1 = 287,
    VBAR_EL2 = 288,
    VBAR_EL3 = 289,
    CP_REG = 290,
    ENDING = 291,
}

impl RegisterARM64 {
    // alias registers
    // (assoc) IP0 = 215,
    // (assoc) IP1 = 216,
    // (assoc) FP = 1,
    // (assoc) LR = 2,
    pub const IP0: RegisterARM64 = RegisterARM64::X16;
    pub const IP1: RegisterARM64 = RegisterARM64::X17;
    pub const FP: RegisterARM64 = RegisterARM64::X29;
    pub const LR: RegisterARM64 = RegisterARM64::X30;
}

impl From<RegisterARM64> for i32 {
    fn from(r: RegisterARM64) -> Self {
        r as i32
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Arm64CpuModel {
    UC_CPU_ARM64_A57 = 0,
    UC_CPU_ARM64_A53 = 1,
    UC_CPU_ARM64_A72 = 2,
    UC_CPU_ARM64_MAX = 3,
}

impl From<Arm64CpuModel> for i32 {
    fn from(value: Arm64CpuModel) -> Self {
        value as i32
    }
}

impl From<&Arm64CpuModel> for i32 {
    fn from(value: &Arm64CpuModel) -> Self {
        (*value) as i32
    }
}
