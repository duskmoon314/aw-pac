#[doc = "Register `cir_tmcr` reader"]
pub type R = crate::R<CIR_TMCR_SPEC>;
#[doc = "Register `cir_tmcr` writer"]
pub type W = crate::W<CIR_TMCR_SPEC>;
#[doc = "Field `rfmc` reader - Reference Frequency of modulated carrier.\n\nReference Frequency of modulated carrier based on a division of a fixed functional clock (FCLK). The range of the modulated carrier is usually 30 kHz to 60 kHz. Most consumer electronics is 38 kHz.\n\nThe default modulated carrier is 38 kHz when FCLK is 12 MHz.\n\nRFMC= FCLK/((N+1)*(DRMC+2))."]
pub type RFMC_R = crate::FieldReader;
#[doc = "Field `rfmc` writer - Reference Frequency of modulated carrier.\n\nReference Frequency of modulated carrier based on a division of a fixed functional clock (FCLK). The range of the modulated carrier is usually 30 kHz to 60 kHz. Most consumer electronics is 38 kHz.\n\nThe default modulated carrier is 38 kHz when FCLK is 12 MHz.\n\nRFMC= FCLK/((N+1)*(DRMC+2))."]
pub type RFMC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    pub fn rfmc(&mut self) -> RFMC_W<CIR_TMCR_SPEC> {
        RFMC_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CIR Transmit Modulation Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_tmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_tmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIR_TMCR_SPEC;
impl crate::RegisterSpec for CIR_TMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir_tmcr::R`](R) reader structure"]
impl crate::Readable for CIR_TMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cir_tmcr::W`](W) writer structure"]
impl crate::Writable for CIR_TMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_tmcr to value 0x9e"]
impl crate::Resettable for CIR_TMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x9e;
}
