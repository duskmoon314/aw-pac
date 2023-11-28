#[doc = "Register `rtc_day` reader"]
pub type R = crate::R<RTC_DAY_SPEC>;
#[doc = "Register `rtc_day` writer"]
pub type W = crate::W<RTC_DAY_SPEC>;
#[doc = "Field `day` reader - Set Day Range from 0 to 65535."]
pub type DAY_R = crate::FieldReader<u16>;
#[doc = "Field `day` writer - Set Day Range from 0 to 65535."]
pub type DAY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Set Day Range from 0 to 65535."]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set Day Range from 0 to 65535."]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DAY_W<RTC_DAY_SPEC> {
        DAY_W::new(self, 0)
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
#[doc = "RTC Year-Month-Day Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_day::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_day::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_DAY_SPEC;
impl crate::RegisterSpec for RTC_DAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_day::R`](R) reader structure"]
impl crate::Readable for RTC_DAY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_day::W`](W) writer structure"]
impl crate::Writable for RTC_DAY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtc_day to value 0"]
impl crate::Resettable for RTC_DAY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
