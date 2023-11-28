#[doc = "Register `dlh` reader"]
pub type R = crate::R<DLH_SPEC>;
#[doc = "Register `dlh` writer"]
pub type W = crate::W<DLH_SPEC>;
#[doc = "Field `dlh` reader - "]
pub type DLH_R = crate::FieldReader;
#[doc = "Field `dlh` writer - "]
pub type DLH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dlh(&self) -> DLH_R {
        DLH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn dlh(&mut self) -> DLH_W<DLH_SPEC> {
        DLH_W::new(self, 0)
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
#[doc = "UART Divisor Latch High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLH_SPEC;
impl crate::RegisterSpec for DLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlh::R`](R) reader structure"]
impl crate::Readable for DLH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dlh::W`](W) writer structure"]
impl crate::Writable for DLH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dlh to value 0"]
impl crate::Resettable for DLH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
