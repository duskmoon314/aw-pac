#[doc = "Register `pio_pow_val` reader"]
pub type R = crate::R<PIO_POW_VAL_SPEC>;
#[doc = "Field `p_pwr_val[C,D,E,F,G]` reader - PX_Port Power Value"]
pub type P_PWR_VAL_R = crate::BitReader;
#[doc = "Field `vccio_pws_val` reader - VCC_IO Power Value"]
pub type VCCIO_PWS_VAL_R = crate::BitReader;
impl R {
    #[doc = "PX_Port Power Value\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pC_pwr_val` field"]
    #[inline(always)]
    pub fn p_pwr_val(&self, n: u8) -> P_PWR_VAL_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        P_PWR_VAL_R::new(((self.bits >> (n + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - PX_Port Power Value"]
    #[inline(always)]
    pub fn p_c_pwr_val(&self) -> P_PWR_VAL_R {
        P_PWR_VAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PX_Port Power Value"]
    #[inline(always)]
    pub fn p_d_pwr_val(&self) -> P_PWR_VAL_R {
        P_PWR_VAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PX_Port Power Value"]
    #[inline(always)]
    pub fn p_e_pwr_val(&self) -> P_PWR_VAL_R {
        P_PWR_VAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PX_Port Power Value"]
    #[inline(always)]
    pub fn p_f_pwr_val(&self) -> P_PWR_VAL_R {
        P_PWR_VAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PX_Port Power Value"]
    #[inline(always)]
    pub fn p_g_pwr_val(&self) -> P_PWR_VAL_R {
        P_PWR_VAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - VCC_IO Power Value"]
    #[inline(always)]
    pub fn vccio_pws_val(&self) -> VCCIO_PWS_VAL_R {
        VCCIO_PWS_VAL_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "PIO Group Power Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio_pow_val::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIO_POW_VAL_SPEC;
impl crate::RegisterSpec for PIO_POW_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pio_pow_val::R`](R) reader structure"]
impl crate::Readable for PIO_POW_VAL_SPEC {}
#[doc = "`reset()` method sets pio_pow_val to value 0"]
impl crate::Resettable for PIO_POW_VAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
