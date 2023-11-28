#[doc = "Register `tp_cali_data` reader"]
pub type R = crate::R<TP_CALI_DATA_SPEC>;
#[doc = "Register `tp_cali_data` writer"]
pub type W = crate::W<TP_CALI_DATA_SPEC>;
#[doc = "Field `tp_cdat` reader - TP Common Data"]
pub type TP_CDAT_R = crate::FieldReader<u16>;
#[doc = "Field `tp_cdat` writer - TP Common Data"]
pub type TP_CDAT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - TP Common Data"]
    #[inline(always)]
    pub fn tp_cdat(&self) -> TP_CDAT_R {
        TP_CDAT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - TP Common Data"]
    #[inline(always)]
    #[must_use]
    pub fn tp_cdat(&mut self) -> TP_CDAT_W<TP_CALI_DATA_SPEC> {
        TP_CDAT_W::new(self, 0)
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
#[doc = "TP Calibration Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tp_cali_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tp_cali_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TP_CALI_DATA_SPEC;
impl crate::RegisterSpec for TP_CALI_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tp_cali_data::R`](R) reader structure"]
impl crate::Readable for TP_CALI_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tp_cali_data::W`](W) writer structure"]
impl crate::Writable for TP_CALI_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tp_cali_data to value 0"]
impl crate::Resettable for TP_CALI_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
