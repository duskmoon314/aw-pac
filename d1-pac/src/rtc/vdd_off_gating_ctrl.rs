#[doc = "Register `vdd_off_gating_ctrl` reader"]
pub struct R(crate::R<VDD_OFF_GATING_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDD_OFF_GATING_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDD_OFF_GATING_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDD_OFF_GATING_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `vdd_off_gating_ctrl` writer"]
pub struct W(crate::W<VDD_OFF_GATING_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDD_OFF_GATING_CTRL_SPEC>;
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
impl From<crate::W<VDD_OFF_GATING_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDD_OFF_GATING_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> VCCIO_DET_BYPASS_EN_A {
        match self.bits {
            false => VCCIO_DET_BYPASS_EN_A::NOT_BYPASS,
            true => VCCIO_DET_BYPASS_EN_A::BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BYPASS`"]
    #[inline(always)]
    pub fn is_not_bypass(&self) -> bool {
        *self == VCCIO_DET_BYPASS_EN_A::NOT_BYPASS
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == VCCIO_DET_BYPASS_EN_A::BYPASS
    }
}
#[doc = "Field `vccio_det_bypass_en` writer - "]
pub type VCCIO_DET_BYPASS_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, VDD_OFF_GATING_CTRL_SPEC, VCCIO_DET_BYPASS_EN_A, O>;
impl<'a, const O: u8> VCCIO_DET_BYPASS_EN_W<'a, O> {
    #[doc = "not bypass"]
    #[inline(always)]
    pub fn not_bypass(self) -> &'a mut W {
        self.variant(VCCIO_DET_BYPASS_EN_A::NOT_BYPASS)
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(VCCIO_DET_BYPASS_EN_A::BYPASS)
    }
}
#[doc = "Field `vccio_det_spare` reader - - Bit\\[7:5\\]: Reserved, default=0\n- Bit\\[4\\]: Bypass debounce circuit, defaule=0\n- Bit\\[3\\]: Enable control, defaule=0 \n - 0: Disable VCC-IO detection\n - 1: Force the detection output\n- Bit\\[2:0\\]: Gear adjustment\n - 000: Detection threshold is 2.5 V\n - 001: Detection threshold is 2.6 V\n - 010: Detection threshold is 2.7 V (default)\n - 011: Detection threshold is 2.8 V\n - 100: Detection threshold is 2.9 V\n - 101: Detection threshold is 3 V\n - 110: N/A\n - 111: N/A"]
pub type VCCIO_DET_SPARE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vccio_det_spare` writer - - Bit\\[7:5\\]: Reserved, default=0\n- Bit\\[4\\]: Bypass debounce circuit, defaule=0\n- Bit\\[3\\]: Enable control, defaule=0 \n - 0: Disable VCC-IO detection\n - 1: Force the detection output\n- Bit\\[2:0\\]: Gear adjustment\n - 000: Detection threshold is 2.5 V\n - 001: Detection threshold is 2.6 V\n - 010: Detection threshold is 2.7 V (default)\n - 011: Detection threshold is 2.8 V\n - 100: Detection threshold is 2.9 V\n - 101: Detection threshold is 3 V\n - 110: N/A\n - 111: N/A"]
pub type VCCIO_DET_SPARE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VDD_OFF_GATING_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `pwroff_gat_rtc_cfg` writer - Power off gating control signal\n\n(For Debug Use Only)\n\nWhen use vdd_sys to RTC isolation software control, write this bit to 1. It will only be cleared by resetb release."]
pub type PWROFF_GAT_RTC_CFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, VDD_OFF_GATING_CTRL_SPEC, bool, O>;
#[doc = "Field `key_field` writer - Key Field\n\nThis field should be filled with 0x16AA, and then the bit 15 can be configured."]
pub type KEY_FIELD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VDD_OFF_GATING_CTRL_SPEC, u16, u16, 16, O>;
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
    pub fn vccio_det_bypass_en(&mut self) -> VCCIO_DET_BYPASS_EN_W<0> {
        VCCIO_DET_BYPASS_EN_W::new(self)
    }
    #[doc = "Bits 4:11 - - Bit\\[7:5\\]: Reserved, default=0\n- Bit\\[4\\]: Bypass debounce circuit, defaule=0\n- Bit\\[3\\]: Enable control, defaule=0 \n - 0: Disable VCC-IO detection\n - 1: Force the detection output\n- Bit\\[2:0\\]: Gear adjustment\n - 000: Detection threshold is 2.5 V\n - 001: Detection threshold is 2.6 V\n - 010: Detection threshold is 2.7 V (default)\n - 011: Detection threshold is 2.8 V\n - 100: Detection threshold is 2.9 V\n - 101: Detection threshold is 3 V\n - 110: N/A\n - 111: N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vccio_det_spare(&mut self) -> VCCIO_DET_SPARE_W<4> {
        VCCIO_DET_SPARE_W::new(self)
    }
    #[doc = "Bit 15 - Power off gating control signal\n\n(For Debug Use Only)\n\nWhen use vdd_sys to RTC isolation software control, write this bit to 1. It will only be cleared by resetb release."]
    #[inline(always)]
    #[must_use]
    pub fn pwroff_gat_rtc_cfg(&mut self) -> PWROFF_GAT_RTC_CFG_W<15> {
        PWROFF_GAT_RTC_CFG_W::new(self)
    }
    #[doc = "Bits 16:31 - Key Field\n\nThis field should be filled with 0x16AA, and then the bit 15 can be configured."]
    #[inline(always)]
    #[must_use]
    pub fn key_field(&mut self) -> KEY_FIELD_W<16> {
        KEY_FIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VDD Off Gating Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdd_off_gating_ctrl](index.html) module"]
pub struct VDD_OFF_GATING_CTRL_SPEC;
impl crate::RegisterSpec for VDD_OFF_GATING_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdd_off_gating_ctrl::R](R) reader structure"]
impl crate::Readable for VDD_OFF_GATING_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vdd_off_gating_ctrl::W](W) writer structure"]
impl crate::Writable for VDD_OFF_GATING_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets vdd_off_gating_ctrl to value 0x21"]
impl crate::Resettable for VDD_OFF_GATING_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x21;
}
