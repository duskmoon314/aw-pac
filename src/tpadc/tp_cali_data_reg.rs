#[doc = "Register `TP_CALI_DATA_REG` reader"]
pub struct R(crate::R<TP_CALI_DATA_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TP_CALI_DATA_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TP_CALI_DATA_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TP_CALI_DATA_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TP_CALI_DATA_REG` writer"]
pub struct W(crate::W<TP_CALI_DATA_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TP_CALI_DATA_REG_SPEC>;
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
impl From<crate::W<TP_CALI_DATA_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TP_CALI_DATA_REG_SPEC>) -> Self {
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
#[doc = "TP Calibration Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tp_cali_data_reg](index.html) module"]
pub struct TP_CALI_DATA_REG_SPEC;
impl crate::RegisterSpec for TP_CALI_DATA_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tp_cali_data_reg::R](R) reader structure"]
impl crate::Readable for TP_CALI_DATA_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tp_cali_data_reg::W](W) writer structure"]
impl crate::Writable for TP_CALI_DATA_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TP_CALI_DATA_REG to value 0"]
impl crate::Resettable for TP_CALI_DATA_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
