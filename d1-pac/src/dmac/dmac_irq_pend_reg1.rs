#[doc = "Register `DMAC_IRQ_PEND_REG1` reader"]
pub struct R(crate::R<DMAC_IRQ_PEND_REG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_IRQ_PEND_REG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_IRQ_PEND_REG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_IRQ_PEND_REG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_IRQ_PEND_REG1` writer"]
pub struct W(crate::W<DMAC_IRQ_PEND_REG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_IRQ_PEND_REG1_SPEC>;
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
impl From<crate::W<DMAC_IRQ_PEND_REG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_IRQ_PEND_REG1_SPEC>) -> Self {
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
#[doc = "DMAC IRQ Pending Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_irq_pend_reg1](index.html) module"]
pub struct DMAC_IRQ_PEND_REG1_SPEC;
impl crate::RegisterSpec for DMAC_IRQ_PEND_REG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_irq_pend_reg1::R](R) reader structure"]
impl crate::Readable for DMAC_IRQ_PEND_REG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_irq_pend_reg1::W](W) writer structure"]
impl crate::Writable for DMAC_IRQ_PEND_REG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_IRQ_PEND_REG1 to value 0"]
impl crate::Resettable for DMAC_IRQ_PEND_REG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
