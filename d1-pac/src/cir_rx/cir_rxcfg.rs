#[doc = "Register `cir_rxcfg` reader"]
pub struct R(crate::R<CIR_RXCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_RXCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_RXCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_RXCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cir_rxcfg` writer"]
pub struct W(crate::W<CIR_RXCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_RXCFG_SPEC>;
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
impl From<crate::W<CIR_RXCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_RXCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `scs` reader - Sample Clock Select for CIR"]
pub type SCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `scs` writer - Sample Clock Select for CIR"]
pub type SCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIR_RXCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `nthr` reader - Noise Threshold for CIR\n\nWhen the duration of the signal pulse (high or low level) is less than NTHR, the pulse is taken as noise and should be discarded by hardware."]
pub type NTHR_R = crate::FieldReader<u8, NTHR_A>;
#[doc = "Noise Threshold for CIR\n\nWhen the duration of the signal pulse (high or low level) is less than NTHR, the pulse is taken as noise and should be discarded by hardware.\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NTHR_A {
    #[doc = "0: All samples are recorded into RX FIFO"]
    T0 = 0,
    #[doc = "1: If the signal is only one sample duration, it is taken as noise and discarded."]
    T1 = 1,
    #[doc = "2: If the signal is less than 2 sample duration, it is taken as noise and discarded"]
    T2 = 2,
    #[doc = "3: If the signal is less than 3 sample duration, it is taken as noise and discarded"]
    T3 = 3,
    #[doc = "4: If the signal is less than 4 sample duration, it is taken as noise and discarded"]
    T4 = 4,
    #[doc = "5: If the signal is less than 5 sample duration, it is taken as noise and discarded"]
    T5 = 5,
    #[doc = "6: If the signal is less than 6 sample duration, it is taken as noise and discarded"]
    T6 = 6,
    #[doc = "7: If the signal is less than 7 sample duration, it is taken as noise and discarded"]
    T7 = 7,
    #[doc = "8: If the signal is less than 8 sample duration, it is taken as noise and discarded"]
    T8 = 8,
    #[doc = "9: If the signal is less than 9 sample duration, it is taken as noise and discarded"]
    T9 = 9,
    #[doc = "10: If the signal is less than 10 sample duration, it is taken as noise and discarded"]
    T10 = 10,
    #[doc = "11: If the signal is less than 11 sample duration, it is taken as noise and discarded"]
    T11 = 11,
    #[doc = "12: If the signal is less than 12 sample duration, it is taken as noise and discarded"]
    T12 = 12,
    #[doc = "13: If the signal is less than 13 sample duration, it is taken as noise and discarded"]
    T13 = 13,
    #[doc = "14: If the signal is less than 14 sample duration, it is taken as noise and discarded"]
    T14 = 14,
    #[doc = "15: If the signal is less than 15 sample duration, it is taken as noise and discarded"]
    T15 = 15,
    #[doc = "16: If the signal is less than 16 sample duration, it is taken as noise and discarded"]
    T16 = 16,
    #[doc = "17: If the signal is less than 17 sample duration, it is taken as noise and discarded"]
    T17 = 17,
    #[doc = "18: If the signal is less than 18 sample duration, it is taken as noise and discarded"]
    T18 = 18,
    #[doc = "19: If the signal is less than 19 sample duration, it is taken as noise and discarded"]
    T19 = 19,
    #[doc = "20: If the signal is less than 20 sample duration, it is taken as noise and discarded"]
    T20 = 20,
    #[doc = "21: If the signal is less than 21 sample duration, it is taken as noise and discarded"]
    T21 = 21,
    #[doc = "22: If the signal is less than 22 sample duration, it is taken as noise and discarded"]
    T22 = 22,
    #[doc = "23: If the signal is less than 23 sample duration, it is taken as noise and discarded"]
    T23 = 23,
    #[doc = "24: If the signal is less than 24 sample duration, it is taken as noise and discarded"]
    T24 = 24,
    #[doc = "25: If the signal is less than 25 sample duration, it is taken as noise and discarded"]
    T25 = 25,
    #[doc = "26: If the signal is less than 26 sample duration, it is taken as noise and discarded"]
    T26 = 26,
    #[doc = "27: If the signal is less than 27 sample duration, it is taken as noise and discarded"]
    T27 = 27,
    #[doc = "28: If the signal is less than 28 sample duration, it is taken as noise and discarded"]
    T28 = 28,
    #[doc = "29: If the signal is less than 29 sample duration, it is taken as noise and discarded"]
    T29 = 29,
    #[doc = "30: If the signal is less than 30 sample duration, it is taken as noise and discarded"]
    T30 = 30,
    #[doc = "31: If the signal is less than 31 sample duration, it is taken as noise and discarded"]
    T31 = 31,
    #[doc = "32: If the signal is less than 32 sample duration, it is taken as noise and discarded"]
    T32 = 32,
    #[doc = "33: If the signal is less than 33 sample duration, it is taken as noise and discarded"]
    T33 = 33,
    #[doc = "34: If the signal is less than 34 sample duration, it is taken as noise and discarded"]
    T34 = 34,
    #[doc = "35: If the signal is less than 35 sample duration, it is taken as noise and discarded"]
    T35 = 35,
    #[doc = "36: If the signal is less than 36 sample duration, it is taken as noise and discarded"]
    T36 = 36,
    #[doc = "37: If the signal is less than 37 sample duration, it is taken as noise and discarded"]
    T37 = 37,
    #[doc = "38: If the signal is less than 38 sample duration, it is taken as noise and discarded"]
    T38 = 38,
    #[doc = "39: If the signal is less than 39 sample duration, it is taken as noise and discarded"]
    T39 = 39,
    #[doc = "40: If the signal is less than 40 sample duration, it is taken as noise and discarded"]
    T40 = 40,
    #[doc = "41: If the signal is less than 41 sample duration, it is taken as noise and discarded"]
    T41 = 41,
    #[doc = "42: If the signal is less than 42 sample duration, it is taken as noise and discarded"]
    T42 = 42,
    #[doc = "43: If the signal is less than 43 sample duration, it is taken as noise and discarded"]
    T43 = 43,
    #[doc = "44: If the signal is less than 44 sample duration, it is taken as noise and discarded"]
    T44 = 44,
    #[doc = "45: If the signal is less than 45 sample duration, it is taken as noise and discarded"]
    T45 = 45,
    #[doc = "46: If the signal is less than 46 sample duration, it is taken as noise and discarded"]
    T46 = 46,
    #[doc = "47: If the signal is less than 47 sample duration, it is taken as noise and discarded"]
    T47 = 47,
    #[doc = "48: If the signal is less than 48 sample duration, it is taken as noise and discarded"]
    T48 = 48,
    #[doc = "49: If the signal is less than 49 sample duration, it is taken as noise and discarded"]
    T49 = 49,
    #[doc = "50: If the signal is less than 50 sample duration, it is taken as noise and discarded"]
    T50 = 50,
    #[doc = "51: If the signal is less than 51 sample duration, it is taken as noise and discarded"]
    T51 = 51,
    #[doc = "52: If the signal is less than 52 sample duration, it is taken as noise and discarded"]
    T52 = 52,
    #[doc = "53: If the signal is less than 53 sample duration, it is taken as noise and discarded"]
    T53 = 53,
    #[doc = "54: If the signal is less than 54 sample duration, it is taken as noise and discarded"]
    T54 = 54,
    #[doc = "55: If the signal is less than 55 sample duration, it is taken as noise and discarded"]
    T55 = 55,
    #[doc = "56: If the signal is less than 56 sample duration, it is taken as noise and discarded"]
    T56 = 56,
    #[doc = "57: If the signal is less than 57 sample duration, it is taken as noise and discarded"]
    T57 = 57,
    #[doc = "58: If the signal is less than 58 sample duration, it is taken as noise and discarded"]
    T58 = 58,
    #[doc = "59: If the signal is less than 59 sample duration, it is taken as noise and discarded"]
    T59 = 59,
    #[doc = "60: If the signal is less than 60 sample duration, it is taken as noise and discarded"]
    T60 = 60,
    #[doc = "61: If the signal is less than 61 sample duration, it is taken as noise and discarded"]
    T61 = 61,
}
impl From<NTHR_A> for u8 {
    #[inline(always)]
    fn from(variant: NTHR_A) -> Self {
        variant as _
    }
}
impl NTHR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NTHR_A> {
        match self.bits {
            0 => Some(NTHR_A::T0),
            1 => Some(NTHR_A::T1),
            2 => Some(NTHR_A::T2),
            3 => Some(NTHR_A::T3),
            4 => Some(NTHR_A::T4),
            5 => Some(NTHR_A::T5),
            6 => Some(NTHR_A::T6),
            7 => Some(NTHR_A::T7),
            8 => Some(NTHR_A::T8),
            9 => Some(NTHR_A::T9),
            10 => Some(NTHR_A::T10),
            11 => Some(NTHR_A::T11),
            12 => Some(NTHR_A::T12),
            13 => Some(NTHR_A::T13),
            14 => Some(NTHR_A::T14),
            15 => Some(NTHR_A::T15),
            16 => Some(NTHR_A::T16),
            17 => Some(NTHR_A::T17),
            18 => Some(NTHR_A::T18),
            19 => Some(NTHR_A::T19),
            20 => Some(NTHR_A::T20),
            21 => Some(NTHR_A::T21),
            22 => Some(NTHR_A::T22),
            23 => Some(NTHR_A::T23),
            24 => Some(NTHR_A::T24),
            25 => Some(NTHR_A::T25),
            26 => Some(NTHR_A::T26),
            27 => Some(NTHR_A::T27),
            28 => Some(NTHR_A::T28),
            29 => Some(NTHR_A::T29),
            30 => Some(NTHR_A::T30),
            31 => Some(NTHR_A::T31),
            32 => Some(NTHR_A::T32),
            33 => Some(NTHR_A::T33),
            34 => Some(NTHR_A::T34),
            35 => Some(NTHR_A::T35),
            36 => Some(NTHR_A::T36),
            37 => Some(NTHR_A::T37),
            38 => Some(NTHR_A::T38),
            39 => Some(NTHR_A::T39),
            40 => Some(NTHR_A::T40),
            41 => Some(NTHR_A::T41),
            42 => Some(NTHR_A::T42),
            43 => Some(NTHR_A::T43),
            44 => Some(NTHR_A::T44),
            45 => Some(NTHR_A::T45),
            46 => Some(NTHR_A::T46),
            47 => Some(NTHR_A::T47),
            48 => Some(NTHR_A::T48),
            49 => Some(NTHR_A::T49),
            50 => Some(NTHR_A::T50),
            51 => Some(NTHR_A::T51),
            52 => Some(NTHR_A::T52),
            53 => Some(NTHR_A::T53),
            54 => Some(NTHR_A::T54),
            55 => Some(NTHR_A::T55),
            56 => Some(NTHR_A::T56),
            57 => Some(NTHR_A::T57),
            58 => Some(NTHR_A::T58),
            59 => Some(NTHR_A::T59),
            60 => Some(NTHR_A::T60),
            61 => Some(NTHR_A::T61),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `T0`"]
    #[inline(always)]
    pub fn is_t0(&self) -> bool {
        *self == NTHR_A::T0
    }
    #[doc = "Checks if the value of the field is `T1`"]
    #[inline(always)]
    pub fn is_t1(&self) -> bool {
        *self == NTHR_A::T1
    }
    #[doc = "Checks if the value of the field is `T2`"]
    #[inline(always)]
    pub fn is_t2(&self) -> bool {
        *self == NTHR_A::T2
    }
    #[doc = "Checks if the value of the field is `T3`"]
    #[inline(always)]
    pub fn is_t3(&self) -> bool {
        *self == NTHR_A::T3
    }
    #[doc = "Checks if the value of the field is `T4`"]
    #[inline(always)]
    pub fn is_t4(&self) -> bool {
        *self == NTHR_A::T4
    }
    #[doc = "Checks if the value of the field is `T5`"]
    #[inline(always)]
    pub fn is_t5(&self) -> bool {
        *self == NTHR_A::T5
    }
    #[doc = "Checks if the value of the field is `T6`"]
    #[inline(always)]
    pub fn is_t6(&self) -> bool {
        *self == NTHR_A::T6
    }
    #[doc = "Checks if the value of the field is `T7`"]
    #[inline(always)]
    pub fn is_t7(&self) -> bool {
        *self == NTHR_A::T7
    }
    #[doc = "Checks if the value of the field is `T8`"]
    #[inline(always)]
    pub fn is_t8(&self) -> bool {
        *self == NTHR_A::T8
    }
    #[doc = "Checks if the value of the field is `T9`"]
    #[inline(always)]
    pub fn is_t9(&self) -> bool {
        *self == NTHR_A::T9
    }
    #[doc = "Checks if the value of the field is `T10`"]
    #[inline(always)]
    pub fn is_t10(&self) -> bool {
        *self == NTHR_A::T10
    }
    #[doc = "Checks if the value of the field is `T11`"]
    #[inline(always)]
    pub fn is_t11(&self) -> bool {
        *self == NTHR_A::T11
    }
    #[doc = "Checks if the value of the field is `T12`"]
    #[inline(always)]
    pub fn is_t12(&self) -> bool {
        *self == NTHR_A::T12
    }
    #[doc = "Checks if the value of the field is `T13`"]
    #[inline(always)]
    pub fn is_t13(&self) -> bool {
        *self == NTHR_A::T13
    }
    #[doc = "Checks if the value of the field is `T14`"]
    #[inline(always)]
    pub fn is_t14(&self) -> bool {
        *self == NTHR_A::T14
    }
    #[doc = "Checks if the value of the field is `T15`"]
    #[inline(always)]
    pub fn is_t15(&self) -> bool {
        *self == NTHR_A::T15
    }
    #[doc = "Checks if the value of the field is `T16`"]
    #[inline(always)]
    pub fn is_t16(&self) -> bool {
        *self == NTHR_A::T16
    }
    #[doc = "Checks if the value of the field is `T17`"]
    #[inline(always)]
    pub fn is_t17(&self) -> bool {
        *self == NTHR_A::T17
    }
    #[doc = "Checks if the value of the field is `T18`"]
    #[inline(always)]
    pub fn is_t18(&self) -> bool {
        *self == NTHR_A::T18
    }
    #[doc = "Checks if the value of the field is `T19`"]
    #[inline(always)]
    pub fn is_t19(&self) -> bool {
        *self == NTHR_A::T19
    }
    #[doc = "Checks if the value of the field is `T20`"]
    #[inline(always)]
    pub fn is_t20(&self) -> bool {
        *self == NTHR_A::T20
    }
    #[doc = "Checks if the value of the field is `T21`"]
    #[inline(always)]
    pub fn is_t21(&self) -> bool {
        *self == NTHR_A::T21
    }
    #[doc = "Checks if the value of the field is `T22`"]
    #[inline(always)]
    pub fn is_t22(&self) -> bool {
        *self == NTHR_A::T22
    }
    #[doc = "Checks if the value of the field is `T23`"]
    #[inline(always)]
    pub fn is_t23(&self) -> bool {
        *self == NTHR_A::T23
    }
    #[doc = "Checks if the value of the field is `T24`"]
    #[inline(always)]
    pub fn is_t24(&self) -> bool {
        *self == NTHR_A::T24
    }
    #[doc = "Checks if the value of the field is `T25`"]
    #[inline(always)]
    pub fn is_t25(&self) -> bool {
        *self == NTHR_A::T25
    }
    #[doc = "Checks if the value of the field is `T26`"]
    #[inline(always)]
    pub fn is_t26(&self) -> bool {
        *self == NTHR_A::T26
    }
    #[doc = "Checks if the value of the field is `T27`"]
    #[inline(always)]
    pub fn is_t27(&self) -> bool {
        *self == NTHR_A::T27
    }
    #[doc = "Checks if the value of the field is `T28`"]
    #[inline(always)]
    pub fn is_t28(&self) -> bool {
        *self == NTHR_A::T28
    }
    #[doc = "Checks if the value of the field is `T29`"]
    #[inline(always)]
    pub fn is_t29(&self) -> bool {
        *self == NTHR_A::T29
    }
    #[doc = "Checks if the value of the field is `T30`"]
    #[inline(always)]
    pub fn is_t30(&self) -> bool {
        *self == NTHR_A::T30
    }
    #[doc = "Checks if the value of the field is `T31`"]
    #[inline(always)]
    pub fn is_t31(&self) -> bool {
        *self == NTHR_A::T31
    }
    #[doc = "Checks if the value of the field is `T32`"]
    #[inline(always)]
    pub fn is_t32(&self) -> bool {
        *self == NTHR_A::T32
    }
    #[doc = "Checks if the value of the field is `T33`"]
    #[inline(always)]
    pub fn is_t33(&self) -> bool {
        *self == NTHR_A::T33
    }
    #[doc = "Checks if the value of the field is `T34`"]
    #[inline(always)]
    pub fn is_t34(&self) -> bool {
        *self == NTHR_A::T34
    }
    #[doc = "Checks if the value of the field is `T35`"]
    #[inline(always)]
    pub fn is_t35(&self) -> bool {
        *self == NTHR_A::T35
    }
    #[doc = "Checks if the value of the field is `T36`"]
    #[inline(always)]
    pub fn is_t36(&self) -> bool {
        *self == NTHR_A::T36
    }
    #[doc = "Checks if the value of the field is `T37`"]
    #[inline(always)]
    pub fn is_t37(&self) -> bool {
        *self == NTHR_A::T37
    }
    #[doc = "Checks if the value of the field is `T38`"]
    #[inline(always)]
    pub fn is_t38(&self) -> bool {
        *self == NTHR_A::T38
    }
    #[doc = "Checks if the value of the field is `T39`"]
    #[inline(always)]
    pub fn is_t39(&self) -> bool {
        *self == NTHR_A::T39
    }
    #[doc = "Checks if the value of the field is `T40`"]
    #[inline(always)]
    pub fn is_t40(&self) -> bool {
        *self == NTHR_A::T40
    }
    #[doc = "Checks if the value of the field is `T41`"]
    #[inline(always)]
    pub fn is_t41(&self) -> bool {
        *self == NTHR_A::T41
    }
    #[doc = "Checks if the value of the field is `T42`"]
    #[inline(always)]
    pub fn is_t42(&self) -> bool {
        *self == NTHR_A::T42
    }
    #[doc = "Checks if the value of the field is `T43`"]
    #[inline(always)]
    pub fn is_t43(&self) -> bool {
        *self == NTHR_A::T43
    }
    #[doc = "Checks if the value of the field is `T44`"]
    #[inline(always)]
    pub fn is_t44(&self) -> bool {
        *self == NTHR_A::T44
    }
    #[doc = "Checks if the value of the field is `T45`"]
    #[inline(always)]
    pub fn is_t45(&self) -> bool {
        *self == NTHR_A::T45
    }
    #[doc = "Checks if the value of the field is `T46`"]
    #[inline(always)]
    pub fn is_t46(&self) -> bool {
        *self == NTHR_A::T46
    }
    #[doc = "Checks if the value of the field is `T47`"]
    #[inline(always)]
    pub fn is_t47(&self) -> bool {
        *self == NTHR_A::T47
    }
    #[doc = "Checks if the value of the field is `T48`"]
    #[inline(always)]
    pub fn is_t48(&self) -> bool {
        *self == NTHR_A::T48
    }
    #[doc = "Checks if the value of the field is `T49`"]
    #[inline(always)]
    pub fn is_t49(&self) -> bool {
        *self == NTHR_A::T49
    }
    #[doc = "Checks if the value of the field is `T50`"]
    #[inline(always)]
    pub fn is_t50(&self) -> bool {
        *self == NTHR_A::T50
    }
    #[doc = "Checks if the value of the field is `T51`"]
    #[inline(always)]
    pub fn is_t51(&self) -> bool {
        *self == NTHR_A::T51
    }
    #[doc = "Checks if the value of the field is `T52`"]
    #[inline(always)]
    pub fn is_t52(&self) -> bool {
        *self == NTHR_A::T52
    }
    #[doc = "Checks if the value of the field is `T53`"]
    #[inline(always)]
    pub fn is_t53(&self) -> bool {
        *self == NTHR_A::T53
    }
    #[doc = "Checks if the value of the field is `T54`"]
    #[inline(always)]
    pub fn is_t54(&self) -> bool {
        *self == NTHR_A::T54
    }
    #[doc = "Checks if the value of the field is `T55`"]
    #[inline(always)]
    pub fn is_t55(&self) -> bool {
        *self == NTHR_A::T55
    }
    #[doc = "Checks if the value of the field is `T56`"]
    #[inline(always)]
    pub fn is_t56(&self) -> bool {
        *self == NTHR_A::T56
    }
    #[doc = "Checks if the value of the field is `T57`"]
    #[inline(always)]
    pub fn is_t57(&self) -> bool {
        *self == NTHR_A::T57
    }
    #[doc = "Checks if the value of the field is `T58`"]
    #[inline(always)]
    pub fn is_t58(&self) -> bool {
        *self == NTHR_A::T58
    }
    #[doc = "Checks if the value of the field is `T59`"]
    #[inline(always)]
    pub fn is_t59(&self) -> bool {
        *self == NTHR_A::T59
    }
    #[doc = "Checks if the value of the field is `T60`"]
    #[inline(always)]
    pub fn is_t60(&self) -> bool {
        *self == NTHR_A::T60
    }
    #[doc = "Checks if the value of the field is `T61`"]
    #[inline(always)]
    pub fn is_t61(&self) -> bool {
        *self == NTHR_A::T61
    }
}
#[doc = "Field `nthr` writer - Noise Threshold for CIR\n\nWhen the duration of the signal pulse (high or low level) is less than NTHR, the pulse is taken as noise and should be discarded by hardware."]
pub type NTHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIR_RXCFG_SPEC, u8, NTHR_A, 6, O>;
impl<'a, const O: u8> NTHR_W<'a, O> {
    #[doc = "All samples are recorded into RX FIFO"]
    #[inline(always)]
    pub fn t0(self) -> &'a mut W {
        self.variant(NTHR_A::T0)
    }
    #[doc = "If the signal is only one sample duration, it is taken as noise and discarded."]
    #[inline(always)]
    pub fn t1(self) -> &'a mut W {
        self.variant(NTHR_A::T1)
    }
    #[doc = "If the signal is less than 2 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t2(self) -> &'a mut W {
        self.variant(NTHR_A::T2)
    }
    #[doc = "If the signal is less than 3 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t3(self) -> &'a mut W {
        self.variant(NTHR_A::T3)
    }
    #[doc = "If the signal is less than 4 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t4(self) -> &'a mut W {
        self.variant(NTHR_A::T4)
    }
    #[doc = "If the signal is less than 5 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t5(self) -> &'a mut W {
        self.variant(NTHR_A::T5)
    }
    #[doc = "If the signal is less than 6 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t6(self) -> &'a mut W {
        self.variant(NTHR_A::T6)
    }
    #[doc = "If the signal is less than 7 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t7(self) -> &'a mut W {
        self.variant(NTHR_A::T7)
    }
    #[doc = "If the signal is less than 8 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t8(self) -> &'a mut W {
        self.variant(NTHR_A::T8)
    }
    #[doc = "If the signal is less than 9 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t9(self) -> &'a mut W {
        self.variant(NTHR_A::T9)
    }
    #[doc = "If the signal is less than 10 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t10(self) -> &'a mut W {
        self.variant(NTHR_A::T10)
    }
    #[doc = "If the signal is less than 11 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t11(self) -> &'a mut W {
        self.variant(NTHR_A::T11)
    }
    #[doc = "If the signal is less than 12 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t12(self) -> &'a mut W {
        self.variant(NTHR_A::T12)
    }
    #[doc = "If the signal is less than 13 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t13(self) -> &'a mut W {
        self.variant(NTHR_A::T13)
    }
    #[doc = "If the signal is less than 14 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t14(self) -> &'a mut W {
        self.variant(NTHR_A::T14)
    }
    #[doc = "If the signal is less than 15 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t15(self) -> &'a mut W {
        self.variant(NTHR_A::T15)
    }
    #[doc = "If the signal is less than 16 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t16(self) -> &'a mut W {
        self.variant(NTHR_A::T16)
    }
    #[doc = "If the signal is less than 17 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t17(self) -> &'a mut W {
        self.variant(NTHR_A::T17)
    }
    #[doc = "If the signal is less than 18 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t18(self) -> &'a mut W {
        self.variant(NTHR_A::T18)
    }
    #[doc = "If the signal is less than 19 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t19(self) -> &'a mut W {
        self.variant(NTHR_A::T19)
    }
    #[doc = "If the signal is less than 20 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t20(self) -> &'a mut W {
        self.variant(NTHR_A::T20)
    }
    #[doc = "If the signal is less than 21 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t21(self) -> &'a mut W {
        self.variant(NTHR_A::T21)
    }
    #[doc = "If the signal is less than 22 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t22(self) -> &'a mut W {
        self.variant(NTHR_A::T22)
    }
    #[doc = "If the signal is less than 23 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t23(self) -> &'a mut W {
        self.variant(NTHR_A::T23)
    }
    #[doc = "If the signal is less than 24 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t24(self) -> &'a mut W {
        self.variant(NTHR_A::T24)
    }
    #[doc = "If the signal is less than 25 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t25(self) -> &'a mut W {
        self.variant(NTHR_A::T25)
    }
    #[doc = "If the signal is less than 26 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t26(self) -> &'a mut W {
        self.variant(NTHR_A::T26)
    }
    #[doc = "If the signal is less than 27 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t27(self) -> &'a mut W {
        self.variant(NTHR_A::T27)
    }
    #[doc = "If the signal is less than 28 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t28(self) -> &'a mut W {
        self.variant(NTHR_A::T28)
    }
    #[doc = "If the signal is less than 29 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t29(self) -> &'a mut W {
        self.variant(NTHR_A::T29)
    }
    #[doc = "If the signal is less than 30 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t30(self) -> &'a mut W {
        self.variant(NTHR_A::T30)
    }
    #[doc = "If the signal is less than 31 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t31(self) -> &'a mut W {
        self.variant(NTHR_A::T31)
    }
    #[doc = "If the signal is less than 32 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t32(self) -> &'a mut W {
        self.variant(NTHR_A::T32)
    }
    #[doc = "If the signal is less than 33 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t33(self) -> &'a mut W {
        self.variant(NTHR_A::T33)
    }
    #[doc = "If the signal is less than 34 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t34(self) -> &'a mut W {
        self.variant(NTHR_A::T34)
    }
    #[doc = "If the signal is less than 35 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t35(self) -> &'a mut W {
        self.variant(NTHR_A::T35)
    }
    #[doc = "If the signal is less than 36 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t36(self) -> &'a mut W {
        self.variant(NTHR_A::T36)
    }
    #[doc = "If the signal is less than 37 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t37(self) -> &'a mut W {
        self.variant(NTHR_A::T37)
    }
    #[doc = "If the signal is less than 38 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t38(self) -> &'a mut W {
        self.variant(NTHR_A::T38)
    }
    #[doc = "If the signal is less than 39 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t39(self) -> &'a mut W {
        self.variant(NTHR_A::T39)
    }
    #[doc = "If the signal is less than 40 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t40(self) -> &'a mut W {
        self.variant(NTHR_A::T40)
    }
    #[doc = "If the signal is less than 41 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t41(self) -> &'a mut W {
        self.variant(NTHR_A::T41)
    }
    #[doc = "If the signal is less than 42 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t42(self) -> &'a mut W {
        self.variant(NTHR_A::T42)
    }
    #[doc = "If the signal is less than 43 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t43(self) -> &'a mut W {
        self.variant(NTHR_A::T43)
    }
    #[doc = "If the signal is less than 44 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t44(self) -> &'a mut W {
        self.variant(NTHR_A::T44)
    }
    #[doc = "If the signal is less than 45 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t45(self) -> &'a mut W {
        self.variant(NTHR_A::T45)
    }
    #[doc = "If the signal is less than 46 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t46(self) -> &'a mut W {
        self.variant(NTHR_A::T46)
    }
    #[doc = "If the signal is less than 47 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t47(self) -> &'a mut W {
        self.variant(NTHR_A::T47)
    }
    #[doc = "If the signal is less than 48 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t48(self) -> &'a mut W {
        self.variant(NTHR_A::T48)
    }
    #[doc = "If the signal is less than 49 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t49(self) -> &'a mut W {
        self.variant(NTHR_A::T49)
    }
    #[doc = "If the signal is less than 50 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t50(self) -> &'a mut W {
        self.variant(NTHR_A::T50)
    }
    #[doc = "If the signal is less than 51 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t51(self) -> &'a mut W {
        self.variant(NTHR_A::T51)
    }
    #[doc = "If the signal is less than 52 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t52(self) -> &'a mut W {
        self.variant(NTHR_A::T52)
    }
    #[doc = "If the signal is less than 53 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t53(self) -> &'a mut W {
        self.variant(NTHR_A::T53)
    }
    #[doc = "If the signal is less than 54 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t54(self) -> &'a mut W {
        self.variant(NTHR_A::T54)
    }
    #[doc = "If the signal is less than 55 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t55(self) -> &'a mut W {
        self.variant(NTHR_A::T55)
    }
    #[doc = "If the signal is less than 56 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t56(self) -> &'a mut W {
        self.variant(NTHR_A::T56)
    }
    #[doc = "If the signal is less than 57 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t57(self) -> &'a mut W {
        self.variant(NTHR_A::T57)
    }
    #[doc = "If the signal is less than 58 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t58(self) -> &'a mut W {
        self.variant(NTHR_A::T58)
    }
    #[doc = "If the signal is less than 59 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t59(self) -> &'a mut W {
        self.variant(NTHR_A::T59)
    }
    #[doc = "If the signal is less than 60 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t60(self) -> &'a mut W {
        self.variant(NTHR_A::T60)
    }
    #[doc = "If the signal is less than 61 sample duration, it is taken as noise and discarded"]
    #[inline(always)]
    pub fn t61(self) -> &'a mut W {
        self.variant(NTHR_A::T61)
    }
}
#[doc = "Field `ithr` reader - Idle Threshold for CIR\n\nThe Receiver uses it to decide whether the CIR command is received. If there is no CIR signal on the air, the receiver is staying in IDLE status. One active pulse will bring the receiver from IDLE status to Receiving status. After the CIR receiver ends, the inputting signal will keep the specified level (high or low level) for a long time. The receiver can use this idle signal duration to decide that it has received the CIR command. The corresponding flag is asserted. If the corresponding interrupt is enabled, the interrupt line is asserted to the CPU.\n\nWhen the duration of the signal keeps one status (high or low level) for the specified duration ((ITHR + 1)*128 sample_clk), this means that the previous CIR command is finished."]
pub type ITHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ithr` writer - Idle Threshold for CIR\n\nThe Receiver uses it to decide whether the CIR command is received. If there is no CIR signal on the air, the receiver is staying in IDLE status. One active pulse will bring the receiver from IDLE status to Receiving status. After the CIR receiver ends, the inputting signal will keep the specified level (high or low level) for a long time. The receiver can use this idle signal duration to decide that it has received the CIR command. The corresponding flag is asserted. If the corresponding interrupt is enabled, the interrupt line is asserted to the CPU.\n\nWhen the duration of the signal keeps one status (high or low level) for the specified duration ((ITHR + 1)*128 sample_clk), this means that the previous CIR command is finished."]
pub type ITHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIR_RXCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `athr` reader - Active Threshold for CIR\n\nThese bits control the duration of CIR from the idle to the active state. The duration can be calculated by ((ATHR + 1)*(ATHC? Sample Clock: 128*Sample Clock))."]
pub type ATHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `athr` writer - Active Threshold for CIR\n\nThese bits control the duration of CIR from the idle to the active state. The duration can be calculated by ((ATHR + 1)*(ATHC? Sample Clock: 128*Sample Clock))."]
pub type ATHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIR_RXCFG_SPEC, u8, u8, 7, O>;
#[doc = "Field `athc` reader - Active Threshold Control for CIR"]
pub type ATHC_R = crate::BitReader<ATHC_A>;
#[doc = "Active Threshold Control for CIR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATHC_A {
    #[doc = "0: ATHR in a unit of (Sample Clock)"]
    SAMPLE = 0,
    #[doc = "1: ATHR in a unit of (128*Sample Clocks)"]
    SAMPLE_128 = 1,
}
impl From<ATHC_A> for bool {
    #[inline(always)]
    fn from(variant: ATHC_A) -> Self {
        variant as u8 != 0
    }
}
impl ATHC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATHC_A {
        match self.bits {
            false => ATHC_A::SAMPLE,
            true => ATHC_A::SAMPLE_128,
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE`"]
    #[inline(always)]
    pub fn is_sample(&self) -> bool {
        *self == ATHC_A::SAMPLE
    }
    #[doc = "Checks if the value of the field is `SAMPLE_128`"]
    #[inline(always)]
    pub fn is_sample_128(&self) -> bool {
        *self == ATHC_A::SAMPLE_128
    }
}
#[doc = "Field `athc` writer - Active Threshold Control for CIR"]
pub type ATHC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_RXCFG_SPEC, ATHC_A, O>;
impl<'a, const O: u8> ATHC_W<'a, O> {
    #[doc = "ATHR in a unit of (Sample Clock)"]
    #[inline(always)]
    pub fn sample(self) -> &'a mut W {
        self.variant(ATHC_A::SAMPLE)
    }
    #[doc = "ATHR in a unit of (128*Sample Clocks)"]
    #[inline(always)]
    pub fn sample_128(self) -> &'a mut W {
        self.variant(ATHC_A::SAMPLE_128)
    }
}
#[doc = "Field `scs2` reader - Bit2 of Sample Clock Select for CIR\n\nThis bit is defined by SCS bits below."]
pub type SCS2_R = crate::BitReader<bool>;
#[doc = "Field `scs2` writer - Bit2 of Sample Clock Select for CIR\n\nThis bit is defined by SCS bits below."]
pub type SCS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_RXCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Sample Clock Select for CIR"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - Noise Threshold for CIR\n\nWhen the duration of the signal pulse (high or low level) is less than NTHR, the pulse is taken as noise and should be discarded by hardware."]
    #[inline(always)]
    pub fn nthr(&self) -> NTHR_R {
        NTHR_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - Idle Threshold for CIR\n\nThe Receiver uses it to decide whether the CIR command is received. If there is no CIR signal on the air, the receiver is staying in IDLE status. One active pulse will bring the receiver from IDLE status to Receiving status. After the CIR receiver ends, the inputting signal will keep the specified level (high or low level) for a long time. The receiver can use this idle signal duration to decide that it has received the CIR command. The corresponding flag is asserted. If the corresponding interrupt is enabled, the interrupt line is asserted to the CPU.\n\nWhen the duration of the signal keeps one status (high or low level) for the specified duration ((ITHR + 1)*128 sample_clk), this means that the previous CIR command is finished."]
    #[inline(always)]
    pub fn ithr(&self) -> ITHR_R {
        ITHR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - Active Threshold for CIR\n\nThese bits control the duration of CIR from the idle to the active state. The duration can be calculated by ((ATHR + 1)*(ATHC? Sample Clock: 128*Sample Clock))."]
    #[inline(always)]
    pub fn athr(&self) -> ATHR_R {
        ATHR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Active Threshold Control for CIR"]
    #[inline(always)]
    pub fn athc(&self) -> ATHC_R {
        ATHC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bit2 of Sample Clock Select for CIR\n\nThis bit is defined by SCS bits below."]
    #[inline(always)]
    pub fn scs2(&self) -> SCS2_R {
        SCS2_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sample Clock Select for CIR"]
    #[inline(always)]
    #[must_use]
    pub fn scs(&mut self) -> SCS_W<0> {
        SCS_W::new(self)
    }
    #[doc = "Bits 2:7 - Noise Threshold for CIR\n\nWhen the duration of the signal pulse (high or low level) is less than NTHR, the pulse is taken as noise and should be discarded by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn nthr(&mut self) -> NTHR_W<2> {
        NTHR_W::new(self)
    }
    #[doc = "Bits 8:15 - Idle Threshold for CIR\n\nThe Receiver uses it to decide whether the CIR command is received. If there is no CIR signal on the air, the receiver is staying in IDLE status. One active pulse will bring the receiver from IDLE status to Receiving status. After the CIR receiver ends, the inputting signal will keep the specified level (high or low level) for a long time. The receiver can use this idle signal duration to decide that it has received the CIR command. The corresponding flag is asserted. If the corresponding interrupt is enabled, the interrupt line is asserted to the CPU.\n\nWhen the duration of the signal keeps one status (high or low level) for the specified duration ((ITHR + 1)*128 sample_clk), this means that the previous CIR command is finished."]
    #[inline(always)]
    #[must_use]
    pub fn ithr(&mut self) -> ITHR_W<8> {
        ITHR_W::new(self)
    }
    #[doc = "Bits 16:22 - Active Threshold for CIR\n\nThese bits control the duration of CIR from the idle to the active state. The duration can be calculated by ((ATHR + 1)*(ATHC? Sample Clock: 128*Sample Clock))."]
    #[inline(always)]
    #[must_use]
    pub fn athr(&mut self) -> ATHR_W<16> {
        ATHR_W::new(self)
    }
    #[doc = "Bit 23 - Active Threshold Control for CIR"]
    #[inline(always)]
    #[must_use]
    pub fn athc(&mut self) -> ATHC_W<23> {
        ATHC_W::new(self)
    }
    #[doc = "Bit 24 - Bit2 of Sample Clock Select for CIR\n\nThis bit is defined by SCS bits below."]
    #[inline(always)]
    #[must_use]
    pub fn scs2(&mut self) -> SCS2_W<24> {
        SCS2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR Receiver Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_rxcfg](index.html) module"]
pub struct CIR_RXCFG_SPEC;
impl crate::RegisterSpec for CIR_RXCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_rxcfg::R](R) reader structure"]
impl crate::Readable for CIR_RXCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_rxcfg::W](W) writer structure"]
impl crate::Writable for CIR_RXCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_rxcfg to value 0x1828"]
impl crate::Resettable for CIR_RXCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x1828;
}
