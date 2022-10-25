#[doc = "Register `fsin_extcfg` reader"]
pub struct R(crate::R<FSIN_EXTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSIN_EXTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSIN_EXTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSIN_EXTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fsin_extcfg` writer"]
pub struct W(crate::W<FSIN_EXTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSIN_EXTCFG_SPEC>;
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
impl From<crate::W<FSIN_EXTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSIN_EXTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cyclenum` reader - The Cycle Number of Pulse Extend.\n\nThe cycle is BCLK and is at least 1."]
pub type CYCLENUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cyclenum` writer - The Cycle Number of Pulse Extend.\n\nThe cycle is BCLK and is at least 1."]
pub type CYCLENUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSIN_EXTCFG_SPEC, u16, u16, 16, O>;
#[doc = "Field `extend` reader - Extend the bit when using ASRC."]
pub type EXTEND_R = crate::BitReader<EXTEND_A>;
#[doc = "Extend the bit when using ASRC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTEND_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<EXTEND_A> for bool {
    #[inline(always)]
    fn from(variant: EXTEND_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTEND_A {
        match self.bits {
            false => EXTEND_A::DISABLE,
            true => EXTEND_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EXTEND_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EXTEND_A::ENABLE
    }
}
#[doc = "Field `extend` writer - Extend the bit when using ASRC."]
pub type EXTEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSIN_EXTCFG_SPEC, EXTEND_A, O>;
impl<'a, const O: u8> EXTEND_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EXTEND_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EXTEND_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:15 - The Cycle Number of Pulse Extend.\n\nThe cycle is BCLK and is at least 1."]
    #[inline(always)]
    pub fn cyclenum(&self) -> CYCLENUM_R {
        CYCLENUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Extend the bit when using ASRC."]
    #[inline(always)]
    pub fn extend(&self) -> EXTEND_R {
        EXTEND_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - The Cycle Number of Pulse Extend.\n\nThe cycle is BCLK and is at least 1."]
    #[inline(always)]
    #[must_use]
    pub fn cyclenum(&mut self) -> CYCLENUM_W<0> {
        CYCLENUM_W::new(self)
    }
    #[doc = "Bit 16 - Extend the bit when using ASRC."]
    #[inline(always)]
    #[must_use]
    pub fn extend(&mut self) -> EXTEND_W<16> {
        EXTEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASRC Input Sample Pulse Extend Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsin_extcfg](index.html) module"]
pub struct FSIN_EXTCFG_SPEC;
impl crate::RegisterSpec for FSIN_EXTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsin_extcfg::R](R) reader structure"]
impl crate::Readable for FSIN_EXTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsin_extcfg::W](W) writer structure"]
impl crate::Writable for FSIN_EXTCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fsin_extcfg to value 0"]
impl crate::Resettable for FSIN_EXTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
