#[doc = "Register `ac_dac_dap_ctr` reader"]
pub struct R(crate::R<AC_DAC_DAP_CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_DAC_DAP_CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_DAC_DAP_CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_DAC_DAP_CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_dac_dap_ctr` writer"]
pub struct W(crate::W<AC_DAC_DAP_CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_DAC_DAP_CTR_SPEC>;
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
impl From<crate::W<AC_DAC_DAP_CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_DAC_DAP_CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ddap_hpf_en` reader - HPF enable control"]
pub type DDAP_HPF_EN_R = crate::BitReader<DDAP_HPF_EN_A>;
#[doc = "HPF enable control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDAP_HPF_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DDAP_HPF_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DDAP_HPF_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DDAP_HPF_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDAP_HPF_EN_A {
        match self.bits {
            false => DDAP_HPF_EN_A::DISABLED,
            true => DDAP_HPF_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDAP_HPF_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DDAP_HPF_EN_A::ENABLED
    }
}
#[doc = "Field `ddap_hpf_en` writer - HPF enable control"]
pub type DDAP_HPF_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_DAC_DAP_CTR_SPEC, DDAP_HPF_EN_A, O>;
impl<'a, const O: u8> DDAP_HPF_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDAP_HPF_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DDAP_HPF_EN_A::ENABLED)
    }
}
#[doc = "Field `ddap_drc_en` reader - DRC enable control"]
pub type DDAP_DRC_EN_R = crate::BitReader<DDAP_DRC_EN_A>;
#[doc = "DRC enable control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDAP_DRC_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DDAP_DRC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DDAP_DRC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DDAP_DRC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDAP_DRC_EN_A {
        match self.bits {
            false => DDAP_DRC_EN_A::DISABLED,
            true => DDAP_DRC_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDAP_DRC_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DDAP_DRC_EN_A::ENABLED
    }
}
#[doc = "Field `ddap_drc_en` writer - DRC enable control"]
pub type DDAP_DRC_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_DAC_DAP_CTR_SPEC, DDAP_DRC_EN_A, O>;
impl<'a, const O: u8> DDAP_DRC_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDAP_DRC_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DDAP_DRC_EN_A::ENABLED)
    }
}
#[doc = "Field `ddap_en` reader - DAP for DRC enable"]
pub type DDAP_EN_R = crate::BitReader<DDAP_EN_A>;
#[doc = "DAP for DRC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDAP_EN_A {
    #[doc = "0: Bypassed"]
    BYPASSED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DDAP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DDAP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DDAP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDAP_EN_A {
        match self.bits {
            false => DDAP_EN_A::BYPASSED,
            true => DDAP_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == DDAP_EN_A::BYPASSED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DDAP_EN_A::ENABLED
    }
}
#[doc = "Field `ddap_en` writer - DAP for DRC enable"]
pub type DDAP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_DAC_DAP_CTR_SPEC, DDAP_EN_A, O>;
impl<'a, const O: u8> DDAP_EN_W<'a, O> {
    #[doc = "Bypassed"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(DDAP_EN_A::BYPASSED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DDAP_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 28 - HPF enable control"]
    #[inline(always)]
    pub fn ddap_hpf_en(&self) -> DDAP_HPF_EN_R {
        DDAP_HPF_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DRC enable control"]
    #[inline(always)]
    pub fn ddap_drc_en(&self) -> DDAP_DRC_EN_R {
        DDAP_DRC_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - DAP for DRC enable"]
    #[inline(always)]
    pub fn ddap_en(&self) -> DDAP_EN_R {
        DDAP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - HPF enable control"]
    #[inline(always)]
    #[must_use]
    pub fn ddap_hpf_en(&mut self) -> DDAP_HPF_EN_W<28> {
        DDAP_HPF_EN_W::new(self)
    }
    #[doc = "Bit 29 - DRC enable control"]
    #[inline(always)]
    #[must_use]
    pub fn ddap_drc_en(&mut self) -> DDAP_DRC_EN_W<29> {
        DDAP_DRC_EN_W::new(self)
    }
    #[doc = "Bit 31 - DAP for DRC enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddap_en(&mut self) -> DDAP_EN_W<31> {
        DDAP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC DAP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_dac_dap_ctr](index.html) module"]
pub struct AC_DAC_DAP_CTR_SPEC;
impl crate::RegisterSpec for AC_DAC_DAP_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_dac_dap_ctr::R](R) reader structure"]
impl crate::Readable for AC_DAC_DAP_CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_dac_dap_ctr::W](W) writer structure"]
impl crate::Writable for AC_DAC_DAP_CTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_dap_ctr to value 0"]
impl crate::Resettable for AC_DAC_DAP_CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
