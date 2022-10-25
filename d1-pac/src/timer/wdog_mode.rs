#[doc = "Register `wdog_mode` reader"]
pub struct R(crate::R<WDOG_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOG_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOG_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wdog_mode` writer"]
pub struct W(crate::W<WDOG_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOG_MODE_SPEC>;
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
impl From<crate::W<WDOG_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOG_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wdog_en` reader - Watchdog Enable"]
pub type WDOG_EN_R = crate::BitReader<WDOG_EN_A>;
#[doc = "Watchdog Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDOG_EN_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<WDOG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl WDOG_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG_EN_A {
        match self.bits {
            false => WDOG_EN_A::NO_EFFECT,
            true => WDOG_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == WDOG_EN_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WDOG_EN_A::ENABLE
    }
}
#[doc = "Field `wdog_en` writer - Watchdog Enable"]
pub type WDOG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDOG_MODE_SPEC, WDOG_EN_A, O>;
impl<'a, const O: u8> WDOG_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(WDOG_EN_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WDOG_EN_A::ENABLE)
    }
}
#[doc = "Field `wdog_intv_value` reader - Watchdog Interval Value"]
pub type WDOG_INTV_VALUE_R = crate::FieldReader<u8, WDOG_INTV_VALUE_A>;
#[doc = "Watchdog Interval Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDOG_INTV_VALUE_A {
    #[doc = "0: `0`"]
    C16000 = 0,
    #[doc = "1: `1`"]
    C32000 = 1,
    #[doc = "2: `10`"]
    C64000 = 2,
    #[doc = "3: `11`"]
    C96000 = 3,
    #[doc = "4: `100`"]
    C128000 = 4,
    #[doc = "5: `101`"]
    C160000 = 5,
    #[doc = "6: `110`"]
    C192000 = 6,
    #[doc = "7: `111`"]
    C256000 = 7,
    #[doc = "8: `1000`"]
    C320000 = 8,
    #[doc = "9: `1001`"]
    C384000 = 9,
    #[doc = "10: `1010`"]
    C448000 = 10,
    #[doc = "11: `1011`"]
    C512000 = 11,
}
impl From<WDOG_INTV_VALUE_A> for u8 {
    #[inline(always)]
    fn from(variant: WDOG_INTV_VALUE_A) -> Self {
        variant as _
    }
}
impl WDOG_INTV_VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDOG_INTV_VALUE_A> {
        match self.bits {
            0 => Some(WDOG_INTV_VALUE_A::C16000),
            1 => Some(WDOG_INTV_VALUE_A::C32000),
            2 => Some(WDOG_INTV_VALUE_A::C64000),
            3 => Some(WDOG_INTV_VALUE_A::C96000),
            4 => Some(WDOG_INTV_VALUE_A::C128000),
            5 => Some(WDOG_INTV_VALUE_A::C160000),
            6 => Some(WDOG_INTV_VALUE_A::C192000),
            7 => Some(WDOG_INTV_VALUE_A::C256000),
            8 => Some(WDOG_INTV_VALUE_A::C320000),
            9 => Some(WDOG_INTV_VALUE_A::C384000),
            10 => Some(WDOG_INTV_VALUE_A::C448000),
            11 => Some(WDOG_INTV_VALUE_A::C512000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `C16000`"]
    #[inline(always)]
    pub fn is_c16000(&self) -> bool {
        *self == WDOG_INTV_VALUE_A::C16000
    }
    #[doc = "Checks if the value of the field is `C32000`"]
    #[inline(always)]
    pub fn is_c32000(&self) -> bool {
        *self == WDOG_INTV_VALUE_A::C32000
    }
    #[doc = "Checks if the value of the field is `C64000`"]
    #[inline(always)]
    pub fn is_c64000(&self) -> bool {
        *self == WDOG_INTV_VALUE_A::C64000
    }
    #[doc = "Checks if the value of the field is `C96000`"]
    #[inline(always)]
    pub fn is_c96000(&self) -> bool {
        *self == WDOG_INTV_VALUE_A::C96000
    }
    #[doc = "Checks if the value of the field is `C128000`"]
    #[inline(always)]
    pub fn is_c128000(&self) -> bool {
        *self == WDOG_INTV_VALUE_A::C128000
    }
    #[doc = "Checks if the value of the field is `C160000`"]
    #[inline(always)]
    pub fn is_c160000(&self) -> bool {
        *self == WDOG_INTV_VALUE_A::C160000
    }
    #[doc = "Checks if the value of the field is `C192000`"]
    #[inline(always)]
    pub fn is_c192000(&self) -> bool {
        *self == WDOG_INTV_VALUE_A::C192000
    }
    #[doc = "Checks if the value of the field is `C256000`"]
    #[inline(always)]
    pub fn is_c256000(&self) -> bool {
        *self == WDOG_INTV_VALUE_A::C256000
    }
    #[doc = "Checks if the value of the field is `C320000`"]
    #[inline(always)]
    pub fn is_c320000(&self) -> bool {
        *self == WDOG_INTV_VALUE_A::C320000
    }
    #[doc = "Checks if the value of the field is `C384000`"]
    #[inline(always)]
    pub fn is_c384000(&self) -> bool {
        *self == WDOG_INTV_VALUE_A::C384000
    }
    #[doc = "Checks if the value of the field is `C448000`"]
    #[inline(always)]
    pub fn is_c448000(&self) -> bool {
        *self == WDOG_INTV_VALUE_A::C448000
    }
    #[doc = "Checks if the value of the field is `C512000`"]
    #[inline(always)]
    pub fn is_c512000(&self) -> bool {
        *self == WDOG_INTV_VALUE_A::C512000
    }
}
#[doc = "Field `wdog_intv_value` writer - Watchdog Interval Value"]
pub type WDOG_INTV_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WDOG_MODE_SPEC, u8, WDOG_INTV_VALUE_A, 4, O>;
impl<'a, const O: u8> WDOG_INTV_VALUE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn c16000(self) -> &'a mut W {
        self.variant(WDOG_INTV_VALUE_A::C16000)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn c32000(self) -> &'a mut W {
        self.variant(WDOG_INTV_VALUE_A::C32000)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn c64000(self) -> &'a mut W {
        self.variant(WDOG_INTV_VALUE_A::C64000)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn c96000(self) -> &'a mut W {
        self.variant(WDOG_INTV_VALUE_A::C96000)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn c128000(self) -> &'a mut W {
        self.variant(WDOG_INTV_VALUE_A::C128000)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn c160000(self) -> &'a mut W {
        self.variant(WDOG_INTV_VALUE_A::C160000)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn c192000(self) -> &'a mut W {
        self.variant(WDOG_INTV_VALUE_A::C192000)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn c256000(self) -> &'a mut W {
        self.variant(WDOG_INTV_VALUE_A::C256000)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn c320000(self) -> &'a mut W {
        self.variant(WDOG_INTV_VALUE_A::C320000)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn c384000(self) -> &'a mut W {
        self.variant(WDOG_INTV_VALUE_A::C384000)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn c448000(self) -> &'a mut W {
        self.variant(WDOG_INTV_VALUE_A::C448000)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn c512000(self) -> &'a mut W {
        self.variant(WDOG_INTV_VALUE_A::C512000)
    }
}
#[doc = "Field `key_field` writer - Key Field"]
pub type KEY_FIELD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WDOG_MODE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Watchdog Enable"]
    #[inline(always)]
    pub fn wdog_en(&self) -> WDOG_EN_R {
        WDOG_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Watchdog Interval Value"]
    #[inline(always)]
    pub fn wdog_intv_value(&self) -> WDOG_INTV_VALUE_R {
        WDOG_INTV_VALUE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdog_en(&mut self) -> WDOG_EN_W<0> {
        WDOG_EN_W::new(self)
    }
    #[doc = "Bits 4:7 - Watchdog Interval Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdog_intv_value(&mut self) -> WDOG_INTV_VALUE_W<4> {
        WDOG_INTV_VALUE_W::new(self)
    }
    #[doc = "Bits 16:31 - Key Field"]
    #[inline(always)]
    #[must_use]
    pub fn key_field(&mut self) -> KEY_FIELD_W<16> {
        KEY_FIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_mode](index.html) module"]
pub struct WDOG_MODE_SPEC;
impl crate::RegisterSpec for WDOG_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdog_mode::R](R) reader structure"]
impl crate::Readable for WDOG_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdog_mode::W](W) writer structure"]
impl crate::Writable for WDOG_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wdog_mode to value 0"]
impl crate::Resettable for WDOG_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
