#[doc = "Register `pll_cpu_tun` reader"]
pub struct R(crate::R<PLL_CPU_TUN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_CPU_TUN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_CPU_TUN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_CPU_TUN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pll_cpu_tun` writer"]
pub struct W(crate::W<PLL_CPU_TUN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_CPU_TUN_SPEC>;
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
impl From<crate::W<PLL_CPU_TUN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_CPU_TUN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pll_b_out` reader - PLL-B-OUT \\[6:0\\] for verify"]
pub type PLL_B_OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pll_reg_od1` reader - PLL-REG-OD1 for verify"]
pub type PLL_REG_OD1_R = crate::BitReader<bool>;
#[doc = "Field `pll_reg_od1` writer - PLL-REG-OD1 for verify"]
pub type PLL_REG_OD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_CPU_TUN_SPEC, bool, O>;
#[doc = "Field `pll_b_in` reader - PLL-B-IN \\[6:0\\] for verify"]
pub type PLL_B_IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pll_b_in` writer - PLL-B-IN \\[6:0\\] for verify"]
pub type PLL_B_IN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_CPU_TUN_SPEC, u8, u8, 7, O>;
#[doc = "Field `pll_reg_od` reader - PLL-REG-OD0 for verify"]
pub type PLL_REG_OD_R = crate::BitReader<bool>;
#[doc = "Field `pll_reg_od` writer - PLL-REG-OD0 for verify"]
pub type PLL_REG_OD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_CPU_TUN_SPEC, bool, O>;
#[doc = "Field `pll_cnt_int` reader - Counter initial control"]
pub type PLL_CNT_INT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pll_cnt_int` writer - Counter initial control"]
pub type PLL_CNT_INT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_CPU_TUN_SPEC, u8, u8, 7, O>;
#[doc = "Field `pll_vco_gain` reader - KVCO gain control"]
pub type PLL_VCO_GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pll_vco_gain` writer - KVCO gain control"]
pub type PLL_VCO_GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_CPU_TUN_SPEC, u8, u8, 3, O>;
#[doc = "Field `pll_vco` reader - VCO range control"]
pub type PLL_VCO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pll_vco` writer - VCO range control"]
pub type PLL_VCO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_CPU_TUN_SPEC, u8, u8, 3, O>;
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
    pub fn pll_reg_od1(&mut self) -> PLL_REG_OD1_W<7> {
        PLL_REG_OD1_W::new(self)
    }
    #[doc = "Bits 8:14 - PLL-B-IN \\[6:0\\] for verify"]
    #[inline(always)]
    #[must_use]
    pub fn pll_b_in(&mut self) -> PLL_B_IN_W<8> {
        PLL_B_IN_W::new(self)
    }
    #[doc = "Bit 15 - PLL-REG-OD0 for verify"]
    #[inline(always)]
    #[must_use]
    pub fn pll_reg_od(&mut self) -> PLL_REG_OD_W<15> {
        PLL_REG_OD_W::new(self)
    }
    #[doc = "Bits 16:22 - Counter initial control"]
    #[inline(always)]
    #[must_use]
    pub fn pll_cnt_int(&mut self) -> PLL_CNT_INT_W<16> {
        PLL_CNT_INT_W::new(self)
    }
    #[doc = "Bits 24:26 - KVCO gain control"]
    #[inline(always)]
    #[must_use]
    pub fn pll_vco_gain(&mut self) -> PLL_VCO_GAIN_W<24> {
        PLL_VCO_GAIN_W::new(self)
    }
    #[doc = "Bits 28:30 - VCO range control"]
    #[inline(always)]
    #[must_use]
    pub fn pll_vco(&mut self) -> PLL_VCO_W<28> {
        PLL_VCO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL_CPU Tuning Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_cpu_tun](index.html) module"]
pub struct PLL_CPU_TUN_SPEC;
impl crate::RegisterSpec for PLL_CPU_TUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_cpu_tun::R](R) reader structure"]
impl crate::Readable for PLL_CPU_TUN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_cpu_tun::W](W) writer structure"]
impl crate::Writable for PLL_CPU_TUN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pll_cpu_tun to value 0"]
impl crate::Resettable for PLL_CPU_TUN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
