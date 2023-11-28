#[doc = "Register `twi_data` reader"]
pub type R = crate::R<TWI_DATA_SPEC>;
#[doc = "Register `twi_data` writer"]
pub type W = crate::W<TWI_DATA_SPEC>;
#[doc = "Field `data` reader - Data byte transmitted or received"]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `data` writer - Data byte transmitted or received"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data byte transmitted or received"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte transmitted or received"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<TWI_DATA_SPEC> {
        DATA_W::new(self, 0)
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
#[doc = "TWI Data Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_DATA_SPEC;
impl crate::RegisterSpec for TWI_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_data::R`](R) reader structure"]
impl crate::Readable for TWI_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_data::W`](W) writer structure"]
impl crate::Writable for TWI_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_data to value 0"]
impl crate::Resettable for TWI_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
