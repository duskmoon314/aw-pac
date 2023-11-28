#[doc = "Register `alarm0_day_set` reader"]
pub type R = crate::R<ALARM0_DAY_SET_SPEC>;
#[doc = "Register `alarm0_day_set` writer"]
pub type W = crate::W<ALARM0_DAY_SET_SPEC>;
#[doc = "Field `alarm0_counter` reader - Alarm 0 Counter is based on Day."]
pub type ALARM0_COUNTER_R = crate::FieldReader<u16>;
#[doc = "Field `alarm0_counter` writer - Alarm 0 Counter is based on Day."]
pub type ALARM0_COUNTER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Alarm 0 Counter is based on Day."]
    #[inline(always)]
    pub fn alarm0_counter(&self) -> ALARM0_COUNTER_R {
        ALARM0_COUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Alarm 0 Counter is based on Day."]
    #[inline(always)]
    #[must_use]
    pub fn alarm0_counter(&mut self) -> ALARM0_COUNTER_W<ALARM0_DAY_SET_SPEC> {
        ALARM0_COUNTER_W::new(self, 0)
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
#[doc = "Alarm 0 Day Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm0_day_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0_day_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARM0_DAY_SET_SPEC;
impl crate::RegisterSpec for ALARM0_DAY_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm0_day_set::R`](R) reader structure"]
impl crate::Readable for ALARM0_DAY_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarm0_day_set::W`](W) writer structure"]
impl crate::Writable for ALARM0_DAY_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets alarm0_day_set to value 0"]
impl crate::Resettable for ALARM0_DAY_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
