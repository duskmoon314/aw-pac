#[doc = "Register `PLL_CPU_TUN` reader"]
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
#[doc = "Register `PLL_CPU_TUN` writer"]
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
#[doc = "Field `PLL_VCO` reader - VCO range control"]
pub struct PLL_VCO_R(crate::FieldReader<u8>);
impl PLL_VCO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_VCO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_VCO_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_VCO` writer - VCO range control"]
pub struct PLL_VCO_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_VCO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 28)) | ((value as u32 & 7) << 28);
        self.w
    }
}
#[doc = "Field `PLL_VCO_GAIN` reader - KVCO gain control"]
pub struct PLL_VCO_GAIN_R(crate::FieldReader<u8>);
impl PLL_VCO_GAIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_VCO_GAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_VCO_GAIN_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_VCO_GAIN` writer - KVCO gain control"]
pub struct PLL_VCO_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_VCO_GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 24)) | ((value as u32 & 7) << 24);
        self.w
    }
}
#[doc = "Field `PLL_CNT_INT` reader - Counter initial control"]
pub struct PLL_CNT_INT_R(crate::FieldReader<u8>);
impl PLL_CNT_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_CNT_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_CNT_INT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_CNT_INT` writer - Counter initial control"]
pub struct PLL_CNT_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CNT_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `PLL_REG_OD` reader - PLL-REG-OD0 for verify"]
pub struct PLL_REG_OD_R(crate::FieldReader<bool>);
impl PLL_REG_OD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_REG_OD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_REG_OD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_REG_OD` writer - PLL-REG-OD0 for verify"]
pub struct PLL_REG_OD_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_REG_OD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `PLL_B_IN` reader - PLL-B-IN \\[6:0\\]
for verify"]
pub struct PLL_B_IN_R(crate::FieldReader<u8>);
impl PLL_B_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_B_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_B_IN_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_B_IN` writer - PLL-B-IN \\[6:0\\]
for verify"]
pub struct PLL_B_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_B_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `PLL_REG_OD1` reader - PLL-REG-OD1 for verify"]
pub struct PLL_REG_OD1_R(crate::FieldReader<bool>);
impl PLL_REG_OD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_REG_OD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_REG_OD1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_REG_OD1` writer - PLL-REG-OD1 for verify"]
pub struct PLL_REG_OD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_REG_OD1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `PLL_B_OUT` reader - PLL-B-OUT \\[6:0\\]
for verify"]
pub struct PLL_B_OUT_R(crate::FieldReader<u8>);
impl PLL_B_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_B_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_B_OUT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 28:30 - VCO range control"]
    #[inline(always)]
    pub fn pll_vco(&self) -> PLL_VCO_R {
        PLL_VCO_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bits 24:26 - KVCO gain control"]
    #[inline(always)]
    pub fn pll_vco_gain(&self) -> PLL_VCO_GAIN_R {
        PLL_VCO_GAIN_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 16:22 - Counter initial control"]
    #[inline(always)]
    pub fn pll_cnt_int(&self) -> PLL_CNT_INT_R {
        PLL_CNT_INT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - PLL-REG-OD0 for verify"]
    #[inline(always)]
    pub fn pll_reg_od(&self) -> PLL_REG_OD_R {
        PLL_REG_OD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 8:14 - PLL-B-IN \\[6:0\\]
for verify"]
    #[inline(always)]
    pub fn pll_b_in(&self) -> PLL_B_IN_R {
        PLL_B_IN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 7 - PLL-REG-OD1 for verify"]
    #[inline(always)]
    pub fn pll_reg_od1(&self) -> PLL_REG_OD1_R {
        PLL_REG_OD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 0:6 - PLL-B-OUT \\[6:0\\]
for verify"]
    #[inline(always)]
    pub fn pll_b_out(&self) -> PLL_B_OUT_R {
        PLL_B_OUT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30 - VCO range control"]
    #[inline(always)]
    pub fn pll_vco(&mut self) -> PLL_VCO_W {
        PLL_VCO_W { w: self }
    }
    #[doc = "Bits 24:26 - KVCO gain control"]
    #[inline(always)]
    pub fn pll_vco_gain(&mut self) -> PLL_VCO_GAIN_W {
        PLL_VCO_GAIN_W { w: self }
    }
    #[doc = "Bits 16:22 - Counter initial control"]
    #[inline(always)]
    pub fn pll_cnt_int(&mut self) -> PLL_CNT_INT_W {
        PLL_CNT_INT_W { w: self }
    }
    #[doc = "Bit 15 - PLL-REG-OD0 for verify"]
    #[inline(always)]
    pub fn pll_reg_od(&mut self) -> PLL_REG_OD_W {
        PLL_REG_OD_W { w: self }
    }
    #[doc = "Bits 8:14 - PLL-B-IN \\[6:0\\]
for verify"]
    #[inline(always)]
    pub fn pll_b_in(&mut self) -> PLL_B_IN_W {
        PLL_B_IN_W { w: self }
    }
    #[doc = "Bit 7 - PLL-REG-OD1 for verify"]
    #[inline(always)]
    pub fn pll_reg_od1(&mut self) -> PLL_REG_OD1_W {
        PLL_REG_OD1_W { w: self }
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
}
#[doc = "`reset()` method sets PLL_CPU_TUN to value 0"]
impl crate::Resettable for PLL_CPU_TUN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
