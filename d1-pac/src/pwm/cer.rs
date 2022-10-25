#[doc = "Register `cer` reader"]
pub struct R(crate::R<CER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cer` writer"]
pub struct W(crate::W<CER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CER_SPEC>;
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
impl From<crate::W<CER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cap_en[0-7]` reader - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
pub type CAP_EN_R = crate::BitReader<CAP_EN_A>;
#[doc = "When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP_EN_A {
    #[doc = "0: Capture disable"]
    DISABLE = 0,
    #[doc = "1: Capture enable"]
    ENABLE = 1,
}
impl From<CAP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CAP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP_EN_A {
        match self.bits {
            false => CAP_EN_A::DISABLE,
            true => CAP_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP_EN_A::ENABLE
    }
}
#[doc = "Field `cap_en[0-7]` writer - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
pub type CAP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CER_SPEC, CAP_EN_A, O>;
impl<'a, const O: u8> CAP_EN_W<'a, O> {
    #[doc = "Capture disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP_EN_A::DISABLE)
    }
    #[doc = "Capture enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub unsafe fn cap_en(&self, n: u8) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap0_en(&self) -> CAP_EN_R {
        CAP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap1_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap2_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap3_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap4_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap5_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap6_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    pub fn cap7_en(&self) -> CAP_EN_R {
        CAP_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn cap_en<const O: u8>(&mut self) -> CAP_EN_W<O> {
        CAP_EN_W::new(self)
    }
    #[doc = "Bit 0 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap0_en(&mut self) -> CAP_EN_W<0> {
        CAP_EN_W::new(self)
    }
    #[doc = "Bit 1 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap1_en(&mut self) -> CAP_EN_W<1> {
        CAP_EN_W::new(self)
    }
    #[doc = "Bit 2 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap2_en(&mut self) -> CAP_EN_W<2> {
        CAP_EN_W::new(self)
    }
    #[doc = "Bit 3 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap3_en(&mut self) -> CAP_EN_W<3> {
        CAP_EN_W::new(self)
    }
    #[doc = "Bit 4 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap4_en(&mut self) -> CAP_EN_W<4> {
        CAP_EN_W::new(self)
    }
    #[doc = "Bit 5 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap5_en(&mut self) -> CAP_EN_W<5> {
        CAP_EN_W::new(self)
    }
    #[doc = "Bit 6 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap6_en(&mut self) -> CAP_EN_W<6> {
        CAP_EN_W::new(self)
    }
    #[doc = "Bit 7 - When enabling the capture function, the 16-bit up-counter starts working, and the capture channel is permitted to capture external falling edge or rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap7_en(&mut self) -> CAP_EN_W<7> {
        CAP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cer](index.html) module"]
pub struct CER_SPEC;
impl crate::RegisterSpec for CER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cer::R](R) reader structure"]
impl crate::Readable for CER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cer::W](W) writer structure"]
impl crate::Writable for CER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cer to value 0"]
impl crate::Resettable for CER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
