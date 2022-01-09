#[doc = "Register `WORK_MODE_REG` reader"]
pub struct R(crate::R<WORK_MODE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WORK_MODE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WORK_MODE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WORK_MODE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WORK_MODE_REG` writer"]
pub struct W(crate::W<WORK_MODE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WORK_MODE_REG_SPEC>;
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
impl From<crate::W<WORK_MODE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WORK_MODE_REG_SPEC>) -> Self {
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
#[doc = "Work Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [work_mode_reg](index.html) module"]
pub struct WORK_MODE_REG_SPEC;
impl crate::RegisterSpec for WORK_MODE_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [work_mode_reg::R](R) reader structure"]
impl crate::Readable for WORK_MODE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [work_mode_reg::W](W) writer structure"]
impl crate::Writable for WORK_MODE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WORK_MODE_REG to value 0"]
impl crate::Resettable for WORK_MODE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
