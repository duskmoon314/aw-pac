#[doc = "Register `avs_cnt_ctl` reader"]
pub struct R(crate::R<AVS_CNT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AVS_CNT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AVS_CNT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AVS_CNT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `avs_cnt_ctl` writer"]
pub struct W(crate::W<AVS_CNT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AVS_CNT_CTL_SPEC>;
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
impl From<crate::W<AVS_CNT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AVS_CNT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `avs_cnt_en[0-1]` reader - Audio Video Sync Counter Enable"]
pub type AVS_CNT_EN_R = crate::BitReader<AVS_CNT_EN_A>;
#[doc = "Audio Video Sync Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVS_CNT_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<AVS_CNT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AVS_CNT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AVS_CNT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVS_CNT_EN_A {
        match self.bits {
            false => AVS_CNT_EN_A::DISABLED,
            true => AVS_CNT_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AVS_CNT_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AVS_CNT_EN_A::ENABLED
    }
}
#[doc = "Field `avs_cnt_en[0-1]` writer - Audio Video Sync Counter Enable"]
pub type AVS_CNT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AVS_CNT_CTL_SPEC, AVS_CNT_EN_A, O>;
impl<'a, const O: u8> AVS_CNT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AVS_CNT_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AVS_CNT_EN_A::ENABLED)
    }
}
#[doc = "Field `avs_cnt_ps[0-1]` reader - Audio Video Sync Counter Pause Control"]
pub type AVS_CNT_PS_R = crate::BitReader<AVS_CNT_PS_A>;
#[doc = "Audio Video Sync Counter Pause Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVS_CNT_PS_A {
    #[doc = "0: `0`"]
    NOT_PAUSE = 0,
    #[doc = "1: `1`"]
    PAUSE = 1,
}
impl From<AVS_CNT_PS_A> for bool {
    #[inline(always)]
    fn from(variant: AVS_CNT_PS_A) -> Self {
        variant as u8 != 0
    }
}
impl AVS_CNT_PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVS_CNT_PS_A {
        match self.bits {
            false => AVS_CNT_PS_A::NOT_PAUSE,
            true => AVS_CNT_PS_A::PAUSE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PAUSE`"]
    #[inline(always)]
    pub fn is_not_pause(&self) -> bool {
        *self == AVS_CNT_PS_A::NOT_PAUSE
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == AVS_CNT_PS_A::PAUSE
    }
}
#[doc = "Field `avs_cnt_ps[0-1]` writer - Audio Video Sync Counter Pause Control"]
pub type AVS_CNT_PS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AVS_CNT_CTL_SPEC, AVS_CNT_PS_A, O>;
impl<'a, const O: u8> AVS_CNT_PS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_pause(self) -> &'a mut W {
        self.variant(AVS_CNT_PS_A::NOT_PAUSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pause(self) -> &'a mut W {
        self.variant(AVS_CNT_PS_A::PAUSE)
    }
}
impl R {
    #[doc = "Audio Video Sync Counter Enable"]
    #[inline(always)]
    pub unsafe fn avs_cnt_en(&self, n: u8) -> AVS_CNT_EN_R {
        AVS_CNT_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Audio Video Sync Counter Enable"]
    #[inline(always)]
    pub fn avs_cnt0_en(&self) -> AVS_CNT_EN_R {
        AVS_CNT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Audio Video Sync Counter Enable"]
    #[inline(always)]
    pub fn avs_cnt1_en(&self) -> AVS_CNT_EN_R {
        AVS_CNT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Audio Video Sync Counter Pause Control"]
    #[inline(always)]
    pub unsafe fn avs_cnt_ps(&self, n: u8) -> AVS_CNT_PS_R {
        AVS_CNT_PS_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Bit 8 - Audio Video Sync Counter Pause Control"]
    #[inline(always)]
    pub fn avs_cnt0_ps(&self) -> AVS_CNT_PS_R {
        AVS_CNT_PS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Audio Video Sync Counter Pause Control"]
    #[inline(always)]
    pub fn avs_cnt1_ps(&self) -> AVS_CNT_PS_R {
        AVS_CNT_PS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Audio Video Sync Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn avs_cnt_en<const O: u8>(&mut self) -> AVS_CNT_EN_W<O> {
        AVS_CNT_EN_W::new(self)
    }
    #[doc = "Bit 0 - Audio Video Sync Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn avs_cnt0_en(&mut self) -> AVS_CNT_EN_W<0> {
        AVS_CNT_EN_W::new(self)
    }
    #[doc = "Bit 1 - Audio Video Sync Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn avs_cnt1_en(&mut self) -> AVS_CNT_EN_W<1> {
        AVS_CNT_EN_W::new(self)
    }
    #[doc = "Audio Video Sync Counter Pause Control"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn avs_cnt_ps<const O: u8>(&mut self) -> AVS_CNT_PS_W<O> {
        AVS_CNT_PS_W::new(self)
    }
    #[doc = "Bit 8 - Audio Video Sync Counter Pause Control"]
    #[inline(always)]
    #[must_use]
    pub fn avs_cnt0_ps(&mut self) -> AVS_CNT_PS_W<8> {
        AVS_CNT_PS_W::new(self)
    }
    #[doc = "Bit 9 - Audio Video Sync Counter Pause Control"]
    #[inline(always)]
    #[must_use]
    pub fn avs_cnt1_ps(&mut self) -> AVS_CNT_PS_W<9> {
        AVS_CNT_PS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AVS Counter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [avs_cnt_ctl](index.html) module"]
pub struct AVS_CNT_CTL_SPEC;
impl crate::RegisterSpec for AVS_CNT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [avs_cnt_ctl::R](R) reader structure"]
impl crate::Readable for AVS_CNT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [avs_cnt_ctl::W](W) writer structure"]
impl crate::Writable for AVS_CNT_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets avs_cnt_ctl to value 0"]
impl crate::Resettable for AVS_CNT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
