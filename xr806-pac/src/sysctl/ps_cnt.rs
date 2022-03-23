#[doc = "Register `PS_CNT` reader"]
pub struct R(crate::R<PS_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PS_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PS_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PS_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLK250M_CNT` reader - clk 250M counter"]
pub struct CLK250M_CNT_R(crate::FieldReader<u16, u16>);
impl CLK250M_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CLK250M_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK250M_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - clk 250M counter"]
    #[inline(always)]
    pub fn clk250m_cnt(&self) -> CLK250M_CNT_R {
        CLK250M_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Psensor counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_cnt](index.html) module"]
pub struct PS_CNT_SPEC;
impl crate::RegisterSpec for PS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ps_cnt::R](R) reader structure"]
impl crate::Readable for PS_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PS_CNT to value 0"]
impl crate::Resettable for PS_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
