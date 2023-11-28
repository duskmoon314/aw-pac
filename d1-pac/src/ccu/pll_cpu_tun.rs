#[doc = "Register `pll_cpu_tun` reader"]
pub type R = crate::R<PLL_CPU_TUN_SPEC>;
#[doc = "Register `pll_cpu_tun` writer"]
pub type W = crate::W<PLL_CPU_TUN_SPEC>;
#[doc = "Field `pll_b_out` reader - PLL-B-OUT \\[6:0\\] for verify"]
pub type PLL_B_OUT_R = crate::FieldReader;
#[doc = "Field `pll_reg_od1` reader - PLL-REG-OD1 for verify"]
pub type PLL_REG_OD1_R = crate::BitReader;
#[doc = "Field `pll_reg_od1` writer - PLL-REG-OD1 for verify"]
pub type PLL_REG_OD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll_b_in` reader - PLL-B-IN \\[6:0\\] for verify"]
pub type PLL_B_IN_R = crate::FieldReader;
#[doc = "Field `pll_b_in` writer - PLL-B-IN \\[6:0\\] for verify"]
pub type PLL_B_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `pll_reg_od` reader - PLL-REG-OD0 for verify"]
pub type PLL_REG_OD_R = crate::BitReader;
#[doc = "Field `pll_reg_od` writer - PLL-REG-OD0 for verify"]
pub type PLL_REG_OD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll_cnt_int` reader - Counter initial control"]
pub type PLL_CNT_INT_R = crate::FieldReader;
#[doc = "Field `pll_cnt_int` writer - Counter initial control"]
pub type PLL_CNT_INT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `pll_vco_gain` reader - KVCO gain control"]
pub type PLL_VCO_GAIN_R = crate::FieldReader;
#[doc = "Field `pll_vco_gain` writer - KVCO gain control"]
pub type PLL_VCO_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pll_vco` reader - VCO range control"]
pub type PLL_VCO_R = crate::FieldReader;
#[doc = "Field `pll_vco` writer - VCO range control"]
pub type PLL_VCO_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:6 - PLL-B-OUT \\[6:0\\] for verify"]
    #[inline(always)]
    pub fn pll_b_out(&self) -> PLL_B_OUT_R {
        PLL_B_OUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - PLL-REG-OD1 for verify"]
    #[inline(always)]
    pub fn pll_reg_od1(&self) -> PLL_REG_OD1_R {
        PLL_REG_OD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - PLL-B-IN \\[6:0\\] for verify"]
    #[inline(always)]
    pub fn pll_b_in(&self) -> PLL_B_IN_R {
        PLL_B_IN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - PLL-REG-OD0 for verify"]
    #[inline(always)]
    pub fn pll_reg_od(&self) -> PLL_REG_OD_R {
        PLL_REG_OD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Counter initial control"]
    #[inline(always)]
    pub fn pll_cnt_int(&self) -> PLL_CNT_INT_R {
        PLL_CNT_INT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:26 - KVCO gain control"]
    #[inline(always)]
    pub fn pll_vco_gain(&self) -> PLL_VCO_GAIN_R {
        PLL_VCO_GAIN_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - VCO range control"]
    #[inline(always)]
    pub fn pll_vco(&self) -> PLL_VCO_R {
        PLL_VCO_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - PLL-REG-OD1 for verify"]
    #[inline(always)]
    #[must_use]
    pub fn pll_reg_od1(&mut self) -> PLL_REG_OD1_W<PLL_CPU_TUN_SPEC> {
        PLL_REG_OD1_W::new(self, 7)
    }
    #[doc = "Bits 8:14 - PLL-B-IN \\[6:0\\] for verify"]
    #[inline(always)]
    #[must_use]
    pub fn pll_b_in(&mut self) -> PLL_B_IN_W<PLL_CPU_TUN_SPEC> {
        PLL_B_IN_W::new(self, 8)
    }
    #[doc = "Bit 15 - PLL-REG-OD0 for verify"]
    #[inline(always)]
    #[must_use]
    pub fn pll_reg_od(&mut self) -> PLL_REG_OD_W<PLL_CPU_TUN_SPEC> {
        PLL_REG_OD_W::new(self, 15)
    }
    #[doc = "Bits 16:22 - Counter initial control"]
    #[inline(always)]
    #[must_use]
    pub fn pll_cnt_int(&mut self) -> PLL_CNT_INT_W<PLL_CPU_TUN_SPEC> {
        PLL_CNT_INT_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - KVCO gain control"]
    #[inline(always)]
    #[must_use]
    pub fn pll_vco_gain(&mut self) -> PLL_VCO_GAIN_W<PLL_CPU_TUN_SPEC> {
        PLL_VCO_GAIN_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - VCO range control"]
    #[inline(always)]
    #[must_use]
    pub fn pll_vco(&mut self) -> PLL_VCO_W<PLL_CPU_TUN_SPEC> {
        PLL_VCO_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PLL_CPU Tuning Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_cpu_tun::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_cpu_tun::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL_CPU_TUN_SPEC;
impl crate::RegisterSpec for PLL_CPU_TUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_cpu_tun::R`](R) reader structure"]
impl crate::Readable for PLL_CPU_TUN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll_cpu_tun::W`](W) writer structure"]
impl crate::Writable for PLL_CPU_TUN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pll_cpu_tun to value 0"]
impl crate::Resettable for PLL_CPU_TUN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
