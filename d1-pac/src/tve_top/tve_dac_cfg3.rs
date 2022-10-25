#[doc = "Register `tve_dac_cfg3` reader"]
pub struct R(crate::R<TVE_DAC_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_DAC_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_DAC_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_DAC_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_dac_cfg3` writer"]
pub struct W(crate::W<TVE_DAC_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_DAC_CFG3_SPEC>;
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
impl From<crate::W<TVE_DAC_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_DAC_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `force_data_en` reader - "]
pub type FORCE_DATA_EN_R = crate::BitReader<FORCE_DATA_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_DATA_EN_A {
    #[doc = "0: DAC input data from TVE"]
    TVE = 0,
    #[doc = "1: DAC input data from FORCE_DATA_SET"]
    FORCE_DATA_SET = 1,
}
impl From<FORCE_DATA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_DATA_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCE_DATA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE_DATA_EN_A {
        match self.bits {
            false => FORCE_DATA_EN_A::TVE,
            true => FORCE_DATA_EN_A::FORCE_DATA_SET,
        }
    }
    #[doc = "Checks if the value of the field is `TVE`"]
    #[inline(always)]
    pub fn is_tve(&self) -> bool {
        *self == FORCE_DATA_EN_A::TVE
    }
    #[doc = "Checks if the value of the field is `FORCE_DATA_SET`"]
    #[inline(always)]
    pub fn is_force_data_set(&self) -> bool {
        *self == FORCE_DATA_EN_A::FORCE_DATA_SET
    }
}
#[doc = "Field `force_data_en` writer - "]
pub type FORCE_DATA_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_DAC_CFG3_SPEC, FORCE_DATA_EN_A, O>;
impl<'a, const O: u8> FORCE_DATA_EN_W<'a, O> {
    #[doc = "DAC input data from TVE"]
    #[inline(always)]
    pub fn tve(self) -> &'a mut W {
        self.variant(FORCE_DATA_EN_A::TVE)
    }
    #[doc = "DAC input data from FORCE_DATA_SET"]
    #[inline(always)]
    pub fn force_data_set(self) -> &'a mut W {
        self.variant(FORCE_DATA_EN_A::FORCE_DATA_SET)
    }
}
#[doc = "Field `force_data_set` reader - Force DAC input data"]
pub type FORCE_DATA_SET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `force_data_set` writer - Force DAC input data"]
pub type FORCE_DATA_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_DAC_CFG3_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn force_data_en(&self) -> FORCE_DATA_EN_R {
        FORCE_DATA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:25 - Force DAC input data"]
    #[inline(always)]
    pub fn force_data_set(&self) -> FORCE_DATA_SET_R {
        FORCE_DATA_SET_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn force_data_en(&mut self) -> FORCE_DATA_EN_W<0> {
        FORCE_DATA_EN_W::new(self)
    }
    #[doc = "Bits 16:25 - Force DAC input data"]
    #[inline(always)]
    #[must_use]
    pub fn force_data_set(&mut self) -> FORCE_DATA_SET_W<16> {
        FORCE_DATA_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder DAC CFG2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_dac_cfg3](index.html) module"]
pub struct TVE_DAC_CFG3_SPEC;
impl crate::RegisterSpec for TVE_DAC_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_dac_cfg3::R](R) reader structure"]
impl crate::Readable for TVE_DAC_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_dac_cfg3::W](W) writer structure"]
impl crate::Writable for TVE_DAC_CFG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_dac_cfg3 to value 0"]
impl crate::Resettable for TVE_DAC_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
