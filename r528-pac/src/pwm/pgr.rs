#[doc = "Register `PGR%s` reader"]
pub struct R(crate::R<PGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGR%s` writer"]
pub struct W(crate::W<PGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - The PWM channels selected in CS start to output PWM waveform at the same time."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - The PWM channels selected in CS start to output PWM waveform at the same time."]
pub type START_W<'a> = crate::BitWriter<'a, u32, PGR_SPEC, bool, 17>;
#[doc = "Field `EN` reader - PWM Group Enable."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - PWM Group Enable."]
pub type EN_W<'a> = crate::BitWriter<'a, u32, PGR_SPEC, bool, 16>;
#[doc = "Field `CS` reader - If bit\\[i\\]
is set, the PWM i is selected as one channel of PWM Group\\[g\\]."]
pub type CS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CS` writer - If bit\\[i\\]
is set, the PWM i is selected as one channel of PWM Group\\[g\\]."]
pub type CS_W<'a> = crate::FieldWriter<'a, u32, PGR_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bit 17 - The PWM channels selected in CS start to output PWM waveform at the same time."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - PWM Group Enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 0:15 - If bit\\[i\\]
is set, the PWM i is selected as one channel of PWM Group\\[g\\]."]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 17 - The PWM channels selected in CS start to output PWM waveform at the same time."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W::new(self)
    }
    #[doc = "Bit 16 - PWM Group Enable."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W::new(self)
    }
    #[doc = "Bits 0:15 - If bit\\[i\\]
is set, the PWM i is selected as one channel of PWM Group\\[g\\]."]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Group\\[g\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgr](index.html) module"]
pub struct PGR_SPEC;
impl crate::RegisterSpec for PGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgr::R](R) reader structure"]
impl crate::Readable for PGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgr::W](W) writer structure"]
impl crate::Writable for PGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PGR%s to value 0"]
impl crate::Resettable for PGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
