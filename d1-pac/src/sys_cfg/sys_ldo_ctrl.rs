#[doc = "Register `sys_ldo_ctrl` reader"]
pub struct R(crate::R<SYS_LDO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_LDO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_LDO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_LDO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sys_ldo_ctrl` writer"]
pub struct W(crate::W<SYS_LDO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_LDO_CTRL_SPEC>;
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
impl From<crate::W<SYS_LDO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_LDO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ldoa_trim` reader - "]
pub type LDOA_TRIM_R = crate::FieldReader<u8, LDOA_TRIM_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LDOA_TRIM_A {
    #[doc = "0: `0`"]
    O1_593 = 0,
    #[doc = "1: `1`"]
    O1_607 = 1,
    #[doc = "2: `10`"]
    O1_627 = 2,
    #[doc = "3: `11`"]
    O1_64 = 3,
    #[doc = "4: `100`"]
    O1_653 = 4,
    #[doc = "5: `101`"]
    O1_667 = 5,
    #[doc = "6: `110`"]
    O1_680 = 6,
    #[doc = "7: `111`"]
    O1_693 = 7,
    #[doc = "8: `1000`"]
    O1_707 = 8,
    #[doc = "9: `1001`"]
    O1_720 = 9,
    #[doc = "10: `1010`"]
    O1_733 = 10,
    #[doc = "11: `1011`"]
    O1_747 = 11,
    #[doc = "12: `1100`"]
    O1_76 = 12,
    #[doc = "13: `1101`"]
    O1_773 = 13,
    #[doc = "14: `1110`"]
    O1_787 = 14,
    #[doc = "15: `1111`"]
    O1_8 = 15,
    #[doc = "16: `10000`"]
    O1_813 = 16,
    #[doc = "17: `10001`"]
    O1_827 = 17,
    #[doc = "18: `10010`"]
    O1_84 = 18,
    #[doc = "19: `10011`"]
    O1_853 = 19,
    #[doc = "20: `10100`"]
    O1_867 = 20,
    #[doc = "21: `10101`"]
    O1_88 = 21,
    #[doc = "22: `10110`"]
    O1_893 = 22,
    #[doc = "23: `10111`"]
    O1_907 = 23,
    #[doc = "24: `11000`"]
    O1_92 = 24,
    #[doc = "25: `11001`"]
    O1_933 = 25,
    #[doc = "26: `11010`"]
    O1_947 = 26,
    #[doc = "27: `11011`"]
    O1_96 = 27,
    #[doc = "28: `11100`"]
    O1_973 = 28,
    #[doc = "29: `11101`"]
    O1_987 = 29,
    #[doc = "30: `11110`"]
    O2 = 30,
    #[doc = "31: `11111`"]
    O2_013 = 31,
}
impl From<LDOA_TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: LDOA_TRIM_A) -> Self {
        variant as _
    }
}
impl LDOA_TRIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LDOA_TRIM_A> {
        match self.bits {
            0 => Some(LDOA_TRIM_A::O1_593),
            1 => Some(LDOA_TRIM_A::O1_607),
            2 => Some(LDOA_TRIM_A::O1_627),
            3 => Some(LDOA_TRIM_A::O1_64),
            4 => Some(LDOA_TRIM_A::O1_653),
            5 => Some(LDOA_TRIM_A::O1_667),
            6 => Some(LDOA_TRIM_A::O1_680),
            7 => Some(LDOA_TRIM_A::O1_693),
            8 => Some(LDOA_TRIM_A::O1_707),
            9 => Some(LDOA_TRIM_A::O1_720),
            10 => Some(LDOA_TRIM_A::O1_733),
            11 => Some(LDOA_TRIM_A::O1_747),
            12 => Some(LDOA_TRIM_A::O1_76),
            13 => Some(LDOA_TRIM_A::O1_773),
            14 => Some(LDOA_TRIM_A::O1_787),
            15 => Some(LDOA_TRIM_A::O1_8),
            16 => Some(LDOA_TRIM_A::O1_813),
            17 => Some(LDOA_TRIM_A::O1_827),
            18 => Some(LDOA_TRIM_A::O1_84),
            19 => Some(LDOA_TRIM_A::O1_853),
            20 => Some(LDOA_TRIM_A::O1_867),
            21 => Some(LDOA_TRIM_A::O1_88),
            22 => Some(LDOA_TRIM_A::O1_893),
            23 => Some(LDOA_TRIM_A::O1_907),
            24 => Some(LDOA_TRIM_A::O1_92),
            25 => Some(LDOA_TRIM_A::O1_933),
            26 => Some(LDOA_TRIM_A::O1_947),
            27 => Some(LDOA_TRIM_A::O1_96),
            28 => Some(LDOA_TRIM_A::O1_973),
            29 => Some(LDOA_TRIM_A::O1_987),
            30 => Some(LDOA_TRIM_A::O2),
            31 => Some(LDOA_TRIM_A::O2_013),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `O1_593`"]
    #[inline(always)]
    pub fn is_o1_593(&self) -> bool {
        *self == LDOA_TRIM_A::O1_593
    }
    #[doc = "Checks if the value of the field is `O1_607`"]
    #[inline(always)]
    pub fn is_o1_607(&self) -> bool {
        *self == LDOA_TRIM_A::O1_607
    }
    #[doc = "Checks if the value of the field is `O1_627`"]
    #[inline(always)]
    pub fn is_o1_627(&self) -> bool {
        *self == LDOA_TRIM_A::O1_627
    }
    #[doc = "Checks if the value of the field is `O1_64`"]
    #[inline(always)]
    pub fn is_o1_64(&self) -> bool {
        *self == LDOA_TRIM_A::O1_64
    }
    #[doc = "Checks if the value of the field is `O1_653`"]
    #[inline(always)]
    pub fn is_o1_653(&self) -> bool {
        *self == LDOA_TRIM_A::O1_653
    }
    #[doc = "Checks if the value of the field is `O1_667`"]
    #[inline(always)]
    pub fn is_o1_667(&self) -> bool {
        *self == LDOA_TRIM_A::O1_667
    }
    #[doc = "Checks if the value of the field is `O1_680`"]
    #[inline(always)]
    pub fn is_o1_680(&self) -> bool {
        *self == LDOA_TRIM_A::O1_680
    }
    #[doc = "Checks if the value of the field is `O1_693`"]
    #[inline(always)]
    pub fn is_o1_693(&self) -> bool {
        *self == LDOA_TRIM_A::O1_693
    }
    #[doc = "Checks if the value of the field is `O1_707`"]
    #[inline(always)]
    pub fn is_o1_707(&self) -> bool {
        *self == LDOA_TRIM_A::O1_707
    }
    #[doc = "Checks if the value of the field is `O1_720`"]
    #[inline(always)]
    pub fn is_o1_720(&self) -> bool {
        *self == LDOA_TRIM_A::O1_720
    }
    #[doc = "Checks if the value of the field is `O1_733`"]
    #[inline(always)]
    pub fn is_o1_733(&self) -> bool {
        *self == LDOA_TRIM_A::O1_733
    }
    #[doc = "Checks if the value of the field is `O1_747`"]
    #[inline(always)]
    pub fn is_o1_747(&self) -> bool {
        *self == LDOA_TRIM_A::O1_747
    }
    #[doc = "Checks if the value of the field is `O1_76`"]
    #[inline(always)]
    pub fn is_o1_76(&self) -> bool {
        *self == LDOA_TRIM_A::O1_76
    }
    #[doc = "Checks if the value of the field is `O1_773`"]
    #[inline(always)]
    pub fn is_o1_773(&self) -> bool {
        *self == LDOA_TRIM_A::O1_773
    }
    #[doc = "Checks if the value of the field is `O1_787`"]
    #[inline(always)]
    pub fn is_o1_787(&self) -> bool {
        *self == LDOA_TRIM_A::O1_787
    }
    #[doc = "Checks if the value of the field is `O1_8`"]
    #[inline(always)]
    pub fn is_o1_8(&self) -> bool {
        *self == LDOA_TRIM_A::O1_8
    }
    #[doc = "Checks if the value of the field is `O1_813`"]
    #[inline(always)]
    pub fn is_o1_813(&self) -> bool {
        *self == LDOA_TRIM_A::O1_813
    }
    #[doc = "Checks if the value of the field is `O1_827`"]
    #[inline(always)]
    pub fn is_o1_827(&self) -> bool {
        *self == LDOA_TRIM_A::O1_827
    }
    #[doc = "Checks if the value of the field is `O1_84`"]
    #[inline(always)]
    pub fn is_o1_84(&self) -> bool {
        *self == LDOA_TRIM_A::O1_84
    }
    #[doc = "Checks if the value of the field is `O1_853`"]
    #[inline(always)]
    pub fn is_o1_853(&self) -> bool {
        *self == LDOA_TRIM_A::O1_853
    }
    #[doc = "Checks if the value of the field is `O1_867`"]
    #[inline(always)]
    pub fn is_o1_867(&self) -> bool {
        *self == LDOA_TRIM_A::O1_867
    }
    #[doc = "Checks if the value of the field is `O1_88`"]
    #[inline(always)]
    pub fn is_o1_88(&self) -> bool {
        *self == LDOA_TRIM_A::O1_88
    }
    #[doc = "Checks if the value of the field is `O1_893`"]
    #[inline(always)]
    pub fn is_o1_893(&self) -> bool {
        *self == LDOA_TRIM_A::O1_893
    }
    #[doc = "Checks if the value of the field is `O1_907`"]
    #[inline(always)]
    pub fn is_o1_907(&self) -> bool {
        *self == LDOA_TRIM_A::O1_907
    }
    #[doc = "Checks if the value of the field is `O1_92`"]
    #[inline(always)]
    pub fn is_o1_92(&self) -> bool {
        *self == LDOA_TRIM_A::O1_92
    }
    #[doc = "Checks if the value of the field is `O1_933`"]
    #[inline(always)]
    pub fn is_o1_933(&self) -> bool {
        *self == LDOA_TRIM_A::O1_933
    }
    #[doc = "Checks if the value of the field is `O1_947`"]
    #[inline(always)]
    pub fn is_o1_947(&self) -> bool {
        *self == LDOA_TRIM_A::O1_947
    }
    #[doc = "Checks if the value of the field is `O1_96`"]
    #[inline(always)]
    pub fn is_o1_96(&self) -> bool {
        *self == LDOA_TRIM_A::O1_96
    }
    #[doc = "Checks if the value of the field is `O1_973`"]
    #[inline(always)]
    pub fn is_o1_973(&self) -> bool {
        *self == LDOA_TRIM_A::O1_973
    }
    #[doc = "Checks if the value of the field is `O1_987`"]
    #[inline(always)]
    pub fn is_o1_987(&self) -> bool {
        *self == LDOA_TRIM_A::O1_987
    }
    #[doc = "Checks if the value of the field is `O2`"]
    #[inline(always)]
    pub fn is_o2(&self) -> bool {
        *self == LDOA_TRIM_A::O2
    }
    #[doc = "Checks if the value of the field is `O2_013`"]
    #[inline(always)]
    pub fn is_o2_013(&self) -> bool {
        *self == LDOA_TRIM_A::O2_013
    }
}
#[doc = "Field `ldoa_trim` writer - "]
pub type LDOA_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYS_LDO_CTRL_SPEC, u8, LDOA_TRIM_A, 8, O>;
impl<'a, const O: u8> LDOA_TRIM_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn o1_593(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_593)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn o1_607(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_607)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn o1_627(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_627)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn o1_64(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_64)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn o1_653(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_653)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn o1_667(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_667)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn o1_680(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_680)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn o1_693(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_693)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn o1_707(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_707)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn o1_720(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_720)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn o1_733(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_733)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn o1_747(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_747)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn o1_76(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_76)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn o1_773(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_773)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn o1_787(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_787)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn o1_8(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_8)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn o1_813(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_813)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn o1_827(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_827)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn o1_84(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_84)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn o1_853(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_853)
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn o1_867(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_867)
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn o1_88(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_88)
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn o1_893(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_893)
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn o1_907(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_907)
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn o1_92(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_92)
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn o1_933(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_933)
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn o1_947(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_947)
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn o1_96(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_96)
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn o1_973(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_973)
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn o1_987(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O1_987)
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn o2(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O2)
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn o2_013(self) -> &'a mut W {
        self.variant(LDOA_TRIM_A::O2_013)
    }
}
#[doc = "Field `ldob_trim` reader - "]
pub type LDOB_TRIM_R = crate::FieldReader<u8, LDOB_TRIM_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LDOB_TRIM_A {
    #[doc = "0: `0`"]
    O1_167 = 0,
    #[doc = "1: `1`"]
    O1_18 = 1,
    #[doc = "2: `10`"]
    O1_193 = 2,
    #[doc = "3: `11`"]
    O1_207 = 3,
    #[doc = "4: `100`"]
    O1_22 = 4,
    #[doc = "5: `101`"]
    O1_233 = 5,
    #[doc = "6: `110`"]
    O1_247 = 6,
    #[doc = "7: `111`"]
    O1_260 = 7,
    #[doc = "8: `1000`"]
    O1_273 = 8,
    #[doc = "9: `1001`"]
    O1_287 = 9,
    #[doc = "10: `1010`"]
    O1_3 = 10,
    #[doc = "11: `1011`"]
    O1_313 = 11,
    #[doc = "12: `1100`"]
    O1_327 = 12,
    #[doc = "13: `1101`"]
    O1_340 = 13,
    #[doc = "14: `1110`"]
    O1_353 = 14,
    #[doc = "15: `1111`"]
    O1_367 = 15,
    #[doc = "16: `10000`"]
    O1_38 = 16,
    #[doc = "17: `10001`"]
    O1_393 = 17,
    #[doc = "18: `10010`"]
    O1_407 = 18,
    #[doc = "19: `10011`"]
    O1_42 = 19,
    #[doc = "20: `10100`"]
    O1_433 = 20,
    #[doc = "21: `10101`"]
    O1_447 = 21,
    #[doc = "22: `10110`"]
    O1_46 = 22,
    #[doc = "23: `10111`"]
    O1_473 = 23,
    #[doc = "24: `11000`"]
    O1_487 = 24,
    #[doc = "25: `11001`"]
    O1_5 = 25,
    #[doc = "26: `11010`"]
    O1_513 = 26,
    #[doc = "27: `11011`"]
    O1_527 = 27,
    #[doc = "28: `11100`"]
    O1_54 = 28,
    #[doc = "29: `11101`"]
    O1_553 = 29,
    #[doc = "30: `11110`"]
    O1_567 = 30,
    #[doc = "31: `11111`"]
    O1_58 = 31,
    #[doc = "32: `100000`"]
    O1_593 = 32,
    #[doc = "33: `100001`"]
    O1_607 = 33,
    #[doc = "34: `100010`"]
    O1_627 = 34,
    #[doc = "35: `100011`"]
    O1_64 = 35,
    #[doc = "36: `100100`"]
    O1_653 = 36,
    #[doc = "37: `100101`"]
    O1_667 = 37,
    #[doc = "38: `100110`"]
    O1_680 = 38,
    #[doc = "39: `100111`"]
    O1_693 = 39,
    #[doc = "40: `101000`"]
    O1_707 = 40,
    #[doc = "41: `101001`"]
    O1_720 = 41,
    #[doc = "42: `101010`"]
    O1_733 = 42,
    #[doc = "43: `101011`"]
    O1_747 = 43,
    #[doc = "44: `101100`"]
    O1_76 = 44,
    #[doc = "45: `101101`"]
    O1_773 = 45,
    #[doc = "46: `101110`"]
    O1_787 = 46,
    #[doc = "47: `101111`"]
    O1_8 = 47,
    #[doc = "48: `110000`"]
    O1_813 = 48,
    #[doc = "49: `110001`"]
    O1_827 = 49,
    #[doc = "50: `110010`"]
    O1_84 = 50,
    #[doc = "51: `110011`"]
    O1_853 = 51,
    #[doc = "52: `110100`"]
    O1_867 = 52,
    #[doc = "53: `110101`"]
    O1_88 = 53,
    #[doc = "54: `110110`"]
    O1_893 = 54,
    #[doc = "55: `110111`"]
    O1_907 = 55,
    #[doc = "56: `111000`"]
    O1_92 = 56,
    #[doc = "57: `111001`"]
    O1_933 = 57,
    #[doc = "58: `111010`"]
    O1_947 = 58,
    #[doc = "59: `111011`"]
    O1_96 = 59,
    #[doc = "60: `111100`"]
    O1_973 = 60,
    #[doc = "61: `111101`"]
    O1_987 = 61,
    #[doc = "62: `111110`"]
    O2 = 62,
    #[doc = "63: `111111`"]
    O2_013 = 63,
}
impl From<LDOB_TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: LDOB_TRIM_A) -> Self {
        variant as _
    }
}
impl LDOB_TRIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LDOB_TRIM_A> {
        match self.bits {
            0 => Some(LDOB_TRIM_A::O1_167),
            1 => Some(LDOB_TRIM_A::O1_18),
            2 => Some(LDOB_TRIM_A::O1_193),
            3 => Some(LDOB_TRIM_A::O1_207),
            4 => Some(LDOB_TRIM_A::O1_22),
            5 => Some(LDOB_TRIM_A::O1_233),
            6 => Some(LDOB_TRIM_A::O1_247),
            7 => Some(LDOB_TRIM_A::O1_260),
            8 => Some(LDOB_TRIM_A::O1_273),
            9 => Some(LDOB_TRIM_A::O1_287),
            10 => Some(LDOB_TRIM_A::O1_3),
            11 => Some(LDOB_TRIM_A::O1_313),
            12 => Some(LDOB_TRIM_A::O1_327),
            13 => Some(LDOB_TRIM_A::O1_340),
            14 => Some(LDOB_TRIM_A::O1_353),
            15 => Some(LDOB_TRIM_A::O1_367),
            16 => Some(LDOB_TRIM_A::O1_38),
            17 => Some(LDOB_TRIM_A::O1_393),
            18 => Some(LDOB_TRIM_A::O1_407),
            19 => Some(LDOB_TRIM_A::O1_42),
            20 => Some(LDOB_TRIM_A::O1_433),
            21 => Some(LDOB_TRIM_A::O1_447),
            22 => Some(LDOB_TRIM_A::O1_46),
            23 => Some(LDOB_TRIM_A::O1_473),
            24 => Some(LDOB_TRIM_A::O1_487),
            25 => Some(LDOB_TRIM_A::O1_5),
            26 => Some(LDOB_TRIM_A::O1_513),
            27 => Some(LDOB_TRIM_A::O1_527),
            28 => Some(LDOB_TRIM_A::O1_54),
            29 => Some(LDOB_TRIM_A::O1_553),
            30 => Some(LDOB_TRIM_A::O1_567),
            31 => Some(LDOB_TRIM_A::O1_58),
            32 => Some(LDOB_TRIM_A::O1_593),
            33 => Some(LDOB_TRIM_A::O1_607),
            34 => Some(LDOB_TRIM_A::O1_627),
            35 => Some(LDOB_TRIM_A::O1_64),
            36 => Some(LDOB_TRIM_A::O1_653),
            37 => Some(LDOB_TRIM_A::O1_667),
            38 => Some(LDOB_TRIM_A::O1_680),
            39 => Some(LDOB_TRIM_A::O1_693),
            40 => Some(LDOB_TRIM_A::O1_707),
            41 => Some(LDOB_TRIM_A::O1_720),
            42 => Some(LDOB_TRIM_A::O1_733),
            43 => Some(LDOB_TRIM_A::O1_747),
            44 => Some(LDOB_TRIM_A::O1_76),
            45 => Some(LDOB_TRIM_A::O1_773),
            46 => Some(LDOB_TRIM_A::O1_787),
            47 => Some(LDOB_TRIM_A::O1_8),
            48 => Some(LDOB_TRIM_A::O1_813),
            49 => Some(LDOB_TRIM_A::O1_827),
            50 => Some(LDOB_TRIM_A::O1_84),
            51 => Some(LDOB_TRIM_A::O1_853),
            52 => Some(LDOB_TRIM_A::O1_867),
            53 => Some(LDOB_TRIM_A::O1_88),
            54 => Some(LDOB_TRIM_A::O1_893),
            55 => Some(LDOB_TRIM_A::O1_907),
            56 => Some(LDOB_TRIM_A::O1_92),
            57 => Some(LDOB_TRIM_A::O1_933),
            58 => Some(LDOB_TRIM_A::O1_947),
            59 => Some(LDOB_TRIM_A::O1_96),
            60 => Some(LDOB_TRIM_A::O1_973),
            61 => Some(LDOB_TRIM_A::O1_987),
            62 => Some(LDOB_TRIM_A::O2),
            63 => Some(LDOB_TRIM_A::O2_013),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `O1_167`"]
    #[inline(always)]
    pub fn is_o1_167(&self) -> bool {
        *self == LDOB_TRIM_A::O1_167
    }
    #[doc = "Checks if the value of the field is `O1_18`"]
    #[inline(always)]
    pub fn is_o1_18(&self) -> bool {
        *self == LDOB_TRIM_A::O1_18
    }
    #[doc = "Checks if the value of the field is `O1_193`"]
    #[inline(always)]
    pub fn is_o1_193(&self) -> bool {
        *self == LDOB_TRIM_A::O1_193
    }
    #[doc = "Checks if the value of the field is `O1_207`"]
    #[inline(always)]
    pub fn is_o1_207(&self) -> bool {
        *self == LDOB_TRIM_A::O1_207
    }
    #[doc = "Checks if the value of the field is `O1_22`"]
    #[inline(always)]
    pub fn is_o1_22(&self) -> bool {
        *self == LDOB_TRIM_A::O1_22
    }
    #[doc = "Checks if the value of the field is `O1_233`"]
    #[inline(always)]
    pub fn is_o1_233(&self) -> bool {
        *self == LDOB_TRIM_A::O1_233
    }
    #[doc = "Checks if the value of the field is `O1_247`"]
    #[inline(always)]
    pub fn is_o1_247(&self) -> bool {
        *self == LDOB_TRIM_A::O1_247
    }
    #[doc = "Checks if the value of the field is `O1_260`"]
    #[inline(always)]
    pub fn is_o1_260(&self) -> bool {
        *self == LDOB_TRIM_A::O1_260
    }
    #[doc = "Checks if the value of the field is `O1_273`"]
    #[inline(always)]
    pub fn is_o1_273(&self) -> bool {
        *self == LDOB_TRIM_A::O1_273
    }
    #[doc = "Checks if the value of the field is `O1_287`"]
    #[inline(always)]
    pub fn is_o1_287(&self) -> bool {
        *self == LDOB_TRIM_A::O1_287
    }
    #[doc = "Checks if the value of the field is `O1_3`"]
    #[inline(always)]
    pub fn is_o1_3(&self) -> bool {
        *self == LDOB_TRIM_A::O1_3
    }
    #[doc = "Checks if the value of the field is `O1_313`"]
    #[inline(always)]
    pub fn is_o1_313(&self) -> bool {
        *self == LDOB_TRIM_A::O1_313
    }
    #[doc = "Checks if the value of the field is `O1_327`"]
    #[inline(always)]
    pub fn is_o1_327(&self) -> bool {
        *self == LDOB_TRIM_A::O1_327
    }
    #[doc = "Checks if the value of the field is `O1_340`"]
    #[inline(always)]
    pub fn is_o1_340(&self) -> bool {
        *self == LDOB_TRIM_A::O1_340
    }
    #[doc = "Checks if the value of the field is `O1_353`"]
    #[inline(always)]
    pub fn is_o1_353(&self) -> bool {
        *self == LDOB_TRIM_A::O1_353
    }
    #[doc = "Checks if the value of the field is `O1_367`"]
    #[inline(always)]
    pub fn is_o1_367(&self) -> bool {
        *self == LDOB_TRIM_A::O1_367
    }
    #[doc = "Checks if the value of the field is `O1_38`"]
    #[inline(always)]
    pub fn is_o1_38(&self) -> bool {
        *self == LDOB_TRIM_A::O1_38
    }
    #[doc = "Checks if the value of the field is `O1_393`"]
    #[inline(always)]
    pub fn is_o1_393(&self) -> bool {
        *self == LDOB_TRIM_A::O1_393
    }
    #[doc = "Checks if the value of the field is `O1_407`"]
    #[inline(always)]
    pub fn is_o1_407(&self) -> bool {
        *self == LDOB_TRIM_A::O1_407
    }
    #[doc = "Checks if the value of the field is `O1_42`"]
    #[inline(always)]
    pub fn is_o1_42(&self) -> bool {
        *self == LDOB_TRIM_A::O1_42
    }
    #[doc = "Checks if the value of the field is `O1_433`"]
    #[inline(always)]
    pub fn is_o1_433(&self) -> bool {
        *self == LDOB_TRIM_A::O1_433
    }
    #[doc = "Checks if the value of the field is `O1_447`"]
    #[inline(always)]
    pub fn is_o1_447(&self) -> bool {
        *self == LDOB_TRIM_A::O1_447
    }
    #[doc = "Checks if the value of the field is `O1_46`"]
    #[inline(always)]
    pub fn is_o1_46(&self) -> bool {
        *self == LDOB_TRIM_A::O1_46
    }
    #[doc = "Checks if the value of the field is `O1_473`"]
    #[inline(always)]
    pub fn is_o1_473(&self) -> bool {
        *self == LDOB_TRIM_A::O1_473
    }
    #[doc = "Checks if the value of the field is `O1_487`"]
    #[inline(always)]
    pub fn is_o1_487(&self) -> bool {
        *self == LDOB_TRIM_A::O1_487
    }
    #[doc = "Checks if the value of the field is `O1_5`"]
    #[inline(always)]
    pub fn is_o1_5(&self) -> bool {
        *self == LDOB_TRIM_A::O1_5
    }
    #[doc = "Checks if the value of the field is `O1_513`"]
    #[inline(always)]
    pub fn is_o1_513(&self) -> bool {
        *self == LDOB_TRIM_A::O1_513
    }
    #[doc = "Checks if the value of the field is `O1_527`"]
    #[inline(always)]
    pub fn is_o1_527(&self) -> bool {
        *self == LDOB_TRIM_A::O1_527
    }
    #[doc = "Checks if the value of the field is `O1_54`"]
    #[inline(always)]
    pub fn is_o1_54(&self) -> bool {
        *self == LDOB_TRIM_A::O1_54
    }
    #[doc = "Checks if the value of the field is `O1_553`"]
    #[inline(always)]
    pub fn is_o1_553(&self) -> bool {
        *self == LDOB_TRIM_A::O1_553
    }
    #[doc = "Checks if the value of the field is `O1_567`"]
    #[inline(always)]
    pub fn is_o1_567(&self) -> bool {
        *self == LDOB_TRIM_A::O1_567
    }
    #[doc = "Checks if the value of the field is `O1_58`"]
    #[inline(always)]
    pub fn is_o1_58(&self) -> bool {
        *self == LDOB_TRIM_A::O1_58
    }
    #[doc = "Checks if the value of the field is `O1_593`"]
    #[inline(always)]
    pub fn is_o1_593(&self) -> bool {
        *self == LDOB_TRIM_A::O1_593
    }
    #[doc = "Checks if the value of the field is `O1_607`"]
    #[inline(always)]
    pub fn is_o1_607(&self) -> bool {
        *self == LDOB_TRIM_A::O1_607
    }
    #[doc = "Checks if the value of the field is `O1_627`"]
    #[inline(always)]
    pub fn is_o1_627(&self) -> bool {
        *self == LDOB_TRIM_A::O1_627
    }
    #[doc = "Checks if the value of the field is `O1_64`"]
    #[inline(always)]
    pub fn is_o1_64(&self) -> bool {
        *self == LDOB_TRIM_A::O1_64
    }
    #[doc = "Checks if the value of the field is `O1_653`"]
    #[inline(always)]
    pub fn is_o1_653(&self) -> bool {
        *self == LDOB_TRIM_A::O1_653
    }
    #[doc = "Checks if the value of the field is `O1_667`"]
    #[inline(always)]
    pub fn is_o1_667(&self) -> bool {
        *self == LDOB_TRIM_A::O1_667
    }
    #[doc = "Checks if the value of the field is `O1_680`"]
    #[inline(always)]
    pub fn is_o1_680(&self) -> bool {
        *self == LDOB_TRIM_A::O1_680
    }
    #[doc = "Checks if the value of the field is `O1_693`"]
    #[inline(always)]
    pub fn is_o1_693(&self) -> bool {
        *self == LDOB_TRIM_A::O1_693
    }
    #[doc = "Checks if the value of the field is `O1_707`"]
    #[inline(always)]
    pub fn is_o1_707(&self) -> bool {
        *self == LDOB_TRIM_A::O1_707
    }
    #[doc = "Checks if the value of the field is `O1_720`"]
    #[inline(always)]
    pub fn is_o1_720(&self) -> bool {
        *self == LDOB_TRIM_A::O1_720
    }
    #[doc = "Checks if the value of the field is `O1_733`"]
    #[inline(always)]
    pub fn is_o1_733(&self) -> bool {
        *self == LDOB_TRIM_A::O1_733
    }
    #[doc = "Checks if the value of the field is `O1_747`"]
    #[inline(always)]
    pub fn is_o1_747(&self) -> bool {
        *self == LDOB_TRIM_A::O1_747
    }
    #[doc = "Checks if the value of the field is `O1_76`"]
    #[inline(always)]
    pub fn is_o1_76(&self) -> bool {
        *self == LDOB_TRIM_A::O1_76
    }
    #[doc = "Checks if the value of the field is `O1_773`"]
    #[inline(always)]
    pub fn is_o1_773(&self) -> bool {
        *self == LDOB_TRIM_A::O1_773
    }
    #[doc = "Checks if the value of the field is `O1_787`"]
    #[inline(always)]
    pub fn is_o1_787(&self) -> bool {
        *self == LDOB_TRIM_A::O1_787
    }
    #[doc = "Checks if the value of the field is `O1_8`"]
    #[inline(always)]
    pub fn is_o1_8(&self) -> bool {
        *self == LDOB_TRIM_A::O1_8
    }
    #[doc = "Checks if the value of the field is `O1_813`"]
    #[inline(always)]
    pub fn is_o1_813(&self) -> bool {
        *self == LDOB_TRIM_A::O1_813
    }
    #[doc = "Checks if the value of the field is `O1_827`"]
    #[inline(always)]
    pub fn is_o1_827(&self) -> bool {
        *self == LDOB_TRIM_A::O1_827
    }
    #[doc = "Checks if the value of the field is `O1_84`"]
    #[inline(always)]
    pub fn is_o1_84(&self) -> bool {
        *self == LDOB_TRIM_A::O1_84
    }
    #[doc = "Checks if the value of the field is `O1_853`"]
    #[inline(always)]
    pub fn is_o1_853(&self) -> bool {
        *self == LDOB_TRIM_A::O1_853
    }
    #[doc = "Checks if the value of the field is `O1_867`"]
    #[inline(always)]
    pub fn is_o1_867(&self) -> bool {
        *self == LDOB_TRIM_A::O1_867
    }
    #[doc = "Checks if the value of the field is `O1_88`"]
    #[inline(always)]
    pub fn is_o1_88(&self) -> bool {
        *self == LDOB_TRIM_A::O1_88
    }
    #[doc = "Checks if the value of the field is `O1_893`"]
    #[inline(always)]
    pub fn is_o1_893(&self) -> bool {
        *self == LDOB_TRIM_A::O1_893
    }
    #[doc = "Checks if the value of the field is `O1_907`"]
    #[inline(always)]
    pub fn is_o1_907(&self) -> bool {
        *self == LDOB_TRIM_A::O1_907
    }
    #[doc = "Checks if the value of the field is `O1_92`"]
    #[inline(always)]
    pub fn is_o1_92(&self) -> bool {
        *self == LDOB_TRIM_A::O1_92
    }
    #[doc = "Checks if the value of the field is `O1_933`"]
    #[inline(always)]
    pub fn is_o1_933(&self) -> bool {
        *self == LDOB_TRIM_A::O1_933
    }
    #[doc = "Checks if the value of the field is `O1_947`"]
    #[inline(always)]
    pub fn is_o1_947(&self) -> bool {
        *self == LDOB_TRIM_A::O1_947
    }
    #[doc = "Checks if the value of the field is `O1_96`"]
    #[inline(always)]
    pub fn is_o1_96(&self) -> bool {
        *self == LDOB_TRIM_A::O1_96
    }
    #[doc = "Checks if the value of the field is `O1_973`"]
    #[inline(always)]
    pub fn is_o1_973(&self) -> bool {
        *self == LDOB_TRIM_A::O1_973
    }
    #[doc = "Checks if the value of the field is `O1_987`"]
    #[inline(always)]
    pub fn is_o1_987(&self) -> bool {
        *self == LDOB_TRIM_A::O1_987
    }
    #[doc = "Checks if the value of the field is `O2`"]
    #[inline(always)]
    pub fn is_o2(&self) -> bool {
        *self == LDOB_TRIM_A::O2
    }
    #[doc = "Checks if the value of the field is `O2_013`"]
    #[inline(always)]
    pub fn is_o2_013(&self) -> bool {
        *self == LDOB_TRIM_A::O2_013
    }
}
#[doc = "Field `ldob_trim` writer - "]
pub type LDOB_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYS_LDO_CTRL_SPEC, u8, LDOB_TRIM_A, 8, O>;
impl<'a, const O: u8> LDOB_TRIM_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn o1_167(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_167)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn o1_18(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_18)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn o1_193(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_193)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn o1_207(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_207)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn o1_22(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_22)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn o1_233(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_233)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn o1_247(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_247)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn o1_260(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_260)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn o1_273(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_273)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn o1_287(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_287)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn o1_3(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_3)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn o1_313(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_313)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn o1_327(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_327)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn o1_340(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_340)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn o1_353(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_353)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn o1_367(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_367)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn o1_38(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_38)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn o1_393(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_393)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn o1_407(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_407)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn o1_42(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_42)
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn o1_433(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_433)
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn o1_447(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_447)
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn o1_46(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_46)
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn o1_473(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_473)
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn o1_487(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_487)
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn o1_5(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_5)
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn o1_513(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_513)
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn o1_527(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_527)
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn o1_54(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_54)
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn o1_553(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_553)
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn o1_567(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_567)
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn o1_58(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_58)
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn o1_593(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_593)
    }
    #[doc = "`100001`"]
    #[inline(always)]
    pub fn o1_607(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_607)
    }
    #[doc = "`100010`"]
    #[inline(always)]
    pub fn o1_627(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_627)
    }
    #[doc = "`100011`"]
    #[inline(always)]
    pub fn o1_64(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_64)
    }
    #[doc = "`100100`"]
    #[inline(always)]
    pub fn o1_653(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_653)
    }
    #[doc = "`100101`"]
    #[inline(always)]
    pub fn o1_667(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_667)
    }
    #[doc = "`100110`"]
    #[inline(always)]
    pub fn o1_680(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_680)
    }
    #[doc = "`100111`"]
    #[inline(always)]
    pub fn o1_693(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_693)
    }
    #[doc = "`101000`"]
    #[inline(always)]
    pub fn o1_707(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_707)
    }
    #[doc = "`101001`"]
    #[inline(always)]
    pub fn o1_720(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_720)
    }
    #[doc = "`101010`"]
    #[inline(always)]
    pub fn o1_733(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_733)
    }
    #[doc = "`101011`"]
    #[inline(always)]
    pub fn o1_747(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_747)
    }
    #[doc = "`101100`"]
    #[inline(always)]
    pub fn o1_76(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_76)
    }
    #[doc = "`101101`"]
    #[inline(always)]
    pub fn o1_773(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_773)
    }
    #[doc = "`101110`"]
    #[inline(always)]
    pub fn o1_787(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_787)
    }
    #[doc = "`101111`"]
    #[inline(always)]
    pub fn o1_8(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_8)
    }
    #[doc = "`110000`"]
    #[inline(always)]
    pub fn o1_813(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_813)
    }
    #[doc = "`110001`"]
    #[inline(always)]
    pub fn o1_827(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_827)
    }
    #[doc = "`110010`"]
    #[inline(always)]
    pub fn o1_84(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_84)
    }
    #[doc = "`110011`"]
    #[inline(always)]
    pub fn o1_853(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_853)
    }
    #[doc = "`110100`"]
    #[inline(always)]
    pub fn o1_867(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_867)
    }
    #[doc = "`110101`"]
    #[inline(always)]
    pub fn o1_88(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_88)
    }
    #[doc = "`110110`"]
    #[inline(always)]
    pub fn o1_893(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_893)
    }
    #[doc = "`110111`"]
    #[inline(always)]
    pub fn o1_907(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_907)
    }
    #[doc = "`111000`"]
    #[inline(always)]
    pub fn o1_92(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_92)
    }
    #[doc = "`111001`"]
    #[inline(always)]
    pub fn o1_933(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_933)
    }
    #[doc = "`111010`"]
    #[inline(always)]
    pub fn o1_947(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_947)
    }
    #[doc = "`111011`"]
    #[inline(always)]
    pub fn o1_96(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_96)
    }
    #[doc = "`111100`"]
    #[inline(always)]
    pub fn o1_973(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_973)
    }
    #[doc = "`111101`"]
    #[inline(always)]
    pub fn o1_987(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O1_987)
    }
    #[doc = "`111110`"]
    #[inline(always)]
    pub fn o2(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O2)
    }
    #[doc = "`111111`"]
    #[inline(always)]
    pub fn o2_013(self) -> &'a mut W {
        self.variant(LDOB_TRIM_A::O2_013)
    }
}
#[doc = "Field `spare` reader - "]
pub type SPARE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spare` writer - "]
pub type SPARE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYS_LDO_CTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ldoa_trim(&self) -> LDOA_TRIM_R {
        LDOA_TRIM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ldob_trim(&self) -> LDOB_TRIM_R {
        LDOB_TRIM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ldoa_trim(&mut self) -> LDOA_TRIM_W<0> {
        LDOA_TRIM_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn ldob_trim(&mut self) -> LDOB_TRIM_W<8> {
        LDOB_TRIM_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SPARE_W<24> {
        SPARE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System LDO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_ldo_ctrl](index.html) module"]
pub struct SYS_LDO_CTRL_SPEC;
impl crate::RegisterSpec for SYS_LDO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_ldo_ctrl::R](R) reader structure"]
impl crate::Readable for SYS_LDO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_ldo_ctrl::W](W) writer structure"]
impl crate::Writable for SYS_LDO_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_ldo_ctrl to value 0"]
impl crate::Resettable for SYS_LDO_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
