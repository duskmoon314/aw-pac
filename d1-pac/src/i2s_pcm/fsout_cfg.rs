#[doc = "Register `fsout_cfg` reader"]
pub struct R(crate::R<FSOUT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSOUT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSOUT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSOUT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fsout_cfg` writer"]
pub struct W(crate::W<FSOUT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSOUT_CFG_SPEC>;
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
impl From<crate::W<FSOUT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSOUT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fsout_gate` reader - fsout Clock Gate Enable Control"]
pub type FSOUT_GATE_R = crate::BitReader<FSOUT_GATE_A>;
#[doc = "fsout Clock Gate Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSOUT_GATE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FSOUT_GATE_A> for bool {
    #[inline(always)]
    fn from(variant: FSOUT_GATE_A) -> Self {
        variant as u8 != 0
    }
}
impl FSOUT_GATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSOUT_GATE_A {
        match self.bits {
            false => FSOUT_GATE_A::DISABLE,
            true => FSOUT_GATE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FSOUT_GATE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FSOUT_GATE_A::ENABLE
    }
}
#[doc = "Field `fsout_gate` writer - fsout Clock Gate Enable Control"]
pub type FSOUT_GATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSOUT_CFG_SPEC, FSOUT_GATE_A, O>;
impl<'a, const O: u8> FSOUT_GATE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FSOUT_GATE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FSOUT_GATE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 20 - fsout Clock Gate Enable Control"]
    #[inline(always)]
    pub fn fsout_gate(&self) -> FSOUT_GATE_R {
        FSOUT_GATE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - fsout Clock Gate Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn fsout_gate(&mut self) -> FSOUT_GATE_W<20> {
        FSOUT_GATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASRC Out Sample Rate Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsout_cfg](index.html) module"]
pub struct FSOUT_CFG_SPEC;
impl crate::RegisterSpec for FSOUT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsout_cfg::R](R) reader structure"]
impl crate::Readable for FSOUT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsout_cfg::W](W) writer structure"]
impl crate::Writable for FSOUT_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fsout_cfg to value 0"]
impl crate::Resettable for FSOUT_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
