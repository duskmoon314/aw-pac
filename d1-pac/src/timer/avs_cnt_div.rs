#[doc = "Register `avs_cnt_div` reader"]
pub struct R(crate::R<AVS_CNT_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AVS_CNT_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AVS_CNT_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AVS_CNT_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `avs_cnt_div` writer"]
pub struct W(crate::W<AVS_CNT_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AVS_CNT_DIV_SPEC>;
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
impl From<crate::W<AVS_CNT_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AVS_CNT_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `avs_cnt_d[0-1]` reader - The divisor factor of AVS"]
pub type AVS_CNT_D_R = crate::FieldReader<u16, u16>;
#[doc = "Field `avs_cnt_d[0-1]` writer - The divisor factor of AVS"]
pub type AVS_CNT_D_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AVS_CNT_DIV_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "The divisor factor of AVS"]
    #[inline(always)]
    pub unsafe fn avs_cnt_d(&self, n: u8) -> AVS_CNT_D_R {
        AVS_CNT_D_R::new(((self.bits >> (n * 16)) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - The divisor factor of AVS"]
    #[inline(always)]
    pub fn avs_cnt0_d(&self) -> AVS_CNT_D_R {
        AVS_CNT_D_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - The divisor factor of AVS"]
    #[inline(always)]
    pub fn avs_cnt1_d(&self) -> AVS_CNT_D_R {
        AVS_CNT_D_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "The divisor factor of AVS"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn avs_cnt_d<const O: u8>(&mut self) -> AVS_CNT_D_W<O> {
        AVS_CNT_D_W::new(self)
    }
    #[doc = "Bits 0:11 - The divisor factor of AVS"]
    #[inline(always)]
    #[must_use]
    pub fn avs_cnt0_d(&mut self) -> AVS_CNT_D_W<0> {
        AVS_CNT_D_W::new(self)
    }
    #[doc = "Bits 16:27 - The divisor factor of AVS"]
    #[inline(always)]
    #[must_use]
    pub fn avs_cnt1_d(&mut self) -> AVS_CNT_D_W<16> {
        AVS_CNT_D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AVS Counter Divisor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [avs_cnt_div](index.html) module"]
pub struct AVS_CNT_DIV_SPEC;
impl crate::RegisterSpec for AVS_CNT_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [avs_cnt_div::R](R) reader structure"]
impl crate::Readable for AVS_CNT_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [avs_cnt_div::W](W) writer structure"]
impl crate::Writable for AVS_CNT_DIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets avs_cnt_div to value 0"]
impl crate::Resettable for AVS_CNT_DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
