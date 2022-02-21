#[doc = "Register `TVE_034_REG` reader"]
pub struct R(crate::R<TVE_034_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_034_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_034_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_034_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TVE_034_REG` writer"]
pub struct W(crate::W<TVE_034_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_034_REG_SPEC>;
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
impl From<crate::W<TVE_034_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_034_REG_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Auto Detection Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_034_reg](index.html) module"]
pub struct TVE_034_REG_SPEC;
impl crate::RegisterSpec for TVE_034_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_034_reg::R](R) reader structure"]
impl crate::Readable for TVE_034_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_034_reg::W](W) writer structure"]
impl crate::Writable for TVE_034_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TVE_034_REG to value 0"]
impl crate::Resettable for TVE_034_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
