#[doc = "Register `GP_CS_EN` reader"]
pub struct R(crate::R<GP_CS_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_CS_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_CS_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_CS_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_CS_EN` writer"]
pub struct W(crate::W<GP_CS_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_CS_EN_SPEC>;
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
impl From<crate::W<GP_CS_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_CS_EN_SPEC>) -> Self {
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
#[doc = "GPADC Compare and Select Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_cs_en](index.html) module"]
pub struct GP_CS_EN_SPEC;
impl crate::RegisterSpec for GP_CS_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_cs_en::R](R) reader structure"]
impl crate::Readable for GP_CS_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_cs_en::W](W) writer structure"]
impl crate::Writable for GP_CS_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_CS_EN to value 0"]
impl crate::Resettable for GP_CS_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
