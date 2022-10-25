#[doc = "Register `csic_prs_signal_sta` reader"]
pub struct R(crate::R<CSIC_PRS_SIGNAL_STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_PRS_SIGNAL_STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_PRS_SIGNAL_STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_PRS_SIGNAL_STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_prs_signal_sta` writer"]
pub struct W(crate::W<CSIC_PRS_SIGNAL_STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_PRS_SIGNAL_STA_SPEC>;
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
impl From<crate::W<CSIC_PRS_SIGNAL_STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_PRS_SIGNAL_STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `data_sta` reader - Indicates the Dn status (n=0-23), MSB for D23, LSB for D0"]
pub type DATA_STA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `pclk_sta` reader - Indicates the pclk status"]
pub type PCLK_STA_R = crate::BitReader<PCLK_STA_A>;
#[doc = "Indicates the pclk status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCLK_STA_A {
    #[doc = "0: low"]
    LOW = 0,
    #[doc = "1: high"]
    HIGH = 1,
}
impl From<PCLK_STA_A> for bool {
    #[inline(always)]
    fn from(variant: PCLK_STA_A) -> Self {
        variant as u8 != 0
    }
}
impl PCLK_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_STA_A {
        match self.bits {
            false => PCLK_STA_A::LOW,
            true => PCLK_STA_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PCLK_STA_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PCLK_STA_A::HIGH
    }
}
impl R {
    #[doc = "Bits 0:23 - Indicates the Dn status (n=0-23), MSB for D23, LSB for D0"]
    #[inline(always)]
    pub fn data_sta(&self) -> DATA_STA_R {
        DATA_STA_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Indicates the pclk status"]
    #[inline(always)]
    pub fn pclk_sta(&self) -> PCLK_STA_R {
        PCLK_STA_R::new(((self.bits >> 24) & 1) != 0)
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
#[doc = "CSIC Parser Signal Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_prs_signal_sta](index.html) module"]
pub struct CSIC_PRS_SIGNAL_STA_SPEC;
impl crate::RegisterSpec for CSIC_PRS_SIGNAL_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_prs_signal_sta::R](R) reader structure"]
impl crate::Readable for CSIC_PRS_SIGNAL_STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_prs_signal_sta::W](W) writer structure"]
impl crate::Writable for CSIC_PRS_SIGNAL_STA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_prs_signal_sta to value 0"]
impl crate::Resettable for CSIC_PRS_SIGNAL_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
