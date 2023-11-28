#[doc = "Register `gp_fifo_data` reader"]
pub type R = crate::R<GP_FIFO_DATA_SPEC>;
#[doc = "Register `gp_fifo_data` writer"]
pub type W = crate::W<GP_FIFO_DATA_SPEC>;
#[doc = "Field `gp_fifo_data` reader - GPADC Data in FIFO"]
pub type GP_FIFO_DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - GPADC Data in FIFO"]
    #[inline(always)]
    pub fn gp_fifo_data(&self) -> GP_FIFO_DATA_R {
        GP_FIFO_DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
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
#[doc = "GPADC FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_fifo_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_fifo_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GP_FIFO_DATA_SPEC;
impl crate::RegisterSpec for GP_FIFO_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp_fifo_data::R`](R) reader structure"]
impl crate::Readable for GP_FIFO_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gp_fifo_data::W`](W) writer structure"]
impl crate::Writable for GP_FIFO_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gp_fifo_data to value 0"]
impl crate::Resettable for GP_FIFO_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
