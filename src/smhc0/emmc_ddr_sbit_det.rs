#[doc = "Register `EMMC_DDR_SBIT_DET` reader"]
pub struct R(crate::R<EMMC_DDR_SBIT_DET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_DDR_SBIT_DET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_DDR_SBIT_DET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_DDR_SBIT_DET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMC_DDR_SBIT_DET` writer"]
pub struct W(crate::W<EMMC_DDR_SBIT_DET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMC_DDR_SBIT_DET_SPEC>;
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
impl From<crate::W<EMMC_DDR_SBIT_DET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMC_DDR_SBIT_DET_SPEC>) -> Self {
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
#[doc = "eMMC4.5 DDR Start Bit Detection Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_ddr_sbit_det](index.html) module"]
pub struct EMMC_DDR_SBIT_DET_SPEC;
impl crate::RegisterSpec for EMMC_DDR_SBIT_DET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emmc_ddr_sbit_det::R](R) reader structure"]
impl crate::Readable for EMMC_DDR_SBIT_DET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmc_ddr_sbit_det::W](W) writer structure"]
impl crate::Writable for EMMC_DDR_SBIT_DET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMMC_DDR_SBIT_DET to value 0"]
impl crate::Resettable for EMMC_DDR_SBIT_DET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
