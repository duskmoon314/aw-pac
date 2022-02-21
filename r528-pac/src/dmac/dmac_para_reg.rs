#[doc = "Register `DMAC_PARA_REG%s` reader"]
pub struct R(crate::R<DMAC_PARA_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_PARA_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_PARA_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_PARA_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_PARA_REG%s` writer"]
pub struct W(crate::W<DMAC_PARA_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_PARA_REG_SPEC>;
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
impl From<crate::W<DMAC_PARA_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_PARA_REG_SPEC>) -> Self {
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
#[doc = "DMAC Channel Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_para_reg](index.html) module"]
pub struct DMAC_PARA_REG_SPEC;
impl crate::RegisterSpec for DMAC_PARA_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_para_reg::R](R) reader structure"]
impl crate::Readable for DMAC_PARA_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_para_reg::W](W) writer structure"]
impl crate::Writable for DMAC_PARA_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_PARA_REG%s to value 0"]
impl crate::Resettable for DMAC_PARA_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
