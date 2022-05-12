#[doc = "Register `pio_pow_val` reader"]
pub struct R(crate::R<PIO_POW_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO_POW_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO_POW_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO_POW_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VCCIO_PWS_VAL` reader - VCC_IO Power Value"]
pub type VCCIO_PWS_VAL_R = crate::BitReader<bool>;
#[doc = "Fields `P(0-4)_PWR_VAL` reader - PX_Port Power Value"]
pub type P_PWR_VAL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 12 - VCC_IO Power Value"]
    #[inline(always)]
    pub fn vccio_pws_val(&self) -> VCCIO_PWS_VAL_R {
        VCCIO_PWS_VAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "PX_Port Power Value"]
    #[inline(always)]
    pub unsafe fn p_pwr_val(&self, n: u8) -> P_PWR_VAL_R {
        P_PWR_VAL_R::new(((self.bits >> (n + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - PX_Port Power Value"]
    #[inline(always)]
    pub fn pc_pwr_val(&self) -> P_PWR_VAL_R {
        P_PWR_VAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PX_Port Power Value"]
    #[inline(always)]
    pub fn pd_pwr_val(&self) -> P_PWR_VAL_R {
        P_PWR_VAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PX_Port Power Value"]
    #[inline(always)]
    pub fn pe_pwr_val(&self) -> P_PWR_VAL_R {
        P_PWR_VAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PX_Port Power Value"]
    #[inline(always)]
    pub fn pf_pwr_val(&self) -> P_PWR_VAL_R {
        P_PWR_VAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PX_Port Power Value"]
    #[inline(always)]
    pub fn pg_pwr_val(&self) -> P_PWR_VAL_R {
        P_PWR_VAL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "PIO Group Power Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pow_val](index.html) module"]
pub struct PIO_POW_VAL_SPEC;
impl crate::RegisterSpec for PIO_POW_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio_pow_val::R](R) reader structure"]
impl crate::Readable for PIO_POW_VAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets pio_pow_val to value 0"]
impl crate::Resettable for PIO_POW_VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
