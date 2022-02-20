#[doc = "Register `GP_FIFO_INTC` reader"]
pub struct R(crate::R<GP_FIFO_INTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_FIFO_INTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_FIFO_INTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_FIFO_INTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_FIFO_INTC` writer"]
pub struct W(crate::W<GP_FIFO_INTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_FIFO_INTC_SPEC>;
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
impl From<crate::W<GP_FIFO_INTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_FIFO_INTC_SPEC>) -> Self {
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
#[doc = "GPADC FIFO Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_fifo_intc](index.html) module"]
pub struct GP_FIFO_INTC_SPEC;
impl crate::RegisterSpec for GP_FIFO_INTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_fifo_intc::R](R) reader structure"]
impl crate::Readable for GP_FIFO_INTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_fifo_intc::W](W) writer structure"]
impl crate::Writable for GP_FIFO_INTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_FIFO_INTC to value 0"]
impl crate::Resettable for GP_FIFO_INTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
