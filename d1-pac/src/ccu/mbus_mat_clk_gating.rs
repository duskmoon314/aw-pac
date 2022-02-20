#[doc = "Register `MBUS_MAT_CLK_GATING` reader"]
pub struct R(crate::R<MBUS_MAT_CLK_GATING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBUS_MAT_CLK_GATING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBUS_MAT_CLK_GATING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBUS_MAT_CLK_GATING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MBUS_MAT_CLK_GATING` writer"]
pub struct W(crate::W<MBUS_MAT_CLK_GATING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MBUS_MAT_CLK_GATING_SPEC>;
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
impl From<crate::W<MBUS_MAT_CLK_GATING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MBUS_MAT_CLK_GATING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Gating MBUS Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RISCV_MCLK_EN_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<RISCV_MCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RISCV_MCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RISCV_MCLK_EN` reader - Gating MBUS Clock"]
pub struct RISCV_MCLK_EN_R(crate::FieldReader<bool, RISCV_MCLK_EN_A>);
impl RISCV_MCLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RISCV_MCLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RISCV_MCLK_EN_A {
        match self.bits {
            false => RISCV_MCLK_EN_A::MASK,
            true => RISCV_MCLK_EN_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == RISCV_MCLK_EN_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == RISCV_MCLK_EN_A::PASS
    }
}
impl core::ops::Deref for RISCV_MCLK_EN_R {
    type Target = crate::FieldReader<bool, RISCV_MCLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RISCV_MCLK_EN` writer - Gating MBUS Clock"]
pub struct RISCV_MCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RISCV_MCLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RISCV_MCLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(RISCV_MCLK_EN_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(RISCV_MCLK_EN_A::PASS)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Gating MBUS Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum G2D_MCLK_EN_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<G2D_MCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: G2D_MCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G2D_MCLK_EN` reader - Gating MBUS Clock"]
pub struct G2D_MCLK_EN_R(crate::FieldReader<bool, G2D_MCLK_EN_A>);
impl G2D_MCLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G2D_MCLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> G2D_MCLK_EN_A {
        match self.bits {
            false => G2D_MCLK_EN_A::MASK,
            true => G2D_MCLK_EN_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == G2D_MCLK_EN_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == G2D_MCLK_EN_A::PASS
    }
}
impl core::ops::Deref for G2D_MCLK_EN_R {
    type Target = crate::FieldReader<bool, G2D_MCLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G2D_MCLK_EN` writer - Gating MBUS Clock"]
pub struct G2D_MCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> G2D_MCLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: G2D_MCLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(G2D_MCLK_EN_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(G2D_MCLK_EN_A::PASS)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Gating MBUS Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSI_MCLK_EN_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<CSI_MCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CSI_MCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSI_MCLK_EN` reader - Gating MBUS Clock"]
pub struct CSI_MCLK_EN_R(crate::FieldReader<bool, CSI_MCLK_EN_A>);
impl CSI_MCLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSI_MCLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSI_MCLK_EN_A {
        match self.bits {
            false => CSI_MCLK_EN_A::MASK,
            true => CSI_MCLK_EN_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == CSI_MCLK_EN_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == CSI_MCLK_EN_A::PASS
    }
}
impl core::ops::Deref for CSI_MCLK_EN_R {
    type Target = crate::FieldReader<bool, CSI_MCLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSI_MCLK_EN` writer - Gating MBUS Clock"]
pub struct CSI_MCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSI_MCLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSI_MCLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(CSI_MCLK_EN_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(CSI_MCLK_EN_A::PASS)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Gating MBUS Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TVIN_MCLK_EN_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<TVIN_MCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TVIN_MCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TVIN_MCLK_EN` reader - Gating MBUS Clock"]
pub struct TVIN_MCLK_EN_R(crate::FieldReader<bool, TVIN_MCLK_EN_A>);
impl TVIN_MCLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TVIN_MCLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TVIN_MCLK_EN_A {
        match self.bits {
            false => TVIN_MCLK_EN_A::MASK,
            true => TVIN_MCLK_EN_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == TVIN_MCLK_EN_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == TVIN_MCLK_EN_A::PASS
    }
}
impl core::ops::Deref for TVIN_MCLK_EN_R {
    type Target = crate::FieldReader<bool, TVIN_MCLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TVIN_MCLK_EN` writer - Gating MBUS Clock"]
pub struct TVIN_MCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TVIN_MCLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TVIN_MCLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(TVIN_MCLK_EN_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(TVIN_MCLK_EN_A::PASS)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Gating MBUS Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CE_MCLK_EN_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<CE_MCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CE_MCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CE_MCLK_EN` reader - Gating MBUS Clock"]
pub struct CE_MCLK_EN_R(crate::FieldReader<bool, CE_MCLK_EN_A>);
impl CE_MCLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CE_MCLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CE_MCLK_EN_A {
        match self.bits {
            false => CE_MCLK_EN_A::MASK,
            true => CE_MCLK_EN_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == CE_MCLK_EN_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == CE_MCLK_EN_A::PASS
    }
}
impl core::ops::Deref for CE_MCLK_EN_R {
    type Target = crate::FieldReader<bool, CE_MCLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CE_MCLK_EN` writer - Gating MBUS Clock"]
pub struct CE_MCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_MCLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CE_MCLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(CE_MCLK_EN_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(CE_MCLK_EN_A::PASS)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Gating MBUS Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VE_MCLK_EN_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<VE_MCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VE_MCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VE_MCLK_EN` reader - Gating MBUS Clock"]
pub struct VE_MCLK_EN_R(crate::FieldReader<bool, VE_MCLK_EN_A>);
impl VE_MCLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VE_MCLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VE_MCLK_EN_A {
        match self.bits {
            false => VE_MCLK_EN_A::MASK,
            true => VE_MCLK_EN_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == VE_MCLK_EN_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == VE_MCLK_EN_A::PASS
    }
}
impl core::ops::Deref for VE_MCLK_EN_R {
    type Target = crate::FieldReader<bool, VE_MCLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VE_MCLK_EN` writer - Gating MBUS Clock"]
pub struct VE_MCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VE_MCLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VE_MCLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(VE_MCLK_EN_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(VE_MCLK_EN_A::PASS)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Gating MBUS Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_MCLK_EN_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<DMA_MCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_MCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_MCLK_EN` reader - Gating MBUS Clock"]
pub struct DMA_MCLK_EN_R(crate::FieldReader<bool, DMA_MCLK_EN_A>);
impl DMA_MCLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_MCLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_MCLK_EN_A {
        match self.bits {
            false => DMA_MCLK_EN_A::MASK,
            true => DMA_MCLK_EN_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == DMA_MCLK_EN_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == DMA_MCLK_EN_A::PASS
    }
}
impl core::ops::Deref for DMA_MCLK_EN_R {
    type Target = crate::FieldReader<bool, DMA_MCLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_MCLK_EN` writer - Gating MBUS Clock"]
pub struct DMA_MCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_MCLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_MCLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(DMA_MCLK_EN_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(DMA_MCLK_EN_A::PASS)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn riscv_mclk_en(&self) -> RISCV_MCLK_EN_R {
        RISCV_MCLK_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn g2d_mclk_en(&self) -> G2D_MCLK_EN_R {
        G2D_MCLK_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn csi_mclk_en(&self) -> CSI_MCLK_EN_R {
        CSI_MCLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn tvin_mclk_en(&self) -> TVIN_MCLK_EN_R {
        TVIN_MCLK_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn ce_mclk_en(&self) -> CE_MCLK_EN_R {
        CE_MCLK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn ve_mclk_en(&self) -> VE_MCLK_EN_R {
        VE_MCLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn dma_mclk_en(&self) -> DMA_MCLK_EN_R {
        DMA_MCLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn riscv_mclk_en(&mut self) -> RISCV_MCLK_EN_W {
        RISCV_MCLK_EN_W { w: self }
    }
    #[doc = "Bit 10 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn g2d_mclk_en(&mut self) -> G2D_MCLK_EN_W {
        G2D_MCLK_EN_W { w: self }
    }
    #[doc = "Bit 8 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn csi_mclk_en(&mut self) -> CSI_MCLK_EN_W {
        CSI_MCLK_EN_W { w: self }
    }
    #[doc = "Bit 7 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn tvin_mclk_en(&mut self) -> TVIN_MCLK_EN_W {
        TVIN_MCLK_EN_W { w: self }
    }
    #[doc = "Bit 2 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn ce_mclk_en(&mut self) -> CE_MCLK_EN_W {
        CE_MCLK_EN_W { w: self }
    }
    #[doc = "Bit 1 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn ve_mclk_en(&mut self) -> VE_MCLK_EN_W {
        VE_MCLK_EN_W { w: self }
    }
    #[doc = "Bit 0 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn dma_mclk_en(&mut self) -> DMA_MCLK_EN_W {
        DMA_MCLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MBUS Master Clock Gating Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbus_mat_clk_gating](index.html) module"]
pub struct MBUS_MAT_CLK_GATING_SPEC;
impl crate::RegisterSpec for MBUS_MAT_CLK_GATING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbus_mat_clk_gating::R](R) reader structure"]
impl crate::Readable for MBUS_MAT_CLK_GATING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mbus_mat_clk_gating::W](W) writer structure"]
impl crate::Writable for MBUS_MAT_CLK_GATING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MBUS_MAT_CLK_GATING to value 0"]
impl crate::Resettable for MBUS_MAT_CLK_GATING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
