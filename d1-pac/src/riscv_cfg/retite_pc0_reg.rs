#[doc = "Register `retite_pc0_reg` reader"]
pub struct R(crate::R<RETITE_PC0_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETITE_PC0_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETITE_PC0_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETITE_PC0_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Retire PC0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retite_pc0_reg](index.html) module"]
pub struct RETITE_PC0_REG_SPEC;
impl crate::RegisterSpec for RETITE_PC0_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retite_pc0_reg::R](R) reader structure"]
impl crate::Readable for RETITE_PC0_REG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets retite_pc0_reg to value 0"]
impl crate::Resettable for RETITE_PC0_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
