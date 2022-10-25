#[doc = "Register `dcxo_ctrl` reader"]
pub struct R(crate::R<DCXO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCXO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCXO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCXO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dcxo_ctrl` writer"]
pub struct W(crate::W<DCXO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCXO_CTRL_SPEC>;
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
impl From<crate::W<DCXO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCXO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clk16m_rc_en` reader - The related register configuration is necessary to ensure the reset debounce circuit has a stable clock source. The first time SoC starts up, by default, the reset debounce circuit of SoC uses 32K divided by RC16M. In power-off, software reads the related bit to ensure whether EXT32K is working normally, if it is normal, first switch the clock source of debounce circuit to EXT32K, then close RC16M. Without EXT32K scenario or external RTC scenario, software confirms firstly whether EXT32K is working normally before switching, or software does not close RC16M."]
pub type CLK16M_RC_EN_R = crate::BitReader<CLK16M_RC_EN_A>;
#[doc = "The related register configuration is necessary to ensure the reset debounce circuit has a stable clock source. The first time SoC starts up, by default, the reset debounce circuit of SoC uses 32K divided by RC16M. In power-off, software reads the related bit to ensure whether EXT32K is working normally, if it is normal, first switch the clock source of debounce circuit to EXT32K, then close RC16M. Without EXT32K scenario or external RTC scenario, software confirms firstly whether EXT32K is working normally before switching, or software does not close RC16M.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK16M_RC_EN_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<CLK16M_RC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLK16M_RC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK16M_RC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK16M_RC_EN_A {
        match self.bits {
            true => CLK16M_RC_EN_A::ENABLE,
            false => CLK16M_RC_EN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLK16M_RC_EN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CLK16M_RC_EN_A::DISABLE
    }
}
#[doc = "Field `clk16m_rc_en` writer - The related register configuration is necessary to ensure the reset debounce circuit has a stable clock source. The first time SoC starts up, by default, the reset debounce circuit of SoC uses 32K divided by RC16M. In power-off, software reads the related bit to ensure whether EXT32K is working normally, if it is normal, first switch the clock source of debounce circuit to EXT32K, then close RC16M. Without EXT32K scenario or external RTC scenario, software confirms firstly whether EXT32K is working normally before switching, or software does not close RC16M."]
pub type CLK16M_RC_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCXO_CTRL_SPEC, CLK16M_RC_EN_A, O>;
impl<'a, const O: u8> CLK16M_RC_EN_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLK16M_RC_EN_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CLK16M_RC_EN_A::DISABLE)
    }
}
#[doc = "Field `dcxo_en` reader - DCXO enable"]
pub type DCXO_EN_R = crate::BitReader<DCXO_EN_A>;
#[doc = "DCXO enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCXO_EN_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<DCXO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DCXO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCXO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCXO_EN_A {
        match self.bits {
            true => DCXO_EN_A::ENABLE,
            false => DCXO_EN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DCXO_EN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DCXO_EN_A::DISABLE
    }
}
#[doc = "Field `dcxo_en` writer - DCXO enable"]
pub type DCXO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCXO_CTRL_SPEC, DCXO_EN_A, O>;
impl<'a, const O: u8> DCXO_EN_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DCXO_EN_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DCXO_EN_A::DISABLE)
    }
}
#[doc = "Field `rsto_dly_sel` reader - For Debug Use Only.\n\nIt cannot configure to 0 in normal state."]
pub type RSTO_DLY_SEL_R = crate::BitReader<bool>;
#[doc = "Field `rsto_dly_sel` writer - For Debug Use Only.\n\nIt cannot configure to 0 in normal state."]
pub type RSTO_DLY_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCXO_CTRL_SPEC, bool, O>;
#[doc = "Field `dcxo_rfclk_enhance` reader - DCXO rfclk enhance\n\nEnhance driving capacity of output OUT_RF_REFCLK, 0x0 for 5 pF, 0x1 for 10 pF, 0x2 for 15 pF, 0x3 for 20 pF."]
pub type DCXO_RFCLK_ENHANCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcxo_rfclk_enhance` writer - DCXO rfclk enhance\n\nEnhance driving capacity of output OUT_RF_REFCLK, 0x0 for 5 pF, 0x1 for 10 pF, 0x2 for 15 pF, 0x3 for 20 pF."]
pub type DCXO_RFCLK_ENHANCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCXO_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `xtal_mode` reader - Xtal mode enable signal, active high"]
pub type XTAL_MODE_R = crate::BitReader<XTAL_MODE_A>;
#[doc = "Xtal mode enable signal, active high\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XTAL_MODE_A {
    #[doc = "0: For external clk input mode"]
    FOR_EXTERNAL = 0,
    #[doc = "1: For normal mode"]
    FOR_NORMAL_MODE = 1,
}
impl From<XTAL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl XTAL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTAL_MODE_A {
        match self.bits {
            false => XTAL_MODE_A::FOR_EXTERNAL,
            true => XTAL_MODE_A::FOR_NORMAL_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `FOR_EXTERNAL`"]
    #[inline(always)]
    pub fn is_for_external(&self) -> bool {
        *self == XTAL_MODE_A::FOR_EXTERNAL
    }
    #[doc = "Checks if the value of the field is `FOR_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_for_normal_mode(&self) -> bool {
        *self == XTAL_MODE_A::FOR_NORMAL_MODE
    }
}
#[doc = "Field `xtal_mode` writer - Xtal mode enable signal, active high"]
pub type XTAL_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCXO_CTRL_SPEC, XTAL_MODE_A, O>;
impl<'a, const O: u8> XTAL_MODE_W<'a, O> {
    #[doc = "For external clk input mode"]
    #[inline(always)]
    pub fn for_external(self) -> &'a mut W {
        self.variant(XTAL_MODE_A::FOR_EXTERNAL)
    }
    #[doc = "For normal mode"]
    #[inline(always)]
    pub fn for_normal_mode(self) -> &'a mut W {
        self.variant(XTAL_MODE_A::FOR_NORMAL_MODE)
    }
}
#[doc = "Field `dcxo_ldo_inrushb` reader - DCXO LDO driving capacity signal, active high"]
pub type DCXO_LDO_INRUSHB_R = crate::BitReader<bool>;
#[doc = "Field `dcxo_ldo_inrushb` writer - DCXO LDO driving capacity signal, active high"]
pub type DCXO_LDO_INRUSHB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCXO_CTRL_SPEC, bool, O>;
#[doc = "Field `dcxo_bg` reader - DCXO bandgap output voltage"]
pub type DCXO_BG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcxo_bg` writer - DCXO bandgap output voltage"]
pub type DCXO_BG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCXO_CTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `dcxo_trim` reader - DCXO cap array value\n\nThe capacity cell is 55 fF."]
pub type DCXO_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcxo_trim` writer - DCXO cap array value\n\nThe capacity cell is 55 fF."]
pub type DCXO_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCXO_CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `dcxo_ictrl` reader - DCXO current control value"]
pub type DCXO_ICTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcxo_ictrl` writer - DCXO current control value"]
pub type DCXO_ICTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCXO_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `clk_req_enb` reader - Clock REQ enable"]
pub type CLK_REQ_ENB_R = crate::BitReader<CLK_REQ_ENB_A>;
#[doc = "Clock REQ enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_REQ_ENB_A {
    #[doc = "0: Enable DCXO wake up function"]
    ENABLE = 0,
    #[doc = "1: Disable DCXO wake up function"]
    DISABLE = 1,
}
impl From<CLK_REQ_ENB_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_REQ_ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_REQ_ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_REQ_ENB_A {
        match self.bits {
            false => CLK_REQ_ENB_A::ENABLE,
            true => CLK_REQ_ENB_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLK_REQ_ENB_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CLK_REQ_ENB_A::DISABLE
    }
}
#[doc = "Field `clk_req_enb` writer - Clock REQ enable"]
pub type CLK_REQ_ENB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCXO_CTRL_SPEC, CLK_REQ_ENB_A, O>;
impl<'a, const O: u8> CLK_REQ_ENB_W<'a, O> {
    #[doc = "Enable DCXO wake up function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLK_REQ_ENB_A::ENABLE)
    }
    #[doc = "Disable DCXO wake up function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CLK_REQ_ENB_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 0 - The related register configuration is necessary to ensure the reset debounce circuit has a stable clock source. The first time SoC starts up, by default, the reset debounce circuit of SoC uses 32K divided by RC16M. In power-off, software reads the related bit to ensure whether EXT32K is working normally, if it is normal, first switch the clock source of debounce circuit to EXT32K, then close RC16M. Without EXT32K scenario or external RTC scenario, software confirms firstly whether EXT32K is working normally before switching, or software does not close RC16M."]
    #[inline(always)]
    pub fn clk16m_rc_en(&self) -> CLK16M_RC_EN_R {
        CLK16M_RC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCXO enable"]
    #[inline(always)]
    pub fn dcxo_en(&self) -> DCXO_EN_R {
        DCXO_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For Debug Use Only.\n\nIt cannot configure to 0 in normal state."]
    #[inline(always)]
    pub fn rsto_dly_sel(&self) -> RSTO_DLY_SEL_R {
        RSTO_DLY_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - DCXO rfclk enhance\n\nEnhance driving capacity of output OUT_RF_REFCLK, 0x0 for 5 pF, 0x1 for 10 pF, 0x2 for 15 pF, 0x3 for 20 pF."]
    #[inline(always)]
    pub fn dcxo_rfclk_enhance(&self) -> DCXO_RFCLK_ENHANCE_R {
        DCXO_RFCLK_ENHANCE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Xtal mode enable signal, active high"]
    #[inline(always)]
    pub fn xtal_mode(&self) -> XTAL_MODE_R {
        XTAL_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DCXO LDO driving capacity signal, active high"]
    #[inline(always)]
    pub fn dcxo_ldo_inrushb(&self) -> DCXO_LDO_INRUSHB_R {
        DCXO_LDO_INRUSHB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - DCXO bandgap output voltage"]
    #[inline(always)]
    pub fn dcxo_bg(&self) -> DCXO_BG_R {
        DCXO_BG_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:22 - DCXO cap array value\n\nThe capacity cell is 55 fF."]
    #[inline(always)]
    pub fn dcxo_trim(&self) -> DCXO_TRIM_R {
        DCXO_TRIM_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - DCXO current control value"]
    #[inline(always)]
    pub fn dcxo_ictrl(&self) -> DCXO_ICTRL_R {
        DCXO_ICTRL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Clock REQ enable"]
    #[inline(always)]
    pub fn clk_req_enb(&self) -> CLK_REQ_ENB_R {
        CLK_REQ_ENB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The related register configuration is necessary to ensure the reset debounce circuit has a stable clock source. The first time SoC starts up, by default, the reset debounce circuit of SoC uses 32K divided by RC16M. In power-off, software reads the related bit to ensure whether EXT32K is working normally, if it is normal, first switch the clock source of debounce circuit to EXT32K, then close RC16M. Without EXT32K scenario or external RTC scenario, software confirms firstly whether EXT32K is working normally before switching, or software does not close RC16M."]
    #[inline(always)]
    #[must_use]
    pub fn clk16m_rc_en(&mut self) -> CLK16M_RC_EN_W<0> {
        CLK16M_RC_EN_W::new(self)
    }
    #[doc = "Bit 1 - DCXO enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcxo_en(&mut self) -> DCXO_EN_W<1> {
        DCXO_EN_W::new(self)
    }
    #[doc = "Bit 2 - For Debug Use Only.\n\nIt cannot configure to 0 in normal state."]
    #[inline(always)]
    #[must_use]
    pub fn rsto_dly_sel(&mut self) -> RSTO_DLY_SEL_W<2> {
        RSTO_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 4:5 - DCXO rfclk enhance\n\nEnhance driving capacity of output OUT_RF_REFCLK, 0x0 for 5 pF, 0x1 for 10 pF, 0x2 for 15 pF, 0x3 for 20 pF."]
    #[inline(always)]
    #[must_use]
    pub fn dcxo_rfclk_enhance(&mut self) -> DCXO_RFCLK_ENHANCE_W<4> {
        DCXO_RFCLK_ENHANCE_W::new(self)
    }
    #[doc = "Bit 6 - Xtal mode enable signal, active high"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_mode(&mut self) -> XTAL_MODE_W<6> {
        XTAL_MODE_W::new(self)
    }
    #[doc = "Bit 7 - DCXO LDO driving capacity signal, active high"]
    #[inline(always)]
    #[must_use]
    pub fn dcxo_ldo_inrushb(&mut self) -> DCXO_LDO_INRUSHB_W<7> {
        DCXO_LDO_INRUSHB_W::new(self)
    }
    #[doc = "Bits 8:12 - DCXO bandgap output voltage"]
    #[inline(always)]
    #[must_use]
    pub fn dcxo_bg(&mut self) -> DCXO_BG_W<8> {
        DCXO_BG_W::new(self)
    }
    #[doc = "Bits 16:22 - DCXO cap array value\n\nThe capacity cell is 55 fF."]
    #[inline(always)]
    #[must_use]
    pub fn dcxo_trim(&mut self) -> DCXO_TRIM_W<16> {
        DCXO_TRIM_W::new(self)
    }
    #[doc = "Bits 24:27 - DCXO current control value"]
    #[inline(always)]
    #[must_use]
    pub fn dcxo_ictrl(&mut self) -> DCXO_ICTRL_W<24> {
        DCXO_ICTRL_W::new(self)
    }
    #[doc = "Bit 31 - Clock REQ enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_req_enb(&mut self) -> CLK_REQ_ENB_W<31> {
        CLK_REQ_ENB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCXO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcxo_ctrl](index.html) module"]
pub struct DCXO_CTRL_SPEC;
impl crate::RegisterSpec for DCXO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcxo_ctrl::R](R) reader structure"]
impl crate::Readable for DCXO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcxo_ctrl::W](W) writer structure"]
impl crate::Writable for DCXO_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dcxo_ctrl to value 0x883f_10f7"]
impl crate::Resettable for DCXO_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x883f_10f7;
}
