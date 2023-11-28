#[doc = "Register `twi_srst` reader"]
pub type R = crate::R<TWI_SRST_SPEC>;
#[doc = "Register `twi_srst` writer"]
pub type W = crate::W<TWI_SRST_SPEC>;
#[doc = "Field `soft_rst` reader - Soft Reset"]
pub type SOFT_RST_R = crate::BitReader;
#[doc = "Field `soft_rst` writer - Soft Reset"]
pub type SOFT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Soft Reset"]
    #[inline(always)]
    pub fn soft_rst(&self) -> SOFT_RST_R {
        SOFT_RST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Soft Reset"]
    #[inline(always)]
    #[must_use]
    pub fn soft_rst(&mut self) -> SOFT_RST_W<TWI_SRST_SPEC> {
        SOFT_RST_W::new(self, 0)
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
#[doc = "TWI Software Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_srst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_srst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_SRST_SPEC;
impl crate::RegisterSpec for TWI_SRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_srst::R`](R) reader structure"]
impl crate::Readable for TWI_SRST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_srst::W`](W) writer structure"]
impl crate::Writable for TWI_SRST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_srst to value 0"]
impl crate::Resettable for TWI_SRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
