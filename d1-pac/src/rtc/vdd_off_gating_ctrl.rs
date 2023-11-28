#[doc = "Register `vdd_off_gating_ctrl` reader"]
pub type R = crate::R<VDD_OFF_GATING_CTRL_SPEC>;
#[doc = "Register `vdd_off_gating_ctrl` writer"]
pub type W = crate::W<VDD_OFF_GATING_CTRL_SPEC>;
#[doc = "Field `vccio_det_bypass_en` reader - "]
pub type VCCIO_DET_BYPASS_EN_R = crate::BitReader<VCCIO_DET_BYPASS_EN_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCCIO_DET_BYPASS_EN_A {
    #[doc = "0: not bypass"]
    NOT_BYPASS = 0,
    #[doc = "1: bypass"]
    BYPASS = 1,
}
impl From<VCCIO_DET_BYPASS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VCCIO_DET_BYPASS_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl VCCIO_DET_BYPASS_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCCIO_DET_BYPASS_EN_A {
        match self.bits {
            false => VCCIO_DET_BYPASS_EN_A::NOT_BYPASS,
            true => VCCIO_DET_BYPASS_EN_A::BYPASS,
        }
    }
    #[doc = "not bypass"]
    #[inline(always)]
    pub fn is_not_bypass(&self) -> bool {
        *self == VCCIO_DET_BYPASS_EN_A::NOT_BYPASS
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == VCCIO_DET_BYPASS_EN_A::BYPASS
    }
}
#[doc = "Field `vccio_det_bypass_en` writer - "]
pub type VCCIO_DET_BYPASS_EN_W<'a, REG> = crate::BitWriter<'a, REG, VCCIO_DET_BYPASS_EN_A>;
impl<'a, REG> VCCIO_DET_BYPASS_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not bypass"]
    #[inline(always)]
    pub fn not_bypass(self) -> &'a mut crate::W<REG> {
        self.variant(VCCIO_DET_BYPASS_EN_A::NOT_BYPASS)
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(VCCIO_DET_BYPASS_EN_A::BYPASS)
    }
}
#[doc = "Field `vccio_det_spare` reader - - Bit\\[7:5\\]: Reserved, default=0\n- Bit\\[4\\]: Bypass debounce circuit, defaule=0\n- Bit\\[3\\]: Enable control, defaule=0 \n - 0: Disable VCC-IO detection\n - 1: Force the detection output\n- Bit\\[2:0\\]: Gear adjustment\n - 000: Detection threshold is 2.5 V\n - 001: Detection threshold is 2.6 V\n - 010: Detection threshold is 2.7 V (default)\n - 011: Detection threshold is 2.8 V\n - 100: Detection threshold is 2.9 V\n - 101: Detection threshold is 3 V\n - 110: N/A\n - 111: N/A"]
pub type VCCIO_DET_SPARE_R = crate::FieldReader;
#[doc = "Field `vccio_det_spare` writer - - Bit\\[7:5\\]: Reserved, default=0\n- Bit\\[4\\]: Bypass debounce circuit, defaule=0\n- Bit\\[3\\]: Enable control, defaule=0 \n - 0: Disable VCC-IO detection\n - 1: Force the detection output\n- Bit\\[2:0\\]: Gear adjustment\n - 000: Detection threshold is 2.5 V\n - 001: Detection threshold is 2.6 V\n - 010: Detection threshold is 2.7 V (default)\n - 011: Detection threshold is 2.8 V\n - 100: Detection threshold is 2.9 V\n - 101: Detection threshold is 3 V\n - 110: N/A\n - 111: N/A"]
pub type VCCIO_DET_SPARE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pwroff_gat_rtc_cfg` writer - Power off gating control signal\n\n(For Debug Use Only)\n\nWhen use vdd_sys to RTC isolation software control, write this bit to 1. It will only be cleared by resetb release."]
pub type PWROFF_GAT_RTC_CFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `key_field` writer - Key Field\n\nThis field should be filled with 0x16AA, and then the bit 15 can be configured."]
pub type KEY_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vccio_det_bypass_en(&self) -> VCCIO_DET_BYPASS_EN_R {
        VCCIO_DET_BYPASS_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:11 - - Bit\\[7:5\\]: Reserved, default=0\n- Bit\\[4\\]: Bypass debounce circuit, defaule=0\n- Bit\\[3\\]: Enable control, defaule=0 \n - 0: Disable VCC-IO detection\n - 1: Force the detection output\n- Bit\\[2:0\\]: Gear adjustment\n - 000: Detection threshold is 2.5 V\n - 001: Detection threshold is 2.6 V\n - 010: Detection threshold is 2.7 V (default)\n - 011: Detection threshold is 2.8 V\n - 100: Detection threshold is 2.9 V\n - 101: Detection threshold is 3 V\n - 110: N/A\n - 111: N/A"]
    #[inline(always)]
    pub fn vccio_det_spare(&self) -> VCCIO_DET_SPARE_R {
        VCCIO_DET_SPARE_R::new(((self.bits >> 4) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn vccio_det_bypass_en(&mut self) -> VCCIO_DET_BYPASS_EN_W<VDD_OFF_GATING_CTRL_SPEC> {
        VCCIO_DET_BYPASS_EN_W::new(self, 0)
    }
    #[doc = "Bits 4:11 - - Bit\\[7:5\\]: Reserved, default=0\n- Bit\\[4\\]: Bypass debounce circuit, defaule=0\n- Bit\\[3\\]: Enable control, defaule=0 \n - 0: Disable VCC-IO detection\n - 1: Force the detection output\n- Bit\\[2:0\\]: Gear adjustment\n - 000: Detection threshold is 2.5 V\n - 001: Detection threshold is 2.6 V\n - 010: Detection threshold is 2.7 V (default)\n - 011: Detection threshold is 2.8 V\n - 100: Detection threshold is 2.9 V\n - 101: Detection threshold is 3 V\n - 110: N/A\n - 111: N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vccio_det_spare(&mut self) -> VCCIO_DET_SPARE_W<VDD_OFF_GATING_CTRL_SPEC> {
        VCCIO_DET_SPARE_W::new(self, 4)
    }
    #[doc = "Bit 15 - Power off gating control signal\n\n(For Debug Use Only)\n\nWhen use vdd_sys to RTC isolation software control, write this bit to 1. It will only be cleared by resetb release."]
    #[inline(always)]
    #[must_use]
    pub fn pwroff_gat_rtc_cfg(&mut self) -> PWROFF_GAT_RTC_CFG_W<VDD_OFF_GATING_CTRL_SPEC> {
        PWROFF_GAT_RTC_CFG_W::new(self, 15)
    }
    #[doc = "Bits 16:31 - Key Field\n\nThis field should be filled with 0x16AA, and then the bit 15 can be configured."]
    #[inline(always)]
    #[must_use]
    pub fn key_field(&mut self) -> KEY_FIELD_W<VDD_OFF_GATING_CTRL_SPEC> {
        KEY_FIELD_W::new(self, 16)
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
#[doc = "VDD Off Gating Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdd_off_gating_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdd_off_gating_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VDD_OFF_GATING_CTRL_SPEC;
impl crate::RegisterSpec for VDD_OFF_GATING_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdd_off_gating_ctrl::R`](R) reader structure"]
impl crate::Readable for VDD_OFF_GATING_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vdd_off_gating_ctrl::W`](W) writer structure"]
impl crate::Writable for VDD_OFF_GATING_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets vdd_off_gating_ctrl to value 0x21"]
impl crate::Resettable for VDD_OFF_GATING_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x21;
}
