#[doc = "Register `rtc_hh_mm_ss` reader"]
pub type R = crate::R<RTC_HH_MM_SS_SPEC>;
#[doc = "Register `rtc_hh_mm_ss` writer"]
pub type W = crate::W<RTC_HH_MM_SS_SPEC>;
#[doc = "Field `second` reader - Set second Range from 0 to 59."]
pub type SECOND_R = crate::FieldReader;
#[doc = "Field `second` writer - Set second Range from 0 to 59."]
pub type SECOND_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `minute` reader - Set minute Range from 0 to 59."]
pub type MINUTE_R = crate::FieldReader;
#[doc = "Field `minute` writer - Set minute Range from 0 to 59."]
pub type MINUTE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `hour` reader - Set hour Range from 0 to 23."]
pub type HOUR_R = crate::FieldReader;
#[doc = "Field `hour` writer - Set hour Range from 0 to 23."]
pub type HOUR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - Set second Range from 0 to 59."]
    #[inline(always)]
    pub fn second(&self) -> SECOND_R {
        SECOND_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Set minute Range from 0 to 59."]
    #[inline(always)]
    pub fn minute(&self) -> MINUTE_R {
        MINUTE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Set hour Range from 0 to 23."]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Set second Range from 0 to 59."]
    #[inline(always)]
    #[must_use]
    pub fn second(&mut self) -> SECOND_W<RTC_HH_MM_SS_SPEC> {
        SECOND_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Set minute Range from 0 to 59."]
    #[inline(always)]
    #[must_use]
    pub fn minute(&mut self) -> MINUTE_W<RTC_HH_MM_SS_SPEC> {
        MINUTE_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Set hour Range from 0 to 23."]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HOUR_W<RTC_HH_MM_SS_SPEC> {
        HOUR_W::new(self, 16)
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
#[doc = "RTC Hour-Minute-Second Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_hh_mm_ss::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_hh_mm_ss::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_HH_MM_SS_SPEC;
impl crate::RegisterSpec for RTC_HH_MM_SS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_hh_mm_ss::R`](R) reader structure"]
impl crate::Readable for RTC_HH_MM_SS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_hh_mm_ss::W`](W) writer structure"]
impl crate::Writable for RTC_HH_MM_SS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtc_hh_mm_ss to value 0"]
impl crate::Resettable for RTC_HH_MM_SS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
