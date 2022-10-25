#[doc = "Register `wakeup_en` reader"]
pub struct R(crate::R<WAKEUP_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEUP_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEUP_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEUP_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wakeup_en` writer"]
pub struct W(crate::W<WAKEUP_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEUP_EN_SPEC>;
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
impl From<crate::W<WAKEUP_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEUP_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wp_en` reader - Wakeup Enable"]
pub type WP_EN_R = crate::BitReader<bool>;
#[doc = "Field `wp_en` writer - Wakeup Enable"]
pub type WP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKEUP_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Wakeup Enable"]
    #[inline(always)]
    pub fn wp_en(&self) -> WP_EN_R {
        WP_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wp_en(&mut self) -> WP_EN_W<0> {
        WP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup_en](index.html) module"]
pub struct WAKEUP_EN_SPEC;
impl crate::RegisterSpec for WAKEUP_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeup_en::R](R) reader structure"]
impl crate::Readable for WAKEUP_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeup_en::W](W) writer structure"]
impl crate::Writable for WAKEUP_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wakeup_en to value 0"]
impl crate::Resettable for WAKEUP_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
