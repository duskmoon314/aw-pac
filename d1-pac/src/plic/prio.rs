#[doc = "Register `prio[%s]` reader"]
pub struct R(crate::R<PRIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `prio[%s]` writer"]
pub struct W(crate::W<PRIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIO_SPEC>;
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
impl From<crate::W<PRIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `priority` reader - "]
pub type PRIORITY_R = crate::FieldReader<u8, PRIORITY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRIORITY_A {
    #[doc = "0: Priority 0 (never interrupt)"]
    P0 = 0,
    #[doc = "1: Priority 1"]
    P1 = 1,
    #[doc = "2: Priority 2"]
    P2 = 2,
    #[doc = "3: Priority 3"]
    P3 = 3,
    #[doc = "4: Priority 4"]
    P4 = 4,
    #[doc = "5: Priority 5"]
    P5 = 5,
    #[doc = "6: Priority 6"]
    P6 = 6,
    #[doc = "7: Priority 7"]
    P7 = 7,
    #[doc = "8: Priority 8"]
    P8 = 8,
    #[doc = "9: Priority 9"]
    P9 = 9,
    #[doc = "10: Priority 10"]
    P10 = 10,
    #[doc = "11: Priority 11"]
    P11 = 11,
    #[doc = "12: Priority 12"]
    P12 = 12,
    #[doc = "13: Priority 13"]
    P13 = 13,
    #[doc = "14: Priority 14"]
    P14 = 14,
    #[doc = "15: Priority 15"]
    P15 = 15,
    #[doc = "16: Priority 16"]
    P16 = 16,
    #[doc = "17: Priority 17"]
    P17 = 17,
    #[doc = "18: Priority 18"]
    P18 = 18,
    #[doc = "19: Priority 19"]
    P19 = 19,
    #[doc = "20: Priority 20"]
    P20 = 20,
    #[doc = "21: Priority 21"]
    P21 = 21,
    #[doc = "22: Priority 22"]
    P22 = 22,
    #[doc = "23: Priority 23"]
    P23 = 23,
    #[doc = "24: Priority 24"]
    P24 = 24,
    #[doc = "25: Priority 25"]
    P25 = 25,
    #[doc = "26: Priority 26"]
    P26 = 26,
    #[doc = "27: Priority 27"]
    P27 = 27,
    #[doc = "28: Priority 28"]
    P28 = 28,
    #[doc = "29: Priority 29"]
    P29 = 29,
    #[doc = "30: Priority 30"]
    P30 = 30,
    #[doc = "31: Priority 31"]
    P31 = 31,
}
impl From<PRIORITY_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIORITY_A) -> Self {
        variant as _
    }
}
impl PRIORITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRIORITY_A {
        match self.bits {
            0 => PRIORITY_A::P0,
            1 => PRIORITY_A::P1,
            2 => PRIORITY_A::P2,
            3 => PRIORITY_A::P3,
            4 => PRIORITY_A::P4,
            5 => PRIORITY_A::P5,
            6 => PRIORITY_A::P6,
            7 => PRIORITY_A::P7,
            8 => PRIORITY_A::P8,
            9 => PRIORITY_A::P9,
            10 => PRIORITY_A::P10,
            11 => PRIORITY_A::P11,
            12 => PRIORITY_A::P12,
            13 => PRIORITY_A::P13,
            14 => PRIORITY_A::P14,
            15 => PRIORITY_A::P15,
            16 => PRIORITY_A::P16,
            17 => PRIORITY_A::P17,
            18 => PRIORITY_A::P18,
            19 => PRIORITY_A::P19,
            20 => PRIORITY_A::P20,
            21 => PRIORITY_A::P21,
            22 => PRIORITY_A::P22,
            23 => PRIORITY_A::P23,
            24 => PRIORITY_A::P24,
            25 => PRIORITY_A::P25,
            26 => PRIORITY_A::P26,
            27 => PRIORITY_A::P27,
            28 => PRIORITY_A::P28,
            29 => PRIORITY_A::P29,
            30 => PRIORITY_A::P30,
            31 => PRIORITY_A::P31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P0`"]
    #[inline(always)]
    pub fn is_p0(&self) -> bool {
        *self == PRIORITY_A::P0
    }
    #[doc = "Checks if the value of the field is `P1`"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        *self == PRIORITY_A::P1
    }
    #[doc = "Checks if the value of the field is `P2`"]
    #[inline(always)]
    pub fn is_p2(&self) -> bool {
        *self == PRIORITY_A::P2
    }
    #[doc = "Checks if the value of the field is `P3`"]
    #[inline(always)]
    pub fn is_p3(&self) -> bool {
        *self == PRIORITY_A::P3
    }
    #[doc = "Checks if the value of the field is `P4`"]
    #[inline(always)]
    pub fn is_p4(&self) -> bool {
        *self == PRIORITY_A::P4
    }
    #[doc = "Checks if the value of the field is `P5`"]
    #[inline(always)]
    pub fn is_p5(&self) -> bool {
        *self == PRIORITY_A::P5
    }
    #[doc = "Checks if the value of the field is `P6`"]
    #[inline(always)]
    pub fn is_p6(&self) -> bool {
        *self == PRIORITY_A::P6
    }
    #[doc = "Checks if the value of the field is `P7`"]
    #[inline(always)]
    pub fn is_p7(&self) -> bool {
        *self == PRIORITY_A::P7
    }
    #[doc = "Checks if the value of the field is `P8`"]
    #[inline(always)]
    pub fn is_p8(&self) -> bool {
        *self == PRIORITY_A::P8
    }
    #[doc = "Checks if the value of the field is `P9`"]
    #[inline(always)]
    pub fn is_p9(&self) -> bool {
        *self == PRIORITY_A::P9
    }
    #[doc = "Checks if the value of the field is `P10`"]
    #[inline(always)]
    pub fn is_p10(&self) -> bool {
        *self == PRIORITY_A::P10
    }
    #[doc = "Checks if the value of the field is `P11`"]
    #[inline(always)]
    pub fn is_p11(&self) -> bool {
        *self == PRIORITY_A::P11
    }
    #[doc = "Checks if the value of the field is `P12`"]
    #[inline(always)]
    pub fn is_p12(&self) -> bool {
        *self == PRIORITY_A::P12
    }
    #[doc = "Checks if the value of the field is `P13`"]
    #[inline(always)]
    pub fn is_p13(&self) -> bool {
        *self == PRIORITY_A::P13
    }
    #[doc = "Checks if the value of the field is `P14`"]
    #[inline(always)]
    pub fn is_p14(&self) -> bool {
        *self == PRIORITY_A::P14
    }
    #[doc = "Checks if the value of the field is `P15`"]
    #[inline(always)]
    pub fn is_p15(&self) -> bool {
        *self == PRIORITY_A::P15
    }
    #[doc = "Checks if the value of the field is `P16`"]
    #[inline(always)]
    pub fn is_p16(&self) -> bool {
        *self == PRIORITY_A::P16
    }
    #[doc = "Checks if the value of the field is `P17`"]
    #[inline(always)]
    pub fn is_p17(&self) -> bool {
        *self == PRIORITY_A::P17
    }
    #[doc = "Checks if the value of the field is `P18`"]
    #[inline(always)]
    pub fn is_p18(&self) -> bool {
        *self == PRIORITY_A::P18
    }
    #[doc = "Checks if the value of the field is `P19`"]
    #[inline(always)]
    pub fn is_p19(&self) -> bool {
        *self == PRIORITY_A::P19
    }
    #[doc = "Checks if the value of the field is `P20`"]
    #[inline(always)]
    pub fn is_p20(&self) -> bool {
        *self == PRIORITY_A::P20
    }
    #[doc = "Checks if the value of the field is `P21`"]
    #[inline(always)]
    pub fn is_p21(&self) -> bool {
        *self == PRIORITY_A::P21
    }
    #[doc = "Checks if the value of the field is `P22`"]
    #[inline(always)]
    pub fn is_p22(&self) -> bool {
        *self == PRIORITY_A::P22
    }
    #[doc = "Checks if the value of the field is `P23`"]
    #[inline(always)]
    pub fn is_p23(&self) -> bool {
        *self == PRIORITY_A::P23
    }
    #[doc = "Checks if the value of the field is `P24`"]
    #[inline(always)]
    pub fn is_p24(&self) -> bool {
        *self == PRIORITY_A::P24
    }
    #[doc = "Checks if the value of the field is `P25`"]
    #[inline(always)]
    pub fn is_p25(&self) -> bool {
        *self == PRIORITY_A::P25
    }
    #[doc = "Checks if the value of the field is `P26`"]
    #[inline(always)]
    pub fn is_p26(&self) -> bool {
        *self == PRIORITY_A::P26
    }
    #[doc = "Checks if the value of the field is `P27`"]
    #[inline(always)]
    pub fn is_p27(&self) -> bool {
        *self == PRIORITY_A::P27
    }
    #[doc = "Checks if the value of the field is `P28`"]
    #[inline(always)]
    pub fn is_p28(&self) -> bool {
        *self == PRIORITY_A::P28
    }
    #[doc = "Checks if the value of the field is `P29`"]
    #[inline(always)]
    pub fn is_p29(&self) -> bool {
        *self == PRIORITY_A::P29
    }
    #[doc = "Checks if the value of the field is `P30`"]
    #[inline(always)]
    pub fn is_p30(&self) -> bool {
        *self == PRIORITY_A::P30
    }
    #[doc = "Checks if the value of the field is `P31`"]
    #[inline(always)]
    pub fn is_p31(&self) -> bool {
        *self == PRIORITY_A::P31
    }
}
#[doc = "Field `priority` writer - "]
pub type PRIORITY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRIO_SPEC, u8, PRIORITY_A, 5, O>;
impl<'a, const O: u8> PRIORITY_W<'a, O> {
    #[doc = "Priority 0 (never interrupt)"]
    #[inline(always)]
    pub fn p0(self) -> &'a mut W {
        self.variant(PRIORITY_A::P0)
    }
    #[doc = "Priority 1"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut W {
        self.variant(PRIORITY_A::P1)
    }
    #[doc = "Priority 2"]
    #[inline(always)]
    pub fn p2(self) -> &'a mut W {
        self.variant(PRIORITY_A::P2)
    }
    #[doc = "Priority 3"]
    #[inline(always)]
    pub fn p3(self) -> &'a mut W {
        self.variant(PRIORITY_A::P3)
    }
    #[doc = "Priority 4"]
    #[inline(always)]
    pub fn p4(self) -> &'a mut W {
        self.variant(PRIORITY_A::P4)
    }
    #[doc = "Priority 5"]
    #[inline(always)]
    pub fn p5(self) -> &'a mut W {
        self.variant(PRIORITY_A::P5)
    }
    #[doc = "Priority 6"]
    #[inline(always)]
    pub fn p6(self) -> &'a mut W {
        self.variant(PRIORITY_A::P6)
    }
    #[doc = "Priority 7"]
    #[inline(always)]
    pub fn p7(self) -> &'a mut W {
        self.variant(PRIORITY_A::P7)
    }
    #[doc = "Priority 8"]
    #[inline(always)]
    pub fn p8(self) -> &'a mut W {
        self.variant(PRIORITY_A::P8)
    }
    #[doc = "Priority 9"]
    #[inline(always)]
    pub fn p9(self) -> &'a mut W {
        self.variant(PRIORITY_A::P9)
    }
    #[doc = "Priority 10"]
    #[inline(always)]
    pub fn p10(self) -> &'a mut W {
        self.variant(PRIORITY_A::P10)
    }
    #[doc = "Priority 11"]
    #[inline(always)]
    pub fn p11(self) -> &'a mut W {
        self.variant(PRIORITY_A::P11)
    }
    #[doc = "Priority 12"]
    #[inline(always)]
    pub fn p12(self) -> &'a mut W {
        self.variant(PRIORITY_A::P12)
    }
    #[doc = "Priority 13"]
    #[inline(always)]
    pub fn p13(self) -> &'a mut W {
        self.variant(PRIORITY_A::P13)
    }
    #[doc = "Priority 14"]
    #[inline(always)]
    pub fn p14(self) -> &'a mut W {
        self.variant(PRIORITY_A::P14)
    }
    #[doc = "Priority 15"]
    #[inline(always)]
    pub fn p15(self) -> &'a mut W {
        self.variant(PRIORITY_A::P15)
    }
    #[doc = "Priority 16"]
    #[inline(always)]
    pub fn p16(self) -> &'a mut W {
        self.variant(PRIORITY_A::P16)
    }
    #[doc = "Priority 17"]
    #[inline(always)]
    pub fn p17(self) -> &'a mut W {
        self.variant(PRIORITY_A::P17)
    }
    #[doc = "Priority 18"]
    #[inline(always)]
    pub fn p18(self) -> &'a mut W {
        self.variant(PRIORITY_A::P18)
    }
    #[doc = "Priority 19"]
    #[inline(always)]
    pub fn p19(self) -> &'a mut W {
        self.variant(PRIORITY_A::P19)
    }
    #[doc = "Priority 20"]
    #[inline(always)]
    pub fn p20(self) -> &'a mut W {
        self.variant(PRIORITY_A::P20)
    }
    #[doc = "Priority 21"]
    #[inline(always)]
    pub fn p21(self) -> &'a mut W {
        self.variant(PRIORITY_A::P21)
    }
    #[doc = "Priority 22"]
    #[inline(always)]
    pub fn p22(self) -> &'a mut W {
        self.variant(PRIORITY_A::P22)
    }
    #[doc = "Priority 23"]
    #[inline(always)]
    pub fn p23(self) -> &'a mut W {
        self.variant(PRIORITY_A::P23)
    }
    #[doc = "Priority 24"]
    #[inline(always)]
    pub fn p24(self) -> &'a mut W {
        self.variant(PRIORITY_A::P24)
    }
    #[doc = "Priority 25"]
    #[inline(always)]
    pub fn p25(self) -> &'a mut W {
        self.variant(PRIORITY_A::P25)
    }
    #[doc = "Priority 26"]
    #[inline(always)]
    pub fn p26(self) -> &'a mut W {
        self.variant(PRIORITY_A::P26)
    }
    #[doc = "Priority 27"]
    #[inline(always)]
    pub fn p27(self) -> &'a mut W {
        self.variant(PRIORITY_A::P27)
    }
    #[doc = "Priority 28"]
    #[inline(always)]
    pub fn p28(self) -> &'a mut W {
        self.variant(PRIORITY_A::P28)
    }
    #[doc = "Priority 29"]
    #[inline(always)]
    pub fn p29(self) -> &'a mut W {
        self.variant(PRIORITY_A::P29)
    }
    #[doc = "Priority 30"]
    #[inline(always)]
    pub fn p30(self) -> &'a mut W {
        self.variant(PRIORITY_A::P30)
    }
    #[doc = "Priority 31"]
    #[inline(always)]
    pub fn p31(self) -> &'a mut W {
        self.variant(PRIORITY_A::P31)
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<0> {
        PRIORITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio](index.html) module"]
pub struct PRIO_SPEC;
impl crate::RegisterSpec for PRIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prio::R](R) reader structure"]
impl crate::Readable for PRIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prio::W](W) writer structure"]
impl crate::Writable for PRIO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets prio[%s] to value 0"]
impl crate::Resettable for PRIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
