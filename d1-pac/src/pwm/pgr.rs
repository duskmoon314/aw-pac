#[doc = "Register `pgr%s` reader"]
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
#[doc = "Register `pgr%s` writer"]
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
#[doc = "Field `cs` reader - If bit\\[i\\] is set, the PWM i is selected as one channel of PWM Group\\[g\\]."]
pub type CS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cs` writer - If bit\\[i\\] is set, the PWM i is selected as one channel of PWM Group\\[g\\]."]
pub type CS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PGR_SPEC, u16, u16, 16, O>;
#[doc = "Field `en` reader - PWM Group Enable."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `en` writer - PWM Group Enable."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PGR_SPEC, bool, O>;
#[doc = "Field `start` reader - The PWM channels selected in CS start to output PWM waveform at the same time."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `start` writer - The PWM channels selected in CS start to output PWM waveform at the same time."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, PGR_SPEC, bool, O>;
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
    pub fn cs(&mut self) -> CS_W<0> {
        CS_W::new(self)
    }
    #[doc = "Bit 16 - PWM Group Enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<16> {
        EN_W::new(self)
    }
    #[doc = "Bit 17 - The PWM channels selected in CS start to output PWM waveform at the same time."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<17> {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Group\\[g\\] Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgr](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pgr%s to value 0"]
impl crate::Resettable for PGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
