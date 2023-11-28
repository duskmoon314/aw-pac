#[doc = "Register `gp_cdata` reader"]
pub type R = crate::R<GP_CDATA_SPEC>;
#[doc = "Register `gp_cdata` writer"]
pub type W = crate::W<GP_CDATA_SPEC>;
#[doc = "Field `gp_cdata` reader - GPADC Calibration Data"]
pub type GP_CDATA_R = crate::FieldReader<u16>;
#[doc = "Field `gp_cdata` writer - GPADC Calibration Data"]
pub type GP_CDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - GPADC Calibration Data"]
    #[inline(always)]
    pub fn gp_cdata(&self) -> GP_CDATA_R {
        GP_CDATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - GPADC Calibration Data"]
    #[inline(always)]
    #[must_use]
    pub fn gp_cdata(&mut self) -> GP_CDATA_W<GP_CDATA_SPEC> {
        GP_CDATA_W::new(self, 0)
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
#[doc = "GPADC Calibration Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_cdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_cdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GP_CDATA_SPEC;
impl crate::RegisterSpec for GP_CDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp_cdata::R`](R) reader structure"]
impl crate::Readable for GP_CDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gp_cdata::W`](W) writer structure"]
impl crate::Writable for GP_CDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gp_cdata to value 0"]
impl crate::Resettable for GP_CDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
