#[doc = "Register `ths_per` reader"]
pub struct R(crate::R<THS_PER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THS_PER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THS_PER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THS_PER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ths_per` writer"]
pub struct W(crate::W<THS_PER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THS_PER_SPEC>;
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
impl From<crate::W<THS_PER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THS_PER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `thermal_per` reader - Temperature measurement period\n\n4096*(n + 1)/CLK_IN\n\nThe default value is 10 ms."]
pub type THERMAL_PER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `thermal_per` writer - Temperature measurement period\n\n4096*(n + 1)/CLK_IN\n\nThe default value is 10 ms."]
pub type THERMAL_PER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, THS_PER_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 12:31 - Temperature measurement period\n\n4096*(n + 1)/CLK_IN\n\nThe default value is 10 ms."]
    #[inline(always)]
    pub fn thermal_per(&self) -> THERMAL_PER_R {
        THERMAL_PER_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - Temperature measurement period\n\n4096*(n + 1)/CLK_IN\n\nThe default value is 10 ms."]
    #[inline(always)]
    #[must_use]
    pub fn thermal_per(&mut self) -> THERMAL_PER_W<12> {
        THERMAL_PER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "THS Period Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ths_per](index.html) module"]
pub struct THS_PER_SPEC;
impl crate::RegisterSpec for THS_PER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ths_per::R](R) reader structure"]
impl crate::Readable for THS_PER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ths_per::W](W) writer structure"]
impl crate::Writable for THS_PER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ths_per to value 0x0003_a000"]
impl crate::Resettable for THS_PER_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_a000;
}
