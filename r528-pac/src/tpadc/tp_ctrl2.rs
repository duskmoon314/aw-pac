#[doc = "Register `TP_CTRL2` reader"]
pub struct R(crate::R<TP_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TP_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TP_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TP_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TP_CTRL2` writer"]
pub struct W(crate::W<TP_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TP_CTRL2_SPEC>;
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
impl From<crate::W<TP_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TP_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TP_SENSITIVE_ADJUST` reader - Internal Pull-up Resistor Control"]
pub type TP_SENSITIVE_ADJUST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TP_SENSITIVE_ADJUST` writer - Internal Pull-up Resistor Control"]
pub type TP_SENSITIVE_ADJUST_W<'a> = crate::FieldWriter<'a, u32, TP_CTRL2_SPEC, u8, u8, 4, 28>;
#[doc = "TP Access Data Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TP_FIFO_MODE_SELECT_A {
    #[doc = "0: FIFO store X1 Y1 data for single touch no pressure mode"]
    X1Y1 = 0,
    #[doc = "1: FIFO store X1 Y1 ΔX ΔY data for dual touch no pressure mode"]
    X1Y1DXDY = 1,
    #[doc = "2: FIFO store X1 Y1 X2 Y2 data for dual touch no pressure mode"]
    X1Y1X2Y2 = 2,
    #[doc = "3: FIFO store X1 Y1 X2 Y2 Z1 Z2 data for dual touch and pressure mode"]
    X1Y1X2Y2Z1Z2 = 3,
}
impl From<TP_FIFO_MODE_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TP_FIFO_MODE_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TP_FIFO_MODE_SELECT` reader - TP Access Data Mode Select"]
pub type TP_FIFO_MODE_SELECT_R = crate::FieldReader<u8, TP_FIFO_MODE_SELECT_A>;
impl TP_FIFO_MODE_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_FIFO_MODE_SELECT_A {
        match self.bits {
            0 => TP_FIFO_MODE_SELECT_A::X1Y1,
            1 => TP_FIFO_MODE_SELECT_A::X1Y1DXDY,
            2 => TP_FIFO_MODE_SELECT_A::X1Y1X2Y2,
            3 => TP_FIFO_MODE_SELECT_A::X1Y1X2Y2Z1Z2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `X1Y1`"]
    #[inline(always)]
    pub fn is_x1y1(&self) -> bool {
        *self == TP_FIFO_MODE_SELECT_A::X1Y1
    }
    #[doc = "Checks if the value of the field is `X1Y1DXDY`"]
    #[inline(always)]
    pub fn is_x1y1d_xd_y(&self) -> bool {
        *self == TP_FIFO_MODE_SELECT_A::X1Y1DXDY
    }
    #[doc = "Checks if the value of the field is `X1Y1X2Y2`"]
    #[inline(always)]
    pub fn is_x1y1x2y2(&self) -> bool {
        *self == TP_FIFO_MODE_SELECT_A::X1Y1X2Y2
    }
    #[doc = "Checks if the value of the field is `X1Y1X2Y2Z1Z2`"]
    #[inline(always)]
    pub fn is_x1y1x2y2z1z2(&self) -> bool {
        *self == TP_FIFO_MODE_SELECT_A::X1Y1X2Y2Z1Z2
    }
}
#[doc = "Field `TP_FIFO_MODE_SELECT` writer - TP Access Data Mode Select"]
pub type TP_FIFO_MODE_SELECT_W<'a> =
    crate::FieldWriterSafe<'a, u32, TP_CTRL2_SPEC, u8, TP_FIFO_MODE_SELECT_A, 2, 26>;
impl<'a> TP_FIFO_MODE_SELECT_W<'a> {
    #[doc = "FIFO store X1 Y1 data for single touch no pressure mode"]
    #[inline(always)]
    pub fn x1y1(self) -> &'a mut W {
        self.variant(TP_FIFO_MODE_SELECT_A::X1Y1)
    }
    #[doc = "FIFO store X1 Y1 ΔX ΔY data for dual touch no pressure mode"]
    #[inline(always)]
    pub fn x1y1d_xd_y(self) -> &'a mut W {
        self.variant(TP_FIFO_MODE_SELECT_A::X1Y1DXDY)
    }
    #[doc = "FIFO store X1 Y1 X2 Y2 data for dual touch no pressure mode"]
    #[inline(always)]
    pub fn x1y1x2y2(self) -> &'a mut W {
        self.variant(TP_FIFO_MODE_SELECT_A::X1Y1X2Y2)
    }
    #[doc = "FIFO store X1 Y1 X2 Y2 Z1 Z2 data for dual touch and pressure mode"]
    #[inline(always)]
    pub fn x1y1x2y2z1z2(self) -> &'a mut W {
        self.variant(TP_FIFO_MODE_SELECT_A::X1Y1X2Y2Z1Z2)
    }
}
#[doc = "TP Pressure Measurement Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRE_MEA_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<PRE_MEA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PRE_MEA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRE_MEA_EN` reader - TP Pressure Measurement Enable Control"]
pub type PRE_MEA_EN_R = crate::BitReader<PRE_MEA_EN_A>;
impl PRE_MEA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRE_MEA_EN_A {
        match self.bits {
            false => PRE_MEA_EN_A::DISABLE,
            true => PRE_MEA_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PRE_MEA_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PRE_MEA_EN_A::ENABLE
    }
}
#[doc = "Field `PRE_MEA_EN` writer - TP Pressure Measurement Enable Control"]
pub type PRE_MEA_EN_W<'a> = crate::BitWriter<'a, u32, TP_CTRL2_SPEC, PRE_MEA_EN_A, 24>;
impl<'a> PRE_MEA_EN_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PRE_MEA_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRE_MEA_EN_A::ENABLE)
    }
}
#[doc = "Field `PRE_MEA_THRE_CNT` reader - TP Pressure Measurement Threshold Control"]
pub type PRE_MEA_THRE_CNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRE_MEA_THRE_CNT` writer - TP Pressure Measurement Threshold Control"]
pub type PRE_MEA_THRE_CNT_W<'a> = crate::FieldWriter<'a, u32, TP_CTRL2_SPEC, u32, u32, 24, 0>;
impl R {
    #[doc = "Bits 28:31 - Internal Pull-up Resistor Control"]
    #[inline(always)]
    pub fn tp_sensitive_adjust(&self) -> TP_SENSITIVE_ADJUST_R {
        TP_SENSITIVE_ADJUST_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 26:27 - TP Access Data Mode Select"]
    #[inline(always)]
    pub fn tp_fifo_mode_select(&self) -> TP_FIFO_MODE_SELECT_R {
        TP_FIFO_MODE_SELECT_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 24 - TP Pressure Measurement Enable Control"]
    #[inline(always)]
    pub fn pre_mea_en(&self) -> PRE_MEA_EN_R {
        PRE_MEA_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 0:23 - TP Pressure Measurement Threshold Control"]
    #[inline(always)]
    pub fn pre_mea_thre_cnt(&self) -> PRE_MEA_THRE_CNT_R {
        PRE_MEA_THRE_CNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 28:31 - Internal Pull-up Resistor Control"]
    #[inline(always)]
    pub fn tp_sensitive_adjust(&mut self) -> TP_SENSITIVE_ADJUST_W {
        TP_SENSITIVE_ADJUST_W::new(self)
    }
    #[doc = "Bits 26:27 - TP Access Data Mode Select"]
    #[inline(always)]
    pub fn tp_fifo_mode_select(&mut self) -> TP_FIFO_MODE_SELECT_W {
        TP_FIFO_MODE_SELECT_W::new(self)
    }
    #[doc = "Bit 24 - TP Pressure Measurement Enable Control"]
    #[inline(always)]
    pub fn pre_mea_en(&mut self) -> PRE_MEA_EN_W {
        PRE_MEA_EN_W::new(self)
    }
    #[doc = "Bits 0:23 - TP Pressure Measurement Threshold Control"]
    #[inline(always)]
    pub fn pre_mea_thre_cnt(&mut self) -> PRE_MEA_THRE_CNT_W {
        PRE_MEA_THRE_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TP Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tp_ctrl2](index.html) module"]
pub struct TP_CTRL2_SPEC;
impl crate::RegisterSpec for TP_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tp_ctrl2::R](R) reader structure"]
impl crate::Readable for TP_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tp_ctrl2::W](W) writer structure"]
impl crate::Writable for TP_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TP_CTRL2 to value 0"]
impl crate::Resettable for TP_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
