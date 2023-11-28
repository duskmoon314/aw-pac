#[doc = "Register `twi_xaddr` reader"]
pub type R = crate::R<TWI_XADDR_SPEC>;
#[doc = "Register `twi_xaddr` writer"]
pub type W = crate::W<TWI_XADDR_SPEC>;
#[doc = "Field `slax` reader - Extend Slave Address\n\nSLAX\\[7:0\\]"]
pub type SLAX_R = crate::FieldReader;
#[doc = "Field `slax` writer - Extend Slave Address\n\nSLAX\\[7:0\\]"]
pub type SLAX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Extend Slave Address\n\nSLAX\\[7:0\\]"]
    #[inline(always)]
    pub fn slax(&self) -> SLAX_R {
        SLAX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Extend Slave Address\n\nSLAX\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn slax(&mut self) -> SLAX_W<TWI_XADDR_SPEC> {
        SLAX_W::new(self, 0)
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
#[doc = "TWI Extended Slave Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_xaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_xaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_XADDR_SPEC;
impl crate::RegisterSpec for TWI_XADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_xaddr::R`](R) reader structure"]
impl crate::Readable for TWI_XADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_xaddr::W`](W) writer structure"]
impl crate::Writable for TWI_XADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_xaddr to value 0"]
impl crate::Resettable for TWI_XADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
