#[doc = "Register `asrcmancfg` reader"]
pub struct R(crate::R<ASRCMANCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASRCMANCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASRCMANCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASRCMANCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `asrcmancfg` writer"]
pub struct W(crate::W<ASRCMANCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASRCMANCFG_SPEC>;
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
impl From<crate::W<ASRCMANCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASRCMANCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `asrc_ratio_value_manual_cfg` reader - ASRD Ration Value Manual Configure"]
pub type ASRC_RATIO_VALUE_MANUAL_CFG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `asrc_ratio_value_manual_cfg` writer - ASRD Ration Value Manual Configure"]
pub type ASRC_RATIO_VALUE_MANUAL_CFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ASRCMANCFG_SPEC, u32, u32, 26, O>;
#[doc = "Field `asrc_ratio_manual_en` reader - Manual Configuration of ASRC Ratio Enable"]
pub type ASRC_RATIO_MANUAL_EN_R = crate::BitReader<ASRC_RATIO_MANUAL_EN_A>;
#[doc = "Manual Configuration of ASRC Ratio Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASRC_RATIO_MANUAL_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ASRC_RATIO_MANUAL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_RATIO_MANUAL_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ASRC_RATIO_MANUAL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRC_RATIO_MANUAL_EN_A {
        match self.bits {
            false => ASRC_RATIO_MANUAL_EN_A::DISABLE,
            true => ASRC_RATIO_MANUAL_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ASRC_RATIO_MANUAL_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ASRC_RATIO_MANUAL_EN_A::ENABLE
    }
}
#[doc = "Field `asrc_ratio_manual_en` writer - Manual Configuration of ASRC Ratio Enable"]
pub type ASRC_RATIO_MANUAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ASRCMANCFG_SPEC, ASRC_RATIO_MANUAL_EN_A, O>;
impl<'a, const O: u8> ASRC_RATIO_MANUAL_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ASRC_RATIO_MANUAL_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ASRC_RATIO_MANUAL_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:25 - ASRD Ration Value Manual Configure"]
    #[inline(always)]
    pub fn asrc_ratio_value_manual_cfg(&self) -> ASRC_RATIO_VALUE_MANUAL_CFG_R {
        ASRC_RATIO_VALUE_MANUAL_CFG_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 31 - Manual Configuration of ASRC Ratio Enable"]
    #[inline(always)]
    pub fn asrc_ratio_manual_en(&self) -> ASRC_RATIO_MANUAL_EN_R {
        ASRC_RATIO_MANUAL_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - ASRD Ration Value Manual Configure"]
    #[inline(always)]
    #[must_use]
    pub fn asrc_ratio_value_manual_cfg(&mut self) -> ASRC_RATIO_VALUE_MANUAL_CFG_W<0> {
        ASRC_RATIO_VALUE_MANUAL_CFG_W::new(self)
    }
    #[doc = "Bit 31 - Manual Configuration of ASRC Ratio Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asrc_ratio_manual_en(&mut self) -> ASRC_RATIO_MANUAL_EN_W<31> {
        ASRC_RATIO_MANUAL_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASRC Manual Ratio Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asrcmancfg](index.html) module"]
pub struct ASRCMANCFG_SPEC;
impl crate::RegisterSpec for ASRCMANCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asrcmancfg::R](R) reader structure"]
impl crate::Readable for ASRCMANCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asrcmancfg::W](W) writer structure"]
impl crate::Writable for ASRCMANCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets asrcmancfg to value 0"]
impl crate::Resettable for ASRCMANCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
