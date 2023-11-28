#[doc = "Register `pgr%s` reader"]
pub type R = crate::R<PGR_SPEC>;
#[doc = "Register `pgr%s` writer"]
pub type W = crate::W<PGR_SPEC>;
#[doc = "Field `cs` reader - If bit\\[i\\] is set, the PWM i is selected as one channel of PWM Group\\[g\\]."]
pub type CS_R = crate::FieldReader<u16>;
#[doc = "Field `cs` writer - If bit\\[i\\] is set, the PWM i is selected as one channel of PWM Group\\[g\\]."]
pub type CS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `en` reader - PWM Group Enable."]
pub type EN_R = crate::BitReader;
#[doc = "Field `en` writer - PWM Group Enable."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `start` reader - The PWM channels selected in CS start to output PWM waveform at the same time."]
pub type START_R = crate::BitReader;
#[doc = "Field `start` writer - The PWM channels selected in CS start to output PWM waveform at the same time."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - If bit\\[i\\] is set, the PWM i is selected as one channel of PWM Group\\[g\\]."]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - PWM Group Enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The PWM channels selected in CS start to output PWM waveform at the same time."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit\\[i\\] is set, the PWM i is selected as one channel of PWM Group\\[g\\]."]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<PGR_SPEC> {
        CS_W::new(self, 0)
    }
    #[doc = "Bit 16 - PWM Group Enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<PGR_SPEC> {
        EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - The PWM channels selected in CS start to output PWM waveform at the same time."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<PGR_SPEC> {
        START_W::new(self, 17)
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
#[doc = "PWM Group\\[g\\] Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PGR_SPEC;
impl crate::RegisterSpec for PGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pgr::R`](R) reader structure"]
impl crate::Readable for PGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pgr::W`](W) writer structure"]
impl crate::Writable for PGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pgr%s to value 0"]
impl crate::Resettable for PGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
