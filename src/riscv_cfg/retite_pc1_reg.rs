#[doc = "Register `RETITE_PC1_REG` reader"]
pub struct R(crate::R<RETITE_PC1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETITE_PC1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETITE_PC1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETITE_PC1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETITE_PC1_REG` writer"]
pub struct W(crate::W<RETITE_PC1_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETITE_PC1_REG_SPEC>;
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
impl From<crate::W<RETITE_PC1_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETITE_PC1_REG_SPEC>) -> Self {
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
#[doc = "Retire PC1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retite_pc1_reg](index.html) module"]
pub struct RETITE_PC1_REG_SPEC;
impl crate::RegisterSpec for RETITE_PC1_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retite_pc1_reg::R](R) reader structure"]
impl crate::Readable for RETITE_PC1_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retite_pc1_reg::W](W) writer structure"]
impl crate::Writable for RETITE_PC1_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RETITE_PC1_REG to value 0"]
impl crate::Resettable for RETITE_PC1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
