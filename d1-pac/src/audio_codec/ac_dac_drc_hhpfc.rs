#[doc = "Register `ac_dac_drc_hhpfc` reader"]
pub type R = crate::R<AC_DAC_DRC_HHPFC_SPEC>;
#[doc = "Register `ac_dac_drc_hhpfc` writer"]
pub type W = crate::W<AC_DAC_DRC_HHPFC_SPEC>;
#[doc = "Field `hhpfc` reader - HPF coefficient setting and the data is 3.24 format."]
pub type HHPFC_R = crate::FieldReader<u16>;
#[doc = "Field `hhpfc` writer - HPF coefficient setting and the data is 3.24 format."]
pub type HHPFC_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - HPF coefficient setting and the data is 3.24 format."]
    #[inline(always)]
    pub fn hhpfc(&self) -> HHPFC_R {
        HHPFC_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - HPF coefficient setting and the data is 3.24 format."]
    #[inline(always)]
    #[must_use]
    pub fn hhpfc(&mut self) -> HHPFC_W<AC_DAC_DRC_HHPFC_SPEC> {
        HHPFC_W::new(self, 0)
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
#[doc = "DAC DRC High HPF Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hhpfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hhpfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_DAC_DRC_HHPFC_SPEC;
impl crate::RegisterSpec for AC_DAC_DRC_HHPFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_dac_drc_hhpfc::R`](R) reader structure"]
impl crate::Readable for AC_DAC_DRC_HHPFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_dac_drc_hhpfc::W`](W) writer structure"]
impl crate::Writable for AC_DAC_DRC_HHPFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_drc_hhpfc to value 0xff"]
impl crate::Resettable for AC_DAC_DRC_HHPFC_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
