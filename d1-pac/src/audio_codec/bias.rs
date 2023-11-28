#[doc = "Register `bias` reader"]
pub type R = crate::R<BIAS_SPEC>;
#[doc = "Register `bias` writer"]
pub type W = crate::W<BIAS_SPEC>;
#[doc = "Field `biasdata` reader - Bias Current Register Setting Data"]
pub type BIASDATA_R = crate::FieldReader;
#[doc = "Field `biasdata` writer - Bias Current Register Setting Data"]
pub type BIASDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bias Current Register Setting Data"]
    #[inline(always)]
    pub fn biasdata(&self) -> BIASDATA_R {
        BIASDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bias Current Register Setting Data"]
    #[inline(always)]
    #[must_use]
    pub fn biasdata(&mut self) -> BIASDATA_W<BIAS_SPEC> {
        BIASDATA_W::new(self, 0)
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
#[doc = "BIAS Analog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bias::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bias::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIAS_SPEC;
impl crate::RegisterSpec for BIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bias::R`](R) reader structure"]
impl crate::Readable for BIAS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bias::W`](W) writer structure"]
impl crate::Writable for BIAS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bias to value 0x80"]
impl crate::Resettable for BIAS_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
