#[doc = "Register `ths_cdata` reader"]
pub type R = crate::R<THS_CDATA_SPEC>;
#[doc = "Register `ths_cdata` writer"]
pub type W = crate::W<THS_CDATA_SPEC>;
#[doc = "Field `ths_cdata` reader - Thermal sensor calibration data"]
pub type THS_CDATA_R = crate::FieldReader<u16>;
#[doc = "Field `ths_cdata` writer - Thermal sensor calibration data"]
pub type THS_CDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Thermal sensor calibration data"]
    #[inline(always)]
    pub fn ths_cdata(&self) -> THS_CDATA_R {
        THS_CDATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Thermal sensor calibration data"]
    #[inline(always)]
    #[must_use]
    pub fn ths_cdata(&mut self) -> THS_CDATA_W<THS_CDATA_SPEC> {
        THS_CDATA_W::new(self, 0)
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
#[doc = "THS Calibration Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_cdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_cdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THS_CDATA_SPEC;
impl crate::RegisterSpec for THS_CDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ths_cdata::R`](R) reader structure"]
impl crate::Readable for THS_CDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ths_cdata::W`](W) writer structure"]
impl crate::Writable for THS_CDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ths_cdata to value 0x0800"]
impl crate::Resettable for THS_CDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
