#[doc = "Register `cir_tac` reader"]
pub struct R(crate::R<CIR_TAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_TAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_TAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_TAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cir_tac` writer"]
pub struct W(crate::W<CIR_TAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_TAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CIR_TAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_TAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tac` reader - TX FIFO Available Space Counter"]
pub type TAC_R = crate::FieldReader<u8, TAC_A>;
#[doc = "TX FIFO Available Space Counter\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAC_A {
    #[doc = "0: No available space in TX FIFO"]
    B0 = 0,
    #[doc = "1: 1 byte available space in TX FIFO"]
    B1 = 1,
    #[doc = "2: 2 bytes available space in TX FIFO"]
    B2 = 2,
    #[doc = "3: 3 bytes available space in TX FIFO"]
    B3 = 3,
    #[doc = "4: 4 bytes available space in TX FIFO"]
    B4 = 4,
    #[doc = "5: 5 bytes available space in TX FIFO"]
    B5 = 5,
    #[doc = "6: 6 bytes available space in TX FIFO"]
    B6 = 6,
    #[doc = "7: 7 bytes available space in TX FIFO"]
    B7 = 7,
    #[doc = "8: 8 bytes available space in TX FIFO"]
    B8 = 8,
    #[doc = "9: 9 bytes available space in TX FIFO"]
    B9 = 9,
    #[doc = "10: 10 bytes available space in TX FIFO"]
    B10 = 10,
    #[doc = "11: 11 bytes available space in TX FIFO"]
    B11 = 11,
    #[doc = "12: 12 bytes available space in TX FIFO"]
    B12 = 12,
    #[doc = "13: 13 bytes available space in TX FIFO"]
    B13 = 13,
    #[doc = "14: 14 bytes available space in TX FIFO"]
    B14 = 14,
    #[doc = "15: 15 bytes available space in TX FIFO"]
    B15 = 15,
    #[doc = "16: 16 bytes available space in TX FIFO"]
    B16 = 16,
    #[doc = "17: 17 bytes available space in TX FIFO"]
    B17 = 17,
    #[doc = "18: 18 bytes available space in TX FIFO"]
    B18 = 18,
    #[doc = "19: 19 bytes available space in TX FIFO"]
    B19 = 19,
    #[doc = "20: 20 bytes available space in TX FIFO"]
    B20 = 20,
    #[doc = "21: 21 bytes available space in TX FIFO"]
    B21 = 21,
    #[doc = "22: 22 bytes available space in TX FIFO"]
    B22 = 22,
    #[doc = "23: 23 bytes available space in TX FIFO"]
    B23 = 23,
    #[doc = "24: 24 bytes available space in TX FIFO"]
    B24 = 24,
    #[doc = "25: 25 bytes available space in TX FIFO"]
    B25 = 25,
    #[doc = "26: 26 bytes available space in TX FIFO"]
    B26 = 26,
    #[doc = "27: 27 bytes available space in TX FIFO"]
    B27 = 27,
    #[doc = "28: 28 bytes available space in TX FIFO"]
    B28 = 28,
    #[doc = "29: 29 bytes available space in TX FIFO"]
    B29 = 29,
    #[doc = "30: 30 bytes available space in TX FIFO"]
    B30 = 30,
    #[doc = "31: 31 bytes available space in TX FIFO"]
    B31 = 31,
    #[doc = "32: 32 bytes available space in TX FIFO"]
    B32 = 32,
    #[doc = "33: 33 bytes available space in TX FIFO"]
    B33 = 33,
    #[doc = "34: 34 bytes available space in TX FIFO"]
    B34 = 34,
    #[doc = "35: 35 bytes available space in TX FIFO"]
    B35 = 35,
    #[doc = "36: 36 bytes available space in TX FIFO"]
    B36 = 36,
    #[doc = "37: 37 bytes available space in TX FIFO"]
    B37 = 37,
    #[doc = "38: 38 bytes available space in TX FIFO"]
    B38 = 38,
    #[doc = "39: 39 bytes available space in TX FIFO"]
    B39 = 39,
    #[doc = "40: 40 bytes available space in TX FIFO"]
    B40 = 40,
    #[doc = "41: 41 bytes available space in TX FIFO"]
    B41 = 41,
    #[doc = "42: 42 bytes available space in TX FIFO"]
    B42 = 42,
    #[doc = "43: 43 bytes available space in TX FIFO"]
    B43 = 43,
    #[doc = "44: 44 bytes available space in TX FIFO"]
    B44 = 44,
    #[doc = "45: 45 bytes available space in TX FIFO"]
    B45 = 45,
    #[doc = "46: 46 bytes available space in TX FIFO"]
    B46 = 46,
    #[doc = "47: 47 bytes available space in TX FIFO"]
    B47 = 47,
    #[doc = "48: 48 bytes available space in TX FIFO"]
    B48 = 48,
    #[doc = "49: 49 bytes available space in TX FIFO"]
    B49 = 49,
    #[doc = "50: 50 bytes available space in TX FIFO"]
    B50 = 50,
    #[doc = "51: 51 bytes available space in TX FIFO"]
    B51 = 51,
    #[doc = "52: 52 bytes available space in TX FIFO"]
    B52 = 52,
    #[doc = "53: 53 bytes available space in TX FIFO"]
    B53 = 53,
    #[doc = "54: 54 bytes available space in TX FIFO"]
    B54 = 54,
    #[doc = "55: 55 bytes available space in TX FIFO"]
    B55 = 55,
    #[doc = "56: 56 bytes available space in TX FIFO"]
    B56 = 56,
    #[doc = "57: 57 bytes available space in TX FIFO"]
    B57 = 57,
    #[doc = "58: 58 bytes available space in TX FIFO"]
    B58 = 58,
    #[doc = "59: 59 bytes available space in TX FIFO"]
    B59 = 59,
    #[doc = "60: 60 bytes available space in TX FIFO"]
    B60 = 60,
    #[doc = "61: 61 bytes available space in TX FIFO"]
    B61 = 61,
    #[doc = "62: 62 bytes available space in TX FIFO"]
    B62 = 62,
    #[doc = "63: 63 bytes available space in TX FIFO"]
    B63 = 63,
    #[doc = "64: 64 bytes available space in TX FIFO"]
    B64 = 64,
    #[doc = "65: 65 bytes available space in TX FIFO"]
    B65 = 65,
    #[doc = "66: 66 bytes available space in TX FIFO"]
    B66 = 66,
    #[doc = "67: 67 bytes available space in TX FIFO"]
    B67 = 67,
    #[doc = "68: 68 bytes available space in TX FIFO"]
    B68 = 68,
    #[doc = "69: 69 bytes available space in TX FIFO"]
    B69 = 69,
    #[doc = "70: 70 bytes available space in TX FIFO"]
    B70 = 70,
    #[doc = "71: 71 bytes available space in TX FIFO"]
    B71 = 71,
    #[doc = "72: 72 bytes available space in TX FIFO"]
    B72 = 72,
    #[doc = "73: 73 bytes available space in TX FIFO"]
    B73 = 73,
    #[doc = "74: 74 bytes available space in TX FIFO"]
    B74 = 74,
    #[doc = "75: 75 bytes available space in TX FIFO"]
    B75 = 75,
    #[doc = "76: 76 bytes available space in TX FIFO"]
    B76 = 76,
    #[doc = "77: 77 bytes available space in TX FIFO"]
    B77 = 77,
    #[doc = "78: 78 bytes available space in TX FIFO"]
    B78 = 78,
    #[doc = "79: 79 bytes available space in TX FIFO"]
    B79 = 79,
    #[doc = "80: 80 bytes available space in TX FIFO"]
    B80 = 80,
    #[doc = "81: 81 bytes available space in TX FIFO"]
    B81 = 81,
    #[doc = "82: 82 bytes available space in TX FIFO"]
    B82 = 82,
    #[doc = "83: 83 bytes available space in TX FIFO"]
    B83 = 83,
    #[doc = "84: 84 bytes available space in TX FIFO"]
    B84 = 84,
    #[doc = "85: 85 bytes available space in TX FIFO"]
    B85 = 85,
    #[doc = "86: 86 bytes available space in TX FIFO"]
    B86 = 86,
    #[doc = "87: 87 bytes available space in TX FIFO"]
    B87 = 87,
    #[doc = "88: 88 bytes available space in TX FIFO"]
    B88 = 88,
    #[doc = "89: 89 bytes available space in TX FIFO"]
    B89 = 89,
    #[doc = "90: 90 bytes available space in TX FIFO"]
    B90 = 90,
    #[doc = "91: 91 bytes available space in TX FIFO"]
    B91 = 91,
    #[doc = "92: 92 bytes available space in TX FIFO"]
    B92 = 92,
    #[doc = "93: 93 bytes available space in TX FIFO"]
    B93 = 93,
    #[doc = "94: 94 bytes available space in TX FIFO"]
    B94 = 94,
    #[doc = "95: 95 bytes available space in TX FIFO"]
    B95 = 95,
    #[doc = "96: 96 bytes available space in TX FIFO"]
    B96 = 96,
    #[doc = "97: 97 bytes available space in TX FIFO"]
    B97 = 97,
    #[doc = "98: 98 bytes available space in TX FIFO"]
    B98 = 98,
    #[doc = "99: 99 bytes available space in TX FIFO"]
    B99 = 99,
    #[doc = "100: 100 bytes available space in TX FIFO"]
    B100 = 100,
    #[doc = "101: 101 bytes available space in TX FIFO"]
    B101 = 101,
    #[doc = "102: 102 bytes available space in TX FIFO"]
    B102 = 102,
    #[doc = "103: 103 bytes available space in TX FIFO"]
    B103 = 103,
    #[doc = "104: 104 bytes available space in TX FIFO"]
    B104 = 104,
    #[doc = "105: 105 bytes available space in TX FIFO"]
    B105 = 105,
    #[doc = "106: 106 bytes available space in TX FIFO"]
    B106 = 106,
    #[doc = "107: 107 bytes available space in TX FIFO"]
    B107 = 107,
    #[doc = "108: 108 bytes available space in TX FIFO"]
    B108 = 108,
    #[doc = "109: 109 bytes available space in TX FIFO"]
    B109 = 109,
    #[doc = "110: 110 bytes available space in TX FIFO"]
    B110 = 110,
    #[doc = "111: 111 bytes available space in TX FIFO"]
    B111 = 111,
    #[doc = "112: 112 bytes available space in TX FIFO"]
    B112 = 112,
    #[doc = "113: 113 bytes available space in TX FIFO"]
    B113 = 113,
    #[doc = "114: 114 bytes available space in TX FIFO"]
    B114 = 114,
    #[doc = "115: 115 bytes available space in TX FIFO"]
    B115 = 115,
    #[doc = "116: 116 bytes available space in TX FIFO"]
    B116 = 116,
    #[doc = "117: 117 bytes available space in TX FIFO"]
    B117 = 117,
    #[doc = "118: 118 bytes available space in TX FIFO"]
    B118 = 118,
    #[doc = "119: 119 bytes available space in TX FIFO"]
    B119 = 119,
    #[doc = "120: 120 bytes available space in TX FIFO"]
    B120 = 120,
    #[doc = "121: 121 bytes available space in TX FIFO"]
    B121 = 121,
    #[doc = "122: 122 bytes available space in TX FIFO"]
    B122 = 122,
    #[doc = "123: 123 bytes available space in TX FIFO"]
    B123 = 123,
    #[doc = "124: 124 bytes available space in TX FIFO"]
    B124 = 124,
    #[doc = "125: 125 bytes available space in TX FIFO"]
    B125 = 125,
    #[doc = "126: 126 bytes available space in TX FIFO"]
    B126 = 126,
    #[doc = "127: 127 bytes available space in TX FIFO"]
    B127 = 127,
    #[doc = "128: 128 bytes available space in TX FIFO"]
    B128 = 128,
}
impl From<TAC_A> for u8 {
    #[inline(always)]
    fn from(variant: TAC_A) -> Self {
        variant as _
    }
}
impl TAC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TAC_A> {
        match self.bits {
            0 => Some(TAC_A::B0),
            1 => Some(TAC_A::B1),
            2 => Some(TAC_A::B2),
            3 => Some(TAC_A::B3),
            4 => Some(TAC_A::B4),
            5 => Some(TAC_A::B5),
            6 => Some(TAC_A::B6),
            7 => Some(TAC_A::B7),
            8 => Some(TAC_A::B8),
            9 => Some(TAC_A::B9),
            10 => Some(TAC_A::B10),
            11 => Some(TAC_A::B11),
            12 => Some(TAC_A::B12),
            13 => Some(TAC_A::B13),
            14 => Some(TAC_A::B14),
            15 => Some(TAC_A::B15),
            16 => Some(TAC_A::B16),
            17 => Some(TAC_A::B17),
            18 => Some(TAC_A::B18),
            19 => Some(TAC_A::B19),
            20 => Some(TAC_A::B20),
            21 => Some(TAC_A::B21),
            22 => Some(TAC_A::B22),
            23 => Some(TAC_A::B23),
            24 => Some(TAC_A::B24),
            25 => Some(TAC_A::B25),
            26 => Some(TAC_A::B26),
            27 => Some(TAC_A::B27),
            28 => Some(TAC_A::B28),
            29 => Some(TAC_A::B29),
            30 => Some(TAC_A::B30),
            31 => Some(TAC_A::B31),
            32 => Some(TAC_A::B32),
            33 => Some(TAC_A::B33),
            34 => Some(TAC_A::B34),
            35 => Some(TAC_A::B35),
            36 => Some(TAC_A::B36),
            37 => Some(TAC_A::B37),
            38 => Some(TAC_A::B38),
            39 => Some(TAC_A::B39),
            40 => Some(TAC_A::B40),
            41 => Some(TAC_A::B41),
            42 => Some(TAC_A::B42),
            43 => Some(TAC_A::B43),
            44 => Some(TAC_A::B44),
            45 => Some(TAC_A::B45),
            46 => Some(TAC_A::B46),
            47 => Some(TAC_A::B47),
            48 => Some(TAC_A::B48),
            49 => Some(TAC_A::B49),
            50 => Some(TAC_A::B50),
            51 => Some(TAC_A::B51),
            52 => Some(TAC_A::B52),
            53 => Some(TAC_A::B53),
            54 => Some(TAC_A::B54),
            55 => Some(TAC_A::B55),
            56 => Some(TAC_A::B56),
            57 => Some(TAC_A::B57),
            58 => Some(TAC_A::B58),
            59 => Some(TAC_A::B59),
            60 => Some(TAC_A::B60),
            61 => Some(TAC_A::B61),
            62 => Some(TAC_A::B62),
            63 => Some(TAC_A::B63),
            64 => Some(TAC_A::B64),
            65 => Some(TAC_A::B65),
            66 => Some(TAC_A::B66),
            67 => Some(TAC_A::B67),
            68 => Some(TAC_A::B68),
            69 => Some(TAC_A::B69),
            70 => Some(TAC_A::B70),
            71 => Some(TAC_A::B71),
            72 => Some(TAC_A::B72),
            73 => Some(TAC_A::B73),
            74 => Some(TAC_A::B74),
            75 => Some(TAC_A::B75),
            76 => Some(TAC_A::B76),
            77 => Some(TAC_A::B77),
            78 => Some(TAC_A::B78),
            79 => Some(TAC_A::B79),
            80 => Some(TAC_A::B80),
            81 => Some(TAC_A::B81),
            82 => Some(TAC_A::B82),
            83 => Some(TAC_A::B83),
            84 => Some(TAC_A::B84),
            85 => Some(TAC_A::B85),
            86 => Some(TAC_A::B86),
            87 => Some(TAC_A::B87),
            88 => Some(TAC_A::B88),
            89 => Some(TAC_A::B89),
            90 => Some(TAC_A::B90),
            91 => Some(TAC_A::B91),
            92 => Some(TAC_A::B92),
            93 => Some(TAC_A::B93),
            94 => Some(TAC_A::B94),
            95 => Some(TAC_A::B95),
            96 => Some(TAC_A::B96),
            97 => Some(TAC_A::B97),
            98 => Some(TAC_A::B98),
            99 => Some(TAC_A::B99),
            100 => Some(TAC_A::B100),
            101 => Some(TAC_A::B101),
            102 => Some(TAC_A::B102),
            103 => Some(TAC_A::B103),
            104 => Some(TAC_A::B104),
            105 => Some(TAC_A::B105),
            106 => Some(TAC_A::B106),
            107 => Some(TAC_A::B107),
            108 => Some(TAC_A::B108),
            109 => Some(TAC_A::B109),
            110 => Some(TAC_A::B110),
            111 => Some(TAC_A::B111),
            112 => Some(TAC_A::B112),
            113 => Some(TAC_A::B113),
            114 => Some(TAC_A::B114),
            115 => Some(TAC_A::B115),
            116 => Some(TAC_A::B116),
            117 => Some(TAC_A::B117),
            118 => Some(TAC_A::B118),
            119 => Some(TAC_A::B119),
            120 => Some(TAC_A::B120),
            121 => Some(TAC_A::B121),
            122 => Some(TAC_A::B122),
            123 => Some(TAC_A::B123),
            124 => Some(TAC_A::B124),
            125 => Some(TAC_A::B125),
            126 => Some(TAC_A::B126),
            127 => Some(TAC_A::B127),
            128 => Some(TAC_A::B128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B0`"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TAC_A::B0
    }
    #[doc = "Checks if the value of the field is `B1`"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TAC_A::B1
    }
    #[doc = "Checks if the value of the field is `B2`"]
    #[inline(always)]
    pub fn is_b2(&self) -> bool {
        *self == TAC_A::B2
    }
    #[doc = "Checks if the value of the field is `B3`"]
    #[inline(always)]
    pub fn is_b3(&self) -> bool {
        *self == TAC_A::B3
    }
    #[doc = "Checks if the value of the field is `B4`"]
    #[inline(always)]
    pub fn is_b4(&self) -> bool {
        *self == TAC_A::B4
    }
    #[doc = "Checks if the value of the field is `B5`"]
    #[inline(always)]
    pub fn is_b5(&self) -> bool {
        *self == TAC_A::B5
    }
    #[doc = "Checks if the value of the field is `B6`"]
    #[inline(always)]
    pub fn is_b6(&self) -> bool {
        *self == TAC_A::B6
    }
    #[doc = "Checks if the value of the field is `B7`"]
    #[inline(always)]
    pub fn is_b7(&self) -> bool {
        *self == TAC_A::B7
    }
    #[doc = "Checks if the value of the field is `B8`"]
    #[inline(always)]
    pub fn is_b8(&self) -> bool {
        *self == TAC_A::B8
    }
    #[doc = "Checks if the value of the field is `B9`"]
    #[inline(always)]
    pub fn is_b9(&self) -> bool {
        *self == TAC_A::B9
    }
    #[doc = "Checks if the value of the field is `B10`"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == TAC_A::B10
    }
    #[doc = "Checks if the value of the field is `B11`"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == TAC_A::B11
    }
    #[doc = "Checks if the value of the field is `B12`"]
    #[inline(always)]
    pub fn is_b12(&self) -> bool {
        *self == TAC_A::B12
    }
    #[doc = "Checks if the value of the field is `B13`"]
    #[inline(always)]
    pub fn is_b13(&self) -> bool {
        *self == TAC_A::B13
    }
    #[doc = "Checks if the value of the field is `B14`"]
    #[inline(always)]
    pub fn is_b14(&self) -> bool {
        *self == TAC_A::B14
    }
    #[doc = "Checks if the value of the field is `B15`"]
    #[inline(always)]
    pub fn is_b15(&self) -> bool {
        *self == TAC_A::B15
    }
    #[doc = "Checks if the value of the field is `B16`"]
    #[inline(always)]
    pub fn is_b16(&self) -> bool {
        *self == TAC_A::B16
    }
    #[doc = "Checks if the value of the field is `B17`"]
    #[inline(always)]
    pub fn is_b17(&self) -> bool {
        *self == TAC_A::B17
    }
    #[doc = "Checks if the value of the field is `B18`"]
    #[inline(always)]
    pub fn is_b18(&self) -> bool {
        *self == TAC_A::B18
    }
    #[doc = "Checks if the value of the field is `B19`"]
    #[inline(always)]
    pub fn is_b19(&self) -> bool {
        *self == TAC_A::B19
    }
    #[doc = "Checks if the value of the field is `B20`"]
    #[inline(always)]
    pub fn is_b20(&self) -> bool {
        *self == TAC_A::B20
    }
    #[doc = "Checks if the value of the field is `B21`"]
    #[inline(always)]
    pub fn is_b21(&self) -> bool {
        *self == TAC_A::B21
    }
    #[doc = "Checks if the value of the field is `B22`"]
    #[inline(always)]
    pub fn is_b22(&self) -> bool {
        *self == TAC_A::B22
    }
    #[doc = "Checks if the value of the field is `B23`"]
    #[inline(always)]
    pub fn is_b23(&self) -> bool {
        *self == TAC_A::B23
    }
    #[doc = "Checks if the value of the field is `B24`"]
    #[inline(always)]
    pub fn is_b24(&self) -> bool {
        *self == TAC_A::B24
    }
    #[doc = "Checks if the value of the field is `B25`"]
    #[inline(always)]
    pub fn is_b25(&self) -> bool {
        *self == TAC_A::B25
    }
    #[doc = "Checks if the value of the field is `B26`"]
    #[inline(always)]
    pub fn is_b26(&self) -> bool {
        *self == TAC_A::B26
    }
    #[doc = "Checks if the value of the field is `B27`"]
    #[inline(always)]
    pub fn is_b27(&self) -> bool {
        *self == TAC_A::B27
    }
    #[doc = "Checks if the value of the field is `B28`"]
    #[inline(always)]
    pub fn is_b28(&self) -> bool {
        *self == TAC_A::B28
    }
    #[doc = "Checks if the value of the field is `B29`"]
    #[inline(always)]
    pub fn is_b29(&self) -> bool {
        *self == TAC_A::B29
    }
    #[doc = "Checks if the value of the field is `B30`"]
    #[inline(always)]
    pub fn is_b30(&self) -> bool {
        *self == TAC_A::B30
    }
    #[doc = "Checks if the value of the field is `B31`"]
    #[inline(always)]
    pub fn is_b31(&self) -> bool {
        *self == TAC_A::B31
    }
    #[doc = "Checks if the value of the field is `B32`"]
    #[inline(always)]
    pub fn is_b32(&self) -> bool {
        *self == TAC_A::B32
    }
    #[doc = "Checks if the value of the field is `B33`"]
    #[inline(always)]
    pub fn is_b33(&self) -> bool {
        *self == TAC_A::B33
    }
    #[doc = "Checks if the value of the field is `B34`"]
    #[inline(always)]
    pub fn is_b34(&self) -> bool {
        *self == TAC_A::B34
    }
    #[doc = "Checks if the value of the field is `B35`"]
    #[inline(always)]
    pub fn is_b35(&self) -> bool {
        *self == TAC_A::B35
    }
    #[doc = "Checks if the value of the field is `B36`"]
    #[inline(always)]
    pub fn is_b36(&self) -> bool {
        *self == TAC_A::B36
    }
    #[doc = "Checks if the value of the field is `B37`"]
    #[inline(always)]
    pub fn is_b37(&self) -> bool {
        *self == TAC_A::B37
    }
    #[doc = "Checks if the value of the field is `B38`"]
    #[inline(always)]
    pub fn is_b38(&self) -> bool {
        *self == TAC_A::B38
    }
    #[doc = "Checks if the value of the field is `B39`"]
    #[inline(always)]
    pub fn is_b39(&self) -> bool {
        *self == TAC_A::B39
    }
    #[doc = "Checks if the value of the field is `B40`"]
    #[inline(always)]
    pub fn is_b40(&self) -> bool {
        *self == TAC_A::B40
    }
    #[doc = "Checks if the value of the field is `B41`"]
    #[inline(always)]
    pub fn is_b41(&self) -> bool {
        *self == TAC_A::B41
    }
    #[doc = "Checks if the value of the field is `B42`"]
    #[inline(always)]
    pub fn is_b42(&self) -> bool {
        *self == TAC_A::B42
    }
    #[doc = "Checks if the value of the field is `B43`"]
    #[inline(always)]
    pub fn is_b43(&self) -> bool {
        *self == TAC_A::B43
    }
    #[doc = "Checks if the value of the field is `B44`"]
    #[inline(always)]
    pub fn is_b44(&self) -> bool {
        *self == TAC_A::B44
    }
    #[doc = "Checks if the value of the field is `B45`"]
    #[inline(always)]
    pub fn is_b45(&self) -> bool {
        *self == TAC_A::B45
    }
    #[doc = "Checks if the value of the field is `B46`"]
    #[inline(always)]
    pub fn is_b46(&self) -> bool {
        *self == TAC_A::B46
    }
    #[doc = "Checks if the value of the field is `B47`"]
    #[inline(always)]
    pub fn is_b47(&self) -> bool {
        *self == TAC_A::B47
    }
    #[doc = "Checks if the value of the field is `B48`"]
    #[inline(always)]
    pub fn is_b48(&self) -> bool {
        *self == TAC_A::B48
    }
    #[doc = "Checks if the value of the field is `B49`"]
    #[inline(always)]
    pub fn is_b49(&self) -> bool {
        *self == TAC_A::B49
    }
    #[doc = "Checks if the value of the field is `B50`"]
    #[inline(always)]
    pub fn is_b50(&self) -> bool {
        *self == TAC_A::B50
    }
    #[doc = "Checks if the value of the field is `B51`"]
    #[inline(always)]
    pub fn is_b51(&self) -> bool {
        *self == TAC_A::B51
    }
    #[doc = "Checks if the value of the field is `B52`"]
    #[inline(always)]
    pub fn is_b52(&self) -> bool {
        *self == TAC_A::B52
    }
    #[doc = "Checks if the value of the field is `B53`"]
    #[inline(always)]
    pub fn is_b53(&self) -> bool {
        *self == TAC_A::B53
    }
    #[doc = "Checks if the value of the field is `B54`"]
    #[inline(always)]
    pub fn is_b54(&self) -> bool {
        *self == TAC_A::B54
    }
    #[doc = "Checks if the value of the field is `B55`"]
    #[inline(always)]
    pub fn is_b55(&self) -> bool {
        *self == TAC_A::B55
    }
    #[doc = "Checks if the value of the field is `B56`"]
    #[inline(always)]
    pub fn is_b56(&self) -> bool {
        *self == TAC_A::B56
    }
    #[doc = "Checks if the value of the field is `B57`"]
    #[inline(always)]
    pub fn is_b57(&self) -> bool {
        *self == TAC_A::B57
    }
    #[doc = "Checks if the value of the field is `B58`"]
    #[inline(always)]
    pub fn is_b58(&self) -> bool {
        *self == TAC_A::B58
    }
    #[doc = "Checks if the value of the field is `B59`"]
    #[inline(always)]
    pub fn is_b59(&self) -> bool {
        *self == TAC_A::B59
    }
    #[doc = "Checks if the value of the field is `B60`"]
    #[inline(always)]
    pub fn is_b60(&self) -> bool {
        *self == TAC_A::B60
    }
    #[doc = "Checks if the value of the field is `B61`"]
    #[inline(always)]
    pub fn is_b61(&self) -> bool {
        *self == TAC_A::B61
    }
    #[doc = "Checks if the value of the field is `B62`"]
    #[inline(always)]
    pub fn is_b62(&self) -> bool {
        *self == TAC_A::B62
    }
    #[doc = "Checks if the value of the field is `B63`"]
    #[inline(always)]
    pub fn is_b63(&self) -> bool {
        *self == TAC_A::B63
    }
    #[doc = "Checks if the value of the field is `B64`"]
    #[inline(always)]
    pub fn is_b64(&self) -> bool {
        *self == TAC_A::B64
    }
    #[doc = "Checks if the value of the field is `B65`"]
    #[inline(always)]
    pub fn is_b65(&self) -> bool {
        *self == TAC_A::B65
    }
    #[doc = "Checks if the value of the field is `B66`"]
    #[inline(always)]
    pub fn is_b66(&self) -> bool {
        *self == TAC_A::B66
    }
    #[doc = "Checks if the value of the field is `B67`"]
    #[inline(always)]
    pub fn is_b67(&self) -> bool {
        *self == TAC_A::B67
    }
    #[doc = "Checks if the value of the field is `B68`"]
    #[inline(always)]
    pub fn is_b68(&self) -> bool {
        *self == TAC_A::B68
    }
    #[doc = "Checks if the value of the field is `B69`"]
    #[inline(always)]
    pub fn is_b69(&self) -> bool {
        *self == TAC_A::B69
    }
    #[doc = "Checks if the value of the field is `B70`"]
    #[inline(always)]
    pub fn is_b70(&self) -> bool {
        *self == TAC_A::B70
    }
    #[doc = "Checks if the value of the field is `B71`"]
    #[inline(always)]
    pub fn is_b71(&self) -> bool {
        *self == TAC_A::B71
    }
    #[doc = "Checks if the value of the field is `B72`"]
    #[inline(always)]
    pub fn is_b72(&self) -> bool {
        *self == TAC_A::B72
    }
    #[doc = "Checks if the value of the field is `B73`"]
    #[inline(always)]
    pub fn is_b73(&self) -> bool {
        *self == TAC_A::B73
    }
    #[doc = "Checks if the value of the field is `B74`"]
    #[inline(always)]
    pub fn is_b74(&self) -> bool {
        *self == TAC_A::B74
    }
    #[doc = "Checks if the value of the field is `B75`"]
    #[inline(always)]
    pub fn is_b75(&self) -> bool {
        *self == TAC_A::B75
    }
    #[doc = "Checks if the value of the field is `B76`"]
    #[inline(always)]
    pub fn is_b76(&self) -> bool {
        *self == TAC_A::B76
    }
    #[doc = "Checks if the value of the field is `B77`"]
    #[inline(always)]
    pub fn is_b77(&self) -> bool {
        *self == TAC_A::B77
    }
    #[doc = "Checks if the value of the field is `B78`"]
    #[inline(always)]
    pub fn is_b78(&self) -> bool {
        *self == TAC_A::B78
    }
    #[doc = "Checks if the value of the field is `B79`"]
    #[inline(always)]
    pub fn is_b79(&self) -> bool {
        *self == TAC_A::B79
    }
    #[doc = "Checks if the value of the field is `B80`"]
    #[inline(always)]
    pub fn is_b80(&self) -> bool {
        *self == TAC_A::B80
    }
    #[doc = "Checks if the value of the field is `B81`"]
    #[inline(always)]
    pub fn is_b81(&self) -> bool {
        *self == TAC_A::B81
    }
    #[doc = "Checks if the value of the field is `B82`"]
    #[inline(always)]
    pub fn is_b82(&self) -> bool {
        *self == TAC_A::B82
    }
    #[doc = "Checks if the value of the field is `B83`"]
    #[inline(always)]
    pub fn is_b83(&self) -> bool {
        *self == TAC_A::B83
    }
    #[doc = "Checks if the value of the field is `B84`"]
    #[inline(always)]
    pub fn is_b84(&self) -> bool {
        *self == TAC_A::B84
    }
    #[doc = "Checks if the value of the field is `B85`"]
    #[inline(always)]
    pub fn is_b85(&self) -> bool {
        *self == TAC_A::B85
    }
    #[doc = "Checks if the value of the field is `B86`"]
    #[inline(always)]
    pub fn is_b86(&self) -> bool {
        *self == TAC_A::B86
    }
    #[doc = "Checks if the value of the field is `B87`"]
    #[inline(always)]
    pub fn is_b87(&self) -> bool {
        *self == TAC_A::B87
    }
    #[doc = "Checks if the value of the field is `B88`"]
    #[inline(always)]
    pub fn is_b88(&self) -> bool {
        *self == TAC_A::B88
    }
    #[doc = "Checks if the value of the field is `B89`"]
    #[inline(always)]
    pub fn is_b89(&self) -> bool {
        *self == TAC_A::B89
    }
    #[doc = "Checks if the value of the field is `B90`"]
    #[inline(always)]
    pub fn is_b90(&self) -> bool {
        *self == TAC_A::B90
    }
    #[doc = "Checks if the value of the field is `B91`"]
    #[inline(always)]
    pub fn is_b91(&self) -> bool {
        *self == TAC_A::B91
    }
    #[doc = "Checks if the value of the field is `B92`"]
    #[inline(always)]
    pub fn is_b92(&self) -> bool {
        *self == TAC_A::B92
    }
    #[doc = "Checks if the value of the field is `B93`"]
    #[inline(always)]
    pub fn is_b93(&self) -> bool {
        *self == TAC_A::B93
    }
    #[doc = "Checks if the value of the field is `B94`"]
    #[inline(always)]
    pub fn is_b94(&self) -> bool {
        *self == TAC_A::B94
    }
    #[doc = "Checks if the value of the field is `B95`"]
    #[inline(always)]
    pub fn is_b95(&self) -> bool {
        *self == TAC_A::B95
    }
    #[doc = "Checks if the value of the field is `B96`"]
    #[inline(always)]
    pub fn is_b96(&self) -> bool {
        *self == TAC_A::B96
    }
    #[doc = "Checks if the value of the field is `B97`"]
    #[inline(always)]
    pub fn is_b97(&self) -> bool {
        *self == TAC_A::B97
    }
    #[doc = "Checks if the value of the field is `B98`"]
    #[inline(always)]
    pub fn is_b98(&self) -> bool {
        *self == TAC_A::B98
    }
    #[doc = "Checks if the value of the field is `B99`"]
    #[inline(always)]
    pub fn is_b99(&self) -> bool {
        *self == TAC_A::B99
    }
    #[doc = "Checks if the value of the field is `B100`"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == TAC_A::B100
    }
    #[doc = "Checks if the value of the field is `B101`"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == TAC_A::B101
    }
    #[doc = "Checks if the value of the field is `B102`"]
    #[inline(always)]
    pub fn is_b102(&self) -> bool {
        *self == TAC_A::B102
    }
    #[doc = "Checks if the value of the field is `B103`"]
    #[inline(always)]
    pub fn is_b103(&self) -> bool {
        *self == TAC_A::B103
    }
    #[doc = "Checks if the value of the field is `B104`"]
    #[inline(always)]
    pub fn is_b104(&self) -> bool {
        *self == TAC_A::B104
    }
    #[doc = "Checks if the value of the field is `B105`"]
    #[inline(always)]
    pub fn is_b105(&self) -> bool {
        *self == TAC_A::B105
    }
    #[doc = "Checks if the value of the field is `B106`"]
    #[inline(always)]
    pub fn is_b106(&self) -> bool {
        *self == TAC_A::B106
    }
    #[doc = "Checks if the value of the field is `B107`"]
    #[inline(always)]
    pub fn is_b107(&self) -> bool {
        *self == TAC_A::B107
    }
    #[doc = "Checks if the value of the field is `B108`"]
    #[inline(always)]
    pub fn is_b108(&self) -> bool {
        *self == TAC_A::B108
    }
    #[doc = "Checks if the value of the field is `B109`"]
    #[inline(always)]
    pub fn is_b109(&self) -> bool {
        *self == TAC_A::B109
    }
    #[doc = "Checks if the value of the field is `B110`"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == TAC_A::B110
    }
    #[doc = "Checks if the value of the field is `B111`"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == TAC_A::B111
    }
    #[doc = "Checks if the value of the field is `B112`"]
    #[inline(always)]
    pub fn is_b112(&self) -> bool {
        *self == TAC_A::B112
    }
    #[doc = "Checks if the value of the field is `B113`"]
    #[inline(always)]
    pub fn is_b113(&self) -> bool {
        *self == TAC_A::B113
    }
    #[doc = "Checks if the value of the field is `B114`"]
    #[inline(always)]
    pub fn is_b114(&self) -> bool {
        *self == TAC_A::B114
    }
    #[doc = "Checks if the value of the field is `B115`"]
    #[inline(always)]
    pub fn is_b115(&self) -> bool {
        *self == TAC_A::B115
    }
    #[doc = "Checks if the value of the field is `B116`"]
    #[inline(always)]
    pub fn is_b116(&self) -> bool {
        *self == TAC_A::B116
    }
    #[doc = "Checks if the value of the field is `B117`"]
    #[inline(always)]
    pub fn is_b117(&self) -> bool {
        *self == TAC_A::B117
    }
    #[doc = "Checks if the value of the field is `B118`"]
    #[inline(always)]
    pub fn is_b118(&self) -> bool {
        *self == TAC_A::B118
    }
    #[doc = "Checks if the value of the field is `B119`"]
    #[inline(always)]
    pub fn is_b119(&self) -> bool {
        *self == TAC_A::B119
    }
    #[doc = "Checks if the value of the field is `B120`"]
    #[inline(always)]
    pub fn is_b120(&self) -> bool {
        *self == TAC_A::B120
    }
    #[doc = "Checks if the value of the field is `B121`"]
    #[inline(always)]
    pub fn is_b121(&self) -> bool {
        *self == TAC_A::B121
    }
    #[doc = "Checks if the value of the field is `B122`"]
    #[inline(always)]
    pub fn is_b122(&self) -> bool {
        *self == TAC_A::B122
    }
    #[doc = "Checks if the value of the field is `B123`"]
    #[inline(always)]
    pub fn is_b123(&self) -> bool {
        *self == TAC_A::B123
    }
    #[doc = "Checks if the value of the field is `B124`"]
    #[inline(always)]
    pub fn is_b124(&self) -> bool {
        *self == TAC_A::B124
    }
    #[doc = "Checks if the value of the field is `B125`"]
    #[inline(always)]
    pub fn is_b125(&self) -> bool {
        *self == TAC_A::B125
    }
    #[doc = "Checks if the value of the field is `B126`"]
    #[inline(always)]
    pub fn is_b126(&self) -> bool {
        *self == TAC_A::B126
    }
    #[doc = "Checks if the value of the field is `B127`"]
    #[inline(always)]
    pub fn is_b127(&self) -> bool {
        *self == TAC_A::B127
    }
    #[doc = "Checks if the value of the field is `B128`"]
    #[inline(always)]
    pub fn is_b128(&self) -> bool {
        *self == TAC_A::B128
    }
}
impl R {
    #[doc = "Bits 0:7 - TX FIFO Available Space Counter"]
    #[inline(always)]
    pub fn tac(&self) -> TAC_R {
        TAC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR Transmit FIFO Available Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_tac](index.html) module"]
pub struct CIR_TAC_SPEC;
impl crate::RegisterSpec for CIR_TAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_tac::R](R) reader structure"]
impl crate::Readable for CIR_TAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_tac::W](W) writer structure"]
impl crate::Writable for CIR_TAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_tac to value 0x80"]
impl crate::Resettable for CIR_TAC_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
