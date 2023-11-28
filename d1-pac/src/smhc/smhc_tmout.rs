#[doc = "Register `smhc_tmout` reader"]
pub type R = crate::R<SMHC_TMOUT_SPEC>;
#[doc = "Register `smhc_tmout` writer"]
pub type W = crate::W<SMHC_TMOUT_SPEC>;
#[doc = "Field `rto_lmt` reader - Response Timeout Limit"]
pub type RTO_LMT_R = crate::FieldReader;
#[doc = "Field `rto_lmt` writer - Response Timeout Limit"]
pub type RTO_LMT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `dto_lmt` reader - Data Iimeout Limit"]
pub type DTO_LMT_R = crate::FieldReader<u32>;
#[doc = "Field `dto_lmt` writer - Data Iimeout Limit"]
pub type DTO_LMT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - Response Timeout Limit"]
    #[inline(always)]
    pub fn rto_lmt(&self) -> RTO_LMT_R {
        RTO_LMT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Data Iimeout Limit"]
    #[inline(always)]
    pub fn dto_lmt(&self) -> DTO_LMT_R {
        DTO_LMT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - Response Timeout Limit"]
    #[inline(always)]
    #[must_use]
    pub fn rto_lmt(&mut self) -> RTO_LMT_W<SMHC_TMOUT_SPEC> {
        RTO_LMT_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - Data Iimeout Limit"]
    #[inline(always)]
    #[must_use]
    pub fn dto_lmt(&mut self) -> DTO_LMT_W<SMHC_TMOUT_SPEC> {
        DTO_LMT_W::new(self, 8)
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
#[doc = "Time Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_tmout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_tmout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_TMOUT_SPEC;
impl crate::RegisterSpec for SMHC_TMOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_tmout::R`](R) reader structure"]
impl crate::Readable for SMHC_TMOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_tmout::W`](W) writer structure"]
impl crate::Writable for SMHC_TMOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_tmout to value 0"]
impl crate::Resettable for SMHC_TMOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
