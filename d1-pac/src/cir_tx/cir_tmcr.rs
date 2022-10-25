#[doc = "Register `cir_tmcr` reader"]
pub struct R(crate::R<CIR_TMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_TMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_TMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_TMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cir_tmcr` writer"]
pub struct W(crate::W<CIR_TMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_TMCR_SPEC>;
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
impl From<crate::W<CIR_TMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_TMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rfmc` reader - Reference Frequency of modulated carrier.\n\nReference Frequency of modulated carrier based on a division of a fixed functional clock (FCLK). The range of the modulated carrier is usually 30 kHz to 60 kHz. Most consumer electronics is 38 kHz.\n\nThe default modulated carrier is 38 kHz when FCLK is 12 MHz.\n\nRFMC= FCLK/((N+1)*(DRMC+2))."]
pub type RFMC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rfmc` writer - Reference Frequency of modulated carrier.\n\nReference Frequency of modulated carrier based on a division of a fixed functional clock (FCLK). The range of the modulated carrier is usually 30 kHz to 60 kHz. Most consumer electronics is 38 kHz.\n\nThe default modulated carrier is 38 kHz when FCLK is 12 MHz.\n\nRFMC= FCLK/((N+1)*(DRMC+2))."]
pub type RFMC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIR_TMCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Reference Frequency of modulated carrier.\n\nReference Frequency of modulated carrier based on a division of a fixed functional clock (FCLK). The range of the modulated carrier is usually 30 kHz to 60 kHz. Most consumer electronics is 38 kHz.\n\nThe default modulated carrier is 38 kHz when FCLK is 12 MHz.\n\nRFMC= FCLK/((N+1)*(DRMC+2))."]
    #[inline(always)]
    pub fn rfmc(&self) -> RFMC_R {
        RFMC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reference Frequency of modulated carrier.\n\nReference Frequency of modulated carrier based on a division of a fixed functional clock (FCLK). The range of the modulated carrier is usually 30 kHz to 60 kHz. Most consumer electronics is 38 kHz.\n\nThe default modulated carrier is 38 kHz when FCLK is 12 MHz.\n\nRFMC= FCLK/((N+1)*(DRMC+2))."]
    #[inline(always)]
    #[must_use]
    pub fn rfmc(&mut self) -> RFMC_W<0> {
        RFMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR Transmit Modulation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_tmcr](index.html) module"]
pub struct CIR_TMCR_SPEC;
impl crate::RegisterSpec for CIR_TMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_tmcr::R](R) reader structure"]
impl crate::Readable for CIR_TMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_tmcr::W](W) writer structure"]
impl crate::Writable for CIR_TMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_tmcr to value 0x9e"]
impl crate::Resettable for CIR_TMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x9e;
}
